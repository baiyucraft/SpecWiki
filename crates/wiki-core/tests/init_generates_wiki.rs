use std::fs;

use tempfile::tempdir;
use wiki_core::storage::metadata_store::read_metadata;
use wiki_core::workflows::init::run_init;

#[test]
fn init_writes_wiki_layout() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();

    assert!(repo_root.join(".wiki/项目概述.md").exists());
    assert!(repo_root.join(".wiki/系统架构.md").exists());
    assert!(repo_root.join(".wiki/wiki.metadata.json").exists());
    assert!(repo_root.join(".wiki/.cache").exists());
    assert!(repo_root.join(".wiki/.cache/repo-scan.json").exists());
    assert!(repo_root.join(".wiki/.cache/module-tree.json").exists());

    let metadata = read_metadata(repo_root).unwrap();
    assert!(!metadata.modules.is_empty());
    assert!(metadata
        .wiki_items
        .iter()
        .any(|item| item.path.ends_with("系统架构.md")));
    assert!(metadata
        .wiki_items
        .iter()
        .any(|item| item.item_type == "module"));
}
