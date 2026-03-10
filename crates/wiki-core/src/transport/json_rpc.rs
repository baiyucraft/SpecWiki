use std::io::{self, Write};

use crate::transport::dto::{CoreCommand, CoreEvent, CoreResponse};
use crate::workflows::progress::{ProgressSink, WorkflowProgressEvent};

/// 处理已经解析好的命令对象。
/// 这里故意保持很薄，只做 transport -> CLI dispatch 的桥接。
///
/// # 参数
/// - `command`：已经完成反序列化的核心命令对象。
///
/// # 返回
/// - 返回统一协议格式的核心响应对象。
pub fn handle(command: CoreCommand) -> CoreResponse {
    crate::transport::cli::dispatch(command)
}

/// 处理 JSON 文本输入。
/// 这个入口专门给 Agent 的 stdin/stdout IPC 使用。
///
/// # 参数
/// - `input`：从标准输入读到的 JSON 命令文本。
///
/// # 返回
/// - 成功时返回统一协议格式的核心响应对象。
///
/// # 错误
/// - 当命令 JSON 反序列化失败时返回错误。
pub fn handle_json(input: &str) -> Result<CoreResponse, serde_json::Error> {
    let command: CoreCommand = serde_json::from_str(input)?;
    Ok(handle(command))
}

/// 判断原始 JSON 输入是否属于长流程事件流。
pub fn should_stream(input: &str) -> bool {
    let Ok(command) = serde_json::from_str::<CoreCommand>(input) else {
        return false;
    };

    matches!(command.action.as_str(), "init" | "update" | "rebuild")
}

/// 处理长流程 JSON IPC 请求，并把 stdout 编码成 NDJSON 事件流。
pub fn handle_json_stream<W>(input: &str, writer: &mut W) -> io::Result<()>
where
    W: Write,
{
    let command: CoreCommand =
        serde_json::from_str(input).map_err(|error| io::Error::other(error.to_string()))?;
    handle_stream(command, writer)
}

/// 以 NDJSON 事件流的方式处理已经反序列化的命令。
pub fn handle_stream<W>(command: CoreCommand, writer: &mut W) -> io::Result<()>
where
    W: Write,
{
    let mut sink = NdjsonProgressSink::new(writer);
    let response = crate::transport::cli::dispatch_with_progress(command, &mut sink);
    sink.finish(response)
}

struct NdjsonProgressSink<'a, W>
where
    W: Write,
{
    writer: &'a mut W,
    write_error: Option<io::Error>,
}

impl<'a, W> NdjsonProgressSink<'a, W>
where
    W: Write,
{
    fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            write_error: None,
        }
    }

    fn finish(mut self, response: CoreResponse) -> io::Result<()> {
        if let Some(error) = self.write_error.take() {
            return Err(error);
        }
        self.write_event(CoreEvent::terminal(response))
    }

    fn write_event(&mut self, event: CoreEvent) -> io::Result<()> {
        serde_json::to_writer(&mut self.writer, &event)
            .map_err(|error| io::Error::other(error.to_string()))?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()
    }
}

impl<W> ProgressSink for NdjsonProgressSink<'_, W>
where
    W: Write,
{
    fn report(&mut self, event: WorkflowProgressEvent) {
        if self.write_error.is_some() {
            return;
        }

        if let Err(error) = self.write_event(CoreEvent::progress(event)) {
            self.write_error = Some(error);
        }
    }
}
