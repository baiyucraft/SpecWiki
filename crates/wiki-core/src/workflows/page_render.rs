//! 页面预渲染辅助层：
//! `run_compose_pipeline()` 封装 knowledge planning → research → compose，
//! 并负责 pipeline checkpoint / cache 驱动的中断恢复。

use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use crate::debug_trace;
use crate::domain::checkpoint::{compute_facts_input_hash, PipelineCheckpoint, PipelineStage};
use crate::domain::compose::PageDraft;
use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use crate::domain::module_tree::ModuleTree;
use crate::domain::research::{DomainResearch, PageDigest, SystemResearch, UnitResearch};
use crate::domain::steering::SteeringConfig;
use crate::generation::compose_engine::{
    compose_index_page, compose_leaf_page, compose_parent_page, compose_system_page,
};
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
    sqlite_store::write_knowledge_domains(&conn, &domains)?;
    sqlite_store::write_knowledge_units(&conn, &units)?;

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
    )?;

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
        )?;
        domain_researches.insert(domain.id.clone(), research);
    }

    let mut unit_researches = BTreeMap::new();
    let mut unit_digests_for_research = BTreeMap::new();
    for unit_id in &knowledge_tree.processing_order {
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        if matches!(
            unit.unit_type,
            UnitType::Overview | UnitType::Architecture | UnitType::DomainIndex
        ) {
            continue;
        }

        let child_digests: Vec<PageDigest> = unit
            .child_unit_ids
            .iter()
            .filter_map(|child_id| unit_digests_for_research.get(child_id).cloned())
            .collect();
        let unit_input_hash =
            compute_unit_input_hash(&facts_input_hash, unit, &child_digests, steering);
        let research = load_or_compute_research(
            &conn,
            "unit",
            &unit.id,
            &facts_input_hash,
            &unit_input_hash,
            resume_enabled,
            PipelineStage::ResearchUnit,
            Some(unit.id.clone()),
            || research_provider.research_unit(unit, &research_ds, &child_digests),
        )?;
        record_unit_research_stop(unit, &research);

        let digest = build_research_digest(unit, &research);
        unit_digests_for_research.insert(unit.id.clone(), digest);
        unit_researches.insert(unit.id.clone(), research);
    }

    record_workflow_research_stop_summary(&unit_researches);

    let mut page_drafts = Vec::new();
    let mut digests = BTreeMap::new();
    for unit_id in &knowledge_tree.processing_order {
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
                continue;
            }
        }

        let child_digests = collect_compose_input_digests(unit, &knowledge_tree, &digests);
        let (draft, digest, stage) = compose_unit_page(
            unit,
            &child_digests,
            &system_research,
            &domain_researches,
            &unit_researches,
        )
        .map_err(|error| {
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

        let _ = stage;
    }

    let planned_pages = plan_pages_from_knowledge_tree(&knowledge_tree);
    Ok(ComposePipelineOutput {
        page_drafts,
        digests,
        knowledge_tree,
        planned_pages,
    })
}

fn prepare_resume_state(conn: &Connection, facts_input_hash: &str) -> io::Result<bool> {
    let checkpoint = sqlite_store::read_pipeline_checkpoint(conn)?;
    let Some((_checkpoint_id, checkpoint_hash, _stage, _target, _message)) = checkpoint else {
        sqlite_store::clear_page_drafts(conn)?;
        sqlite_store::clear_page_digests(conn)?;
        return Ok(false);
    };

    if checkpoint_hash != facts_input_hash {
        sqlite_store::clear_pipeline_checkpoint(conn)?;
        sqlite_store::clear_page_drafts(conn)?;
        sqlite_store::clear_page_digests(conn)?;
        return Ok(false);
    }

    Ok(true)
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
    }
}

