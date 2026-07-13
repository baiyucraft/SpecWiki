use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::domain::stable_id::stable_id;

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
    pub summary_status: KnowledgeResearchSummaryStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reasons: Vec<KnowledgeResearchStatusReason>,
}

/// `KnowledgeResearchSummaryStatus` 是 derived research summary 的正式状态。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeResearchSummaryStatus {
    #[default]
    Ready,
    Degraded,
    Blocked,
}

impl KnowledgeResearchSummaryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Blocked => "blocked",
        }
    }
}

/// `KnowledgeResearchStatusReasonKind` 收敛 research summary 当前允许暴露的最小原因集合。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeResearchStatusReasonKind {
    MissingSummary,
    MissingSourceRefs,
    MissingCitationRefs,
    ProviderError,
    InvalidOutput,
    CallBudgetRejected,
}

impl KnowledgeResearchStatusReasonKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingSummary => "missing_summary",
            Self::MissingSourceRefs => "missing_source_refs",
            Self::MissingCitationRefs => "missing_citation_refs",
            Self::ProviderError => "provider_error",
            Self::InvalidOutput => "invalid_output",
            Self::CallBudgetRejected => "call_budget_rejected",
        }
    }

    pub fn expected_status(self) -> KnowledgeResearchSummaryStatus {
        match self {
            Self::MissingSummary | Self::MissingSourceRefs | Self::MissingCitationRefs => {
                KnowledgeResearchSummaryStatus::Degraded
            }
            Self::ProviderError | Self::InvalidOutput | Self::CallBudgetRejected => {
                KnowledgeResearchSummaryStatus::Blocked
            }
        }
    }
}

/// `KnowledgeResearchStatusReason` 是 research summary 的最小 machine-readable 原因对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeResearchStatusReason {
    pub reason_kind: Option<KnowledgeResearchStatusReasonKind>,
    #[serde(default)]
    pub reason_message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
}

impl KnowledgeResearchStatusReason {
    pub fn canonicalize(&mut self) {
        self.reason_message = self.reason_message.trim().to_string();
        self.upstream_ref = self
            .upstream_ref
            .as_ref()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }
}

impl KnowledgeResearchSummary {
    pub fn canonicalize(&mut self) {
        self.unit_id = self.unit_id.trim().to_string();
        self.unit_type = self.unit_type.trim().to_string();
        self.title = self.title.trim().to_string();
        self.summary = self.summary.trim().to_string();
        self.positioning = self.positioning.trim().to_string();
        self.input_hash = self.input_hash.trim().to_string();
        self.key_sources = sorted_unique(&self.key_sources);
        self.source_refs = sorted_unique(&self.source_refs);
        self.citation_refs = sorted_unique(&self.citation_refs);
        for reason in &mut self.status_reasons {
            reason.canonicalize();
        }
        self.status_reasons.sort_by(|left, right| {
            (
                left.reason_kind
                    .map(|value| value.as_str())
                    .unwrap_or_default(),
                left.reason_message.as_str(),
                left.upstream_ref.as_deref().unwrap_or_default(),
            )
                .cmp(&(
                    right
                        .reason_kind
                        .map(|value| value.as_str())
                        .unwrap_or_default(),
                    right.reason_message.as_str(),
                    right.upstream_ref.as_deref().unwrap_or_default(),
                ))
        });
        self.status_reasons.dedup_by(|left, right| left == right);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.unit_id.is_empty() {
            return Err("research summary 缺少 unit_id".to_string());
        }
        if self.unit_type.is_empty() {
            return Err(format!(
                "research summary '{}' 缺少 unit_type",
                self.unit_id
            ));
        }

        match self.summary_status {
            KnowledgeResearchSummaryStatus::Ready => {
                if !self.status_reasons.is_empty() {
                    return Err(format!(
                        "research summary '{}' 为 ready 时不允许携带 status_reasons",
                        self.unit_id
                    ));
                }
            }
            KnowledgeResearchSummaryStatus::Degraded | KnowledgeResearchSummaryStatus::Blocked => {
                if self.status_reasons.is_empty() {
                    return Err(format!(
                        "research summary '{}' 为 {} 时必须携带至少一条 status_reason",
                        self.unit_id,
                        self.summary_status.as_str()
                    ));
                }
            }
        }

        for reason in &self.status_reasons {
            let Some(reason_kind) = reason.reason_kind else {
                return Err(format!(
                    "research summary '{}' 的 status_reason 缺少 reason_kind",
                    self.unit_id
                ));
            };
            if reason.reason_message.is_empty() {
                return Err(format!(
                    "research summary '{}' 的 status_reason '{}' 缺少 reason_message",
                    self.unit_id,
                    reason_kind.as_str()
                ));
            }
            if reason_kind.expected_status() != self.summary_status {
                return Err(format!(
                    "research summary '{}' 的 status_reason '{}' 与 summary_status '{}' 不一致",
                    self.unit_id,
                    reason_kind.as_str(),
                    self.summary_status.as_str()
                ));
            }
        }

        Ok(())
    }
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
    Deprecated,
    Superseded,
    Replaced,
}

impl DeclaredKnowledgeRecordStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Superseded => "superseded",
            Self::Replaced => "replaced",
        }
    }
}

/// `DeclaredKnowledgeScopeKind` 收敛 declared record 当前允许绑定的最小 scope 类型。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredKnowledgeScopeKind {
    #[default]
    Repo,
    Domain,
    Unit,
    Module,
    Page,
    Source,
}

impl DeclaredKnowledgeScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Repo => "repo",
            Self::Domain => "domain",
            Self::Unit => "unit",
            Self::Module => "module",
            Self::Page => "page",
            Self::Source => "source",
        }
    }
}

/// `DeclaredKnowledgeScope` 是 declared record 的最小 typed scope object。
/// 它取代松散字符串 scope，供 artifact、restore 与 workflow 一致消费。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredKnowledgeScope {
    pub kind: DeclaredKnowledgeScopeKind,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<String>,
}

impl DeclaredKnowledgeScope {
    pub fn canonicalize(&mut self) {
        self.r#ref = self.r#ref.trim().to_string();
        let mut selectors = self
            .selectors
            .iter()
            .map(|selector| selector.trim().to_string())
            .filter(|selector| !selector.is_empty())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        selectors.sort();
        self.selectors = selectors;
    }

    pub fn canonical_key(&self) -> String {
        let mut normalized = self.clone();
        normalized.canonicalize();
        let selectors = if normalized.selectors.is_empty() {
            String::new()
        } else {
            format!("?{}", normalized.selectors.join(","))
        };
        format!(
            "{}:{}{}",
            normalized.kind.as_str(),
            normalized.r#ref,
            selectors
        )
    }
}

/// `DeclaredKnowledgeRelationKind` 定义当前 formal lifecycle 允许持久化的最小关系集合。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredKnowledgeRelationKind {
    Supersedes,
    ReplacedBy,
    Deprecated,
}

impl DeclaredKnowledgeRelationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supersedes => "supersedes",
            Self::ReplacedBy => "replaced_by",
            Self::Deprecated => "deprecated",
        }
    }
}

/// `DeclaredKnowledgeRelation` 保留一条稳定的 lifecycle 引用。
/// 本轮只承诺单向 formal relation，不自动补反向边。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredKnowledgeRelation {
    pub relation_kind: DeclaredKnowledgeRelationKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_record_ref: Option<String>,
}

impl DeclaredKnowledgeRelation {
    pub fn canonicalize(&mut self) {
        self.target_record_ref = self
            .target_record_ref
            .take()
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty());
    }

    pub fn canonical_key(&self) -> String {
        let mut normalized = self.clone();
        normalized.canonicalize();
        match normalized.target_record_ref.as_deref() {
            Some(target) => format!("{}:{target}", normalized.relation_kind.as_str()),
            None => normalized.relation_kind.as_str().to_string(),
        }
    }
}

