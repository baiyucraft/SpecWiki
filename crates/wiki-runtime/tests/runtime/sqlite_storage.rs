//! SQLite 存储层单元测试。
//! 覆盖 DB 初始化、kv 读写、page cache 读写、事务原子性、DB 损坏回退。

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use tempfile::TempDir;
use wiki_index::store::{
    FolderRecord, GraphPhaseStatus, GraphReadinessStatus, GraphSnapshot, SourceFileRecord,
};
use wiki_index::symbol_graph::{
    CommunityMember, CommunityNode, GraphAnalysisSnapshot, ProcessNode, ProcessStep,
    ResolvedGraphSnapshot, ResolvedSymbolEdge,
};
use wiki_index::symbols::{
    GraphPhase, RawCallCapture, RawCaptureBase, RawCaptureKind, RawImportCapture, ReferenceKind,
    SourceRange, SymbolNode, UnresolvedRef,
};
use wiki_runtime::domain::metadata::DirtyState;
use wiki_runtime::domain::module_tree::ModuleTree;
use wiki_runtime::domain::state::{BuildState, WikiState};
use wiki_runtime::storage::cache_store::has_cache_layout;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::storage::state_store::{load_or_rebuild_state, read_state};
use wiki_runtime::storage::wiki_fs::remove_cache_db;

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
        SymbolNode::legacy(
            "symbol-service".to_string(),
            "PaymentService".to_string(),
            "class".to_string(),
            "src/service.ts".to_string(),
            1,
            12,
            true,
            "typescript".to_string(),
        ),
        SymbolNode::legacy(
            "symbol-helper".to_string(),
            "runHelper".to_string(),
            "function".to_string(),
            "src/helper.ts".to_string(),
            1,
            3,
            true,
            "typescript".to_string(),
        ),
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

fn sample_graph_snapshot_with_raw_and_unresolved() -> GraphSnapshot {
    let symbols = sample_symbols();
    let (resolved_graph, analysis) = sample_graph();
    let import_base = RawCaptureBase::new(
        RawCaptureKind::Import,
        "src/service.ts",
        "typescript",
        1,
        "import { runHelper } from './helper'",
        Some("./helper".to_string()),
        Some("symbol-service".to_string()),
    );
    let call_base = RawCaptureBase::new(
        RawCaptureKind::Call,
        "src/service.ts",
        "typescript",
        3,
        "runHelper()",
        Some("runHelper".to_string()),
        Some("symbol-service".to_string()),
    );
    GraphSnapshot {
        snapshot_id: "snapshot-1".to_string(),
        source_fingerprint: "source-fp-1".to_string(),
        files: vec![
            SourceFileRecord {
                file_id: "file:src/service.ts".to_string(),
                path: "src/service.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                fingerprint: "fp-service".to_string(),
                size: 128,
                indexed_at: "2026-07-01T00:00:00Z".to_string(),
                diagnostics: Vec::new(),
            },
            SourceFileRecord {
                file_id: "file:src/helper.ts".to_string(),
                path: "src/helper.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                fingerprint: "fp-helper".to_string(),
                size: 64,
                indexed_at: "2026-07-01T00:00:00Z".to_string(),
                diagnostics: Vec::new(),
            },
        ],
        folders: vec![FolderRecord {
            folder_id: "folder:src".to_string(),
            path: "src".to_string(),
            parent_id: None,
        }],
        symbols,
        edges: resolved_graph.edges.clone(),
        raw_imports: vec![RawImportCapture {
            base: import_base.clone(),
            file_path: "src/service.ts".to_string(),
            raw_path: "./helper".to_string(),
            imported_name: Some("runHelper".to_string()),
            alias: None,
            line: 1,
            language: "typescript".to_string(),
            source_symbol_id: Some("symbol-service".to_string()),
            source_text: import_base.raw_text.clone(),
        }],
        raw_calls: vec![RawCallCapture {
            base: call_base.clone(),
            file_path: "src/service.ts".to_string(),
            called_name: "runHelper".to_string(),
            line: 3,
            language: "typescript".to_string(),
            source_symbol_id: Some("symbol-service".to_string()),
            receiver_text: None,
            source_text: call_base.raw_text.clone(),
            argument_shape: Some("arity:0".to_string()),
        }],
        raw_heritage: Vec::new(),
        unresolved_refs: vec![UnresolvedRef {
            unresolved_ref_id: "unresolved-1".to_string(),
            capture_id: call_base.capture_id.clone(),
            file_id: "file:src/service.ts".to_string(),
            resolver_phase: GraphPhase::ResolveCalls,
            reference_kind: ReferenceKind::Call,
            reference_name: "runMissing".to_string(),
            target_hint: Some("runMissing".to_string()),
            range: SourceRange::new("file:src/service.ts", "src/service.ts", 4, 4, 0, 12),
            candidates: Vec::new(),
            reason: "target_not_found".to_string(),
            diagnostics: vec!["call target was not resolved".to_string()],
        }],
        analysis,
        phase_statuses: vec![GraphPhaseStatus {
            phase: GraphPhase::BuildFts,
            status: "ready".to_string(),
            input_fingerprint: Some("source-fp-1".to_string()),
            output_fingerprint: Some("graph-fp-1".to_string()),
            started_at: Some("2026-07-01T00:00:00Z".to_string()),
            completed_at: Some("2026-07-01T00:00:01Z".to_string()),
            diagnostics: Vec::new(),
        }],
    }
}

