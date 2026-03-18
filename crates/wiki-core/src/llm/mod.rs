//! `llm` 模块负责承载可选的 LLM 辅助层。
//! 它只消费已经稳定的事实输入，负责 prompt 组装、缓存、预算控制和输出校验，
//! 不直接接触文件系统扫描、模块树持久化或 Agent 宿主实现。
//! It does not directly touch repository scanning, module persistence, or agent hosts.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::debug_trace;
use crate::domain::context::{
    ModuleContext, PageContext, PageDiagramInput, PageEvidenceGroup, PageResearchResult,
    PageResearchSessionState, PageResearchTurn, PageToolArtifactRef, RepoContext, TargetedSnippet,
};
use crate::domain::module_tree::ModuleTree;
use crate::domain::research::{ResearchSessionStats, ResearchStopReason};
use crate::domain::stable_id::stable_id;
use crate::domain::steering::{
    persist_learned_tools_mode, resolve_learned_tools_mode, LlmCacheMode, LlmConfig,
    LlmProviderConfig, LlmProviderRequestFormat, LlmToolsMode,
};
use crate::generation::planner::PlannedPage;
use crate::generation::sections::{section_key_for_title, section_titles_for_page_type};
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::scanner::FilePurpose;
use crate::repo::symbol_graph::{GraphAnalysisSnapshot, ResolvedGraphSnapshot};
use crate::repo::symbols::{ParsedSymbolsSnapshot, SymbolNode};
use crate::storage::sqlite_store::{read_llm_cache, write_llm_cache, LlmCacheEntry};

const FILE_PURPOSE_PROMPT_VERSION: &str = "file-purpose/v1";
const TOP_LEVEL_PROMOTION_PROMPT_VERSION: &str = "top-level-promotion/v1";
const MODULE_KIND_PROMPT_VERSION: &str = "module-kind/v1";
const DEPENDENCY_PROMPT_VERSION: &str = "dependency-edge/v1";
const PAGE_RESEARCH_PROMPT_VERSION: &str = "page-research/v1";
const LLM_REQUEST_PROTOCOL: &str = "agent_session_v1";
const LEGACY_LLM_REQUEST_PROTOCOL: &str = "ndjson_session_v1";
const FILE_PURPOSE_BATCH_SIZE: usize = 8;
const TOP_LEVEL_PROMOTION_BATCH_SIZE: usize = 8;
const DEPENDENCY_EDGE_BATCH_SIZE: usize = 8;
const PROVIDER_TOOLS_TTL_HOURS: u64 = 24 * 7;
const NEGATIVE_LLM_CACHE_STATUS: &str = "negative";
const FILE_PURPOSE_ALLOWED_VALUES: [&str; 24] = [
    "entry",
    "router",
    "controller",
    "handler",
    "service",
    "model",
    "repository",
    "domain",
    "agent",
    "library",
    "middleware",
    "plugin",
    "utility",
    "helper",
    "constant",
    "type",
    "page",
    "component",
    "widget",
    "layout",
    "config",
    "migration",
    "test",
    "docs",
];

/// `SelectedLlmPath` 表示当前 workflow 实际采用的 LLM 请求路径。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedLlmPath {
    /// 由 core 直接调用 provider API。
    ProviderApi,
    /// 由 Agent 通过 JSON IPC 协议代调用。
    AgentBridge,
}

/// `LlmBridgeConfig` 表示 transport 层是否协商开启双向 LLM 会话。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct LlmBridgeConfig {
    /// 当前协商使用的协议名。
    #[serde(default = "default_llm_bridge_protocol")]
    pub protocol: String,
}

impl LlmBridgeConfig {
    /// 当前协商是否启用了 core/Agent 双向会话。
    pub fn supports_session(&self) -> bool {
        matches!(
            self.protocol.as_str(),
            LLM_REQUEST_PROTOCOL | LEGACY_LLM_REQUEST_PROTOCOL
        )
    }
}
fn default_llm_bridge_protocol() -> String {
    LLM_REQUEST_PROTOCOL.to_string()
}
/// `PromptType` 表示当前 LLM 请求属于哪一类辅助任务。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptType {
    FilePurpose,
    TopLevelPromotion,
    ModuleKind,
    DependencyEdge,
    PageResearch,
}

impl PromptType {
    /// 返回稳定 prompt 类型名，供缓存键和协议层复用。
    pub fn as_str(&self) -> &'static str {
        match self {
            PromptType::FilePurpose => "file_purpose",
            PromptType::TopLevelPromotion => "top_level_promotion",
            PromptType::ModuleKind => "module_kind",
            PromptType::DependencyEdge => "dependency_edge",
            PromptType::PageResearch => "page_research",
        }
    }

    /// 返回当前 prompt 的版本号。
    pub fn version(&self) -> &'static str {
        match self {
            PromptType::FilePurpose => FILE_PURPOSE_PROMPT_VERSION,
            PromptType::TopLevelPromotion => TOP_LEVEL_PROMPT_VERSION_FIXTURE,
            PromptType::ModuleKind => MODULE_KIND_PROMPT_VERSION,
            PromptType::DependencyEdge => DEPENDENCY_PROMPT_VERSION,
            PromptType::PageResearch => PAGE_RESEARCH_PROMPT_VERSION,
        }
    }
}

const TOP_LEVEL_PROMPT_VERSION_FIXTURE: &str = TOP_LEVEL_PROMOTION_PROMPT_VERSION;

/// `LlmPromptRequest` �?core -> Agent 的结构化请求。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmPromptRequest {
    /// 单次请求的稳�?ID。
    pub request_id: String,
    /// prompt 类别。
    pub prompt_type: String,
    /// prompt 版本。
    pub prompt_version: String,
    /// 归一化输入哈希，Agent 可据此做自己的去重。
    pub input_hash: String,
    /// 期望使用的模型标识；未指定时表示由宿主自行选择。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// 对模型的系统级约束。
    pub system: String,
    /// 具体任务说明。
    pub instruction: String,
    /// 当前任务的结构化输入。
    pub input: Value,
    /// 期望返回的结构化 schema 提示。
    pub response_schema: Value,
    /// 可选的 provider tools 定义。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Value>,
    /// 可选的 tool 选择器。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    /// 顶层 response_format；provider 路径默认发送。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    /// 可选的 session 摘要状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<PageResearchSessionState>,
}

/// `LlmCompletion` �?Agent -> core 的结构化响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmCompletion {
    /// 模型输出�?JSON 结果。
    pub output: Value,
    /// 实际使用的模型标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// 当前请求�?usage 统计。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<LlmUsage>,
}

/// 单次 LLM 请求�?token 统计。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct LlmUsage {
    pub request_count: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
    pub source: String,
}

/// provider/model �?prompt_type 维度�?usage bucket。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct LlmUsageBucket {
    pub key: String,
    pub request_count: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
}

/// workflow 可观测的 usage 快照。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct LlmUsageSnapshot {
    pub request_count: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
    #[serde(default)]
    pub by_prompt_type: Vec<LlmUsageBucket>,
    #[serde(default)]
    pub by_provider_model: Vec<LlmUsageBucket>,
}

/// `LlmService` 抽象 transport 之外的真�?LLM 调用通道。
pub trait LlmService {
    /// 发起一次结构化 LLM 请求。
    ///
    /// # 参数
    /// - `request`：当�?prompt 的完整协议对象。
    ///
    /// # 返回
    /// - 成功时返回结构化 JSON 输出。
    ///
    /// # 错误
    /// - 当桥接不可用、宿主拒绝或协议失败时返回错误。
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion>;
}

enum RuntimeLlmService<'svc> {
    Provider(ProviderApiLlmService),
    Agent(&'svc mut dyn LlmService),
}

impl RuntimeLlmService<'_> {
    fn path(&self) -> SelectedLlmPath {
        match self {
            Self::Provider(_) => SelectedLlmPath::ProviderApi,
            Self::Agent(_) => SelectedLlmPath::AgentBridge,
        }
    }

    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        match self {
            Self::Provider(service) => service.request(request),
            Self::Agent(service) => service.request(request),
        }
    }
}

struct ProviderApiLlmService {
    client: Client,
    provider: LlmProviderConfig,
    selected_model_id: String,
}

#[derive(Debug, Clone, Default)]
struct ProviderChatResponse {
    model: Option<String>,
    content: Option<String>,
    tool_calls: Vec<ProviderToolCall>,
    usage: Option<LlmUsage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct ProviderToolCall {
    id: String,
    #[serde(default = "default_function_type")]
    r#type: String,
    function: ProviderToolFunction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct ProviderToolFunction {
    name: String,
    arguments: String,
}

impl ProviderApiLlmService {
    /// 基于 steering 配置构�?provider 直连 service。
    fn from_config(config: &LlmConfig) -> io::Result<Self> {
        let selected = config.resolve_selected_model().ok_or_else(|| {
            io::Error::other("provider direct call requires llm.model=provider/model")
        })?;
        let timeout = Duration::from_secs(selected.provider.timeout_seconds.max(1));
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|error| io::Error::other(error.to_string()))?;

        Ok(Self {
            client,
            provider: selected.provider.clone(),
            selected_model_id: selected
                .model
                .resolved_model_id(selected.model_name)
                .to_string(),
        })
    }

    fn request_model(&self) -> io::Result<&str> {
        (!self.selected_model_id.trim().is_empty())
            .then_some(self.selected_model_id.as_str())
            .ok_or_else(|| {
                io::Error::other("provider direct call requires a resolved provider model")
            })
    }

    fn default_messages(request: &LlmPromptRequest) -> Vec<Value> {
        vec![
            json!({
                "role": "system",
                "content": format!(
                    "{}\n你必须只返回一个 JSON 对象，不要使用 Markdown 代码块。",
                    request.system
                ),
            }),
            json!({
                "role": "user",
                "content": build_provider_user_message(request),
            }),
        ]
    }

    fn request_body_with_messages(
        &self,
        request: &LlmPromptRequest,
        messages: &[Value],
        include_response_format: bool,
    ) -> io::Result<Value> {
        let model = self.request_model()?;
        let response_format = include_response_format.then(|| {
            request
                .response_format
                .as_ref()
                .map(normalize_provider_response_format)
                .unwrap_or_else(|| build_provider_response_format(&request.response_schema))
        });
        let mut body = match self.provider.request_format {
            LlmProviderRequestFormat::ChatCompletions => json!({
                "model": model,
                "temperature": 0,
                "messages": messages,
            }),
            LlmProviderRequestFormat::Responses => json!({
                "model": model,
                "temperature": 0,
                "input": build_provider_response_input(messages),
            }),
        };
        if !request.tools.is_empty() {
            body["tools"] = Value::Array(
                request
                    .tools
                    .iter()
                    .map(|tool| match self.provider.request_format {
                        LlmProviderRequestFormat::ChatCompletions => {
                            normalize_provider_tool_definition(tool)
                        }
                        LlmProviderRequestFormat::Responses => {
                            normalize_provider_response_tool_definition(tool)
                        }
                    })
                    .collect(),
            );
        }
        if let Some(tool_choice) = &request.tool_choice {
            body["tool_choice"] = match self.provider.request_format {
                LlmProviderRequestFormat::ChatCompletions => tool_choice.clone(),
                LlmProviderRequestFormat::Responses => {
                    normalize_provider_response_tool_choice(tool_choice)
                }
            };
        }
        if let Some(response_format) = response_format {
            match self.provider.request_format {
                LlmProviderRequestFormat::ChatCompletions => {
                    body["response_format"] = response_format;
                }
                LlmProviderRequestFormat::Responses => {
                    body["text"] = json!({
                        "format": normalize_provider_responses_text_format(&response_format)
                    });
                }
            }
        }
        Ok(body)
    }

    fn send_request(
        &mut self,
        request: &LlmPromptRequest,
        request_body: &Value,
    ) -> io::Result<Value> {
        let endpoint = self.provider.endpoint_url().ok_or_else(|| {
            io::Error::other("provider direct call requires llm.providers.<provider>.api_base")
        })?;
        debug_trace::record_json(
            "llm_provider_request",
            &json!({
                "request": request,
                "http": {
                    "url": endpoint,
                    "body": request_body.clone(),
                },
            }),
        );
        let max_attempts = self.provider.max_retries.max(1);
        for attempt in 1..=max_attempts {
            let mut http_request = self
                .client
                .post(&endpoint)
                .header(CONTENT_TYPE, "application/json");
            if let Some(api_key) = self.provider.resolved_api_key() {
                http_request = http_request.header(AUTHORIZATION, format!("Bearer {api_key}"));
            }
            let response = match http_request.json(request_body).send() {
                Ok(response) => response,
                Err(error) => {
                    if attempt < max_attempts && is_retryable_provider_transport_error(&error) {
                        record_provider_retry(
                            request,
                            "http",
                            attempt,
                            max_attempts,
                            &error.to_string(),
                            None,
                        );
                        sleep_before_retry(self.provider.retry_backoff_ms, attempt);
                        continue;
                    }
                    debug_trace::record_json(
                        "llm_provider_error",
                        &json!({
                            "request_id": request.request_id,
                            "stage": "http",
                            "attempt": attempt,
                            "error": error.to_string(),
                        }),
                    );
                    return Err(io::Error::other(error.to_string()));
                }
            };
            let status = response.status();
            let response_json = match response.json::<Value>() {
                Ok(response_json) => response_json,
                Err(error) => {
                    if attempt < max_attempts && is_retryable_provider_transport_error(&error) {
                        record_provider_retry(
                            request,
                            "decode_response",
                            attempt,
                            max_attempts,
                            &error.to_string(),
                            None,
                        );
                        sleep_before_retry(self.provider.retry_backoff_ms, attempt);
                        continue;
                    }
                    debug_trace::record_json(
                        "llm_provider_error",
                        &json!({
                            "request_id": request.request_id,
                            "stage": "decode_response",
                            "attempt": attempt,
                            "error": error.to_string(),
                        }),
                    );
                    return Err(io::Error::other(error.to_string()));
                }
            };
            if !status.is_success() {
                if attempt < max_attempts && is_retryable_provider_status(status.as_u16()) {
                    record_provider_retry(
                        request,
                        "http_status",
                        attempt,
                        max_attempts,
                        &format!("provider returned {status}"),
                        Some(&response_json),
                    );
                    sleep_before_retry(self.provider.retry_backoff_ms, attempt);
                    continue;
                }
                let error =
                    io::Error::other(format!("provider returned {status}: {response_json}"));
                debug_trace::record_json(
                    "llm_provider_error",
                    &json!({
                        "request_id": request.request_id,
                        "stage": "http_status",
                        "attempt": attempt,
                        "status": status.as_u16(),
                        "response": response_json.clone(),
                    }),
                );
                return Err(error);
            }
            debug_trace::record_json(
                "llm_provider_response",
                &json!({
                    "request_id": request.request_id,
                    "attempt": attempt,
                    "response": response_json.clone(),
                }),
            );
            return Ok(response_json);
        }

        Err(io::Error::other("provider request exhausted retries"))
    }

    fn request_chat(&mut self, request: &LlmPromptRequest) -> io::Result<ProviderChatResponse> {
        let messages = Self::default_messages(request);
        self.request_chat_with_messages(request, &messages)
    }

    fn request_chat_with_messages(
        &mut self,
        request: &LlmPromptRequest,
        messages: &[Value],
    ) -> io::Result<ProviderChatResponse> {
        let request_body = self.request_body_with_messages(
            request,
            messages,
            self.provider.capabilities.response_format,
        )?;
        let response_json = match self.send_request(request, &request_body) {
            Ok(response) => response,
            Err(error)
                if provider_request_uses_response_format(&request_body)
                    && is_response_format_transport_error(&error) =>
            {
                let retry_body = self.request_body_with_messages(request, messages, false)?;
                self.send_request(request, &retry_body)?
            }
            Err(error) => return Err(error),
        };
        match self.provider.request_format {
            LlmProviderRequestFormat::ChatCompletions => parse_provider_chat_response(
                request,
                self.request_model()?,
                &response_json,
                &request_body,
            ),
            LlmProviderRequestFormat::Responses => parse_provider_responses_response(
                request,
                self.request_model()?,
                &response_json,
                &request_body,
            ),
        }
    }
}

impl LlmService for ProviderApiLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        let completion = self.request_chat(request)?;
        let content = completion
            .content
            .as_deref()
            .ok_or_else(|| io::Error::other("provider response did not contain final content"))?;
        let output = match parse_provider_json_output(content) {
            Ok(output) => output,
            Err(error) => {
                debug_trace::record_json(
                    "llm_provider_error",
                    &json!({
                        "request_id": request.request_id,
                        "stage": "parse_output",
                        "error": error.to_string(),
                        "content": content,
                    }),
                );
                json!({
                    "_provider_invalid_json_output": true,
                    "raw_content": content,
                })
            }
        };
        debug_trace::record_json(
            "llm_provider_completion",
            &json!({
                "request_id": request.request_id,
                "completion": {
                    "model": completion.model.clone(),
                    "output": output.clone(),
                    "usage": completion.usage.clone(),
                },
            }),
        );

