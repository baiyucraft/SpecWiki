//! 这组测试覆盖 query、sync 与 rebuild 在真实 runtime 上的协同行为。
//! 它们保护人工改页同步、Markdown 回退查询和显式重建恢复能力。

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_model::domain::query::QueryRouteTag;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::storage::state_store::{facts_snapshot_ready, index_graph_ready};
use wiki_runtime::workflows::{
    init::run_init, query::run_query, rebuild::run_rebuild, status::run_status, sync::run_sync,
    update::run_update,
};

const DECLARED_RUNTIME_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract kind=policy scope=repo status=active source=manual -->\n",
    "当前仓库必须先写 formal artifact，再谈 query。\n",
    "<!-- wiki:declared:end -->\n"
);

const DECLARED_RUNTIME_CONFLICT_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract-v2 kind=policy scope=repo status=active source=manual -->\n",
    "当前仓库必须先写 formal artifact，且由另一条并行 policy 再次声明。\n",
    "<!-- wiki:declared:end -->\n"
);

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn write_graph_query_repo(repo_root: &Path) {
    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"graph-query-demo","workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/payments/package.json",
        r#"{"name":"payments"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/payments/index.ts",
        "export const paymentsModule = true;\n",
    );
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

fn set_declared_blocks_for_query(repo_root: &Path, declared_blocks: &str) {
    let overview_path = official_overview_path(repo_root);
    mark_first_managed_section_declared(&overview_path);
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let mut new_content = content[..pos]
        .replace(DECLARED_RUNTIME_BLOCK, "")
        .replace(DECLARED_RUNTIME_CONFLICT_BLOCK, "");
    new_content.push_str(declared_blocks);
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();
}

fn mark_first_managed_section_declared(page_path: &Path) {
    let content = fs::read_to_string(page_path).unwrap();
    let marker = "<!-- wiki:managed:start";
    let start = content
        .find(marker)
        .expect("should have managed start marker");
    let end = content[start..].find('\n').unwrap() + start;
    let line = &content[start..end];
    let declared_line = line.replace("owner=derived_managed", "owner=declared_managed");
    if line == declared_line {
        assert!(line.contains("owner=declared_managed"));
        return;
    }

    let mut new_content = content[..start].to_string();
    new_content.push_str(&declared_line);
    new_content.push_str(&content[end..]);
    fs::write(page_path, &new_content).unwrap();
}

fn official_overview_path(repo_root: &Path) -> std::path::PathBuf {
    repo_root.join(".wiki/INDEX.md")
}

/// 场景：人工改页后，sync 必须更新状态层；随后 query 和 rebuild 仍应可用。
#[test]
fn sync_detects_manual_markdown_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = official_overview_path(repo_root);
    fs::write(&overview, "# 项目概述\n\n自定义说明\n").unwrap();

    let result = run_sync(repo_root).unwrap();
    assert_eq!(result.state, "fresh");
    assert!(result
        .synced_pages
        .iter()
        .any(|path| path.ends_with("INDEX.md")));

    let query = run_query(repo_root, "项目概述").unwrap();
    assert_eq!(query.term, "项目概述");
    assert!(!query.matched_pages.is_empty());
    assert!(!query.matches.is_empty());
    assert!(query
        .matches
        .iter()
        .any(|page| page.path.ends_with("INDEX.md")));
    assert!(
        query.provenance_summary.contains("knowledge_hit")
            || query.provenance_summary.contains("page_fallback"),
        "expected knowledge or page fallback route tag, got {}",
        query.provenance_summary
    );

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

    let overview = official_overview_path(repo_root);
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
    assert_eq!(
        serde_json::to_value(&markdown_query).unwrap()["answer"]["answer_mode"],
        "degraded"
    );
    assert_eq!(
        serde_json::to_value(&markdown_query).unwrap()["answer"]["answer_trust"],
        "constrained"
    );
    assert!(markdown_query.matches[0]
        .summary
        .contains("Markdown 内容匹配"));
    assert!(markdown_query
        .answer
        .provenance
        .iter()
        .any(|item| item == "page_fallback"));

    let empty_query = run_query(repo_root, "definitely-no-query-hit").unwrap();
    assert!(empty_query.matches.is_empty());
    assert!(empty_query.matched_pages.is_empty());
    assert!(empty_query.matched_modules.is_empty());
    assert!(empty_query.matched_sources.is_empty());
    assert!(empty_query.matched_relations.is_empty());
    assert_eq!(
        empty_query.answer.answer_mode,
        wiki_runtime::domain::runtime_profile::AnswerMode::Refuse
    );
    assert_eq!(
        empty_query.answer.answer_trust,
        wiki_runtime::domain::runtime_profile::AnswerTrust::Unsupported
    );
    assert!(empty_query.answer.supporting_refs.is_empty());
}

