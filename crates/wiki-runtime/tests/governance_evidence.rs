use std::fs;
use std::path::Path;

use tempfile::TempDir;
use wiki_model::domain::governance::{GovernanceArtifactStatus, GovernanceLocation};
use wiki_runtime::storage::governance_fs::FsGovernanceEvidenceStore;

#[test]
fn governance_evidence_store_distinguishes_disabled_and_enabled_empty_repositories() {
    let disabled = TempDir::new().expect("temp repo");
    let disabled_snapshot = FsGovernanceEvidenceStore::new(disabled.path())
        .discover_repo()
        .expect("discover disabled repo");
    assert!(!disabled_snapshot.enabled);
    assert!(disabled_snapshot.active_changes.is_empty());
    assert!(disabled_snapshot.archived_changes.is_empty());

    let enabled = TempDir::new().expect("temp repo");
    fs::create_dir_all(enabled.path().join(".spec/changes")).expect("create changes root");
    fs::create_dir_all(enabled.path().join(".spec/archive")).expect("create archive root");
    let enabled_snapshot = FsGovernanceEvidenceStore::new(enabled.path())
        .discover_repo()
        .expect("discover enabled repo");
    assert!(enabled_snapshot.enabled);
    assert!(enabled_snapshot.active_changes.is_empty());
    assert!(enabled_snapshot.archived_changes.is_empty());
    assert!(enabled_snapshot.failures.is_empty());
}

#[test]
fn governance_evidence_store_discovers_valid_changes_and_isolates_failures() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        ".spec/changes/parent-child",
        child_metadata("parent-child", "parent", 2),
        &[("proposal.md", "# proposal\n")],
    );
    write_change(
        repo.path(),
        ".spec/archive/2026-07-01-done",
        standalone_metadata("done", "archive"),
        &[("proposal.md", "# proposal\n")],
    );
    write_change(
        repo.path(),
        ".spec/changes/broken",
        "id: [not valid\nstage: tasks\n",
        &[],
    );

    let snapshot = FsGovernanceEvidenceStore::new(repo.path())
        .discover_repo()
        .expect("discover governance evidence");

    assert_eq!(snapshot.active_changes.len(), 1);
    assert_eq!(snapshot.active_changes[0].id, "parent-child");
    assert_eq!(
        snapshot.active_changes[0].location,
        GovernanceLocation::Active
    );
    assert_eq!(snapshot.active_changes[0].metadata.stage, "tasks");
    assert_eq!(snapshot.archived_changes.len(), 1);
    assert_eq!(snapshot.archived_changes[0].id, "done");
    assert_eq!(
        snapshot.archived_changes[0].location,
        GovernanceLocation::Archived
    );
    assert_eq!(snapshot.failures.len(), 1);
    assert_eq!(snapshot.failures[0].change_id.as_deref(), Some("broken"));
    assert_eq!(snapshot.failures[0].rule_id, "evidence.metadata.parse");

    let proposal = snapshot
        .artifacts
        .iter()
        .find(|artifact| artifact.change_id == "parent-child" && artifact.kind == "proposal")
        .expect("proposal reference");
    assert_eq!(proposal.status, GovernanceArtifactStatus::Present);
    assert_eq!(
        proposal.relative_path,
        ".spec/changes/parent-child/proposal.md"
    );
    assert!(proposal
        .content_hash
        .as_deref()
        .is_some_and(|hash| !hash.is_empty()));
    assert!(snapshot
        .artifacts
        .iter()
        .all(|artifact| !artifact.relative_path.contains("..")));
}

#[test]
fn governance_fingerprint_depends_on_content_and_not_file_timestamps() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        ".spec/changes/alpha",
        standalone_metadata("alpha", "proposal"),
        &[("proposal.md", "# first\n")],
    );
    let store = FsGovernanceEvidenceStore::new(repo.path());
    let first = store.discover_repo().expect("first discovery");

    fs::write(
        repo.path().join(".spec/changes/alpha/proposal.md"),
        "# first\n",
    )
    .expect("rewrite identical content");
    let same_content = store.discover_repo().expect("second discovery");
    assert_eq!(first.fingerprint, same_content.fingerprint);

    fs::write(
        repo.path().join(".spec/changes/alpha/proposal.md"),
        "# second\n",
    )
    .expect("change content");
    let changed = store.discover_repo().expect("third discovery");
    assert_ne!(first.fingerprint, changed.fingerprint);
}