        Ok(LlmCompletion {
            output,
            model: completion.model,
            usage: completion.usage,
        })
    }
}

/// 文件角色辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePurposeAssistInput {
    /// 当前文件的仓库内相对路径。
    pub path: String,
    /// scanner 已经判定出的粗粒�?kind。
    pub kind: String,
    /// 当前文件语言。
    pub language: String,
    /// 文件大小，主要供宿主做成本控制。
    pub file_size: usize,
    /// deterministic 分类结果。
    pub deterministic: String,
    /// 截断后的源码预览。
    pub preview: String,
}

/// 顶层目录晋升辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopLevelPromotionAssistInput {
    /// 候选顶层目录路径。
    pub root_path: String,
    /// 当前 deterministic 评分。
    pub score: usize,
    /// 目录内总文件数。
    pub total_files: usize,
    /// 目录内有效源码文件数。
    pub source_files: usize,
    /// 配置文件数。
    pub config_files: usize,
    /// 入口文件数。
    pub entry_points: usize,
    /// 是否存在子目录。
    pub has_subdirs: bool,
    /// 目录内主要语言集合。
    pub languages: Vec<String>,
    /// 顶层标签集合。
    pub tags: Vec<String>,
}

/// 模块类型辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleKindAssistInput {
    /// 模块根路径。
    pub root_path: String,
    /// 当前模块标签集合。
    pub tags: Vec<String>,
    /// 是否来自显式模块边界。
    pub explicit_root: bool,
    /// 是否还有子模块。
    pub has_children: bool,
    /// 是否具备主入口文件。
    pub has_main_entry: bool,
    /// 是否存在应用源码。
    pub has_app_source: bool,
    /// 是否存在基础设施文件。
    pub has_infra_files: bool,
    /// 是否属于 workspace 成员。
    pub workspace_member: bool,
}

/// 低置信度跨模块关系辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependencyAssistInput {
    /// 当前关系类型。
    pub relation_type: String,
    /// 关系起点路径。
    pub source_path: String,
    /// 关系终点路径。
    pub target_path: String,
    /// 起点模块展示名。
    pub source_module: String,
    /// 终点模块展示名。
    pub target_module: String,
    /// 现有置信度标签。
    pub confidence: String,
}

/// 页面 research session 输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageResearchInput {
    /// 页面稳定 ID。
    pub page_id: String,
    /// 页面类型。
    pub page_type: String,
    /// 页面标题。
    pub title: String,
    /// 页面作用域。
    pub scope: String,
    /// 当前页面的稳�?facts。
    pub facts: Vec<String>,
    /// 当前页面的补充摘要输入。
    pub summary_inputs: Vec<String>,
    /// 当前页面 steering hints。
    pub hints: Vec<String>,
    /// 当前页面允许的受控章节槽位；为空时再退�?page_type 默认模板。
    #[serde(default)]
    pub allowed_section_slots: Vec<PageResearchSectionSlot>,
    /// 稳定 evidence groups。
    #[serde(default)]
    pub evidence_groups: Vec<PageEvidenceGroup>,
    /// 稳定图输入。
    #[serde(default)]
    pub diagram_inputs: Vec<PageDiagramInput>,
    /// session 当前压缩状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<PageResearchSessionState>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageResearchSectionSlot {
    pub section_key: String,
    pub section_title: String,
}

impl PageResearchInput {
    pub fn from_page(page: &PlannedPage, context: &PageContext) -> Self {
        Self {
            page_id: page.id.clone(),
            page_type: page.page_type.clone(),
            title: page.title.clone(),
            scope: page.scope.clone(),
            facts: context.facts.clone(),
            summary_inputs: context.summary_inputs.clone(),
            hints: context.hints.clone(),
            allowed_section_slots: section_titles_for_page_type(&page.page_type)
                .into_iter()
                .map(|title| PageResearchSectionSlot {
                    section_key: section_key_for_title(&page.page_type, title),
                    section_title: title.to_string(),
                })
                .collect(),
            evidence_groups: context.evidence_groups.clone(),
            diagram_inputs: context.diagram_inputs.clone(),
            session: None,
        }
    }

    pub fn with_allowed_sections(
        mut self,
        allowed_section_slots: Vec<PageResearchSectionSlot>,
    ) -> Self {
        if !allowed_section_slots.is_empty() {
            self.allowed_section_slots = allowed_section_slots;
        }
        self
    }
}

/// 当前页面 research session 的工具执行环境。
pub struct PageResearchRuntimeContext<'a> {
    pub page: &'a PlannedPage,
    pub page_context: &'a PageContext,
    pub scan_report: &'a crate::repo::scanner::ScanReport,
    pub module_tree: &'a ModuleTree,
    pub repo_context: &'a RepoContext,
    pub module_contexts: &'a [ModuleContext],
    pub symbol_snapshot: &'a ParsedSymbolsSnapshot,
    pub resolved_graph: &'a ResolvedGraphSnapshot,
    pub graph_analysis: &'a GraphAnalysisSnapshot,
    pub allowed_section_slots: &'a [PageResearchSectionSlot],
}

/// 单页 research session 的结构化输出。
#[derive(Debug, Clone)]
pub struct PageResearchSessionOutput {
    pub result: PageResearchResult,
    pub session: PageResearchSessionState,
}

/// 单页 research session 的停止结果。
#[derive(Debug, Clone)]
pub struct PageResearchSessionResult {
    pub output: Option<PageResearchSessionOutput>,
    pub stop_reason: ResearchStopReason,
    pub stats: ResearchSessionStats,
}

impl PageResearchSessionResult {
    pub fn is_some(&self) -> bool {
        self.output.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.output.is_none()
    }

    pub fn expect(self, message: &str) -> PageResearchSessionOutput {
        self.output.expect(message)
    }

    fn not_run() -> Self {
        Self {
            output: None,
            stop_reason: ResearchStopReason::NotRun,
            stats: ResearchSessionStats::default(),
        }
    }

    fn call_budget_rejected() -> Self {
        Self {
            output: None,
            stop_reason: ResearchStopReason::CallBudgetRejected,
            stats: ResearchSessionStats::default(),
        }
    }

    fn invalid_output(stats: ResearchSessionStats) -> Self {
        Self {
            output: None,
            stop_reason: ResearchStopReason::InvalidOutput,
            stats,
        }
    }

    fn no_meaningful_delta(stats: ResearchSessionStats) -> Self {
        Self {
            output: None,
            stop_reason: ResearchStopReason::NoMeaningfulDelta,
            stats,
        }
    }

    fn turn_budget_exhausted(stats: ResearchSessionStats) -> Self {
        Self {
            output: None,
            stop_reason: ResearchStopReason::TurnBudgetExhausted,
            stats,
        }
    }

    fn completed(
        output: PageResearchSessionOutput,
        stop_reason: ResearchStopReason,
        stats: ResearchSessionStats,
    ) -> Self {
        Self {
            output: Some(output),
            stop_reason,
            stats,
        }
    }
}

impl PageResearchResult {
    /// �?research 结果做轻量结构校验，避免污染 deterministic renderer。
    pub fn sanitize_for_context(
        mut self,
        context: &PageContext,
        allowed_section_slots: &[PageResearchSectionSlot],
    ) -> Option<Self> {
        self.summary = normalize_sentence(&self.summary);
        self.page_positioning = normalize_sentence(&self.page_positioning);
        self.open_questions = dedupe_non_empty(self.open_questions)
            .into_iter()
            .take(3)
            .collect();

        let allowed_group_keys = context
            .evidence_groups
            .iter()
            .map(|group| group.group_id.clone())
            .collect::<BTreeSet<_>>();
        let allowed_diagrams = context
            .diagram_inputs
            .iter()
            .map(|diagram| diagram.diagram_id.clone())
            .collect::<BTreeSet<_>>();
        let allowed_children = BTreeSet::<String>::new();
        let allowed_slots = if allowed_section_slots.is_empty() {
            section_titles_for_page_type(&context.page_type)
                .into_iter()
                .map(|title| PageResearchSectionSlot {
                    section_key: section_key_for_title(&context.page_type, title),
                    section_title: title.to_string(),
                })
                .collect::<Vec<_>>()
        } else {
            allowed_section_slots.to_vec()
        };
        let allowed_slot_keys = allowed_slots
            .iter()
            .map(|slot| (slot.section_key.clone(), slot.section_title.clone()))
            .collect::<BTreeMap<_, _>>();
        let allowed_slot_titles = allowed_slots
            .iter()
            .map(|slot| (slot.section_title.clone(), slot.section_key.clone()))
            .collect::<BTreeMap<_, _>>();
        let max_section_count = allowed_slots.len();

        let mut seen_section_keys = BTreeSet::new();
        self.section_plan = self
            .section_plan
            .into_iter()
            .filter_map(|mut section| {
                let normalized_key = if section.section_key.trim().is_empty() {
                    allowed_slot_titles
                        .get(section.section_title.trim())
                        .cloned()
                        .unwrap_or_else(|| {
                            section_key_for_title(&context.page_type, &section.section_title)
                        })
                } else {
                    section.section_key.trim().to_string()
                };
                let section_title = allowed_slot_keys.get(&normalized_key)?.to_string();
                if !seen_section_keys.insert(normalized_key.clone()) {
                    return None;
                }
                section.section_key = normalized_key;
                section.section_title = section_title;
                section.section_summary = normalize_multiline(&section.section_summary);
                section.evidence_refs = dedupe_non_empty(section.evidence_refs)
                    .into_iter()
                    .filter(|reference| allowed_group_keys.contains(reference))
                    .take(6)
                    .collect();
                section.diagram_refs = dedupe_non_empty(section.diagram_refs)
                    .into_iter()
                    .filter(|reference| allowed_diagrams.contains(reference))
                    .take(4)
                    .collect();
                section.child_refs = dedupe_non_empty(section.child_refs)
                    .into_iter()
                    .filter(|reference| allowed_children.contains(reference))
                    .take(6)
                    .collect();

                (!section.section_summary.is_empty()
                    || !section.evidence_refs.is_empty()
                    || !section.diagram_refs.is_empty()
                    || !section.child_refs.is_empty())
                .then_some(section)
            })
            .take(max_section_count)
            .collect();

        self.evidence_rollup = self
            .evidence_rollup
            .into_iter()
            .filter_map(|mut group| {
                group.group_key = group.group_key.trim().to_string();
                group.title = group.title.trim().to_string();
                if !allowed_group_keys.contains(&group.group_key) || group.title.is_empty() {
                    return None;
                }
                group.items = group
                    .items
                    .into_iter()
                    .filter_map(|mut item| {
                        item.path = item.path.trim().to_string();
                        item.note = normalize_sentence(&item.note);
                        item.evidence_type = item.evidence_type.trim().to_string();
                        item.section_refs = dedupe_non_empty(item.section_refs)
                            .into_iter()
                            .filter(|reference| seen_section_keys.contains(reference))
                            .collect();
                        (!item.path.is_empty()).then_some(item)
                    })
                    .take(6)
                    .collect();
                (!group.items.is_empty()).then_some(group)
            })
            .take(4)
            .collect();
        self.diagram_rollup = self
            .diagram_rollup
            .into_iter()
            .filter_map(|mut diagram| {
                diagram.diagram_key = diagram.diagram_key.trim().to_string();
                diagram.title = diagram.title.trim().to_string();
                diagram.summary = normalize_sentence(&diagram.summary);
                (allowed_diagrams.contains(&diagram.diagram_key) && !diagram.title.is_empty())
                    .then_some(diagram)
            })
            .take(3)
            .collect();

        (!self.summary.is_empty()
            || !self.page_positioning.is_empty()
            || !self.section_plan.is_empty()
            || !self.evidence_rollup.is_empty()
            || !self.diagram_rollup.is_empty()
            || !self.open_questions.is_empty())
        .then_some(self)
    }
}

#[derive(Debug, Clone)]
struct PendingPromptBatch<T> {
    items: Vec<(usize, String, T)>,
    request: LlmPromptRequest,
}

#[derive(Debug, Clone, Serialize)]
struct FilePurposeBatchInput {
    items: Vec<FilePurposeAssistInput>,
}

#[derive(Debug, Clone, Serialize)]
struct TopLevelPromotionBatchInput {
    items: Vec<TopLevelPromotionAssistInput>,
}

#[derive(Debug, Clone, Serialize)]
struct DependencyAssistBatchInput {
    items: Vec<DependencyAssistInput>,
}

#[derive(Debug, Clone)]
struct PreparedPromptPayload {
    input: Value,
    session: Option<PageResearchSessionState>,
}

/// `LlmRuntime` 统一收口 steering、预算、缓存和真实调用。
pub struct LlmRuntime<'cfg, 'svc> {
    repo_root: &'cfg Path,
    config: &'cfg LlmConfig,
    service: Option<RuntimeLlmService<'svc>>,
    real_calls: usize,
    uncertainty_calls: usize,
    page_calls: usize,
    core_page_calls: usize,
    usage: LlmUsageSnapshot,
    usage_by_prompt_type: BTreeMap<String, LlmUsageBucket>,
    usage_by_provider_model: BTreeMap<String, LlmUsageBucket>,
    usage_reporter: Option<Box<dyn FnMut(LlmUsageSnapshot) + 'svc>>,
}

impl<'cfg, 'svc> LlmRuntime<'cfg, 'svc> {
    /// 创建当前 workflow 使用�?LLM runtime。
    pub fn new(
        repo_root: &'cfg Path,
        config: &'cfg LlmConfig,
        agent_service: Option<&'svc mut dyn LlmService>,
    ) -> Self {
        let service = select_runtime_service(config, agent_service);
        let selected_path = service.as_ref().map(|service| match service.path() {
            SelectedLlmPath::ProviderApi => "provider_api",
            SelectedLlmPath::AgentBridge => "agent_bridge",
        });
        debug_trace::record_json(
            "llm_runtime_selected",
            &json!({
                "enabled": config.enabled,
                "model": config.model,
                "selected_path": selected_path,
                "provider_parallel_requests": if selected_path == Some("provider_api") {
                    config.provider_parallel_requests()
                } else {
                    1
                },
            }),
        );
        Self {
            repo_root,
            config,
            service,
            real_calls: 0,
            uncertainty_calls: 0,
            page_calls: 0,
            core_page_calls: 0,
            usage: LlmUsageSnapshot::default(),
            usage_by_prompt_type: BTreeMap::new(),
            usage_by_provider_model: BTreeMap::new(),
            usage_reporter: None,
        }
    }

    /// 注册普通模式下的实�?usage 回调。
    pub fn set_usage_reporter(
        &mut self,
        reporter: Option<Box<dyn FnMut(LlmUsageSnapshot) + 'svc>>,
    ) {
        self.usage_reporter = reporter;
    }

    /// 当前 workflow 是否真正具备可用�?LLM 请求路径。
    pub fn service_available(&self) -> bool {
        self.config.enabled && self.service.is_some()
    }

    /// bounded research session 是否可用。
    pub fn session_enabled(&self) -> bool {
        self.service_available() && self.config.enabled
    }

    /// 返回当前 workflow 实际选中的请求路径。
    pub fn selected_path(&self) -> Option<SelectedLlmPath> {
        self.service.as_ref().map(RuntimeLlmService::path)
    }

