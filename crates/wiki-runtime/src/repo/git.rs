use std::fs;
use std::path::Path;

/// 是否存在 `.git` 目录。
/// 现在 Git 只是可选元信息来源，所以这个判断不能再作为 init 的前置条件。
///
/// # 参数
/// - `path`：本地代码目录。
///
/// # 返回
/// - 如果目录下存在 `.git`，则返回 `true`。
pub fn looks_like_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

/// 读取当前分支名。
/// 如果目录不是 Git 仓库，就返回 `unknown`，而不是报错中断主流程。
///
/// # 参数
/// - `path`：本地代码目录。
///
/// # 返回
/// - 返回当前分支名；没有 Git 信息时返回 `unknown` 或 `detached`。
pub fn current_branch(path: &Path) -> String {
    let head = read_head(path);

    match head {
        Some(value) if value.starts_with("ref: ") => value
            .trim_start_matches("ref: ")
            .rsplit('/')
            .next()
            .unwrap_or("unknown")
            .to_string(),
        Some(_) => "detached".to_string(),
        None => "unknown".to_string(),
    }
}

/// 读取当前 commit。
/// 没有 Git 信息时返回 `unindexed`，用于明确区分“仓库存在但没有版本控制元信息”。
///
/// # 参数
/// - `path`：本地代码目录。
///
/// # 返回
/// - 返回当前 commit 哈希；没有 Git 信息时返回 `unindexed`。
pub fn current_commit(path: &Path) -> String {
    let Some(head) = read_head(path) else {
        return "unindexed".to_string();
    };

    if let Some(reference) = head.strip_prefix("ref: ") {
        let ref_path = path.join(".git").join(reference);
        return fs::read_to_string(ref_path)
            .map(|value| value.trim().to_string())
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unindexed".to_string());
    }

    if head.trim().is_empty() {
        return "unindexed".to_string();
    }

    head.trim().to_string()
}

/// 读取 `.git/HEAD`，并做最基础的空值过滤。
///
/// # 参数
/// - `path`：本地代码目录。
///
/// # 返回
/// - 如果成功读到有效的 HEAD 内容，则返回对应字符串。
fn read_head(path: &Path) -> Option<String> {
    fs::read_to_string(path.join(".git/HEAD"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

