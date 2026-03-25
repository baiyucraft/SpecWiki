//! `debug_trace` 模块负责在显式开启时记录 workflow/LLM 调试轨迹。
//! 它只向文件和 `stderr` 写日志，不参与 stdout JSON IPC 的正式协议。

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::{json, Value};

use crate::domain::steering::DebugConfig;
use crate::transport::dto::CoreEvent;

/// 启动参数层的 debug trace 选项。
/// 这层配置在进程启动时解析，后续由 workflow 与 steering 配置合并。
#[derive(Debug, Clone, Default)]
pub struct StartupDebugTraceOptions {
    /// 是否显式要求开启 debug trace。
    pub enabled: bool,
    /// 期望写入的 trace 目录；为空时走 workflow 默认目录。
    pub trace_dir: Option<PathBuf>,
    /// 是否把 trace 条目实时镜像到 `stderr`。
    pub echo_to_stderr: bool,
}

struct DebugTraceSession {
    trace_path: PathBuf,
    echo_to_stderr: bool,
    file: File,
}

#[derive(Default)]
struct DebugTraceState {
    startup: StartupDebugTraceOptions,
    session: Option<DebugTraceSession>,
}

#[derive(Debug, Clone)]
struct ResolvedDebugTraceOptions {
    enabled: bool,
    trace_dir: Option<PathBuf>,
    echo_to_stderr: bool,
}

fn state() -> &'static Mutex<DebugTraceState> {
    static STATE: OnceLock<Mutex<DebugTraceState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(DebugTraceState::default()))
}

/// 注册本进程启动参数层的 debug trace 选项。
pub fn set_startup_options(options: StartupDebugTraceOptions) {
    let mut state = state().lock().unwrap();
    state.startup = options;
    state.session = None;
}

/// 清理启动参数与当前 session。
/// 测试会复用同一进程，因此需要显式重置全局状态。
pub fn clear_startup_options() {
    set_startup_options(StartupDebugTraceOptions::default());
}

/// 基于 steering 配置和启动参数决定是否开启当前 workflow 的 trace session。
///
/// # 参数
/// - `action`：当前 workflow 动作名。
/// - `repo_root`：当前仓库根目录；相对 trace 目录会基于它解析。
/// - `config`：来自 steering 的 debug 配置。
///
/// # 返回
/// - 开启时返回 `trace.ndjson` 路径；未开启时返回 `None`。
///
/// # 错误
/// - 当显式开启了 debug trace 但 trace 目录不可创建时返回错误。
pub fn begin_session(
    action: &str,
    repo_root: &Path,
    config: &DebugConfig,
) -> io::Result<Option<PathBuf>> {
    let mut state = state().lock().unwrap();
    let resolved = resolve_options(&state.startup, config);
    if !resolved.enabled {
        state.session = None;
        return Ok(None);
    }

    if let Some(session) = state.session.as_ref() {
        return Ok(Some(session.trace_path.clone()));
    }

    let output_dir = resolve_output_dir(repo_root, action, resolved.trace_dir.as_deref());
    fs::create_dir_all(&output_dir)?;
    let trace_path = output_dir.join("trace.ndjson");
    let session = DebugTraceSession {
        trace_path: trace_path.clone(),
        echo_to_stderr: resolved.echo_to_stderr,
        file: File::create(&trace_path)?,
    };
    state.session = Some(session);
    drop(state);

    record_json(
        "session_start",
        &json!({
            "action": action,
            "repo_root": repo_root.display().to_string(),
            "trace_dir": output_dir.display().to_string(),
            "trace_path": trace_path.display().to_string(),
        }),
    );
    Ok(Some(trace_path))
}

/// 记录一条完整的 core 事件。
pub fn record_core_event(event: &CoreEvent) {
    record_serializable("core_event", event);
}

/// 记录任意可序列化 payload。
pub fn record_serializable<T>(kind: &str, payload: &T)
where
    T: Serialize,
{
    let Ok(value) = serde_json::to_value(payload) else {
        return;
    };
    record_json(kind, &value);
}

/// 记录一条通用 JSON trace。
pub fn record_json(kind: &str, payload: &Value) {
    let mut state = state().lock().unwrap();
    let Some(session) = state.session.as_mut() else {
        return;
    };

    let entry = json!({
        "ts_ms": current_timestamp_ms(),
        "kind": kind,
        "payload": payload,
    });
    if serde_json::to_writer(&mut session.file, &entry).is_err() {
        return;
    }
    if session.file.write_all(b"\n").is_err() {
        return;
    }
    let _ = session.file.flush();

    if session.echo_to_stderr {
        if let Ok(text) = serde_json::to_string_pretty(&entry) {
            eprintln!("{text}");
        }
    }
}

fn resolve_options(
    startup: &StartupDebugTraceOptions,
    config: &DebugConfig,
) -> ResolvedDebugTraceOptions {
    let config_requested =
        config.enabled || !config.trace_dir.trim().is_empty() || config.echo_to_stderr;
    let startup_requested =
        startup.enabled || startup.trace_dir.is_some() || startup.echo_to_stderr;

    ResolvedDebugTraceOptions {
        enabled: startup_requested || config_requested,
        trace_dir: startup.trace_dir.clone().or_else(|| {
            (!config.trace_dir.trim().is_empty()).then_some(PathBuf::from(config.trace_dir.trim()))
        }),
        echo_to_stderr: startup.echo_to_stderr || config.echo_to_stderr,
    }
}

fn resolve_output_dir(repo_root: &Path, action: &str, explicit_dir: Option<&Path>) -> PathBuf {
    if let Some(explicit_dir) = explicit_dir {
        return if explicit_dir.is_absolute() {
            explicit_dir.to_path_buf()
        } else {
            repo_root.join(explicit_dir)
        };
    }

    repo_root
        .join(".debug")
        .join("wiki-runtime")
        .join(format!("{action}-{}", current_timestamp_ms()))
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}



