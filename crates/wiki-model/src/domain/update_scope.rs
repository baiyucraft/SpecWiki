use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// `ScopeEscalationLevel` 描述 knowledge-first update 当前需要放大的刷新层级。
/// 它只服务于 `update` 主线的知识范围诊断，不直接等价于 lifecycle workflow 名称。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeEscalationLevel {
    LocalRefresh,
    SubtreeReplan,
    RepoReplan,
    RebuildRecommended,
}

impl ScopeEscalationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalRefresh => "local_refresh",
            Self::SubtreeReplan => "subtree_replan",
            Self::RepoReplan => "repo_replan",
            Self::RebuildRecommended => "rebuild_recommended",
        }
    }
}

/// `ScopeEscalation` 保留本次 scope 规划的升级层级与原因摘要。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeEscalation {
    pub level: ScopeEscalationLevel,
    pub reason: String,
}

impl Default for ScopeEscalation {
    fn default() -> Self {
        Self {
            level: ScopeEscalationLevel::LocalRefresh,
            reason: "direct_unit_refresh".to_string(),
        }
    }
}

/// `AffectedKnowledgeScope` 是 `ChangeSet` 与页面投影之间的正式中间层。
/// 它先表达哪些 KnowledgeUnit / Domain 被命中，再由 runtime 派生页面与 section 影响集。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffectedKnowledgeScope {
    /// 被脏源码、graph 依赖或结构变化直接命中的知识单元。
    pub direct_unit_ids: Vec<String>,
    /// 因 child contract 变化而向上卷入的 parent 单元。
    pub propagated_parent_unit_ids: Vec<String>,
    /// 因 declared writeback 或 contract 漂移被显式标记为 stale 的知识单元。
    #[serde(default)]
    pub stale_unit_ids: Vec<String>,
    /// 上一轮存在、本轮已消失的知识单元。
    pub removed_unit_ids: Vec<String>,
    /// 需要重新汇总或重算的知识域。
    pub affected_domain_ids: Vec<String>,
    /// 本轮明确命中的 declared records。
    #[serde(default)]
    pub declared_record_ids: Vec<String>,
    /// 因 contract 或 projection 漂移被标记为 stale 的 projection targets。
    #[serde(default)]
    pub stale_projection_ids: Vec<String>,
    /// 由当前知识范围派生出的页面投影目标。
    pub projection_target_page_ids: Vec<String>,
    /// 本轮 scope 命中的 health signal target refs。
    #[serde(default)]
    pub health_signal_targets: Vec<String>,
    /// 当前 knowledge-first update 的升级层级与原因。
    pub escalation: ScopeEscalation,
}

impl AffectedKnowledgeScope {
    /// 返回本次 refresh 涉及的全部“仍然存在的”知识单元。
    pub fn active_unit_ids(&self) -> Vec<String> {
        let mut unit_ids = BTreeSet::new();
        unit_ids.extend(self.direct_unit_ids.iter().cloned());
        unit_ids.extend(self.propagated_parent_unit_ids.iter().cloned());
        unit_ids.extend(self.stale_unit_ids.iter().cloned());
        unit_ids.into_iter().collect()
    }

    /// 返回本次 scope 涉及的全部知识单元，包含已移除单元。
    pub fn all_unit_ids(&self) -> Vec<String> {
        let mut unit_ids = BTreeSet::new();
        unit_ids.extend(self.direct_unit_ids.iter().cloned());
        unit_ids.extend(self.propagated_parent_unit_ids.iter().cloned());
        unit_ids.extend(self.stale_unit_ids.iter().cloned());
        unit_ids.extend(self.removed_unit_ids.iter().cloned());
        unit_ids.into_iter().collect()
    }

    /// 当前 scope 是否没有命中任何知识单元与页面目标。
    pub fn is_empty(&self) -> bool {
        self.direct_unit_ids.is_empty()
            && self.propagated_parent_unit_ids.is_empty()
            && self.stale_unit_ids.is_empty()
            && self.removed_unit_ids.is_empty()
            && self.affected_domain_ids.is_empty()
            && self.declared_record_ids.is_empty()
            && self.stale_projection_ids.is_empty()
            && self.projection_target_page_ids.is_empty()
            && self.health_signal_targets.is_empty()
    }
}