#[test]
fn query_ignores_pages_directory_markdown_fallback() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let ignored_phrase = "only-inside-runtime-surface-outside-pages";
    write_repo_file(
        repo_root,
        ".wiki/pages/ignored.md",
        &format!("# Ignored\n\n{ignored_phrase}\n"),
    );

    let query = run_query(repo_root, ignored_phrase).unwrap();
    assert!(query.matches.is_empty());
    assert!(query.matched_pages.is_empty());
}

#[test]
fn query_keeps_textual_page_fallback_degraded_even_with_graph_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();

    let overview = official_overview_path(repo_root);
    let existing = fs::read_to_string(&overview).unwrap();
    fs::write(
        &overview,
        format!("{existing}\nhandleCheckout textual fallback\n"),
    )
    .unwrap();
    run_sync(repo_root).unwrap();

    let query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(
        query.provenance_summary.contains("page_fallback"),
        "expected textual fallback route tag, got {}",
        query.provenance_summary
    );
    assert_eq!(
        query.answer.answer_mode,
        wiki_runtime::domain::runtime_profile::AnswerMode::Degraded
    );
    assert!(query
        .answer
        .provenance
        .iter()
        .any(|item| item == "page_fallback"));
    assert!(query.results.iter().all(|result| result.route_tag
        != QueryRouteTag::RenderedPageDebugFallback
        || result.ref_kind == wiki_model::domain::query::QueryRefKind::RenderedPage));
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

    assert_eq!(module_page.match_mode, "knowledge_digest");
    assert!(
        module_page
            .provenance
            .iter()
            .any(|item| item.starts_with("knowledge:")),
        "expected knowledge provenance, got {:?}",
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
    assert!(query
        .results
        .iter()
        .filter(|result| result.route_tag.is_index_route())
        .all(|result| {
            result.provenance.state == wiki_model::domain::query::QueryProvenanceState::Stale
        }));
    assert_eq!(payload["recommended_action"], "update");
    assert_eq!(payload["answer"]["answer_mode"], "degraded");
    assert_eq!(payload["answer"]["answer_trust"], "constrained");
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

    assert_eq!(module_page.match_mode, "knowledge_digest");
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
        query.provenance_summary.contains("index_hit"),
        "expected index route tag in provenance summary, got {}",
        query.provenance_summary
    );
    assert!(
        !query.provenance_summary.contains("page_fallback"),
        "symbol-backed page projection must not be treated as markdown fallback: {}",
        query.provenance_summary
    );
    assert_eq!(
        query.answer.answer_mode,
        wiki_runtime::domain::runtime_profile::AnswerMode::Direct
    );
    assert_eq!(
        query.answer.answer_trust,
        wiki_runtime::domain::runtime_profile::AnswerTrust::Grounded
    );
    assert!(query.answer.supporting_refs.iter().any(|supporting_ref| {
        supporting_ref.ref_kind == "symbol" || supporting_ref.ref_kind == "graph_edge"
    }));
    assert!(!query
        .answer
        .provenance
        .iter()
        .any(|item| item == "page_fallback"));
}

#[test]
fn query_fusion_outputs_route_groups_and_results() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "handleCheckout").unwrap();
    assert!(
        query
            .route_groups
            .iter()
            .any(|group| group.route_tag == QueryRouteTag::IndexSymbolHit),
        "expected index symbol route group, got {:#?}",
        query.route_groups
    );
    assert!(
        query
            .route_groups
            .iter()
            .any(|group| group.route_tag == QueryRouteTag::IndexGraphHit),
        "expected index graph route group, got {:#?}",
        query.route_groups
    );
    let module_query = run_query(repo_root, "payments").unwrap();
    let module_group = module_query
        .route_groups
        .iter()
        .find(|group| group.route_tag == QueryRouteTag::IndexModuleHit)
        .unwrap_or_else(|| {
            panic!(
                "expected index module route group, matched modules: {:#?}",
                module_query.matched_modules
            )
        });
    assert_eq!(module_group.returned_count, module_group.results.len());
    assert_eq!(module_group.total_count, module_group.results.len());
    assert!(!module_group.truncated);
    assert_eq!(module_group.results[0].rank, 1);
    assert_eq!(
        module_group.results[0].ref_kind,
        wiki_model::domain::query::QueryRefKind::SourceModule
    );
    assert!(
        query.results.iter().any(|result| {
            result.route_tag == QueryRouteTag::IndexSymbolHit && !result.source_refs.is_empty()
        }),
        "expected query results with source refs, got {:#?}",
        query.results
    );
    let symbol_result = query
        .results
        .iter()
        .find(|result| result.route_tag == QueryRouteTag::IndexSymbolHit)
        .expect("expected symbol result");
    assert!(symbol_result.score.is_some_and(|score| score < 0.0));
    assert_eq!(
        symbol_result.confidence,
        wiki_model::domain::query::QueryConfidence::High,
        "ready symbol confidence must not be derived from raw lower-is-better BM25"
    );
    assert!(
        query.provenance_summary.contains("index_hit"),
        "provenance_summary should remain as a derived summary"
    );
}

