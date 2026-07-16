use std::io;
use std::path::PathBuf;

use tempfile::tempdir;
use wiki_index::scanner::{scan_repo, scan_repo_with_boundary_and_assist, FilePurpose};
use wiki_runtime::llm::{LlmCompletion, LlmPromptRequest, LlmRuntime, LlmService};

fn mock_completion(output: serde_json::Value, model: &str) -> LlmCompletion {
    LlmCompletion {
        output,
        model: Some(model.to_string()),
        usage: None,
    }
}

#[derive(Default)]
struct ScanBatchFilePurposeService {
    calls: usize,
}

impl LlmService for ScanBatchFilePurposeService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        self.calls += 1;
        let items = request
            .input
            .get("items")
            .and_then(serde_json::Value::as_array)
            .cloned()
            .unwrap_or_default();
        let output = serde_json::json!({
            "items": items.into_iter().filter_map(|item| {
                let path = item.get("path")?.as_str()?;
                let purpose = if path.contains("middleware") {
                    "middleware"
                } else if path.contains("helper") || path.contains("promote") {
                    "helper"
                } else {
                    "utility"
                };
                Some(serde_json::json!({
                    "path": path,
                    "purpose": purpose,
                }))
            }).collect::<Vec<_>>()
        });

        Ok(mock_completion(output, "scan-batch-model"))
    }
}

#[derive(Default)]
struct StructuralOverrideService;

impl LlmService for StructuralOverrideService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        let output = match request.prompt_type.as_str() {
            "file_purpose" => {
                let items = request
                    .input
                    .get("items")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                serde_json::json!({
                    "items": items.into_iter().filter_map(|item| {
                        let path = item.get("path")?.as_str()?;
                        let purpose = if path.ends_with("mux.go") {
                            "router"
                        } else if path.ends_with("Makefile") || path.ends_with("wiki.dev.yaml") {
                            "config"
                        } else {
                            "utility"
                        };
                        Some(serde_json::json!({
                            "path": path,
                            "purpose": purpose,
                        }))
                    }).collect::<Vec<_>>()
                })
            }
            _ => serde_json::json!({}),
        };

        Ok(mock_completion(output, "structural-override-model"))
    }
}

#[test]
fn scan_repo_discovers_files_and_detected_stack() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-node-repo"),
        PathBuf::from("crates/wiki-runtime/tests/fixtures/minimal-node-repo"),
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
        PathBuf::from("crates/wiki-runtime/tests/fixtures/mixed-local-repo"),
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
        PathBuf::from("crates/wiki-runtime/tests/fixtures/manifest-monorepo"),
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
        PathBuf::from("crates/wiki-runtime/tests/fixtures/rust-workspace"),
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
        PathBuf::from("crates/wiki-runtime/tests/fixtures/java-modules"),
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
        PathBuf::from("crates/wiki-runtime/tests/fixtures/baseline-hierarchy-repo"),
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

#[test]
fn scan_repo_batches_utility_file_purpose_candidates() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::create_dir_all(root.join("src/promote")).unwrap();
    std::fs::write(
        root.join("src/promote/logger.go"),
        "package promote\nfunc Logger(next Handler) {}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/promote/auth.go"),
        "package promote\nfunc RequireAuth(next Handler) {}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/promote/strings.go"),
        "package promote\nfunc Join(parts []string) string {}\n",
    )
    .unwrap();

    let config = wiki_runtime::domain::steering::LlmConfig {
        enabled: true,
        model: "bridge/mock-model".to_string(),
        max_calls: 8,
        cache_ttl_seconds: 60 * 60,
        allow_mermaid: true,
        providers: std::collections::BTreeMap::new(),
        ..wiki_runtime::domain::steering::LlmConfig::default()
    };
    let mut service = ScanBatchFilePurposeService::default();
    let mut runtime = LlmRuntime::new(root, &config, Some(&mut service));
    let report = scan_repo_with_boundary_and_assist(root, &[], &[], Some(&mut runtime)).unwrap();
    drop(runtime);

    let by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(service.calls, 1);
    assert_eq!(
        by_path.get("src/promote/logger.go"),
        Some(&FilePurpose::Helper)
    );
    assert_eq!(
        by_path.get("src/promote/auth.go"),
        Some(&FilePurpose::Helper)
    );
    assert_eq!(
        by_path.get("src/promote/strings.go"),
        Some(&FilePurpose::Utility)
    );
}

