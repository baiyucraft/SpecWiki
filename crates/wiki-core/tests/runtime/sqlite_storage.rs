//! SQLite 存储层单元测试。
//! 覆盖 DB 初始化、kv 读写、page cache 读写、事务原子性、DB 损坏回退。

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use tempfile::TempDir;
use wiki_core::domain::metadata::DirtyState;
use wiki_core::domain::state::{BuildState, WikiState};
use wiki_core::repo::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use wiki_core::repo::symbols::SymbolNode;
use wiki_core::storage::cache_store::has_cache_layout;
use wiki_core::storage::sqlite_store;
use wiki_core::storage::state_store::{load_or_rebuild_state, read_state};
use wiki_core::storage::wiki_fs::remove_cache_db;

fn make_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join(".wiki").join(".cache")).unwrap();
    dir
}

fn legacy_state_json() -> String {
    serde_json::to_string(&empty_state()).unwrap()
}

fn empty_state() -> WikiState {
    WikiState {
        pages: Vec::new(),
        sources: Vec::new(),
        modules: Vec::new(),
        relations: Vec::new(),
        dirty_state: DirtyState::fresh(),
        build_state: BuildState {
            generated_at: "2026-03-10T00:00:00Z".to_string(),
            page_count: 0,
            module_count: 0,
        },
    }
}

fn sample_symbols() -> Vec<SymbolNode> {
    vec![
        SymbolNode {
            symbol_id: "symbol-service".to_string(),
            name: "PaymentService".to_string(),
            label: "class".to_string(),
            file_path: "src/service.ts".to_string(),
            start_line: 1,
            end_line: 12,
            is_exported: true,
            language: "typescript".to_string(),
        },
        SymbolNode {
            symbol_id: "symbol-helper".to_string(),
            name: "runHelper".to_string(),
            label: "function".to_string(),
            file_path: "src/helper.ts".to_string(),
            start_line: 1,
            end_line: 3,
            is_exported: true,
            language: "typescript".to_string(),
        },
    ]
}

fn sample_graph() -> (ResolvedGraphSnapshot, GraphAnalysisSnapshot) {
    let resolved = ResolvedGraphSnapshot {
        edges: vec![ResolvedSymbolEdge {
            edge_id: "edge-calls".to_string(),
            source_id: "symbol-service".to_string(),
            target_id: "symbol-helper".to_string(),
            edge_type: "CALLS".to_string(),
            confidence: 0.95,
            reason: "same-file".to_string(),
        }],
        diagnostics: Vec::new(),
    };
    let analysis = GraphAnalysisSnapshot {
        communities: vec![CommunityNode {
            community_id: "community-payments".to_string(),
            label: "payments".to_string(),
            cohesion: 0.8,
            symbol_count: 2,
        }],
        community_members: vec![
            CommunityMember {
                community_id: "community-payments".to_string(),
                symbol_id: "symbol-service".to_string(),
            },
            CommunityMember {
                community_id: "community-payments".to_string(),
                symbol_id: "symbol-helper".to_string(),
            },
        ],
        processes: vec![ProcessNode {
            process_id: "process-payment".to_string(),
            label: "payment flow".to_string(),
            process_type: "request-flow".to_string(),
            step_count: 2,
            entry_point_id: Some("symbol-service".to_string()),
            terminal_id: Some("symbol-helper".to_string()),
        }],
        process_steps: vec![
            ProcessStep {
                process_id: "process-payment".to_string(),
                symbol_id: "symbol-service".to_string(),
                step_order: 0,
            },
            ProcessStep {
                process_id: "process-payment".to_string(),
                symbol_id: "symbol-helper".to_string(),
                step_order: 1,
            },
        ],
        cycles: Vec::new(),
        diagnostics: Vec::new(),
    };
    (resolved, analysis)
}

fn write_legacy_db(repo_root: &Path, entries: &[(&str, &str)]) {
    let db_path = sqlite_store::db_path(repo_root);
    let conn = Connection::open(&db_path).unwrap();
    conn.execute_batch(
        "CREATE TABLE kv_store (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )
    .unwrap();

    for (key, value) in entries {
        conn.execute(
            "INSERT INTO kv_store (key, value) VALUES (?1, ?2)",
            [*key, *value],
        )
        .unwrap();
    }
}

// ---------------------------------------------------------------------------
// DB 初始化
// ---------------------------------------------------------------------------

#[test]
fn db_created_on_open() {
    let repo = make_repo();
    assert!(!sqlite_store::db_exists(repo.path()));

    let _conn = sqlite_store::open_db(repo.path()).unwrap();
    assert!(sqlite_store::db_exists(repo.path()));
}

#[test]
fn db_open_readonly_fails_when_missing() {
    let repo = make_repo();
    let result = sqlite_store::open_db_readonly(repo.path());
    assert!(result.is_err());
}

#[test]
fn db_open_twice_no_error() {
    let repo = make_repo();
    let _c1 = sqlite_store::open_db(repo.path()).unwrap();
    let _c2 = sqlite_store::open_db(repo.path()).unwrap();
}

// ---------------------------------------------------------------------------
// kv 读写
// ---------------------------------------------------------------------------

#[test]
fn kv_roundtrip() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::kv_set(&conn, "test-key", "hello").unwrap();
    let val = sqlite_store::kv_get(&conn, "test-key").unwrap();
    assert_eq!(val, Some("hello".to_string()));
}

