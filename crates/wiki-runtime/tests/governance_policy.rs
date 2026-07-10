use std::fs;
use std::path::Path;

use tempfile::TempDir;
use wiki_model::domain::governance::{GovernanceGateStatus, GovernanceReadiness};
use wiki_runtime::domain::governance::GovernancePolicy;
use wiki_runtime::storage::governance_fs::FsGovernanceEvidenceStore;

#[test]
fn governance_policy_matches_versioned_stage_and_metadata_fixtures() {
    let ready_repo = TempDir::new().expect("temp repo");
    write_change(
        ready_repo.path(),
        "alpha",
        standalone_metadata("alpha", "design"),
        &[("proposal.md", "# proposal\n"), ("design.md", "# design\n")],
    );
    let ready = evaluate(ready_repo.path());
    assert_eq!(ready.readiness, GovernanceReadiness::Ready);
    assert!(ready.issues.is_empty());
    let alpha = ready
        .changes
        .iter()
        .find(|change| change.id == "alpha")
        .expect("alpha summary");
    assert_eq!(alpha.gate.artifact_gate, GovernanceGateStatus::Passed);
    assert_eq!(alpha.gate.archive_readiness, GovernanceGateStatus::Pending);

    let missing_repo = TempDir::new().expect("temp repo");
    write_change(
        missing_repo.path(),
        "missing-tasks",
        standalone_metadata("missing-tasks", "tasks"),
        &[
            ("proposal.md", "# proposal\n"),
            ("design.md", "# design\n"),
            ("system-tests.md", "# cases\n"),
        ],
    );
    let missing = evaluate(missing_repo.path());
    assert_eq!(missing.readiness, GovernanceReadiness::Blocked);
    assert!(missing
        .issues
        .iter()
        .any(|issue| issue.rule_id == "artifact.required.tasks"));

    let mismatch_repo = TempDir::new().expect("temp repo");
    write_change(
        mismatch_repo.path(),
        "path-id",
        standalone_metadata("metadata-id", "proposal"),
        &[("proposal.md", "# proposal\n")],
    );
    let mismatch = evaluate(mismatch_repo.path());
    assert_eq!(mismatch.readiness, GovernanceReadiness::Conflict);
    assert!(mismatch
        .issues
        .iter()
        .any(|issue| issue.rule_id == "metadata.id.matches_path"));
}

#[test]
fn governance_policy_validates_parent_child_metadata_consistency() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        "parent",
        "id: parent\nstage: exploration\ndeliveryShape: multi-change\nmultiChange:\n  role: parent\n  children:\n    - id: unrelated-child\n      order: 1\n      dependsOn: [missing-sibling]\n",
        &[("split.md", "# split\n")],
    );
    write_change(
        repo.path(),
        "bad-child",
        "id: bad-child\nstage: exploration\ndeliveryShape: multi-change\nmultiChange:\n  role: child\n  parent: parent\n",
        &[],
    );

    let result = evaluate(repo.path());
    assert_eq!(result.readiness, GovernanceReadiness::Conflict);
    for rule_id in [
        "metadata.parent.child_prefix",
        "metadata.parent.dependency_declared",
        "metadata.child.delivery_shape",
        "metadata.child.order",
    ] {
        assert!(
            result.issues.iter().any(|issue| issue.rule_id == rule_id),
            "missing rule {rule_id}"
        );
    }
}

#[test]
fn normal_in_progress_stage_is_not_blocked_by_archive_readiness() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        "designing",
        standalone_metadata("designing", "design"),
        &[("proposal.md", "# proposal\n"), ("design.md", "# design\n")],
    );

    let result = evaluate(repo.path());
    assert_eq!(result.readiness, GovernanceReadiness::Ready);
    assert!(result.issues.is_empty());
    assert_eq!(
        result.changes[0].gate.archive_readiness,
        GovernanceGateStatus::Pending
    );
}

