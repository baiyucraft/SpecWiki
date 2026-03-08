//! `storage` 层负责 `.wiki/` runtime 的落盘和读取。
//! 这里明确区分页面文件、metadata 和 cache 三类产物。

pub mod cache_store;
pub mod metadata_store;
pub mod wiki_fs;
