//! `analyze` 负责基于 resolved symbol graph 生成 graph-derived 视图。
//! 当前包含 community、process 和 cycle 三类 deterministic 分析器。

mod communities;
mod cycles;
mod processes;

pub use communities::detect_communities;
pub use cycles::detect_cycles;
pub use processes::detect_processes;

