//! 这组测试覆盖 WikiState、change planning 与 cache 回退边界。
//! 它们保护状态层与正式索引之间的 roundtrip、一致性和降级语义。

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_core::domain::change_set::plan_runtime_changes;
use wiki_core::storage::cache_store::{
    page_context_cache_path, page_generation_cache_path, read_page_context_cache,
    read_page_generation_cache,
};
use wiki_core::storage::state_store::state_path;
use wiki_core::workflows::{
    init::run_init,
    query::run_query,
    status::run_status,
};

/// 8.1 WikiState -> MetadataMapper -> WikiMetadata 的 roundtrip 一致性。
#[test]
fn state_metadata_roundtrip_produces_consistent_output() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"roundtrip-test","version":"1.0.0"}"#).unwrap();
    fs::write(repo_root.join("index.ts"), "export const main = () => {};").unwrap();

    run_init(repo_root).unwrap();

    // 读取 init 产出的 WikiState 和 WikiMetadata
    let state = wiki_core::storage::state_store::read_state(repo_root).unwrap();
    let metadata = wiki_core::storage::metadata_store::read_metadata(repo_root).unwrap();

    // 页面数量一致
    assert_eq!(state.pages.len(), metadata.wiki_items.len());

    // 源码数量一致
    assert_eq!(state.sources.len(), metadata.source_files.len());

    // 模块数量一致
    assert_eq!(state.modules.len(), metadata.modules.len());

    // 关系数量一致
    assert_eq!(state.relations.len(), metadata.relations.len());

    // 逐页面校验核心字段
    for (page, item) in state.pages.iter().zip(metadata.wiki_items.iter()) {
        assert_eq!(page.page_id, item.id);
        assert_eq!(page.title, item.title);
        assert_eq!(page.path, item.path);
        assert_eq!(page.page_type, item.item_type);
        assert_eq!(page.parent_id, item.parent_id);
        assert_eq!(page.ancestor_ids, item.ancestor_ids);
        assert_eq!(page.content_hash, item.content_hash);
        assert_eq!(page.module_ids, item.module_ids);
        assert_eq!(page.source_paths, item.source_files);
        assert_eq!(page.summary, item.summary);
        assert_eq!(page.provenance, item.provenance);
    }

    // 逐源码校验核心字段
    for (source, record) in state.sources.iter().zip(metadata.source_files.iter()) {
        assert_eq!(source.source_id, record.id);
        assert_eq!(source.path, record.path);
        assert_eq!(source.fingerprint, record.fingerprint);
        assert_eq!(source.page_ids, record.wiki_item_ids);
        assert_eq!(source.module_ids, record.module_ids);
    }

    // dirty_state 一致
    assert_eq!(state.dirty_state.status, metadata.dirty_state.status);

    // build_state 与 metadata 展示字段一致
    assert_eq!(state.build_state.generated_at, metadata.generated_at);
}

/// 8.2 query 输出包含 context_pack 和 provenance_summary。
#[test]
fn query_output_includes_context_pack_and_provenance_summary() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"query-test"}"#).unwrap();
    fs::write(repo_root.join("app.ts"), "export function run() {}").unwrap();

    run_init(repo_root).unwrap();

    let report = run_query(repo_root, "项目概述").unwrap();
    assert!(!report.matches.is_empty(), "应该命中至少一个页面");
    assert!(!report.provenance_summary.is_empty(), "provenance_summary 不应为空");

    for hit in &report.matches {
        // context_pack 应该有内容（至少有模块摘要）
        assert!(
            !hit.context_pack.module_summaries.is_empty()
                || !hit.context_pack.key_source_paths.is_empty(),
            "context_pack 应包含模块摘要或关键源码"
        );
    }
}

/// 7.3 + 8.3 删除 wiki-state.json 后 status/query 仍能从 metadata 重建并正常工作。
#[test]
fn status_and_query_work_after_state_cache_deleted() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"fallback-test"}"#).unwrap();
    fs::write(repo_root.join("lib.ts"), "export const x = 1;").unwrap();

    run_init(repo_root).unwrap();

    // 确认 wiki-state.json 存在
    assert!(state_path(repo_root).exists());

    // 删除 wiki-state.json
    fs::remove_file(state_path(repo_root)).unwrap();
    assert!(!state_path(repo_root).exists());

    // status 应该报告 needs_rebuild（因为 has_cache_layout 现在检查 wiki-state.json）
    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(status.needs_rebuild_reason.as_deref(), Some("cache_missing"));

    // query 仍然能从 metadata 回退工作
    let query = run_query(repo_root, "项目概述").unwrap();
    assert!(!query.matches.is_empty(), "query 应该能从 metadata 回退并返回结果");
    assert!(!query.provenance_summary.is_empty());
}

