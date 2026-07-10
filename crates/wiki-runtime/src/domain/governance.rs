//! `.spec` evidence 的确定性治理策略。
//!
//! Policy 只消费规范化 snapshot，不读取文件、不写缓存，也不调用外部 UniSpec CLI。

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

use serde::Serialize;
use wiki_model::domain::governance::{
    GovernanceArtifactRef, GovernanceArtifactStatus, GovernanceBlockingIssue,
    GovernanceChangeSummary, GovernanceGateStatus, GovernanceGateSummary, GovernanceIssueSeverity,
    GovernanceReadiness, GovernanceRecommendedAction, GovernanceRuleResult, GovernanceSummary,
    GovernanceValidationResult,
};

use crate::storage::governance_fs::{
    GovernanceChangeEvidence, GovernanceChildEvidence, GovernanceEvidenceFailure,
    GovernanceEvidenceSnapshot,
};

const STAGES: [&str; 10] = [
    "exploration",
    "proposal",
    "delivery",
    "design",
    "cases",
    "tasks",
    "implementation",
    "review",
    "verification",
    "archive",
];

/// Policy 对一次 evidence snapshot 的完整派生结果。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceEvaluation {
    /// 仓库级产品摘要。
    pub summary: GovernanceSummary,
    /// 可供 list/inspect/query 消费的 change 投影。
    pub changes: Vec<GovernanceChangeSummary>,
    /// 可供 inspect/query/cache 消费的无正文 artifact refs。
    pub artifacts: Vec<GovernanceArtifactRef>,
    /// 本次执行的失败规则结果；rule id 和顺序稳定。
    pub rule_results: Vec<GovernanceRuleResult>,
}

impl GovernanceEvaluation {
    /// 将 evaluation 投影为 validate 使用的结构化结果。
    pub fn validation_result(&self) -> GovernanceValidationResult {
        GovernanceValidationResult {
            valid: !matches!(
                self.summary.readiness,
                GovernanceReadiness::Blocked | GovernanceReadiness::Conflict
            ),
            readiness: self.summary.readiness,
            rule_results: self.rule_results.clone(),
            issues: self.summary.issues.clone(),
        }
    }

    /// 消费 evaluation 并生成 validate result。
    pub fn into_validation_result(self) -> GovernanceValidationResult {
        GovernanceValidationResult {
            valid: !matches!(
                self.summary.readiness,
                GovernanceReadiness::Blocked | GovernanceReadiness::Conflict
            ),
            readiness: self.summary.readiness,
            rule_results: self.rule_results,
            issues: self.summary.issues,
        }
    }
}

impl Deref for GovernanceEvaluation {
    type Target = GovernanceSummary;

    fn deref(&self) -> &Self::Target {
        &self.summary
    }
}

/// 与 `@uni-sw/unispec` 0.1.0 stage/artifact/report 规则保持 fixture parity 的 policy。
#[derive(Debug, Clone, Copy)]
pub struct GovernancePolicy {
    version: &'static str,
}

impl GovernancePolicy {
    /// 返回当前第一版治理规则目录。
    pub const fn v1() -> Self {
        Self {
            version: "unispec-0.1.0",
        }
    }

