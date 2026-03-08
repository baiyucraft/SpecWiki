use serde::{Deserialize, Serialize};

use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;
use crate::domain::wiki_item::WikiItem;

/// `SourceFileRecord` 是 metadata 里的源码索引项。
/// 它连接源码、页面和模块三层关系，是 update/query 的基础映射表。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceFileRecord {
    pub id: String,
    pub path: String,
    pub fingerprint: String,
    pub wiki_item_ids: Vec<String>,
    #[serde(default)]
    pub module_ids: Vec<String>,
}

/// `DirtyState` 描述当前 Repo Wiki 是否过期，以及为什么过期。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirtyState {
    pub status: String,
    pub dirty_pages: Vec<String>,
    pub dirty_sources: Vec<String>,
    pub needs_rebuild_reason: Option<String>,
}

/// `WikiMetadata` 是 `.wiki/wiki.metadata.json` 的外部文件格式。
/// 它是 runtime 的正式索引层，供 status/update/query/sync 统一读取。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiMetadata {
    pub schema_version: String,
    pub language: String,
    pub repo_root: String,
    pub branch: String,
    pub generated_at: String,
    pub last_indexed_commit: String,
    #[serde(default)]
    pub modules: Vec<ModuleNode>,
    pub wiki_items: Vec<WikiItem>,
    pub relations: Vec<WikiRelation>,
    pub source_files: Vec<SourceFileRecord>,
    pub dirty_state: DirtyState,
}

impl DirtyState {
    /// `fresh` 表示运行时和源码是一致的。
    pub fn fresh() -> Self {
        Self {
            status: "fresh".to_string(),
            dirty_pages: Vec::new(),
            dirty_sources: Vec::new(),
            needs_rebuild_reason: None,
        }
    }

    /// `stale` 表示已知有源码变化，但通常还不需要全量重建。
    pub fn stale(dirty_sources: Vec<String>, dirty_pages: Vec<String>) -> Self {
        Self {
            status: "stale".to_string(),
            dirty_pages,
            dirty_sources,
            needs_rebuild_reason: None,
        }
    }

    /// `needs_rebuild` 表示运行时已经缺页或缺缓存，增量更新不足以修复。
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
    /// `sample` 主要服务测试和结构示例，不参与真实运行时流程。
    pub fn sample() -> Self {
        Self {
            schema_version: "1".to_string(),
            language: "zh".to_string(),
            repo_root: ".".to_string(),
            branch: "main".to_string(),
            generated_at: "2026-03-07T00:00:00Z".to_string(),
            last_indexed_commit: "abc123".to_string(),
            modules: vec![ModuleNode {
                id: "module-root".to_string(),
                name: "主模块".to_string(),
                kind: "application".to_string(),
                root_paths: vec![".".to_string()],
                source_ids: vec!["source-package-json".to_string()],
                parent_id: None,
                child_ids: Vec::new(),
                entry_points: Vec::new(),
                tags: vec!["frontend".to_string()],
            }],
            wiki_items: vec![WikiItem {
                id: "overview".to_string(),
                title: "项目概述".to_string(),
                path: ".wiki/项目概述.md".to_string(),
                item_type: "overview".to_string(),
                parent_id: None,
                module_ids: vec!["module-root".to_string()],
                source_files: vec!["package.json".to_string()],
                content_hash: "hash-overview".to_string(),
                summary: "仓库概览".to_string(),
            }],
            relations: vec![WikiRelation {
                source_id: "overview".to_string(),
                target_id: "package-json".to_string(),
                relation_type: "DOCUMENTS".to_string(),
                evidence: vec!["package.json".to_string()],
            }],
            source_files: vec![SourceFileRecord {
                id: "source-package-json".to_string(),
                path: "package.json".to_string(),
                fingerprint: "fingerprint-package-json".to_string(),
                wiki_item_ids: vec!["overview".to_string()],
                module_ids: vec!["module-root".to_string()],
            }],
            dirty_state: DirtyState::fresh(),
        }
    }
}
