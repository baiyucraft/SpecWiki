//! `llm` 模块负责承载可选的 LLM 辅助层。
//! 它只消费已经稳定的事实输入，负责 prompt 组装、缓存、预算控制和输出校验，
//! 不直接接触文件系统扫描、模块树持久化或 Agent 宿主实现。

use std::collections::{BTreeMap, BTreeSet};
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
use serde_json::{json, Value};

use crate::debug_trace;
use crate::domain::context::{PageContext, PageDiagramInput, PageEvidenceGroup};
use crate::domain::stable_id::stable_id;
use crate::domain::steering::{LlmConfig, LlmProviderConfig};
use crate::generation::planner::PlannedPage;
use crate::generation::sections::section_titles_for_page_type;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::scanner::FilePurpose;
use crate::storage::sqlite_store::{read_llm_cache, write_llm_cache, LlmCacheEntry};

const FILE_PURPOSE_PROMPT_VERSION: &str = "file-purpose/v1";
const TOP_LEVEL_PROMOTION_PROMPT_VERSION: &str = "top-level-promotion/v1";
const MODULE_KIND_PROMPT_VERSION: &str = "module-kind/v1";
const DEPENDENCY_PROMPT_VERSION: &str = "dependency-edge/v1";
const PAGE_ENRICHMENT_PROMPT_VERSION: &str = "page-enrichment/v2";
const LLM_REQUEST_PROTOCOL: &str = "ndjson_session_v1";
const FILE_PURPOSE_BATCH_SIZE: usize = 8;
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
        self.protocol == LLM_REQUEST_PROTOCOL
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
    PageEnrichment,
}

impl PromptType {
    /// 返回稳定 prompt 类型名，供缓存键和协议层复用。
    pub fn as_str(&self) -> &'static str {
        match self {
            PromptType::FilePurpose => "file_purpose",
            PromptType::TopLevelPromotion => "top_level_promotion",
            PromptType::ModuleKind => "module_kind",
            PromptType::DependencyEdge => "dependency_edge",
            PromptType::PageEnrichment => "page_enrichment",
        }
    }

    /// 返回当前 prompt 的版本号。
    pub fn version(&self) -> &'static str {
        match self {
            PromptType::FilePurpose => FILE_PURPOSE_PROMPT_VERSION,
            PromptType::TopLevelPromotion => TOP_LEVEL_PROMPT_VERSION_FIXTURE,
            PromptType::ModuleKind => MODULE_KIND_PROMPT_VERSION,
            PromptType::DependencyEdge => DEPENDENCY_PROMPT_VERSION,
            PromptType::PageEnrichment => PAGE_ENRICHMENT_PROMPT_VERSION,
        }
    }
}

const TOP_LEVEL_PROMPT_VERSION_FIXTURE: &str = TOP_LEVEL_PROMOTION_PROMPT_VERSION;

/// `LlmPromptRequest` 是 core -> Agent 的结构化请求。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmPromptRequest {
    /// 单次请求的稳定 ID。
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
}

/// `LlmCompletion` 是 Agent -> core 的结构化响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmCompletion {
    /// 模型输出的 JSON 结果。
    pub output: Value,
    /// 实际使用的模型标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// `LlmService` 抽象 transport 之外的真实 LLM 调用通道。
