use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use wiki_model::domain::knowledge::{DecompositionProfile, KnowledgeUnit};
use wiki_model::domain::knowledge_artifact::{
    KnowledgeResearchStatusReason, KnowledgeResearchStatusReasonKind, KnowledgeResearchSummary,
    KnowledgeResearchSummaryStatus, ProviderFailureKind,
};

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

/// PageResearchEvidenceItem 是 research 结果里的单条 evidence。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageResearchEvidenceItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub start_line: usize,
    #[serde(default)]
    pub end_line: usize,
    #[serde(default)]
    pub evidence_type: String,
    #[serde(default)]
    pub section_refs: Vec<String>,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub coarse_span: bool,
}

/// PageResearchEvidenceGroup 是 research 结果里的证据分组。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageResearchEvidenceGroup {
    pub group_key: String,
    pub title: String,
    #[serde(default)]
    pub items: Vec<PageResearchEvidenceItem>,
}

/// PageResearchDiagramRollup 是 research 结果里的图摘要。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageResearchDiagramRollup {
    pub diagram_key: String,
    pub diagram_type: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
}

/// PageResearchSectionPlan 是 research 用来驱动正式 section 结构的稳定对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageResearchSectionPlan {
    pub section_key: String,
    pub section_title: String,
    #[serde(default)]
    pub section_summary: String,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub diagram_refs: Vec<String>,
    #[serde(default)]
    pub child_refs: Vec<String>,
    #[serde(default)]
    pub child_digest_slot: bool,
}

/// PageResearchResult 固定为结构化研究结果，不承载最终 Markdown。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageResearchResult {
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub page_positioning: String,
    #[serde(default)]
    pub section_plan: Vec<PageResearchSectionPlan>,
    #[serde(default)]
    pub skeleton_profile: Option<SkeletonProfile>,
    #[serde(default)]
    pub key_source_clusters: Vec<KeySourceCluster>,
    #[serde(default)]
    pub section_grounding_refs: Vec<SectionGroundingRef>,
    #[serde(default)]
    pub evidence_rollup: Vec<PageResearchEvidenceGroup>,
    #[serde(default)]
    pub diagram_rollup: Vec<PageResearchDiagramRollup>,
    #[serde(default)]
    pub open_questions: Vec<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_failure_kind: Option<ProviderFailureKind>,
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

    /// 把 unit research 收敛成 `.wiki/.knowledge/**` 可共享的最小摘要。
    pub fn to_artifact_summary(&self, unit: &KnowledgeUnit) -> KnowledgeResearchSummary {
        let source_refs = self
            .key_sources
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let citation_refs = self
            .evidence_clusters
            .iter()
            .flat_map(|cluster| cluster.citations.iter())
            .map(citation_ref_for_summary)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut status_reasons = Vec::new();
        if let Some(stop_reason) = self.provider_stop_reason.as_ref() {
            let reason_kind = status_reason_kind_for_provider_stop(stop_reason);
            if let Some(reason_kind) = reason_kind {
                status_reasons.push(KnowledgeResearchStatusReason {
                    reason_kind: Some(reason_kind),
                    reason_message: format!(
                        "provider-backed research stop reason: {}",
                        stop_reason.as_str()
                    ),
                    upstream_ref: Some(format!("provider_stop_reason:{}", stop_reason.as_str())),
                });
            }
        }
        if status_reasons.is_empty() && self.summary.trim().is_empty() {
            status_reasons.push(KnowledgeResearchStatusReason {
                reason_kind: Some(KnowledgeResearchStatusReasonKind::MissingSummary),
                reason_message: "research summary 为空".to_string(),
                upstream_ref: None,
            });
        }
        if status_reasons.is_empty() && source_refs.is_empty() {
            status_reasons.push(KnowledgeResearchStatusReason {
                reason_kind: Some(KnowledgeResearchStatusReasonKind::MissingSourceRefs),
                reason_message: "research summary 缺少 source refs".to_string(),
                upstream_ref: None,
            });
        }
        if status_reasons.is_empty() && citation_refs.is_empty() {
            status_reasons.push(KnowledgeResearchStatusReason {
                reason_kind: Some(KnowledgeResearchStatusReasonKind::MissingCitationRefs),
                reason_message: "research summary 缺少 citation refs".to_string(),
                upstream_ref: None,
            });
        }
        let summary_status = if status_reasons.iter().any(|reason| {
            reason.reason_kind.is_some_and(|kind| {
                kind.expected_status() == KnowledgeResearchSummaryStatus::Blocked
            })
        }) {
            KnowledgeResearchSummaryStatus::Blocked
        } else if status_reasons.is_empty() {
            KnowledgeResearchSummaryStatus::Ready
        } else {
            KnowledgeResearchSummaryStatus::Degraded
        };
        let mut summary = KnowledgeResearchSummary {
            unit_id: unit.id.clone(),
            unit_type: unit.unit_type.as_str().to_string(),
            title: unit.title.clone(),
            summary: self.summary.clone(),
            positioning: self.positioning.clone(),
            input_hash: self.input_hash.clone(),
            key_sources: self.key_sources.clone(),
            source_refs,
            citation_refs,
            summary_status,
            provider_failure_kind: self.provider_failure_kind,
            status_reasons,
        };
        summary.canonicalize();
        summary
    }
}

