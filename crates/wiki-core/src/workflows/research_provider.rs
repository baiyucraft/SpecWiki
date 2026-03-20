//! workflow 级 research provider 选择层。
//! 统一管理 provider-backed research 与显式 fallback。

use crate::debug_trace;
use crate::domain::context::{PageContext, PageResearchResult};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use crate::domain::research::{
    DiagramEdgeSuggestion, DiagramNodeSuggestion, DiagramSuggestion, DomainResearch,
    EvidenceCluster, PageDigest, ResearchProfile, ResearchStopReason, SourceCitation,
    SystemResearch, UnitResearch,
};
use crate::domain::steering::SteeringConfig;
use crate::generation::context::build_page_context_with_graph_inputs;
use crate::generation::planner::PlannedPage;
use crate::generation::research_engine::{
    ResearchDataSource, ResearchProvider, StructuralResearchProvider,
};
use crate::llm::{
    LlmRuntime, PageResearchInput, PageResearchRuntimeContext, PageResearchSectionSlot,
    SelectedLlmPath,
};
use crate::repo::scanner::ScanReport;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::io;

use serde_json::json;

/// 当前 runtime 最终选择的 research provider 结果。
pub struct SelectedResearchProvider<'a> {
    pub provider: Box<dyn ResearchProvider + 'a>,
    pub mode: &'static str,
    pub summary: String,
    pub fallback_reason: Option<String>,
}

struct ProviderBackedResearchProvider<'rt, 'cfg, 'svc> {
    runtime: RefCell<&'rt mut LlmRuntime<'cfg, 'svc>>,
    structural: StructuralResearchProvider,
}

const RETRY_FACT_LIMIT: usize = 8;
const RETRY_HINT_LIMIT: usize = 6;
const RETRY_SUMMARY_LIMIT: usize = 6;
const RETRY_EVIDENCE_LIMIT: usize = 4;
const RETRY_DIAGRAM_LIMIT: usize = 3;
const FORCE_NO_TOOLS_SPARSE_FACT_LIMIT: usize = 4;
const FORCE_NO_TOOLS_SPARSE_SUMMARY_LIMIT: usize = 3;
const FORCE_NO_TOOLS_SPARSE_EVIDENCE_GROUP_LIMIT: usize = 1;
const FORCE_NO_TOOLS_SPARSE_EVIDENCE_ITEM_LIMIT: usize = 2;

impl ResearchProvider for ProviderBackedResearchProvider<'_, '_, '_> {
    fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
        self.structural.research_system(ds)
    }

    fn research_domain(
        &self,
        domain: &crate::domain::knowledge::KnowledgeDomain,
        ds: &ResearchDataSource,
    ) -> io::Result<DomainResearch> {
        self.structural.research_domain(domain, ds)
    }

    fn research_unit(
        &self,
        unit: &KnowledgeUnit,
        ds: &ResearchDataSource,
        child_digests: &[PageDigest],
    ) -> io::Result<UnitResearch> {
        let mut merged = self.structural.research_unit(unit, ds, child_digests)?;
        let page = build_provider_page(unit, ds.knowledge_tree);
        let child_summaries = child_digests
            .iter()
            .map(|digest| format!("- **{}**：{}", digest.title, digest.summary))
            .collect::<Vec<_>>();
        let hints = build_provider_hints(unit, merged.research_profile.as_ref());
        let page_context = build_page_context_with_graph_inputs(
            &page,
            ds.report,
            ds.module_tree,
            ds.repo_context,
            ds.module_contexts,
            Some(ds.symbol_snapshot),
            Some(ds.graph_analysis),
            hints,
            child_summaries,
        );
        let input = PageResearchInput::from_page(&page, &page_context).with_allowed_sections(
            merged
                .section_plan
                .iter()
                .map(|planned| PageResearchSectionSlot {
                    section_key: planned.section_key.clone(),
                    section_title: planned.title.clone(),
                })
                .collect(),
        );
        let input = apply_provider_tools_policy(input, merged.research_profile.as_ref());
        let (input, pretrim_applied) = canonicalize_provider_input(&input);
        if let Some(reason) = force_no_tools_short_circuit_reason(
            unit,
            merged.research_profile.as_ref(),
            child_digests,
            &input,
        ) {
            merged.provider_stop_reason = Some(ResearchStopReason::NotRun);
            debug_trace::record_json(
                "provider_research_stop",
                &json!({
                    "page_id": page.id,
                    "unit_id": unit.id,
                    "stop_reason": ResearchStopReason::NotRun.as_str(),
                    "force_no_tools_short_circuited_units": 1,
                    "force_no_tools_short_circuit_reason": reason,
                }),
            );
            return Ok(merged);
        }
        let runtime_context = PageResearchRuntimeContext {
            page: &page,
            page_context: &page_context,
            scan_report: ds.report,
            module_tree: ds.module_tree,
            repo_context: ds.repo_context,
            module_contexts: ds.module_contexts,
            symbol_snapshot: ds.symbol_snapshot,
            resolved_graph: ds.resolved_graph,
            graph_analysis: ds.graph_analysis,
            allowed_section_slots: &input.allowed_section_slots,
        };

        let first_attempt = {
            let mut runtime = self.runtime.borrow_mut();
            runtime.research_page(&input, &runtime_context)
        };
        let mut retry_input_applied = pretrim_applied;
        let mut session_result = match first_attempt {
            Ok(result) => result,
            Err(error) => {
                let Some(retry_input) = build_retry_input(&input) else {
                    mark_provider_error(&mut merged, &page.id, &unit.id, &error);
                    return Ok(merged);
                };
                retry_input_applied = true;
                debug_trace::record_json(
                    "provider_research_retry",
                    &json!({
                        "page_id": page.id,
                        "unit_id": unit.id,
                        "reason": error.to_string(),
                        "facts": {
                            "before": input.facts.len(),
                            "after": retry_input.facts.len(),
                        },
                        "hints": {
                            "before": input.hints.len(),
                            "after": retry_input.hints.len(),
                        },
                        "summary_inputs": {
                            "before": input.summary_inputs.len(),
                            "after": retry_input.summary_inputs.len(),
                        },
                        "evidence_groups": {
                            "before": input.evidence_groups.len(),
                            "after": retry_input.evidence_groups.len(),
                        },
                        "diagram_inputs": {
                            "before": input.diagram_inputs.len(),
                            "after": retry_input.diagram_inputs.len(),
                        },
                    }),
                );
                let retry_context = PageResearchRuntimeContext {
                    page: &page,
                    page_context: &page_context,
                    scan_report: ds.report,
                    module_tree: ds.module_tree,
                    repo_context: ds.repo_context,
                    module_contexts: ds.module_contexts,
                    symbol_snapshot: ds.symbol_snapshot,
                    resolved_graph: ds.resolved_graph,
                    graph_analysis: ds.graph_analysis,
                    allowed_section_slots: &retry_input.allowed_section_slots,
                };
                let retry_attempt = {
                    let mut runtime = self.runtime.borrow_mut();
                    runtime.research_page(&retry_input, &retry_context)
                };
                match retry_attempt {
                    Ok(result) => result,
                    Err(retry_error) => {
                        mark_provider_error(&mut merged, &page.id, &unit.id, &retry_error);
                        return Ok(merged);
                    }
                }
            }
        };
        session_result.stats.retry_input_applied = Some(retry_input_applied);
        merged.provider_stop_reason = Some(session_result.stop_reason.clone());
        merged.provider_session_stats = Some(session_result.stats.clone());
        debug_trace::record_json(
            "provider_research_stop",
            &json!({
                "page_id": page.id,
                "unit_id": unit.id,
                "stop_reason": session_result.stop_reason.as_str(),
                "has_output": session_result.output.is_some(),
                "stats": session_result.stats,
            }),
        );
        if let Some(output) = session_result.output {
            merge_provider_research(
                &mut merged,
                &output.result,
                &page_context,
                child_digests,
                ds.report,
            );
        }

        Ok(merged)
    }
}

