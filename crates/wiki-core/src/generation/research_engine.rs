use std::collections::BTreeMap;
use std::io;

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{KnowledgeDomain, KnowledgeTree, KnowledgeUnit};
use crate::domain::module_tree::ModuleTree;
use crate::domain::research::{
    DomainResearch, EvidenceCluster, PageDigest, PlannedSection, SourceCitation, SystemResearch,
    UnitResearch,
};
use crate::domain::stable_id::stable_id;
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::GraphSummary;

// ─── ResearchDataSource ─────────────────────────────────────

/// Research 层的统一数据源——把 Facts 层输出打包成研究引擎的输入。
pub struct ResearchDataSource<'a> {
    pub report: &'a ScanReport,
    pub module_tree: &'a ModuleTree,
    pub repo_context: &'a RepoContext,
    pub module_contexts: &'a [ModuleContext],
    pub graph_summary: &'a GraphSummary,
    pub knowledge_tree: &'a KnowledgeTree,
}

// ─── ResearchProvider trait ─────────────────────────────────

/// 三层 research 的统一入口。
/// 生产环境由 LlmRuntime 实现（LLM-required），测试可用 StructuralResearchProvider 替代。
pub trait ResearchProvider {
    fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch>;

    fn research_domain(
        &self,
        domain: &KnowledgeDomain,
        ds: &ResearchDataSource,
    ) -> io::Result<DomainResearch>;

    fn research_unit(
        &self,
        unit: &KnowledgeUnit,
        ds: &ResearchDataSource,
        child_digests: &[PageDigest],
    ) -> io::Result<UnitResearch>;
}

// ─── StructuralResearchProvider ─────────────────────────────

/// 纯结构分析的 research 实现——仅从 Facts 数据推导，不调用 LLM。
/// 用于单元测试和开发阶段；生产环境应使用 LLM-backed 实现。
pub struct StructuralResearchProvider;

impl ResearchProvider for StructuralResearchProvider {
    fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
        Ok(research_system_structural(ds))
    }

    fn research_domain(
        &self,
        domain: &KnowledgeDomain,
        ds: &ResearchDataSource,
    ) -> io::Result<DomainResearch> {
        Ok(research_domain_structural(domain, ds))
    }

    fn research_unit(
        &self,
        unit: &KnowledgeUnit,
        ds: &ResearchDataSource,
        child_digests: &[PageDigest],
    ) -> io::Result<UnitResearch> {
        Ok(research_unit_structural(unit, ds, child_digests))
    }
}

// ─── Structural implementations ─────────────────────────────

fn research_system_structural(ds: &ResearchDataSource) -> SystemResearch {
    let project_name = ds
        .report
        .root
        .rsplit(['/', '\\'])
        .find(|s| !s.is_empty())
        .unwrap_or("project")
        .to_string();

    let tech_stack = ds.repo_context.tech_stack.clone();
    let architecture_pattern = if ds.module_tree.modules.len() > 10 {
        "monorepo / multi-module".to_string()
    } else if ds.module_tree.modules.len() > 3 {
        "multi-module".to_string()
    } else {
        "single-module".to_string()
    };

    let key_domains: Vec<String> = ds
        .knowledge_tree
        .domains
        .values()
        .map(|d| d.label.clone())
        .collect();

    SystemResearch {
        project_name,
        description: format!(
            "包含 {} 个文件、{} 个模块的项目",
            ds.report.files.len(),
            ds.module_tree
                .modules
                .iter()
                .filter(|m| m.parent_id.is_some())
                .count()
        ),
        project_type: infer_project_type(ds),
        target_users: vec!["开发者".to_string()],
        system_boundary: format!("仓库根路径: {}", ds.report.root),
        tech_stack,
        architecture_pattern,
        key_domains,
        input_hash: String::new(),
    }
}

