use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use regex::Regex;

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{
    DecompositionProfile, KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitType,
};
use crate::domain::module_tree::ModuleTree;
use crate::domain::research::{
    DomainResearch, EvidenceCluster, PageDigest, PlannedSection, ResearchProfile, SourceCitation,
    SystemResearch, UnitResearch,
};
use crate::domain::stable_id::stable_id;
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
use crate::repo::symbols::ParsedSymbolsSnapshot;

// ─── ResearchDataSource ─────────────────────────────────────

/// Research 层的统一数据源——把 Facts 层输出打包成研究引擎的输入。
pub struct ResearchDataSource<'a> {
    pub report: &'a ScanReport,
    pub module_tree: &'a ModuleTree,
    pub repo_context: &'a RepoContext,
    pub module_contexts: &'a [ModuleContext],
    pub symbol_snapshot: &'a ParsedSymbolsSnapshot,
    pub resolved_graph: &'a ResolvedGraphSnapshot,
    pub graph_analysis: &'a GraphAnalysisSnapshot,
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

fn research_domain_structural(domain: &KnowledgeDomain, ds: &ResearchDataSource) -> DomainResearch {
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
    let doc_reference_citations = collect_doc_reference_citations(unit, ds);

    let mut key_sources: Vec<String> = unit
        .scope
        .module_ids
        .iter()
        .filter_map(|mid| module_context_index.get(mid.as_str()))
        .flat_map(|ctx| ctx.key_sources.iter().cloned())
        .take(8)
        .collect();
    key_sources.extend(
        doc_reference_citations
            .iter()
            .map(|citation| citation.path.clone())
            .take(8),
    );
    key_sources.extend(
        unit.scope
            .docs_anchors
            .iter()
            .map(|anchor| anchor.file_path.clone())
            .chain(
                unit.scope
                    .config_surfaces
                    .iter()
                    .map(|surface| surface.file_path.clone()),
            )
            .take(8),
    );
    dedup_preserving_order(&mut key_sources);

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

    let research_profile = infer_research_profile(unit);
    let mut section_plan =
        build_docs_backed_section_plan(unit, ds, research_profile.as_ref(), child_digests)
            .unwrap_or_else(|| {
                build_default_section_plan(unit, research_profile.as_ref(), child_digests)
            });
    let evidence_clusters =
        build_evidence_clusters_from_scope(unit, ds, &key_sources, &doc_reference_citations);
    seed_section_evidence_clusters(&mut section_plan, &evidence_clusters);

    if let Some(first_section) = section_plan.first_mut() {
        if !(first_section.intent.trim().is_empty() && !unit.scope.docs_anchors.is_empty()) {
            let mut overview_parts = Vec::new();
            if !positioning.trim().is_empty() {
                overview_parts.push(positioning.clone());
            }
            if !summary.trim().is_empty() {
                overview_parts.push(summary.clone());
            }
            if !first_section.section_summary.trim().is_empty() {
                overview_parts.push(first_section.section_summary.clone());
            }
            first_section.section_summary = overview_parts.join("\n\n");
        }
        first_section.evidence_cluster_keys = evidence_clusters
            .iter()
            .map(|c| c.cluster_key.clone())
            .collect();
    }

    UnitResearch {
        unit_id: unit.id.clone(),
        decomposition_profile: unit.decomposition_profile.clone(),
        research_profile,
        positioning,
        summary,
        section_plan,
        evidence_clusters,
        diagram_suggestions: Vec::new(),
        key_sources,
        provider_stop_reason: None,
        provider_session_stats: None,
        input_hash: String::new(),
    }
}

fn build_docs_backed_section_plan(
    unit: &KnowledgeUnit,
    ds: &ResearchDataSource,
    research_profile: Option<&ResearchProfile>,
    child_digests: &[PageDigest],
) -> Option<Vec<PlannedSection>> {
    let primary_doc = unit.scope.docs_anchors.first()?.file_path.clone();
    let doc_path =
        Path::new(&ds.report.root).join(primary_doc.replace('/', std::path::MAIN_SEPARATOR_STR));
    let content = fs::read_to_string(doc_path).ok()?;
    let normalized_content = strip_markdown_frontmatter(&content);
    let (lead, sections) = parse_markdown_sections(&normalized_content);

    if lead.trim().is_empty() && sections.len() < 2 {
        return None;
    }

    if !should_preserve_docs_markdown(unit, &sections) {
        return Some(build_reference_docs_section_plan(
            unit,
            research_profile,
            child_digests,
            &lead,
            &sections,
        ));
    }

    let mut plan = Vec::new();
    if !lead.trim().is_empty() {
        plan.push(PlannedSection {
            section_key: "preamble".to_string(),
            title: String::new(),
            intent: String::new(),
            section_summary: lead.trim().to_string(),
            evidence_cluster_keys: Vec::new(),
            child_digest_slot: false,
            preserve_source_markdown: true,
        });
    }

    for (index, (title, body)) in sections.into_iter().take(10).enumerate() {
        if body.trim().is_empty() {
            continue;
        }
        plan.push(PlannedSection {
            section_key: stable_id("section-plan", format!("{}:{index}:{title}", unit.id)),
            title,
            intent: String::new(),
            section_summary: body,
            evidence_cluster_keys: Vec::new(),
            child_digest_slot: false,
            preserve_source_markdown: true,
        });
    }

    if !child_digests.is_empty() && !plan.iter().any(|section| section.child_digest_slot) {
        plan.push(build_planned_section(
            unit,
            "sub-units",
            "子单元",
            "汇总子单元的摘要信息",
            true,
        ));
    }

    (!plan.is_empty()).then_some(plan)
}

