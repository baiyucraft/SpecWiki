use std::path::PathBuf;

use crate::workflows::init::run_init;
use crate::workflows::query::run_query;
use crate::workflows::rebuild::run_rebuild;
use crate::workflows::status::run_status;
use crate::workflows::sync::run_sync;
use crate::workflows::update::run_update;
use crate::transport::dto::{CoreCommand, CoreResponse};

/// 按 `action` 分发到具体 workflow。
/// transport 层不直接做业务判断，它只负责把协议转成 workflow 调用。
///
/// # 参数
/// - `command`：Agent 传入的核心命令协议对象。
///
/// # 返回
/// - 返回统一编码后的 `CoreResponse`。
pub fn dispatch(command: CoreCommand) -> CoreResponse {
    let repo_root = command
        .repo_root
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    // `query` 是唯一依赖 `term` 的动作；其他动作只需要 repo_root。
    match command.action.as_str() {
        "init" => encode_result(run_init(&repo_root).and_then(as_json)),
        "status" => encode_result(run_status(&repo_root).and_then(as_json)),
        "update" => encode_result(run_update(&repo_root).and_then(as_json)),
        "query" => encode_result(run_query(&repo_root, command.term.as_deref().unwrap_or("")).and_then(as_json)),
        "sync" => encode_result(run_sync(&repo_root).and_then(as_json)),
        "rebuild" => encode_result(run_rebuild(&repo_root).and_then(as_json)),
        other => CoreResponse::error(format!("unsupported_action:{other}")),
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
