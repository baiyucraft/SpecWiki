use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::repo::symbols::parse_symbols;
use wiki_core::storage::sqlite_store;
use wiki_core::workflows::{init::run_init, query::run_query, update::run_update};

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn parse_repo_symbols(repo_root: &Path) -> wiki_core::repo::symbols::ParsedSymbolsSnapshot {
    let scan_report = scan_repo(repo_root, &[]).unwrap();
    parse_symbols(repo_root, &scan_report).unwrap()
}

#[test]
fn init_persists_symbols_and_query_returns_symbol_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(
        repo_root.join("src/index.ts"),
        "export function settlePayment() { return true; }\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();

    let symbols = sqlite_store::list_symbols(repo_root).unwrap();
    assert!(
        symbols.iter().any(|symbol| {
            symbol.name == "settlePayment"
                && symbol.label == "function"
                && symbol.language == "typescript"
                && symbol.is_exported
        }),
        "unexpected symbols: {symbols:#?}"
    );

    let query = run_query(repo_root, "settlePayment").unwrap();
    assert!(query
        .matched_symbols
        .iter()
        .any(|symbol| symbol.name == "settlePayment"));
    assert!(query
        .matches
        .iter()
        .any(|page| page.reasons.iter().any(|reason| reason == "关联符号匹配")));
}

#[test]
fn update_removes_symbols_for_deleted_source() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(
        repo_root.join("src/index.ts"),
        "export function settlePayment() { return true; }\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();
    fs::remove_file(repo_root.join("src/index.ts")).unwrap();
    run_update(repo_root).unwrap();

    let symbols = sqlite_store::list_symbols(repo_root).unwrap();
    assert!(symbols.is_empty());

    let query = run_query(repo_root, "settlePayment").unwrap();
    assert!(query.matched_symbols.is_empty());
}

#[test]
fn update_reparses_modified_source_symbols() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("src")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(
        repo_root.join("src/index.ts"),
        "export function settlePayment() { return true; }\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();
    fs::write(
        repo_root.join("src/index.ts"),
        "export function settleInvoice() { return true; }\n",
    )
    .unwrap();
    run_update(repo_root).unwrap();

    let symbols = sqlite_store::list_symbols(repo_root).unwrap();
    assert!(symbols.iter().any(|symbol| symbol.name == "settleInvoice"));
    assert!(!symbols.iter().any(|symbol| symbol.name == "settlePayment"));

    let old_query = run_query(repo_root, "settlePayment").unwrap();
    assert!(old_query.matched_symbols.is_empty());

    let new_query = run_query(repo_root, "settleInvoice").unwrap();
    assert!(new_query
        .matched_symbols
        .iter()
        .any(|symbol| symbol.name == "settleInvoice"));
}

#[test]
fn parse_symbols_keeps_raw_import_call_and_heritage_metadata() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/index.ts",
        concat!(
            "import { helper } from \"./helper\";\n",
            "import { BaseService } from \"./base\";\n",
            "\n",
            "export class PaymentService extends BaseService {\n",
            "  settle() {\n",
            "    helper();\n",
            "    this.audit();\n",
            "  }\n",
            "\n",
            "  audit() {\n",
            "    return true;\n",
            "  }\n",
            "}\n",
        ),
    );

    let snapshot = parse_repo_symbols(repo_root);
    let parsed = snapshot.files.get("src/index.ts").unwrap();
    let service_symbol = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "PaymentService")
        .unwrap();
    let settle_symbol = parsed
        .symbols
        .iter()
        .find(|symbol| symbol.name == "settle")
        .unwrap();

    assert!(parsed.imports.iter().any(|capture| {
        capture.raw_path == "./helper"
            && capture.line == 1
            && capture.source_symbol_id.is_none()
            && capture.source_text.contains("helper")
    }));
    assert!(parsed.imports.iter().any(|capture| {
        capture.raw_path == "./base"
            && capture.line == 2
            && capture.source_symbol_id.is_none()
    }));

    let helper_call = parsed
        .calls
        .iter()
        .find(|capture| capture.called_name == "helper")
        .unwrap();
    assert_eq!(helper_call.line, 6);
    assert_eq!(
        helper_call.source_symbol_id.as_deref(),
        Some(settle_symbol.symbol_id.as_str())
    );
    assert_eq!(helper_call.receiver_text, None);

    let audit_call = parsed
        .calls
        .iter()
        .find(|capture| capture.called_name == "audit")
        .unwrap();
    assert_eq!(audit_call.line, 7);
    assert_eq!(audit_call.receiver_text.as_deref(), Some("this"));
    assert_eq!(
        audit_call.source_symbol_id.as_deref(),
        Some(settle_symbol.symbol_id.as_str())
    );

    let extends_capture = parsed
        .heritage
        .iter()
        .find(|capture| capture.target_name == "BaseService")
        .unwrap();
    assert_eq!(extends_capture.line, 4);
    assert_eq!(extends_capture.relation_kind, "extends");
    assert_eq!(extends_capture.owner_name, "PaymentService");
    assert_eq!(
        extends_capture.owner_symbol_id.as_deref(),
        Some(service_symbol.symbol_id.as_str())
    );
}

