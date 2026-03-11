use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::repo::symbol_graph::resolve::{
    build_import_resolution_context, resolve_calls, resolve_heritage, resolve_imports,
};
use wiki_core::repo::symbols::parse_symbols;
use wiki_core::storage::sqlite_store;
use wiki_core::workflows::{init::run_init, update::run_update};

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

#[test]
fn resolve_imports_supports_tsconfig_aliases() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "tsconfig.json",
        r#"{
          "compilerOptions": {
            "baseUrl": ".",
            "paths": {
              "@/*": ["src/*"]
            }
          }
        }"#,
    );
    write_repo_file(
        repo_root,
        "src/lib/payment.ts",
        "export function settlePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/index.ts",
        concat!(
            "import { settlePayment } from \"@/lib/payment\";\n",
            "export function runCheckout() {\n",
            "  return settlePayment();\n",
            "}\n",
        ),
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed = parse_symbols(repo_root, &scan_report).unwrap();
    let context = build_import_resolution_context(repo_root, &scan_report).unwrap();
    let resolved = resolve_imports(&parsed, &context);
    let run_checkout = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "runCheckout")
        .unwrap();
    let settle_payment = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "settlePayment")
        .unwrap();

    assert!(resolved.edges.iter().any(|edge| {
        edge.edge_type == "IMPORTS"
            && edge.source_id == run_checkout.symbol_id
            && edge.target_id == settle_payment.symbol_id
            && edge.reason.contains("@/lib/payment")
    }));
}

#[test]
fn resolve_imports_supports_rust_crate_paths() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(
        repo_root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write_repo_file(repo_root, "src/support.rs", "pub fn helper() {}\n");
    write_repo_file(
        repo_root,
        "src/lib.rs",
        concat!(
            "use crate::support::helper;\n",
            "\n",
            "pub fn run() {\n",
            "  helper();\n",
            "}\n",
        ),
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed = parse_symbols(repo_root, &scan_report).unwrap();
    let context = build_import_resolution_context(repo_root, &scan_report).unwrap();
    let resolved = resolve_imports(&parsed, &context);
    let run = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "run")
        .unwrap();
    let helper = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "helper" && symbol.file_path == "src/support.rs")
        .unwrap();

    assert!(resolved.edges.iter().any(|edge| {
        edge.edge_type == "IMPORTS"
            && edge.source_id == run.symbol_id
            && edge.target_id == helper.symbol_id
            && edge.reason.contains("crate/support/helper")
    }));
}

#[test]
fn resolve_imports_supports_swift_target_directories() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(
        repo_root,
        "Sources/App/main.swift",
        concat!("import SharedCore\n", "\n", "public func run() {\n", "}\n",),
    );
    write_repo_file(
        repo_root,
        "Sources/SharedCore/Helper.swift",
        "public func assist() {}\n",
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed = parse_symbols(repo_root, &scan_report).unwrap();
    let context = build_import_resolution_context(repo_root, &scan_report).unwrap();
    let resolved = resolve_imports(&parsed, &context);
    let run = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "run")
        .unwrap();
    let assist = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "assist")
        .unwrap();

    assert!(resolved.edges.iter().any(|edge| {
        edge.edge_type == "IMPORTS"
            && edge.source_id == run.symbol_id
            && edge.target_id == assist.symbol_id
            && edge.reason.contains("SharedCore")
    }));
}

#[test]
fn resolve_calls_prefers_same_file_and_import_targets_and_filters_noise() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/helper.ts",
        "export function settlePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { settlePayment } from \"./helper\";\n",
            "\n",
            "export class PaymentService {\n",
            "  run() {\n",
            "    settlePayment();\n",
            "    this.audit();\n",
            "    console.log(\"noop\");\n",
            "  }\n",
            "\n",
            "  audit() {\n",
            "    return true;\n",
            "  }\n",
            "}\n",
        ),
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed = parse_symbols(repo_root, &scan_report).unwrap();
    let context = build_import_resolution_context(repo_root, &scan_report).unwrap();
    let import_graph = resolve_imports(&parsed, &context);
    let call_graph = resolve_calls(&parsed, &import_graph);
    let run = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "run")
        .unwrap();
    let settle_payment = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "settlePayment")
        .unwrap();
    let audit = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "audit")
        .unwrap();

    assert!(call_graph.edges.iter().any(|edge| {
        edge.edge_type == "CALLS"
            && edge.source_id == run.symbol_id
            && edge.target_id == settle_payment.symbol_id
            && edge.reason == "import-resolved"
            && edge.confidence >= 0.85
    }));
    assert!(call_graph.edges.iter().any(|edge| {
        edge.edge_type == "CALLS"
            && edge.source_id == run.symbol_id
            && edge.target_id == audit.symbol_id
            && edge.reason == "same-file"
            && edge.confidence >= 0.95
    }));
    assert!(!call_graph
        .edges
        .iter()
        .any(|edge| edge.reason.contains("log") || edge.target_id.contains("log")));
}

