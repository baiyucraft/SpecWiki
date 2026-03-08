use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::{write_module_tree_cache, write_scan_cache};
use crate::storage::metadata_store::write_metadata;
use crate::storage::state_store::{load_or_rebuild_state, write_state};
use crate::storage::wiki_fs::resolve_page_path;
use crate::workflows::init::current_timestamp;

/// `sync` 用来接受人工修改后的 Markdown 页面，并把变化同步回 WikiState 和 metadata。
#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    pub state: String,
    pub synced_pages: Vec<String>,
}

/// 同步运行时页面状态。
/// 读取 WikiState → 更新 content_hash → 持久化 WikiState → 通过 MetadataMapper 导出 metadata。
///
/// # 参数
/// - `repo_root`：包含 `.wiki/` 运行时的本地代码目录。
///
/// # 返回
/// - 成功时返回同步后的状态和本次被识别为变化的页面列表。
///
/// # 错误
/// - 当状态层、页面文件或缓存写入失败时返回错误。
pub fn run_sync(repo_root: &Path) -> io::Result<SyncReport> {
    let mut wiki_state = load_or_rebuild_state(repo_root)?;
    let mut synced_pages = Vec::new();

    for page in &mut wiki_state.pages {
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = fs::read_to_string(&page_path)?;
        let new_hash = fingerprint_bytes(content.as_bytes());

        if page.content_hash != new_hash {
            page.content_hash = new_hash;
            synced_pages.push(page.path.clone());
        }
    }

    let generated_at = current_timestamp();
    wiki_state.dirty_state = crate::domain::metadata::DirtyState::fresh();
    wiki_state.build_state.generated_at = generated_at.clone();
    write_state(repo_root, &wiki_state)?;

    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at,
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&wiki_state, &export_context);
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
