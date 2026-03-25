//! `transport` 层负责 core 对外暴露的最小协议。
//! 当前实现只有 JSON IPC，没有额外服务端或守护进程层。

pub mod cli;
pub mod dto;
pub mod json_rpc;



