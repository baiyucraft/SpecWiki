//! 页面拓扑集成测试。
//! 验证嵌套模块的父子关系按模块树层级分配、小模块被合并到父模块页面、
//! 合并后模块仍保留在 WikiState.modules 中。

use std::fs;
use tempfile::TempDir;

use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::workflows::init::run_init;

/// 构造一个有嵌套模块结构的仓库。
/// packages/
///   core/       (4 source files → 独立页面)
///   utils/      (1 source file → 被合并)
///   domain/
///     auth/     (5 source files → 独立页面)
///     config/   (1 source file → 被合并)
fn make_nested_repo() -> TempDir {
    let dir = TempDir::new().unwrap();

    // 根 manifest
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0","workspaces":["packages/*","packages/domain/*"]}"#,
    )
    .unwrap();

    // packages/core (4 files)
    let core = dir.path().join("packages/core/src");
    fs::create_dir_all(&core).unwrap();
    fs::write(
        dir.path().join("packages/core/package.json"),
        r#"{"name":"core"}"#,
    )
    .unwrap();
    for i in 0..4 {
        fs::write(
            core.join(format!("mod{i}.ts")),
            format!("export const m{i} = {i};\n"),
        )
        .unwrap();
    }

    // packages/utils (1 file → should be merged)
    let utils = dir.path().join("packages/utils/src");
    fs::create_dir_all(&utils).unwrap();
    fs::write(
        dir.path().join("packages/utils/package.json"),
        r#"{"name":"utils"}"#,
    )
    .unwrap();
    fs::write(utils.join("index.ts"), "export function u() {}\n").unwrap();

    // packages/domain/auth (5 files)
    let auth = dir.path().join("packages/domain/auth/src");
    fs::create_dir_all(&auth).unwrap();
    fs::write(
        dir.path().join("packages/domain/auth/package.json"),
        r#"{"name":"auth"}"#,
    )
    .unwrap();
    for i in 0..5 {
        fs::write(
            auth.join(format!("auth{i}.ts")),
            format!("export const a{i} = {i};\n"),
        )
        .unwrap();
    }

    // packages/domain/config (1 file → should be merged)
    let config = dir.path().join("packages/domain/config/src");
    fs::create_dir_all(&config).unwrap();
    fs::write(
        dir.path().join("packages/domain/config/package.json"),
        r#"{"name":"config"}"#,
    )
    .unwrap();
    fs::write(config.join("index.ts"), "export const c = 1;\n").unwrap();

    dir
}

#[test]
fn nested_modules_have_hierarchical_parent_child() {
    let repo = make_nested_repo();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &steering.scan.ignore).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    // 找到有独立页面的模块页
    let module_pages: Vec<_> = pages.iter().filter(|p| p.page_type == "module").collect();

    // 检查是否存在嵌套父子关系（非所有模块页都挂在 overview 下）
    let overview_page = pages.iter().find(|p| p.page_type == "overview").unwrap();
    let non_overview_parents: Vec<_> = module_pages
        .iter()
        .filter(|p| p.parent_id.as_deref() != Some(&overview_page.id))
        .collect();

    // 嵌套仓库应该有一些模块页的父页面不是 overview
    // （domain/auth 的父页面应该是 domain 或 packages）
    if module_pages.len() > 2 {
        assert!(
            !non_overview_parents.is_empty(),
            "nested modules should have hierarchical parent-child, not all flat under overview"
        );
    }
}

#[test]
fn small_modules_merged_to_parent_page() {
    let repo = make_nested_repo();
    // utils weight=4, config weight=4 → need threshold > 4 to trigger merge
    let mut steering = SteeringConfig::default();
    steering.merge_threshold = 5;
    let report = scan_repo(repo.path(), &steering.scan.ignore).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    // 检查是否有页面包含 merged_module_ids
    let pages_with_merged: Vec<_> = pages
        .iter()
        .filter(|p| !p.merged_module_ids.is_empty())
        .collect();

    // 小模块（utils, config）应该被合并
    assert!(
        !pages_with_merged.is_empty(),
        "some small modules should be merged into parent pages with threshold=5"
    );
}

#[test]
fn merged_modules_still_in_wiki_state() {
    let repo = make_nested_repo();
    run_init(repo.path()).unwrap();

    let state = wiki_core::storage::state_store::read_state(repo.path()).unwrap();

    // WikiState.modules 应该包含所有模块（包括被合并的）
    assert!(
        state.modules.len() >= 4,
        "WikiState should retain all modules including merged ones, got {}",
        state.modules.len()
    );
}

#[test]
fn overview_and_architecture_always_present() {
    let repo = make_nested_repo();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &steering.scan.ignore).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    assert!(pages.iter().any(|p| p.page_type == "overview"));
    assert!(pages.iter().any(|p| p.page_type == "architecture"));

    let arch = pages
        .iter()
        .find(|p| p.page_type == "architecture")
        .unwrap();
    let overview = pages.iter().find(|p| p.page_type == "overview").unwrap();
    assert_eq!(arch.parent_id.as_deref(), Some(overview.id.as_str()));
}
