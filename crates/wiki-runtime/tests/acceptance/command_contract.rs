use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_runtime::domain::steering::SteeringLoadMode;
use wiki_runtime::transport::dto::{CoreCommand, CoreResponse};
use wiki_runtime::workflows::progress::NoopProgressSink;
use wiki_runtime::workflows::{
    init::run_init_with_progress_and_llm_as_with_mode, query::run_query,
};

struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.as_ref() {
            std::env::set_var(self.key, previous);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn write_graph_query_repo(repo_root: &Path) {
    write_repo_file(repo_root, "package.json", r#"{"name":"graph-query-demo"}"#);
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

fn write_dev_mode_init_repo(repo_root: &Path) {
    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"dev-mode-init-demo"}"#,
    );
    write_repo_file(
        repo_root,
        "src/main.ts",
        "export function handleCheckout() { return true; }\n",
    );
    write_repo_file(repo_root, ".wiki/config.yaml", "debug: {}\nllm: {}\n");
    write_repo_file(
        repo_root,
        "wiki.dev.yaml",
        "debug:\n  enabled: true\n  trace_dir: .debug/dev-mode\n",
    );
}

fn run_init_in_development(repo_root: &Path) {
    let mut sink = NoopProgressSink;
    run_init_with_progress_and_llm_as_with_mode(
        "init",
        repo_root,
        &mut sink,
        None,
        SteeringLoadMode::Development,
    )
    .unwrap();
}

#[test]
fn parses_init_command() {
    let cmd: CoreCommand = serde_json::from_str(r#"{"action":"init","repoRoot":"."}"#).unwrap();

    assert_eq!(cmd.action, "init");
    assert_eq!(cmd.repo_root.as_deref(), Some("."));
    assert!(!cmd.development_mode);
}

#[test]
fn parses_development_mode_command_flag() {
    let cmd: CoreCommand =
        serde_json::from_str(r#"{"action":"init","repoRoot":".","developmentMode":true}"#).unwrap();

    assert!(cmd.development_mode);
}

#[test]
fn serializes_error_response() {
    let response = CoreResponse::error("not_git_repo");
    let json = serde_json::to_string(&response).unwrap();

    assert!(json.contains("not_git_repo"));
}

#[test]
fn handles_json_command_and_returns_error_payload() {
    let response =
        wiki_runtime::transport::json_rpc::handle_json(r#"{"action":"unknown-action"}"#).unwrap();

    assert!(!response.ok);
    assert_eq!(
        response.error.as_deref(),
        Some("unsupported_action:unknown-action")
    );
}

#[test]
fn query_transport_returns_slim_payload_but_internal_query_stays_rich() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);
    run_init_in_development(repo_root);

    let internal = run_query(repo_root, "handleCheckout").unwrap();
    let internal_json = serde_json::to_value(&internal).unwrap();
    assert!(internal_json["matched_symbols"][0]
        .get("symbol_id")
        .is_some());
    assert!(internal_json["matched_symbol_edges"][0]
        .get("edge_id")
        .is_some());
    assert!(internal_json["matched_symbol_edges"][0]
        .get("confidence")
        .is_some());
    assert!(internal_json["matched_symbol_edges"][0]
        .get("hop_distance")
        .is_some());

    let response = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "query".to_string(),
        repo_root: Some(repo_root.display().to_string()),
        term: Some("handleCheckout".to_string()),
        development_mode: false,
        stream_progress: false,
        llm_bridge: None,
    });

    assert!(response.ok);
    let payload = response.data.expect("query transport should include data");
    assert_eq!(payload["term"], "handleCheckout");
    assert!(payload.get("query_mode").is_some());
    assert!(payload.get("query_trust").is_some());
    assert!(payload.get("recommended_action").is_some());
    assert!(payload.get("provenance_summary").is_some());
    assert!(payload.get("summary").is_some());
    assert!(payload.get("hits").is_some());
    assert!(payload["matched_pages"].is_array());

    assert!(payload.get("matched_symbols").is_none());
    assert!(payload.get("matched_sources").is_none());
    assert!(payload.get("matched_modules").is_none());
    assert!(payload.get("matched_symbol_edges").is_none());
    assert!(payload.get("matches").is_none());

    let hits = payload["hits"]
        .as_array()
        .expect("query transport should expose compact hits");
    assert!(hits.iter().any(|hit| {
        hit["hit_type"] == "symbol"
            && hit["title"] == "handleCheckout"
            && hit["location"] == "src/controller.ts:2"
    }));
    assert!(hits.iter().any(|hit| {
        hit["hit_type"] == "call_edge" && hit["title"] == "handleCheckout -> runPayment"
    }));
    let symbol_hit = hits
        .iter()
        .find(|hit| hit["hit_type"] == "symbol" && hit["title"] == "handleCheckout")
        .expect("symbol hit should be present");
    assert_eq!(symbol_hit["line_start"], 2);
    assert_eq!(symbol_hit["line_end"], 4);
}