fn status_reason_kind_for_provider_stop(
    stop_reason: &ResearchStopReason,
) -> Option<KnowledgeResearchStatusReasonKind> {
    match stop_reason {
        ResearchStopReason::ProviderError => Some(KnowledgeResearchStatusReasonKind::ProviderError),
        ResearchStopReason::InvalidOutput => Some(KnowledgeResearchStatusReasonKind::InvalidOutput),
        ResearchStopReason::CallBudgetRejected => {
            Some(KnowledgeResearchStatusReasonKind::CallBudgetRejected)
        }
        ResearchStopReason::NotRun => Some(KnowledgeResearchStatusReasonKind::ProviderNotRun),
        ResearchStopReason::NoMeaningfulDelta => {
            Some(KnowledgeResearchStatusReasonKind::NoMeaningfulDelta)
        }
        ResearchStopReason::TurnBudgetExhausted => {
            Some(KnowledgeResearchStatusReasonKind::TurnBudgetExhausted)
        }
        ResearchStopReason::Completed | ResearchStopReason::NoFurtherToolCalls => None,
    }
}

fn citation_ref_for_summary(citation: &SourceCitation) -> String {
    citation.source_id.clone().unwrap_or_else(|| {
        format!(
            "{}:{}-{}",
            citation.path, citation.start_line, citation.end_line
        )
    })
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

/// `ProjectionDigestStatus` 是 PageDigest 当前可被 formal runtime 消费的投影状态。
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionDigestStatus {
    #[default]
    Ready,
    Stale,
    Blocked,
}

impl ProjectionDigestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }
}

/// `ProjectionDigestStatusReasonKind` 收敛当前 formal projection 允许暴露的最小原因集合。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionDigestStatusReasonKind {
    MissingPageOutput,
    PageSnapshotMismatch,
    DeclaredOrDerivedChanged,
    BlockedByResearch,
}

impl ProjectionDigestStatusReasonKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingPageOutput => "missing_page_output",
            Self::PageSnapshotMismatch => "page_snapshot_mismatch",
            Self::DeclaredOrDerivedChanged => "declared_or_derived_changed",
            Self::BlockedByResearch => "blocked_by_research",
        }
    }

    pub fn expected_status(self) -> ProjectionDigestStatus {
        match self {
            Self::MissingPageOutput
            | Self::PageSnapshotMismatch
            | Self::DeclaredOrDerivedChanged => ProjectionDigestStatus::Stale,
            Self::BlockedByResearch => ProjectionDigestStatus::Blocked,
        }
    }
}

/// `ProjectionDigestStatusReason` 是 projection digest 的最小 machine-readable 原因对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionDigestStatusReason {
    pub reason_kind: Option<ProjectionDigestStatusReasonKind>,
    #[serde(default)]
    pub reason_message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
}

impl ProjectionDigestStatusReason {
    pub fn canonicalize(&mut self) {
        self.reason_message = self.reason_message.trim().to_string();
        self.upstream_ref = self
            .upstream_ref
            .as_ref()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }
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
    pub projection_status: ProjectionDigestStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reasons: Vec<ProjectionDigestStatusReason>,
    #[serde(default)]
    pub readiness_stage: String,
}