    /// 返回规则版本；cache 必须把它纳入 schema/fingerprint 绑定。
    pub const fn version(&self) -> &'static str {
        self.version
    }

    /// 计算仓库级 summary、change gates、artifact refs 和 rule results。
    pub fn evaluate(&self, snapshot: &GovernanceEvidenceSnapshot) -> GovernanceEvaluation {
        if !snapshot.enabled {
            return GovernanceEvaluation {
                summary: GovernanceSummary {
                    readiness: GovernanceReadiness::NotEnabled,
                    fingerprint: None,
                    active_count: 0,
                    archived_count: 0,
                    issues: Vec::new(),
                    recommended_action: GovernanceRecommendedAction::None,
                },
                changes: Vec::new(),
                artifacts: Vec::new(),
                rule_results: Vec::new(),
            };
        }

        let mut issues = Vec::new();
        for failure in &snapshot.failures {
            issues.push(classify_failure(failure, snapshot));
        }

        let all_changes = snapshot
            .active_changes
            .iter()
            .chain(snapshot.archived_changes.iter())
            .collect::<Vec<_>>();
        let active_ids = snapshot
            .active_changes
            .iter()
            .map(|change| change.id.as_str())
            .collect::<BTreeSet<_>>();
        let archived_by_id = snapshot.archived_changes.iter().fold(
            BTreeMap::<&str, Vec<&GovernanceChangeEvidence>>::new(),
            |mut grouped, change| {
                grouped.entry(change.id.as_str()).or_default().push(change);
                grouped
            },
        );

        for (id, entries) in &archived_by_id {
            if entries.len() > 1 {
                issues.push(conflict_issue(
                    "evidence.archive.duplicate",
                    Some(id),
                    None,
                    "multiple archive directories claim the same change id",
                ));
            }
        }

        let mut changes = Vec::new();
        for change in all_changes {
            let artifacts = artifacts_for_change(snapshot, change);
            let gate = self.evaluate_change(
                change,
                &artifacts,
                &active_ids,
                &archived_by_id,
                &snapshot.active_changes,
                &mut issues,
            );
            let multi = change.metadata.multi_change.as_ref();
            let mut depends_on = multi
                .map(|value| value.depends_on.clone())
                .unwrap_or_default();
            depends_on.sort();
            depends_on.dedup();
            changes.push(GovernanceChangeSummary {
                id: change.id.clone(),
                location: change.location,
                stage: change.metadata.stage.clone(),
                role: multi.map(|value| value.role.clone()),
                parent: multi.and_then(|value| value.parent.clone()),
                order: multi.and_then(|value| value.order),
                depends_on,
                gate,
            });
        }

        changes.sort_by(|left, right| {
            left.location
                .cmp(&right.location)
                .then_with(|| {
                    left.order
                        .unwrap_or(u32::MAX)
                        .cmp(&right.order.unwrap_or(u32::MAX))
                })
                .then_with(|| left.id.cmp(&right.id))
        });
        issues.sort_by(|left, right| {
            left.change_id
                .cmp(&right.change_id)
                .then_with(|| left.rule_id.cmp(&right.rule_id))
                .then_with(|| {
                    left.artifact_ref
                        .as_ref()
                        .map(|artifact| artifact.relative_path.as_str())
                        .cmp(
                            &right
                                .artifact_ref
                                .as_ref()
                                .map(|artifact| artifact.relative_path.as_str()),
                        )
                })
        });
        issues.dedup_by(|left, right| {
            left.rule_id == right.rule_id
                && left.change_id == right.change_id
                && left.artifact_ref == right.artifact_ref
        });

        let readiness = if issues.iter().any(is_conflict_issue) {
            GovernanceReadiness::Conflict
        } else if issues
            .iter()
            .any(|issue| issue.severity == GovernanceIssueSeverity::Blocking)
        {
            GovernanceReadiness::Blocked
        } else {
            GovernanceReadiness::Ready
        };
        let rule_results = issues.iter().map(issue_to_rule_result).collect();

        GovernanceEvaluation {
            summary: GovernanceSummary {
                readiness,
                fingerprint: snapshot.fingerprint.clone(),
                active_count: snapshot.active_changes.len(),
                archived_count: snapshot.archived_changes.len(),
                issues,
                recommended_action: if matches!(
                    readiness,
                    GovernanceReadiness::Blocked | GovernanceReadiness::Conflict
                ) {
                    GovernanceRecommendedAction::ReviewGovernance
                } else {
                    GovernanceRecommendedAction::None
                },
            },
            changes,
            artifacts: snapshot.artifacts.clone(),
            rule_results,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn evaluate_change(
        &self,
        change: &GovernanceChangeEvidence,
        artifacts: &[GovernanceArtifactRef],
        active_ids: &BTreeSet<&str>,
        archived_by_id: &BTreeMap<&str, Vec<&GovernanceChangeEvidence>>,
        active_changes: &[GovernanceChangeEvidence],
        issues: &mut Vec<GovernanceBlockingIssue>,
    ) -> GovernanceGateSummary {
        self.evaluate_metadata(change, active_changes, issues);

        let required = required_artifacts(change);
        let mut artifact_gate = GovernanceGateStatus::Passed;
        for kind in required {
            let artifact = artifacts.iter().find(|artifact| artifact.kind == *kind);
            if artifact.is_none_or(|artifact| artifact.status != GovernanceArtifactStatus::Present)
            {
                artifact_gate = GovernanceGateStatus::Failed;
                issues.push(blocked_issue(
                    &format!("artifact.required.{kind}"),
                    Some(&change.id),
                    artifact.cloned(),
                    &format!("required artifact is missing or empty: {kind}"),
                ));
            }
        }

        let is_parent = change
            .metadata
            .multi_change
            .as_ref()
            .is_some_and(|multi| multi.role == "parent");
        let checks_reports =
            !is_parent && matches!(change.metadata.stage.as_str(), "verification" | "archive");
        let review_parse_conflict = issues.iter().any(|issue| {
            issue.change_id.as_deref() == Some(&change.id) && issue.rule_id == "report.review.parse"
        });
        let test_parse_conflict = issues.iter().any(|issue| {
            issue.change_id.as_deref() == Some(&change.id)
                && issue.rule_id == "report.verification.parse"
        });

        let review_gate = if !checks_reports {
            GovernanceGateStatus::NotApplicable
        } else if review_parse_conflict {
            GovernanceGateStatus::Conflict
        } else if let Some(report) = &change.review_report {
            let mut passed = true;
            if report.review_result != "pass" {
                passed = false;
                issues.push(blocked_issue(
                    "report.review.pass",
                    Some(&change.id),
                    artifact_by_kind(artifacts, "review-report"),
                    "review report result must be pass",
                ));
            }
            if report.scope != "full" {
                passed = false;
                issues.push(blocked_issue(
                    "report.review.scope_full",
                    Some(&change.id),
                    artifact_by_kind(artifacts, "review-report"),
                    "review report scope must be full",
                ));
            }
            if passed {
                GovernanceGateStatus::Passed
            } else {
                GovernanceGateStatus::Failed
            }
        } else {
            GovernanceGateStatus::Failed
        };

        let verification_gate = if !checks_reports {
            GovernanceGateStatus::NotApplicable
        } else if test_parse_conflict {
            GovernanceGateStatus::Conflict
        } else if let Some(report) = &change.test_report {
            let mut passed = true;
            if report.verification_result != "pass" {
                passed = false;
                issues.push(blocked_issue(
                    "report.verification.pass",
                    Some(&change.id),
                    artifact_by_kind(artifacts, "test-report"),
                    "test report result must be pass",
                ));
            }
            if report.scope != "full" {
                passed = false;
                issues.push(blocked_issue(
                    "report.verification.scope_full",
                    Some(&change.id),
                    artifact_by_kind(artifacts, "test-report"),
                    "test report scope must be full",
                ));
            }
            if passed {
                GovernanceGateStatus::Passed
            } else {
                GovernanceGateStatus::Failed
            }
        } else {
            GovernanceGateStatus::Failed
        };

        let archive_readiness = if is_parent {
            self.evaluate_parent_archive(change, active_ids, archived_by_id, issues)
        } else if checks_reports {
            if artifact_gate == GovernanceGateStatus::Passed
                && review_gate == GovernanceGateStatus::Passed
                && verification_gate == GovernanceGateStatus::Passed
            {
                GovernanceGateStatus::Passed
            } else if review_gate == GovernanceGateStatus::Conflict
                || verification_gate == GovernanceGateStatus::Conflict
            {
                GovernanceGateStatus::Conflict
            } else {
                GovernanceGateStatus::Failed
            }
        } else {
            GovernanceGateStatus::Pending
        };

        GovernanceGateSummary {
            artifact_gate,
            review_gate,
            verification_gate,
            archive_readiness,
        }
    }

    fn evaluate_metadata(
        &self,
        change: &GovernanceChangeEvidence,
        active_changes: &[GovernanceChangeEvidence],
        issues: &mut Vec<GovernanceBlockingIssue>,
    ) {
        if change.metadata.id != change.id {
            issues.push(conflict_issue(
                "metadata.id.matches_path",
                Some(&change.id),
                None,
                "metadata id does not match change directory id",
            ));
        }
        if !STAGES.contains(&change.metadata.stage.as_str()) {
            issues.push(conflict_issue(
                "metadata.stage.valid",
                Some(&change.id),
                None,
                "metadata stage is not a supported UniSpec stage",
            ));
        }
        if let Some(shape) = &change.metadata.delivery_shape {
            if !matches!(shape.as_str(), "single-change" | "multi-change") {
                issues.push(conflict_issue(
                    "metadata.delivery_shape.valid",
                    Some(&change.id),
                    None,
                    "deliveryShape must be single-change or multi-change",
                ));
            }
        }

        let multi = change.metadata.multi_change.as_ref();
        let standalone_exploration = change.metadata.stage == "exploration"
            && change.metadata.delivery_shape.as_deref() == Some("single-change")
            && multi.is_none();
        if change.metadata.stage == "exploration"
            && !standalone_exploration
            && multi.is_none_or(|multi| !matches!(multi.role.as_str(), "parent" | "child"))
        {
            issues.push(conflict_issue(
                "metadata.exploration.role",
                Some(&change.id),
                None,
                "exploration change must be standalone or declare parent/child role",
            ));
        }

        let Some(multi) = multi else {
            return;
        };
        match multi.role.as_str() {
            "parent" => {
                if change.metadata.delivery_shape.as_deref() != Some("multi-change") {
                    issues.push(conflict_issue(
                        "metadata.parent.delivery_shape",
                        Some(&change.id),
                        None,
                        "parent must use multi-change delivery shape",
                    ));
                }
                if multi.children.is_empty() {
                    issues.push(conflict_issue(
                        "metadata.parent.children",
                        Some(&change.id),
                        None,
                        "parent must declare at least one child",
                    ));
                    return;
                }
                let child_ids = multi
                    .children
                    .iter()
                    .map(|child| child.id.as_str())
                    .collect::<BTreeSet<_>>();
                if child_ids.len() != multi.children.len() {
                    issues.push(conflict_issue(
                        "metadata.parent.child_unique",
                        Some(&change.id),
                        None,
                        "parent child ids must be unique",
                    ));
                }
                for child in &multi.children {
                    if !child.id.starts_with(&format!("{}-", change.id)) {
                        issues.push(conflict_issue(
                            "metadata.parent.child_prefix",
                            Some(&change.id),
                            None,
                            "child id must start with parent id prefix",
                        ));
                    }
                    if child
                        .depends_on
                        .iter()
                        .any(|dependency| !child_ids.contains(dependency.as_str()))
                    {
                        issues.push(conflict_issue(
                            "metadata.parent.dependency_declared",
                            Some(&change.id),
                            None,
                            "child dependency must reference a declared sibling",
                        ));
                    }
                }
            }
            "child" => {
                if change.metadata.delivery_shape.as_deref() != Some("single-change") {
                    issues.push(conflict_issue(
                        "metadata.child.delivery_shape",
                        Some(&change.id),
                        None,
                        "child must use single-change delivery shape",
                    ));
                }
                if multi.order.is_none_or(|order| order == 0) {
                    issues.push(conflict_issue(
                        "metadata.child.order",
                        Some(&change.id),
                        None,
                        "child must declare a positive order",
                    ));
                }
                let Some(parent_id) = multi.parent.as_deref() else {
                    issues.push(conflict_issue(
                        "metadata.child.parent",
                        Some(&change.id),
                        None,
                        "child must declare parent",
                    ));
                    return;
                };
                if !change.id.starts_with(&format!("{parent_id}-")) {
                    issues.push(conflict_issue(
                        "metadata.child.parent_prefix",
                        Some(&change.id),
                        None,
                        "child id must start with parent id prefix",
                    ));
                }
                if let Some(parent) = active_changes
                    .iter()
                    .find(|candidate| candidate.id == parent_id)
                {
                    self.evaluate_child_against_parent(change, parent, issues);
                }
            }
            _ => issues.push(conflict_issue(
                "metadata.multi_change.role",
                Some(&change.id),
                None,
                "multiChange role must be parent or child",
            )),
        }
    }

    fn evaluate_child_against_parent(
        &self,
        child: &GovernanceChangeEvidence,
        parent: &GovernanceChangeEvidence,
        issues: &mut Vec<GovernanceBlockingIssue>,
    ) {
        let Some(parent_multi) = parent.metadata.multi_change.as_ref() else {
            issues.push(conflict_issue(
                "metadata.child.parent_consistent",
                Some(&child.id),
                None,
                "declared parent has no multiChange metadata",
            ));
            return;
        };
        let Some(reference) = parent_multi
            .children
            .iter()
            .find(|item| item.id == child.id)
        else {
            issues.push(conflict_issue(
                "metadata.child.parent_consistent",
                Some(&child.id),
                None,
                "parent does not declare child",
            ));
            return;
        };
        let child_multi = child
            .metadata
            .multi_change
            .as_ref()
            .expect("checked by caller");
        let mut left_dependencies = child_multi.depends_on.clone();
        let mut right_dependencies = reference.depends_on.clone();
        left_dependencies.sort();
        right_dependencies.sort();
        if child_multi.order != Some(reference.order) || left_dependencies != right_dependencies {
            issues.push(conflict_issue(
                "metadata.child.parent_consistent",
                Some(&child.id),
                None,
                "child order or dependencies differ from parent metadata",
            ));
        }
    }

    fn evaluate_parent_archive(
        &self,
        parent: &GovernanceChangeEvidence,
        active_ids: &BTreeSet<&str>,
        archived_by_id: &BTreeMap<&str, Vec<&GovernanceChangeEvidence>>,
        issues: &mut Vec<GovernanceBlockingIssue>,
    ) -> GovernanceGateStatus {
        let Some(multi) = parent.metadata.multi_change.as_ref() else {
            return GovernanceGateStatus::Conflict;
        };
        let mut all_archived = !multi.children.is_empty();
        let mut conflict = false;
        for child in &multi.children {
            let archived_entries = archived_by_id.get(child.id.as_str());
            let archived = archived_entries.is_some_and(|entries| entries.len() == 1);
            let active = active_ids.contains(child.id.as_str());
            let claims_archived = child.archive_status.as_deref() == Some("archived");
            if claims_archived {
                let marker_valid = archive_marker_valid(
                    parent,
                    child,
                    archived_entries.and_then(|entries| entries.first().copied()),
                    active,
                );
                if !marker_valid {
                    conflict = true;
                    issues.push(conflict_issue(
                        "parent.archive_marker.consistent",
                        Some(&parent.id),
                        None,
                        &format!("archive marker is inconsistent for child {}", child.id),
                    ));
                }
            } else if archived {
                conflict = true;
                issues.push(conflict_issue(
                    "parent.archive_marker.consistent",
                    Some(&parent.id),
                    None,
                    &format!("archived child {} is missing parent markers", child.id),
                ));
            }
            if !archived || active {
                all_archived = false;
            }
        }
        if conflict {
            GovernanceGateStatus::Conflict
        } else if all_archived {
            GovernanceGateStatus::Passed
        } else {
            GovernanceGateStatus::Pending
        }
    }
}

fn required_artifacts(change: &GovernanceChangeEvidence) -> &'static [&'static str] {
    let multi = change.metadata.multi_change.as_ref();
    if multi.is_some_and(|multi| multi.role == "parent") {
        return &["split", "metadata"];
    }
    if change.metadata.stage == "exploration" {
        return &["metadata"];
    }
    match change.metadata.stage.as_str() {
        "proposal" | "delivery" => &["proposal", "metadata"],
        "design" => &["proposal", "design", "metadata"],
        "cases" => &["proposal", "design", "cases", "metadata"],
        "tasks" | "implementation" | "review" => {
            &["proposal", "design", "cases", "tasks", "metadata"]
        }
        "verification" | "archive" => &[
            "proposal",
            "design",
            "cases",
            "tasks",
            "review-report",
            "test-report",
            "metadata",
        ],
        _ => &["metadata"],
    }
}

