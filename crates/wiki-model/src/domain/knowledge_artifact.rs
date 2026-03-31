use serde::{Deserialize, Serialize};

/// `KnowledgeResearchSummary` 是 `.wiki/.knowledge/derived/**` 中可共享的最小 research 摘要。
/// 它只保留恢复 parent/unit contract 所需的稳定字段，不承载完整 session 或 draft 细节。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeResearchSummary {
    pub unit_id: String,
    pub unit_type: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub positioning: String,
    #[serde(default)]
    pub input_hash: String,
    #[serde(default)]
    pub key_sources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_stop_reason: Option<String>,
}

/// `KnowledgeRuntimeGateRecord` 是 `.wiki/.knowledge/runtime/**` 中的最小 gate snapshot。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeRuntimeGateRecord {
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

/// `KnowledgeRecoveryManifest` 是 `.wiki/.knowledge/runtime/recovery-manifest.json` 的文件格式。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeRecoveryManifest {
    pub schema_version: String,
    pub repo_root: String,
    pub workflow_action: String,
    pub generated_at: String,
    pub facts_input_hash: String,
    pub knowledge_snapshot_id: String,
    pub metadata_hash: String,
    #[serde(default)]
    pub page_count: usize,
    #[serde(default)]
    pub unit_count: usize,
}