#[test]
fn query_projects_symbol_range_and_graph_refs_into_public_results() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);

    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "handleCheckout").unwrap();
    let symbol_result = query
        .results
        .iter()
        .find(|result| result.route_tag == QueryRouteTag::IndexSymbolHit)
        .expect("expected index symbol result");
    assert!(
        symbol_result.score.is_some_and(|score| score < 0.0),
        "fixture should exercise a negative raw BM25 score: {symbol_result:#?}"
    );
    assert_eq!(
        symbol_result.confidence,
        wiki_model::domain::query::QueryConfidence::High,
        "raw BM25 magnitude must not determine symbol confidence"
    );
    assert!(symbol_result.source_refs.iter().any(|source_ref| {
        source_ref.path.as_deref() == Some("src/controller.ts")
            && source_ref.start_line.is_some()
            && source_ref.end_line.is_some()
            && source_ref
                .provenance
                .iter()
                .any(|item| item.starts_with("parser:"))
    }));

    let path_query = run_query(repo_root, "controller.ts").unwrap();
    let path_result = path_query
        .results
        .iter()
        .find(|result| result.route_tag == QueryRouteTag::IndexPathHit)
        .expect("expected index path result");
    assert_ne!(path_result.score, Some(0.75));
    assert!(path_result
        .source_refs
        .iter()
        .any(|source_ref| source_ref.path.as_deref() == Some("src/controller.ts")));

    let graph_result = query
        .results
        .iter()
        .find(|result| result.route_tag == QueryRouteTag::IndexGraphHit)
        .expect("expected index graph result");
    assert!(graph_result.source_refs.iter().any(|source_ref| {
        source_ref.path.as_deref() == Some("src/controller.ts")
            && source_ref
                .provenance
                .iter()
                .any(|item| item == "index:call_trace")
    }));
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
    assert_eq!(status.state, "missing");
    assert_eq!(
        serde_json::to_value(&status).unwrap()["readiness"]["fusion"],
        "blocked"
    );
    assert_eq!(
        serde_json::to_value(&status).unwrap()["recommended_action"],
        "init"
    );

    let query = run_query(repo_root, "handleCheckout").unwrap();
    assert_eq!(
        query.query_trust,
        wiki_runtime::domain::runtime_profile::QueryTrust::StaleButQueryable
    );
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
    assert!(!index_graph_ready(repo_root).unwrap());
    assert!(!query.matches.is_empty());
    assert_eq!(query.runtime_state, "fresh");
    let payload = serde_json::to_value(&query).unwrap();
    assert_eq!(payload["readiness"]["index"], "missing");
    assert_eq!(payload["readiness"]["knowledge"], "ready");
    assert_eq!(payload["readiness"]["projection"], "ready");
    assert_eq!(payload["readiness"]["fusion"], "degraded");
    assert_eq!(payload["readiness"]["restored_level"], "level1");
    assert!(query.matched_modules.is_empty());
    assert!(query.matched_sources.is_empty());
    assert!(query.matched_symbols.is_empty());
    assert!(query.matched_symbol_edges.is_empty());
}

#[test]
fn query_does_not_emit_index_routes_when_index_is_not_ready() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let query = run_query(repo_root, "项目概述").unwrap();
    let payload = serde_json::to_value(&query).unwrap();

    assert_eq!(payload["readiness"]["index"], "missing");
    assert_eq!(payload["query_trust"], "stale_but_queryable");
    assert_eq!(payload["recommended_action"], "rebuild");
    assert!(
        query
            .results
            .iter()
            .all(|result| !result.route_tag.is_index_route()),
        "index route must not be emitted when index is not ready: {:#?}",
        query.results
    );
    assert!(
        query
            .route_groups
            .iter()
            .all(|group| !group.route_tag.is_index_route()),
        "index route group must not be emitted when index is not ready: {:#?}",
        query.route_groups
    );
}

