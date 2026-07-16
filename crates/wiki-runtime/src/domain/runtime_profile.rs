//! runtime_profile 为宿主暴露稳定的状态、query 与 gate 投影。
//! 它只收口公开消费语义，不在这里泄漏内部 workflow 细节。

use serde::{Deserialize, Serialize};

use crate::domain::checkpoint::PipelineRuntimeSummary;
use crate::domain::checkpoint::UnitRuntimeGate;
use wiki_model::domain::governance::{
    GovernanceReadiness, GovernanceRecommendedAction, GovernanceSummary,
};
use wiki_model::domain::knowledge_artifact::{
    KnowledgeHealthRecommendedAction, KnowledgeHealthSeverity, KnowledgeHealthSignal,
    KnowledgeHealthSignalKind, KnowledgeHealthSummary,
};

/// 宿主侧查询当前阶段 runtime 是否可以直接进入 query。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryReadiness {
    Ready,
    NeedsInit,
    NeedsUpdate,
    Blocked,
}

/// 宿主看到 runtime 状态后推荐执行的下一步动作。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
    None,
    Init,
    Review,
    ReviewGovernance,
    Update,
    Rebuild,
    Sync,
}

/// `status` 只能输出预判型 LLM 模式提示，不能冒充真实执行路径。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LlmModeHint {
    ProviderConfigured,
    DeterministicDefault,
}

/// 当前 query 结果主要来自哪一层。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMode {
    IndexFirst,
    KnowledgeFirst,
    PageFallback,
    Mixed,
}

/// 当前 query 结果在宿主侧的可信度提示。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryTrust {
    Ready,
    StaleButQueryable,
    Blocked,
}

/// runtime 分层 readiness 的单层状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerReadiness {
    Ready,
    Stale,
    Missing,
    Rebuilding,
    Conflict,
    Blocked,
    NotEnabled,
}

/// 跨层融合后的整体消费状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FusionReadiness {
    Ready,
    Degraded,
    Blocked,
}

/// 当前 runtime 是否来自 restore，以及 restore 的可信层级。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RestoredLevel {
    None,
    Level1,
    Level2,
}

/// status/query 共享的机器可读 readiness 主合同。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RuntimeReadiness {
    pub index: LayerReadiness,
    pub knowledge: LayerReadiness,
    pub projection: LayerReadiness,
    pub fusion: FusionReadiness,
    pub restored_level: RestoredLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
}

impl RuntimeReadiness {
    pub fn missing(reason: impl Into<String>) -> Self {
        Self {
            index: LayerReadiness::Missing,
            knowledge: LayerReadiness::Missing,
            projection: LayerReadiness::Missing,
            fusion: FusionReadiness::Blocked,
            restored_level: RestoredLevel::None,
            snapshot_id: None,
            reasons: vec![reason.into()],
        }
    }

    pub fn ready(snapshot_id: Option<String>) -> Self {
        Self {
            index: LayerReadiness::Ready,
            knowledge: LayerReadiness::Ready,
            projection: LayerReadiness::Ready,
            fusion: FusionReadiness::Ready,
            restored_level: RestoredLevel::Level2,
            snapshot_id,
            reasons: Vec::new(),
        }
    }

    pub fn level1(snapshot_id: Option<String>, mut reasons: Vec<String>) -> Self {
        if reasons.is_empty() {
            reasons.push("level1_restore_without_index_graph".to_string());
        }
        Self {
            index: LayerReadiness::Missing,
            knowledge: LayerReadiness::Ready,
            projection: LayerReadiness::Ready,
            fusion: FusionReadiness::Degraded,
            restored_level: RestoredLevel::Level1,
            snapshot_id,
            reasons,
        }
    }

    pub fn blocked(reason: impl Into<String>) -> Self {
        Self {
            index: LayerReadiness::Blocked,
            knowledge: LayerReadiness::Blocked,
            projection: LayerReadiness::Blocked,
            fusion: FusionReadiness::Blocked,
            restored_level: RestoredLevel::None,
            snapshot_id: None,
            reasons: vec![reason.into()],
        }
    }