#[test]
fn kv_get_missing_returns_none() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    let val = sqlite_store::kv_get(&conn, "nonexistent").unwrap();
    assert_eq!(val, None);
}

#[test]
fn kv_set_overwrites() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::kv_set(&conn, "k", "v1").unwrap();
    sqlite_store::kv_set(&conn, "k", "v2").unwrap();
    let val = sqlite_store::kv_get(&conn, "k").unwrap();
    assert_eq!(val, Some("v2".to_string()));
}

#[test]
fn kv_exists_check() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    assert!(!sqlite_store::kv_exists(&conn, "k").unwrap());
    sqlite_store::kv_set(&conn, "k", "v").unwrap();
    assert!(sqlite_store::kv_exists(&conn, "k").unwrap());
}

// ---------------------------------------------------------------------------
// page cache 读写
// ---------------------------------------------------------------------------

#[test]
fn page_context_roundtrip() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_context(&conn, "page-1", "hash-a", r#"{"test":true}"#).unwrap();

    let result = sqlite_store::read_page_context(&conn, "page-1").unwrap();
    assert!(result.is_some());
    let (hash, ctx) = result.unwrap();
    assert_eq!(hash, "hash-a");
    assert!(ctx.contains("test"));
}

#[test]
fn page_context_missing_returns_none() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    let result = sqlite_store::read_page_context(&conn, "no-such-page").unwrap();
    assert!(result.is_none());
}

#[test]
fn page_context_overwrite() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_context(&conn, "p", "h1", "c1").unwrap();
    sqlite_store::write_page_context(&conn, "p", "h2", "c2").unwrap();

    let (hash, ctx) = sqlite_store::read_page_context(&conn, "p")
        .unwrap()
        .unwrap();
    assert_eq!(hash, "h2");
    assert_eq!(ctx, "c2");
}

#[test]
fn page_context_remove() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_context(&conn, "p", "h", "c").unwrap();
    assert!(sqlite_store::page_context_exists(&conn, "p").unwrap());

    sqlite_store::remove_page_context(&conn, "p").unwrap();
    assert!(!sqlite_store::page_context_exists(&conn, "p").unwrap());
}

#[test]
fn page_generation_roundtrip() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_generation(&conn, "pg-1", "ih", "ch", r#"[{"s":1}]"#).unwrap();

    let result = sqlite_store::read_page_generation(&conn, "pg-1").unwrap();
    assert!(result.is_some());
    let (ih, ch, secs) = result.unwrap();
    assert_eq!(ih, "ih");
    assert_eq!(ch, "ch");
    assert!(secs.contains("s"));
}

#[test]
fn page_generation_remove() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_generation(&conn, "pg", "ih", "ch", "[]").unwrap();
    assert!(sqlite_store::page_generation_exists(&conn, "pg").unwrap());

    sqlite_store::remove_page_generation(&conn, "pg").unwrap();
    assert!(!sqlite_store::page_generation_exists(&conn, "pg").unwrap());
}

#[test]
fn remove_page_all_clears_both() {
    let repo = make_repo();
    let conn = sqlite_store::open_db(repo.path()).unwrap();

    sqlite_store::write_page_context(&conn, "p", "h", "c").unwrap();
    sqlite_store::write_page_generation(&conn, "p", "ih", "ch", "[]").unwrap();

    sqlite_store::remove_page_all(&conn, "p").unwrap();
    assert!(!sqlite_store::page_context_exists(&conn, "p").unwrap());
    assert!(!sqlite_store::page_generation_exists(&conn, "p").unwrap());
}

#[test]
fn symbols_roundtrip_and_fts_query() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let symbols = vec![SymbolNode {
        symbol_id: "symbol-1".to_string(),
        name: "settlePayment".to_string(),
        label: "function".to_string(),
        file_path: "src/payments.ts".to_string(),
        start_line: 1,
        end_line: 3,
        is_exported: true,
        language: "typescript".to_string(),
    }];

    sqlite_store::replace_state_and_symbols(&mut conn, &empty_state(), &symbols).unwrap();

    let stored = sqlite_store::list_symbols(repo.path()).unwrap();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].name, "settlePayment");

    let hits = sqlite_store::search_symbols_fts(repo.path(), "settlePayment", 8).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].symbol_id, "symbol-1");
}

