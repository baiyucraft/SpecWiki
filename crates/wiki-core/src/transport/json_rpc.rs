use std::cell::RefCell;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

use crate::llm::{LlmCompletion, LlmPromptRequest, LlmService};
use crate::transport::dto::CoreSessionInput;
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

/// 判断已经解析好的命令是否属于长流程事件流。
pub fn should_stream_command(command: &CoreCommand) -> bool {
    matches!(command.action.as_str(), "init" | "update" | "rebuild")
}

/// 判断原始 JSON 输入是否属于长流程事件流。
pub fn should_stream(input: &str) -> bool {
    let Ok(command) = serde_json::from_str::<CoreCommand>(input) else {
        return false;
    };

    should_stream_command(&command)
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

/// 在长流程下处理可选的双向 LLM 会话。
pub fn handle_stream_session<R, W>(
    command: CoreCommand,
    reader: &mut R,
    writer: &mut W,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let shared_writer = Rc::new(RefCell::new(writer));
    let shared_error = Rc::new(RefCell::new(None::<String>));
    let mut sink = NdjsonProgressSink::shared(shared_writer.clone(), shared_error.clone());
    let response = if command
        .llm_bridge
        .as_ref()
        .is_some_and(|bridge| bridge.supports_session())
    {
        let shared_reader = Rc::new(RefCell::new(reader));
        let mut llm_service =
            SessionLlmService::new(shared_reader, shared_writer.clone(), shared_error.clone());
        crate::transport::cli::dispatch_with_runtime(command, &mut sink, Some(&mut llm_service))
    } else {
        crate::transport::cli::dispatch_with_runtime(command, &mut sink, None)
    };
    sink.finish(response)
}

struct NdjsonProgressSink<'a, W>
where
    W: Write,
{
    writer: Rc<RefCell<&'a mut W>>,
    write_error: Rc<RefCell<Option<String>>>,
}

impl<'a, W> NdjsonProgressSink<'a, W>
where
    W: Write,
{
    fn new(writer: &'a mut W) -> Self {
        Self {
            writer: Rc::new(RefCell::new(writer)),
            write_error: Rc::new(RefCell::new(None)),
        }
    }

    fn shared(writer: Rc<RefCell<&'a mut W>>, write_error: Rc<RefCell<Option<String>>>) -> Self {
        Self {
            writer,
            write_error,
        }
    }

    fn finish(mut self, response: CoreResponse) -> io::Result<()> {
        if let Some(error) = self.write_error.borrow_mut().take() {
            return Err(io::Error::other(error));
        }
        self.write_event(CoreEvent::terminal(response))
    }

    fn write_event(&mut self, event: CoreEvent) -> io::Result<()> {
        let mut writer = self.writer.borrow_mut();
        serde_json::to_writer(&mut **writer, &event)
            .map_err(|error| io::Error::other(error.to_string()))?;
        writer.write_all(b"\n")?;
        writer.flush()
    }
}

impl<W> ProgressSink for NdjsonProgressSink<'_, W>
where
    W: Write,
{
    fn report(&mut self, event: WorkflowProgressEvent) {
        if self.write_error.borrow().is_some() {
            return;
        }

        if let Err(error) = self.write_event(CoreEvent::progress(event)) {
            *self.write_error.borrow_mut() = Some(error.to_string());
        }
    }
}

struct SessionLlmService<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    reader: Rc<RefCell<&'a mut R>>,
    writer: Rc<RefCell<&'a mut W>>,
    write_error: Rc<RefCell<Option<String>>>,
}

impl<'a, R, W> SessionLlmService<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    fn new(
        reader: Rc<RefCell<&'a mut R>>,
        writer: Rc<RefCell<&'a mut W>>,
        write_error: Rc<RefCell<Option<String>>>,
    ) -> Self {
        Self {
            reader,
            writer,
            write_error,
        }
    }

    fn write_request(&mut self, request: LlmPromptRequest) -> io::Result<()> {
        let event = CoreEvent::llm_request(request);
        let mut writer = self.writer.borrow_mut();
        serde_json::to_writer(&mut **writer, &event)
            .map_err(|error| io::Error::other(error.to_string()))?;
        writer.write_all(b"\n")?;
        writer.flush()
    }
}

impl<R, W> LlmService for SessionLlmService<'_, R, W>
where
    R: BufRead,
    W: Write,
{
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        if let Some(error) = self.write_error.borrow().clone() {
            return Err(io::Error::other(error));
        }

        self.write_request(request.clone())?;

        loop {
            let mut line = String::new();
            let read = self.reader.borrow_mut().read_line(&mut line)?;
            if read == 0 {
                return Err(io::Error::other("llm session closed before response"));
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let event: CoreSessionInput = serde_json::from_str(trimmed)
                .map_err(|error| io::Error::other(error.to_string()))?;
            match event {
                CoreSessionInput::LlmResponse {
                    request_id,
                    response,
                } if request_id == request.request_id => return Ok(response),
                CoreSessionInput::LlmUnavailable { request_id, reason }
                    if request_id == request.request_id =>
                {
                    return Err(io::Error::other(format!("llm unavailable: {reason}")))
                }
                CoreSessionInput::LlmResponse { .. } | CoreSessionInput::LlmUnavailable { .. } => {
                    return Err(io::Error::other(
                        "received out-of-order llm session event from agent",
                    ))
                }
            }
        }
    }
}