fn research_domain_structural(
    domain: &KnowledgeDomain,
    ds: &ResearchDataSource,
) -> DomainResearch {
    let module_context_index: BTreeMap<&str, &ModuleContext> = ds
        .module_contexts
        .iter()
        .map(|c| (c.module_id.as_str(), c))
        .collect();

    let key_modules: Vec<String> = domain
        .source_modules
        .iter()
        .filter_map(|mid| ds.module_tree.module_by_id(mid))
        .map(|m| m.name.clone())
        .collect();

    let key_apis: Vec<String> = domain
        .evidence
        .public_api_surfaces
        .iter()
        .flat_map(|s| s.exported_symbols.iter().cloned())
        .take(10)
        .collect();

    let relationships: Vec<String> = domain
        .source_modules
        .iter()
        .filter_map(|mid| module_context_index.get(mid.as_str()))
        .flat_map(|ctx| {
            ctx.dependencies
                .iter()
                .map(|dep| format!("依赖 → {dep}"))
                .chain(ctx.dependents.iter().map(|dep| format!("被依赖 ← {dep}")))
        })
        .take(10)
        .collect();

    let internal_structure = if key_modules.len() > 1 {
        format!(
            "域内包含 {} 个模块: {}",
            key_modules.len(),
            key_modules.join(", ")
        )
    } else {
        "域内包含单一模块".to_string()
    };

    DomainResearch {
        domain_id: domain.id.clone(),
        domain_summary: format!(
            "{}（{}）：包含 {} 个模块",
            domain.label,
            domain.domain_type.as_str(),
            domain.source_modules.len()
        ),
        internal_structure,
        key_modules,
        key_apis,
        relationships,
        diagram_suggestion: None,
        input_hash: String::new(),
    }
}

fn research_unit_structural(
    unit: &KnowledgeUnit,
    ds: &ResearchDataSource,
    child_digests: &[PageDigest],
) -> UnitResearch {
    let module_context_index: BTreeMap<&str, &ModuleContext> = ds
        .module_contexts
        .iter()
        .map(|c| (c.module_id.as_str(), c))
        .collect();

    let positioning = format!("{}（类型: {}）", unit.title, unit.unit_type.as_str());

    let key_sources: Vec<String> = unit
        .scope
        .module_ids
        .iter()
        .filter_map(|mid| module_context_index.get(mid.as_str()))
        .flat_map(|ctx| ctx.key_sources.iter().cloned())
        .take(8)
        .collect();

    let summary = if !child_digests.is_empty() {
        format!("{} 包含 {} 个子单元", unit.title, child_digests.len())
    } else {
        format!(
            "{} 覆盖 {} 个模块、{} 个源文件",
            unit.title,
            unit.scope.module_ids.len(),
            unit.scope.source_ids.len()
        )
    };

    let mut section_plan = build_default_section_plan(unit, child_digests);
    let evidence_clusters = build_evidence_clusters_from_scope(unit, ds, &key_sources);

    if let Some(first_section) = section_plan.first_mut() {
        first_section.evidence_cluster_keys = evidence_clusters
            .iter()
            .map(|c| c.cluster_key.clone())
            .collect();
    }

    UnitResearch {
        unit_id: unit.id.clone(),
        positioning,
        summary,
        section_plan,
        evidence_clusters,
        diagram_suggestions: Vec::new(),
        key_sources,
        input_hash: String::new(),
    }
}

