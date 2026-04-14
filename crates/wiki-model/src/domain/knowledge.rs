use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::domain::stable_id::stable_id;

// ─── DomainType ─────────────────────────────────────────────

/// 知识域类型，决定域内知识单元的拆分策略。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainType {
    CoreRuntime,
    Framework,
    PlatformBinding,
    CompilerToolchain,
    PluginEcosystem,
    TestingInfra,
    BuildSystem,

    ConceptGuide,
    ApiReference,
    ConfigReference,
    Troubleshooting,

    MultiFramework,
    ThemeSystem,
    DevTooling,
}

impl DomainType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CoreRuntime => "core_runtime",
            Self::Framework => "framework",
            Self::PlatformBinding => "platform_binding",
            Self::CompilerToolchain => "compiler_toolchain",
            Self::PluginEcosystem => "plugin_ecosystem",
            Self::TestingInfra => "testing_infra",
            Self::BuildSystem => "build_system",
            Self::ConceptGuide => "concept_guide",
            Self::ApiReference => "api_reference",
            Self::ConfigReference => "config_reference",
            Self::Troubleshooting => "troubleshooting",
            Self::MultiFramework => "multi_framework",
            Self::ThemeSystem => "theme_system",
            Self::DevTooling => "dev_tooling",
        }
    }
}

// ─── UnitType ───────────────────────────────────────────────

/// 知识单元类型，统一取代旧的 topic / family / module 三套页面语义。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitType {
    Overview,
    Architecture,
    DomainIndex,
    ModuleDoc,
    ApiDoc,
    ConfigDoc,
    ConceptGuide,
    WorkflowDoc,
    TroubleshootDoc,
    IntegrationDoc,
    TestDoc,
    ExampleDoc,
}

impl UnitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Architecture => "architecture",
            Self::DomainIndex => "domain_index",
            Self::ModuleDoc => "module_doc",
            Self::ApiDoc => "api_doc",
            Self::ConfigDoc => "config_doc",
            Self::ConceptGuide => "concept_guide",
            Self::WorkflowDoc => "workflow_doc",
            Self::TroubleshootDoc => "troubleshoot_doc",
            Self::IntegrationDoc => "integration_doc",
            Self::TestDoc => "test_doc",
            Self::ExampleDoc => "example_doc",
        }
    }
}

// ─── DecompositionProfile ──────────────────────────────────

/// 知识单元的中粒度拆分画像。
/// 它描述 planner 是按哪类信号把当前单元从域内拆出来。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecompositionProfile {
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

impl DecompositionProfile {
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

// ─── Planner Diagnostics ───────────────────────────────────

/// planner 信号类型，用来区分仓库原型信号、surface 聚类与叶子拆分策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerSignalKind {
    RepoArchetype,
    SurfaceCluster,
    LeafDecomposition,
}

/// 一个可诊断的 planner 信号束，描述某个单元为何被拆出或保留。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerSignalBundle {
    pub kind: PlannerSignalKind,
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub matched_keywords: Vec<String>,
    #[serde(default)]
    pub matched_source_ids: Vec<String>,
    #[serde(default)]
    pub matched_paths: Vec<String>,
}

/// collapse guard 的触发原因，用来解释为何候选被收回父页或聚合页。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollapseGuardReason {
    RawConfigSurfaceAggregation,
    TopicScopeRefinement,
}

/// 记录一次显式的折叠决策，既保留被吸收候选，也保留最终保留下来的候选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseGuardDecision {
    pub reason: CollapseGuardReason,
    pub owner_title: String,
    #[serde(default)]
    pub collapsed_source_ids: Vec<String>,
    #[serde(default)]
    pub collapsed_paths: Vec<String>,
    #[serde(default)]
    pub preserved_source_ids: Vec<String>,
    #[serde(default)]
    pub preserved_paths: Vec<String>,
}

// ─── DomainEvidence ─────────────────────────────────────────

/// 知识域被发现时的证据，记录哪些模块/文件/锚点触发了该域的识别。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DomainEvidence {
    pub matched_modules: Vec<String>,
    pub matched_file_patterns: Vec<String>,
    #[serde(default)]
    pub docs_anchors: Vec<DocsAnchor>,
    #[serde(default)]
    pub config_surfaces: Vec<ConfigSurface>,
    #[serde(default)]
    pub public_api_surfaces: Vec<ApiSurface>,
}

/// 文档结构锚点——从 .md / README 提取的标题层级和链接。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsAnchor {
    pub file_path: String,
    pub heading: String,
    pub level: u32,
    #[serde(default)]
    pub links: Vec<String>,
}

/// 配置入口表面——配置文件中提取的顶层键或结构摘要。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSurface {
    pub file_path: String,
    pub keys: Vec<String>,
}

