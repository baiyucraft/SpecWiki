use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::storage::metadata_store::read_metadata;

#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    pub matched_pages: Vec<String>,
}

pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    let metadata = read_metadata(repo_root)?;
    let needle = term.to_lowercase();
    let mut matched_pages = Vec::new();

    for item in metadata.wiki_items {
        let page_path = repo_root.join(".wiki").join(item.path.trim_start_matches(".wiki/"));
        let content = fs::read_to_string(&page_path).unwrap_or_default();

        if item.title.to_lowercase().contains(&needle)
            || item.path.to_lowercase().contains(&needle)
            || content.to_lowercase().contains(&needle)
        {
            matched_pages.push(item.path);
        }
    }

    Ok(QueryReport { matched_pages })
}