fn mark_provider_error(
    research: &mut UnitResearch,
    page_id: &str,
    unit_id: &str,
    error: &io::Error,
) {
    research.provider_stop_reason = Some(ResearchStopReason::ProviderError);
    debug_trace::record_json(
        "provider_research_stop",
        &json!({
            "page_id": page_id,
            "unit_id": unit_id,
            "stop_reason": ResearchStopReason::ProviderError.as_str(),
            "error": error.to_string(),
        }),
    );
}

/// 基于稳定 `ResearchProfile` 决定本次 request 是否收窄为 `NoTools`。
fn apply_provider_tools_policy(
    input: PageResearchInput,
    profile: Option<&ResearchProfile>,
) -> PageResearchInput {
    if should_force_no_tools(profile) {
        return input.force_no_tools();
    }
    input
}

/// 对 provider 输入做 deterministic canonicalization，保证首请求就使用稳定缓存键。
fn canonicalize_provider_input(input: &PageResearchInput) -> (PageResearchInput, bool) {
    let mut canonical = input.clone();
    let mut changed = false;

    changed |= trim_vec(&mut canonical.facts, RETRY_FACT_LIMIT);
    changed |= trim_vec(&mut canonical.hints, RETRY_HINT_LIMIT);
    changed |= trim_vec(&mut canonical.summary_inputs, RETRY_SUMMARY_LIMIT);
    changed |= trim_vec(&mut canonical.evidence_groups, RETRY_EVIDENCE_LIMIT);
    changed |= trim_vec(&mut canonical.diagram_inputs, RETRY_DIAGRAM_LIMIT);
    let retry_input_applied = changed || canonical.retry_input_applied;
    canonical = canonical.with_retry_input_applied(retry_input_applied);

    (canonical, changed)
}

/// 把只读型 research profile 收窄到 `NoTools`，避免把无工具页错误地绑定到 provider learned state。
fn should_force_no_tools(profile: Option<&ResearchProfile>) -> bool {
    matches!(
        profile,
        Some(
            ResearchProfile::DocsGuide
                | ResearchProfile::ConfigSurface
                | ResearchProfile::ExampleTutorial
                | ResearchProfile::Troubleshooting
                | ResearchProfile::IntegrationPlatform
        )
    )
}

/// 对只读型且输入稀薄的 leaf unit，直接保留 structural baseline，
/// 避免再打一发几乎没有增量的 no-tools provider request。
fn force_no_tools_short_circuit_reason(
    unit: &KnowledgeUnit,
    profile: Option<&ResearchProfile>,
    child_digests: &[PageDigest],
    input: &PageResearchInput,
) -> Option<&'static str> {
    if !input.force_no_tools || !should_force_no_tools(profile) {
        return None;
    }
    if !child_digests.is_empty() || !unit.child_unit_ids.is_empty() {
        return None;
    }
    if matches!(
        unit.unit_type,
        UnitType::Overview | UnitType::Architecture | UnitType::DomainIndex
    ) {
        return None;
    }
    if !unit.scope.docs_anchors.is_empty() {
        return None;
    }

    let evidence_items = input
        .evidence_groups
        .iter()
        .map(|group| group.items.len())
        .sum::<usize>();
    let sparse_leaf = input.facts.len() <= FORCE_NO_TOOLS_SPARSE_FACT_LIMIT
        && input.summary_inputs.len() <= FORCE_NO_TOOLS_SPARSE_SUMMARY_LIMIT
        && input.evidence_groups.len() <= FORCE_NO_TOOLS_SPARSE_EVIDENCE_GROUP_LIMIT
        && evidence_items <= FORCE_NO_TOOLS_SPARSE_EVIDENCE_ITEM_LIMIT
        && input.diagram_inputs.is_empty();

    sparse_leaf.then_some("sparse_force_no_tools_leaf")
}

fn build_retry_input(input: &PageResearchInput) -> Option<PageResearchInput> {
    let (retry, changed) = canonicalize_provider_input(input);
    changed.then_some(retry)
}

