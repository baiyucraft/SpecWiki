use std::io;
use std::path::Path;

use crate::domain::state::{rebuild_state_from_metadata, WikiState};
use crate::storage::metadata_store::read_metadata;
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::sqlite_store;
use wiki_index::scanner::ScanReport;
use wiki_index::store::IndexSnapshotStore;
use wiki_index::symbol_graph::{GraphAnalysisSnapshot, ResolvedGraphSnapshot};
use wiki_index::symbols::SymbolNode;
use wiki_model::domain::module_tree::ModuleTree;

/// 写入 facts snapshot 到 SQLite index 存储。
/// 这条路径只负责 facts/index substrate，不携带 `WikiState` 或页面层写入。
pub fn write_facts_snapshot(
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.write_scan_report(scan_report)?;
    index_store.write_module_tree(module_tree)?;
    index_store.replace_symbol_graph(symbols, &resolved_graph.edges, analysis, resolved_graph)
}

/// 写入 scan/module snapshot，并按文件刷新 symbols / edges。
/// scoped update 仍然整体重算 graph-derived 结果，但 symbol/edge 按工作集覆盖。
pub fn write_facts_snapshot_for_files(
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    file_paths: &[String],
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.write_scan_report(scan_report)?;
    index_store.write_module_tree(module_tree)?;
    index_store.replace_symbol_graph_for_files(
        file_paths,
        symbols,
        &resolved_graph.edges,
        analysis,
        resolved_graph,
    )
}

/// facts snapshot 是否已经达到可查询的最小 ready 条件。
pub fn facts_snapshot_ready(repo_root: &Path) -> io::Result<bool> {
    if !sqlite_store::db_exists(repo_root) {
        return Ok(false);
    }

    let index_store = SqliteIndexStore::new(repo_root);
    let Some(scan_report) = index_store.read_scan_report()? else {
        return Ok(false);
    };
    let Some(module_tree) = index_store.read_module_tree()? else {
        return Ok(false);
    };

    let sources = index_store.list_sources()?;
    let modules = index_store.list_modules()?;
    Ok((scan_report.files.is_empty() || !sources.is_empty())
        && (module_tree.modules.is_empty() || !modules.is_empty()))
}

/// 写入 WikiState 到 SQLite 关系型状态表。
pub fn write_state(repo_root: &Path, state: &WikiState) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_rows(&mut conn, state)
}

/// 从 SQLite 关系型状态表读取 WikiState。
pub fn read_state(repo_root: &Path) -> io::Result<WikiState> {
    let index_store = SqliteIndexStore::new(repo_root);
    let module_tree = index_store.read_module_tree()?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "module-tree not found in sqlite index snapshot",
        )
    })?;
    let conn = sqlite_store::open_db_readonly(repo_root)?;
    sqlite_store::load_state_rows(&conn, &module_tree)
}

/// 优先从 SQLite 读 WikiState，不存在时从 WikiMetadata 重建，都不存在时返回错误。
/// DB 损坏时输出 warning 并回退到 metadata 重建。
pub fn load_or_rebuild_state(repo_root: &Path) -> io::Result<WikiState> {
    match read_state(repo_root) {
        Ok(state) => return Ok(state),
        Err(e) => {
            if sqlite_store::db_exists(repo_root) {
                eprintln!("[warn] failed to read WikiState from DB, falling back to metadata: {e}");
            }
        }
    }

    let metadata = read_metadata(repo_root)?;
    Ok(rebuild_state_from_metadata(&metadata))
}