/// 公共 API 表面——模块导出符号的摘要。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSurface {
    pub module_id: String,
    pub exported_symbols: Vec<String>,
}

// ─── KnowledgeDomain ────────────────────────────────────────

/// 一个知识域代表仓库中一块独立的知识领域。
/// 知识域从 Facts 层的 ModuleTree + SymbolGraph + ScanReport 中确定性发现，
/// 每个域下辖多个 KnowledgeUnit。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDomain {
    pub id: String,
    pub domain_type: DomainType,
    pub label: String,
    pub evidence: DomainEvidence,
    pub source_modules: Vec<String>,
    pub source_files: Vec<String>,
    pub estimated_depth: u32,
}

impl KnowledgeDomain {
    pub fn new(domain_type: DomainType, label: impl Into<String>) -> Self {
        let label = label.into();
        let id = stable_id("domain", format!("{}:{}", domain_type.as_str(), &label));
        Self {
            id,
            domain_type,
            label,
            evidence: DomainEvidence::default(),
            source_modules: Vec::new(),
            source_files: Vec::new(),
            estimated_depth: 1,
        }
    }
}

// ─── UnitScope ──────────────────────────────────────────────

/// 知识单元的输入范围——标识该单元覆盖哪些模块/文件/符号/文档/API/配置。
/// 统一取代旧的 RepoDossier / ModuleDossier / TopicDossier / FamilyDossier。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitScope {
    pub module_ids: Vec<String>,
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub symbol_ids: Vec<String>,
    #[serde(default)]
    pub relation_ids: Vec<String>,
    #[serde(default)]
    pub docs_anchors: Vec<DocsAnchor>,
    #[serde(default)]
    pub api_surfaces: Vec<ApiSurface>,
    #[serde(default)]
    pub config_surfaces: Vec<ConfigSurface>,
}

// ─── KnowledgeUnit ──────────────────────────────────────────

/// `KnowledgeUnitStatus` 表示 knowledge runtime 视角下该单元的最小正式状态。
/// 它不等价于页面渲染状态，也不直接表达索引状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeUnitStatus {
    #[default]
    Active,
    Stale,
    Blocked,
    Removed,
}

/// 一个知识单元对应一个最终的 Wiki 页面。
/// KnowledgeUnit 是 compose 层的输入单位，统一取代旧的 topic / family / module 三套页面语义。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeUnit {
    pub id: String,
    pub unit_type: UnitType,
    #[serde(default)]
    pub decomposition_profile: Option<DecompositionProfile>,
    pub title: String,
    pub domain_id: String,
    pub parent_unit_id: Option<String>,
    #[serde(default)]
    pub child_unit_ids: Vec<String>,
    #[serde(default)]
    pub planner_signal_bundles: Vec<PlannerSignalBundle>,
    #[serde(default)]
    pub collapse_guard: Option<CollapseGuardDecision>,
    pub scope: UnitScope,
    pub relative_path: String,
    pub priority: f32,
    #[serde(default)]
    pub declared_record_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_research_ref: Option<String>,
    #[serde(default)]
    pub projection_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub citation_refs: Vec<String>,
    #[serde(default)]
    pub status: KnowledgeUnitStatus,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidation_reason: Option<String>,
}

impl KnowledgeUnit {
    pub fn new(
        unit_type: UnitType,
        title: impl Into<String>,
        domain_id: impl Into<String>,
        relative_path: impl Into<String>,
    ) -> Self {
        let title = title.into();
        let domain_id = domain_id.into();
        let relative_path = relative_path.into();
        let id = stable_id(
            "unit",
            format!("{}:{}:{}", unit_type.as_str(), &domain_id, &title),
        );
        Self {
            id,
            unit_type,
            decomposition_profile: None,
            title,
            domain_id,
            parent_unit_id: None,
            child_unit_ids: Vec::new(),
            planner_signal_bundles: Vec::new(),
            collapse_guard: None,
            scope: UnitScope::default(),
            relative_path,
            priority: 1.0,
            declared_record_refs: Vec::new(),
            derived_research_ref: None,
            projection_refs: Vec::new(),
            source_refs: Vec::new(),
            citation_refs: Vec::new(),
            status: KnowledgeUnitStatus::Active,
            updated_at: String::new(),
            invalidation_reason: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.child_unit_ids.is_empty()
    }
}

// ─── KnowledgeTree ──────────────────────────────────────────

/// 知识树是 Knowledge Planning 层的最终输出。
/// 包含所有域、所有单元，以及叶子优先的处理顺序。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTree {
    pub overview_unit_id: String,
    pub units: BTreeMap<String, KnowledgeUnit>,
    pub domains: BTreeMap<String, KnowledgeDomain>,
    /// 叶子优先的处理顺序——DFS 先子后父。
    pub processing_order: Vec<String>,
}

