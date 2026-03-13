use serde::{Deserialize, Serialize};

/// Pipeline 中断的阶段标识。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStage {
    KnowledgePlanning,
    ResearchSystem,
    ResearchDomain,
    ResearchUnit,
    ComposeLeaf,
    ComposeParent,
    ComposeIndex,
    ComposeSystem,
    Assemble,
}

impl PipelineStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KnowledgePlanning => "knowledge_planning",
            Self::ResearchSystem => "research_system",
            Self::ResearchDomain => "research_domain",
            Self::ResearchUnit => "research_unit",
            Self::ComposeLeaf => "compose_leaf",
            Self::ComposeParent => "compose_parent",
            Self::ComposeIndex => "compose_index",
            Self::ComposeSystem => "compose_system",
            Self::Assemble => "assemble",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "knowledge_planning" => Some(Self::KnowledgePlanning),
            "research_system" => Some(Self::ResearchSystem),
            "research_domain" => Some(Self::ResearchDomain),
            "research_unit" => Some(Self::ResearchUnit),
            "compose_leaf" => Some(Self::ComposeLeaf),
            "compose_parent" => Some(Self::ComposeParent),
            "compose_index" => Some(Self::ComposeIndex),
            "compose_system" => Some(Self::ComposeSystem),
            "assemble" => Some(Self::Assemble),
            _ => None,
        }
    }
}

/// Pipeline 检查点——记录中断位置，以便下次恢复。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineCheckpoint {
    pub checkpoint_id: String,
    /// Facts 层输入的哈希，用于判断检查点是否仍然有效。
    pub facts_input_hash: String,
    /// 中断发生的 pipeline 阶段。
    pub interrupted_stage: PipelineStage,
    /// 中断时正在处理的 target ID（如 domain_id / unit_id）。
    pub interrupted_target_id: Option<String>,
    /// 导致中断的错误消息。
    pub error_message: Option<String>,
}

impl PipelineCheckpoint {
    pub fn new(
        facts_input_hash: impl Into<String>,
        stage: PipelineStage,
        target_id: Option<String>,
        error_message: Option<String>,
    ) -> Self {
        Self {
            checkpoint_id: crate::domain::stable_id::stable_id(
                "checkpoint",
                &format!("{:?}", std::time::SystemTime::now()),
            ),
            facts_input_hash: facts_input_hash.into(),
            interrupted_stage: stage,
            interrupted_target_id: target_id,
            error_message,
        }
    }
}

/// 计算 facts 层输入的哈希值——基于 ScanReport + ModuleTree fingerprint。
pub fn compute_facts_input_hash(
    scan_report: &crate::repo::scanner::ScanReport,
    module_tree: &crate::domain::module_tree::ModuleTree,
) -> String {
    use crate::repo::fingerprint::fingerprint_bytes;

    let mut hasher_input = Vec::new();
    for file in &scan_report.files {
        hasher_input.extend_from_slice(file.id.as_bytes());
        hasher_input.extend_from_slice(file.fingerprint.as_bytes());
    }
    for module in &module_tree.modules {
        hasher_input.extend_from_slice(module.id.as_bytes());
        hasher_input.extend_from_slice(module.name.as_bytes());
        hasher_input.extend_from_slice(module.kind.as_bytes());
    }
    fingerprint_bytes(&hasher_input)
}
