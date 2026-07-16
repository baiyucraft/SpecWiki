use serde::{Deserialize, Serialize};

/// Pipeline working state 的完整复用身份。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineResumeIdentity {
    pub contract_version: String,
    pub workflow_action: String,
    pub facts_input_hash: String,
    pub knowledge_tree_hash: String,
    pub research_contract_hash: String,
    pub resume_key: String,
}

impl PipelineResumeIdentity {
    pub fn new(
        workflow_action: impl Into<String>,
        facts_input_hash: impl Into<String>,
        knowledge_tree_hash: impl Into<String>,
        research_contract_hash: impl Into<String>,
    ) -> Self {
        let workflow_action = workflow_action.into();
        let facts_input_hash = facts_input_hash.into();
        let knowledge_tree_hash = knowledge_tree_hash.into();
        let research_contract_hash = research_contract_hash.into();
        let contract_version = "pipeline-resume-v1".to_string();
        let resume_key = crate::domain::stable_id::stable_id(
            "resume",
            format!(
                "{contract_version}:{workflow_action}:{facts_input_hash}:{knowledge_tree_hash}:{research_contract_hash}"
            ),
        );
        Self {
            contract_version,
            workflow_action,
            facts_input_hash,
            knowledge_tree_hash,
            research_contract_hash,
            resume_key,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.contract_version == "pipeline-resume-v1"
            && !self.workflow_action.is_empty()
            && !self.facts_input_hash.is_empty()
            && !self.knowledge_tree_hash.is_empty()
            && !self.research_contract_hash.is_empty()
            && *self
                == Self::new(
                    &self.workflow_action,
                    &self.facts_input_hash,
                    &self.knowledge_tree_hash,
                    &self.research_contract_hash,
                )
    }
}

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

    #[allow(clippy::should_implement_trait)]
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
    pub resume_identity: PipelineResumeIdentity,
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
        resume_identity: PipelineResumeIdentity,
        stage: PipelineStage,
        target_id: Option<String>,
        error_message: Option<String>,
    ) -> Self {
        Self {
            checkpoint_id: crate::domain::stable_id::stable_id(
                "checkpoint",
                format!("{:?}", std::time::SystemTime::now()),
            ),
            facts_input_hash: resume_identity.facts_input_hash.clone(),
            resume_identity,
            interrupted_stage: stage,
            interrupted_target_id: target_id,
            error_message,
        }
    }
}

/// workflow 级 runtime readiness 摘要。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineRuntimeSummary {
    #[serde(default)]
    pub resume_identity: PipelineResumeIdentity,
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

pub fn compute_knowledge_tree_hash(
    tree: &wiki_model::domain::knowledge::KnowledgeTree,
) -> std::io::Result<String> {
    let mut normalized = tree.clone();
    normalized.processing_order = normalized.units.keys().cloned().collect();
    for unit in normalized.units.values_mut() {
        unit.updated_at.clear();
        unit.child_unit_ids.sort();
        unit.planner_signal_bundles
            .sort_by(|left, right| left.key.cmp(&right.key));
        unit.scope.module_ids.sort();
        unit.scope.source_ids.sort();
        unit.scope.symbol_ids.sort();
        unit.scope.relation_ids.sort();
        unit.declared_record_refs.sort();
        unit.projection_refs.sort();
        unit.source_refs.sort();
        unit.citation_refs.sort();
    }
    for domain in normalized.domains.values_mut() {
        domain.source_modules.sort();
        domain.source_files.sort();
    }
    let serialized = serde_json::to_vec(&normalized).map_err(|error| {
        std::io::Error::other(format!("serialize stable knowledge tree: {error}"))
    })?;
    Ok(wiki_index::fingerprint::fingerprint_bytes(&serialized))
}

#[cfg(test)]
mod tests {
    use super::PipelineResumeIdentity;

    #[test]
    fn resume_identity_changes_for_each_contract_dimension() {
        let identity = |action, facts, tree, contract| {
            PipelineResumeIdentity::new(action, facts, tree, contract)
        };
        let base = identity("init", "facts-a", "tree-a", "contract-a");
        assert_eq!(base, identity("init", "facts-a", "tree-a", "contract-a"));
        for changed in [
            identity("rebuild", "facts-a", "tree-a", "contract-a"),
            identity("init", "facts-b", "tree-a", "contract-a"),
            identity("init", "facts-a", "tree-b", "contract-a"),
            identity("init", "facts-a", "tree-a", "contract-b"),
        ] {
            assert_ne!(base.resume_key, changed.resume_key);
        }
    }
}