    pub fn query_trust(&self) -> QueryTrust {
        match self.fusion {
            FusionReadiness::Ready => QueryTrust::Ready,
            FusionReadiness::Degraded => QueryTrust::StaleButQueryable,
            FusionReadiness::Blocked => QueryTrust::Blocked,
        }
    }
}

/// `AnswerMode` 表示当前 answer assembly 的正式装配模式。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnswerMode {
    Direct,
    Degraded,
    Refuse,
}

/// `AnswerTrust` 为宿主暴露最小 answer 可信度分层。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnswerTrust {
    Grounded,
    Constrained,
    Unsupported,
}

/// `AnswerSupportingRef` 是宿主可直接消费的最小 supporting reference。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AnswerSupportingRef {
    pub ref_kind: String,
    pub ref_id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<String>,
}

/// `AnswerEnvelope` 是当前 query/answer surface 共用的最小正式 answer 壳。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AnswerEnvelope {
    pub text: String,
    pub answer_mode: AnswerMode,
    pub answer_trust: AnswerTrust,
    pub recommended_action: RecommendedAction,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_refs: Vec<AnswerSupportingRef>,
}

/// 实际长流程执行结束后回传给宿主的真实执行路径。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LlmExecutionMode {
    ProviderDirect,
    AgentBridge,
    DeterministicOnly,
}

/// `PipelineRuntimeSummary` 的宿主消费投影。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RuntimeSummaryProjection {
    pub workflow_action: String,
    pub runtime_state: String,
    pub researched_units: usize,
    pub compose_ready_units: usize,
    pub composed_units: usize,
    pub assembled_pages: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_units: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_ready_stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_reason: Option<String>,
}

/// `status/query` 共享的最小 preflight 投影。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct RuntimePreflight {
    pub facts_ready: bool,
    pub query_readiness: QueryReadiness,
    pub recommended_action: RecommendedAction,
}

/// Runtime 内部用于区分“内容是否当前”与“物理上是否仍可受限消费”的 freshness。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerFreshness {
    Current,
    Stale,
    Unknown,
    Invalid,
}

/// Runtime 内部的单层消费能力；它不直接扩张公开 query DTO。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerConsumability {
    Ready,
    Degraded,
    Missing,
    Rebuilding,
    Blocked,
}

/// Reliability reducer 当前评估的正式层和工作层闭集。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReliabilityLayer {
    Facts,
    Index,
    Declared,
    Derived,
    Projection,
    MetadataMirror,
    Cache,
}

/// 单层 assessment 同时保存 freshness 与 consumability，避免 stale 被误报为 ready。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct LayerAssessment {
    pub layer: ReliabilityLayer,
    pub freshness: LayerFreshness,
    pub consumability: LayerConsumability,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
}

/// status/query/workflow preflight 共用的内部 reliability 决策结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ReliabilityAssessment {
    pub runtime_state: String,
    pub layers: Vec<LayerAssessment>,
    pub readiness: RuntimeReadiness,
    pub preflight: RuntimePreflight,
    pub core_action: RecommendedAction,
    pub trust_ceiling: QueryTrust,
}

/// gate 摘要里保留的稳定 blocker 片段，避免宿主依赖完整内部 gate 结构。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RuntimeGateBlocker {
    pub unit_id: String,
    pub unit_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_dependencies: Vec<String>,
}

/// 当前阶段可供宿主消费的 gate 聚合视图。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RuntimeGateSummary {
    pub total_units: usize,
    pub ready_for_compose_units: usize,
    pub composed_units: usize,
    pub assembled_units: usize,
    pub blocked_units: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blockers: Vec<RuntimeGateBlocker>,
}

impl RuntimeSummaryProjection {
    pub fn from_summary(summary: PipelineRuntimeSummary) -> Self {
        Self {
            workflow_action: summary.workflow_action,
            runtime_state: summary.runtime_state,
            researched_units: summary.researched_units,
            compose_ready_units: summary.compose_ready_units,
            composed_units: summary.composed_units,
            assembled_pages: summary.assembled_pages,
            blocked_units: summary.blocked_units,
            last_ready_stage: summary.last_ready_stage,
            summary_reason: summary.summary_reason,
        }
    }
}

