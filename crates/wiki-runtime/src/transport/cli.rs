use std::path::{Path, PathBuf};

use serde_json::json;

use crate::domain::runtime_profile::FusionReadiness;
use crate::domain::runtime_profile::{blocker_hint_from, RuntimeSummaryProjection};
use crate::domain::steering::SteeringLoadMode;
use crate::llm::LlmService;
use crate::transport::dto::{BootstrapOutcome, CoreCommand, CoreErrorKind, CoreResponse};
use crate::transport::query_payload::map_query_report;
use crate::workflows::archive::ArchiveService;
use crate::workflows::governance::GovernanceService;
use crate::workflows::init::run_init_with_progress_and_llm_as_with_mode;
use crate::workflows::page_render::{
    load_runtime_gate_summary_for_repo, load_runtime_summary_for_repo,
};
use crate::workflows::progress::{NoopProgressSink, ProgressSink};
use crate::workflows::query::run_query_with_mode;
use crate::workflows::rebuild::run_rebuild_with_progress_and_llm_as_with_mode;
use crate::workflows::status::run_status_with_mode;
use crate::workflows::sync::run_sync_with_mode;
use crate::workflows::update::run_update_with_progress_and_llm_as_with_mode;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum UnifiedInitOutcome {
    Ready,
    Partial,
}

#[derive(Debug, Serialize)]
struct UnifiedInitReport<TInit, TStatus> {
    outcome: UnifiedInitOutcome,
    bootstrap: crate::transport::dto::BootstrapReport,
    runtime: TInit,
    landing: TStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_hint: Option<String>,
}

/// 按 `action` 分发到具体 workflow。
/// transport 层不直接做业务判断，它只负责把协议转成 workflow 调用。
///
/// # 参数
/// - `command`：Agent 传入的核心命令协议对象。
///
/// # 返回
/// - 返回统一编码后的 `CoreResponse`。
pub fn dispatch(command: CoreCommand) -> CoreResponse {
    let mut sink = NoopProgressSink;
    dispatch_with_runtime(command, &mut sink, None)
}

/// 在 transport 已经决定启用流式协议时，允许 workflow 上报阶段进度。
pub fn dispatch_with_progress(
    command: CoreCommand,
    progress_sink: &mut dyn ProgressSink,
) -> CoreResponse {
    dispatch_with_runtime(command, progress_sink, None)
}

