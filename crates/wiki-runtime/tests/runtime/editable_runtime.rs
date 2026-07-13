//! editable runtime 的 workflow 集成测试。
//! 覆盖"手工插入 section → sync → source change → update"后 user content 保留，
//! 以及 rebuild 不会误删同页 user sections。

use std::fs;

use tempfile::tempdir;
use wiki_model::domain::knowledge_artifact::{
    DeclaredKnowledgeRecord, DeclaredKnowledgeRecordStatus, DeclaredKnowledgeRelationKind,
};
use wiki_runtime::storage::knowledge_artifacts::{
    load_conflict_records, load_knowledge_artifacts, restore_runtime_cache_from_artifacts,
};
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::storage::wiki_fs::resolve_page_path;
use wiki_runtime::workflows::{
    init::run_init, rebuild::run_rebuild, sync::run_sync, update::run_update,
};

const DECLARED_RUNTIME_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract kind=policy scope=repo status=active source=manual -->\n",
    "当前仓库必须先写 formal artifact，再谈 query。\n",
    "<!-- wiki:declared:end -->\n"
);

const DECLARED_QUERY_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-query-contract kind=convention scope=repo status=active source=manual -->\n",
    "query 必须先命中 knowledge，再考虑 page fallback。\n",
    "<!-- wiki:declared:end -->\n"
);

const DECLARED_RUNTIME_CONFLICT_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract-v2 kind=policy scope=repo status=active source=manual -->\n",
    "当前仓库必须先写 formal artifact，且由另一条并行 policy 再次声明。\n",
    "<!-- wiki:declared:end -->\n"
);

const DECLARED_LIFECYCLE_BLOCKS: &str = concat!(
    "\n<!-- wiki:declared id=runtime-active kind=policy scope=module:runtime status=active source=manual -->\n",
    "runtime active contract.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=runtime-deprecated kind=policy scope=module:runtime-deprecated status=deprecated deprecated=true source=manual -->\n",
    "runtime deprecated contract.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=runtime-old kind=policy scope=module:runtime status=superseded replaced_by=runtime-active source=manual -->\n",
    "runtime old contract.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=runtime-legacy kind=policy scope=module:runtime-supersedes status=active source=manual -->\n",
    "runtime legacy contract.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=runtime-new kind=policy scope=module:runtime-supersedes status=replaced supersedes=runtime-legacy source=manual -->\n",
    "runtime new contract.\n",
    "<!-- wiki:declared:end -->\n"
);

/// 辅助：在 init 后的页面中插入 user section。
fn insert_user_section_after_init(repo_root: &std::path::Path) -> String {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();

    // 找到第一个 managed end marker 后插入 user section
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");
    let end_of_line = content[pos..].find('\n').unwrap() + pos + 1;

    let mut new_content = content[..end_of_line].to_string();
    new_content.push_str("\n## 手工笔记\n\n这是用户手写的内容，不应被覆盖。\n\n");
    new_content.push_str(&content[end_of_line..]);

    fs::write(&overview_path, &new_content).unwrap();
    new_content
}

/// 辅助：在第一个 managed section 结束前插入合法 declared block。
fn insert_declared_block_into_first_managed_section(repo_root: &std::path::Path) {
    mark_first_managed_section_declared(repo_root);
    set_declared_blocks_in_first_managed_section(repo_root, DECLARED_RUNTIME_BLOCK);
}

fn insert_two_declared_blocks_into_first_managed_section(repo_root: &std::path::Path) {
    mark_first_managed_section_declared(repo_root);
    set_declared_blocks_in_first_managed_section(
        repo_root,
        &format!("{DECLARED_RUNTIME_BLOCK}{DECLARED_QUERY_BLOCK}"),
    );
}

fn insert_conflicting_declared_blocks_into_first_managed_section(repo_root: &std::path::Path) {
    mark_first_managed_section_declared(repo_root);
    set_declared_blocks_in_first_managed_section(
        repo_root,
        &format!("{DECLARED_RUNTIME_BLOCK}{DECLARED_RUNTIME_CONFLICT_BLOCK}"),
    );
}

