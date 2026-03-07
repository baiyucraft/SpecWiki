use serde::{Deserialize, Serialize};

use crate::domain::relation::WikiRelation;
use crate::domain::wiki_item::WikiItem;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceFileRecord {
    pub path: String,
    pub fingerprint: String,
    pub wiki_item_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirtyState {
    pub status: String,
    pub dirty_pages: Vec<String>,
    pub dirty_sources: Vec<String>,
    pub needs_rebuild_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiMetadata {
    pub schema_version: String,
    pub language: String,
    pub repo_root: String,
    pub branch: String,
    pub generated_at: String,
    pub last_indexed_commit: String,
    pub wiki_items: Vec<WikiItem>,
    pub relations: Vec<WikiRelation>,
    pub source_files: Vec<SourceFileRecord>,
    pub dirty_state: DirtyState,
}

impl DirtyState {
    pub fn fresh() -> Self {
        Self {
            status: "fresh".to_string(),
            dirty_pages: Vec::new(),
            dirty_sources: Vec::new(),
            needs_rebuild_reason: None,
        }
    }

    pub fn stale(dirty_sources: Vec<String>, dirty_pages: Vec<String>) -> Self {
        Self {
            status: "stale".to_string(),
            dirty_pages,
            dirty_sources,
            needs_rebuild_reason: None,
        }
    }

    pub fn needs_rebuild(reason: impl Into<String>, dirty_pages: Vec<String>) -> Self {
        Self {
            status: "needs_rebuild".to_string(),
            dirty_pages,
            dirty_sources: Vec::new(),
            needs_rebuild_reason: Some(reason.into()),
        }
    }
}

impl WikiMetadata {
    pub fn sample() -> Self {
        Self {
            schema_version: "1".to_string(),
            language: "zh".to_string(),
            repo_root: ".".to_string(),
            branch: "main".to_string(),
            generated_at: "2026-03-07T00:00:00Z".to_string(),
            last_indexed_commit: "abc123".to_string(),
            wiki_items: vec![WikiItem {
                id: "overview".to_string(),
                title: "项目概述".to_string(),
                path: ".wiki/项目概述.md".to_string(),
                item_type: "overview".to_string(),
                parent_id: None,
                source_files: vec!["package.json".to_string()],
                content_hash: "hash-overview".to_string(),
            }],
            relations: vec![WikiRelation {
                source_id: "overview".to_string(),
                target_id: "package-json".to_string(),
                relation_type: "DOCUMENTS".to_string(),
            }],
            source_files: vec![SourceFileRecord {
                path: "package.json".to_string(),
                fingerprint: "fingerprint-package-json".to_string(),
                wiki_item_ids: vec!["overview".to_string()],
            }],
            dirty_state: DirtyState::fresh(),
        }
    }
}