#[test]
fn governance_evidence_store_reports_oversized_known_artifacts_without_hiding_other_changes() {
    let repo = TempDir::new().expect("temp repo");
    write_change(
        repo.path(),
        ".spec/changes/good",
        standalone_metadata("good", "proposal"),
        &[("proposal.md", "ok")],
    );
    let oversized = "x".repeat(512);
    write_change(
        repo.path(),
        ".spec/changes/large",
        standalone_metadata("large", "proposal"),
        &[("proposal.md", oversized.as_str())],
    );

    let snapshot = FsGovernanceEvidenceStore::new(repo.path())
        .with_max_file_bytes(256)
        .discover_repo()
        .expect("discover governance evidence");

    assert!(snapshot
        .active_changes
        .iter()
        .any(|change| change.id == "good"));
    assert!(snapshot
        .failures
        .iter()
        .any(|failure| failure.change_id.as_deref() == Some("large")
            && failure.rule_id == "evidence.artifact.too_large"));
}

#[cfg(unix)]
#[test]
fn governance_evidence_store_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let repo = TempDir::new().expect("temp repo");
    let outside = TempDir::new().expect("outside repo");
    fs::write(outside.path().join("proposal.md"), "outside").expect("outside artifact");
    write_change(
        repo.path(),
        ".spec/changes/escape",
        standalone_metadata("escape", "proposal"),
        &[],
    );
    symlink(
        outside.path().join("proposal.md"),
        repo.path().join(".spec/changes/escape/proposal.md"),
    )
    .expect("create symlink");

    let snapshot = FsGovernanceEvidenceStore::new(repo.path())
        .discover_repo()
        .expect("discover governance evidence");
    assert!(snapshot.failures.iter().any(|failure| {
        failure.change_id.as_deref() == Some("escape") && failure.rule_id == "evidence.path.escape"
    }));
}

#[cfg(unix)]
#[test]
fn governance_evidence_store_rejects_spec_root_symlink_escape() {
    use std::os::unix::fs::symlink;

    let repo = TempDir::new().expect("temp repo");
    let outside = TempDir::new().expect("outside repo");
    fs::create_dir_all(outside.path().join("changes/escape")).expect("outside change dir");
    fs::write(
        outside.path().join("changes/escape/meta.yaml"),
        standalone_metadata("escape", "proposal"),
    )
    .expect("outside metadata");
    symlink(outside.path(), repo.path().join(".spec")).expect("symlink spec root");

    let snapshot = FsGovernanceEvidenceStore::new(repo.path())
        .discover_repo()
        .expect("discover governance evidence");
    assert!(snapshot.active_changes.is_empty());
    assert!(snapshot
        .failures
        .iter()
        .any(|failure| failure.rule_id == "evidence.path.escape"));
}

fn write_change(
    root: &Path,
    relative_dir: &str,
    metadata: impl AsRef<[u8]>,
    files: &[(&str, &str)],
) {
    let change_dir = root.join(relative_dir);
    fs::create_dir_all(&change_dir).expect("create change dir");
    fs::write(change_dir.join("meta.yaml"), metadata).expect("write metadata");
    for (name, content) in files {
        fs::write(change_dir.join(name), content).expect("write artifact");
    }
}

fn standalone_metadata(id: &str, stage: &str) -> String {
    format!("id: {id}\nstage: {stage}\ndeliveryShape: single-change\n")
}

fn child_metadata(id: &str, parent: &str, order: u32) -> String {
    format!(
        "id: {id}\nstage: tasks\ndeliveryShape: single-change\nmultiChange:\n  role: child\n  parent: {parent}\n  order: {order}\n  dependsOn: []\n"
    )
}