/// 把内部 runtime 状态字符串投影成宿主可消费的 preflight。
pub fn preflight_for_state(state: &str, facts_ready: bool) -> RuntimePreflight {
    match state {
        "fresh" => RuntimePreflight {
            facts_ready,
            query_readiness: QueryReadiness::Ready,
            recommended_action: RecommendedAction::None,
        },
        "stale" | "needs_update" => RuntimePreflight {
            facts_ready,
            query_readiness: QueryReadiness::NeedsUpdate,
            recommended_action: RecommendedAction::Update,
        },
        "runtime_incomplete" => RuntimePreflight {
            facts_ready,
            query_readiness: if facts_ready {
                QueryReadiness::NeedsUpdate
            } else {
                QueryReadiness::NeedsInit
            },
            recommended_action: if facts_ready {
                RecommendedAction::Update
            } else {
                RecommendedAction::Init
            },
        },
        "missing" => RuntimePreflight {
            facts_ready: false,
            query_readiness: QueryReadiness::NeedsInit,
            recommended_action: RecommendedAction::Init,
        },
        "blocker" => RuntimePreflight {
            facts_ready,
            query_readiness: QueryReadiness::Blocked,
            recommended_action: RecommendedAction::Rebuild,
        },
        _ => RuntimePreflight {
            facts_ready: false,
            query_readiness: QueryReadiness::Blocked,
            recommended_action: RecommendedAction::Rebuild,
        },
    }
}

/// 把已收集的 runtime state、graph 和 mirror evidence 一次投影为共享 assessment。
///
/// 该函数只负责决策，不读取文件系统。调用方必须先完成 restore 和 live evidence 收集。
pub fn assess_runtime_reliability(
    state: &str,
    graph_ready: bool,
    mirror_ready: bool,
    reason: Option<&str>,
) -> ReliabilityAssessment {
    let readiness = readiness_for_state(state, graph_ready, mirror_ready, reason);
    let preflight = preflight_for_state(state, graph_ready);
    let core_action = action_for_readiness(preflight.recommended_action, &readiness);
    let trust_ceiling = readiness.query_trust();
    let layers = assess_layers(state, graph_ready, mirror_ready, reason);

    ReliabilityAssessment {
        runtime_state: state.to_string(),
        layers,
        readiness,
        preflight,
        core_action,
        trust_ceiling,
    }
}

fn readiness_for_state(
    state: &str,
    graph_ready: bool,
    mirror_ready: bool,
    reason: Option<&str>,
) -> RuntimeReadiness {
    let reasons = || {
        reason
            .map(|value| vec![value.to_string()])
            .unwrap_or_default()
    };
    match state {
        "fresh" if graph_ready => RuntimeReadiness::ready(None),
        "fresh" if mirror_ready => RuntimeReadiness {
            index: LayerReadiness::Missing,
            knowledge: LayerReadiness::Ready,
            projection: LayerReadiness::Ready,
            fusion: FusionReadiness::Degraded,
            restored_level: RestoredLevel::Level1,
            snapshot_id: None,
            reasons: reasons(),
        },
        "stale" | "needs_update" => RuntimeReadiness {
            index: if graph_ready {
                LayerReadiness::Stale
            } else {
                LayerReadiness::Missing
            },
            knowledge: if mirror_ready {
                LayerReadiness::Stale
            } else {
                LayerReadiness::Missing
            },
            projection: if mirror_ready {
                LayerReadiness::Stale
            } else {
                LayerReadiness::Missing
            },
            fusion: FusionReadiness::Degraded,
            restored_level: if mirror_ready {
                RestoredLevel::Level1
            } else {
                RestoredLevel::None
            },
            snapshot_id: None,
            reasons: reason
                .map(|value| vec![value.to_string()])
                .unwrap_or_else(|| vec!["runtime_stale".to_string()]),
        },
        "runtime_incomplete" if mirror_ready => RuntimeReadiness {
            index: if graph_ready {
                LayerReadiness::Rebuilding
            } else {
                LayerReadiness::Missing
            },
            knowledge: LayerReadiness::Rebuilding,
            projection: LayerReadiness::Rebuilding,
            fusion: FusionReadiness::Degraded,
            restored_level: if graph_ready {
                RestoredLevel::Level2
            } else {
                RestoredLevel::Level1
            },
            snapshot_id: None,
            reasons: reason
                .map(|value| vec![value.to_string()])
                .unwrap_or_else(|| vec!["runtime_incomplete".to_string()]),
        },
        "missing" => RuntimeReadiness::missing("runtime_missing"),
        "blocker" | "needs_rebuild" => RuntimeReadiness::blocked(
            reason
                .map(str::to_string)
                .unwrap_or_else(|| "runtime_blocker".to_string()),
        ),
        _ if mirror_ready => RuntimeReadiness {
            index: if graph_ready {
                LayerReadiness::Ready
            } else {
                LayerReadiness::Missing
            },
            knowledge: LayerReadiness::Ready,
            projection: LayerReadiness::Ready,
            fusion: if graph_ready {
                FusionReadiness::Ready
            } else {
                FusionReadiness::Degraded
            },
            restored_level: if graph_ready {
                RestoredLevel::Level2
            } else {
                RestoredLevel::Level1
            },
            snapshot_id: None,
            reasons: reasons(),
        },
        _ => RuntimeReadiness {
            index: LayerReadiness::Stale,
            knowledge: LayerReadiness::Stale,
            projection: LayerReadiness::Stale,
            fusion: FusionReadiness::Degraded,
            restored_level: RestoredLevel::None,
            snapshot_id: None,
            reasons: reasons(),
        },
    }
}