fn should_preserve_docs_markdown(unit: &KnowledgeUnit, sections: &[(String, String)]) -> bool {
    let Some(anchor) = unit.scope.docs_anchors.first() else {
        return false;
    };
    if docs_path_is_localized(&anchor.file_path) {
        return true;
    }

    let reference_headings = sections
        .iter()
        .filter(|(title, _)| is_reference_outline_heading(title))
        .count();
    reference_headings >= 3
}

fn build_reference_docs_section_plan(
    unit: &KnowledgeUnit,
    research_profile: Option<&ResearchProfile>,
    child_digests: &[PageDigest],
    lead: &str,
    sections: &[(String, String)],
) -> Vec<PlannedSection> {
    let mut plan = vec![
        PlannedSection {
            section_key: "preamble".to_string(),
            title: String::new(),
            intent: String::new(),
            section_summary: String::new(),
            evidence_cluster_keys: Vec::new(),
            child_digest_slot: false,
            preserve_source_markdown: false,
        },
        PlannedSection {
            section_key: "toc".to_string(),
            title: "目录".to_string(),
            intent: "给出本页章节导航".to_string(),
            section_summary: String::new(),
            evidence_cluster_keys: Vec::new(),
            child_digest_slot: false,
            preserve_source_markdown: false,
        },
    ];

    let raw_outline = sections
        .iter()
        .map(|(title, _)| title.clone())
        .take(6)
        .collect::<Vec<_>>();
    let lead_summary = summarize_doc_lead(lead);
    let outline_summary = if raw_outline.is_empty() {
        String::new()
    } else {
        format!("原始文档重点覆盖：{}。", raw_outline.join("、"))
    };

    plan.extend([
        build_reference_section(
            "intro",
            "简介",
            match research_profile {
                Some(ResearchProfile::ApiSurface) => {
                    format!("说明 {} 对外能力、适用范围和阅读入口", unit.title)
                }
                Some(ResearchProfile::ConfigSurface) => {
                    format!("说明 {} 的配置目标、适用场景和使用前提", unit.title)
                }
                Some(ResearchProfile::Troubleshooting) => {
                    format!("说明 {} 聚焦的问题域、典型症状和排查入口", unit.title)
                }
                _ => format!("说明 {} 的定位、范围和核心问题", unit.title),
            },
            lead_summary,
        ),
        build_reference_section(
            "structure",
            "项目结构",
            "概述相关目录、入口文件与实现落点".to_string(),
            String::new(),
        ),
        build_reference_section(
            "components",
            "核心组件",
            "总结主流程涉及的关键模块、类型、接口或配置面".to_string(),
            outline_summary.clone(),
        ),
        build_reference_section(
            "architecture",
            "架构总览",
            "串联调用链、数据流和模块边界".to_string(),
            String::new(),
        ),
        build_reference_section(
            "detailed-analysis",
            "详细组件分析",
            "分解关键组件、关键章节和实现职责".to_string(),
            outline_summary,
        ),
        build_reference_section(
            "dependencies",
            "依赖关系分析",
            "说明对内对外依赖、调用关系和耦合点".to_string(),
            String::new(),
        ),
        build_reference_section(
            "performance",
            "性能考量",
            "说明性能影响点、构建成本或运行时开销".to_string(),
            String::new(),
        ),
        build_reference_section(
            "troubleshooting",
            "故障排查指南",
            "给出常见问题、定位入口和修复路径".to_string(),
            String::new(),
        ),
        build_reference_section(
            "conclusion",
            "结论",
            "总结本页覆盖的主结论和推荐阅读顺序".to_string(),
            String::new(),
        ),
        build_reference_section(
            "appendix",
            "附录",
            "补充原始章节、额外参考和扩展材料".to_string(),
            String::new(),
        ),
    ]);

    if !child_digests.is_empty() {
        plan.push(build_planned_section(
            unit,
            "sub-units",
            "子单元",
            "汇总子单元的摘要信息",
            true,
        ));
    }

    plan
}

fn build_reference_section(
    section_key: &str,
    title: &str,
    intent: String,
    section_summary: String,
) -> PlannedSection {
    PlannedSection {
        section_key: section_key.to_string(),
        title: title.to_string(),
        intent,
        section_summary,
        evidence_cluster_keys: Vec::new(),
        child_digest_slot: false,
        preserve_source_markdown: false,
    }
}

fn summarize_doc_lead(lead: &str) -> String {
    lead.lines()
        .map(str::trim)
        .filter(|line| {
            !line.is_empty()
                && !line.starts_with("[![")
                && !line.starts_with("![")
                && !line.starts_with("<img")
        })
        .take(3)
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_reference_outline_heading(title: &str) -> bool {
    matches!(
        title.trim(),
        "目录"
            | "简介"
            | "项目结构"
            | "核心组件"
            | "架构总览"
            | "详细组件分析"
            | "依赖关系分析"
            | "性能考量"
            | "故障排查指南"
            | "结论"
            | "附录"
    )
}

fn docs_path_is_localized(path: &str) -> bool {
    path.chars().any(|character| !character.is_ascii())
}

fn strip_markdown_frontmatter(content: &str) -> String {
    let mut lines = content.lines();
    if lines.next().map(str::trim) != Some("---") {
        return content.to_string();
    }

    let mut remaining = Vec::new();
    let mut in_frontmatter = true;
    for line in lines {
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            continue;
        }
        if !in_frontmatter {
            remaining.push(line);
        }
    }

    if in_frontmatter {
        content.to_string()
    } else {
        remaining.join("\n")
    }
}

