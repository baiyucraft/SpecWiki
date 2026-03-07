use std::fs;

use tempfile::tempdir;
use wiki_core::app::{init::run_init, status::run_status, update::run_update};

#[test]
fn update_only_marks_changed_pages_dirty() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 1;").unwrap();

    run_init(repo_root).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 2;").unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "stale");

    let update = run_update(repo_root).unwrap();
    assert!(!update.updated_pages.is_empty());
}