pub trait LlmService {
    /// 发起一次结构化 LLM 请求。
    ///
    /// # 参数
    /// - `request`：当前 prompt 的完整协议对象。
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

impl ProviderApiLlmService {
    /// 基于 steering 配置构造 provider 直连 service。
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
}

impl LlmService for ProviderApiLlmService {
    fn request(&mut self, request: &LlmPromptRequest) -> io::Result<LlmCompletion> {
        let endpoint = self.provider.endpoint_url().ok_or_else(|| {
            io::Error::other("provider direct call requires llm.providers.<provider>.api_base")
        })?;
        let model = self.request_model()?;
        let request_body = json!({
            "model": model,
            "temperature": 0,
            "messages": [
                {
                    "role": "system",
                    "content": format!(
                        "{}\n你必须只返回一个 JSON 对象，不要使用 Markdown 代码块。",
                        request.system
                    ),
                },
                {
                    "role": "user",
                    "content": build_provider_user_message(request),
                }
            ]
        });
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
        let mut http_request = self
            .client
            .post(endpoint)
            .header(CONTENT_TYPE, "application/json");
        if let Some(api_key) = self.provider.resolved_api_key() {
            http_request = http_request.header(AUTHORIZATION, format!("Bearer {api_key}"));
        }

        let response = http_request
            .json(&request_body)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|error| {
                debug_trace::record_json(
                    "llm_provider_error",
                    &json!({
                        "request_id": request.request_id,
                        "stage": "http",
                        "error": error.to_string(),
                    }),
                );
                io::Error::other(error.to_string())
            })?;

        let response_json = response
            .json::<Value>()
            .map_err(|error| {
                debug_trace::record_json(
                    "llm_provider_error",
                    &json!({
                        "request_id": request.request_id,
                        "stage": "decode_response",
                        "error": error.to_string(),
                    }),
                );
                io::Error::other(error.to_string())
            })?;
        debug_trace::record_json(
            "llm_provider_response",
            &json!({
                "request_id": request.request_id,
                "response": response_json.clone(),
            }),
        );
        let response_model = response_json
            .get("model")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| Some(model.to_string()));
        let content = extract_provider_content(&response_json).inspect_err(|error| {
            debug_trace::record_json(
                "llm_provider_error",
                &json!({
                    "request_id": request.request_id,
                    "stage": "extract_content",
                    "error": error.to_string(),
                }),
            );
        })?;
        let output = parse_provider_json_output(&content).inspect_err(|error| {
            debug_trace::record_json(
                "llm_provider_error",
                &json!({
                    "request_id": request.request_id,
                    "stage": "parse_output",
                    "error": error.to_string(),
                    "content": content,
                }),
            );
        })?;
        debug_trace::record_json(
            "llm_provider_completion",
            &json!({
                "request_id": request.request_id,
                "completion": {
                    "model": response_model.clone(),
                    "output": output.clone(),
                },
            }),
        );

        Ok(LlmCompletion {
            output,
            model: response_model,
        })
    }
}

/// 文件角色辅助判断输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePurposeAssistInput {
    /// 当前文件的仓库内相对路径。
    pub path: String,
    /// scanner 已经判定出的粗粒度 kind。
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

/// 页面增强输入。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageEnrichmentInput {
    /// 页面稳定 ID。
    pub page_id: String,
    /// 页面类型。
    pub page_type: String,
    /// 页面标题。
    pub title: String,
    /// 页面作用域。
    pub scope: String,
    /// 当前页面允许覆盖的 section 标题集合。
    pub section_titles: Vec<String>,
    /// 必须保真的事实输入。
    pub facts: Vec<String>,
    /// 可用于组织正文的补充输入。
    pub summary_inputs: Vec<String>,
    /// steering 传入的页面提示。
    pub hints: Vec<String>,
    /// 来自子页面的摘要。
    pub child_summaries: Vec<String>,
    /// 当前页面的稳定 evidence groups。
    #[serde(default)]
    pub evidence_groups: Vec<PageEvidenceGroup>,
    /// 当前页面的 deterministic 图输入。
    #[serde(default)]
    pub diagram_inputs: Vec<PageDiagramInput>,
    /// 当前页面是否允许产出 Mermaid 图。
    pub allow_mermaid: bool,
}

