use serde::Serialize;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use crate::domain::metadata::DirtyState;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::has_cache_layout;
use crate::storage::metadata_store::{metadata_exists, read_metadata};
use crate::storage::wiki_fs::{page_exists, wiki_root};

/// `status` 只回答一件事：当前 Repo Wiki 是否仍然可用且新鲜。
#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    pub state: String,
    pub dirty_sources: Vec<String>,
    pub dirty_pages: Vec<String>,
    pub needs_rebuild_reason: Option<String>,
}

/// 检查 runtime 是否缺失、是否需要重建、是否存在脏源码。
///
/// # 参数
/// - `repo_root`：要检查的本地代码目录。
///
/// # 返回
/// - 成功时返回当前 Repo Wiki 的状态报告。
///
/// # 错误
/// - 当 metadata 或源码目录读取失败时返回错误。
pub fn run_status(repo_root: &Path) -> io::Result<StatusReport> {
    if !wiki_root(repo_root).exists() || !metadata_exists(repo_root) {
        return Ok(StatusReport {
            state: "missing".to_string(),
            dirty_sources: Vec::new(),
            dirty_pages: Vec::new(),
            needs_rebuild_reason: None,
        });
    }

    let metadata = read_metadata(repo_root)?;

    // 缺页是最直接的“需要重建”信号，因为 metadata 和磁盘已经失配。
    let missing_pages = metadata
        .wiki_items
        .iter()
        .filter(|item| !page_exists(repo_root, &item.path))
        .map(|item| item.path.clone())
        .collect::<Vec<_>>();

    if !missing_pages.is_empty() {
        let dirty_state = DirtyState::needs_rebuild("page_missing", missing_pages.clone());

        return Ok(StatusReport {
            state: dirty_state.status,
            dirty_sources: dirty_state.dirty_sources,
            dirty_pages: missing_pages,
            needs_rebuild_reason: dirty_state.needs_rebuild_reason,
        });
    }

    // cache 缺失时也直接提升到 needs_rebuild，避免 update 在半残状态上继续工作。
    if !has_cache_layout(repo_root) {
        let dirty_pages = metadata
            .wiki_items
            .iter()
            .map(|item| item.path.clone())
            .collect::<Vec<_>>();
        let dirty_state = DirtyState::needs_rebuild("cache_missing", dirty_pages.clone());

        return Ok(StatusReport {
            state: dirty_state.status,
            dirty_sources: dirty_state.dirty_sources,
            dirty_pages,
            needs_rebuild_reason: dirty_state.needs_rebuild_reason,
        });
    }

    let scan_report = scan_repo(repo_root)?;

    let current = scan_report
        .files
        .iter()
        .map(|file| (file.path.clone(), file.fingerprint.clone()))
        .collect::<BTreeMap<_, _>>();
    let known_sources = metadata
        .source_files
        .iter()
        .map(|source| source.path.clone())
        .collect::<std::collections::BTreeSet<_>>();

    let mut dirty_sources = Vec::new();

    // 先检查已知源码是否变更或消失。
    for source in &metadata.source_files {
        match current.get(&source.path) {
            Some(fingerprint) if fingerprint == &source.fingerprint => {}
            _ => dirty_sources.push(source.path.clone()),
        }
    }

    // 再检查是否出现了 metadata 里还没登记的新文件。
    for current_path in current.keys() {
        if !known_sources.contains(current_path) {
            dirty_sources.push(current_path.clone());
        }
    }

    dirty_sources.sort();
    dirty_sources.dedup();

    let dirty_pages = if dirty_sources.is_empty() {
        Vec::new()
    } else {
        metadata
            .wiki_items
            .iter()
            .filter(|item| {
                item.source_files
                    .iter()
                    .any(|source| dirty_sources.contains(source))
            })
            .map(|item| item.path.clone())
            .collect::<Vec<_>>()
    };

    let state = if dirty_sources.is_empty() { "fresh" } else { "stale" };

    Ok(StatusReport {
        state: state.to_string(),
        dirty_sources,
        dirty_pages,
        needs_rebuild_reason: None,
    })
}
