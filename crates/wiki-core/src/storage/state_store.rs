use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::domain::state::{rebuild_state_from_metadata, WikiState};
use crate::storage::cache_store::{cache_dir, ensure_cache_dir};
use crate::storage::metadata_store::read_metadata;

/// WikiState 的固定存放路径。
pub fn state_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("wiki-state.json")
}

/// 写入 WikiState 到 `.wiki/.cache/wiki-state.json`。
pub fn write_state(repo_root: &Path, state: &WikiState) -> io::Result<()> {
    ensure_cache_dir(repo_root)?;
    let json = serde_json::to_string_pretty(state)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(state_path(repo_root), json)
}

/// 读取 `.wiki/.cache/wiki-state.json`。
pub fn read_state(repo_root: &Path) -> io::Result<WikiState> {
    let json = fs::read_to_string(state_path(repo_root))?;
    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

/// 优先读 WikiState，不存在时从 WikiMetadata 重建，都不存在时返回错误。
pub fn load_or_rebuild_state(repo_root: &Path) -> io::Result<WikiState> {
    if let Ok(state) = read_state(repo_root) {
        return Ok(state);
    }

    let metadata = read_metadata(repo_root)?;
    Ok(rebuild_state_from_metadata(&metadata))
}
