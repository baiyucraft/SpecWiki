//! `governance` 定义 runtime、transport 与宿主共享的治理对象语言。
//! 本模块只描述稳定状态和引用，不读取 `.spec`、执行规则或改变 core readiness。

use serde::{Deserialize, Serialize};

/// Archive 命令的稳定执行模式。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveMode {
    DryRun,
    Apply,
    Resume,
}

/// Archive 对外可观察的结果。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveOutcome {
    Ready,
    Completed,
    AlreadyCompleted,
    Blocked,
    Conflict,
    RecoveryRequired,
    Rejected,
}

/// Durable archive operation 的聚合状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveOperationStatus {
    Planned,
    Applying,
    RecoveryRequired,
    Completed,
    Rejected,
}

/// Archive 状态机中的可恢复步骤。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStep {
    Prepared,
    SourceMoved,
    ParentMetaUpdated,
    ParentSplitUpdated,
    Completed,
}

/// 单个步骤在 checkpoint 聚合视图中的状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Archive workflow 的稳定错误分类。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveErrorKind {
    NotReady,
    PreconditionChanged,
    Conflict,
    Locked,
    RecoveryRequired,
    ManifestInvalid,
    Io,
}

/// Source tree 中参与 precondition digest 的单个条目。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveArtifactHash {
    pub relative_path: String,
    pub file_type: String,
    pub size: u64,
    pub content_hash: String,
}

/// Child archive 对 active parent 的预期更新。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveParentDiff {
    pub parent_id: String,
    pub meta_path: String,
    pub split_path: String,
    pub archived_at: String,
    pub archived_to: String,
    pub meta_before_hash: String,
    pub meta_after_hash: String,
    pub split_before_hash: String,
    pub split_after_hash: String,
}

/// Append-only checkpoint 的稳定记录。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveCheckpoint {
    pub sequence: u64,
    pub attempt: u32,
    pub step: ArchiveStep,
    pub status: ArchiveStepStatus,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Dry-run 输出和 durable operation 聚合共用的 manifest。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveOperationManifest {
    pub schema_version: String,
    pub policy_version: String,
    pub algorithm_version: String,
    pub operation_id: String,
    pub change_id: String,
    pub mode: ArchiveMode,
    pub outcome: ArchiveOutcome,
    pub status: ArchiveOperationStatus,
    pub step: ArchiveStep,
    pub step_status: ArchiveStepStatus,
    pub source_path: String,
    pub target_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_root: Option<String>,
    pub created_at: String,
    pub persisted: bool,
    pub resumable: bool,
    pub validation: GovernanceValidationResult,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_hash_summary: Vec<ArchiveArtifactHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_diff: Option<ArchiveParentDiff>,
    pub precondition_digest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub completed_steps: Vec<ArchiveStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_step: Option<ArchiveStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wiki_sync_issues: Vec<GovernanceBlockingIssue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<GovernanceArtifactRef>,
}

/// Archive workflow 输入。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveRequest {
    pub change_id: String,
    pub mode: ArchiveMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// Archive workflow 输出。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArchiveReport {
    pub governance: GovernanceSummary,
    pub validation: GovernanceValidationResult,
    pub manifest: ArchiveOperationManifest,
}

/// 仓库级治理证据及其派生缓存的可用状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReadiness {
    /// 仓库不存在 `.spec` 治理根目录。
    NotEnabled,
    /// 治理证据可解析，规则成立且派生缓存与当前 fingerprint 一致。
    Ready,
    /// live evidence 已变化或缓存缺失，需通过 update 刷新派生数据。
    Stale,
    /// 证据可解释，但必需 artifact、依赖或 gate 尚未满足。
    Blocked,
    /// 证据损坏或相互矛盾，无法形成唯一可信状态。
    Conflict,
}

/// 治理 issue 对当前操作的影响等级。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceIssueSeverity {
    /// 当前治理操作必须先解决该问题。
    Blocking,
    /// 问题需要展示，但不阻断当前治理操作。
    Warning,
}

/// Change evidence 所在的治理生命周期区域。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceLocation {
    /// Change 位于 `.spec/changes`，仍处于活动生命周期。
    Active,
    /// Change 位于 `.spec/archive`，只作为历史证据读取。
    Archived,
}

/// 已知治理 artifact 的文件内容状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceArtifactStatus {
    /// 预期路径不存在可读取的普通文件。
    Missing,
    /// 文件存在，但去除空白后没有内容。
    Empty,
    /// 文件存在且包含非空内容；该状态不代表内容已通过 policy。
    Present,
}

/// 单个治理 gate 的规范化判断状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceGateStatus {
    /// 当前 change 类型或 stage 不要求该 gate。
    NotApplicable,
    /// Gate 尚未具备完整证据，但当前阶段允许继续推进。
    Pending,
    /// Gate 所需证据和规则均已满足。
    Passed,
    /// Gate 所需证据存在但未满足规则，或必需证据缺失。
    Failed,
    /// Gate 依赖的证据损坏或冲突，无法可靠判断。
    Conflict,
}

