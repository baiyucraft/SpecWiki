use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::domain::metadata::WikiMetadata;

/// 读取 `.wiki/wiki.metadata.json`。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时返回反序列化后的 `WikiMetadata`。
///
/// # 错误
/// - 当文件读取或 JSON 解析失败时返回错误。
pub fn read_metadata(repo_root: &Path) -> io::Result<WikiMetadata> {
    let json = fs::read_to_string(metadata_path(repo_root))?;

    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

/// 把 metadata 序列化并写回磁盘。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `metadata`：要写回磁盘的 metadata 对象。
///
/// # 返回
/// - 成功时把 metadata 写到 `.wiki/wiki.metadata.json`。
///
/// # 错误
/// - 当目录创建、JSON 序列化或文件写入失败时返回错误。
pub fn write_metadata(repo_root: &Path, metadata: &WikiMetadata) -> io::Result<()> {
    let wiki_root = repo_root.join(".wiki");
    fs::create_dir_all(&wiki_root)?;

    let json =
        serde_json::to_string_pretty(metadata).map_err(|err| io::Error::other(err.to_string()))?;

    fs::write(metadata_path(repo_root), json)
}

/// metadata 的固定存放路径。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回 `.wiki/wiki.metadata.json` 的固定路径。
pub fn metadata_path(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join("wiki.metadata.json")
}

/// 判断 metadata 是否已存在。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 如果 metadata 文件存在，则返回 `true`。
pub fn metadata_exists(repo_root: &Path) -> bool {
    metadata_path(repo_root).exists()
}



