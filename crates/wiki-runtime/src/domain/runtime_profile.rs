//! runtime_profile 为宿主暴露稳定的状态、query 与 gate 投影。
//! 它只收口公开消费语义，不在这里泄漏内部 workflow 细节。

use serde::{Deserialize, Serialize};

use crate::domain::checkpoint::PipelineRuntimeSummary;
use crate::domain::checkpoint::UnitRuntimeGate;
use wiki_model::domain::knowledge_artifact::{
    KnowledgeHealthRecommendedAction, KnowledgeHealthSeverity, KnowledgeHealthSignal,
    KnowledgeHealthSummary,
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

    let promoted = match health_summary.recommended_action {
        KnowledgeHealthRecommendedAction::None => current,
        KnowledgeHealthRecommendedAction::Sync => {
            if current == RecommendedAction::None {
                RecommendedAction::Sync
            } else {
                current
            }
        }
        KnowledgeHealthRecommendedAction::Update => match current {
            RecommendedAction::None | RecommendedAction::Sync => RecommendedAction::Update,
            _ => current,
        },
        KnowledgeHealthRecommendedAction::Rebuild => RecommendedAction::Rebuild,
        KnowledgeHealthRecommendedAction::Review => {
            if current == RecommendedAction::None {
                RecommendedAction::Sync
            } else {
                current
            }
        }
    };

    promoted
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
        KnowledgeHealthRecommendedAction::Sync => 2,
        KnowledgeHealthRecommendedAction::Update => 3,
        KnowledgeHealthRecommendedAction::Rebuild => 4,
    }
}
