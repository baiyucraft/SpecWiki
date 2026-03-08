use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 把页面内容写到 `.wiki/` 下的目标路径。
/// 页面路径由 planner 决定，这里只负责落盘。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `relative_path`：相对于 `.wiki/` 的页面路径。
/// - `content`：要写入页面的 Markdown 文本。
///
/// # 返回
/// - 成功时把页面写到目标路径。
///
/// # 错误
/// - 当目录创建或文件写入失败时返回错误。
pub fn write_page(repo_root: &Path, relative_path: &str, content: &str) -> io::Result<()> {
    let target = wiki_root(repo_root).join(relative_path);

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(target, content)
}

/// `.wiki/` 是 Repo Wiki 的运行时根目录。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 返回 `.wiki/` 目录路径。
pub fn wiki_root(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki")
}

/// 把 metadata 里记录的页面路径转成实际磁盘路径。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_path`：metadata 中记录的页面路径。
///
/// # 返回
/// - 返回页面对应的磁盘路径。
pub fn resolve_page_path(repo_root: &Path, page_path: &str) -> PathBuf {
    wiki_root(repo_root).join(page_path.trim_start_matches(".wiki/"))
}

/// 判断页面文件是否真的存在。
///
/// # 参数
/// - `repo_root`：本地代码目录。
/// - `page_path`：metadata 中记录的页面路径。
///
/// # 返回
/// - 如果页面文件存在，则返回 `true`。
pub fn page_exists(repo_root: &Path, page_path: &str) -> bool {
    resolve_page_path(repo_root, page_path).exists()
}

/// 在重新 init / rebuild 前清理整个 runtime。
/// 这是当前全量重建策略的基础动作。
///
/// # 参数
/// - `repo_root`：本地代码目录。
///
/// # 返回
/// - 成功时删除已有的 `.wiki/` 运行时目录。
///
/// # 错误
/// - 当删除已有 runtime 失败时返回错误。
pub fn remove_runtime(repo_root: &Path) -> io::Result<()> {
    let wiki_root = wiki_root(repo_root);

    if wiki_root.exists() {
        fs::remove_dir_all(wiki_root)?;
    }

    Ok(())
}