fn keep_only_second_declared_block(repo_root: &std::path::Path) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let new_content = content.replace(DECLARED_RUNTIME_BLOCK, "");
    fs::write(&overview_path, &new_content).unwrap();
}

fn remove_all_declared_blocks_from_first_managed_section(repo_root: &std::path::Path) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let new_content = content
        .replace(DECLARED_RUNTIME_BLOCK, "")
        .replace(DECLARED_RUNTIME_CONFLICT_BLOCK, "")
        .replace(DECLARED_QUERY_BLOCK, "");
    fs::write(&overview_path, &new_content).unwrap();
}

fn set_declared_blocks_in_first_managed_section(
    repo_root: &std::path::Path,
    declared_blocks: &str,
) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    set_declared_blocks_in_first_managed_section_path(overview_path.as_path(), declared_blocks);
}

fn set_declared_blocks_in_first_managed_section_path(
    page_path: &std::path::Path,
    declared_blocks: &str,
) {
    mark_first_managed_section_declared_path(page_path);
    let content = fs::read_to_string(page_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let mut new_content = content[..pos]
        .replace(DECLARED_RUNTIME_BLOCK, "")
        .replace(DECLARED_RUNTIME_CONFLICT_BLOCK, "")
        .replace(DECLARED_QUERY_BLOCK, "")
        .replace(DECLARED_LIFECYCLE_BLOCKS, "");
    new_content.push_str(declared_blocks);
    new_content.push_str(&content[pos..]);
    fs::write(page_path, &new_content).unwrap();
}

fn set_declared_blocks_in_first_managed_section_without_owner_change(
    repo_root: &std::path::Path,
    declared_blocks: &str,
) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let mut new_content = content[..pos]
        .replace(DECLARED_RUNTIME_BLOCK, "")
        .replace(DECLARED_RUNTIME_CONFLICT_BLOCK, "")
        .replace(DECLARED_QUERY_BLOCK, "");
    new_content.push_str(declared_blocks);
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();
}

fn mark_first_managed_section_declared(repo_root: &std::path::Path) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    mark_first_managed_section_declared_path(overview_path.as_path());
}

fn mark_first_managed_section_declared_path(page_path: &std::path::Path) {
    let content = fs::read_to_string(page_path).unwrap();
    let marker = "<!-- wiki:managed:start";
    let start = content
        .find(marker)
        .expect("should have managed start marker");
    let end = content[start..].find('\n').unwrap() + start;
    let line = &content[start..end];
    let declared_line = line.replace("owner=derived_managed", "owner=declared_managed");
    if line == declared_line {
        assert!(
            line.contains("owner=declared_managed"),
            "first managed section should be derived or already declared"
        );
        return;
    }

    let mut new_content = content[..start].to_string();
    new_content.push_str(&declared_line);
    new_content.push_str(&content[end..]);
    fs::write(page_path, &new_content).unwrap();
}

fn declared_record<'a>(
    records: &'a [DeclaredKnowledgeRecord],
    authoring_id: &str,
) -> &'a DeclaredKnowledgeRecord {
    records
        .iter()
        .find(|record| record.authoring_id == authoring_id)
        .unwrap_or_else(|| panic!("declared record {authoring_id} should exist"))
}

/// 辅助：同时注入合法 declared block 和非法 managed 正文漂移。
fn inject_declared_block_and_managed_drift(repo_root: &std::path::Path) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let injected = concat!(
        "\n这是不允许直接改写的 managed 正文。\n",
        "\n<!-- wiki:declared id=repo-runtime-contract kind=policy scope=repo status=active source=manual -->\n",
        "当前仓库必须先写 formal artifact，再谈 query。\n",
        "<!-- wiki:declared:end -->\n"
    );
    let mut new_content = content[..pos].to_string();
    new_content.push_str(injected);
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();
}

fn remove_declared_blocks_and_inject_managed_drift(repo_root: &std::path::Path) {
    remove_all_declared_blocks_from_first_managed_section(repo_root);
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let mut new_content = content[..pos].to_string();
    new_content.push_str("\n这是不允许直接改写的 managed 正文。\n");
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();
}

