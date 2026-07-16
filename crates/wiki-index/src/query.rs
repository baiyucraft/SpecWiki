//! `wiki-index::query` 收口 facts-owned 的正式查询面。
//! 它只消费 facts snapshot，不允许回读 runtime state 或 page fallback。

use std::collections::{BTreeMap, BTreeSet};
use std::io;

use serde::{Deserialize, Serialize};

use crate::store::{
    CallTraceHit, EntrypointRecord, IndexQueryStore, IndexSnapshotStore, ModuleRecord,
    ModuleSourceLink, SourceFileRecord, SourceRecord,
};
use crate::symbols::{SourceRange, SymbolNode, SymbolProvenance};

const INDEX_NOT_READY_MESSAGE: &str = "index not ready: facts snapshot missing";
const DEFAULT_LOOKUP_LIMIT: usize = 8;
const DEFAULT_GRAPH_LIMIT: usize = 48;
const DEFAULT_GRAPH_DEPTH: usize = 3;

/// 正式支持的 index-first 查询意图。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum IndexQueryIntent {
    #[default]
    Auto,
    SymbolLookup,
    SourceLookup,
    ModuleLookup,
    EntrypointLookup,
    Callers,
    Callees,
    ImpactSlice,
}

/// 当前 facts substrate 可稳定暴露的命中依据。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchBasis {
    SymbolFts,
    SourcePath,
    SourceFilename,
    ModuleName,
    ModuleRootPath,
    EntrypointMembership,
    CallTrace,
}

/// `wiki-index` 对外的最小查询请求。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct IndexQueryRequest {
    pub intent: IndexQueryIntent,
    pub text: String,
    pub limit: usize,
    pub max_depth: usize,
    pub graph_limit: usize,
}

impl Default for IndexQueryRequest {
    fn default() -> Self {
        Self {
            intent: IndexQueryIntent::Auto,
            text: String::new(),
            limit: DEFAULT_LOOKUP_LIMIT,
            max_depth: DEFAULT_GRAPH_DEPTH,
            graph_limit: DEFAULT_GRAPH_LIMIT,
        }
    }
}

/// symbol lookup 的正式命中结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SymbolHit {
    pub symbol_id: String,
    pub file_id: String,
    pub name: String,
    pub label: String,
    pub symbol_kind: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub range: SourceRange,
    pub language: String,
    pub provenance: SymbolProvenance,
    pub module_ids: Vec<String>,
    pub match_basis: MatchBasis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// source lookup 的正式命中结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SourceHit {
    pub source_id: String,
    pub file_id: String,
    pub path: String,
    pub language: String,
    pub kind: String,
    pub module_ids: Vec<String>,
    pub match_basis: MatchBasis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// module lookup 的正式命中结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleHit {
    pub module_id: String,
    pub name: String,
    pub kind: String,
    pub root_paths: Vec<String>,
    pub tags: Vec<String>,
    pub match_basis: MatchBasis,
}

/// entrypoint lookup 的正式命中结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EntrypointHit {
    pub path: String,
    pub source_id: Option<String>,
    pub module_ids: Vec<String>,
    pub match_basis: MatchBasis,
}

/// callers / callees / impact slice 共享的稳定边视图。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CallEdgeHitView {
    pub edge_id: String,
    pub source_id: String,
    pub source_name: String,
    pub source_path: String,
    pub target_id: String,
    pub target_name: String,
    pub target_path: String,
    pub traversal_direction: String,
    pub hop_distance: usize,
    pub match_basis: MatchBasis,
    pub confidence: f64,
    pub reason: String,
    #[serde(default)]
    pub provenance: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// `impact_slice` 的最小正式结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ImpactSlice {
    pub seed_symbol_ids: Vec<String>,
    pub supporting_edges: Vec<CallEdgeHitView>,
    pub truncated: bool,
}

/// `wiki-index::query` 的最小统一返回壳。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct IndexQueryResult {
    pub intent: IndexQueryIntent,
    pub symbols: Vec<SymbolHit>,
    pub sources: Vec<SourceHit>,
    pub modules: Vec<ModuleHit>,
    pub entrypoints: Vec<EntrypointHit>,
    pub call_edges: Vec<CallEdgeHitView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_slice: Option<ImpactSlice>,
}

