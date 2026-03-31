//! 发布收敛开关只服务当前 `v0.1.0` 的 index-only 版本。
//! 后续完整 runtime 发布时，这个模块应随短路逻辑一起删除。

use std::io;
use std::path::Path;

use crate::storage::sqlite_store;

/// 当前临时发布收敛开关。
pub const V0_1_INDEX_ONLY_ENV: &str = "SPEC_WIKI_V0_1_INDEX_ONLY";
const RELEASE_SCOPE_META_KEY: &str = "release_scope";
pub const INDEX_ONLY_RUNTIME_STATE: &str = "index_only";

/// 当前 action 是否启用了 `v0.1.0 index-only` 短路。
pub fn v0_1_index_only_enabled(action: &str) -> bool {
    matches!(action, "init" | "update") && env_flag_enabled(V0_1_INDEX_ONLY_ENV)
}

/// 在 facts snapshot 已经提交的前提下，持久化当前 release scope。
pub fn persist_index_only_release_scope(repo_root: &Path) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::runtime_meta_set(&conn, RELEASE_SCOPE_META_KEY, INDEX_ONLY_RUNTIME_STATE)
}

/// 把内部脏状态投影成当前 `v0.1.0` 下的对外状态。
/// 只要 facts/index 已可用，而 knowledge/page runtime 还没形成稳定外部状态，
/// 宿主侧统一都看到 `index_only`，避免继续暴露 `missing + facts_ready` 双重语义。
pub fn project_external_runtime_state(
    _repo_root: &Path,
    planned_state: &str,
    facts_ready: bool,
) -> String {
    if planned_state == "missing" && facts_ready {
        return INDEX_ONLY_RUNTIME_STATE.to_string();
    }

    planned_state.to_string()
}

fn env_flag_enabled(key: &str) -> bool {
    let Some(raw) = std::env::var_os(key) else {
        return false;
    };

    match raw.to_string_lossy().trim().to_ascii_lowercase().as_str() {
        "" | "0" | "false" | "off" | "no" => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::storage::sqlite_store;

    use std::sync::Mutex;

    use super::{
        persist_index_only_release_scope, project_external_runtime_state, v0_1_index_only_enabled,
        INDEX_ONLY_RUNTIME_STATE, V0_1_INDEX_ONLY_ENV,
    };

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn index_only_flag_is_action_scoped_and_truthy_checked() {
        let _env_lock = ENV_LOCK.lock().unwrap();
        std::env::remove_var(V0_1_INDEX_ONLY_ENV);
        assert!(!v0_1_index_only_enabled("init"));

        std::env::set_var(V0_1_INDEX_ONLY_ENV, "1");
        assert!(v0_1_index_only_enabled("init"));
        assert!(v0_1_index_only_enabled("update"));
        assert!(!v0_1_index_only_enabled("query"));

        std::env::set_var(V0_1_INDEX_ONLY_ENV, "false");
        assert!(!v0_1_index_only_enabled("init"));

        std::env::remove_var(V0_1_INDEX_ONLY_ENV);
    }

    #[test]
    fn facts_ready_missing_always_projects_to_index_only() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let _ = sqlite_store::open_db(repo_root).unwrap();
        persist_index_only_release_scope(repo_root).unwrap();

        assert_eq!(
            project_external_runtime_state(repo_root, "missing", true),
            INDEX_ONLY_RUNTIME_STATE
        );
        assert_eq!(
            project_external_runtime_state(repo_root, "fresh", true),
            "fresh"
        );
        assert_eq!(
            project_external_runtime_state(repo_root, "missing", false),
            "missing"
        );
    }
}
