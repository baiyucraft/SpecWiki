//! Steering 配置模块。
//! 定义 `.wiki/wiki.steering.yaml` / `wiki.dev.yaml` 的 schema、读取、兼容归一化和默认值。

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::{Duration as TimeDuration, OffsetDateTime};

/// Steering 配置根结构。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct SteeringConfig {
    /// Schema 版本号。
    pub version: u32,
    /// 扫描边界配置。
    pub scan: ScanConfig,
    /// 模块提升/降级配置。
    pub modules: ModulesConfig,
    /// 页面优先级和提示配置。
    pub pages: PagesConfig,
    /// 可选的 debug trace 配置。
    pub debug: DebugConfig,
    /// LLM 辅助增强配置。
    pub llm: LlmConfig,
    /// 小模块合并阈值（源文件数 ≤ 该值且无子模块的模块被合并）。
    pub merge_threshold: u32,
}

/// 扫描边界配置。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ScanConfig {
    /// 追加忽略路径。
    pub ignore: Vec<String>,
    /// 显式恢复被忽略路径。
    pub include: Vec<String>,
}

/// 模块提升/降级配置。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModulesConfig {
    /// 强制生成独立页面的模块路径。
    pub promote: Vec<ModuleOverride>,
    /// 强制合并到父模块页面的模块路径。
    pub demote: Vec<ModuleOverride>,
}

/// 单条模块提升/降级条目。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleOverride {
    /// 模块根路径（相对于仓库根目录）。
    pub path: String,
    /// 提升/降级原因（可选，供文档和调试使用）。
    #[serde(default)]
    pub reason: String,
}

/// 页面优先级和提示配置。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PagesConfig {
    /// 页面优先级提升条目。
    pub priority: Vec<PagePriority>,
    /// 自定义页面提示。
    /// 当前同时服务 deterministic hints 透传和 LLM 增强输入。
    pub hints: Vec<PageHint>,
}

/// debug trace 配置。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct DebugConfig {
    /// 是否允许 workflow 进入 debug trace 路径。
    pub enabled: bool,
    /// trace 输出目录；相对路径基于 repo root 解析。
    #[serde(alias = "traceDir")]
    pub trace_dir: String,
    /// 是否把 trace 条目实时镜像到 `stderr`。
    #[serde(alias = "echoToStderr")]
    pub echo_to_stderr: bool,
}

/// 页面优先级提升条目。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagePriority {
    /// 模块路径。
    pub path: String,
    /// 优先级提升值。
    #[serde(default = "default_boost")]
    pub boost: i32,
}

/// 页面提示条目。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageHint {
    /// 目标页面类型。
    pub page_type: String,
    /// 目标页面 ID；为空表示按 `page_type` 广播。
    #[serde(default)]
    pub page_id: String,
    /// 目标模块路径；为空表示不限制模块范围。
    #[serde(default, rename = "modulePath")]
    pub module_path: String,
    /// 提示文本。
    pub hint: String,
}

/// LLM 增强配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LlmConfig {
    /// 是否允许 workflow 进入 LLM 辅助路径。
    pub enabled: bool,
    /// 当前 workflow 使用的模型选择，格式为 `provider/model`。
    pub model: String,
    /// 单次 workflow 允许的真实调用上限。
    pub max_calls: usize,
    /// 是否允许 `uncertainty_gate` 路径发起 LLM 请求。
    pub uncertainty_gate_enabled: bool,
    /// 是否允许 `content_enrichment` 路径发起 LLM 请求。
    pub content_enrichment_enabled: bool,
    /// 是否允许 `module/topic` 页进入 bounded research session。
    pub session_enabled: bool,
    /// `uncertainty_gate` 的单请求输入上限。
    pub uncertainty_gate_max_input_tokens: usize,
    /// `page_enrichment` 的单请求输入上限。
    pub page_enrichment_max_input_tokens: usize,
    /// bounded research session 的上下文上限。
    pub session_max_context_tokens: usize,
    /// bounded research session 保留的最近轮次窗口。
    pub session_max_recent_turns: usize,
    /// `uncertainty_gate` 的安全并行度。
    pub uncertainty_gate_parallel_requests: usize,
    /// provider 直连路径下允许的页面增强请求并行度。
    pub page_enrichment_parallel_requests: usize,
    /// prompt 级缓存 TTL（秒）。
    pub cache_ttl_seconds: u64,
    /// LLM cache 生命周期模式。
    pub cache_mode: LlmCacheMode,
    /// 是否允许写出 Mermaid fenced block。
    pub allow_mermaid: bool,
    /// 可选 provider registry。
    pub providers: BTreeMap<String, LlmProviderConfig>,
}