/// 场景：init → 手工插入 → sync → 验证 user section 被识别。
#[test]
fn sync_recognizes_user_section_after_hand_edit() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    let init = run_init(repo_root).unwrap();
    assert!(init.initialized);

    // 验证 init 输出包含 managed marker
    let overview_content = fs::read_to_string(repo_root.join(".wiki/INDEX.md")).unwrap();
    assert!(overview_content.contains("<!-- wiki:managed:start"));

    // 插入 user section
    insert_user_section_after_init(repo_root);

    // sync 应该检测到变化
    let sync_result = run_sync(repo_root).unwrap();
    assert_eq!(sync_result.state, "fresh");
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.ends_with("INDEX.md")));

    // 验证 state 中有 user section
    let state = read_state(repo_root).unwrap();
    let overview_page = state
        .pages
        .iter()
        .find(|p| p.page_type == "overview")
        .unwrap();
    let has_user_section = overview_page.sections.iter().any(|s| !s.managed);
    assert!(has_user_section, "sync should create user section state");
}

/// 场景：init → 手工插入 → sync → 修改源码 → update → user section 仍保留。
#[test]
fn update_preserves_user_section_after_source_change() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 插入 user section 并 sync
    insert_user_section_after_init(repo_root);
    run_sync(repo_root).unwrap();

    // 修改源码触发 update
    fs::write(
        repo_root.join("main.rs"),
        "fn main() { println!(\"hello\"); }",
    )
    .unwrap();
    let update_result = run_update(repo_root).unwrap();
    assert_eq!(update_result.state, "fresh");

    // 验证 user section 仍在页面中
    let overview_content = fs::read_to_string(repo_root.join(".wiki/INDEX.md")).unwrap();
    assert!(
        overview_content.contains("手工笔记"),
        "user section heading should be preserved after update"
    );
    assert!(
        overview_content.contains("用户手写的内容"),
        "user section body should be preserved after update"
    );
}

/// 场景：init → 手工插入 → sync → rebuild → user section 仍保留。
#[test]
fn rebuild_preserves_user_section() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 插入 user section 并 sync
    insert_user_section_after_init(repo_root);
    run_sync(repo_root).unwrap();

    // rebuild 应该保留 user section
    let rebuild_result = run_rebuild(repo_root).unwrap();
    assert_eq!(rebuild_result.state, "fresh");

    let overview_content = fs::read_to_string(repo_root.join(".wiki/INDEX.md")).unwrap();
    assert!(
        overview_content.contains("手工笔记"),
        "user section heading should be preserved after rebuild"
    );
    assert!(
        overview_content.contains("用户手写的内容"),
        "user section body should be preserved after rebuild"
    );
}

/// 场景：init 输出的页面必须包含 managed marker。
#[test]
fn init_outputs_managed_markers() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("lib.rs"), "pub fn hello() {}").unwrap();
    run_init(repo_root).unwrap();

    // 检查所有生成的页面都包含 managed marker
    for entry in fs::read_dir(repo_root.join(".wiki")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).unwrap();
            assert!(
                content.contains("<!-- wiki:managed:start"),
                "page {} should contain managed markers",
                path.display()
            );
        }
    }
}

/// 场景：sync 检测 managed drift（用户改了 managed section 正文）。
#[test]
fn sync_detects_managed_drift() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 修改 managed section 的正文（不删除 marker）
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker_start = "<!-- wiki:managed:start";
    let pos = content
        .find(marker_start)
        .expect("should have managed start marker");
    let body_start = content[pos..].find('\n').unwrap() + pos + 1;
    let mut modified = content[..body_start].to_string();
    modified.push_str("我手工改了这段内容\n\n");
    modified.push_str(&content[body_start..]);
    fs::write(&overview_path, &modified).unwrap();

    let sync_result = run_sync(repo_root).unwrap();
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.ends_with("INDEX.md")));

    // 应该有 managed drift 警告
    assert!(
        sync_result
            .warnings
            .iter()
            .any(|w| w.contains("managed drift") || w.contains("正文漂移")),
        "sync should warn about managed drift, got: {:?}",
        sync_result.warnings
    );
}

