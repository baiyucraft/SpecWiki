use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
}