/// 执行一次 facts-only 的 index 查询。
pub fn run_query<S>(store: &S, request: &IndexQueryRequest) -> io::Result<IndexQueryResult>
where
    S: IndexSnapshotStore + IndexQueryStore,
{
    let needle = request.text.trim().to_lowercase();
    let limit = request.limit.max(1);
    let max_depth = request.max_depth.max(1);
    let graph_limit = request.graph_limit.max(1);

    if needle.is_empty() {
        return Ok(IndexQueryResult {
            intent: request.intent,
            ..IndexQueryResult::default()
        });
    }

    let facts = FactsContext::load(store)?;

    match request.intent {
        IndexQueryIntent::Auto => {
            run_auto_query(store, &facts, &needle, limit, max_depth, graph_limit)
        }
        IndexQueryIntent::SymbolLookup => Ok(IndexQueryResult {
            intent: request.intent,
            symbols: lookup_symbols(store, &facts, &needle, limit)?,
            ..IndexQueryResult::default()
        }),
        IndexQueryIntent::SourceLookup => Ok(IndexQueryResult {
            intent: request.intent,
            sources: lookup_sources(store, &facts, &needle, limit)?,
            ..IndexQueryResult::default()
        }),
        IndexQueryIntent::ModuleLookup => Ok(IndexQueryResult {
            intent: request.intent,
            modules: lookup_modules(&facts, &needle, limit),
            ..IndexQueryResult::default()
        }),
        IndexQueryIntent::EntrypointLookup => Ok(IndexQueryResult {
            intent: request.intent,
            entrypoints: lookup_entrypoints(&facts, &needle, limit),
            ..IndexQueryResult::default()
        }),
        IndexQueryIntent::Callers => run_graph_query(
            store,
            &facts,
            &needle,
            limit,
            max_depth,
            graph_limit,
            Some("inbound"),
            request.intent,
        ),
        IndexQueryIntent::Callees => run_graph_query(
            store,
            &facts,
            &needle,
            limit,
            max_depth,
            graph_limit,
            Some("outbound"),
            request.intent,
        ),
        IndexQueryIntent::ImpactSlice => run_graph_query(
            store,
            &facts,
            &needle,
            limit,
            max_depth,
            graph_limit,
            None,
            request.intent,
        ),
    }
}

#[derive(Debug)]
struct FactsContext {
    sources: Vec<SourceRecord>,
    files: Vec<SourceFileRecord>,
    modules: Vec<ModuleRecord>,
    entrypoints: Vec<EntrypointRecord>,
    source_id_by_path: BTreeMap<String, String>,
    module_ids_by_source_id: BTreeMap<String, Vec<String>>,
}

impl FactsContext {
    fn load<S>(store: &S) -> io::Result<Self>
    where
        S: IndexSnapshotStore + IndexQueryStore,
    {
        let scan_report = store.read_scan_report()?.ok_or_else(index_not_ready)?;
        let module_tree = store.read_module_tree()?.ok_or_else(index_not_ready)?;
        let sources = store.list_sources()?;
        let files = store.list_files()?;
        let modules = store.list_modules()?;
        let entrypoints = store.list_entrypoints()?;
        let links = store.list_module_source_links()?;

        if !scan_report.files.is_empty() && sources.is_empty() {
            return Err(index_not_ready());
        }
        if !module_tree.modules.is_empty() && modules.is_empty() {
            return Err(index_not_ready());
        }

        let source_id_by_path = sources
            .iter()
            .map(|source| (source.path.clone(), source.source_id.clone()))
            .collect::<BTreeMap<_, _>>();
        let module_ids_by_source_id = build_module_ids_by_source_id(&links);

        Ok(Self {
            sources,
            files,
            modules,
            entrypoints,
            source_id_by_path,
            module_ids_by_source_id,
        })
    }
}

