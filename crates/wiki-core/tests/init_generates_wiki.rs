use std::fs;

use tempfile::tempdir;
use wiki_core::app::init::run_init;

#[test]
fn init_writes_wiki_layout() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();

    assert!(repo_root.join(".wiki/项目概述.md").exists());
    assert!(repo_root.join(".wiki/wiki.metadata.json").exists());
    assert!(repo_root.join(".wiki/.cache").exists());
    assert!(repo_root.join(".wiki/.cache/repo-scan.json").exists());
}
