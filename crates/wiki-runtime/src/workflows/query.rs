//! query workflow 负责把 term-only 外部输入路由到 index、knowledge 与 page fallback。
//! 它输出面向宿主的稳定 query route、trust 和 provenance 投影。

use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::change_set::plan_runtime_changes_with_mode;
use crate::domain::module_tree::ModuleNode;
use crate::domain::runtime_profile::{
    AnswerEnvelope, AnswerMode, AnswerSupportingRef, AnswerTrust,
    merge_recommended_action, preflight_for_state, query_trust_for, summarize_health_signals,
    QueryMode, QueryTrust, RecommendedAction,
};
use crate::domain::state::{WikiPageState, WikiState};
use crate::domain::steering::SteeringLoadMode;
use crate::storage::cache_store::cache_dir;
use crate::storage::knowledge_artifacts::{
    load_health_signals, load_knowledge_artifacts, restore_runtime_cache_from_artifacts,
};
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::state_store::{facts_snapshot_ready, load_or_rebuild_state};
use crate::storage::wiki_fs::resolve_page_path;
use crate::workflows::release_scope::project_external_runtime_state;
use wiki_index::query::{self as index_query, IndexQueryRequest, MatchBasis};
use wiki_knowledge::plan_pages_from_knowledge_tree;
use wiki_model::domain::knowledge_artifact::KnowledgeHealthSignal;

const ANSWER_SUPPORTING_REF_LIMIT: usize = 8;

/// `QueryMatch` 描述一个命中的页面，以及它为什么命中。
#[derive(Debug, Clone, Serialize)]
pub struct QueryMatch {
    /// 命中页面的稳定 ID。
    pub page_id: String,
    /// 页面标题。
    pub title: String,
    /// 相对 `.wiki/` 的 Markdown 路径。
    pub path: String,
    /// 页面类别，例如 `overview / architecture / module / topic`。
    pub item_type: String,
    /// 与该页面关联的模块 ID 集合。
    pub module_ids: Vec<String>,
    /// 页面关键源码路径集合。
    pub source_files: Vec<String>,
    /// 面向人读的命中原因。
    pub reasons: Vec<String>,
    /// provenance 原始标签，显式区分 page fallback。
    pub provenance: Vec<String>,
    /// 页面摘要文本。
    pub summary: String,
    /// 命中模式，区分结构化、page fallback 或 mixed。
    pub match_mode: String,
    /// 附带返回的模块/源码/关系/符号上下文。
    #[serde(default)]
    pub context_pack: QueryContextPack,
}

/// `QueryContextPack` 让 Agent 不需要二次查询就能获得页面的关联上下文。
#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryContextPack {
    /// 与当前页面相关的模块摘要。
    pub module_summaries: Vec<String>,
    /// 当前页面关键源码路径。
    pub key_source_paths: Vec<String>,
    /// 来自关系命中的证据片段。
    pub relation_evidence: Vec<String>,
    /// 与页面关联的核心符号名摘要。
    pub symbols: Vec<String>,
}

/// `QueryModuleMatch` 是 query 返回的模块视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryModuleMatch {
    /// 模块稳定 ID。
    pub module_id: String,
    /// 模块展示名。
    pub name: String,
    /// 模块 kind，例如 repository / package / service。
    pub kind: String,
    /// 模块根路径集合。
    pub root_paths: Vec<String>,
    /// 扫描阶段给出的模块标签。
    pub tags: Vec<String>,
    /// 模块命中原因。
    pub reasons: Vec<String>,
}

/// `QuerySourceMatch` 是 query 返回的源码视图。
#[derive(Debug, Clone, Serialize)]
pub struct QuerySourceMatch {
    /// 源码稳定 ID。
    pub source_id: String,
    /// 相对仓库根目录的源码路径。
    pub path: String,
    /// 所属模块 ID。
    pub module_ids: Vec<String>,
    /// 源码命中原因。
    pub reasons: Vec<String>,
}

/// `QueryRelationMatch` 预留给后续知识/关系消费迭代。
#[derive(Debug, Clone, Serialize)]
pub struct QueryRelationMatch {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
    pub evidence: Vec<String>,
    pub reasons: Vec<String>,
}

/// `QuerySymbolMatch` 是 query 返回的符号视图。
#[derive(Debug, Clone, Serialize)]
pub struct QuerySymbolMatch {
    /// 命中符号的稳定 ID。
    pub symbol_id: String,
    /// 符号名。
    pub name: String,
    /// 符号标签，例如 function / class / method。
    pub label: String,
    /// 符号所在源码路径。
    pub file_path: String,
    /// 符号起始行。
    pub start_line: usize,
    /// 符号结束行。
    pub end_line: usize,
    /// 符号所属语言。
    pub language: String,
    /// 由源码映射回来的页面 ID。
    pub page_ids: Vec<String>,
    /// 由源码映射回来的模块 ID。
    pub module_ids: Vec<String>,
    /// 符号命中原因。
    pub reasons: Vec<String>,
    /// FTS 返回的原始分数，供排序和调试使用。
    pub score: f64,
}

