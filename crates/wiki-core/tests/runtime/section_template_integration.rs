//! Section 模板集成测试。
//! 验证 overview 页面包含技术栈 section、architecture 页面包含层级化模块结构、
//! workflow 页面在有 CI/CD 线索时生成。

use std::fs;
use tempfile::TempDir;

use wiki_core::domain::context::{PageResearchResult, PageResearchSectionPlan};
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{
    build_module_contexts, build_page_context, build_repo_context,
};
use wiki_core::generation::planner::plan_pages;
use wiki_core::generation::renderer::render_page_bundle;
use wiki_core::generation::sections::section_titles_for_page_type;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::workflows::init::run_init;

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

fn make_repo_with_process_only() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/shared.ts"),
        "export function finalizePayment() { return true; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/service.ts"),
        concat!(
            "import { finalizePayment } from \"./shared\";\n",
            "export function runPayment() {\n",
            "  return finalizePayment();\n",
            "}\n",
        ),
    )
    .unwrap();
    fs::write(
        dir.path().join("src/controller.ts"),
        concat!(
            "import { runPayment } from \"./service\";\n",
            "export function handleCheckout() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    )
    .unwrap();
    dir
}

fn make_repo_with_topics_and_edges() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"topic-edge-demo","private":true,"workspaces":["packages/*"]}"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("router.ts"),
        "export function router() { return true; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("handler.ts"),
        "export function handleRoot() { return router(); }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("middleware.ts"),
        "import { handleRoot } from \"./handler\";\nexport function middleware() { return handleRoot(); }\n",
    )
    .unwrap();

    fs::create_dir_all(dir.path().join("packages/app/src")).unwrap();
    fs::create_dir_all(dir.path().join("packages/shared/src")).unwrap();
    fs::write(
        dir.path().join("packages/app/package.json"),
        r#"{"name":"app","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/shared/package.json"),
        r#"{"name":"shared","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/shared/src/util.ts"),
        "export function util() { return true; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/app/src/index.ts"),
        "import { util } from \"../../shared/src/util\";\nexport function run() { return util(); }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/app/src/handler_a.ts"),
        "export function handleA() { return true; }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/app/src/handler_b.ts"),
        "export function handleB() { return true; }\n",
    )
    .unwrap();

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
    assert!(
        !rendered
            .content
            .contains("由 codebuddy-wiki 自动生成的仓库概览。"),
        "overview fallback should be narrative instead of boilerplate"
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
fn workflow_page_generated_from_detected_process_without_ci() {
    let repo = make_repo_with_process_only();
    run_init(repo.path()).unwrap();

    let workflow_path = repo.path().join(".wiki/工作流与部署.md");
    assert!(
        workflow_path.exists(),
        "expected workflow page to be generated"
    );

    let workflow = fs::read_to_string(workflow_path).unwrap();
    assert!(
        workflow.contains("handleCheckout flow"),
        "expected graph-derived process on workflow page, got:\n{workflow}"
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
        assert!(
            !rendered
                .content
                .contains("该页面围绕单个模块整理其边界、入口和依赖。"),
            "module fallback should no longer use the old boilerplate"
        );
    }
}

#[test]
fn architecture_page_renders_evidence_block_and_mermaid() {
    let repo = make_repo_with_topics_and_edges();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let arch = pages
        .iter()
        .find(|page| page.page_type == "architecture")
        .unwrap();
    let ctx = build_page_context(arch, &report, &tree, &repo_ctx, &mod_ctxs);
    let rendered = render_page_bundle(arch, &ctx);

    assert!(rendered.content.contains("**架构关键来源**"));
    assert!(rendered.content.contains("```mermaid"));
    assert!(rendered.content.contains("graph LR"));
}

#[test]
fn topic_page_renders_stable_evidence_block() {
    let repo = make_repo_with_topics_and_edges();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let topic_page = pages
        .iter()
        .find(|page| {
            page.page_type == "topic" && page.topic_kind.as_deref() == Some("root-mechanism")
        })
        .unwrap();
    let ctx = build_page_context(topic_page, &report, &tree, &repo_ctx, &mod_ctxs);
    let rendered = render_page_bundle(topic_page, &ctx);

    assert!(rendered.content.contains("关键证据"));
    assert!(rendered.content.contains("**关键来源**"));
    assert!(rendered.content.contains("router.ts"));
}

#[test]
fn module_page_prefers_section_plan_order_and_summary() {
    let repo = make_repo_with_ci();
    let steering = SteeringConfig::default();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let module_page = pages
        .iter()
        .find(|page| page.page_type == "module")
        .expect("module page should exist");
    let mut ctx = build_page_context(module_page, &report, &tree, &repo_ctx, &mod_ctxs);
    ctx.research_result = Some(PageResearchResult {
        summary: "research summary".to_string(),
        section_plan: vec![
            PageResearchSectionPlan {
                section_key: "dependencies".to_string(),
                section_title: "依赖关系".to_string(),
                section_summary: "先解释依赖链。".to_string(),
                evidence_refs: Vec::new(),
                diagram_refs: Vec::new(),
                child_refs: Vec::new(),
            },
            PageResearchSectionPlan {
                section_key: "module-intro".to_string(),
                section_title: "模块说明".to_string(),
                section_summary: "再解释模块职责。".to_string(),
                evidence_refs: Vec::new(),
                diagram_refs: Vec::new(),
                child_refs: Vec::new(),
            },
        ],
        evidence_rollup: Vec::new(),
        diagram_rollup: Vec::new(),
        open_questions: Vec::new(),
    });

    let rendered = render_page_bundle(module_page, &ctx);
    let dependency_pos = rendered
        .content
        .find("## 依赖关系")
        .expect("dependency section should exist");
    let intro_pos = rendered
        .content
        .find("## 模块说明")
        .expect("module intro section should exist");

    assert!(dependency_pos < intro_pos);
    assert!(rendered.content.contains("先解释依赖链。"));
    assert!(rendered.content.contains("再解释模块职责。"));
}
