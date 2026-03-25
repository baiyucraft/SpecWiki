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

/// workflow 级 runtime readiness 摘要。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineRuntimeSummary {
    pub facts_input_hash: String,
    #[serde(default)]
    pub workflow_action: String,
    #[serde(default)]
    pub runtime_state: String,
    #[serde(default)]
    pub researched_units: usize,
    #[serde(default)]
    pub compose_ready_units: usize,
    #[serde(default)]
    pub composed_units: usize,
    #[serde(default)]
    pub assembled_pages: usize,
    #[serde(default)]
    pub blocked_units: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_ready_stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_interrupted_stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_reason: Option<String>,
    /// 当前正在 research 的 unit，帮助脚本区分“还在排队”还是“某个 unit 已经跑很久”。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_research_unit_id: Option<String>,
    /// 当前 research unit 的稳定类型，便于按 unit 语义聚合慢点。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_research_unit_type: Option<String>,
    /// 当前 research unit 开始时间戳；只在 `runtime_state=researching` 时有意义。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_research_started_at: Option<String>,
    /// 最近一个完成 research 的 unit，帮助专项报告判断吞吐推进到了哪里。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_researched_unit_id: Option<String>,
    /// 最近一个完成 research 的 unit 耗时，帮助区分 provider 延迟与纯排队。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_research_elapsed_ms: Option<u64>,
}

/// unit 级 runtime gate 状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitRuntimeGate {
    pub unit_id: String,
    pub unit_type: String,
    #[serde(default)]
    pub research_status: String,
    #[serde(default)]
    pub compose_status: String,
    #[serde(default)]
    pub assemble_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_ready_stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(default)]
    pub missing_dependencies: Vec<String>,
    #[serde(default)]
    pub updated_at: String,
}

/// 计算 facts 层输入的哈希值——基于 ScanReport + ModuleTree fingerprint。
pub fn compute_facts_input_hash(
    scan_report: &wiki_index::scanner::ScanReport,
    module_tree: &crate::domain::module_tree::ModuleTree,
) -> String {
    use wiki_index::fingerprint::fingerprint_bytes;

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



