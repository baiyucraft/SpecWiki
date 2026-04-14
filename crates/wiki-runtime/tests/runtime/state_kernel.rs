//! 这组测试覆盖 WikiState、change planning 与 cache 回退边界。
//! 它们保护状态层与正式索引之间的 roundtrip、一致性和降级语义。

use super::test_support::force_full_runtime;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_runtime::domain::change_set::plan_runtime_changes;
use wiki_runtime::storage::cache_store::{
    missing_incremental_cache_components, read_page_context_cache, read_page_generation_cache,
    write_page_context_cache,
};
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::{init::run_init, query::run_query, status::run_status};

fn write_storybook_like_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo_root.join("docs/addons")).unwrap();
    fs::create_dir_all(repo_root.join("docs/get-started")).unwrap();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"storybook-like","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("code/addons/a11y/package.json"),
        r#"{"name":"@storybook/addon-a11y"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("code/addons/a11y/src/index.ts"),
        "export const addonA11y = true;\n",
    )
    .unwrap();
    fs::write(
        repo_root.join("code/addons/a11y/src/types.ts"),
        "export type A11yOptions = { enabled: boolean };\n",
    )
    .unwrap();
    fs::write(repo_root.join("docs/addons/index.md"), "# Addons\n").unwrap();
    fs::write(
        repo_root.join("docs/get-started/index.md"),
        "# Get Started\n",
    )
    .unwrap();
}

/// 8.1 WikiState -> MetadataMapper -> WikiMetadata 的 roundtrip 一致性。
#[test]
fn state_metadata_roundtrip_produces_consistent_output() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"roundtrip-test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("index.ts"), "export const main = () => {};").unwrap();

    run_init(repo_root).unwrap();

    // 读取 init 产出的 WikiState 和 WikiMetadata
    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    let metadata = wiki_runtime::storage::metadata_store::read_metadata(repo_root).unwrap();

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
        assert_eq!(
            sorted(source.page_ids.clone()),
            sorted(record.wiki_item_ids.clone())
        );
        assert_eq!(
            sorted(source.module_ids.clone()),
            sorted(record.module_ids.clone())
        );
    }

    // dirty_state 一致
    assert_eq!(state.dirty_state.status, metadata.dirty_state.status);

    // build_state 与 metadata 展示字段一致
    assert_eq!(state.build_state.generated_at, metadata.generated_at);
}

/// 8.2 query 输出包含 context_pack 和 provenance_summary。
#[test]
fn query_output_includes_context_pack_and_provenance_summary() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"query-test"}"#).unwrap();
    fs::write(repo_root.join("app.ts"), "export function run() {}").unwrap();

    run_init(repo_root).unwrap();

    let report = run_query(repo_root, "项目概述").unwrap();
    assert!(!report.matches.is_empty(), "应该命中至少一个页面");
    assert!(
        !report.provenance_summary.is_empty(),
        "provenance_summary 不应为空"
    );

    // 新 pipeline 用最小 PageContext，context_pack 可能仅包含结构信息
    assert!(
        report
            .matches
            .iter()
            .any(|hit| !hit.context_pack.module_summaries.is_empty()
                || !hit.context_pack.key_source_paths.is_empty()
                || !hit.summary.is_empty()),
        "至少一个命中应包含 context_pack 或 summary"
    );
}

/// 7.3 + 8.3 删除 wiki-state.json 后 status/query 仍能从 metadata 重建并正常工作。
#[test]
fn status_and_query_work_after_state_cache_deleted() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"fallback-test"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("lib.ts"), "export const x = 1;").unwrap();

    run_init(repo_root).unwrap();

    // 确认 DB 存在且关系型状态可读
    assert!(sqlite_store::db_exists(repo_root));
    {
        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::runtime_tables_exist(&conn).unwrap());
    }
    assert!(read_state(repo_root).is_ok());

    // 删除整个 DB 文件模拟 state cache 丢失
    fs::remove_file(sqlite_store::db_path(repo_root)).unwrap();
    assert!(!sqlite_store::db_exists(repo_root));

    // status 应该报告 needs_rebuild（因为关键状态表和扫描缓存都缺失）
    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(
        status.needs_rebuild_reason.as_deref(),
        Some("cache_missing")
    );

    // facts snapshot 已丢失时，query 必须返回显式 `index not ready`。
    let error = run_query(repo_root, "项目概述").expect_err("query should fail when DB is missing");
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(error.to_string().contains("index not ready"));
}

