use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub citation_refs: Vec<String>,
    #[serde(default)]
    pub summary_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_stop_reason: Option<String>,
}

/// `DeclaredKnowledgeRecordKind` 收敛当前阶段允许进入 formal runtime 的最小声明类别。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredKnowledgeRecordKind {
    #[default]
    Policy,
    Convention,
    Pitfall,
    Decision,
}

impl DeclaredKnowledgeRecordKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Policy => "policy",
            Self::Convention => "convention",
            Self::Pitfall => "pitfall",
            Self::Decision => "decision",
        }
    }
}

/// `DeclaredKnowledgeRecordStatus` 表示 declared truth 自身的稳定状态。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredKnowledgeRecordStatus {
    #[default]
    Active,
    Superseded,
    Removed,
}

/// `DeclaredKnowledgeRecord` 是 `.wiki/.knowledge/declared/records.jsonl` 的正式对象。
/// 它必须能独立表达稳定 identity、作用范围、来源与 projection 锚点。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredKnowledgeRecord {
    pub record_id: String,
    pub record_kind: DeclaredKnowledgeRecordKind,
    pub scope_ref: String,
    pub status: DeclaredKnowledgeRecordStatus,
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub updated_at: String,
    #[serde(default)]
    pub unit_refs: Vec<String>,
    #[serde(default)]
    pub projection_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub page_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub section_id: String,
    #[serde(default)]
    pub ordinal: usize,
    #[serde(default)]
    pub body: String,
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

/// `KnowledgeHealthSignalKind` 收敛 status 可稳定消费的最小 runtime health 诊断面。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeHealthSignalKind {
    #[default]
    OrphanUnit,
    MissingProvenance,
    DerivedStale,
    ProjectionStale,
    DeclaredDerivedDivergence,
    IllegalDrift,
}

impl KnowledgeHealthSignalKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrphanUnit => "orphan_unit",
            Self::MissingProvenance => "missing_provenance",
            Self::DerivedStale => "derived_stale",
            Self::ProjectionStale => "projection_stale",
            Self::DeclaredDerivedDivergence => "declared_derived_divergence",
            Self::IllegalDrift => "illegal_drift",
        }
    }
}

/// `KnowledgeHealthSeverity` 把 health signal 收敛到最小严重度分层。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeHealthSeverity {
    Info,
    #[default]
    Warning,
    Error,
}

impl KnowledgeHealthSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// `KnowledgeHealthRecommendedAction` 为宿主暴露稳定的单值建议动作。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeHealthRecommendedAction {
    #[default]
    None,
    Sync,
    Update,
    Rebuild,
    Review,
}

impl KnowledgeHealthRecommendedAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Sync => "sync",
            Self::Update => "update",
            Self::Rebuild => "rebuild",
            Self::Review => "review",
        }
    }
}

/// `KnowledgeHealthSignal` 是 `.wiki/.knowledge/runtime/health-signals.jsonl` 的正式对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeHealthSignal {
    pub signal_id: String,
    pub signal_kind: KnowledgeHealthSignalKind,
    pub severity: KnowledgeHealthSeverity,
    pub target_ref: String,
    pub recommended_action: KnowledgeHealthRecommendedAction,
    pub reason: String,
}

/// `KnowledgeHealthSummary` 为 `status` 暴露最小聚合视图，独立于 readiness 状态机。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeHealthSummary {
    #[serde(default)]
    pub total_signals: usize,
    #[serde(default)]
    pub degraded: bool,
    #[serde(default)]
    pub counts_by_kind: BTreeMap<String, usize>,
    #[serde(default)]
    pub counts_by_severity: BTreeMap<String, usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_severity: Option<KnowledgeHealthSeverity>,
    #[serde(default)]
    pub recommended_action: KnowledgeHealthRecommendedAction,
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
