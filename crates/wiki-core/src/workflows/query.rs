use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;
use crate::domain::state::{SourceState, WikiPageState, WikiState};
use crate::repo::symbols::SymbolNode;
use crate::storage::sqlite_store;
use crate::storage::state_store::load_or_rebuild_state;
use crate::storage::wiki_fs::resolve_page_path;

/// `QueryMatch` 描述一个命中的页面，以及它为什么命中。
#[derive(Debug, Clone, Serialize)]
pub struct QueryMatch {
    /// 命中页面的稳定 ID。
    pub page_id: String,
    /// 页面标题。
    pub title: String,
    /// 相对 `.wiki/` 的 Markdown 路径。
    pub path: String,
    /// 页面类别，例如 `overview / architecture / family-index / family-child / family-leaf-doc / module / topic`。
    pub item_type: String,
    /// 与该页面关联的模块 ID 集合。
    pub module_ids: Vec<String>,
    /// 页面关键源码路径集合。
    pub source_files: Vec<String>,
    /// 面向人读的命中原因。
    pub reasons: Vec<String>,
    /// provenance 原始标签，保留 BM25 / 结构化来源。
    pub provenance: Vec<String>,
    /// 页面摘要文本。
    pub summary: String,
    /// 命中模式，区分结构化、BM25 或 fallback。
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

/// `QueryRelationMatch` 是 query 返回的关系视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryRelationMatch {
    /// 关系起点源码 ID。
    pub source_id: String,
    /// 关系终点源码 ID。
    pub target_id: String,
    /// 关系类型。
    pub relation_type: String,
    /// 关系证据。
    pub evidence: Vec<String>,
    /// 关系命中原因。
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
    /// 关系类型，例如 `CALLS` / `IMPORTS`。
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
    /// 该边是直接命中、下游调用链还是上游影响范围。
    #[serde(default)]
    pub traversal_modes: Vec<String>,
    /// graph 命中的原因。
    pub reasons: Vec<String>,
    /// graph 命中 provenance。
    pub provenance: Vec<String>,
}

/// `QueryProcessMatch` 是 query 返回的 process 视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryProcessMatch {
    /// process 稳定 ID。
    pub process_id: String,
    /// process 展示标签。
    pub label: String,
    /// process 类型。
    pub process_type: String,
    /// 用于展示的步骤符号名。
    pub steps: Vec<String>,
    /// 当前 query 直接命中或经 graph 扩展命中的 symbol IDs。
    pub matched_symbol_ids: Vec<String>,
    /// graph 命中的原因。
    pub reasons: Vec<String>,
    /// graph 命中 provenance。
    pub provenance: Vec<String>,
}

/// `QueryCommunityMatch` 是 query 返回的 community 视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryCommunityMatch {
    /// community 稳定 ID。
    pub community_id: String,
    /// community 标签。
    pub label: String,
    /// 内聚度。
    pub cohesion: f64,
    /// 成员数量。
    pub symbol_count: usize,
    /// 当前 query 直接命中或经 graph 扩展命中的 symbol IDs。
    pub matched_symbol_ids: Vec<String>,
    /// 用于展示的成员符号名。
    pub member_symbols: Vec<String>,
    /// graph 命中的原因。
    pub reasons: Vec<String>,
    /// graph 命中 provenance。
    pub provenance: Vec<String>,
}

/// `QueryReport` 是当前对 Agent 最友好的结构化返回。
#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    /// 用户查询原词。
    pub term: String,
    /// 命中的页面 ID 列表，便于快速判断覆盖面。
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
}

#[derive(Default)]
struct PageMatchState {
    reasons: BTreeSet<String>,
    provenance: BTreeSet<String>,
    match_mode: String,
}

#[derive(Default)]
struct RelatedMatchState {
    reasons: BTreeSet<String>,
}

struct SymbolMatchState {
    symbol_id: String,
    name: String,
    label: String,
    file_path: String,
    language: String,
    score: f64,
    reasons: BTreeSet<String>,
    page_ids: BTreeSet<String>,
    module_ids: BTreeSet<String>,
}

#[derive(Default)]
struct GraphEdgeMatchState {
    edge_id: String,
    edge_type: String,
    source_symbol_id: String,
    source_symbol: String,
    target_symbol_id: String,
    target_symbol: String,
    confidence: f64,
    reason: String,
    hop_distance: usize,
    traversal_modes: BTreeSet<String>,
    reasons: BTreeSet<String>,
    provenance: BTreeSet<String>,
}

#[derive(Default)]
struct ProcessMatchState {
    process_id: String,
    label: String,
    process_type: String,
    steps: BTreeSet<String>,
    matched_symbol_ids: BTreeSet<String>,
    reasons: BTreeSet<String>,
    provenance: BTreeSet<String>,
}

#[derive(Default)]
struct CommunityMatchState {
    community_id: String,
    label: String,
    cohesion: f64,
    symbol_count: usize,
    matched_symbol_ids: BTreeSet<String>,
    member_symbols: BTreeSet<String>,
    reasons: BTreeSet<String>,
    provenance: BTreeSet<String>,
}

