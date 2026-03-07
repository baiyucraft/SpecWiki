use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeSet {
    pub dirty_sources: Vec<String>,
    pub dirty_pages: Vec<String>,
}
