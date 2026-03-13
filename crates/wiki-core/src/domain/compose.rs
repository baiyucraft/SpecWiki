use serde::{Deserialize, Serialize};

use crate::domain::research::SourceCitation;

// ─── PageDraft ──────────────────────────────────────────────

/// Compose 层的最终输出——一个完整页面的内容草稿。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDraft {
    pub page_id: String,
    pub unit_id: String,
    pub title: String,
    pub relative_path: String,
    pub sections: Vec<ComposeSectionDraft>,
    #[serde(default)]
    pub diagrams: Vec<DiagramDraft>,
    pub citation_count: usize,
}

// ─── ComposeSectionDraft ────────────────────────────────────

/// Compose 层产出的节内容——包含 Markdown 正文和源码引用。
/// 注意：与旧的 generation::sections::SectionDraft 区分，这是新 pipeline 的输出。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeSectionDraft {
    pub section_key: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub citations: Vec<SourceCitation>,
    #[serde(default)]
    pub managed: bool,
}

// ─── DiagramDraft ───────────────────────────────────────────

/// Compose 层产出的图表草稿——渲染为 Mermaid 或 ASCII。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramDraft {
    pub diagram_id: String,
    pub diagram_type: String,
    pub title: String,
    pub content: String,
}
