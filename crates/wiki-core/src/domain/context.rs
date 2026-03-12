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
    /// 对应源码起始行号。
    #[serde(default)]
    pub start_line: usize,
    /// 对应源码结束行号。
    #[serde(default)]
    pub end_line: usize,
    /// 证据类别，例如 `entry-point / symbol / process / child-rollup`。
    #[serde(default)]
    pub evidence_type: String,
    /// 当前 evidence 直接支撑的 section keys。
    #[serde(default)]
    pub section_refs: Vec<String>,
    /// evidence 的补充说明。
    #[serde(default)]
    pub note: String,
    /// 是否只能提供文件级粗粒度跨度。
    #[serde(default)]
    pub coarse_span: bool,
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

/// `TargetedSnippet` 是 dossier / tool 共享的定点源码片段。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TargetedSnippet {
    /// 当前 snippet 的稳定 ID。
    pub snippet_id: String,
    /// 对应源码稳定 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// 对应源码路径。
    pub path: String,
    /// 片段起始行号。
    pub start_line: usize,
    /// 片段结束行号。
    pub end_line: usize,
    /// 片段类别，例如 `symbol / entry-point / process / evidence / child-rollup`。
    #[serde(default)]
    pub snippet_kind: String,
    /// 片段打分，供裁剪阶段稳定排序。
    #[serde(default)]
    pub score: i32,
    /// 当前片段直接关联的稳定 symbol IDs。
    #[serde(default)]
    pub symbol_ids: Vec<String>,
    /// 片段内容。
    #[serde(default)]
    pub content: String,
}

/// 兼容现有调用路径，`SourceSnippet` 继续指向 9.3 的定点片段结构。
pub type SourceSnippet = TargetedSnippet;

/// `PageResearchEvidenceItem` 是 research 结果里的单条 evidence。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchEvidenceItem {
    /// 对应源码稳定 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// 对应源码路径。
    pub path: String,
    /// 起始行号。
    #[serde(default)]
    pub start_line: usize,
    /// 结束行号。
    #[serde(default)]
    pub end_line: usize,
    /// 证据类别。
    #[serde(default)]
    pub evidence_type: String,
    /// 当前 evidence 直接支撑的 section keys。
    #[serde(default)]
    pub section_refs: Vec<String>,
    /// evidence 说明。
    #[serde(default)]
    pub note: String,
    /// 是否只能提供粗粒度跨度。
    #[serde(default)]
    pub coarse_span: bool,
}

/// `PageResearchEvidenceGroup` 是 research 结果里的证据分组。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchEvidenceGroup {
    /// 分组稳定键，默认对齐 `PageEvidenceGroup.group_id`。
    pub group_key: String,
    /// 分组展示标题。
    pub title: String,
    /// 分组中的 evidence 项。
    #[serde(default)]
    pub items: Vec<PageResearchEvidenceItem>,
}

/// `PageResearchDiagramRollup` 是 research 结果里的图摘要。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchDiagramRollup {
    /// 图稳定键，默认对齐 `PageDiagramInput.diagram_id`。
    pub diagram_key: String,
    /// 图类别，例如 `module_dependency / hierarchy / process`。
    pub diagram_type: String,
    /// 图展示标题。
    pub title: String,
    /// 图的高密度摘要。
    #[serde(default)]
    pub summary: String,
}

/// `PageResearchSectionPlan` 是 research 用来驱动正式 section 结构的稳定对象。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchSectionPlan {
    /// section 稳定 key。
    pub section_key: String,
    /// section 展示标题。
    pub section_title: String,
    /// 该节高密度摘要。
    #[serde(default)]
    pub section_summary: String,
    /// 当前节直接引用的 evidence groups。
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// 当前节直接引用的 deterministic 图输入。
    #[serde(default)]
    pub diagram_refs: Vec<String>,
    /// 当前节直接引用的子页 rollup。
    #[serde(default)]
    pub child_refs: Vec<String>,
}

/// `ChildPageRollup` 表示父页可复用的结构化子页结果。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChildPageRollup {
    /// 子页稳定 ID。
    pub page_id: String,
    /// 子页标题。
    pub title: String,
    /// 子页类型。
    pub page_type: String,
    /// 子页高密度摘要。
    #[serde(default)]
    pub summary: String,
    /// 子页上卷后的 section plan。
    #[serde(default)]
    pub section_plan_rollup: Vec<PageResearchSectionPlan>,
    /// 子页上卷后的关键源码路径。
    #[serde(default)]
    pub key_sources_rollup: Vec<String>,
    /// 子页上卷后的证据分组。
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    /// 子页上卷后的图摘要。
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    /// 子页尚未确认的开放问题。
    #[serde(default)]
    pub open_questions: Vec<String>,
}

