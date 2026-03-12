//! 这组测试覆盖 status 与 update 的状态流转和增量更新边界。
//! 它们重点保护 stale 检测、局部更新、结构变化与 rebuild 回退行为。

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_core::llm::{LlmCompletion, LlmPromptRequest, LlmService};
use wiki_core::storage::metadata_store::read_metadata;
use wiki_core::storage::sqlite_store;
use wiki_core::storage::state_store::read_state;
use wiki_core::workflows::progress::NoopProgressSink;
use wiki_core::workflows::{
    init::{run_init, run_init_with_progress_and_llm_as},
    status::run_status,
    update::run_update,
};

struct StatusStabilityLlmService;

impl LlmService for StatusStabilityLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> std::io::Result<LlmCompletion> {
        let output = match request.prompt_type.as_str() {
            "file_purpose" => {
                let items = request
                    .input
                    .get("items")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                serde_json::json!({
                    "items": items.into_iter().filter_map(|item| {
                        let path = item.get("path")?.as_str()?;
                        let purpose = if path.ends_with("mux.ts") {
                            "router"
                        } else if path.ends_with("Makefile") || path.ends_with("wiki.dev.yaml") {
                            "config"
                        } else {
                            "utility"
                        };
                        Some(serde_json::json!({
                            "path": path,
                            "purpose": purpose,
                        }))
                    }).collect::<Vec<_>>()
                })
            }
            "page_enrichment" => serde_json::json!({
                "summary": "增强摘要",
                "section_overrides": {},
                "mermaid_blocks": {}
            }),
            _ => serde_json::json!({}),
        };

        Ok(LlmCompletion {
            output,
            model: Some("status-stability-model".to_string()),
            usage: None,
        })
    }
}

/// 场景：正式索引还没建立时，status 必须明确返回 `missing`。
#[test]
fn status_reports_missing_before_init() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "missing");
}

/// 场景：仓库级 cache 被删光后，status 必须升级为 `needs_rebuild`。
#[test]
fn status_reports_needs_rebuild_when_cache_is_missing() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(
        status.needs_rebuild_reason.as_deref(),
        Some("cache_missing")
    );
}

/// 场景：单页 cache 缺失时，update 必须回退到 rebuild 并补全 page cache。
#[test]
fn update_falls_back_to_rebuild_when_page_cache_is_missing() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    let overview_page = read_state(repo_root)
        .unwrap()
        .pages
        .into_iter()
        .find(|page| page.page_type == "overview")
        .unwrap();

    {
        let conn = sqlite_store::open_db(repo_root).unwrap();
        sqlite_store::remove_page_context(&conn, &overview_page.page_id).unwrap();
        sqlite_store::remove_page_generation(&conn, &overview_page.page_id).unwrap();
    }

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "needs_rebuild");
    assert_eq!(update.state, "fresh");
    {
        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::page_context_exists(&conn, &overview_page.page_id).unwrap());
        assert!(sqlite_store::page_generation_exists(&conn, &overview_page.page_id).unwrap());
    }

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "fresh");
}

/// 场景：普通源码变更后，update 必须把 runtime 从 `stale` 刷回 `fresh`。
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

/// 场景：单文件只影响单一模块页时，未命中的页面和 section hash 不应被误改。
#[test]
fn incremental_update_only_touches_related_pages() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_workspace_repo(repo_root);
    run_init(repo_root).unwrap();

    let shared_page_path = ".wiki/核心模块/packages/shared.md";
    let app_page_path = ".wiki/核心模块/packages/app.md";
    let app_section_hashes_before = read_state(repo_root)
        .unwrap()
        .pages
        .into_iter()
        .find(|page| page.path == app_page_path)
        .unwrap()
        .sections
        .into_iter()
        .map(|section| section.content_hash)
        .collect::<Vec<_>>();

    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 2;",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "stale");
    assert!(status
        .dirty_pages
        .iter()
        .any(|path| path == shared_page_path));
    assert!(!status.dirty_pages.iter().any(|path| path == app_page_path));

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "stale");
    assert_eq!(update.state, "fresh");
    assert!(update
        .updated_pages
        .iter()
        .any(|path| path == shared_page_path));
    assert!(!update
        .updated_pages
        .iter()
        .any(|path| path == app_page_path));

    let app_section_hashes_after = read_state(repo_root)
        .unwrap()
        .pages
        .into_iter()
        .find(|page| page.path == app_page_path)
        .unwrap()
        .sections
        .into_iter()
        .map(|section| section.content_hash)
        .collect::<Vec<_>>();
    assert_eq!(app_section_hashes_before, app_section_hashes_after);
}

