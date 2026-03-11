//! Steering 配置模块。
//! 定义 `.wiki/wiki.steering.yaml` / `wiki.dev.yaml` 的 schema、读取、兼容归一化和默认值。

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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
    /// provider 直连路径下允许的页面增强请求并行度。
    pub parallel_requests: usize,
    /// prompt 级缓存 TTL（秒）。
    pub cache_ttl_seconds: u64,
    /// 是否允许写出 Mermaid fenced block。
    pub allow_mermaid: bool,
    /// 可选 provider registry。
    pub providers: BTreeMap<String, LlmProviderConfig>,
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
    /// 当前 provider 下可选的模型 registry。
    pub models: BTreeMap<String, LlmProviderModelConfig>,
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

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawSteeringConfig {
    version: Option<u32>,
    scan: Option<ScanConfig>,
    ignore: Option<LegacyIgnoreConfig>,
    modules: Option<ModulesConfig>,
    pages: Option<PagesConfig>,
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
    llm: Option<RawLlmConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawLlmConfig {
    enabled: Option<bool>,
    model: Option<String>,
    max_calls: Option<usize>,
    parallel_requests: Option<usize>,
    cache_ttl_seconds: Option<u64>,
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
    models: Option<BTreeMap<String, RawLlmProviderModelConfig>>,
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
            parallel_requests: 3,
            cache_ttl_seconds: 60 * 60 * 24 * 7,
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
            models: BTreeMap::new(),
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
        let (provider_name, model_name) = selection.split_once('/')?;
        let provider_name = provider_name.trim();
        let model_name = model_name.trim();
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
        self.parallel_requests.max(1)
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

    let mut config = SteeringConfig::default();
    if let Some(raw) = read_yaml_file::<RawSteeringConfig>(&shared_path, "using defaults") {
        apply_raw_steering_config(&mut config, raw);
    }
    if let Some(raw) = read_yaml_file::<RawDevConfig>(&dev_path, "ignoring dev overrides") {
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
    if let Some(parallel_requests) = raw.parallel_requests {
        config.parallel_requests = parallel_requests;
    }
    if let Some(cache_ttl_seconds) = raw.cache_ttl_seconds {
        config.cache_ttl_seconds = cache_ttl_seconds;
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
    if let Some(raw_models) = raw.models {
        for (model_name, raw_model) in raw_models {
            let model = config.models.entry(model_name).or_default();
            apply_raw_llm_provider_model_config(model, raw_model);
        }
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

fn normalize_llm_config(config: &mut LlmConfig) {
    config.model = config.model.trim().to_string();
    if config.parallel_requests == 0 {
        config.parallel_requests = LlmConfig::default().parallel_requests;
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
        if provider.timeout_seconds == 0 {
            provider.timeout_seconds = LlmProviderConfig::default().timeout_seconds;
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
