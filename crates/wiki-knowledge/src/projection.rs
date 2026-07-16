//! `projection` 定义 KnowledgeTree 到页面投影决策的稳定合同。
//! 这里产出的 `PagePlan` 属于 knowledge 侧，不代表 runtime 持久化真相。

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use wiki_model::domain::knowledge::{
    is_official_wiki_relative_path, official_wiki_relative_path, KnowledgeTree, KnowledgeUnit,
    UnitType,
};
use wiki_model::domain::projection::{
    PageProjectionDecision, ProjectionAction, ProjectionEligibility, ProjectionLifecycle,
    ProjectionPolicy,
};
use wiki_model::domain::stable_id::stable_id;

/// `PagePlan` 描述“要生成什么页面”的稳定投影决策。
/// 它只负责页面结构和依赖范围，不承载 runtime 写盘状态。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagePlan {
    /// 页面稳定 ID，会被状态层、缓存层和关系层引用。
    pub id: String,
    /// 页面标题，最终渲染为一级标题。
    pub title: String,
    /// 页面在 `.wiki/` 下的稳定相对路径。
    pub relative_path: String,
    /// 页面类型，如 `overview / architecture / domain-index / module / concept-guide`。
    pub page_type: String,
    /// 父页面 ID，用于恢复页面树结构。
    pub parent_id: Option<String>,
    /// 页面作用域标签，帮助 runtime 区分 repository / domain / unit 页面。
    pub scope: String,
    /// 当前页面直接对应的知识单元 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 当前页面直接对应的知识单元类型。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// 当前页面归属的知识域 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 页面直接依赖的源码 ID 集合。
    pub source_ids: Vec<String>,
    /// 页面直接映射到的模块 ID 集合。
    pub module_ids: Vec<String>,
    /// 页面直接依赖的关系 ID 集合。
    pub relation_ids: Vec<String>,
    /// 当前页面采用的生成模式标记，主要用于调试和后续扩展。
    pub generation_mode: String,
    /// 页面规划优先级，供后续稳定排序。
    pub priority: usize,
    /// 被合并到本页面的子模块 ID 集合。
    #[serde(default)]
    pub merged_module_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionPolicyError {
    UnknownUnitRef(String),
    StructuralExclude(String),
    NonLeafOverride(String),
    ConflictingOverride(String),
    InvalidDecision(String),
}

