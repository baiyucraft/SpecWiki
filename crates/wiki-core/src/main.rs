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

        if wiki_core::transport::json_rpc::should_stream(&input) {
            let mut stdout = io::stdout().lock();
            if let Err(error) =
                wiki_core::transport::json_rpc::handle_json_stream(&input, &mut stdout)
            {
                let response = wiki_core::transport::dto::CoreResponse::error(error.to_string());
                let event = wiki_core::transport::dto::CoreEvent::terminal(response);
                serde_json::to_writer(&mut stdout, &event).expect("terminal event should serialize");
                use std::io::Write;
                stdout
                    .write_all(b"\n")
                    .expect("terminal event newline should be writable");
            }
            return;
        }

        let response = wiki_core::transport::json_rpc::handle_json(&input)
            .unwrap_or_else(|error| wiki_core::transport::dto::CoreResponse::error(error.to_string()));

        println!("{}", serde_json::to_string(&response).expect("response should serialize"));
        return;
    }

    println!("{}", wiki_core::workspace_name());
}
