use std::path::PathBuf;

use tempfile::tempdir;
use wiki_core::repo::scanner::{scan_repo, FilePurpose};

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
    let report = scan_repo(&fixture, &[]).unwrap();

    assert!(report
        .files
        .iter()
        .any(|f| f.path.ends_with("package.json")));
    assert!(report
        .entry_points
        .iter()
        .any(|path| path.ends_with("src/index.ts")));
    assert!(report.tech_hints.iter().any(|topic| topic == "frontend"));
    assert!(report
        .dependency_hints
        .iter()
        .any(|hint| { hint.from == "src/index.ts" && hint.to.starts_with("src/utils") }));
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
    let report = scan_repo(&fixture, &[]).unwrap();

    assert!(report.files.iter().any(|file| file.path == "spider/app.py"));
    assert!(report
        .files
        .iter()
        .any(|file| file.path == "nginx/conf/nginx.conf"));
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
    assert!(report
        .dependency_hints
        .iter()
        .any(|hint| { hint.from == "spider/app.py" && hint.to.starts_with("spider/modules") }));
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
    let report = scan_repo(&fixture, &[]).unwrap();

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
        .any(|hint| hint.from == "packages/app/src/main.ts"
            && hint.to.starts_with("packages/shared")));
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
    let report = scan_repo(&fixture, &[]).unwrap();

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
    let report = scan_repo(&fixture, &[]).unwrap();

    assert!(report
        .files
        .iter()
        .any(|file| file.language == "java" && file.path == "api/src/Main.java"));
    assert!(report
        .dependency_hints
        .iter()
        .any(|hint| { hint.from == "api/src/Main.java" && hint.to.starts_with("core/") }));
}

#[test]
fn scan_repo_parses_nested_workspace_aliases_into_dependency_hints() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/baseline-hierarchy-repo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/baseline-hierarchy-repo"),
        PathBuf::from("tests/fixtures/baseline-hierarchy-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for baseline hierarchy scan tests");
    let report = scan_repo(&fixture, &[]).unwrap();

    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "packages/domain/auth"));
    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "packages/domain/shared"));
    assert!(report
        .workspace_roots
        .iter()
        .any(|root| root == "infra/nginx"));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "apps/web/src/main.ts" && hint.to.starts_with("packages/domain/auth")
    }));
    assert!(report.dependency_hints.iter().any(|hint| {
        hint.from == "packages/domain/auth/src/index.ts"
            && hint.to.starts_with("packages/domain/shared")
    }));
}

#[test]
fn scan_repo_classifies_file_purpose_deterministically() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("package.json"), r#"{"name":"purpose-test"}"#).unwrap();
    std::fs::create_dir_all(root.join("src/routes")).unwrap();
    std::fs::create_dir_all(root.join("src/services")).unwrap();
    std::fs::create_dir_all(root.join("src/types")).unwrap();
    std::fs::create_dir_all(root.join("docs")).unwrap();
    std::fs::create_dir_all(root.join("migrations")).unwrap();

    std::fs::write(
        root.join("src/routes/index.ts"),
        "export const router = {};\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/services/payment_service.ts"),
        "export function pay() {}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/types/payment.ts"),
        "export type Payment = {};\n",
    )
    .unwrap();
    std::fs::write(root.join("docs/guide.md"), "# Guide\n").unwrap();
    std::fs::write(
        root.join("migrations/001_init.sql"),
        "create table demo(id int);\n",
    )
    .unwrap();

    let report = scan_repo(root, &[]).unwrap();

    let by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(by_path.get("package.json"), Some(&FilePurpose::Config));
    assert_eq!(
        by_path.get("src/routes/index.ts"),
        Some(&FilePurpose::Router)
    );
    assert_eq!(
        by_path.get("src/services/payment_service.ts"),
        Some(&FilePurpose::Service)
    );
    assert_eq!(
        by_path.get("src/types/payment.ts"),
        Some(&FilePurpose::Type)
    );
    assert_eq!(by_path.get("docs/guide.md"), Some(&FilePurpose::Docs));
    assert_eq!(
        by_path.get("migrations/001_init.sql"),
        Some(&FilePurpose::Migration)
    );
}