/// 为每个 unit 生成 deterministic eligibility/lifecycle/action decision。
pub fn plan_projection_intents(
    tree: &KnowledgeTree,
    policy: &ProjectionPolicy,
    previous: &[PageProjectionDecision],
) -> Result<Vec<PageProjectionDecision>, ProjectionPolicyError> {
    let include = normalized_override_refs(&policy.include);
    let exclude = normalized_override_refs(&policy.exclude);
    for unit_ref in include.union(&exclude) {
        let Some(unit) = tree.get_unit(unit_ref) else {
            return Err(ProjectionPolicyError::UnknownUnitRef(unit_ref.clone()));
        };
        if is_structural(&unit.unit_type) && exclude.contains(unit_ref) {
            return Err(ProjectionPolicyError::StructuralExclude(unit_ref.clone()));
        }
        if !is_structural(&unit.unit_type) && !unit.is_leaf() {
            return Err(ProjectionPolicyError::NonLeafOverride(unit_ref.clone()));
        }
        if include.contains(unit_ref) && exclude.contains(unit_ref) {
            return Err(ProjectionPolicyError::ConflictingOverride(unit_ref.clone()));
        }
    }

    let mut budget_selected = BTreeSet::new();
    let mut candidates_by_domain = BTreeMap::<String, Vec<&KnowledgeUnit>>::new();
    for unit in tree.units.values() {
        if is_structural(&unit.unit_type)
            || !unit.is_leaf()
            || include.contains(&unit.id)
            || exclude.contains(&unit.id)
        {
            continue;
        }
        candidates_by_domain
            .entry(unit.domain_id.clone())
            .or_default()
            .push(unit);
    }
    for candidates in candidates_by_domain.values_mut() {
        candidates.sort_by(|left, right| {
            projection_priority(right, policy)
                .partial_cmp(&projection_priority(left, policy))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.id.cmp(&right.id))
        });
        budget_selected.extend(
            candidates
                .iter()
                .take(policy.max_projected_leaf_pages_per_domain)
                .map(|unit| unit.id.clone()),
        );
    }

    let previous_by_unit = previous
        .iter()
        .map(|decision| (decision.unit_ref.as_str(), decision))
        .collect::<BTreeMap<_, _>>();
    let mut unit_order = tree.processing_order.clone();
    for unit_id in tree.units.keys() {
        if !unit_order.contains(unit_id) {
            unit_order.push(unit_id.clone());
        }
    }
    let mut decisions = Vec::with_capacity(tree.units.len());
    for unit_id in unit_order {
        let Some(unit) = tree.get_unit(&unit_id) else {
            continue;
        };
        let relative_path =
            official_wiki_relative_path(&unit.unit_type, &unit.title, &unit.relative_path);
        let page_id = stable_id("page", &relative_path);
        let (eligibility, reason_ref, policy_ref) = if is_structural(&unit.unit_type) {
            (
                ProjectionEligibility::Required,
                "structural_required",
                "policy:structural",
            )
        } else if include.contains(&unit.id) {
            (
                ProjectionEligibility::Selected,
                "explicit_include",
                "policy:pages.include",
            )
        } else if exclude.contains(&unit.id) {
            (
                ProjectionEligibility::KnowledgeOnly,
                "explicit_exclude",
                "policy:pages.exclude",
            )
        } else if budget_selected.contains(&unit.id) {
            (
                ProjectionEligibility::Selected,
                "domain_budget_selected",
                "policy:pages.max_projected_leaf_pages_per_domain",
            )
        } else {
            (
                ProjectionEligibility::KnowledgeOnly,
                "domain_budget_exceeded",
                "policy:pages.max_projected_leaf_pages_per_domain",
            )
        };
        let (lifecycle, action) =
            projection_transition(eligibility, previous_by_unit.get(unit.id.as_str()).copied());
        let policy_seed = canonical_policy_seed(policy);
        let input_hash = stable_id(
            "projection-input",
            format!(
                "{}:{}:{}:{}:{policy_seed}",
                unit.id, relative_path, unit.priority, reason_ref
            ),
        );
        let mut decision = PageProjectionDecision {
            decision_id: stable_id("projection-decision", &unit.id),
            unit_ref: unit.id.clone(),
            page_id,
            relative_path,
            domain_ref: unit.domain_id.clone(),
            eligibility,
            lifecycle,
            action,
            reason_refs: vec![reason_ref.to_string()],
            policy_refs: vec![policy_ref.to_string()],
            input_hash,
        };
        decision.canonicalize();
        decision
            .validate()
            .map_err(|_| ProjectionPolicyError::InvalidDecision(unit.id.clone()))?;
        decisions.push(decision);
    }
    Ok(decisions)
}

/// 将 required/selected decisions 转为实际 PagePlan。
pub fn project_page_plans(
    tree: &KnowledgeTree,
    decisions: &[PageProjectionDecision],
) -> Result<Vec<PagePlan>, ProjectionPolicyError> {
    decisions
        .iter()
        .filter(|decision| decision.eligibility.is_projectable())
        .map(|decision| {
            let unit = tree
                .get_unit(&decision.unit_ref)
                .ok_or_else(|| ProjectionPolicyError::UnknownUnitRef(decision.unit_ref.clone()))?;
            let page = knowledge_unit_to_planned_page(unit, tree);
            if page.id != decision.page_id || page.relative_path != decision.relative_path {
                return Err(ProjectionPolicyError::InvalidDecision(
                    decision.decision_id.clone(),
                ));
            }
            Ok(page)
        })
        .collect()
}

fn normalized_override_refs(
    overrides: &[wiki_model::domain::projection::PageProjectionOverride],
) -> BTreeSet<String> {
    overrides
        .iter()
        .map(|entry| entry.unit_ref.trim().to_string())
        .filter(|unit_ref| !unit_ref.is_empty())
        .collect()
}

fn is_structural(unit_type: &UnitType) -> bool {
    matches!(
        unit_type,
        UnitType::Overview | UnitType::Architecture | UnitType::DomainIndex
    )
}

fn projection_priority(unit: &KnowledgeUnit, policy: &ProjectionPolicy) -> f32 {
    let relative_path =
        official_wiki_relative_path(&unit.unit_type, &unit.title, &unit.relative_path);
    let boost = policy
        .priority
        .iter()
        .filter(|entry| {
            let path = entry.path.trim().replace('\\', "/");
            path == unit.relative_path.replace('\\', "/") || path == relative_path
        })
        .map(|entry| entry.boost)
        .sum::<i32>();
    unit.priority + boost as f32
}