#[test]
fn sync_classifies_valid_declared_block_as_declared_writeback() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_declared_block_into_first_managed_section(repo_root);

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["recommended_action"],
        "update"
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["stale_unit_ids"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["stale_projection_ids"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
    assert_eq!(
        artifacts.declared_records[0].authoring_id,
        "marker:repo-runtime-contract"
    );
    assert!(artifacts.health_signals.iter().any(|signal| {
        signal.signal_kind
            == wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::DeclaredDerivedDivergence
    }));
}

#[test]
fn sync_rejects_declared_block_inside_derived_section() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    set_declared_blocks_in_first_managed_section_without_owner_change(
        repo_root,
        DECLARED_RUNTIME_BLOCK,
    );

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"], "illegal_drift",
        "warnings = {:?}, sync = {:?}",
        sync_result.warnings, sync_json
    );
    assert!(sync_result
        .warnings
        .iter()
        .any(|warning| warning.contains("不允许 declared writeback")));

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert!(artifacts.declared_records.is_empty());
}

#[test]
fn sync_generates_conflict_artifact_for_parallel_active_declared() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_conflicting_declared_blocks_into_first_managed_section(repo_root);

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();
    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );

    let conflicts = load_conflict_records(repo_root).unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(
        conflicts[0].conflict_kind.as_str(),
        "parallel_active_declared"
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert!(artifacts.health_signals.iter().any(|signal| {
        signal.signal_kind
            == wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::GovernanceConflict
    }));
}

#[test]
fn sync_classifies_user_only_edit_as_metadata_only() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_user_section_after_init(repo_root);

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"], "metadata_only",
        "warnings = {:?}, sync = {:?}",
        sync_result.warnings, sync_json
    );
    assert_eq!(sync_json["page_outcomes"][0]["recommended_action"], "none");
}

#[test]
fn sync_keeps_user_only_edit_as_metadata_only_when_declared_snapshot_unchanged() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_declared_block_into_first_managed_section(repo_root);
    let initial_sync = run_sync(repo_root).unwrap();
    assert_eq!(
        serde_json::to_value(&initial_sync).unwrap()["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );

    let overview_path = repo_root.join(".wiki/INDEX.md");
    let before_user_edit = fs::read_to_string(&overview_path).unwrap();
    insert_user_section_after_init(repo_root);
    let after_user_edit = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let before_prefix = &before_user_edit[..before_user_edit.find(marker).unwrap()];
    let after_prefix = &after_user_edit[..after_user_edit.find(marker).unwrap()];
    assert_eq!(
        before_prefix, after_prefix,
        "user-only edit should not touch managed prefix"
    );

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "metadata_only"
    );
    assert_eq!(sync_json["page_outcomes"][0]["recommended_action"], "none");

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
}

#[test]
fn sync_prioritizes_illegal_drift_over_declared_writeback() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    inject_declared_block_and_managed_drift(repo_root);

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "illegal_drift"
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["recommended_action"],
        "rebuild"
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert!(artifacts.declared_records.is_empty());
    assert!(artifacts.health_signals.iter().any(|signal| {
        signal.signal_kind
            == wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::IllegalDrift
    }));
}

