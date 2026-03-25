//! `storage` 层负责 `.wiki/` runtime 的落盘和读取。
//! 这里明确区分页面文件、metadata、cache 和状态内核四类产物。

pub mod cache_store;
pub mod metadata_store;
pub mod sqlite;
pub mod sqlite_store;
pub mod state_store;
pub mod wiki_fs;




