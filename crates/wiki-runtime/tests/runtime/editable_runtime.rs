//! editable runtime 的 workflow 集成测试。
//! 覆盖"手工插入 section → sync → source change → update"后 user content 保留，
//! 以及 rebuild 不会误删同页 user sections。

use std::fs;

use tempfile::tempdir;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::{
    init::run_init, rebuild::run_rebuild, sync::run_sync, update::run_update,
};

/// 辅助：在 init 后的页面中插入 user section。
fn insert_user_section_after_init(repo_root: &std::path::Path) -> String {
    let overview_path = repo_root.join(".wiki/项目概述.md");
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

/// 场景：init → 手工插入 → sync → 验证 user section 被识别。
#[test]
fn sync_recognizes_user_section_after_hand_edit() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    let init = run_init(repo_root).unwrap();
    assert!(init.initialized);

    // 验证 init 输出包含 managed marker
    let overview_content = fs::read_to_string(repo_root.join(".wiki/项目概述.md")).unwrap();
    assert!(overview_content.contains("<!-- wiki:managed:start"));

    // 插入 user section
    insert_user_section_after_init(repo_root);

    // sync 应该检测到变化
    let sync_result = run_sync(repo_root).unwrap();
    assert_eq!(sync_result.state, "fresh");
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.contains("项目概述")));

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
    let overview_content = fs::read_to_string(repo_root.join(".wiki/项目概述.md")).unwrap();
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

    let overview_content = fs::read_to_string(repo_root.join(".wiki/项目概述.md")).unwrap();
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
    let overview_path = repo_root.join(".wiki/项目概述.md");
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
        .any(|p| p.contains("项目概述")));

    // 应该有 managed drift 警告
    assert!(
        sync_result
            .warnings
            .iter()
            .any(|w| w.contains("managed drift")),
        "sync should warn about managed drift, got: {:?}",
        sync_result.warnings
    );
}



