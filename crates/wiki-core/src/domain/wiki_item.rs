use serde::{Deserialize, Serialize};

/// `WikiItem` 是单个 Wiki 页面的 metadata 记录。
/// 它描述页面本身，不承载源码级细节。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiItem {
    pub id: String,
    pub title: String,
    pub path: String,
    pub item_type: String,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub ancestor_ids: Vec<String>,
    #[serde(default)]
    pub module_ids: Vec<String>,
    pub source_files: Vec<String>,
    pub content_hash: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub provenance: Vec<String>,
}
