use serde::{Deserialize, Serialize};

/// 单条源码引用——精确到文件路径和行号范围。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceCitation {
    pub path: String,
    pub start_line: usize,
    pub end_line: usize,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub symbol_id: Option<String>,
    #[serde(default)]
    pub note: String,
}