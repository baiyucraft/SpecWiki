use std::fs;

use tempfile::TempDir;
use wiki_model::domain::governance::{
    ArchiveCheckpoint, ArchiveMode, ArchiveOperationManifest, ArchiveOperationStatus,
    ArchiveOutcome, ArchiveStep, ArchiveStepStatus, GovernanceReadiness,
    GovernanceValidationResult,
};
use wiki_runtime::storage::archive_fs::{patch_parent_meta, patch_parent_split, ArchiveFs};

#[test]
fn canonical_snapshot_and_digest_ignore_creation_order_but_bind_content() {
    let left = TempDir::new().unwrap();
    let right = TempDir::new().unwrap();
    for root in [left.path(), right.path()] {
        fs::create_dir_all(root.join(".spec/changes/demo/sub")).unwrap();
    }
    fs::write(left.path().join(".spec/changes/demo/b.md"), "b").unwrap();
    fs::write(left.path().join(".spec/changes/demo/sub/a.md"), "a").unwrap();
    fs::write(right.path().join(".spec/changes/demo/sub/a.md"), "a").unwrap();
    fs::write(right.path().join(".spec/changes/demo/b.md"), "b").unwrap();

    let left_snapshot = ArchiveFs::new(left.path())
        .snapshot_tree(".spec/changes/demo")
        .unwrap();
    let right_snapshot = ArchiveFs::new(right.path())
        .snapshot_tree(".spec/changes/demo")
        .unwrap();
    assert_eq!(left_snapshot, right_snapshot);
    let left_digest = ArchiveFs::new(left.path())
        .compute_digest(&left_snapshot)
        .unwrap();
    let right_digest = ArchiveFs::new(right.path())
        .compute_digest(&right_snapshot)
        .unwrap();
    assert_eq!(left_digest, right_digest);

    fs::write(right.path().join(".spec/changes/demo/b.md"), "changed").unwrap();
    let changed = ArchiveFs::new(right.path())
        .snapshot_tree(".spec/changes/demo")
        .unwrap();
    assert_ne!(
        left_digest,
        ArchiveFs::new(right.path())
            .compute_digest(&changed)
            .unwrap()
    );
}

#[test]
fn parent_patches_preserve_unknown_yaml_and_require_unique_split_marker() {
    let meta = "id: parent\nunknown:\n  keep: true\nmultiChange:\n  role: parent\n  children:\n    - id: parent-demo\n      order: 1\n";
    let patched = patch_parent_meta(
        meta,
        "parent-demo",
        "2026-07-13T00:00:00Z",
        ".spec/archive/2026-07-13-parent-demo",
    )
    .unwrap();
    let parsed: serde_yaml::Value = serde_yaml::from_str(&patched).unwrap();
    assert_eq!(parsed["unknown"]["keep"], true);
    assert_eq!(
        parsed["multiChange"]["children"][0]["archiveStatus"],
        "archived"
    );

    let split = "### 1. parent-demo\n- 归档状态：[ ] pending\n";
    assert!(patch_parent_split(split, "parent-demo")
        .unwrap()
        .contains("[x] archived"));
    assert!(patch_parent_split("### 1. parent-demo\n", "parent-demo").is_err());
    assert!(patch_parent_split(&format!("{split}{split}"), "parent-demo").is_err());
}

fn plan_fixture(operation_id: &str) -> ArchiveOperationManifest {
    ArchiveOperationManifest {
        schema_version: "archive-operation-v1".to_string(),
        policy_version: "unispec-0.1.0".to_string(),
        algorithm_version: "archive-precondition-v1".to_string(),
        operation_id: operation_id.to_string(),
        change_id: "parent-demo".to_string(),
        mode: ArchiveMode::Apply,
        outcome: ArchiveOutcome::Ready,
        status: ArchiveOperationStatus::Applying,
        step: ArchiveStep::Prepared,
        step_status: ArchiveStepStatus::Pending,
        source_path: ".spec/changes/parent-demo".to_string(),
        target_path: ".spec/archive/2026-07-13-parent-demo".to_string(),
        operation_root: Some(format!(".spec/.runtime/archive-operations/{operation_id}")),
        created_at: "2026-07-13T00:00:00Z".to_string(),
        persisted: true,
        resumable: true,
        validation: GovernanceValidationResult {
            valid: true,
            readiness: GovernanceReadiness::Ready,
            rule_results: Vec::new(),
            issues: Vec::new(),
        },
        artifact_hash_summary: Vec::new(),
        parent_diff: None,
        precondition_digest: "digest".to_string(),
        completed_steps: Vec::new(),
        failure_step: None,
        recovery_hint: None,
        wiki_sync_issues: Vec::new(),
        evidence_refs: Vec::new(),
    }
}

