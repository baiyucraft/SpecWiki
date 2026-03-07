use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::storage::metadata_store::read_metadata;
use crate::storage::wiki_fs::resolve_page_path;

#[derive(Debug, Clone, Serialize)]
pub struct QueryMatch {
    pub page_id: String,
    pub title: String,
    pub path: String,
    pub item_type: String,
    pub source_files: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    pub term: String,
    pub matched_pages: Vec<String>,
    pub matches: Vec<QueryMatch>,
}

pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    let metadata = read_metadata(repo_root)?;
    let needle = term.trim().to_lowercase();
    let mut matched_pages = Vec::new();
    let mut matches = Vec::new();

    if needle.is_empty() {
        return Ok(QueryReport {
            term: term.to_string(),
            matched_pages,
            matches,
        });
    }

    for item in metadata.wiki_items {
        let page_path = resolve_page_path(repo_root, &item.path);
        let content = fs::read_to_string(&page_path).unwrap_or_default();
        let mut reasons = Vec::new();

        if item.title.to_lowercase().contains(&needle) {
            reasons.push("标题匹配");
        }

        if item.path.to_lowercase().contains(&needle) {
            reasons.push("路径匹配");
        }

        if content.to_lowercase().contains(&needle) {
            reasons.push("内容匹配");
        }

        if !reasons.is_empty() {
            matched_pages.push(item.path.clone());
            matches.push(QueryMatch {
                page_id: item.id,
                title: item.title,
                path: item.path,
                item_type: item.item_type,
                source_files: item.source_files,
                summary: reasons.join("、"),
            });
        }
    }

    Ok(QueryReport {
        term: term.to_string(),
        matched_pages,
        matches,
    })
}
