//! 页面预渲染辅助层：
//! `run_compose_pipeline()` 封装 knowledge planning → research → compose，
//! 并负责 pipeline checkpoint / cache 驱动的中断恢复。

use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::time::Instant;

use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use crate::debug_trace;
use crate::domain::checkpoint::{
    compute_facts_input_hash, PipelineCheckpoint, PipelineRuntimeSummary, PipelineStage,
    UnitRuntimeGate,
};
use crate::domain::compose::PageDraft;
use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use crate::domain::module_tree::ModuleTree;
use crate::domain::research::{
    DomainResearch, PageDiagramDigest, PageDigest, PageSectionDigest, SystemResearch, UnitResearch,
};
use crate::domain::steering::SteeringConfig;
use crate::generation::compose_engine::{compose_leaf_page, compose_parent_page};
use crate::generation::knowledge_planner::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use crate::generation::planner::{plan_pages_from_knowledge_tree, PlannedPage};
use crate::generation::research_engine::{ResearchDataSource, ResearchProvider};
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
use crate::repo::symbols::ParsedSymbolsSnapshot;
use crate::storage::sqlite_store;

/// 新 compose pipeline 的统一输出。
pub struct ComposePipelineOutput {
    pub page_drafts: Vec<PageDraft>,
    pub digests: BTreeMap<String, PageDigest>,
    pub knowledge_tree: KnowledgeTree,
    pub planned_pages: Vec<PlannedPage>,
}

/// 封装 knowledge planning → research → compose 全链路。
/// 由 init / rebuild / update 共用。
/// `research_provider` 控制 research 层的实现——LLM-backed 或 structural-only。
pub fn run_compose_pipeline(
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    symbol_snapshot: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
    graph_analysis: &GraphAnalysisSnapshot,
    graph_summary: &GraphSummary,
    steering: &SteeringConfig,
    research_provider: &dyn ResearchProvider,
) -> io::Result<ComposePipelineOutput> {
    run_compose_pipeline_with_action(
        "compose_pipeline",
        repo_root,
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
        symbol_snapshot,
        resolved_graph,
        graph_analysis,
        graph_summary,
        steering,
        research_provider,
    )
}

/// 带 workflow action 的 compose pipeline 入口。
pub fn run_compose_pipeline_with_action(
    workflow_action: &str,
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    symbol_snapshot: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
    graph_analysis: &GraphAnalysisSnapshot,
    graph_summary: &GraphSummary,
    steering: &SteeringConfig,
    research_provider: &dyn ResearchProvider,
) -> io::Result<ComposePipelineOutput> {
    let conn = sqlite_store::open_db(repo_root)?;
    let facts_input_hash = compute_facts_input_hash(scan_report, module_tree);
    let resume_enabled = prepare_resume_state(&conn, &facts_input_hash)?;

    let domains = discover_knowledge_domains(
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
        graph_summary,
        steering,
    );
    let units = plan_knowledge_units(
        &domains,
        module_tree,
        scan_report,
        module_contexts,
        steering,
    );
    let knowledge_tree = build_knowledge_tree(domains.clone(), units.clone());
    if !resume_enabled {
        sqlite_store::write_knowledge_domains(&conn, &domains)?;
        sqlite_store::write_knowledge_units(&conn, &units)?;
    }
    initialize_runtime_gates(&conn, workflow_action, &facts_input_hash, &knowledge_tree)?;
    let mut runtime_summary =
        load_runtime_summary(&conn)?.unwrap_or_else(|| PipelineRuntimeSummary {
            facts_input_hash: facts_input_hash.clone(),
            workflow_action: workflow_action.to_string(),
            runtime_state: "knowledge_planning_complete".to_string(),
            researched_units: 0,
            compose_ready_units: 0,
            composed_units: 0,
            assembled_pages: 0,
            blocked_units: Vec::new(),
            last_ready_stage: Some(PipelineStage::KnowledgePlanning.as_str().to_string()),
            last_interrupted_stage: None,
            summary_reason: None,
            current_research_unit_id: None,
            current_research_unit_type: None,
            current_research_started_at: None,
            last_researched_unit_id: None,
            last_research_elapsed_ms: None,
        });
    runtime_summary.workflow_action = workflow_action.to_string();
    runtime_summary.facts_input_hash = facts_input_hash.clone();
    runtime_summary.runtime_state = "researching".to_string();
    runtime_summary.last_ready_stage = Some(PipelineStage::KnowledgePlanning.as_str().to_string());
    runtime_summary.last_interrupted_stage = None;
    runtime_summary.summary_reason = None;
    runtime_summary.current_research_unit_id = None;
    runtime_summary.current_research_unit_type = None;
    runtime_summary.current_research_started_at = None;
    runtime_summary.last_researched_unit_id = None;
    runtime_summary.last_research_elapsed_ms = None;
    save_runtime_summary(&conn, &runtime_summary)?;

    let research_ds = ResearchDataSource {
        report: scan_report,
        module_tree,
        repo_context,
        module_contexts,
        symbol_snapshot,
        resolved_graph,
        graph_analysis,
        graph_summary,
        knowledge_tree: &knowledge_tree,
    };

    let system_input_hash = compute_system_input_hash(&facts_input_hash, &research_ds, steering);
    let system_research = load_or_compute_research(
        &conn,
        "system",
        "system",
        &facts_input_hash,
        &system_input_hash,
        resume_enabled,
        PipelineStage::ResearchSystem,
        None,
        || research_provider.research_system(&research_ds),
    )
    .map_err(|error| {
        runtime_summary.runtime_state = "interrupted".to_string();
        runtime_summary.last_interrupted_stage =
            Some(PipelineStage::ResearchSystem.as_str().to_string());
        runtime_summary.summary_reason = Some(error.to_string());
        let _ = save_runtime_summary(&conn, &runtime_summary);
        error
    })?;

    let mut domain_researches = BTreeMap::new();
    for domain in knowledge_tree.domains.values() {
        let domain_input_hash = compute_domain_input_hash(&facts_input_hash, domain, steering);
        let research = load_or_compute_research(
            &conn,
            "domain",
            &domain.id,
            &facts_input_hash,
            &domain_input_hash,
            resume_enabled,
            PipelineStage::ResearchDomain,
            Some(domain.id.clone()),
            || research_provider.research_domain(domain, &research_ds),
        )
        .map_err(|error| {
            runtime_summary.runtime_state = "interrupted".to_string();
            runtime_summary.last_interrupted_stage =
                Some(PipelineStage::ResearchDomain.as_str().to_string());
            runtime_summary.summary_reason = Some(error.to_string());
            let _ = save_runtime_summary(&conn, &runtime_summary);
            error
        })?;
        domain_researches.insert(domain.id.clone(), research);
    }

    let pipeline_order = build_pipeline_unit_order(&knowledge_tree);

    let mut unit_researches = BTreeMap::new();
    let mut unit_digests_for_research = BTreeMap::new();
    for unit_id in &pipeline_order {
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        let research_started_at = Instant::now();
        runtime_summary.current_research_unit_id = Some(unit.id.clone());
        runtime_summary.current_research_unit_type = Some(unit.unit_type.as_str().to_string());
        runtime_summary.current_research_started_at = Some(current_runtime_timestamp());
        save_runtime_summary(&conn, &runtime_summary)?;
        let child_digests =
            collect_compose_input_digests(unit, &knowledge_tree, &unit_digests_for_research);
        let unit_input_hash =
            compute_unit_input_hash(&facts_input_hash, unit, &child_digests, steering);
        let mut research = load_or_compute_research(
            &conn,
            "unit",
            &unit.id,
            &facts_input_hash,
            &unit_input_hash,
            resume_enabled,
            PipelineStage::ResearchUnit,
            Some(unit.id.clone()),
            || research_provider.research_unit(unit, &research_ds, &child_digests),
        )
        .map_err(|error| {
            runtime_summary.runtime_state = "interrupted".to_string();
            runtime_summary.last_interrupted_stage =
                Some(PipelineStage::ResearchUnit.as_str().to_string());
            runtime_summary.summary_reason = Some(error.to_string());
            let _ = save_runtime_summary(&conn, &runtime_summary);
            error
        })?;
        enrich_parent_research(
            unit,
            &mut research,
            &system_research,
            &domain_researches,
            &child_digests,
        );
        record_unit_research_stop(unit, &research);

        let digest = build_research_digest(unit, &research);
        unit_digests_for_research.insert(unit.id.clone(), digest);
        unit_researches.insert(unit.id.clone(), research);
        runtime_summary.current_research_unit_id = None;
        runtime_summary.current_research_unit_type = None;
        runtime_summary.current_research_started_at = None;
        runtime_summary.last_researched_unit_id = Some(unit.id.clone());
        runtime_summary.last_research_elapsed_ms = Some(
            research_started_at
                .elapsed()
                .as_millis()
                .min(u64::MAX as u128) as u64,
        );
        runtime_summary.researched_units += 1;
        runtime_summary.compose_ready_units += 1;
        runtime_summary.last_ready_stage = Some(PipelineStage::ResearchUnit.as_str().to_string());
        write_unit_gate(
            &conn,
            unit,
            "ready",
            "ready",
            "pending",
            Some(PipelineStage::ResearchUnit.as_str().to_string()),
            None,
            Vec::new(),
        )?;
        save_runtime_summary(&conn, &runtime_summary)?;
    }

    record_workflow_research_stop_summary(&unit_researches);
    runtime_summary.runtime_state = "compose_pending".to_string();
    runtime_summary.last_interrupted_stage = None;
    runtime_summary.summary_reason = None;
    runtime_summary.current_research_unit_id = None;
    runtime_summary.current_research_unit_type = None;
    runtime_summary.current_research_started_at = None;
    save_runtime_summary(&conn, &runtime_summary)?;

    let mut page_drafts = Vec::new();
    let mut digests = BTreeMap::new();
    for unit_id in &pipeline_order {
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        if resume_enabled {
            if let (Some(draft), Some(digest)) = (
                read_cached_json::<PageDraft, _>(
                    &conn,
                    unit.id.as_str(),
                    sqlite_store::read_page_draft,
                )?,
                read_cached_json::<PageDigest, _>(
                    &conn,
                    unit.id.as_str(),
                    sqlite_store::read_page_digest,
                )?,
            ) {
                digests.insert(unit.id.clone(), digest);
                page_drafts.push(draft);
                persist_compose_progress(
                    &conn,
                    &mut runtime_summary,
                    unit,
                    stage_for_compose_error(unit),
                )?;
                continue;
            }
        }

        let child_digests = collect_compose_input_digests(unit, &knowledge_tree, &digests);
        let (draft, digest, stage) = compose_unit_page(unit, &child_digests, &unit_researches)
            .map_err(|error| {
                runtime_summary.runtime_state = "interrupted".to_string();
                runtime_summary.last_interrupted_stage =
                    Some(stage_for_compose_error(unit).as_str().to_string());
                runtime_summary.summary_reason = Some(error.to_string());
                let _ = save_runtime_summary(&conn, &runtime_summary);
                save_checkpoint_and_return(
                    &conn,
                    &facts_input_hash,
                    stage_for_compose_error(unit),
                    Some(unit.id.clone()),
                    error,
                )
            })?;

        persist_compose_result(&conn, unit, &draft, &digest)?;
        digests.insert(unit.id.clone(), digest);
        page_drafts.push(draft);
        persist_compose_progress(&conn, &mut runtime_summary, unit, stage)?;
    }

    let planned_pages = plan_pages_from_knowledge_tree(&knowledge_tree);
    runtime_summary.runtime_state = "compose_complete".to_string();
    runtime_summary.last_interrupted_stage = None;
    runtime_summary.summary_reason = None;
    save_runtime_summary(&conn, &runtime_summary)?;
    Ok(ComposePipelineOutput {
        page_drafts,
        digests,
        knowledge_tree,
        planned_pages,
    })
}