fn trim_vec<T>(items: &mut Vec<T>, limit: usize) -> bool {
    if items.len() <= limit {
        return false;
    }
    items.truncate(limit);
    true
}

/// 统一选择 workflow 级 research provider。
pub fn select_runtime_research_provider<'a, 'cfg, 'svc>(
    steering: &SteeringConfig,
    llm_runtime: &'a mut LlmRuntime<'cfg, 'svc>,
) -> SelectedResearchProvider<'a> {
    if steering.llm.enabled && llm_runtime.selected_path() == Some(SelectedLlmPath::ProviderApi) {
        return SelectedResearchProvider {
            provider: Box::new(ProviderBackedResearchProvider {
                runtime: RefCell::new(llm_runtime),
                structural: StructuralResearchProvider,
            }),
            mode: "provider_backed",
            summary: "当前 workflow 使用 provider-backed unit research，并保留 structural research 作为稳定基线".to_string(),
            fallback_reason: None,
        };
    }

    if steering.llm.enabled && llm_runtime.service_available() {
        return SelectedResearchProvider {
            provider: Box::new(StructuralResearchProvider),
            mode: "agent_runtime_structural_fallback",
            summary: "LLM runtime 已启用，但当前不是 provider 直连路径，research 继续显式回退到 structural provider".to_string(),
            fallback_reason: Some(
                "research_page 当前仅支持 provider-backed runtime，agent bridge 仍未接入".to_string(),
            ),
        };
    }

    if steering.llm.enabled && !llm_runtime.service_available() {
        return SelectedResearchProvider {
            provider: Box::new(StructuralResearchProvider),
            mode: "provider_unavailable_structural_fallback",
            summary: "LLM research 已启用，但当前 runtime 不可用，显式回退到 structural provider"
                .to_string(),
            fallback_reason: Some(
                "llm runtime unavailable: provider/api bridge not ready for this workflow"
                    .to_string(),
            ),
        };
    }

    SelectedResearchProvider {
        provider: Box::new(StructuralResearchProvider),
        mode: "structural_only",
        summary: "当前 workflow 使用 structural research provider".to_string(),
        fallback_reason: None,
    }
}

fn build_provider_page(unit: &KnowledgeUnit, tree: &KnowledgeTree) -> PlannedPage {
    let page_type = match unit.unit_type {
        UnitType::Overview => "overview",
        UnitType::Architecture => "architecture",
        UnitType::ModuleDoc => "module",
        UnitType::DomainIndex => "family-index",
        _ if unit.child_unit_ids.is_empty() => "topic",
        _ => "family-child",
    };
    let parent_id = unit.parent_unit_id.as_ref().and_then(|parent_unit_id| {
        tree.get_unit(parent_unit_id)
            .map(|parent| crate::domain::stable_id::stable_id("page", &parent.relative_path))
    });

    PlannedPage {
        id: crate::domain::stable_id::stable_id("page", &unit.relative_path),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        page_type: page_type.to_string(),
        parent_id,
        scope: provider_page_scope(unit, page_type),
        unit_id: Some(unit.id.clone()),
        unit_type: Some(unit.unit_type.as_str().to_string()),
        domain_id: Some(unit.domain_id.clone()),
        source_ids: unit.scope.source_ids.clone(),
        module_ids: unit.scope.module_ids.clone(),
        relation_ids: unit.scope.relation_ids.clone(),
        generation_mode: "knowledge-tree:provider-research".to_string(),
        priority: (unit.priority * 100.0) as usize,
        merged_module_ids: Vec::new(),
    }
}

fn provider_page_scope(unit: &KnowledgeUnit, page_type: &str) -> String {
    match page_type {
        "overview" | "architecture" => "repository".to_string(),
        "module" => unit
            .scope
            .module_ids
            .first()
            .map(|module_id| format!("module:{module_id}"))
            .unwrap_or_else(|| "module".to_string()),
        "family-index" => format!("domain:{}", unit.domain_id),
        "family-child" => format!("unit-group:{}", unit.domain_id),
        _ => format!("unit:{}", unit.id),
    }
}

fn build_provider_hints(unit: &KnowledgeUnit, profile: Option<&ResearchProfile>) -> Vec<String> {
    let mut hints = vec![
        format!("knowledge_unit={}", unit.unit_type.as_str()),
        format!("relative_path={}", unit.relative_path),
        "decomposition_contract=facts_first_llm_assist".to_string(),
    ];
    if let Some(profile) = profile {
        hints.push(format!(
            "research_profile={}",
            research_profile_key(profile)
        ));
    }
    if let Some(profile) = unit.decomposition_profile.as_ref() {
        hints.push(format!(
            "decomposition_profile={}",
            format!("{profile:?}").to_lowercase()
        ));
    }
    hints
}

fn research_profile_key(profile: &ResearchProfile) -> &'static str {
    match profile {
        ResearchProfile::Runtime => "runtime",
        ResearchProfile::ApiSurface => "api_surface",
        ResearchProfile::ConfigSurface => "config_surface",
        ResearchProfile::DocsGuide => "docs_guide",
        ResearchProfile::Testing => "testing",
        ResearchProfile::ExampleTutorial => "example_tutorial",
        ResearchProfile::Troubleshooting => "troubleshooting",
        ResearchProfile::IntegrationPlatform => "integration_platform",
        ResearchProfile::CompilerPipeline => "compiler_pipeline",
    }
}