fn sample_graph_snapshot_with_spec_path() -> GraphSnapshot {
    let mut snapshot = sample_graph_snapshot_with_raw_and_unresolved();
    snapshot.files = vec![SourceFileRecord {
        file_id: "file:.spec/changes/demo/tasks.md".to_string(),
        path: ".spec/changes/demo/tasks.md".to_string(),
        language: "markdown".to_string(),
        kind: "source".to_string(),
        fingerprint: "fp-spec".to_string(),
        size: 32,
        indexed_at: "2026-07-01T00:00:00Z".to_string(),
        diagnostics: Vec::new(),
    }];
    snapshot.folders = vec![FolderRecord {
        folder_id: "folder:.spec".to_string(),
        path: ".spec".to_string(),
        parent_id: None,
    }];
    snapshot.symbols = vec![SymbolNode::legacy(
        "symbol-spec".to_string(),
        "FakeSpecSymbol".to_string(),
        "function".to_string(),
        ".spec/changes/demo/tasks.md".to_string(),
        1,
        1,
        true,
        "markdown".to_string(),
    )];
    let base = RawCaptureBase::new(
        RawCaptureKind::Import,
        ".spec/changes/demo/tasks.md",
        "markdown",
        1,
        "governance import",
        Some("governance".to_string()),
        Some("symbol-spec".to_string()),
    );
    snapshot.raw_imports = vec![RawImportCapture {
        base: base.clone(),
        file_path: ".spec/changes/demo/tasks.md".to_string(),
        raw_path: "governance".to_string(),
        imported_name: Some("governance".to_string()),
        alias: None,
        line: 1,
        language: "markdown".to_string(),
        source_symbol_id: Some("symbol-spec".to_string()),
        source_text: base.raw_text.clone(),
    }];
    snapshot.raw_calls = Vec::new();
    snapshot.raw_heritage = Vec::new();
    snapshot.unresolved_refs = Vec::new();
    snapshot.edges = Vec::new();
    snapshot.analysis = GraphAnalysisSnapshot::default();
    snapshot
}

