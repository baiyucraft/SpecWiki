use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiItem {
    pub id: String,
    pub title: String,
    pub path: String,
    pub item_type: String,
    pub parent_id: Option<String>,
    pub source_files: Vec<String>,
    pub content_hash: String,
}