fn build_default_section_plan(
    unit: &KnowledgeUnit,
    child_digests: &[PageDigest],
) -> Vec<PlannedSection> {
    let mut sections = Vec::new();

    sections.push(PlannedSection {
        section_key: "overview".to_string(),
        title: "概述".to_string(),
        intent: format!("简要说明 {} 的定位、职责和核心功能", unit.title),
        evidence_cluster_keys: Vec::new(),
        child_digest_slot: false,
    });

    match unit.unit_type {
        crate::domain::knowledge::UnitType::Overview => {
            sections.push(PlannedSection {
                section_key: "tech-stack".to_string(),
                title: "技术栈".to_string(),
                intent: "列出项目使用的主要技术、语言和框架".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
            sections.push(PlannedSection {
                section_key: "key-modules".to_string(),
                title: "核心模块".to_string(),
                intent: "概述项目的主要模块和它们的职责".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: true,
            });
        }
        crate::domain::knowledge::UnitType::Architecture => {
            sections.push(PlannedSection {
                section_key: "module-structure".to_string(),
                title: "模块结构".to_string(),
                intent: "描述系统的整体模块结构和层级关系".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
            sections.push(PlannedSection {
                section_key: "dependencies".to_string(),
                title: "依赖关系".to_string(),
                intent: "说明模块间的关键依赖关系".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
        }
        crate::domain::knowledge::UnitType::DomainIndex => {
            sections.push(PlannedSection {
                section_key: "domain-units".to_string(),
                title: "域内单元".to_string(),
                intent: "列出该知识域下的所有文档页面".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: true,
            });
        }
        crate::domain::knowledge::UnitType::ModuleDoc => {
            sections.push(PlannedSection {
                section_key: "key-sources".to_string(),
                title: "关键源码".to_string(),
                intent: "列出最能代表该模块职责的关键源码文件".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
            sections.push(PlannedSection {
                section_key: "api-surface".to_string(),
                title: "公共接口".to_string(),
                intent: "描述该模块暴露的主要 API 和导出符号".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
            sections.push(PlannedSection {
                section_key: "dependencies".to_string(),
                title: "依赖关系".to_string(),
                intent: "说明该模块与其他模块的依赖关系".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
        }
        crate::domain::knowledge::UnitType::ApiDoc => {
            sections.push(PlannedSection {
                section_key: "api-overview".to_string(),
                title: "API 概览".to_string(),
                intent: "列出该模块的公共 API 及其用途".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
        }
        _ => {
            sections.push(PlannedSection {
                section_key: "details".to_string(),
                title: "详细说明".to_string(),
                intent: format!("详细说明 {} 的内容和使用方式", unit.title),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
            });
        }
    }

    if !child_digests.is_empty() && !sections.iter().any(|s| s.child_digest_slot) {
        sections.push(PlannedSection {
            section_key: "sub-units".to_string(),
            title: "子单元".to_string(),
            intent: "汇总子单元的摘要信息".to_string(),
            evidence_cluster_keys: Vec::new(),
            child_digest_slot: true,
        });
    }

    sections
}

fn build_evidence_clusters_from_scope(
    unit: &KnowledgeUnit,
    ds: &ResearchDataSource,
    key_sources: &[String],
) -> Vec<EvidenceCluster> {
    let mut clusters = Vec::new();

    if !key_sources.is_empty() {
        let citations: Vec<SourceCitation> = key_sources
            .iter()
            .take(8)
            .map(|path| {
                let source_id = ds
                    .report
                    .files
                    .iter()
                    .find(|f| f.path == *path)
                    .map(|f| f.id.clone());
                SourceCitation {
                    path: path.clone(),
                    start_line: 1,
                    end_line: 24,
                    source_id,
                    symbol_id: None,
                    note: String::new(),
                }
            })
            .collect();

        clusters.push(EvidenceCluster {
            cluster_key: stable_id("evidence-cluster", &format!("{}:key-sources", unit.id)),
            label: "关键源码".to_string(),
            citations,
        });
    }

    if !unit.scope.api_surfaces.is_empty() {
        let citations: Vec<SourceCitation> = unit
            .scope
            .api_surfaces
            .iter()
            .flat_map(|api| {
                api.exported_symbols
                    .iter()
                    .take(3)
                    .map(|sym| SourceCitation {
                        path: sym.clone(),
                        start_line: 1,
                        end_line: 1,
                        source_id: None,
                        symbol_id: Some(sym.clone()),
                        note: "导出符号".to_string(),
                    })
            })
            .take(6)
            .collect();

        if !citations.is_empty() {
            clusters.push(EvidenceCluster {
                cluster_key: stable_id("evidence-cluster", &format!("{}:api-surface", unit.id)),
                label: "公共 API".to_string(),
                citations,
            });
        }
    }

    clusters
}

fn infer_project_type(ds: &ResearchDataSource) -> String {
    let techs = ds.repo_context.tech_stack.join(" ").to_ascii_lowercase();
    if techs.contains("react") || techs.contains("vue") || techs.contains("angular") {
        "前端应用/组件库".to_string()
    } else if techs.contains("spring") || techs.contains("django") || techs.contains("express") {
        "后端服务".to_string()
    } else if techs.contains("rust") && techs.contains("cli") {
        "CLI 工具".to_string()
    } else if ds.module_tree.modules.len() > 15 {
        "大型多模块项目".to_string()
    } else {
        "软件项目".to_string()
    }
}
