use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::llm::{LlmBridgeConfig, LlmCompletion, LlmPromptRequest};
use crate::workflows::progress::WorkflowProgressEvent;
use wiki_model::domain::governance::ArchiveMode;

/// CLI 宿主 bootstrap 的总结果。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapOutcome {
    Ready,
    Partial,
    Failed,
}

/// 单个宿主资产写入结果，由 TypeScript CLI 生成并交给 `cli_init` 保留。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapHostReport {
    pub host: String,
    pub status: BootstrapOutcome,
    #[serde(default)]
    pub files: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// CLI 在进入 Rust runtime 前完成的宿主 bootstrap 事实。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapReport {
    pub outcome: BootstrapOutcome,
    #[serde(default)]
    pub hosts: Vec<BootstrapHostReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_hint: Option<String>,
}

/// `CoreCommand` 是 Agent -> core 的最小命令协议。
/// `streamProgress` 保留为协议字段，但长流程现在统一按事件流输出。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreCommand {
    pub action: String,
    #[serde(rename = "repoRoot")]
    pub repo_root: Option<String>,
    pub term: Option<String>,
    #[serde(rename = "changeId")]
    pub change_id: Option<String>,
    #[serde(rename = "archiveMode")]
    pub archive_mode: Option<ArchiveMode>,
    #[serde(rename = "archiveOperationId")]
    pub archive_operation_id: Option<String>,
    pub bootstrap: Option<BootstrapReport>,
    #[serde(rename = "developmentMode", default)]
    pub development_mode: bool,
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
    #[serde(rename = "errorKind", default, skip_serializing_if = "Option::is_none")]
    pub error_kind: Option<CoreErrorKind>,
    pub data: Option<Value>,
}

/// Transport 可稳定映射的错误分类。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoreErrorKind {
    InvalidArgument,
    GovernanceNotEnabled,
    ChangeNotFound,
    ArchiveNotReady,
    ArchivePreconditionChanged,
    ArchiveConflict,
    ArchiveLocked,
    ArchiveRecoveryRequired,
    ArchiveManifestInvalid,
    WorkflowFailed,
    ProtocolError,
    InternalError,
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
            error_kind: Some(CoreErrorKind::WorkflowFailed),
            data: None,
        }
    }

    /// 构造带结构化上下文的失败响应。
    ///
    /// # 参数
    /// - `error`：要写入协议层的错误文本。
    /// - `data`：失败终态附带的结构化上下文。
    pub fn error_with_data(error: impl Into<String>, data: Value) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            error_kind: Some(CoreErrorKind::WorkflowFailed),
            data: Some(data),
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
            error_kind: None,
            data: Some(data),
        }
    }

    /// 构造带稳定分类的失败响应。
    pub fn typed_error(kind: CoreErrorKind, error: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            error_kind: Some(kind),
            data: None,
        }
    }

    /// 构造带稳定分类和结构化上下文的失败响应。
    pub fn typed_error_with_data(
        kind: CoreErrorKind,
        error: impl Into<String>,
        data: Value,
    ) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            error_kind: Some(kind),
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
