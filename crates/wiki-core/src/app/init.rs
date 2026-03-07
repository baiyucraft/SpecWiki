use std::io;
use std::path::Path;

use crate::domain::metadata::{DirtyState, SourceFileRecord, WikiMetadata};
use crate::domain::wiki_item::WikiItem;
use crate::generation::planner::plan_pages;
use crate::generation::renderer::render_page;
use crate::repo::git::looks_like_git_repo;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::ensure_cache_dir;
use crate::storage::metadata_store::write_metadata;
use crate::storage::wiki_fs::write_page;

pub fn run_init(repo_root: &Path) -> io::Result<()> {
    if !looks_like_git_repo(repo_root) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repo root must be a git repository",
        ));
    }

    let scan_report = scan_repo(repo_root)?;
    let pages = plan_pages(&scan_report);

    ensure_cache_dir(repo_root)?;

    let mut wiki_items = Vec::new();

    for page in pages {
        let content = render_page(&page, &scan_report);
        write_page(repo_root, &page.relative_path, &content)?;

        wiki_items.push(WikiItem {
            id: page.id,
            title: page.title,
            path: format!(".wiki/{}", page.relative_path),
            item_type: page.page_type,
            parent_id: None,
            source_files: scan_report.files.iter().map(|file| file.path.clone()).collect(),
            content_hash: crate::repo::fingerprint::fingerprint_bytes(content.as_bytes()),
        });
    }

    let metadata = WikiMetadata {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: "unknown".to_string(),
        generated_at: "2026-03-07T00:00:00Z".to_string(),
        last_indexed_commit: "unindexed".to_string(),
        wiki_items,
        relations: Vec::new(),
        source_files: scan_report
            .files
            .iter()
            .map(|file| SourceFileRecord {
                path: file.path.clone(),
                fingerprint: file.fingerprint.clone(),
                wiki_item_ids: vec!["overview".to_string()],
            })
            .collect(),
        dirty_state: DirtyState {
            status: "fresh".to_string(),
            dirty_pages: Vec::new(),
            dirty_sources: Vec::new(),
        },
    };

    write_metadata(repo_root, &metadata)
}
