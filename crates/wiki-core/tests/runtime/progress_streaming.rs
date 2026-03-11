use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_core::debug_trace::{clear_startup_options, set_startup_options, StartupDebugTraceOptions};
use wiki_core::transport::dto::CoreEvent;
use wiki_core::transport::json_rpc::{handle_json_stream, should_stream};
use wiki_core::workflows::progress::{ProgressSink, WorkflowProgressEvent};
use wiki_core::workflows::{init::run_init, update::run_update_with_progress_as};

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

#[derive(Default)]
struct ProgressCollector {
    events: Vec<WorkflowProgressEvent>,
}

impl ProgressSink for ProgressCollector {
    fn report(&mut self, event: WorkflowProgressEvent) {
        self.events.push(event);
    }
}

#[test]
fn json_rpc_streams_init_progress_with_single_terminal_event() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"demo"}"#);
    write_repo_file(
        repo_root,
        "src/index.ts",
        "export function settlePayment() { return true; }\n",
    );

    let mut output = Vec::new();
    let command = format!(
        r#"{{"action":"init","repoRoot":"{}"}}"#,
        repo_root.display().to_string().replace('\\', "/")
    );
    handle_json_stream(&command, &mut output).unwrap();

    let text = String::from_utf8(output).unwrap();
    let events = text
        .lines()
        .map(|line| serde_json::from_str::<CoreEvent>(line).unwrap())
        .collect::<Vec<_>>();

    let phases = events
        .iter()
        .filter_map(|event| match event {
            CoreEvent::Progress(progress) => Some(progress.phase.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let terminals = events
        .iter()
        .filter(|event| !matches!(event, CoreEvent::Progress(_)))
        .collect::<Vec<_>>();

    assert!(
        !phases.is_empty(),
        "expected progress events, got {events:#?}"
    );
    assert_eq!(terminals.len(), 1, "expected a single terminal event");
    assert!(matches!(events.last(), Some(CoreEvent::Result { .. })));

    let first_occurrence = [
        "scan",
        "parse_symbols",
        "resolve_symbol_graph",
        "analyze_symbol_graph",
        "build_module_tree",
        "build_contexts",
        "plan_pages",
        "render_pages",
        "write_state",
        "write_metadata",
    ]
    .iter()
    .map(|phase| {
        phases
            .iter()
            .position(|candidate| candidate == phase)
            .unwrap_or(usize::MAX)
    })
    .collect::<Vec<_>>();

    assert!(
        first_occurrence
            .windows(2)
            .all(|window| window[0] < window[1]),
        "unexpected phase order: {phases:?}"
    );
}

#[test]
fn json_rpc_streams_terminal_error_once_for_invalid_repo() {
    let mut output = Vec::new();
    handle_json_stream(
        r#"{"action":"init","repoRoot":"E:/missing-repo"}"#,
        &mut output,
    )
    .unwrap();

    let text = String::from_utf8(output).unwrap();
    let events = text
        .lines()
        .map(|line| serde_json::from_str::<CoreEvent>(line).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(events.len(), 1, "invalid repo should terminate immediately");
    assert!(matches!(events[0], CoreEvent::Error { .. }));
}

#[test]
fn should_stream_treats_long_running_actions_as_ndjson_by_default() {
    assert!(should_stream(
        r#"{"action":"init","repoRoot":"tmp/test/demo"}"#
    ));
    assert!(should_stream(
        r#"{"action":"update","repoRoot":"tmp/test/demo"}"#
    ));
    assert!(should_stream(
        r#"{"action":"rebuild","repoRoot":"tmp/test/demo"}"#
    ));
    assert!(!should_stream(
        r#"{"action":"query","repoRoot":"tmp/test/demo","term":"demo"}"#
    ));
}

#[test]
fn update_reports_scoped_symbol_edge_refresh_for_small_graph_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(repo_root, "package.json", r#"{"name":"graph-demo"}"#);
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

    run_init(repo_root).unwrap();
    write_repo_file(
        repo_root,
        "src/shared.ts",
        "export function finalizePayment() { return \"ok\"; }\n",
    );

    let mut collector = ProgressCollector::default();
    run_update_with_progress_as("update", repo_root, &mut collector).unwrap();

    assert!(collector.events.iter().any(|event| {
        event.phase == "plan_changes" && event.message.contains("局部 symbol/edge 工作集")
    }));
}

#[test]
fn json_rpc_can_write_debug_trace_without_polluting_stdout_protocol() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let trace_root = tempdir().unwrap();

    write_repo_file(repo_root, "package.json", r#"{"name":"debug-demo"}"#);
    write_repo_file(
        repo_root,
        "src/index.ts",
        "export function renderDebugPage() { return true; }\n",
    );

    set_startup_options(StartupDebugTraceOptions {
        enabled: true,
        trace_dir: Some(trace_root.path().join("trace-run")),
        echo_to_stderr: false,
    });

    let mut output = Vec::new();
    let command = format!(
        r#"{{"action":"init","repoRoot":"{}"}}"#,
        repo_root.display().to_string().replace('\\', "/")
    );
    handle_json_stream(&command, &mut output).unwrap();
    clear_startup_options();

    let text = String::from_utf8(output).unwrap();
    let events = text
        .lines()
        .map(|line| serde_json::from_str::<CoreEvent>(line).unwrap())
        .collect::<Vec<_>>();
    let trace_path = trace_root.path().join("trace-run").join("trace.ndjson");
    let trace_text = fs::read_to_string(&trace_path).unwrap();

    assert!(matches!(events.last(), Some(CoreEvent::Result { .. })));
    assert!(trace_text.contains("\"kind\":\"session_start\""));
    assert!(trace_text.contains("\"kind\":\"core_event\""));
}
