use serde::{Deserialize, Serialize};

/// `TopicSeed` 表示 planner 可消费的稳定主题线索。
/// 它来自 hierarchy 或后续聚合输入，但在进入 planner 前仍保持纯事实型描述。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TopicSeed {
    /// 主题类别，例如 `root-mechanism / module-capability / process`。
    pub topic_kind: String,
    /// 在同一 `topic_kind + parent_scope` 下稳定唯一的主题键。
    pub topic_key: String,
    /// 主题展示标题。
    pub title: String,
    /// 供 topic 页面直接消费的稳定摘要。
    #[serde(default)]
    pub summary: String,
    /// 当前主题直接依赖的源码 ID。
    #[serde(default)]
    pub source_ids: Vec<String>,
    /// 当前主题直接依赖的源码路径。
    #[serde(default)]
    pub source_paths: Vec<String>,
    /// 当前主题关联的模块 ID。
    #[serde(default)]
    pub module_ids: Vec<String>,
    /// 当前主题关联的关系 ID。
    #[serde(default)]
    pub relation_ids: Vec<String>,
}

/// `PageEvidenceItem` 表示页面或 section 中的单条稳定来源线索。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageEvidenceItem {
    /// 当前 evidence 的稳定 ID。
    pub evidence_id: String,
    /// 页面内展示标签，通常是源码路径或简短名称。
    pub label: String,
    /// 对应源码路径。
    pub path: String,
    /// 可选的源码稳定 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// evidence 的补充说明。
    #[serde(default)]
    pub note: String,
}

/// `PageEvidenceGroup` 让 renderer/LLM/测试共享同一组 evidence 分组。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageEvidenceGroup {
    /// 当前分组的稳定 ID。
    pub group_id: String,
    /// 该分组归属的稳定 section 标题。
    pub section_title: String,
    /// 当前分组展示标题。
    pub title: String,
    /// 当前分组摘要。
    #[serde(default)]
    pub summary: String,
    /// 当前分组的 evidence 项集合。
    #[serde(default)]
    pub items: Vec<PageEvidenceItem>,
}

/// `PageDiagramNode` 是受控 Mermaid 输入中的稳定节点。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageDiagramNode {
    /// 图节点稳定 ID。
    pub node_id: String,
    /// 节点展示文本。
    pub label: String,
}

/// `PageDiagramEdge` 是受控 Mermaid 输入中的稳定边。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageDiagramEdge {
    /// 边起点节点 ID。
    pub source: String,
    /// 边终点节点 ID。
    pub target: String,
    /// 可选边标签。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// `PageDiagramInput` 描述一张 deterministic-first 的结构图输入。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageDiagramInput {
    /// 当前图的稳定 ID。
    pub diagram_id: String,
    /// 图归属的稳定 section 标题。
    pub section_title: String,
    /// 图类别，例如 `structure / dependency / flow`。
    pub diagram_type: String,
    /// 图标题。
    pub title: String,
    /// 图摘要。
    #[serde(default)]
    pub summary: String,
    /// 图节点集合。
    #[serde(default)]
    pub nodes: Vec<PageDiagramNode>,
    /// 图边集合。
    #[serde(default)]
    pub edges: Vec<PageDiagramEdge>,
}

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
    #[serde(default)]
    pub root_topics: Vec<TopicSeed>,
    #[serde(default)]
    pub process_topics: Vec<TopicSeed>,
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
    #[serde(default)]
    pub capability_topics: Vec<TopicSeed>,
}

/// `PageContext` 是最终传给渲染器的统一输入。
/// 到这一层以后，页面生成只需要关心“如何表达”，不用再重新做结构分析。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageContext {
    /// 当前页面的稳定 ID。
    pub page_id: String,
    /// 当前页面类型，例如 `overview / module`。
    pub page_type: String,
    /// 页面作用域标签。
    pub scope: String,
    /// 页面直接依赖的源码 ID。
    pub source_ids: Vec<String>,
    /// 页面直接映射到的模块 ID。
    pub module_ids: Vec<String>,
    /// 页面直接依赖的关系 ID。
    pub relation_ids: Vec<String>,
    /// 页面必须稳定落盘的事实输入。
    pub facts: Vec<String>,
    /// 页面用来组织正文的补充说明输入。
    pub summary_inputs: Vec<String>,
    /// 来自 steering 的页面提示。
    #[serde(default)]
    pub hints: Vec<String>,
    /// 叶子优先增强阶段从子页汇总出的摘要。
    #[serde(default)]
    pub child_summaries: Vec<String>,
    /// 页面稳定 evidence groups，供 renderer、LLM 和验证脚本共享。
    #[serde(default)]
    pub evidence_groups: Vec<PageEvidenceGroup>,
    /// 页面稳定图输入，供 renderer 直接构造 Mermaid。
    #[serde(default)]
    pub diagram_inputs: Vec<PageDiagramInput>,
}