fn parse_markdown_sections(content: &str) -> (String, Vec<(String, String)>) {
    let mut lead_lines = Vec::new();
    let mut sections = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_lines = Vec::new();

    for line in content.lines() {
        if let Some(title) = line.strip_prefix("## ").map(str::trim) {
            if let Some(existing_title) = current_title.take() {
                sections.push((existing_title, current_lines.join("\n").trim().to_string()));
                current_lines.clear();
            }
            current_title = Some(title.to_string());
            continue;
        }

        if current_title.is_some() {
            current_lines.push(line.to_string());
        } else if !line.starts_with("# ") {
            lead_lines.push(line.to_string());
        }
    }

    if let Some(existing_title) = current_title.take() {
        sections.push((existing_title, current_lines.join("\n").trim().to_string()));
    }

    let lead = lead_lines.join("\n").trim().to_string();
    (lead, sections)
}

fn build_default_section_plan(
    unit: &KnowledgeUnit,
    research_profile: Option<&ResearchProfile>,
    child_digests: &[PageDigest],
) -> Vec<PlannedSection> {
    if let Some(profile) = research_profile {
        let mut sections = build_profile_section_plan(unit, profile);
        if !child_digests.is_empty() && !sections.iter().any(|s| s.child_digest_slot) {
            sections.push(build_planned_section(
                unit,
                "sub-units",
                "子单元",
                "汇总子单元的摘要信息",
                true,
            ));
        }
        return sections;
    }

    let mut sections = Vec::new();

    sections.push(build_planned_section(
        unit,
        "overview",
        "概述",
        format!("简要说明 {} 的定位、职责和核心功能", unit.title),
        false,
    ));

    match unit.unit_type {
        UnitType::Overview => {
            sections.push(build_planned_section(
                unit,
                "tech-stack",
                "技术栈",
                "列出项目使用的主要技术、语言和框架",
                false,
            ));
            sections.push(build_planned_section(
                unit,
                "key-modules",
                "核心模块",
                "概述项目的主要模块和它们的职责",
                true,
            ));
        }
        UnitType::Architecture => {
            sections.push(build_planned_section(
                unit,
                "module-structure",
                "模块结构",
                "描述系统的整体模块结构和层级关系",
                false,
            ));
            sections.push(build_planned_section(
                unit,
                "dependencies",
                "依赖关系",
                "说明模块间的关键依赖关系",
                false,
            ));
        }
        UnitType::DomainIndex => {
            sections.push(build_planned_section(
                unit,
                "domain-units",
                "域内单元",
                "列出该知识域下的所有文档页面",
                true,
            ));
        }
        UnitType::ModuleDoc => {
            sections.push(build_planned_section(
                unit,
                "key-sources",
                "关键源码",
                "列出最能代表该模块职责的关键源码文件",
                false,
            ));
            sections.push(build_planned_section(
                unit,
                "api-surface",
                "公共接口",
                "描述该模块暴露的主要 API 和导出符号",
                false,
            ));
            sections.push(build_planned_section(
                unit,
                "dependencies",
                "依赖关系",
                "说明该模块与其他模块的依赖关系",
                false,
            ));
        }
        UnitType::ApiDoc => {
            sections.push(build_planned_section(
                unit,
                "api-overview",
                "API 概览",
                "列出该模块的公共 API 及其用途",
                false,
            ));
        }
        _ => {
            sections.push(build_planned_section(
                unit,
                "details",
                "详细说明",
                format!("详细说明 {} 的内容和使用方式", unit.title),
                false,
            ));
        }
    }

    if !child_digests.is_empty() && !sections.iter().any(|s| s.child_digest_slot) {
        sections.push(build_planned_section(
            unit,
            "sub-units",
            "子单元",
            "汇总子单元的摘要信息",
            true,
        ));
    }

    sections
}