/// `RepoDossier` 是 overview / architecture research 前的稳定研究包。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RepoDossier {
    /// dossier 稳定 ID。
    pub dossier_id: String,
    /// dossier 展示标题。
    pub title: String,
    /// 当前仓库研究包的关键源码路径。
    #[serde(default)]
    pub key_sources: Vec<String>,
    /// 当前仓库研究包的关键符号 ID。
    #[serde(default)]
    pub key_symbols: Vec<String>,
    /// 当前仓库研究包的定点片段。
    #[serde(default)]
    pub targeted_snippets: Vec<TargetedSnippet>,
    /// 当前仓库研究包的跨模块关系摘要。
    #[serde(default)]
    pub cross_module_edges: Vec<String>,
    /// 当前仓库研究包的流程候选。
    #[serde(default)]
    pub process_candidates: Vec<String>,
    /// 当前仓库研究包的重点模块。
    #[serde(default)]
    pub module_focus: Vec<String>,
    /// 当前仓库 research 使用的 evidence 上卷。
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    /// 当前仓库 research 使用的图摘要。
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    /// 当前仓库 research 使用的子页上卷。
    #[serde(default)]
    pub child_page_rollup: Vec<ChildPageRollup>,
}

/// `ModuleDossier` 是模块页 research 前的稳定研究包。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleDossier {
    /// dossier 稳定 ID。
    pub dossier_id: String,
    /// 对应模块 ID。
    pub module_id: String,
    /// dossier 展示标题。
    pub title: String,
    /// 当前模块的关键源码路径。
    #[serde(default)]
    pub key_sources: Vec<String>,
    /// 当前模块的关键符号 ID。
    #[serde(default)]
    pub key_symbols: Vec<String>,
    /// 当前模块的定点源码片段。
    #[serde(default)]
    pub targeted_snippets: Vec<TargetedSnippet>,
    /// 当前模块的跨模块关系摘要。
    #[serde(default)]
    pub cross_module_edges: Vec<String>,
    /// 当前模块涉及的流程候选。
    #[serde(default)]
    pub process_candidates: Vec<String>,
    /// 当前模块 evidence 上卷。
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    /// 当前模块 diagram 上卷。
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    /// 当前模块消费的子页上卷。
    #[serde(default)]
    pub child_page_rollup: Vec<ChildPageRollup>,
}

/// `TopicDossier` 是专题页 research 前的稳定研究包。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TopicDossier {
    /// dossier 稳定 ID。
    pub dossier_id: String,
    /// 对应主题稳定键。
    pub topic_key: String,
    /// 主题类别。
    pub topic_kind: String,
    /// dossier 展示标题。
    pub title: String,
    /// 主题摘要。
    #[serde(default)]
    pub summary: String,
    /// 关键源码路径。
    #[serde(default)]
    pub key_sources: Vec<String>,
    /// 关键符号 ID。
    #[serde(default)]
    pub key_symbols: Vec<String>,
    /// 定点源码片段。
    #[serde(default)]
    pub targeted_snippets: Vec<TargetedSnippet>,
    /// evidence 上卷。
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    /// diagram 上卷。
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    /// 子页上卷。
    #[serde(default)]
    pub child_page_rollup: Vec<ChildPageRollup>,
}

/// `PageResearchResult` 固定为结构化研究结果，不承载最终 Markdown。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchResult {
    /// 页面高密度摘要。
    #[serde(default)]
    pub summary: String,
    /// 驱动正式页面结构的稳定 section 计划。
    #[serde(default)]
    pub section_plan: Vec<PageResearchSectionPlan>,
    /// 证据上卷。
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    /// 图摘要上卷。
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    /// 待确认点。
    #[serde(default)]
    pub open_questions: Vec<String>,
}

/// `PageResearchTurn` 是 session 最近轮次里的单条消息。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchTurn {
    /// turn 角色，例如 `system / user / assistant / tool`。
    pub role: String,
    /// turn 摘要。
    #[serde(default)]
    pub content: String,
}

/// `PageToolArtifactRef` 是 session 工具结果的可复用引用。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageToolArtifactRef {
    /// 工具名。
    pub tool_name: String,
    /// artifact 稳定 ID。
    pub artifact_id: String,
    /// artifact 摘要。
    #[serde(default)]
    pub summary: String,
}

/// `PageResearchSessionState` 是 research session 的可复用压缩状态。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageResearchSessionState {
    /// 本次 research session 的稳定 ID。
    pub session_id: String,
    /// 历史会话压缩摘要。
    #[serde(default)]
    pub session_summary: String,
    /// 最近窗口内保留的原始 turn。
    #[serde(default)]
    pub recent_turns: Vec<PageResearchTurn>,
    /// 工具结果引用。
    #[serde(default)]
    pub tool_artifact_refs: Vec<PageToolArtifactRef>,
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
    /// 供父页消费的结构化子页上卷。
    #[serde(default)]
    pub child_rollups: Vec<ChildPageRollup>,
    /// 页面稳定 evidence groups，供 renderer、LLM 和验证脚本共享。
    #[serde(default)]
    pub evidence_groups: Vec<PageEvidenceGroup>,
    /// 页面稳定图输入，供 renderer 直接构造 Mermaid。
    #[serde(default)]
    pub diagram_inputs: Vec<PageDiagramInput>,
    /// 当前页面关联的仓库级 dossier。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_dossier: Option<RepoDossier>,
    /// 当前页面关联的模块级 dossier。
    #[serde(default)]
    pub module_dossiers: Vec<ModuleDossier>,
    /// 当前页面关联的专题 dossier。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_dossier: Option<TopicDossier>,
    /// 当前页面 research session 的结构化结果。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub research_result: Option<PageResearchResult>,
    /// 当前页面 research session 的压缩状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub research_session: Option<PageResearchSessionState>,
}
