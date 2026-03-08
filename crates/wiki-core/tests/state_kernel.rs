use std::fs;

use tempfile::tempdir;
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
