//! cache 存储层负责 `.wiki/.cache/` 下各类运行时缓存的读写与失效。
//! 它只处理持久化布局，不参与业务层的变化判断与页面生成。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::context::PageContext;
use crate::domain::module_tree::ModuleTree;
use crate::domain::state::WikiState;
use crate::generation::sections::SectionDraft;
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
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回 `.wiki/.cache/` 的绝对路径。
pub fn cache_dir(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join(".cache")
}

/// `repo-scan.json` 保存最近一次扫描结果。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回扫描缓存文件路径。
pub fn scan_cache_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("repo-scan.json")
}

/// `module-tree.json` 保存最近一次层级拆分结果。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回模块树缓存文件路径。
pub fn module_tree_cache_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("module-tree.json")
}

/// 每页上下文缓存目录。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回 page context cache 目录路径。
pub fn page_context_cache_dir(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("page-contexts")
}

/// 每页生成缓存目录。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回 page generation cache 目录路径。
pub fn page_generation_cache_dir(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("page-generation")
}

/// `PageContextCacheEntry` 让 update 可以按页面复用上下文输入签名。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageContextCacheEntry {
    /// 当前缓存命中的页面稳定 ID。
    pub page_id: String,
    /// 当前页面上下文对应的输入摘要。
    pub input_hash: String,
    /// 当前页面的完整上下文输入，供增量 update 复用。
    pub context: PageContext,
}

/// `PageGenerationCacheEntry` 保存按页面拆分的 section 级生成结果。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageGenerationCacheEntry {
    /// 当前缓存命中的页面稳定 ID。
    pub page_id: String,
    /// 生成这份缓存时对应的页面输入摘要。
    pub input_hash: String,
    /// 整页 Markdown 的内容摘要。
    pub content_hash: String,
    /// 本次生成得到的 section 草稿集合。
    pub sections: Vec<SectionDraft>,
}

