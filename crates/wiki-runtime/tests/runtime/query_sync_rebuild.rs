//! 这组测试覆盖 query、sync 与 rebuild 在真实 runtime 上的协同行为。
//! 它们保护人工改页同步、Markdown 回退查询和显式重建恢复能力。

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::facts_snapshot_ready;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::{
    init::run_init, query::run_query, rebuild::run_rebuild, status::run_status, sync::run_sync,
    update::run_update,
};

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn write_graph_query_repo(repo_root: &Path) {
    write_repo_file(repo_root, "package.json", r#"{"name":"graph-query-demo"}"#);
    write_repo_file(
        repo_root,
        "src/shared.ts",
        "export function finalizePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { finalizePayment } from \"./shared\";\n",
            "export function runPayment() {\n",
            "  return finalizePayment();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "src/controller.ts",
        concat!(
            "import { runPayment } from \"./service\";\n",
            "export function handleCheckout() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    );
}

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
        .all(|page| page.match_mode == "fallback_markdown"));

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
    assert_eq!(
        serde_json::to_value(&markdown_query).unwrap()["query_mode"],
        "page_fallback"
    );
    assert_eq!(
        serde_json::to_value(&markdown_query).unwrap()["query_trust"],
        "stale_but_queryable"
    );
    assert_eq!(
        serde_json::to_value(&markdown_query).unwrap()["recommended_action"],
        "rebuild"
    );
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
    let repo_root = fixture.path().join("payments-demo");
    fs::create_dir_all(&repo_root).unwrap();

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

    run_init(&repo_root).unwrap();

    let query = run_query(&repo_root, "payments").unwrap();
    assert_eq!(serde_json::to_value(&query).unwrap()["query_mode"], "mixed");
    assert_eq!(
        serde_json::to_value(&query).unwrap()["query_trust"],
        "ready"
    );
    let module_page = query
        .matches
        .iter()
        .find(|page| {
            page.item_type == "module"
                && page
                    .source_files
                    .iter()
                    .any(|path| path == "packages/payments/src/index.ts")
        })
        .expect("payments module page should be matched");

    assert_eq!(module_page.match_mode, "fallback_markdown");
    assert!(
        module_page
            .provenance
            .iter()
            .any(|item| item.starts_with("page-fallback:")),
        "expected page fallback provenance, got {:?}",
        module_page.provenance
    );
}

#[test]
fn query_marks_stale_runtime_as_queryable_but_recommends_update() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();
    write_repo_file(
        repo_root,
        "src/shared.ts",
        "export function finalizePayment() { return 'changed'; }\n",
    );

    let query = run_query(repo_root, "finalizePayment").unwrap();
    let payload = serde_json::to_value(&query).unwrap();

    assert_eq!(payload["runtime_state"], "stale");
    assert_eq!(payload["query_trust"], "stale_but_queryable");
    assert_eq!(payload["recommended_action"], "update");
}

/// 场景：FTS 索引为空时，query 仍应回退到结构化命中。
#[test]
fn query_falls_back_when_fts_index_is_empty() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path().join("payments-demo");
    fs::create_dir_all(&repo_root).unwrap();

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

    run_init(&repo_root).unwrap();

    {
        let conn = sqlite_store::open_db(&repo_root).unwrap();
        conn.execute("DELETE FROM wiki_pages_fts", []).unwrap();
    }

    let query = run_query(&repo_root, "payments").unwrap();
    let module_page = query
        .matches
        .iter()
        .find(|page| {
            page.item_type == "module"
                && page
                    .source_files
                    .iter()
                    .any(|path| path == "packages/payments/src/index.ts")
        })
        .expect("payments module page should still be matched");

    assert_eq!(module_page.match_mode, "fallback_markdown");
    assert!(
        !module_page
            .reasons
            .iter()
            .any(|reason| reason.starts_with("FTS ")),
        "FTS reasons should disappear when index is empty, got {:?}",
        module_page.reasons
    );
}

#[test]
fn query_returns_graph_context_for_symbol_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(
        !query.matched_symbols.is_empty(),
        "expected symbol matches for handleCheckout"
    );
    assert!(
        query
            .matched_symbol_edges
            .iter()
            .any(|edge| edge.edge_type == "CALLS"
                && edge
                    .provenance
                    .iter()
                    .any(|item| item == "index:call_trace")),
        "expected graph CALLS edges, got {:#?}",
        query.matched_symbol_edges
    );
    assert!(
        query.matched_symbol_edges.iter().any(|edge| {
            edge.hop_distance >= 2
                && edge.traversal_modes.iter().any(|mode| mode == "outbound")
                && edge.reason == "import-resolved"
        }),
        "expected multi-hop outbound call-chain expansion, got {:#?}",
        query.matched_symbol_edges
    );
    assert!(
        query.provenance_summary.contains("扩展"),
        "expected graph expansion in provenance summary, got {}",
        query.provenance_summary
    );
}