/// `QueryGraphEdgeMatch` 是 query 返回的 graph edge 视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryGraphEdgeMatch {
    /// graph edge 稳定 ID。
    pub edge_id: String,
    /// 当前 query adapter 只投影 `CALLS`。
    pub edge_type: String,
    /// edge 起点 symbol ID。
    pub source_symbol_id: String,
    /// edge 起点 symbol 名称。
    pub source_symbol: String,
    /// edge 终点 symbol ID。
    pub target_symbol_id: String,
    /// edge 终点 symbol 名称。
    pub target_symbol: String,
    /// edge 置信度。
    pub confidence: f64,
    /// resolve 阶段留下的解释文本。
    pub reason: String,
    /// 相对命中 symbol 的最短跳数。
    #[serde(default)]
    pub hop_distance: usize,
    /// 该边是上游影响还是下游调用链。
    #[serde(default)]
    pub traversal_modes: Vec<String>,
    /// graph 命中的原因。
    pub reasons: Vec<String>,
    /// graph 命中 provenance。
    pub provenance: Vec<String>,
}

/// `QueryProcessMatch` 预留给后续图消费迭代。
#[derive(Debug, Clone, Serialize)]
pub struct QueryProcessMatch {
    pub process_id: String,
    pub label: String,
    pub process_type: String,
    pub steps: Vec<String>,
    pub matched_symbol_ids: Vec<String>,
    pub reasons: Vec<String>,
    pub provenance: Vec<String>,
}

/// `QueryCommunityMatch` 预留给后续图消费迭代。
#[derive(Debug, Clone, Serialize)]
pub struct QueryCommunityMatch {
    pub community_id: String,
    pub label: String,
    pub cohesion: f64,
    pub symbol_count: usize,
    pub matched_symbol_ids: Vec<String>,
    pub member_symbols: Vec<String>,
    pub reasons: Vec<String>,
    pub provenance: Vec<String>,
}

/// `QueryReport` 是当前对 Agent 最友好的结构化返回。
#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    /// 用户查询原词。
    pub term: String,
    /// 当前 query 时 runtime 的外部状态。
    pub runtime_state: String,
    /// 当前结果主要来自结构化 facts/index 还是 page fallback。
    pub query_mode: QueryMode,
    /// 当前阶段宿主可消费的 query trust。
    pub query_trust: QueryTrust,
    /// 若 runtime 未 fully ready，明确告诉宿主下一步动作。
    pub recommended_action: RecommendedAction,
    /// 命中的页面路径列表，便于快速判断覆盖面。
    pub matched_pages: Vec<String>,
    /// 结构化模块命中。
    pub matched_modules: Vec<QueryModuleMatch>,
    /// 结构化源码命中。
    pub matched_sources: Vec<QuerySourceMatch>,
    /// 结构化关系命中。
    pub matched_relations: Vec<QueryRelationMatch>,
    /// 结构化符号命中。
    pub matched_symbols: Vec<QuerySymbolMatch>,
    /// graph edge 命中。
    #[serde(default)]
    pub matched_symbol_edges: Vec<QueryGraphEdgeMatch>,
    /// process 命中。
    #[serde(default)]
    pub matched_processes: Vec<QueryProcessMatch>,
    /// community 命中。
    #[serde(default)]
    pub matched_communities: Vec<QueryCommunityMatch>,
    /// Agent 直接消费的页面级结果。
    pub matches: Vec<QueryMatch>,
    /// provenance 汇总文本，方便测试和日志检查。
    #[serde(default)]
    pub provenance_summary: String,
    /// 当前 query 可直接附带的最小 answer contract。
    pub answer: AnswerEnvelope,
}

/// 执行关键词查询。
/// 对外入口仍保持 `term`，内部统一转成 `wiki-index::query(auto)`。
pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    run_query_with_mode(repo_root, term, SteeringLoadMode::Production)
}