fn build_profile_section_plan(
    unit: &KnowledgeUnit,
    profile: &ResearchProfile,
) -> Vec<PlannedSection> {
    let overview_intent = match profile {
        ResearchProfile::Runtime => format!("说明 {} 的运行时职责、边界和关键组件", unit.title),
        ResearchProfile::ApiSurface => format!("说明 {} 暴露的 API 面与调用入口", unit.title),
        ResearchProfile::ConfigSurface => {
            format!("说明 {} 的配置入口、关键键和生效方式", unit.title)
        }
        ResearchProfile::DocsGuide => format!("说明 {} 的核心概念、使用前提和适用范围", unit.title),
        ResearchProfile::Testing => format!("说明 {} 的测试范围、关键用例和验证目标", unit.title),
        ResearchProfile::ExampleTutorial => {
            format!("说明 {} 的快速开始、示例路径和上手顺序", unit.title)
        }
        ResearchProfile::Troubleshooting => {
            format!("说明 {} 的典型问题、排查路径和修复线索", unit.title)
        }
        ResearchProfile::IntegrationPlatform => {
            format!("说明 {} 的集成对象、接入方式和平台约束", unit.title)
        }
        ResearchProfile::CompilerPipeline => {
            format!("说明 {} 的编译阶段、处理流程和产物边界", unit.title)
        }
    };

    let mut sections = vec![PlannedSection {
        section_key: "overview".to_string(),
        title: "概述".to_string(),
        section_summary: structural_section_summary(unit, "概述", &overview_intent, false),
        intent: overview_intent,
        evidence_cluster_keys: Vec::new(),
        child_digest_slot: false,
        preserve_source_markdown: false,
    }];

    let profile_sections = match profile {
        ResearchProfile::Runtime => vec![
            (
                "runtime-responsibility",
                "运行时职责",
                "解释运行时主流程、状态或执行入口",
            ),
            (
                "key-sources",
                "关键源码",
                "列出最核心的运行时实现文件和入口",
            ),
            (
                "dependencies",
                "依赖关系",
                "说明与其它模块的关键依赖和调用边界",
            ),
        ],
        ResearchProfile::ApiSurface => vec![
            ("api-overview", "API 概览", "列出对外暴露的主要接口和用途"),
            ("key-symbols", "关键接口", "说明重要导出符号、类型或类"),
            ("usage-boundary", "使用边界", "说明调用前提、兼容范围和限制"),
        ],
        ResearchProfile::ConfigSurface => vec![
            ("config-entry", "配置入口", "列出主要配置文件和入口位置"),
            ("config-keys", "关键配置项", "总结重要配置键和值域"),
            (
                "config-effects",
                "生效方式",
                "说明配置如何影响运行时或构建行为",
            ),
        ],
        ResearchProfile::DocsGuide => vec![
            ("core-concepts", "核心概念", "总结核心术语、职责和心智模型"),
            (
                "key-sources",
                "关键来源",
                "指出支撑当前主题的源码或文档入口",
            ),
            (
                "related-topics",
                "相关主题",
                "说明与其它主题的关系和延伸阅读",
            ),
        ],
        ResearchProfile::Testing => vec![
            ("test-scope", "测试范围", "说明当前测试覆盖的对象和边界"),
            ("test-entry", "关键测试入口", "列出关键测试目录、类或用例"),
            (
                "validation-strategy",
                "验证策略",
                "总结断言方式、集成策略和注意点",
            ),
        ],
        ResearchProfile::ExampleTutorial => vec![
            ("getting-started", "快速开始", "给出阅读或上手顺序"),
            ("example-paths", "示例路径", "指出关键示例文件、目录和入口"),
            (
                "practice-notes",
                "实践要点",
                "总结示例背后的关键约束和常见坑",
            ),
        ],
        ResearchProfile::Troubleshooting => vec![
            ("symptoms", "常见症状", "总结容易出现的问题现象"),
            (
                "investigation",
                "排查线索",
                "列出推荐的排查路径、关键日志和入口",
            ),
            ("resolution", "处理建议", "总结可行的修复方式和规避建议"),
        ],
        ResearchProfile::IntegrationPlatform => vec![
            (
                "integration-target",
                "集成对象",
                "说明当前页面面向的平台、框架或扩展对象",
            ),
            (
                "integration-entry",
                "接入方式",
                "列出接入入口、桥接层或适配点",
            ),
            (
                "platform-constraints",
                "平台约束",
                "说明平台差异、兼容性或接入限制",
            ),
        ],
        ResearchProfile::CompilerPipeline => vec![
            (
                "pipeline-stages",
                "编译阶段",
                "解释处理流程、阶段边界和产物流转",
            ),
            (
                "processor-entry",
                "关键处理器",
                "列出关键编译器、注解处理器或 codegen 入口",
            ),
            ("outputs", "产物与边界", "说明编译输出、生成物和依赖边界"),
        ],
    };

    sections.extend(
        profile_sections
            .into_iter()
            .map(|(key, title, intent)| build_planned_section(unit, key, title, intent, false)),
    );

    sections
}

fn build_planned_section(
    unit: &KnowledgeUnit,
    section_key: impl Into<String>,
    title: impl Into<String>,
    intent: impl Into<String>,
    child_digest_slot: bool,
) -> PlannedSection {
    let section_key = section_key.into();
    let title = title.into();
    let intent = intent.into();
    PlannedSection {
        section_key,
        title: title.clone(),
        intent: intent.clone(),
        section_summary: structural_section_summary(unit, &title, &intent, child_digest_slot),
        evidence_cluster_keys: Vec::new(),
        child_digest_slot,
        preserve_source_markdown: false,
    }
}

