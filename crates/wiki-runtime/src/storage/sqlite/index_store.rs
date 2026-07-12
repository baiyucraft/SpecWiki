//! index_store 聚合 facts / index 相关的 SQLite 读写入口。

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use wiki_index::scanner::ScanReport;
use wiki_index::store::{
    CallTraceHit, EntrypointRecord, FileSearchHit, FolderRecord, GraphPhaseStatus, GraphReadiness,
    GraphSnapshot, IndexQueryStore, IndexSnapshotStore, ModuleRecord, ModuleSourceLink,
    SourceFileRecord, SourceRecord, SymbolSearchHit,
};
use wiki_index::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use wiki_index::symbols::{
    RawCallCapture, RawHeritageCapture, RawImportCapture, SymbolNode, UnresolvedRef,
};
use wiki_model::domain::module_tree::ModuleTree;

use crate::storage::sqlite_store;

/// `SqliteIndexStore` 用于在 runtime 中实现 wiki-index 的 store trait。
pub struct SqliteIndexStore {
    repo_root: PathBuf,
}

impl SqliteIndexStore {
    /// 绑定某个仓库根目录，后续所有 facts/index 读写都落到该 repo 的 SQLite。
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }
}

impl IndexSnapshotStore for SqliteIndexStore {
    fn write_scan_report(&self, report: &ScanReport) -> io::Result<()> {
        let conn = sqlite_store::open_db(&self.repo_root)?;
        let json =
            serde_json::to_string(report).map_err(|error| io::Error::other(error.to_string()))?;
        sqlite_store::scan_cache_set(&conn, "repo-scan", &json)
    }

