//! governance workflow 组合 live `.spec` evidence、确定性 policy 与可重建 SQLite cache。
//! 本模块不执行 archive 或修复，也不把治理问题写入 core runtime readiness。

use std::io;
use std::path::{Path, PathBuf};

use wiki_model::domain::governance::{
    GovernanceArtifactRef, GovernanceChangeSummary, GovernanceGateStatus, GovernanceIssueSeverity,
    GovernanceReadiness, GovernanceRecommendedAction, GovernanceSummary,
    GovernanceValidationResult,
};
use wiki_model::domain::query::{
    QueryConfidence, QueryProvenance, QueryRefKind, QueryResultDto, QueryRouteTag, QuerySourceRef,
    RecommendedAction,
};

use crate::domain::governance::{GovernanceEvaluation, GovernancePolicy};
use crate::storage::governance_fs::FsGovernanceEvidenceStore;
use crate::storage::sqlite::governance_store::{GovernanceCacheSnapshot, SqliteGovernanceCache};
use crate::storage::sqlite_store;

/// 当前治理 cache schema 版本。
pub const GOVERNANCE_CACHE_SCHEMA_VERSION: &str = "1";

/// Repo-local 只读治理服务。
#[derive(Debug, Clone)]
pub struct GovernanceService {
    repo_root: PathBuf,
    policy: GovernancePolicy,
}