#[test]
fn query_transport_keeps_page_provenance_inside_compact_page_hits() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    run_init_in_development(repo_root);

    let response = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "query".to_string(),
        repo_root: Some(repo_root.display().to_string()),
        term: Some("项目概述".to_string()),
        development_mode: false,
        stream_progress: false,
        llm_bridge: None,
    });

    assert!(response.ok);
    let payload = response.data.expect("query transport should include data");
    let page = payload["hits"]
        .as_array()
        .and_then(|hits| hits.iter().find(|hit| hit["hit_type"] == "page"))
        .expect("overview query should produce page match");
    assert!(page.get("provenance").is_some());
    assert!(payload.get("matches").is_none());
}

#[test]
fn init_transport_requires_provider_in_production_but_allows_explicit_development_mode() {
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let production_fixture = tempdir().unwrap();
    let production_repo_root = production_fixture.path();
    write_dev_mode_init_repo(production_repo_root);

    let production_init = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "init".to_string(),
        repo_root: Some(production_repo_root.display().to_string()),
        term: None,
        development_mode: false,
        stream_progress: false,
        llm_bridge: None,
    });
    assert!(!production_init.ok);
    assert_eq!(
        production_init.error.as_deref(),
        Some("provider research unavailable: production workflow requires llm.enabled provider_direct path")
    );
    assert!(!production_repo_root
        .join(".debug")
        .join("dev-mode")
        .join("trace.ndjson")
        .exists());

    let development_fixture = tempdir().unwrap();
    let development_repo_root = development_fixture.path();
    write_dev_mode_init_repo(development_repo_root);

    let development_init = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "init".to_string(),
        repo_root: Some(development_repo_root.display().to_string()),
        term: None,
        development_mode: true,
        stream_progress: false,
        llm_bridge: None,
    });
    assert!(
        development_init.ok,
        "development init should succeed with dev mode: {:?}",
        development_init.error
    );
    let development_payload = development_init
        .data
        .expect("development init should include payload");
    assert_eq!(development_payload["state"], "fresh");
    assert!(development_repo_root
        .join(".debug")
        .join("dev-mode")
        .join("trace.ndjson")
        .exists());
}

#[test]
fn blocker_transport_contract_never_looks_like_success() {
    let _structural_guard = EnvVarGuard::set("SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME", "0");
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_dev_mode_init_repo(repo_root);

    let init_response = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "init".to_string(),
        repo_root: Some(repo_root.display().to_string()),
        term: None,
        development_mode: false,
        stream_progress: false,
        llm_bridge: None,
    });

    assert!(!init_response.ok);
    let init_payload = init_response
        .data
        .expect("failed init should still return blocker context");
    assert!(init_payload.get("runtime_summary").is_some());
    assert!(init_payload.get("blocker_hint").is_some());

    let status_response = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: "status".to_string(),
        repo_root: Some(repo_root.display().to_string()),
        term: None,
        development_mode: false,
        stream_progress: false,
        llm_bridge: None,
    });

    assert!(status_response.ok);
    let status_payload = status_response
        .data
        .expect("status transport should include payload");
    assert_eq!(status_payload["state"], "blocker");
    assert_eq!(status_payload["query_readiness"], "blocked");
    assert_eq!(status_payload["recommended_action"], "rebuild");
    assert!(status_payload.get("blocker_hint").is_some());
}
