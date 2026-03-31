use std::io::{self, BufRead};

/// CLI 入口目前只有两种模式：
/// 1. `--json`：走 JSON IPC，给 Agent 调用。
/// 2. 默认模式：输出工作区名称，主要用于简单检查二进制是否可运行。
fn main() {
    let mut json_mode = false;
    let mut debug_trace = wiki_runtime::debug_trace::StartupDebugTraceOptions::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json" => json_mode = true,
            "--debug-trace" => {
                debug_trace.enabled = true;
            }
            "--debug-trace-dir" => {
                debug_trace.enabled = true;
                let Some(value) = args.next() else {
                    eprintln!("missing value for --debug-trace-dir");
                    std::process::exit(2);
                };
                debug_trace.trace_dir = Some(value.into());
            }
            "--debug-trace-console" => {
                debug_trace.enabled = true;
                debug_trace.echo_to_stderr = true;
            }
            _ => {}
        }
    }
    wiki_runtime::debug_trace::set_startup_options(debug_trace);

    if json_mode {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin.lock());
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .expect("stdin should be readable");
        let trimmed = input.trim();

        let command = serde_json::from_str::<wiki_runtime::transport::dto::CoreCommand>(trimmed)
            .unwrap_or_else(|error| {
                let response = wiki_runtime::transport::dto::CoreResponse::error(error.to_string());
                println!(
                    "{}",
                    serde_json::to_string(&response).expect("response should serialize")
                );
                std::process::exit(0);
            });

        if wiki_runtime::transport::json_rpc::should_stream_command(&command) {
            let mut stdout = io::stdout().lock();
            if let Err(error) = wiki_runtime::transport::json_rpc::handle_stream_session(
                command,
                &mut reader,
                &mut stdout,
            ) {
                let response = wiki_runtime::transport::dto::CoreResponse::error(error.to_string());
                let event = wiki_runtime::transport::dto::CoreEvent::terminal(response);
                serde_json::to_writer(&mut stdout, &event)
                    .expect("terminal event should serialize");
                use std::io::Write;
                stdout
                    .write_all(b"\n")
                    .expect("terminal event newline should be writable");
            }
            return;
        }

        let response = wiki_runtime::transport::json_rpc::handle(command);

        println!(
            "{}",
            serde_json::to_string(&response).expect("response should serialize")
        );
        return;
    }

    println!("{}", wiki_runtime::workspace_name());
}