    /// 当前 LLM 模型标识。
    pub fn model_id(&self) -> Option<&str> {
        (!self.config.model.trim().is_empty()).then_some(self.config.model.trim())
    }

    /// provider 直连路径下当前可用的页面增强并行度。
    pub fn provider_parallel_requests(&self) -> usize {
        if self.selected_path() == Some(SelectedLlmPath::ProviderApi) {
            self.config.provider_parallel_requests()
        } else {
            1
        }
    }

    /// provider 直连路径下当前可用的 uncertainty gate 并行度。
    pub fn uncertainty_parallel_requests(&self) -> usize {
        if self.selected_path() == Some(SelectedLlmPath::ProviderApi) {
            3
        } else {
            1
        }
    }

    /// 当前 workflow 是否允许读已�?LLM cache。
    pub fn cache_reads_enabled(&self) -> bool {
        self.config.cache_mode != LlmCacheMode::Refresh
    }

    /// 当前 workflow �?cache mode。
    pub fn cache_mode(&self) -> LlmCacheMode {
        self.config.cache_mode
    }

    /// 返回当前累计 usage 快照。
    pub fn usage_snapshot(&self) -> LlmUsageSnapshot {
        let mut snapshot = self.usage.clone();
        snapshot.by_prompt_type = self.usage_by_prompt_type.values().cloned().collect();
        snapshot.by_provider_model = self.usage_by_provider_model.values().cloned().collect();
        snapshot
    }

    fn prompt_enabled(&self, prompt_type: PromptType) -> bool {
        match prompt_type {
            PromptType::FilePurpose
            | PromptType::TopLevelPromotion
            | PromptType::ModuleKind
            | PromptType::DependencyEdge
            | PromptType::PageResearch => self.config.enabled,
        }
    }

    fn record_completion_usage(&mut self, prompt_type: PromptType, completion: &LlmCompletion) {
        let Some(usage) = completion.usage.clone() else {
            return;
        };
        self.usage.request_count += usage.request_count;
        self.usage.input_tokens += usage.input_tokens;
        self.usage.output_tokens += usage.output_tokens;
        self.usage.total_tokens += usage.total_tokens;

        let prompt_key = prompt_type.as_str().to_string();
        let prompt_bucket = self
            .usage_by_prompt_type
            .entry(prompt_key.clone())
            .or_insert_with(|| LlmUsageBucket {
                key: prompt_key,
                ..LlmUsageBucket::default()
            });
        prompt_bucket.request_count += usage.request_count;
        prompt_bucket.input_tokens += usage.input_tokens;
        prompt_bucket.output_tokens += usage.output_tokens;
        prompt_bucket.total_tokens += usage.total_tokens;

        let provider_key = completion
            .model
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let provider_bucket = self
            .usage_by_provider_model
            .entry(provider_key.clone())
            .or_insert_with(|| LlmUsageBucket {
                key: provider_key,
                ..LlmUsageBucket::default()
            });
        provider_bucket.request_count += usage.request_count;
        provider_bucket.input_tokens += usage.input_tokens;
        provider_bucket.output_tokens += usage.output_tokens;
        provider_bucket.total_tokens += usage.total_tokens;

        let snapshot = self.usage_snapshot();
        if let Some(reporter) = self.usage_reporter.as_mut() {
            reporter(snapshot);
        }
    }

    fn record_chat_usage(&mut self, prompt_type: PromptType, chat: &ProviderChatResponse) {
        let completion = LlmCompletion {
            output: Value::Null,
            model: chat.model.clone(),
            usage: chat.usage.clone(),
        };
        self.record_completion_usage(prompt_type, &completion);
    }

    fn prepare_prompt_payload<T>(
        &self,
        prompt_type: PromptType,
        input: &T,
        session: Option<PageResearchSessionState>,
    ) -> io::Result<PreparedPromptPayload>
    where
        T: Serialize,
    {
        let mut input =
            serde_json::to_value(input).map_err(|error| io::Error::other(error.to_string()))?;
        let mut session = session.map(|state| trim_session_state(state, self.config));
        let estimated_tokens_before = estimate_payload_tokens(&input, session.as_ref());
        let limit_tokens = prompt_token_limit(prompt_type, self.config);

        apply_prompt_budget_trim(prompt_type, &mut input, &mut session, self.config);

        let estimated_tokens_after = estimate_payload_tokens(&input, session.as_ref());
        if estimated_tokens_after < estimated_tokens_before {
            debug_trace::record_json(
                "llm_budget_trim",
                &json!({
                    "prompt_type": prompt_type.as_str(),
                    "limit_tokens": limit_tokens,
                    "estimated_tokens_before": estimated_tokens_before,
                    "estimated_tokens_after": estimated_tokens_after,
                    "trimmed": true,
                }),
            );
        }

        Ok(PreparedPromptPayload { input, session })
    }

    /// 基于模糊文件角色补一个可选判断。
    pub fn classify_file_purpose(
        &mut self,
        input: &FilePurposeAssistInput,
    ) -> io::Result<Option<FilePurpose>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct Output {
            purpose: String,
        }

