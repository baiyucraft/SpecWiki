use std::fs;

use tempfile::tempdir;
use wiki_core::storage::sqlite_store;
use wiki_core::workflows::{init::run_init, query::run_query, update::run_update};

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
