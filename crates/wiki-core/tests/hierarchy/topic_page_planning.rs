//! 专题页规划测试。
//! 覆盖根级主题、模块能力主题以及 topic page identity 稳定性。

use std::fs;

use tempfile::TempDir;
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;

fn make_topic_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"topic-demo","private":true,"workspaces":["packages/*"]}"#,
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
        "export function middleware() { return handleRoot(); }\n",
    )
    .unwrap();

    fs::create_dir_all(dir.path().join("packages/app/src")).unwrap();
    fs::write(
        dir.path().join("packages/app/package.json"),
        r#"{"name":"app","version":"1.0.0"}"#,
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
    fs::write(
        dir.path().join("packages/app/src/index.ts"),
        "import { handleA } from \"./handler_a\";\nexport function run() { return handleA(); }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("packages/app/src/service.ts"),
        "export function service() { return true; }\n",
    )
    .unwrap();

    dir
}

fn plan_topic_pages(repo: &TempDir) -> Vec<wiki_core::generation::planner::PlannedPage> {
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    plan_pages(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    )
}

#[test]
fn planner_generates_root_and_module_topic_pages() {
    let repo = make_topic_repo();
    let pages = plan_topic_pages(&repo);

    let root_topic = pages
        .iter()
        .find(|page| {
            page.page_type == "topic" && page.topic_kind.as_deref() == Some("root-mechanism")
        })
        .expect("root topic page should exist");
    let module_topic = pages
        .iter()
        .find(|page| {
            page.page_type == "topic" && page.topic_kind.as_deref() == Some("module-capability")
        })
        .expect("module capability topic page should exist");
    let archetype_topic = pages
        .iter()
        .find(|page| {
            page.page_type == "topic"
                && page.topic_kind.as_deref() == Some("repo-archetype")
                && page.topic_key.as_deref() == Some("request-lifecycle")
        })
        .expect("request lifecycle topic page should exist");
    let architecture = pages
        .iter()
        .find(|page| page.page_type == "architecture")
        .unwrap();
    let app_module_page = pages
        .iter()
        .find(|page| {
            page.page_type == "module" && page.relative_path.ends_with("核心模块/packages/app.md")
        })
        .unwrap();

    assert_eq!(
        root_topic.parent_id.as_deref(),
        Some(architecture.id.as_str())
    );
    assert_eq!(
        module_topic.parent_id.as_deref(),
        Some(app_module_page.id.as_str())
    );
    assert_eq!(
        archetype_topic.parent_id.as_deref(),
        Some(architecture.id.as_str())
    );
    assert!(!root_topic.source_ids.is_empty());
    assert!(!module_topic.source_ids.is_empty());
    assert!(archetype_topic
        .topic_summary
        .as_deref()
        .is_some_and(|summary| summary.contains("请求链")));
}

#[test]
fn topic_page_identity_is_stable_across_runs() {
    let repo_a = make_topic_repo();
    let repo_b = make_topic_repo();

    let pages_a = plan_topic_pages(&repo_a);
    let pages_b = plan_topic_pages(&repo_b);

    let topics_a = pages_a
        .iter()
        .filter(|page| page.page_type == "topic")
        .map(|page| {
            (
                (page.topic_kind.clone(), page.topic_key.clone()),
                (&page.id, &page.relative_path),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let topics_b = pages_b
        .iter()
        .filter(|page| page.page_type == "topic")
        .map(|page| {
            (
                (page.topic_kind.clone(), page.topic_key.clone()),
                (&page.id, &page.relative_path),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    for (key, (id_a, path_a)) in topics_a {
        let (id_b, path_b) = topics_b.get(&key).expect("topic should exist in both runs");
        assert_eq!(id_a, *id_b);
        assert_eq!(path_a, *path_b);
    }
}