/// 在 transport 层同时具备进度上报和可选 LLM 桥接时的统一分发入口。
pub fn dispatch_with_runtime<'a>(
    command: CoreCommand,
    progress_sink: &'a mut dyn ProgressSink,
    mut llm_service: Option<&'a mut dyn LlmService>,
) -> CoreResponse {
    let steering_mode = resolve_steering_mode(&command);
    let repo_root = command
        .repo_root
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    // `query` 是唯一依赖 `term` 的动作；其他动作只需要 repo_root。
    match command.action.as_str() {
        "init" => encode_long_result(
            &repo_root,
            run_init_with_progress_and_llm_as_with_mode(
                "init",
                &repo_root,
                progress_sink,
                llm_service.take(),
                steering_mode,
            )
            .and_then(as_json),
        ),
        "status" => {
            encode_result(run_status_with_mode(&repo_root, steering_mode).and_then(as_json))
        }
        "cli_init" => {
            let Some(bootstrap) = command.bootstrap.clone() else {
                return CoreResponse::typed_error(
                    CoreErrorKind::InvalidArgument,
                    "cli_init requires bootstrap report",
                );
            };
            if bootstrap.outcome == BootstrapOutcome::Failed {
                return CoreResponse::typed_error_with_data(
                    CoreErrorKind::InvalidArgument,
                    "cli_init cannot start with failed bootstrap",
                    serde_json::to_value(&bootstrap).unwrap_or(serde_json::Value::Null),
                );
            }
            let init = run_init_with_progress_and_llm_as_with_mode(
                "cli_init",
                &repo_root,
                progress_sink,
                llm_service.take(),
                steering_mode,
            );
            let init = match init {
                Ok(value) => value,
                Err(error) => {
                    return CoreResponse::typed_error_with_data(
                        CoreErrorKind::WorkflowFailed,
                        error.to_string(),
                        json!({
                            "bootstrap": bootstrap,
                            "recovery_hint": "rerun spec-wiki init after resolving the runtime blocker",
                        }),
                    )
                }
            };
            let landing = match run_status_with_mode(&repo_root, steering_mode) {
                Ok(value) => value,
                Err(error) => {
                    return CoreResponse::typed_error_with_data(
                        CoreErrorKind::WorkflowFailed,
                        error.to_string(),
                        json!({
                            "bootstrap": bootstrap,
                            "runtime": init,
                            "recovery_hint": "run spec-wiki status after resolving the landing status error",
                        }),
                    )
                }
            };
            let outcome = if bootstrap.outcome == BootstrapOutcome::Ready
                && landing.readiness.fusion == FusionReadiness::Ready
            {
                UnifiedInitOutcome::Ready
            } else {
                UnifiedInitOutcome::Partial
            };
            encode_result(as_json(UnifiedInitReport {
                outcome,
                bootstrap,
                runtime: init,
                landing,
                recovery_hint: matches!(outcome, UnifiedInitOutcome::Partial)
                    .then(|| "inspect landing state and rerun the recommended action".to_string()),
            }))
        }
        "update" => encode_long_result(
            &repo_root,
            run_update_with_progress_and_llm_as_with_mode(
                "update",
                &repo_root,
                progress_sink,
                llm_service.take(),
                steering_mode,
            )
            .and_then(as_json),
        ),
        "query" => {
            let Some(term) = non_empty(command.term.as_deref()) else {
                return CoreResponse::typed_error(
                    CoreErrorKind::InvalidArgument,
                    "query requires a non-empty term",
                );
            };
            encode_query_result(
                run_query_with_mode(&repo_root, term, steering_mode)
                    .map(map_query_report)
                    .and_then(as_json),
            )
        }
        "sync" => encode_result(run_sync_with_mode(&repo_root, steering_mode).and_then(as_json)),
        "rebuild" => encode_long_result(
            &repo_root,
            run_rebuild_with_progress_and_llm_as_with_mode(
                "rebuild",
                &repo_root,
                progress_sink,
                llm_service.take(),
                steering_mode,
            )
            .and_then(as_json),
        ),
        "changes" => encode_governance_result(
            GovernanceService::new(&repo_root)
                .changes_report()
                .and_then(as_json),
        ),
        "change" => {
            let Some(change_id) = non_empty(command.change_id.as_deref()) else {
                return CoreResponse::typed_error(
                    CoreErrorKind::InvalidArgument,
                    "change requires changeId",
                );
            };
            encode_governance_result(
                GovernanceService::new(&repo_root)
                    .change_report(change_id)
                    .and_then(as_json),
            )
        }
        "validate" => {
            let Some(change_id) = non_empty(command.change_id.as_deref()) else {
                return CoreResponse::typed_error(
                    CoreErrorKind::InvalidArgument,
                    "validate requires changeId",
                );
            };
            encode_governance_result(
                GovernanceService::new(&repo_root)
                    .validate_report(change_id)
                    .and_then(as_json),
            )
        }
        "archive" => {
            let Some(change_id) = non_empty(command.change_id.as_deref()) else {
                return CoreResponse::typed_error(
                    CoreErrorKind::InvalidArgument,
                    "archive requires changeId",
                );
            };
            let mode = command
                .archive_mode
                .unwrap_or(wiki_model::domain::governance::ArchiveMode::DryRun);
            let result = match mode {
                wiki_model::domain::governance::ArchiveMode::DryRun => {
                    ArchiveService::new(&repo_root).plan(change_id)
                }
                wiki_model::domain::governance::ArchiveMode::Apply => {
                    ArchiveService::new(&repo_root).apply(change_id)
                }
                wiki_model::domain::governance::ArchiveMode::Resume => {
                    let Some(operation_id) = non_empty(command.archive_operation_id.as_deref())
                    else {
                        return CoreResponse::typed_error(
                            CoreErrorKind::InvalidArgument,
                            "archive resume requires archiveOperationId",
                        );
                    };
                    ArchiveService::new(&repo_root).resume(change_id, operation_id)
                }
            };
            encode_archive_result(result)
        }
        other => CoreResponse::error(format!("unsupported_action:{other}")),
    }
}