fn merge_provider_research(
    research: &mut UnitResearch,
    result: &PageResearchResult,
    context: &PageContext,
    child_digests: &[PageDigest],
    report: &ScanReport,
) {
    if !result.page_positioning.trim().is_empty() {
        research.positioning = result.page_positioning.trim().to_string();
    }
    if !result.summary.trim().is_empty() {
        research.summary = result.summary.trim().to_string();
    }

    let (provider_clusters, group_map, provider_sources) =
        build_provider_evidence_clusters(&result.evidence_rollup, report);
    let mut existing_cluster_keys = research
        .evidence_clusters
        .iter()
        .map(|cluster| cluster.cluster_key.clone())
        .collect::<BTreeSet<_>>();
    for cluster in provider_clusters {
        if existing_cluster_keys.insert(cluster.cluster_key.clone()) {
            research.evidence_clusters.push(cluster);
        }
    }
    for source in provider_sources {
        if !research
            .key_sources
            .iter()
            .any(|existing| existing == &source)
        {
            research.key_sources.push(source);
        }
    }

    let mut matched_provider_indexes = BTreeSet::new();
    for planned in &mut research.section_plan {
        let Some((provider_index, provider_section)) =
            select_provider_section(planned, &result.section_plan, &matched_provider_indexes)
        else {
            continue;
        };
        matched_provider_indexes.insert(provider_index);
        if !provider_section.section_summary.trim().is_empty() {
            planned.section_summary = provider_section.section_summary.trim().to_string();
        }
        for evidence_ref in &provider_section.evidence_refs {
            let Some(cluster_key) = group_map.get(evidence_ref) else {
                continue;
            };
            if !planned
                .evidence_cluster_keys
                .iter()
                .any(|existing| existing == cluster_key)
            {
                planned.evidence_cluster_keys.push(cluster_key.clone());
            }
        }
        if !provider_section.child_refs.is_empty() && !child_digests.is_empty() {
            planned.child_digest_slot = true;
        }
    }

    let mut existing_diagrams = research
        .diagram_suggestions
        .iter()
        .map(|diagram| diagram.title.clone())
        .collect::<BTreeSet<_>>();
    for diagram in build_provider_diagram_suggestions(&result.diagram_rollup, context) {
        if existing_diagrams.insert(diagram.title.clone()) {
            research.diagram_suggestions.push(diagram);
        }
    }
}

fn select_provider_section<'a>(
    planned: &crate::domain::research::PlannedSection,
    provider_sections: &'a [crate::domain::context::PageResearchSectionPlan],
    matched_indexes: &BTreeSet<usize>,
) -> Option<(usize, &'a crate::domain::context::PageResearchSectionPlan)> {
    let by_key = provider_sections
        .iter()
        .enumerate()
        .find(|(index, section)| {
            !matched_indexes.contains(index) && section.section_key == planned.section_key
        });
    if by_key.is_some() {
        return by_key;
    }

    let by_title = provider_sections
        .iter()
        .enumerate()
        .find(|(index, section)| {
            !matched_indexes.contains(index) && section.section_title == planned.title
        });
    if by_title.is_some() {
        return by_title;
    }

    provider_sections
        .iter()
        .enumerate()
        .find(|(index, _)| !matched_indexes.contains(index))
}

