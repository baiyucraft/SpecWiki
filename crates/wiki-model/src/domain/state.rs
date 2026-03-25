use serde::{Deserialize, Serialize};

use crate::domain::metadata::DirtyState;
use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;

/// `WikiSectionState` 记录页面内部每个章节的稳定状态。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiSectionState {
    pub section_id: String,
    pub title: String,
    pub managed: bool,
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_after_section_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_before_section_id: Option<String>,
    pub source_ids: Vec<String>,
    pub relation_ids: Vec<String>,
}

/// `WikiPageState` 是内部运行时使用的页面状态模型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiPageState {
    pub page_id: String,
    pub title: String,
    pub path: String,
    pub page_type: String,
    pub parent_id: Option<String>,
    pub ancestor_ids: Vec<String>,
    pub input_hash: String,
    pub content_hash: String,
    pub source_ids: Vec<String>,
    pub source_paths: Vec<String>,
    pub module_ids: Vec<String>,
    pub summary: String,
    pub provenance: Vec<String>,
    #[serde(default)]
    pub section_anchors: Vec<String>,
    #[serde(default)]
    pub sections: Vec<WikiSectionState>,
}

/// `SourceState` 是内部运行时的源码状态模型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceState {
    pub source_id: String,
    pub path: String,
    pub fingerprint: String,
    pub page_ids: Vec<String>,
    pub module_ids: Vec<String>,
}

/// `BuildState` 记录一次构建完成后的整体摘要。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildState {
    pub generated_at: String,
    pub page_count: usize,
    pub module_count: usize,
}

/// `WikiState` 是所有 workflow 的内部事实主模型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiState {
    pub pages: Vec<WikiPageState>,
    pub sources: Vec<SourceState>,
    pub modules: Vec<ModuleNode>,
    pub relations: Vec<WikiRelation>,
    pub dirty_state: DirtyState,
    pub build_state: BuildState,
}

impl WikiPageState {
    pub fn section_ids(&self) -> Vec<String> {
        self.sections
            .iter()
            .map(|section| section.section_id.clone())
            .collect()
    }

    pub fn managed_section_anchors(&self) -> Vec<String> {
        if !self.section_anchors.is_empty() {
            return self.section_anchors.clone();
        }

        self.sections
            .iter()
            .filter(|section| section.managed)
            .map(|section| section.section_id.clone())
            .collect()
    }
}