/// `DeclaredKnowledgeRecord` 是 `.wiki/.knowledge/declared/records.jsonl` 的正式对象。
/// 它必须能独立表达稳定 identity、作用范围、来源与 projection 锚点。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredKnowledgeRecord {
    pub record_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub authoring_id: String,
    pub record_kind: DeclaredKnowledgeRecordKind,
    pub scope: DeclaredKnowledgeScope,
    pub status: DeclaredKnowledgeRecordStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<DeclaredKnowledgeRelation>,
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

impl DeclaredKnowledgeRecord {
    pub fn record_id_from_authoring_id(authoring_id: &str) -> String {
        stable_id("declared", authoring_id)
    }

    pub fn canonicalize(&mut self) {
        self.authoring_id = self.authoring_id.trim().to_string();
        self.scope.canonicalize();
        let mut relations = self.relations.clone();
        for relation in &mut relations {
            relation.canonicalize();
        }
        relations.sort_by(|left, right| left.canonical_key().cmp(&right.canonical_key()));
        relations.dedup_by(|left, right| left.canonical_key() == right.canonical_key());
        self.relations = relations;
        self.source_ref = self.source_ref.trim().to_string();
        self.unit_refs = sorted_unique(&self.unit_refs);
        self.projection_refs = sorted_unique(&self.projection_refs);
    }

    pub fn validate_lifecycle(&self) -> Result<(), String> {
        let deprecated = self
            .relations
            .iter()
            .filter(|relation| relation.relation_kind == DeclaredKnowledgeRelationKind::Deprecated)
            .count();
        let replaced_by = self
            .relations
            .iter()
            .filter(|relation| relation.relation_kind == DeclaredKnowledgeRelationKind::ReplacedBy)
            .count();
        let supersedes = self
            .relations
            .iter()
            .filter(|relation| relation.relation_kind == DeclaredKnowledgeRelationKind::Supersedes)
            .count();

        if deprecated > 1 {
            return Err(format!(
                "declared record '{}' 同时包含多个 deprecated relation",
                self.record_id
            ));
        }
        if deprecated > 0 && (replaced_by > 0 || supersedes > 0) {
            return Err(format!(
                "declared record '{}' 不允许同时混用 deprecated 与 replaced/supersedes relation",
                self.record_id
            ));
        }
        if replaced_by > 0 && supersedes > 0 {
            return Err(format!(
                "declared record '{}' 不允许同时包含 replaced_by 与 supersedes relation",
                self.record_id
            ));
        }

        match self.status {
            DeclaredKnowledgeRecordStatus::Active => {
                if deprecated > 0 || replaced_by > 0 || supersedes > 0 {
                    return Err(format!(
                        "declared record '{}' 为 active 时不能携带 lifecycle relation",
                        self.record_id
                    ));
                }
            }
            DeclaredKnowledgeRecordStatus::Deprecated => {
                if deprecated != 1 || replaced_by > 0 || supersedes > 0 {
                    return Err(format!(
                        "declared record '{}' 为 deprecated 时只能存在单条 deprecated relation",
                        self.record_id
                    ));
                }
            }
            DeclaredKnowledgeRecordStatus::Superseded => {
                if replaced_by == 0 || deprecated > 0 || supersedes > 0 {
                    return Err(format!(
                        "declared record '{}' 为 superseded 时必须且只能使用 replaced_by relation",
                        self.record_id
                    ));
                }
            }
            DeclaredKnowledgeRecordStatus::Replaced => {
                if supersedes == 0 || deprecated > 0 || replaced_by > 0 {
                    return Err(format!(
                        "declared record '{}' 为 replaced 时必须且只能使用 supersedes relation",
                        self.record_id
                    ));
                }
            }
        }

        for relation in &self.relations {
            match relation.relation_kind {
                DeclaredKnowledgeRelationKind::Deprecated => {
                    if relation.target_record_ref.is_some() {
                        return Err(format!(
                            "declared record '{}' 的 deprecated relation 不允许带 target",
                            self.record_id
                        ));
                    }
                }
                DeclaredKnowledgeRelationKind::Supersedes
                | DeclaredKnowledgeRelationKind::ReplacedBy => {
                    let Some(target) = relation.target_record_ref.as_ref() else {
                        return Err(format!(
                            "declared record '{}' 的 relation '{}' 缺少 target",
                            self.record_id,
                            relation.relation_kind.as_str()
                        ));
                    };
                    if target == &self.authoring_id || target == &self.record_id {
                        return Err(format!(
                            "declared record '{}' 不允许 self-target relation",
                            self.record_id
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

/// `KnowledgeConflictKind` 收敛第一批 deterministic governance conflict 种类。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeConflictKind {
    #[default]
    ParallelActiveDeclared,
    LifecycleHeadAmbiguity,
}

impl KnowledgeConflictKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ParallelActiveDeclared => "parallel_active_declared",
            Self::LifecycleHeadAmbiguity => "lifecycle_head_ambiguity",
        }
    }
}

/// `KnowledgeConflictStatus` 限定当前 conflict artifact 的生命周期状态。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeConflictStatus {
    #[default]
    Open,
}

impl KnowledgeConflictStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
        }
    }
}

