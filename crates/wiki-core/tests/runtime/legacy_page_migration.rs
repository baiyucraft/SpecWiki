//! legacy 页面回归测试。
//! 覆盖无 marker 页面迁移、标题完整时成功迁移、标题损坏时 warning 边界。

use std::fs;

use tempfile::tempdir;
use wiki_core::storage::state_store::read_state;
use wiki_core::workflows::{init::run_init, sync::run_sync};

/// 辅助：把 init 生成的带 marker 页面退化为 legacy 格式（去掉所有 marker）。
fn strip_markers_from_page(repo_root: &std::path::Path, filename: &str) -> String {
    let page_path = repo_root.join(".wiki").join(filename);
    let content = fs::read_to_string(&page_path).unwrap();

    let mut legacy_lines = Vec::new();
    for line in content.lines() {
        if line.trim().starts_with("<!-- wiki:managed:") {
            continue;
        }
        legacy_lines.push(line);
    }

    let legacy_content = legacy_lines.join("\n");
    fs::write(&page_path, &legacy_content).unwrap();
    legacy_content
}

/// 场景：legacy 页面保留完整已知标题时，sync 应成功迁移为 managed/user sections。
#[test]
fn sync_migrates_legacy_page_with_complete_titles() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 退化为 legacy 格式
    strip_markers_from_page(repo_root, "项目概述.md");

    // sync 应该能识别 legacy 页面并迁移
    let sync_result = run_sync(repo_root).unwrap();
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.contains("项目概述")));

    // 验证 state 中有 managed sections（从 legacy 标题恢复）
    let state = read_state(repo_root).unwrap();
    let overview = state
        .pages
        .iter()
        .find(|p| p.page_type == "overview")
        .unwrap();
    let managed_count = overview.sections.iter().filter(|s| s.managed).count();
    assert!(
        managed_count > 0,
        "legacy migration should produce managed sections"
    );
}

/// 场景：legacy 页面中插入了用户手写区段，sync 应把它识别为 user section。
#[test]
fn sync_migrates_legacy_page_with_user_section() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 退化为 legacy 格式
    let legacy_content = strip_markers_from_page(repo_root, "项目概述.md");

    // 在已知标题之间插入用户区段
    let page_path = repo_root.join(".wiki/项目概述.md");
    let modified = legacy_content.replace(
        "## 项目事实",
        "## 手工笔记\n\n用户手写内容。\n\n## 项目事实",
    );
    fs::write(&page_path, &modified).unwrap();

    let sync_result = run_sync(repo_root).unwrap();
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.contains("项目概述")));

    // 验证 state 中同时有 managed 和 user sections
    let state = read_state(repo_root).unwrap();
    let overview = state
        .pages
        .iter()
        .find(|p| p.page_type == "overview")
        .unwrap();
    let managed_count = overview.sections.iter().filter(|s| s.managed).count();
    let user_count = overview.sections.iter().filter(|s| !s.managed).count();
    assert!(
        managed_count > 0,
        "should have managed sections from known titles"
    );
    assert!(
        user_count > 0,
        "should have user section from unknown title"
    );
}

/// 场景：legacy 页面标题被破坏（无法匹配已知标题），sync 应给出 warning。
#[test]
fn sync_warns_on_legacy_page_with_broken_titles() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 把页面内容替换为完全不匹配的标题
    let page_path = repo_root.join(".wiki/项目概述.md");
    let broken_content = "# 项目概述\n\n## 完全不认识的标题\n\n一些内容。\n";
    fs::write(&page_path, broken_content).unwrap();

    let sync_result = run_sync(repo_root).unwrap();
    assert!(sync_result
        .synced_pages
        .iter()
        .any(|p| p.contains("项目概述")));

    // 应该有 warning 说明无法识别 managed section
    assert!(
        sync_result.warnings.iter().any(|w| w.contains("无法识别")),
        "sync should warn about unrecognizable legacy page, got: {:?}",
        sync_result.warnings
    );
}

/// 场景：legacy 页面未发生变化时，sync 不应触发迁移。
#[test]
fn sync_skips_unchanged_legacy_page() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    // 第一次 sync（无变化）
    let sync_result = run_sync(repo_root).unwrap();
    assert!(
        sync_result.synced_pages.is_empty(),
        "no changes should mean no synced pages"
    );
}
