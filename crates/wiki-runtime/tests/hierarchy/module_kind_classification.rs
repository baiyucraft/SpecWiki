use std::path::PathBuf;

use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::scan_repo;

fn fixture_path() -> PathBuf {
    [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/module-kind-repo"),
        PathBuf::from("crates/wiki-runtime/tests/fixtures/module-kind-repo"),
        PathBuf::from("tests/fixtures/module-kind-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("module-kind-repo fixture should exist")
}

#[test]
fn cargo_lib_module_classified_as_library() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let engine = module_tree
        .modules
        .iter()
        .find(|m| m.name == "engine")
        .expect("engine module should exist");

    assert_eq!(
        engine.kind, "library",
        "Cargo.toml with [lib] should produce kind=library, got: {}",
        engine.kind
    );
}

#[test]
fn cargo_bin_module_classified_as_cli_tool() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let cli = module_tree
        .modules
        .iter()
        .find(|m| m.name == "cli")
        .expect("cli module should exist");

    assert_eq!(
        cli.kind, "cli-tool",
        "Cargo.toml with [[bin]] should produce kind=cli-tool, got: {}",
        cli.kind
    );
}

#[test]
fn infra_only_directory_classified_as_infrastructure() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let deploy = module_tree.modules.iter().find(|m| m.name == "deploy");

    // deploy may or may not be promoted depending on scoring,
    // but if it is, it should be infrastructure
    if let Some(deploy) = deploy {
        assert_eq!(
            deploy.kind, "infrastructure",
            "directory with only Dockerfile/shell should be infrastructure, got: {}",
            deploy.kind
        );
    }
}

#[test]
fn agent_directory_not_classified_as_frontend_app() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let connector = module_tree
        .modules
        .iter()
        .find(|m| m.name == "connector")
        .expect("connector module should exist");

    assert_ne!(
        connector.kind, "frontend-app",
        "agent/connector directory should not be frontend-app, got: {}",
        connector.kind
    );
}



