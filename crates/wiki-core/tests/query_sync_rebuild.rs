use std::fs;

use tempfile::tempdir;
use wiki_core::app::{init::run_init, query::run_query, rebuild::run_rebuild, sync::run_sync};

#[test]
fn sync_detects_manual_markdown_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    fs::write(&overview, "# 项目概述\n\n自定义说明\n").unwrap();

    let result = run_sync(repo_root).unwrap();
    assert!(result
        .synced_pages
        .iter()
        .any(|path| path.ends_with("项目概述.md")));

    let query = run_query(repo_root, "项目概述").unwrap();
    assert!(!query.matched_pages.is_empty());

    let rebuild = run_rebuild(repo_root).unwrap();
    assert!(!rebuild.updated_pages.is_empty());
}