impl PageDigest {
    pub fn canonicalize(&mut self) {
        self.digest_id = self.digest_id.trim().to_string();
        self.unit_id = self.unit_id.trim().to_string();
        self.page_id = self.page_id.trim().to_string();
        self.title = self.title.trim().to_string();
        self.summary = self.summary.trim().to_string();
        self.key_topics = self
            .key_topics
            .iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        self.key_sources = sorted_unique_strings(&self.key_sources);
        self.planned_key_sources = sorted_unique_strings(&self.planned_key_sources);
        self.grounded_key_sources = sorted_unique_strings(&self.grounded_key_sources);
        self.readiness_stage = self.readiness_stage.trim().to_string();
        for reason in &mut self.status_reasons {
            reason.canonicalize();
        }
        self.status_reasons.sort_by(|left, right| {
            (
                left.reason_kind
                    .map(|value| value.as_str())
                    .unwrap_or_default(),
                left.reason_message.as_str(),
                left.upstream_ref.as_deref().unwrap_or_default(),
            )
                .cmp(&(
                    right
                        .reason_kind
                        .map(|value| value.as_str())
                        .unwrap_or_default(),
                    right.reason_message.as_str(),
                    right.upstream_ref.as_deref().unwrap_or_default(),
                ))
        });
        self.status_reasons.dedup_by(|left, right| left == right);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.digest_id.is_empty() {
            return Err("page digest 缺少 digest_id".to_string());
        }
        if self.unit_id.is_empty() {
            return Err(format!("page digest '{}' 缺少 unit_id", self.digest_id));
        }
        if self.page_id.is_empty() {
            return Err(format!("page digest '{}' 缺少 page_id", self.digest_id));
        }
        if self.title.is_empty() {
            return Err(format!("page digest '{}' 缺少 title", self.digest_id));
        }

        match self.projection_status {
            ProjectionDigestStatus::Ready => {
                if !self.status_reasons.is_empty() {
                    return Err(format!(
                        "page digest '{}' 为 ready 时不允许携带 status_reasons",
                        self.digest_id
                    ));
                }
            }
            ProjectionDigestStatus::Stale | ProjectionDigestStatus::Blocked => {
                if self.status_reasons.is_empty() {
                    return Err(format!(
                        "page digest '{}' 为 {} 时必须携带至少一条 status_reason",
                        self.digest_id,
                        self.projection_status.as_str()
                    ));
                }
            }
        }

        for reason in &self.status_reasons {
            let Some(reason_kind) = reason.reason_kind else {
                return Err(format!(
                    "page digest '{}' 的 status_reason 缺少 reason_kind",
                    self.digest_id
                ));
            };
            if reason.reason_message.is_empty() {
                return Err(format!(
                    "page digest '{}' 的 status_reason '{}' 缺少 reason_message",
                    self.digest_id,
                    reason_kind.as_str()
                ));
            }
            if reason_kind.expected_status() != self.projection_status {
                return Err(format!(
                    "page digest '{}' 的 status_reason '{}' 与 projection_status '{}' 不一致",
                    self.digest_id,
                    reason_kind.as_str(),
                    self.projection_status.as_str()
                ));
            }
        }

        Ok(())
    }
}

pub fn validate_page_digest_snapshot(digests: &[PageDigest]) -> Result<(), String> {
    let mut seen_digest_ids = BTreeSet::new();
    let mut seen_unit_ids = BTreeSet::new();
    let mut seen_page_ids = BTreeSet::new();

    for digest in digests {
        let mut normalized = digest.clone();
        normalized.canonicalize();
        normalized.validate()?;
        if !seen_digest_ids.insert(normalized.digest_id.clone()) {
            return Err(format!(
                "page digest digest_id 冲突: '{}'",
                normalized.digest_id
            ));
        }
        if !seen_unit_ids.insert(normalized.unit_id.clone()) {
            return Err(format!(
                "page digest unit_id 冲突: '{}'",
                normalized.unit_id
            ));
        }
        if !seen_page_ids.insert(normalized.page_id.clone()) {
            return Err(format!(
                "page digest page_id 冲突: '{}'",
                normalized.page_id
            ));
        }
    }

    Ok(())
}