fn structural_section_summary(
    unit: &KnowledgeUnit,
    title: &str,
    intent: &str,
    child_digest_slot: bool,
) -> String {
    let mut paragraphs = vec![format!(
        "本节围绕“{}”展开，重点回答 {}，帮助读者快速判断 {} 在仓库中的职责边界与阅读入口。",
        title, intent, unit.title
    )];

    let detail = match title {
        "概述" => Some(format!(
            "应先建立 {} 的定位，再交代它与同域其它知识单元之间的关系，以及本页后续章节分别覆盖哪些实现侧面。",
            unit.title
        )),
        "技术栈" => Some("需要把核心语言、框架、构建工具与运行依赖串成一条可理解的实现链，而不是只罗列名词。".to_string()),
        "核心模块" | "域内单元" | "子单元" => Some("应该把子页的职责分工、先后阅读顺序与相互依赖关系连起来，避免只列标题。".to_string()),
        "模块结构" | "依赖关系" | "相关主题" => Some("要说明哪些模块或主题形成主链，哪些只是支撑层，并指出变化会沿哪些调用或依赖路径扩散。".to_string()),
        "关键源码" | "关键来源" | "关键测试入口" | "关键接口" | "关键处理器" => Some("应优先覆盖最能解释主流程、对外契约或关键状态变化的文件、符号和目录入口。".to_string()),
        "公共接口" | "API 概览" => Some("需要把对外暴露的入口、典型调用方式与使用边界放在一起说明，避免源码入口与 API 语义脱节。".to_string()),
        "配置入口" | "关键配置项" | "生效方式" => Some("要交代配置从哪里声明、如何解析、何时生效，以及错误配置通常会影响哪条执行路径。".to_string()),
        "测试范围" | "验证策略" => Some("应把测试覆盖对象、断言重点与集成方式写清楚，让读者能从测试反推核心行为约束。".to_string()),
        "快速开始" | "示例路径" | "实践要点" => Some("需要给出从示例到正式实现的最短路径，并说明哪些目录只是演示、哪些路径代表真实产品主线。".to_string()),
        "常见症状" | "排查线索" | "处理建议" => Some("应按现象、定位入口、修复动作的顺序组织内容，让排障链路可直接执行。".to_string()),
        "编译阶段" | "产物与边界" => Some("要把输入、处理中间阶段和输出产物串成完整链路，并解释每个阶段的边界。".to_string()),
        "集成对象" | "接入方式" | "平台约束" => Some("需要说明与哪些外部平台或框架发生耦合、接入点在哪里、约束来自哪一层。".to_string()),
        _ => None,
    };
    if let Some(detail) = detail {
        paragraphs.push(detail);
    }

    if child_digest_slot {
        paragraphs.push(
            "本节还应消费子页摘要，把叶子页已经确认的职责、证据入口和差异点向父页回收。"
                .to_string(),
        );
    }

    paragraphs.join("\n\n")
}

