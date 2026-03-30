use serde::{Deserialize, Serialize};

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

/// `ResearchSurface` 表示 dossier 中的非源码 surface 输入。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResearchSurface {
    /// surface 稳定 ID。
    pub surface_id: String,
    /// surface 类别，例如 `docs-anchor / public-api / config / type`。
    pub surface_type: String,
    /// 对应源码稳定 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// 对应路径。
    pub path: String,
    /// 展示标题。
    #[serde(default)]
    pub title: String,
    /// 对应锚点或命名线索。
    #[serde(default)]
    pub anchor: String,
    /// 摘要说明。
    #[serde(default)]
    pub summary: String,
}

/// `PageComposeSection` 是 renderer 真正消费的章节计划。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageComposeSection {
    /// section 稳定 key。
    pub section_key: String,
    /// section 展示标题。
    pub section_title: String,
    /// section 最终待渲染正文。
    #[serde(default)]
    pub content: String,
    /// 当前节引用的 evidence groups。
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// 当前节引用的 deterministic 图输入。
    #[serde(default)]
    pub diagram_refs: Vec<String>,
    /// 当前节引用的子页结果。
    #[serde(default)]
    pub child_refs: Vec<String>,
}

/// `PageComposePlan` 是 renderer 前的显式 compose 结果。
/// 新 pipeline 已由 `PageDraft` + compose 层取代，此类型仅保留用于序列化兼容。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageComposePlan {
    #[serde(default)]
    pub page_positioning: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub sections: Vec<PageComposeSection>,
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

/// `PageContext` 是最终传给渲染器的统一输入。
/// 到这一层以后，页面生成只需要关心“如何表达”，不用再重新做结构分析。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageContext {
    /// 当前页面的稳定 ID。
    pub page_id: String,
    /// 当前页面类型，例如 `overview / module`。
    pub page_type: String,
    /// 页面作用域标签。
    pub scope: String,
    /// 当前页面直接对应的知识单元 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 当前页面直接对应的知识单元类型。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// 当前页面归属的知识域 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
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
    /// parent page 直接消费的子 KnowledgeUnit IDs。
    #[serde(default)]
    pub child_unit_ids: Vec<String>,
    /// parent page 直接消费的子页面 IDs。
    #[serde(default)]
    pub child_page_ids: Vec<String>,
    /// parent page 直接消费的子页 digest IDs。
    #[serde(default)]
    pub child_digest_ids: Vec<String>,
    /// 当前页面的 parent compose contract readiness 状态。
    #[serde(default)]
    pub readiness_status: String,
    /// 当前页面引用的 citation digest 摘要 IDs。
    #[serde(default)]
    pub citation_digest_refs: Vec<String>,
    /// 当前页面引用的 diagram digest 摘要 IDs。
    #[serde(default)]
    pub diagram_digest_refs: Vec<String>,
    /// 当前页面是否已经拥有自己的 unit-scoped research contract。
    #[serde(default)]
    pub has_unit_research_contract: bool,
    /// 对应 unit research 的输入哈希；帮助区分真实 parent research 与 seed-only 页面。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_research_input_hash: Option<String>,
    /// 当前 parent contract 仍缺失的直接 child unit IDs。
    #[serde(default)]
    pub missing_child_unit_ids: Vec<String>,
    /// 页面稳定 evidence groups，供 renderer、LLM 和验证脚本共享。
    #[serde(default)]
    pub evidence_groups: Vec<PageEvidenceGroup>,
    /// 页面稳定图输入，供 renderer 直接构造 Mermaid。
    #[serde(default)]
    pub diagram_inputs: Vec<PageDiagramInput>,
}