        self.request_structured(
            PromptType::FilePurpose,
            input,
            "你是 Repo Wiki 的扫描辅助模型，只能在给定候选语义附近做保守判断。",
            &format!(
                concat!(
                    "根据文件路径、语言和预览判断该文件最可能的稳定角色。",
                    "`purpose` 必须从以下稳定枚举中选择一个：{}。",
                    "不要返回中文描述、解释句或额外字段。"
                ),
                FILE_PURPOSE_ALLOWED_VALUES.join(", ")
            ),
            file_purpose_response_schema(false),
        )
        .map(|result: Option<Output>| result.and_then(|output| parse_file_purpose(&output.purpose)))
    }

    /// 按批次判断一�?`FilePurpose::Utility` 兜底文件。
    /// 每个文件仍按单条输入命中/写入缓存，只是把未命中的候选合并成更少的真实请求。
    pub fn classify_file_purposes(
        &mut self,
        inputs: &[FilePurposeAssistInput],
    ) -> io::Result<Vec<Option<FilePurpose>>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct SingleOutput {
            purpose: String,
        }

        if inputs.is_empty() {
            return Ok(Vec::new());
        }

        if !self.prompt_enabled(PromptType::FilePurpose) {
            return Ok(vec![None; inputs.len()]);
        }

        let model = self.model_id().map(str::to_string);
        let mut results = vec![None; inputs.len()];
        let mut pending = Vec::<(usize, String, FilePurposeAssistInput)>::new();

        for (index, input) in inputs.iter().enumerate() {
            let input_hash =
                build_prompt_input_hash(PromptType::FilePurpose, model.as_deref(), input);
            if self.cache_reads_enabled() {
                if let Some(cached) = read_llm_cache(
                    self.repo_root,
                    &input_hash,
                    PromptType::FilePurpose.as_str(),
                    PromptType::FilePurpose.version(),
                    model.as_deref(),
                )? {
                    if let Ok(parsed) = serde_json::from_str::<SingleOutput>(&cached.response) {
                        results[index] = parse_file_purpose(&parsed.purpose);
                        continue;
                    }
                }
            }

            if self.service.is_none() {
                continue;
            }

            pending.push((index, input_hash, input.clone()));
        }

        let mut pending_batches = Vec::<PendingPromptBatch<FilePurposeAssistInput>>::new();
        for chunk in pending.chunks(FILE_PURPOSE_BATCH_SIZE) {
            if !self.try_consume_budget(PromptType::FilePurpose, None) {
                break;
            }

            let batch_input = FilePurposeBatchInput {
                items: chunk.iter().map(|(_, _, input)| input.clone()).collect(),
            };
            let prepared =
                self.prepare_prompt_payload(PromptType::FilePurpose, &batch_input, None)?;
            let batch_input_hash =
                build_prompt_input_hash(PromptType::FilePurpose, model.as_deref(), &prepared.input);
            let request = build_structured_request(
                PromptType::FilePurpose,
                &batch_input_hash,
                model.clone(),
                "你是 Repo Wiki 的扫描辅助模型，只能在给定候选语义附近做保守判断。",
                &format!(
                    concat!(
                        "对输入 items 中的每个文件判断最可能的稳定角色。",
                        "返回 `items` 数组；每项必须保留原 `path`，并且 `purpose` 只能从以下稳定枚举中选择：{}。",
                        "不要返回中文描述、解释句或额外字段。"
                    ),
                    FILE_PURPOSE_ALLOWED_VALUES.join(", ")
                ),
                &prepared.input,
                file_purpose_response_schema(true),
                prepared.session,
            );
            pending_batches.push(PendingPromptBatch {
                items: chunk.to_vec(),
                request,
            });
        }

        let mut completions = vec![None; pending_batches.len()];
        let parallel_requests = self.uncertainty_parallel_requests();
        let run_in_parallel = self.selected_path() == Some(SelectedLlmPath::ProviderApi)
            && parallel_requests > 1
            && pending_batches.len() > 1;
        if run_in_parallel {
            let next_index = AtomicUsize::new(0);
            let completions_ref = Mutex::new(vec![None; pending_batches.len()]);
            let worker_count = parallel_requests.min(pending_batches.len());
            thread::scope(|scope| {
                for _ in 0..worker_count {
                    let next_index_ref = &next_index;
                    let batches_ref = &pending_batches;
                    let completions_lock = &completions_ref;
                    let config = self.config;
                    scope.spawn(move || {
                        let Ok(mut service) = ProviderApiLlmService::from_config(config) else {
                            return;
                        };
                        loop {
                            let task_index = next_index_ref.fetch_add(1, Ordering::Relaxed);
                            if task_index >= batches_ref.len() {
                                break;
                            }
                            completions_lock.lock().unwrap()[task_index] =
                                service.request(&batches_ref[task_index].request).ok();
                        }
                    });
                }
            });
            completions = completions_ref.into_inner().unwrap();
        } else {
            for (task_index, batch) in pending_batches.iter().enumerate() {
                let completion = match self.service.as_mut() {
                    Some(service) => service.request(&batch.request),
                    None => break,
                };
                completions[task_index] = completion.ok();
            }
        }

        for (task_index, completion) in completions.into_iter().enumerate() {
            let Some(completion) = completion else {
                continue;
            };
            self.record_completion_usage(PromptType::FilePurpose, &completion);
            let completion_model = completion.model.clone();
            let resolved = parse_file_purpose_batch_output(completion.output);
            if resolved.is_empty() {
                continue;
            }

            for (index, input_hash, input) in &pending_batches[task_index].items {
                let Some(purpose) = resolved.get(&input.path).copied() else {
                    continue;
                };
                let single_output = SingleOutput {
                    purpose: file_purpose_to_str(purpose).to_string(),
                };
                self.write_cached_response(
                    PromptType::FilePurpose,
                    input_hash.clone(),
                    model.clone(),
                    completion_model.clone(),
                    &single_output,
                )?;
                results[*index] = Some(purpose);
            }
        }

        Ok(results)
    }

    /// 为临界顶层目录提供可选晋升判断。
    pub fn decide_top_level_promotion(
        &mut self,
        input: &TopLevelPromotionAssistInput,
    ) -> io::Result<Option<bool>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct Output {
            promote: bool,
        }

        self.request_structured(
            PromptType::TopLevelPromotion,
            input,
            "你是 Repo Wiki 的模块边界辅助模型，只能做保守的是/否判断。",
            "判断该顶层目录是否值得提升为独立模块，只返回 `promote` 布尔值。",
            json!({
                "type": "object",
                "required": ["promote"],
                "properties": {
                    "promote": {"type": "boolean"}
                }
            }),
        )
        .map(|result: Option<Output>| result.map(|output| output.promote))
    }

    /// 批量判断一组临界顶层目录是否应晋升为独立模块。
    pub fn decide_top_level_promotions(
        &mut self,
        inputs: &[TopLevelPromotionAssistInput],
    ) -> io::Result<Vec<Option<bool>>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct SingleOutput {
            root_path: String,
            promote: bool,
        }

        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        if !self.prompt_enabled(PromptType::TopLevelPromotion) {
            return Ok(vec![None; inputs.len()]);
        }

        let model = self.model_id().map(str::to_string);
        let mut results = vec![None; inputs.len()];
        let mut pending = Vec::<(usize, String, TopLevelPromotionAssistInput)>::new();

        for (index, input) in inputs.iter().enumerate() {
            let input_hash =
                build_prompt_input_hash(PromptType::TopLevelPromotion, model.as_deref(), input);
            if self.cache_reads_enabled() {
                if let Some(cached) = read_llm_cache(
                    self.repo_root,
                    &input_hash,
                    PromptType::TopLevelPromotion.as_str(),
                    PromptType::TopLevelPromotion.version(),
                    model.as_deref(),
                )? {
                    if let Ok(parsed) = serde_json::from_str::<SingleOutput>(&cached.response) {
                        results[index] = Some(parsed.promote);
                        continue;
                    }
                }
            }

            if self.service.is_none() {
                continue;
            }
            pending.push((index, input_hash, input.clone()));
        }

        for chunk in pending.chunks(TOP_LEVEL_PROMOTION_BATCH_SIZE) {
            if !self.try_consume_budget(PromptType::TopLevelPromotion, None) {
                break;
            }

            let batch_input = TopLevelPromotionBatchInput {
                items: chunk.iter().map(|(_, _, input)| input.clone()).collect(),
            };
            let prepared =
                self.prepare_prompt_payload(PromptType::TopLevelPromotion, &batch_input, None)?;
            let batch_input_hash = build_prompt_input_hash(
                PromptType::TopLevelPromotion,
                model.as_deref(),
                &prepared.input,
            );
            let request = build_structured_request(
                PromptType::TopLevelPromotion,
                &batch_input_hash,
                model.clone(),
                "你是 Repo Wiki 的模块边界辅助模型，只能做保守的是/否判断。",
                "判断每个顶层目录是否值得提升为独立模块，返回 `items` 数组；每项必须保留原 `root_path` 和 `promote` 布尔值。",
                &prepared.input,
                json!({
                    "type": "object",
                    "required": ["items"],
                    "properties": {
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["root_path", "promote"],
                                "properties": {
                                    "root_path": {"type": "string"},
                                    "promote": {"type": "boolean"}
                                }
                            }
                        }
                    }
                }),
                prepared.session,
            );
            let completion = match self.service.as_mut() {
                Some(service) => service.request(&request),
                None => break,
            };
            let Ok(completion) = completion else {
                continue;
            };
            self.record_completion_usage(PromptType::TopLevelPromotion, &completion);
            let completion_model = completion.model.clone();
            let resolved = completion
                .output
                .get("items")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|item| {
                    let root_path = item.get("root_path")?.as_str()?.to_string();
                    let promote = item.get("promote")?.as_bool()?;
                    Some((root_path, promote))
                })
                .collect::<BTreeMap<_, _>>();
            for (index, input_hash, input) in chunk {
                let Some(promote) = resolved.get(&input.root_path).copied() else {
                    continue;
                };
                let single_output = SingleOutput {
                    root_path: input.root_path.clone(),
                    promote,
                };
                self.write_cached_response(
                    PromptType::TopLevelPromotion,
                    input_hash.clone(),
                    model.clone(),
                    completion_model.clone(),
                    &single_output,
                )?;
                results[*index] = Some(promote);
            }
        }

        Ok(results)
    }

    /// �?`module_kind` 兜底分支提供可选判断。
    pub fn classify_module_kind(
        &mut self,
        input: &ModuleKindAssistInput,
    ) -> io::Result<Option<String>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct Output {
            kind: String,
        }

        self.request_structured(
            PromptType::ModuleKind,
            input,
            "你是 Repo Wiki 的模块分类辅助模型，只能在既有 kind 集合内做保守判断。",
            "判断该模块更适合作为哪种 kind，只返回 `kind` 字段。",
            json!({
                "type": "object",
                "required": ["kind"],
                "properties": {
                    "kind": {"type": "string"}
                }
            }),
        )
        .map(|result: Option<Output>| result.and_then(|output| sanitize_module_kind(&output.kind)))
    }

    /// 为低置信度跨模块关系提供可选保留判断。
    pub fn keep_dependency_edge(
        &mut self,
        input: &DependencyAssistInput,
    ) -> io::Result<Option<bool>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct Output {
            keep: bool,
        }

        self.request_structured(
            PromptType::DependencyEdge,
            input,
            "你是 Repo Wiki 的关系辅助模型，只能判断该关系是否值得保留。",
            "根据 source/target 路径和模块名判断这条低置信度关系是否应保留，只返回 `keep` 布尔值。",
            json!({
                "type": "object",
                "required": ["keep"],
                "properties": {
                    "keep": {"type": "boolean"}
                }
            }),
        )
        .map(|result: Option<Output>| result.map(|output| output.keep))
    }

    /// 批量判断一组低置信度跨模块关系是否值得保留。
    pub fn keep_dependency_edges(
        &mut self,
        inputs: &[DependencyAssistInput],
    ) -> io::Result<Vec<Option<bool>>> {
        #[derive(Debug, Deserialize, Serialize)]
        struct SingleOutput {
            key: String,
            keep: bool,
        }

        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        if !self.prompt_enabled(PromptType::DependencyEdge) {
            return Ok(vec![None; inputs.len()]);
        }

        let model = self.model_id().map(str::to_string);
        let mut results = vec![None; inputs.len()];
        let mut pending = Vec::<(usize, String, DependencyAssistInput)>::new();

        for (index, input) in inputs.iter().enumerate() {
            let input_hash =
                build_prompt_input_hash(PromptType::DependencyEdge, model.as_deref(), input);
            if self.cache_reads_enabled() {
                if let Some(cached) = read_llm_cache(
                    self.repo_root,
                    &input_hash,
                    PromptType::DependencyEdge.as_str(),
                    PromptType::DependencyEdge.version(),
                    model.as_deref(),
                )? {
                    if let Ok(parsed) = serde_json::from_str::<SingleOutput>(&cached.response) {
                        results[index] = Some(parsed.keep);
                        continue;
                    }
                }
            }
            if self.service.is_none() {
                continue;
            }
            pending.push((index, input_hash, input.clone()));
        }

        for chunk in pending.chunks(DEPENDENCY_EDGE_BATCH_SIZE) {
            if !self.try_consume_budget(PromptType::DependencyEdge, None) {
                break;
            }

            let batch_input = DependencyAssistBatchInput {
                items: chunk.iter().map(|(_, _, input)| input.clone()).collect(),
            };
            let prepared =
                self.prepare_prompt_payload(PromptType::DependencyEdge, &batch_input, None)?;
            let batch_input_hash = build_prompt_input_hash(
                PromptType::DependencyEdge,
                model.as_deref(),
                &prepared.input,
            );
            let request = build_structured_request(
                PromptType::DependencyEdge,
                &batch_input_hash,
                model.clone(),
                "你是 Repo Wiki 的关系辅助模型，只能判断低置信度跨模块关系是否值得保留。",
                "根据 source/target 路径和模块名判断每条低置信度关系是否应保留，返回 `items` 数组；每项必须保留 `key` 和 `keep` 布尔值。",
                &prepared.input,
                json!({
                    "type": "object",
                    "required": ["items"],
                    "properties": {
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["key", "keep"],
                                "properties": {
                                    "key": {"type": "string"},
                                    "keep": {"type": "boolean"}
                                }
                            }
                        }
                    }
                }),
                prepared.session,
            );
            let completion = match self.service.as_mut() {
                Some(service) => service.request(&request),
                None => break,
            };
            let Ok(completion) = completion else {
                continue;
            };
            self.record_completion_usage(PromptType::DependencyEdge, &completion);
            let completion_model = completion.model.clone();
            let resolved = completion
                .output
                .get("items")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|item| {
                    let key = item.get("key")?.as_str()?.to_string();
                    let keep = item.get("keep")?.as_bool()?;
                    Some((key, keep))
                })
                .collect::<BTreeMap<_, _>>();
            for (index, input_hash, input) in chunk {
                let key = dependency_assist_key(input);
                let Some(keep) = resolved.get(&key).copied() else {
                    continue;
                };
                let single_output = SingleOutput { key, keep };
                self.write_cached_response(
                    PromptType::DependencyEdge,
                    input_hash.clone(),
                    model.clone(),
                    completion_model.clone(),
                    &single_output,
                )?;
                results[*index] = Some(keep);
            }
        }

        Ok(results)
    }

    /// �?research 支持页面执行 bounded provider research session。
    pub fn research_page(
        &mut self,
        input: &PageResearchInput,
        runtime: &PageResearchRuntimeContext<'_>,
    ) -> io::Result<PageResearchSessionResult> {
        if !matches!(
            input.page_type.as_str(),
            "overview" | "architecture" | "module" | "topic" | "family-index" | "family-child"
        ) || !(self.service_available() && self.config.enabled)
        {
            return Ok(PageResearchSessionResult::not_run());
        }
        let Some(RuntimeLlmService::Provider(_)) = self.service.as_ref() else {
            return Ok(PageResearchSessionResult::not_run());
        };
        if !self.try_consume_budget(PromptType::PageResearch, Some(input.page_type.as_str())) {
            return Ok(PageResearchSessionResult::call_budget_rejected());
        }

        let model = self.model_id().map(str::to_string);
        let prepared =
            self.prepare_prompt_payload(PromptType::PageResearch, input, input.session.clone())?;
        let input_hash =
            build_prompt_input_hash(PromptType::PageResearch, model.as_deref(), &prepared.input);
        if self.cache_reads_enabled() {
            if let Some(cached) = read_llm_cache(
                self.repo_root,
                &input_hash,
                PromptType::PageResearch.as_str(),
                PromptType::PageResearch.version(),
                model.as_deref(),
            )? {
                if is_negative_cache_payload(&cached.response) {
                    return Ok(PageResearchSessionResult::invalid_output(
                        ResearchSessionStats::default(),
                    ));
                }
                if let Ok(parsed) = serde_json::from_str::<PageResearchResult>(&cached.response) {
                    if let Some(result) = parsed
                        .sanitize_for_context(runtime.page_context, runtime.allowed_section_slots)
                    {
                        let mut session =
                            input
                                .session
                                .clone()
                                .unwrap_or_else(|| PageResearchSessionState {
                                    session_id: stable_id("research-session", &input.page_id),
                                    session_summary: String::new(),
                                    recent_turns: Vec::new(),
                                    tool_artifact_refs: Vec::new(),
                                });
                        session.session_summary = result.summary.clone();
                        let stats = Self::build_result_session_stats(0, 0, &result);
                        return Ok(PageResearchSessionResult::completed(
                            PageResearchSessionOutput { session, result },
                            ResearchStopReason::Completed,
                            stats,
                        ));
                    }
                }
            }
        }

        let mut request = build_structured_request(
            PromptType::PageResearch,
            &input_hash,
            model.clone(),
            "你是 Repo Wiki 的 research session 模型，只能围绕当前 dossier、child rollup、evidence 和工具结果做保守研究，不得直接输出 Markdown 页面。",
            &build_page_research_instruction(input),
            &prepared.input,
            page_research_response_schema(),
            prepared.session,
        );

        let explicit_tools_mode = self
            .config
            .resolve_selected_model()
            .map(|selected| selected.provider.capabilities.tools_mode())
            .unwrap_or(LlmToolsMode::NoTools);
        let effective_tools_mode = if explicit_tools_mode == LlmToolsMode::Auto {
            self.config
                .resolve_selected_model()
                .and_then(|selected| {
                    resolve_learned_tools_mode(
                        selected.provider_name,
                        selected.provider,
                        selected.model_name,
                    )
                })
                .unwrap_or(LlmToolsMode::NativeTools)
        } else {
            explicit_tools_mode
        };

        let session_result = match effective_tools_mode {
            LlmToolsMode::NativeTools => self.run_provider_research_with_fallback(
                &mut request,
                runtime,
                LlmToolsMode::NativeTools,
            )?,
            LlmToolsMode::EmulatedTools => self.run_provider_research_with_fallback(
                &mut request,
                runtime,
                LlmToolsMode::EmulatedTools,
            )?,
            LlmToolsMode::NoTools | LlmToolsMode::Auto => {
                self.run_provider_research_no_tools(&request, runtime)?
            }
        };

        if session_result.output.is_none() {
            if session_result.stop_reason == ResearchStopReason::InvalidOutput {
                self.write_negative_cached_response(
                    PromptType::PageResearch,
                    input_hash,
                    model,
                    None,
                    "invalid_output",
                )?;
            }
            return Ok(session_result);
        }
        let output = session_result
            .output
            .as_ref()
            .expect("session output should exist");
        self.write_cached_response(
            PromptType::PageResearch,
            input_hash,
            model,
            Some(
                self.config
                    .resolve_selected_model()
                    .map(|selected| {
                        selected
                            .model
                            .resolved_model_id(selected.model_name)
                            .to_string()
                    })
                    .unwrap_or_default(),
            ),
            &output.result,
        )?;
        Ok(session_result)
    }

    fn run_provider_research_with_fallback(
        &mut self,
        request: &mut LlmPromptRequest,
        runtime: &PageResearchRuntimeContext<'_>,
        mode: LlmToolsMode,
    ) -> io::Result<PageResearchSessionResult> {
        let result = match mode {
            LlmToolsMode::NativeTools => self.run_provider_research_native_tools(request, runtime),
            LlmToolsMode::EmulatedTools => {
                self.run_provider_research_emulated_tools(request, runtime)
            }
            _ => self.run_provider_research_no_tools(request, runtime),
        };
        match result {
            Ok(output) => Ok(output),
            Err(error) if is_provider_tools_unsupported(&error) => {
                if let Some(selected) = self.config.resolve_selected_model() {
                    let next_mode = match mode {
                        LlmToolsMode::NativeTools => LlmToolsMode::EmulatedTools,
                        LlmToolsMode::EmulatedTools => LlmToolsMode::NoTools,
                        _ => LlmToolsMode::NoTools,
                    };
                    let _ = persist_learned_tools_mode(
                        selected.provider_name,
                        selected.model_name,
                        selected.provider,
                        next_mode,
                        "provider_rejected_tools",
                        PROVIDER_TOOLS_TTL_HOURS,
                    );
                    return match next_mode {
                        LlmToolsMode::EmulatedTools => {
                            self.run_provider_research_emulated_tools(request, runtime)
                        }
                        LlmToolsMode::NoTools | LlmToolsMode::Auto | LlmToolsMode::NativeTools => {
                            self.run_provider_research_no_tools(request, runtime)
                        }
                    };
                }
                Err(error)
            }
            Err(error) => Err(error),
        }
    }

    fn run_provider_research_no_tools(
        &mut self,
        request: &LlmPromptRequest,
        runtime: &PageResearchRuntimeContext<'_>,
    ) -> io::Result<PageResearchSessionResult> {
        let completion = match self.service.as_mut() {
            Some(RuntimeLlmService::Provider(service)) => service.request(request)?,
            _ => return Ok(PageResearchSessionResult::not_run()),
        };
        self.record_completion_usage(PromptType::PageResearch, &completion);
        let Some(result) = parse_page_research_output(completion.output, runtime.page_context)
            .and_then(|result| {
                result.sanitize_for_context(runtime.page_context, runtime.allowed_section_slots)
            })
        else {
            return Ok(PageResearchSessionResult::invalid_output(
                ResearchSessionStats {
                    turns_used: 1,
                    ..ResearchSessionStats::default()
                },
            ));
        };
        let stats = Self::build_result_session_stats(1, 0, &result);
        if !Self::page_research_result_is_minimally_complete(&result) {
            return Ok(PageResearchSessionResult::invalid_output(stats));
        }
        Ok(PageResearchSessionResult::completed(
            PageResearchSessionOutput {
                session: finalize_research_session(
                    &runtime.page.id,
                    request.session.as_ref(),
                    &result,
                    vec![PageResearchTurn {
                        role: "assistant".to_string(),
                        content: result.summary.clone(),
                    }],
                    Vec::new(),
                ),
                result,
            },
            ResearchStopReason::NoFurtherToolCalls,
            stats,
        ))
    }

    fn run_provider_research_native_tools(
        &mut self,
        request: &mut LlmPromptRequest,
        runtime: &PageResearchRuntimeContext<'_>,
    ) -> io::Result<PageResearchSessionResult> {
        request.tools = research_tool_definitions();
        request.tool_choice = Some(json!("auto"));
        self.run_provider_research_loop(request, runtime, false)
    }

    fn run_provider_research_emulated_tools(
        &mut self,
        request: &mut LlmPromptRequest,
        runtime: &PageResearchRuntimeContext<'_>,
    ) -> io::Result<PageResearchSessionResult> {
        request.tools = research_tool_definitions();
        request.tool_choice = Some(json!("auto"));
        self.run_provider_research_loop(request, runtime, true)
    }

    fn run_provider_research_loop(
        &mut self,
        request: &LlmPromptRequest,
        runtime: &PageResearchRuntimeContext<'_>,
        emulated_tools: bool,
    ) -> io::Result<PageResearchSessionResult> {
        let mut messages = ProviderApiLlmService::default_messages(request);
        let mut recent_turns = request
            .session
            .as_ref()
            .map(|session| session.recent_turns.clone())
            .unwrap_or_default();
        let mut tool_artifact_refs = request
            .session
            .as_ref()
            .map(|session| session.tool_artifact_refs.clone())
            .unwrap_or_default();
        let max_turns = self.config.page_research_max_turns.clamp(4, 16);
        let mut tool_call_count = 0usize;
        let mut no_delta_rounds = 0usize;
        let mut seen_tool_signatures = BTreeSet::new();

        if emulated_tools {
            messages[1]["content"] = Value::String(build_emulated_tool_user_message(request));
        }

        for turn_index in 0..max_turns {
            let turns_used = turn_index + 1;
            debug_trace::record_json(
                "llm_research_session_turn",
                &json!({
                    "request_id": request.request_id,
                    "page_id": runtime.page.id,
                    "turn_index": recent_turns.len(),
                    "session_id": request.session.as_ref().map(|session| session.session_id.clone()),
                    "tool_artifact_refs": tool_artifact_refs,
                }),
            );
            let chat = match self.service.as_mut() {
                Some(RuntimeLlmService::Provider(service)) => {
                    service.request_chat_with_messages(request, &messages)?
                }
                _ => return Ok(PageResearchSessionResult::not_run()),
            };
            self.record_chat_usage(PromptType::PageResearch, &chat);
            if !chat.tool_calls.is_empty() {
                let assistant_message = json!({
                    "role": "assistant",
                    "content": Value::Null,
                    "tool_calls": chat.tool_calls,
                });
                messages.push(assistant_message.clone());
                recent_turns.push(PageResearchTurn {
                    role: "assistant".to_string(),
                    content: "tool_call".to_string(),
                });
                let mut new_tool_signatures = 0usize;
                let mut new_artifacts = 0usize;
                for tool_call in assistant_message
                    .get("tool_calls")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                {
                    let parsed = serde_json::from_value::<ProviderToolCall>(tool_call)
                        .map_err(|error| io::Error::other(error.to_string()))?;
                    tool_call_count += 1;
                    if seen_tool_signatures.insert(Self::tool_call_signature(&parsed)) {
                        new_tool_signatures += 1;
                    }
                    let (result, artifact_ref) = execute_research_tool(&parsed, runtime)?;
                    if let Some(artifact_ref) = artifact_ref {
                        new_artifacts += 1;
                        tool_artifact_refs.push(artifact_ref);
                    }
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": parsed.id,
                        "name": parsed.function.name,
                        "content": serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()),
                    }));
                    recent_turns.push(PageResearchTurn {
                        role: "tool".to_string(),
                        content: parsed.function.name,
                    });
                }
                if new_tool_signatures == 0 && new_artifacts == 0 {
                    no_delta_rounds += 1;
                } else {
                    no_delta_rounds = 0;
                }
                if no_delta_rounds >= 2 {
                    return Ok(PageResearchSessionResult::no_meaningful_delta(
                        ResearchSessionStats {
                            turns_used,
                            tool_calls: tool_call_count,
                            ..ResearchSessionStats::default()
                        },
                    ));
                }
                continue;
            }

            if let Some(content) = chat.content.as_deref() {
                if emulated_tools {
                    let value = match parse_provider_json_output(content) {
                        Ok(value) => value,
                        Err(_) => {
                            return Ok(PageResearchSessionResult::invalid_output(
                                ResearchSessionStats {
                                    turns_used,
                                    tool_calls: tool_call_count,
                                    ..ResearchSessionStats::default()
                                },
                            ))
                        }
                    };
                    if let Some(tool_calls) = value.get("tool_calls").and_then(Value::as_array) {
                        let assistant_message = json!({
                            "role": "assistant",
                            "content": Value::Null,
                            "tool_calls": tool_calls.clone(),
                        });
                        messages.push(assistant_message);
                        let mut new_tool_signatures = 0usize;
                        let mut new_artifacts = 0usize;
                        for tool_call in tool_calls {
                            let parsed =
                                serde_json::from_value::<ProviderToolCall>(tool_call.clone())
                                    .map_err(|error| io::Error::other(error.to_string()))?;
                            tool_call_count += 1;
                            if seen_tool_signatures.insert(Self::tool_call_signature(&parsed)) {
                                new_tool_signatures += 1;
                            }
                            let (result, artifact_ref) = execute_research_tool(&parsed, runtime)?;
                            if let Some(artifact_ref) = artifact_ref {
                                new_artifacts += 1;
                                tool_artifact_refs.push(artifact_ref);
                            }
                            messages.push(json!({
                                "role": "tool",
                                "tool_call_id": parsed.id,
                                "name": parsed.function.name,
                                "content": serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()),
                            }));
                        }
                        if new_tool_signatures == 0 && new_artifacts == 0 {
                            no_delta_rounds += 1;
                        } else {
                            no_delta_rounds = 0;
                        }
                        if no_delta_rounds >= 2 {
                            return Ok(PageResearchSessionResult::no_meaningful_delta(
                                ResearchSessionStats {
                                    turns_used,
                                    tool_calls: tool_call_count,
                                    ..ResearchSessionStats::default()
                                },
                            ));
                        }
                        continue;
                    }
                    let final_result = value
                        .get("result")
                        .cloned()
                        .or_else(|| value.get("output").cloned())
                        .unwrap_or(value);
                    let Some(result) =
                        parse_page_research_output(final_result, runtime.page_context).and_then(
                            |result| {
                                result.sanitize_for_context(
                                    runtime.page_context,
                                    runtime.allowed_section_slots,
                                )
                            },
                        )
                    else {
                        return Ok(PageResearchSessionResult::invalid_output(
                            ResearchSessionStats {
                                turns_used,
                                tool_calls: tool_call_count,
                                ..ResearchSessionStats::default()
                            },
                        ));
                    };
                    let stats =
                        Self::build_result_session_stats(turns_used, tool_call_count, &result);
                    if !Self::page_research_result_is_minimally_complete(&result) {
                        return Ok(PageResearchSessionResult::invalid_output(stats));
                    }
                    debug_trace::record_json(
                        "llm_research_session_final",
                        &json!({
                            "request_id": request.request_id,
                            "page_id": runtime.page.id,
                            "emulated_tools": true,
                            "recent_turns": recent_turns,
                            "tool_artifact_refs": tool_artifact_refs,
                            "stop_reason": ResearchStopReason::Completed.as_str(),
                            "stats": stats,
                            "result": result,
                        }),
                    );
                    return Ok(PageResearchSessionResult::completed(
                        PageResearchSessionOutput {
                            session: finalize_research_session(
                                &runtime.page.id,
                                request.session.as_ref(),
                                &result,
                                recent_turns,
                                tool_artifact_refs,
                            ),
                            result,
                        },
                        ResearchStopReason::Completed,
                        stats,
                    ));
                }

                let value = match parse_provider_json_output(content) {
                    Ok(value) => value,
                    Err(_) => {
                        return Ok(PageResearchSessionResult::invalid_output(
                            ResearchSessionStats {
                                turns_used,
                                tool_calls: tool_call_count,
                                ..ResearchSessionStats::default()
                            },
                        ))
                    }
                };
                let Some(result) = parse_page_research_output(value, runtime.page_context)
                    .and_then(|result| {
                        result.sanitize_for_context(
                            runtime.page_context,
                            runtime.allowed_section_slots,
                        )
                    })
                else {
                    return Ok(PageResearchSessionResult::invalid_output(
                        ResearchSessionStats {
                            turns_used,
                            tool_calls: tool_call_count,
                            ..ResearchSessionStats::default()
                        },
                    ));
                };
                let stats = Self::build_result_session_stats(turns_used, tool_call_count, &result);
                if !Self::page_research_result_is_minimally_complete(&result) {
                    return Ok(PageResearchSessionResult::invalid_output(stats));
                }
                let stop_reason = if tool_call_count > 0 {
                    ResearchStopReason::Completed
                } else {
                    ResearchStopReason::NoFurtherToolCalls
                };
                debug_trace::record_json(
                    "llm_research_session_final",
                    &json!({
                        "request_id": request.request_id,
                        "page_id": runtime.page.id,
                        "emulated_tools": false,
                        "recent_turns": recent_turns,
                        "tool_artifact_refs": tool_artifact_refs,
                        "stop_reason": stop_reason.as_str(),
                        "stats": stats,
                        "result": result,
                    }),
                );
                return Ok(PageResearchSessionResult::completed(
                    PageResearchSessionOutput {
                        session: finalize_research_session(
                            &runtime.page.id,
                            request.session.as_ref(),
                            &result,
                            recent_turns,
                            tool_artifact_refs,
                        ),
                        result,
                    },
                    stop_reason,
                    stats,
                ));
            }

            no_delta_rounds += 1;
            if no_delta_rounds >= 2 {
                return Ok(PageResearchSessionResult::no_meaningful_delta(
                    ResearchSessionStats {
                        turns_used,
                        tool_calls: tool_call_count,
                        ..ResearchSessionStats::default()
                    },
                ));
            }
        }

        Ok(PageResearchSessionResult::turn_budget_exhausted(
            ResearchSessionStats {
                turns_used: max_turns,
                tool_calls: tool_call_count,
                ..ResearchSessionStats::default()
            },
        ))
    }

    fn build_result_session_stats(
        turns_used: usize,
        tool_calls: usize,
        result: &PageResearchResult,
    ) -> ResearchSessionStats {
        ResearchSessionStats {
            turns_used,
            tool_calls,
            delta_evidence_count: result.evidence_rollup.len(),
            delta_section_count: result.section_plan.len(),
            delta_diagram_count: result.diagram_rollup.len(),
            child_digest_delta: result
                .section_plan
                .iter()
                .map(|section| section.child_refs.len())
                .sum(),
        }
    }

    fn page_research_result_is_minimally_complete(result: &PageResearchResult) -> bool {
        (!result.summary.trim().is_empty() || !result.page_positioning.trim().is_empty())
            && (!result.section_plan.is_empty()
                || !result.evidence_rollup.is_empty()
                || !result.diagram_rollup.is_empty())
    }

    fn tool_call_signature(tool_call: &ProviderToolCall) -> String {
        format!(
            "{}:{}",
            tool_call.function.name.trim(),
            tool_call.function.arguments.trim()
        )
    }

    fn request_structured<TInput, TOutput>(
        &mut self,
        prompt_type: PromptType,
        input: &TInput,
        system: &str,
        instruction: &str,
        response_schema: Value,
    ) -> io::Result<Option<TOutput>>
    where
        TInput: Serialize,
        TOutput: DeserializeOwned + Serialize,
    {
        if !self.prompt_enabled(prompt_type) {
            return Ok(None);
        }

        let model = self.model_id().map(str::to_string);
        let prepared = self.prepare_prompt_payload(prompt_type, input, None)?;
        let input_hash = build_prompt_input_hash(prompt_type, model.as_deref(), &prepared.input);
        if self.cache_reads_enabled() {
            if let Some(cached) = read_llm_cache(
                self.repo_root,
                &input_hash,
                prompt_type.as_str(),
                prompt_type.version(),
                model.as_deref(),
            )? {
                if is_negative_cache_payload(&cached.response) {
                    return Ok(None);
                }
                if let Ok(parsed) = serde_json::from_str::<TOutput>(&cached.response) {
                    return Ok(Some(parsed));
                }
            }
        }

        if self.service.is_none() {
            return Ok(None);
        }

        if !self.try_consume_budget(prompt_type, None) {
            return Ok(None);
        }

        let request = build_structured_request(
            prompt_type,
            &input_hash,
            model.clone(),
            system,
            instruction,
            &prepared.input,
            response_schema,
            prepared.session,
        );

        let completion = match self.service.as_mut() {
            Some(service) => service.request(&request),
            None => return Ok(None),
        };

        let completion = match completion {
            Ok(completion) => completion,
            Err(_) => return Ok(None),
        };
        self.record_completion_usage(prompt_type, &completion);
        let completion_model = completion.model.clone();
        let parsed = match serde_json::from_value::<TOutput>(completion.output.clone()) {
            Ok(parsed) => parsed,
            Err(_) => {
                self.write_negative_cached_response(
                    prompt_type,
                    input_hash,
                    model,
                    completion_model,
                    "invalid_output",
                )?;
                return Ok(None);
            }
        };

        self.write_cached_response(prompt_type, input_hash, model, completion.model, &parsed)?;

        Ok(Some(parsed))
    }

    fn budget_available(&self, prompt_type: PromptType) -> bool {
        match prompt_type {
            PromptType::PageResearch => false,
            _ => self.uncertainty_calls < self.max_uncertainty_calls(),
        }
    }

    fn page_budget_available(&self, page_type: Option<&str>) -> bool {
        if self.page_calls >= self.max_page_calls() {
            return false;
        }

        let reserved_core_slots = self.reserved_core_page_calls();
        let non_core_page_calls = self.page_calls.saturating_sub(self.core_page_calls);
        let non_core_budget_available =
            non_core_page_calls < self.max_page_calls().saturating_sub(reserved_core_slots);

        if is_core_page_type(page_type) {
            self.core_page_calls < reserved_core_slots || non_core_budget_available
        } else {
            non_core_budget_available
        }
    }

    fn page_research_budget_available(&self, page_type: Option<&str>) -> bool {
        self.page_budget_available(page_type)
    }

    fn try_consume_budget(&mut self, prompt_type: PromptType, page_type: Option<&str>) -> bool {
        let allowed = match prompt_type {
            PromptType::PageResearch => self.page_research_budget_available(page_type),
            _ => self.budget_available(prompt_type),
        };
        if !allowed {
            debug_trace::record_json(
                "llm_budget_rejected",
                &json!({
                    "prompt_type": prompt_type.as_str(),
                    "page_type": page_type,
                    "real_calls": self.real_calls,
                    "uncertainty_calls": self.uncertainty_calls,
                    "page_calls": self.page_calls,
                    "core_page_calls": self.core_page_calls,
                    "max_calls": self.config.max_calls,
                    "max_uncertainty_calls": self.max_uncertainty_calls(),
                    "max_page_calls": self.max_page_calls(),
                    "reserved_core_page_calls": self.reserved_core_page_calls(),
                }),
            );
            return false;
        }

        self.real_calls += 1;
        match prompt_type {
            PromptType::PageResearch => {
                self.page_calls += 1;
                if is_core_page_type(page_type) {
                    self.core_page_calls += 1;
                }
            }
            _ => self.uncertainty_calls += 1,
        }
        true
    }

    fn max_page_calls(&self) -> usize {
        self.config.max_research_calls
    }

    fn reserved_core_page_calls(&self) -> usize {
        reserved_core_page_call_budget(self.max_page_calls())
    }

    fn max_uncertainty_calls(&self) -> usize {
        self.config.max_calls
    }

    fn write_cached_response<TOutput>(
        &self,
        prompt_type: PromptType,
        input_hash: String,
        model: Option<String>,
        completion_model: Option<String>,
        parsed: &TOutput,
    ) -> io::Result<()>
    where
        TOutput: Serialize,
    {
        let response_json =
            serde_json::to_string(parsed).map_err(|error| io::Error::other(error.to_string()))?;
        write_llm_cache(
            self.repo_root,
            &LlmCacheEntry {
                input_hash: input_hash.clone(),
                prompt_type: prompt_type.as_str().to_string(),
                prompt_version: prompt_type.version().to_string(),
                response: response_json,
                model: model.or(completion_model),
                created_at: crate::workflows::init::current_timestamp(),
                ttl_seconds: self.config.cache_ttl_seconds as i64,
            },
        )
        .inspect_err(|error| {
            debug_trace::record_json(
                "llm_cache_write_error",
                &json!({
                    "prompt_type": prompt_type.as_str(),
                    "input_hash": input_hash,
                    "error": error.to_string(),
                }),
            );
        })
    }

    fn write_negative_cached_response(
        &self,
        prompt_type: PromptType,
        input_hash: String,
        model: Option<String>,
        completion_model: Option<String>,
        reason: &str,
    ) -> io::Result<()> {
        write_llm_cache(
            self.repo_root,
            &LlmCacheEntry {
                input_hash: input_hash.clone(),
                prompt_type: prompt_type.as_str().to_string(),
                prompt_version: prompt_type.version().to_string(),
                response: negative_cache_payload(reason),
                model: model.or(completion_model),
                created_at: crate::workflows::init::current_timestamp(),
                ttl_seconds: self.config.cache_ttl_seconds as i64,
            },
        )
        .inspect_err(|error| {
            debug_trace::record_json(
                "llm_cache_write_error",
                &json!({
                    "prompt_type": prompt_type.as_str(),
                    "input_hash": input_hash,
                    "reason": reason,
                    "error": error.to_string(),
                }),
            );
        })
    }
}