fn run_auto_query<S>(
    store: &S,
    facts: &FactsContext,
    needle: &str,
    limit: usize,
    max_depth: usize,
    graph_limit: usize,
) -> io::Result<IndexQueryResult>
where
    S: IndexSnapshotStore + IndexQueryStore,
{
    let symbols = lookup_symbols(store, facts, needle, limit)?;
    let sources = lookup_sources(store, facts, needle, limit)?;
    let modules = lookup_modules(facts, needle, limit);
    let entrypoints = lookup_entrypoints(facts, needle, limit);
    let graph = build_graph_projection(
        store,
        &store.list_symbols()?,
        symbols.iter().map(|hit| hit.symbol_id.clone()).collect(),
        max_depth,
        graph_limit,
        None,
    )?;

    Ok(IndexQueryResult {
        intent: IndexQueryIntent::Auto,
        symbols,
        sources,
        modules,
        entrypoints,
        call_edges: graph.0,
        impact_slice: graph.1,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_graph_query<S>(
    store: &S,
    _facts: &FactsContext,
    needle: &str,
    limit: usize,
    max_depth: usize,
    graph_limit: usize,
    direction: Option<&str>,
    intent: IndexQueryIntent,
) -> io::Result<IndexQueryResult>
where
    S: IndexSnapshotStore + IndexQueryStore,
{
    let symbols = lookup_symbols(store, _facts, needle, limit)?;
    let all_symbols = store.list_symbols()?;
    let (call_edges, impact_slice) = build_graph_projection(
        store,
        &all_symbols,
        symbols.iter().map(|hit| hit.symbol_id.clone()).collect(),
        max_depth,
        graph_limit,
        direction,
    )?;

    Ok(IndexQueryResult {
        intent,
        symbols,
        call_edges,
        impact_slice: matches!(intent, IndexQueryIntent::ImpactSlice).then_some(
            impact_slice.unwrap_or(ImpactSlice {
                seed_symbol_ids: Vec::new(),
                supporting_edges: Vec::new(),
                truncated: false,
            }),
        ),
        ..IndexQueryResult::default()
    })
}

fn lookup_symbols<S>(
    store: &S,
    facts: &FactsContext,
    needle: &str,
    limit: usize,
) -> io::Result<Vec<SymbolHit>>
where
    S: IndexQueryStore,
{
    store.search_symbols(needle, limit).map(|hits| {
        hits.into_iter()
            .map(|hit| SymbolHit {
                module_ids: facts
                    .source_id_by_path
                    .get(&hit.file_path)
                    .and_then(|source_id| facts.module_ids_by_source_id.get(source_id))
                    .cloned()
                    .unwrap_or_default(),
                symbol_id: hit.symbol_id,
                file_id: hit.file_id,
                name: hit.name,
                label: hit.label,
                symbol_kind: hit.symbol_kind,
                file_path: hit.file_path,
                start_line: hit.start_line,
                end_line: hit.end_line,
                range: hit.range,
                language: hit.language,
                provenance: hit.provenance,
                match_basis: MatchBasis::SymbolFts,
                score: Some(hit.score),
            })
            .collect()
    })
}

fn lookup_sources<S>(
    store: &S,
    facts: &FactsContext,
    needle: &str,
    limit: usize,
) -> io::Result<Vec<SourceHit>>
where
    S: IndexQueryStore,
{
    let mut hits = store
        .search_files(needle, limit)?
        .into_iter()
        .map(|hit| {
            let source_id = facts
                .source_id_by_path
                .get(&hit.path)
                .cloned()
                .unwrap_or_else(|| hit.file_id.clone());
            SourceHit {
                source_id: source_id.clone(),
                file_id: hit.file_id,
                path: hit.path,
                language: hit.language,
                kind: hit.kind,
                module_ids: facts
                    .module_ids_by_source_id
                    .get(&source_id)
                    .cloned()
                    .unwrap_or_default(),
                match_basis: MatchBasis::SourcePath,
                score: Some(hit.score),
                diagnostics: Vec::new(),
            }
        })
        .collect::<Vec<_>>();
    let has_ranked_fts_hits = !hits.is_empty();
    if hits.is_empty() {
        hits = facts
            .files
            .iter()
            .filter_map(|file| source_file_match_basis(file, needle).map(|basis| (file, basis)))
            .map(|(file, match_basis)| map_source_file_hit(facts, file, match_basis, None))
            .collect::<Vec<_>>();
    }
    if hits.is_empty() {
        hits = facts
            .sources
            .iter()
            .filter_map(|source| {
                source_match_basis(source, needle).map(|match_basis| SourceHit {
                    source_id: source.source_id.clone(),
                    file_id: source.source_id.clone(),
                    path: source.path.clone(),
                    language: source.language.clone(),
                    kind: source.kind.clone(),
                    module_ids: facts
                        .module_ids_by_source_id
                        .get(&source.source_id)
                        .cloned()
                        .unwrap_or_default(),
                    match_basis,
                    score: None,
                    diagnostics: Vec::new(),
                })
            })
            .collect::<Vec<_>>();
    }
    if !has_ranked_fts_hits {
        hits.sort_by(|left, right| left.path.cmp(&right.path));
    }
    hits.truncate(limit);
    Ok(hits)
}

fn lookup_modules(facts: &FactsContext, needle: &str, limit: usize) -> Vec<ModuleHit> {
    let mut hits = facts
        .modules
        .iter()
        .filter_map(|module| {
            let match_basis = if contains_case_insensitive(&module.name, needle) {
                Some(MatchBasis::ModuleName)
            } else if module
                .root_paths
                .iter()
                .any(|path| contains_case_insensitive(path, needle))
            {
                Some(MatchBasis::ModuleRootPath)
            } else {
                None
            }?;

            Some(ModuleHit {
                module_id: module.module_id.clone(),
                name: module.name.clone(),
                kind: module.kind.clone(),
                root_paths: module.root_paths.clone(),
                tags: module.tags.clone(),
                match_basis,
            })
        })
        .collect::<Vec<_>>();
    hits.sort_by(|left, right| left.module_id.cmp(&right.module_id));
    hits.truncate(limit);
    hits
}

fn lookup_entrypoints(facts: &FactsContext, needle: &str, limit: usize) -> Vec<EntrypointHit> {
    let mut hits = facts
        .entrypoints
        .iter()
        .filter(|entry| contains_case_insensitive(&entry.path, needle))
        .map(|entry| EntrypointHit {
            path: entry.path.clone(),
            source_id: entry.source_id.clone(),
            module_ids: entry.module_ids.clone(),
            match_basis: MatchBasis::EntrypointMembership,
        })
        .collect::<Vec<_>>();
    hits.sort_by(|left, right| left.path.cmp(&right.path));
    hits.truncate(limit);
    hits
}

fn build_graph_projection<S>(
    store: &S,
    all_symbols: &[SymbolNode],
    seed_symbol_ids: Vec<String>,
    max_depth: usize,
    graph_limit: usize,
    direction: Option<&str>,
) -> io::Result<(Vec<CallEdgeHitView>, Option<ImpactSlice>)>
where
    S: IndexQueryStore,
{
    if seed_symbol_ids.is_empty() {
        return Ok((Vec::new(), None));
    }

    let symbol_index = all_symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let edges = store.trace_call_edges_from_seeds(&seed_symbol_ids, max_depth, graph_limit)?;
    let truncated = edges.len() >= graph_limit;
    let supporting_edges = edges
        .into_iter()
        .filter(|edge| direction.is_none_or(|expected| edge.traversal_direction == expected))
        .map(|edge| map_call_edge_hit(&symbol_index, edge))
        .collect::<Vec<_>>();
    let impact_slice = ImpactSlice {
        seed_symbol_ids,
        supporting_edges: supporting_edges.clone(),
        truncated,
    };

    Ok((supporting_edges, Some(impact_slice)))
}

fn map_call_edge_hit(
    symbol_index: &BTreeMap<String, &SymbolNode>,
    edge: CallTraceHit,
) -> CallEdgeHitView {
    let source = symbol_index.get(&edge.source_id).copied();
    let target = symbol_index.get(&edge.target_id).copied();

    CallEdgeHitView {
        edge_id: edge.edge_id,
        source_id: edge.source_id,
        source_name: source.map(|symbol| symbol.name.clone()).unwrap_or_default(),
        source_path: source
            .map(|symbol| symbol.file_path.clone())
            .unwrap_or_default(),
        target_id: edge.target_id,
        target_name: target.map(|symbol| symbol.name.clone()).unwrap_or_default(),
        target_path: target
            .map(|symbol| symbol.file_path.clone())
            .unwrap_or_default(),
        traversal_direction: edge.traversal_direction,
        hop_distance: edge.hop_distance,
        match_basis: MatchBasis::CallTrace,
        confidence: edge.confidence,
        reason: edge.reason,
        provenance: vec!["index:call_trace".to_string()],
        diagnostics: Vec::new(),
    }
}

fn build_module_ids_by_source_id(links: &[ModuleSourceLink]) -> BTreeMap<String, Vec<String>> {
    let mut module_ids_by_source_id = BTreeMap::<String, BTreeSet<String>>::new();
    for link in links {
        module_ids_by_source_id
            .entry(link.source_id.clone())
            .or_default()
            .insert(link.module_id.clone());
    }

    module_ids_by_source_id
        .into_iter()
        .map(|(source_id, module_ids)| (source_id, module_ids.into_iter().collect()))
        .collect()
}

fn source_match_basis(source: &SourceRecord, needle: &str) -> Option<MatchBasis> {
    if source
        .path
        .rsplit('/')
        .next()
        .is_some_and(|name| contains_case_insensitive(name, needle))
    {
        return Some(MatchBasis::SourceFilename);
    }

    contains_case_insensitive(&source.path, needle).then_some(MatchBasis::SourcePath)
}

fn source_file_match_basis(source: &SourceFileRecord, needle: &str) -> Option<MatchBasis> {
    if source
        .path
        .rsplit('/')
        .next()
        .is_some_and(|name| contains_case_insensitive(name, needle))
    {
        return Some(MatchBasis::SourceFilename);
    }

    contains_case_insensitive(&source.path, needle).then_some(MatchBasis::SourcePath)
}

fn map_source_file_hit(
    facts: &FactsContext,
    source: &SourceFileRecord,
    match_basis: MatchBasis,
    score: Option<f64>,
) -> SourceHit {
    let source_id = facts
        .source_id_by_path
        .get(&source.path)
        .cloned()
        .unwrap_or_else(|| source.file_id.clone());
    SourceHit {
        source_id: source_id.clone(),
        file_id: source.file_id.clone(),
        path: source.path.clone(),
        language: source.language.clone(),
        kind: source.kind.clone(),
        module_ids: facts
            .module_ids_by_source_id
            .get(&source_id)
            .cloned()
            .unwrap_or_default(),
        match_basis,
        score,
        diagnostics: source.diagnostics.clone(),
    }
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(needle)
}

fn index_not_ready() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, INDEX_NOT_READY_MESSAGE)
}

#[cfg(test)]
mod tests {
    use super::{run_query, IndexQueryIntent, IndexQueryRequest, MatchBasis};
    use crate::scanner::{FilePurpose, ScanReport, ScannedFile};
    use crate::store::{
        CallTraceHit, EntrypointRecord, FileSearchHit, IndexQueryStore, IndexSnapshotStore,
        ModuleRecord, ModuleSourceLink, SourceFileRecord, SourceRecord, SymbolSearchHit,
    };
    use crate::symbol_graph::{
        CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
        ResolvedGraphSnapshot, ResolvedSymbolEdge,
    };
    use crate::symbols::SymbolNode;
    use std::io;
    use wiki_model::domain::module_tree::{ModuleNode, ModuleTree, RelationEdge};

    #[derive(Clone, Default)]
    struct MemoryStore {
        scan_report: Option<ScanReport>,
        module_tree: Option<ModuleTree>,
        modules: Vec<ModuleRecord>,
        module_source_links: Vec<ModuleSourceLink>,
        sources: Vec<SourceRecord>,
        files: Vec<SourceFileRecord>,
        file_hits: Vec<FileSearchHit>,
        entrypoints: Vec<EntrypointRecord>,
        symbols: Vec<SymbolNode>,
        symbol_hits: Vec<SymbolSearchHit>,
        call_trace_hits: Vec<CallTraceHit>,
    }

    impl IndexSnapshotStore for MemoryStore {
        fn write_scan_report(&self, _report: &ScanReport) -> io::Result<()> {
            Ok(())
        }

        fn read_scan_report(&self) -> io::Result<Option<ScanReport>> {
            Ok(self.scan_report.clone())
        }

        fn write_module_tree(&self, _tree: &ModuleTree) -> io::Result<()> {
            Ok(())
        }

        fn read_module_tree(&self) -> io::Result<Option<ModuleTree>> {
            Ok(self.module_tree.clone())
        }

        fn list_modules(&self) -> io::Result<Vec<ModuleRecord>> {
            Ok(self.modules.clone())
        }

        fn list_module_source_links(&self) -> io::Result<Vec<ModuleSourceLink>> {
            Ok(self.module_source_links.clone())
        }

        fn list_sources(&self) -> io::Result<Vec<SourceRecord>> {
            Ok(self.sources.clone())
        }

        fn list_entrypoints(&self) -> io::Result<Vec<EntrypointRecord>> {
            Ok(self.entrypoints.clone())
        }

        fn replace_symbol_graph(
            &self,
            _symbols: &[SymbolNode],
            _edges: &[ResolvedSymbolEdge],
            _analysis: &GraphAnalysisSnapshot,
            _resolved_graph: &ResolvedGraphSnapshot,
        ) -> io::Result<()> {
            Ok(())
        }

        fn replace_symbol_graph_for_files(
            &self,
            _file_paths: &[String],
            _symbols: &[SymbolNode],
            _edges: &[ResolvedSymbolEdge],
            _analysis: &GraphAnalysisSnapshot,
            _resolved_graph: &ResolvedGraphSnapshot,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    impl IndexQueryStore for MemoryStore {
        fn list_symbols(&self) -> io::Result<Vec<SymbolNode>> {
            Ok(self.symbols.clone())
        }

        fn list_symbols_for_files(&self, _source_paths: &[String]) -> io::Result<Vec<SymbolNode>> {
            Ok(self.symbols.clone())
        }

        fn count_symbol_files(&self) -> io::Result<usize> {
            Ok(self
                .symbols
                .iter()
                .map(|symbol| symbol.file_path.clone())
                .collect::<std::collections::BTreeSet<_>>()
                .len())
        }

        fn search_symbols(&self, _term: &str, limit: usize) -> io::Result<Vec<SymbolSearchHit>> {
            Ok(self.symbol_hits.iter().take(limit).cloned().collect())
        }

        fn list_files(&self) -> io::Result<Vec<SourceFileRecord>> {
            Ok(self.files.clone())
        }

        fn search_files(&self, _term: &str, limit: usize) -> io::Result<Vec<FileSearchHit>> {
            Ok(self.file_hits.iter().take(limit).cloned().collect())
        }

        fn list_edges(&self) -> io::Result<Vec<ResolvedSymbolEdge>> {
            Ok(Vec::new())
        }

        fn list_edges_for_files(
            &self,
            _source_paths: &[String],
        ) -> io::Result<Vec<ResolvedSymbolEdge>> {
            Ok(Vec::new())
        }

        fn list_adjacent_symbol_files(&self, _source_paths: &[String]) -> io::Result<Vec<String>> {
            Ok(Vec::new())
        }

        fn trace_call_edges(
            &self,
            _symbol_id: &str,
            _max_depth: usize,
        ) -> io::Result<Vec<ResolvedSymbolEdge>> {
            Ok(Vec::new())
        }

        fn trace_call_edges_from_seeds(
            &self,
            _symbol_ids: &[String],
            _max_depth: usize,
            limit: usize,
        ) -> io::Result<Vec<CallTraceHit>> {
            Ok(self.call_trace_hits.iter().take(limit).cloned().collect())
        }

        fn list_communities(&self) -> io::Result<Vec<CommunityNode>> {
            Ok(Vec::new())
        }

        fn list_community_members(&self, _community_id: &str) -> io::Result<Vec<CommunityMember>> {
            Ok(Vec::new())
        }

        fn list_processes(&self) -> io::Result<Vec<ProcessNode>> {
            Ok(Vec::new())
        }

        fn list_process_steps(&self, _process_id: &str) -> io::Result<Vec<ProcessStep>> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn lookup_intents_expose_facts_owned_hits() {
        let store = sample_store();

        let symbol_result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::SymbolLookup,
                text: "handleCheckout".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        assert_eq!(symbol_result.symbols.len(), 1);
        assert_eq!(symbol_result.symbols[0].match_basis, MatchBasis::SymbolFts);
        assert_eq!(symbol_result.symbols[0].score, Some(0.92));
        assert_eq!(symbol_result.symbols[0].module_ids, vec!["app".to_string()]);

        let source_result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::SourceLookup,
                text: "main.ts".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        assert_eq!(source_result.sources.len(), 1);
        assert_eq!(source_result.sources[0].match_basis, MatchBasis::SourcePath);
        assert_eq!(source_result.sources[0].score, Some(0.81));
        assert_eq!(source_result.sources[0].module_ids, vec!["app".to_string()]);

        let module_result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::ModuleLookup,
                text: "payments".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        assert_eq!(module_result.modules.len(), 1);
        assert_eq!(module_result.modules[0].match_basis, MatchBasis::ModuleName);

        let entrypoint_result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::EntrypointLookup,
                text: "src/main".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        assert_eq!(entrypoint_result.entrypoints.len(), 1);
        assert_eq!(
            entrypoint_result.entrypoints[0].match_basis,
            MatchBasis::EntrypointMembership
        );
        assert_eq!(
            entrypoint_result.entrypoints[0].module_ids,
            vec!["app".to_string()]
        );
    }

    #[test]
    fn graph_intents_preserve_confidence_reason_and_truncation() {
        let store = sample_store();

        let callers = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::Callers,
                text: "handleCheckout".to_string(),
                graph_limit: 4,
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        assert_eq!(callers.call_edges.len(), 1);
        assert_eq!(callers.call_edges[0].match_basis, MatchBasis::CallTrace);
        assert_eq!(callers.call_edges[0].confidence, 0.88);
        assert_eq!(callers.call_edges[0].reason, "resolved-call");
        assert_eq!(callers.call_edges[0].traversal_direction, "inbound");

        let impact = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::ImpactSlice,
                text: "handleCheckout".to_string(),
                graph_limit: 1,
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();
        let slice = impact.impact_slice.expect("impact slice should exist");
        assert!(slice.truncated);
        assert_eq!(slice.supporting_edges.len(), 1);
        assert_eq!(slice.supporting_edges[0].confidence, 0.88);
        assert_eq!(slice.supporting_edges[0].reason, "resolved-call");
    }

    #[test]
    fn auto_query_combines_lookup_hits_and_graph_slice() {
        let store = sample_store();
        let result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::Auto,
                text: "handleCheckout".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();

        assert_eq!(result.symbols.len(), 1);
        assert_eq!(result.call_edges.len(), 2);
        assert!(result.impact_slice.is_some());
    }

    #[test]
    fn index_query_returns_enriched_symbol_path_and_graph_hits() {
        let store = sample_store();
        let result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::Auto,
                text: "handleCheckout".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();

        assert!(result.symbols.iter().any(|hit| {
            hit.symbol_id == "symbol:handleCheckout"
                && hit.file_id == hit.range.file_id
                && hit.symbol_kind == "function"
                && hit.range.path == "src/main.ts"
                && hit.provenance.parser_id == "tree-sitter"
        }));
        assert!(result.sources.iter().any(|hit| {
            hit.file_id == "file:src/main.ts"
                && hit.path == "src/main.ts"
                && hit.kind == "source"
                && hit.score == Some(0.81)
        }));
        assert!(result.call_edges.iter().any(|edge| {
            edge.edge_id == "edge:caller"
                && edge.confidence > 0.0
                && edge.hop_distance >= 1
                && edge
                    .provenance
                    .iter()
                    .any(|item| item == "index:call_trace")
        }));
    }

    #[test]
    fn source_query_preserves_store_relevance_order() {
        let mut store = sample_store();
        store.file_hits = vec![
            FileSearchHit {
                file_id: "file:z-most-relevant.ts".to_string(),
                path: "z-most-relevant.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                score: -2.0,
            },
            FileSearchHit {
                file_id: "file:a-less-relevant.ts".to_string(),
                path: "a-less-relevant.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                score: -1.0,
            },
        ];

        let result = run_query(
            &store,
            &IndexQueryRequest {
                intent: IndexQueryIntent::SourceLookup,
                text: "relevant".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .unwrap();

        assert_eq!(result.sources[0].path, "z-most-relevant.ts");
        assert_eq!(result.sources[0].score, Some(-2.0));
        assert_eq!(result.sources[1].path, "a-less-relevant.ts");
    }

    #[test]
    fn missing_snapshot_returns_index_not_ready() {
        let error = run_query(
            &MemoryStore::default(),
            &IndexQueryRequest {
                intent: IndexQueryIntent::SourceLookup,
                text: "src".to_string(),
                ..IndexQueryRequest::default()
            },
        )
        .expect_err("missing facts snapshot should fail");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(error.to_string().contains("index not ready"));
    }

    fn sample_store() -> MemoryStore {
        let scan_report = ScanReport {
            root: ".".to_string(),
            files: vec![ScannedFile {
                id: "source:src/main.ts".to_string(),
                path: "src/main.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                purpose: FilePurpose::Entry,
                fingerprint: "fp-main".to_string(),
                size: 64,
                tags: vec!["entry-point".to_string()],
            }],
            tech_hints: vec!["typescript".to_string()],
            workspace_roots: vec![".".to_string()],
            config_files: vec!["package.json".to_string()],
            entry_points: vec!["src/main.ts".to_string()],
            dependency_hints: Vec::new(),
        };
        let module_tree = ModuleTree {
            root_modules: vec!["app".to_string()],
            modules: vec![ModuleNode {
                id: "app".to_string(),
                name: "payments".to_string(),
                kind: "service".to_string(),
                root_paths: vec!["src".to_string()],
                source_ids: vec!["source:src/main.ts".to_string()],
                parent_id: None,
                child_ids: Vec::new(),
                entry_points: vec!["src/main.ts".to_string()],
                tags: vec!["entry-point".to_string()],
            }],
            cross_module_edges: Vec::<RelationEdge>::new(),
            architecture_hints: Vec::new(),
        };
        let symbols = vec![
            SymbolNode::legacy(
                "symbol:handleCheckout".to_string(),
                "handleCheckout".to_string(),
                "function".to_string(),
                "src/main.ts".to_string(),
                1,
                4,
                true,
                "typescript".to_string(),
            ),
            SymbolNode::legacy(
                "symbol:runPayment".to_string(),
                "runPayment".to_string(),
                "function".to_string(),
                "src/main.ts".to_string(),
                6,
                9,
                false,
                "typescript".to_string(),
            ),
        ];

        let symbol_hit_source = symbols[0].clone();

        MemoryStore {
            scan_report: Some(scan_report),
            module_tree: Some(module_tree),
            modules: vec![ModuleRecord {
                module_id: "app".to_string(),
                name: "payments".to_string(),
                kind: "service".to_string(),
                root_paths: vec!["src".to_string()],
                parent_id: None,
                child_ids: Vec::new(),
                entry_points: vec!["src/main.ts".to_string()],
                tags: vec!["entry-point".to_string()],
            }],
            module_source_links: vec![ModuleSourceLink {
                module_id: "app".to_string(),
                source_id: "source:src/main.ts".to_string(),
            }],
            sources: vec![SourceRecord {
                source_id: "source:src/main.ts".to_string(),
                path: "src/main.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                tags: vec!["entry-point".to_string()],
            }],
            files: vec![SourceFileRecord {
                file_id: "file:src/main.ts".to_string(),
                path: "src/main.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                fingerprint: "fp-main".to_string(),
                size: 64,
                indexed_at: "test".to_string(),
                diagnostics: Vec::new(),
            }],
            file_hits: vec![FileSearchHit {
                file_id: "file:src/main.ts".to_string(),
                path: "src/main.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                score: 0.81,
            }],
            entrypoints: vec![EntrypointRecord {
                path: "src/main.ts".to_string(),
                source_id: Some("source:src/main.ts".to_string()),
                module_ids: vec!["app".to_string()],
            }],
            symbols,
            symbol_hits: vec![SymbolSearchHit {
                symbol_id: "symbol:handleCheckout".to_string(),
                name: "handleCheckout".to_string(),
                label: "function".to_string(),
                symbol_kind: symbol_hit_source.symbol_kind.clone(),
                file_path: "src/main.ts".to_string(),
                file_id: symbol_hit_source.file_id.clone(),
                start_line: 1,
                end_line: 4,
                range: symbol_hit_source.range.clone(),
                language: "typescript".to_string(),
                provenance: symbol_hit_source.provenance.clone(),
                score: 0.92,
            }],
            call_trace_hits: vec![
                CallTraceHit {
                    edge_id: "edge:caller".to_string(),
                    source_id: "symbol:runPayment".to_string(),
                    target_id: "symbol:handleCheckout".to_string(),
                    traversal_direction: "inbound".to_string(),
                    hop_distance: 1,
                    confidence: 0.88,
                    reason: "resolved-call".to_string(),
                },
                CallTraceHit {
                    edge_id: "edge:callee".to_string(),
                    source_id: "symbol:handleCheckout".to_string(),
                    target_id: "symbol:runPayment".to_string(),
                    traversal_direction: "outbound".to_string(),
                    hop_distance: 1,
                    confidence: 0.74,
                    reason: "reverse-impact".to_string(),
                },
            ],
        }
    }
}