/// cache 全部删除后，status 仍能从 metadata 回退工作。
#[test]
fn status_works_after_full_cache_deletion() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"cache-test"}"#).unwrap();

    run_init(repo_root).unwrap();

    // 删除整个 .cache 目录
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    // status 应该报告 needs_rebuild 而不是崩溃
    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(status.needs_rebuild_reason.as_deref(), Some("cache_missing"));
}

/// 场景：init 必须一次性写出页面 input hash、section 状态和 page-level cache。
#[test]
fn init_persists_page_input_hash_sections_and_page_caches() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"cache-layout-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 'hello';",
    );

    run_init(repo_root).unwrap();

    let state = wiki_core::storage::state_store::read_state(repo_root).unwrap();
    assert!(!state.pages.is_empty());

    for page in &state.pages {
        assert!(!page.input_hash.is_empty(), "页面必须持久化 input_hash");
        assert!(!page.sections.is_empty(), "页面必须包含稳定 section 状态");
        assert!(page_context_cache_path(repo_root, &page.page_id).exists());
        assert!(page_generation_cache_path(repo_root, &page.page_id).exists());

        let context_cache = read_page_context_cache(repo_root, &page.page_id).unwrap();
        let generation_cache = read_page_generation_cache(repo_root, &page.page_id).unwrap();
        assert_eq!(context_cache.input_hash, page.input_hash);
        assert_eq!(generation_cache.input_hash, page.input_hash);
        assert_eq!(generation_cache.content_hash, page.content_hash);
        assert_eq!(generation_cache.sections.len(), page.sections.len());
    }
}

/// 场景：change planning 必须区分普通源码修改与触发 replan 的结构变化。
#[test]
fn change_plan_detects_modified_and_structural_sources() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"change-plan-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 1;",
    );
    write_file(
        repo_root.join("src/util.ts").as_path(),
        "export const util = () => 1;",
    );

    run_init(repo_root).unwrap();

    write_file(
        repo_root.join("src/util.ts").as_path(),
        "export const util = () => 2;",
    );
    let modified_plan = plan_runtime_changes(repo_root).unwrap();
    assert_eq!(modified_plan.state(), "stale");
    assert!(modified_plan.change_set.modified_sources.iter().any(|path| path == "src/util.ts"));
    assert!(!modified_plan.change_set.requires_replan);

    run_init(repo_root).unwrap();
    write_file(
        repo_root.join("packages/shared/package.json").as_path(),
        r#"{"name":"shared"}"#,
    );
    write_file(
        repo_root.join("packages/shared/src/util.ts").as_path(),
        "export const util = () => 1;",
    );

    let structural_plan = plan_runtime_changes(repo_root).unwrap();
    assert_eq!(structural_plan.state(), "stale");
    assert!(structural_plan.change_set.added_sources.iter().any(|path| path == "packages/shared/package.json"));
    assert!(structural_plan.change_set.requires_replan);
    assert!(!structural_plan.affected_set.affected_page_ids.is_empty());
}

/// 场景：源码删除不能被折叠成普通 stale，必须保留 removed source 证据。
#[test]
fn change_plan_detects_removed_sources() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"remove-source-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 1;",
    );

    run_init(repo_root).unwrap();
    fs::remove_file(repo_root.join("src/index.ts")).unwrap();

    let plan = plan_runtime_changes(repo_root).unwrap();
    assert_eq!(plan.state(), "stale");
    assert!(plan.change_set.removed_sources.iter().any(|path| path == "src/index.ts"));
    assert!(plan.change_set.requires_replan);
}

/// 场景：缺失单页 generation cache 时，status 必须升级为 `needs_rebuild`。
#[test]
fn status_reports_needs_rebuild_when_page_level_cache_is_missing() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"page-cache-missing-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 1;",
    );

    run_init(repo_root).unwrap();
    let state = wiki_core::storage::state_store::read_state(repo_root).unwrap();
    let overview_page = state
        .pages
        .iter()
        .find(|page| page.page_type == "overview")
        .unwrap();

    fs::remove_file(page_generation_cache_path(repo_root, &overview_page.page_id)).unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(status.needs_rebuild_reason.as_deref(), Some("cache_missing"));
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(path, content).unwrap();
}