fn prepare_resume_state(conn: &Connection, facts_input_hash: &str) -> io::Result<bool> {
    let checkpoint = sqlite_store::read_pipeline_checkpoint(conn)?;
    let runtime_summary = load_runtime_summary(conn)?;
    let Some((_checkpoint_id, checkpoint_hash, _stage, _target, _message)) = checkpoint else {
        if let Some(summary) = runtime_summary {
            if summary.facts_input_hash == facts_input_hash
                && summary.runtime_state != "completed"
                && summary.runtime_state != "assemble_complete"
            {
                return Ok(true);
            }
        }
        sqlite_store::clear_page_drafts(conn)?;
        sqlite_store::clear_page_digests(conn)?;
        sqlite_store::clear_unit_runtime_gates(conn)?;
        return Ok(false);
    };

    if checkpoint_hash != facts_input_hash {
        sqlite_store::clear_pipeline_checkpoint(conn)?;
        sqlite_store::clear_page_drafts(conn)?;
        sqlite_store::clear_page_digests(conn)?;
        sqlite_store::clear_unit_runtime_gates(conn)?;
        return Ok(false);
    }

    Ok(true)
}

fn build_pipeline_unit_order(knowledge_tree: &KnowledgeTree) -> Vec<String> {
    let mut non_system = Vec::new();
    let mut system_units = Vec::new();

    for unit_id in &knowledge_tree.processing_order {
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        if matches!(unit.unit_type, UnitType::Overview | UnitType::Architecture) {
            system_units.push(unit.id.clone());
        } else {
            non_system.push(unit.id.clone());
        }
    }

    system_units.sort_by_key(|unit_id| {
        knowledge_tree
            .get_unit(unit_id)
            .map(|unit| match unit.unit_type {
                UnitType::Architecture => 0usize,
                UnitType::Overview => 1usize,
                _ => 2usize,
            })
            .unwrap_or(2usize)
    });

    non_system.extend(system_units);
    non_system
}

fn initialize_runtime_gates(
    conn: &Connection,
    workflow_action: &str,
    facts_input_hash: &str,
    knowledge_tree: &KnowledgeTree,
) -> io::Result<()> {
    sqlite_store::clear_unit_runtime_gates(conn)?;
    for unit_id in &knowledge_tree.processing_order {
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        write_unit_gate(
            conn,
            unit,
            "pending",
            "pending",
            "pending",
            None,
            None,
            Vec::new(),
        )?;
    }

    let summary = PipelineRuntimeSummary {
        facts_input_hash: facts_input_hash.to_string(),
        workflow_action: workflow_action.to_string(),
        runtime_state: "knowledge_planning_complete".to_string(),
        researched_units: 0,
        compose_ready_units: 0,
        composed_units: 0,
        assembled_pages: 0,
        blocked_units: Vec::new(),
        last_ready_stage: Some(PipelineStage::KnowledgePlanning.as_str().to_string()),
        last_interrupted_stage: None,
        summary_reason: None,
        current_research_unit_id: None,
        current_research_unit_type: None,
        current_research_started_at: None,
        last_researched_unit_id: None,
        last_research_elapsed_ms: None,
    };
    save_runtime_summary(conn, &summary)
}

fn save_runtime_summary(conn: &Connection, summary: &PipelineRuntimeSummary) -> io::Result<()> {
    let summary_json = serde_json::to_string(summary)
        .map_err(|error| io::Error::other(format!("serialize runtime summary: {error}")))?;
    sqlite_store::runtime_meta_set(conn, "pipeline_runtime_summary", &summary_json)
}

fn load_runtime_summary(conn: &Connection) -> io::Result<Option<PipelineRuntimeSummary>> {
    let Some(raw) = sqlite_store::runtime_meta_get(conn, "pipeline_runtime_summary")? else {
        return Ok(None);
    };
    serde_json::from_str(&raw)
        .map(Some)
        .map_err(|error| io::Error::other(format!("deserialize runtime summary: {error}")))
}

fn current_runtime_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn write_unit_gate(
    conn: &Connection,
    unit: &KnowledgeUnit,
    research_status: &str,
    compose_status: &str,
    assemble_status: &str,
    last_ready_stage: Option<String>,
    blocked_reason: Option<String>,
    missing_dependencies: Vec<String>,
) -> io::Result<()> {
    sqlite_store::write_unit_runtime_gate(
        conn,
        &UnitRuntimeGate {
            unit_id: unit.id.clone(),
            unit_type: unit.unit_type.as_str().to_string(),
            research_status: research_status.to_string(),
            compose_status: compose_status.to_string(),
            assemble_status: assemble_status.to_string(),
            last_ready_stage,
            blocked_reason,
            missing_dependencies,
            updated_at: current_runtime_timestamp(),
        },
    )
}

