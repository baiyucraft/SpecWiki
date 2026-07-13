use std::fs;

use tempfile::TempDir;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use wiki_runtime::workflows::archive::{ArchivePlanOptions, ArchiveService};
use wiki_runtime::workflows::governance::GovernanceService;

fn ready_child_repo() -> TempDir {
    let repo = TempDir::new().unwrap();
    let root = repo.path();
    fs::create_dir_all(root.join(".spec/changes/parent-demo")).unwrap();
    fs::create_dir_all(root.join(".spec/changes/parent")).unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/meta.yaml"),
        "id: parent-demo\nstage: verification\ndeliveryShape: single-change\nmultiChange:\n  role: child\n  parent: parent\n  order: 1\n  dependsOn: []\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/proposal.md"),
        "# proposal\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/design.md"),
        "# design\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/system-tests.md"),
        "# cases\n",
    )
    .unwrap();
    fs::write(root.join(".spec/changes/parent-demo/tasks.md"), "# tasks\n").unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/review-report.md"),
        "---\nreview-result: pass\nscope: full\n---\n# review\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent-demo/test-report.md"),
        "---\nverification-result: pass\nscope: full\n---\n# tests\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent/meta.yaml"),
        "id: parent\nstage: exploration\ndeliveryShape: multi-change\nmultiChange:\n  role: parent\n  children:\n    - id: parent-demo\n      order: 1\n      dependsOn: []\n",
    )
    .unwrap();
    fs::write(
        root.join(".spec/changes/parent/split.md"),
        "### 1. parent-demo\n- 归档状态：[ ] pending\n",
    )
    .unwrap();
    repo
}

#[test]
fn archive_plan_uses_injected_utc_clock_and_operation_id() {
    let repo = ready_child_repo();
    let now = OffsetDateTime::parse("2026-07-13T23:59:59Z", &Rfc3339).unwrap();
    let service = ArchiveService::new(repo.path());
    let options = ArchivePlanOptions {
        now,
        operation_id: "op-fixed".to_string(),
    };

    let validation = GovernanceService::new(repo.path())
        .validate_report("parent-demo")
        .unwrap();
    assert!(
        validation.validation.valid,
        "{:?}",
        validation.validation.issues
    );
    let first = service
        .plan_with_options("parent-demo", options.clone())
        .unwrap();
    let next_day = OffsetDateTime::parse("2026-07-14T00:01:00Z", &Rfc3339).unwrap();
    let resumed = service
        .resume_target_for_test(&first.manifest, next_day)
        .unwrap();

    assert_eq!(first.manifest.operation_id, "op-fixed");
    assert_eq!(
        first.manifest.target_path,
        ".spec/archive/2026-07-13-parent-demo"
    );
    assert_eq!(resumed, first.manifest.target_path);
}

#[test]
fn apply_moves_child_updates_parent_and_preserves_child_bytes() {
    let repo = ready_child_repo();
    let source = repo.path().join(".spec/changes/parent-demo");
    let original_meta = fs::read(source.join("meta.yaml")).unwrap();
    let report = ArchiveService::new(repo.path())
        .apply("parent-demo")
        .unwrap();
    assert_eq!(
        report.manifest.outcome,
        wiki_model::domain::governance::ArchiveOutcome::Completed
    );
    assert!(!source.exists());
    let target = repo.path().join(&report.manifest.target_path);
    assert!(target.exists());
    assert_eq!(fs::read(target.join("meta.yaml")).unwrap(), original_meta);
    assert!(
        fs::read_to_string(repo.path().join(".spec/changes/parent/meta.yaml"))
            .unwrap()
            .contains("archiveStatus: archived")
    );
    assert!(
        fs::read_to_string(repo.path().join(".spec/changes/parent/split.md"))
            .unwrap()
            .contains("[x] archived")
    );
    assert!(repo
        .path()
        .join(".spec/.runtime/archive-operations")
        .join(&report.manifest.operation_id)
        .join("result.json")
        .exists());
}

#[test]
fn dry_run_does_not_create_runtime_or_mutate_truth() {
    let repo = ready_child_repo();
    let before = fs::read(repo.path().join(".spec/changes/parent-demo/meta.yaml")).unwrap();
    let report = ArchiveService::new(repo.path())
        .plan("parent-demo")
        .unwrap();
    assert!(!report.manifest.persisted);
    assert!(!report.manifest.resumable);
    assert!(!repo.path().join(".spec/.runtime").exists());
    assert_eq!(
        fs::read(repo.path().join(".spec/changes/parent-demo/meta.yaml")).unwrap(),
        before
    );
}