/// `KnowledgeConflictRecord` 是 `.wiki/.knowledge/runtime/conflict-records.jsonl` 的正式对象。
/// 它是从 formal snapshots 推导出的治理诊断对象，而不是新的 truth source。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeConflictRecord {
    pub conflict_id: String,
    pub conflict_kind: KnowledgeConflictKind,
    #[serde(default)]
    pub status: KnowledgeConflictStatus,
    #[serde(default)]
    pub severity: KnowledgeHealthSeverity,
    pub scope: DeclaredKnowledgeScope,
    #[serde(default)]
    pub record_ids: Vec<String>,
    #[serde(default)]
    pub authoring_ids: Vec<String>,
    #[serde(default)]
    pub unit_refs: Vec<String>,
    #[serde(default)]
    pub projection_refs: Vec<String>,
    #[serde(default)]
    pub reason: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub detected_at: String,
}

impl KnowledgeConflictRecord {
    pub fn canonicalize(&mut self) {
        self.conflict_id = self.conflict_id.trim().to_string();
        self.scope.canonicalize();
        self.record_ids = sorted_unique(&self.record_ids);
        self.authoring_ids = sorted_unique(&self.authoring_ids);
        self.unit_refs = sorted_unique(&self.unit_refs);
        self.projection_refs = sorted_unique(&self.projection_refs);
        self.reason = self.reason.trim().to_string();
        self.detected_at = self.detected_at.trim().to_string();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.conflict_id.is_empty() {
            return Err("knowledge conflict 缺少 conflict_id".to_string());
        }
        if self.scope.r#ref.trim().is_empty() {
            return Err(format!(
                "knowledge conflict '{}' 缺少 scope ref",
                self.conflict_id
            ));
        }
        if self.status != KnowledgeConflictStatus::Open {
            return Err(format!(
                "knowledge conflict '{}' 当前仅允许 open status",
                self.conflict_id
            ));
        }
        if self.record_ids.len() < 2 {
            return Err(format!(
                "knowledge conflict '{}' 至少需要两条 record_ids",
                self.conflict_id
            ));
        }
        if self.authoring_ids.len() < 2 {
            return Err(format!(
                "knowledge conflict '{}' 至少需要两条 authoring_ids",
                self.conflict_id
            ));
        }
        if self.reason.is_empty() {
            return Err(format!(
                "knowledge conflict '{}' 缺少 reason",
                self.conflict_id
            ));
        }

