use std::fs;

use tempfile::tempdir;
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::storage::runtime_commit::{
    execute_runtime_commit, has_incomplete_runtime_commit, recover_runtime_commits,
    RuntimeCommitPhase, RuntimeCommitPlan, RuntimeCommitWrite,
};
use wiki_runtime::workflows::{init::run_init, sync::run_sync};

use super::test_support::force_full_runtime;

#[test]
fn runtime_commit_rejects_operation_ids_that_escape_the_journal_root() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path().join("repo");
    fs::create_dir_all(repo_root.join(".wiki")).unwrap();
    let writes = || {
        vec![RuntimeCommitWrite {
            relative_path: ".wiki/wiki.metadata.json".to_string(),
            content: br#"{"current_snapshot_id":"after"}"#.to_vec(),
            commit_pointer: true,
        }]
    };
    let absolute = fixture.path().join("absolute-operation");
    let invalid_ids = [
        "../outside-operation".to_string(),
        "nested/operation".to_string(),
        r"nested\operation".to_string(),
        ".".to_string(),
        ".lock".to_string(),
        absolute.to_string_lossy().into_owned(),
    ];

    for operation_id in invalid_ids {
        let capture = RuntimeCommitPlan::capture(
            &repo_root,
            operation_id.clone(),
            None,
            "after",
            writes(),
            Vec::new(),
        );
        assert_eq!(
            capture.unwrap_err().kind(),
            std::io::ErrorKind::InvalidInput,
            "capture accepted {operation_id:?}"
        );

        let mut plan = RuntimeCommitPlan::capture(
            &repo_root,
            "valid-operation",
            None,
            "after",
            writes(),
            Vec::new(),
        )
        .unwrap();
        plan.operation_id = operation_id.clone();
        let execute = execute_runtime_commit(&repo_root, &plan, None);
        assert_eq!(
            execute.unwrap_err().kind(),
            std::io::ErrorKind::InvalidInput,
            "execute accepted {operation_id:?}"
        );
    }

    assert!(!fixture.path().join("outside-operation").exists());
    assert!(!absolute.exists());
    assert!(!repo_root.join(".wiki/.cache/runtime-commits").exists());
}

#[test]
fn runtime_commit_recovers_idempotently_on_both_sides_of_commit_pointer() {
    for (index, phase) in RuntimeCommitPhase::ALL.into_iter().enumerate() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::create_dir_all(repo_root.join(".wiki")).unwrap();
        fs::write(repo_root.join(".wiki/page.md"), "before\n").unwrap();
        fs::write(
            repo_root.join(".wiki/wiki.metadata.json"),
            r#"{"current_snapshot_id":"before"}"#,
        )
        .unwrap();
        let plan = RuntimeCommitPlan::capture(
            repo_root,
            format!("phase-{index}"),
            Some("before".to_string()),
            "after",
            vec![
                RuntimeCommitWrite {
                    relative_path: ".wiki/page.md".to_string(),
                    content: b"after\n".to_vec(),
                    commit_pointer: false,
                },
                RuntimeCommitWrite {
                    relative_path: ".wiki/wiki.metadata.json".to_string(),
                    content: br#"{"current_snapshot_id":"after"}"#.to_vec(),
                    commit_pointer: true,
                },
            ],
            Vec::new(),
        )
        .unwrap();

        let interrupted = execute_runtime_commit(repo_root, &plan, Some(phase));
        assert_eq!(
            interrupted.unwrap_err().kind(),
            std::io::ErrorKind::Interrupted
        );
        recover_runtime_commits(repo_root)
            .unwrap_or_else(|error| panic!("recover phase {phase:?}: {error}"));
        recover_runtime_commits(repo_root)
            .unwrap_or_else(|error| panic!("repeat recover phase {phase:?}: {error}"));
        let roll_forward = phase >= RuntimeCommitPhase::PointerCommitted;
        assert_eq!(
            fs::read_to_string(repo_root.join(".wiki/page.md")).unwrap(),
            if roll_forward { "after\n" } else { "before\n" }
        );
        assert_eq!(
            fs::read_to_string(repo_root.join(".wiki/wiki.metadata.json")).unwrap(),
            if roll_forward {
                r#"{"current_snapshot_id":"after"}"#
            } else {
                r#"{"current_snapshot_id":"before"}"#
            }
        );
        assert!(!has_incomplete_runtime_commit(repo_root));
    }

    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("main.rs"), "fn main() {}\n").unwrap();

    run_init(repo_root).unwrap();
    let before = read_metadata(repo_root)
        .unwrap()
        .current_snapshot_id
        .unwrap();
    let overview = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview).unwrap();
    let content = content.replacen("owner=derived_managed", "owner=declared_managed", 1);
    let end = content.find("<!-- wiki:managed:end").unwrap();
    let declared = concat!(
        "\n<!-- wiki:declared id=runtime-commit-policy kind=policy scope=repo status=active source=manual -->\n",
        "formal changes participate in snapshot identity.\n",
        "<!-- wiki:declared:end -->\n"
    );
    let content = format!("{}{declared}{}", &content[..end], &content[end..]);
    fs::write(&overview, content).unwrap();
    run_sync(repo_root).unwrap();

    let after = read_metadata(repo_root)
        .unwrap()
        .current_snapshot_id
        .unwrap();
    assert_ne!(
        before, after,
        "declared-only commit must change snapshot id"
    );
}