#[test]
fn query_expands_inbound_impact_range_for_terminal_symbol() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "finalizePayment").unwrap();
    assert!(
        query.matched_symbol_edges.iter().any(|edge| {
            edge.hop_distance >= 2
                && edge.traversal_modes.iter().any(|mode| mode == "inbound")
                && edge.reason == "import-resolved"
        }),
        "expected inbound impact expansion for finalizePayment, got {:#?}",
        query.matched_symbol_edges
    );
}

#[test]
fn query_falls_back_without_graph_rows_and_rebuild_restores_them() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();
    {
        let conn = sqlite_store::open_db(repo_root).unwrap();
        conn.execute("DELETE FROM process_steps", []).unwrap();
        conn.execute("DELETE FROM processes", []).unwrap();
        conn.execute("DELETE FROM community_members", []).unwrap();
        conn.execute("DELETE FROM communities", []).unwrap();
        conn.execute("DELETE FROM edges", []).unwrap();
    }

    let fallback_query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(
        !fallback_query.matched_symbols.is_empty(),
        "symbol query should still work without graph rows"
    );
    assert!(fallback_query.matched_symbol_edges.is_empty());

    run_rebuild(repo_root).unwrap();
    let restored_query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(!restored_query.matched_symbol_edges.is_empty());
}

#[test]
fn update_recomputes_process_labels_and_workflow_page() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();
    write_repo_file(
        repo_root,
        "src/controller.ts",
        concat!(
            "import { runPayment } from \"./service\";\n",
            "export function handleBilling() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    );

    run_update(repo_root).unwrap();

    let processes = sqlite_store::list_processes(repo_root).unwrap();
    assert!(
        processes
            .iter()
            .any(|process| process.label.contains("handleBilling")),
        "expected updated process labels, got {:#?}",
        processes
    );
    assert!(
        !processes
            .iter()
            .any(|process| process.label.contains("handleCheckout")),
        "expected stale process labels to disappear, got {:#?}",
        processes
    );

    let state = wiki_runtime::storage::state_store::read_state(repo_root).unwrap();
    assert!(
        state.pages.len() >= 2,
        "update should produce pages after process change"
    );
}

/// 场景：facts snapshot 已提交后，即使 metadata/state 缺失，status 与 query 仍应可工作。
#[test]
fn query_stays_available_when_facts_snapshot_outlives_downstream_state() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();
    fs::remove_file(repo_root.join(".wiki/wiki.metadata.json")).unwrap();
    {
        let conn = sqlite_store::open_db(repo_root).unwrap();
        conn.execute("DELETE FROM wiki_pages", []).unwrap();
    }

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "index_only");
    assert!(status.facts_ready);
    assert_eq!(
        serde_json::to_value(&status).unwrap()["query_readiness"],
        "ready"
    );

    let query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(!query.matched_symbols.is_empty());
    assert!(!query.matched_symbol_edges.is_empty());
}

/// 场景：index 未就绪必须返回显式错误，而不是伪装成空命中。
#[test]
fn query_distinguishes_index_not_ready_from_empty_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);

    let error = run_query(repo_root, "demo").expect_err("query should fail before init");
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(error.to_string().contains("index not ready"));

    run_init(repo_root).unwrap();
    let empty_query = run_query(repo_root, "definitely-no-query-hit").unwrap();
    assert!(empty_query.matched_symbols.is_empty());
    assert!(empty_query.matched_sources.is_empty());
    assert!(empty_query.matched_modules.is_empty());
    assert!(empty_query.matched_symbol_edges.is_empty());
}

/// 场景：`.cache` 缺失时，query 应先恢复本地 runtime，再继续消费 page fallback。
#[test]
fn query_restores_runtime_cache_from_formal_artifacts() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();
    assert!(!facts_snapshot_ready(repo_root).unwrap());

    let query = run_query(repo_root, "项目概述").unwrap();
    assert!(repo_root.join(".wiki/.cache/wiki-cache.db").exists());
    assert!(facts_snapshot_ready(repo_root).unwrap());
    assert!(!query.matches.is_empty());
    assert_eq!(query.runtime_state, "fresh");
}
