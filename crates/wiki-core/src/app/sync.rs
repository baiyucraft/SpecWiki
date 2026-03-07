use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::app::init::current_timestamp;
use crate::domain::metadata::DirtyState;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::write_scan_cache;
use crate::storage::metadata_store::{read_metadata, write_metadata};
use crate::storage::wiki_fs::resolve_page_path;

#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    pub state: String,
    pub synced_pages: Vec<String>,
}

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

    let scan_report = scan_repo(repo_root)?;
    write_scan_cache(repo_root, &scan_report)?;

    Ok(SyncReport {
        state: "fresh".to_string(),
        synced_pages,
    })
}
