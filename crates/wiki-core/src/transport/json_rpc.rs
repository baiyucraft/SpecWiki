use crate::transport::dto::{CoreCommand, CoreResponse};

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
