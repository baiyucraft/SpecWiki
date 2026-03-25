//! status workflow 负责对外报告当前 runtime 是否 fresh、stale、missing 或 needs_rebuild。
//! 它复用 change planning 内核，不自己维护独立的脏判断规则。

use serde::Serialize;
use std::io;
use std::path::Path;

use crate::domain::change_set::plan_runtime_changes;

/// `status` 只回答一件事：当前 Repo Wiki 是否仍然可用且新鲜。
#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    /// 当前 runtime 的外部状态，取值为 `fresh / stale / missing / needs_rebuild`。
    pub state: String,
    /// 本次检测到的脏源码路径集合。
    pub dirty_sources: Vec<String>,
    /// 本次检测到的受影响页面路径集合。
    pub dirty_pages: Vec<String>,
    /// 当状态为 `needs_rebuild` 时，对外返回的原因说明。
    pub needs_rebuild_reason: Option<String>,
}

/// 检查 runtime 是否缺失、是否需要重建、是否存在脏源码。
/// 统一复用 change planning 内核，避免 status/update 维护两套脏判断。
///
/// # 参数
/// - `repo_root`：要检查的本地代码目录。
///
/// # 返回
/// - 成功时返回当前 Repo Wiki 的状态报告。
///
/// # 错误
/// - 当状态层、源码目录或变化规划读取失败时返回错误。
pub fn run_status(repo_root: &Path) -> io::Result<StatusReport> {
    let plan = plan_runtime_changes(repo_root)?;

    Ok(StatusReport {
        state: plan.state().to_string(),
        dirty_sources: plan.change_set.dirty_sources(),
        dirty_pages: plan.dirty_page_paths(),
        needs_rebuild_reason: plan.needs_rebuild_reason,
    })
}