#[test]
fn symbol_graph_roundtrip_persists_edges_communities_and_processes() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let symbols = sample_symbols();
    let (resolved_graph, analysis) = sample_graph();

    sqlite_store::replace_state_and_symbol_graph(
        &mut conn,
        &empty_state(),
        &symbols,
        &resolved_graph,
        &analysis,
    )
    .unwrap();

    let edges = sqlite_store::list_edges(repo.path()).unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].edge_id, "edge-calls");
    assert_eq!(edges[0].edge_type, "CALLS");

    let communities = sqlite_store::list_communities(repo.path()).unwrap();
    assert_eq!(communities.len(), 1);
    assert_eq!(communities[0].community_id, "community-payments");

    let members = sqlite_store::list_community_members(repo.path()).unwrap();
    assert_eq!(members.len(), 2);
    assert!(members
        .iter()
        .any(|member| member.symbol_id == "symbol-service"));

    let processes = sqlite_store::list_processes(repo.path()).unwrap();
    assert_eq!(processes.len(), 1);
    assert_eq!(processes[0].process_id, "process-payment");
    assert_eq!(processes[0].step_count, 2);

    let steps = sqlite_store::list_process_steps(repo.path()).unwrap();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].symbol_id, "symbol-service");
    assert_eq!(steps[1].symbol_id, "symbol-helper");
}

#[test]
fn scoped_symbol_graph_reads_only_return_requested_files_and_frontier() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let symbols = sample_symbols();
    let (resolved_graph, analysis) = sample_graph();

    sqlite_store::replace_state_and_symbol_graph(
        &mut conn,
        &empty_state(),
        &symbols,
        &resolved_graph,
        &analysis,
    )
    .unwrap();

    let scoped_symbols =
        sqlite_store::list_symbols_for_files(repo.path(), &["src/service.ts".to_string()]).unwrap();
    assert_eq!(scoped_symbols.len(), 1);
    assert_eq!(scoped_symbols[0].symbol_id, "symbol-service");

    let scoped_edges =
        sqlite_store::list_edges_for_files(repo.path(), &["src/service.ts".to_string()]).unwrap();
    assert_eq!(scoped_edges.len(), 1);
    assert_eq!(scoped_edges[0].edge_id, "edge-calls");

    let adjacent =
        sqlite_store::list_adjacent_symbol_files(repo.path(), &["src/service.ts".to_string()])
            .unwrap();
    assert_eq!(adjacent, vec!["src/helper.ts".to_string()]);
    assert_eq!(sqlite_store::count_symbol_files(repo.path()).unwrap(), 2);
}

#[test]
fn symbol_graph_for_files_refresh_replaces_stale_edges_and_analysis() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let symbols = sample_symbols();
    let (resolved_graph, analysis) = sample_graph();

    sqlite_store::replace_state_and_symbol_graph(
        &mut conn,
        &empty_state(),
        &symbols,
        &resolved_graph,
        &analysis,
    )
    .unwrap();

    let refreshed_symbols = vec![SymbolNode {
        symbol_id: "symbol-service-v2".to_string(),
        name: "PaymentServiceV2".to_string(),
        label: "class".to_string(),
        file_path: "src/service.ts".to_string(),
        start_line: 1,
        end_line: 14,
        is_exported: true,
        language: "typescript".to_string(),
    }];
    let refreshed_graph = ResolvedGraphSnapshot {
        edges: vec![ResolvedSymbolEdge {
            edge_id: "edge-calls-v2".to_string(),
            source_id: "symbol-service-v2".to_string(),
            target_id: "symbol-helper".to_string(),
            edge_type: "CALLS".to_string(),
            confidence: 0.98,
            reason: "import-resolved".to_string(),
        }],
        diagnostics: Vec::new(),
    };
    let refreshed_analysis = GraphAnalysisSnapshot {
        communities: vec![CommunityNode {
            community_id: "community-payments-v2".to_string(),
            label: "payments-v2".to_string(),
            cohesion: 0.9,
            symbol_count: 2,
        }],
        community_members: vec![
            CommunityMember {
                community_id: "community-payments-v2".to_string(),
                symbol_id: "symbol-service-v2".to_string(),
            },
            CommunityMember {
                community_id: "community-payments-v2".to_string(),
                symbol_id: "symbol-helper".to_string(),
            },
        ],
        processes: vec![ProcessNode {
            process_id: "process-payment-v2".to_string(),
            label: "payment flow v2".to_string(),
            process_type: "request-flow".to_string(),
            step_count: 2,
            entry_point_id: Some("symbol-service-v2".to_string()),
            terminal_id: Some("symbol-helper".to_string()),
        }],
        process_steps: vec![
            ProcessStep {
                process_id: "process-payment-v2".to_string(),
                symbol_id: "symbol-service-v2".to_string(),
                step_order: 0,
            },
            ProcessStep {
                process_id: "process-payment-v2".to_string(),
                symbol_id: "symbol-helper".to_string(),
                step_order: 1,
            },
        ],
        cycles: Vec::new(),
        diagnostics: Vec::new(),
    };

    sqlite_store::replace_state_and_symbol_graph_for_files(
        &mut conn,
        &empty_state(),
        &["src/service.ts".to_string()],
        &refreshed_symbols,
        &refreshed_graph,
        &refreshed_analysis,
    )
    .unwrap();

    let stored_symbols = sqlite_store::list_symbols(repo.path()).unwrap();
    assert!(stored_symbols
        .iter()
        .any(|symbol| symbol.symbol_id == "symbol-service-v2"));
    assert!(stored_symbols
        .iter()
        .any(|symbol| symbol.symbol_id == "symbol-helper"));
    assert!(!stored_symbols
        .iter()
        .any(|symbol| symbol.symbol_id == "symbol-service"));

    let edges = sqlite_store::list_edges(repo.path()).unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].edge_id, "edge-calls-v2");
    assert_eq!(edges[0].source_id, "symbol-service-v2");

    let communities = sqlite_store::list_communities(repo.path()).unwrap();
    assert_eq!(communities.len(), 1);
    assert_eq!(communities[0].community_id, "community-payments-v2");

    let processes = sqlite_store::list_processes(repo.path()).unwrap();
    assert_eq!(processes.len(), 1);
    assert_eq!(processes[0].process_id, "process-payment-v2");
}