        Ok(())
    }
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
    GovernanceConflict,
    DeclaredDerivedDivergence,
    DeclaredLifecycle,
    IllegalDrift,
}

impl KnowledgeHealthSignalKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrphanUnit => "orphan_unit",
            Self::MissingProvenance => "missing_provenance",
            Self::DerivedStale => "derived_stale",
            Self::ProjectionStale => "projection_stale",
            Self::GovernanceConflict => "governance_conflict",
            Self::DeclaredDerivedDivergence => "declared_derived_divergence",
            Self::DeclaredLifecycle => "declared_lifecycle",
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

/// `CommittedSnapshotManifest` 是 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml` 的文件格式。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommittedSnapshotManifest {
    pub schema_version: String,
    pub snapshot_id: String,
    pub repo_root: String,
    pub workflow_action: String,
    pub generated_at: String,
    pub facts_input_hash: String,
    #[serde(default)]
    pub graph_snapshot_id: String,
    pub knowledge_snapshot_id: String,
    pub declared_snapshot_id: String,
    #[serde(default)]
    pub projection_snapshot_id: String,
    pub metadata_hash: String,
    #[serde(default)]
    pub page_hashes: BTreeMap<String, String>,
    #[serde(default)]
    pub projection_digest_refs: Vec<String>,
    #[serde(default)]
    pub runtime_gate_refs: Vec<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub page_count: usize,
    #[serde(default)]
    pub unit_count: usize,
}