impl PageEnrichmentInput {
    /// 基于 `PlannedPage + PageContext` 构造稳定增强输入。
    pub fn from_page(page: &PlannedPage, context: &PageContext, allow_mermaid: bool) -> Self {
        Self {
            page_id: page.id.clone(),
            page_type: page.page_type.clone(),
            title: page.title.clone(),
            scope: page.scope.clone(),
            section_titles: section_titles_for_page_type(&page.page_type)
                .into_iter()
                .map(str::to_string)
                .collect(),
            facts: context.facts.clone(),
            summary_inputs: context.summary_inputs.clone(),
            hints: context.hints.clone(),
            child_summaries: context.child_summaries.clone(),
            evidence_groups: context.evidence_groups.clone(),
            diagram_inputs: context.diagram_inputs.clone(),
            allow_mermaid,
        }
    }
}

/// 页面增强输出。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageEnrichmentResult {
    /// 父页面消费的叶子摘要。
    #[serde(default)]
    pub summary: String,
    /// section 标题 -> 增强正文。
    #[serde(default)]
    pub section_overrides: BTreeMap<String, String>,
    /// section 标题 -> Mermaid 图正文（不带 fenced block）。
    #[serde(default)]
    pub mermaid_blocks: BTreeMap<String, String>,
    /// 实际命中的页面提示。
    #[serde(default)]
    pub consumed_hints: Vec<String>,
    /// 实际消费的子页摘要。
    #[serde(default)]
    pub consumed_child_summaries: Vec<String>,
}

impl PageEnrichmentResult {
    /// 对模型输出做轻量结构校验，避免污染正式页面。
    pub fn sanitize_for_input(mut self, input: &PageEnrichmentInput) -> Option<Self> {
        self.summary = normalize_sentence(&self.summary);
        self.consumed_hints = dedupe_non_empty(self.consumed_hints);
        self.consumed_child_summaries = dedupe_non_empty(self.consumed_child_summaries);

        let allowed_titles = input
            .section_titles
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let diagram_titles = input
            .diagram_inputs
            .iter()
            .map(|diagram| diagram.section_title.clone())
            .collect::<BTreeSet<_>>();
        self.section_overrides = self
            .section_overrides
            .into_iter()
            .filter_map(|(title, content)| {
                let title = title.trim().to_string();
                let content = normalize_multiline(&content);
                (allowed_titles.contains(&title) && !content.is_empty()).then_some((title, content))
            })
            .collect();
        self.mermaid_blocks = self
            .mermaid_blocks
            .into_iter()
            .filter_map(|(title, content)| {
                let title = title.trim().to_string();
                if !input.allow_mermaid
                    || !allowed_titles.contains(&title)
                    || !diagram_titles.contains(&title)
                {
                    return None;
                }
                sanitize_mermaid_body(&content).map(|normalized| (title, normalized))
            })
            .collect();

        (!self.summary.is_empty()
            || !self.section_overrides.is_empty()
            || !self.mermaid_blocks.is_empty())
        .then_some(self)
    }
}

#[derive(Debug, Clone)]
struct PendingPageEnrichmentRequest {
    index: usize,
    input_hash: String,
    request: LlmPromptRequest,
}

#[derive(Debug, Clone, Serialize)]
struct FilePurposeBatchInput {
    items: Vec<FilePurposeAssistInput>,
}

/// `LlmRuntime` 统一收口 steering、预算、缓存和真实调用。
pub struct LlmRuntime<'cfg, 'svc> {
    repo_root: &'cfg Path,
    config: &'cfg LlmConfig,
    service: Option<RuntimeLlmService<'svc>>,
    real_calls: usize,
    uncertainty_calls: usize,
    enrichment_calls: usize,
}

