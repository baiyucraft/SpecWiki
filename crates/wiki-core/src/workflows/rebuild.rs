use serde::Serialize;
use std::io;
use std::path::Path;

use crate::workflows::init::run_init;

/// `rebuild` 是显式的“强制重建”入口。
/// 目前它直接复用 init，语义上与 update 的区别是调用方明确要求全量重来。
#[derive(Debug, Clone, Serialize)]
pub struct RebuildReport {
    pub state: String,
    pub updated_pages: Vec<String>,
}

/// 强制全量重建 Repo Wiki。
///
/// # 参数
/// - `repo_root`：要重建的本地代码目录。
///
/// # 返回
/// - 成功时返回重建后的状态和本次写出的页面列表。
///
/// # 错误
/// - 当底层初始化流程失败时返回错误。
pub fn run_rebuild(repo_root: &Path) -> io::Result<RebuildReport> {
    let init = run_init(repo_root)?;

    Ok(RebuildReport {
        state: init.state,
        updated_pages: init.generated_pages,
    })
}