fn action_for_readiness(
    current: RecommendedAction,
    readiness: &RuntimeReadiness,
) -> RecommendedAction {
    if current == RecommendedAction::Init {
        return current;
    }
    match readiness.fusion {
        FusionReadiness::Ready => current,
        FusionReadiness::Degraded => match current {
            RecommendedAction::None => RecommendedAction::Rebuild,
            action => action,
        },
        FusionReadiness::Blocked => RecommendedAction::Rebuild,
    }
}

fn assess_layers(
    state: &str,
    graph_ready: bool,
    mirror_ready: bool,
    reason: Option<&str>,
) -> Vec<LayerAssessment> {
    let stale = matches!(state, "stale" | "needs_update");
    let invalid = matches!(state, "blocker" | "needs_rebuild");
    let rebuilding = state == "runtime_incomplete";
    let layer = |layer, available: bool, follows_source: bool| {
        let (freshness, consumability) = if invalid {
            (LayerFreshness::Invalid, LayerConsumability::Blocked)
        } else if rebuilding {
            (LayerFreshness::Unknown, LayerConsumability::Rebuilding)
        } else if stale && follows_source {
            (LayerFreshness::Stale, LayerConsumability::Degraded)
        } else if available {
            (LayerFreshness::Current, LayerConsumability::Ready)
        } else {
            (LayerFreshness::Unknown, LayerConsumability::Missing)
        };
        LayerAssessment {
            layer,
            freshness,
            consumability,
            reasons: reason
                .map(|value| vec![value.to_string()])
                .unwrap_or_default(),
        }
    };

    vec![
        layer(ReliabilityLayer::Facts, graph_ready, true),
        layer(ReliabilityLayer::Index, graph_ready, true),
        layer(ReliabilityLayer::Declared, mirror_ready, false),
        layer(ReliabilityLayer::Derived, mirror_ready, true),
        layer(ReliabilityLayer::Projection, mirror_ready, true),
        layer(ReliabilityLayer::MetadataMirror, mirror_ready, false),
        layer(ReliabilityLayer::Cache, mirror_ready, true),
    ]
}

/// 当前阶段 query trust 只基于 runtime state 与 facts readiness 做最小判断。
pub fn query_trust_for(state: &str, facts_ready: bool) -> QueryTrust {
    match preflight_for_state(state, facts_ready).query_readiness {
        QueryReadiness::Ready => QueryTrust::Ready,
        QueryReadiness::NeedsUpdate => QueryTrust::StaleButQueryable,
        QueryReadiness::NeedsInit | QueryReadiness::Blocked => QueryTrust::Blocked,
    }
}