impl<'cfg, 'svc> LlmRuntime<'cfg, 'svc> {
    /// 创建当前 workflow 使用的 LLM runtime。
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
            enrichment_calls: 0,
        }
    }

    /// 当前 workflow 是否真正具备可用的 LLM 请求路径。
    pub fn service_available(&self) -> bool {
        self.config.enabled && self.service.is_some()
    }

    /// 页面增强是否处于可用状态。
    pub fn enrichment_enabled(&self) -> bool {
        self.service_available()
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

    /// 按批次判断一组 `FilePurpose::Utility` 兜底文件。
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

        if !self.config.enabled {
            return Ok(vec![None; inputs.len()]);
        }

        let model = self.model_id().map(str::to_string);
        let mut results = vec![None; inputs.len()];
        let mut pending = Vec::<(usize, String, FilePurposeAssistInput)>::new();

        for (index, input) in inputs.iter().enumerate() {
            let input_hash =
                build_prompt_input_hash(PromptType::FilePurpose, model.as_deref(), input);
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

            if self.service.is_none() {
                continue;
            }

            pending.push((index, input_hash, input.clone()));
        }

        for chunk in pending.chunks(FILE_PURPOSE_BATCH_SIZE) {
            if !self.try_consume_budget(PromptType::FilePurpose) {
                break;
            }

            let batch_input = FilePurposeBatchInput {
                items: chunk.iter().map(|(_, _, input)| input.clone()).collect(),
            };
            let batch_input_hash =
                build_prompt_input_hash(PromptType::FilePurpose, model.as_deref(), &batch_input);
            let request = build_structured_request(
                PromptType::FilePurpose,
                &batch_input_hash,
                model.clone(),
                "你是 Repo Wiki 的扫描辅助模型，只能在给定候选语义附近做保守判断。",
                &format!(
                    concat!(
                        "对输入 items 中的每个文件判断最可能的稳定角色。",
                        "返回 `items` 数组；每项必须保留原始 `path`，并且 `purpose` 只能从以下稳定枚举中选择：{}。",
                        "不要返回中文描述、解释句或额外字段。"
                    ),
                    FILE_PURPOSE_ALLOWED_VALUES.join(", ")
                ),
                &batch_input,
                file_purpose_response_schema(true),
            );

            let completion = match self.service.as_mut() {
                Some(service) => service.request(&request),
                None => break,
            };
            let Ok(completion) = completion else {
                continue;
            };
            let completion_model = completion.model.clone();
            let resolved = parse_file_purpose_batch_output(completion.output);
            if resolved.is_empty() {
                continue;
            }

            for (index, input_hash, input) in chunk {
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
            "判断该顶层目录是否值得提升为独立模块，只返回 promote 布尔值。",
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

    /// 为 `module_kind` 兜底分支提供可选判断。
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
            "判断该模块更适合的 kind，只返回 kind 字段。",
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
            "根据 source/target 路径和模块名判断这条低置信度关系是否应保留，只返回 keep 布尔值。",
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

    /// 基于稳定页面输入生成可回退的增强正文。
    pub fn enrich_page(
        &mut self,
        input: &PageEnrichmentInput,
    ) -> io::Result<Option<PageEnrichmentResult>> {
        let instruction = build_page_enrichment_instruction(input);
        self.request_structured(
            PromptType::PageEnrichment,
            input,
            "你是 Repo Wiki 的页面增强模型，只能改写解释层，不能虚构 facts、源码结构或依赖关系，也不要机械复述原始前缀标签。",
            &instruction,
            json!({
                "type": "object",
                "required": ["summary", "section_overrides", "mermaid_blocks"],
                "properties": {
                    "summary": {"type": "string"},
                    "section_overrides": {"type": "object"},
                    "mermaid_blocks": {"type": "object"},
                    "consumed_hints": {"type": "array", "items": {"type": "string"}},
                    "consumed_child_summaries": {"type": "array", "items": {"type": "string"}}
                }
            }),
        )
        .map(|result: Option<PageEnrichmentResult>| {
            result.and_then(|output| output.sanitize_for_input(input))
        })
    }

    /// 对一组页面增强输入执行可选的批量增强。
    /// provider 直连时会在同一深度层内做有限并行；Agent bridge 保持串行。
    pub fn enrich_pages(
        &mut self,
        inputs: &[PageEnrichmentInput],
    ) -> io::Result<Vec<Option<PageEnrichmentResult>>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }

        let parallel_requests = self.provider_parallel_requests();
        if self.selected_path() != Some(SelectedLlmPath::ProviderApi)
            || parallel_requests <= 1
            || inputs.len() <= 1
        {
            return inputs.iter().map(|input| self.enrich_page(input)).collect();
        }

        self.enrich_pages_via_provider(inputs, parallel_requests)
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
        if !self.config.enabled {
            return Ok(None);
        }

        let model = self.model_id().map(str::to_string);
        let input_hash = build_prompt_input_hash(prompt_type, model.as_deref(), input);
        if let Some(cached) = read_llm_cache(
            self.repo_root,
            &input_hash,
            prompt_type.as_str(),
            prompt_type.version(),
            model.as_deref(),
        )? {
            if let Ok(parsed) = serde_json::from_str::<TOutput>(&cached.response) {
                return Ok(Some(parsed));
            }
        }

        if self.service.is_none() {
            return Ok(None);
        }

        if !self.try_consume_budget(prompt_type) {
            return Ok(None);
        }

        let request = build_structured_request(
            prompt_type,
            &input_hash,
            model.clone(),
            system,
            instruction,
            input,
            response_schema,
        );

        let completion = match self.service.as_mut() {
            Some(service) => service.request(&request),
            None => return Ok(None),
        };

        let completion = match completion {
            Ok(completion) => completion,
            Err(_) => return Ok(None),
        };
        let parsed = match serde_json::from_value::<TOutput>(completion.output.clone()) {
            Ok(parsed) => parsed,
            Err(_) => return Ok(None),
        };

        self.write_cached_response(prompt_type, input_hash, model, completion.model, &parsed)?;

        Ok(Some(parsed))
    }

    fn enrich_pages_via_provider(
        &mut self,
        inputs: &[PageEnrichmentInput],
        parallel_requests: usize,
    ) -> io::Result<Vec<Option<PageEnrichmentResult>>> {
        let model = self.model_id().map(str::to_string);
        let mut results = vec![None; inputs.len()];
        let mut pending = Vec::new();

        for (index, input) in inputs.iter().enumerate() {
            let input_hash =
                build_prompt_input_hash(PromptType::PageEnrichment, model.as_deref(), input);
            if let Some(cached) = read_llm_cache(
                self.repo_root,
                &input_hash,
                PromptType::PageEnrichment.as_str(),
                PromptType::PageEnrichment.version(),
                model.as_deref(),
            )? {
                if let Ok(parsed) = serde_json::from_str::<PageEnrichmentResult>(&cached.response) {
                    results[index] = parsed.sanitize_for_input(input);
                    continue;
                }
            }

            if !self.try_consume_budget(PromptType::PageEnrichment) {
                continue;
            }

            let instruction = build_page_enrichment_instruction(input);
            pending.push(PendingPageEnrichmentRequest {
                index,
                input_hash: input_hash.clone(),
                request: build_structured_request(
                    PromptType::PageEnrichment,
                    &input_hash,
                    model.clone(),
                    "你是 Repo Wiki 的页面增强模型，只能改写解释层，不能虚构 facts、源码结构或依赖关系，也不要机械复述原始前缀标签。",
                    &instruction,
                    input,
                    json!({
                        "type": "object",
                        "required": ["summary", "section_overrides", "mermaid_blocks"],
                        "properties": {
                            "summary": {"type": "string"},
                            "section_overrides": {"type": "object"},
                            "mermaid_blocks": {"type": "object"},
                            "consumed_hints": {"type": "array", "items": {"type": "string"}},
                            "consumed_child_summaries": {"type": "array", "items": {"type": "string"}}
                        }
                    }),
                ),
            });
        }

        if pending.is_empty() {
            return Ok(results);
        }

        let next_index = AtomicUsize::new(0);
        let completions = Mutex::new(vec![None; pending.len()]);
        let worker_count = parallel_requests.max(1).min(pending.len());
        thread::scope(|scope| {
            for _ in 0..worker_count {
                let next_index_ref = &next_index;
                let pending_ref = &pending;
                let completions_ref = &completions;
                let config = self.config;
                scope.spawn(move || {
                    let Ok(mut service) = ProviderApiLlmService::from_config(config) else {
                        return;
                    };
                    loop {
                        let task_index = next_index_ref.fetch_add(1, Ordering::Relaxed);
                        if task_index >= pending_ref.len() {
                            break;
                        }
                        let completion = service.request(&pending_ref[task_index].request).ok();
                        completions_ref.lock().unwrap()[task_index] = completion;
                    }
                });
            }
        });

        for (task_index, completion) in completions.into_inner().unwrap().into_iter().enumerate() {
            let Some(completion) = completion else {
                continue;
            };
            let pending_request = &pending[task_index];
            let parsed =
                match serde_json::from_value::<PageEnrichmentResult>(completion.output.clone()) {
                    Ok(parsed) => parsed,
                    Err(_) => continue,
                };
            self.write_cached_response(
                PromptType::PageEnrichment,
                pending_request.input_hash.clone(),
                model.clone(),
                completion.model,
                &parsed,
            )?;
            results[pending_request.index] =
                parsed.sanitize_for_input(&inputs[pending_request.index]);
        }

        Ok(results)
    }

    fn budget_available(&self, prompt_type: PromptType) -> bool {
        if self.real_calls >= self.config.max_calls {
            return false;
        }

        match prompt_type {
            PromptType::PageEnrichment => self.enrichment_calls < self.max_enrichment_calls(),
            _ => self.uncertainty_calls < self.max_uncertainty_calls(),
        }
    }

    fn try_consume_budget(&mut self, prompt_type: PromptType) -> bool {
        if !self.budget_available(prompt_type) {
            return false;
        }

        self.real_calls += 1;
        match prompt_type {
            PromptType::PageEnrichment => self.enrichment_calls += 1,
            _ => self.uncertainty_calls += 1,
        }
        true
    }

    fn max_enrichment_calls(&self) -> usize {
        reserved_enrichment_call_budget(self.config.max_calls)
    }

    fn max_uncertainty_calls(&self) -> usize {
        self.config
            .max_calls
            .saturating_sub(self.max_enrichment_calls())
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
                input_hash,
                prompt_type: prompt_type.as_str().to_string(),
                prompt_version: prompt_type.version().to_string(),
                response: response_json,
                model: model.or(completion_model),
                created_at: crate::workflows::init::current_timestamp(),
                ttl_seconds: self.config.cache_ttl_seconds as i64,
            },
        )
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
    }
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