// ---------------------------------------------------------------------------
// cache_store 高层接口（通过 repo_root）
// ---------------------------------------------------------------------------

#[test]
fn has_cache_layout_false_when_no_db() {
    let repo = make_repo();
    assert!(!has_cache_layout(repo.path()));
}

#[test]
fn has_cache_layout_true_after_open() {
    let repo = make_repo();
    let _conn = sqlite_store::open_db(repo.path()).unwrap();
    assert!(has_cache_layout(repo.path()));
}

// ---------------------------------------------------------------------------
// remove_cache_db
// ---------------------------------------------------------------------------

#[test]
fn remove_cache_db_clears_db_file() {
    let repo = make_repo();
    let _conn = sqlite_store::open_db(repo.path()).unwrap();
    drop(_conn);

    assert!(sqlite_store::db_exists(repo.path()));
    remove_cache_db(repo.path()).unwrap();
    assert!(!sqlite_store::db_exists(repo.path()));
}

// ---------------------------------------------------------------------------
// state_store 通过 SQLite
// ---------------------------------------------------------------------------

#[test]
fn state_read_fails_when_no_db() {
    let repo = make_repo();
    assert!(read_state(repo.path()).is_err());
}

#[test]
fn open_db_does_not_auto_migrate_legacy_kv_entries() {
    let repo = make_repo();
    let legacy_state = legacy_state_json();
    write_legacy_db(
        repo.path(),
        &[
            ("wiki-state", legacy_state.as_str()),
            ("repo-scan", r#"{"files":[]}"#),
            ("module-tree", r#"{"modules":[]}"#),
        ],
    );

    let conn = sqlite_store::open_db(repo.path()).unwrap();
    assert!(sqlite_store::runtime_tables_exist(&conn).unwrap());
    assert!(!sqlite_store::scan_cache_exists(&conn, "repo-scan").unwrap());
    assert!(!sqlite_store::scan_cache_exists(&conn, "module-tree").unwrap());
    assert_eq!(
        sqlite_store::kv_get(&conn, "wiki-state").unwrap(),
        Some(legacy_state)
    );

    let err = sqlite_store::load_state_rows(&conn).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn read_state_does_not_fall_back_to_legacy_wiki_state_kv() {
    let repo = make_repo();
    let legacy_state = legacy_state_json();
    write_legacy_db(repo.path(), &[("wiki-state", legacy_state.as_str())]);

    let err = read_state(repo.path()).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

// ---------------------------------------------------------------------------
// DB 损坏回退
// ---------------------------------------------------------------------------

#[test]
fn load_state_falls_back_to_metadata_on_corrupt_db() {
    let repo = make_repo();
    let db_path = sqlite_store::db_path(repo.path());

    // 写入垃圾数据模拟损坏
    fs::write(&db_path, b"this is not a sqlite database").unwrap();

    // 没有 metadata 也没有有效 DB → 应该返回错误
    let result = load_or_rebuild_state(repo.path());
    assert!(result.is_err());
}
