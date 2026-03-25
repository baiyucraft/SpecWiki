//! `wiki-model` 收口跨 crate 共享的稳定对象语言。
//! 当前迭代先把稳定 DTO 迁入独立 crate，后续再继续收紧边界。

pub mod domain;

/// 返回当前 Rust crate 的工作区名称。
pub fn workspace_name() -> &'static str {
    "wiki-model"
}