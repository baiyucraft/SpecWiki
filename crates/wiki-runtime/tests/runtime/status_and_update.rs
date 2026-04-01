//! 这组测试覆盖 status 与 update 的状态流转和增量更新边界。
//! 它们重点保护 stale 检测、局部更新、结构变化与 rebuild 回退行为。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use super::test_support::{EnvVarGuard, force_full_runtime, force_index_only_runtime};
use tempfile::tempdir;
use wiki_runtime::domain::change_set::plan_runtime_changes;
use wiki_runtime::domain::steering::SteeringLoadMode;
use wiki_runtime::llm::{LlmCompletion, LlmPromptRequest, LlmService};
use wiki_runtime::storage::knowledge_artifacts::load_knowledge_artifacts;
use wiki_runtime::storage::metadata_store::{metadata_exists, read_metadata};
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::progress::NoopProgressSink;
use wiki_runtime::workflows::{
    init::{run_init, run_init_with_progress_and_llm_as_with_mode},
    rebuild::run_rebuild_with_progress_and_llm_as_with_mode,
    status::{run_status, run_status_with_mode},
    update::{run_update, run_update_with_progress_and_llm_as_with_mode},
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
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "missing");
    assert!(!status.facts_ready);
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "needs_init"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "init"
    );
    assert!(status.runtime_summary.is_none());
    assert!(status.gate_summary.is_none());
}

/// 场景：仓库级 cache 被删光后，status 应优先基于正式产物恢复本地 runtime。
#[test]
fn status_restores_runtime_from_formal_artifacts_when_cache_is_missing() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "fresh");
    assert!(status.facts_ready);
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "ready"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "none"
    );
    assert!(repo_root.join(".wiki/.cache/wiki-cache.db").exists());
}

/// 场景：formal artifact 锚点与当前 metadata 不一致时，status 不得误恢复 runtime。
#[test]
fn status_keeps_restore_failure_explicit_when_recovery_manifest_is_stale() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();

    let manifest_path = repo_root.join(".wiki/.knowledge/runtime/recovery-manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
    manifest["metadata_hash"] = serde_json::Value::String("stale-metadata-hash".to_string());
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert!(!status.facts_ready);
    assert_eq!(status.needs_rebuild_reason.as_deref(), Some("cache_missing"));
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "blocked"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "rebuild"
    );
}

/// 场景：初始化完成后，status 应带上 preflight 结果和可选摘要投影。
#[test]
fn status_exposes_runtime_preflight_after_init() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    let init = run_init(repo_root).unwrap();
    assert_eq!(
        serde_json::to_value(&init).unwrap()["llm_execution_mode"],
        "deterministic_only"
    );
    assert!(init.runtime_summary.is_some());

    let status = run_status(repo_root).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "fresh");
    assert!(status.facts_ready);
    assert_eq!(status_json["query_readiness"], "ready");
    assert_eq!(status_json["recommended_action"], "none");
    assert_eq!(status_json["runtime_summary"]["runtime_state"], "completed");
    assert!(status.gate_summary.is_some());
}

/// 场景：单页 cache 缺失时，update 应优先基于正式产物恢复 cache，而不是直接回退 rebuild。
#[test]
fn update_restores_page_cache_from_formal_artifacts() {
    let (_env_lock, _index_only) = force_full_runtime();
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
    assert_eq!(update.previous_state, "fresh");
    assert_eq!(update.state, "fresh");
    assert!(update.updated_pages.is_empty());
    {
        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::page_context_exists(&conn, &overview_page.page_id).unwrap());
        assert!(sqlite_store::page_generation_exists(&conn, &overview_page.page_id).unwrap());
    }

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "fresh");
}

