use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::metadata::DirtyState;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::hierarchy::build_module_tree;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::{write_module_tree_cache, write_scan_cache};
use crate::storage::metadata_store::{read_metadata, write_metadata};
use crate::storage::wiki_fs::resolve_page_path;
use crate::workflows::init::current_timestamp;

/// `sync` 用来接受人工修改后的 Markdown 页面，并把变化同步回 metadata/cache。
#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    pub state: String,
    pub synced_pages: Vec<String>,
}

/// 同步运行时页面状态。
/// 当前实现以内容哈希为主，先保证“手工改了页面后 metadata 不会继续过期”。
///
/// # 参数
/// - `repo_root`：包含 `.wiki/` 运行时的本地代码目录。
///
/// # 返回
/// - 成功时返回同步后的状态和本次被识别为变化的页面列表。
///
/// # 错误
/// - 当 metadata、页面文件或缓存写入失败时返回错误。
pub fn run_sync(repo_root: &Path) -> io::Result<SyncReport> {
    let mut metadata = read_metadata(repo_root)?;
    let mut synced_pages = Vec::new();

    for item in &mut metadata.wiki_items {
        let page_path = resolve_page_path(repo_root, &item.path);
        let content = fs::read_to_string(&page_path)?;
        let new_hash = fingerprint_bytes(content.as_bytes());

        if item.content_hash != new_hash {
            item.content_hash = new_hash;
            synced_pages.push(item.path.clone());
        }
    }

    metadata.generated_at = current_timestamp();
    metadata.dirty_state = DirtyState::fresh();
    write_metadata(repo_root, &metadata)?;

    // 页面被人工修改后，缓存中的扫描和模块树也一并刷新，保证 query/status 使用的是同一套事实。
    let scan_report = scan_repo(repo_root)?;
    let module_tree = build_module_tree(&scan_report);
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    Ok(SyncReport {
        state: "fresh".to_string(),
        synced_pages,
    })
}