fn collect_compose_input_digests(
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
    system_research: &SystemResearch,
    domain_researches: &BTreeMap<String, DomainResearch>,
    unit_researches: &BTreeMap<String, UnitResearch>,
) -> io::Result<(PageDraft, PageDigest, PipelineStage)> {
    match unit.unit_type {
        UnitType::Overview | UnitType::Architecture => {
            let draft = compose_system_page(unit, system_research, child_digests);
            let digest = PageDigest {
                unit_id: unit.id.clone(),
                page_id: draft.page_id.clone(),
                title: unit.title.clone(),
                decomposition_profile: unit.decomposition_profile.clone(),
                research_profile: None,
                summary: system_research.description.clone(),
                key_topics: system_research.key_domains.clone(),
                key_sources: child_digests
                    .iter()
                    .flat_map(|digest| digest.key_sources.iter().cloned())
                    .fold(Vec::new(), |mut acc, source| {
                        if !acc.iter().any(|existing: &String| existing == &source) && acc.len() < 5
                        {
                            acc.push(source);
                        }
                        acc
                    }),
                citations: child_digests
                    .iter()
                    .flat_map(|digest| digest.citations.iter().cloned())
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
            };
            Ok((draft, digest, PipelineStage::ComposeSystem))
        }
        UnitType::DomainIndex => {
            let Some(domain_research) = domain_researches.get(&unit.domain_id) else {
                return Err(io::Error::other(format!(
                    "missing domain research for {}",
                    unit.domain_id
                )));
            };
            let draft = compose_index_page(unit, domain_research, child_digests);
            let digest = PageDigest {
                unit_id: unit.id.clone(),
                page_id: draft.page_id.clone(),
                title: unit.title.clone(),
                decomposition_profile: unit.decomposition_profile.clone(),
                research_profile: None,
                summary: domain_research.domain_summary.clone(),
                key_topics: child_digests
                    .iter()
                    .map(|digest| digest.title.clone())
                    .collect(),
                key_sources: child_digests
                    .iter()
                    .flat_map(|digest| digest.key_sources.iter().cloned())
                    .fold(Vec::new(), |mut acc, source| {
                        if !acc.iter().any(|existing: &String| existing == &source) && acc.len() < 5
                        {
                            acc.push(source);
                        }
                        acc
                    }),
                citations: child_digests
                    .iter()
                    .flat_map(|digest| digest.citations.iter().cloned())
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
            };
            Ok((draft, digest, PipelineStage::ComposeIndex))
        }
        _ => {
            let Some(unit_research) = unit_researches.get(&unit.id) else {
                return Err(io::Error::other(format!(
                    "missing unit research for {}",
                    unit.id
                )));
            };
            let (draft, digest) = if unit.is_leaf() {
                compose_leaf_page(unit, unit_research)
            } else {
                compose_parent_page(unit, unit_research, child_digests)
            };
            let stage = if unit.is_leaf() {
                PipelineStage::ComposeLeaf
            } else {
                PipelineStage::ComposeParent
            };
            Ok((draft, digest, stage))
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::io;
    use std::rc::Rc;

    use tempfile::tempdir;

    use crate::domain::knowledge::{KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitType};
    use crate::domain::research::{
        DomainResearch, PageDigest, ResearchSessionStats, ResearchStopReason, SystemResearch,
        UnitResearch,
    };
    use crate::domain::steering::{load_steering_config, SteeringConfig};
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

    use super::{collect_compose_input_digests, compute_unit_input_hash, run_compose_pipeline};

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
                unit_id: domain_index.id.clone(),
                page_id: "page-domain".to_string(),
                title: "核心模块".to_string(),
                decomposition_profile: None,
                research_profile: None,
                summary: "域摘要".to_string(),
                key_topics: vec!["运行时".to_string()],
                key_sources: vec!["src/runtime.ts".to_string()],
                citations: Vec::new(),
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
            unit_id: "unit-child".to_string(),
            page_id: "page-child".to_string(),
            title: "子页".to_string(),
            decomposition_profile: None,
            research_profile: None,
            summary: "子页摘要".to_string(),
            key_topics: vec!["调度".to_string()],
            key_sources: vec!["src/runtime.ts".to_string()],
            citations: Vec::new(),
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
            });
            Ok(research)
        }
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