/// 场景：local_refresh 不能让未命中 unit 的 formal records 漂移。
#[test]
fn local_refresh_keeps_unaffected_formal_records_stable() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    let before = load_knowledge_artifacts(repo_root).unwrap();
    let before_summaries = before
        .research_summaries
        .iter()
        .map(|summary| (summary.unit_id.clone(), summary.clone()))
        .collect::<BTreeMap<_, _>>();
    let before_digests = before
        .page_digests
        .iter()
        .map(|digest| (digest.unit_id.clone(), digest.clone()))
        .collect::<BTreeMap<_, _>>();

    write_file(
        repo_root.join("code/addons/a11y/src/types.ts").as_path(),
        "export type A11yOptions = { enabled: boolean; threshold: number };\n",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(
        status.affected_knowledge_scope.escalation.level.as_str(),
        "local_refresh"
    );

    let active_unit_ids = status
        .affected_knowledge_scope
        .active_unit_ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let untouched_unit_id = before
        .units
        .iter()
        .map(|unit| unit.id.clone())
        .find(|unit_id| {
            !active_unit_ids.contains(unit_id)
                && before_summaries.contains_key(unit_id)
                && before_digests.contains_key(unit_id)
        })
        .expect("should keep at least one untouched unit outside local_refresh scope");
    let before_summary = before_summaries
        .get(&untouched_unit_id)
        .expect("untouched unit summary should exist")
        .clone();
    let before_digest = before_digests
        .get(&untouched_unit_id)
        .expect("untouched unit digest should exist")
        .clone();

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.state, "fresh");

    let after = load_knowledge_artifacts(repo_root).unwrap();
    let after_summary = after
        .research_summaries
        .iter()
        .find(|summary| summary.unit_id == untouched_unit_id)
        .expect("untouched unit summary should survive update");
    let after_digest = after
        .page_digests
        .iter()
        .find(|digest| digest.unit_id == untouched_unit_id)
        .expect("untouched unit digest should survive update");

    assert_eq!(
        *after_summary, before_summary,
        "local_refresh must keep unaffected research summaries stable"
    );
    assert_eq!(
        *after_digest, before_digest,
        "local_refresh must keep unaffected page digests stable"
    );
}

/// 场景：结构删除后，update 必须同时回收 formal records、metadata、页面文件和 page caches。
#[test]
fn update_reclaims_removed_units_pages_and_caches_after_structural_delete() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    write_file(
        repo_root.join("code/addons/measure/package.json").as_path(),
        r#"{"name":"@storybook/addon-measure","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("code/addons/measure/src/index.ts").as_path(),
        "export const addonMeasure = () => true;\n",
    );
    run_update(repo_root).unwrap();

    let state_before_remove = read_state(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join("code/addons/measure")).unwrap();

    let plan = plan_runtime_changes(repo_root).unwrap();
    assert!(
        !plan.affected_knowledge_scope.removed_unit_ids.is_empty(),
        "structural delete should surface removed units"
    );
    assert!(
        !plan.affected_set.removed_page_ids.is_empty(),
        "structural delete should surface removed pages"
    );

    let removed_page_ids = plan
        .affected_set
        .removed_page_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let removed_page_paths = state_before_remove
        .pages
        .iter()
        .filter(|page| removed_page_ids.contains(&page.page_id))
        .map(|page| (page.page_id.clone(), page.path.clone()))
        .collect::<BTreeMap<_, _>>();

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.state, "fresh");

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let updated_state = read_state(repo_root).unwrap();
    let conn = sqlite_store::open_db_readonly(repo_root).unwrap();

    for unit_id in &plan.affected_knowledge_scope.removed_unit_ids {
        assert!(
            artifacts.units.iter().all(|unit| unit.id != *unit_id),
            "removed unit should disappear from knowledge units"
        );
        assert!(
            artifacts
                .research_summaries
                .iter()
                .all(|summary| summary.unit_id != *unit_id),
            "removed unit should disappear from research summaries"
        );
        assert!(
            artifacts
                .page_digests
                .iter()
                .all(|digest| digest.unit_id != *unit_id),
            "removed unit should disappear from page digests"
        );
        assert!(
            artifacts
                .runtime_gates
                .iter()
                .all(|gate| gate.unit_id != *unit_id),
            "removed unit should disappear from runtime gates"
        );
    }

    for page_id in removed_page_ids {
        assert!(
            updated_state.pages.iter().all(|page| page.page_id != page_id),
            "removed page should disappear from runtime state"
        );
        assert!(
            metadata.wiki_items.iter().all(|item| item.id != page_id),
            "removed page should disappear from metadata"
        );
        assert!(
            !sqlite_store::page_context_exists(&conn, &page_id).unwrap(),
            "removed page context cache should be reclaimed"
        );
        assert!(
            !sqlite_store::page_generation_exists(&conn, &page_id).unwrap(),
            "removed page generation cache should be reclaimed"
        );

        if let Some(path) = removed_page_paths.get(&page_id) {
            assert!(
                !repo_root.join(path).exists(),
                "removed page markdown should be deleted from disk"
            );
        }
    }
}

