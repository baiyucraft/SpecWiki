//! `workflows` 层定义对外可调用的业务动作。
//! 目前包括 init / status / update / query / sync / rebuild 六条主链路。

pub mod archive;
pub mod governance;
pub mod init;
pub mod page_render;
pub mod progress;
pub mod query;
pub mod rebuild;
pub mod release_scope;
pub mod research_provider;
pub mod status;
pub mod sync;
pub mod update;