#[test]
fn governance_policy_classifies_report_and_archive_evidence() {
    let blocked_repo = TempDir::new().expect("temp repo");
    write_change(
        blocked_repo.path(),
        "verifying",
        standalone_metadata("verifying", "verification"),
        &[
            ("proposal.md", "# proposal\n"),
            ("design.md", "# design\n"),
            ("system-tests.md", "# cases\n"),
            ("tasks.md", "# tasks\n"),
            (
                "review-report.md",
                "---\nreview-result: fail\nscope: full\n---\n# review\n",
            ),
            (
                "test-report.md",
                "---\nverification-result: pass\nscope: full\n---\n# tests\n",
            ),
        ],
    );
    let blocked = evaluate(blocked_repo.path());
    assert_eq!(blocked.readiness, GovernanceReadiness::Blocked);
    assert!(blocked
        .issues
        .iter()
        .any(|issue| issue.rule_id == "report.review.pass"));

    let malformed_repo = TempDir::new().expect("temp repo");
    write_change(
        malformed_repo.path(),
        "malformed",
        standalone_metadata("malformed", "verification"),
        &[
            ("proposal.md", "# proposal\n"),
            ("design.md", "# design\n"),
            ("system-tests.md", "# cases\n"),
            ("tasks.md", "# tasks\n"),
            ("review-report.md", "review-result: pass\n"),
            (
                "test-report.md",
                "---\nverification-result: pass\nscope: full\n---\n# tests\n",
            ),
        ],
    );
    let malformed = evaluate(malformed_repo.path());
    assert_eq!(malformed.readiness, GovernanceReadiness::Conflict);
    assert!(malformed
        .issues
        .iter()
        .any(|issue| issue.rule_id == "report.review.parse"));

    let marker_repo = TempDir::new().expect("temp repo");
    write_change(
        marker_repo.path(),
        "parent",
        "id: parent\nstage: exploration\ndeliveryShape: multi-change\nmultiChange:\n  role: parent\n  children:\n    - id: parent-child\n      order: 1\n      archiveStatus: archived\n      archivedAt: 2026-07-01T00:00:00Z\n      archivedTo: .spec/archive/2026-07-01-parent-child\n",
        &[("split.md", "# split\n")],
    );
    let marker = evaluate(marker_repo.path());
    assert_eq!(marker.readiness, GovernanceReadiness::Conflict);
    assert!(marker
        .issues
        .iter()
        .any(|issue| issue.rule_id == "parent.archive_marker.consistent"));
}

#[test]
fn governance_policy_requires_archive_marker_to_match_the_actual_directory() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        "parent",
        "id: parent\nstage: exploration\ndeliveryShape: multi-change\nmultiChange:\n  role: parent\n  children:\n    - id: parent-child\n      order: 1\n      archiveStatus: archived\n      archivedAt: 2026-07-02T00:00:00Z\n      archivedTo: .spec/archive/2026-07-02-parent-child\n",
        &[(
            "split.md",
            "### 1. parent-child\n- 归档状态：[x] archived\n",
        )],
    );
    let archived_dir = repo.path().join(".spec/archive/2026-07-01-parent-child");
    fs::create_dir_all(&archived_dir).expect("create archive dir");
    fs::write(
        archived_dir.join("meta.yaml"),
        standalone_metadata("parent-child", "archive"),
    )
    .expect("write archived metadata");

    let result = evaluate(repo.path());
    assert_eq!(result.readiness, GovernanceReadiness::Conflict);
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.rule_id == "parent.archive_marker.consistent"));
}

fn evaluate(repo_root: &Path) -> wiki_runtime::domain::governance::GovernanceEvaluation {
    let snapshot = FsGovernanceEvidenceStore::new(repo_root)
        .discover_repo()
        .expect("discover governance evidence");
    GovernancePolicy::v1().evaluate(&snapshot)
}

fn write_change(root: &Path, id: &str, metadata: impl AsRef<[u8]>, files: &[(&str, &str)]) {
    let change_dir = root.join(".spec/changes").join(id);
    fs::create_dir_all(&change_dir).expect("create change dir");
    fs::write(change_dir.join("meta.yaml"), metadata).expect("write metadata");
    for (name, content) in files {
        fs::write(change_dir.join(name), content).expect("write artifact");
    }
}

fn standalone_metadata(id: &str, stage: &str) -> String {
    format!("id: {id}\nstage: {stage}\ndeliveryShape: single-change\n")
}