/// 执行带显式 steering mode 的关键词查询。
/// `query` 也需要和 `init/status/update` 使用同一套 steering 视图，
/// 否则 transport 层会出现 `developmentMode` 只对部分动作生效的协议裂缝。
pub fn run_query_with_mode(
    repo_root: &Path,
    term: &str,
    steering_mode: SteeringLoadMode,
) -> io::Result<QueryReport> {
    let mut plan = plan_runtime_changes_with_mode(repo_root, steering_mode)?;
    let mut facts_ready = facts_snapshot_ready(repo_root)?;
    if !facts_ready
        && plan.needs_rebuild_reason.as_deref() == Some("cache_missing")
        && !cache_dir(repo_root).exists()
        && restore_runtime_cache_from_artifacts(repo_root)?
    {
        plan = plan_runtime_changes_with_mode(repo_root, steering_mode)?;
        facts_ready = facts_snapshot_ready(repo_root)?;
    }
    let runtime_state = project_external_runtime_state(repo_root, plan.state(), facts_ready);
    let preflight = preflight_for_state(&runtime_state, facts_ready);
    let health_signals = load_health_signals(repo_root).unwrap_or_default();
    let health_summary = summarize_health_signals(&health_signals);
    let recommended_action =
        merge_recommended_action(preflight.recommended_action, health_summary.as_ref());
    let needle = term.trim().to_lowercase();

    if needle.is_empty() {
        return Ok(empty_query_report(
            term,
            &runtime_state,
            recommended_action,
            effective_query_trust(&runtime_state, facts_ready, recommended_action, false),
        ));
    }

    if !facts_ready {
        return Err(index_not_ready_error());
    }

    let index_result = index_query::run_query(
        &SqliteIndexStore::new(repo_root),
        &IndexQueryRequest {
            intent: index_query::IndexQueryIntent::Auto,
            text: term.to_string(),
            ..IndexQueryRequest::default()
        },
    )?;
    let fallback_state = load_or_rebuild_state(repo_root).ok();
    let page_ids_by_source_path = fallback_state
        .as_ref()
        .map(build_page_ids_by_source_path)
        .unwrap_or_default();
    let matched_symbols = project_symbol_matches(&index_result, &page_ids_by_source_path);
    let matched_symbols_by_file = build_symbols_by_file(&matched_symbols);
    let knowledge_matches = collect_knowledge_matches(
        repo_root,
        &needle,
        fallback_state.as_ref(),
        &matched_symbols_by_file,
    );
    let page_fallback_matches = if knowledge_matches.is_empty() {
        fallback_state
            .as_ref()
            .map(|state| {
                collect_page_fallback_matches(repo_root, state, &needle, &matched_symbols_by_file)
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let has_knowledge_hits = !knowledge_matches.is_empty();
    let has_page_fallback = page_fallback_matches
        .iter()
        .any(is_textual_page_fallback_match);
    let matches = if has_knowledge_hits {
        knowledge_matches
    } else {
        page_fallback_matches
    };
    let has_index_hits = !index_result.modules.is_empty()
        || !index_result.sources.is_empty()
        || !index_result.symbols.is_empty()
        || !index_result.call_edges.is_empty();
    let query_mode = match (has_index_hits, has_knowledge_hits, has_page_fallback) {
        (_, _, true) if has_index_hits || has_knowledge_hits => QueryMode::Mixed,
        (false, false, true) => QueryMode::PageFallback,
        (true, true, false) => QueryMode::Mixed,
        (false, true, false) => QueryMode::KnowledgeFirst,
        _ => QueryMode::IndexFirst,
    };
    let query_trust = match query_trust_for(&runtime_state, facts_ready) {
        QueryTrust::Blocked if has_index_hits || !matches.is_empty() => {
            QueryTrust::StaleButQueryable
        }
        QueryTrust::Ready
            if recommended_action != RecommendedAction::None
                && (has_index_hits || !matches.is_empty()) =>
        {
            QueryTrust::StaleButQueryable
        }
        trust => trust,
    };
    let matched_modules = project_module_matches(&index_result);
    let matched_sources = project_source_matches(&index_result);
    let matched_symbol_edges = project_graph_edge_matches(&index_result);
    let matched_relations = fallback_state
        .as_ref()
        .map(|state| project_relation_matches(state, &index_result, &matched_symbols, &matches))
        .unwrap_or_default();
    let provenance_summary =
        build_provenance_summary(has_index_hits, has_knowledge_hits, has_page_fallback);
    let answer = build_answer_envelope(
        term,
        query_trust,
        recommended_action,
        &provenance_summary,
        &matches,
        &matched_modules,
        &matched_sources,
        &matched_symbols,
        &matched_symbol_edges,
        &health_signals,
    );

    Ok(QueryReport {
        term: term.to_string(),
        runtime_state: runtime_state.clone(),
        query_mode,
        query_trust,
        recommended_action,
        matched_pages: matches.iter().map(|page| page.path.clone()).collect(),
        matched_modules,
        matched_sources,
        matched_relations,
        matched_symbols,
        matched_symbol_edges,
        matched_processes: Vec::new(),
        matched_communities: Vec::new(),
        provenance_summary,
        answer,
        matches,
    })
}

fn empty_query_report(
    term: &str,
    runtime_state: &str,
    recommended_action: RecommendedAction,
    query_trust: QueryTrust,
) -> QueryReport {
    let provenance_summary = String::new();
    let answer = build_answer_envelope(
        term,
        query_trust,
        recommended_action,
        &provenance_summary,
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
    );
    QueryReport {
        term: term.to_string(),
        runtime_state: runtime_state.to_string(),
        query_mode: QueryMode::IndexFirst,
        query_trust,
        recommended_action,
        matched_pages: Vec::new(),
        matched_modules: Vec::new(),
        matched_sources: Vec::new(),
        matched_relations: Vec::new(),
        matched_symbols: Vec::new(),
        matched_symbol_edges: Vec::new(),
        matched_processes: Vec::new(),
        matched_communities: Vec::new(),
        matches: Vec::new(),
        provenance_summary,
        answer,
    }
}

fn effective_query_trust(
    runtime_state: &str,
    facts_ready: bool,
    recommended_action: RecommendedAction,
    has_hits: bool,
) -> QueryTrust {
    match query_trust_for(runtime_state, facts_ready) {
        QueryTrust::Ready if recommended_action != RecommendedAction::None && has_hits => {
            QueryTrust::StaleButQueryable
        }
        trust => trust,
    }
}

fn project_module_matches(index_result: &index_query::IndexQueryResult) -> Vec<QueryModuleMatch> {
    index_result
        .modules
        .iter()
        .map(|module| QueryModuleMatch {
            module_id: module.module_id.clone(),
            name: module.name.clone(),
            kind: module.kind.clone(),
            root_paths: module.root_paths.clone(),
            tags: module.tags.clone(),
            reasons: vec![match_basis_reason(module.match_basis)],
        })
        .collect()
}

fn project_source_matches(index_result: &index_query::IndexQueryResult) -> Vec<QuerySourceMatch> {
    index_result
        .sources
        .iter()
        .map(|source| QuerySourceMatch {
            source_id: source.source_id.clone(),
            path: source.path.clone(),
            module_ids: source.module_ids.clone(),
            reasons: vec![match_basis_reason(source.match_basis)],
        })
        .collect()
}

fn project_relation_matches(
    state: &WikiState,
    index_result: &index_query::IndexQueryResult,
    matched_symbols: &[QuerySymbolMatch],
    matches: &[QueryMatch],
) -> Vec<QueryRelationMatch> {
    let mut reasons_by_module = BTreeMap::<String, BTreeSet<String>>::new();

    for module in &index_result.modules {
        reasons_by_module
            .entry(module.module_id.clone())
            .or_default()
            .insert(match_basis_reason(module.match_basis));
    }

    for source in &index_result.sources {
        for module_id in &source.module_ids {
            reasons_by_module
                .entry(module_id.clone())
                .or_default()
                .insert(match_basis_reason(source.match_basis));
        }
    }

    for symbol in matched_symbols {
        for module_id in &symbol.module_ids {
            let bucket = reasons_by_module.entry(module_id.clone()).or_default();
            for reason in &symbol.reasons {
                bucket.insert(reason.clone());
            }
        }
    }

    for page in matches {
        for module_id in &page.module_ids {
            reasons_by_module
                .entry(module_id.clone())
                .or_default()
                .insert(match_route_tag(page).to_string());
        }
    }

    let mut relations = state
        .relations
        .iter()
        .filter_map(|relation| {
            let mut reasons = BTreeSet::new();
            if let Some(source_reasons) = reasons_by_module.get(&relation.source_id) {
                reasons.extend(source_reasons.iter().cloned());
            }
            if let Some(target_reasons) = reasons_by_module.get(&relation.target_id) {
                reasons.extend(target_reasons.iter().cloned());
            }
            if reasons.is_empty() {
                return None;
            }

            Some(QueryRelationMatch {
                source_id: relation.source_id.clone(),
                target_id: relation.target_id.clone(),
                relation_type: relation.relation_type.clone(),
                evidence: relation.evidence.clone(),
                reasons: reasons.into_iter().collect(),
            })
        })
        .collect::<Vec<_>>();
    relations.sort_by(|left, right| {
        left.source_id
            .cmp(&right.source_id)
            .then(left.relation_type.cmp(&right.relation_type))
            .then(left.target_id.cmp(&right.target_id))
    });
    relations
}

fn collect_knowledge_matches(
    repo_root: &Path,
    needle: &str,
    state: Option<&WikiState>,
    matched_symbols_by_file: &BTreeMap<String, Vec<String>>,
) -> Vec<QueryMatch> {
    let Ok(artifacts) = load_knowledge_artifacts(repo_root) else {
        return Vec::new();
    };
    let planned_pages = plan_pages_from_knowledge_tree(&artifacts.knowledge_tree)
        .into_iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let module_index = state
        .map(|runtime| {
            runtime
                .modules
                .iter()
                .map(|module| (module.id.clone(), module))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let page_index = state
        .map(|runtime| {
            runtime
                .pages
                .iter()
                .map(|page| (page.page_id.clone(), page))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let mut matches = artifacts
        .page_digests
        .iter()
        .filter_map(|digest| {
            let planned_page = planned_pages.get(&digest.page_id)?;
            let runtime_page = page_index.get(&digest.page_id).copied();
            let mut reasons = Vec::new();
            let mut provenance = Vec::new();

            if contains_case_insensitive(&digest.title, needle) {
                reasons.push("知识标题匹配".to_string());
                provenance.push("knowledge:title".to_string());
            }
            if contains_case_insensitive(&digest.summary, needle) {
                reasons.push("知识摘要匹配".to_string());
                provenance.push("knowledge:summary".to_string());
            }
            if digest
                .key_topics
                .iter()
                .any(|topic| contains_case_insensitive(topic, needle))
            {
                reasons.push("知识主题匹配".to_string());
                provenance.push("knowledge:topic".to_string());
            }
            if digest.section_digests.iter().any(|section| {
                contains_case_insensitive(&section.title, needle)
                    || contains_case_insensitive(&section.summary, needle)
            }) {
                reasons.push("知识章节匹配".to_string());
                provenance.push("knowledge:section".to_string());
            }

            let source_files = collect_knowledge_source_files(digest, runtime_page);
            if source_files
                .iter()
                .any(|source_path| contains_case_insensitive(source_path, needle))
            {
                reasons.push("知识源码锚点匹配".to_string());
                provenance.push("knowledge:key_source".to_string());
            }

            if reasons.is_empty() {
                return None;
            }

            Some(QueryMatch {
                page_id: planned_page.id.clone(),
                title: planned_page.title.clone(),
                path: format!(".wiki/{}", planned_page.relative_path),
                item_type: planned_page.page_type.clone(),
                module_ids: planned_page.module_ids.clone(),
                source_files: source_files.clone(),
                reasons: reasons.clone(),
                provenance,
                summary: if digest.summary.trim().is_empty() {
                    reasons.join("、")
                } else {
                    digest.summary.clone()
                },
                match_mode: "knowledge_digest".to_string(),
                context_pack: build_knowledge_context_pack(
                    planned_page,
                    state,
                    &module_index,
                    matched_symbols_by_file,
                    source_files,
                ),
            })
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.page_id.cmp(&right.page_id))
    });
    matches
}

fn project_symbol_matches(
    index_result: &index_query::IndexQueryResult,
    page_ids_by_source_path: &BTreeMap<String, Vec<String>>,
) -> Vec<QuerySymbolMatch> {
    index_result
        .symbols
        .iter()
        .map(|symbol| QuerySymbolMatch {
            symbol_id: symbol.symbol_id.clone(),
            name: symbol.name.clone(),
            label: symbol.label.clone(),
            file_path: symbol.file_path.clone(),
            start_line: symbol.start_line,
            end_line: symbol.end_line,
            language: symbol.language.clone(),
            page_ids: page_ids_by_source_path
                .get(&symbol.file_path)
                .cloned()
                .unwrap_or_default(),
            module_ids: symbol.module_ids.clone(),
            reasons: vec![match_basis_reason(symbol.match_basis)],
            score: symbol.score.unwrap_or_default(),
        })
        .collect()
}

fn project_graph_edge_matches(
    index_result: &index_query::IndexQueryResult,
) -> Vec<QueryGraphEdgeMatch> {
    index_result
        .call_edges
        .iter()
        .map(|edge| QueryGraphEdgeMatch {
            edge_id: edge.edge_id.clone(),
            edge_type: "CALLS".to_string(),
            source_symbol_id: edge.source_id.clone(),
            source_symbol: edge.source_name.clone(),
            target_symbol_id: edge.target_id.clone(),
            target_symbol: edge.target_name.clone(),
            confidence: edge.confidence,
            reason: edge.reason.clone(),
            hop_distance: edge.hop_distance,
            traversal_modes: vec![edge.traversal_direction.clone()],
            reasons: vec![match_basis_reason(edge.match_basis)],
            provenance: vec!["index:call_trace".to_string()],
        })
        .collect()
}

fn collect_page_fallback_matches(
    repo_root: &Path,
    state: &WikiState,
    needle: &str,
    matched_symbols_by_file: &BTreeMap<String, Vec<String>>,
) -> Vec<QueryMatch> {
    let module_index = state
        .modules
        .iter()
        .map(|module| (module.id.clone(), module))
        .collect::<BTreeMap<_, _>>();
    let mut matches = state
        .pages
        .iter()
        .filter_map(|page| {
            let page_path = resolve_page_path(repo_root, &page.path);
            let content = fs::read_to_string(&page_path).unwrap_or_default();
            let mut reasons = Vec::new();
            let mut provenance = Vec::new();
            let mut has_textual_fallback = false;
            let has_symbol_match = page
                .source_paths
                .iter()
                .any(|source_path| matched_symbols_by_file.contains_key(source_path));

            if contains_case_insensitive(&page.title, needle) {
                reasons.push("页面标题匹配".to_string());
                provenance.push("page-fallback:title".to_string());
                has_textual_fallback = true;
            }
            if contains_case_insensitive(&content, needle) {
                reasons.push("Markdown 内容匹配".to_string());
                provenance.push("page-fallback:markdown".to_string());
                has_textual_fallback = true;
            }
            if has_symbol_match {
                reasons.push("关联符号匹配".to_string());
                provenance.push("index:symbol".to_string());
            }
            if reasons.is_empty() {
                return None;
            }

            provenance.extend(page.provenance.clone());
            Some(QueryMatch {
                page_id: page.page_id.clone(),
                title: page.title.clone(),
                path: page.path.clone(),
                item_type: page.page_type.clone(),
                module_ids: page.module_ids.clone(),
                source_files: page.source_paths.clone(),
                summary: reasons.join("、"),
                reasons,
                provenance,
                match_mode: if has_textual_fallback {
                    "fallback_markdown".to_string()
                } else {
                    "index_projection".to_string()
                },
                context_pack: build_context_pack(
                    page,
                    state,
                    &module_index,
                    matched_symbols_by_file,
                ),
            })
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.page_id.cmp(&right.page_id))
    });
    matches
}

fn collect_knowledge_source_files(
    digest: &wiki_knowledge::domain::research::PageDigest,
    runtime_page: Option<&WikiPageState>,
) -> Vec<String> {
    let mut source_files = BTreeSet::new();
    source_files.extend(digest.key_sources.iter().cloned());
    source_files.extend(digest.planned_key_sources.iter().cloned());
    source_files.extend(digest.grounded_key_sources.iter().cloned());
    if let Some(page) = runtime_page {
        source_files.extend(page.source_paths.iter().cloned());
    }
    source_files.into_iter().collect()
}

fn build_page_ids_by_source_path(state: &WikiState) -> BTreeMap<String, Vec<String>> {
    let mut page_ids_by_source_path = BTreeMap::<String, Vec<String>>::new();
    for page in &state.pages {
        for source_path in &page.source_paths {
            page_ids_by_source_path
                .entry(source_path.clone())
                .or_default()
                .push(page.page_id.clone());
        }
    }
    page_ids_by_source_path
}

fn build_context_pack(
    page: &WikiPageState,
    state: &WikiState,
    module_index: &BTreeMap<String, &ModuleNode>,
    matched_symbols_by_file: &BTreeMap<String, Vec<String>>,
) -> QueryContextPack {
    let module_summaries = page
        .module_ids
        .iter()
        .filter_map(|mid| module_index.get(mid))
        .map(|module| {
            let tags = if module.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", module.tags.join(", "))
            };
            format!("{} ({}){}", module.name, module.kind, tags)
        })
        .collect();
    let key_source_paths = page.source_paths.iter().take(8).cloned().collect();
    let relation_evidence = state
        .relations
        .iter()
        .filter(|relation| {
            page.module_ids.contains(&relation.source_id)
                || page.module_ids.contains(&relation.target_id)
        })
        .map(|relation| {
            format!(
                "{} -[{}]-> {}",
                relation.source_id, relation.relation_type, relation.target_id
            )
        })
        .take(10)
        .collect();
    let symbols = page
        .source_paths
        .iter()
        .filter_map(|source_path| matched_symbols_by_file.get(source_path))
        .flat_map(|items| items.iter().cloned())
        .take(12)
        .collect();

    QueryContextPack {
        module_summaries,
        key_source_paths,
        relation_evidence,
        symbols,
    }
}

fn build_knowledge_context_pack(
    page: &wiki_knowledge::PlannedPage,
    state: Option<&WikiState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    matched_symbols_by_file: &BTreeMap<String, Vec<String>>,
    source_files: Vec<String>,
) -> QueryContextPack {
    let module_summaries = page
        .module_ids
        .iter()
        .filter_map(|mid| module_index.get(mid))
        .map(|module| {
            let tags = if module.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", module.tags.join(", "))
            };
            format!("{} ({}){}", module.name, module.kind, tags)
        })
        .collect();
    let relation_evidence = state
        .map(|runtime| {
            runtime
                .relations
                .iter()
                .filter(|relation| {
                    page.module_ids.contains(&relation.source_id)
                        || page.module_ids.contains(&relation.target_id)
                })
                .map(|relation| {
                    format!(
                        "{} -[{}]-> {}",
                        relation.source_id, relation.relation_type, relation.target_id
                    )
                })
                .take(10)
                .collect()
        })
        .unwrap_or_default();
    let symbols = source_files
        .iter()
        .filter_map(|source_path| matched_symbols_by_file.get(source_path))
        .flat_map(|items| items.iter().cloned())
        .take(12)
        .collect();

    QueryContextPack {
        module_summaries,
        key_source_paths: source_files.into_iter().take(8).collect(),
        relation_evidence,
        symbols,
    }
}

fn build_symbols_by_file(symbol_matches: &[QuerySymbolMatch]) -> BTreeMap<String, Vec<String>> {
    let mut symbols_by_file = BTreeMap::<String, Vec<String>>::new();
    for symbol in symbol_matches {
        symbols_by_file
            .entry(symbol.file_path.clone())
            .or_default()
            .push(format!(
                "{} {} ({})",
                symbol.label, symbol.name, symbol.language
            ));
    }
    for items in symbols_by_file.values_mut() {
        items.sort();
        items.dedup();
    }
    symbols_by_file
}

fn build_provenance_summary(
    has_index_hits: bool,
    has_knowledge_hits: bool,
    has_page_fallback: bool,
) -> String {
    let mut tags = Vec::new();
    if has_index_hits {
        tags.push("index_hit");
    }
    if has_knowledge_hits {
        tags.push("knowledge_hit");
    }
    if has_page_fallback {
        tags.push("page_fallback");
    }
    tags.join(",")
}

fn is_textual_page_fallback_match(query_match: &QueryMatch) -> bool {
    query_match.match_mode == "fallback_markdown"
        || query_match
            .provenance
            .iter()
            .any(|item| item.starts_with("page-fallback:"))
}

fn build_answer_envelope(
    term: &str,
    query_trust: QueryTrust,
    recommended_action: RecommendedAction,
    provenance_summary: &str,
    matches: &[QueryMatch],
    matched_modules: &[QueryModuleMatch],
    matched_sources: &[QuerySourceMatch],
    matched_symbols: &[QuerySymbolMatch],
    matched_symbol_edges: &[QueryGraphEdgeMatch],
    health_signals: &[KnowledgeHealthSignal],
) -> AnswerEnvelope {
    let base_supporting_refs = collect_answer_supporting_refs(
        matches,
        matched_modules,
        matched_sources,
        matched_symbols,
        matched_symbol_edges,
        &[],
    );
    let provenance = collect_answer_provenance(
        provenance_summary,
        query_trust,
        recommended_action,
        matched_symbols,
        matched_symbol_edges,
        health_signals,
    );
    let has_page_fallback = provenance.iter().any(|tag| tag == "page_fallback");
    let answer_mode = if base_supporting_refs.is_empty() {
        AnswerMode::Refuse
    } else if query_trust != QueryTrust::Ready
        || recommended_action != RecommendedAction::None
        || has_page_fallback
    {
        AnswerMode::Degraded
    } else {
        AnswerMode::Direct
    };
    let answer_trust = match answer_mode {
        AnswerMode::Direct => AnswerTrust::Grounded,
        AnswerMode::Degraded => AnswerTrust::Constrained,
        AnswerMode::Refuse => AnswerTrust::Unsupported,
    };
    let supporting_refs = match answer_mode {
        AnswerMode::Direct => base_supporting_refs,
        AnswerMode::Degraded => collect_answer_supporting_refs(
            matches,
            matched_modules,
            matched_sources,
            matched_symbols,
            matched_symbol_edges,
            health_signals,
        ),
        AnswerMode::Refuse => Vec::new(),
    };

    AnswerEnvelope {
        text: build_answer_text(
            term,
            answer_mode,
            recommended_action,
            &provenance,
            supporting_refs.len(),
        ),
        answer_mode,
        answer_trust,
        recommended_action,
        provenance,
        supporting_refs,
    }
}

fn collect_answer_supporting_refs(
    matches: &[QueryMatch],
    matched_modules: &[QueryModuleMatch],
    matched_sources: &[QuerySourceMatch],
    matched_symbols: &[QuerySymbolMatch],
    matched_symbol_edges: &[QueryGraphEdgeMatch],
    health_signals: &[KnowledgeHealthSignal],
) -> Vec<AnswerSupportingRef> {
    let mut refs = Vec::new();
    let mut seen = BTreeSet::new();

    for symbol in matched_symbols {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: "symbol".to_string(),
                ref_id: symbol.symbol_id.clone(),
                label: format!("{} {}", symbol.label, symbol.name),
                provenance: vec!["index_hit".to_string()],
            },
        );
    }

    for edge in matched_symbol_edges {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: "graph_edge".to_string(),
                ref_id: edge.edge_id.clone(),
                label: format!("{} -> {}", edge.source_symbol, edge.target_symbol),
                provenance: if edge.provenance.is_empty() {
                    vec!["graph_hit".to_string()]
                } else {
                    edge.provenance.clone()
                },
            },
        );
    }

    for page in matches {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: if page.match_mode == "knowledge_digest" {
                    "projection_ref".to_string()
                } else {
                    "page_ref".to_string()
                },
                ref_id: page.page_id.clone(),
                label: page.title.clone(),
                provenance: if page.provenance.is_empty() {
                    vec![match_route_tag(page).to_string()]
                } else {
                    page.provenance.clone()
                },
            },
        );
    }

    for source in matched_sources {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: "source".to_string(),
                ref_id: source.source_id.clone(),
                label: source.path.clone(),
                provenance: vec!["index_hit".to_string()],
            },
        );
    }

    for module in matched_modules {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: "module".to_string(),
                ref_id: module.module_id.clone(),
                label: module.name.clone(),
                provenance: vec!["index_hit".to_string()],
            },
        );
    }

    for signal in health_signals {
        push_answer_supporting_ref(
            &mut refs,
            &mut seen,
            AnswerSupportingRef {
                ref_kind: "health_signal".to_string(),
                ref_id: signal.signal_id.clone(),
                label: signal.reason.clone(),
                provenance: vec![format!("health:{}", signal.signal_kind.as_str())],
            },
        );
    }

    refs
}