fn load_or_compute_research<T, F>(
    conn: &Connection,
    research_type: &str,
    target_id: &str,
    facts_input_hash: &str,
    input_hash: &str,
    resume_enabled: bool,
    stage: PipelineStage,
    interrupted_target_id: Option<String>,
    compute: F,
) -> io::Result<T>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> io::Result<T>,
{
    if let Some(cached) = read_cached_research(conn, research_type, target_id, input_hash)? {
        return Ok(cached);
    }

    let result = compute().map_err(|error| {
        save_checkpoint_and_return(conn, facts_input_hash, stage, interrupted_target_id, error)
    })?;
    let result_json = serde_json::to_string(&result)
        .map_err(|error| io::Error::other(format!("serialize research cache: {error}")))?;
    sqlite_store::write_research_cache(
        conn,
        research_type,
        target_id,
        input_hash,
        &result_json,
        None,
    )?;

    if !resume_enabled {
        sqlite_store::clear_pipeline_checkpoint(conn)?;
    }

    Ok(result)
}

fn read_cached_research<T: DeserializeOwned>(
    conn: &Connection,
    research_type: &str,
    target_id: &str,
    input_hash: &str,
) -> io::Result<Option<T>> {
    let Some(raw) = sqlite_store::read_research_cache(conn, research_type, target_id, input_hash)?
    else {
        return Ok(None);
    };
    serde_json::from_str(&raw)
        .map(Some)
        .map_err(|error| io::Error::other(format!("deserialize research cache: {error}")))
}

fn read_cached_json<T, F>(conn: &Connection, unit_id: &str, reader: F) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
    F: Fn(&Connection, &str) -> io::Result<Option<String>>,
{
    let Some(raw) = reader(conn, unit_id)? else {
        return Ok(None);
    };
    serde_json::from_str(&raw)
        .map(Some)
        .map_err(|error| io::Error::other(format!("deserialize cached json: {error}")))
}

fn build_research_digest(unit: &KnowledgeUnit, research: &UnitResearch) -> PageDigest {
    PageDigest {
        digest_id: crate::domain::stable_id::stable_id("digest", &unit.id),
        unit_id: unit.id.clone(),
        page_id: crate::domain::stable_id::stable_id("page", &unit.relative_path),
        title: unit.title.clone(),
        decomposition_profile: research.decomposition_profile.clone(),
        research_profile: research.research_profile.clone(),
        summary: research.summary.clone(),
        key_topics: research
            .section_plan
            .iter()
            .map(|section| section.title.clone())
            .collect(),
        key_sources: research.key_sources.clone(),
        citations: research
            .evidence_clusters
            .iter()
            .flat_map(|cluster| cluster.citations.iter().cloned())
            .fold(Vec::new(), |mut acc, citation| {
                let already_present =
                    acc.iter()
                        .any(|existing: &crate::domain::research::SourceCitation| {
                            existing.path == citation.path
                                && existing.start_line == citation.start_line
                                && existing.end_line == citation.end_line
                                && existing.note == citation.note
                        });
                if !already_present && acc.len() < 12 {
                    acc.push(citation);
                }
                acc
            }),
        section_digests: build_section_digests_from_research(research),
        diagram_digests: build_diagram_digests_from_research(research),
        readiness_stage: "research_ready".to_string(),
    }
}

fn build_section_digests_from_research(research: &UnitResearch) -> Vec<PageSectionDigest> {
    research
        .section_plan
        .iter()
        .map(|section| {
            let citations = section
                .evidence_cluster_keys
                .iter()
                .flat_map(|cluster_key| {
                    research
                        .evidence_clusters
                        .iter()
                        .find(|cluster| cluster.cluster_key == *cluster_key)
                        .into_iter()
                        .flat_map(|cluster| cluster.citations.iter().cloned())
                })
                .collect::<Vec<_>>();
            PageSectionDigest {
                digest_id: crate::domain::stable_id::stable_id(
                    "section-digest",
                    format!("{}:{}", research.unit_id, section.section_key),
                ),
                section_key: section.section_key.clone(),
                title: section.title.clone(),
                summary: section.section_summary.clone(),
                key_sources: citations
                    .iter()
                    .map(|citation| citation.path.clone())
                    .collect(),
                citations,
            }
        })
        .collect()
}

fn build_diagram_digests_from_research(research: &UnitResearch) -> Vec<PageDiagramDigest> {
    research
        .diagram_suggestions
        .iter()
        .map(|diagram| PageDiagramDigest {
            digest_id: crate::domain::stable_id::stable_id(
                "diagram-digest",
                format!("{}:{}", research.unit_id, diagram.title),
            ),
            diagram_type: diagram.diagram_type.clone(),
            title: diagram.title.clone(),
            summary: diagram.description.clone(),
        })
        .collect()
}

pub(crate) fn collect_compose_input_digests(
    unit: &KnowledgeUnit,
    knowledge_tree: &KnowledgeTree,
    digests: &BTreeMap<String, PageDigest>,
) -> Vec<PageDigest> {
    if matches!(unit.unit_type, UnitType::Overview | UnitType::Architecture) {
        return knowledge_tree
            .units
            .values()
            .filter(|candidate| candidate.unit_type == UnitType::DomainIndex)
            .filter_map(|candidate| digests.get(&candidate.id).cloned())
            .collect();
    }

    unit.child_unit_ids
        .iter()
        .filter_map(|child_id| digests.get(child_id).cloned())
        .collect()
}

fn compose_unit_page(
    unit: &KnowledgeUnit,
    child_digests: &[PageDigest],
    unit_researches: &BTreeMap<String, UnitResearch>,
) -> io::Result<(PageDraft, PageDigest, PipelineStage)> {
    let Some(unit_research) = unit_researches.get(&unit.id) else {
        return Err(io::Error::other(format!(
            "missing unit research for {}",
            unit.id
        )));
    };
    let (draft, mut digest) = if unit.is_leaf() {
        compose_leaf_page(unit, unit_research)
    } else {
        compose_parent_page(unit, unit_research, child_digests)
    };
    digest.digest_id = crate::domain::stable_id::stable_id("digest", &unit.id);
    digest.section_digests = build_section_digests_from_draft(&draft);
    digest.diagram_digests = build_diagram_digests_from_draft(&draft);
    digest.readiness_stage = "compose_ready".to_string();
    let stage = stage_for_compose_error(unit);
    Ok((draft, digest, stage))
}

fn build_section_digests_from_draft(draft: &PageDraft) -> Vec<PageSectionDigest> {
    draft
        .sections
        .iter()
        .map(|section| PageSectionDigest {
            digest_id: crate::domain::stable_id::stable_id(
                "section-digest",
                format!("{}:{}", draft.page_id, section.section_key),
            ),
            section_key: section.section_key.clone(),
            title: section.title.clone(),
            summary: section
                .content
                .lines()
                .take(4)
                .collect::<Vec<_>>()
                .join("\n"),
            key_sources: section
                .citations
                .iter()
                .map(|citation| citation.path.clone())
                .collect(),
            citations: section.citations.clone(),
        })
        .collect()
}

fn build_diagram_digests_from_draft(draft: &PageDraft) -> Vec<PageDiagramDigest> {
    draft
        .diagrams
        .iter()
        .map(|diagram| PageDiagramDigest {
            digest_id: crate::domain::stable_id::stable_id(
                "diagram-digest",
                format!("{}:{}", draft.page_id, diagram.diagram_id),
            ),
            diagram_type: diagram.diagram_type.clone(),
            title: diagram.title.clone(),
            summary: diagram.description.clone(),
        })
        .collect()
}

