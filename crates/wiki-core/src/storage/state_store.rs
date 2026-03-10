use std::io;
use std::path::Path;

use crate::domain::state::{rebuild_state_from_metadata, WikiState};
use crate::repo::symbols::SymbolNode;
use crate::storage::metadata_store::read_metadata;
use crate::storage::sqlite_store;

/// 写入 WikiState 到 SQLite 关系型状态表。
pub fn write_state(repo_root: &Path, state: &WikiState) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_rows(&mut conn, state)
}

/// 写入 WikiState 和完整 symbol snapshot。
pub fn write_state_with_symbols(
    repo_root: &Path,
    state: &WikiState,
    symbols: &[SymbolNode],
) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_and_symbols(&mut conn, state, symbols)
}

/// 写入 WikiState，并只对指定文件刷新符号。
pub fn write_state_with_symbols_for_files(
    repo_root: &Path,
    state: &WikiState,
    file_paths: &[String],
    symbols: &[SymbolNode],
) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_and_symbols_for_files(&mut conn, state, file_paths, symbols)
}

/// 从 SQLite 关系型状态表读取 WikiState。
pub fn read_state(repo_root: &Path) -> io::Result<WikiState> {
    let conn = sqlite_store::open_db_readonly(repo_root)?;
    sqlite_store::load_state_rows(&conn)
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
