//! SQLite 存储层集成测试。
//! 验证完整 init → update → sync → rebuild 生命周期中 DB 的创建、读写和清理行为。

use std::fs;
use std::path::Path;

use wiki_core::storage::sqlite_store;
use wiki_core::workflows::init::run_init;
use wiki_core::workflows::rebuild::run_rebuild;
use wiki_core::workflows::sync::run_sync;
use wiki_core::workflows::update::run_update;

fn make_fixture_repo() -> tempfile::TempDir {
    let dir = tempfile::TempDir::new().unwrap();
    // 最小 Node 仓库结构
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(dir.path().join("src/utils.ts"), "export function f() {}\n").unwrap();
    fs::write(dir.path().join("src/helper.ts"), "export function h() {}\n").unwrap();
    fs::write(dir.path().join("src/types.ts"), "export type T = string;\n").unwrap();
    dir
}

fn db_exists(repo_root: &Path) -> bool {
    sqlite_store::db_exists(repo_root)
}

#[test]
fn init_creates_db_and_state() {
    let repo = make_fixture_repo();
    assert!(!db_exists(repo.path()));

    let report = run_init(repo.path()).unwrap();
    assert!(report.initialized);
    assert_eq!(report.state, "fresh");

    // DB 应该已创建
    assert!(db_exists(repo.path()));

    // wiki-state 应该可读
    let state = wiki_core::storage::state_store::read_state(repo.path()).unwrap();
    assert!(!state.pages.is_empty());

    // scan cache 和 module tree cache 应该可读
    let scan = wiki_core::storage::cache_store::read_scan_cache(repo.path());
    assert!(scan.is_ok());
    let tree = wiki_core::storage::cache_store::read_module_tree_cache(repo.path());
    assert!(tree.is_ok());
}

#[test]
fn update_after_init_preserves_db() {
    let repo = make_fixture_repo();
    run_init(repo.path()).unwrap();

    // update 不应破坏 DB
    let update_report = run_update(repo.path()).unwrap();
    assert_eq!(update_report.state, "fresh");
    assert!(db_exists(repo.path()));

    let state = wiki_core::storage::state_store::read_state(repo.path()).unwrap();
    assert!(!state.pages.is_empty());
}

#[test]
fn sync_after_init_preserves_db() {
    let repo = make_fixture_repo();
    run_init(repo.path()).unwrap();

    let sync_report = run_sync(repo.path()).unwrap();
    assert_eq!(sync_report.state, "fresh");
    assert!(db_exists(repo.path()));
}

#[test]
fn rebuild_recreates_db() {
    let repo = make_fixture_repo();
    run_init(repo.path()).unwrap();

    let state_before = wiki_core::storage::state_store::read_state(repo.path()).unwrap();
    let page_count_before = state_before.pages.len();

    let rebuild_report = run_rebuild(repo.path()).unwrap();
    assert_eq!(rebuild_report.state, "fresh");
    assert!(db_exists(repo.path()));

    let state_after = wiki_core::storage::state_store::read_state(repo.path()).unwrap();
    // rebuild 后页面数量应该一致（同一仓库结构）
    assert_eq!(state_after.pages.len(), page_count_before);
}

#[test]
fn full_lifecycle_init_update_sync_rebuild() {
    let repo = make_fixture_repo();

    // 1. init
    let init_report = run_init(repo.path()).unwrap();
    assert!(init_report.initialized);
    assert!(db_exists(repo.path()));

    // 2. 修改一个文件触发 stale
    fs::write(
        repo.path().join("src/index.ts"),
        "export const x = 2; // changed\n",
    )
    .unwrap();

    // 3. update
    let update_report = run_update(repo.path()).unwrap();
    assert_eq!(update_report.state, "fresh");
    assert!(db_exists(repo.path()));

    // 4. sync
    let sync_report = run_sync(repo.path()).unwrap();
    assert_eq!(sync_report.state, "fresh");

    // 5. rebuild
    let rebuild_report = run_rebuild(repo.path()).unwrap();
    assert_eq!(rebuild_report.state, "fresh");
    assert!(db_exists(repo.path()));

    // 最终状态应该完整
    let final_state = wiki_core::storage::state_store::read_state(repo.path()).unwrap();
    assert!(!final_state.pages.is_empty());
    assert!(final_state.pages.iter().any(|p| p.page_type == "overview"));
}