#[test]
fn scan_repo_detects_go_stack_from_go_mod_and_source_files() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::write(
        root.join("main.go"),
        concat!(
            "package main\n\n",
            "func main() {\n",
            "  println(\"demo\")\n",
            "}\n",
        ),
    )
    .unwrap();

    let report = scan_repo(root, &[]).unwrap();

    assert!(report.tech_hints.iter().any(|topic| topic == "backend"));
    assert!(report.tech_hints.iter().any(|topic| topic == "go"));
}

#[test]
fn llm_file_purpose_override_does_not_change_structural_sets() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::write(root.join("Makefile"), "build:\n\tgo test ./...\n").unwrap();
    std::fs::write(root.join("wiki.dev.yaml"), "llm:\n  enabled: true\n").unwrap();
    std::fs::write(root.join("main.go"), "package main\nfunc main() {}\n").unwrap();
    std::fs::write(root.join("mux.go"), "package main\nfunc routeMux() {}\n").unwrap();

    let deterministic = scan_repo(root, &[]).unwrap();
    let config = wiki_runtime::domain::steering::LlmConfig {
        enabled: true,
        model: "bridge/mock-model".to_string(),
        max_calls: 8,
        cache_ttl_seconds: 60 * 60,
        allow_mermaid: true,
        providers: std::collections::BTreeMap::new(),
        ..wiki_runtime::domain::steering::LlmConfig::default()
    };
    let mut service = StructuralOverrideService;
    let mut runtime = LlmRuntime::new(root, &config, Some(&mut service));
    let llm_report =
        scan_repo_with_boundary_and_assist(root, &[], &[], Some(&mut runtime)).unwrap();
    drop(runtime);

    let by_path = llm_report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(by_path.get("Makefile"), Some(&FilePurpose::Config));
    assert_eq!(by_path.get("wiki.dev.yaml"), Some(&FilePurpose::Config));
    assert_eq!(by_path.get("mux.go"), Some(&FilePurpose::Router));
    assert_eq!(llm_report.config_files, deterministic.config_files);
    assert_eq!(llm_report.entry_points, deterministic.entry_points);
}

#[test]
fn scan_repo_batches_all_utility_file_purpose_candidates_without_hard_cap() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::create_dir_all(root.join("src/promote")).unwrap();
    for index in 0..66 {
        std::fs::write(
            root.join(format!("src/promote/candidate-{index:02}.go")),
            "package promote\nfunc RequireAuth(next Handler) {}\n",
        )
        .unwrap();
    }

    let config = wiki_runtime::domain::steering::LlmConfig {
        enabled: true,
        model: "bridge/mock-model".to_string(),
        max_calls: 16,
        cache_ttl_seconds: 60 * 60,
        allow_mermaid: true,
        providers: std::collections::BTreeMap::new(),
        ..wiki_runtime::domain::steering::LlmConfig::default()
    };
    let mut service = ScanBatchFilePurposeService::default();
    let mut runtime = LlmRuntime::new(root, &config, Some(&mut service));
    let report = scan_repo_with_boundary_and_assist(root, &[], &[], Some(&mut runtime)).unwrap();
    drop(runtime);

    let by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(service.calls, 5);
    assert_eq!(
        by_path.get("src/promote/candidate-00.go"),
        Some(&FilePurpose::Helper)
    );
    assert_eq!(
        by_path.get("src/promote/candidate-63.go"),
        Some(&FilePurpose::Helper)
    );
    assert_eq!(
        by_path.get("src/promote/candidate-64.go"),
        Some(&FilePurpose::Helper)
    );
    assert_eq!(
        by_path.get("src/promote/candidate-65.go"),
        Some(&FilePurpose::Helper)
    );
}