/// 为页面输入计算稳定的 prompt 级哈希。
pub fn build_prompt_input_hash<T>(prompt_type: PromptType, model: Option<&str>, input: &T) -> String
where
    T: Serialize,
{
    let serialized = serde_json::to_vec(&json!({
        "prompt_type": prompt_type.as_str(),
        "prompt_version": prompt_type.version(),
        "model": model.unwrap_or_default(),
        "input": input,
    }))
    .unwrap_or_default();
    fingerprint_bytes(&serialized)
}

fn build_structured_request<TInput>(
    prompt_type: PromptType,
    input_hash: &str,
    model: Option<String>,
    system: &str,
    instruction: &str,
    input: &TInput,
    response_schema: Value,
    session: Option<PageResearchSessionState>,
) -> LlmPromptRequest
where
    TInput: Serialize,
{
    LlmPromptRequest {
        request_id: stable_id(
            "llm-request",
            &format!("{}:{input_hash}", prompt_type.as_str()),
        ),
        prompt_type: prompt_type.as_str().to_string(),
        prompt_version: prompt_type.version().to_string(),
        input_hash: input_hash.to_string(),
        model,
        system: system.to_string(),
        instruction: instruction.to_string(),
        input: serde_json::to_value(input).unwrap_or(Value::Null),
        response_schema,
        tools: Vec::new(),
        tool_choice: None,
        response_format: None,
        session,
    }
}

fn negative_cache_payload(reason: &str) -> String {
    json!({
        "_cache_status": NEGATIVE_LLM_CACHE_STATUS,
        "reason": reason,
    })
    .to_string()
}

