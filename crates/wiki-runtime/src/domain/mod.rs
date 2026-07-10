//! `domain` 层放 Repo Wiki 的核心数据模型。
//! 这些结构会在扫描、生成、运行时状态和协议之间反复流转。

pub mod change_set;
pub mod checkpoint;
pub mod context;
pub mod governance;
pub mod knowledge;
pub mod metadata;
pub mod metadata_mapper;
pub mod module_tree;
pub mod relation;
pub mod runtime_profile;
pub mod stable_id;
pub mod state;
pub mod steering;
pub mod wiki_item;