fn projection_transition(
    eligibility: ProjectionEligibility,
    previous: Option<&PageProjectionDecision>,
) -> (ProjectionLifecycle, ProjectionAction) {
    if eligibility.is_projectable() {
        return match previous.map(|decision| decision.lifecycle) {
            Some(ProjectionLifecycle::Projected | ProjectionLifecycle::Retiring) => {
                (ProjectionLifecycle::Projected, ProjectionAction::Retain)
            }
            _ => (ProjectionLifecycle::Absent, ProjectionAction::Promote),
        };
    }
    match previous.map(|decision| decision.lifecycle) {
        Some(ProjectionLifecycle::Projected | ProjectionLifecycle::Retiring) => {
            (ProjectionLifecycle::Retiring, ProjectionAction::Demote)
        }
        Some(ProjectionLifecycle::Retired) => {
            (ProjectionLifecycle::Retired, ProjectionAction::None)
        }
        _ => (ProjectionLifecycle::Absent, ProjectionAction::None),
    }
}

fn canonical_policy_seed(policy: &ProjectionPolicy) -> String {
    let include = normalized_override_refs(&policy.include)
        .into_iter()
        .collect::<Vec<_>>()
        .join(",");
    let exclude = normalized_override_refs(&policy.exclude)
        .into_iter()
        .collect::<Vec<_>>()
        .join(",");
    let mut priority = policy
        .priority
        .iter()
        .map(|entry| format!("{}:{}", entry.path.trim().replace('\\', "/"), entry.boost))
        .collect::<Vec<_>>();
    priority.sort();
    let mut hints = policy
        .hint_refs
        .iter()
        .map(|hint| hint.trim().to_string())
        .filter(|hint| !hint.is_empty())
        .collect::<Vec<_>>();
    hints.sort();
    format!(
        "include={include};exclude={exclude};priority={};hints={};budget={}",
        priority.join(","),
        hints.join(","),
        policy.max_projected_leaf_pages_per_domain
    )
}

fn knowledge_unit_to_planned_page(unit: &KnowledgeUnit, tree: &KnowledgeTree) -> PagePlan {
    let page_type = unit_type_to_page_type(&unit.unit_type);
    let scope = unit_type_to_scope(&unit.unit_type);

    let relative_path =
        official_wiki_relative_path(&unit.unit_type, &unit.title, &unit.relative_path);

    debug_assert!(
        is_official_wiki_relative_path(&relative_path),
        "planner produced non-official wiki page path: {relative_path}"
    );

    let parent_id = unit.parent_unit_id.as_ref().and_then(|parent_unit_id| {
        tree.get_unit(parent_unit_id).map(|parent| {
            stable_id(
                "page",
                official_wiki_relative_path(
                    &parent.unit_type,
                    &parent.title,
                    &parent.relative_path,
                ),
            )
        })
    });

    PagePlan {
        id: stable_id("page", &relative_path),
        title: unit.title.clone(),
        relative_path,
        page_type: page_type.to_string(),
        parent_id,
        scope: scope.to_string(),
        source_ids: unit.scope.source_ids.clone(),
        module_ids: unit.scope.module_ids.clone(),
        relation_ids: unit.scope.relation_ids.clone(),
        generation_mode: "knowledge-tree".to_string(),
        priority: (unit.priority * 100.0) as usize,
        merged_module_ids: Vec::new(),
        unit_id: Some(unit.id.clone()),
        unit_type: Some(unit.unit_type.as_str().to_string()),
        domain_id: Some(unit.domain_id.clone()),
    }
}

fn unit_type_to_page_type(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview => "overview",
        UnitType::Architecture => "architecture",
        UnitType::DomainIndex => "domain-index",
        UnitType::ModuleDoc => "module",
        UnitType::ApiDoc => "api-reference",
        UnitType::ConfigDoc => "config-reference",
        UnitType::ConceptGuide => "concept-guide",
        UnitType::WorkflowDoc => "workflow",
        UnitType::TroubleshootDoc => "troubleshoot",
        UnitType::IntegrationDoc => "integration",
        UnitType::TestDoc => "test-doc",
        UnitType::ExampleDoc => "example",
    }
}

fn unit_type_to_scope(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview | UnitType::Architecture => "repository",
        UnitType::DomainIndex => "domain",
        _ => "unit",
    }
}