/// 场景：新增或删除模块源码后，update 必须同步新增或删除对应页面与 metadata。
#[test]
fn update_adds_and_removes_pages_after_structural_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_single_package_workspace(repo_root);
    run_init(repo_root).unwrap();

    let shared_page_path = repo_root.join(".wiki/核心模块/packages/shared.md");

    write_file(
        repo_root.join("packages/shared/package.json").as_path(),
        r#"{"name":"shared","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 1;",
    );

    let add_status = run_status(repo_root).unwrap();
    assert_eq!(add_status.state, "stale");

    let add_update = run_update(repo_root).unwrap();
    assert_eq!(add_update.state, "fresh");
    assert!(shared_page_path.exists());
    assert!(add_update
        .updated_pages
        .iter()
        .any(|path| path.ends_with("核心模块/packages/shared.md")));
    assert!(read_metadata(repo_root)
        .unwrap()
        .wiki_items
        .iter()
        .any(|item| item.path.ends_with("核心模块/packages/shared.md")));

    fs::remove_dir_all(repo_root.join("packages/shared")).unwrap();

    let remove_status = run_status(repo_root).unwrap();
    assert_eq!(remove_status.state, "stale");

    let remove_update = run_update(repo_root).unwrap();
    assert_eq!(remove_update.state, "fresh");
    assert!(!shared_page_path.exists());
    assert!(remove_update
        .updated_pages
        .iter()
        .any(|path| path.ends_with("核心模块/packages/shared.md")));
    assert!(!read_metadata(repo_root)
        .unwrap()
        .wiki_items
        .iter()
        .any(|item| item.path.ends_with("核心模块/packages/shared.md")));
}

#[test]
fn update_rebuilds_topic_page_and_parent_pages_when_topic_sources_change() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"topic-update-demo"}"#,
    );
    write_file(
        repo_root.join("router.ts").as_path(),
        "export function router() { return true; }",
    );
    write_file(
        repo_root.join("handler.ts").as_path(),
        "export function handleRoot() { return router(); }",
    );
    write_file(
        repo_root.join("middleware.ts").as_path(),
        "export function middleware() { return handleRoot(); }",
    );

    run_init(repo_root).unwrap();

    let initial_state = read_state(repo_root).unwrap();
    let topic_page = initial_state
        .pages
        .iter()
        .find(|page| page.page_type == "topic")
        .expect("topic page should exist after init");
    let architecture_page = initial_state
        .pages
        .iter()
        .find(|page| page.page_type == "architecture")
        .unwrap();

    write_file(
        repo_root.join("router.ts").as_path(),
        "export function router() { return false; }",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "stale");
    assert!(status
        .dirty_pages
        .iter()
        .any(|path| path == &topic_page.path));
    assert!(status
        .dirty_pages
        .iter()
        .any(|path| path == &architecture_page.path));

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.state, "fresh");
    assert!(update
        .updated_pages
        .iter()
        .any(|path| path == &topic_page.path));
    assert!(update
        .updated_pages
        .iter()
        .any(|path| path == &architecture_page.path));
}

#[test]
fn status_stays_fresh_after_llm_init_when_only_structural_sets_would_drift() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("wiki.dev.yaml").as_path(),
        concat!(
            "llm:\n",
            "  enabled: true\n",
            "  model: bridge/mock-model\n",
            "  max_calls: 8\n",
            "  parallel_requests: 2\n",
            "  cache_ttl_seconds: 3600\n",
            "  allow_mermaid: true\n",
        ),
    );
    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"status-llm-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );
    write_file(
        repo_root.join("mux.ts").as_path(),
        "export const mux = () => app();\n",
    );
    write_file(
        repo_root.join("Makefile").as_path(),
        "build:\n\tpnpm test\n",
    );

    let mut sink = NoopProgressSink;
    let mut llm_service = StatusStabilityLlmService;
    let init =
        run_init_with_progress_and_llm_as("init", repo_root, &mut sink, Some(&mut llm_service))
            .unwrap();
    assert_eq!(init.state, "fresh");

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "fresh");
    assert!(status.dirty_sources.is_empty());
    assert!(status.dirty_pages.is_empty());
}

fn create_workspace_repo(repo_root: &Path) {
    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"workspace-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_file(
        repo_root.join("packages/app/package.json").as_path(),
        r#"{"name":"app","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("packages/app/src/main.ts").as_path(),
        "export const main = () => 'app';",
    );
    write_file(
        repo_root.join("packages/shared/package.json").as_path(),
        r#"{"name":"shared","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 1;",
    );
}

fn create_single_package_workspace(repo_root: &Path) {
    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"workspace-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_file(
        repo_root.join("packages/app/package.json").as_path(),
        r#"{"name":"app","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("packages/app/src/main.ts").as_path(),
        "export const main = () => 'app';",
    );
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(path, content).unwrap();
}
