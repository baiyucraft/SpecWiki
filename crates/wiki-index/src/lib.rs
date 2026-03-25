//! `wiki-index` 承载 facts / index 能力边界。
//! 它持有 deterministic 扫描、模块树、符号图与相关 assist 合同。

pub mod assist;
pub mod detectors;
pub mod fingerprint;
pub mod hierarchy;
pub mod language_processors;
pub mod parsers;
pub mod scanner;
pub mod store;
pub mod symbol_graph;
pub mod symbols;
pub mod topic_seed;

pub use topic_seed::TopicSeed;

/// 返回当前 Rust crate 的工作区名称。
pub fn workspace_name() -> &'static str {
    "wiki-index"
}