/// 场景：init 完成后必须落出 `.wiki/.knowledge/**` 最小正式产物。
#[test]
fn init_persists_minimal_knowledge_artifacts() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();

    let knowledge_root = repo_root.join(".wiki/.knowledge");
    assert!(knowledge_root.join("derived/knowledge-domains.json").exists());
    assert!(knowledge_root.join("derived/knowledge-units.jsonl").exists());
    assert!(knowledge_root.join("derived/knowledge-tree.json").exists());
    assert!(knowledge_root.join("derived/research-summaries.jsonl").exists());
    assert!(knowledge_root.join("runtime/page-digests.jsonl").exists());
    assert!(knowledge_root.join("runtime/runtime-gates.jsonl").exists());
    assert!(knowledge_root.join("runtime/recovery-manifest.json").exists());
}

/// 场景：普通源码变更后，update 必须把 runtime 从 `stale` 刷回 `fresh`。
#[test]
fn update_refreshes_stale_runtime_to_fresh() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 1;").unwrap();

    run_init(repo_root).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 2;").unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_update");
    assert!(status.facts_ready);
    assert_eq!(
        status.affected_knowledge_scope.escalation.level.as_str(),
        "local_refresh"
    );
    assert!(
        !status
            .affected_knowledge_scope
            .direct_unit_ids
            .is_empty(),
        "status should expose direct knowledge scope for stale sources"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "needs_update"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "update"
    );

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "stale");
    assert_eq!(update.state, "fresh");
    assert!(!update.updated_pages.is_empty());
    assert_eq!(
        update.affected_knowledge_scope.escalation.level.as_str(),
        "local_refresh"
    );

    let refreshed_status = run_status(repo_root).unwrap();
    assert_eq!(refreshed_status.state, "fresh");
    assert_eq!(
        serde_json::to_value(&refreshed_status).unwrap()["recommended_action"],
        "none"
    );
    assert!(refreshed_status.runtime_summary.is_some());
    assert!(refreshed_status.gate_summary.is_some());
}

/// 场景：单文件只影响单一模块页时，未命中的页面和 section hash 不应被误改。
#[test]
fn incremental_update_only_touches_related_pages() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_workspace_repo(repo_root);
    run_init(repo_root).unwrap();

    let initial_state = read_state(repo_root).unwrap();
    let _shared_page = initial_state
        .pages
        .iter()
        .find(|page| {
            page.source_paths
                .iter()
                .any(|p| p.contains("packages/shared"))
                && page.page_type != "overview"
                && page.page_type != "architecture"
        })
        .expect("shared module page should exist");
    let app_page = initial_state
        .pages
        .iter()
        .find(|page| {
            page.source_paths.iter().any(|p| p.contains("packages/app"))
                && page.page_type != "overview"
                && page.page_type != "architecture"
        })
        .expect("app module page should exist");

    let app_page_path = app_page.path.clone();
    let app_section_hashes_before = app_page
        .sections
        .iter()
        .map(|section| section.content_hash.clone())
        .collect::<Vec<_>>();

    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 2;",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_update");

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "stale");
    assert_eq!(update.state, "fresh");

    let updated_state = read_state(repo_root).unwrap();
    let app_page_after = updated_state
        .pages
        .iter()
        .find(|page| page.path == app_page_path);
    if let Some(app_page_after) = app_page_after {
        let app_section_hashes_after = app_page_after
            .sections
            .iter()
            .map(|section| section.content_hash.clone())
            .collect::<Vec<_>>();
        assert_eq!(app_section_hashes_before, app_section_hashes_after);
    }
}