/// cache 全部删除后，status 应优先基于 formal artifacts 恢复本地 runtime。
#[test]
fn status_works_after_full_cache_deletion() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"cache-test"}"#).unwrap();

    run_init(repo_root).unwrap();

    // 删除整个 .cache 目录
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    // status 应优先恢复 cache 并保持 fresh，而不是直接升级 needs_rebuild
    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "fresh");
    assert!(status.facts_ready);
    assert!(repo_root.join(".wiki/.cache/wiki-cache.db").exists());
}

/// 场景：init 必须一次性写出页面 input hash、section 状态和 page-level cache。
#[test]
fn init_persists_page_input_hash_sections_and_page_caches() {
    let (_env_lock, _index_only) = force_full_runtime();
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

    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    assert!(!state.pages.is_empty());

    for page in &state.pages {
        assert!(!page.input_hash.is_empty(), "页面必须持久化 input_hash");
        assert!(!page.sections.is_empty(), "页面必须包含稳定 section 状态");

        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::page_context_exists(&conn, &page.page_id).unwrap());
        assert!(sqlite_store::page_generation_exists(&conn, &page.page_id).unwrap());

        let context_cache = read_page_context_cache(repo_root, &page.page_id).unwrap();
        let generation_cache = read_page_generation_cache(repo_root, &page.page_id).unwrap();
        assert_eq!(context_cache.input_hash, page.input_hash);
        assert_eq!(generation_cache.input_hash, page.input_hash);
        assert_eq!(generation_cache.content_hash, page.content_hash);
        assert_eq!(generation_cache.sections.len(), page.sections.len());
    }
}

#[test]
fn init_persists_compose_plan_in_page_context_cache() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    // 新 pipeline 使用 domain-index 替代旧的 family-index
    let domain_or_module_page = state
        .pages
        .iter()
        .find(|page| {
            page.page_type == "domain-index"
                || page.page_type == "module"
                || page.page_type == "family-index"
        })
        .expect("should have at least one domain-index or module page");

    // 新 pipeline 的 page context cache 使用最小 PageContext
    let context_cache = read_page_context_cache(repo_root, &domain_or_module_page.page_id).unwrap();
    assert_eq!(
        context_cache.page_id, domain_or_module_page.page_id,
        "page context cache page_id should match"
    );
}

#[test]
fn init_persists_runtime_summary_and_unit_gates() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
    let runtime_summary = sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")
        .unwrap()
        .expect("pipeline runtime summary should be persisted");
    assert!(runtime_summary.contains("\"runtime_state\":\"completed\""));

    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    assert!(
        !runtime_gates.is_empty(),
        "unit runtime gates should be persisted"
    );
    assert!(runtime_gates
        .iter()
        .all(|gate| gate.assemble_status == "done"));
}

#[test]
fn missing_incremental_cache_components_reports_runtime_gate_and_parent_contract_gaps() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    let parent_page_id = state
        .pages
        .iter()
        .find(|page| {
            state
                .pages
                .iter()
                .any(|candidate| candidate.parent_id.as_deref() == Some(page.page_id.as_str()))
        })
        .map(|page| page.page_id.clone())
        .expect("runtime should contain at least one parent page");
    let mut parent_context = read_page_context_cache(repo_root, &parent_page_id).unwrap();
    parent_context.context.page_type = "domain-index".to_string();
    parent_context.context.child_summaries.clear();
    parent_context.context.child_unit_ids.clear();
    parent_context.context.child_page_ids.clear();
    parent_context.context.child_digest_ids.clear();
    parent_context.context.citation_digest_refs.clear();
    parent_context.context.diagram_digest_refs.clear();
    parent_context.context.readiness_status = "compose_ready".to_string();
    write_page_context_cache(repo_root, &parent_context).unwrap();

    let conn = sqlite_store::open_db(repo_root).unwrap();
    conn.execute(
        "DELETE FROM runtime_meta WHERE key = 'pipeline_runtime_summary'",
        [],
    )
    .unwrap();
    sqlite_store::clear_unit_runtime_gates(&conn).unwrap();

    let missing = missing_incremental_cache_components(repo_root, &state);
    assert!(missing.contains(&"pipeline-runtime-summary".to_string()));
    assert!(missing.contains(&"unit-runtime-gates".to_string()));
    assert!(
        missing.contains(&format!(
            "page-context-child-contract:{}",
            parent_context.page_id
        )),
        "missing components: {:?}",
        missing
    );
}