fn archive_marker_valid(
    parent: &GovernanceChangeEvidence,
    child: &GovernanceChildEvidence,
    archived: Option<&GovernanceChangeEvidence>,
    active: bool,
) -> bool {
    child
        .archived_at
        .as_deref()
        .is_some_and(|value| !value.is_empty())
        && child.archived_to.as_deref().is_some_and(|path| {
            path.starts_with(".spec/archive/")
                && path
                    .rsplit_once('/')
                    .is_some_and(|(_, entry)| archive_entry_matches(entry, &child.id))
        })
        && archived.is_some_and(|archived| {
            child.archived_to.as_deref() == Some(archived.relative_path.as_str())
        })
        && !active
        && parent.split_archived_children.contains(&child.id)
}

fn archive_entry_matches(entry: &str, child_id: &str) -> bool {
    let bytes = entry.as_bytes();
    bytes.len() > 11
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes.get(10) == Some(&b'-')
        && &entry[11..] == child_id
}

fn artifacts_for_change(
    snapshot: &GovernanceEvidenceSnapshot,
    change: &GovernanceChangeEvidence,
) -> Vec<GovernanceArtifactRef> {
    let prefix = format!("{}/", change.relative_path);
    snapshot
        .artifacts
        .iter()
        .filter(|artifact| artifact.relative_path.starts_with(&prefix))
        .cloned()
        .collect()
}

