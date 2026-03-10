//! `resolve` 负责把 raw imports / calls / heritage 转成稳定 symbol edges。
//! 当前先落 import resolution，上下文和索引结构后续会继续被 calls / heritage 复用。

mod calls;
mod heritage;
mod imports;

pub use imports::{
    build_import_resolution_context, collect_import_target_files, resolve_imports,
    ImportResolutionContext, SuffixIndex,
};
pub use calls::resolve_calls;
pub use heritage::resolve_heritage;