fn sorted_unique_strings(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiki_model::domain::knowledge::{KnowledgeUnit, UnitType};
    use wiki_model::domain::knowledge_artifact::{
        KnowledgeResearchStatusReasonKind, KnowledgeResearchSummaryStatus,
    };

    #[test]
    fn artifact_summary_carries_provenance_and_status() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            summary: "负责核心运行时调度。".to_string(),
            positioning: "解释运行时主入口与阶段职责。".to_string(),
            key_sources: vec!["src/runtime.rs".to_string(), "src/runtime.rs".to_string()],
            evidence_clusters: vec![EvidenceCluster {
                cluster_key: "runtime".to_string(),
                label: "运行时".to_string(),
                citations: vec![SourceCitation {
                    path: "src/runtime.rs".to_string(),
                    start_line: 10,
                    end_line: 24,
                    source_id: Some("source-runtime".to_string()),
                    symbol_id: None,
                    note: "主入口".to_string(),
                }],
            }],
            input_hash: "research-hash".to_string(),
            ..UnitResearch::default()
        };

        let summary = research.to_artifact_summary(&unit);

        assert_eq!(summary.unit_id, unit.id);
        assert_eq!(summary.source_refs, vec!["src/runtime.rs".to_string()]);
        assert_eq!(summary.citation_refs, vec!["source-runtime".to_string()]);
        assert_eq!(
            summary.summary_status,
            KnowledgeResearchSummaryStatus::Ready
        );
        assert!(summary.status_reasons.is_empty());
    }

    #[test]
    fn artifact_summary_marks_missing_summary_as_degraded() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            key_sources: vec!["src/runtime.rs".to_string()],
            evidence_clusters: vec![EvidenceCluster {
                cluster_key: "runtime".to_string(),
                label: "运行时".to_string(),
                citations: vec![SourceCitation {
                    path: "src/runtime.rs".to_string(),
                    start_line: 10,
                    end_line: 24,
                    source_id: Some("source-runtime".to_string()),
                    symbol_id: None,
                    note: "主入口".to_string(),
                }],
            }],
            ..UnitResearch::default()
        };

        let summary = research.to_artifact_summary(&unit);

        assert_eq!(
            summary.summary_status,
            KnowledgeResearchSummaryStatus::Degraded
        );
        assert_eq!(
            summary.status_reasons[0].reason_kind,
            Some(KnowledgeResearchStatusReasonKind::MissingSummary)
        );
    }

    #[test]
    fn artifact_summary_marks_provider_error_as_blocked() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            summary: "provider 失败".to_string(),
            key_sources: vec!["src/runtime.rs".to_string()],
            provider_stop_reason: Some(ResearchStopReason::ProviderError),
            ..UnitResearch::default()
        };

        let summary = research.to_artifact_summary(&unit);

        assert_eq!(
            summary.summary_status,
            KnowledgeResearchSummaryStatus::Blocked
        );
        assert_eq!(
            summary.status_reasons[0].reason_kind,
            Some(KnowledgeResearchStatusReasonKind::ProviderError)
        );
    }

    #[test]
    fn artifact_summary_blocks_every_provider_stop_without_valid_output() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        for stop_reason in [
            ResearchStopReason::NotRun,
            ResearchStopReason::NoMeaningfulDelta,
            ResearchStopReason::TurnBudgetExhausted,
            ResearchStopReason::CallBudgetRejected,
            ResearchStopReason::ProviderError,
            ResearchStopReason::InvalidOutput,
        ] {
            let research = UnitResearch {
                unit_id: unit.id.clone(),
                summary: "仅有 structural seed".to_string(),
                positioning: "没有有效 provider output".to_string(),
                key_sources: vec!["src/runtime.rs".to_string()],
                evidence_clusters: vec![EvidenceCluster {
                    cluster_key: "runtime".to_string(),
                    label: "运行时".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/runtime.rs".to_string(),
                        start_line: 1,
                        end_line: 2,
                        source_id: Some("source-runtime".to_string()),
                        symbol_id: None,
                        note: "structural".to_string(),
                    }],
                }],
                provider_stop_reason: Some(stop_reason.clone()),
                ..UnitResearch::default()
            };

            assert_eq!(
                research.to_artifact_summary(&unit).summary_status,
                KnowledgeResearchSummaryStatus::Blocked,
                "{} must not be accepted without provider output",
                stop_reason.as_str()
            );
        }
    }

    #[test]
    fn page_digest_ready_disallows_status_reasons() {
        let mut digest = PageDigest {
            digest_id: "digest-runtime".to_string(),
            unit_id: "unit-runtime".to_string(),
            page_id: "page-runtime".to_string(),
            title: "运行时".to_string(),
            projection_status: ProjectionDigestStatus::Ready,
            status_reasons: vec![ProjectionDigestStatusReason {
                reason_kind: Some(ProjectionDigestStatusReasonKind::MissingPageOutput),
                reason_message: "页面还没写出".to_string(),
                upstream_ref: None,
            }],
            ..PageDigest::default()
        };

        digest.canonicalize();

        assert!(digest.validate().is_err());
    }

    #[test]
    fn page_digest_snapshot_requires_unique_unit_ids() {
        let digest = PageDigest {
            digest_id: "digest-runtime".to_string(),
            unit_id: "unit-runtime".to_string(),
            page_id: "page-runtime".to_string(),
            title: "运行时".to_string(),
            projection_status: ProjectionDigestStatus::Stale,
            status_reasons: vec![ProjectionDigestStatusReason {
                reason_kind: Some(ProjectionDigestStatusReasonKind::MissingPageOutput),
                reason_message: "页面还没写出".to_string(),
                upstream_ref: None,
            }],
            readiness_stage: "research_ready".to_string(),
            ..PageDigest::default()
        };

        assert!(validate_page_digest_snapshot(&[digest.clone(), digest]).is_err());
    }
}
