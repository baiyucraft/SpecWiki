use std::io;

use wiki_model::domain::module_tree::ModuleTree;

use crate::scanner::ScanReport;
use crate::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use crate::symbols::SymbolNode;

/// 模块快照读取面暴露的稳定模块视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleRecord {
    pub module_id: String,
    pub name: String,
    pub kind: String,
    pub root_paths: Vec<String>,
    pub parent_id: Option<String>,
    pub child_ids: Vec<String>,
    pub entry_points: Vec<String>,
    pub tags: Vec<String>,
}

/// module_source_map 对外暴露的稳定关联记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSourceLink {
    pub module_id: String,
    pub source_id: String,
}

/// facts-owned source 视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRecord {
    pub source_id: String,
    pub path: String,
    pub language: String,
    pub kind: String,
    pub tags: Vec<String>,
}

/// facts-owned entrypoint 视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntrypointRecord {
    pub path: String,
    pub source_id: Option<String>,
    pub module_ids: Vec<String>,
}

/// 符号 FTS 查询的稳定结果。
#[derive(Debug, Clone)]
pub struct SymbolSearchHit {
    pub symbol_id: String,
    pub name: String,
    pub label: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub language: String,
    pub score: f64,
}

/// 调用图扩展使用的稳定 trace 命中。
#[derive(Debug, Clone)]
pub struct CallTraceHit {
    pub edge_id: String,
    pub source_id: String,
    pub target_id: String,
    pub traversal_direction: String,
    pub hop_distance: usize,
    pub confidence: f64,
    pub reason: String,
}

/// facts / index 快照的写入与恢复合同。
pub trait IndexSnapshotStore {
    fn write_scan_report(&self, report: &ScanReport) -> io::Result<()>;
    fn read_scan_report(&self) -> io::Result<Option<ScanReport>>;

    fn write_module_tree(&self, tree: &ModuleTree) -> io::Result<()>;
    fn read_module_tree(&self) -> io::Result<Option<ModuleTree>>;
    fn list_modules(&self) -> io::Result<Vec<ModuleRecord>>;
    fn list_module_source_links(&self) -> io::Result<Vec<ModuleSourceLink>>;
    fn list_sources(&self) -> io::Result<Vec<SourceRecord>>;
    fn list_entrypoints(&self) -> io::Result<Vec<EntrypointRecord>>;

    fn replace_symbol_graph(
        &self,
        symbols: &[SymbolNode],
        edges: &[ResolvedSymbolEdge],
        analysis: &GraphAnalysisSnapshot,
        resolved_graph: &ResolvedGraphSnapshot,
    ) -> io::Result<()>;

    fn replace_symbol_graph_for_files(
        &self,
        file_paths: &[String],
        symbols: &[SymbolNode],
        edges: &[ResolvedSymbolEdge],
        analysis: &GraphAnalysisSnapshot,
        resolved_graph: &ResolvedGraphSnapshot,
    ) -> io::Result<()>;
}

/// facts / index 查询面的读取合同。
pub trait IndexQueryStore {
    fn list_symbols(&self) -> io::Result<Vec<SymbolNode>>;
    fn list_symbols_for_files(&self, source_paths: &[String]) -> io::Result<Vec<SymbolNode>>;
    fn count_symbol_files(&self) -> io::Result<usize>;
    fn search_symbols(&self, term: &str, limit: usize) -> io::Result<Vec<SymbolSearchHit>>;

    fn list_edges(&self) -> io::Result<Vec<ResolvedSymbolEdge>>;
    fn list_edges_for_files(&self, source_paths: &[String]) -> io::Result<Vec<ResolvedSymbolEdge>>;
    fn list_adjacent_symbol_files(&self, source_paths: &[String]) -> io::Result<Vec<String>>;
    fn trace_call_edges(
        &self,
        symbol_id: &str,
        max_depth: usize,
    ) -> io::Result<Vec<ResolvedSymbolEdge>>;
    fn trace_call_edges_from_seeds(
        &self,
        symbol_ids: &[String],
        max_depth: usize,
        limit: usize,
    ) -> io::Result<Vec<CallTraceHit>>;

    fn list_communities(&self) -> io::Result<Vec<CommunityNode>>;
    fn list_community_members(&self, community_id: &str) -> io::Result<Vec<CommunityMember>>;
    fn list_processes(&self) -> io::Result<Vec<ProcessNode>>;
    fn list_process_steps(&self, process_id: &str) -> io::Result<Vec<ProcessStep>>;
}