#[test]
fn kotlin_symbols_and_raw_captures_survive_iter8_queries() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(
        repo_root,
        "src/Greeter.kt",
        concat!(
            "import demo.shared.BaseGreeter\n",
            "\n",
            "class Greeter : BaseGreeter {\n",
            "  fun greet() {\n",
            "    helper()\n",
            "    service.render()\n",
            "  }\n",
            "}\n",
        ),
    );

    let snapshot = parse_repo_symbols(repo_root);
    let parsed = snapshot.files.get("src/Greeter.kt").unwrap();

    assert!(
        parsed.diagnostics.is_empty(),
        "unexpected diagnostics: {:#?}",
        parsed.diagnostics
    );
    assert!(parsed
        .symbols
        .iter()
        .any(|symbol| symbol.name == "Greeter" && symbol.label == "class"));
    assert!(parsed
        .symbols
        .iter()
        .any(|symbol| symbol.name == "greet" && symbol.label == "function"));
    assert!(parsed
        .imports
        .iter()
        .any(|capture| capture.raw_path == "demo.shared.BaseGreeter"));
    assert!(parsed
        .calls
        .iter()
        .any(|capture| capture.called_name == "helper"));
    assert!(parsed.calls.iter().any(|capture| {
        capture.called_name == "render" && capture.receiver_text.as_deref() == Some("service")
    }));
    assert!(parsed.heritage.iter().any(|capture| {
        capture.owner_name == "Greeter" && capture.target_name.contains("BaseGreeter")
    }));
}

#[test]
fn csharp_symbols_and_raw_captures_survive_iter8_queries() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(
        repo_root,
        "src/Greeter.cs",
        concat!(
            "using Demo.Shared;\n",
            "\n",
            "namespace Demo.Core;\n",
            "\n",
            "public class Greeter : BaseGreeter {\n",
            "  public void Greet() {\n",
            "    Helper();\n",
            "    service.Render();\n",
            "  }\n",
            "}\n",
        ),
    );

    let snapshot = parse_repo_symbols(repo_root);
    let parsed = snapshot.files.get("src/Greeter.cs").unwrap();

    assert!(
        parsed.diagnostics.is_empty(),
        "unexpected diagnostics: {:#?}",
        parsed.diagnostics
    );
    assert!(parsed
        .symbols
        .iter()
        .any(|symbol| symbol.name == "Greeter" && symbol.label == "class"));
    assert!(parsed
        .symbols
        .iter()
        .any(|symbol| symbol.name == "Greet" && symbol.label == "method"));
    assert!(parsed
        .imports
        .iter()
        .any(|capture| capture.raw_path == "Demo.Shared"));
    assert!(parsed
        .calls
        .iter()
        .any(|capture| capture.called_name == "Helper"));
    assert!(parsed.calls.iter().any(|capture| {
        capture.called_name == "Render" && capture.receiver_text.as_deref() == Some("service")
    }));
    assert!(parsed.heritage.iter().any(|capture| {
        capture.owner_name == "Greeter" && capture.target_name.contains("BaseGreeter")
    }));
}

#[test]
fn vue_wrapper_symbols_keep_original_line_numbers() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/App.vue",
        concat!(
            "<template>\n",
            "  <div />\n",
            "</template>\n",
            "<script setup lang=\"ts\">\n",
            "const title = \"demo\";\n",
            "\n",
            "export function useWidget() {\n",
            "  return title;\n",
            "}\n",
            "</script>\n",
        ),
    );

    let snapshot = parse_repo_symbols(repo_root);
    let parsed = snapshot.files.get("src/App.vue").unwrap();
    let symbol = parsed
        .symbols
        .iter()
        .find(|entry| entry.name == "useWidget")
        .unwrap();

    assert_eq!(parsed.language, "typescript");
    assert_eq!(symbol.file_path, "src/App.vue");
    assert_eq!(symbol.language, "typescript");
    assert_eq!(symbol.start_line, 7);
}

#[test]
fn svelte_wrapper_symbols_keep_original_line_numbers() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/App.svelte",
        concat!(
            "<h1>Hello</h1>\n",
            "\n",
            "<script lang=\"ts\">\n",
            "  export function useCounter() {\n",
            "    return 1;\n",
            "  }\n",
            "</script>\n",
        ),
    );

    let snapshot = parse_repo_symbols(repo_root);
    let parsed = snapshot.files.get("src/App.svelte").unwrap();
    let symbol = parsed
        .symbols
        .iter()
        .find(|entry| entry.name == "useCounter")
        .unwrap();

    assert_eq!(parsed.language, "typescript");
    assert_eq!(symbol.file_path, "src/App.svelte");
    assert_eq!(symbol.language, "typescript");
    assert_eq!(symbol.start_line, 4);
}