fn push_answer_supporting_ref(
    refs: &mut Vec<AnswerSupportingRef>,
    seen: &mut BTreeSet<String>,
    supporting_ref: AnswerSupportingRef,
) {
    if refs.len() >= ANSWER_SUPPORTING_REF_LIMIT {
        return;
    }
    let key = format!(
        "{}|{}|{}",
        supporting_ref.ref_kind, supporting_ref.ref_id, supporting_ref.label
    );
    if seen.insert(key) {
        refs.push(supporting_ref);
    }
}

fn collect_answer_provenance(
    provenance_summary: &str,
    query_trust: QueryTrust,
    recommended_action: RecommendedAction,
    matched_symbols: &[QuerySymbolMatch],
    matched_symbol_edges: &[QueryGraphEdgeMatch],
    health_signals: &[KnowledgeHealthSignal],
) -> Vec<String> {
    let mut provenance = provenance_summary
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();

    if !matched_symbols.is_empty() {
        provenance.insert("index_hit".to_string());
    }
    if !matched_symbol_edges.is_empty() {
        provenance.insert("graph_hit".to_string());
    }
    if query_trust != QueryTrust::Ready || recommended_action != RecommendedAction::None {
        provenance.insert("health_degraded".to_string());
    }
    if (query_trust != QueryTrust::Ready || recommended_action == RecommendedAction::Review)
        && health_signals.iter().any(|signal| {
        signal.signal_kind.as_str() == "governance_conflict"
            || signal.recommended_action == wiki_model::domain::knowledge_artifact::KnowledgeHealthRecommendedAction::Review
    }) {
        provenance.insert("governance_conflict".to_string());
    }

    provenance.into_iter().collect()
}