fn encode_archive_result(
    result: std::io::Result<wiki_model::domain::governance::ArchiveReport>,
) -> CoreResponse {
    match result {
        Ok(report) => encode_result(as_json(report)),
        Err(error) if crate::storage::archive_fs::archive_failure_kind(&error).is_some() => {
            let kind = match crate::storage::archive_fs::archive_failure_kind(&error).unwrap() {
                wiki_model::domain::governance::ArchiveErrorKind::NotReady => {
                    CoreErrorKind::ArchiveNotReady
                }
                wiki_model::domain::governance::ArchiveErrorKind::PreconditionChanged => {
                    CoreErrorKind::ArchivePreconditionChanged
                }
                wiki_model::domain::governance::ArchiveErrorKind::Conflict => {
                    CoreErrorKind::ArchiveConflict
                }
                wiki_model::domain::governance::ArchiveErrorKind::Locked => {
                    CoreErrorKind::ArchiveLocked
                }
                wiki_model::domain::governance::ArchiveErrorKind::RecoveryRequired => {
                    CoreErrorKind::ArchiveRecoveryRequired
                }
                wiki_model::domain::governance::ArchiveErrorKind::ManifestInvalid => {
                    CoreErrorKind::ArchiveManifestInvalid
                }
                wiki_model::domain::governance::ArchiveErrorKind::Io => {
                    CoreErrorKind::WorkflowFailed
                }
            };
            CoreResponse::typed_error(kind, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            CoreResponse::typed_error(CoreErrorKind::ChangeNotFound, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            CoreResponse::typed_error(CoreErrorKind::ArchiveLocked, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
            CoreResponse::typed_error(CoreErrorKind::ArchiveRecoveryRequired, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            CoreResponse::typed_error(CoreErrorKind::ArchiveConflict, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::InvalidInput => {
            CoreResponse::typed_error(CoreErrorKind::InvalidArgument, error.to_string())
        }
        Err(error)
            if error.kind() == std::io::ErrorKind::InvalidData
                && error.to_string().contains("not ready") =>
        {
            CoreResponse::typed_error(CoreErrorKind::ArchiveNotReady, error.to_string())
        }
        Err(error)
            if error.kind() == std::io::ErrorKind::InvalidData
                && error.to_string().contains("precondition") =>
        {
            CoreResponse::typed_error(CoreErrorKind::ArchivePreconditionChanged, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => {
            CoreResponse::typed_error(CoreErrorKind::ArchiveConflict, error.to_string())
        }
        Err(error) => CoreResponse::typed_error(CoreErrorKind::WorkflowFailed, error.to_string()),
    }
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn encode_governance_result(result: std::io::Result<serde_json::Value>) -> CoreResponse {
    match result {
        Ok(data) => CoreResponse::success(data),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            CoreResponse::typed_error(CoreErrorKind::ChangeNotFound, error.to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::Unsupported => {
            CoreResponse::typed_error(CoreErrorKind::GovernanceNotEnabled, error.to_string())
        }
        Err(error) => CoreResponse::typed_error(CoreErrorKind::WorkflowFailed, error.to_string()),
    }
}

fn encode_query_result(result: std::io::Result<serde_json::Value>) -> CoreResponse {
    match result {
        Ok(data) => CoreResponse::success(data),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            CoreResponse::typed_error_with_data(
                CoreErrorKind::IndexNotReady,
                error.to_string(),
                json!({
                    "reason": "facts_snapshot_missing",
                    "recommended_action": "init",
                }),
            )
        }
        Err(error) if error.kind() == std::io::ErrorKind::InvalidInput => {
            CoreResponse::typed_error(CoreErrorKind::InvalidArgument, error.to_string())
        }
        Err(error) => CoreResponse::typed_error(CoreErrorKind::WorkflowFailed, error.to_string()),
    }
}

fn resolve_steering_mode(command: &CoreCommand) -> SteeringLoadMode {
    if command.development_mode {
        SteeringLoadMode::Development
    } else {
        SteeringLoadMode::Production
    }
}

/// 把 workflow 的 `io::Result<Value>` 统一编码成 `CoreResponse`。
///
/// # 参数
/// - `result`：workflow 返回的 JSON 值结果或 I/O 错误。
///
/// # 返回
/// - 返回成功或失败都可统一落到协议层的 `CoreResponse`。
fn encode_result(result: std::io::Result<serde_json::Value>) -> CoreResponse {
    match result {
        Ok(data) => CoreResponse::success(data),
        Err(error) => CoreResponse::error(error.to_string()),
    }
}

/// 为长流程终态编码结果；失败时尽量补齐 runtime 摘要与 blocker 线索。
fn encode_long_result(
    repo_root: &Path,
    result: std::io::Result<serde_json::Value>,
) -> CoreResponse {
    match result {
        Ok(data) => CoreResponse::success(data),
        Err(error) => {
            let runtime_summary = load_runtime_summary_for_repo(repo_root)
                .ok()
                .flatten()
                .map(RuntimeSummaryProjection::from_summary);
            let gate_summary = load_runtime_gate_summary_for_repo(repo_root).ok().flatten();
            let blocker_hint = blocker_hint_from(runtime_summary.as_ref(), gate_summary.as_ref());

            if runtime_summary.is_none() && blocker_hint.is_none() {
                return CoreResponse::error(error.to_string());
            }

            let mut data = serde_json::Map::new();
            if let Some(runtime_summary) = runtime_summary {
                data.insert(
                    "runtime_summary".to_string(),
                    serde_json::to_value(runtime_summary).unwrap_or(serde_json::Value::Null),
                );
            }
            if let Some(blocker_hint) = blocker_hint {
                data.insert("blocker_hint".to_string(), json!(blocker_hint));
            }

            CoreResponse::error_with_data(error.to_string(), serde_json::Value::Object(data))
        }
    }
}

/// 把任意可序列化的 workflow 结果提升成 JSON 值。
///
/// # 参数
/// - `value`：任意实现了 `Serialize` 的 workflow 返回值。
///
/// # 返回
/// - 成功时返回 JSON 值；序列化失败时返回 I/O 错误。
fn as_json<T>(value: T) -> std::io::Result<serde_json::Value>
where
    T: serde::Serialize,
{
    serde_json::to_value(value).map_err(|err| std::io::Error::other(err.to_string()))
}