impl KnowledgeTree {
    pub fn new(overview_unit_id: impl Into<String>) -> Self {
        Self {
            overview_unit_id: overview_unit_id.into(),
            units: BTreeMap::new(),
            domains: BTreeMap::new(),
            processing_order: Vec::new(),
        }
    }

    pub fn add_domain(&mut self, domain: KnowledgeDomain) {
        self.domains.insert(domain.id.clone(), domain);
    }

    pub fn add_unit(&mut self, unit: KnowledgeUnit) {
        self.units.insert(unit.id.clone(), unit);
    }

    pub fn get_unit(&self, id: &str) -> Option<&KnowledgeUnit> {
        self.units.get(id)
    }

    pub fn get_domain(&self, id: &str) -> Option<&KnowledgeDomain> {
        self.domains.get(id)
    }

    /// 构建叶子优先的处理顺序：DFS 先收集子单元再加入当前单元。
    pub fn build_processing_order(&mut self) {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();

        fn dfs(
            unit_id: &str,
            units: &BTreeMap<String, KnowledgeUnit>,
            order: &mut Vec<String>,
            visited: &mut std::collections::HashSet<String>,
        ) {
            if visited.contains(unit_id) {
                return;
            }
            visited.insert(unit_id.to_string());

            if let Some(unit) = units.get(unit_id) {
                let mut child_ids = unit.child_unit_ids.clone();
                child_ids.sort_by(|left, right| compare_unit_priority(right, left, units));
                for child_id in child_ids {
                    dfs(&child_id, units, order, visited);
                }
            }
            order.push(unit_id.to_string());
        }

        dfs(
            &self.overview_unit_id.clone(),
            &self.units,
            &mut order,
            &mut visited,
        );

        let mut remaining_unit_ids = self.units.keys().cloned().collect::<Vec<_>>();
        remaining_unit_ids.sort_by(|left, right| compare_unit_priority(right, left, &self.units));
        for unit_id in remaining_unit_ids {
            if !visited.contains(unit_id.as_str()) {
                dfs(unit_id.as_str(), &self.units, &mut order, &mut visited);
            }
        }

        self.processing_order = order;
    }

    pub fn leaf_units(&self) -> Vec<&KnowledgeUnit> {
        self.units.values().filter(|u| u.is_leaf()).collect()
    }

    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }
}

fn compare_unit_priority(
    left_id: &str,
    right_id: &str,
    units: &BTreeMap<String, KnowledgeUnit>,
) -> std::cmp::Ordering {
    let left_priority = units
        .get(left_id)
        .map(|unit| unit.priority)
        .unwrap_or_default();
    let right_priority = units
        .get(right_id)
        .map(|unit| unit.priority)
        .unwrap_or_default();
    left_priority
        .partial_cmp(&right_priority)
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| left_id.cmp(right_id))
}

#[cfg(test)]
mod tests {
    use super::{KnowledgeTree, KnowledgeUnit, UnitType};
    use std::collections::BTreeMap;

    #[test]
    fn processing_order_prefers_high_priority_leaves_before_parents() {
        let mut tree = KnowledgeTree::new("overview".to_string());

        let mut overview =
            KnowledgeUnit::new(UnitType::Overview, "项目概述", "system", "项目概述.md");
        overview.id = "overview".to_string();

        let mut low = KnowledgeUnit::new(UnitType::ModuleDoc, "low", "domain", "low.md");
        low.id = "low".to_string();
        low.priority = 0.2;
        low.parent_unit_id = Some(overview.id.clone());

        let mut high = KnowledgeUnit::new(UnitType::ModuleDoc, "high", "domain", "high.md");
        high.id = "high".to_string();
        high.priority = 0.9;
        high.parent_unit_id = Some(overview.id.clone());

        let mut high_leaf = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "high-leaf",
            "domain",
            "high-leaf.md",
        );
        high_leaf.id = "high-leaf".to_string();
        high_leaf.priority = 1.0;
        high_leaf.parent_unit_id = Some(high.id.clone());

        overview.child_unit_ids = vec![low.id.clone(), high.id.clone()];
        high.child_unit_ids = vec![high_leaf.id.clone()];

        tree.units = BTreeMap::from([
            (overview.id.clone(), overview),
            (low.id.clone(), low),
            (high.id.clone(), high),
            (high_leaf.id.clone(), high_leaf),
        ]);

        tree.build_processing_order();

        assert_eq!(
            tree.processing_order,
            vec![
                "high-leaf".to_string(),
                "high".to_string(),
                "low".to_string(),
                "overview".to_string(),
            ]
        );
    }
}
