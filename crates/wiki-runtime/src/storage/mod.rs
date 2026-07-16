//! `storage` 层负责 `.wiki/` runtime 的落盘和读取。
//! 这里明确区分页面文件、metadata、cache 和状态内核四类产物。

pub mod archive_fs;
pub mod cache_store;
pub mod governance_fs;
pub mod knowledge_artifacts;
pub mod metadata_store;
pub mod runtime_commit;
pub mod sqlite;
pub mod sqlite_store;
pub mod state_store;
pub mod wiki_fs;
