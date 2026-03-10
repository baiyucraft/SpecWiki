use serde::{Deserialize, Serialize};

/// `RepoContext` 是仓库级页面生成的输入。
/// 它把底层扫描/模块树结果压缩成“页面真正关心的信息”。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoContext {
    pub repo_summary_inputs: Vec<String>,
    pub top_modules: Vec<String>,
    pub key_entry_points: Vec<String>,
    pub global_relations: Vec<String>,
    pub tech_stack: Vec<String>,
    #[serde(default)]
    pub graph_hotspots: Vec<String>,
    #[serde(default)]
    pub detected_processes: Vec<String>,
    #[serde(default)]
    pub community_labels: Vec<String>,
    #[serde(default)]
    pub cycle_warnings: Vec<String>,
}

/// `ModuleContext` 是模块页生成的输入。
/// 这里聚焦模块角色、公开入口和上下游关系，避免渲染层直接读全量扫描结果。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleContext {
    pub module_id: String,
    pub role_hints: Vec<String>,
    pub public_surface: Vec<String>,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub key_sources: Vec<String>,
    #[serde(default)]
    pub graph_hotspots: Vec<String>,
    #[serde(default)]
    pub communities: Vec<String>,
    #[serde(default)]
    pub cycle_warnings: Vec<String>,
}

/// `PageContext` 是最终传给渲染器的统一输入。
/// 到这一层以后，页面生成只需要关心“如何表达”，不用再重新做结构分析。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageContext {
    pub page_id: String,
    pub page_type: String,
    pub scope: String,
    pub source_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub relation_ids: Vec<String>,
    pub facts: Vec<String>,
    pub summary_inputs: Vec<String>,
}
