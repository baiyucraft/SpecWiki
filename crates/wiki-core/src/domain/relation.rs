use serde::{Deserialize, Serialize};

/// `WikiRelation` 是 metadata 中统一的关系表示。
/// 页面父子关系、模块依赖关系都会落成这个结构。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}