fn sample_scoped_graph_snapshot_for_file() -> GraphSnapshot {
    let mut snapshot = sample_graph_snapshot_with_raw_and_unresolved();
    snapshot.snapshot_id = "snapshot-2".to_string();
    snapshot.source_fingerprint = "source-fp-2".to_string();
    snapshot.files = vec![SourceFileRecord {
        file_id: "file:src/service.ts".to_string(),
        path: "src/service.ts".to_string(),
        language: "typescript".to_string(),
        kind: "source".to_string(),
        fingerprint: "fp-service-v2".to_string(),
        size: 256,
        indexed_at: "2026-07-01T00:01:00Z".to_string(),
        diagnostics: Vec::new(),
    }];
    snapshot.folders = vec![FolderRecord {
        folder_id: "folder:src".to_string(),
        path: "src".to_string(),
        parent_id: None,
    }];
    snapshot.symbols = vec![SymbolNode::legacy(
        "symbol-service-v2".to_string(),
        "PaymentServiceV2".to_string(),
        "class".to_string(),
        "src/service.ts".to_string(),
        1,
        14,
        true,
        "typescript".to_string(),
    )];
    let call_base = RawCaptureBase::new(
        RawCaptureKind::Call,
        "src/service.ts",
        "typescript",
        4,
        "PaymentServiceV2.run()",
        Some("PaymentServiceV2".to_string()),
        Some("symbol-service-v2".to_string()),
    );
    snapshot.raw_imports = Vec::new();
    snapshot.raw_calls = vec![RawCallCapture {
        base: call_base.clone(),
        file_path: "src/service.ts".to_string(),
        called_name: "PaymentServiceV2".to_string(),
        line: 4,
        language: "typescript".to_string(),
        source_symbol_id: Some("symbol-service-v2".to_string()),
        receiver_text: None,
        source_text: call_base.raw_text.clone(),
        argument_shape: Some("arity:0".to_string()),
    }];
    snapshot.raw_heritage = Vec::new();
    snapshot.unresolved_refs = vec![UnresolvedRef {
        unresolved_ref_id: "unresolved-v2".to_string(),
        capture_id: call_base.capture_id.clone(),
        file_id: "file:src/service.ts".to_string(),
        resolver_phase: GraphPhase::ResolveCalls,
        reference_kind: ReferenceKind::Call,
        reference_name: "PaymentServiceV2".to_string(),
        target_hint: Some("PaymentServiceV2".to_string()),
        range: SourceRange::new("file:src/service.ts", "src/service.ts", 4, 4, 0, 20),
        candidates: Vec::new(),
        reason: "target_not_found".to_string(),
        diagnostics: Vec::new(),
    }];
    snapshot.edges = vec![ResolvedSymbolEdge {
        edge_id: "edge-calls-v2".to_string(),
        source_id: "symbol-service-v2".to_string(),
        target_id: "symbol-helper".to_string(),
        edge_type: "CALLS".to_string(),
        confidence: 0.98,
        reason: "import-resolved".to_string(),
    }];
    snapshot.analysis = GraphAnalysisSnapshot::default();
    snapshot
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
    let symbols = vec![SymbolNode::legacy(
        "symbol-1".to_string(),
        "settlePayment".to_string(),
        "function".to_string(),
        "src/payments.ts".to_string(),
        1,
        3,
        true,
        "typescript".to_string(),
    )];

    sqlite_store::replace_state_and_symbols(&mut conn, &empty_state(), &symbols).unwrap();

    let stored = sqlite_store::list_symbols(repo.path()).unwrap();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].name, "settlePayment");

    let hits = sqlite_store::search_symbols_fts(repo.path(), "settlePayment", 8).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].symbol.symbol_id, "symbol-1");
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
fn graph_snapshot_roundtrip_persists_raw_unresolved_phase_and_fts_atomically() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_raw_and_unresolved();

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();

    assert_eq!(sqlite_store::list_symbols(repo.path()).unwrap().len(), 2);
    assert_eq!(
        sqlite_store::list_raw_imports(repo.path()).unwrap().len(),
        1
    );
    assert_eq!(sqlite_store::list_raw_calls(repo.path()).unwrap().len(), 1);
    assert_eq!(
        sqlite_store::list_unresolved_refs(repo.path())
            .unwrap()
            .len(),
        1
    );
    assert!(sqlite_store::search_files_fts(repo.path(), "service", 8)
        .unwrap()
        .iter()
        .any(|hit| hit.path == "src/service.ts"));
    assert!(sqlite_store::list_graph_phase_runs(repo.path())
        .unwrap()
        .iter()
        .any(|phase| phase.phase == GraphPhase::BuildFts));
    assert_eq!(
        sqlite_store::read_current_graph_snapshot(repo.path())
            .unwrap()
            .unwrap()
            .snapshot_id,
        snapshot.snapshot_id
    );
}

