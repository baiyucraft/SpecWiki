use serde::{Deserialize, Serialize};

/// `TopicSeed` 表示 planner 可消费的稳定主题线索。
/// 它来自 hierarchy 或后续聚合输入，但在进入 planner 前仍保持纯事实型描述。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TopicSeed {
    pub topic_kind: String,
    pub topic_key: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub source_paths: Vec<String>,
    #[serde(default)]
    pub module_ids: Vec<String>,
    #[serde(default)]
    pub relation_ids: Vec<String>,
}
