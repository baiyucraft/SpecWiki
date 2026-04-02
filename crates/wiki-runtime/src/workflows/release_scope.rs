//! `release_scope` 只保留公开状态投影入口。
//! `v0.2.0` 起不再把 facts-only runtime 改写成额外成功态。

use std::path::Path;

/// 把内部 runtime 状态投影成公开状态。
/// 当前 `v0.2.0` 合同下不再额外生成 `index_only` 一类成功态。
pub fn project_external_runtime_state(
    _repo_root: &Path,
    planned_state: &str,
    _facts_ready: bool,
) -> String {
    planned_state.to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::project_external_runtime_state;

    #[test]
    fn release_scope_keeps_planned_state_without_index_only_projection() {
        let repo_root = Path::new(".");
        assert_eq!(
            project_external_runtime_state(repo_root, "missing", true),
            "missing"
        );
        assert_eq!(
            project_external_runtime_state(repo_root, "fresh", true),
            "fresh"
        );
    }
}