fn sanitize_mermaid_body(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let stripped = trimmed
        .strip_prefix("```mermaid")
        .map(str::trim)
        .and_then(|value| value.strip_suffix("```").map(str::trim))
        .unwrap_or(trimmed);
    let normalized = normalize_multiline(stripped);
    let mut lines = normalized.lines();
    let header = lines.next()?.trim();

    if !matches!(
        header,
        "graph TD" | "graph LR" | "flowchart TD" | "flowchart LR"
    ) {
        return None;
    }
    if normalized.contains("```") {
        return None;
    }
    if normalized.lines().count() < 2 {
        return None;
    }

    Some(normalized)
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

fn build_page_enrichment_instruction(input: &PageEnrichmentInput) -> String {
    format!(
        concat!(
            "页面类型：{page_type}\n",
            "页面标题：{title}\n",
            "页面作用域：{scope}\n",
            "允许覆盖的 section 标题：{section_titles}\n",
            "evidence groups 数量：{evidence_group_count}\n",
            "diagram inputs 数量：{diagram_input_count}\n",
            "{focus}\n",
            "输出要求：\n",
            "1. `summary` 用 1 到 2 句总结页面真正关心的核心内容。\n",
            "2. `section_overrides` 尽量为每个 section 标题生成非空正文；优先写解释性段落，必要时可用 3 到 6 条项目符号。\n",
            "3. 不要简单回显 `技术栈：`、`图热点：`、`关系：` 这类原始前缀；请去重、压缩重复项，并过滤低价值噪音。\n",
            "4. 只有在 facts 明确支持时才提及循环、社区或流程；如果信息不足，请明确写出“当前事实未显示”或“当前未检测到”，不要猜测。\n",
            "5. 若 `hints`、`child_summaries`、`evidence_groups` 或 `diagram_inputs` 对正文有帮助，请在内容里自然吸收，并把实际使用的 hints/child summaries 写入 `consumed_hints` / `consumed_child_summaries`。\n",
            "6. `mermaid_blocks` 只能复用 `diagram_inputs` 已经给出的稳定图类型与关系；若当前 section 没有 diagram input，请不要返回 Mermaid。\n",
            "7. 不要删除、改写或重新命名 evidence groups 的稳定身份；正文应解释这些 evidence 为什么重要。\n",
            "8. 整体目标是把稳定 facts 组织成可读、可复用的 Wiki 正文，而不是列出原始事实清单。"
        ),
        page_type = input.page_type,
        title = input.title,
        scope = input.scope,
        section_titles = input.section_titles.join("、"),
        evidence_group_count = input.evidence_groups.len(),
        diagram_input_count = input.diagram_inputs.len(),
        focus = page_enrichment_focus(input.page_type.as_str()),
    )
}

fn page_enrichment_focus(page_type: &str) -> &'static str {
    match page_type {
        "overview" => "优先总结仓库定位、主要能力、核心模块和典型执行路径。",
        "architecture" => "优先解释顶层模块分工、结构边界、跨模块协作和关键流程。",
        "module" => "优先解释模块职责、关键源码入口、上下游依赖以及子模块分工。",
        "workflow" => "优先解释构建、CI/CD、部署与运行流程。",
        "topic" => "优先解释专题边界、关键 evidence、相关模块以及图中体现出的稳定关系。",
        _ => "优先解释页面主题和稳定事实之间的关系。",
    }
}

