use std::io;
use std::path::Path;

use crate::domain::state::{rebuild_state_from_metadata, WikiState};
use wiki_index::symbol_graph::{GraphAnalysisSnapshot, ResolvedGraphSnapshot};
use wiki_index::symbols::SymbolNode;
use crate::storage::metadata_store::read_metadata;
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::sqlite_store;
use wiki_index::store::IndexSnapshotStore;

/// 写入 WikiState 到 SQLite 关系型状态表。
pub fn write_state(repo_root: &Path, state: &WikiState) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_rows(&mut conn, state)
}

/// 写入 WikiState、完整 symbol snapshot 与 symbol graph。
pub fn write_state_with_symbol_graph(
    repo_root: &Path,
    state: &WikiState,
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    write_state(repo_root, state)?;
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.replace_symbol_graph(symbols, &resolved_graph.edges, analysis, resolved_graph)
}

/// 写入 WikiState，并按文件刷新 symbols / edges，同时整体替换 graph-derived 结果。
pub fn write_state_with_symbol_graph_for_files(
    repo_root: &Path,
    state: &WikiState,
    file_paths: &[String],
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    write_state(repo_root, state)?;
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.replace_symbol_graph_for_files(
        file_paths,
        symbols,
        &resolved_graph.edges,
        analysis,
        resolved_graph,
    )
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
            // DB 存在但读取失败 → warning
            if sqlite_store::db_exists(repo_root) {
                eprintln!("[warn] failed to read WikiState from DB, falling back to metadata: {e}");
            }
        }
    }

    let metadata = read_metadata(repo_root)?;
    Ok(rebuild_state_from_metadata(&metadata))
}