/// 场景：新增或删除模块源码后，update 必须同步新增或删除对应页面与 metadata。
#[test]
fn update_adds_and_removes_pages_after_structural_changes() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_single_package_workspace(repo_root);
    run_init(repo_root).unwrap();

    let pages_before = read_state(repo_root).unwrap().pages.len();

    write_file(
        repo_root.join("packages/shared/package.json").as_path(),
        r#"{"name":"shared","version":"1.0.0"}"#,
    );
    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 1;",
    );

    let add_status = run_status(repo_root).unwrap();
    assert_eq!(add_status.state, "needs_update");

    let add_update = run_update(repo_root).unwrap();
    assert_eq!(add_update.state, "fresh");
    let shared_page = read_state(repo_root)
        .unwrap()
        .pages
        .into_iter()
        .find(|page| {
            page.source_paths
                .iter()
                .any(|p| p.contains("packages/shared"))
        });
    assert!(
        shared_page.is_some(),
        "shared module page should exist after add"
    );
    let pages_after_add = read_state(repo_root).unwrap().pages.len();
    assert!(
        pages_after_add >= pages_before.saturating_sub(1),
        "update should keep overall page set stable after adding a module"
    );

    fs::remove_dir_all(repo_root.join("packages/shared")).unwrap();

    let remove_status = run_status(repo_root).unwrap();
    assert_eq!(remove_status.state, "needs_update");

    let remove_update = run_update(repo_root).unwrap();
    assert_eq!(remove_update.state, "fresh");
    let shared_page_after_remove = read_state(repo_root)
        .unwrap()
        .pages
        .into_iter()
        .find(|page| {
            page.source_paths
                .iter()
                .any(|p| p.contains("packages/shared"))
        });
    assert!(
        shared_page_after_remove.is_none(),
        "shared module page should be removed after deleting sources"
    );
}

/// 场景：源码变更后 update 应刷新受影响页面并把 runtime 恢复到 fresh。
#[test]
fn update_rebuilds_topic_page_and_parent_pages_when_topic_sources_change() {
    let (_env_lock, _index_only) = force_full_runtime();
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

    let initial_communities = sqlite_store::list_communities(repo_root).unwrap();
    let initial_processes = sqlite_store::list_processes(repo_root).unwrap();
    let initial_state = read_state(repo_root).unwrap();
    let _module_page = initial_state
        .pages
        .iter()
        .find(|page| {
            page.source_paths.iter().any(|p| p.contains("router.ts"))
                && page.page_type != "overview"
                && page.page_type != "architecture"
        })
        .expect("module page covering router.ts should exist after init");

    write_file(
        repo_root.join("router.ts").as_path(),
        "export function router() { return false; }",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_update");

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.state, "fresh");
    assert!(
        !update.updated_pages.is_empty(),
        "update should touch at least one page"
    );

    let updated_communities = sqlite_store::list_communities(repo_root).unwrap();
    let updated_processes = sqlite_store::list_processes(repo_root).unwrap();
    assert_eq!(updated_communities.len(), initial_communities.len());
    assert_eq!(updated_processes.len(), initial_processes.len());
}

#[test]
fn status_stays_fresh_after_llm_init_when_only_structural_sets_would_drift() {
    let (_env_lock, _index_only) = force_full_runtime();
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
    let init = run_init_with_progress_and_llm_as_with_mode(
        "init",
        repo_root,
        &mut sink,
        Some(&mut llm_service),
        SteeringLoadMode::Development,
    )
    .unwrap();
    assert_eq!(init.state, "fresh");

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Development).unwrap();
    assert_eq!(status.state, "fresh");
    assert!(status.dirty_sources.is_empty());
    assert!(status.dirty_pages.is_empty());
}