/// 治理产品响应建议执行的下一步动作。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceRecommendedAction {
    /// 当前治理状态不要求额外操作。
    None,
    /// 运行 update 刷新 fingerprint 绑定的治理派生缓存。
    Update,
    /// 检查并修复 `.spec` evidence 或未满足的治理 gate。
    ReviewGovernance,
}

/// 指向单个 `.spec` artifact 的稳定、无正文引用。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceArtifactRef {
    /// Artifact 所属 change id。
    pub change_id: String,
    /// Policy 使用的稳定 artifact kind，例如 `proposal` 或 `review-report`。
    pub kind: String,
    /// 仓库相对路径；不得包含绝对路径或仓库逃逸段。
    pub relative_path: String,
    /// 当前文件内容状态。
    pub status: GovernanceArtifactStatus,
    /// 文件存在且可读取时的内容 hash；missing artifact 为 `None`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

/// 单个 change 的 artifact、review、verification 与 archive gate 摘要。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceGateSummary {
    /// 当前 stage 所需 artifact 是否齐备且可解析。
    pub artifact_gate: GovernanceGateStatus,
    /// Review report evidence 的规范化 gate 状态。
    pub review_gate: GovernanceGateStatus,
    /// Test report evidence 的规范化 gate 状态。
    pub verification_gate: GovernanceGateStatus,
    /// 当前 change 是否已满足只读归档条件。
    pub archive_readiness: GovernanceGateStatus,
}

/// Active 或 archived change 的稳定列表投影。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceChangeSummary {
    /// 目录名与 metadata 必须共同确认的 change id。
    pub id: String,
    /// Change 当前位于 active 还是 archived evidence tree。
    pub location: GovernanceLocation,
    /// 从 metadata 读取的 lifecycle stage。
    pub stage: String,
    /// Multi-change role；standalone change 为 `None`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Child change 声明的 parent id；非 child 为 `None`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Child 在 parent split 中的正整数顺序；非 child 为 `None`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    /// 当前 change 显式声明的同级依赖，按稳定顺序输出。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    /// 当前 change 的四类治理 gate 摘要。
    pub gate: GovernanceGateSummary,
}

/// Policy 产生的可操作治理问题。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceBlockingIssue {
    /// 可由测试和宿主稳定匹配的机器规则 id。
    pub rule_id: String,
    /// Issue 对当前治理操作的阻断等级。
    pub severity: GovernanceIssueSeverity,
    /// 面向用户的说明文案；机器逻辑不得依赖此字段。
    pub message: String,
    /// 问题可以归属到单个 change 时携带其 id。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    /// 问题可以定位到单个 artifact 时携带其引用，不包含正文。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<GovernanceArtifactRef>,
    /// 修复或继续当前治理操作的建议动作。
    pub recommended_action: GovernanceRecommendedAction,
}

/// 单条 policy rule 的规范化校验结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceRuleResult {
    /// 可由 fixture 和宿主稳定匹配的机器规则 id。
    pub rule_id: String,
    /// 当前 evidence 是否满足该规则。
    pub passed: bool,
    /// 规则失败时对当前操作的影响等级。
    pub severity: GovernanceIssueSeverity,
    /// 可选展示文案；机器判断只依赖 rule id 和结构化字段。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 规则涉及单个 artifact 时提供定位引用。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<GovernanceArtifactRef>,
}

/// 与 core runtime readiness 并列的仓库级治理摘要。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceSummary {
    /// 当前治理 evidence 与派生缓存的产品级 readiness。
    pub readiness: GovernanceReadiness,
    /// 当前 live evidence 的内容 fingerprint；未启用治理时为 `None`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// `.spec/changes` 下成功识别的 active change 数量。
    pub active_count: usize,
    /// `.spec/archive` 下成功识别的 archived change 数量。
    pub archived_count: usize,
    /// Repo 级或 change 级治理问题，按 policy 规定的稳定顺序输出。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<GovernanceBlockingIssue>,
    /// 当前治理状态建议执行的下一步动作。
    pub recommended_action: GovernanceRecommendedAction,
}

/// 对 live evidence 执行只读 policy 校验后的完整结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct GovernanceValidationResult {
    /// 所有 blocking rule 是否均已通过。
    pub valid: bool,
    /// Live evidence 的治理 readiness；validate 不读取派生 cache freshness。
    pub readiness: GovernanceReadiness,
    /// 所有已执行 rule 的结构化结果，按稳定 rule 顺序输出。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_results: Vec<GovernanceRuleResult>,
    /// 校验发现的 blocking 与 warning issues。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<GovernanceBlockingIssue>,
}