#[test]
fn query_suppresses_index_routes_for_non_ready_graph_phase_states() {
    for phase_status in ["stale", "rebuilding", "blocked"] {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        write_graph_query_repo(repo_root);
        run_init(repo_root).unwrap();

        {
            let conn = sqlite_store::open_db(repo_root).unwrap();
            conn.execute(
                "INSERT OR REPLACE INTO graph_phase_runs
                 (phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics)
                 VALUES ('build_fts', ?1, NULL, NULL, NULL, NULL, '[]')",
                [phase_status],
            )
            .unwrap();
        }

        let query = run_query(repo_root, "handleCheckout").unwrap();
        assert!(
            query
                .results
                .iter()
                .all(|result| !result.route_tag.is_index_route()),
            "index route must not be emitted for {phase_status}: {:#?}",
            query.results
        );
        assert!(
            query
                .route_groups
                .iter()
                .all(|group| !group.route_tag.is_index_route()),
            "index route group must not be emitted for {phase_status}: {:#?}",
            query.route_groups
        );
    }
}

#[test]
fn workflows_write_graph_phase_diagnostics_and_unresolved_refs() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("package.json"), r#"{"name":"phase-demo"}"#).unwrap();
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { missing } from './missing';\n",
            "export function run() {\n",
            "  return missing();\n",
            "}\n",
        ),
    );

    run_init(repo_root).unwrap();

    let phases = sqlite_store::list_graph_phase_runs(repo_root).unwrap();
    assert!(phases
        .iter()
        .any(|phase| phase.phase == wiki_index::symbols::GraphPhase::ResolveImports));
    assert!(phases
        .iter()
        .any(|phase| phase.phase == wiki_index::symbols::GraphPhase::ResolveCalls));

    let unresolved = sqlite_store::list_unresolved_refs(repo_root).unwrap();
    assert!(unresolved.iter().any(|item| {
        item.reference_name == "missing"
            && item.file_id.starts_with("file:")
            && matches!(
                item.resolver_phase,
                wiki_index::symbols::GraphPhase::ResolveImports
                    | wiki_index::symbols::GraphPhase::ResolveCalls
            )
    }));

    let snapshot = sqlite_store::read_current_graph_snapshot(repo_root)
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.source_fingerprint.is_empty(), false);
}

#[test]
fn query_marks_governance_conflict_answer_as_degraded() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"query-governance-answer-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const runtime = true;\n").unwrap();

    run_init(repo_root).unwrap();
    set_declared_blocks_for_query(
        repo_root,
        &format!("{DECLARED_RUNTIME_BLOCK}{DECLARED_RUNTIME_CONFLICT_BLOCK}"),
    );
    run_sync(repo_root).unwrap();

    let query = run_query(repo_root, "formal artifact").unwrap();
    let payload = serde_json::to_value(&query).unwrap();

    assert_eq!(payload["answer"]["answer_mode"], "degraded");
    assert_eq!(payload["answer"]["answer_trust"], "constrained");
    assert_eq!(payload["answer"]["recommended_action"], "review_governance");
    assert!(query
        .answer
        .provenance
        .iter()
        .any(|item| item == "governance_conflict"));
    assert!(query.answer.supporting_refs.iter().any(|supporting_ref| {
        supporting_ref.ref_kind == "health_signal"
            && supporting_ref
                .provenance
                .iter()
                .any(|item| item == "health:governance_conflict")
    }));
}

#[test]
fn query_emits_declared_knowledge_route_for_declared_records() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"query-declared-route-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const runtime = true;\n").unwrap();

    run_init(repo_root).unwrap();
    set_declared_blocks_for_query(repo_root, DECLARED_RUNTIME_BLOCK);
    run_sync(repo_root).unwrap();

    let query = run_query(repo_root, "formal artifact").unwrap();

    assert!(
        query.results.iter().any(
            |result| result.route_tag == QueryRouteTag::KnowledgeDeclaredHit
                && result.ref_kind == wiki_model::domain::query::QueryRefKind::KnowledgeRecord
                && !result.source_refs.is_empty()
        ),
        "declared knowledge route should be emitted from formal declared records: {:#?}",
        query.results
    );
    assert!(
        query
            .route_groups
            .iter()
            .any(|group| group.route_tag == QueryRouteTag::KnowledgeDeclaredHit),
        "declared knowledge route group should be emitted: {:#?}",
        query.route_groups
    );
}

#[test]
fn query_reports_governance_not_enabled_without_blocking_query() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let query = run_query(repo_root, "项目概述").unwrap();
    let payload = serde_json::to_value(&query).unwrap();

    assert_eq!(payload["governance"]["readiness"], "not_enabled");
    assert!(payload.get("governance_readiness").is_none());
    assert!(!query.results.iter().any(|result| {
        matches!(
            result.route_tag,
            QueryRouteTag::GovernanceEvidenceRef | QueryRouteTag::GovernanceSummaryHit
        )
    }));
    assert!(!query.matches.is_empty());
}
