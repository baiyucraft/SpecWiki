use serde::{Deserialize, Serialize};

use crate::domain::knowledge::DecompositionProfile;

// ─── ResearchProfile ───────────────────────────────────────

/// Research 层的研究画像。
/// 它与 decomposition profile 对齐，但用于控制 section plan 与 evidence 表达。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchProfile {
    Runtime,
    ApiSurface,
    ConfigSurface,
    DocsGuide,
    Testing,
    ExampleTutorial,
    Troubleshooting,
    IntegrationPlatform,
    CompilerPipeline,
}

impl ResearchProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::ApiSurface => "api_surface",
            Self::ConfigSurface => "config_surface",
            Self::DocsGuide => "docs_guide",
            Self::Testing => "testing",
            Self::ExampleTutorial => "example_tutorial",
            Self::Troubleshooting => "troubleshooting",
            Self::IntegrationPlatform => "integration_platform",
            Self::CompilerPipeline => "compiler_pipeline",
        }
    }
}

/// provider-backed research session 的停止原因。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchStopReason {
    NotRun,
    Completed,
    NoFurtherToolCalls,
    NoMeaningfulDelta,
    TurnBudgetExhausted,
    CallBudgetRejected,
    ProviderError,
    InvalidOutput,
}

impl ResearchStopReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::Completed => "completed",
            Self::NoFurtherToolCalls => "no_further_tool_calls",
            Self::NoMeaningfulDelta => "no_meaningful_delta",
            Self::TurnBudgetExhausted => "turn_budget_exhausted",
            Self::CallBudgetRejected => "call_budget_rejected",
            Self::ProviderError => "provider_error",
            Self::InvalidOutput => "invalid_output",
        }
    }
}

/// provider-backed research session 的观测指标。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResearchSessionStats {
    #[serde(default)]
    pub turns_used: usize,
    #[serde(default)]
    pub tool_calls: usize,
    #[serde(default)]
    pub delta_evidence_count: usize,
    #[serde(default)]
    pub delta_section_count: usize,
    #[serde(default)]
    pub delta_diagram_count: usize,
    #[serde(default)]
    pub child_digest_delta: usize,
    /// 单次 provider-backed research 的总耗时，单位毫秒。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
    /// 当前结果是否直接命中 page research cache。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,
    /// 当前 request 最终实际采用的 tools 模式。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools_mode: Option<String>,
    /// 当前 request 是否在首请求前就应用了 deterministic pre-trim。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_input_applied: Option<bool>,
}

// ─── SystemResearch ─────────────────────────────────────────

/// R1: 全局系统研究结果——从 FactsSnapshot 全量中提取的项目级理解。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemResearch {
    pub project_name: String,
    pub description: String,
    pub project_type: String,
    pub target_users: Vec<String>,
    pub system_boundary: String,
    pub tech_stack: Vec<String>,
    pub architecture_pattern: String,
    pub key_domains: Vec<String>,
    #[serde(default)]
    pub input_hash: String,
}

// ─── DomainResearch ─────────────────────────────────────────

/// R2: 域级研究结果——每个 KnowledgeDomain 一份。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DomainResearch {
    pub domain_id: String,
    pub domain_summary: String,
    pub internal_structure: String,
    pub key_modules: Vec<String>,
    pub key_apis: Vec<String>,
    pub relationships: Vec<String>,
    #[serde(default)]
    pub diagram_suggestion: Option<DiagramSuggestion>,
    #[serde(default)]
    pub input_hash: String,
}

// ─── UnitResearch ───────────────────────────────────────────

/// R3: 单元级研究结果——每个 KnowledgeUnit 一份。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitResearch {
    pub unit_id: String,
    #[serde(default)]
    pub decomposition_profile: Option<DecompositionProfile>,
    #[serde(default)]
    pub research_profile: Option<ResearchProfile>,
    pub positioning: String,
    pub summary: String,
    pub section_plan: Vec<PlannedSection>,
    pub evidence_clusters: Vec<EvidenceCluster>,
    #[serde(default)]
    pub diagram_suggestions: Vec<DiagramSuggestion>,
    #[serde(default)]
    pub key_sources: Vec<String>,
    #[serde(default)]
    pub provider_stop_reason: Option<ResearchStopReason>,
    #[serde(default)]
    pub provider_session_stats: Option<ResearchSessionStats>,
    #[serde(default)]
    pub input_hash: String,
}

// ─── PlannedSection ─────────────────────────────────────────

/// Research 层规划的节标题和预期内容方向。
/// Compose 层消费 PlannedSection 来构造 SectionDraft。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedSection {
    pub section_key: String,
    pub title: String,
    pub intent: String,
    #[serde(default)]
    pub section_summary: String,
    #[serde(default)]
    pub evidence_cluster_keys: Vec<String>,
    #[serde(default)]
    pub child_digest_slot: bool,
    /// 当 section 直接承接 docs/rdb 原始 Markdown 时为 true。
    /// renderer/compose 需要据此保留 reference 章节骨架，避免再补通用 filler。
    #[serde(default)]
    pub preserve_source_markdown: bool,
}

// ─── EvidenceCluster ────────────────────────────────────────

/// 一组相互关联的源码引用，围绕同一个知识点聚合。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCluster {
    pub cluster_key: String,
    pub label: String,
    pub citations: Vec<SourceCitation>,
}

// ─── SourceCitation ─────────────────────────────────────────

/// 单条源码引用——精确到文件路径和行号范围。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceCitation {
    pub path: String,
    pub start_line: usize,
    pub end_line: usize,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub symbol_id: Option<String>,
    #[serde(default)]
    pub note: String,
}

// ─── DiagramSuggestion ──────────────────────────────────────

/// Research 层建议的图表——Compose 层决定是否渲染。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramSuggestion {
    pub diagram_type: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub nodes: Vec<DiagramNodeSuggestion>,
    #[serde(default)]
    pub edges: Vec<DiagramEdgeSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramNodeSuggestion {
    pub node_id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramEdgeSuggestion {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub label: Option<String>,
}

// ─── PageDigest ─────────────────────────────────────────────

/// 页面章节摘要——供父页逐层消费子页的 section 级信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageSectionDigest {
    pub digest_id: String,
    pub section_key: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub key_sources: Vec<String>,
    #[serde(default)]
    pub citations: Vec<SourceCitation>,
}

/// 页面图摘要——供父页感知子页已有图表达。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageDiagramDigest {
    pub digest_id: String,
    pub diagram_type: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
}

/// 页面摘要——子页 compose 完成后产出的精简摘要，供父页消费。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageDigest {
    pub digest_id: String,
    pub unit_id: String,
    pub page_id: String,
    pub title: String,
    #[serde(default)]
    pub decomposition_profile: Option<DecompositionProfile>,
    #[serde(default)]
    pub research_profile: Option<ResearchProfile>,
    pub summary: String,
    #[serde(default)]
    pub key_topics: Vec<String>,
    #[serde(default)]
    pub key_sources: Vec<String>,
    #[serde(default)]
    pub citations: Vec<SourceCitation>,
    #[serde(default)]
    pub section_digests: Vec<PageSectionDigest>,
    #[serde(default)]
    pub diagram_digests: Vec<PageDiagramDigest>,
    #[serde(default)]
    pub readiness_stage: String,
}