fn is_negative_cache_payload(payload: &str) -> bool {
    serde_json::from_str::<Value>(payload)
        .ok()
        .and_then(|value| {
            value
                .get("_cache_status")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .is_some_and(|status| status == NEGATIVE_LLM_CACHE_STATUS)
}

fn prompt_token_limit(prompt_type: PromptType, _config: &LlmConfig) -> usize {
    match prompt_type {
        PromptType::FilePurpose
        | PromptType::TopLevelPromotion
        | PromptType::ModuleKind
        | PromptType::DependencyEdge => 12_000_usize.max(256),
        PromptType::PageResearch => 16_000_usize.max(512),
    }
}

fn estimate_payload_tokens(input: &Value, session: Option<&PageResearchSessionState>) -> usize {
    estimate_tokens(
        &serde_json::to_string(&json!({
            "input": input,
            "session": session,
        }))
        .unwrap_or_default(),
    )
}

fn trim_session_state(
    mut state: PageResearchSessionState,
    config: &LlmConfig,
) -> PageResearchSessionState {
    let max_turns = config.page_research_max_turns.clamp(4, 16);
    if state.recent_turns.len() > max_turns {
        state.recent_turns = state
            .recent_turns
            .into_iter()
            .rev()
            .take(max_turns)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    }
    if state.tool_artifact_refs.len() > max_turns {
        state.tool_artifact_refs = state
            .tool_artifact_refs
            .into_iter()
            .rev()
            .take(max_turns)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    }
    truncate_string(&mut state.session_summary, 1_200);
    for turn in &mut state.recent_turns {
        truncate_string(&mut turn.content, 800);
    }
    for artifact in &mut state.tool_artifact_refs {
        truncate_string(&mut artifact.summary, 800);
    }
    state
}

fn apply_prompt_budget_trim(
    prompt_type: PromptType,
    input: &mut Value,
    session: &mut Option<PageResearchSessionState>,
    config: &LlmConfig,
) {
    let limit_tokens = prompt_token_limit(prompt_type, config);
    if estimate_payload_tokens(input, session.as_ref()) <= limit_tokens {
        return;
    }

    if let Some(object) = input.as_object_mut() {
        match prompt_type {
            PromptType::PageResearch => {
                trim_named_array_field(object, &["evidence_groups", "diagram_inputs"], 6);
                trim_named_array_field(object, &["summary_inputs", "hints", "facts"], 10);
                trim_named_array_field(
                    object,
                    &[
                        "targeted_snippets",
                        "evidence_rollup",
                        "family_scoped_evidence",
                        "child_page_results",
                        "docs_anchors",
                        "public_api_surfaces",
                        "config_surfaces",
                        "type_surfaces",
                    ],
                    8,
                );
            }
            PromptType::FilePurpose => {
                trim_named_array_field(object, &["items"], FILE_PURPOSE_BATCH_SIZE);
            }
            PromptType::TopLevelPromotion => {
                trim_named_array_field(object, &["items"], TOP_LEVEL_PROMOTION_BATCH_SIZE);
            }
            PromptType::DependencyEdge => {
                trim_named_array_field(object, &["items"], DEPENDENCY_EDGE_BATCH_SIZE);
            }
            PromptType::ModuleKind => {}
        }
    }

    let mut max_string_chars = match prompt_type {
        PromptType::FilePurpose => 1_200,
        PromptType::TopLevelPromotion | PromptType::DependencyEdge | PromptType::ModuleKind => 800,
        PromptType::PageResearch => 2_000,
    };
    let mut max_array_items = match prompt_type {
        PromptType::PageResearch => 8,
        _ => 8,
    };

    for _ in 0..8 {
        if estimate_payload_tokens(input, session.as_ref()) <= limit_tokens {
            break;
        }
        shrink_json_value(input, max_string_chars, max_array_items);
        if let Some(state) = session.as_mut() {
            *state = trim_session_state(state.clone(), config);
            truncate_string(&mut state.session_summary, max_string_chars);
            for turn in &mut state.recent_turns {
                truncate_string(&mut turn.content, max_string_chars.min(600));
            }
            for artifact in &mut state.tool_artifact_refs {
                truncate_string(&mut artifact.summary, max_string_chars.min(600));
            }
            if state.recent_turns.len() > max_array_items {
                state.recent_turns = state
                    .recent_turns
                    .iter()
                    .rev()
                    .take(max_array_items)
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
            }
            if state.tool_artifact_refs.len() > max_array_items {
                state.tool_artifact_refs = state
                    .tool_artifact_refs
                    .iter()
                    .rev()
                    .take(max_array_items)
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
            }
        }
        max_string_chars = (max_string_chars / 2).max(160);
        max_array_items = max_array_items.saturating_sub(1).max(2);
    }
}

fn trim_named_array_field(
    object: &mut serde_json::Map<String, Value>,
    field_names: &[&str],
    keep: usize,
) {
    for field_name in field_names {
        let Some(array) = object.get_mut(*field_name).and_then(Value::as_array_mut) else {
            continue;
        };
        if array.len() > keep {
            array.truncate(keep);
        }
    }
}

fn shrink_json_value(value: &mut Value, max_string_chars: usize, max_array_items: usize) {
    match value {
        Value::String(text) => truncate_string(text, max_string_chars),
        Value::Array(items) => {
            for item in items.iter_mut() {
                shrink_json_value(item, max_string_chars, max_array_items);
            }
            if items.len() > max_array_items {
                items.truncate(max_array_items);
            }
        }
        Value::Object(object) => {
            for field in object.values_mut() {
                shrink_json_value(field, max_string_chars, max_array_items);
            }
        }
        _ => {}
    }
}

fn truncate_string(value: &mut String, max_chars: usize) {
    if value.chars().count() <= max_chars {
        return;
    }
    let mut truncated = value.chars().take(max_chars).collect::<String>();
    truncated.push_str("...");
    *value = truncated;
}

fn dependency_assist_key(input: &DependencyAssistInput) -> String {
    format!(
        "{}|{}|{}",
        input.relation_type, input.source_path, input.target_path
    )
}

fn parse_file_purpose(value: &str) -> Option<FilePurpose> {
    let normalized = value
        .trim()
        .trim_matches(|ch: char| {
            matches!(ch, '"' | '\'' | '`' | '.' | ',' | '。' | '，' | ':' | '：')
        })
        .to_ascii_lowercase();

    match normalized.as_str() {
        "entry" => Some(FilePurpose::Entry),
        "router" => Some(FilePurpose::Router),
        "controller" => Some(FilePurpose::Controller),
        "handler" => Some(FilePurpose::Handler),
        "service" => Some(FilePurpose::Service),
        "model" => Some(FilePurpose::Model),
        "repository" => Some(FilePurpose::Repository),
        "domain" => Some(FilePurpose::Domain),
        "agent" => Some(FilePurpose::Agent),
        "library" => Some(FilePurpose::Library),
        "middleware" => Some(FilePurpose::Middleware),
        "plugin" => Some(FilePurpose::Plugin),
        "utility" => Some(FilePurpose::Utility),
        "helper" => Some(FilePurpose::Helper),
        "constant" => Some(FilePurpose::Constant),
        "type" => Some(FilePurpose::Type),
        "page" => Some(FilePurpose::Page),
        "component" => Some(FilePurpose::Component),
        "widget" => Some(FilePurpose::Widget),
        "layout" => Some(FilePurpose::Layout),
        "config" => Some(FilePurpose::Config),
        "migration" => Some(FilePurpose::Migration),
        "test" => Some(FilePurpose::Test),
        "docs" => Some(FilePurpose::Docs),
        _ => parse_file_purpose_from_description(value, &normalized),
    }
}

fn file_purpose_to_str(value: FilePurpose) -> &'static str {
    match value {
        FilePurpose::Entry => "entry",
        FilePurpose::Router => "router",
        FilePurpose::Controller => "controller",
        FilePurpose::Handler => "handler",
        FilePurpose::Service => "service",
        FilePurpose::Model => "model",
        FilePurpose::Repository => "repository",
        FilePurpose::Domain => "domain",
        FilePurpose::Agent => "agent",
        FilePurpose::Library => "library",
        FilePurpose::Middleware => "middleware",
        FilePurpose::Plugin => "plugin",
        FilePurpose::Utility => "utility",
        FilePurpose::Helper => "helper",
        FilePurpose::Constant => "constant",
        FilePurpose::Type => "type",
        FilePurpose::Page => "page",
        FilePurpose::Component => "component",
        FilePurpose::Widget => "widget",
        FilePurpose::Layout => "layout",
        FilePurpose::Config => "config",
        FilePurpose::Migration => "migration",
        FilePurpose::Test => "test",
        FilePurpose::Docs => "docs",
    }
}

fn file_purpose_response_schema(is_batch: bool) -> Value {
    if !is_batch {
        return json!({
            "type": "object",
            "required": ["purpose"],
            "properties": {
                "purpose": {
                    "type": "string",
                    "enum": FILE_PURPOSE_ALLOWED_VALUES
                }
            }
        });
    }

    json!({
        "type": "object",
        "required": ["items"],
        "properties": {
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["path", "purpose"],
                    "properties": {
                        "path": {"type": "string"},
                        "purpose": {
                            "type": "string",
                            "enum": FILE_PURPOSE_ALLOWED_VALUES
                        }
                    }
                }
            }
        }
    })
}

fn parse_file_purpose_from_description(raw: &str, normalized: &str) -> Option<FilePurpose> {
    if contains_any(
        raw,
        normalized,
        &["utility", "util", "工具函数", "工具模块", "通用工具"],
    ) {
        return Some(FilePurpose::Utility);
    }
    if contains_any(
        raw,
        normalized,
        &["helper", "helpers", "辅助函数", "辅助模块", "帮助函数"],
    ) {
        return Some(FilePurpose::Helper);
    }
    if contains_any(raw, normalized, &["constant", "const", "常量"]) {
        return Some(FilePurpose::Constant);
    }
    if contains_any(
        raw,
        normalized,
        &[
            "type",
            "types",
            "interface",
            "interfaces",
            "类型定义",
            "接口定义",
            "类型声明",
        ],
    ) {
        return Some(FilePurpose::Type);
    }
    if contains_any(raw, normalized, &["middleware", "middlewares", "中间件"]) {
        return Some(FilePurpose::Middleware);
    }
    if contains_any(raw, normalized, &["plugin", "plugins", "插件"]) {
        return Some(FilePurpose::Plugin);
    }
    if contains_any(raw, normalized, &["router", "routes", "route", "路由"]) {
        return Some(FilePurpose::Router);
    }
    if contains_any(raw, normalized, &["controller", "controllers", "控制器"]) {
        return Some(FilePurpose::Controller);
    }
    if contains_any(
        raw,
        normalized,
        &["handler", "handlers", "处理器", "请求处理"],
    ) {
        return Some(FilePurpose::Handler);
    }
    if contains_any(
        raw,
        normalized,
        &["service", "services", "业务服务", "服务层"],
    ) {
        return Some(FilePurpose::Service);
    }
    if contains_any(
        raw,
        normalized,
        &["repository", "repositories", "repo", "dao", "存储访问"],
    ) {
        return Some(FilePurpose::Repository);
    }
    if contains_any(raw, normalized, &["domain", "domains", "领域"]) {
        return Some(FilePurpose::Domain);
    }
    if contains_any(
        raw,
        normalized,
        &["model", "models", "entity", "schema", "数据模型", "实体"],
    ) {
        return Some(FilePurpose::Model);
    }
    if contains_any(raw, normalized, &["agent", "agents", "智能体"]) {
        return Some(FilePurpose::Agent);
    }
    if contains_any(raw, normalized, &["library", "lib", "共享库", "公共库"]) {
        return Some(FilePurpose::Library);
    }
    if contains_any(raw, normalized, &["layout", "layouts", "布局"]) {
        return Some(FilePurpose::Layout);
    }
    if contains_any(raw, normalized, &["widget", "widgets", "控件", "部件"]) {
        return Some(FilePurpose::Widget);
    }
    if contains_any(raw, normalized, &["component", "components", "组件"]) {
        return Some(FilePurpose::Component);
    }
    if contains_any(raw, normalized, &["page", "pages", "页面"]) {
        return Some(FilePurpose::Page);
    }
    if contains_any(raw, normalized, &["config", "configs", "配置文件", "配置"]) {
        return Some(FilePurpose::Config);
    }
    if contains_any(raw, normalized, &["migration", "migrations", "迁移"]) {
        return Some(FilePurpose::Migration);
    }
    if contains_any(raw, normalized, &["test", "tests", "测试"]) {
        return Some(FilePurpose::Test);
    }
    if contains_any(raw, normalized, &["docs", "doc", "readme", "文档", "说明"]) {
        return Some(FilePurpose::Docs);
    }
    if contains_any(raw, normalized, &["entry", "main", "bootstrap", "入口"]) {
        return Some(FilePurpose::Entry);
    }

    None
}

fn contains_any(raw: &str, normalized: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| {
        if needle.is_ascii() {
            normalized.contains(needle)
        } else {
            raw.contains(needle)
        }
    })
}

fn parse_file_purpose_batch_output(output: Value) -> BTreeMap<String, FilePurpose> {
    let mut resolved = BTreeMap::new();

    let mut visit_item = |path: &str, purpose: &str| {
        if let Some(purpose) = parse_file_purpose(purpose) {
            resolved.insert(path.to_string(), purpose);
        }
    };

    if let Some(items) = output.get("items").and_then(Value::as_array) {
        for item in items {
            let Some(path) = item.get("path").and_then(Value::as_str) else {
                continue;
            };
            let Some(purpose) = item.get("purpose").and_then(Value::as_str) else {
                continue;
            };
            visit_item(path, purpose);
        }
        return resolved;
    }

    if let Some(items) = output.as_array() {
        for item in items {
            let Some(path) = item.get("path").and_then(Value::as_str) else {
                continue;
            };
            let Some(purpose) = item.get("purpose").and_then(Value::as_str) else {
                continue;
            };
            visit_item(path, purpose);
        }
        return resolved;
    }

    if let Some(map) = output.as_object() {
        for (path, value) in map {
            let Some(purpose) = value.as_str() else {
                continue;
            };
            visit_item(path, purpose);
        }
    }

    resolved
}

fn sanitize_module_kind(value: &str) -> Option<String> {
    let normalized = value.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "library"
            | "cli-tool"
            | "backend-service"
            | "frontend-app"
            | "module"
            | "module-group"
            | "workspace-member"
            | "infrastructure"
            | "application"
    )
    .then_some(normalized)
}

fn normalize_sentence(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn normalize_multiline(value: &str) -> String {
    value
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn dedupe_non_empty(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();

    for value in values {
        let normalized = normalize_multiline(&value);
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        deduped.push(normalized);
    }

    deduped
}

fn select_runtime_service<'svc>(
    config: &LlmConfig,
    agent_service: Option<&'svc mut dyn LlmService>,
) -> Option<RuntimeLlmService<'svc>> {
    if !config.enabled {
        return None;
    }

    if config.provider_configured() {
        match ProviderApiLlmService::from_config(config) {
            Ok(service) => return Some(RuntimeLlmService::Provider(service)),
            Err(error) => {
                eprintln!("[warn] failed to initialize provider LLM service: {error}");
            }
        }
    }

    agent_service.map(RuntimeLlmService::Agent)
}

fn reserved_core_page_call_budget(max_page_calls: usize) -> usize {
    match max_page_calls {
        0 => 0,
        1..=2 => 1,
        3..=8 => 2,
        _ => 3,
    }
}

fn is_core_page_type(page_type: Option<&str>) -> bool {
    matches!(page_type, Some("overview" | "architecture"))
}

fn default_function_type() -> String {
    "function".to_string()
}

fn build_provider_response_format(response_schema: &Value) -> Value {
    json!({
        "type": "json_schema",
        "json_schema": {
            "name": "repo_wiki_response",
            "schema": normalize_provider_json_schema(response_schema),
        }
    })
}

fn normalize_provider_response_format(response_format: &Value) -> Value {
    let mut normalized = response_format.clone();
    if let Some(schema) = normalized
        .get_mut("json_schema")
        .and_then(Value::as_object_mut)
        .and_then(|json_schema| json_schema.get_mut("schema"))
    {
        *schema = normalize_provider_json_schema(schema);
    }
    normalized
}

fn normalize_provider_responses_text_format(response_format: &Value) -> Value {
    let normalized = normalize_provider_response_format(response_format);
    let Some(mut object) = normalized.as_object().cloned() else {
        return normalized;
    };
    if let Some(Value::Object(json_schema)) = object.remove("json_schema") {
        object.extend(json_schema);
    }
    if let Some(schema) = object.get_mut("schema") {
        *schema = normalize_provider_json_schema(schema);
    }
    if object.get("type").and_then(Value::as_str) == Some("json_schema")
        && !object.contains_key("name")
    {
        object.insert("name".to_string(), json!("repo_wiki_response"));
    }
    Value::Object(object)
}