/// 执行关键词查询。
/// 优先从 WikiState 构建查询索引，WikiState 丢失时从 metadata 重建。
/// 结构化索引是主命中来源，仅在没有结构命中时回退到 Markdown 内容匹配。
pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    let state = load_or_rebuild_state(repo_root)?;
    let needle = term.trim().to_lowercase();

    if needle.is_empty() {
        return Ok(QueryReport {
            term: term.to_string(),
            matched_pages: Vec::new(),
            matched_modules: Vec::new(),
            matched_sources: Vec::new(),
            matched_relations: Vec::new(),
            matched_symbols: Vec::new(),
            matched_symbol_edges: Vec::new(),
            matched_processes: Vec::new(),
            matched_communities: Vec::new(),
            matches: Vec::new(),
            provenance_summary: String::new(),
        });
    }

    let page_index = state
        .pages
        .iter()
        .map(|page| (page.page_id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let module_index = state
        .modules
        .iter()
        .map(|module| (module.id.clone(), module))
        .collect::<BTreeMap<_, _>>();
    let source_index = state
        .sources
        .iter()
        .map(|source| (source.source_id.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let source_path_index = state
        .sources
        .iter()
        .map(|source| (source.path.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let page_ids_by_module = build_page_ids_by_module(&state);
    let source_ids_by_module = build_source_ids_by_module(&state);

    let mut page_matches = BTreeMap::new();
    let mut module_matches = BTreeMap::new();
    let mut source_matches = BTreeMap::new();
    let mut relation_matches = BTreeMap::new();
    let mut symbol_matches = BTreeMap::new();
    let mut graph_edge_matches = BTreeMap::new();
    let mut process_matches = BTreeMap::new();
    let mut community_matches = BTreeMap::new();

    collect_fts_page_matches(repo_root, &needle, &page_index, &mut page_matches);
    collect_symbol_matches(
        repo_root,
        &needle,
        &page_index,
        &module_index,
        &source_path_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
        &mut symbol_matches,
    );
    collect_page_matches(&state, &needle, &mut page_matches);
    collect_module_matches(
        &state,
        &needle,
        &page_ids_by_module,
        &source_ids_by_module,
        &page_index,
        &source_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
    );
    collect_source_matches(
        &state,
        &needle,
        &page_index,
        &module_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
    );
    collect_relation_matches(
        &state,
        &needle,
        &page_index,
        &module_index,
        &source_index,
        &source_path_index,
        &page_ids_by_module,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
        &mut relation_matches,
    );
    collect_graph_matches(
        repo_root,
        &needle,
        &page_index,
        &module_index,
        &source_path_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
        &symbol_matches,
        &mut graph_edge_matches,
        &mut process_matches,
        &mut community_matches,
    );

    // 页面命中建立后，再补齐与这些页面相关联的模块和源码，保证返回结果自洽。
    let matched_page_ids = page_matches.keys().cloned().collect::<Vec<_>>();
    for page_id in &matched_page_ids {
        if let Some(page) = page_index.get(page_id) {
            for module_id in &page.module_ids {
                record_related_match(&mut module_matches, module_id, ["关联页面匹配"]);
            }

            for source_path in &page.source_paths {
                if let Some(source) = source_path_index.get(source_path) {
                    record_related_match(&mut source_matches, &source.source_id, ["关联页面匹配"]);
                }
            }
        }
    }

    let had_structural_match = !page_matches.is_empty()
        || !module_matches.is_empty()
        || !source_matches.is_empty()
        || !relation_matches.is_empty()
        || !symbol_matches.is_empty()
        || !graph_edge_matches.is_empty()
        || !process_matches.is_empty()
        || !community_matches.is_empty();

    if !had_structural_match {
        collect_markdown_fallback_matches(
            repo_root,
            &state,
            &needle,
            &source_path_index,
            &mut page_matches,
            &mut module_matches,
            &mut source_matches,
        );
    }

    let provenance_summary = build_provenance_summary(
        &page_matches,
        &module_matches,
        &source_matches,
        &relation_matches,
        &symbol_matches,
        &graph_edge_matches,
        &process_matches,
        &community_matches,
    );
    let matched_symbols = finalize_symbol_matches(&symbol_matches);
    let matched_symbols_by_file = build_symbols_by_file(&matched_symbols);

    Ok(QueryReport {
        term: term.to_string(),
        matched_pages: finalize_page_paths(&page_matches, &page_index),
        matched_modules: finalize_module_matches(&module_matches, &module_index),
        matched_sources: finalize_source_matches(&source_matches, &source_index),
        matched_relations: finalize_relation_matches(&relation_matches, &state.relations),
        matched_symbols,
        matched_symbol_edges: finalize_graph_edge_matches(&graph_edge_matches),
        matched_processes: finalize_process_matches(&process_matches),
        matched_communities: finalize_community_matches(&community_matches),
        matches: finalize_page_matches(
            &page_matches,
            &page_index,
            &state,
            &module_index,
            &matched_symbols_by_file,
        ),
        provenance_summary,
    })
}

fn collect_page_matches(
    state: &WikiState,
    needle: &str,
    page_matches: &mut BTreeMap<String, PageMatchState>,
) {
    for page in &state.pages {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&page.title, needle) {
            reasons.push("页面标题匹配");
        }

        if contains_case_insensitive(&page.path, needle) {
            reasons.push("页面路径匹配");
        }

        if contains_case_insensitive(&page.page_type, needle) {
            reasons.push("页面类型匹配");
        }

        if contains_case_insensitive(&page.summary, needle) {
            reasons.push("页面摘要匹配");
        }

        if !reasons.is_empty() {
            record_page_match(
                page_matches,
                &page.page_id,
                reasons,
                page.provenance.clone(),
                "structure",
            );
        }
    }
}

fn collect_fts_page_matches(
    repo_root: &Path,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
) {
    let hits = match sqlite_store::search_pages_fts(repo_root, needle, 16) {
        Ok(hits) => hits,
        Err(_) => return,
    };

    for hit in hits {
        if !page_index.contains_key(&hit.page_id) {
            continue;
        }

        let mut reasons = Vec::new();
        if contains_case_insensitive(&hit.title, needle) {
            reasons.push("FTS 标题匹配");
        }
        if contains_case_insensitive(&hit.path, needle) {
            reasons.push("FTS 路径匹配");
        }
        if reasons.is_empty() {
            reasons.push("FTS BM25 匹配");
        }

        record_page_match(
            page_matches,
            &hit.page_id,
            reasons,
            [format!("fts:bm25:{:.4}", hit.score)],
            "fts",
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_symbol_matches(
    repo_root: &Path,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    symbol_matches: &mut BTreeMap<String, SymbolMatchState>,
) {
    let hits = match sqlite_store::search_symbols_fts(repo_root, needle, 24) {
        Ok(hits) => hits,
        Err(_) => return,
    };

    for hit in hits {
        let reasons = symbol_match_reasons(&hit, needle);
        let source = source_path_index.get(&hit.file_path).copied();
        let page_ids = source
            .map(|source| source.page_ids.clone())
            .unwrap_or_default();
        let module_ids = source
            .map(|source| source.module_ids.clone())
            .unwrap_or_default();

        symbol_matches.insert(
            hit.symbol_id.clone(),
            SymbolMatchState {
                symbol_id: hit.symbol_id.clone(),
                name: hit.name.clone(),
                label: hit.label.clone(),
                file_path: hit.file_path.clone(),
                language: hit.language.clone(),
                score: hit.score,
                reasons: reasons.iter().cloned().map(str::to_string).collect(),
                page_ids: page_ids.iter().cloned().collect(),
                module_ids: module_ids.iter().cloned().collect(),
            },
        );

        if let Some(source) = source {
            record_related_match(source_matches, &source.source_id, ["符号匹配"]);
        }

        for page_id in &page_ids {
            if page_index.contains_key(page_id) {
                record_page_match(
                    page_matches,
                    page_id,
                    ["关联符号匹配"],
                    [
                        format!("symbol:{}", hit.name),
                        format!("source:{}", hit.file_path),
                        format!("symbol_fts:bm25:{:.4}", hit.score),
                    ],
                    "structure",
                );
            }
        }

        for module_id in &module_ids {
            if module_index.contains_key(module_id) {
                record_related_match(module_matches, module_id, ["关联符号匹配"]);
            }
        }
    }
}

fn symbol_match_reasons(hit: &sqlite_store::FtsSymbolHit, needle: &str) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if contains_case_insensitive(&hit.name, needle) {
        reasons.push("符号名称匹配");
    }
    if contains_case_insensitive(&hit.label, needle) {
        reasons.push("符号类型匹配");
    }
    if contains_case_insensitive(&hit.file_path, needle) {
        reasons.push("符号路径匹配");
    }
    if reasons.is_empty() {
        reasons.push("符号 FTS BM25 匹配");
    }
    reasons
}

fn graph_expansion_seed_ids(
    needle: &str,
    symbol_matches: &BTreeMap<String, SymbolMatchState>,
) -> Vec<String> {
    let mut candidates = symbol_matches
        .values()
        .map(|state| {
            let exact_name = state.name.to_lowercase() == needle;
            let name_match = state.reasons.contains("符号名称匹配");
            let priority = if exact_name {
                0
            } else if name_match {
                1
            } else {
                2
            };

            (
                priority,
                state.score,
                state.file_path.clone(),
                state.symbol_id.clone(),
            )
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| {
                left.1
                    .partial_cmp(&right.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then(left.2.cmp(&right.2))
            .then(left.3.cmp(&right.3))
    });

    candidates
        .into_iter()
        .map(|(_, _, _, symbol_id)| symbol_id)
        .take(4)
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn collect_graph_matches(
    repo_root: &Path,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    symbol_matches: &BTreeMap<String, SymbolMatchState>,
    graph_edge_matches: &mut BTreeMap<String, GraphEdgeMatchState>,
    process_matches: &mut BTreeMap<String, ProcessMatchState>,
    community_matches: &mut BTreeMap<String, CommunityMatchState>,
) {
    let symbols = match sqlite_store::list_symbols(repo_root) {
        Ok(symbols) => symbols,
        Err(_) => return,
    };
    if symbols.is_empty() {
        return;
    }

    let symbols_by_id = symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let direct_matched_symbol_ids = symbol_matches.keys().cloned().collect::<BTreeSet<_>>();
    let graph_seed_symbol_ids = graph_expansion_seed_ids(needle, symbol_matches);
    let traced_call_edges =
        sqlite_store::trace_call_edges(repo_root, &graph_seed_symbol_ids, 3, 48)
            .unwrap_or_default();
    let mut expanded_symbol_ids = direct_matched_symbol_ids.clone();

    collect_graph_edge_matches(
        repo_root,
        needle,
        &symbols_by_id,
        &direct_matched_symbol_ids,
        &traced_call_edges,
        page_index,
        module_index,
        source_path_index,
        page_matches,
        module_matches,
        source_matches,
        graph_edge_matches,
        &mut expanded_symbol_ids,
    );
    collect_process_graph_matches(
        repo_root,
        needle,
        &symbols_by_id,
        &direct_matched_symbol_ids,
        &expanded_symbol_ids,
        page_index,
        module_index,
        source_path_index,
        page_matches,
        module_matches,
        source_matches,
        process_matches,
    );
    collect_community_graph_matches(
        repo_root,
        needle,
        &symbols_by_id,
        &direct_matched_symbol_ids,
        &expanded_symbol_ids,
        page_index,
        module_index,
        source_path_index,
        page_matches,
        module_matches,
        source_matches,
        community_matches,
    );
}

#[allow(clippy::too_many_arguments)]
fn collect_graph_edge_matches(
    repo_root: &Path,
    needle: &str,
    symbols_by_id: &BTreeMap<String, &SymbolNode>,
    matched_symbol_ids: &BTreeSet<String>,
    traced_call_edges: &[sqlite_store::GraphTraceEdgeHit],
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    graph_edge_matches: &mut BTreeMap<String, GraphEdgeMatchState>,
    expanded_symbol_ids: &mut BTreeSet<String>,
) {
    let edges = match sqlite_store::list_edges(repo_root) {
        Ok(edges) => edges,
        Err(_) => return,
    };

    for edge in edges {
        let Some(source_symbol) = symbols_by_id.get(&edge.source_id).copied() else {
            continue;
        };
        let Some(target_symbol) = symbols_by_id.get(&edge.target_id).copied() else {
            continue;
        };
        let source_matched = matched_symbol_ids.contains(&edge.source_id);
        let target_matched = matched_symbol_ids.contains(&edge.target_id);
        let mut reasons = Vec::new();
        let mut provenance = Vec::new();

        if source_matched {
            reasons.push("命中符号的出边");
            provenance.push("graph-direct:outbound".to_string());
        }
        if target_matched {
            reasons.push("命中符号的入边");
            provenance.push("graph-direct:inbound".to_string());
        }
        if contains_case_insensitive(&edge.edge_type, needle) {
            reasons.push("图关系类型匹配");
            provenance.push("graph-filter:type".to_string());
        }
        if contains_case_insensitive(&edge.reason, needle) {
            reasons.push("图关系原因匹配");
            provenance.push("graph-filter:reason".to_string());
        }
        if contains_case_insensitive(&source_symbol.name, needle)
            || contains_case_insensitive(&target_symbol.name, needle)
        {
            reasons.push("图关系端点匹配");
            provenance.push("graph-filter:endpoint".to_string());
        }

        if reasons.is_empty() {
            continue;
        }

        record_graph_edge_match(
            graph_edge_matches,
            &edge.edge_id,
            &edge.edge_type,
            &edge.source_id,
            &source_symbol.name,
            &edge.target_id,
            &target_symbol.name,
            edge.confidence,
            &edge.reason,
            1,
            Some("direct"),
            reasons.into_iter().map(str::to_string),
            provenance,
        );

        record_graph_symbol_context(
            source_symbol,
            page_index,
            module_index,
            source_path_index,
            page_matches,
            module_matches,
            source_matches,
            "关联图关系匹配",
            &[
                format!("graph-edge:{}", edge.edge_type),
                format!("source:{}", source_symbol.file_path),
            ],
        );
        record_graph_symbol_context(
            target_symbol,
            page_index,
            module_index,
            source_path_index,
            page_matches,
            module_matches,
            source_matches,
            "关联图关系匹配",
            &[
                format!("graph-edge:{}", edge.edge_type),
                format!("source:{}", target_symbol.file_path),
            ],
        );
    }

    for traced in traced_call_edges {
        let Some(source_symbol) = symbols_by_id.get(&traced.source_id).copied() else {
            continue;
        };
        let Some(target_symbol) = symbols_by_id.get(&traced.target_id).copied() else {
            continue;
        };

        expanded_symbol_ids.insert(traced.source_id.clone());
        expanded_symbol_ids.insert(traced.target_id.clone());

        let (reason, mode, provenance, context_reason) = match traced.traversal_direction.as_str() {
            "inbound" => (
                "命中 symbol 的上游影响范围",
                "impact-inbound",
                format!("graph-cte:inbound:depth={}", traced.hop_distance),
                "关联影响范围匹配",
            ),
            _ => (
                "命中 symbol 的下游调用链",
                "call-chain-outbound",
                format!("graph-cte:outbound:depth={}", traced.hop_distance),
                "关联调用链匹配",
            ),
        };

        record_graph_edge_match(
            graph_edge_matches,
            &traced.edge_id,
            "CALLS",
            &traced.source_id,
            &source_symbol.name,
            &traced.target_id,
            &target_symbol.name,
            traced.confidence,
            &traced.reason,
            traced.hop_distance,
            Some(mode),
            [reason.to_string()],
            [provenance],
        );

        record_graph_symbol_context(
            source_symbol,
            page_index,
            module_index,
            source_path_index,
            page_matches,
            module_matches,
            source_matches,
            context_reason,
            &[
                format!("graph-cte:{}", traced.traversal_direction),
                format!("source:{}", source_symbol.file_path),
            ],
        );
        record_graph_symbol_context(
            target_symbol,
            page_index,
            module_index,
            source_path_index,
            page_matches,
            module_matches,
            source_matches,
            context_reason,
            &[
                format!("graph-cte:{}", traced.traversal_direction),
                format!("source:{}", target_symbol.file_path),
            ],
        );
    }
}

fn record_graph_edge_match<I, P>(
    graph_edge_matches: &mut BTreeMap<String, GraphEdgeMatchState>,
    edge_id: &str,
    edge_type: &str,
    source_symbol_id: &str,
    source_symbol: &str,
    target_symbol_id: &str,
    target_symbol: &str,
    confidence: f64,
    reason: &str,
    hop_distance: usize,
    traversal_mode: Option<&str>,
    reasons: I,
    provenance: P,
) where
    I: IntoIterator<Item = String>,
    P: IntoIterator<Item = String>,
{
    let state = graph_edge_matches
        .entry(edge_id.to_string())
        .or_insert_with(|| GraphEdgeMatchState {
            edge_id: edge_id.to_string(),
            edge_type: edge_type.to_string(),
            source_symbol_id: source_symbol_id.to_string(),
            source_symbol: source_symbol.to_string(),
            target_symbol_id: target_symbol_id.to_string(),
            target_symbol: target_symbol.to_string(),
            confidence,
            reason: reason.to_string(),
            hop_distance,
            ..GraphEdgeMatchState::default()
        });

    if state.hop_distance == 0 || hop_distance < state.hop_distance {
        state.hop_distance = hop_distance;
    }
    if let Some(mode) = traversal_mode {
        state.traversal_modes.insert(mode.to_string());
    }
    state
        .reasons
        .extend(reasons.into_iter().filter(|item| !item.is_empty()));
    state
        .provenance
        .extend(provenance.into_iter().filter(|item| !item.is_empty()));
}

#[allow(clippy::too_many_arguments)]
fn collect_process_graph_matches(
    repo_root: &Path,
    needle: &str,
    symbols_by_id: &BTreeMap<String, &SymbolNode>,
    direct_matched_symbol_ids: &BTreeSet<String>,
    expanded_symbol_ids: &BTreeSet<String>,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    process_matches: &mut BTreeMap<String, ProcessMatchState>,
) {
    let processes = match sqlite_store::list_processes(repo_root) {
        Ok(processes) => processes,
        Err(_) => return,
    };
    let process_steps = match sqlite_store::list_process_steps(repo_root) {
        Ok(process_steps) => process_steps,
        Err(_) => return,
    };
    let steps_by_process =
        process_steps
            .iter()
            .fold(BTreeMap::<String, Vec<_>>::new(), |mut acc, step| {
                acc.entry(step.process_id.clone()).or_default().push(step);
                acc
            });

    for process in processes {
        let steps = steps_by_process
            .get(&process.process_id)
            .cloned()
            .unwrap_or_default();
        let step_symbols = steps
            .iter()
            .filter_map(|step| symbols_by_id.get(&step.symbol_id).copied())
            .collect::<Vec<_>>();
        let direct_hits = step_symbols
            .iter()
            .filter(|symbol| direct_matched_symbol_ids.contains(&symbol.symbol_id))
            .map(|symbol| symbol.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        let expanded_hits = step_symbols
            .iter()
            .filter(|symbol| expanded_symbol_ids.contains(&symbol.symbol_id))
            .map(|symbol| symbol.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        let step_name_match = step_symbols
            .iter()
            .any(|symbol| contains_case_insensitive(&symbol.name, needle));
        let mut reasons = Vec::new();
        let mut provenance = Vec::new();

        if !direct_hits.is_empty() {
            reasons.push("命中符号参与流程");
            provenance.push("process-trace:matched-symbol".to_string());
        } else if !expanded_hits.is_empty() {
            reasons.push("扩展调用链参与流程");
            provenance.push("process-trace:graph-expansion".to_string());
        }
        if contains_case_insensitive(&process.label, needle) {
            reasons.push("流程标签匹配");
            provenance.push("process-trace:label".to_string());
        }
        if contains_case_insensitive(&process.process_type, needle) {
            reasons.push("流程类型匹配");
            provenance.push("process-trace:type".to_string());
        }
        if step_name_match {
            reasons.push("流程步骤匹配");
            provenance.push("process-trace:step".to_string());
        }

        if reasons.is_empty() {
            continue;
        }

        let state = process_matches
            .entry(process.process_id.clone())
            .or_insert_with(|| ProcessMatchState {
                process_id: process.process_id.clone(),
                label: process.label.clone(),
                process_type: process.process_type.clone(),
                ..ProcessMatchState::default()
            });
        state
            .steps
            .extend(step_symbols.iter().map(|symbol| symbol.name.clone()));
        state.matched_symbol_ids.extend(expanded_hits);
        state
            .reasons
            .extend(reasons.into_iter().map(str::to_string));
        state.provenance.extend(provenance);

        for symbol in step_symbols.into_iter().take(8) {
            record_graph_symbol_context(
                symbol,
                page_index,
                module_index,
                source_path_index,
                page_matches,
                module_matches,
                source_matches,
                "关联流程匹配",
                &[
                    format!("process:{}", process.process_id),
                    format!("source:{}", symbol.file_path),
                ],
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_community_graph_matches(
    repo_root: &Path,
    needle: &str,
    symbols_by_id: &BTreeMap<String, &SymbolNode>,
    direct_matched_symbol_ids: &BTreeSet<String>,
    expanded_symbol_ids: &BTreeSet<String>,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    community_matches: &mut BTreeMap<String, CommunityMatchState>,
) {
    let communities = match sqlite_store::list_communities(repo_root) {
        Ok(communities) => communities,
        Err(_) => return,
    };
    let community_members = match sqlite_store::list_community_members(repo_root) {
        Ok(community_members) => community_members,
        Err(_) => return,
    };
    let members_by_community =
        community_members
            .iter()
            .fold(BTreeMap::<String, Vec<String>>::new(), |mut acc, member| {
                acc.entry(member.community_id.clone())
                    .or_default()
                    .push(member.symbol_id.clone());
                acc
            });

    for community in communities {
        let member_ids = members_by_community
            .get(&community.community_id)
            .cloned()
            .unwrap_or_default();
        let member_symbols = member_ids
            .iter()
            .filter_map(|symbol_id| symbols_by_id.get(symbol_id).copied())
            .collect::<Vec<_>>();
        let direct_hits = member_symbols
            .iter()
            .filter(|symbol| direct_matched_symbol_ids.contains(&symbol.symbol_id))
            .map(|symbol| symbol.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        let expanded_hits = member_symbols
            .iter()
            .filter(|symbol| expanded_symbol_ids.contains(&symbol.symbol_id))
            .map(|symbol| symbol.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        let member_name_match = member_symbols
            .iter()
            .any(|symbol| contains_case_insensitive(&symbol.name, needle));
        let mut reasons = Vec::new();
        let mut provenance = Vec::new();

        if !direct_hits.is_empty() {
            reasons.push("命中符号属于社区");
            provenance.push("community-membership:matched-symbol".to_string());
        } else if !expanded_hits.is_empty() {
            reasons.push("扩展调用链属于社区");
            provenance.push("community-membership:graph-expansion".to_string());
        }
        if contains_case_insensitive(&community.label, needle) {
            reasons.push("社区标签匹配");
            provenance.push("community-membership:label".to_string());
        }
        if member_name_match {
            reasons.push("社区成员匹配");
            provenance.push("community-membership:member".to_string());
        }

        if reasons.is_empty() {
            continue;
        }

        let state = community_matches
            .entry(community.community_id.clone())
            .or_insert_with(|| CommunityMatchState {
                community_id: community.community_id.clone(),
                label: community.label.clone(),
                cohesion: community.cohesion,
                symbol_count: community.symbol_count,
                ..CommunityMatchState::default()
            });
        state.matched_symbol_ids.extend(expanded_hits);
        state
            .member_symbols
            .extend(member_symbols.iter().map(|symbol| symbol.name.clone()));
        state
            .reasons
            .extend(reasons.into_iter().map(str::to_string));
        state.provenance.extend(provenance);

        for symbol in member_symbols.into_iter().take(8) {
            record_graph_symbol_context(
                symbol,
                page_index,
                module_index,
                source_path_index,
                page_matches,
                module_matches,
                source_matches,
                "关联社区匹配",
                &[
                    format!("community:{}", community.community_id),
                    format!("source:{}", symbol.file_path),
                ],
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_module_matches(
    state: &WikiState,
    needle: &str,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    source_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_index: &BTreeMap<String, &WikiPageState>,
    source_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for module in &state.modules {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&module.name, needle) {
            reasons.push("模块名称匹配");
        }

        if contains_case_insensitive(&module.kind, needle) {
            reasons.push("模块类型匹配");
        }

        if module
            .root_paths
            .iter()
            .any(|path| contains_case_insensitive(path, needle))
        {
            reasons.push("模块路径匹配");
        }

        if module
            .tags
            .iter()
            .any(|tag| contains_case_insensitive(tag, needle))
        {
            reasons.push("模块标签匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(module_matches, &module.id, reasons.iter().copied());

        if let Some(page_ids) = page_ids_by_module.get(&module.id) {
            for page_id in page_ids {
                if page_index.contains_key(page_id) {
                    record_page_match(
                        page_matches,
                        page_id,
                        ["关联模块匹配"],
                        [format!("module:{}", module.id)],
                        "structure",
                    );
                }
            }
        }

        if let Some(source_ids) = source_ids_by_module.get(&module.id) {
            for source_id in source_ids {
                if source_index.contains_key(source_id) {
                    record_related_match(source_matches, source_id, ["关联模块匹配"]);
                }
            }
        }
    }
}

fn collect_source_matches(
    state: &WikiState,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for source in &state.sources {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&source.path, needle) {
            reasons.push("源码路径匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(source_matches, &source.source_id, reasons.iter().copied());

        for page_id in &source.page_ids {
            if page_index.contains_key(page_id) {
                record_page_match(
                    page_matches,
                    page_id,
                    ["关联源码匹配"],
                    [format!("source:{}", source.path)],
                    "structure",
                );
            }
        }

        for module_id in &source.module_ids {
            if module_index.contains_key(module_id) {
                record_related_match(module_matches, module_id, ["关联源码匹配"]);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_relation_matches(
    state: &WikiState,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_index: &BTreeMap<String, &SourceState>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    relation_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for relation in &state.relations {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&relation.relation_type, needle) {
            reasons.push("关系类型匹配");
        }

        if relation
            .evidence
            .iter()
            .any(|evidence| contains_case_insensitive(evidence, needle))
        {
            reasons.push("关系证据匹配");
        }

        let source_label =
            relation_endpoint_label(&relation.source_id, page_index, module_index, source_index);
        let target_label =
            relation_endpoint_label(&relation.target_id, page_index, module_index, source_index);

        if contains_case_insensitive(&source_label, needle)
            || contains_case_insensitive(&target_label, needle)
        {
            reasons.push("关系端点匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(
            relation_matches,
            &relation_identity(relation),
            reasons.iter().copied(),
        );
        record_relation_endpoint_match(
            relation,
            &relation.source_id,
            page_index,
            module_index,
            page_ids_by_module,
            page_matches,
            module_matches,
            source_matches,
        );
        record_relation_endpoint_match(
            relation,
            &relation.target_id,
            page_index,
            module_index,
            page_ids_by_module,
            page_matches,
            module_matches,
            source_matches,
        );

        for evidence_path in &relation.evidence {
            if let Some(source) = source_path_index.get(evidence_path) {
                record_related_match(source_matches, &source.source_id, ["关系证据匹配"]);

                for page_id in &source.page_ids {
                    if page_index.contains_key(page_id) {
                        record_page_match(
                            page_matches,
                            page_id,
                            ["关系证据匹配"],
                            [
                                format!("relation:{}", relation.relation_type),
                                format!("source:{}", source.path),
                            ],
                            "structure",
                        );
                    }
                }
            }
        }
    }
}

fn collect_markdown_fallback_matches(
    repo_root: &Path,
    state: &WikiState,
    needle: &str,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for page in &state.pages {
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = fs::read_to_string(&page_path).unwrap_or_default();
        if !contains_case_insensitive(&content, needle) {
            continue;
        }

        record_page_match(
            page_matches,
            &page.page_id,
            ["Markdown 内容匹配"],
            page.provenance.clone(),
            "fallback_markdown",
        );

        for module_id in &page.module_ids {
            record_related_match(module_matches, module_id, ["Markdown 回退关联"]);
        }

        for source_path in &page.source_paths {
            if let Some(source) = source_path_index.get(source_path) {
                record_related_match(source_matches, &source.source_id, ["Markdown 回退关联"]);
            }
        }
    }
}

fn record_relation_endpoint_match(
    relation: &WikiRelation,
    endpoint_id: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    if page_index.contains_key(endpoint_id) {
        record_page_match(
            page_matches,
            endpoint_id,
            ["关系端点匹配"],
            [format!("relation:{}", relation.relation_type)],
            "structure",
        );
        return;
    }

    if module_index.contains_key(endpoint_id) {
        record_related_match(module_matches, endpoint_id, ["关系端点匹配"]);

        if let Some(page_ids) = page_ids_by_module.get(endpoint_id) {
            for page_id in page_ids {
                if page_index.contains_key(page_id) {
                    record_page_match(
                        page_matches,
                        page_id,
                        ["关系端点匹配"],
                        [format!("relation:{}", relation.relation_type)],
                        "structure",
                    );
                }
            }
        }
        return;
    }

    record_related_match(source_matches, endpoint_id, ["关系端点匹配"]);
}

fn relation_endpoint_label(
    endpoint_id: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_index: &BTreeMap<String, &SourceState>,
) -> String {
    if let Some(page) = page_index.get(endpoint_id) {
        return format!("{} {}", page.title, page.path);
    }

    if let Some(module) = module_index.get(endpoint_id) {
        return format!("{} {}", module.name, module.root_paths.join(" "));
    }

    if let Some(source) = source_index.get(endpoint_id) {
        return source.path.clone();
    }

    endpoint_id.to_string()
}

fn record_page_match<I, P>(
    page_matches: &mut BTreeMap<String, PageMatchState>,
    page_id: &str,
    reasons: I,
    provenance: P,
    match_mode: &str,
) where
    I: IntoIterator,
    I::Item: AsRef<str>,
    P: IntoIterator,
    P::Item: Into<String>,
{
    let state = page_matches.entry(page_id.to_string()).or_default();

    for reason in reasons {
        state.reasons.insert(reason.as_ref().to_string());
    }

    for provenance_item in provenance {
        state.provenance.insert(provenance_item.into());
    }

    state.match_mode = merge_match_mode(&state.match_mode, match_mode);
}

fn record_related_match<I>(matches: &mut BTreeMap<String, RelatedMatchState>, id: &str, reasons: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let state = matches.entry(id.to_string()).or_default();

    for reason in reasons {
        state.reasons.insert(reason.as_ref().to_string());
    }
}

fn build_page_ids_by_module(state: &WikiState) -> BTreeMap<String, Vec<String>> {
    let mut page_ids_by_module = BTreeMap::new();

    for page in &state.pages {
        for module_id in &page.module_ids {
            page_ids_by_module
                .entry(module_id.clone())
                .or_insert_with(Vec::new)
                .push(page.page_id.clone());
        }
    }

    page_ids_by_module
}

fn build_source_ids_by_module(state: &WikiState) -> BTreeMap<String, Vec<String>> {
    let mut source_ids_by_module = BTreeMap::new();

    for source in &state.sources {
        for module_id in &source.module_ids {
            source_ids_by_module
                .entry(module_id.clone())
                .or_insert_with(Vec::new)
                .push(source.source_id.clone());
        }
    }

    source_ids_by_module
}

fn build_provenance_summary(
    page_matches: &BTreeMap<String, PageMatchState>,
    module_matches: &BTreeMap<String, RelatedMatchState>,
    source_matches: &BTreeMap<String, RelatedMatchState>,
    relation_matches: &BTreeMap<String, RelatedMatchState>,
    symbol_matches: &BTreeMap<String, SymbolMatchState>,
    graph_edge_matches: &BTreeMap<String, GraphEdgeMatchState>,
    process_matches: &BTreeMap<String, ProcessMatchState>,
    community_matches: &BTreeMap<String, CommunityMatchState>,
) -> String {
    let mut parts = Vec::new();

    let structural_pages = page_matches
        .values()
        .filter(|s| s.match_mode == "structure" || s.match_mode == "fts+structure")
        .count();
    let fts_pages = page_matches
        .values()
        .filter(|s| s.match_mode == "fts" || s.match_mode == "fts+structure")
        .count();
    let fallback_pages = page_matches
        .values()
        .filter(|s| s.match_mode == "fallback_markdown")
        .count();

    if fts_pages > 0 {
        parts.push(format!("FTS 命中 {} 页", fts_pages));
    }
    if structural_pages > 0 {
        parts.push(format!("结构命中 {} 页", structural_pages));
    }
    if fallback_pages > 0 {
        parts.push(format!("Markdown 回退命中 {} 页", fallback_pages));
    }
    if !module_matches.is_empty() {
        parts.push(format!("关联 {} 模块", module_matches.len()));
    }
    if !source_matches.is_empty() {
        parts.push(format!("关联 {} 源码", source_matches.len()));
    }
    if !relation_matches.is_empty() {
        parts.push(format!("关联 {} 关系", relation_matches.len()));
    }
    if !symbol_matches.is_empty() {
        parts.push(format!("命中 {} 符号", symbol_matches.len()));
    }
    if !graph_edge_matches.is_empty() {
        parts.push(format!("扩展 {} 图边", graph_edge_matches.len()));
    }
    if !process_matches.is_empty() {
        parts.push(format!("扩展 {} 流程", process_matches.len()));
    }
    if !community_matches.is_empty() {
        parts.push(format!("扩展 {} 社区", community_matches.len()));
    }

    parts.join("、")
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
        .map(|m| {
            let tags = if m.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", m.tags.join(", "))
            };
            format!("{} ({}){}", m.name, m.kind, tags)
        })
        .collect();

    let key_source_paths = page.source_paths.iter().take(8).cloned().collect();

    let relation_evidence = state
        .relations
        .iter()
        .filter(|r| {
            page.module_ids.contains(&r.source_id) || page.module_ids.contains(&r.target_id)
        })
        .flat_map(|r| {
            std::iter::once(format!(
                "{} -[{}]-> {}",
                r.source_id, r.relation_type, r.target_id
            ))
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

fn finalize_page_paths(
    page_matches: &BTreeMap<String, PageMatchState>,
    page_index: &BTreeMap<String, &WikiPageState>,
) -> Vec<String> {
    let mut pages = page_matches
        .keys()
        .filter_map(|page_id| page_index.get(page_id).map(|page| page.path.clone()))
        .collect::<Vec<_>>();
    pages.sort();
    pages
}

fn finalize_page_matches(
    page_matches: &BTreeMap<String, PageMatchState>,
    page_index: &BTreeMap<String, &WikiPageState>,
    state: &WikiState,
    module_index: &BTreeMap<String, &ModuleNode>,
    matched_symbols_by_file: &BTreeMap<String, Vec<String>>,
) -> Vec<QueryMatch> {
    let mut matches = page_matches
        .iter()
        .filter_map(|(page_id, match_state)| {
            let page = page_index.get(page_id)?;
            let reasons = match_state.reasons.iter().cloned().collect::<Vec<_>>();
            Some(QueryMatch {
                page_id: page.page_id.clone(),
                title: page.title.clone(),
                path: page.path.clone(),
                item_type: page.page_type.clone(),
                module_ids: page.module_ids.clone(),
                source_files: page.source_paths.clone(),
                reasons: reasons.clone(),
                provenance: match_state.provenance.iter().cloned().collect(),
                summary: reasons.join("、"),
                match_mode: match_state.match_mode.clone(),
                context_pack: build_context_pack(
                    page,
                    state,
                    module_index,
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

fn finalize_module_matches(
    module_matches: &BTreeMap<String, RelatedMatchState>,
    module_index: &BTreeMap<String, &ModuleNode>,
) -> Vec<QueryModuleMatch> {
    let mut matches = module_matches
        .iter()
        .filter_map(|(module_id, state)| {
            let module = module_index.get(module_id)?;
            Some(QueryModuleMatch {
                module_id: module.id.clone(),
                name: module.name.clone(),
                kind: module.kind.clone(),
                root_paths: module.root_paths.clone(),
                tags: module.tags.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.root_paths
            .first()
            .cmp(&right.root_paths.first())
            .then(left.module_id.cmp(&right.module_id))
    });
    matches
}

fn finalize_source_matches(
    source_matches: &BTreeMap<String, RelatedMatchState>,
    source_index: &BTreeMap<String, &SourceState>,
) -> Vec<QuerySourceMatch> {
    let mut matches = source_matches
        .iter()
        .filter_map(|(source_id, state)| {
            let source = source_index.get(source_id)?;
            Some(QuerySourceMatch {
                source_id: source.source_id.clone(),
                path: source.path.clone(),
                module_ids: source.module_ids.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| left.path.cmp(&right.path));
    matches
}

fn finalize_relation_matches(
    relation_matches: &BTreeMap<String, RelatedMatchState>,
    relations: &[WikiRelation],
) -> Vec<QueryRelationMatch> {
    let relation_index = relations
        .iter()
        .map(|relation| (relation_identity(relation), relation))
        .collect::<BTreeMap<_, _>>();
    let mut matches = relation_matches
        .iter()
        .filter_map(|(relation_id, state)| {
            let relation = relation_index.get(relation_id)?;
            Some(QueryRelationMatch {
                source_id: relation.source_id.clone(),
                target_id: relation.target_id.clone(),
                relation_type: relation.relation_type.clone(),
                evidence: relation.evidence.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.source_id
            .cmp(&right.source_id)
            .then(left.target_id.cmp(&right.target_id))
            .then(left.relation_type.cmp(&right.relation_type))
    });
    matches
}

fn finalize_symbol_matches(
    symbol_matches: &BTreeMap<String, SymbolMatchState>,
) -> Vec<QuerySymbolMatch> {
    let mut matches = symbol_matches
        .values()
        .map(|state| QuerySymbolMatch {
            symbol_id: state.symbol_id.clone(),
            name: state.name.clone(),
            label: state.label.clone(),
            file_path: state.file_path.clone(),
            language: state.language.clone(),
            page_ids: state.page_ids.iter().cloned().collect(),
            module_ids: state.module_ids.iter().cloned().collect(),
            reasons: state.reasons.iter().cloned().collect(),
            score: state.score,
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.name.cmp(&right.name))
            .then(left.label.cmp(&right.label))
            .then(left.symbol_id.cmp(&right.symbol_id))
    });
    matches
}

fn finalize_graph_edge_matches(
    graph_edge_matches: &BTreeMap<String, GraphEdgeMatchState>,
) -> Vec<QueryGraphEdgeMatch> {
    let mut matches = graph_edge_matches
        .values()
        .map(|state| QueryGraphEdgeMatch {
            edge_id: state.edge_id.clone(),
            edge_type: state.edge_type.clone(),
            source_symbol_id: state.source_symbol_id.clone(),
            source_symbol: state.source_symbol.clone(),
            target_symbol_id: state.target_symbol_id.clone(),
            target_symbol: state.target_symbol.clone(),
            confidence: state.confidence,
            reason: state.reason.clone(),
            hop_distance: state.hop_distance,
            traversal_modes: state.traversal_modes.iter().cloned().collect(),
            reasons: state.reasons.iter().cloned().collect(),
            provenance: state.provenance.iter().cloned().collect(),
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.hop_distance
            .cmp(&right.hop_distance)
            .then(left.source_symbol.cmp(&right.source_symbol))
            .then(left.target_symbol.cmp(&right.target_symbol))
            .then(left.edge_type.cmp(&right.edge_type))
            .then(left.edge_id.cmp(&right.edge_id))
    });
    matches
}

fn finalize_process_matches(
    process_matches: &BTreeMap<String, ProcessMatchState>,
) -> Vec<QueryProcessMatch> {
    let mut matches = process_matches
        .values()
        .map(|state| QueryProcessMatch {
            process_id: state.process_id.clone(),
            label: state.label.clone(),
            process_type: state.process_type.clone(),
            steps: state.steps.iter().cloned().take(12).collect(),
            matched_symbol_ids: state.matched_symbol_ids.iter().cloned().collect(),
            reasons: state.reasons.iter().cloned().collect(),
            provenance: state.provenance.iter().cloned().collect(),
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.label
            .cmp(&right.label)
            .then(left.process_id.cmp(&right.process_id))
    });
    matches
}

fn finalize_community_matches(
    community_matches: &BTreeMap<String, CommunityMatchState>,
) -> Vec<QueryCommunityMatch> {
    let mut matches = community_matches
        .values()
        .map(|state| QueryCommunityMatch {
            community_id: state.community_id.clone(),
            label: state.label.clone(),
            cohesion: state.cohesion,
            symbol_count: state.symbol_count,
            matched_symbol_ids: state.matched_symbol_ids.iter().cloned().collect(),
            member_symbols: state.member_symbols.iter().cloned().take(12).collect(),
            reasons: state.reasons.iter().cloned().collect(),
            provenance: state.provenance.iter().cloned().collect(),
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.label
            .cmp(&right.label)
            .then(left.community_id.cmp(&right.community_id))
    });
    matches
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

fn relation_identity(relation: &WikiRelation) -> String {
    format!(
        "{}|{}|{}|{}",
        relation.source_id,
        relation.target_id,
        relation.relation_type,
        relation.evidence.join("|")
    )
}

fn contains_case_insensitive(value: &str, needle: &str) -> bool {
    value.to_lowercase().contains(needle)
}

#[allow(clippy::too_many_arguments)]
fn record_graph_symbol_context(
    symbol: &SymbolNode,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    reason: &str,
    provenance: &[String],
) {
    let Some(source) = source_path_index.get(&symbol.file_path) else {
        return;
    };

    record_related_match(source_matches, &source.source_id, [reason]);

    for page_id in &source.page_ids {
        if page_index.contains_key(page_id) {
            record_page_match(
                page_matches,
                page_id,
                [reason],
                provenance.iter().cloned(),
                "structure",
            );
        }
    }

    for module_id in &source.module_ids {
        if module_index.contains_key(module_id) {
            record_related_match(module_matches, module_id, [reason]);
        }
    }
}

fn merge_match_mode(current: &str, next: &str) -> String {
    if current.is_empty() || current == next {
        return next.to_string();
    }

    if current == "fts+structure" || next == "fts+structure" {
        return "fts+structure".to_string();
    }

    if (current == "fts" && next == "structure") || (current == "structure" && next == "fts") {
        return "fts+structure".to_string();
    }

    if next == "structure" {
        return "structure".to_string();
    }

    current.to_string()
}