impl GovernanceService {
    /// 为仓库创建使用当前 policy 的治理服务。
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
            policy: GovernancePolicy::v1(),
        }
    }

    /// 读取 live evidence 并执行 policy，不消费 derived cache。
    pub fn evaluate_live(&self) -> io::Result<GovernanceEvaluation> {
        let snapshot = FsGovernanceEvidenceStore::new(&self.repo_root).discover_repo()?;
        Ok(self.policy.evaluate(&snapshot))
    }

    /// 返回产品级治理状态；status 只比较 cache freshness，不写 cache。
    pub fn status(&self) -> io::Result<GovernanceSummary> {
        let evaluation = self.evaluate_live()?;
        let mut summary = evaluation.summary.clone();
        if matches!(
            summary.readiness,
            GovernanceReadiness::NotEnabled
                | GovernanceReadiness::Blocked
                | GovernanceReadiness::Conflict
        ) || summary.active_count + summary.archived_count == 0
        {
            return Ok(summary);
        }

        let Some(fingerprint) = summary.fingerprint.as_deref() else {
            return Ok(summary);
        };
        if self.read_cached(fingerprint)?.is_none() {
            summary.readiness = GovernanceReadiness::Stale;
            summary.recommended_action = GovernanceRecommendedAction::Update;
        }
        Ok(summary)
    }

    /// 从 live evidence 刷新 SQLite derived cache，并返回刷新后的 live summary。
    pub fn refresh(&self) -> io::Result<GovernanceSummary> {
        let evaluation = self.evaluate_live()?;
        if evaluation.summary.readiness == GovernanceReadiness::NotEnabled {
            if sqlite_store::db_exists(&self.repo_root) {
                let mut conn = sqlite_store::open_db(&self.repo_root)?;
                SqliteGovernanceCache::new(&mut conn).clear()?;
            }
            return Ok(evaluation.summary);
        }

        let cache_snapshot = cache_snapshot(&evaluation, self.policy.version())?;
        let mut conn = sqlite_store::open_db(&self.repo_root)?;
        SqliteGovernanceCache::new(&mut conn).replace_snapshot(&cache_snapshot)?;
        Ok(evaluation.summary)
    }

    /// 列出 live evidence 中可解释的 active/archive changes。
    pub fn list_changes(&self) -> io::Result<Vec<GovernanceChangeSummary>> {
        Ok(self.evaluate_live()?.changes)
    }

    /// 读取单个 live change summary。
    pub fn inspect_change(&self, change_id: &str) -> io::Result<GovernanceChangeSummary> {
        self.evaluate_live()?
            .changes
            .into_iter()
            .find(|change| change.id == change_id)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("governance change not found: {change_id}"),
                )
            })
    }

    /// 对单个 change 执行 live validate；不会因 cache stale 而失败。
    pub fn validate_change(&self, change_id: &str) -> io::Result<GovernanceValidationResult> {
        let evaluation = self.evaluate_live()?;
        let change = evaluation
            .changes
            .iter()
            .find(|change| change.id == change_id)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("governance change not found: {change_id}"),
                )
            })?;
        let issues = evaluation
            .summary
            .issues
            .iter()
            .filter(|issue| issue.change_id.as_deref() == Some(change_id))
            .cloned()
            .collect::<Vec<_>>();
        let rule_results = evaluation
            .rule_results
            .iter()
            .filter(|result| {
                result
                    .artifact_ref
                    .as_ref()
                    .is_some_and(|artifact| artifact.change_id == change_id)
                    || issues.iter().any(|issue| issue.rule_id == result.rule_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        let gate_conflict = [
            change.gate.artifact_gate,
            change.gate.review_gate,
            change.gate.verification_gate,
            change.gate.archive_readiness,
        ]
        .contains(&GovernanceGateStatus::Conflict);
        let readiness = if gate_conflict {
            GovernanceReadiness::Conflict
        } else if issues
            .iter()
            .any(|issue| issue.severity == GovernanceIssueSeverity::Blocking)
        {
            GovernanceReadiness::Blocked
        } else {
            GovernanceReadiness::Ready
        };
        Ok(GovernanceValidationResult {
            valid: matches!(readiness, GovernanceReadiness::Ready),
            readiness,
            rule_results,
            issues,
        })
    }

    /// 从 fingerprint 一致的 cache 查询结构化 change/artifact refs。
    pub fn query_refs(&self, term: &str, limit: usize) -> io::Result<Vec<QueryResultDto>> {
        let evaluation = self.evaluate_live()?;
        let summary = evaluation.summary.clone();
        if matches!(summary.readiness, GovernanceReadiness::NotEnabled) {
            return Ok(Vec::new());
        }
        if summary.readiness == GovernanceReadiness::Conflict {
            return Ok(query_issue_refs(&summary, term, limit));
        }
        let Some(fingerprint) = summary.fingerprint.as_deref() else {
            return Ok(Vec::new());
        };
        let Some(cache) = self.read_cached(fingerprint)? else {
            return if summary.readiness == GovernanceReadiness::Blocked {
                Ok(query_issue_refs(&summary, term, limit))
            } else {
                Ok(Vec::new())
            };
        };
        let changes = parse_values::<GovernanceChangeSummary>("change", &cache.changes)?;
        let artifacts =
            parse_values::<GovernanceArtifactRef>("artifact ref", &cache.artifact_refs)?;
        Ok(query_cached_refs(
            &summary, &changes, &artifacts, term, limit,
        ))
    }

    fn read_cached(&self, fingerprint: &str) -> io::Result<Option<GovernanceCacheSnapshot>> {
        if !sqlite_store::db_exists(&self.repo_root) {
            return Ok(None);
        }
        let mut conn = match sqlite_store::open_db_readonly(&self.repo_root) {
            Ok(conn) => conn,
            Err(_) => return Ok(None),
        };
        match SqliteGovernanceCache::new(&mut conn).read_snapshot(
            fingerprint,
            GOVERNANCE_CACHE_SCHEMA_VERSION,
            self.policy.version(),
        ) {
            Ok(snapshot) => Ok(snapshot),
            Err(_) => Ok(None),
        }
    }
}

fn cache_snapshot(
    evaluation: &GovernanceEvaluation,
    policy_version: &str,
) -> io::Result<GovernanceCacheSnapshot> {
    let fingerprint = evaluation
        .summary
        .fingerprint
        .clone()
        .ok_or_else(|| io::Error::other("enabled governance evaluation has no fingerprint"))?;
    Ok(GovernanceCacheSnapshot {
        schema_version: GOVERNANCE_CACHE_SCHEMA_VERSION.to_string(),
        policy_version: policy_version.to_string(),
        evidence_fingerprint: fingerprint,
        summary: serde_json::to_value(&evaluation.summary)
            .map_err(|error| io::Error::other(format!("serialize governance summary: {error}")))?,
        changes: serialize_values("change", &evaluation.changes)?,
        artifact_refs: serialize_values("artifact ref", &evaluation.artifacts)?,
        issues: serialize_values("issue", &evaluation.summary.issues)?,
    })
}

fn serialize_values<T: serde::Serialize>(
    kind: &str,
    values: &[T],
) -> io::Result<Vec<serde_json::Value>> {
    values
        .iter()
        .map(|value| {
            serde_json::to_value(value)
                .map_err(|error| io::Error::other(format!("serialize governance {kind}: {error}")))
        })
        .collect()
}

fn parse_values<T: serde::de::DeserializeOwned>(
    kind: &str,
    values: &[serde_json::Value],
) -> io::Result<Vec<T>> {
    values
        .iter()
        .cloned()
        .map(|value| {
            serde_json::from_value(value)
                .map_err(|error| io::Error::other(format!("parse governance {kind}: {error}")))
        })
        .collect()
}

fn query_cached_refs(
    summary: &GovernanceSummary,
    changes: &[GovernanceChangeSummary],
    artifacts: &[GovernanceArtifactRef],
    term: &str,
    limit: usize,
) -> Vec<QueryResultDto> {
    let needle = term.trim().to_ascii_lowercase();
    if needle.is_empty() || limit == 0 {
        return Vec::new();
    }
    let mut results = Vec::new();
    for change in changes {
        let searchable = format!(
            "{} {} {} {} {}",
            change.id,
            change.stage,
            change.role.as_deref().unwrap_or_default(),
            change.parent.as_deref().unwrap_or_default(),
            change.depends_on.join(" ")
        )
        .to_ascii_lowercase();
        if searchable.contains(&needle) {
            results.push(QueryResultDto {
                route_tag: QueryRouteTag::GovernanceSummaryHit,
                ref_kind: QueryRefKind::GovernanceChange,
                ref_id: change.id.clone(),
                label: format!("{} ({})", change.id, change.stage),
                score: if change.id.eq_ignore_ascii_case(term.trim()) {
                    1.0
                } else {
                    0.85
                },
                provenance: governance_provenance(summary),
                confidence: governance_confidence(summary),
                recommended_action: governance_query_action(summary),
                source_refs: Vec::new(),
            });
        }
    }
    for artifact in artifacts {
        let searchable = format!(
            "{} {} {}",
            artifact.change_id, artifact.kind, artifact.relative_path
        )
        .to_ascii_lowercase();
        if searchable.contains(&needle) {
            results.push(QueryResultDto {
                route_tag: QueryRouteTag::GovernanceEvidenceRef,
                ref_kind: QueryRefKind::GovernanceArtifact,
                ref_id: format!("{}:{}", artifact.change_id, artifact.kind),
                label: artifact.relative_path.clone(),
                score: 0.8,
                provenance: governance_provenance(summary),
                confidence: governance_confidence(summary),
                recommended_action: governance_query_action(summary),
                source_refs: vec![QuerySourceRef {
                    ref_kind: QueryRefKind::GovernanceArtifact,
                    ref_id: format!("{}:{}", artifact.change_id, artifact.kind),
                    label: Some(artifact.kind.clone()),
                    path: Some(artifact.relative_path.clone()),
                    start_line: None,
                    end_line: None,
                    provenance: vec!["governance:evidence_ref".to_string()],
                    diagnostics: Vec::new(),
                }],
            });
        }
    }
    results.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then(left.ref_id.cmp(&right.ref_id))
    });
    results.truncate(limit);
    results
}