#[test]
fn sync_keeps_page_level_atomicity_while_unrelated_page_commits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    let state = read_state(repo_root).unwrap();
    let unrelated_page = state
        .pages
        .iter()
        .find(|page| page.path != "INDEX.md")
        .expect("fixture should generate a second page");
    let unrelated_page_id = unrelated_page.page_id.clone();
    let unrelated_path = unrelated_page.path.clone();

    inject_declared_block_and_managed_drift(repo_root);
    let unrelated_page_path = resolve_page_path(repo_root, &unrelated_path);
    set_declared_blocks_in_first_managed_section_path(
        unrelated_page_path.as_path(),
        DECLARED_QUERY_BLOCK,
    );

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();
    let outcomes = sync_json["page_outcomes"].as_array().unwrap();
    assert_eq!(outcomes.len(), 2, "sync = {sync_json:?}");

    let overview = outcomes
        .iter()
        .find(|outcome| {
            outcome["path"]
                .as_str()
                .map(|path| path.ends_with("INDEX.md"))
                .unwrap_or(false)
        })
        .expect("overview outcome should exist");
    let unrelated = outcomes
        .iter()
        .find(|outcome| outcome["path"] == unrelated_path)
        .expect("unrelated outcome should exist");

    assert_eq!(overview["result_kind"], "illegal_drift");
    assert_eq!(unrelated["result_kind"], "declared_writeback");
    assert_eq!(unrelated["page_id"], unrelated_page_id);

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
    assert_eq!(
        artifacts.declared_records[0].authoring_id,
        "marker:repo-query-contract"
    );
    assert_eq!(artifacts.declared_records[0].page_id, unrelated_page_id);
}

#[test]
fn sync_keeps_previous_conflict_snapshot_when_illegal_drift_happens_after_conflict() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_conflicting_declared_blocks_into_first_managed_section(repo_root);
    run_sync(repo_root).unwrap();
    assert_eq!(load_conflict_records(repo_root).unwrap().len(), 1);

    inject_declared_block_and_managed_drift(repo_root);
    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "illegal_drift"
    );
    assert_eq!(load_conflict_records(repo_root).unwrap().len(), 1);
}

#[test]
fn sync_clears_conflict_artifact_when_conflicting_declared_removed() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_conflicting_declared_blocks_into_first_managed_section(repo_root);
    run_sync(repo_root).unwrap();
    assert_eq!(load_conflict_records(repo_root).unwrap().len(), 1);

    insert_declared_block_into_first_managed_section(repo_root);
    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );
    assert!(load_conflict_records(repo_root).unwrap().is_empty());
}

#[test]
fn sync_prunes_removed_declared_records_after_full_delete() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_declared_block_into_first_managed_section(repo_root);
    let declared_owner_overview = fs::read_to_string(repo_root.join(".wiki/INDEX.md")).unwrap();
    run_sync(repo_root).unwrap();

    remove_all_declared_blocks_from_first_managed_section(repo_root);
    let removed_overview = fs::read_to_string(repo_root.join(".wiki/INDEX.md")).unwrap();
    assert_eq!(
        removed_overview, declared_owner_overview.replace(DECLARED_RUNTIME_BLOCK, ""),
        "removing declared blocks should preserve declared section ownership and restore section body"
    );
    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"], "declared_writeback",
        "warnings = {:?}, sync = {:?}",
        sync_result.warnings, sync_json
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["recommended_action"],
        "update"
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["declared_record_ids"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert!(artifacts.declared_records.is_empty());
}

#[test]
fn sync_prunes_removed_declared_records_after_partial_delete() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_two_declared_blocks_into_first_managed_section(repo_root);
    run_sync(repo_root).unwrap();

    keep_only_second_declared_block(repo_root);
    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"], "declared_writeback",
        "warnings = {:?}, sync = {:?}",
        sync_result.warnings, sync_json
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["declared_record_ids"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
    assert_eq!(
        artifacts.declared_records[0].authoring_id,
        "marker:repo-query-contract"
    );
}

#[test]
fn sync_keeps_previous_declared_snapshot_when_delete_is_illegal_drift() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    insert_declared_block_into_first_managed_section(repo_root);
    run_sync(repo_root).unwrap();

    remove_declared_blocks_and_inject_managed_drift(repo_root);
    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "illegal_drift"
    );
    assert!(sync_json["page_outcomes"][0]["declared_record_ids"]
        .as_array()
        .map(|items| items.is_empty())
        .unwrap_or(true));
    assert!(sync_json["page_outcomes"][0]["stale_unit_ids"]
        .as_array()
        .map(|items| items.is_empty())
        .unwrap_or(true));
    assert!(sync_json["page_outcomes"][0]["stale_projection_ids"]
        .as_array()
        .map(|items| items.is_empty())
        .unwrap_or(true));

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
    assert_eq!(
        artifacts.declared_records[0].authoring_id,
        "marker:repo-runtime-contract"
    );
}