/// `LlmCacheMode` 控制 workflow 对 LLM cache 的处理方式。
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LlmCacheMode {
    /// 默认保留并复用已有缓存。
    #[default]
    Preserve,
    /// 显式清空相关缓存后再运行。
    Clear,
    /// 保留现有缓存，但本轮强制刷新读取结果。
    Refresh,
}

/// `LlmToolsMode` 表示 provider tool-calling 的能力模式。
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LlmToolsMode {
    /// 自动探测并允许 learned state 生效。
    #[default]
    Auto,
    /// 直接使用 provider 原生 tools。
    NativeTools,
    /// 通过消息内协议模拟 tools。
    EmulatedTools,
    /// 不走 tool-calling。
    NoTools,
}

/// LLM provider 直连配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LlmProviderConfig {
    /// openai-compatible API 的 base URL，或完整 `/chat/completions` 端点。
    #[serde(alias = "apiBase")]
    pub api_base: String,
    /// 本地直接写入的 API key；为空时再看 `api_key_env`。
    #[serde(alias = "apiKey")]
    pub api_key: String,
    /// API key 对应的环境变量名。
    #[serde(alias = "apiKeyEnv")]
    pub api_key_env: String,
    /// provider HTTP 请求超时（秒）。
    #[serde(alias = "timeoutSeconds")]
    pub timeout_seconds: u64,
    /// provider 瞬时失败时的总尝试次数，包含首次请求。
    #[serde(alias = "maxRetries")]
    pub max_retries: usize,
    /// provider 级默认模型名；当顶层 `llm.model` 为空时可作为回退。
    #[serde(alias = "defaultModel")]
    pub default_model: String,
    /// provider 能力声明。
    #[serde(default)]
    pub capabilities: LlmProviderCapabilitiesConfig,
    /// 当前 provider 下可选的模型 registry。
    pub models: BTreeMap<String, LlmProviderModelConfig>,
}

/// provider 能力声明。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LlmProviderCapabilitiesConfig {
    /// tool-calling 能力模式。
    #[serde(alias = "toolsMode")]
    pub tools_mode: LlmToolsMode,
    /// 是否默认发送顶层 `response_format`。
    #[serde(alias = "responseFormat")]
    pub response_format: bool,
}

/// provider 下的单个模型配置。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LlmProviderModelConfig {
    /// 实际发给 provider API 的模型标识；为空时默认使用 `models` 键名。
    #[serde(alias = "modelId")]
    pub model_id: String,
}

/// 解析后的 provider/model 选择结果。
#[derive(Debug, Clone)]
pub struct ResolvedLlmModel<'a> {
    /// 顶层 `llm.model` 中声明的 provider 名称。
    pub provider_name: &'a str,
    /// 顶层 `llm.model` 中声明的模型名称。
    pub model_name: &'a str,
    /// 命中的 provider 配置。
    pub provider: &'a LlmProviderConfig,
    /// 命中的模型配置。
    pub model: &'a LlmProviderModelConfig,
}

/// `LearnedStateFile` 保存用户级 learned capability 结果。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LearnedStateFile {
    pub version: u32,
    pub learned: LearnedProvidersState,
}

/// learned providers 根对象。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LearnedProvidersState {
    pub providers: BTreeMap<String, LearnedProviderState>,
}

/// 单个 provider 的 learned 结果。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LearnedProviderState {
    pub models: BTreeMap<String, LearnedProviderModelState>,
}

