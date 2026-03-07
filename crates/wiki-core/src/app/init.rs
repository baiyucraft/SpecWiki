use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::domain::metadata::{DirtyState, SourceFileRecord, WikiMetadata};
use crate::domain::relation::WikiRelation;
use crate::domain::wiki_item::WikiItem;
use crate::generation::planner::plan_pages;
use crate::generation::renderer::render_page;
use crate::repo::git::{current_branch, current_commit, looks_like_git_repo};
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::{ensure_cache_dir, write_scan_cache};
use crate::storage::metadata_store::write_metadata;
use crate::storage::wiki_fs::{remove_runtime, write_page};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Clone, serde::Serialize)]
pub struct InitReport {
    pub initialized: bool,
    pub state: String,
    pub generated_pages: Vec<String>,
}

pub fn run_init(repo_root: &Path) -> io::Result<InitReport> {
    if !looks_like_git_repo(repo_root) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repo root must be a git repository",
        ));
    }

    remove_runtime(repo_root)?;

    let scan_report = scan_repo(repo_root)?;
    let pages = plan_pages(&scan_report);

    ensure_cache_dir(repo_root)?;
    write_scan_cache(repo_root, &scan_report)?;

    let mut wiki_items = Vec::new();
    let mut relations = Vec::new();
    let mut generated_pages = Vec::new();
    let all_page_ids = pages.iter().map(|page| page.id.clone()).collect::<Vec<_>>();
    let all_source_files = scan_report
        .files
        .iter()
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    let generated_at = current_timestamp();

    for page in pages {
        let content = render_page(&page, &scan_report);
        write_page(repo_root, &page.relative_path, &content)?;
        let page_path = format!(".wiki/{}", page.relative_path);
        generated_pages.push(page_path.clone());

        wiki_items.push(WikiItem {
            id: page.id.clone(),
            title: page.title,
            path: page_path.clone(),
            item_type: page.page_type,
            parent_id: page.parent_id.clone(),
            source_files: all_source_files.clone(),
            content_hash: crate::repo::fingerprint::fingerprint_bytes(content.as_bytes()),
        });

        if let Some(parent_id) = page.parent_id {
            relations.push(WikiRelation {
                source_id: parent_id,
                target_id: page.id,
                relation_type: "PARENT_CHILD".to_string(),
            });
        }
    }

    let metadata = WikiMetadata {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at,
        last_indexed_commit: current_commit(repo_root),
        wiki_items,
        relations,
        source_files: scan_report
            .files
            .iter()
            .map(|file| SourceFileRecord {
                path: file.path.clone(),
                fingerprint: file.fingerprint.clone(),
                wiki_item_ids: all_page_ids.clone(),
            })
            .collect(),
        dirty_state: DirtyState::fresh(),
    };

    write_metadata(repo_root, &metadata)?;

    Ok(InitReport {
        initialized: true,
        state: metadata.dirty_state.status,
        generated_pages,
    })
}

pub(crate) fn current_timestamp() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| duration.as_secs().to_string())
                .unwrap_or_else(|_| "0".to_string())
        })
}
