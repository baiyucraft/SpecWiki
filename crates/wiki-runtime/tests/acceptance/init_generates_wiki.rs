use std::fs;

use tempfile::tempdir;
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::init::run_init;

#[test]
fn init_writes_wiki_layout() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::write(repo_root.join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(repo_root.join("src/app.ts"), "export function app() {}\n").unwrap();
    fs::write(repo_root.join("src/utils.ts"), "export function u() {}\n").unwrap();
    fs::write(repo_root.join("src/types.ts"), "export type T = string;\n").unwrap();

    run_init(repo_root).unwrap();

    assert!(repo_root.join(".wiki/项目概述.md").exists());
    assert!(repo_root.join(".wiki/系统架构.md").exists());
    assert!(repo_root.join(".wiki/wiki.metadata.json").exists());
    assert!(repo_root.join(".wiki/.cache").exists());
    // scan cache 和关系型状态都应该落在 SQLite DB 中
    assert!(sqlite_store::db_exists(repo_root));
    {
        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::runtime_tables_exist(&conn).unwrap());
        assert!(sqlite_store::scan_cache_exists(&conn, "repo-scan").unwrap());
        assert!(sqlite_store::scan_cache_exists(&conn, "module-tree").unwrap());
    }
    assert!(read_state(repo_root).is_ok());

    let metadata = read_metadata(repo_root).unwrap();
    assert!(!metadata.modules.is_empty());
    assert!(metadata
        .wiki_items
        .iter()
        .any(|item| item.path.ends_with("系统架构.md")));
    assert!(metadata.wiki_items.len() >= 2);
}



