//! `domain` 层放 Repo Wiki 的核心数据模型。
//! 这些结构会在扫描、生成、运行时状态和协议之间反复流转。

pub mod change_set;
pub mod context;
pub mod metadata;
pub mod module_tree;
pub mod relation;
pub mod state;
pub mod stable_id;
pub mod wiki_item;
