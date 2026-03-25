//! `projection` 定义 KnowledgeTree 到页面投影决策的稳定合同。
//! 这里产出的 `PlannedPage` 属于 knowledge 侧，不代表 runtime 持久化真相。

use serde::{Deserialize, Serialize};
use wiki_model::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use wiki_model::domain::stable_id::stable_id;

/// `PlannedPage` 描述“要生成什么页面”的稳定投影决策。
/// 它只负责页面结构和依赖范围，不承载 runtime 写盘状态。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlannedPage {
    /// 页面稳定 ID，会被状态层、缓存层和关系层引用。
    pub id: String,
    /// 页面标题，最终渲染为一级标题。
    pub title: String,
    /// 页面在 `.wiki/` 下的稳定相对路径。
    pub relative_path: String,
    /// 页面类型，如 `overview / architecture / domain-index / module / concept-guide`。
    pub page_type: String,
    /// 父页面 ID，用于恢复页面树结构。
    pub parent_id: Option<String>,
    /// 页面作用域标签，帮助 runtime 区分 repository / domain / unit 页面。
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
    /// 页面直接依赖的源码 ID 集合。
    pub source_ids: Vec<String>,
    /// 页面直接映射到的模块 ID 集合。
    pub module_ids: Vec<String>,
    /// 页面直接依赖的关系 ID 集合。
    pub relation_ids: Vec<String>,
    /// 当前页面采用的生成模式标记，主要用于调试和后续扩展。
    pub generation_mode: String,
    /// 页面规划优先级，供后续稳定排序。
    pub priority: usize,
    /// 被合并到本页面的子模块 ID 集合。
    #[serde(default)]
    pub merged_module_ids: Vec<String>,
}

/// 从 KnowledgeTree 生成稳定页面投影列表。
/// 这是 KnowledgeUnit 主线上的正式 projection decision 入口。
pub fn plan_pages_from_knowledge_tree(tree: &KnowledgeTree) -> Vec<PlannedPage> {
    tree.processing_order
        .iter()
        .filter_map(|unit_id| tree.get_unit(unit_id))
        .map(|unit| knowledge_unit_to_planned_page(unit, tree))
        .collect()
}

fn knowledge_unit_to_planned_page(unit: &KnowledgeUnit, tree: &KnowledgeTree) -> PlannedPage {
    let page_type = unit_type_to_page_type(&unit.unit_type);
    let scope = unit_type_to_scope(&unit.unit_type);

    let parent_id = unit.parent_unit_id.as_ref().and_then(|parent_unit_id| {
        tree.get_unit(parent_unit_id)
            .map(|parent| stable_id("page", &parent.relative_path))
    });

    PlannedPage {
        id: stable_id("page", &unit.relative_path),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        page_type: page_type.to_string(),
        parent_id,
        scope: scope.to_string(),
        source_ids: unit.scope.source_ids.clone(),
        module_ids: unit.scope.module_ids.clone(),
        relation_ids: unit.scope.relation_ids.clone(),
        generation_mode: "knowledge-tree".to_string(),
        priority: (unit.priority * 100.0) as usize,
        merged_module_ids: Vec::new(),
        unit_id: Some(unit.id.clone()),
        unit_type: Some(unit.unit_type.as_str().to_string()),
        domain_id: Some(unit.domain_id.clone()),
    }
}

fn unit_type_to_page_type(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview => "overview",
        UnitType::Architecture => "architecture",
        UnitType::DomainIndex => "domain-index",
        UnitType::ModuleDoc => "module",
        UnitType::ApiDoc => "api-reference",
        UnitType::ConfigDoc => "config-reference",
        UnitType::ConceptGuide => "concept-guide",
        UnitType::WorkflowDoc => "workflow",
        UnitType::TroubleshootDoc => "troubleshoot",
        UnitType::IntegrationDoc => "integration",
        UnitType::TestDoc => "test-doc",
        UnitType::ExampleDoc => "example",
    }
}

fn unit_type_to_scope(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview | UnitType::Architecture => "repository",
        UnitType::DomainIndex => "domain",
        _ => "unit",
    }
}