fn normalize_provider_tool_definition(tool_definition: &Value) -> Value {
    let mut normalized = tool_definition.clone();
    if let Some(parameters) = normalized
        .get_mut("function")
        .and_then(Value::as_object_mut)
        .and_then(|function| function.get_mut("parameters"))
    {
        *parameters = normalize_provider_json_schema(parameters);
    }
    normalized
}

fn normalize_provider_response_tool_definition(tool_definition: &Value) -> Value {
    let normalized = normalize_provider_tool_definition(tool_definition);
    let Some(function) = normalized.get("function").and_then(Value::as_object) else {
        return normalized;
    };

    let mut response_tool = Map::new();
    response_tool.insert("type".to_string(), json!("function"));
    if let Some(name) = function.get("name") {
        response_tool.insert("name".to_string(), name.clone());
    }
    if let Some(description) = function.get("description") {
        response_tool.insert("description".to_string(), description.clone());
    }
    if let Some(parameters) = function.get("parameters") {
        response_tool.insert("parameters".to_string(), parameters.clone());
    }
    Value::Object(response_tool)
}

fn normalize_provider_response_tool_choice(tool_choice: &Value) -> Value {
    let Some(object) = tool_choice.as_object() else {
        return tool_choice.clone();
    };
    let Some(function) = object.get("function").and_then(Value::as_object) else {
        return tool_choice.clone();
    };

    let mut normalized = Map::new();
    normalized.insert("type".to_string(), json!("function"));
    if let Some(name) = function.get("name") {
        normalized.insert("name".to_string(), name.clone());
    }
    Value::Object(normalized)
}

fn normalize_provider_json_schema(schema: &Value) -> Value {
    match schema {
        Value::Object(object) => {
            let mut normalized = Map::new();
            for (key, value) in object {
                normalized.insert(key.clone(), normalize_provider_json_schema(value));
            }
            if schema_declares_object(object) {
                let existing_required = normalized
                    .get("required")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                if let Some(properties) = normalized
                    .get_mut("properties")
                    .and_then(Value::as_object_mut)
                {
                    let required_keys = properties.keys().cloned().collect::<Vec<_>>();
                    for key in &required_keys {
                        if existing_required.contains(key) {
                            continue;
                        }
                        if let Some(property_schema) = properties.get_mut(key) {
                            *property_schema = make_provider_schema_nullable(property_schema);
                        }
                    }
                    normalized.insert(
                        "required".to_string(),
                        Value::Array(
                            required_keys
                                .into_iter()
                                .map(Value::String)
                                .collect::<Vec<_>>(),
                        ),
                    );
                }
                if !normalized.contains_key("additionalProperties") {
                    normalized.insert("additionalProperties".to_string(), Value::Bool(false));
                }
            }
            Value::Object(normalized)
        }
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(normalize_provider_json_schema)
                .collect::<Vec<_>>(),
        ),
        _ => schema.clone(),
    }
}

fn schema_declares_object(schema: &Map<String, Value>) -> bool {
    if schema.contains_key("properties") {
        return true;
    }
    match schema.get("type") {
        Some(Value::String(kind)) => kind == "object",
        Some(Value::Array(kinds)) => kinds
            .iter()
            .any(|kind| kind.as_str().is_some_and(|kind| kind == "object")),
        _ => false,
    }
}

fn make_provider_schema_nullable(schema: &Value) -> Value {
    let mut normalized = schema.clone();
    if let Some(items) = normalized.get_mut("enum").and_then(Value::as_array_mut) {
        if !items.iter().any(Value::is_null) {
            items.push(Value::Null);
        }
    }
    if let Some(type_value) = normalized.get_mut("type") {
        match type_value {
            Value::String(kind) if kind != "null" => {
                let original = kind.clone();
                *type_value = Value::Array(vec![
                    Value::String(original),
                    Value::String("null".to_string()),
                ]);
            }
            Value::Array(kinds) => {
                if !kinds
                    .iter()
                    .any(|kind| kind.as_str().is_some_and(|kind| kind == "null"))
                {
                    kinds.push(Value::String("null".to_string()));
                }
            }
            _ => {}
        }
    }
    normalized
}

fn build_provider_user_message(request: &LlmPromptRequest) -> String {
    serde_json::to_string_pretty(&json!({
        "prompt_type": request.prompt_type,
        "prompt_version": request.prompt_version,
        "input_hash": request.input_hash,
        "instruction": request.instruction,
        "response_schema": request.response_schema,
        "input": request.input,
        "available_tools": request.tools,
        "session": request.session,
    }))
    .map(|payload| format!("{payload}\n请只返回符合 response_schema 的 JSON 对象。"))
    .unwrap_or_else(|_| request.instruction.clone())
}

fn build_provider_response_input(messages: &[Value]) -> Value {
    Value::Array(
        messages
            .iter()
            .flat_map(build_provider_response_input_items)
            .collect(),
    )
}

fn build_provider_response_input_items(message: &Value) -> Vec<Value> {
    let role = message
        .get("role")
        .and_then(Value::as_str)
        .unwrap_or("user")
        .to_string();
    let mut items = Vec::new();
    if let Some(tool_calls) = message.get("tool_calls").and_then(Value::as_array) {
        for tool_call in tool_calls {
            let call_id = tool_call
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let function = tool_call.get("function").and_then(Value::as_object);
            let name = function
                .and_then(|function| function.get("name"))
                .and_then(Value::as_str)
                .unwrap_or_default();
            let arguments = function
                .and_then(|function| function.get("arguments"))
                .and_then(Value::as_str)
                .unwrap_or("{}");
            if !name.is_empty() {
                items.push(json!({
                    "type": "function_call",
                    "call_id": call_id,
                    "name": name,
                    "arguments": arguments,
                }));
            }
        }
    }
    if role == "tool" {
        let call_id = message
            .get("tool_call_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let output = message
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or("{}");
        if !call_id.is_empty() {
            items.push(json!({
                "type": "function_call_output",
                "call_id": call_id,
                "output": output,
            }));
        }
        return items;
    }
    if let Some(content) = extract_provider_content_from_message(message) {
        if !content.trim().is_empty() {
            items.push(json!({
                "role": role,
                "content": [
                    {
                        "type": "input_text",
                        "text": content,
                    }
                ],
            }));
        }
    }
    items
}
fn parse_provider_chat_response(
    request: &LlmPromptRequest,
    fallback_model: &str,
    response: &Value,
    request_body: &Value,
) -> io::Result<ProviderChatResponse> {
    let choice = response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .ok_or_else(|| io::Error::other("provider response missing choices[0]"))?;
    let message = choice
        .get("message")
        .ok_or_else(|| io::Error::other("provider response missing message"))?;
    let content = extract_provider_content_from_message(message);
    let tool_calls = extract_provider_tool_calls(message)?;
    let usage = extract_provider_usage(response, request_body, content.as_deref());
    let model = response
        .get("model")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| Some(fallback_model.to_string()));
    debug_trace::record_json(
        "llm_provider_chat_completion",
        &json!({
            "request_id": request.request_id,
            "tool_calls": tool_calls,
            "content": content,
            "usage": usage,
        }),
    );

    Ok(ProviderChatResponse {
        model,
        content,
        tool_calls,
        usage,
    })
}

fn parse_provider_responses_response(
    request: &LlmPromptRequest,
    fallback_model: &str,
    response: &Value,
    request_body: &Value,
) -> io::Result<ProviderChatResponse> {
    let content = extract_provider_content_from_responses_output(response);
    let tool_calls = extract_provider_tool_calls_from_responses_output(response)?;
    let usage = extract_provider_usage(response, request_body, content.as_deref());
    let model = response
        .get("model")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| Some(fallback_model.to_string()));
    debug_trace::record_json(
        "llm_provider_responses_completion",
        &json!({
            "request_id": request.request_id,
            "tool_calls": tool_calls,
            "content": content,
            "usage": usage,
        }),
    );

    Ok(ProviderChatResponse {
        model,
        content,
        tool_calls,
        usage,
    })
}

fn extract_provider_content_from_message(message: &Value) -> Option<String> {
    if let Some(content) = message.get("content").and_then(Value::as_str) {
        return Some(content.to_string());
    }
    if let Some(parts) = message.get("content").and_then(Value::as_array) {
        let content = parts
            .iter()
            .filter_map(|part| part.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("\n");
        if !content.trim().is_empty() {
            return Some(content);
        }
    }
    None
}

fn extract_provider_tool_calls(message: &Value) -> io::Result<Vec<ProviderToolCall>> {
    let Some(raw_tool_calls) = message.get("tool_calls") else {
        return Ok(Vec::new());
    };
    if raw_tool_calls.is_null() {
        return Ok(Vec::new());
    }
    serde_json::from_value::<Vec<ProviderToolCall>>(raw_tool_calls.clone())
        .map_err(|error| io::Error::other(error.to_string()))
}

fn extract_provider_content_from_responses_output(response: &Value) -> Option<String> {
    if let Some(output_text) = response.get("output_text").and_then(Value::as_str) {
        if !output_text.trim().is_empty() {
            return Some(output_text.to_string());
        }
    }
    let output = response.get("output").and_then(Value::as_array)?;
    let content = output
        .iter()
        .filter(|item| item.get("type").and_then(Value::as_str) == Some("message"))
        .filter_map(|item| item.get("content").and_then(Value::as_array))
        .flatten()
        .filter_map(|part| {
            part.get("text").and_then(Value::as_str).or_else(|| {
                part.get("content")
                    .and_then(Value::as_array)
                    .and_then(|nested| nested.first())
                    .and_then(|entry| entry.get("text"))
                    .and_then(Value::as_str)
            })
        })
        .collect::<Vec<_>>()
        .join("\n");
    (!content.trim().is_empty()).then_some(content)
}

fn extract_provider_tool_calls_from_responses_output(
    response: &Value,
) -> io::Result<Vec<ProviderToolCall>> {
    let Some(output) = response.get("output").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut tool_calls = Vec::new();
    for item in output {
        if item.get("type").and_then(Value::as_str) != Some("function_call") {
            continue;
        }
        let name = item
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if name.is_empty() {
            continue;
        }
        let id = item
            .get("call_id")
            .or_else(|| item.get("id"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let arguments = item
            .get("arguments")
            .map(|arguments| match arguments {
                Value::String(value) => value.clone(),
                other => serde_json::to_string(other).unwrap_or_else(|_| "{}".to_string()),
            })
            .unwrap_or_else(|| "{}".to_string());
        tool_calls.push(ProviderToolCall {
            id,
            r#type: default_function_type(),
            function: ProviderToolFunction { name, arguments },
        });
    }
    Ok(tool_calls)
}

fn parse_provider_json_output(content: &str) -> io::Result<Value> {
    let trimmed = content.trim();
    let stripped = trimmed
        .strip_prefix("```json")
        .map(str::trim)
        .and_then(|value| value.strip_suffix("```").map(str::trim))
        .or_else(|| {
            trimmed
                .strip_prefix("```")
                .map(str::trim)
                .and_then(|value| value.strip_suffix("```").map(str::trim))
        })
        .unwrap_or(trimmed);

    serde_json::from_str(stripped).map_err(|error| io::Error::other(error.to_string()))
}

fn parse_page_research_output(value: Value, _context: &PageContext) -> Option<PageResearchResult> {
    let object = value.as_object()?;
    let required = [
        "summary",
        "page_positioning",
        "section_plan",
        "evidence_rollup",
        "diagram_rollup",
        "open_questions",
    ];
    if !required.iter().all(|key| object.contains_key(*key)) {
        return None;
    }
    serde_json::from_value::<PageResearchResult>(Value::Object(object.clone())).ok()
}

fn build_page_research_instruction(input: &PageResearchInput) -> String {
    let allowed_sections = if input.allowed_section_slots.is_empty() {
        section_titles_for_page_type(&input.page_type)
            .into_iter()
            .map(|title| PageResearchSectionSlot {
                section_key: section_key_for_title(&input.page_type, title),
                section_title: title.to_string(),
            })
            .collect::<Vec<_>>()
    } else {
        input.allowed_section_slots.clone()
    };
    let allowed_sections = allowed_sections
        .into_iter()
        .map(|slot| format!("{} ({})", slot.section_key, slot.section_title))
        .collect::<Vec<_>>()
        .join("、");
    format!(
        concat!(
            "页面类型：{page_type}\n",
            "页面标题：{title}\n",
            "页面作用域：{scope}\n",
            "evidence groups 数量：{evidence_groups}\n",
            "diagram inputs 数量：{diagram_inputs}\n",
            "允许的 section 槽位：{allowed_sections}\n",
            "输出要求：\n",
            "1. 最终结果必须严格符合 response_schema，对应 `PageResearchResult`。\n",
            "2. `page_positioning` 用 1 到 2 句说明当前页面在整体知识树中的定位，不得输出 Markdown 页面。\n",
            "3. `summary` 只能写 1 段高密度摘要，不得输出 Markdown 页面。\n",
            "4. `section_plan` 必须只使用允许的 section 槽位；每节都应给出 `section_key / section_title / section_summary`，并尽量补充 `evidence_refs / diagram_refs / child_refs`。\n",
            "5. `evidence_rollup` 只能引用当前输入中已存在的 evidence group / source path / line span。\n",
            "6. `diagram_rollup` 只能引用 deterministic 已存在的 diagram inputs。\n",
            "7. 若信息不足，可提出 `open_questions`，但不得凭空捏造事实。\n",
            "8. 若工具调用有帮助，可以先调用工具，再返回最终结构化结果。\n",
            "9. 不要返回自由新章节；重点是决定受控 section 槽位的顺序、重点和支撑材料。\n",
            "10. 页面拆分与章节骨架以 facts/planner 预先给出的 contract 为准；LLM 只能在现有 contract 内补充优先级、摘要与证据组织，不能自由发明新的页树或章节体系。"
        ),
        page_type = input.page_type,
        title = input.title,
        scope = input.scope,
        evidence_groups = input.evidence_groups.len(),
        diagram_inputs = input.diagram_inputs.len(),
        allowed_sections = allowed_sections,
    )
}

fn page_research_response_schema() -> Value {
    json!({
        "type": "object",
        "required": ["summary", "page_positioning", "section_plan", "evidence_rollup", "diagram_rollup", "open_questions"],
        "properties": {
            "summary": {"type": "string"},
            "page_positioning": {"type": "string"},
            "section_plan": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["section_key", "section_title", "section_summary", "evidence_refs", "diagram_refs", "child_refs"],
                    "properties": {
                        "section_key": {"type": "string"},
                        "section_title": {"type": "string"},
                        "section_summary": {"type": "string"},
                        "evidence_refs": {"type": "array", "items": {"type": "string"}},
                        "diagram_refs": {"type": "array", "items": {"type": "string"}},
                        "child_refs": {"type": "array", "items": {"type": "string"}}
                    }
                }
            },
            "evidence_rollup": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["group_key", "title", "items"],
                    "properties": {
                        "group_key": {"type": "string"},
                        "title": {"type": "string"},
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["path", "start_line", "end_line", "evidence_type", "section_refs", "note", "coarse_span"],
                                "properties": {
                                    "source_id": {"type": "string"},
                                    "path": {"type": "string"},
                                    "start_line": {"type": "integer"},
                                    "end_line": {"type": "integer"},
                                    "evidence_type": {"type": "string"},
                                    "section_refs": {"type": "array", "items": {"type": "string"}},
                                    "note": {"type": "string"},
                                    "coarse_span": {"type": "boolean"}
                                }
                            }
                        }
                    }
                }
            },
            "diagram_rollup": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["diagram_key", "diagram_type", "title", "summary"],
                    "properties": {
                        "diagram_key": {"type": "string"},
                        "diagram_type": {"type": "string"},
                        "title": {"type": "string"},
                        "summary": {"type": "string"}
                    }
                }
            },
            "open_questions": {"type": "array", "items": {"type": "string"}}
        }
    })
}