#[test]
fn sync_rejects_same_scope_duplicate_records_without_explicit_ids() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    mark_first_managed_section_declared(repo_root);

    let overview_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let duplicate_blocks = concat!(
        "\n<!-- wiki:declared kind=policy scope=repo status=active source=manual -->\n",
        "第一条规则。\n",
        "<!-- wiki:declared:end -->\n",
        "\n<!-- wiki:declared kind=policy scope=repo status=active source=manual -->\n",
        "第二条规则。\n",
        "<!-- wiki:declared:end -->\n"
    );
    let mut new_content = content[..pos].to_string();
    new_content.push_str(duplicate_blocks);
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "illegal_drift"
    );
    assert!(sync_result
        .warnings
        .iter()
        .any(|warning| warning.contains("authoring identity 冲突")));
    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert!(artifacts.declared_records.is_empty());
}

#[test]
fn sync_materializes_declared_lifecycle_relations_from_managed_blocks() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    mark_first_managed_section_declared(repo_root);
    set_declared_blocks_in_first_managed_section(repo_root, DECLARED_LIFECYCLE_BLOCKS);

    let sync_result = run_sync(repo_root).unwrap();
    let sync_json = serde_json::to_value(&sync_result).unwrap();

    assert_eq!(
        sync_json["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );
    assert_eq!(
        sync_json["page_outcomes"][0]["recommended_action"],
        "update"
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 5);

    let active = declared_record(&artifacts.declared_records, "marker:runtime-active");
    assert_eq!(active.status, DeclaredKnowledgeRecordStatus::Active);
    assert!(active.relations.is_empty());
    assert_eq!(active.scope.canonical_key(), "module:runtime");

    let deprecated = declared_record(&artifacts.declared_records, "marker:runtime-deprecated");
    assert_eq!(deprecated.status, DeclaredKnowledgeRecordStatus::Deprecated);
    assert_eq!(deprecated.relations.len(), 1);
    assert_eq!(
        deprecated.relations[0].relation_kind,
        DeclaredKnowledgeRelationKind::Deprecated
    );
    assert!(deprecated.relations[0].target_record_ref.is_none());

    let old = declared_record(&artifacts.declared_records, "marker:runtime-old");
    assert_eq!(old.status, DeclaredKnowledgeRecordStatus::Superseded);
    assert_eq!(old.relations.len(), 1);
    assert_eq!(
        old.relations[0].relation_kind,
        DeclaredKnowledgeRelationKind::ReplacedBy
    );
    assert_eq!(
        old.relations[0].target_record_ref.as_deref(),
        Some("marker:runtime-active")
    );

    let legacy = declared_record(&artifacts.declared_records, "marker:runtime-legacy");
    assert_eq!(legacy.status, DeclaredKnowledgeRecordStatus::Active);
    assert!(legacy.relations.is_empty());

    let new = declared_record(&artifacts.declared_records, "marker:runtime-new");
    assert_eq!(new.status, DeclaredKnowledgeRecordStatus::Replaced);
    assert_eq!(new.relations.len(), 1);
    assert_eq!(
        new.relations[0].relation_kind,
        DeclaredKnowledgeRelationKind::Supersedes
    );
    assert_eq!(
        new.relations[0].target_record_ref.as_deref(),
        Some("marker:runtime-legacy")
    );

    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();
    assert!(
        restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );

    let restored = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(restored.declared_records.len(), 5);
    let restored_old = declared_record(&restored.declared_records, "marker:runtime-old");
    assert_eq!(
        restored_old.status,
        DeclaredKnowledgeRecordStatus::Superseded
    );
    assert_eq!(
        restored_old.relations[0].target_record_ref.as_deref(),
        Some("marker:runtime-active")
    );
    let restored_new = declared_record(&restored.declared_records, "marker:runtime-new");
    assert_eq!(restored_new.status, DeclaredKnowledgeRecordStatus::Replaced);
    assert_eq!(
        restored_new.relations[0].target_record_ref.as_deref(),
        Some("marker:runtime-legacy")
    );
}