fn build_evidence_clusters_from_scope(
    unit: &KnowledgeUnit,
    ds: &ResearchDataSource,
    key_sources: &[String],
    doc_reference_citations: &[SourceCitation],
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

    if !doc_reference_citations.is_empty() {
        clusters.push(EvidenceCluster {
            cluster_key: stable_id("evidence-cluster", &format!("{}:docs-references", unit.id)),
            label: "文档引用的实现入口".to_string(),
            citations: doc_reference_citations.iter().take(8).cloned().collect(),
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

    if !unit.scope.config_surfaces.is_empty() {
        let citations: Vec<SourceCitation> = unit
            .scope
            .config_surfaces
            .iter()
            .map(|surface| SourceCitation {
                path: surface.file_path.clone(),
                start_line: 1,
                end_line: 40,
                source_id: ds
                    .report
                    .files
                    .iter()
                    .find(|file| file.path == surface.file_path)
                    .map(|file| file.id.clone()),
                symbol_id: None,
                note: if surface.keys.is_empty() {
                    "配置入口".to_string()
                } else {
                    format!("配置键：{}", surface.keys.join(", "))
                },
            })
            .take(6)
            .collect();

        if !citations.is_empty() {
            clusters.push(EvidenceCluster {
                cluster_key: stable_id("evidence-cluster", &format!("{}:config-surface", unit.id)),
                label: "配置入口".to_string(),
                citations,
            });
        }
    }

    if !unit.scope.docs_anchors.is_empty() {
        let citations: Vec<SourceCitation> = unit
            .scope
            .docs_anchors
            .iter()
            .map(|anchor| SourceCitation {
                path: anchor.file_path.clone(),
                start_line: 1,
                end_line: 24,
                source_id: ds
                    .report
                    .files
                    .iter()
                    .find(|file| file.path == anchor.file_path)
                    .map(|file| file.id.clone()),
                symbol_id: None,
                note: if anchor.heading.is_empty() {
                    "文档锚点".to_string()
                } else {
                    format!("文档锚点：{}", anchor.heading)
                },
            })
            .take(6)
            .collect();

        if !citations.is_empty() {
            clusters.push(EvidenceCluster {
                cluster_key: stable_id("evidence-cluster", &format!("{}:docs-anchor", unit.id)),
                label: "文档入口".to_string(),
                citations,
            });
        }
    }

    clusters
}

fn seed_section_evidence_clusters(
    section_plan: &mut [PlannedSection],
    evidence_clusters: &[EvidenceCluster],
) {
    if evidence_clusters.is_empty() {
        return;
    }

    let all_cluster_keys = evidence_clusters
        .iter()
        .map(|cluster| cluster.cluster_key.clone())
        .collect::<Vec<_>>();

    for section in section_plan {
        if section.title.trim().is_empty() || section.title.trim() == "目录" {
            continue;
        }

        let suggested_keys = suggested_cluster_keys_for_section(section, evidence_clusters);
        if suggested_keys.is_empty() {
            continue;
        }

        for cluster_key in suggested_keys {
            if !section
                .evidence_cluster_keys
                .iter()
                .any(|existing| existing == &cluster_key)
            {
                section.evidence_cluster_keys.push(cluster_key);
            }
        }

        if section.title.trim() == "附录" && section.evidence_cluster_keys.is_empty() {
            section.evidence_cluster_keys = all_cluster_keys.clone();
        }
    }
}

fn suggested_cluster_keys_for_section(
    section: &PlannedSection,
    evidence_clusters: &[EvidenceCluster],
) -> Vec<String> {
    let section_title = section.title.trim();
    let bucket_order: &[&str] = match section_title {
        "简介" => &["docs-references", "docs-anchor", "key-sources"],
        "项目结构" => &["key-sources", "config-surface", "docs-anchor"],
        "核心组件" => &["api-surface", "key-sources", "docs-references"],
        "架构总览" => &["key-sources", "api-surface", "config-surface"],
        "详细组件分析" => &[
            "api-surface",
            "docs-references",
            "key-sources",
            "config-surface",
        ],
        "依赖关系分析" => &["api-surface", "key-sources"],
        "性能考量" => &["config-surface", "key-sources"],
        "故障排查指南" => &["docs-references", "docs-anchor", "key-sources"],
        "结论" => &["key-sources", "docs-references"],
        "附录" => &[
            "key-sources",
            "docs-references",
            "api-surface",
            "config-surface",
            "docs-anchor",
        ],
        _ => &[],
    };

    let mut matches = Vec::new();
    for bucket in bucket_order {
        for cluster in evidence_clusters {
            if cluster_key_matches_bucket(&cluster.cluster_key, bucket)
                && !matches
                    .iter()
                    .any(|existing: &String| existing == &cluster.cluster_key)
            {
                matches.push(cluster.cluster_key.clone());
            }
        }
    }

    if matches.is_empty() && uses_reference_outline_section_title(section_title) {
        evidence_clusters
            .iter()
            .take(2)
            .map(|cluster| cluster.cluster_key.clone())
            .collect()
    } else {
        matches
    }
}

fn cluster_key_matches_bucket(cluster_key: &str, bucket: &str) -> bool {
    cluster_key.ends_with(bucket)
}

fn uses_reference_outline_section_title(title: &str) -> bool {
    matches!(
        title,
        "简介"
            | "项目结构"
            | "核心组件"
            | "架构总览"
            | "详细组件分析"
            | "依赖关系分析"
            | "性能考量"
            | "故障排查指南"
            | "结论"
            | "附录"
    )
}

fn collect_doc_reference_citations(
    unit: &KnowledgeUnit,
    ds: &ResearchDataSource,
) -> Vec<SourceCitation> {
    let mut citations = Vec::new();
    let mut seen = BTreeSet::new();
    let repo_root = Path::new(&ds.report.root);

    for anchor in &unit.scope.docs_anchors {
        let doc_path = repo_root.join(anchor.file_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let Ok(content) = fs::read_to_string(doc_path) else {
            continue;
        };
        for citation in extract_doc_reference_citations_from_content(&content, ds, &anchor.heading)
        {
            let dedup_key = format!(
                "{}:{}:{}",
                citation.path, citation.start_line, citation.end_line
            );
            if seen.insert(dedup_key) {
                citations.push(citation);
            }
            if citations.len() >= 12 {
                return citations;
            }
        }
    }

    citations
}

fn extract_doc_reference_citations_from_content(
    content: &str,
    ds: &ResearchDataSource,
    anchor_heading: &str,
) -> Vec<SourceCitation> {
    let file_link_pattern =
        Regex::new(r"file://([^)#]+?)(?:#L?(\d+)-L?(\d+))?\)").expect("valid file link regex");
    let mut citations = Vec::new();
    for captures in file_link_pattern.captures_iter(content) {
        let Some(raw_path) = captures.get(1).map(|m| m.as_str().trim()) else {
            continue;
        };
        let path = normalize_repo_reference_path(raw_path);
        let Some(source_id) = ds
            .report
            .files
            .iter()
            .find(|file| file.path == path)
            .map(|file| file.id.clone())
        else {
            continue;
        };
        let start_line = captures
            .get(2)
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(1);
        let end_line = captures
            .get(3)
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(start_line.saturating_add(23));
        citations.push(SourceCitation {
            path,
            start_line,
            end_line,
            source_id: Some(source_id),
            symbol_id: None,
            note: if anchor_heading.trim().is_empty() {
                "文档引用".to_string()
            } else {
                format!("文档引用：{}", anchor_heading.trim())
            },
        });
    }
    citations
}

fn normalize_repo_reference_path(path: &str) -> String {
    path.replace('\\', "/").trim().to_string()
}

fn dedup_preserving_order(values: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

fn infer_research_profile(unit: &KnowledgeUnit) -> Option<ResearchProfile> {
    match unit.decomposition_profile.as_ref() {
        Some(DecompositionProfile::Runtime) => Some(ResearchProfile::Runtime),
        Some(DecompositionProfile::ApiSurface) => Some(ResearchProfile::ApiSurface),
        Some(DecompositionProfile::ConfigSurface) => Some(ResearchProfile::ConfigSurface),
        Some(DecompositionProfile::DocsGuide) => Some(ResearchProfile::DocsGuide),
        Some(DecompositionProfile::Testing) => Some(ResearchProfile::Testing),
        Some(DecompositionProfile::ExampleTutorial) => Some(ResearchProfile::ExampleTutorial),
        Some(DecompositionProfile::Troubleshooting) => Some(ResearchProfile::Troubleshooting),
        Some(DecompositionProfile::IntegrationPlatform) => {
            Some(ResearchProfile::IntegrationPlatform)
        }
        Some(DecompositionProfile::CompilerPipeline) => Some(ResearchProfile::CompilerPipeline),
        None => match unit.unit_type {
            UnitType::ApiDoc => Some(ResearchProfile::ApiSurface),
            UnitType::ConfigDoc => Some(ResearchProfile::ConfigSurface),
            UnitType::ConceptGuide => Some(ResearchProfile::DocsGuide),
            UnitType::TestDoc => Some(ResearchProfile::Testing),
            UnitType::ExampleDoc => Some(ResearchProfile::ExampleTutorial),
            UnitType::TroubleshootDoc => Some(ResearchProfile::Troubleshooting),
            UnitType::IntegrationDoc => Some(ResearchProfile::IntegrationPlatform),
            UnitType::ModuleDoc => Some(ResearchProfile::Runtime),
            _ => None,
        },
    }
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

#[cfg(test)]
mod tests {
    use super::{
        build_docs_backed_section_plan, collect_doc_reference_citations,
        seed_section_evidence_clusters, EvidenceCluster, ResearchDataSource,
    };
    use crate::domain::context::RepoContext;
    use crate::domain::knowledge::{DocsAnchor, KnowledgeTree, KnowledgeUnit, UnitScope, UnitType};
    use crate::domain::module_tree::ModuleTree;
    use crate::domain::research::SourceCitation;
    use crate::repo::scanner::{DependencyHint, FilePurpose, ScanReport, ScannedFile};
    use crate::repo::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
    use crate::repo::symbols::ParsedSymbolsSnapshot;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn collect_doc_reference_citations_promotes_markdown_file_links() {
        let temp = tempdir().unwrap();
        let report = write_docs_fixture(
            temp.path(),
            r#"# 主题

<cite>
- [index.ts](file://src/index.ts#L3-L17)
- [manager.tsx](file://src/manager.tsx)
</cite>
"#,
        );
        let ds_env = test_research_ds(&report);
        let ds = ds_env.as_ds(&report);
        let mut unit = KnowledgeUnit::new(UnitType::ConceptGuide, "主题", "domain-docs", "主题.md");
        unit.scope = UnitScope {
            docs_anchors: vec![DocsAnchor {
                file_path: "docs/topic.md".to_string(),
                heading: "主题".to_string(),
                level: 1,
                links: Vec::new(),
            }],
            ..UnitScope::default()
        };

        let citations = collect_doc_reference_citations(&unit, &ds);

        assert_eq!(citations.len(), 2);
        assert_eq!(citations[0].path, "src/index.ts");
        assert_eq!(citations[0].start_line, 3);
        assert_eq!(citations[0].end_line, 17);
        assert_eq!(citations[1].path, "src/manager.tsx");
        assert_eq!(citations[1].start_line, 1);
        assert_eq!(citations[1].end_line, 24);
    }

    #[test]
    fn build_docs_backed_section_plan_preserves_markdown_structure() {
        let temp = tempdir().unwrap();
        let report = write_docs_fixture(
            temp.path(),
            r#"# 主题

前言段落。

## 目录
1. [简介](#简介)

## 简介
这里是简介。

## 架构总览
这里是架构说明。
"#,
        );
        let ds_env = test_research_ds(&report);
        let ds = ds_env.as_ds(&report);
        let mut unit = KnowledgeUnit::new(UnitType::ConceptGuide, "主题", "domain-docs", "主题.md");
        unit.scope = UnitScope {
            docs_anchors: vec![DocsAnchor {
                file_path: "docs/topic.md".to_string(),
                heading: "主题".to_string(),
                level: 1,
                links: Vec::new(),
            }],
            ..UnitScope::default()
        };

        let section_plan = build_docs_backed_section_plan(
            &unit,
            &ds,
            Some(&crate::domain::research::ResearchProfile::DocsGuide),
            &[],
        )
        .unwrap();

        assert!(section_plan[0].title.is_empty());
        assert!(section_plan[0].section_summary.contains("前言段落"));
        assert!(section_plan[0].preserve_source_markdown);
        assert_eq!(section_plan[1].title, "目录");
        assert_eq!(section_plan[2].title, "简介");
        assert!(section_plan[3].section_summary.contains("架构说明"));
        assert!(section_plan[3].preserve_source_markdown);
    }

    #[test]
    fn raw_docs_section_plan_falls_back_to_reference_outline() {
        let temp = tempdir().unwrap();
        let report = write_docs_fixture(
            temp.path(),
            r#"---
title: Accessibility tests
---

Accessibility tests audit the rendered DOM.

## Install the addon
Install steps.

## Check for violations
Check steps.
"#,
        );
        let ds_env = test_research_ds(&report);
        let ds = ds_env.as_ds(&report);
        let mut unit = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "Accessibility Testing",
            "domain-docs",
            "概念指南/Accessibility-Testing.md",
        );
        unit.scope = UnitScope {
            docs_anchors: vec![DocsAnchor {
                file_path: "docs/topic.md".to_string(),
                heading: "Accessibility Testing".to_string(),
                level: 1,
                links: Vec::new(),
            }],
            ..UnitScope::default()
        };

        let section_plan = build_docs_backed_section_plan(
            &unit,
            &ds,
            Some(&crate::domain::research::ResearchProfile::DocsGuide),
            &[],
        )
        .unwrap();

        assert_eq!(section_plan[0].title, "");
        assert_eq!(section_plan[1].title, "目录");
        assert_eq!(section_plan[2].title, "简介");
        assert_eq!(section_plan[3].title, "项目结构");
        assert!(!section_plan[2].section_summary.contains("title:"));
        assert!(!section_plan[2].section_summary.contains("---"));
        assert!(section_plan
            .iter()
            .all(|section| !section.preserve_source_markdown));
    }

    #[test]
    fn reference_outline_sections_receive_seeded_evidence_clusters() {
        let temp = tempdir().unwrap();
        let report = write_docs_fixture(
            temp.path(),
            r#"---
title: Accessibility tests
---

Accessibility tests audit the rendered DOM.
"#,
        );
        let ds_env = test_research_ds(&report);
        let ds = ds_env.as_ds(&report);
        let mut unit = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "Accessibility Testing",
            "domain-docs",
            "概念指南/Accessibility-Testing.md",
        );
        unit.scope = UnitScope {
            docs_anchors: vec![DocsAnchor {
                file_path: "docs/topic.md".to_string(),
                heading: "Accessibility Testing".to_string(),
                level: 1,
                links: Vec::new(),
            }],
            ..UnitScope::default()
        };

        let mut section_plan = build_docs_backed_section_plan(
            &unit,
            &ds,
            Some(&crate::domain::research::ResearchProfile::DocsGuide),
            &[],
        )
        .unwrap();
        let evidence_clusters = vec![
            cluster("cluster:key-sources", "src/index.ts"),
            cluster("cluster:docs-references", "docs/topic.md"),
            cluster("cluster:api-surface", "src/api.ts"),
            cluster("cluster:config-surface", "config/main.ts"),
        ];

        seed_section_evidence_clusters(&mut section_plan, &evidence_clusters);

        let structure = section_plan
            .iter()
            .find(|section| section.title == "项目结构")
            .expect("structure section should exist");
        let components = section_plan
            .iter()
            .find(|section| section.title == "核心组件")
            .expect("components section should exist");
        let appendix = section_plan
            .iter()
            .find(|section| section.title == "附录")
            .expect("appendix section should exist");

        assert!(structure
            .evidence_cluster_keys
            .iter()
            .any(|key| key.ends_with("key-sources")));
        assert!(components
            .evidence_cluster_keys
            .iter()
            .any(|key| key.ends_with("api-surface")));
        assert!(appendix.evidence_cluster_keys.len() >= 3);
    }

    fn scanned_file(path: &str, kind: &str) -> ScannedFile {
        ScannedFile {
            id: format!("source:{path}"),
            path: path.to_string(),
            language: "markdown".to_string(),
            kind: kind.to_string(),
            purpose: if kind == "docs" {
                FilePurpose::Docs
            } else {
                FilePurpose::Utility
            },
            fingerprint: "fp".to_string(),
            size: 0,
            tags: Vec::new(),
        }
    }

    fn cluster(cluster_key: &str, path: &str) -> EvidenceCluster {
        EvidenceCluster {
            cluster_key: cluster_key.to_string(),
            label: cluster_key.to_string(),
            citations: vec![SourceCitation {
                path: path.to_string(),
                start_line: 1,
                end_line: 10,
                source_id: None,
                symbol_id: None,
                note: String::new(),
            }],
        }
    }

    fn write_docs_fixture(root: &std::path::Path, docs_content: &str) -> ScanReport {
        let docs_root = root.join("docs");
        fs::create_dir_all(&docs_root).unwrap();
        fs::write(docs_root.join("topic.md"), docs_content).unwrap();
        ScanReport {
            root: root.to_string_lossy().to_string(),
            files: vec![
                scanned_file("docs/topic.md", "docs"),
                scanned_file("src/index.ts", "source"),
                scanned_file("src/manager.tsx", "source"),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::<DependencyHint>::new(),
        }
    }

    struct TestResearchDs {
        repo_context: RepoContext,
        module_tree: ModuleTree,
        module_contexts: Vec<crate::domain::context::ModuleContext>,
        symbol_snapshot: ParsedSymbolsSnapshot,
        resolved_graph: ResolvedGraphSnapshot,
        graph_analysis: GraphAnalysisSnapshot,
        graph_summary: GraphSummary,
        knowledge_tree: KnowledgeTree,
    }

    impl TestResearchDs {
        fn as_ds<'a>(&'a self, report: &'a ScanReport) -> ResearchDataSource<'a> {
            ResearchDataSource {
                report,
                module_tree: &self.module_tree,
                repo_context: &self.repo_context,
                module_contexts: &self.module_contexts,
                symbol_snapshot: &self.symbol_snapshot,
                resolved_graph: &self.resolved_graph,
                graph_analysis: &self.graph_analysis,
                graph_summary: &self.graph_summary,
                knowledge_tree: &self.knowledge_tree,
            }
        }
    }

    fn test_research_ds(_report: &ScanReport) -> TestResearchDs {
        TestResearchDs {
            repo_context: RepoContext {
                repo_summary_inputs: Vec::new(),
                top_modules: Vec::new(),
                key_entry_points: Vec::new(),
                global_relations: Vec::new(),
                tech_stack: Vec::new(),
                graph_hotspots: Vec::new(),
                detected_processes: Vec::new(),
                community_labels: Vec::new(),
                cycle_warnings: Vec::new(),
                root_topics: Vec::new(),
                process_topics: Vec::new(),
            },
            module_tree: ModuleTree {
                root_modules: Vec::new(),
                modules: Vec::new(),
                cross_module_edges: Vec::new(),
                architecture_hints: Vec::new(),
            },
            module_contexts: Vec::new(),
            symbol_snapshot: ParsedSymbolsSnapshot::default(),
            resolved_graph: ResolvedGraphSnapshot::default(),
            graph_analysis: GraphAnalysisSnapshot::default(),
            graph_summary: GraphSummary::default(),
            knowledge_tree: KnowledgeTree::new("overview".to_string()),
        }
    }
}