/// 当前 cache layout 认三份关键缓存。
/// 缺任意一份，都说明 runtime 还不足以支持稳定的 status/update。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 当关键仓库级 cache 和 page cache 目录都存在时返回 `true`。
pub fn has_cache_layout(repo_root: &Path) -> bool {
    cache_dir(repo_root).exists()
        && scan_cache_path(repo_root).exists()
        && module_tree_cache_path(repo_root).exists()
        && crate::storage::state_store::state_path(repo_root).exists()
        && page_context_cache_dir(repo_root).exists()
        && page_generation_cache_dir(repo_root).exists()
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

/// 读取最近一次扫描缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时返回最近一次持久化的 `ScanReport`。
///
/// # 错误
/// - 当缓存文件不存在、不可读或反序列化失败时返回错误。
pub fn read_scan_cache(repo_root: &Path) -> io::Result<ScanReport> {
    let json = fs::read_to_string(scan_cache_path(repo_root))?;
    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
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

/// 读取最近一次模块树缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时返回最近一次持久化的 `ModuleTree`。
///
/// # 错误
/// - 当缓存文件不存在、不可读或反序列化失败时返回错误。
pub fn read_module_tree_cache(repo_root: &Path) -> io::Result<ModuleTree> {
    let json = fs::read_to_string(module_tree_cache_path(repo_root))?;
    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

/// 初始化 page context / generation 两类每页缓存目录。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时保证两类每页缓存目录都存在。
///
/// # 错误
/// - 当目录创建失败时返回错误。
pub fn ensure_page_cache_dirs(repo_root: &Path) -> io::Result<()> {
    ensure_cache_dir(repo_root)?;
    fs::create_dir_all(page_context_cache_dir(repo_root))?;
    fs::create_dir_all(page_generation_cache_dir(repo_root))
}

/// 返回指定页面的上下文缓存路径。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_id`：页面稳定 ID。
///
/// # 返回
/// - 返回指定页面的 page context cache 文件路径。
pub fn page_context_cache_path(repo_root: &Path, page_id: &str) -> PathBuf {
    page_context_cache_dir(repo_root).join(format!("{page_id}.json"))
}

/// 返回指定页面的生成缓存路径。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_id`：页面稳定 ID。
///
/// # 返回
/// - 返回指定页面的 page generation cache 文件路径。
pub fn page_generation_cache_path(repo_root: &Path, page_id: &str) -> PathBuf {
    page_generation_cache_dir(repo_root).join(format!("{page_id}.json"))
}

/// 写入单页上下文缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `entry`：要持久化的单页上下文缓存。
///
/// # 返回
/// - 成功时把缓存写入 `page-contexts/<page-id>.json`。
///
/// # 错误
/// - 当目录创建、序列化或文件写入失败时返回错误。
pub fn write_page_context_cache(
    repo_root: &Path,
    entry: &PageContextCacheEntry,
) -> io::Result<()> {
    ensure_page_cache_dirs(repo_root)?;
    let json = serde_json::to_string_pretty(entry)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(page_context_cache_path(repo_root, &entry.page_id), json)
}

/// 读取单页上下文缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_id`：页面稳定 ID。
///
/// # 返回
/// - 成功时返回对应页面的上下文缓存。
///
/// # 错误
/// - 当缓存不存在、不可读或反序列化失败时返回错误。
pub fn read_page_context_cache(repo_root: &Path, page_id: &str) -> io::Result<PageContextCacheEntry> {
    let json = fs::read_to_string(page_context_cache_path(repo_root, page_id))?;
    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

/// 写入单页生成缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `entry`：要持久化的单页生成缓存。
///
/// # 返回
/// - 成功时把缓存写入 `page-generation/<page-id>.json`。
///
/// # 错误
/// - 当目录创建、序列化或文件写入失败时返回错误。
pub fn write_page_generation_cache(
    repo_root: &Path,
    entry: &PageGenerationCacheEntry,
) -> io::Result<()> {
    ensure_page_cache_dirs(repo_root)?;
    let json = serde_json::to_string_pretty(entry)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(page_generation_cache_path(repo_root, &entry.page_id), json)
}

/// 读取单页生成缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_id`：页面稳定 ID。
///
/// # 返回
/// - 成功时返回对应页面的生成缓存。
///
/// # 错误
/// - 当缓存不存在、不可读或反序列化失败时返回错误。
pub fn read_page_generation_cache(
    repo_root: &Path,
    page_id: &str,
) -> io::Result<PageGenerationCacheEntry> {
    let json = fs::read_to_string(page_generation_cache_path(repo_root, page_id))?;
    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

/// 删除某个页面对应的所有增量缓存。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_id`：要失效的页面稳定 ID。
///
/// # 返回
/// - 成功时删除该页面对应的上下文缓存和生成缓存。
///
/// # 错误
/// - 当缓存文件删除失败时返回错误。
pub fn remove_page_caches(repo_root: &Path, page_id: &str) -> io::Result<()> {
    let context_path = page_context_cache_path(repo_root, page_id);
    if context_path.exists() {
        fs::remove_file(context_path)?;
    }

    let generation_path = page_generation_cache_path(repo_root, page_id);
    if generation_path.exists() {
        fs::remove_file(generation_path)?;
    }

    Ok(())
}

/// 逐页检查增量 runtime 所需的 cache 组件是否完整。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `state`：当前已知的页面状态集合。
///
/// # 返回
/// - 返回缺失或损坏的 cache 组件标识列表；空列表表示布局完整。
pub fn missing_incremental_cache_components(repo_root: &Path, state: &WikiState) -> Vec<String> {
    let mut missing = Vec::new();

    if !scan_cache_path(repo_root).exists() || read_scan_cache(repo_root).is_err() {
        missing.push("repo-scan".to_string());
    }

    if !module_tree_cache_path(repo_root).exists() || read_module_tree_cache(repo_root).is_err() {
        missing.push("module-tree".to_string());
    }

    if !crate::storage::state_store::state_path(repo_root).exists() {
        missing.push("wiki-state".to_string());
    }

    if !page_context_cache_dir(repo_root).exists() {
        missing.push("page-contexts".to_string());
    }

    if !page_generation_cache_dir(repo_root).exists() {
        missing.push("page-generation".to_string());
    }

    for page in &state.pages {
        if !page_context_cache_path(repo_root, &page.page_id).exists()
            || read_page_context_cache(repo_root, &page.page_id).is_err()
        {
            missing.push(format!("page-context:{}", page.page_id));
        }

        if !page_generation_cache_path(repo_root, &page.page_id).exists()
            || read_page_generation_cache(repo_root, &page.page_id).is_err()
        {
            missing.push(format!("page-generation:{}", page.page_id));
        }
    }

    missing
}