#[test]
fn resume_completed_operation_is_idempotent() {
    let repo = ready_child_repo();
    let service = ArchiveService::new(repo.path());
    let applied = service.apply("parent-demo").unwrap();
    let resumed = service
        .resume("parent-demo", &applied.manifest.operation_id)
        .unwrap();
    assert_eq!(
        resumed.manifest.outcome,
        wiki_model::domain::governance::ArchiveOutcome::AlreadyCompleted
    );
    assert_eq!(resumed.manifest.target_path, applied.manifest.target_path);
    let repeated_apply = service.apply("parent-demo").unwrap();
    assert_eq!(
        repeated_apply.manifest.outcome,
        wiki_model::domain::governance::ArchiveOutcome::AlreadyCompleted
    );
}

#[test]
fn resume_refuses_unknown_parent_changes() {
    let repo = ready_child_repo();
    let service = ArchiveService::new(repo.path());
    let applied = service.apply("parent-demo").unwrap();
    let result_path = repo
        .path()
        .join(".spec/.runtime/archive-operations")
        .join(&applied.manifest.operation_id)
        .join("result.json");
    fs::remove_file(result_path).unwrap();
    fs::write(
        repo.path().join(".spec/changes/parent/meta.yaml"),
        "externally changed\n",
    )
    .unwrap();
    assert!(service
        .resume("parent-demo", &applied.manifest.operation_id)
        .is_err());
}

#[test]
fn resume_requires_the_persisted_parent_staging_files() {
    let repo = ready_child_repo();
    let service = ArchiveService::new(repo.path());
    let applied = service.apply("parent-demo").unwrap();
    let operation_root = repo
        .path()
        .join(".spec/.runtime/archive-operations")
        .join(&applied.manifest.operation_id);
    fs::remove_file(operation_root.join("result.json")).unwrap();
    fs::remove_file(operation_root.join("staging/parent-meta.after")).unwrap();

    let error = service
        .resume("parent-demo", &applied.manifest.operation_id)
        .unwrap_err();
    assert_eq!(
        wiki_runtime::storage::archive_fs::archive_failure_kind(&error),
        Some(wiki_model::domain::governance::ArchiveErrorKind::ManifestInvalid)
    );
    assert!(error.to_string().contains("manifest staging"));
}

#[test]
fn completed_operation_requires_a_valid_result_and_unchanged_parent() {
    let repo = ready_child_repo();
    let service = ArchiveService::new(repo.path());
    let applied = service.apply("parent-demo").unwrap();
    let result_path = repo
        .path()
        .join(".spec/.runtime/archive-operations")
        .join(&applied.manifest.operation_id)
        .join("result.json");
    fs::write(&result_path, "not-json").unwrap();
    let error = service.apply("parent-demo").unwrap_err();
    assert_eq!(
        wiki_runtime::storage::archive_fs::archive_failure_kind(&error),
        Some(wiki_model::domain::governance::ArchiveErrorKind::ManifestInvalid)
    );

    fs::write(
        &result_path,
        serde_json::to_vec_pretty(&applied.manifest).unwrap(),
    )
    .unwrap();
    fs::write(
        repo.path().join(".spec/changes/parent/split.md"),
        "externally changed\n",
    )
    .unwrap();
    let error = service.apply("parent-demo").unwrap_err();
    assert_eq!(
        wiki_runtime::storage::archive_fs::archive_failure_kind(&error),
        Some(wiki_model::domain::governance::ArchiveErrorKind::Conflict)
    );
}

#[test]
fn resume_rejects_corrupt_staging_before_parent_write() {
    let repo = ready_child_repo();
    let service = ArchiveService::new(repo.path());
    let applied = service.apply("parent-demo").unwrap();
    let operation_root = repo
        .path()
        .join(".spec/.runtime/archive-operations")
        .join(&applied.manifest.operation_id);
    fs::remove_file(operation_root.join("result.json")).unwrap();
    fs::write(
        operation_root.join("staging/parent-meta.after"),
        "corrupt staging\n",
    )
    .unwrap();
    let parent_meta_path = repo.path().join(".spec/changes/parent/meta.yaml");
    let before = fs::read(&parent_meta_path).unwrap();

    let error = service
        .resume("parent-demo", &applied.manifest.operation_id)
        .unwrap_err();
    assert_eq!(
        wiki_runtime::storage::archive_fs::archive_failure_kind(&error),
        Some(wiki_model::domain::governance::ArchiveErrorKind::ManifestInvalid)
    );
    assert_eq!(fs::read(parent_meta_path).unwrap(), before);
}
