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

/// 把 reference-style 标题归一到稳定集合，避免 research/compose 各自维护一套判断。
pub fn canonical_reference_outline_title(title: &str) -> Option<&'static str> {
    match title.trim() {
        "目录" => Some("目录"),
        "引言" | "简介" => Some("简介"),
        "项目结构" => Some("项目结构"),
        "核心组件" => Some("核心组件"),
        "架构总览" => Some("架构总览"),
        "详细组件分析" => Some("详细组件分析"),
        "依赖分析" | "依赖关系分析" => Some("依赖关系分析"),
        "性能考虑" | "性能考量" => Some("性能考量"),
        "故障排查指南" => Some("故障排查指南"),
        "结论" => Some("结论"),
        "附录" => Some("附录"),
        _ => None,
    }
}

pub fn is_reference_outline_title(title: &str) -> bool {
    canonical_reference_outline_title(title).is_some()
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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

// ─── ResearchPageSeed ──────────────────────────────────────

/// 页面骨架里的单个稳定 section 槽位。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkeletonSection {
    /// section 的稳定身份；compose/renderer 依赖它保持章节对齐。
    pub section_key: String,
    /// 最终 Markdown 里展示的章节标题。
    pub title: String,
}

/// 研究阶段给 compose 提供的稳定页面骨架画像。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkeletonProfile {
    /// 骨架画像标识；供 runtime/report 判断当前页属于哪类 skeleton contract。
    pub profile_key: String,
    #[serde(default)]
    /// research 允许 compose 直接消费的稳定 section 槽位集合。
    pub seed_sections: Vec<SkeletonSection>,
}

/// 关键来源簇，供 compose 在 section 级做显式 grounding。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeySourceCluster {
    /// 来源簇的稳定键；section grounding 通过它回连关键源码集合。
    pub cluster_key: String,
    /// 供 prompt/render/report 使用的人类可读标签。
    pub label: String,
    #[serde(default)]
    /// 本簇绑定的源码路径集合。
    pub source_paths: Vec<String>,
    #[serde(default)]
    /// 与该来源簇关联的 evidence cluster 键。
    pub evidence_cluster_keys: Vec<String>,
}

/// section 级 grounding 合同，显式声明一节消费哪些来源簇、evidence、child digest 和图输入。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionGroundingRef {
    /// 被 grounding 的目标 section。
    pub section_key: String,
    #[serde(default)]
    /// 当前 section 消费的关键来源簇。
    pub key_source_cluster_keys: Vec<String>,
    #[serde(default)]
    /// 当前 section 消费的 evidence clusters。
    pub evidence_cluster_keys: Vec<String>,
    #[serde(default)]
    /// 当前 section 允许吸收的 child digest 引用。
    pub child_digest_refs: Vec<String>,
    #[serde(default)]
    /// 当前 section 可消费的图输入引用。
    pub diagram_refs: Vec<String>,
}

/// 可被 compose 直接消费的 research page seed。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchPageSeed {
    #[serde(default)]
    /// 供最终页面摘要/导语使用的高层概括。
    pub summary: String,
    #[serde(default)]
    /// 页面定位说明；帮助 compose 保持内容边界。
    pub positioning: String,
    #[serde(default)]
    /// research 产出的正式 section plan。
    pub section_plan: Vec<SectionPlan>,
    #[serde(default)]
    /// 稳定骨架画像；用于高层页和 docs-backed 页保持章节身份。
    pub skeleton_profile: Option<SkeletonProfile>,
    #[serde(default)]
    /// compose 必须围绕其展开正文的关键来源簇。
    pub key_source_clusters: Vec<KeySourceCluster>,
    #[serde(default)]
    /// section 级 grounding 合同。
    pub section_grounding_refs: Vec<SectionGroundingRef>,
    #[serde(default)]
    /// 可直接被 compose 消费的证据簇。
    pub evidence_clusters: Vec<EvidenceCluster>,
    #[serde(default)]
    /// research 侧建议的图表达输入。
    pub diagram_suggestions: Vec<DiagramSuggestion>,
}

