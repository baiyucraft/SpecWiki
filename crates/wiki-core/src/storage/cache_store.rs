use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::domain::module_tree::ModuleTree;
use crate::repo::scanner::ScanReport;

/// 确保 `.wiki/.cache/` 存在。
/// cache 是运行时复用层，不能和正式 metadata 混放。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时保证 cache 目录已存在。
///
/// # 错误
/// - 当目录创建失败时返回错误。
pub fn ensure_cache_dir(repo_root: &Path) -> io::Result<()> {
    fs::create_dir_all(cache_dir(repo_root))
}

/// 返回 cache 根目录。
pub fn cache_dir(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join(".cache")
}

/// `repo-scan.json` 保存最近一次扫描结果。
pub fn scan_cache_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("repo-scan.json")
}

/// `module-tree.json` 保存最近一次层级拆分结果。
pub fn module_tree_cache_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("module-tree.json")
}

/// 当前 cache layout 认三份关键缓存。
/// 缺任意一份，都说明 runtime 还不足以支持稳定的 status/update。
pub fn has_cache_layout(repo_root: &Path) -> bool {
    cache_dir(repo_root).exists()
        && scan_cache_path(repo_root).exists()
        && module_tree_cache_path(repo_root).exists()
        && crate::storage::state_store::state_path(repo_root).exists()
}

/// 写入扫描缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `report`：本次扫描得到的扫描报告。
///
/// # 返回
/// - 成功时把扫描缓存写到 `.wiki/.cache/repo-scan.json`。
///
/// # 错误
/// - 当目录创建、序列化或文件写入失败时返回错误。
pub fn write_scan_cache(repo_root: &Path, report: &ScanReport) -> io::Result<()> {
    ensure_cache_dir(repo_root)?;
    let json = serde_json::to_string_pretty(report)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(scan_cache_path(repo_root), json)
}

/// 写入模块树缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 成功时把模块树缓存写到 `.wiki/.cache/module-tree.json`。
///
/// # 错误
/// - 当目录创建、序列化或文件写入失败时返回错误。
pub fn write_module_tree_cache(repo_root: &Path, module_tree: &ModuleTree) -> io::Result<()> {
    ensure_cache_dir(repo_root)?;
    let json = serde_json::to_string_pretty(module_tree)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(module_tree_cache_path(repo_root), json)
}