#[test]
fn scan_repo_classifies_build_tooling_files_as_config() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::create_dir_all(root.join("app/src/main")).unwrap();
    std::fs::write(
        root.join("BUILD"),
        r#"java_library(name = "demo")
"#,
    )
    .unwrap();
    std::fs::write(
        root.join("build_defs.bzl"),
        r#"def wiki_rule():
    pass
"#,
    )
    .unwrap();
    std::fs::write(
        root.join("build.gradle.kts"),
        r#"plugins { kotlin("jvm") }
"#,
    )
    .unwrap();
    std::fs::write(
        root.join("settings.gradle.kts"),
        r#"rootProject.name = "demo"
"#,
    )
    .unwrap();
    std::fs::write(
        root.join("gradle.properties"),
        "org.gradle.jvmargs=-Xmx2g\n",
    )
    .unwrap();
    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::write(
        root.join("app/src/main/AndroidManifest.xml"),
        r#"<manifest package="demo.app" />
"#,
    )
    .unwrap();

    let report = scan_repo(root, &[]).unwrap();
    let by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(by_path.get("BUILD"), Some(&FilePurpose::Config));
    assert_eq!(by_path.get("build_defs.bzl"), Some(&FilePurpose::Config));
    assert_eq!(by_path.get("build.gradle.kts"), Some(&FilePurpose::Config));
    assert_eq!(
        by_path.get("settings.gradle.kts"),
        Some(&FilePurpose::Config)
    );
    assert_eq!(by_path.get("gradle.properties"), Some(&FilePurpose::Config));
    assert_eq!(by_path.get("go.mod"), Some(&FilePurpose::Config));
    assert_eq!(
        by_path.get("app/src/main/AndroidManifest.xml"),
        Some(&FilePurpose::Config)
    );
}

#[test]
fn scan_repo_skips_story_files_but_keeps_internal_bootstrap_reviewable() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();

    std::fs::write(root.join("go.mod"), "module example.com/demo\n\ngo 1.23\n").unwrap();
    std::fs::create_dir_all(root.join("src/internal")).unwrap();
    std::fs::write(
        root.join("src/internal/bootstrap.go"),
        "package internal\nfunc Register(router Router) {}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/Button.stories.tsx"),
        "export const Basic = () => <Button />;\n",
    )
    .unwrap();
    std::fs::write(
        root.join("build.gradle.kts"),
        r#"plugins { kotlin("jvm") }
"#,
    )
    .unwrap();

    let config = wiki_runtime::domain::steering::LlmConfig {
        enabled: true,
        model: "bridge/mock-model".to_string(),
        max_calls: 8,
        cache_ttl_seconds: 60 * 60,
        allow_mermaid: true,
        providers: std::collections::BTreeMap::new(),
        ..wiki_runtime::domain::steering::LlmConfig::default()
    };
    let mut service = ScanBatchFilePurposeService::default();
    let mut runtime = LlmRuntime::new(root, &config, Some(&mut service));
    let report = scan_repo_with_boundary_and_assist(root, &[], &[], Some(&mut runtime)).unwrap();
    drop(runtime);

    let by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.purpose))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(service.calls, 1);
    assert_eq!(
        by_path.get("src/Button.stories.tsx"),
        Some(&FilePurpose::Test)
    );
    assert_eq!(by_path.get("build.gradle.kts"), Some(&FilePurpose::Config));
    assert_eq!(
        by_path.get("src/internal/bootstrap.go"),
        Some(&FilePurpose::Utility)
    );
}