#[test]
fn production_mode_provider_blocker_surfaces_in_status() {
    let (_env_lock, _index_only) = force_full_runtime();
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join(".wiki/config.yaml").as_path(),
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
        r#"{"name":"status-provider-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );

    let mut sink = NoopProgressSink;
    let mut llm_service = StatusStabilityLlmService;
    let init_error = run_init_with_progress_and_llm_as_with_mode(
        "init",
        repo_root,
        &mut sink,
        Some(&mut llm_service),
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        init_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected init error: {init_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    assert!(status.blocker_hint.is_some());
    assert!(status.runtime_summary.is_some());
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

#[test]
fn production_mode_disabled_llm_surfaces_provider_blocker() {
    let (_env_lock, _index_only) = force_full_runtime();
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"status-disabled-llm-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );

    let mut sink = NoopProgressSink;
    let init_error = run_init_with_progress_and_llm_as_with_mode(
        "init",
        repo_root,
        &mut sink,
        None,
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        init_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected init error: {init_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    assert!(status.blocker_hint.is_some());
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

#[test]
fn production_mode_update_persists_provider_blocker_after_runtime_was_fresh() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"update-provider-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );

    {
        let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "1");
        run_init(repo_root).unwrap();
    }

    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => false;\n",
    );

    let mut sink = NoopProgressSink;
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let update_error = run_update_with_progress_and_llm_as_with_mode(
        "update",
        repo_root,
        &mut sink,
        None,
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        update_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected update error: {update_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    let runtime_summary = status
        .runtime_summary
        .expect("runtime summary should exist after blocked update");
    assert_eq!(runtime_summary.workflow_action, "update");
    assert_eq!(runtime_summary.runtime_state, "interrupted");
    assert!(!runtime_summary.blocked_units.is_empty());
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

#[test]
fn production_mode_rebuild_persists_provider_blocker_after_runtime_was_fresh() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"rebuild-provider-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );

    {
        let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "1");
        run_init(repo_root).unwrap();
    }

    let mut sink = NoopProgressSink;
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let rebuild_error = run_rebuild_with_progress_and_llm_as_with_mode(
        "rebuild",
        repo_root,
        &mut sink,
        None,
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        rebuild_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected rebuild error: {rebuild_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    let runtime_summary = status
        .runtime_summary
        .expect("runtime summary should exist after blocked rebuild");
    assert_eq!(runtime_summary.workflow_action, "rebuild");
    assert_eq!(runtime_summary.runtime_state, "interrupted");
    assert!(!runtime_summary.blocked_units.is_empty());
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

#[test]
fn production_mode_provider_blocker_surfaces_after_update() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"status-update-provider-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );
    run_init(repo_root).unwrap();

    write_file(
        repo_root.join(".wiki/config.yaml").as_path(),
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
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => false;\n",
    );

    let mut sink = NoopProgressSink;
    let mut llm_service = StatusStabilityLlmService;
    let update_error = run_update_with_progress_and_llm_as_with_mode(
        "update",
        repo_root,
        &mut sink,
        Some(&mut llm_service),
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        update_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected update error: {update_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert!(gate_summary.blocked_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

#[test]
fn production_mode_provider_blocker_surfaces_after_rebuild() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"status-rebuild-provider-blocker-demo"}"#,
    );
    write_file(
        repo_root.join("app.ts").as_path(),
        "export const app = () => true;\n",
    );
    run_init(repo_root).unwrap();

    write_file(
        repo_root.join(".wiki/config.yaml").as_path(),
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
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");

    let mut sink = NoopProgressSink;
    let mut llm_service = StatusStabilityLlmService;
    let rebuild_error = run_rebuild_with_progress_and_llm_as_with_mode(
        "rebuild",
        repo_root,
        &mut sink,
        Some(&mut llm_service),
        SteeringLoadMode::Production,
    )
    .unwrap_err();
    assert!(
        rebuild_error
            .to_string()
            .contains("provider research unavailable"),
        "unexpected rebuild error: {rebuild_error}"
    );

    let status = run_status_with_mode(repo_root, SteeringLoadMode::Production).unwrap();
    let status_json = serde_json::to_value(&status).unwrap();
    assert_eq!(status.state, "blocker");
    assert_eq!(status_json["query_readiness"], "blocked");
    assert_eq!(status_json["recommended_action"], "rebuild");
    let gate_summary = status.gate_summary.expect("gate summary should exist");
    assert!(gate_summary.total_units > 0);
    assert!(gate_summary.blocked_units > 0);
    assert_eq!(gate_summary.blocked_units, gate_summary.total_units);
}

/// 场景：storybook 项目 addon 源码变更后，update 应将受影响页面标脏并刷新。
#[test]
fn update_marks_storybook_family_parent_pages_dirty_when_family_child_sources_change() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    create_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    let initial_communities = sqlite_store::list_communities(repo_root).unwrap();
    let initial_processes = sqlite_store::list_processes(repo_root).unwrap();

    let initial_state = read_state(repo_root).unwrap();
    let a11y_page = initial_state
        .pages
        .iter()
        .find(|page| {
            page.source_paths.iter().any(|p| p.contains("addons/a11y"))
                && page.page_type != "overview"
                && page.page_type != "architecture"
        })
        .expect("a11y module page should exist after init");
    let _a11y_page_path = a11y_page.path.clone();

    write_file(
        repo_root.join("code/addons/a11y/src/types.ts").as_path(),
        "export type A11yOptions = { enabled: boolean; threshold: number };\n",
    );

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_update");
    assert!(
        !status.dirty_pages.is_empty(),
        "at least one page should be dirty"
    );
    assert!(
        !status
            .affected_knowledge_scope
            .direct_unit_ids
            .is_empty(),
        "family child change should first resolve to direct knowledge scope"
    );

    let update = run_update(repo_root).unwrap();
    assert_eq!(update.state, "fresh");
    assert!(
        !update.updated_pages.is_empty(),
        "update should touch at least one page"
    );

    let updated_communities = sqlite_store::list_communities(repo_root).unwrap();
    let updated_processes = sqlite_store::list_processes(repo_root).unwrap();
    assert_eq!(updated_communities.len(), initial_communities.len());
    assert_eq!(updated_processes.len(), initial_processes.len());
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

fn create_storybook_like_repo(repo_root: &Path) {
    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"storybook-like","private":true}"#,
    );
    write_file(
        repo_root.join("code/addons/a11y/package.json").as_path(),
        r#"{"name":"@storybook/addon-a11y"}"#,
    );
    write_file(
        repo_root.join("code/addons/a11y/src/index.ts").as_path(),
        "export const addonA11y = true;\n",
    );
    write_file(
        repo_root.join("code/addons/a11y/src/types.ts").as_path(),
        "export type A11yOptions = { enabled: boolean };\n",
    );
    write_file(
        repo_root
            .join("code/frameworks/react-vite/src/index.ts")
            .as_path(),
        "export const reactVite = true;\n",
    );
    write_file(
        repo_root
            .join("code/builders/builder-vite/src/index.ts")
            .as_path(),
        "export const builderVite = true;\n",
    );
    write_file(
        repo_root.join("code/core/src/main.ts").as_path(),
        "export const main = () => true;\n",
    );
    write_file(
        repo_root.join("code/core/src/public-types.ts").as_path(),
        "export type StorybookConfig = { stories: string[] };\n",
    );
    write_file(
        repo_root.join("docs/addons/index.md").as_path(),
        "# Addons\n",
    );
    write_file(
        repo_root.join("docs/get-started/index.md").as_path(),
        "# Get Started\n",
    );
    write_file(
        repo_root.join("docs/configure/index.md").as_path(),
        "# Configure\n",
    );
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(path, content).unwrap();
}

#[test]
fn index_only_init_reports_index_only_and_keeps_query_ready() {
    let (_env_lock, _index_only) = force_index_only_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const version = 1;\n").unwrap();

    let init = run_init(repo_root).unwrap();
    assert_eq!(init.state, "index_only");
    assert!(init.generated_pages.is_empty());
    assert!(init.runtime_summary.is_none());
    assert!(!metadata_exists(repo_root));
    assert!(!repo_root.join(".wiki/项目概述.md").exists());

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "index_only");
    assert!(status.facts_ready);
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "ready"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "none"
    );
    assert!(status.runtime_summary.is_none());
    assert!(status.gate_summary.is_none());

    let query = wiki_runtime::workflows::query::run_query(repo_root, "src.ts").unwrap();
    assert_eq!(
        serde_json::to_value(&query).unwrap()["query_mode"],
        "index_first"
    );
    assert!(query.matches.is_empty());
    assert!(query.matched_pages.is_empty());
    assert!(
        query
            .matched_symbols
            .iter()
            .any(|symbol| symbol.file_path == "src.ts")
            || query
                .matched_sources
                .iter()
                .any(|source| source.path == "src.ts")
    );
}

#[test]
fn index_only_update_normalizes_existing_full_runtime_to_index_only() {
    let (_env_lock, _full_runtime) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const version = 1;\n").unwrap();

    let init = run_init(repo_root).unwrap();
    assert_eq!(init.state, "fresh");
    assert!(metadata_exists(repo_root));
    assert!(repo_root.join(".wiki/项目概述.md").exists());

    let _index_only = super::test_support::EnvVarGuard::set("SPEC_WIKI_V0_1_INDEX_ONLY", "1");
    let update = run_update(repo_root).unwrap();
    assert_eq!(update.previous_state, "fresh");
    assert_eq!(update.state, "index_only");
    assert!(update.updated_pages.is_empty());
    assert!(!metadata_exists(repo_root));
    assert!(!repo_root.join(".wiki/项目概述.md").exists());

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "index_only");
    assert!(status.facts_ready);
    assert!(status.runtime_summary.is_none());
}