fn research_tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "type": "function",
            "function": {
                "name": "read_source_snippets",
                "description": "按 source_ids 读取受控源码片段",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "source_ids": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["source_ids"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "get_module_context",
                "description": "读取模块上下文摘要",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "module_ids": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["module_ids"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "get_symbol_neighbors",
                "description": "读取符号邻居；当前 provider-first 版本保守返回已知邻居或空结果",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "symbol_ids": {"type": "array", "items": {"type": "string"}},
                        "edge_types": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["symbol_ids"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "get_process_trace",
                "description": "读取流程 trace 摘要",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "process_id": {"type": "string"}
                    },
                    "required": ["process_id"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "get_page_children",
                "description": "读取页面的 child rollup",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "page_id": {"type": "string"}
                    },
                    "required": ["page_id"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "get_evidence_group",
                "description": "读取页面 evidence group",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "page_id": {"type": "string"},
                        "group_key": {"type": "string"}
                    },
                    "required": ["page_id", "group_key"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "search_topic_candidates",
                "description": "检索当前 scope 下的 topic candidates",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "scope": {"type": "string"},
                        "scope_id": {"type": "string"},
                        "query": {"type": "string"}
                    },
                    "required": ["scope", "scope_id", "query"]
                }
            }
        }),
    ]
}

fn build_emulated_tool_user_message(request: &LlmPromptRequest) -> String {
    serde_json::to_string_pretty(&json!({
        "prompt_type": request.prompt_type,
        "prompt_version": request.prompt_version,
        "input_hash": request.input_hash,
        "instruction": request.instruction,
        "response_schema": request.response_schema,
        "available_tools": request.tools,
        "input": request.input,
        "tool_protocol": {
            "assistant": {
                "type": "assistant",
                "content": null,
                "tool_calls": [
                    {
                        "id": "call_001",
                        "type": "function",
                        "function": {
                            "name": "read_source_snippets",
                            "arguments": "{\"source_ids\":[\"src:a\"]}"
                        }
                    }
                ]
            },
            "tool": {
                "type": "tool",
                "tool_call_id": "call_001",
                "name": "read_source_snippets",
                "content": "{\"snippets\":[]}"
            },
            "final": {
                "type": "final",
                "result": {
                    "summary": "...",
                    "page_positioning": "...",
                    "section_plan": [],
                    "evidence_rollup": [],
                    "diagram_rollup": [],
                    "open_questions": []
                }
            }
        }
    }))
    .map(|payload| format!("{payload}\n请优先输出 tool_calls 或 final 结构，不要输出 Markdown。"))
    .unwrap_or_else(|_| request.instruction.clone())
}

fn finalize_research_session(
    page_id: &str,
    previous: Option<&PageResearchSessionState>,
    result: &PageResearchResult,
    recent_turns: Vec<PageResearchTurn>,
    tool_artifact_refs: Vec<PageToolArtifactRef>,
) -> PageResearchSessionState {
    let session_id = previous
        .map(|state| state.session_id.clone())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| stable_id("research-session", page_id));
    PageResearchSessionState {
        session_id,
        session_summary: result.summary.clone(),
        recent_turns: recent_turns
            .into_iter()
            .rev()
            .take(8)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        tool_artifact_refs,
    }
}

fn execute_research_tool(
    tool_call: &ProviderToolCall,
    runtime: &PageResearchRuntimeContext<'_>,
) -> io::Result<(Value, Option<PageToolArtifactRef>)> {
    let args = parse_provider_json_output(&tool_call.function.arguments)?;
    let source_index = runtime
        .scan_report
        .files
        .iter()
        .map(|file| (file.id.clone(), file.path.clone()))
        .collect::<BTreeMap<_, _>>();
    let symbol_index = runtime
        .symbol_snapshot
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let snippet_pool = collect_research_snippets(runtime.page_context);
    match tool_call.function.name.as_str() {
        "read_source_snippets" => {
            let source_ids = args
                .get("source_ids")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<Vec<_>>();
            let snippets = source_ids
                .iter()
                .filter_map(|source_id| {
                    resolve_tool_snippet(
                        source_id,
                        &source_index,
                        &snippet_pool,
                        &symbol_index,
                        runtime.scan_report,
                    )
                })
                .map(|snippet| targeted_snippet_to_json(&snippet))
                .collect::<Vec<_>>();
            Ok((
                json!({ "snippets": snippets }),
                Some(PageToolArtifactRef {
                    tool_name: tool_call.function.name.clone(),
                    artifact_id: tool_call.id.clone(),
                    summary: format!("读取了 {} 个源码片段", snippets.len()),
                }),
            ))
        }
        "get_module_context" => {
            let module_ids = args
                .get("module_ids")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<Vec<_>>();
            let modules = module_ids
                .iter()
                .filter_map(|module_id| {
                    let context = runtime
                        .module_contexts
                        .iter()
                        .find(|context| context.module_id == *module_id)?;
                    let title = runtime
                        .module_tree
                        .module_by_id(module_id)
                        .map(|module| module.name.clone())
                        .unwrap_or_else(|| module_id.clone());
                    Some(json!({
                        "module_id": module_id,
                        "title": title,
                        "summary": context.role_hints.join("、"),
                        "key_sources": context.key_sources,
                        "graph_hotspots": context.graph_hotspots,
                        "communities": context.communities,
                        "dependencies": context.dependencies,
                        "dependents": context.dependents,
                        "child_page_ids": Vec::<String>::new(),
                    }))
                })
                .collect::<Vec<_>>();
            Ok((json!({ "modules": modules }), None))
        }
        "get_symbol_neighbors" => {
            let symbol_ids = args
                .get("symbol_ids")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<BTreeSet<_>>();
            let edge_types = args
                .get("edge_types")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<BTreeSet<_>>();
            let neighbors = runtime
                .resolved_graph
                .edges
                .iter()
                .filter(|edge| {
                    (symbol_ids.contains(&edge.source_id) || symbol_ids.contains(&edge.target_id))
                        && (edge_types.is_empty() || edge_types.contains(&edge.edge_type))
                })
                .take(24)
                .filter_map(|edge| {
                    let source = symbol_index.get(&edge.source_id)?;
                    let target = symbol_index.get(&edge.target_id)?;
                    Some(json!({
                        "edge_id": edge.edge_id,
                        "edge_type": edge.edge_type,
                        "confidence": edge.confidence,
                        "reason": edge.reason,
                        "source": symbol_to_json(source),
                        "target": symbol_to_json(target),
                    }))
                })
                .collect::<Vec<_>>();
            Ok((json!({ "neighbors": neighbors }), None))
        }
        "get_process_trace" => {
            let process_id = args
                .get("process_id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let process = runtime
                .graph_analysis
                .processes
                .iter()
                .find(|process| {
                    process.process_id == process_id || process.label.contains(process_id)
                })
                .or_else(|| runtime.graph_analysis.processes.first())
                .map(|process| {
                    let steps = runtime
                        .graph_analysis
                        .process_steps
                        .iter()
                        .filter(|step| step.process_id == process.process_id)
                        .take(12)
                        .filter_map(|step| {
                            let symbol = symbol_index.get(&step.symbol_id)?;
                            Some(json!({
                                "step_order": step.step_order,
                                "symbol": symbol_to_json(symbol),
                            }))
                        })
                        .collect::<Vec<_>>();
                    json!({
                        "process": {
                            "process_id": process.process_id,
                            "title": process.label,
                            "process_type": process.process_type,
                            "steps": steps,
                        }
                    })
                })
                .unwrap_or_else(|| json!({ "process": Value::Null }));
            Ok((process, None))
        }
        "get_page_children" => {
            let children = Vec::<Value>::new();
            Ok((json!({ "children": children }), None))
        }
        "get_evidence_group" => {
            let page_id = args
                .get("page_id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let group_key = args
                .get("group_key")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let group = if page_id == runtime.page.id {
                runtime
                    .page_context
                    .evidence_groups
                    .iter()
                    .find(|group| group.group_id == group_key)
                    .map(|group| {
                        json!({
                            "group": {
                                "group_key": group.group_id,
                                "title": group.title,
                                "summary": group.summary,
                                "items": group.items.iter().map(|item| {
                                    json!({
                                        "source_id": item.source_id,
                                        "path": item.path,
                                        "start_line": item.start_line,
                                        "end_line": item.end_line,
                                        "evidence_type": item.evidence_type,
                                        "section_refs": item.section_refs,
                                        "note": item.note,
                                        "coarse_span": item.coarse_span,
                                    })
                                }).collect::<Vec<_>>(),
                            }
                        })
                    })
                    .unwrap_or_else(|| json!({ "group": Value::Null }))
            } else {
                json!({ "group": Value::Null })
            };
            Ok((group, None))
        }
        "search_topic_candidates" => {
            let scope = args.get("scope").and_then(Value::as_str).unwrap_or("repo");
            let scope_id = args
                .get("scope_id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let query = args
                .get("query")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            let candidates = runtime
                .repo_context
                .root_topics
                .iter()
                .chain(runtime.repo_context.process_topics.iter())
                .chain(
                    runtime
                        .module_contexts
                        .iter()
                        .flat_map(|context| context.capability_topics.iter()),
                )
                .filter(|topic| {
                    (scope == "repo"
                        || topic
                            .module_ids
                            .iter()
                            .any(|module_id| module_id == scope_id))
                        && (query.is_empty()
                            || topic.title.to_ascii_lowercase().contains(&query)
                            || topic.summary.to_ascii_lowercase().contains(&query))
                })
                .take(8)
                .map(|topic| {
                    json!({
                        "topic_key": topic.topic_key,
                        "title": topic.title,
                        "reason": topic.summary,
                        "evidence_source_ids": topic.source_ids,
                    })
                })
                .collect::<Vec<_>>();
            Ok((json!({ "candidates": candidates }), None))
        }
        other => Err(io::Error::other(format!("unsupported tool call: {other}"))),
    }
}

fn collect_research_snippets(_context: &PageContext) -> Vec<TargetedSnippet> {
    Vec::new()
}

fn resolve_tool_snippet(
    source_id: &str,
    source_index: &BTreeMap<String, String>,
    snippet_pool: &[TargetedSnippet],
    symbol_index: &BTreeMap<String, &SymbolNode>,
    scan_report: &crate::repo::scanner::ScanReport,
) -> Option<TargetedSnippet> {
    if let Some(snippet) = snippet_pool
        .iter()
        .find(|snippet| snippet.source_id.as_deref() == Some(source_id))
    {
        return Some(snippet.clone());
    }

    let path = source_index.get(source_id)?;
    if let Some(symbol) = symbol_index
        .values()
        .find(|symbol| symbol.file_path == *path)
    {
        return read_tool_snippet(
            scan_report,
            path,
            symbol.start_line,
            symbol.end_line,
            "tool-symbol",
            vec![symbol.symbol_id.clone()],
            Some(source_id.to_string()),
        );
    }

    read_tool_snippet(
        scan_report,
        path,
        1,
        24,
        "tool-file-window",
        Vec::new(),
        Some(source_id.to_string()),
    )
}

fn read_tool_snippet(
    scan_report: &crate::repo::scanner::ScanReport,
    path: &str,
    start_line: usize,
    end_line: usize,
    snippet_kind: &str,
    symbol_ids: Vec<String>,
    source_id: Option<String>,
) -> Option<TargetedSnippet> {
    let absolute_path = Path::new(&scan_report.root).join(path);
    let content = fs::read_to_string(absolute_path).ok()?;
    let lines = content.lines().collect::<Vec<_>>();
    let start = start_line.max(1).min(lines.len().max(1));
    let end = end_line.max(start).min(lines.len().max(start));
    let bounded_end = (start + 23).min(end).max(start);
    let snippet_content = lines
        .iter()
        .skip(start.saturating_sub(1))
        .take(bounded_end.saturating_sub(start) + 1)
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    if snippet_content.is_empty() {
        return None;
    }

    Some(TargetedSnippet {
        snippet_id: stable_id(
            "snippet",
            &format!("{path}:{start}:{bounded_end}:{snippet_kind}"),
        ),
        source_id,
        path: path.to_string(),
        start_line: start,
        end_line: bounded_end,
        snippet_kind: snippet_kind.to_string(),
        score: 0,
        symbol_ids,
        content: snippet_content,
    })
}

fn targeted_snippet_to_json(snippet: &TargetedSnippet) -> Value {
    json!({
        "snippet_id": snippet.snippet_id,
        "source_id": snippet.source_id,
        "path": snippet.path,
        "start_line": snippet.start_line,
        "end_line": snippet.end_line,
        "snippet_kind": snippet.snippet_kind,
        "score": snippet.score,
        "symbol_ids": snippet.symbol_ids,
        "content": snippet.content,
    })
}

fn symbol_to_json(symbol: &SymbolNode) -> Value {
    json!({
        "symbol_id": symbol.symbol_id,
        "name": symbol.name,
        "label": symbol.label,
        "file_path": symbol.file_path,
        "start_line": symbol.start_line,
        "end_line": symbol.end_line,
        "is_exported": symbol.is_exported,
        "language": symbol.language,
    })
}

fn is_provider_tools_unsupported(error: &io::Error) -> bool {
    let message = error.to_string().to_ascii_lowercase();
    message.contains("tool")
        || message.contains("tool_calls")
        || message.contains("tool_choice")
        || message.contains("unsupported")
}
fn provider_request_uses_response_format(request_body: &Value) -> bool {
    request_body.get("response_format").is_some()
        || request_body
            .get("text")
            .and_then(Value::as_object)
            .is_some_and(|text| text.contains_key("format"))
}
fn extract_provider_usage(
    response: &Value,
    request_body: &Value,
    content: Option<&str>,
) -> Option<LlmUsage> {
    let usage = response.get("usage");
    let input_tokens = usage
        .and_then(|usage| {
            usage
                .get("input_tokens")
                .or_else(|| usage.get("prompt_tokens"))
                .and_then(Value::as_u64)
        })
        .map(|value| value as usize)
        .unwrap_or_else(|| estimate_tokens(&request_body.to_string()));
    let output_tokens = usage
        .and_then(|usage| {
            usage
                .get("output_tokens")
                .or_else(|| usage.get("completion_tokens"))
                .and_then(Value::as_u64)
        })
        .map(|value| value as usize)
        .unwrap_or_else(|| estimate_tokens(content.unwrap_or_default()));
    let total_tokens = usage
        .and_then(|usage| usage.get("total_tokens").and_then(Value::as_u64))
        .map(|value| value as usize)
        .unwrap_or(input_tokens + output_tokens);
    Some(LlmUsage {
        request_count: 1,
        input_tokens,
        output_tokens,
        total_tokens,
        source: if usage.is_some() {
            "provider_usage".to_string()
        } else {
            "local_estimate".to_string()
        },
    })
}

fn estimate_tokens(value: &str) -> usize {
    if value.trim().is_empty() {
        0
    } else {
        value.chars().count().div_ceil(4)
    }
}

fn sleep_before_retry(base_delay_ms: u64, attempt: usize) {
    let delay_ms = base_delay_ms.saturating_mul(attempt as u64);
    thread::sleep(Duration::from_millis(delay_ms));
}

fn record_provider_retry(
    request: &LlmPromptRequest,
    stage: &str,
    attempt: usize,
    max_attempts: usize,
    reason: &str,
    response: Option<&Value>,
) {
    debug_trace::record_json(
        "llm_provider_retry",
        &json!({
            "request_id": request.request_id,
            "stage": stage,
            "attempt": attempt,
            "next_attempt": attempt + 1,
            "max_attempts": max_attempts,
            "reason": reason,
            "response": response.cloned(),
        }),
    );
}

fn is_retryable_provider_transport_error(error: &reqwest::Error) -> bool {
    error.is_timeout()
        || error.is_connect()
        || error.is_request()
        || error.is_body()
        || error.is_decode()
}

fn is_retryable_provider_status(status: u16) -> bool {
    matches!(status, 408 | 429) || (500..=599).contains(&status)
}

fn is_response_format_transport_error(error: &io::Error) -> bool {
    let message = error.to_string().to_ascii_lowercase();
    message.contains("response_format")
        || message.contains("json_schema")
        || message.contains("text.format")
        || message.contains("text format")
        || message.contains("unsupported")
}

#[cfg(test)]
mod tests {
    use super::reserved_core_page_call_budget;
    use crate::domain::steering::LlmConfig;

    #[test]
    fn higher_default_budget_scales_page_research_slots() {
        assert_eq!(LlmConfig::default().max_research_calls, 256);
        assert_eq!(reserved_core_page_call_budget(256), 3);
    }
}
