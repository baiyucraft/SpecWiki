//! Steering 配置模块。
//! 定义 `.wiki/wiki.steering.yaml` 的 schema、读取、兼容归一化和默认值。

use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    /// 自定义页面提示（供后续 LLM 增强使用）。
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
    /// 提示文本。
    pub hint: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RawSteeringConfig {
    version: Option<u32>,
    scan: Option<ScanConfig>,
    ignore: Option<LegacyIgnoreConfig>,
    modules: Option<ModulesConfig>,
    pages: Option<PagesConfig>,
    merge_threshold: Option<u32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct LegacyIgnoreConfig {
    global: Vec<String>,
    #[serde(flatten)]
    per_language: HashMap<String, Vec<String>>,
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
            merge_threshold: 3,
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
}

/// 从 `.wiki/wiki.steering.yaml` 读取 steering 配置。
/// 文件不存在时返回默认配置，格式非法时输出 warning 并回退到默认值。
pub fn load_steering_config(repo_root: &Path) -> SteeringConfig {
    let config_path = repo_root.join(".wiki").join("wiki.steering.yaml");

    if !config_path.exists() {
        return SteeringConfig::default();
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => match serde_yaml::from_str::<RawSteeringConfig>(&content) {
            Ok(raw) => normalize_steering_config(raw),
            Err(e) => {
                eprintln!(
                    "[warn] failed to parse {}: {e}, using defaults",
                    config_path.display()
                );
                SteeringConfig::default()
            }
        },
        Err(e) => {
            eprintln!(
                "[warn] failed to read {}: {e}, using defaults",
                config_path.display()
            );
            SteeringConfig::default()
        }
    }
}

fn normalize_steering_config(raw: RawSteeringConfig) -> SteeringConfig {
    let mut config = SteeringConfig {
        version: raw.version.unwrap_or(1),
        scan: raw.scan.unwrap_or_default(),
        modules: raw.modules.unwrap_or_default(),
        pages: raw.pages.unwrap_or_default(),
        merge_threshold: raw.merge_threshold.unwrap_or(3),
    };

    if let Some(legacy_ignore) = raw.ignore {
        config.scan.ignore.extend(legacy_ignore.global);
        for paths in legacy_ignore.per_language.into_values() {
            config.scan.ignore.extend(paths);
        }
    }

    normalize_patterns(&mut config.scan.ignore);
    normalize_patterns(&mut config.scan.include);

    config
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