#[test]
fn graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_raw_and_unresolved();

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();
    sqlite_store::clear_scan_cache_for_test(&conn).unwrap();

    assert!(sqlite_store::list_files(repo.path())
        .unwrap()
        .iter()
        .any(|file| file.path == "src/service.ts"));
    assert!(sqlite_store::list_folders(repo.path())
        .unwrap()
        .iter()
        .any(|folder| folder.path == "src"));
    assert!(!sqlite_store::list_raw_imports(repo.path())
        .unwrap()
        .is_empty());
    assert!(!sqlite_store::list_unresolved_refs(repo.path())
        .unwrap()
        .is_empty());
    assert!(!sqlite_store::list_graph_phase_runs(repo.path())
        .unwrap()
        .is_empty());
    assert!(!sqlite_store::search_files_fts(repo.path(), "service", 8)
        .unwrap()
        .is_empty());
    assert!(sqlite_store::read_current_graph_snapshot(repo.path())
        .unwrap()
        .is_some());
}

#[test]
fn graph_snapshot_rejects_spec_paths_before_tables_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_spec_path();

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();

    assert!(sqlite_store::list_files(repo.path())
        .unwrap()
        .iter()
        .all(|file| !file.path.starts_with(".spec/")));
    assert!(sqlite_store::list_folders(repo.path())
        .unwrap()
        .iter()
        .all(|folder| !folder.path.starts_with(".spec")));
    assert!(sqlite_store::list_symbols(repo.path())
        .unwrap()
        .iter()
        .all(|symbol| !symbol.file_path.starts_with(".spec/")));
    assert!(sqlite_store::list_raw_imports(repo.path())
        .unwrap()
        .is_empty());
    assert!(sqlite_store::search_files_fts(repo.path(), "governance", 8)
        .unwrap()
        .is_empty());
    assert!(
        sqlite_store::search_symbols_fts(repo.path(), "FakeSpecSymbol", 8)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn scoped_graph_snapshot_refresh_replaces_symbols_raw_unresolved_edges_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let initial = sample_graph_snapshot_with_raw_and_unresolved();
    sqlite_store::replace_graph_snapshot(&mut conn, &initial).unwrap();

    let refreshed = sample_scoped_graph_snapshot_for_file();
    sqlite_store::replace_graph_snapshot_for_files(
        &mut conn,
        &["src/service.ts".to_string()],
        &refreshed,
    )
    .unwrap();

    let symbols = sqlite_store::list_symbols(repo.path()).unwrap();
    assert!(symbols
        .iter()
        .any(|symbol| symbol.name == "PaymentServiceV2"));
    assert!(symbols.iter().any(|symbol| symbol.name == "runHelper"));
    assert!(symbols.iter().all(|symbol| symbol.name != "PaymentService"));
    assert!(sqlite_store::list_raw_calls(repo.path())
        .unwrap()
        .iter()
        .all(|capture| capture.file_path != "src/service.ts"
            || capture.called_name == "PaymentServiceV2"));
    assert!(sqlite_store::list_unresolved_refs(repo.path())
        .unwrap()
        .iter()
        .all(|item| item.file_id != "file:src/service.ts"
            || item.reference_name == "PaymentServiceV2"));
    assert!(
        sqlite_store::search_symbols_fts(repo.path(), "PaymentService", 8)
            .unwrap()
            .iter()
            .all(|hit| hit.symbol.name != "PaymentService")
    );
}

