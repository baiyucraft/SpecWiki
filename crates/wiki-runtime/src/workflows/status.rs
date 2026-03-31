//! status workflow 负责对外报告当前 runtime 是否 fresh、stale、index_only、missing 或 needs_rebuild。
//! 它复用 change planning 内核，不自己维护独立的脏判断规则。

use serde::Serialize;
use std::io;
use std::path::Path;

use crate::domain::change_set::plan_runtime_changes_with_mode;
use crate::domain::runtime_profile::{
    LlmModeHint, QueryReadiness, RecommendedAction, RuntimeGateSummary, RuntimeSummaryProjection,
    blocker_hint_from, preflight_for_state,
};
use crate::domain::steering::{SteeringLoadMode, load_steering_config_with_mode};
use crate::storage::state_store::facts_snapshot_ready;
use crate::workflows::page_render::{
    load_runtime_gate_summary_for_repo, load_runtime_summary_for_repo,
};
use crate::workflows::release_scope::project_external_runtime_state;

/// `status` 只回答一件事：当前 Repo Wiki 是否仍然可用、是否被阻塞，以及下一步动作。
#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    /// 当前 runtime 的外部状态，取值为 `fresh / needs_update / runtime_incomplete / blocker / index_only / missing / needs_rebuild`。
    pub state: String,
    /// 本次检测到的脏源码路径集合。
    pub dirty_sources: Vec<String>,
    /// 本次检测到的受影响页面路径集合。
    pub dirty_pages: Vec<String>,
    /// 当状态为 `needs_rebuild` 时，对外返回的原因说明。
    pub needs_rebuild_reason: Option<String>,
    /// 当前仓库是否已经持有可供 query 兜底使用的 facts/runtime snapshot。
    pub facts_ready: bool,
    /// 当前阶段宿主是否适合直接发起 query。
    pub query_readiness: QueryReadiness,
    /// 面向宿主的下一步建议动作。
    pub recommended_action: RecommendedAction,
    /// 当前阶段仅允许输出预判型 LLM 模式提示。
    pub llm_mode_hint: LlmModeHint,
    /// 已持久化时回传 workflow runtime 摘要。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_summary: Option<RuntimeSummaryProjection>,
    /// 已持久化时回传 gate 聚合摘要。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gate_summary: Option<RuntimeGateSummary>,
    /// 宿主可直接消费的 blocker 线索。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocker_hint: Option<String>,
}

/// 检查 runtime 是否缺失、是否需要重建、是否存在脏源码。
/// 统一复用 change planning 内核，避免 status/update 维护两套脏判断。
///
/// # 参数
/// - `repo_root`：要检查的本地代码目录。
///
/// # 返回
/// - 成功时返回当前 Repo Wiki 的状态报告。
///
/// # 错误
/// - 当状态层、源码目录或变化规划读取失败时返回错误。
pub fn run_status(repo_root: &Path) -> io::Result<StatusReport> {
    run_status_with_mode(repo_root, SteeringLoadMode::Production)
}

pub fn run_status_with_mode(
    repo_root: &Path,
    steering_mode: SteeringLoadMode,
) -> io::Result<StatusReport> {
    let plan = plan_runtime_changes_with_mode(repo_root, steering_mode)?;
    let facts_ready = facts_snapshot_ready(repo_root)?;
    let projected_state = project_external_runtime_state(repo_root, plan.state(), facts_ready);
    let runtime_summary = load_runtime_summary_for_repo(repo_root)
        .ok()
        .flatten()
        .map(RuntimeSummaryProjection::from_summary);
    let gate_summary = load_runtime_gate_summary_for_repo(repo_root).ok().flatten();
    let external_state = derive_runtime_state(
        &projected_state,
        runtime_summary.as_ref(),
        gate_summary.as_ref(),
    );
    let preflight = preflight_for_state(&external_state, facts_ready);
    let blocker_hint = blocker_hint_from(runtime_summary.as_ref(), gate_summary.as_ref());
    let llm_mode_hint = if load_steering_config_with_mode(repo_root, steering_mode)
        .llm
        .provider_configured()
    {
        LlmModeHint::ProviderConfigured
    } else {
        LlmModeHint::DeterministicDefault
    };

    Ok(StatusReport {
        state: external_state,
        dirty_sources: plan.change_set.dirty_sources(),
        dirty_pages: plan.dirty_page_paths(),
        needs_rebuild_reason: plan.needs_rebuild_reason,
        facts_ready: preflight.facts_ready,
        query_readiness: preflight.query_readiness,
        recommended_action: preflight.recommended_action,
        llm_mode_hint,
        runtime_summary,
        gate_summary,
        blocker_hint,
    })
}

fn derive_runtime_state(
    projected_state: &str,
    runtime_summary: Option<&RuntimeSummaryProjection>,
    gate_summary: Option<&RuntimeGateSummary>,
) -> String {
    let has_blocker = runtime_summary
        .map(|summary| summary.runtime_state == "interrupted")
        .unwrap_or(false)
        || gate_summary
            .map(|summary| summary.blocked_units > 0)
            .unwrap_or(false);
    if has_blocker {
        return "blocker".to_string();
    }

    let is_incomplete = runtime_summary
        .map(|summary| {
            summary.runtime_state != "completed"
                && summary.runtime_state != "interrupted"
                && !summary.runtime_state.is_empty()
        })
        .unwrap_or(false);
    if is_incomplete {
        return "runtime_incomplete".to_string();
    }

    let gate_only_incomplete = gate_summary
        .map(|summary| summary.total_units > 0 && summary.assembled_units < summary.total_units)
        .unwrap_or(false);
    if gate_only_incomplete {
        return "runtime_incomplete".to_string();
    }

    if projected_state == "stale" {
        return "needs_update".to_string();
    }

    projected_state.to_string()
}

#[cfg(test)]
mod tests {
    use super::derive_runtime_state;
    use crate::domain::runtime_profile::{RuntimeGateSummary, RuntimeSummaryProjection};

    #[test]
    fn derive_runtime_state_marks_gate_only_partial_runtime_as_incomplete() {
        let gate_summary = RuntimeGateSummary {
            total_units: 12,
            ready_for_compose_units: 3,
            composed_units: 1,
            assembled_units: 0,
            blocked_units: 0,
            blockers: Vec::new(),
        };

        let state = derive_runtime_state("missing", None, Some(&gate_summary));

        assert_eq!(state, "runtime_incomplete");
    }

    #[test]
    fn derive_runtime_state_keeps_blocker_precedence_over_gate_only_partial_runtime() {
        let runtime_summary = RuntimeSummaryProjection {
            workflow_action: "init".to_string(),
            runtime_state: "interrupted".to_string(),
            researched_units: 0,
            compose_ready_units: 0,
            composed_units: 0,
            assembled_pages: 0,
            blocked_units: vec!["unit-runtime".to_string()],
            last_ready_stage: None,
            summary_reason: Some("provider blocked".to_string()),
        };
        let gate_summary = RuntimeGateSummary {
            total_units: 12,
            ready_for_compose_units: 3,
            composed_units: 1,
            assembled_units: 0,
            blocked_units: 1,
            blockers: Vec::new(),
        };

        let state = derive_runtime_state("missing", Some(&runtime_summary), Some(&gate_summary));

        assert_eq!(state, "blocker");
    }
}
