//! `query` 收口跨 runtime、transport 和宿主消费的稳定查询对象语言。
//! 它只定义公开 DTO 与闭集枚举，不承载 runtime 私有装配逻辑。

use serde::{Deserialize, Serialize};

/// Query 结果的公开 route 分类。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum QueryRouteTag {
    IndexSymbolHit,
    IndexPathHit,
    IndexModuleHit,
    IndexGraphHit,
    KnowledgeDeclaredHit,
    KnowledgeDerivedHit,
    GovernanceEvidenceRef,
    GovernanceSummaryHit,
    ProjectionRef,
    RenderedPageDebugFallback,
}

impl QueryRouteTag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IndexSymbolHit => "index_symbol_hit",
            Self::IndexPathHit => "index_path_hit",
            Self::IndexModuleHit => "index_module_hit",
            Self::IndexGraphHit => "index_graph_hit",
            Self::KnowledgeDeclaredHit => "knowledge_declared_hit",
            Self::KnowledgeDerivedHit => "knowledge_derived_hit",
            Self::GovernanceEvidenceRef => "governance_evidence_ref",
            Self::GovernanceSummaryHit => "governance_summary_hit",
            Self::ProjectionRef => "projection_ref",
            Self::RenderedPageDebugFallback => "rendered_page_debug_fallback",
        }
    }

    pub fn is_index_route(self) -> bool {
        matches!(
            self,
            Self::IndexSymbolHit | Self::IndexPathHit | Self::IndexModuleHit | Self::IndexGraphHit
        )
    }

    pub fn is_governance_route(self) -> bool {
        matches!(
            self,
            Self::GovernanceEvidenceRef | Self::GovernanceSummaryHit
        )
    }
}

/// Query result 指向的对象类型。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum QueryRefKind {
    SourcePath,
    SourceSymbol,
    SourceModule,
    IndexGraphEdge,
    KnowledgePage,
    KnowledgeRecord,
    ProjectionPage,
    ProjectionSection,
    GovernanceChange,
    GovernanceArtifact,
    RenderedPage,
}

impl QueryRefKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourcePath => "source_path",
            Self::SourceSymbol => "source_symbol",
            Self::SourceModule => "source_module",
            Self::IndexGraphEdge => "index_graph_edge",
            Self::KnowledgePage => "knowledge_page",
            Self::KnowledgeRecord => "knowledge_record",
            Self::ProjectionPage => "projection_page",
            Self::ProjectionSection => "projection_section",
            Self::GovernanceChange => "governance_change",
            Self::GovernanceArtifact => "governance_artifact",
            Self::RenderedPage => "rendered_page",
        }
    }
}

/// 单条 query result 的可信度。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum QueryConfidence {
    High,
    Medium,
    Low,
}

impl QueryConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// 单条 result 的建议动作。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
    None,
    OpenReference,
    OpenSourceRef,
    OpenKnowledgeRef,
    OpenProjectionRef,
    ReviewGovernance,
    RebuildIndex,
    UpdateKnowledge,
    Rebuild,
    Update,
    Sync,
}

impl RecommendedAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OpenReference => "open_reference",
            Self::OpenSourceRef => "open_source_ref",
            Self::OpenKnowledgeRef => "open_knowledge_ref",
            Self::OpenProjectionRef => "open_projection_ref",
            Self::ReviewGovernance => "review_governance",
            Self::RebuildIndex => "rebuild_index",
            Self::UpdateKnowledge => "update_knowledge",
            Self::Rebuild => "rebuild",
            Self::Update => "update",
            Self::Sync => "sync",
        }
    }
}

/// Query result 所属的正式数据层。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryProvenanceLayer {
    Index,
    Knowledge,
    Governance,
    Projection,
    Fallback,
}

/// Query result 所属数据层在查询时的真实可用状态。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryProvenanceState {
    Ready,
    Stale,
    Missing,
    Rebuilding,
    Conflict,
    Blocked,
    NotEnabled,
    Fallback,
}

/// Route group 的排序依据；只在组内有意义。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryRankingBasis {
    Bm25,
    StructuralMatch,
    GraphConfidence,
    DeterministicMatch,
}

/// Route-local score 的相关性方向。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryScoreDirection {
    LowerIsBetter,
    HigherIsBetter,
    None,
}

/// Query result 的 provenance 摘要对象。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QueryProvenance {
    pub layer: QueryProvenanceLayer,
    pub state: QueryProvenanceState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Query result 引用的来源对象。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QuerySourceRef {
    pub ref_kind: QueryRefKind,
    pub ref_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<String>,
}

/// 单条公开 query 命中结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QueryResultDto {
    pub route_tag: QueryRouteTag,
    pub ref_kind: QueryRefKind,
    pub ref_id: String,
    pub label: String,
    pub rank: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    pub provenance: QueryProvenance,
    pub confidence: QueryConfidence,
    pub recommended_action: RecommendedAction,
    #[serde(default)]
    pub source_refs: Vec<QuerySourceRef>,
}

/// 按 route 分组的 query 结果。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QueryRouteGroup {
    pub route_tag: QueryRouteTag,
    pub ranking_basis: QueryRankingBasis,
    pub score_direction: QueryScoreDirection,
    pub total_count: usize,
    pub returned_count: usize,
    pub truncated: bool,
    #[serde(default)]
    pub results: Vec<QueryResultDto>,
}