#[test]
fn resolve_heritage_uses_import_resolved_targets() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(repo_root, "src/base.ts", "export class BaseService {}\n");
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { BaseService } from \"./base\";\n",
            "\n",
            "export class PaymentService extends BaseService {}\n",
        ),
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed = parse_symbols(repo_root, &scan_report).unwrap();
    let context = build_import_resolution_context(repo_root, &scan_report).unwrap();
    let import_graph = resolve_imports(&parsed, &context);
    let heritage_graph = resolve_heritage(&parsed, &import_graph);
    let payment_service = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "PaymentService")
        .unwrap();
    let base_service = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "BaseService")
        .unwrap();

    assert!(heritage_graph.edges.iter().any(|edge| {
        edge.edge_type == "EXTENDS"
            && edge.source_id == payment_service.symbol_id
            && edge.target_id == base_service.symbol_id
            && edge.reason == "import-resolved"
            && edge.confidence >= 0.9
    }));
}

#[test]
fn init_persists_symbol_graph_edges() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(repo_root, "src/base.ts", "export class BaseService {}\n");
    write_repo_file(
        repo_root,
        "src/helper.ts",
        "export function settlePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { settlePayment } from \"./helper\";\n",
            "import { BaseService } from \"./base\";\n",
            "\n",
            "export class PaymentService extends BaseService {\n",
            "  run() {\n",
            "    settlePayment();\n",
            "  }\n",
            "}\n",
        ),
    );

    run_init(repo_root).unwrap();

    let edges = sqlite_store::list_edges(repo_root).unwrap();
    assert!(edges.iter().any(|edge| edge.edge_type == "IMPORTS"));
    assert!(edges.iter().any(|edge| edge.edge_type == "CALLS"));
    assert!(edges.iter().any(|edge| edge.edge_type == "EXTENDS"));
}

#[test]
fn update_removes_edges_for_deleted_source_files() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/helper.ts",
        "export function settlePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { settlePayment } from \"./helper\";\n",
            "export function runCheckout() {\n",
            "  return settlePayment();\n",
            "}\n",
        ),
    );

    run_init(repo_root).unwrap();
    fs::remove_file(repo_root.join("src/service.ts")).unwrap();
    run_update(repo_root).unwrap();

    let edges = sqlite_store::list_edges(repo_root).unwrap();
    assert!(
        edges.is_empty(),
        "unexpected edges after delete: {edges:#?}"
    );
    let symbols = sqlite_store::list_symbols(repo_root).unwrap();
    assert!(symbols.iter().any(|symbol| symbol.name == "settlePayment"));
}

#[test]
fn update_refreshes_importer_edges_when_only_import_target_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/helper.ts",
        "export function settlePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { settlePayment } from \"./helper\";\n",
            "export function runCheckout() {\n",
            "  return settlePayment();\n",
            "}\n",
        ),
    );

    run_init(repo_root).unwrap();
    fs::write(
        repo_root.join("src/helper.ts"),
        "export function settleInvoice() { return true; }\n",
    )
    .unwrap();
    run_update(repo_root).unwrap();

    let edges = sqlite_store::list_edges(repo_root).unwrap();
    assert!(edges.iter().any(|edge| edge.edge_type == "IMPORTS"));
    assert!(
        !edges.iter().any(|edge| edge.edge_type == "CALLS"),
        "unexpected stale CALLS edges after importer fan-out refresh: {edges:#?}"
    );
}
