use std::fs;

use tempfile::tempdir;
use wiki_core::workflows::{init::run_init, status::run_status, update::run_update};

#[test]
fn status_reports_missing_before_init() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "missing");
}

#[test]
fn status_reports_needs_rebuild_when_cache_is_missing() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(status.needs_rebuild_reason.as_deref(), Some("cache_missing"));
}

#[test]
fn update_refreshes_stale_runtime_to_fresh() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 1;").unwrap();

    run_init(repo_root).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 2;").unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "stale");

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "stale");
    assert_eq!(update.state, "fresh");
    assert!(!update.updated_pages.is_empty());

    let refreshed_status = run_status(repo_root).unwrap();
    assert_eq!(refreshed_status.state, "fresh");
}
