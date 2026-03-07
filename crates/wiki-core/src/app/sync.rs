use std::fs;
use std::io;
use std::path::Path;

use crate::repo::fingerprint::fingerprint_bytes;
use crate::storage::metadata_store::{read_metadata, write_metadata};

#[derive(Debug, Clone)]
pub struct SyncReport {
    pub synced_pages: Vec<String>,
}

pub fn run_sync(repo_root: &Path) -> io::Result<SyncReport> {
    let mut metadata = read_metadata(repo_root)?;
    let mut synced_pages = Vec::new();

    for item in &mut metadata.wiki_items {
        let relative = item.path.trim_start_matches(".wiki/");
        let page_path = repo_root.join(".wiki").join(relative);
        let content = fs::read_to_string(&page_path)?;
        let new_hash = fingerprint_bytes(content.as_bytes());

        if item.content_hash != new_hash {
            item.content_hash = new_hash;
            synced_pages.push(item.path.clone());
        }
    }

    if !synced_pages.is_empty() {
        metadata.dirty_state.status = "fresh".to_string();
        write_metadata(repo_root, &metadata)?;
    }

    Ok(SyncReport { synced_pages })
}