fn query_issue_refs(summary: &GovernanceSummary, term: &str, limit: usize) -> Vec<QueryResultDto> {
    let needle = term.trim().to_ascii_lowercase();
    summary
        .issues
        .iter()
        .filter(|issue| {
            format!(
                "{} {}",
                issue.rule_id,
                issue.change_id.as_deref().unwrap_or_default()
            )
            .to_ascii_lowercase()
            .contains(&needle)
        })
        .take(limit)
        .map(|issue| QueryResultDto {
            route_tag: QueryRouteTag::GovernanceEvidenceRef,
            ref_kind: QueryRefKind::GovernanceChange,
            ref_id: issue
                .change_id
                .clone()
                .unwrap_or_else(|| issue.rule_id.clone()),
            label: issue.rule_id.clone(),
            score: 0.5,
            provenance: governance_provenance(summary),
            confidence: QueryConfidence::Low,
            recommended_action: RecommendedAction::ReviewGovernance,
            source_refs: issue
                .artifact_ref
                .iter()
                .map(|artifact| QuerySourceRef {
                    ref_kind: QueryRefKind::GovernanceArtifact,
                    ref_id: format!("{}:{}", artifact.change_id, artifact.kind),
                    label: Some(artifact.kind.clone()),
                    path: Some(artifact.relative_path.clone()),
                    start_line: None,
                    end_line: None,
                    provenance: vec!["governance:diagnostic_ref".to_string()],
                    diagnostics: vec![issue.rule_id.clone()],
                })
                .collect(),
        })
        .collect()
}

fn governance_provenance(summary: &GovernanceSummary) -> QueryProvenance {
    QueryProvenance {
        layer: "governance".to_string(),
        state: Some(format!("{:?}", summary.readiness).to_ascii_lowercase()),
        reason: summary.issues.first().map(|issue| issue.rule_id.clone()),
    }
}

fn governance_confidence(summary: &GovernanceSummary) -> QueryConfidence {
    match summary.readiness {
        GovernanceReadiness::Ready => QueryConfidence::High,
        GovernanceReadiness::Blocked => QueryConfidence::Medium,
        _ => QueryConfidence::Low,
    }
}

fn governance_query_action(summary: &GovernanceSummary) -> RecommendedAction {
    match summary.recommended_action {
        GovernanceRecommendedAction::None => RecommendedAction::OpenReference,
        GovernanceRecommendedAction::Update => RecommendedAction::Update,
        GovernanceRecommendedAction::ReviewGovernance => RecommendedAction::ReviewGovernance,
    }
}

/// Convenience helper for callers that only have a repo path.
pub fn governance_status(repo_root: &Path) -> io::Result<GovernanceSummary> {
    GovernanceService::new(repo_root).status()
}
