//! Section 模板集成测试。
//! 验证 overview 页面包含技术栈 section、architecture 页面包含层级化模块结构、
//! workflow 页面在有 CI/CD 线索时生成。

use std::fs;
use tempfile::TempDir;

use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{
    build_module_contexts, build_page_context, build_repo_context,
};
use wiki_core::generation::planner::plan_pages;
use wiki_core::generation::renderer::render_page_bundle;
use wiki_core::generation::sections::section_titles_for_page_type;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;

fn make_repo_with_ci() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(dir.path().join("src/app.ts"), "export function app() {}\n").unwrap();
    fs::write(dir.path().join("src/utils.ts"), "export function u() {}\n").unwrap();
    fs::write(dir.path().join("src/types.ts"), "export type T = string;\n").unwrap();

    // CI/CD 线索
    fs::create_dir_all(dir.path().join(".github/workflows")).unwrap();
    fs::write(
        dir.path().join(".github/workflows/ci.yml"),
        "name: CI\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n",
    )
    .unwrap();

    // Dockerfile
    fs::write(dir.path().join("Dockerfile"), "FROM node:18\nCOPY . .\n").unwrap();

    // Makefile
    fs::write(dir.path().join("Makefile"), "build:\n\tnpm run build\n").unwrap();

    dir
}

fn make_repo_without_ci() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(dir.path().join("src/app.ts"), "export function app() {}\n").unwrap();
    fs::write(dir.path().join("src/utils.ts"), "export function u() {}\n").unwrap();
    fs::write(dir.path().join("src/types.ts"), "export type T = string;\n").unwrap();
    dir
}

#[test]
fn overview_page_contains_tech_stack_section() {
    let repo = make_repo_with_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let overview = pages.iter().find(|p| p.page_type == "overview").unwrap();
    let titles = section_titles_for_page_type("overview");

    assert!(
        titles.contains(&"技术栈"),
        "overview should have 技术栈 section"
    );
    assert!(
        titles.contains(&"入口与构建"),
        "overview should have 入口与构建 section"
    );

    // Render and check content
    let ctx = build_page_context(overview, &report, &tree, &repo_ctx, &mod_ctxs);
    let rendered = render_page_bundle(overview, &ctx);
    assert!(
        rendered.content.contains("技术栈"),
        "rendered overview should contain 技术栈 section"
    );
}

#[test]
fn architecture_page_contains_hierarchical_module_structure() {
    let repo = make_repo_with_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let arch = pages
        .iter()
        .find(|p| p.page_type == "architecture")
        .unwrap();
    let titles = section_titles_for_page_type("architecture");

    assert!(
        titles.contains(&"架构提示"),
        "architecture should have 架构提示 section"
    );
    assert!(
        titles.contains(&"跨模块关系"),
        "architecture should have 跨模块关系 section"
    );

    let ctx = build_page_context(arch, &report, &tree, &repo_ctx, &mod_ctxs);
    let rendered = render_page_bundle(arch, &ctx);
    assert!(
        rendered.content.contains("模块结构"),
        "rendered architecture should contain 模块结构 section"
    );
}

#[test]
fn workflow_page_generated_when_ci_exists() {
    let repo = make_repo_with_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let workflow = pages.iter().find(|p| p.page_type == "workflow");
    assert!(
        workflow.is_some(),
        "workflow page should be generated when CI/CD files exist"
    );

    let workflow = workflow.unwrap();
    let ctx = build_page_context(workflow, &report, &tree, &repo_ctx, &mod_ctxs);
    let rendered = render_page_bundle(workflow, &ctx);

    assert!(
        rendered.content.contains("工作流概述"),
        "workflow page should have 工作流概述 section"
    );
    assert!(
        rendered.content.contains("构建流程"),
        "workflow page should have 构建流程 section"
    );
    assert!(
        rendered.content.contains("CI/CD 配置"),
        "workflow page should have CI/CD 配置 section"
    );
    assert!(
        rendered.content.contains("容器化"),
        "workflow page should have 容器化 section"
    );
}

#[test]
fn workflow_page_not_generated_without_ci() {
    let repo = make_repo_without_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let workflow = pages.iter().find(|p| p.page_type == "workflow");
    assert!(
        workflow.is_none(),
        "workflow page should NOT be generated without CI/CD files"
    );
}

#[test]
fn module_page_has_expanded_sections() {
    let repo = make_repo_with_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let module_page = pages.iter().find(|p| p.page_type == "module");
    if let Some(mp) = module_page {
        let titles = section_titles_for_page_type("module");
        assert!(
            titles.contains(&"关键源码"),
            "module should have 关键源码 section"
        );
        assert!(
            titles.contains(&"依赖关系"),
            "module should have 依赖关系 section"
        );
        assert!(
            titles.contains(&"子模块概述"),
            "module should have 子模块概述 section"
        );

        let ctx = build_page_context(mp, &report, &tree, &repo_ctx, &mod_ctxs);
        let rendered = render_page_bundle(mp, &ctx);
        assert!(
            rendered.content.contains("模块说明"),
            "module page should contain 模块说明"
        );
        assert!(
            rendered.content.contains("模块事实"),
            "module page should contain 模块事实"
        );
    }
}