fn sorted_unique(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn validate_research_summary_snapshot(
    summaries: &[KnowledgeResearchSummary],
) -> Result<(), String> {
    let mut seen_unit_ids = BTreeSet::new();

    for summary in summaries {
        let mut normalized = summary.clone();
        normalized.canonicalize();
        normalized.validate()?;
        if !seen_unit_ids.insert(normalized.unit_id.clone()) {
            return Err(format!(
                "research summary unit_id 冲突: '{}'",
                normalized.unit_id
            ));
        }
    }

    Ok(())
}

pub fn validate_declared_record_snapshot(
    records: &[DeclaredKnowledgeRecord],
) -> Result<(), String> {
    let mut records_by_id = BTreeMap::<String, &DeclaredKnowledgeRecord>::new();
    let mut authoring_ids = BTreeSet::new();
    let mut kind_and_scope_by_authoring_id =
        BTreeMap::<String, (DeclaredKnowledgeRecordKind, String)>::new();
    let mut outgoing_edges = BTreeMap::<String, Vec<String>>::new();

    for record in records {
        if record.record_id.trim().is_empty() {
            return Err("declared record 缺少 record_id".to_string());
        }
        if record.authoring_id.trim().is_empty() {
            return Err(format!(
                "declared record '{}' 缺少 authoring_id",
                record.record_id
            ));
        }

        let existing = records_by_id.insert(record.record_id.clone(), record);
        if let Some(previous) = existing {
            return Err(format!(
                "declared record id 冲突: '{}' 同时出现在 '{}' 和 '{}'",
                record.record_id, previous.page_id, record.page_id
            ));
        }
        if !authoring_ids.insert(record.authoring_id.clone()) {
            return Err(format!(
                "declared authoring_id 冲突: '{}'",
                record.authoring_id
            ));
        }
        record.validate_lifecycle()?;
        kind_and_scope_by_authoring_id.insert(
            record.authoring_id.clone(),
            (record.record_kind, record.scope.canonical_key()),
        );
    }

    for record in records {
        for relation in &record.relations {
            let Some(target) = relation.target_record_ref.as_ref() else {
                continue;
            };
            let Some((target_kind, target_scope)) = kind_and_scope_by_authoring_id.get(target)
            else {
                return Err(format!(
                    "declared record '{}' 指向了不存在的 target '{}'",
                    record.record_id, target
                ));
            };
            if *target_kind != record.record_kind {
                return Err(format!(
                    "declared record '{}' 的 relation target '{}' kind 不一致",
                    record.record_id, target
                ));
            }
            if *target_scope != record.scope.canonical_key() {
                return Err(format!(
                    "declared record '{}' 的 relation target '{}' scope 不一致",
                    record.record_id, target
                ));
            }
            outgoing_edges
                .entry(record.authoring_id.clone())
                .or_default()
                .push(target.clone());
        }
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for authoring_id in authoring_ids {
        if detect_declared_cycle(&authoring_id, &outgoing_edges, &mut visiting, &mut visited) {
            return Err(format!(
                "declared lifecycle relation 不允许形成环: '{}'",
                authoring_id
            ));
        }
    }

    Ok(())
}

pub fn validate_conflict_record_snapshot(
    conflicts: &[KnowledgeConflictRecord],
    declared_records: &[DeclaredKnowledgeRecord],
) -> Result<(), String> {
    let declared_by_record_id = declared_records
        .iter()
        .map(|record| (record.record_id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let declared_by_authoring_id = declared_records
        .iter()
        .map(|record| (record.authoring_id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let mut seen_conflict_ids = BTreeSet::new();

    for conflict in conflicts {
        let mut normalized = conflict.clone();
        normalized.canonicalize();
        normalized.validate()?;
        if !seen_conflict_ids.insert(normalized.conflict_id.clone()) {
            return Err(format!(
                "knowledge conflict id 冲突: '{}'",
                normalized.conflict_id
            ));
        }

        let mut expected_scope: Option<String> = None;
        let mut unit_refs = BTreeSet::new();
        let mut projection_refs = BTreeSet::new();

        for record_id in &normalized.record_ids {
            let Some(record) = declared_by_record_id.get(record_id.as_str()) else {
                return Err(format!(
                    "knowledge conflict '{}' 引用了不存在的 record '{}'",
                    normalized.conflict_id, record_id
                ));
            };
            let scope_key = record.scope.canonical_key();
            if let Some(expected) = expected_scope.as_ref() {
                if expected != &scope_key {
                    return Err(format!(
                        "knowledge conflict '{}' 引用了不同 canonical scope 的 declared records",
                        normalized.conflict_id
                    ));
                }
            } else {
                expected_scope = Some(scope_key);
            }
            unit_refs.extend(record.unit_refs.iter().cloned());
            projection_refs.extend(record.projection_refs.iter().cloned());
        }

        for authoring_id in &normalized.authoring_ids {
            let Some(record) = declared_by_authoring_id.get(authoring_id.as_str()) else {
                return Err(format!(
                    "knowledge conflict '{}' 引用了不存在的 authoring_id '{}'",
                    normalized.conflict_id, authoring_id
                ));
            };
            if !normalized
                .record_ids
                .iter()
                .any(|record_id| record_id == &record.record_id)
            {
                return Err(format!(
                    "knowledge conflict '{}' 的 authoring_id '{}' 未对齐 record_ids",
                    normalized.conflict_id, authoring_id
                ));
            }
        }

        if normalized.scope.canonical_key()
            != expected_scope.unwrap_or_else(|| normalized.scope.canonical_key())
        {
            return Err(format!(
                "knowledge conflict '{}' 的 scope 与 declared records 不一致",
                normalized.conflict_id
            ));
        }
        if normalized.unit_refs != unit_refs.into_iter().collect::<Vec<_>>() {
            return Err(format!(
                "knowledge conflict '{}' 的 unit_refs 与 declared snapshot 不一致",
                normalized.conflict_id
            ));
        }
        if normalized.projection_refs != projection_refs.into_iter().collect::<Vec<_>>() {
            return Err(format!(
                "knowledge conflict '{}' 的 projection_refs 与 declared snapshot 不一致",
                normalized.conflict_id
            ));
        }
    }

    Ok(())
}

fn detect_declared_cycle(
    node: &str,
    edges: &BTreeMap<String, Vec<String>>,
    visiting: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node.to_string()) {
        return true;
    }
    if let Some(targets) = edges.get(node) {
        for target in targets {
            if detect_declared_cycle(target, edges, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node.to_string());
    false
}

#[cfg(test)]
mod tests {
    use super::{
        validate_conflict_record_snapshot, validate_declared_record_snapshot,
        validate_research_summary_snapshot, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordKind,
        DeclaredKnowledgeRecordStatus, DeclaredKnowledgeRelation, DeclaredKnowledgeRelationKind,
        DeclaredKnowledgeScope, DeclaredKnowledgeScopeKind, KnowledgeConflictKind,
        KnowledgeConflictRecord, KnowledgeConflictStatus, KnowledgeHealthSeverity,
        KnowledgeResearchStatusReason, KnowledgeResearchStatusReasonKind, KnowledgeResearchSummary,
        KnowledgeResearchSummaryStatus,
    };

    fn sample_scope() -> DeclaredKnowledgeScope {
        DeclaredKnowledgeScope {
            kind: DeclaredKnowledgeScopeKind::Repo,
            r#ref: "repo".to_string(),
            selectors: vec!["beta".to_string(), "alpha".to_string()],
        }
    }

    fn sample_record(authoring_id: &str) -> DeclaredKnowledgeRecord {
        let mut record = DeclaredKnowledgeRecord {
            record_id: DeclaredKnowledgeRecord::record_id_from_authoring_id(authoring_id),
            authoring_id: authoring_id.to_string(),
            record_kind: DeclaredKnowledgeRecordKind::Policy,
            scope: sample_scope(),
            status: DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: "2026-04-14T00:00:00Z".to_string(),
            unit_refs: vec!["unit-b".to_string(), "unit-a".to_string()],
            projection_refs: vec!["page:b".to_string(), "page:a".to_string()],
            page_id: "overview".to_string(),
            section_id: "section:intro".to_string(),
            ordinal: 0,
            body: "body".to_string(),
        };
        record.canonicalize();
        record
    }

    fn sample_research_summary() -> KnowledgeResearchSummary {
        KnowledgeResearchSummary {
            unit_id: "unit-runtime".to_string(),
            unit_type: "module_doc".to_string(),
            title: "运行时".to_string(),
            summary: "负责运行时主流程。".to_string(),
            positioning: "解释运行时主入口。".to_string(),
            input_hash: "research-hash".to_string(),
            key_sources: vec!["src/runtime.rs".to_string()],
            source_refs: vec!["src/runtime.rs".to_string()],
            citation_refs: vec!["source-runtime".to_string()],
            summary_status: KnowledgeResearchSummaryStatus::Ready,
            status_reasons: Vec::new(),
        }
    }

    #[test]
    fn declared_scope_canonical_key_is_stable() {
        let mut scope = sample_scope();
        scope.canonicalize();
        assert_eq!(
            scope.selectors,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        assert_eq!(scope.canonical_key(), "repo:repo?alpha,beta");
    }

    #[test]
    fn declared_record_validates_single_meaning_lifecycle() {
        let mut record = sample_record("marker:rule-new");
        record.status = DeclaredKnowledgeRecordStatus::Replaced;
        record.relations = vec![DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::Supersedes,
            target_record_ref: Some("marker:rule-old".to_string()),
        }];
        record.canonicalize();

        assert!(record.validate_lifecycle().is_ok());
    }

    #[test]
    fn declared_record_rejects_conflicting_lifecycle_inputs() {
        let mut record = sample_record("marker:rule-old");
        record.status = DeclaredKnowledgeRecordStatus::Deprecated;
        record.relations = vec![
            DeclaredKnowledgeRelation {
                relation_kind: DeclaredKnowledgeRelationKind::Deprecated,
                target_record_ref: None,
            },
            DeclaredKnowledgeRelation {
                relation_kind: DeclaredKnowledgeRelationKind::ReplacedBy,
                target_record_ref: Some("marker:rule-new".to_string()),
            },
        ];
        record.canonicalize();

        assert!(record.validate_lifecycle().is_err());
    }

    #[test]
    fn declared_snapshot_rejects_relation_cycles() {
        let mut left = sample_record("marker:left");
        left.status = DeclaredKnowledgeRecordStatus::Replaced;
        left.relations = vec![DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::Supersedes,
            target_record_ref: Some("marker:right".to_string()),
        }];
        left.canonicalize();

        let mut right = sample_record("marker:right");
        right.status = DeclaredKnowledgeRecordStatus::Replaced;
        right.relations = vec![DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::Supersedes,
            target_record_ref: Some("marker:left".to_string()),
        }];
        right.canonicalize();

        assert!(validate_declared_record_snapshot(&[left, right]).is_err());
    }

    #[test]
    fn research_summary_snapshot_accepts_ready_summary_without_reasons() {
        let summary = sample_research_summary();
        assert!(validate_research_summary_snapshot(&[summary]).is_ok());
    }

    #[test]
    fn research_summary_snapshot_rejects_ready_summary_with_reason() {
        let mut summary = sample_research_summary();
        summary.status_reasons.push(KnowledgeResearchStatusReason {
            reason_kind: Some(KnowledgeResearchStatusReasonKind::MissingSummary),
            reason_message: "摘要缺失".to_string(),
            upstream_ref: None,
        });

        assert!(validate_research_summary_snapshot(&[summary]).is_err());
    }

    #[test]
    fn research_summary_snapshot_rejects_duplicate_unit_ids() {
        let left = sample_research_summary();
        let mut right = sample_research_summary();
        right.title = "运行时二".to_string();

        assert!(validate_research_summary_snapshot(&[left, right]).is_err());
    }

    #[test]
    fn conflict_snapshot_accepts_declared_aligned_conflict() {
        let left = sample_record("marker:left");
        let right = sample_record("marker:right");
        let mut conflict = KnowledgeConflictRecord {
            conflict_id: "conflict-repo-policy".to_string(),
            conflict_kind: KnowledgeConflictKind::ParallelActiveDeclared,
            status: KnowledgeConflictStatus::Open,
            severity: KnowledgeHealthSeverity::Warning,
            scope: sample_scope(),
            record_ids: vec![left.record_id.clone(), right.record_id.clone()],
            authoring_ids: vec![left.authoring_id.clone(), right.authoring_id.clone()],
            unit_refs: vec!["unit-b".to_string(), "unit-a".to_string()],
            projection_refs: vec!["page:b".to_string(), "page:a".to_string()],
            reason: "同 scope 下存在多条 active declared records".to_string(),
            detected_at: "2026-04-15T00:00:00Z".to_string(),
        };
        conflict.canonicalize();

        assert!(validate_conflict_record_snapshot(&[conflict], &[left, right]).is_ok());
    }

    #[test]
    fn conflict_snapshot_rejects_missing_declared_record_ref() {
        let left = sample_record("marker:left");
        let mut conflict = KnowledgeConflictRecord {
            conflict_id: "conflict-repo-policy".to_string(),
            conflict_kind: KnowledgeConflictKind::ParallelActiveDeclared,
            status: KnowledgeConflictStatus::Open,
            severity: KnowledgeHealthSeverity::Warning,
            scope: sample_scope(),
            record_ids: vec![left.record_id.clone(), "missing-record".to_string()],
            authoring_ids: vec![left.authoring_id.clone(), "marker:missing".to_string()],
            unit_refs: vec!["unit-a".to_string()],
            projection_refs: vec!["page:a".to_string()],
            reason: "同 scope 下存在多条 active declared records".to_string(),
            detected_at: "2026-04-15T00:00:00Z".to_string(),
        };
        conflict.canonicalize();

        assert!(validate_conflict_record_snapshot(&[conflict], &[left]).is_err());
    }
}
