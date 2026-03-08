use std::path::PathBuf;

use wiki_core::repo::scanner::scan_repo;

#[test]
fn scan_repo_discovers_files_and_detected_stack() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-node-repo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/minimal-node-repo"),
        PathBuf::from("tests/fixtures/minimal-node-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for repo scan tests");
    let report = scan_repo(&fixture).unwrap();

    assert!(report.files.iter().any(|f| f.path.ends_with("package.json")));
    assert!(report.entry_points.iter().any(|path| path.ends_with("src/index.ts")));
    assert!(report
        .tech_hints
        .iter()
        .any(|topic| topic == "frontend"));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "src/index.ts" && hint.to.starts_with("src/utils")
    }));
}

#[test]
fn scan_repo_ignores_runtime_artifacts_and_detects_mixed_stack() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mixed-local-repo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/mixed-local-repo"),
        PathBuf::from("tests/fixtures/mixed-local-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for mixed repo scan tests");
    let report = scan_repo(&fixture).unwrap();

    assert!(report.files.iter().any(|file| file.path == "spider/app.py"));
    assert!(report.files.iter().any(|file| file.path == "nginx/conf/nginx.conf"));
    assert!(!report
        .files
        .iter()
        .any(|file| file.path.contains("node_modules") || file.path.contains("logs")));
    assert!(report.tech_hints.iter().any(|topic| topic == "frontend"));
    assert!(report.tech_hints.iter().any(|topic| topic == "backend"));
    assert!(report
        .tech_hints
        .iter()
        .any(|topic| topic == "infrastructure"));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "spider/app.py" && hint.to.starts_with("spider/modules")
    }));
}

#[test]
fn scan_repo_parses_manifest_workspaces_and_internal_package_aliases() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest-monorepo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/manifest-monorepo"),
        PathBuf::from("tests/fixtures/manifest-monorepo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for manifest monorepo scan tests");
    let report = scan_repo(&fixture).unwrap();

    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "packages/app"));
    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "packages/shared"));
    assert!(report
        .dependency_hints
        .iter()
        .any(|hint| hint.from == "packages/app/src/main.ts" && hint.to.starts_with("packages/shared")));
}

#[test]
fn scan_repo_parses_rust_workspace_dependencies_with_syn() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rust-workspace"),
        PathBuf::from("crates/wiki-core/tests/fixtures/rust-workspace"),
        PathBuf::from("tests/fixtures/rust-workspace"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for rust workspace scan tests");
    let report = scan_repo(&fixture).unwrap();

    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "crates/api"));
    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "crates/core"));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "crates/api/src/main.rs" && hint.to.starts_with("crates/core")
    }));
}

#[test]
fn scan_repo_parses_java_package_aliases_into_dependency_hints() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/java-modules"),
        PathBuf::from("crates/wiki-core/tests/fixtures/java-modules"),
        PathBuf::from("tests/fixtures/java-modules"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for java module scan tests");
    let report = scan_repo(&fixture).unwrap();

    assert!(report
        .files
        .iter()
        .any(|file| file.language == "java" && file.path == "api/src/Main.java"));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "api/src/Main.java" && hint.to.starts_with("core/")
    }));
}