fn enrich_parent_research(
    unit: &KnowledgeUnit,
    research: &mut UnitResearch,
    system_research: &SystemResearch,
    domain_researches: &BTreeMap<String, DomainResearch>,
    child_digests: &[PageDigest],
) {
    match unit.unit_type {
        UnitType::Overview | UnitType::Architecture => {
            if research.summary.trim().is_empty() {
                research.summary = system_research.description.clone();
            } else if !system_research.description.trim().is_empty()
                && !research
                    .summary
                    .contains(system_research.description.as_str())
            {
                research.summary =
                    format!("{}\n\n{}", system_research.description, research.summary);
            }
        }
        UnitType::DomainIndex => {
            if let Some(domain_research) = domain_researches.get(&unit.domain_id) {
                if research.summary.trim().is_empty() {
                    research.summary = domain_research.domain_summary.clone();
                } else if !domain_research.domain_summary.trim().is_empty()
                    && !research
                        .summary
                        .contains(domain_research.domain_summary.as_str())
                {
                    research.summary =
                        format!("{}\n\n{}", domain_research.domain_summary, research.summary);
                }
            }
        }
        _ => {}
    }

    for digest in child_digests {
        for source in &digest.key_sources {
            if !research
                .key_sources
                .iter()
                .any(|existing| existing == source)
            {
                research.key_sources.push(source.clone());
            }
        }
    }
}

fn persist_compose_result(
    conn: &Connection,
    unit: &KnowledgeUnit,
    draft: &PageDraft,
    digest: &PageDigest,
) -> io::Result<()> {
    let draft_json = serde_json::to_string(draft)
        .map_err(|error| io::Error::other(format!("serialize page draft: {error}")))?;
    let digest_json = serde_json::to_string(digest)
        .map_err(|error| io::Error::other(format!("serialize page digest: {error}")))?;
    let draft_hash = fingerprint_bytes(draft_json.as_bytes());
    let digest_hash = fingerprint_bytes(digest_json.as_bytes());
    sqlite_store::write_page_draft(conn, &unit.id, &draft_json, Some(&draft_hash))?;
    sqlite_store::write_page_digest(conn, &unit.id, &digest_json, Some(&digest_hash))?;
    Ok(())
}

fn persist_compose_progress(
    conn: &Connection,
    runtime_summary: &mut PipelineRuntimeSummary,
    unit: &KnowledgeUnit,
    stage: PipelineStage,
) -> io::Result<()> {
    runtime_summary.composed_units += 1;
    runtime_summary.last_ready_stage = Some(stage.as_str().to_string());
    runtime_summary.last_interrupted_stage = None;
    runtime_summary.summary_reason = None;
    write_unit_gate(
        conn,
        unit,
        "ready",
        "done",
        "pending",
        Some(stage.as_str().to_string()),
        None,
        Vec::new(),
    )?;
    save_runtime_summary(conn, runtime_summary)
}

fn compute_system_input_hash(
    facts_input_hash: &str,
    ds: &ResearchDataSource<'_>,
    steering: &SteeringConfig,
) -> String {
    stable_hash(&(
        facts_input_hash,
        &ds.repo_context.tech_stack,
        ds.knowledge_tree.domain_count(),
        llm_research_cache_contract(steering),
    ))
}

fn compute_domain_input_hash(
    facts_input_hash: &str,
    domain: &crate::domain::knowledge::KnowledgeDomain,
    steering: &SteeringConfig,
) -> String {
    stable_hash(&(
        facts_input_hash,
        domain,
        llm_research_cache_contract(steering),
    ))
}

fn compute_unit_input_hash(
    facts_input_hash: &str,
    unit: &KnowledgeUnit,
    child_digests: &[PageDigest],
    steering: &SteeringConfig,
) -> String {
    stable_hash(&(
        facts_input_hash,
        unit,
        child_digests,
        llm_research_cache_contract(steering),
    ))
}

fn llm_research_cache_contract(steering: &SteeringConfig) -> (bool, &str, usize, usize, usize) {
    (
        steering.llm.enabled,
        steering.llm.model.as_str(),
        steering.llm.max_calls,
        steering.llm.max_research_calls,
        steering.llm.page_research_max_turns,
    )
}

fn stable_hash<T: Serialize>(value: &T) -> String {
    let serialized = serde_json::to_vec(value).unwrap_or_default();
    fingerprint_bytes(&serialized)
}

fn save_checkpoint_and_return(
    conn: &Connection,
    facts_input_hash: &str,
    stage: PipelineStage,
    interrupted_target_id: Option<String>,
    error: io::Error,
) -> io::Error {
    debug_trace::record_json(
        "workflow_checkpoint_saved",
        &json!({
            "facts_input_hash": facts_input_hash,
            "stage": stage.as_str(),
            "target_id": interrupted_target_id,
            "error": error.to_string(),
        }),
    );
    let checkpoint = PipelineCheckpoint::new(
        facts_input_hash.to_string(),
        stage,
        interrupted_target_id.clone(),
        Some(error.to_string()),
    );
    let _ = sqlite_store::write_pipeline_checkpoint(
        conn,
        &checkpoint.checkpoint_id,
        &checkpoint.facts_input_hash,
        checkpoint.interrupted_stage.as_str(),
        interrupted_target_id.as_deref(),
        checkpoint.error_message.as_deref(),
    );
    error
}

fn record_unit_research_stop(unit: &KnowledgeUnit, research: &UnitResearch) {
    let stop_reason = research
        .provider_stop_reason
        .as_ref()
        .map(|reason| reason.as_str())
        .unwrap_or("not_run");
    let stats = research
        .provider_session_stats
        .as_ref()
        .map(|stats| {
            json!({
                "turns_used": stats.turns_used,
                "tool_calls": stats.tool_calls,
                "delta_evidence_count": stats.delta_evidence_count,
                "delta_section_count": stats.delta_section_count,
                "delta_diagram_count": stats.delta_diagram_count,
                "child_digest_delta": stats.child_digest_delta,
                "elapsed_ms": stats.elapsed_ms,
                "cache_hit": stats.cache_hit,
                "tools_mode": stats.tools_mode,
                "retry_input_applied": stats.retry_input_applied,
            })
        })
        .unwrap_or_else(|| json!(null));
    debug_trace::record_json(
        "workflow_unit_research_stop",
        &json!({
            "unit_id": unit.id,
            "unit_type": unit.unit_type.as_str(),
            "title": unit.title,
            "stop_reason": stop_reason,
            "stats": stats,
        }),
    );
}

fn record_workflow_research_stop_summary(unit_researches: &BTreeMap<String, UnitResearch>) {
    let mut reasons = BTreeMap::<String, usize>::new();
    let mut turns_used = 0usize;
    let mut tool_calls = 0usize;
    let mut delta_evidence_count = 0usize;
    let mut delta_section_count = 0usize;
    let mut delta_diagram_count = 0usize;
    let mut child_digest_delta = 0usize;
    let mut elapsed_ms_total = 0u64;
    let mut cache_hit_pages = 0usize;
    let mut retry_input_applied_pages = 0usize;
    let mut tools_mode_breakdown = BTreeMap::<String, usize>::new();

    for research in unit_researches.values() {
        let reason = research
            .provider_stop_reason
            .as_ref()
            .map(|item| item.as_str())
            .unwrap_or("not_run")
            .to_string();
        *reasons.entry(reason).or_default() += 1;
        if let Some(stats) = research.provider_session_stats.as_ref() {
            turns_used += stats.turns_used;
            tool_calls += stats.tool_calls;
            delta_evidence_count += stats.delta_evidence_count;
            delta_section_count += stats.delta_section_count;
            delta_diagram_count += stats.delta_diagram_count;
            child_digest_delta += stats.child_digest_delta;
            elapsed_ms_total =
                elapsed_ms_total.saturating_add(stats.elapsed_ms.unwrap_or_default());
            if stats.cache_hit == Some(true) {
                cache_hit_pages += 1;
            }
            if stats.retry_input_applied == Some(true) {
                retry_input_applied_pages += 1;
            }
            if let Some(mode) = stats.tools_mode.as_ref() {
                *tools_mode_breakdown.entry(mode.clone()).or_default() += 1;
            }
        }
    }

    debug_trace::record_json(
        "workflow_research_stop_summary",
        &json!({
            "unit_count": unit_researches.len(),
            "budget_stopped_pages": reasons.get("turn_budget_exhausted").copied().unwrap_or_default()
                + reasons.get("call_budget_rejected").copied().unwrap_or_default(),
            "stalled_pages": reasons.get("no_meaningful_delta").copied().unwrap_or_default(),
            "invalid_output_pages": reasons.get("invalid_output").copied().unwrap_or_default(),
            "provider_failed_pages": reasons.get("provider_error").copied().unwrap_or_default(),
            "stop_reasons": reasons,
            "stats": {
                "turns_used": turns_used,
                "tool_calls": tool_calls,
                "delta_evidence_count": delta_evidence_count,
                "delta_section_count": delta_section_count,
                "delta_diagram_count": delta_diagram_count,
                "child_digest_delta": child_digest_delta,
                "elapsed_ms_total": elapsed_ms_total,
                "cache_hit_pages": cache_hit_pages,
                "retry_input_applied_pages": retry_input_applied_pages,
                "tools_mode_breakdown": tools_mode_breakdown,
            },
        }),
    );
}