// ─── SystemResearch ─────────────────────────────────────────

/// R1: 全局系统研究结果——从 FactsSnapshot 全量中提取的项目级理解。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub overview_seed: ResearchPageSeed,
    #[serde(default)]
    pub architecture_seed: ResearchPageSeed,
    #[serde(default)]
    pub input_hash: String,
}

// ─── DomainResearch ─────────────────────────────────────────

/// R2: 域级研究结果——每个 KnowledgeDomain 一份。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub compose_seed: ResearchPageSeed,
    #[serde(default)]
    pub input_hash: String,
}

// ─── UnitResearch ───────────────────────────────────────────

/// R3: 单元级研究结果——每个 KnowledgeUnit 一份。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnitResearch {
    pub unit_id: String,
    #[serde(default)]
    pub decomposition_profile: Option<DecompositionProfile>,
    #[serde(default)]
    pub research_profile: Option<ResearchProfile>,
    pub positioning: String,
    pub summary: String,
    pub section_plan: Vec<SectionPlan>,
    #[serde(default)]
    pub skeleton_profile: Option<SkeletonProfile>,
    #[serde(default)]
    pub key_source_clusters: Vec<KeySourceCluster>,
    #[serde(default)]
    pub section_grounding_refs: Vec<SectionGroundingRef>,
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

impl UnitResearch {
    /// 把 unit research 收敛成可复用于 system/domain/unit compose 的统一 seed。
    pub fn to_seed(&self) -> ResearchPageSeed {
        ResearchPageSeed {
            summary: self.summary.clone(),
            positioning: self.positioning.clone(),
            section_plan: self.section_plan.clone(),
            skeleton_profile: self.skeleton_profile.clone(),
            key_source_clusters: self.key_source_clusters.clone(),
            section_grounding_refs: self.section_grounding_refs.clone(),
            evidence_clusters: self.evidence_clusters.clone(),
            diagram_suggestions: self.diagram_suggestions.clone(),
        }
    }
}

// ─── SectionPlan ─────────────────────────────────────────

/// Research 层规划的节标题和预期内容方向。
/// Compose 层消费 SectionPlan 来构造 SectionDraft。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionPlan {
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceCluster {
    pub cluster_key: String,
    pub label: String,
    pub citations: Vec<SourceCitation>,
}

// ─── SourceCitation ─────────────────────────────────────────

pub use wiki_model::domain::source_citation::SourceCitation;

// ─── DiagramSuggestion ──────────────────────────────────────

/// Research 层建议的图表——Compose 层决定是否渲染。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramSuggestion {
    pub diagram_type: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub nodes: Vec<DiagramNodeSuggestion>,
    #[serde(default)]
    pub edges: Vec<DiagramEdgeSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramNodeSuggestion {
    pub node_id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramEdgeSuggestion {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub label: Option<String>,
}

// ─── PageDigest ─────────────────────────────────────────────

/// 页面章节摘要——供父页逐层消费子页的 section 级信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageDiagramDigest {
    pub digest_id: String,
    pub diagram_type: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
}

/// 页面摘要——子页 compose 完成后产出的精简摘要，供父页消费。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub planned_key_sources: Vec<String>,
    #[serde(default)]
    pub grounded_key_sources: Vec<String>,
    #[serde(default)]
    pub skeleton_profile: Option<SkeletonProfile>,
    #[serde(default)]
    pub section_grounding_refs: Vec<SectionGroundingRef>,
    #[serde(default)]
    pub citations: Vec<SourceCitation>,
    #[serde(default)]
    pub section_digests: Vec<PageSectionDigest>,
    #[serde(default)]
    pub diagram_digests: Vec<PageDiagramDigest>,
    #[serde(default)]
    pub readiness_stage: String,
}


