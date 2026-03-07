use std::path::PathBuf;

use crate::app::init::run_init;
use crate::app::query::run_query;
use crate::app::rebuild::run_rebuild;
use crate::app::status::run_status;
use crate::app::sync::run_sync;
use crate::app::update::run_update;
use crate::transport::dto::{CoreCommand, CoreResponse};

pub fn dispatch(command: CoreCommand) -> CoreResponse {
    let repo_root = command
        .repo_root
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

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

fn encode_result(result: std::io::Result<serde_json::Value>) -> CoreResponse {
    match result {
        Ok(data) => CoreResponse::success(data),
        Err(error) => CoreResponse::error(error.to_string()),
    }
}

fn as_json<T>(value: T) -> std::io::Result<serde_json::Value>
where
    T: serde::Serialize,
{
    serde_json::to_value(value).map_err(|err| std::io::Error::other(err.to_string()))
}