fn artifact_by_kind(
    artifacts: &[GovernanceArtifactRef],
    kind: &str,
) -> Option<GovernanceArtifactRef> {
    artifacts
        .iter()
        .find(|artifact| artifact.kind == kind)
        .cloned()
}

fn classify_failure(
    failure: &GovernanceEvidenceFailure,
    snapshot: &GovernanceEvidenceSnapshot,
) -> GovernanceBlockingIssue {
    let artifact_ref = snapshot
        .artifacts
        .iter()
        .find(|artifact| artifact.relative_path == failure.relative_path)
        .cloned();
    conflict_issue(
        &failure.rule_id,
        failure.change_id.as_deref(),
        artifact_ref,
        &failure.message,
    )
}

fn blocked_issue(
    rule_id: &str,
    change_id: Option<&str>,
    artifact_ref: Option<GovernanceArtifactRef>,
    message: &str,
) -> GovernanceBlockingIssue {
    GovernanceBlockingIssue {
        rule_id: rule_id.to_string(),
        severity: GovernanceIssueSeverity::Blocking,
        message: message.to_string(),
        change_id: change_id.map(str::to_string),
        artifact_ref,
        recommended_action: GovernanceRecommendedAction::ReviewGovernance,
    }
}

fn conflict_issue(
    rule_id: &str,
    change_id: Option<&str>,
    artifact_ref: Option<GovernanceArtifactRef>,
    message: &str,
) -> GovernanceBlockingIssue {
    blocked_issue(rule_id, change_id, artifact_ref, message)
}

fn is_conflict_issue(issue: &GovernanceBlockingIssue) -> bool {
    !issue.rule_id.starts_with("artifact.required.")
        && !matches!(
            issue.rule_id.as_str(),
            "report.review.pass"
                | "report.review.scope_full"
                | "report.verification.pass"
                | "report.verification.scope_full"
                | "dependency.required"
        )
}

fn issue_to_rule_result(issue: &GovernanceBlockingIssue) -> GovernanceRuleResult {
    GovernanceRuleResult {
        rule_id: issue.rule_id.clone(),
        passed: false,
        severity: issue.severity,
        message: Some(issue.message.clone()),
        artifact_ref: issue.artifact_ref.clone(),
    }
}