/// 单个 provider/model 的 learned capability 结果。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LearnedProviderModelState {
    pub api_base: String,
    pub tools_mode: LlmToolsMode,
    pub detected_at: String,
    pub reason: String,
    pub ttl_hours: u64,
}

impl Default for LearnedStateFile {
    fn default() -> Self {
        Self {
            version: 1,
            learned: LearnedProvidersState::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawSteeringConfig {
    version: Option<u32>,
    scan: Option<ScanConfig>,
    ignore: Option<LegacyIgnoreConfig>,
    modules: Option<ModulesConfig>,
    pages: Option<PagesConfig>,
    debug: Option<RawDebugConfig>,
    llm: Option<RawLlmConfig>,
    merge_threshold: Option<u32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct LegacyIgnoreConfig {
    global: Vec<String>,
    #[serde(flatten)]
    per_language: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawDevConfig {
    debug: Option<RawDebugConfig>,
    llm: Option<RawLlmConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawDebugConfig {
    enabled: Option<bool>,
    #[serde(alias = "traceDir")]
    trace_dir: Option<String>,
    #[serde(alias = "echoToStderr")]
    echo_to_stderr: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawLlmConfig {
    enabled: Option<bool>,
    model: Option<String>,
    max_calls: Option<usize>,
    uncertainty_gate_enabled: Option<bool>,
    content_enrichment_enabled: Option<bool>,
    session_enabled: Option<bool>,
    uncertainty_gate_max_input_tokens: Option<usize>,
    page_enrichment_max_input_tokens: Option<usize>,
    session_max_context_tokens: Option<usize>,
    session_max_recent_turns: Option<usize>,
    uncertainty_gate_parallel_requests: Option<usize>,
    #[serde(alias = "parallel_requests")]
    page_enrichment_parallel_requests: Option<usize>,
    cache_ttl_seconds: Option<u64>,
    cache_mode: Option<LlmCacheMode>,
    allow_mermaid: Option<bool>,
    providers: Option<BTreeMap<String, RawLlmProviderConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawLlmProviderConfig {
    #[serde(alias = "apiBase")]
    api_base: Option<String>,
    #[serde(alias = "apiKey")]
    api_key: Option<String>,
    #[serde(alias = "apiKeyEnv")]
    api_key_env: Option<String>,
    #[serde(alias = "timeoutSeconds")]
    timeout_seconds: Option<u64>,
    #[serde(alias = "maxRetries")]
    max_retries: Option<usize>,
    #[serde(alias = "defaultModel")]
    default_model: Option<String>,
    capabilities: Option<RawLlmProviderCapabilitiesConfig>,
    models: Option<BTreeMap<String, RawLlmProviderModelConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawLlmProviderCapabilitiesConfig {
    #[serde(alias = "toolsMode")]
    tools_mode: Option<LlmToolsMode>,
    #[serde(alias = "responseFormat")]
    response_format: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawLlmProviderModelConfig {
    #[serde(alias = "modelId")]
    model_id: Option<String>,
}

fn default_boost() -> i32 {
    1
}

impl Default for SteeringConfig {
    fn default() -> Self {
        Self {
            version: 1,
            scan: ScanConfig::default(),
            modules: ModulesConfig::default(),
            pages: PagesConfig::default(),
            debug: DebugConfig::default(),
            llm: LlmConfig::default(),
            merge_threshold: 3,
        }
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model: String::new(),
            max_calls: 24,
            uncertainty_gate_enabled: true,
            content_enrichment_enabled: true,
            session_enabled: true,
            uncertainty_gate_max_input_tokens: 12_000,
            page_enrichment_max_input_tokens: 8_000,
            session_max_context_tokens: 16_000,
            session_max_recent_turns: 6,
            uncertainty_gate_parallel_requests: 3,
            page_enrichment_parallel_requests: 3,
            cache_ttl_seconds: 60 * 60 * 24 * 7,
            cache_mode: LlmCacheMode::Preserve,
            allow_mermaid: true,
            providers: BTreeMap::new(),
        }
    }
}

impl Default for LlmProviderConfig {
    fn default() -> Self {
        Self {
            api_base: String::new(),
            api_key: String::new(),
            api_key_env: String::new(),
            timeout_seconds: 90,
            max_retries: 3,
            default_model: String::new(),
            capabilities: LlmProviderCapabilitiesConfig::default(),
            models: BTreeMap::new(),
        }
    }
}

impl Default for LlmProviderCapabilitiesConfig {
    fn default() -> Self {
        Self {
            tools_mode: LlmToolsMode::Auto,
            response_format: true,
        }
    }
}

impl SteeringConfig {
    /// 兼容旧接口：现在忽略语言参数，统一返回 `scan.ignore`。
    pub fn effective_ignore_paths(&self, _primary_language: Option<&str>) -> Vec<String> {
        self.scan.ignore.clone()
    }

    /// 返回当前扫描边界。
    pub fn scan_boundary(&self) -> (&[String], &[String]) {
        (&self.scan.ignore, &self.scan.include)
    }

    /// 检查某个模块路径是否被 promote。
    pub fn is_promoted(&self, module_path: &str) -> bool {
        self.modules
            .promote
            .iter()
            .any(|entry| entry.path == module_path)
    }

    /// 检查某个模块路径是否被 demote。
    pub fn is_demoted(&self, module_path: &str) -> bool {
        self.modules
            .demote
            .iter()
            .any(|entry| entry.path == module_path)
    }

    /// 返回当前页面命中的 steering hints。
    pub fn page_hints_for(
        &self,
        page_type: &str,
        page_id: Option<&str>,
        module_paths: &[String],
    ) -> Vec<String> {
        let module_paths = module_paths.iter().collect::<Vec<_>>();

        self.pages
            .hints
            .iter()
            .filter(|hint| {
                if hint.page_type != page_type {
                    return false;
                }
                if !hint.page_id.is_empty() && page_id != Some(hint.page_id.as_str()) {
                    return false;
                }
                if !hint.module_path.is_empty()
                    && !module_paths
                        .iter()
                        .any(|module_path| module_path.as_str() == hint.module_path)
                {
                    return false;
                }
                true
            })
            .map(|hint| hint.hint.clone())
            .collect()
    }
}

impl LlmConfig {
    /// 返回当前配置是否已经具备 provider 直连所需的最小参数。
    pub fn provider_configured(&self) -> bool {
        self.resolve_selected_model()
            .is_some_and(|selected| selected.provider.endpoint_url().is_some())
    }

    /// 解析顶层 `llm.model = provider/model` 到实际 provider/model 配置。
    pub fn resolve_selected_model(&self) -> Option<ResolvedLlmModel<'_>> {
        let selection = self.model.trim();
        let (provider_name, model_name) = if selection.is_empty() {
            let (provider_name, provider) = self
                .providers
                .iter()
                .find(|(_, provider)| !provider.default_model.trim().is_empty())?;
            (provider_name.as_str(), provider.default_model.trim())
        } else {
            let (provider_name, model_name) = selection.split_once('/')?;
            (provider_name.trim(), model_name.trim())
        };
        if provider_name.is_empty() || model_name.is_empty() {
            return None;
        }

        let provider = self.providers.get(provider_name)?;
        let model = provider.models.get(model_name)?;
        Some(ResolvedLlmModel {
            provider_name,
            model_name,
            provider,
            model,
        })
    }

    /// 返回 provider 直连路径可用的安全并行度。
    pub fn provider_parallel_requests(&self) -> usize {
        self.page_enrichment_parallel_requests.max(1)
    }

    /// 返回 `uncertainty_gate` 是否可用。
    pub fn uncertainty_gate_enabled(&self) -> bool {
        self.enabled && self.uncertainty_gate_enabled
    }

    /// 返回 `content_enrichment` 是否可用。
    pub fn content_enrichment_enabled(&self) -> bool {
        self.enabled && self.content_enrichment_enabled
    }

    /// 返回 bounded research session 是否可用。
    pub fn session_enabled(&self) -> bool {
        self.enabled && self.session_enabled
    }
}

impl LlmProviderConfig {
    /// 解析 openai-compatible `/chat/completions` 端点。
    pub fn endpoint_url(&self) -> Option<String> {
        let api_base = self.api_base.trim().trim_end_matches('/');
        if api_base.is_empty() {
            return None;
        }
        if api_base.ends_with("/chat/completions") {
            return Some(api_base.to_string());
        }
        Some(format!("{api_base}/chat/completions"))
    }

    /// 解析 provider API key；优先使用显式值，再回退环境变量。
    pub fn resolved_api_key(&self) -> Option<String> {
        if !self.api_key.trim().is_empty() {
            return Some(self.api_key.trim().to_string());
        }
        let env_name = self.api_key_env.trim();
        if env_name.is_empty() {
            return None;
        }
        std::env::var(env_name)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    }
}

impl LlmProviderCapabilitiesConfig {
    /// 解析当前 provider 的显式工具模式。
    pub fn tools_mode(&self) -> LlmToolsMode {
        self.tools_mode
    }
}

impl LlmProviderModelConfig {
    /// 返回最终发给 provider API 的模型标识。
    pub fn resolved_model_id<'a>(&'a self, fallback: &'a str) -> &'a str {
        if self.model_id.trim().is_empty() {
            fallback
        } else {
            self.model_id.trim()
        }
    }
}

/// 从 `.wiki/wiki.steering.yaml` 读取共享 steering，再按需叠加 repo 根 `wiki.dev.yaml`。
/// 配置文件不存在时返回默认配置，格式非法时输出 warning 并回退到默认值或忽略 dev 覆盖。
pub fn load_steering_config(repo_root: &Path) -> SteeringConfig {
    let shared_path = repo_root.join(".wiki").join("wiki.steering.yaml");
    let dev_path = repo_root.join("wiki.dev.yaml");
    let user_config_path = spec_wiki_user_config_path();

    let mut config = SteeringConfig::default();
    if let Some(path) = user_config_path.as_deref() {
        if let Some(raw) = read_yaml_file::<RawDevConfig>(path, "ignoring user config") {
            if let Some(raw_debug) = raw.debug {
                apply_raw_debug_config(&mut config.debug, raw_debug);
            }
            if let Some(raw_llm) = raw.llm {
                apply_raw_llm_config(&mut config.llm, raw_llm);
            }
        }
    }
    if let Some(raw) = read_yaml_file::<RawSteeringConfig>(&shared_path, "using defaults") {
        apply_raw_steering_config(&mut config, raw);
    }
    if let Some(raw) = read_yaml_file::<RawDevConfig>(&dev_path, "ignoring dev overrides") {
        if let Some(raw_debug) = raw.debug {
            apply_raw_debug_config(&mut config.debug, raw_debug);
        }
        if let Some(raw_llm) = raw.llm {
            apply_raw_llm_config(&mut config.llm, raw_llm);
        }
    }

    normalize_steering_config(&mut config);
    config
}

fn apply_raw_steering_config(config: &mut SteeringConfig, raw: RawSteeringConfig) {
    if let Some(version) = raw.version {
        config.version = version;
    }
    if let Some(scan) = raw.scan {
        config.scan = scan;
    }
    if let Some(modules) = raw.modules {
        config.modules = modules;
    }
    if let Some(pages) = raw.pages {
        config.pages = pages;
    }
    if let Some(raw_debug) = raw.debug {
        apply_raw_debug_config(&mut config.debug, raw_debug);
    }
    if let Some(raw_llm) = raw.llm {
        apply_raw_llm_config(&mut config.llm, raw_llm);
    }
    if let Some(merge_threshold) = raw.merge_threshold {
        config.merge_threshold = merge_threshold;
    }
    if let Some(legacy_ignore) = raw.ignore {
        config.scan.ignore.extend(legacy_ignore.global);
        for paths in legacy_ignore.per_language.into_values() {
            config.scan.ignore.extend(paths);
        }
    }
}

fn apply_raw_debug_config(config: &mut DebugConfig, raw: RawDebugConfig) {
    if let Some(enabled) = raw.enabled {
        config.enabled = enabled;
    }
    if let Some(trace_dir) = raw.trace_dir {
        config.trace_dir = trace_dir;
    }
    if let Some(echo_to_stderr) = raw.echo_to_stderr {
        config.echo_to_stderr = echo_to_stderr;
    }
}

fn apply_raw_llm_config(config: &mut LlmConfig, raw: RawLlmConfig) {
    if let Some(enabled) = raw.enabled {
        config.enabled = enabled;
    }
    if let Some(model) = raw.model {
        config.model = model;
    }
    if let Some(max_calls) = raw.max_calls {
        config.max_calls = max_calls;
    }
    if let Some(enabled) = raw.uncertainty_gate_enabled {
        config.uncertainty_gate_enabled = enabled;
    }
    if let Some(enabled) = raw.content_enrichment_enabled {
        config.content_enrichment_enabled = enabled;
    }
    if let Some(enabled) = raw.session_enabled {
        config.session_enabled = enabled;
    }
    if let Some(limit) = raw.uncertainty_gate_max_input_tokens {
        config.uncertainty_gate_max_input_tokens = limit;
    }
    if let Some(limit) = raw.page_enrichment_max_input_tokens {
        config.page_enrichment_max_input_tokens = limit;
    }
    if let Some(limit) = raw.session_max_context_tokens {
        config.session_max_context_tokens = limit;
    }
    if let Some(limit) = raw.session_max_recent_turns {
        config.session_max_recent_turns = limit;
    }
    if let Some(parallel_requests) = raw.uncertainty_gate_parallel_requests {
        config.uncertainty_gate_parallel_requests = parallel_requests;
    }
    if let Some(parallel_requests) = raw.page_enrichment_parallel_requests {
        config.page_enrichment_parallel_requests = parallel_requests;
    }
    if let Some(cache_ttl_seconds) = raw.cache_ttl_seconds {
        config.cache_ttl_seconds = cache_ttl_seconds;
    }
    if let Some(cache_mode) = raw.cache_mode {
        config.cache_mode = cache_mode;
    }
    if let Some(allow_mermaid) = raw.allow_mermaid {
        config.allow_mermaid = allow_mermaid;
    }
    if let Some(raw_providers) = raw.providers {
        for (provider_name, raw_provider) in raw_providers {
            let provider = config.providers.entry(provider_name).or_default();
            apply_raw_llm_provider_config(provider, raw_provider);
        }
    }
}

fn apply_raw_llm_provider_config(config: &mut LlmProviderConfig, raw: RawLlmProviderConfig) {
    if let Some(api_base) = raw.api_base {
        config.api_base = api_base;
    }
    if let Some(api_key) = raw.api_key {
        config.api_key = api_key;
    }
    if let Some(api_key_env) = raw.api_key_env {
        config.api_key_env = api_key_env;
    }
    if let Some(timeout_seconds) = raw.timeout_seconds {
        config.timeout_seconds = timeout_seconds;
    }
    if let Some(max_retries) = raw.max_retries {
        config.max_retries = max_retries;
    }
    if let Some(default_model) = raw.default_model {
        config.default_model = default_model;
    }
    if let Some(raw_capabilities) = raw.capabilities {
        apply_raw_llm_provider_capabilities_config(&mut config.capabilities, raw_capabilities);
    }
    if let Some(raw_models) = raw.models {
        for (model_name, raw_model) in raw_models {
            let model = config.models.entry(model_name).or_default();
            apply_raw_llm_provider_model_config(model, raw_model);
        }
    }
}

fn apply_raw_llm_provider_capabilities_config(
    config: &mut LlmProviderCapabilitiesConfig,
    raw: RawLlmProviderCapabilitiesConfig,
) {
    if let Some(tools_mode) = raw.tools_mode {
        config.tools_mode = tools_mode;
    }
    if let Some(response_format) = raw.response_format {
        config.response_format = response_format;
    }
}

fn apply_raw_llm_provider_model_config(
    config: &mut LlmProviderModelConfig,
    raw: RawLlmProviderModelConfig,
) {
    if let Some(model_id) = raw.model_id {
        config.model_id = model_id;
    }
}

fn normalize_steering_config(config: &mut SteeringConfig) {
    normalize_patterns(&mut config.scan.ignore);
    normalize_patterns(&mut config.scan.include);
    normalize_page_hints(&mut config.pages.hints);
    normalize_debug_config(&mut config.debug);
    normalize_llm_config(&mut config.llm);
}

fn normalize_patterns(patterns: &mut Vec<String>) {
    let mut seen = HashMap::new();
    let mut normalized = Vec::new();

    for pattern in patterns.drain(..) {
        let normalized_pattern = pattern
            .trim()
            .replace('\\', "/")
            .trim_start_matches("./")
            .to_string();
        if normalized_pattern.is_empty() || seen.insert(normalized_pattern.clone(), ()).is_some() {
            continue;
        }
        normalized.push(normalized_pattern);
    }

    *patterns = normalized;
}

fn normalize_page_hints(hints: &mut Vec<PageHint>) {
    let mut seen = HashMap::new();
    let mut normalized = Vec::new();

    for mut hint in hints.drain(..) {
        hint.page_type = hint.page_type.trim().to_string();
        hint.page_id = hint.page_id.trim().to_string();
        hint.module_path = hint
            .module_path
            .trim()
            .replace('\\', "/")
            .trim_start_matches("./")
            .to_string();
        hint.hint = hint.hint.trim().to_string();

        if hint.page_type.is_empty() || hint.hint.is_empty() {
            continue;
        }

        let dedupe_key = format!(
            "{}|{}|{}|{}",
            hint.page_type, hint.page_id, hint.module_path, hint.hint
        );
        if seen.insert(dedupe_key, ()).is_some() {
            continue;
        }
        normalized.push(hint);
    }

    *hints = normalized;
}

fn normalize_debug_config(config: &mut DebugConfig) {
    config.trace_dir = config.trace_dir.trim().to_string();
}

fn normalize_llm_config(config: &mut LlmConfig) {
    config.model = config.model.trim().to_string();
    if config.uncertainty_gate_max_input_tokens == 0 {
        config.uncertainty_gate_max_input_tokens =
            LlmConfig::default().uncertainty_gate_max_input_tokens;
    }
    if config.page_enrichment_max_input_tokens == 0 {
        config.page_enrichment_max_input_tokens =
            LlmConfig::default().page_enrichment_max_input_tokens;
    }
    if config.session_max_context_tokens == 0 {
        config.session_max_context_tokens = LlmConfig::default().session_max_context_tokens;
    }
    if config.session_max_recent_turns == 0 {
        config.session_max_recent_turns = LlmConfig::default().session_max_recent_turns;
    }
    if config.uncertainty_gate_parallel_requests == 0 {
        config.uncertainty_gate_parallel_requests =
            LlmConfig::default().uncertainty_gate_parallel_requests;
    }
    if config.page_enrichment_parallel_requests == 0 {
        config.page_enrichment_parallel_requests =
            LlmConfig::default().page_enrichment_parallel_requests;
    }

    let mut normalized_providers = BTreeMap::new();
    for (provider_name, mut provider) in std::mem::take(&mut config.providers) {
        let normalized_provider_name = provider_name.trim().to_string();
        if normalized_provider_name.is_empty() {
            continue;
        }

        provider.api_base = provider.api_base.trim().to_string();
        provider.api_key = provider.api_key.trim().to_string();
        provider.api_key_env = provider.api_key_env.trim().to_string();
        provider.default_model = provider.default_model.trim().to_string();
        if provider.timeout_seconds == 0 {
            provider.timeout_seconds = LlmProviderConfig::default().timeout_seconds;
        }
        if provider.max_retries == 0 {
            provider.max_retries = LlmProviderConfig::default().max_retries;
        }

        let mut normalized_models = BTreeMap::new();
        for (model_name, mut model) in std::mem::take(&mut provider.models) {
            let normalized_model_name = model_name.trim().to_string();
            if normalized_model_name.is_empty() {
                continue;
            }
            model.model_id = model.model_id.trim().to_string();
            normalized_models.insert(normalized_model_name, model);
        }
        provider.models = normalized_models;

        normalized_providers.insert(normalized_provider_name, provider);
    }
    config.providers = normalized_providers;
}

/// 返回 `~/.spec-wiki/` 目录。
pub fn spec_wiki_home_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(std::path::PathBuf::from))
        .map(|home| home.join(".spec-wiki"))
}

/// 返回用户级配置文件路径。
pub fn spec_wiki_user_config_path() -> Option<std::path::PathBuf> {
    spec_wiki_home_dir().map(|dir| dir.join("config.yaml"))
}

/// 返回用户级 learned state 路径。
pub fn spec_wiki_user_state_path() -> Option<std::path::PathBuf> {
    spec_wiki_home_dir().map(|dir| dir.join("state.yaml"))
}

/// 读取用户级 learned state。
pub fn load_spec_wiki_state() -> LearnedStateFile {
    let Some(path) = spec_wiki_user_state_path() else {
        return LearnedStateFile::default();
    };

    read_yaml_file::<LearnedStateFile>(&path, "ignoring learned state").unwrap_or_default()
}

/// 解析当前 provider/model 命中的 learned tools mode。
pub fn resolve_learned_tools_mode(
    provider_name: &str,
    provider: &LlmProviderConfig,
    model_name: &str,
) -> Option<LlmToolsMode> {
    let state = load_spec_wiki_state();
    let learned = state
        .learned
        .providers
        .get(provider_name)?
        .models
        .get(model_name)?;
    if learned.tools_mode == LlmToolsMode::Auto
        || learned.api_base.trim() != provider.api_base.trim()
        || learned_state_expired(learned)
    {
        return None;
    }

    Some(learned.tools_mode)
}

/// 以原子写入方式回写 learned tools mode。
pub fn persist_learned_tools_mode(
    provider_name: &str,
    model_name: &str,
    provider: &LlmProviderConfig,
    tools_mode: LlmToolsMode,
    reason: &str,
    ttl_hours: u64,
) -> io::Result<()> {
    let Some(path) = spec_wiki_user_state_path() else {
        return Ok(());
    };
    let mut state = load_spec_wiki_state();
    state.version = 1;
    state
        .learned
        .providers
        .entry(provider_name.to_string())
        .or_default()
        .models
        .insert(
            model_name.to_string(),
            LearnedProviderModelState {
                api_base: provider.api_base.trim().to_string(),
                tools_mode,
                detected_at: OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .unwrap_or_else(|_| String::new()),
                reason: reason.trim().to_string(),
                ttl_hours: ttl_hours.max(1),
            },
        );
    write_yaml_atomically(&path, &state)
}

fn learned_state_expired(entry: &LearnedProviderModelState) -> bool {
    let Ok(detected_at) = OffsetDateTime::parse(&entry.detected_at, &Rfc3339) else {
        return true;
    };
    let ttl = TimeDuration::hours(entry.ttl_hours.max(1) as i64);
    OffsetDateTime::now_utc() > detected_at + ttl
}

fn write_yaml_atomically<T>(path: &Path, value: &T) -> io::Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_yaml::to_string(value).map_err(|error| io::Error::other(error.to_string()))?;
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, content)?;
    fs::rename(tmp_path, path)
}

fn read_yaml_file<T>(path: &Path, fallback_label: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    if !path.exists() {
        return None;
    }

    match fs::read_to_string(path) {
        Ok(content) => match serde_yaml::from_str::<T>(&content) {
            Ok(raw) => Some(raw),
            Err(error) => {
                eprintln!(
                    "[warn] failed to parse {}: {error}, {fallback_label}",
                    path.display()
                );
                None
            }
        },
        Err(error) => {
            eprintln!(
                "[warn] failed to read {}: {error}, {fallback_label}",
                path.display()
            );
            None
        }
    }
}