fn build_answer_text(
    term: &str,
    answer_mode: AnswerMode,
    recommended_action: RecommendedAction,
    provenance: &[String],
    supporting_ref_count: usize,
) -> String {
    let route_text = if provenance.is_empty() {
        "无正式来源".to_string()
    } else {
        provenance
            .iter()
            .map(|tag| match tag.as_str() {
                "index_hit" => "索引命中",
                "graph_hit" => "图上下文",
                "knowledge_hit" => "knowledge 命中",
                "page_fallback" => "页面兜底",
                "health_degraded" => "health 降级",
                "governance_conflict" => "治理冲突",
                _ => tag,
            })
            .collect::<Vec<_>>()
            .join(" / ")
    };
    let action_text = match recommended_action {
        RecommendedAction::None => String::new(),
        _ => format!("建议下一步执行 {}。", recommended_action_label(recommended_action)),
    };

    match answer_mode {
        AnswerMode::Direct => format!(
            "当前可基于 formal query/knowledge 命中直接回答“{}”。支持依据 {} 条，来源：{}。",
            term, supporting_ref_count, route_text
        ),
        AnswerMode::Degraded => format!(
            "当前只能基于受限的 formal 依据回答“{}”。支持依据 {} 条，来源：{}。{}",
            term, supporting_ref_count, route_text, action_text
        ),
        AnswerMode::Refuse => {
            if action_text.is_empty() {
                format!("当前没有足够的 formal 依据稳定回答“{}”。", term)
            } else {
                format!(
                    "当前没有足够的 formal 依据稳定回答“{}”。{}",
                    term, action_text
                )
            }
        }
    }
}