fn stage_for_compose_error(unit: &KnowledgeUnit) -> PipelineStage {
    match unit.unit_type {
        UnitType::Overview | UnitType::Architecture => PipelineStage::ComposeSystem,
        UnitType::DomainIndex => PipelineStage::ComposeIndex,
        _ if unit.is_leaf() => PipelineStage::ComposeLeaf,
        _ => PipelineStage::ComposeParent,
    }
}

/// workflow assemble 完成后，补齐 runtime summary 与 assemble 状态。
pub fn finalize_pipeline_runtime(
    repo_root: &Path,
    workflow_action: &str,
    assembled_pages: usize,
) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    let mut summary = load_runtime_summary(&conn)?.unwrap_or_default();
    summary.workflow_action = workflow_action.to_string();
    summary.runtime_state = "completed".to_string();
    summary.assembled_pages = assembled_pages;
    summary.last_ready_stage = Some(PipelineStage::Assemble.as_str().to_string());
    summary.last_interrupted_stage = None;
    summary.summary_reason = None;
    save_runtime_summary(&conn, &summary)?;

    for mut gate in sqlite_store::read_unit_runtime_gates(&conn)? {
        gate.assemble_status = "done".to_string();
        gate.last_ready_stage = Some(PipelineStage::Assemble.as_str().to_string());
        gate.updated_at = current_runtime_timestamp();
        sqlite_store::write_unit_runtime_gate(&conn, &gate)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::rc::Rc;

    use tempfile::tempdir;

    use crate::debug_trace;
    use crate::domain::checkpoint::{PipelineRuntimeSummary, PipelineStage};
    use crate::domain::compose::PageDraft;
    use crate::domain::knowledge::{KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitType};
    use crate::domain::research::{
        DomainResearch, PageDigest, ResearchSessionStats, ResearchStopReason, SystemResearch,
        UnitResearch,
    };
    use crate::domain::steering::{load_steering_config, DebugConfig, SteeringConfig};
    use crate::generation::context::{
        build_module_contexts_with_graph, build_repo_context_with_graph,
    };
    use crate::generation::research_engine::{
        ResearchDataSource, ResearchProvider, StructuralResearchProvider,
    };
    use crate::repo::hierarchy::build_module_tree_with_graph;
    use crate::repo::scanner::scan_repo_with_boundary;
    use crate::repo::symbol_graph::{build_graph_summary, resolve_symbol_graph};
    use crate::repo::symbols::parse_symbols;
    use crate::storage::sqlite_store;

    use super::{
        collect_compose_input_digests, compute_unit_input_hash, load_runtime_summary,
        persist_compose_progress, prepare_resume_state, run_compose_pipeline, save_runtime_summary,
    };

    #[test]
    fn prepare_resume_state_keeps_incomplete_runtime_without_checkpoint() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let conn = sqlite_store::open_db(repo_root).unwrap();

        let domain = crate::domain::knowledge::KnowledgeDomain::new(
            crate::domain::knowledge::DomainType::CoreRuntime,
            "核心运行时",
        );
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心运行时/运行时.md",
        );
        sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
        sqlite_store::write_knowledge_units(&conn, &[unit.clone()]).unwrap();

        let draft = PageDraft {
            page_id: crate::domain::stable_id::stable_id("page", &unit.relative_path),
            unit_id: unit.id.clone(),
            title: unit.title.clone(),
            relative_path: unit.relative_path.clone(),
            sections: Vec::new(),
            diagrams: Vec::new(),
            citation_count: 0,
        };
        let digest = PageDigest {
            digest_id: crate::domain::stable_id::stable_id("digest", &unit.id),
            unit_id: unit.id.clone(),
            page_id: draft.page_id.clone(),
            title: unit.title.clone(),
            decomposition_profile: None,
            research_profile: None,
            summary: "半完成摘要".to_string(),
            key_topics: Vec::new(),
            key_sources: Vec::new(),
            citations: Vec::new(),
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        };
        sqlite_store::write_page_draft(
            &conn,
            &unit.id,
            &serde_json::to_string(&draft).unwrap(),
            None,
        )
        .unwrap();
        sqlite_store::write_page_digest(
            &conn,
            &unit.id,
            &serde_json::to_string(&digest).unwrap(),
            None,
        )
        .unwrap();
        save_runtime_summary(
            &conn,
            &PipelineRuntimeSummary {
                facts_input_hash: "facts-same".to_string(),
                workflow_action: "init".to_string(),
                runtime_state: "compose_pending".to_string(),
                researched_units: 1,
                compose_ready_units: 1,
                composed_units: 0,
                assembled_pages: 0,
                blocked_units: Vec::new(),
                last_ready_stage: Some("research_unit".to_string()),
                last_interrupted_stage: None,
                summary_reason: None,
                current_research_unit_id: None,
                current_research_unit_type: None,
                current_research_started_at: None,
                last_researched_unit_id: None,
                last_research_elapsed_ms: None,
            },
        )
        .unwrap();

        assert!(prepare_resume_state(&conn, "facts-same").unwrap());
        assert!(sqlite_store::read_page_draft(&conn, &unit.id)
            .unwrap()
            .is_some());
        assert!(sqlite_store::read_page_digest(&conn, &unit.id)
            .unwrap()
            .is_some());
    }

    #[test]
    fn persist_compose_progress_flushes_runtime_summary_immediately() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let conn = sqlite_store::open_db(repo_root).unwrap();
        let domain = crate::domain::knowledge::KnowledgeDomain::new(
            crate::domain::knowledge::DomainType::CoreRuntime,
            "核心运行时",
        );
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心运行时/运行时.md",
        );
        sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
        sqlite_store::write_knowledge_units(&conn, &[unit.clone()]).unwrap();

        save_runtime_summary(
            &conn,
            &PipelineRuntimeSummary {
                facts_input_hash: "facts-same".to_string(),
                workflow_action: "init".to_string(),
                runtime_state: "compose_pending".to_string(),
                researched_units: 1,
                compose_ready_units: 1,
                composed_units: 0,
                assembled_pages: 0,
                blocked_units: Vec::new(),
                last_ready_stage: Some("research_unit".to_string()),
                last_interrupted_stage: Some("compose_parent".to_string()),
                summary_reason: Some("stale compose error".to_string()),
                current_research_unit_id: None,
                current_research_unit_type: None,
                current_research_started_at: None,
                last_researched_unit_id: None,
                last_research_elapsed_ms: None,
            },
        )
        .unwrap();

        let mut runtime_summary = PipelineRuntimeSummary {
            facts_input_hash: "facts-same".to_string(),
            workflow_action: "init".to_string(),
            runtime_state: "compose_pending".to_string(),
            researched_units: 1,
            compose_ready_units: 1,
            composed_units: 0,
            assembled_pages: 0,
            blocked_units: Vec::new(),
            last_ready_stage: Some("research_unit".to_string()),
            last_interrupted_stage: Some("compose_parent".to_string()),
            summary_reason: Some("stale compose error".to_string()),
            current_research_unit_id: None,
            current_research_unit_type: None,
            current_research_started_at: None,
            last_researched_unit_id: None,
            last_research_elapsed_ms: None,
        };
        persist_compose_progress(
            &conn,
            &mut runtime_summary,
            &unit,
            PipelineStage::ComposeLeaf,
        )
        .unwrap();

        let persisted_summary = load_runtime_summary(&conn)
            .unwrap()
            .expect("runtime summary should be flushed after compose progress");
        assert_eq!(persisted_summary.composed_units, 1);
        assert_eq!(
            persisted_summary.last_ready_stage.as_deref(),
            Some("compose_leaf")
        );
        assert_eq!(persisted_summary.last_interrupted_stage, None);
        assert_eq!(persisted_summary.summary_reason, None);

        let gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].unit_id, unit.id);
        assert_eq!(gates[0].compose_status, "done");
        assert_eq!(gates[0].assemble_status, "pending");
    }

    #[test]
    fn compose_pipeline_produces_deterministic_output() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"page-render-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/lib")).unwrap();
        fs::write(
            repo_root.join("src/index.ts"),
            "export function handleCheckout() { return true; }\n",
        )
        .unwrap();
        fs::write(
            repo_root.join("src/lib/payments.ts"),
            concat!(
                "import { handleCheckout } from \"../index\";\n",
                "export function settlePayment() { return handleCheckout(); }\n",
            ),
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let provider = StructuralResearchProvider;
        let output1 = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
            &graph_summary,
            &steering,
            &provider,
        )
        .unwrap();
        let output2 = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
            &graph_summary,
            &steering,
            &provider,
        )
        .unwrap();

        assert_eq!(output1.page_drafts.len(), output2.page_drafts.len());
        for (left, right) in output1.page_drafts.iter().zip(output2.page_drafts.iter()) {
            assert_eq!(left.page_id, right.page_id);
            assert_eq!(left.title, right.title);
            assert_eq!(left.citation_count, right.citation_count);
        }
        assert!(output1.page_drafts.len() >= 2);
    }

    #[test]
    fn compose_pipeline_saves_checkpoint_and_resumes_from_research_cache() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(repo_root.join("package.json"), r#"{"name":"resume-demo"}"#).unwrap();
        fs::create_dir_all(repo_root.join("src/core")).unwrap();
        fs::write(repo_root.join("src/index.ts"), "export const root = 1;\n").unwrap();
        fs::write(
            repo_root.join("src/core/runtime.ts"),
            "export function runtime() { return root; }\n",
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let fail_after = Rc::new(RefCell::new(Some(0usize)));
        let fail_provider = FailingProvider {
            inner: StructuralResearchProvider,
            fail_after_unit_calls: fail_after.clone(),
            system_calls: Rc::new(RefCell::new(0)),
            domain_calls: Rc::new(RefCell::new(0)),
            unit_calls: Rc::new(RefCell::new(0)),
        };

        let first = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
            &graph_summary,
            &steering,
            &fail_provider,
        );
        assert!(first.is_err());

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let checkpoint = sqlite_store::read_pipeline_checkpoint(&conn).unwrap();
        assert!(checkpoint.is_some());

        let resumed_system_calls = Rc::new(RefCell::new(0usize));
        let resumed_domain_calls = Rc::new(RefCell::new(0usize));
        let resumed_unit_calls = Rc::new(RefCell::new(0usize));
        let resume_provider = FailingProvider {
            inner: StructuralResearchProvider,
            fail_after_unit_calls: Rc::new(RefCell::new(None)),
            system_calls: resumed_system_calls.clone(),
            domain_calls: resumed_domain_calls.clone(),
            unit_calls: resumed_unit_calls.clone(),
        };
        let resumed = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
            &graph_summary,
            &steering,
            &resume_provider,
        )
        .unwrap();

        assert!(!resumed.page_drafts.is_empty());
        assert_eq!(*resumed_system_calls.borrow(), 0);
        assert_eq!(*resumed_domain_calls.borrow(), 0);
        assert!(*resumed_unit_calls.borrow() >= 1);
        let runtime_summary = sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")
            .unwrap()
            .expect("runtime summary should exist after resume");
        assert!(runtime_summary.contains("\"runtime_state\":\"compose_complete\""));
        assert!(
            runtime_summary.contains(&format!("\"composed_units\":{}", resumed.page_drafts.len()))
        );
        assert!(sqlite_store::read_pipeline_checkpoint(&conn)
            .unwrap()
            .is_some());
    }

    #[test]
    fn compose_pipeline_drops_stale_checkpoint_when_facts_change() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"resume-reset-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/core")).unwrap();
        fs::write(repo_root.join("src/index.ts"), "export const root = 1;\n").unwrap();
        fs::write(
            repo_root.join("src/core/runtime.ts"),
            "export function runtime() { return root; }\n",
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_analysis =
            crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let fail_provider = FailingProvider {
            inner: StructuralResearchProvider,
            fail_after_unit_calls: Rc::new(RefCell::new(Some(0))),
            system_calls: Rc::new(RefCell::new(0)),
            domain_calls: Rc::new(RefCell::new(0)),
            unit_calls: Rc::new(RefCell::new(0)),
        };
        let first = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
            &graph_summary,
            &steering,
            &fail_provider,
        );
        assert!(first.is_err());

        let conn = sqlite_store::open_db(repo_root).unwrap();
        assert!(sqlite_store::read_pipeline_checkpoint(&conn)
            .unwrap()
            .is_some());

        fs::write(
            repo_root.join("src/core/runtime.ts"),
            concat!(
                "export function runtime() { return root; }\n",
                "export function runtimeV2() { return runtime(); }\n",
            ),
        )
        .unwrap();

        let rescan = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let resymbols = parse_symbols(repo_root, &rescan).unwrap();
        let reresolved = resolve_symbol_graph(repo_root, &rescan, &resymbols).unwrap();
        let reanalysis = crate::repo::symbol_graph::analyze_symbol_graph(&resymbols, &reresolved);
        let regraph_summary = build_graph_summary(&rescan, &resymbols, &reresolved, &reanalysis);
        let remodule_tree = build_module_tree_with_graph(&rescan, &regraph_summary);
        let rerepo_context =
            build_repo_context_with_graph(&rescan, &remodule_tree, &regraph_summary);
        let remodule_contexts =
            build_module_contexts_with_graph(&rescan, &remodule_tree, &regraph_summary);

        let resumed_system_calls = Rc::new(RefCell::new(0usize));
        let resumed_domain_calls = Rc::new(RefCell::new(0usize));
        let resumed_unit_calls = Rc::new(RefCell::new(0usize));
        let resume_provider = FailingProvider {
            inner: StructuralResearchProvider,
            fail_after_unit_calls: Rc::new(RefCell::new(None)),
            system_calls: resumed_system_calls.clone(),
            domain_calls: resumed_domain_calls.clone(),
            unit_calls: resumed_unit_calls.clone(),
        };

        let resumed = run_compose_pipeline(
            repo_root,
            &rescan,
            &remodule_tree,
            &rerepo_context,
            &remodule_contexts,
            &resymbols,
            &reresolved,
            &reanalysis,
            &regraph_summary,
            &steering,
            &resume_provider,
        )
        .unwrap();

        assert!(!resumed.page_drafts.is_empty());
        assert!(*resumed_system_calls.borrow() >= 1);
        assert!(*resumed_domain_calls.borrow() >= 1);
        assert!(*resumed_unit_calls.borrow() >= 1);
        assert!(sqlite_store::read_pipeline_checkpoint(&conn)
            .unwrap()
            .is_none());
    }

    #[test]
    fn compose_pipeline_marks_summary_as_researching_before_system_research() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"researching-summary-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src")).unwrap();
        fs::write(
            repo_root.join("src/index.ts"),
            "export const runtime = () => true;\n",
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_analysis =
            crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let seen_runtime_state = Rc::new(RefCell::new(None));
        let provider = SummaryInspectingProvider {
            inner: StructuralResearchProvider,
            seen_runtime_state: seen_runtime_state.clone(),
        };

        run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
            &graph_summary,
            &steering,
            &provider,
        )
        .unwrap();

        let snapshot = seen_runtime_state
            .borrow()
            .clone()
            .expect("system research should observe runtime summary");
        assert!(snapshot.contains("\"runtime_state\":\"researching\""));
        assert!(snapshot.contains("\"last_ready_stage\":\"knowledge_planning\""));
    }

    #[test]
    fn compose_pipeline_marks_system_research_failure_as_interrupted() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"system-failure-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src")).unwrap();
        fs::write(repo_root.join("src/index.ts"), "export const root = 1;\n").unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_analysis =
            crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let provider = StageFailProvider {
            inner: StructuralResearchProvider,
            fail_stage: PipelineStage::ResearchSystem,
        };

        let result = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
            &graph_summary,
            &steering,
            &provider,
        );
        assert!(result.is_err());

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let runtime_summary = sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")
            .unwrap()
            .expect("runtime summary should exist after system failure");
        assert!(runtime_summary.contains("\"runtime_state\":\"interrupted\""));
        assert!(runtime_summary.contains("\"last_interrupted_stage\":\"research_system\""));
    }

    #[test]
    fn compose_pipeline_marks_domain_research_failure_as_interrupted() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"domain-failure-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/lib")).unwrap();
        fs::write(repo_root.join("src/index.ts"), "export const root = 1;\n").unwrap();
        fs::write(
            repo_root.join("src/lib/runtime.ts"),
            "export function runtime() { return root; }\n",
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_analysis =
            crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let provider = StageFailProvider {
            inner: StructuralResearchProvider,
            fail_stage: PipelineStage::ResearchDomain,
        };

        let result = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
            &graph_summary,
            &steering,
            &provider,
        );
        assert!(result.is_err());

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let runtime_summary = sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")
            .unwrap()
            .expect("runtime summary should exist after domain failure");
        assert!(runtime_summary.contains("\"runtime_state\":\"interrupted\""));
        assert!(runtime_summary.contains("\"last_interrupted_stage\":\"research_domain\""));
    }

    #[test]
    fn compose_pipeline_invokes_research_in_system_domain_unit_order() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"ordering-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/addons")).unwrap();
        fs::create_dir_all(repo_root.join("docs")).unwrap();
        fs::write(repo_root.join("docs/guide.md"), "# Guide\n").unwrap();
        fs::write(repo_root.join("src/index.ts"), "export const core = 1;\n").unwrap();
        fs::write(
            repo_root.join("src/addons/panel.ts"),
            "export function panel() { return core; }\n",
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

        let call_log = Rc::new(RefCell::new(Vec::<String>::new()));
        let provider = RecordingProvider {
            inner: StructuralResearchProvider,
            call_log: call_log.clone(),
        };

        let output = run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph),
            &graph_summary,
            &steering,
            &provider,
        )
        .unwrap();

        assert!(output.knowledge_tree.domain_count() >= 1);
        let log = call_log.borrow();
        assert!(!log.is_empty());
        assert_eq!(log.first().map(String::as_str), Some("system"));
        let first_unit_index = log
            .iter()
            .position(|entry| entry.starts_with("unit:"))
            .unwrap();
        let last_domain_index = log
            .iter()
            .rposition(|entry| entry.starts_with("domain:"))
            .unwrap();
        assert!(last_domain_index < first_unit_index);
    }

    #[test]
    fn collect_compose_input_digests_uses_domain_indexes_for_system_pages() {
        let overview = KnowledgeUnit::new(
            UnitType::Overview,
            "项目概述",
            "domain-system",
            "项目概述.md",
        );
        let architecture = KnowledgeUnit::new(
            UnitType::Architecture,
            "系统架构",
            "domain-system",
            "系统架构.md",
        );
        let domain_index = KnowledgeUnit::new(
            UnitType::DomainIndex,
            "核心模块",
            "domain-runtime",
            "核心模块/核心模块.md",
        );
        let leaf = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let mut tree = KnowledgeTree::new(overview.id.clone());

        tree.add_unit(overview.clone());
        tree.add_unit(architecture.clone());
        tree.add_unit(domain_index.clone());
        tree.add_unit(leaf.clone());

        let mut digests = BTreeMap::new();
        digests.insert(
            domain_index.id.clone(),
            PageDigest {
                digest_id: "digest-domain-runtime".to_string(),
                unit_id: domain_index.id.clone(),
                page_id: "page-domain".to_string(),
                title: "核心模块".to_string(),
                decomposition_profile: None,
                research_profile: None,
                summary: "域摘要".to_string(),
                key_topics: vec!["运行时".to_string()],
                key_sources: vec!["src/runtime.ts".to_string()],
                citations: Vec::new(),
                section_digests: Vec::new(),
                diagram_digests: Vec::new(),
                readiness_stage: "compose_ready".to_string(),
            },
        );

        let overview_digests = collect_compose_input_digests(&overview, &tree, &digests);
        let architecture_digests = collect_compose_input_digests(&architecture, &tree, &digests);
        let leaf_digests = collect_compose_input_digests(&leaf, &tree, &digests);

        assert_eq!(overview_digests.len(), 1);
        assert_eq!(overview_digests[0].title, "核心模块");
        assert_eq!(architecture_digests.len(), 1);
        assert!(leaf_digests.is_empty());
    }

    #[test]
    fn unit_input_hash_changes_when_llm_budget_contract_changes() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let child_digests = vec![PageDigest {
            digest_id: "digest-unit-child".to_string(),
            unit_id: "unit-child".to_string(),
            page_id: "page-child".to_string(),
            title: "子页".to_string(),
            decomposition_profile: None,
            research_profile: None,
            summary: "子页摘要".to_string(),
            key_topics: vec!["调度".to_string()],
            key_sources: vec!["src/runtime.ts".to_string()],
            citations: Vec::new(),
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let baseline = SteeringConfig::default();
        let mut raised_budget = SteeringConfig::default();
        raised_budget.llm.max_research_calls = baseline.llm.max_research_calls + 64;

        let baseline_hash = compute_unit_input_hash("facts-hash", &unit, &child_digests, &baseline);
        let raised_hash =
            compute_unit_input_hash("facts-hash", &unit, &child_digests, &raised_budget);

        assert_ne!(baseline_hash, raised_hash);
    }

    struct FailingProvider {
        inner: StructuralResearchProvider,
        fail_after_unit_calls: Rc<RefCell<Option<usize>>>,
        system_calls: Rc<RefCell<usize>>,
        domain_calls: Rc<RefCell<usize>>,
        unit_calls: Rc<RefCell<usize>>,
    }

    struct RecordingProvider {
        inner: StructuralResearchProvider,
        call_log: Rc<RefCell<Vec<String>>>,
    }

    struct StopReasonProvider {
        inner: StructuralResearchProvider,
    }

    struct SummaryInspectingProvider {
        inner: StructuralResearchProvider,
        seen_runtime_state: Rc<RefCell<Option<String>>>,
    }

    struct StageFailProvider {
        inner: StructuralResearchProvider,
        fail_stage: PipelineStage,
    }

    impl ResearchProvider for FailingProvider {
        fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
            *self.system_calls.borrow_mut() += 1;
            self.inner.research_system(ds)
        }

        fn research_domain(
            &self,
            domain: &KnowledgeDomain,
            ds: &ResearchDataSource,
        ) -> io::Result<DomainResearch> {
            *self.domain_calls.borrow_mut() += 1;
            self.inner.research_domain(domain, ds)
        }

        fn research_unit(
            &self,
            unit: &KnowledgeUnit,
            ds: &ResearchDataSource,
            child_digests: &[PageDigest],
        ) -> io::Result<UnitResearch> {
            let mut calls = self.unit_calls.borrow_mut();
            *calls += 1;
            if let Some(limit) = *self.fail_after_unit_calls.borrow() {
                if *calls > limit {
                    return Err(io::Error::other(format!("forced failure for {}", unit.id)));
                }
            }
            self.inner.research_unit(unit, ds, child_digests)
        }
    }

    impl ResearchProvider for RecordingProvider {
        fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
            assert!(ds.report.files.len() >= 2);
            assert!(ds.knowledge_tree.domain_count() >= 1);
            self.call_log.borrow_mut().push("system".to_string());
            self.inner.research_system(ds)
        }

        fn research_domain(
            &self,
            domain: &KnowledgeDomain,
            ds: &ResearchDataSource,
        ) -> io::Result<DomainResearch> {
            assert!(ds.knowledge_tree.get_domain(&domain.id).is_some());
            self.call_log
                .borrow_mut()
                .push(format!("domain:{}", domain.id));
            self.inner.research_domain(domain, ds)
        }

        fn research_unit(
            &self,
            unit: &KnowledgeUnit,
            ds: &ResearchDataSource,
            child_digests: &[PageDigest],
        ) -> io::Result<UnitResearch> {
            assert!(ds.knowledge_tree.get_unit(&unit.id).is_some());
            if unit.parent_unit_id.is_some() {
                assert!(child_digests.len() <= unit.child_unit_ids.len());
            }
            self.call_log.borrow_mut().push(format!("unit:{}", unit.id));
            self.inner.research_unit(unit, ds, child_digests)
        }
    }

    impl ResearchProvider for StopReasonProvider {
        fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
            self.inner.research_system(ds)
        }

        fn research_domain(
            &self,
            domain: &KnowledgeDomain,
            ds: &ResearchDataSource,
        ) -> io::Result<DomainResearch> {
            self.inner.research_domain(domain, ds)
        }

        fn research_unit(
            &self,
            unit: &KnowledgeUnit,
            ds: &ResearchDataSource,
            child_digests: &[PageDigest],
        ) -> io::Result<UnitResearch> {
            let mut research = self.inner.research_unit(unit, ds, child_digests)?;
            research.provider_stop_reason = Some(ResearchStopReason::NoMeaningfulDelta);
            research.provider_session_stats = Some(ResearchSessionStats {
                turns_used: 2,
                tool_calls: 1,
                delta_evidence_count: 1,
                delta_section_count: 2,
                delta_diagram_count: 0,
                child_digest_delta: child_digests.len(),
                ..ResearchSessionStats::default()
            });
            Ok(research)
        }
    }

    impl ResearchProvider for SummaryInspectingProvider {
        fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
            let conn = sqlite_store::open_db(Path::new(&ds.report.root))?;
            *self.seen_runtime_state.borrow_mut() =
                sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")?;
            self.inner.research_system(ds)
        }

        fn research_domain(
            &self,
            domain: &KnowledgeDomain,
            ds: &ResearchDataSource,
        ) -> io::Result<DomainResearch> {
            self.inner.research_domain(domain, ds)
        }

        fn research_unit(
            &self,
            unit: &KnowledgeUnit,
            ds: &ResearchDataSource,
            child_digests: &[PageDigest],
        ) -> io::Result<UnitResearch> {
            self.inner.research_unit(unit, ds, child_digests)
        }
    }

    impl ResearchProvider for StageFailProvider {
        fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
            if self.fail_stage == PipelineStage::ResearchSystem {
                return Err(io::Error::other("forced system research failure"));
            }
            self.inner.research_system(ds)
        }

        fn research_domain(
            &self,
            domain: &KnowledgeDomain,
            ds: &ResearchDataSource,
        ) -> io::Result<DomainResearch> {
            if self.fail_stage == PipelineStage::ResearchDomain {
                return Err(io::Error::other("forced domain research failure"));
            }
            self.inner.research_domain(domain, ds)
        }

        fn research_unit(
            &self,
            unit: &KnowledgeUnit,
            ds: &ResearchDataSource,
            child_digests: &[PageDigest],
        ) -> io::Result<UnitResearch> {
            self.inner.research_unit(unit, ds, child_digests)
        }
    }

    #[test]
    fn research_stop_trace_records_extended_provider_stats() {
        let trace_root = tempdir().unwrap();
        debug_trace::clear_startup_options();
        let trace_path = debug_trace::begin_session(
            "compose-pipeline-test",
            trace_root.path(),
            &DebugConfig {
                enabled: true,
                ..DebugConfig::default()
            },
        )
        .unwrap()
        .expect("trace should be enabled");

        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "Payments",
            "runtime-domain",
            "核心模块/payments.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            provider_stop_reason: Some(ResearchStopReason::NoMeaningfulDelta),
            provider_session_stats: Some(ResearchSessionStats {
                turns_used: 2,
                tool_calls: 1,
                delta_evidence_count: 1,
                delta_section_count: 2,
                delta_diagram_count: 1,
                child_digest_delta: 3,
                elapsed_ms: Some(4_800),
                cache_hit: Some(true),
                tools_mode: Some("required".to_string()),
                retry_input_applied: Some(true),
            }),
            ..UnitResearch::default()
        };
        let baseline = UnitResearch {
            unit_id: "unit-baseline".to_string(),
            provider_stop_reason: Some(ResearchStopReason::Completed),
            provider_session_stats: Some(ResearchSessionStats {
                turns_used: 1,
                ..ResearchSessionStats::default()
            }),
            ..UnitResearch::default()
        };

        super::record_unit_research_stop(&unit, &research);
        let mut unit_researches = BTreeMap::new();
        unit_researches.insert(unit.id.clone(), research);
        unit_researches.insert("unit-baseline".to_string(), baseline);
        super::record_workflow_research_stop_summary(&unit_researches);

        let trace_text = fs::read_to_string(&trace_path).unwrap();
        let entries = trace_text
            .lines()
            .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
            .collect::<Vec<_>>();
        let unit_payload = entries
            .iter()
            .find(|entry| {
                entry.get("kind").and_then(serde_json::Value::as_str)
                    == Some("workflow_unit_research_stop")
            })
            .and_then(|entry| entry.get("payload"))
            .cloned()
            .expect("unit trace payload should exist");
        let summary_payload = entries
            .iter()
            .find(|entry| {
                entry.get("kind").and_then(serde_json::Value::as_str)
                    == Some("workflow_research_stop_summary")
            })
            .and_then(|entry| entry.get("payload"))
            .cloned()
            .expect("summary trace payload should exist");

        assert_eq!(
            unit_payload["stats"]["elapsed_ms"],
            serde_json::json!(4_800)
        );
        assert_eq!(unit_payload["stats"]["cache_hit"], serde_json::json!(true));
        assert_eq!(
            unit_payload["stats"]["tools_mode"],
            serde_json::json!("required")
        );
        assert_eq!(
            unit_payload["stats"]["retry_input_applied"],
            serde_json::json!(true)
        );
        assert_eq!(
            summary_payload["stats"]["elapsed_ms_total"],
            serde_json::json!(4_800)
        );
        assert_eq!(
            summary_payload["stats"]["cache_hit_pages"],
            serde_json::json!(1)
        );
        assert_eq!(
            summary_payload["stats"]["retry_input_applied_pages"],
            serde_json::json!(1)
        );
        assert_eq!(
            summary_payload["stats"]["tools_mode_breakdown"]["required"],
            serde_json::json!(1)
        );

        debug_trace::clear_startup_options();
    }
    #[test]
    fn compose_pipeline_persists_unit_research_stop_reason_in_cache() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"page-render-stop-reason-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/lib")).unwrap();
        fs::write(
            repo_root.join("src/index.ts"),
            "export function handleCheckout() { return true; }\n",
        )
        .unwrap();
        fs::write(
            repo_root.join("src/lib/payments.ts"),
            concat!(
                "import { handleCheckout } from \"../index\";\n",
                "export function settlePayment() { return handleCheckout(); }\n",
            ),
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let graph_analysis =
            crate::repo::symbol_graph::analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary = build_graph_summary(
            &scan_report,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
        );
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);
        let provider = StopReasonProvider {
            inner: StructuralResearchProvider,
        };

        run_compose_pipeline(
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &symbol_snapshot,
            &resolved_graph,
            &graph_analysis,
            &graph_summary,
            &steering,
            &provider,
        )
        .unwrap();

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let mut statement = conn
            .prepare("SELECT result FROM research_cache WHERE research_type = 'unit'")
            .unwrap();
        let stored = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|row| row.unwrap())
            .map(|raw| serde_json::from_str::<UnitResearch>(&raw).unwrap())
            .collect::<Vec<_>>();

        assert!(!stored.is_empty());
        assert!(stored.iter().all(|research| {
            research.provider_stop_reason == Some(ResearchStopReason::NoMeaningfulDelta)
        }));
        assert!(stored.iter().all(|research| {
            research
                .provider_session_stats
                .as_ref()
                .map(|stats| stats.turns_used == 2 && stats.tool_calls == 1)
                .unwrap_or(false)
        }));
    }
}
