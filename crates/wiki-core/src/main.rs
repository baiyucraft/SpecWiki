use std::io::{self, Read};

/// CLI 入口目前只有两种模式：
/// 1. `--json`：走 JSON IPC，给 Agent 调用。
/// 2. 默认模式：输出工作区名称，主要用于简单检查二进制是否可运行。
fn main() {
    if std::env::args().any(|arg| arg == "--json") {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("stdin should be readable");

        // 传输层只负责解析/分发；真正的业务逻辑都在 workflow 里。
        let response = wiki_core::transport::json_rpc::handle_json(&input)
            .unwrap_or_else(|error| wiki_core::transport::dto::CoreResponse::error(error.to_string()));

        println!(
            "{}",
            serde_json::to_string(&response).expect("response should serialize")
        );
        return;
    }

    println!("{}", wiki_core::workspace_name());
}