fn reserved_enrichment_call_budget(max_calls: usize) -> usize {
    match max_calls {
        0 => 0,
        1..=3 => 1,
        4..=6 => 2,
        7..=12 => 4,
        _ => (max_calls / 3).clamp(4, 12),
    }
}

fn build_provider_user_message(request: &LlmPromptRequest) -> String {
    serde_json::to_string_pretty(&json!({
        "prompt_type": request.prompt_type,
        "prompt_version": request.prompt_version,
        "input_hash": request.input_hash,
        "instruction": request.instruction,
        "response_schema": request.response_schema,
        "input": request.input,
    }))
    .map(|payload| format!("{payload}\n请只返回符合 response_schema 的 JSON 对象。"))
    .unwrap_or_else(|_| request.instruction.clone())
}

fn extract_provider_content(response: &Value) -> io::Result<String> {
    let choice = response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .ok_or_else(|| io::Error::other("provider response missing choices[0]"))?;
    let message = choice
        .get("message")
        .ok_or_else(|| io::Error::other("provider response missing message"))?;
    if let Some(content) = message.get("content").and_then(Value::as_str) {
        return Ok(content.to_string());
    }
    if let Some(parts) = message.get("content").and_then(Value::as_array) {
        let content = parts
            .iter()
            .filter_map(|part| part.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("\n");
        if !content.trim().is_empty() {
            return Ok(content);
        }
    }

    Err(io::Error::other(
        "provider response missing textual content",
    ))
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
