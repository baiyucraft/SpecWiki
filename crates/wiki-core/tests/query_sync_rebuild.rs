//! 这组测试覆盖 query、sync 与 rebuild 在真实 runtime 上的协同行为。
//! 它们保护人工改页同步、Markdown 回退查询和显式重建恢复能力。

use std::fs;

use tempfile::tempdir;
use wiki_core::storage::sqlite_store;
use wiki_core::storage::state_store::read_state;
use wiki_core::workflows::{
    init::run_init, query::run_query, rebuild::run_rebuild, sync::run_sync,
};

/// 场景：人工改页后，sync 必须更新状态层；随后 query 和 rebuild 仍应可用。
#[test]
fn sync_detects_manual_markdown_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    fs::write(&overview, "# 项目概述\n\n自定义说明\n").unwrap();

    let result = run_sync(repo_root).unwrap();
    assert_eq!(result.state, "fresh");
    assert!(result
        .synced_pages
        .iter()
        .any(|path| path.ends_with("项目概述.md")));

    let query = run_query(repo_root, "项目概述").unwrap();
    assert_eq!(query.term, "项目概述");
    assert!(!query.matched_pages.is_empty());
    assert!(!query.matches.is_empty());
    assert!(query
        .matches
        .iter()
        .any(|page| page.reasons.iter().any(|reason| reason == "页面标题匹配")));
    assert!(query
        .matches
        .iter()
        .all(|page| matches!(page.match_mode.as_str(), "structure" | "fts+structure")));

    let rebuild = run_rebuild(repo_root).unwrap();
    assert_eq!(rebuild.state, "fresh");
    assert!(!rebuild.updated_pages.is_empty());
}

/// 场景：没有结构命中时，query 必须回退到 Markdown 内容匹配，并对空查询返回空结果。
#[test]
fn query_falls_back_to_markdown_and_returns_empty_result() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    let custom_phrase = "仅在 Markdown 中出现的唯一短语";
    fs::write(&overview, format!("# 项目概述\n\n{custom_phrase}\n")).unwrap();
    run_sync(repo_root).unwrap();

    let markdown_query = run_query(repo_root, custom_phrase).unwrap();
    assert_eq!(markdown_query.matches.len(), 1);
    assert_eq!(markdown_query.matches[0].match_mode, "fallback_markdown");
    assert!(markdown_query.matches[0]
        .summary
        .contains("Markdown 内容匹配"));

    let empty_query = run_query(repo_root, "definitely-no-query-hit").unwrap();
    assert!(empty_query.matches.is_empty());
    assert!(empty_query.matched_pages.is_empty());
    assert!(empty_query.matched_modules.is_empty());
    assert!(empty_query.matched_sources.is_empty());
    assert!(empty_query.matched_relations.is_empty());
}

/// 场景：显式 rebuild 必须能补回缺失的 page-level cache。
#[test]
fn rebuild_restores_missing_page_caches() {
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

    let rebuild = run_rebuild(repo_root).unwrap();
    assert_eq!(rebuild.state, "fresh");
    {
        let conn = sqlite_store::open_db_readonly(repo_root).unwrap();
        assert!(sqlite_store::page_context_exists(&conn, &overview_page.page_id).unwrap());
        assert!(sqlite_store::page_generation_exists(&conn, &overview_page.page_id).unwrap());
    }
}

/// 场景：query 应把 FTS/BM25 页面命中和结构化命中合并成单个结果。
#[test]
fn query_merges_fts_and_structural_page_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"demo","workspaces":["packages/*"]}"#,
    )
    .unwrap();
    fs::create_dir_all(repo_root.join("packages/payments/src")).unwrap();
    fs::write(
        repo_root.join("packages/payments/package.json"),
        r#"{"name":"payments"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/payments/src/index.ts"),
        "export function settlePayment() {}\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "payments").unwrap();
    let module_page = query
        .matches
        .iter()
        .find(|page| page.path.contains("payments.md"))
        .expect("payments module page should be matched");

    assert_eq!(module_page.match_mode, "fts+structure");
    assert!(
        module_page
            .reasons
            .iter()
            .any(|reason| reason.starts_with("FTS ")),
        "expected FTS reason, got {:?}",
        module_page.reasons
    );
    assert!(
        module_page
            .provenance
            .iter()
            .any(|item| item.starts_with("fts:bm25:")),
        "expected FTS provenance, got {:?}",
        module_page.provenance
    );
}

/// 场景：FTS 索引为空时，query 仍应回退到结构化命中。
#[test]
fn query_falls_back_when_fts_index_is_empty() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"demo","workspaces":["packages/*"]}"#,
    )
    .unwrap();
    fs::create_dir_all(repo_root.join("packages/payments/src")).unwrap();
    fs::write(
        repo_root.join("packages/payments/package.json"),
        r#"{"name":"payments"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("packages/payments/src/index.ts"),
        "export function settlePayment() {}\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();

    {
        let conn = sqlite_store::open_db(repo_root).unwrap();
        conn.execute("DELETE FROM wiki_pages_fts", []).unwrap();
    }

    let query = run_query(repo_root, "payments").unwrap();
    let module_page = query
        .matches
        .iter()
        .find(|page| page.path.contains("payments.md"))
        .expect("payments module page should still be matched");

    assert_eq!(module_page.match_mode, "structure");
    assert!(
        !module_page
            .reasons
            .iter()
            .any(|reason| reason.starts_with("FTS ")),
        "FTS reasons should disappear when index is empty, got {:?}",
        module_page.reasons
    );
}
