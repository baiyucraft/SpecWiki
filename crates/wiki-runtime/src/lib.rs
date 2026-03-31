//! `wiki-runtime` 是 Repo Wiki 的运行时编排层。
//! 它负责仓库扫描、模块拆分、页面生成、运行时状态维护，以及对外 JSON 协议分发。

pub mod debug_trace;
pub mod domain;
pub mod generation;
pub mod llm;
pub mod repo;
pub mod storage;
pub mod transport;
pub mod workflows;

/// 返回当前 Rust crate 的工作区名称。
/// 这个接口目前主要给二进制入口和调试输出使用。
pub fn workspace_name() -> &'static str {
    "wiki-runtime"
}
