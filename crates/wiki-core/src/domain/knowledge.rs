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

/// 一个知识单元对应一个最终的 Wiki 页面。
/// KnowledgeUnit 是 compose 层的输入单位，统一取代旧的 topic / family / module 三套页面语义。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeUnit {
    pub id: String,
    pub unit_type: UnitType,
    pub title: String,
    pub domain_id: String,
    pub parent_unit_id: Option<String>,
    #[serde(default)]
    pub child_unit_ids: Vec<String>,
    pub scope: UnitScope,
    pub relative_path: String,
    pub priority: f32,
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
        let id = stable_id("unit", format!("{}:{}:{}", unit_type.as_str(), &domain_id, &title));
        Self {
            id,
            unit_type,
            title,
            domain_id,
            parent_unit_id: None,
            child_unit_ids: Vec::new(),
            scope: UnitScope::default(),
            relative_path,
            priority: 1.0,
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
                for child_id in &unit.child_unit_ids {
                    dfs(child_id, units, order, visited);
                }
            }
            order.push(unit_id.to_string());
        }

        dfs(&self.overview_unit_id.clone(), &self.units, &mut order, &mut visited);

        for unit_id in self.units.keys() {
            if !visited.contains(unit_id) {
                dfs(unit_id, &self.units, &mut order, &mut visited);
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
