use serde::Serialize;
use std::io;
use std::path::Path;

use crate::workflows::init::run_init;
use crate::workflows::status::run_status;

/// `update` 当前还是“检测后重跑 init”的实现。
/// 它先保留稳定语义，后续再演进为真正的增量更新。
#[derive(Debug, Clone, Serialize)]
pub struct UpdateReport {
    pub previous_state: String,
    pub state: String,
    pub updated_pages: Vec<String>,
}

/// 更新 Repo Wiki。
/// 如果已经是 `fresh`，直接返回；否则走一次全量重建链。
///
/// # 参数
/// - `repo_root`：要更新的本地代码目录。
///
/// # 返回
/// - 成功时返回更新前状态、更新后状态以及本次更新的页面列表。
///
/// # 错误
/// - 当状态检查失败，或后续全量重建失败时返回错误。
pub fn run_update(repo_root: &Path) -> io::Result<UpdateReport> {
    let status = run_status(repo_root)?;

    if status.state == "fresh" {
        return Ok(UpdateReport {
            previous_state: status.state.clone(),
            state: status.state,
            updated_pages: Vec::new(),
        });
    }

    let init = run_init(repo_root)?;

    Ok(UpdateReport {
        previous_state: status.state,
        state: init.state,
        updated_pages: init.generated_pages,
    })
}
