use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::llm::{LlmBridgeConfig, LlmCompletion, LlmPromptRequest};
use crate::workflows::progress::WorkflowProgressEvent;

/// `CoreCommand` 是 Agent -> core 的最小命令协议。
/// `streamProgress` 保留为协议字段，但长流程现在统一按事件流输出。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreCommand {
    pub action: String,
    #[serde(rename = "repoRoot")]
    pub repo_root: Option<String>,
    pub term: Option<String>,
    #[serde(rename = "streamProgress", default)]
    pub stream_progress: bool,
    #[serde(rename = "llmBridge", default)]
    pub llm_bridge: Option<LlmBridgeConfig>,
}

/// `CoreResponse` 是 core -> Agent 的统一响应协议。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub data: Option<Value>,
}

impl CoreResponse {
    /// 构造失败响应。
    ///
    /// # 参数
    /// - `error`：要写入协议层的错误文本。
    ///
    /// # 返回
    /// - 返回 `ok = false` 的统一响应对象。
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            data: None,
        }
    }

    /// 构造成功响应。
    ///
    /// # 参数
    /// - `data`：要写入协议层的 JSON 数据。
    ///
    /// # 返回
    /// - 返回 `ok = true` 的统一响应对象。
    pub fn success(data: Value) -> Self {
        Self {
            ok: true,
            error: None,
            data: Some(data),
        }
    }
}

/// `CoreEvent` 是长流程 JSON IPC 的事件封装。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CoreEvent {
    Progress(WorkflowProgressEvent),
    LlmRequest {
        request: LlmPromptRequest,
    },
    AgentSessionStart {
        request: LlmPromptRequest,
    },
    AgentMessage {
        #[serde(rename = "requestId")]
        request_id: String,
        message: Value,
    },
    AgentToolCall {
        #[serde(rename = "requestId")]
        request_id: String,
        message: Value,
    },
    AgentToolResult {
        #[serde(rename = "requestId")]
        request_id: String,
        message: Value,
    },
    AgentFinal {
        #[serde(rename = "requestId")]
        request_id: String,
        response: LlmCompletion,
    },
    AgentAbort {
        #[serde(rename = "requestId")]
        request_id: String,
        reason: String,
    },
    Result {
        response: CoreResponse,
    },
    Error {
        response: CoreResponse,
    },
}

impl CoreEvent {
    /// 构造进度事件。
    ///
    /// # 参数
    /// - `event`：workflow 阶段上报的最小进度事实。
    ///
    /// # 返回
    /// - 返回可序列化为 NDJSON 的 `progress` 事件。
    pub fn progress(event: WorkflowProgressEvent) -> Self {
        Self::Progress(event)
    }

    /// 构造 LLM 请求事件。
    pub fn llm_request(request: LlmPromptRequest) -> Self {
        Self::LlmRequest { request }
    }

    /// 构造 agent session 启动事件。
    pub fn agent_session_start(request: LlmPromptRequest) -> Self {
        Self::AgentSessionStart { request }
    }

    /// 构造唯一终态事件。
    ///
    /// # 参数
    /// - `response`：workflow 最终成功或失败响应。
    ///
    /// # 返回
    /// - 成功时返回 `result`，失败时返回 `error`。
    pub fn terminal(response: CoreResponse) -> Self {
        if response.ok {
            Self::Result { response }
        } else {
            Self::Error { response }
        }
    }
}

/// `CoreSessionInput` 是 Agent -> core 的会话内事件。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CoreSessionInput {
    LlmResponse {
        #[serde(rename = "requestId")]
        request_id: String,
        response: LlmCompletion,
    },
    LlmUnavailable {
        #[serde(rename = "requestId")]
        request_id: String,
        reason: String,
    },
    AgentMessage {
        #[serde(rename = "requestId")]
        request_id: String,
        message: Value,
    },
    AgentToolResult {
        #[serde(rename = "requestId")]
        request_id: String,
        message: Value,
    },
    AgentFinal {
        #[serde(rename = "requestId")]
        request_id: String,
        response: LlmCompletion,
    },
    AgentAbort {
        #[serde(rename = "requestId")]
        request_id: String,
        reason: String,
    },
}