    fn read_scan_report(&self) -> io::Result<Option<ScanReport>> {
        let conn = sqlite_store::open_db_readonly(&self.repo_root)?;
        match sqlite_store::scan_cache_get(&conn, "repo-scan")? {
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|error| io::Error::other(error.to_string())),
            None => Ok(None),
        }
    }

    fn write_module_tree(&self, tree: &ModuleTree) -> io::Result<()> {
        let conn = sqlite_store::open_db(&self.repo_root)?;
        let json =
            serde_json::to_string(tree).map_err(|error| io::Error::other(error.to_string()))?;
        sqlite_store::scan_cache_set(&conn, "module-tree", &json)?;
        sqlite_store::replace_module_tree_snapshot(&conn, tree)
    }

    fn read_module_tree(&self) -> io::Result<Option<ModuleTree>> {
        let conn = sqlite_store::open_db_readonly(&self.repo_root)?;
        match sqlite_store::scan_cache_get(&conn, "module-tree")? {
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|error| io::Error::other(error.to_string())),
            None => Ok(None),
        }
    }

    fn list_modules(&self) -> io::Result<Vec<ModuleRecord>> {
        sqlite_store::list_modules_snapshot(&self.repo_root)
    }

    fn list_module_source_links(&self) -> io::Result<Vec<ModuleSourceLink>> {
        sqlite_store::list_module_source_links(&self.repo_root)
    }

    fn list_sources(&self) -> io::Result<Vec<SourceRecord>> {
        let graph_files = sqlite_store::list_files(&self.repo_root)?;
        if !graph_files.is_empty() {
            return Ok(graph_files
                .into_iter()
                .map(|file| SourceRecord {
                    source_id: file.file_id,
                    path: file.path,
                    language: file.language,
                    kind: file.kind,
                    tags: Vec::new(),
                })
                .collect());
        }

        Ok(self
            .read_scan_report()?
            .map(|report| {
                report
                    .files
                    .into_iter()
                    .map(|file| SourceRecord {
                        source_id: file.id,
                        path: file.path,
                        language: file.language,
                        kind: file.kind,
                        tags: file.tags,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    fn list_entrypoints(&self) -> io::Result<Vec<EntrypointRecord>> {
        let Some(report) = self.read_scan_report()? else {
            return Ok(Vec::new());
        };

        let source_id_by_path = self
            .list_sources()?
            .into_iter()
            .map(|source| (source.path, source.source_id))
            .collect::<BTreeMap<_, _>>();
        let mut module_ids_by_path = BTreeMap::<String, BTreeSet<String>>::new();
        for module in self.list_modules()? {
            let module_id = module.module_id;
            for entry_path in module.entry_points {
                module_ids_by_path
                    .entry(entry_path)
                    .or_default()
                    .insert(module_id.clone());
            }
        }

        Ok(report
            .entry_points
            .into_iter()
            .map(|path| EntrypointRecord {
                source_id: source_id_by_path.get(&path).cloned(),
                module_ids: module_ids_by_path
                    .remove(&path)
                    .map(|module_ids| module_ids.into_iter().collect())
                    .unwrap_or_default(),
                path,
            })
            .collect())
    }

    fn replace_symbol_graph(
        &self,
        symbols: &[SymbolNode],
        edges: &[ResolvedSymbolEdge],
        analysis: &GraphAnalysisSnapshot,
        resolved_graph: &ResolvedGraphSnapshot,
    ) -> io::Result<()> {
        sqlite_store::replace_symbol_graph(
            &self.repo_root,
            symbols,
            edges,
            analysis,
            resolved_graph,
        )
    }

    fn replace_symbol_graph_for_files(
        &self,
        file_paths: &[String],
        symbols: &[SymbolNode],
        edges: &[ResolvedSymbolEdge],
        analysis: &GraphAnalysisSnapshot,
        resolved_graph: &ResolvedGraphSnapshot,
    ) -> io::Result<()> {
        sqlite_store::replace_symbol_graph_for_files(
            &self.repo_root,
            file_paths,
            symbols,
            edges,
            analysis,
            resolved_graph,
        )
    }

    fn replace_graph_snapshot(&self, snapshot: &GraphSnapshot) -> io::Result<()> {
        let mut conn = sqlite_store::open_db(&self.repo_root)?;
        sqlite_store::replace_graph_snapshot(&mut conn, snapshot)
    }

    fn replace_graph_snapshot_for_files(
        &self,
        file_paths: &[String],
        snapshot: &GraphSnapshot,
    ) -> io::Result<()> {
        let mut conn = sqlite_store::open_db(&self.repo_root)?;
        sqlite_store::replace_graph_snapshot_for_files(&mut conn, file_paths, snapshot)
    }
}

impl IndexQueryStore for SqliteIndexStore {
    fn list_symbols(&self) -> io::Result<Vec<SymbolNode>> {
        sqlite_store::list_symbols(&self.repo_root)
    }

    fn list_symbols_for_files(&self, source_paths: &[String]) -> io::Result<Vec<SymbolNode>> {
        sqlite_store::list_symbols_for_files(&self.repo_root, source_paths)
    }

    fn count_symbol_files(&self) -> io::Result<usize> {
        sqlite_store::count_symbol_files(&self.repo_root)
    }

    fn search_symbols(&self, term: &str, limit: usize) -> io::Result<Vec<SymbolSearchHit>> {
        sqlite_store::search_symbols_fts(&self.repo_root, term, limit).map(|hits| {
            hits.into_iter()
                .map(|hit| SymbolSearchHit {
                    symbol_id: hit.symbol.symbol_id,
                    name: hit.symbol.name,
                    label: hit.symbol.label,
                    symbol_kind: hit.symbol.symbol_kind,
                    file_path: hit.symbol.file_path,
                    file_id: hit.symbol.file_id,
                    start_line: hit.symbol.start_line,
                    end_line: hit.symbol.end_line,
                    range: hit.symbol.range,
                    language: hit.symbol.language,
                    provenance: hit.symbol.provenance,
                    score: hit.score,
                })
                .collect()
        })
    }

    fn list_edges(&self) -> io::Result<Vec<ResolvedSymbolEdge>> {
        sqlite_store::list_edges(&self.repo_root)
    }

    fn list_edges_for_files(&self, source_paths: &[String]) -> io::Result<Vec<ResolvedSymbolEdge>> {
        sqlite_store::list_edges_for_files(&self.repo_root, source_paths)
    }

    fn list_adjacent_symbol_files(&self, source_paths: &[String]) -> io::Result<Vec<String>> {
        sqlite_store::list_adjacent_symbol_files(&self.repo_root, source_paths)
    }

    fn trace_call_edges(
        &self,
        symbol_id: &str,
        max_depth: usize,
    ) -> io::Result<Vec<ResolvedSymbolEdge>> {
        self.trace_call_edges_from_seeds(&[symbol_id.to_string()], max_depth, 48)
            .map(|hits| {
                hits.into_iter()
                    .map(|hit| ResolvedSymbolEdge {
                        edge_id: hit.edge_id,
                        source_id: hit.source_id,
                        target_id: hit.target_id,
                        edge_type: String::new(),
                        confidence: hit.confidence,
                        reason: hit.reason,
                    })
                    .collect()
            })
    }

    fn trace_call_edges_from_seeds(
        &self,
        symbol_ids: &[String],
        max_depth: usize,
        limit: usize,
    ) -> io::Result<Vec<CallTraceHit>> {
        sqlite_store::trace_call_edges(&self.repo_root, symbol_ids, max_depth, limit).map(|hits| {
            hits.into_iter()
                .map(|hit| CallTraceHit {
                    edge_id: hit.edge_id,
                    source_id: hit.source_id,
                    target_id: hit.target_id,
                    traversal_direction: hit.traversal_direction,
                    hop_distance: hit.hop_distance,
                    confidence: hit.confidence,
                    reason: hit.reason,
                })
                .collect()
        })
    }

    fn list_communities(&self) -> io::Result<Vec<CommunityNode>> {
        sqlite_store::list_communities(&self.repo_root)
    }

    fn list_community_members(&self, community_id: &str) -> io::Result<Vec<CommunityMember>> {
        sqlite_store::list_community_members(&self.repo_root).map(|members| {
            members
                .into_iter()
                .filter(|member| member.community_id == community_id)
                .collect()
        })
    }

    fn list_processes(&self) -> io::Result<Vec<ProcessNode>> {
        sqlite_store::list_processes(&self.repo_root)
    }

    fn list_process_steps(&self, process_id: &str) -> io::Result<Vec<ProcessStep>> {
        sqlite_store::list_process_steps(&self.repo_root).map(|steps| {
            steps
                .into_iter()
                .filter(|step| step.process_id == process_id)
                .collect()
        })
    }

    fn list_files(&self) -> io::Result<Vec<SourceFileRecord>> {
        sqlite_store::list_files(&self.repo_root)
    }

    fn list_folders(&self) -> io::Result<Vec<FolderRecord>> {
        sqlite_store::list_folders(&self.repo_root)
    }

    fn search_files(&self, term: &str, limit: usize) -> io::Result<Vec<FileSearchHit>> {
        sqlite_store::search_files_fts(&self.repo_root, term, limit)
    }

    fn list_raw_imports(&self) -> io::Result<Vec<RawImportCapture>> {
        sqlite_store::list_raw_imports(&self.repo_root)
    }

    fn list_raw_calls(&self) -> io::Result<Vec<RawCallCapture>> {
        sqlite_store::list_raw_calls(&self.repo_root)
    }

    fn list_raw_heritage(&self) -> io::Result<Vec<RawHeritageCapture>> {
        sqlite_store::list_raw_heritage(&self.repo_root)
    }

    fn list_unresolved_refs(&self) -> io::Result<Vec<UnresolvedRef>> {
        sqlite_store::list_unresolved_refs(&self.repo_root)
    }

    fn list_graph_phase_runs(&self) -> io::Result<Vec<GraphPhaseStatus>> {
        sqlite_store::list_graph_phase_runs(&self.repo_root)
    }

    fn read_graph_readiness(&self) -> io::Result<GraphReadiness> {
        sqlite_store::read_graph_readiness(&self.repo_root)
    }
}
