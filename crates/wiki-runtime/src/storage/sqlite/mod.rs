//! `sqlite` 按 index / knowledge / runtime 三段暴露正式存储入口。
//! 业务层应优先依赖这里的 wrapper，而不是直接触碰底层 `sqlite_store`。

pub mod governance_store;
pub mod index_store;
pub mod knowledge_store;
pub mod runtime_store;