fn recommended_action_label(action: RecommendedAction) -> &'static str {
    match action {
        RecommendedAction::None => "none",
        RecommendedAction::Init => "init",
        RecommendedAction::Review => "review",
        RecommendedAction::Update => "update",
        RecommendedAction::Rebuild => "rebuild",
        RecommendedAction::Sync => "sync",
    }
}

fn match_route_tag(query_match: &QueryMatch) -> &'static str {
    if query_match
        .provenance
        .iter()
        .any(|item| item.starts_with("knowledge:"))
    {
        "knowledge_hit"
    } else if is_textual_page_fallback_match(query_match) {
        "page_fallback"
    } else {
        "index_hit"
    }
}

fn match_basis_reason(match_basis: MatchBasis) -> String {
    match match_basis {
        MatchBasis::SymbolFts => "symbol_fts".to_string(),
        MatchBasis::SourcePath => "source_path".to_string(),
        MatchBasis::SourceFilename => "source_filename".to_string(),
        MatchBasis::ModuleName => "module_name".to_string(),
        MatchBasis::ModuleRootPath => "module_root_path".to_string(),
        MatchBasis::EntrypointMembership => "entry_point_membership".to_string(),
        MatchBasis::CallTrace => "call_trace".to_string(),
    }
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(needle)
}

fn index_not_ready_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        "index not ready: facts snapshot missing",
    )
}