#[test]
fn graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready() {
    let missing = make_repo();
    let missing_status = sqlite_store::read_graph_readiness(missing.path()).unwrap();
    assert_eq!(missing_status.status, GraphReadinessStatus::Missing);

    let blocked = make_repo();
    {
        let conn = sqlite_store::open_db(blocked.path()).unwrap();
        conn.execute("DROP TABLE graph_snapshots", []).unwrap();
    }
    let blocked_status = sqlite_store::read_graph_readiness(blocked.path()).unwrap();
    assert_eq!(blocked_status.status, GraphReadinessStatus::Blocked);
    assert!(blocked_status
        .required_tables
        .iter()
        .any(|table| table == "graph_snapshots"));

    let stale = make_repo();
    let mut conn = sqlite_store::open_db(stale.path()).unwrap();
    let mut snapshot = sample_graph_snapshot_with_raw_and_unresolved();
    snapshot.source_fingerprint = "stale-source-fp".to_string();
    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO graph_phase_runs
         (phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics)
         VALUES ('scan', 'stale', 'current-source-fp', 'stale-source-fp', NULL, NULL, '[]')",
        [],
    )
    .unwrap();
    assert_eq!(
        sqlite_store::read_graph_readiness(stale.path())
            .unwrap()
            .status,
        GraphReadinessStatus::Stale
    );

    let rebuilding = make_repo();
    let mut conn = sqlite_store::open_db(rebuilding.path()).unwrap();
    sqlite_store::replace_graph_snapshot(
        &mut conn,
        &sample_graph_snapshot_with_raw_and_unresolved(),
    )
    .unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO graph_phase_runs
         (phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics)
         VALUES ('build_fts', 'rebuilding', NULL, NULL, NULL, NULL, '[]')",
        [],
    )
    .unwrap();
    assert_eq!(
        sqlite_store::read_graph_readiness(rebuilding.path())
            .unwrap()
            .status,
        GraphReadinessStatus::Rebuilding
    );

    let diagnostic_blocked = make_repo();
    let mut conn = sqlite_store::open_db(diagnostic_blocked.path()).unwrap();
    sqlite_store::replace_graph_snapshot(
        &mut conn,
        &sample_graph_snapshot_with_raw_and_unresolved(),
    )
    .unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO graph_phase_runs
         (phase, status, input_fingerprint, output_fingerprint, started_at, completed_at, diagnostics)
         VALUES ('resolve_calls', 'blocked', NULL, NULL, NULL, NULL, '[\"resolver failed\"]')",
        [],
    )
    .unwrap();
    assert_eq!(
        sqlite_store::read_graph_readiness(diagnostic_blocked.path())
            .unwrap()
            .status,
        GraphReadinessStatus::Blocked
    );

    let ready = make_repo();
    let mut conn = sqlite_store::open_db(ready.path()).unwrap();
    sqlite_store::replace_graph_snapshot(
        &mut conn,
        &sample_graph_snapshot_with_raw_and_unresolved(),
    )
    .unwrap();
    let ready_status = sqlite_store::read_graph_readiness(ready.path()).unwrap();
    assert_eq!(ready_status.status, GraphReadinessStatus::Ready);
    assert_eq!(ready_status.snapshot_id, Some("snapshot-1".to_string()));
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

    let refreshed_symbols = vec![SymbolNode::legacy(
        "symbol-service-v2".to_string(),
        "PaymentServiceV2".to_string(),
        "class".to_string(),
        "src/service.ts".to_string(),
        1,
        14,
        true,
        "typescript".to_string(),
    )];
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

    let err = sqlite_store::load_state_rows(
        &conn,
        &ModuleTree {
            root_modules: Vec::new(),
            modules: Vec::new(),
            cross_module_edges: Vec::new(),
            architecture_hints: Vec::new(),
        },
    )
    .unwrap_err();
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
