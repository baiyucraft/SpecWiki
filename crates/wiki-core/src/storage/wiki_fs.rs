use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::domain::steering::LlmCacheMode;

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
pub fn remove_runtime(repo_root: &Path) -> io::Result<()> {
    remove_runtime_with_cache_mode(repo_root, LlmCacheMode::Clear)
}

/// 在重新 init / rebuild 前清理 runtime，并按 cache mode 处理 LLM cache。
pub fn remove_runtime_with_cache_mode(
    repo_root: &Path,
    cache_mode: LlmCacheMode,
) -> io::Result<()> {
    let wiki_root = wiki_root(repo_root);
    let preserved_llm_cache = match cache_mode {
        LlmCacheMode::Preserve | LlmCacheMode::Refresh => {
            crate::storage::sqlite_store::load_all_llm_cache(repo_root)?
        }
        LlmCacheMode::Clear => Vec::new(),
    };

    if wiki_root.exists() {
        remove_dir_all_with_retry(&wiki_root)?;
    }

    if !preserved_llm_cache.is_empty() {
        crate::storage::cache_store::ensure_cache_dir(repo_root)?;
        crate::storage::sqlite_store::restore_llm_cache(repo_root, &preserved_llm_cache)?;
    }

    Ok(())
}

/// 只删除 SQLite 缓存数据库，保留 `.wiki/*.md` 页面文件和 metadata。
/// rebuild 在读取旧页面内容后调用此函数清理缓存。
pub fn remove_cache_db(repo_root: &Path) -> io::Result<()> {
    let db = crate::storage::sqlite_store::db_path(repo_root);
    if db.exists() {
        remove_file_with_retry(&db)?;
    }
    // 清理 WAL / SHM 残留
    let wal = db.with_extension("db-wal");
    if wal.exists() {
        remove_file_with_retry(&wal)?;
    }
    let shm = db.with_extension("db-shm");
    if shm.exists() {
        remove_file_with_retry(&shm)?;
    }
    Ok(())
}

fn remove_dir_all_with_retry(path: &Path) -> io::Result<()> {
    retry_fs_remove(|| fs::remove_dir_all(path))
}

fn remove_file_with_retry(path: &Path) -> io::Result<()> {
    retry_fs_remove(|| fs::remove_file(path))
}

fn retry_fs_remove<F>(mut action: F) -> io::Result<()>
where
    F: FnMut() -> io::Result<()>,
{
    let mut last_error = None;

    for attempt in 0..6 {
        match action() {
            Ok(()) => return Ok(()),
            Err(error) if is_retryable_fs_remove_error(&error) && attempt < 5 => {
                last_error = Some(error);
                thread::sleep(Duration::from_millis(250));
            }
            Err(error) => return Err(error),
        }
    }

    Err(last_error.unwrap_or_else(|| io::Error::other("remove retry failed")))
}

fn is_retryable_fs_remove_error(error: &io::Error) -> bool {
    matches!(error.raw_os_error(), Some(5 | 32))
}
