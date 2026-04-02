use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static INDEX_ONLY_ENV_LOCK: Mutex<()> = Mutex::new(());

pub struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    pub fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.as_ref() {
            std::env::set_var(self.key, previous);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

pub fn lock_index_only_env() -> MutexGuard<'static, ()> {
    INDEX_ONLY_ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner())
}

pub fn force_full_runtime() -> (MutexGuard<'static, ()>, EnvVarGuard) {
    let env_lock = lock_index_only_env();
    let guard = EnvVarGuard::set("SPEC_WIKI_V0_1_INDEX_ONLY", "false");
    (env_lock, guard)
}