fn build_provider_evidence_clusters(
    evidence_rollup: &[crate::domain::context::PageResearchEvidenceGroup],
    report: &ScanReport,
) -> (Vec<EvidenceCluster>, BTreeMap<String, String>, Vec<String>) {
    let mut group_map = BTreeMap::new();
    let mut clusters = Vec::new();
    let mut key_sources = Vec::new();
    let known_paths = report
        .files
        .iter()
        .map(|file| file.path.replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    for group in evidence_rollup {
        let cluster_key = format!("provider:{}", group.group_key);
        let citations = group
            .items
            .iter()
            .filter_map(|item| {
                let normalized_path = item.path.replace('\\', "/");
                known_paths
                    .contains(&normalized_path)
                    .then(|| SourceCitation {
                        path: normalized_path,
                        start_line: item.start_line,
                        end_line: item.end_line,
                        source_id: item.source_id.clone(),
                        symbol_id: None,
                        note: item.note.clone(),
                    })
            })
            .collect::<Vec<_>>();
        if citations.is_empty() {
            continue;
        }
        group_map.insert(group.group_key.clone(), cluster_key.clone());
        for citation in &citations {
            if !key_sources
                .iter()
                .any(|existing| existing == &citation.path)
            {
                key_sources.push(citation.path.clone());
            }
        }
        clusters.push(EvidenceCluster {
            cluster_key,
            label: group.title.clone(),
            citations,
        });
    }
    (clusters, group_map, key_sources)
}

fn build_provider_diagram_suggestions(
    diagram_rollup: &[crate::domain::context::PageResearchDiagramRollup],
    context: &PageContext,
) -> Vec<DiagramSuggestion> {
    let diagram_index = context
        .diagram_inputs
        .iter()
        .map(|diagram| (diagram.diagram_id.as_str(), diagram))
        .collect::<BTreeMap<_, _>>();
    let mut diagrams = Vec::new();
    for rollup in diagram_rollup {
        let Some(input) = diagram_index.get(rollup.diagram_key.as_str()) else {
            continue;
        };
        diagrams.push(DiagramSuggestion {
            diagram_type: if rollup.diagram_type.trim().is_empty() {
                input.diagram_type.clone()
            } else {
                rollup.diagram_type.clone()
            },
            title: if rollup.title.trim().is_empty() {
                input.title.clone()
            } else {
                rollup.title.clone()
            },
            description: if rollup.summary.trim().is_empty() {
                input.summary.clone()
            } else {
                rollup.summary.clone()
            },
            nodes: input
                .nodes
                .iter()
                .map(|node| DiagramNodeSuggestion {
                    node_id: node.node_id.clone(),
                    label: node.label.clone(),
                })
                .collect(),
            edges: input
                .edges
                .iter()
                .map(|edge| DiagramEdgeSuggestion {
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    label: edge.label.clone(),
                })
                .collect(),
        });
    }
    diagrams
}

#[cfg(test)]
mod tests {
    use super::{
        build_provider_diagram_suggestions, build_retry_input, canonicalize_provider_input,
        force_no_tools_short_circuit_reason, mark_provider_error, merge_provider_research,
        select_runtime_research_provider, should_force_no_tools, ProviderBackedResearchProvider,
        RETRY_DIAGRAM_LIMIT, RETRY_EVIDENCE_LIMIT, RETRY_FACT_LIMIT, RETRY_HINT_LIMIT,
        RETRY_SUMMARY_LIMIT,
    };
    use crate::domain::context::{
        PageContext, PageDiagramEdge, PageDiagramInput, PageDiagramNode, PageEvidenceGroup,
        PageResearchDiagramRollup, PageResearchEvidenceGroup, PageResearchEvidenceItem,
        PageResearchResult, PageResearchSectionPlan, RepoContext,
    };
    use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitScope, UnitType};
    use crate::domain::module_tree::ModuleTree;
    use crate::domain::research::{
        PageDigest, PlannedSection, ResearchProfile, ResearchStopReason, UnitResearch,
    };
    use crate::domain::steering::{
        LlmProviderCapabilitiesConfig, LlmProviderConfig, LlmProviderModelConfig, LlmToolsMode,
    };
    use crate::generation::research_engine::{
        ResearchDataSource, ResearchProvider, StructuralResearchProvider,
    };
    use crate::llm::{
        LlmCompletion, LlmPromptRequest, LlmRuntime, LlmService, PageResearchInput,
        PageResearchSectionSlot,
    };
    use crate::repo::scanner::{ScanReport, ScannedFile};
    use crate::repo::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
    use crate::repo::symbols::ParsedSymbolsSnapshot;
    use std::cell::RefCell;
    use std::io;
    use std::path::Path;

    fn provider_enabled_steering() -> crate::domain::steering::SteeringConfig {
        let mut steering = crate::domain::steering::SteeringConfig::default();
        steering.llm.enabled = true;
        steering.llm.model = "dummy/test".to_string();
        let mut provider = LlmProviderConfig {
            api_base: "https://example.invalid/v1".to_string(),
            api_key: "test-key".to_string(),
            default_model: "test".to_string(),
            capabilities: LlmProviderCapabilitiesConfig {
                tools_mode: LlmToolsMode::NoTools,
                response_format: true,
            },
            ..LlmProviderConfig::default()
        };
        provider.models.insert(
            "test".to_string(),
            LlmProviderModelConfig {
                model_id: "dummy-test".to_string(),
            },
        );
        steering.llm.providers.insert("dummy".to_string(), provider);
        steering
    }

    #[derive(Default)]
    struct CountingLlmService {
        calls: usize,
    }

    impl LlmService for CountingLlmService {
        fn request(&mut self, _request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
            self.calls += 1;
            Ok(LlmCompletion {
                output: serde_json::json!({}),
                model: Some("counting".to_string()),
                usage: None,
            })
        }
    }

    #[test]
    fn provider_selection_prefers_provider_backed_runtime() {
        let steering = provider_enabled_steering();
        let mut runtime = LlmRuntime::new(Path::new("."), &steering.llm, None);

        let selected = select_runtime_research_provider(&steering, &mut runtime);

        assert_eq!(selected.mode, "provider_backed");
        assert!(selected.fallback_reason.is_none());
    }

    #[test]
    fn provider_selection_stays_structural_when_llm_disabled() {
        let steering = crate::domain::steering::SteeringConfig::default();
        let mut runtime = LlmRuntime::new(Path::new("."), &steering.llm, None);

        let selected = select_runtime_research_provider(&steering, &mut runtime);

        assert_eq!(selected.mode, "structural_only");
        assert!(selected.fallback_reason.is_none());
    }

    #[test]
    fn provider_research_merges_section_evidence_and_diagrams() {
        let mut research = UnitResearch {
            research_profile: Some(ResearchProfile::Runtime),
            summary: "structural summary".to_string(),
            positioning: "structural positioning".to_string(),
            section_plan: vec![
                PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "old intent".to_string(),
                    section_summary: "old overview".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "runtime-responsibility".to_string(),
                    title: "运行时职责".to_string(),
                    intent: "old runtime".to_string(),
                    section_summary: "old runtime summary".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
            ],
            ..UnitResearch::default()
        };
        let context = PageContext {
            diagram_inputs: vec![PageDiagramInput {
                diagram_id: "runtime-flow".to_string(),
                section_title: "运行时职责".to_string(),
                diagram_type: "flow".to_string(),
                title: "运行时流程".to_string(),
                summary: "流程说明".to_string(),
                nodes: vec![
                    PageDiagramNode {
                        node_id: "start".to_string(),
                        label: "Start".to_string(),
                    },
                    PageDiagramNode {
                        node_id: "end".to_string(),
                        label: "End".to_string(),
                    },
                ],
                edges: vec![PageDiagramEdge {
                    source: "start".to_string(),
                    target: "end".to_string(),
                    label: Some("next".to_string()),
                }],
            }],
            ..PageContext::default()
        };
        let result = PageResearchResult {
            summary: "provider summary".to_string(),
            page_positioning: "provider positioning".to_string(),
            section_plan: vec![
                PageResearchSectionPlan {
                    section_key: "overview".to_string(),
                    section_title: "概述".to_string(),
                    section_summary: "新的概述".to_string(),
                    evidence_refs: vec!["group-a".to_string()],
                    diagram_refs: Vec::new(),
                    child_refs: Vec::new(),
                },
                PageResearchSectionPlan {
                    section_key: "detail".to_string(),
                    section_title: "运行时职责".to_string(),
                    section_summary: "新的职责".to_string(),
                    evidence_refs: vec!["group-a".to_string()],
                    diagram_refs: vec!["runtime-flow".to_string()],
                    child_refs: vec!["child-1".to_string()],
                },
            ],
            evidence_rollup: vec![PageResearchEvidenceGroup {
                group_key: "group-a".to_string(),
                title: "关键入口".to_string(),
                items: vec![PageResearchEvidenceItem {
                    path: "src/runtime.rs".to_string(),
                    start_line: 10,
                    end_line: 30,
                    note: "入口".to_string(),
                    ..PageResearchEvidenceItem::default()
                }],
            }],
            diagram_rollup: vec![PageResearchDiagramRollup {
                diagram_key: "runtime-flow".to_string(),
                diagram_type: "flow".to_string(),
                title: "运行时流程".to_string(),
                summary: "流程说明".to_string(),
            }],
            open_questions: Vec::new(),
        };

        merge_provider_research(
            &mut research,
            &result,
            &context,
            &[PageDigest {
                digest_id: "digest-child-1".to_string(),
                unit_id: "child-1".to_string(),
                page_id: "page-child-1".to_string(),
                title: "子页".to_string(),
                decomposition_profile: None,
                research_profile: None,
                summary: "child".to_string(),
                key_topics: Vec::new(),
                key_sources: Vec::new(),
                citations: Vec::new(),
                section_digests: Vec::new(),
                diagram_digests: Vec::new(),
                readiness_stage: "compose_ready".to_string(),
            }],
            &crate::repo::scanner::ScanReport {
                root: ".".to_string(),
                files: vec![crate::repo::scanner::ScannedFile {
                    id: "source:src/runtime.rs".to_string(),
                    path: "src/runtime.rs".to_string(),
                    language: "rust".to_string(),
                    kind: "source".to_string(),
                    purpose: crate::repo::scanner::FilePurpose::Library,
                    fingerprint: "fp".to_string(),
                    size: 0,
                    tags: Vec::new(),
                }],
                tech_hints: Vec::new(),
                workspace_roots: Vec::new(),
                config_files: Vec::new(),
                entry_points: Vec::new(),
                dependency_hints: Vec::new(),
            },
        );

        assert_eq!(research.summary, "provider summary");
        assert_eq!(research.positioning, "provider positioning");
        assert_eq!(research.section_plan[0].intent, "old intent");
        assert_eq!(research.section_plan[0].section_summary, "新的概述");
        assert_eq!(research.section_plan[1].section_summary, "新的职责");
        assert_eq!(
            research.section_plan[0].evidence_cluster_keys,
            vec!["provider:group-a".to_string()]
        );
        assert!(research.section_plan[1].child_digest_slot);
        assert_eq!(research.diagram_suggestions.len(), 1);
        assert_eq!(research.key_sources, vec!["src/runtime.rs".to_string()]);
    }

    #[test]
    fn provider_research_drops_unknown_evidence_paths() {
        let mut research = UnitResearch::default();
        let context = PageContext::default();
        let result = PageResearchResult {
            summary: "provider summary".to_string(),
            page_positioning: String::new(),
            section_plan: vec![PageResearchSectionPlan {
                section_key: "overview".to_string(),
                section_title: "概述".to_string(),
                section_summary: "概述".to_string(),
                evidence_refs: vec!["group-a".to_string()],
                diagram_refs: Vec::new(),
                child_refs: Vec::new(),
            }],
            evidence_rollup: vec![PageResearchEvidenceGroup {
                group_key: "group-a".to_string(),
                title: "隐藏派生语料".to_string(),
                items: vec![PageResearchEvidenceItem {
                    path: ".qoder/repowiki/zh/content/快速开始.md".to_string(),
                    start_line: 1,
                    end_line: 24,
                    note: "provider hallucination".to_string(),
                    ..PageResearchEvidenceItem::default()
                }],
            }],
            diagram_rollup: Vec::new(),
            open_questions: Vec::new(),
        };
        let report = crate::repo::scanner::ScanReport {
            root: ".".to_string(),
            files: vec![crate::repo::scanner::ScannedFile {
                id: "source:docs/get-started.md".to_string(),
                path: "docs/get-started.md".to_string(),
                language: "markdown".to_string(),
                kind: "docs".to_string(),
                purpose: crate::repo::scanner::FilePurpose::Docs,
                fingerprint: "fp".to_string(),
                size: 0,
                tags: Vec::new(),
            }],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };

        merge_provider_research(&mut research, &result, &context, &[], &report);

        assert!(research.evidence_clusters.is_empty());
        assert!(research.key_sources.is_empty());
    }

    #[test]
    fn provider_diagram_rollup_keeps_deterministic_nodes_and_edges() {
        let context = PageContext {
            diagram_inputs: vec![PageDiagramInput {
                diagram_id: "runtime-flow".to_string(),
                section_title: "概述".to_string(),
                diagram_type: "flow".to_string(),
                title: "运行时流程".to_string(),
                summary: String::new(),
                nodes: vec![
                    PageDiagramNode {
                        node_id: "start".to_string(),
                        label: "Start".to_string(),
                    },
                    PageDiagramNode {
                        node_id: "end".to_string(),
                        label: "End".to_string(),
                    },
                ],
                edges: vec![PageDiagramEdge {
                    source: "start".to_string(),
                    target: "end".to_string(),
                    label: None,
                }],
            }],
            ..PageContext::default()
        };

        let diagrams = build_provider_diagram_suggestions(
            &[PageResearchDiagramRollup {
                diagram_key: "runtime-flow".to_string(),
                diagram_type: "flow".to_string(),
                title: "运行时流程".to_string(),
                summary: "deterministic".to_string(),
            }],
            &context,
        );

        assert_eq!(diagrams.len(), 1);
        assert_eq!(diagrams[0].nodes.len(), 2);
        assert_eq!(diagrams[0].edges.len(), 1);
    }

    #[test]
    fn provider_research_error_is_recorded_without_aborting_structural_baseline() {
        let mut research = UnitResearch {
            summary: "structural summary".to_string(),
            positioning: "structural positioning".to_string(),
            ..UnitResearch::default()
        };

        mark_provider_error(
            &mut research,
            "page-runtime",
            "unit-runtime",
            &io::Error::other("provider request exhausted retries"),
        );

        assert_eq!(
            research.provider_stop_reason,
            Some(ResearchStopReason::ProviderError)
        );
        assert_eq!(research.summary, "structural summary");
        assert_eq!(research.positioning, "structural positioning");
    }

    #[test]
    fn retry_input_trims_heavy_provider_payload() {
        let input = PageResearchInput {
            page_id: "page-heavy".to_string(),
            page_type: "module".to_string(),
            title: "heavy".to_string(),
            scope: "核心模块/heavy.md".to_string(),
            facts: (0..12).map(|index| format!("fact-{index}")).collect(),
            summary_inputs: (0..11).map(|index| format!("summary-{index}")).collect(),
            hints: (0..9).map(|index| format!("hint-{index}")).collect(),
            allowed_section_slots: vec![PageResearchSectionSlot {
                section_key: "overview".to_string(),
                section_title: "概述".to_string(),
            }],
            evidence_groups: (0..7)
                .map(|index| PageEvidenceGroup {
                    group_id: format!("group-{index}"),
                    section_title: "概述".to_string(),
                    title: format!("title-{index}"),
                    summary: String::new(),
                    items: Vec::new(),
                })
                .collect(),
            diagram_inputs: (0..5)
                .map(|index| crate::domain::context::PageDiagramInput {
                    diagram_id: format!("diagram-{index}"),
                    section_title: "概述".to_string(),
                    diagram_type: "flow".to_string(),
                    title: format!("图-{index}"),
                    summary: String::new(),
                    nodes: Vec::new(),
                    edges: Vec::new(),
                })
                .collect(),
            session: None,
            force_no_tools: false,
            retry_input_applied: false,
        };

        let retry = build_retry_input(&input).expect("heavy payload should be trimmed");

        assert_eq!(retry.facts.len(), RETRY_FACT_LIMIT);
        assert_eq!(retry.hints.len(), RETRY_HINT_LIMIT);
        assert_eq!(retry.summary_inputs.len(), RETRY_SUMMARY_LIMIT);
        assert_eq!(retry.evidence_groups.len(), RETRY_EVIDENCE_LIMIT);
        assert_eq!(retry.diagram_inputs.len(), RETRY_DIAGRAM_LIMIT);
        assert_eq!(retry.allowed_section_slots.len(), 1);
        assert!(retry.retry_input_applied);
    }

    #[test]
    fn retry_input_skips_light_provider_payload() {
        let input = PageResearchInput {
            page_id: "page-light".to_string(),
            page_type: "module".to_string(),
            title: "light".to_string(),
            scope: "核心模块/light.md".to_string(),
            facts: vec!["fact".to_string()],
            summary_inputs: vec!["summary".to_string()],
            hints: vec!["hint".to_string()],
            allowed_section_slots: Vec::new(),
            evidence_groups: Vec::new(),
            diagram_inputs: Vec::new(),
            session: None,
            force_no_tools: false,
            retry_input_applied: false,
        };

        assert!(build_retry_input(&input).is_none());
    }

    #[test]
    fn canonical_provider_input_is_idempotent_after_pretrim() {
        let input = PageResearchInput {
            page_id: "page-heavy".to_string(),
            page_type: "module".to_string(),
            title: "heavy".to_string(),
            scope: "核心模块/heavy.md".to_string(),
            facts: (0..12).map(|index| format!("fact-{index}")).collect(),
            summary_inputs: (0..11).map(|index| format!("summary-{index}")).collect(),
            hints: (0..9).map(|index| format!("hint-{index}")).collect(),
            allowed_section_slots: Vec::new(),
            evidence_groups: Vec::new(),
            diagram_inputs: Vec::new(),
            session: None,
            force_no_tools: false,
            retry_input_applied: false,
        };

        let (canonical, changed) = canonicalize_provider_input(&input);
        let retry = build_retry_input(&canonical);

        assert!(changed);
        assert_eq!(canonical.facts.len(), RETRY_FACT_LIMIT);
        assert_eq!(canonical.hints.len(), RETRY_HINT_LIMIT);
        assert_eq!(canonical.summary_inputs.len(), RETRY_SUMMARY_LIMIT);
        assert!(canonical.retry_input_applied);
        assert!(retry.is_none());
    }

    #[test]
    fn provider_tools_policy_follows_research_profile() {
        assert!(should_force_no_tools(Some(&ResearchProfile::DocsGuide)));
        assert!(should_force_no_tools(Some(&ResearchProfile::ConfigSurface)));
        assert!(should_force_no_tools(Some(
            &ResearchProfile::ExampleTutorial
        )));
        assert!(should_force_no_tools(Some(
            &ResearchProfile::Troubleshooting
        )));
        assert!(should_force_no_tools(Some(
            &ResearchProfile::IntegrationPlatform
        )));
        assert!(!should_force_no_tools(Some(&ResearchProfile::ApiSurface)));
        assert!(!should_force_no_tools(Some(&ResearchProfile::Runtime)));
        assert!(!should_force_no_tools(None));
    }

    #[test]
    fn sparse_force_no_tools_leaf_short_circuits_provider_research() {
        let unit = KnowledgeUnit::new(UnitType::ConceptGuide, "Docs", "docs", "docs/docs.md");
        let input = PageResearchInput {
            page_id: "page-docs".to_string(),
            page_type: "topic".to_string(),
            title: "Docs".to_string(),
            scope: "unit:docs".to_string(),
            facts: vec!["入口：docs/index.md".to_string()],
            summary_inputs: vec!["主题：docs".to_string()],
            hints: Vec::new(),
            allowed_section_slots: Vec::new(),
            evidence_groups: vec![PageEvidenceGroup {
                group_id: "group-docs".to_string(),
                section_title: "概述".to_string(),
                title: "文档入口".to_string(),
                summary: String::new(),
                items: vec![crate::domain::context::PageEvidenceItem {
                    evidence_id: "evidence:docs:index".to_string(),
                    label: "docs/index.md".to_string(),
                    path: "docs/index.md".to_string(),
                    source_id: Some("source:docs/index.md".to_string()),
                    start_line: 1,
                    end_line: 8,
                    note: String::new(),
                    coarse_span: false,
                    ..crate::domain::context::PageEvidenceItem::default()
                }],
            }],
            diagram_inputs: Vec::new(),
            session: None,
            force_no_tools: true,
            retry_input_applied: false,
        };

        let reason = force_no_tools_short_circuit_reason(
            &unit,
            Some(&ResearchProfile::DocsGuide),
            &[],
            &input,
        );

        assert_eq!(reason, Some("sparse_force_no_tools_leaf"));
    }

    #[test]
    fn sparse_force_no_tools_leaf_skips_provider_request() {
        let mut unit = KnowledgeUnit::new(UnitType::ConceptGuide, "Docs", "docs", "docs/docs.md");
        unit.scope = UnitScope {
            source_ids: vec!["source:docs/index.md".to_string()],
            ..UnitScope::default()
        };

        let report = ScanReport {
            root: ".".to_string(),
            files: vec![ScannedFile {
                id: "source:docs/index.md".to_string(),
                path: "docs/index.md".to_string(),
                language: "markdown".to_string(),
                kind: "docs".to_string(),
                purpose: crate::repo::scanner::FilePurpose::Docs,
                fingerprint: "fp".to_string(),
                size: 0,
                tags: Vec::new(),
            }],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };
        let module_tree = ModuleTree {
            root_modules: Vec::new(),
            modules: Vec::new(),
            cross_module_edges: Vec::new(),
            architecture_hints: Vec::new(),
        };
        let repo_context = RepoContext {
            repo_summary_inputs: Vec::new(),
            top_modules: Vec::new(),
            key_entry_points: Vec::new(),
            global_relations: Vec::new(),
            tech_stack: Vec::new(),
            graph_hotspots: Vec::new(),
            detected_processes: Vec::new(),
            community_labels: Vec::new(),
            cycle_warnings: Vec::new(),
            root_topics: Vec::new(),
            process_topics: Vec::new(),
        };
        let module_contexts = vec![crate::domain::context::ModuleContext {
            module_id: "module:docs".to_string(),
            role_hints: vec!["docs".to_string()],
            public_surface: Vec::new(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
            key_sources: vec!["docs/index.md".to_string()],
            graph_hotspots: Vec::new(),
            communities: Vec::new(),
            cycle_warnings: Vec::new(),
            capability_topics: Vec::new(),
        }];
        let symbol_snapshot = ParsedSymbolsSnapshot::default();
        let resolved_graph = ResolvedGraphSnapshot::default();
        let graph_analysis = GraphAnalysisSnapshot::default();
        let graph_summary = GraphSummary::default();
        let mut knowledge_tree = KnowledgeTree::new(unit.id.clone());
        knowledge_tree.add_unit(unit.clone());
        knowledge_tree.build_processing_order();
        let ds = ResearchDataSource {
            report: &report,
            module_tree: &module_tree,
            repo_context: &repo_context,
            module_contexts: &module_contexts,
            symbol_snapshot: &symbol_snapshot,
            resolved_graph: &resolved_graph,
            graph_analysis: &graph_analysis,
            graph_summary: &graph_summary,
            knowledge_tree: &knowledge_tree,
        };
        let steering = provider_enabled_steering();
        let mut service = CountingLlmService::default();
        let mut runtime = LlmRuntime::new(Path::new("."), &steering.llm, Some(&mut service));
        let provider = ProviderBackedResearchProvider {
            runtime: RefCell::new(&mut runtime),
            structural: StructuralResearchProvider,
        };

        let research = provider.research_unit(&unit, &ds, &[]).unwrap();

        drop(provider);
        drop(runtime);
        assert_eq!(service.calls, 0);
        assert_eq!(
            research.provider_stop_reason,
            Some(ResearchStopReason::NotRun)
        );
    }

    #[test]
    fn runtime_profile_never_short_circuits_force_no_tools_provider_research() {
        let unit = KnowledgeUnit::new(
            UnitType::WorkflowDoc,
            "Runtime",
            "runtime",
            "runtime/runtime.md",
        );
        let input = PageResearchInput {
            page_id: "page-runtime".to_string(),
            page_type: "topic".to_string(),
            title: "Runtime".to_string(),
            scope: "unit:runtime".to_string(),
            facts: vec!["入口：src/runtime.rs".to_string()],
            summary_inputs: Vec::new(),
            hints: Vec::new(),
            allowed_section_slots: Vec::new(),
            evidence_groups: Vec::new(),
            diagram_inputs: Vec::new(),
            session: None,
            force_no_tools: true,
            retry_input_applied: false,
        };

        let reason = force_no_tools_short_circuit_reason(
            &unit,
            Some(&ResearchProfile::Runtime),
            &[],
            &input,
        );

        assert!(reason.is_none());
    }

    #[test]
    fn parent_units_with_child_rollups_do_not_short_circuit_force_no_tools_provider_research() {
        let mut unit =
            KnowledgeUnit::new(UnitType::ConfigDoc, "Config", "config", "config/config.md");
        unit.child_unit_ids = vec!["unit:child-a".to_string()];
        let input = PageResearchInput {
            page_id: "page-config".to_string(),
            page_type: "family-child".to_string(),
            title: "Config".to_string(),
            scope: "unit-group:config".to_string(),
            facts: vec!["配置族".to_string()],
            summary_inputs: vec!["child rollup".to_string()],
            hints: Vec::new(),
            allowed_section_slots: Vec::new(),
            evidence_groups: Vec::new(),
            diagram_inputs: Vec::new(),
            session: None,
            force_no_tools: true,
            retry_input_applied: false,
        };
        let child_digests = vec![PageDigest {
            digest_id: "digest-child-a".to_string(),
            unit_id: "unit:child-a".to_string(),
            page_id: "page-child-a".to_string(),
            title: "Child A".to_string(),
            decomposition_profile: None,
            research_profile: Some(ResearchProfile::ConfigSurface),
            summary: "child summary".to_string(),
            key_topics: Vec::new(),
            key_sources: Vec::new(),
            citations: Vec::new(),
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let reason = force_no_tools_short_circuit_reason(
            &unit,
            Some(&ResearchProfile::ConfigSurface),
            &child_digests,
            &input,
        );

        assert!(reason.is_none());
    }
}
