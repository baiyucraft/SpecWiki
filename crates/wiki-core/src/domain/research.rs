use serde::{Deserialize, Serialize};

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
    pub positioning: String,
    pub summary: String,
    pub section_plan: Vec<PlannedSection>,
    pub evidence_clusters: Vec<EvidenceCluster>,
    #[serde(default)]
    pub diagram_suggestions: Vec<DiagramSuggestion>,
    #[serde(default)]
    pub key_sources: Vec<String>,
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
    pub evidence_cluster_keys: Vec<String>,
    #[serde(default)]
    pub child_digest_slot: bool,
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

/// 页面摘要——子页 compose 完成后产出的精简摘要，供父页消费。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDigest {
    pub unit_id: String,
    pub page_id: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub key_topics: Vec<String>,
    #[serde(default)]
    pub key_sources: Vec<String>,
}