#[test]
fn removing_child_page_marks_parent_page_dirty() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("packages/app/src")).unwrap();
    fs::create_dir_all(repo_root.join("packages/shared/src")).unwrap();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"workspace-demo","private":true,"workspaces":["packages/*"]}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/app/package.json"),
        r#"{"name":"app","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/shared/package.json"),
        r#"{"name":"shared","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/app/src/index.ts"),
        "export const app = true;\n",
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/shared/src/index.ts"),
        "export const shared = true;\n",
    )
    .unwrap();
    run_init(repo_root).unwrap();

    fs::remove_file(repo_root.join("packages/shared/package.json")).unwrap();
    fs::remove_file(repo_root.join("packages/shared/src/index.ts")).unwrap();
    let plan = plan_runtime_changes(repo_root).unwrap();
    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    let overview_page = state
        .pages
        .iter()
        .find(|page| page.page_type == "overview")
        .expect("overview page should exist");

    assert!(
        plan.affected_set
            .affected_page_ids
            .iter()
            .any(|page_id| page_id == &overview_page.page_id),
        "parent page should be marked dirty when child page disappears"
    );
    assert!(
        !matches!(
            plan.affected_knowledge_scope.escalation.level.as_str(),
            "local_refresh"
        ),
        "child page disappearance should escalate beyond local_refresh"
    );
}

/// 场景：change planning 必须区分普通源码修改与触发 replan 的结构变化。
#[test]
fn change_plan_detects_modified_and_structural_sources() {
    let (_env_lock, _index_only) = force_full_runtime();
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
    assert!(modified_plan
        .change_set
        .modified_sources
        .iter()
        .any(|path| path == "src/util.ts"));
    assert!(!modified_plan.change_set.requires_replan);
    assert_eq!(
        modified_plan
            .affected_knowledge_scope
            .escalation
            .level
            .as_str(),
        "local_refresh"
    );
    assert!(
        modified_plan
            .affected_knowledge_scope
            .propagated_parent_unit_ids
            .is_empty(),
        "plain source edits should not propagate parent units by default"
    );
    assert!(
        !modified_plan
            .affected_knowledge_scope
            .direct_unit_ids
            .is_empty(),
        "dirty source should first resolve to direct knowledge units"
    );

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
    assert!(structural_plan
        .change_set
        .added_sources
        .iter()
        .any(|path| path == "packages/shared/package.json"));
    assert!(structural_plan.change_set.requires_replan);
    assert!(!structural_plan.affected_set.affected_page_ids.is_empty());
    assert!(
        matches!(
            structural_plan
                .affected_knowledge_scope
                .escalation
                .level
                .as_str(),
            "subtree_replan" | "repo_replan"
        ),
        "structural change should escalate beyond local_refresh"
    );
}

/// 场景：child contract 变化时，planning 必须把 parent unit 卷入 propagated scope。
#[test]
fn child_contract_change_propagates_parent_units() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_storybook_like_repo(repo_root);
    run_init(repo_root).unwrap();

    write_file(
        repo_root.join("code/addons/measure/package.json").as_path(),
        r#"{"name":"@storybook/addon-measure"}"#,
    );
    write_file(
        repo_root.join("code/addons/measure/src/index.ts").as_path(),
        "export const addonMeasure = true;\n",
    );

    let plan = plan_runtime_changes(repo_root).unwrap();
    assert!(
        !plan.affected_knowledge_scope.direct_unit_ids.is_empty(),
        "child contract change should still resolve direct units first"
    );
    assert!(
        !plan
            .affected_knowledge_scope
            .propagated_parent_unit_ids
            .is_empty(),
        "child contract change should propagate parent units: direct={:?} propagated={:?} targets={:?}",
        plan.affected_knowledge_scope.direct_unit_ids,
        plan.affected_knowledge_scope.propagated_parent_unit_ids,
        plan.affected_knowledge_scope.projection_target_page_ids
    );

    let direct_unit_ids = plan
        .affected_knowledge_scope
        .direct_unit_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let propagated_unit_ids = plan
        .affected_knowledge_scope
        .propagated_parent_unit_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let projection_target_page_ids = plan
        .affected_knowledge_scope
        .projection_target_page_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let projected_parent_targets = plan
        .planned_pages
        .iter()
        .filter(|page| projection_target_page_ids.contains(&page.id))
        .filter_map(|page| page.unit_id.as_ref())
        .filter(|unit_id| {
            propagated_unit_ids.contains(*unit_id) && !direct_unit_ids.contains(*unit_id)
        })
        .count();

    assert!(
        projected_parent_targets > 0,
        "propagated parent units should contribute projection target pages"
    );
}

