//! `wiki-knowledge` 承载 KnowledgeUnit 主线上的规划、研究与组合能力。

pub mod compose;
pub mod declared_writeback;
pub mod domain;
pub mod planning;
pub mod projection;
pub mod research;
pub mod section_contract;
pub mod store;

pub use domain::context::{ModuleContext, RepoContext};
pub use planning::KnowledgePlannerConfig;
pub use projection::{plan_pages_from_knowledge_tree, PagePlan};
pub use section_contract::{
    section_contract_slots_for_page_type, section_key_for_title, section_title_for_key,
    section_titles_for_page_type, SectionContractSlot,
};
pub use store::{KnowledgeArtifactStore, KnowledgeSnapshotStore};

/// 返回当前 Rust crate 的工作区名称。
pub fn workspace_name() -> &'static str {
    "wiki-knowledge"
}