/// 把 unit gate 聚合成宿主可以稳定消费的 blocker 摘要。
pub fn summarize_runtime_gates(gates: &[UnitRuntimeGate]) -> Option<RuntimeGateSummary> {
    if gates.is_empty() {
        return None;
    }

    let blockers = gates
        .iter()
        .filter(|gate| gate.blocked_reason.is_some() || !gate.missing_dependencies.is_empty())
        .map(|gate| RuntimeGateBlocker {
            unit_id: gate.unit_id.clone(),
            unit_type: gate.unit_type.clone(),
            blocked_reason: gate.blocked_reason.clone(),
            missing_dependencies: gate.missing_dependencies.clone(),
        })
        .take(8)
        .collect::<Vec<_>>();

    Some(RuntimeGateSummary {
        total_units: gates.len(),
        ready_for_compose_units: gates
            .iter()
            .filter(|gate| gate.research_status == "ready")
            .count(),
        composed_units: gates
            .iter()
            .filter(|gate| gate.compose_status == "done")
            .count(),
        assembled_units: gates
            .iter()
            .filter(|gate| gate.assemble_status == "done")
            .count(),
        blocked_units: gates
            .iter()
            .filter(|gate| gate.blocked_reason.is_some() || !gate.missing_dependencies.is_empty())
            .count(),
        blockers,
    })
}