#[test]
fn operation_storage_is_immutable_append_only_and_discoverable() {
    let repo = TempDir::new().unwrap();
    let fs_adapter = ArchiveFs::new(repo.path());
    let plan = plan_fixture("op-1");
    fs_adapter.persist_plan(&plan, None).unwrap();
    assert!(fs_adapter.persist_plan(&plan, None).is_err());
    fs_adapter
        .append_checkpoint(
            "op-1",
            &ArchiveCheckpoint {
                sequence: 1,
                attempt: 1,
                step: ArchiveStep::Prepared,
                status: ArchiveStepStatus::Completed,
                created_at: plan.created_at.clone(),
                message: None,
            },
        )
        .unwrap();
    assert_eq!(
        fs_adapter.discover_incomplete("parent-demo").unwrap(),
        vec!["op-1"]
    );
    fs_adapter.write_result(&plan).unwrap();
    assert!(fs_adapter
        .discover_incomplete("parent-demo")
        .unwrap()
        .is_empty());
}

#[test]
fn mutation_set_lock_blocks_sibling_operations() {
    let repo = TempDir::new().unwrap();
    let fs_adapter = ArchiveFs::new(repo.path());
    let first = fs_adapter
        .acquire_lock("parent-one", Some("parent"))
        .unwrap();
    let second = fs_adapter.acquire_lock("parent-two", Some("parent"));
    assert!(second.is_err());
    drop(first);
    assert!(fs_adapter
        .acquire_lock("parent-two", Some("parent"))
        .is_ok());
}

#[test]
fn invalid_manifest_identity_and_checkpoint_sequence_fail_closed() {
    let repo = TempDir::new().unwrap();
    let fs_adapter = ArchiveFs::new(repo.path());
    let plan = plan_fixture("op-bad");
    fs_adapter.persist_plan(&plan, None).unwrap();
    let plan_path = fs_adapter.operation_root("op-bad").join("plan.json");
    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&plan_path).unwrap()).unwrap();
    value["source_path"] = serde_json::json!("../escape");
    fs::write(&plan_path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    assert!(fs_adapter.read_plan("op-bad").is_err());

    let repo = TempDir::new().unwrap();
    let fs_adapter = ArchiveFs::new(repo.path());
    let plan = plan_fixture("op-order");
    fs_adapter.persist_plan(&plan, None).unwrap();
    for (name, sequence) in [("a.json", 1_u64), ("b.json", 1_u64)] {
        let checkpoint = ArchiveCheckpoint {
            sequence,
            attempt: 1,
            step: ArchiveStep::Prepared,
            status: ArchiveStepStatus::Completed,
            created_at: plan.created_at.clone(),
            message: None,
        };
        fs::write(
            fs_adapter
                .operation_root("op-order")
                .join("checkpoints")
                .join(name),
            serde_json::to_vec(&checkpoint).unwrap(),
        )
        .unwrap();
    }
    assert!(fs_adapter.read_checkpoints("op-order").is_err());
}

#[test]
fn discovery_fails_closed_when_any_operation_manifest_is_invalid() {
    let repo = TempDir::new().unwrap();
    let fs_adapter = ArchiveFs::new(repo.path());
    let plan = plan_fixture("op-corrupt");
    fs_adapter.persist_plan(&plan, None).unwrap();
    fs::write(
        fs_adapter.operation_root("op-corrupt").join("plan.json"),
        "not-json",
    )
    .unwrap();

    let error = fs_adapter.discover_incomplete("parent-demo").unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("manifest"));
}

#[cfg(unix)]
#[test]
fn snapshot_rejects_symlink_entries() {
    use std::os::unix::fs::symlink;
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".spec/changes/demo")).unwrap();
    fs::write(repo.path().join("outside"), "outside").unwrap();
    symlink(
        repo.path().join("outside"),
        repo.path().join(".spec/changes/demo/link"),
    )
    .unwrap();
    assert!(ArchiveFs::new(repo.path())
        .snapshot_tree(".spec/changes/demo")
        .is_err());
}
