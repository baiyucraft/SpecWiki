use serde::Serialize;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use crate::domain::metadata::DirtyState;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::has_cache_layout;
use crate::storage::metadata_store::{metadata_exists, read_metadata};
use crate::storage::wiki_fs::{page_exists, wiki_root};

#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    pub state: String,
    pub dirty_sources: Vec<String>,
    pub dirty_pages: Vec<String>,
    pub needs_rebuild_reason: Option<String>,
}

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

    for source in &metadata.source_files {
        match current.get(&source.path) {
            Some(fingerprint) if fingerprint == &source.fingerprint => {}
            _ => dirty_sources.push(source.path.clone()),
        }
    }

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