/// 从 runtime summary 和 gate 聚合里提炼一条宿主可读的 blocker 线索。
pub fn blocker_hint_from(
    runtime_summary: Option<&RuntimeSummaryProjection>,
    gate_summary: Option<&RuntimeGateSummary>,
) -> Option<String> {
    if let Some(summary) = runtime_summary {
        if let Some(reason) = summary.summary_reason.as_ref() {
            return Some(reason.clone());
        }

        if !summary.blocked_units.is_empty() {
            return Some(format!("blocked_units:{}", summary.blocked_units.join(",")));
        }
    }

    let blockers = gate_summary
        .map(|summary| {
            summary
                .blockers
                .iter()
                .filter_map(|blocker| {
                    blocker.blocked_reason.as_ref().map(|reason| {
                        format!("{}:{}:{}", blocker.unit_type, blocker.unit_id, reason)
                    })
                })
                .take(4)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    (!blockers.is_empty()).then(|| blockers.join("; "))
}

/// 把 formal health signals 聚合成宿主可消费的最小摘要。
pub fn summarize_health_signals(
    signals: &[KnowledgeHealthSignal],
) -> Option<KnowledgeHealthSummary> {
    if signals.is_empty() {
        return None;
    }

    let mut counts_by_kind = std::collections::BTreeMap::new();
    let mut counts_by_severity = std::collections::BTreeMap::new();
    let mut highest_severity = KnowledgeHealthSeverity::Info;
    let mut recommended_action = KnowledgeHealthRecommendedAction::None;
    let mut has_governance_conflict_review = false;

    for signal in signals {
        *counts_by_kind
            .entry(signal.signal_kind.as_str().to_string())
            .or_insert(0) += 1;
        *counts_by_severity
            .entry(signal.severity.as_str().to_string())
            .or_insert(0) += 1;

        if health_severity_rank(signal.severity) > health_severity_rank(highest_severity) {
            highest_severity = signal.severity;
        }
        if health_action_rank(signal.recommended_action) > health_action_rank(recommended_action) {
            recommended_action = signal.recommended_action;
        }
        if signal.signal_kind == KnowledgeHealthSignalKind::GovernanceConflict
            && matches!(
                signal.recommended_action,
                KnowledgeHealthRecommendedAction::Review
                    | KnowledgeHealthRecommendedAction::ReviewGovernance
            )
        {
            has_governance_conflict_review = true;
        }
    }

    if has_governance_conflict_review
        && recommended_action != KnowledgeHealthRecommendedAction::Rebuild
    {
        recommended_action = KnowledgeHealthRecommendedAction::ReviewGovernance;
    }

    Some(KnowledgeHealthSummary {
        total_signals: signals.len(),
        degraded: true,
        counts_by_kind,
        counts_by_severity,
        highest_severity: Some(highest_severity),
        recommended_action,
    })
}

/// 将 formal health evidence 投影回公共 readiness，不改变无关 core layer。
pub fn merge_health_readiness(
    mut readiness: RuntimeReadiness,
    signals: &[KnowledgeHealthSignal],
) -> RuntimeReadiness {
    let projection_governance_blocked = signals.iter().any(|signal| {
        signal.signal_kind == KnowledgeHealthSignalKind::GovernanceConflict
            && signal.target_ref.starts_with("projection:")
    });
    if projection_governance_blocked {
        readiness.projection = LayerReadiness::Conflict;
        if readiness.fusion == FusionReadiness::Ready {
            readiness.fusion = FusionReadiness::Degraded;
        }
        if !readiness
            .reasons
            .iter()
            .any(|reason| reason == "projection_governance_blocked")
        {
            readiness
                .reasons
                .push("projection_governance_blocked".to_string());
        }
    }
    readiness
}

/// 合并 readiness 驱动与 health 驱动的推荐动作。
pub fn merge_recommended_action(
    current: RecommendedAction,
    health_summary: Option<&KnowledgeHealthSummary>,
) -> RecommendedAction {
    let Some(health_summary) = health_summary else {
        return current;
    };
    if current == RecommendedAction::Init {
        return current;
    }
    if current == RecommendedAction::Rebuild {
        return current;
    }

    match health_summary.recommended_action {
        KnowledgeHealthRecommendedAction::None => current,
        KnowledgeHealthRecommendedAction::Sync => RecommendedAction::Sync,
        KnowledgeHealthRecommendedAction::Update => match current {
            RecommendedAction::None | RecommendedAction::Sync => RecommendedAction::Update,
            _ => current,
        },
        KnowledgeHealthRecommendedAction::Rebuild => RecommendedAction::Rebuild,
        KnowledgeHealthRecommendedAction::Review => {
            if current == RecommendedAction::None {
                RecommendedAction::Review
            } else {
                current
            }
        }
        KnowledgeHealthRecommendedAction::ReviewGovernance => {
            if matches!(current, RecommendedAction::None | RecommendedAction::Review) {
                RecommendedAction::ReviewGovernance
            } else {
                current
            }
        }
    }
}

/// 合并 core runtime 与治理状态的产品级建议动作。
/// init/rebuild 这类 core blocker 优先；普通维护动作不得掩盖治理 blocker。
pub fn merge_governance_recommended_action(
    current: RecommendedAction,
    governance: &GovernanceSummary,
) -> RecommendedAction {
    if matches!(
        current,
        RecommendedAction::Init | RecommendedAction::Rebuild
    ) {
        return current;
    }
    match (governance.readiness, governance.recommended_action) {
        (
            GovernanceReadiness::Blocked | GovernanceReadiness::Conflict,
            GovernanceRecommendedAction::ReviewGovernance,
        ) => RecommendedAction::ReviewGovernance,
        (GovernanceReadiness::Stale, GovernanceRecommendedAction::Update)
            if current == RecommendedAction::None =>
        {
            RecommendedAction::Update
        }
        _ => current,
    }
}

fn health_severity_rank(severity: KnowledgeHealthSeverity) -> u8 {
    match severity {
        KnowledgeHealthSeverity::Info => 0,
        KnowledgeHealthSeverity::Warning => 1,
        KnowledgeHealthSeverity::Error => 2,
    }
}

fn health_action_rank(action: KnowledgeHealthRecommendedAction) -> u8 {
    match action {
        KnowledgeHealthRecommendedAction::None => 0,
        KnowledgeHealthRecommendedAction::Review => 1,
        KnowledgeHealthRecommendedAction::ReviewGovernance => 2,
        KnowledgeHealthRecommendedAction::Sync => 3,
        KnowledgeHealthRecommendedAction::Update => 4,
        KnowledgeHealthRecommendedAction::Rebuild => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        merge_governance_recommended_action, merge_recommended_action, summarize_health_signals,
        RecommendedAction,
    };
    use wiki_model::domain::governance::{
        GovernanceReadiness, GovernanceRecommendedAction, GovernanceSummary,
    };
    use wiki_model::domain::knowledge_artifact::{
        KnowledgeHealthRecommendedAction, KnowledgeHealthSeverity, KnowledgeHealthSignal,
        KnowledgeHealthSignalKind, KnowledgeHealthSummary,
    };

    fn governance_summary(
        readiness: GovernanceReadiness,
        recommended_action: GovernanceRecommendedAction,
    ) -> GovernanceSummary {
        GovernanceSummary {
            readiness,
            fingerprint: Some("fingerprint".to_string()),
            active_count: 1,
            archived_count: 0,
            issues: Vec::new(),
            recommended_action,
        }
    }

    #[test]
    fn governance_blocker_preempts_non_blocking_core_maintenance() {
        let governance = governance_summary(
            GovernanceReadiness::Blocked,
            GovernanceRecommendedAction::ReviewGovernance,
        );
        for action in [
            RecommendedAction::None,
            RecommendedAction::Review,
            RecommendedAction::Update,
            RecommendedAction::Sync,
        ] {
            assert_eq!(
                merge_governance_recommended_action(action, &governance),
                RecommendedAction::ReviewGovernance
            );
        }
    }

    #[test]
    fn core_blocker_preempts_governance_blocker() {
        let governance = governance_summary(
            GovernanceReadiness::Conflict,
            GovernanceRecommendedAction::ReviewGovernance,
        );
        assert_eq!(
            merge_governance_recommended_action(RecommendedAction::Init, &governance),
            RecommendedAction::Init
        );
        assert_eq!(
            merge_governance_recommended_action(RecommendedAction::Rebuild, &governance),
            RecommendedAction::Rebuild
        );
    }

    #[test]
    fn governance_conflict_promotes_health_summary_to_review_governance() {
        let summary = summarize_health_signals(&[
            KnowledgeHealthSignal {
                signal_id: "signal:derived".to_string(),
                signal_kind: KnowledgeHealthSignalKind::DeclaredDerivedDivergence,
                severity: KnowledgeHealthSeverity::Warning,
                target_ref: "unit:repo".to_string(),
                recommended_action: KnowledgeHealthRecommendedAction::Update,
                reason: "declared and derived drift".to_string(),
            },
            KnowledgeHealthSignal {
                signal_id: "signal:conflict".to_string(),
                signal_kind: KnowledgeHealthSignalKind::GovernanceConflict,
                severity: KnowledgeHealthSeverity::Warning,
                target_ref: "conflict:repo".to_string(),
                recommended_action: KnowledgeHealthRecommendedAction::Review,
                reason: "parallel active declared conflict".to_string(),
            },
        ])
        .expect("signals should produce summary");

        assert_eq!(
            summary.recommended_action,
            KnowledgeHealthRecommendedAction::ReviewGovernance
        );
    }

    #[test]
    fn governance_conflict_does_not_hide_rebuild_health_action() {
        let summary = summarize_health_signals(&[
            KnowledgeHealthSignal {
                signal_id: "signal:orphan".to_string(),
                signal_kind: KnowledgeHealthSignalKind::OrphanUnit,
                severity: KnowledgeHealthSeverity::Error,
                target_ref: "unit:repo".to_string(),
                recommended_action: KnowledgeHealthRecommendedAction::Rebuild,
                reason: "unit missing valid domain binding".to_string(),
            },
            KnowledgeHealthSignal {
                signal_id: "signal:conflict".to_string(),
                signal_kind: KnowledgeHealthSignalKind::GovernanceConflict,
                severity: KnowledgeHealthSeverity::Warning,
                target_ref: "conflict:repo".to_string(),
                recommended_action: KnowledgeHealthRecommendedAction::Review,
                reason: "parallel active declared conflict".to_string(),
            },
        ])
        .expect("signals should produce summary");

        assert_eq!(
            summary.recommended_action,
            KnowledgeHealthRecommendedAction::Rebuild
        );
    }

    #[test]
    fn pending_declared_sync_precedes_stale_source_update() {
        let health = KnowledgeHealthSummary {
            total_signals: 1,
            degraded: true,
            counts_by_kind: Default::default(),
            counts_by_severity: Default::default(),
            highest_severity: Some(KnowledgeHealthSeverity::Warning),
            recommended_action: KnowledgeHealthRecommendedAction::Sync,
        };

        assert_eq!(
            merge_recommended_action(RecommendedAction::Update, Some(&health)),
            RecommendedAction::Sync
        );
    }
}