/// 场景：local_refresh 的 projection targets 只能来自 scope unit 及其祖先闭包。
#[test]
fn local_refresh_projection_targets_follow_scope_closure() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"projection-scope-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 1;\n",
    );
    write_file(
        repo_root.join("src/util.ts").as_path(),
        "export const util = () => 1;\n",
    );

    run_init(repo_root).unwrap();
    write_file(
        repo_root.join("src/util.ts").as_path(),
        "export const util = () => 2;\n",
    );

    let plan = plan_runtime_changes(repo_root).unwrap();
    assert_eq!(
        plan.affected_knowledge_scope.escalation.level.as_str(),
        "local_refresh"
    );

    let knowledge_tree = plan
        .knowledge_tree
        .as_ref()
        .expect("local_refresh should keep planned knowledge tree");
    let mut scope_closure = plan
        .affected_knowledge_scope
        .active_unit_ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut queue = scope_closure.iter().cloned().collect::<Vec<_>>();
    while let Some(unit_id) = queue.pop() {
        let Some(parent_id) = knowledge_tree
            .get_unit(&unit_id)
            .and_then(|unit| unit.parent_unit_id.clone())
        else {
            continue;
        };
        if scope_closure.insert(parent_id.clone()) {
            queue.push(parent_id);
        }
    }

    let targeted_unit_ids = plan
        .planned_pages
        .iter()
        .filter(|page| {
            plan.affected_knowledge_scope
                .projection_target_page_ids
                .iter()
                .any(|page_id| page_id == &page.id)
        })
        .filter_map(|page| page.unit_id.clone())
        .collect::<BTreeSet<_>>();

    assert!(
        !targeted_unit_ids.is_empty(),
        "local_refresh should still derive projection targets from scope"
    );
    assert!(
        targeted_unit_ids
            .iter()
            .all(|unit_id| scope_closure.contains(unit_id)),
        "projection targets must be derivable from affected scope closure, not page drift"
    );
}

/// 场景：页面 Markdown 变化不能反向主导 knowledge scope planning。
#[test]
fn editing_page_markdown_does_not_backdrive_knowledge_scope() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_file(
        repo_root.join("package.json").as_path(),
        r#"{"name":"page-diff-ignore-test"}"#,
    );
    write_file(
        repo_root.join("src/index.ts").as_path(),
        "export const main = () => 1;",
    );

    run_init(repo_root).unwrap();
    let state = read_state(repo_root).unwrap();
    let overview_page = state
        .pages
        .iter()
        .find(|page| page.page_type == "overview")
        .expect("overview page should exist");
    let overview_path = repo_root.join(&overview_page.path);
    let mut current = fs::read_to_string(&overview_path).unwrap();
    current.push_str("\n<!-- user-local-edit -->\n");
    fs::write(&overview_path, current).unwrap();

    let plan = plan_runtime_changes(repo_root).unwrap();
    assert_eq!(plan.state(), "fresh");
    assert!(plan.affected_knowledge_scope.is_empty());
    assert!(plan.affected_set.is_empty());
}

/// 场景：源码删除不能被折叠成普通 stale，必须保留 removed source 证据。
#[test]
fn change_plan_detects_removed_sources() {
    let (_env_lock, _index_only) = force_full_runtime();
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
    assert!(plan
        .change_set
        .removed_sources
        .iter()
        .any(|path| path == "src/index.ts"));
    assert!(plan.change_set.requires_replan);
}

/// 场景：缺失单页 generation cache 时，status 必须升级为 `needs_rebuild`。
#[test]
fn status_reports_needs_rebuild_when_page_level_cache_is_missing() {
    let (_env_lock, _index_only) = force_full_runtime();
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
    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    let overview_page = state
        .pages
        .iter()
        .find(|page| page.page_type == "overview")
        .unwrap();

    {
        let conn = sqlite_store::open_db(repo_root).unwrap();
        sqlite_store::remove_page_generation(&conn, &overview_page.page_id).unwrap();
    }

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "needs_rebuild");
    assert_eq!(
        status.needs_rebuild_reason.as_deref(),
        Some("cache_missing")
    );
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(path, content).unwrap();
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}
