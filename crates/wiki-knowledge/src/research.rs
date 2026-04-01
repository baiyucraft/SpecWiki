use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use regex::Regex;

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::research::{
    canonical_reference_outline_title, is_reference_outline_title, DiagramSuggestion,
    DomainResearch, EvidenceCluster, KeySourceCluster, PageDigest, PlannedSection,
    ResearchPageSeed, ResearchProfile, SectionGroundingRef, SkeletonProfile, SkeletonSection,
    SourceCitation, SystemResearch, UnitResearch,
};
use wiki_index::scanner::{FilePurpose, ScanReport, ScannedFile};
use wiki_index::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
use wiki_index::symbols::ParsedSymbolsSnapshot;
use wiki_model::domain::knowledge::{
    DecompositionProfile, KnowledgeDomain, KnowledgeTree, KnowledgeUnit, PlannerSignalKind,
    UnitScope, UnitType,
};
use wiki_model::domain::module_tree::ModuleTree;
use wiki_model::domain::stable_id::stable_id;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResearchPageKind {
    SystemOverview,
    SystemArchitecture,
    DomainIndex,
    ApiSurface,
    ConfigSurface,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ResearchKeySourceOrigin {
    PlannerGrounded,
    RepoEntry,
    ModuleKeySource,
    ScopeSource,
    DocReference,
    DocsAnchor,
    ConfigSurface,
    GlobalFallback,
}

#[derive(Debug, Clone)]
struct ResearchKeySourceCandidate {
    path: String,
    origin: ResearchKeySourceOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum KeySourceRole {
    Entry,
    Contract,
    Implementation,
    Verification,
    Support,
    Docs,
    Config,
    Sample,
}

#[derive(Debug, Clone)]
struct ScoredKeySourceCandidate {
    score: i32,
    path: String,
    basename: String,
    role: KeySourceRole,
    family: String,
}

#[derive(Debug, Clone)]
struct KeySourceSelectionPolicy {
    budget: usize,
    duplicate_basename_limit: usize,
    minimum_roles: Vec<(KeySourceRole, usize)>,
    role_limits: BTreeMap<KeySourceRole, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopicFocusKind {
    TypeSystem,
    ThemeStyling,
}

#[derive(Debug, Clone, Default)]
struct TopicFocusHints {
    kind: Option<TopicFocusKind>,
    focus_tokens: BTreeSet<String>,
    anchor_tokens: BTreeSet<String>,
}

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

    let description = format!(
        "包含 {} 个文件、{} 个模块的项目",
        ds.report.files.len(),
        ds.module_tree
            .modules
            .iter()
            .filter(|m| m.parent_id.is_some())
            .count()
    );
    let system_key_sources = select_structural_key_sources(
        ds.report,
        ResearchPageKind::SystemOverview,
        None,
        None,
        collect_system_key_source_candidates(ds),
    );
    let system_architecture_key_sources = select_structural_key_sources(
        ds.report,
        ResearchPageKind::SystemArchitecture,
        None,
        None,
        collect_system_key_source_candidates(ds),
    );
    let overview_unit = KnowledgeUnit::new(UnitType::Overview, "项目概述", "system", "项目概述.md");
    let architecture_unit =
        KnowledgeUnit::new(UnitType::Architecture, "系统架构", "system", "系统架构.md");
    let mut overview_sections = build_default_section_plan(&overview_unit, None, &[]);
    let mut architecture_sections = build_default_section_plan(&architecture_unit, None, &[]);
    let overview_evidence_clusters =
        build_structural_seed_evidence_clusters("system-overview", &system_key_sources, ds.report);
    let architecture_evidence_clusters = build_structural_seed_evidence_clusters(
        "system-architecture",
        &system_architecture_key_sources,
        ds.report,
    );
    seed_section_evidence_clusters(&mut overview_sections, &overview_evidence_clusters);
    seed_section_evidence_clusters(&mut architecture_sections, &architecture_evidence_clusters);

    SystemResearch {
        project_name,
        description: description.clone(),
        project_type: infer_project_type(ds),
        target_users: vec!["开发者".to_string()],
        system_boundary: format!("仓库根路径: {}", ds.report.root),
        tech_stack,
        architecture_pattern,
        key_domains,
        overview_seed: build_page_seed(
            "system_overview",
            &description,
            "说明仓库的整体定位、知识域和主入口。",
            &overview_sections,
            &system_key_sources,
            &overview_evidence_clusters,
            &[],
        ),
        architecture_seed: build_page_seed(
            "system_architecture",
            &description,
            "说明仓库的模块结构、关系与整体架构边界。",
            &architecture_sections,
            &system_architecture_key_sources,
            &architecture_evidence_clusters,
            &[],
        ),
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

    let domain_summary = format!(
        "{}（{}）：包含 {} 个模块",
        domain.label,
        domain.domain_type.as_str(),
        domain.source_modules.len()
    );
    let key_sources = select_structural_key_sources(
        ds.report,
        ResearchPageKind::DomainIndex,
        None,
        None,
        collect_domain_key_source_candidates(domain, &module_context_index),
    );
    let mut domain_unit = KnowledgeUnit::new(
        UnitType::DomainIndex,
        &domain.label,
        &domain.id,
        format!("{0}/{0}.md", sanitize_path_segment(&domain.label)),
    );
    domain_unit.scope = UnitScope {
        module_ids: domain.source_modules.clone(),
        source_ids: ds
            .report
            .files
            .iter()
            .filter(|file| {
                key_sources
                    .iter()
                    .any(|path| normalize_path(&file.path) == *path)
            })
            .map(|file| file.id.clone())
            .collect(),
        ..UnitScope::default()
    };
    let mut section_plan = build_default_section_plan(&domain_unit, None, &[]);
    let evidence_clusters =
        build_structural_seed_evidence_clusters("domain", &key_sources, ds.report);
    seed_section_evidence_clusters(&mut section_plan, &evidence_clusters);

    DomainResearch {
        domain_id: domain.id.clone(),
        domain_summary: domain_summary.clone(),
        internal_structure,
        key_modules,
        key_apis,
        relationships,
        diagram_suggestion: None,
        compose_seed: build_page_seed(
            "domain_index",
            &domain_summary,
            &format!("说明知识域 {} 的结构、关键模块与子页组织。", domain.label),
            &section_plan,
            &key_sources,
            &evidence_clusters,
            &[],
        ),
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
    let research_profile = infer_research_profile(unit);
    let topic_focus = build_unit_topic_focus_hints(unit, ds.report, &doc_reference_citations);

    let key_sources = select_structural_key_sources(
        ds.report,
        research_page_kind(unit, research_profile.as_ref()),
        research_profile.as_ref(),
        topic_focus.as_ref(),
        collect_unit_key_source_candidates(
            unit,
            ds.report,
            &module_context_index,
            &doc_reference_citations,
        ),
    );

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
    }

    let skeleton_profile = build_skeleton_profile(unit.unit_type.as_str(), &section_plan);
    let key_source_clusters = build_key_source_clusters(&key_sources, &evidence_clusters);
    let section_grounding_refs = build_section_grounding_refs(&section_plan, &key_source_clusters);

    UnitResearch {
        unit_id: unit.id.clone(),
        decomposition_profile: unit.decomposition_profile.clone(),
        research_profile,
        positioning,
        summary,
        section_plan,
        skeleton_profile,
        key_source_clusters,
        section_grounding_refs,
        evidence_clusters,
        diagram_suggestions: Vec::new(),
        key_sources,
        provider_stop_reason: None,
        provider_session_stats: None,
        input_hash: String::new(),
    }
}

fn collect_planner_grounded_paths(unit: &KnowledgeUnit, report: &ScanReport) -> Vec<String> {
    let source_paths: BTreeSet<String> = report
        .files
        .iter()
        .map(|file| normalize_path(&file.path))
        .collect();

    let mut bundles = unit.planner_signal_bundles.iter().collect::<Vec<_>>();
    bundles.sort_by_key(|bundle| match bundle.kind {
        PlannerSignalKind::SurfaceCluster => 0usize,
        PlannerSignalKind::LeafDecomposition => 1,
        PlannerSignalKind::RepoArchetype => 2,
    });

    let mut grounded_paths = bundles
        .into_iter()
        .flat_map(|bundle| bundle.matched_paths.iter())
        .map(|path| normalize_path(path))
        .filter(|path| !path.ends_with(".md") && !path.ends_with(".mdx"))
        .filter(|path| source_paths.contains(path))
        .collect::<Vec<_>>();
    dedup_preserving_order(&mut grounded_paths);
    grounded_paths
}

fn collect_system_key_source_candidates(
    ds: &ResearchDataSource,
) -> Vec<ResearchKeySourceCandidate> {
    let mut candidates = Vec::new();
    candidates.extend(
        ds.repo_context
            .key_entry_points
            .iter()
            .cloned()
            .map(|path| ResearchKeySourceCandidate {
                path,
                origin: ResearchKeySourceOrigin::RepoEntry,
            }),
    );
    candidates.extend(
        ds.module_contexts
            .iter()
            .flat_map(|context| context.key_sources.iter().cloned())
            .map(|path| ResearchKeySourceCandidate {
                path,
                origin: ResearchKeySourceOrigin::ModuleKeySource,
            }),
    );
    candidates.extend(ds.report.entry_points.iter().cloned().map(|path| {
        ResearchKeySourceCandidate {
            path,
            origin: ResearchKeySourceOrigin::RepoEntry,
        }
    }));
    candidates.extend(ds.report.config_files.iter().cloned().map(|path| {
        ResearchKeySourceCandidate {
            path,
            origin: ResearchKeySourceOrigin::ConfigSurface,
        }
    }));
    candidates.extend(
        ds.report
            .files
            .iter()
            .filter(|file| is_global_structural_candidate(file))
            .map(|file| ResearchKeySourceCandidate {
                path: normalize_path(&file.path),
                origin: ResearchKeySourceOrigin::GlobalFallback,
            }),
    );
    candidates
}

fn collect_domain_key_source_candidates(
    domain: &KnowledgeDomain,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
) -> Vec<ResearchKeySourceCandidate> {
    let mut candidates = domain
        .source_files
        .iter()
        .cloned()
        .map(|path| ResearchKeySourceCandidate {
            path,
            origin: ResearchKeySourceOrigin::ScopeSource,
        })
        .collect::<Vec<_>>();
    candidates.extend(
        domain
            .source_modules
            .iter()
            .filter_map(|mid| module_context_index.get(mid.as_str()))
            .flat_map(|context| context.key_sources.iter().cloned())
            .map(|path| ResearchKeySourceCandidate {
                path,
                origin: ResearchKeySourceOrigin::ModuleKeySource,
            }),
    );
    candidates.extend(domain.evidence.docs_anchors.iter().map(|anchor| {
        ResearchKeySourceCandidate {
            path: anchor.file_path.clone(),
            origin: ResearchKeySourceOrigin::DocsAnchor,
        }
    }));
    candidates.extend(domain.evidence.config_surfaces.iter().map(|surface| {
        ResearchKeySourceCandidate {
            path: surface.file_path.clone(),
            origin: ResearchKeySourceOrigin::ConfigSurface,
        }
    }));
    candidates
}

fn collect_unit_key_source_candidates(
    unit: &KnowledgeUnit,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    doc_reference_citations: &[SourceCitation],
) -> Vec<ResearchKeySourceCandidate> {
    let mut candidates = collect_planner_grounded_paths(unit, report)
        .into_iter()
        .map(|path| ResearchKeySourceCandidate {
            path,
            origin: ResearchKeySourceOrigin::PlannerGrounded,
        })
        .collect::<Vec<_>>();
    candidates.extend(unit.scope.source_ids.iter().filter_map(|source_id| {
        report
            .files
            .iter()
            .find(|file| &file.id == source_id)
            .map(|file| ResearchKeySourceCandidate {
                path: normalize_path(&file.path),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
    }));
    candidates.extend(
        unit.scope
            .module_ids
            .iter()
            .filter_map(|mid| module_context_index.get(mid.as_str()))
            .flat_map(|context| context.key_sources.iter().cloned())
            .map(|path| ResearchKeySourceCandidate {
                path,
                origin: ResearchKeySourceOrigin::ModuleKeySource,
            }),
    );
    candidates.extend(
        doc_reference_citations
            .iter()
            .map(|citation| ResearchKeySourceCandidate {
                path: citation.path.clone(),
                origin: ResearchKeySourceOrigin::DocReference,
            }),
    );
    candidates.extend(
        unit.scope
            .docs_anchors
            .iter()
            .map(|anchor| ResearchKeySourceCandidate {
                path: anchor.file_path.clone(),
                origin: ResearchKeySourceOrigin::DocsAnchor,
            }),
    );
    candidates.extend(unit.scope.config_surfaces.iter().map(|surface| {
        ResearchKeySourceCandidate {
            path: surface.file_path.clone(),
            origin: ResearchKeySourceOrigin::ConfigSurface,
        }
    }));
    candidates
}

fn build_unit_topic_focus_hints(
    unit: &KnowledgeUnit,
    report: &ScanReport,
    doc_reference_citations: &[SourceCitation],
) -> Option<TopicFocusHints> {
    let mut title_tokens = extract_focus_tokens(&unit.title);
    let mut path_tokens = BTreeSet::new();
    for path in collect_planner_grounded_paths(unit, report) {
        path_tokens.extend(extract_focus_tokens(&path));
    }
    for citation in doc_reference_citations {
        path_tokens.extend(extract_focus_tokens(&citation.path));
    }
    for source_id in &unit.scope.source_ids {
        if let Some(file) = report.files.iter().find(|file| &file.id == source_id) {
            path_tokens.extend(extract_focus_tokens(&file.path));
        }
    }
    for anchor in &unit.scope.docs_anchors {
        path_tokens.extend(extract_focus_tokens(&anchor.file_path));
        title_tokens.extend(extract_focus_tokens(&anchor.heading));
    }
    for surface in &unit.scope.config_surfaces {
        path_tokens.extend(extract_focus_tokens(&surface.file_path));
    }

    let mut focus_tokens = title_tokens.clone();
    focus_tokens.extend(path_tokens.iter().cloned());
    let kind = infer_topic_focus_kind(&focus_tokens);
    if kind.is_none() && title_tokens.is_empty() {
        return None;
    }

    let semantic_anchor_tokens = kind.map(topic_focus_anchor_tokens).unwrap_or_default();
    if matches!(kind, Some(TopicFocusKind::TypeSystem))
        && (doc_reference_citations.is_empty()
            || path_tokens
                .intersection(&semantic_anchor_tokens)
                .next()
                .is_none())
    {
        return None;
    }
    let mut anchor_tokens = title_tokens.clone();
    anchor_tokens.extend(semantic_anchor_tokens.iter().cloned());
    focus_tokens.extend(anchor_tokens.iter().cloned());
    Some(TopicFocusHints {
        kind,
        focus_tokens,
        anchor_tokens,
    })
}

fn infer_topic_focus_kind(tokens: &BTreeSet<String>) -> Option<TopicFocusKind> {
    let theme_triggers = [
        "theme",
        "themes",
        "theming",
        "style",
        "styles",
        "css",
        "font",
        "fonts",
        "color",
        "colors",
        "palette",
        "typography",
    ];
    if tokens
        .iter()
        .any(|token| theme_triggers.contains(&token.as_str()))
    {
        return Some(TopicFocusKind::ThemeStyling);
    }

    let type_triggers = [
        "type",
        "types",
        "typing",
        "typings",
        "arg",
        "args",
        "annotation",
        "annotations",
        "parameter",
        "parameters",
        "contract",
        "schema",
    ];
    tokens
        .iter()
        .any(|token| type_triggers.contains(&token.as_str()))
        .then_some(TopicFocusKind::TypeSystem)
}

fn topic_focus_anchor_tokens(kind: TopicFocusKind) -> BTreeSet<String> {
    match kind {
        TopicFocusKind::TypeSystem => [
            "arg",
            "args",
            "annotation",
            "annotations",
            "component",
            "manager",
            "meta",
            "parameter",
            "parameters",
            "preview",
            "story",
            "stories",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
        TopicFocusKind::ThemeStyling => [
            "a11y",
            "accessibility",
            "color",
            "colors",
            "contrast",
            "css",
            "dark",
            "font",
            "fonts",
            "light",
            "palette",
            "style",
            "styles",
            "theme",
            "themes",
            "theming",
            "typography",
            "vision",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    }
}

fn extract_focus_tokens(value: &str) -> BTreeSet<String> {
    let mut normalized = String::with_capacity(value.len() * 2);
    let mut previous_is_lower_or_digit = false;

    for ch in value.chars() {
        if ch.is_ascii_uppercase() && previous_is_lower_or_digit {
            normalized.push(' ');
        }
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
        } else {
            normalized.push(' ');
        }
        previous_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
    }

    let mut tokens = BTreeSet::new();
    for token in normalized.split_whitespace() {
        if is_focus_stopword(token) {
            continue;
        }
        tokens.insert(token.to_string());
        if let Some(stemmed) = stem_focus_token(token) {
            tokens.insert(stemmed);
        }
    }
    tokens.extend(expand_semantic_alias_tokens(value));
    tokens
}

fn expand_semantic_alias_tokens(value: &str) -> BTreeSet<String> {
    let lower = value.to_ascii_lowercase();
    let mut aliases = BTreeSet::new();

    if value.contains("运行时") || lower.contains("runtime") {
        aliases.insert("runtime".to_string());
    }
    if value.contains("编译") || lower.contains("compiler") {
        aliases.insert("compiler".to_string());
    }
    if value.contains("处理器") || lower.contains("processor") {
        aliases.insert("processor".to_string());
    }
    if value.contains("代码生成") || lower.contains("codegen") {
        aliases.insert("codegen".to_string());
    }
    if value.contains("测试") || lower.contains("test") {
        aliases.insert("test".to_string());
        aliases.insert("testing".to_string());
    }
    if value.contains("配置") || lower.contains("config") {
        aliases.insert("config".to_string());
    }
    if value.contains("主题") || lower.contains("theme") || lower.contains("theming") {
        aliases.insert("theme".to_string());
        aliases.insert("theming".to_string());
    }
    if value.contains("类型") || lower.contains("type") || lower.contains("typing") {
        aliases.insert("type".to_string());
        aliases.insert("types".to_string());
    }
    if value.contains("集成") || lower.contains("integration") {
        aliases.insert("integration".to_string());
    }
    if value.contains("示例") || lower.contains("example") || lower.contains("tutorial") {
        aliases.insert("example".to_string());
        aliases.insert("tutorial".to_string());
    }
    if value.contains("故障") || value.contains("排查") || lower.contains("troubleshoot") {
        aliases.insert("troubleshooting".to_string());
    }
    if value.contains("架构") || lower.contains("architecture") {
        aliases.insert("architecture".to_string());
    }

    aliases
}

fn is_focus_stopword(token: &str) -> bool {
    matches!(
        token,
        "api"
            | "code"
            | "doc"
            | "docs"
            | "json"
            | "jsx"
            | "lib"
            | "main"
            | "md"
            | "mdx"
            | "module"
            | "modules"
            | "repo"
            | "rs"
            | "src"
            | "test"
            | "tests"
            | "ts"
            | "tsx"
            | "wiki"
    )
}

fn stem_focus_token(token: &str) -> Option<String> {
    if token.ends_with("ies") && token.len() > 4 {
        return Some(format!("{}y", &token[..token.len() - 3]));
    }
    if token.ends_with('s') && token.len() > 4 {
        return Some(token[..token.len() - 1].to_string());
    }
    None
}

fn select_structural_key_sources(
    report: &ScanReport,
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
    topic_focus: Option<&TopicFocusHints>,
    candidates: Vec<ResearchKeySourceCandidate>,
) -> Vec<String> {
    let mut grouped = BTreeMap::<String, BTreeSet<ResearchKeySourceOrigin>>::new();
    for candidate in candidates {
        let normalized = normalize_path(&candidate.path);
        if report
            .files
            .iter()
            .any(|file| normalize_path(&file.path) == normalized)
        {
            grouped
                .entry(normalized)
                .or_default()
                .insert(candidate.origin);
        }
    }

    let mut scored = grouped
        .into_iter()
        .filter_map(|(path, origins)| {
            let file = report
                .files
                .iter()
                .find(|file| normalize_path(&file.path) == path)?;
            let origin_vec = origins.iter().copied().collect::<Vec<_>>();
            let score = structural_key_source_score(
                page_kind,
                research_profile,
                topic_focus,
                file,
                &origin_vec,
            );
            let basename = Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let role = classify_key_source_role(research_profile, file, &path);
            let family = key_source_family(&basename, &path, role);
            (score > -1000).then_some(ScoredKeySourceCandidate {
                score,
                path,
                basename,
                role,
                family,
            })
        })
        .collect::<Vec<_>>();

    let budget = structural_key_source_budget(page_kind, research_profile);
    if should_expand_profile_fallback_candidates(&scored, budget) {
        extend_with_profile_fallback_candidates(
            report,
            page_kind,
            research_profile,
            topic_focus,
            &mut scored,
        );
    }

    if scored.is_empty() {
        scored = report
            .files
            .iter()
            .map(|file| {
                let path = normalize_path(&file.path);
                let basename = Path::new(&path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                let fallback_score = fallback_structural_key_source_score(file)
                    + topic_focus_score(topic_focus, &path, &basename);
                let role = classify_key_source_role(research_profile, file, &path);
                let family = key_source_family(&basename, &path, role);
                ScoredKeySourceCandidate {
                    score: fallback_score,
                    path,
                    basename,
                    role,
                    family,
                }
            })
            .collect::<Vec<_>>();
    }

    scored.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.path.len().cmp(&right.path.len()))
            .then_with(|| left.path.cmp(&right.path))
    });
    let policy = structural_selection_policy(page_kind, research_profile, topic_focus);
    select_candidates_with_policy(&scored, research_profile, &policy)
}

fn structural_key_source_budget(
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
) -> usize {
    if is_high_level_page(page_kind) {
        return 14;
    }
    match research_profile {
        Some(ResearchProfile::Testing) => 12,
        Some(ResearchProfile::ApiSurface) => 12,
        Some(ResearchProfile::ConfigSurface) => 10,
        Some(ResearchProfile::CompilerPipeline) => 10,
        _ => 8,
    }
}

fn structural_duplicate_basename_limit(
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
) -> usize {
    if is_high_level_page(page_kind) {
        return 2;
    }
    match research_profile {
        Some(ResearchProfile::ApiSurface) => 2,
        Some(ResearchProfile::Testing) => 2,
        _ => 3,
    }
}

fn structural_selection_policy(
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
    topic_focus: Option<&TopicFocusHints>,
) -> KeySourceSelectionPolicy {
    let mut minimum_roles = Vec::new();
    let mut role_limits = BTreeMap::new();

    match research_profile {
        Some(ResearchProfile::ApiSurface) => {
            let type_system_focus = matches!(
                topic_focus.and_then(|focus| focus.kind),
                Some(TopicFocusKind::TypeSystem)
            );
            let implementation_spine_focus = api_surface_prefers_implementation_spine(topic_focus);
            let (contract_minimum, implementation_minimum) = if type_system_focus {
                (1, 2)
            } else if implementation_spine_focus {
                (1, 2)
            } else {
                (2, 0)
            };
            minimum_roles.push((KeySourceRole::Contract, contract_minimum));
            if implementation_minimum > 0 {
                minimum_roles.push((KeySourceRole::Implementation, implementation_minimum));
            }
            if type_system_focus {
                minimum_roles.push((KeySourceRole::Entry, 1));
                role_limits.insert(KeySourceRole::Contract, 1);
                role_limits.insert(KeySourceRole::Verification, 0);
            }
            role_limits.insert(KeySourceRole::Docs, 1);
            role_limits.insert(KeySourceRole::Support, 2);
            role_limits.insert(KeySourceRole::Sample, 0);
        }
        Some(ResearchProfile::ConfigSurface) => {
            minimum_roles.push((KeySourceRole::Config, 2));
            minimum_roles.push((KeySourceRole::Implementation, 1));
            role_limits.insert(KeySourceRole::Docs, 1);
            role_limits.insert(KeySourceRole::Support, 1);
            role_limits.insert(KeySourceRole::Sample, 0);
        }
        Some(ResearchProfile::Testing) => {
            minimum_roles.push((KeySourceRole::Verification, 2));
            minimum_roles.push((KeySourceRole::Implementation, 1));
            role_limits.insert(KeySourceRole::Docs, 1);
            role_limits.insert(KeySourceRole::Support, 2);
            role_limits.insert(KeySourceRole::Sample, 0);
        }
        Some(ResearchProfile::DocsGuide) | Some(ResearchProfile::Troubleshooting) => {
            role_limits.insert(KeySourceRole::Support, 2);
        }
        _ => {}
    }

    KeySourceSelectionPolicy {
        budget: structural_key_source_budget(page_kind, research_profile),
        duplicate_basename_limit: structural_duplicate_basename_limit(page_kind, research_profile),
        minimum_roles,
        role_limits,
    }
}

fn api_surface_prefers_implementation_spine(topic_focus: Option<&TopicFocusHints>) -> bool {
    topic_focus
        .map(|focus| {
            focus.focus_tokens.contains("compiler")
                || focus.focus_tokens.contains("processor")
                || focus.focus_tokens.contains("codegen")
                || focus.focus_tokens.contains("validation")
        })
        .unwrap_or(false)
}

fn select_candidates_with_policy(
    scored: &[ScoredKeySourceCandidate],
    research_profile: Option<&ResearchProfile>,
    policy: &KeySourceSelectionPolicy,
) -> Vec<String> {
    let mut selected = Vec::new();
    let mut basename_counts = BTreeMap::<String, usize>::new();
    let mut role_counts = BTreeMap::<KeySourceRole, usize>::new();
    let mut family_counts = BTreeMap::<String, usize>::new();

    for (role, minimum) in &policy.minimum_roles {
        for candidate in scored.iter().filter(|candidate| candidate.role == *role) {
            if !can_select_candidate(
                candidate,
                research_profile,
                policy,
                &selected,
                &basename_counts,
                &role_counts,
                &family_counts,
            ) {
                continue;
            }
            push_selected_candidate(
                candidate,
                &mut selected,
                &mut basename_counts,
                &mut role_counts,
                &mut family_counts,
            );
            if role_counts.get(role).copied().unwrap_or(0) >= *minimum
                || selected.len() >= policy.budget
            {
                break;
            }
        }
    }

    if selected.len() < policy.budget {
        for candidate in scored {
            if !can_select_candidate(
                candidate,
                research_profile,
                policy,
                &selected,
                &basename_counts,
                &role_counts,
                &family_counts,
            ) {
                continue;
            }
            push_selected_candidate(
                candidate,
                &mut selected,
                &mut basename_counts,
                &mut role_counts,
                &mut family_counts,
            );
            if selected.len() >= policy.budget {
                break;
            }
        }
    }

    if selected.len() < policy.budget {
        for candidate in scored {
            if selected.iter().any(|existing| existing == &candidate.path) {
                continue;
            }
            let basename_count = basename_counts
                .get(&candidate.basename)
                .copied()
                .unwrap_or(0);
            if basename_count >= policy.duplicate_basename_limit {
                continue;
            }
            push_selected_candidate(
                candidate,
                &mut selected,
                &mut basename_counts,
                &mut role_counts,
                &mut family_counts,
            );
            if selected.len() >= policy.budget {
                break;
            }
        }
    }

    selected
}

fn can_select_candidate(
    candidate: &ScoredKeySourceCandidate,
    research_profile: Option<&ResearchProfile>,
    policy: &KeySourceSelectionPolicy,
    selected: &[String],
    basename_counts: &BTreeMap<String, usize>,
    role_counts: &BTreeMap<KeySourceRole, usize>,
    family_counts: &BTreeMap<String, usize>,
) -> bool {
    if selected.iter().any(|existing| existing == &candidate.path) {
        return false;
    }

    let basename_count = basename_counts
        .get(&candidate.basename)
        .copied()
        .unwrap_or(0);
    if basename_count >= policy.duplicate_basename_limit {
        return false;
    }

    if let Some(limit) = policy.role_limits.get(&candidate.role) {
        let role_count = role_counts.get(&candidate.role).copied().unwrap_or(0);
        if role_count >= *limit {
            return false;
        }
    }

    let family_limit = key_source_family_limit(research_profile, candidate.role, &candidate.family);
    let family_count = family_counts.get(&candidate.family).copied().unwrap_or(0);
    family_count < family_limit
}

fn push_selected_candidate(
    candidate: &ScoredKeySourceCandidate,
    selected: &mut Vec<String>,
    basename_counts: &mut BTreeMap<String, usize>,
    role_counts: &mut BTreeMap<KeySourceRole, usize>,
    family_counts: &mut BTreeMap<String, usize>,
) {
    selected.push(candidate.path.clone());
    *basename_counts
        .entry(candidate.basename.clone())
        .or_default() += 1;
    *role_counts.entry(candidate.role).or_default() += 1;
    *family_counts.entry(candidate.family.clone()).or_default() += 1;
}

fn structural_key_source_score(
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
    topic_focus: Option<&TopicFocusHints>,
    file: &ScannedFile,
    origins: &[ResearchKeySourceOrigin],
) -> i32 {
    let path = normalize_path(&file.path);
    let lower_path = path.to_ascii_lowercase();
    let topic_focus_anchor_overlap = topic_focus_anchor_overlap(topic_focus, &path);
    let file_name = Path::new(&path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if file.is_asset_like() || is_generated_binary_path(&lower_path) || is_lockfile(&file_name) {
        return -1000;
    }

    if is_root_governance_noise(&file_name, &lower_path) && is_high_level_page(page_kind) {
        return -1000;
    }
    if is_bootstrap_wrapper_noise(&file_name) && is_high_level_page(page_kind) {
        return -1000;
    }
    if is_sample_noise_path(&lower_path) && !allows_sample_noise(research_profile) {
        return -1000;
    }

    let mut score = origins
        .iter()
        .map(|origin| origin_score(page_kind, *origin))
        .sum::<i32>();

    if is_root_governance_noise(&file_name, &lower_path) {
        score -= 280;
    }
    if is_bootstrap_wrapper_noise(&file_name) {
        score -= 180;
    }
    if is_sample_noise_path(&lower_path) && !allows_sample_noise(research_profile) {
        score -= 220;
    }
    if file.is_test_like() && !allows_sample_noise(research_profile) {
        score -= 120;
    }
    if file.is_test_like() && matches!(research_profile, Some(ResearchProfile::Testing)) {
        score += 210;
    }
    if is_real_test_case_path(&lower_path)
        && matches!(research_profile, Some(ResearchProfile::Testing))
    {
        score += 160;
    }
    if is_contract_like_path(file, &lower_path) {
        score += match research_profile {
            Some(ResearchProfile::ApiSurface) => 180,
            Some(ResearchProfile::Runtime) => 130,
            Some(ResearchProfile::ConfigSurface) => 80,
            _ => 40,
        };
    }
    if is_internal_support_path(&lower_path) {
        score -= match research_profile {
            Some(ResearchProfile::ApiSurface) if topic_focus_anchor_overlap > 0 => 40,
            Some(ResearchProfile::ConfigSurface) if topic_focus_anchor_overlap > 0 => 60,
            Some(ResearchProfile::Testing) if topic_focus_anchor_overlap > 0 => 50,
            Some(ResearchProfile::Runtime) if topic_focus_anchor_overlap > 0 => 40,
            Some(ResearchProfile::ApiSurface) => 150,
            Some(ResearchProfile::ConfigSurface) => 160,
            Some(ResearchProfile::Testing) => 110,
            Some(ResearchProfile::Runtime) => 90,
            _ => 40,
        };
    }
    if file.is_docs_like() {
        score += match research_profile {
            Some(ResearchProfile::ApiSurface) => 10,
            Some(ResearchProfile::ConfigSurface) => -20,
            Some(ResearchProfile::DocsGuide)
            | Some(ResearchProfile::Troubleshooting)
            | Some(ResearchProfile::ExampleTutorial) => 80,
            _ if is_high_level_page(page_kind) => 180,
            _ => 35,
        };
    }
    if file.is_config_like() {
        score += match page_kind {
            ResearchPageKind::ConfigSurface => 200,
            kind if is_high_level_page(kind) => 120,
            _ => 20,
        };
    }
    if file.is_entry_like() {
        score += if is_high_level_page(page_kind) {
            140
        } else {
            60
        };
    }
    if file.is_substantive_source() {
        score += 90;
    }
    if is_repo_overview_doc_path(&lower_path) {
        score += if is_high_level_page(page_kind) {
            220
        } else {
            40
        };
    }
    if is_root_manifest_or_build_path(&lower_path) {
        score += if is_high_level_page(page_kind) {
            170
        } else {
            50
        };
    }
    if is_public_api_like_path(&lower_path) {
        score += match page_kind {
            ResearchPageKind::ApiSurface => {
                if lower_path.ends_with("/public-types.ts") {
                    120
                } else {
                    170
                }
            }
            kind if is_high_level_page(kind) => 90,
            _ => 45,
        };
    }
    if is_processor_or_runtime_core_path(&lower_path) {
        score += match page_kind {
            ResearchPageKind::ApiSurface => 110,
            kind if is_high_level_page(kind) => 130,
            _ => 45,
        };
    }
    if lower_path.contains("/src/") || lower_path.contains("/main/java/") {
        score += 20;
    }

    score += match file.language.as_str() {
        "markdown" | "mdx" => {
            if is_high_level_page(page_kind) {
                35
            } else {
                5
            }
        }
        "typescript" | "javascript" | "react" | "vue" | "svelte" | "python" | "rust" | "java"
        | "csharp" | "kotlin" | "php" | "swift" => 35,
        _ => 0,
    };

    score += topic_focus_score(topic_focus, &path, &file_name);
    score
}

fn topic_focus_score(topic_focus: Option<&TopicFocusHints>, path: &str, basename: &str) -> i32 {
    let Some(topic_focus) = topic_focus else {
        return 0;
    };

    let path_tokens = extract_focus_tokens(path);
    let shared_focus = path_tokens.intersection(&topic_focus.focus_tokens).count() as i32;
    let shared_anchor = path_tokens.intersection(&topic_focus.anchor_tokens).count() as i32;
    let semantic_anchor = topic_focus_semantic_anchor_overlap(topic_focus, &path_tokens) as i32;

    let mut score = shared_focus.min(4) * 12 + shared_anchor.min(4) * 220;
    if shared_anchor >= 2 {
        score += 60;
    }
    if matches!(topic_focus.kind, Some(TopicFocusKind::TypeSystem)) {
        if path.contains("/preview-api/")
            || path.contains("/manager-api/")
            || path.contains("/csf/")
        {
            score += 180;
        }
    }

    if is_generic_topic_basename(basename) {
        score -= match (topic_focus.kind, shared_anchor, semantic_anchor) {
            (Some(TopicFocusKind::TypeSystem), _, 0) if is_generic_contract_basename(basename) => {
                700
            }
            (_, 0, _) => 300,
            (_, 1, _) => 24,
            _ => 0,
        };
    }

    score
}

fn topic_focus_semantic_anchor_overlap(
    topic_focus: &TopicFocusHints,
    path_tokens: &BTreeSet<String>,
) -> usize {
    let Some(kind) = topic_focus.kind else {
        return 0;
    };
    let semantic_anchor_tokens = topic_focus_anchor_tokens(kind);
    path_tokens.intersection(&semantic_anchor_tokens).count()
}

fn topic_focus_anchor_overlap(topic_focus: Option<&TopicFocusHints>, path: &str) -> usize {
    let Some(topic_focus) = topic_focus else {
        return 0;
    };
    extract_focus_tokens(path)
        .intersection(&topic_focus.anchor_tokens)
        .count()
}

fn is_generic_topic_basename(basename: &str) -> bool {
    matches!(
        basename,
        "index.ts"
            | "index.tsx"
            | "manager.ts"
            | "manager.tsx"
            | "preset.js"
            | "preset.ts"
            | "preview.js"
            | "preview.ts"
            | "preview.tsx"
            | "public-types.ts"
            | "types.ts"
            | "typings.d.ts"
    )
}

fn is_generic_contract_basename(basename: &str) -> bool {
    matches!(basename, "public-types.ts" | "types.ts" | "typings.d.ts")
}

fn classify_key_source_role(
    research_profile: Option<&ResearchProfile>,
    file: &ScannedFile,
    path: &str,
) -> KeySourceRole {
    let lower_path = path.to_ascii_lowercase();
    if is_sample_noise_path(&lower_path) && !allows_sample_noise(research_profile) {
        return KeySourceRole::Sample;
    }
    if file.is_docs_like() {
        return KeySourceRole::Docs;
    }
    if file.is_config_like() {
        return KeySourceRole::Config;
    }
    if is_real_test_case_path(&lower_path) || file.is_test_like() {
        return KeySourceRole::Verification;
    }
    if is_internal_support_path(&lower_path) {
        return KeySourceRole::Support;
    }
    if file.is_entry_like() {
        return KeySourceRole::Entry;
    }
    if is_contract_like_path(file, &lower_path) {
        return KeySourceRole::Contract;
    }
    KeySourceRole::Implementation
}

fn key_source_family(basename: &str, path: &str, role: KeySourceRole) -> String {
    let lower_path = path.to_ascii_lowercase();
    if matches!(role, KeySourceRole::Sample) {
        return "sample-noise".to_string();
    }
    if matches!(role, KeySourceRole::Docs) {
        return "docs-reference".to_string();
    }
    if matches!(role, KeySourceRole::Config) {
        return if basename.starts_with("tsconfig") {
            "tsconfig".to_string()
        } else if lower_path.contains(".storybook/") {
            "storybook-config".to_string()
        } else {
            "config-surface".to_string()
        };
    }
    if matches!(role, KeySourceRole::Verification) {
        return if lower_path.contains("/javatests/") {
            "javatest-case".to_string()
        } else {
            "test-case".to_string()
        };
    }
    if matches!(role, KeySourceRole::Support) {
        return if lower_path.contains("/internal/testing/") {
            "internal-testing".to_string()
        } else if lower_path.contains("/internal/") {
            "internal-support".to_string()
        } else {
            "support".to_string()
        };
    }
    if lower_path.ends_with("/public-types.ts") {
        return "public-types".to_string();
    }
    if lower_path.ends_with("/typings.d.ts") {
        return "typings".to_string();
    }
    if basename == "types.ts" || basename.ends_with(".types.ts") {
        return "types-contract".to_string();
    }
    if lower_path.contains("/validation/") {
        return "validation".to_string();
    }
    if lower_path.contains("codegen") || lower_path.contains("processor") {
        return "processor-codegen".to_string();
    }
    if lower_path.contains("/preview-api/") {
        return "preview-api".to_string();
    }
    if lower_path.contains("/manager-api/") {
        return "manager-api".to_string();
    }
    if lower_path.contains("/csf/") {
        return "csf".to_string();
    }
    if lower_path.contains("/runtime/") || lower_path.contains("dagger-runtime") {
        return "runtime-core".to_string();
    }
    role_family_name(role).to_string()
}

fn role_family_name(role: KeySourceRole) -> &'static str {
    match role {
        KeySourceRole::Entry => "entry",
        KeySourceRole::Contract => "contract",
        KeySourceRole::Implementation => "implementation",
        KeySourceRole::Verification => "verification",
        KeySourceRole::Support => "support",
        KeySourceRole::Docs => "docs",
        KeySourceRole::Config => "config",
        KeySourceRole::Sample => "sample",
    }
}

fn key_source_family_limit(
    research_profile: Option<&ResearchProfile>,
    role: KeySourceRole,
    family: &str,
) -> usize {
    match research_profile {
        Some(ResearchProfile::ApiSurface) => match family {
            "public-types" => 1,
            "docs-reference" => 1,
            "internal-support" | "internal-testing" => 1,
            _ => match role {
                KeySourceRole::Support => 2,
                _ => 3,
            },
        },
        Some(ResearchProfile::ConfigSurface) => match family {
            "docs-reference" => 1,
            "storybook-config" => 3,
            "tsconfig" => 1,
            "internal-support" | "internal-testing" => 1,
            _ => match role {
                KeySourceRole::Support => 1,
                _ => 3,
            },
        },
        Some(ResearchProfile::Testing) => match family {
            "internal-testing" => 1,
            "internal-support" => 1,
            "docs-reference" => 1,
            _ => match role {
                KeySourceRole::Verification => 6,
                _ => 3,
            },
        },
        _ => 3,
    }
}

fn should_expand_profile_fallback_candidates(
    scored: &[ScoredKeySourceCandidate],
    budget: usize,
) -> bool {
    if scored.len() < budget.saturating_div(2).max(2) {
        return true;
    }

    let mut family_counts = BTreeMap::<&str, usize>::new();
    let mut role_counts = BTreeMap::<KeySourceRole, usize>::new();
    for candidate in scored {
        *family_counts.entry(candidate.family.as_str()).or_default() += 1;
        *role_counts.entry(candidate.role).or_default() += 1;
    }

    let dominant_family = family_counts.values().copied().max().unwrap_or(0);
    let dominant_role = role_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(role, count)| (*role, *count));

    dominant_family * 2 >= scored.len()
        || dominant_role
            .map(|(role, count)| {
                count * 2 >= scored.len()
                    && matches!(
                        role,
                        KeySourceRole::Docs
                            | KeySourceRole::Support
                            | KeySourceRole::Sample
                            | KeySourceRole::Config
                    )
            })
            .unwrap_or(false)
}

fn extend_with_profile_fallback_candidates(
    report: &ScanReport,
    page_kind: ResearchPageKind,
    research_profile: Option<&ResearchProfile>,
    topic_focus: Option<&TopicFocusHints>,
    scored: &mut Vec<ScoredKeySourceCandidate>,
) {
    let mut existing_paths = scored
        .iter()
        .map(|candidate| candidate.path.clone())
        .collect::<BTreeSet<_>>();

    for file in &report.files {
        if !is_profile_fallback_candidate(research_profile, file) {
            continue;
        }
        let path = normalize_path(&file.path);
        if existing_paths.contains(&path) {
            continue;
        }
        let score = structural_key_source_score(
            page_kind,
            research_profile,
            topic_focus,
            file,
            &[ResearchKeySourceOrigin::GlobalFallback],
        );
        if score <= -1000 {
            continue;
        }
        let basename = Path::new(&path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let role = classify_key_source_role(research_profile, file, &path);
        let family = key_source_family(&basename, &path, role);
        scored.push(ScoredKeySourceCandidate {
            score,
            path: path.clone(),
            basename,
            role,
            family,
        });
        existing_paths.insert(path);
    }
}

fn is_profile_fallback_candidate(
    research_profile: Option<&ResearchProfile>,
    file: &ScannedFile,
) -> bool {
    let path = normalize_path(&file.path).to_ascii_lowercase();
    if file.is_asset_like() || is_generated_binary_path(&path) {
        return false;
    }

    match research_profile {
        Some(ResearchProfile::ApiSurface) => {
            is_contract_like_path(file, &path)
                || file.is_entry_like()
                || path.contains("/manager-api/")
                || path.contains("/preview-api/")
                || path.contains("/channels/")
                || path.contains("/csf/")
                || is_processor_or_runtime_core_path(&path)
        }
        Some(ResearchProfile::ConfigSurface) => {
            file.is_config_like()
                || file.is_entry_like()
                || is_contract_like_path(file, &path)
                || path.contains("/theming/")
                || path.contains("/preset")
                || path.contains("/preview")
                || path.contains("/manager")
                || path.contains("config")
                || path.contains("setup")
        }
        Some(ResearchProfile::Testing) => {
            is_real_test_case_path(&path)
                || file.is_test_like()
                || is_contract_like_path(file, &path)
                || path.contains("/testing/")
                || path.contains("/tests/")
                || path.contains("/javatests/")
                || path.contains("/spec/")
                || path.contains("runner")
                || path.contains("harness")
                || path.contains("setup")
        }
        Some(ResearchProfile::Runtime) => {
            is_contract_like_path(file, &path)
                || path.contains("/validation/")
                || path.contains("/binding/")
                || path.contains("/multibindings/")
                || is_processor_or_runtime_core_path(&path)
        }
        Some(ResearchProfile::IntegrationPlatform) => {
            file.is_config_like()
                || file.is_entry_like()
                || is_real_test_case_path(&path)
                || path.contains("playwright")
                || path.contains("vitest")
                || path.contains("storybook.setup")
                || path.contains("runner")
                || path.contains("harness")
        }
        _ => false,
    }
}

fn is_contract_like_path(file: &ScannedFile, path: &str) -> bool {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    file.purpose == FilePurpose::Type
        || file_name.ends_with(".d.ts")
        || file_name == "types.ts"
        || file_name.ends_with(".types.ts")
        || file_name == "public-types.ts"
        || file_name == "api.ts"
        || is_public_java_contract_path(path)
}

fn is_public_java_contract_path(path: &str) -> bool {
    let lower_path = path.to_ascii_lowercase();
    lower_path.contains("/main/java/")
        && lower_path.ends_with(".java")
        && !lower_path.contains("/internal/")
        && !lower_path.contains("/processor/")
        && !lower_path.contains("/impl/")
}

fn is_real_test_case_path(path: &str) -> bool {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    path.contains("/javatests/")
        || path.contains("/tests/")
        || path.contains("/__tests__/")
        || file_name.ends_with(".test.ts")
        || file_name.ends_with(".test.tsx")
        || file_name.ends_with(".spec.ts")
        || file_name.ends_with(".spec.tsx")
        || file_name.ends_with("test.java")
        || file_name.ends_with("tests.java")
        || file_name.ends_with("test.kt")
        || file_name.ends_with("test.rs")
}

fn is_internal_support_path(path: &str) -> bool {
    path.contains("/internal/testing/")
        || path.contains("/testing/internal/")
        || path.contains("/internal/")
        || path.contains("/support/")
        || path.contains("/testutil/")
        || path.contains("/test-util/")
}

fn fallback_structural_key_source_score(file: &ScannedFile) -> i32 {
    let path = normalize_path(&file.path).to_ascii_lowercase();
    let file_name = Path::new(&path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let mut score = file.purpose.signal_weight();
    if is_repo_overview_doc_path(&path) {
        score += 120;
    }
    if is_root_manifest_or_build_path(&path) {
        score += 110;
    }
    if file.is_entry_like() {
        score += 90;
    }
    if is_root_governance_noise(file_name, &path) || is_bootstrap_wrapper_noise(file_name) {
        score -= 200;
    }
    score
}

fn origin_score(page_kind: ResearchPageKind, origin: ResearchKeySourceOrigin) -> i32 {
    match (page_kind, origin) {
        (_, ResearchKeySourceOrigin::PlannerGrounded) => 130,
        (ResearchPageKind::ConfigSurface, ResearchKeySourceOrigin::ConfigSurface) => 170,
        (ResearchPageKind::ApiSurface, ResearchKeySourceOrigin::DocsAnchor) => 100,
        (ResearchPageKind::ApiSurface, ResearchKeySourceOrigin::DocReference) => 100,
        (kind, ResearchKeySourceOrigin::RepoEntry) if is_high_level_page(kind) => 140,
        (kind, ResearchKeySourceOrigin::DocsAnchor) if is_high_level_page(kind) => 130,
        (kind, ResearchKeySourceOrigin::DocReference) if is_high_level_page(kind) => 120,
        (kind, ResearchKeySourceOrigin::ConfigSurface) if is_high_level_page(kind) => 110,
        (_, ResearchKeySourceOrigin::ScopeSource) => 85,
        (_, ResearchKeySourceOrigin::ModuleKeySource) => 70,
        (_, ResearchKeySourceOrigin::RepoEntry) => 60,
        (_, ResearchKeySourceOrigin::DocsAnchor) => 55,
        (_, ResearchKeySourceOrigin::DocReference) => 55,
        (_, ResearchKeySourceOrigin::ConfigSurface) => 55,
        (_, ResearchKeySourceOrigin::GlobalFallback) => 0,
    }
}

fn research_page_kind(
    unit: &KnowledgeUnit,
    research_profile: Option<&ResearchProfile>,
) -> ResearchPageKind {
    match unit.unit_type {
        UnitType::Overview => ResearchPageKind::SystemOverview,
        UnitType::Architecture => ResearchPageKind::SystemArchitecture,
        UnitType::DomainIndex => ResearchPageKind::DomainIndex,
        _ => match research_profile {
            Some(ResearchProfile::ApiSurface) => ResearchPageKind::ApiSurface,
            Some(ResearchProfile::ConfigSurface) => ResearchPageKind::ConfigSurface,
            _ => ResearchPageKind::Other,
        },
    }
}

fn is_high_level_page(page_kind: ResearchPageKind) -> bool {
    matches!(
        page_kind,
        ResearchPageKind::SystemOverview
            | ResearchPageKind::SystemArchitecture
            | ResearchPageKind::DomainIndex
    )
}

fn allows_sample_noise(research_profile: Option<&ResearchProfile>) -> bool {
    matches!(research_profile, Some(ResearchProfile::ExampleTutorial))
}

fn is_global_structural_candidate(file: &ScannedFile) -> bool {
    let path = normalize_path(&file.path).to_ascii_lowercase();
    file.is_entry_like()
        || file.is_config_like()
        || file.is_docs_like()
        || file.is_substantive_source()
        || is_repo_overview_doc_path(&path)
        || is_root_manifest_or_build_path(&path)
        || is_public_api_like_path(&path)
        || is_processor_or_runtime_core_path(&path)
}

fn is_generated_binary_path(path: &str) -> bool {
    path.ends_with(".jar") || path.ends_with(".min.js") || path.ends_with(".bundle.js")
}

fn is_lockfile(file_name: &str) -> bool {
    matches!(
        file_name,
        "pnpm-lock.yaml" | "package-lock.json" | "yarn.lock" | "cargo.lock"
    )
}

fn is_root_governance_noise(file_name: &str, path: &str) -> bool {
    if path.contains('/') {
        return false;
    }

    matches!(
        file_name,
        ".env"
            | ".env.example"
            | ".gitignore"
            | ".mailmap"
            | ".nvmrc"
            | ".spelling"
            | "authors"
            | "code_of_conduct.md"
            | "contributing.md"
            | "license"
            | "license.md"
            | "license.txt"
            | "maintainers.md"
    ) || file_name.starts_with("changelog")
}

fn is_bootstrap_wrapper_noise(file_name: &str) -> bool {
    matches!(file_name, "gradlew" | "gradlew.bat" | ".bazelrc")
}

fn is_sample_noise_path(path: &str) -> bool {
    path.starts_with("test-storybooks/")
        || path.contains("/test-storybooks/")
        || path.starts_with("kitchen-sink/")
        || path.contains("/kitchen-sink/")
        || path.starts_with("__mocks")
        || path.contains("/__mocks")
        || path.contains("/__mocks-")
        || path.starts_with("fixtures/")
        || path.contains("/fixtures/")
        || path.starts_with("__fixtures__/")
        || path.contains("/__fixtures__/")
        || path.starts_with("testdata/")
        || path.contains("/testdata/")
        || path.starts_with("sandbox/")
        || path.contains("/sandbox/")
}

fn is_repo_overview_doc_path(path: &str) -> bool {
    path == "readme.md"
        || path.ends_with("/readme.md")
        || path.starts_with("docs/index.")
        || path.ends_with("/docs/index.md")
        || path.ends_with("/docs/index.mdx")
        || path.ends_with("/index.mdx")
}

fn is_root_manifest_or_build_path(path: &str) -> bool {
    matches!(
        path,
        "package.json"
            | "pnpm-workspace.yaml"
            | "turbo.json"
            | "nx.json"
            | "settings.gradle.kts"
            | "settings.gradle"
            | "gradle.properties"
            | "build.gradle"
            | "build.gradle.kts"
            | "cargo.toml"
            | "pyproject.toml"
            | "pom.xml"
    )
}

fn is_public_api_like_path(path: &str) -> bool {
    path.ends_with("/index.ts")
        || path.ends_with("/index.js")
        || path.ends_with("/public-types.ts")
        || path.ends_with("/types.ts")
        || path.ends_with("/api.ts")
        || path.ends_with("/component.java")
        || path.ends_with("/module.java")
        || path.ends_with("/subcomponent.java")
}

fn is_processor_or_runtime_core_path(path: &str) -> bool {
    path.contains("processor")
        || path.contains("/runtime/")
        || path.contains(".storybook/main.")
        || path.contains(".storybook/preview.")
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn sanitize_path_segment(name: &str) -> String {
    name.chars()
        .map(|character| match character {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => character,
        })
        .collect::<String>()
        .trim()
        .trim_matches('.')
        .to_string()
}

fn build_skeleton_profile(
    profile_key: &str,
    section_plan: &[PlannedSection],
) -> Option<SkeletonProfile> {
    let seed_sections = section_plan
        .iter()
        .map(|section| SkeletonSection {
            section_key: section.section_key.clone(),
            title: section.title.clone(),
        })
        .collect::<Vec<_>>();

    (!seed_sections.is_empty()).then_some(SkeletonProfile {
        profile_key: profile_key.to_string(),
        seed_sections,
    })
}

fn build_key_source_clusters(
    key_sources: &[String],
    evidence_clusters: &[EvidenceCluster],
) -> Vec<KeySourceCluster> {
    key_sources
        .iter()
        .enumerate()
        .map(|(index, path)| {
            let evidence_cluster_keys = evidence_clusters
                .iter()
                .filter(|cluster| {
                    cluster
                        .citations
                        .iter()
                        .any(|citation| citation.path == *path)
                })
                .map(|cluster| cluster.cluster_key.clone())
                .collect::<Vec<_>>();
            KeySourceCluster {
                cluster_key: format!("key-source:{index}:{}", stable_id("source-path", path)),
                label: path.clone(),
                source_paths: vec![path.clone()],
                evidence_cluster_keys,
            }
        })
        .collect()
}

fn build_section_grounding_refs(
    section_plan: &[PlannedSection],
    key_source_clusters: &[KeySourceCluster],
) -> Vec<SectionGroundingRef> {
    section_plan
        .iter()
        .enumerate()
        .map(|(section_index, section)| SectionGroundingRef {
            section_key: section.section_key.clone(),
            key_source_cluster_keys: suggested_key_source_cluster_keys_for_section(
                section,
                key_source_clusters,
                section_index,
                section_plan.len(),
            ),
            evidence_cluster_keys: section.evidence_cluster_keys.clone(),
            child_digest_refs: Vec::new(),
            diagram_refs: Vec::new(),
        })
        .collect()
}

fn suggested_key_source_cluster_keys_for_section(
    section: &PlannedSection,
    key_source_clusters: &[KeySourceCluster],
    section_index: usize,
    _section_count: usize,
) -> Vec<String> {
    let mut matches = Vec::new();
    let normalized_title =
        canonical_reference_outline_title(section.title.trim()).unwrap_or(section.title.trim());

    if !section.evidence_cluster_keys.is_empty() {
        for cluster in key_source_clusters {
            if cluster.evidence_cluster_keys.iter().any(|cluster_key| {
                section
                    .evidence_cluster_keys
                    .iter()
                    .any(|section_key| section_key == cluster_key)
            }) {
                matches.push(cluster.cluster_key.clone());
            }
        }
    }

    let limit = match normalized_title {
        "附录" => 4,
        "概述" | "API 概览" | "关键接口" | "使用边界" => 3,
        "简介" | "项目结构" | "核心组件" | "架构总览" | "详细组件分析" | "依赖关系分析"
        | "性能考量" | "故障排查指南" | "结论" => 3,
        _ => 2,
    };

    if matches.len() < limit && uses_structural_fallback_key_sources(normalized_title) {
        let start = if key_source_clusters.is_empty() {
            0
        } else {
            (section_index * limit) % key_source_clusters.len()
        };
        for offset in 0..key_source_clusters.len() {
            let cluster = &key_source_clusters[(start + offset) % key_source_clusters.len()];
            if !matches
                .iter()
                .any(|existing| existing == &cluster.cluster_key)
            {
                matches.push(cluster.cluster_key.clone());
            }
            if matches.len() >= limit {
                break;
            }
        }
    }

    matches.truncate(limit);
    matches
}

fn uses_structural_fallback_key_sources(title: &str) -> bool {
    matches!(
        title,
        "概述"
            | "API 概览"
            | "关键接口"
            | "使用边界"
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

fn build_structural_seed_evidence_clusters(
    prefix: &str,
    key_sources: &[String],
    report: &ScanReport,
) -> Vec<EvidenceCluster> {
    key_sources
        .iter()
        .enumerate()
        .filter_map(|(index, path)| {
            let scanned = report
                .files
                .iter()
                .find(|file| normalize_path(&file.path) == normalize_path(path))?;
            Some(EvidenceCluster {
                cluster_key: format!("{prefix}:seed:{index}"),
                label: path.clone(),
                citations: vec![SourceCitation {
                    path: normalize_path(&scanned.path),
                    start_line: 1,
                    end_line: 1,
                    source_id: Some(scanned.id.clone()),
                    symbol_id: None,
                    note: "structural seed".to_string(),
                }],
            })
        })
        .collect()
}

fn build_page_seed(
    profile_key: &str,
    summary: &str,
    positioning: &str,
    section_plan: &[PlannedSection],
    key_sources: &[String],
    evidence_clusters: &[EvidenceCluster],
    diagram_suggestions: &[DiagramSuggestion],
) -> ResearchPageSeed {
    let key_source_clusters = build_key_source_clusters(key_sources, evidence_clusters);
    ResearchPageSeed {
        summary: summary.to_string(),
        positioning: positioning.to_string(),
        section_plan: section_plan.to_vec(),
        skeleton_profile: build_skeleton_profile(profile_key, section_plan),
        section_grounding_refs: build_section_grounding_refs(section_plan, &key_source_clusters),
        key_source_clusters,
        evidence_clusters: evidence_clusters.to_vec(),
        diagram_suggestions: diagram_suggestions.to_vec(),
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

    build_reference_outline_section_plan(
        unit,
        research_profile,
        child_digests,
        &lead_summary,
        &outline_summary,
    )
}

fn build_reference_outline_section_plan(
    unit: &KnowledgeUnit,
    research_profile: Option<&ResearchProfile>,
    child_digests: &[PageDigest],
    lead_summary: &str,
    outline_summary: &str,
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
            lead_summary.to_string(),
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
            outline_summary.to_string(),
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
            outline_summary.to_string(),
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
    is_reference_outline_title(title)
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
        if matches!(
            profile,
            ResearchProfile::ApiSurface | ResearchProfile::ConfigSurface
        ) {
            return build_reference_outline_section_plan(
                unit,
                Some(profile),
                child_digests,
                "",
                "",
            );
        }
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

    if matches!(
        unit.unit_type,
        UnitType::Overview | UnitType::Architecture | UnitType::DomainIndex
    ) {
        return build_reference_outline_section_plan(unit, None, child_digests, "", "");
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
    let section_title =
        canonical_reference_outline_title(section.title.trim()).unwrap_or(section.title.trim());
    let bucket_order: &[&str] = match section_title {
        "概述" => &["docs-references", "docs-anchor", "key-sources"],
        "API 概览" => &["api-surface", "key-sources", "docs-references"],
        "关键接口" => &["api-surface", "key-sources", "docs-references"],
        "使用边界" => &[
            "config-surface",
            "docs-references",
            "docs-anchor",
            "key-sources",
        ],
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
    is_reference_outline_title(title)
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
        build_default_section_plan, build_docs_backed_section_plan, build_section_grounding_refs,
        build_unit_topic_focus_hints, collect_doc_reference_citations, research_domain_structural,
        research_system_structural, research_unit_structural, seed_section_evidence_clusters,
        select_structural_key_sources, structural_key_source_score, EvidenceCluster,
        ResearchDataSource, ResearchKeySourceCandidate, ResearchKeySourceOrigin, ResearchPageKind,
    };
    use crate::domain::context::{ModuleContext, RepoContext};
    use crate::domain::research::{
        KeySourceCluster, PlannedSection, ResearchProfile, SourceCitation,
    };
    use std::fs;
    use tempfile::tempdir;
    use wiki_index::scanner::{DependencyHint, FilePurpose, ScanReport, ScannedFile};
    use wiki_index::symbol_graph::{GraphAnalysisSnapshot, GraphSummary, ResolvedGraphSnapshot};
    use wiki_index::symbols::ParsedSymbolsSnapshot;
    use wiki_model::domain::knowledge::{
        ConfigSurface, DocsAnchor, DomainType, KnowledgeDomain, KnowledgeTree, KnowledgeUnit,
        UnitScope, UnitType,
    };
    use wiki_model::domain::module_tree::ModuleTree;

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

    #[test]
    fn api_sections_ground_only_relevant_top_key_sources() {
        let mut section_plan = vec![
            PlannedSection {
                section_key: "overview".to_string(),
                title: "概述".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
            PlannedSection {
                section_key: "api-surface".to_string(),
                title: "API 概览".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
            PlannedSection {
                section_key: "usage-boundary".to_string(),
                title: "使用边界".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
        ];
        let evidence_clusters = vec![
            cluster("cluster:key-sources", "src/impl.ts"),
            cluster("cluster:api-surface", "src/public-types.ts"),
            cluster("cluster:docs-references", "docs/api.md"),
            cluster("cluster:config-surface", ".storybook/main.ts"),
        ];

        seed_section_evidence_clusters(&mut section_plan, &evidence_clusters);

        let key_source_clusters = vec![
            KeySourceCluster {
                cluster_key: "ks:impl".to_string(),
                label: "impl".to_string(),
                source_paths: vec!["src/impl.ts".to_string()],
                evidence_cluster_keys: vec!["cluster:key-sources".to_string()],
            },
            KeySourceCluster {
                cluster_key: "ks:api".to_string(),
                label: "api".to_string(),
                source_paths: vec!["src/public-types.ts".to_string()],
                evidence_cluster_keys: vec!["cluster:api-surface".to_string()],
            },
            KeySourceCluster {
                cluster_key: "ks:docs".to_string(),
                label: "docs".to_string(),
                source_paths: vec!["docs/api.md".to_string()],
                evidence_cluster_keys: vec!["cluster:docs-references".to_string()],
            },
            KeySourceCluster {
                cluster_key: "ks:config".to_string(),
                label: "config".to_string(),
                source_paths: vec![".storybook/main.ts".to_string()],
                evidence_cluster_keys: vec!["cluster:config-surface".to_string()],
            },
        ];

        let grounding_refs = build_section_grounding_refs(&section_plan, &key_source_clusters);
        let overview = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "overview")
            .expect("overview grounding should exist");
        let api_surface = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "api-surface")
            .expect("api surface grounding should exist");
        let usage_boundary = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "usage-boundary")
            .expect("usage boundary grounding should exist");

        assert!(overview.key_source_cluster_keys.len() <= 3);
        assert!(overview
            .key_source_cluster_keys
            .iter()
            .any(|key| key == "ks:docs"));
        assert!(api_surface
            .key_source_cluster_keys
            .iter()
            .any(|key| key == "ks:api"));
        assert!(!api_surface
            .key_source_cluster_keys
            .iter()
            .any(|key| key == "ks:config"));
        assert!(usage_boundary
            .key_source_cluster_keys
            .iter()
            .any(|key| key == "ks:config"));
    }

    #[test]
    fn build_section_grounding_refs_distributes_fallback_key_sources_across_sections() {
        let section_plan = vec![
            PlannedSection {
                section_key: "intro".to_string(),
                title: "简介".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
            PlannedSection {
                section_key: "components".to_string(),
                title: "核心组件".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
            PlannedSection {
                section_key: "dependencies".to_string(),
                title: "依赖关系分析".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            },
        ];
        let key_source_clusters = (0..6)
            .map(|index| KeySourceCluster {
                cluster_key: format!("ks:{index}"),
                label: format!("source-{index}"),
                source_paths: vec![format!("src/file-{index}.ts")],
                evidence_cluster_keys: Vec::new(),
            })
            .collect::<Vec<_>>();

        let grounding_refs = build_section_grounding_refs(&section_plan, &key_source_clusters);
        let intro = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "intro")
            .expect("intro grounding should exist");
        let components = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "components")
            .expect("components grounding should exist");
        let dependencies = grounding_refs
            .iter()
            .find(|grounding| grounding.section_key == "dependencies")
            .expect("dependencies grounding should exist");

        assert_eq!(intro.key_source_cluster_keys, vec!["ks:0", "ks:1", "ks:2"]);
        assert_eq!(
            components.key_source_cluster_keys,
            vec!["ks:3", "ks:4", "ks:5"]
        );
        assert_eq!(
            dependencies.key_source_cluster_keys,
            vec!["ks:0", "ks:1", "ks:2"]
        );
    }

    #[test]
    fn system_research_prefers_docs_and_build_entries_over_root_noise() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with("README.md", "docs", FilePurpose::Docs, &[], "markdown"),
                scanned_file_with("docs/index.mdx", "docs", FilePurpose::Docs, &[], "mdx"),
                scanned_file_with(
                    "package.json",
                    "config",
                    FilePurpose::Config,
                    &["entry-point"],
                    "json",
                ),
                scanned_file_with(
                    "code/.storybook/main.ts",
                    "config",
                    FilePurpose::Config,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/preview-api/modules/preview-web/docs-context/DocsContext.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/.storybook/preview.tsx",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "react",
                ),
                scanned_file_with(
                    "code/core/package.json",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "json",
                ),
                scanned_file_with(
                    "code/frameworks/react-vite/src/node/index.ts",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(".env", "config", FilePurpose::Config, &[], "text"),
                scanned_file_with(".nvmrc", "config", FilePurpose::Config, &[], "text"),
                scanned_file_with("LICENSE", "docs", FilePurpose::Docs, &[], "text"),
                scanned_file_with(".mailmap", "docs", FilePurpose::Docs, &[], "text"),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: vec![
                "package.json".to_string(),
                "code/.storybook/main.ts".to_string(),
            ],
            entry_points: vec!["code/.storybook/main.ts".to_string()],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let mut ds_env = test_research_ds(&report);
        ds_env.repo_context.key_entry_points = vec![
            ".env".to_string(),
            ".nvmrc".to_string(),
            "LICENSE".to_string(),
            "README.md".to_string(),
            "package.json".to_string(),
        ];
        ds_env.module_contexts = vec![module_context(
            "storybook-core",
            vec![
                ".mailmap".to_string(),
                "docs/index.mdx".to_string(),
                "code/.storybook/main.ts".to_string(),
                "code/.storybook/preview.tsx".to_string(),
                "code/core/package.json".to_string(),
                "code/frameworks/react-vite/src/node/index.ts".to_string(),
                "code/core/src/preview-api/modules/preview-web/docs-context/DocsContext.ts"
                    .to_string(),
            ],
        )];

        let research = research_system_structural(&ds_env.as_ds(&report));
        let key_sources = research
            .overview_seed
            .key_source_clusters
            .iter()
            .map(|cluster| cluster.label.clone())
            .collect::<Vec<_>>();

        assert!(key_sources.iter().any(|path| path == "README.md"));
        assert!(key_sources.iter().any(|path| path == "docs/index.mdx"));
        assert!(key_sources.iter().any(|path| path == "package.json"));
        assert!(key_sources
            .iter()
            .any(|path| path == "code/.storybook/main.ts"));
        assert!(!key_sources.iter().any(|path| path == ".env"));
        assert!(!key_sources.iter().any(|path| path == ".nvmrc"));
        assert!(!key_sources.iter().any(|path| path == "LICENSE"));
        assert!(!key_sources.iter().any(|path| path == ".mailmap"));
    }

    #[test]
    fn domain_research_uses_reference_outline_and_reorders_domain_noise() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with("docs/index.mdx", "docs", FilePurpose::Docs, &[], "mdx"),
                scanned_file_with(
                    "settings.gradle.kts",
                    "config",
                    FilePurpose::Config,
                    &["entry-point"],
                    "kotlin",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Component.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Module.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Subcomponent.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "hilt-core/main/java/dagger/hilt/DefineComponent.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "gradle.properties",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "kotlin",
                ),
                scanned_file_with("AUTHORS", "docs", FilePurpose::Docs, &[], "text"),
                scanned_file_with("LICENSE.txt", "docs", FilePurpose::Docs, &[], "text"),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: vec!["settings.gradle.kts".to_string()],
            entry_points: vec!["settings.gradle.kts".to_string()],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let mut ds_env = test_research_ds(&report);
        ds_env.module_contexts = vec![module_context(
            "dagger-core",
            vec![
                "AUTHORS".to_string(),
                "docs/index.mdx".to_string(),
                "settings.gradle.kts".to_string(),
                "gradle.properties".to_string(),
                "dagger-runtime/main/java/dagger/Module.java".to_string(),
                "dagger-runtime/main/java/dagger/Subcomponent.java".to_string(),
                "hilt-core/main/java/dagger/hilt/DefineComponent.java".to_string(),
                "dagger-runtime/main/java/dagger/Component.java".to_string(),
                "dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java"
                    .to_string(),
            ],
        )];
        let mut domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心模块");
        domain.source_modules = vec!["dagger-core".to_string()];
        domain.source_files = vec![
            "AUTHORS".to_string(),
            "LICENSE.txt".to_string(),
            "docs/index.mdx".to_string(),
            "settings.gradle.kts".to_string(),
            "gradle.properties".to_string(),
            "dagger-runtime/main/java/dagger/Component.java".to_string(),
            "dagger-runtime/main/java/dagger/Module.java".to_string(),
            "dagger-runtime/main/java/dagger/Subcomponent.java".to_string(),
            "hilt-core/main/java/dagger/hilt/DefineComponent.java".to_string(),
            "dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java".to_string(),
        ];
        domain.evidence.docs_anchors = vec![DocsAnchor {
            file_path: "docs/index.mdx".to_string(),
            heading: "Overview".to_string(),
            level: 1,
            links: Vec::new(),
        }];

        let research = research_domain_structural(&domain, &ds_env.as_ds(&report));
        let section_titles = research
            .compose_seed
            .section_plan
            .iter()
            .map(|section| section.title.as_str())
            .collect::<Vec<_>>();
        let key_sources = research
            .compose_seed
            .key_source_clusters
            .iter()
            .map(|cluster| cluster.label.clone())
            .collect::<Vec<_>>();

        assert!(section_titles.contains(&"简介"));
        assert!(section_titles.contains(&"项目结构"));
        assert!(section_titles.contains(&"核心组件"));
        assert!(section_titles.contains(&"架构总览"));
        assert!(key_sources.iter().any(|path| path == "docs/index.mdx"));
        assert!(key_sources.iter().any(|path| path == "settings.gradle.kts"));
        assert!(key_sources
            .iter()
            .any(|path| path.ends_with("ComponentProcessor.java")));
        assert!(!key_sources.iter().any(|path| path == "AUTHORS"));
        assert!(!key_sources.iter().any(|path| path == "LICENSE.txt"));
    }

    #[test]
    fn config_unit_research_prefers_real_config_entries_over_storybook_samples() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "code/.storybook/main.ts",
                    "config",
                    FilePurpose::Config,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(
                    "code/.storybook/preview.tsx",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "react",
                ),
                scanned_file_with(
                    "code/core/src/common/utils/sync-main-preview-addons.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/.storybook/manager.tsx",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "react",
                ),
                scanned_file_with(
                    "code/builders/builder-vite/build-config.ts",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/frameworks/react-vite/src/node/index.ts",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/csf/csf-factories.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/addons/links/build-config.ts",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/addons/docs/src/blocks/blocks/DocsContext.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "test-storybooks/external-docs/.storybook/main.cjs",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "javascript",
                ),
                scanned_file_with(
                    "test-storybooks/portable-stories-kitchen-sink/react/.storybook/main.ts",
                    "config",
                    FilePurpose::Config,
                    &[],
                    "typescript",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: vec![
                "code/.storybook/main.ts".to_string(),
                "code/.storybook/preview.tsx".to_string(),
            ],
            entry_points: vec!["code/.storybook/main.ts".to_string()],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let ds_env = test_research_ds(&report);
        let mut unit = KnowledgeUnit::new(
            UnitType::ConfigDoc,
            "配置API参考",
            "api-domain",
            "API-参考/配置API参考.md",
        );
        unit.scope = UnitScope {
            source_ids: vec![
                "source:code/.storybook/main.ts".to_string(),
                "source:code/.storybook/preview.tsx".to_string(),
                "source:code/core/src/common/utils/sync-main-preview-addons.ts".to_string(),
                "source:code/.storybook/manager.tsx".to_string(),
                "source:code/builders/builder-vite/build-config.ts".to_string(),
                "source:code/frameworks/react-vite/src/node/index.ts".to_string(),
                "source:code/core/src/csf/csf-factories.ts".to_string(),
                "source:code/addons/links/build-config.ts".to_string(),
                "source:code/addons/docs/src/blocks/blocks/DocsContext.ts".to_string(),
                "source:test-storybooks/external-docs/.storybook/main.cjs".to_string(),
                "source:test-storybooks/portable-stories-kitchen-sink/react/.storybook/main.ts"
                    .to_string(),
            ],
            config_surfaces: vec![
                ConfigSurface {
                    file_path: "code/.storybook/main.ts".to_string(),
                    keys: vec!["stories".to_string(), "addons".to_string()],
                },
                ConfigSurface {
                    file_path: "code/.storybook/preview.tsx".to_string(),
                    keys: vec!["parameters".to_string()],
                },
            ],
            ..UnitScope::default()
        };

        let research = research_unit_structural(&unit, &ds_env.as_ds(&report), &[]);
        let key_sources = research.key_sources.clone();
        let section_titles = research
            .section_plan
            .iter()
            .map(|section| section.title.as_str())
            .collect::<Vec<_>>();

        assert_eq!(key_sources[0], "code/.storybook/main.ts");
        assert!(key_sources
            .iter()
            .any(|path| path == "code/.storybook/preview.tsx"));
        assert!(!key_sources
            .iter()
            .any(|path| path.starts_with("test-storybooks/")));
        assert!(section_titles.contains(&"简介"));
        assert!(section_titles.contains(&"项目结构"));
        assert!(section_titles.contains(&"依赖关系分析"));
    }

    #[test]
    fn api_key_source_selection_limits_same_basename_monoculture() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "code/renderers/html/src/public-types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/renderers/vue3/src/public-types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/renderers/react/src/public-types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/manager-api/typings.d.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/preview/typings.d.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/channels/types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ApiSurface,
            Some(&ResearchProfile::ApiSurface),
            None,
            candidates,
        );
        let public_types_count = selected
            .iter()
            .filter(|path| path.ends_with("/public-types.ts"))
            .count();

        assert!(public_types_count <= 2);
        assert!(selected.iter().any(|path| path.ends_with("/typings.d.ts")));
        assert!(selected.iter().any(|path| path.ends_with("/types.ts")));
    }

    #[test]
    fn testing_profile_prefers_real_test_files_over_internal_test_scaffolding() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "hilt-android-testing/main/java/dagger/hilt/android/internal/testing/TestInjector.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "hilt-android-testing/main/java/dagger/hilt/android/internal/testing/InternalTestRoot.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/test/javatests/dagger/internal/DoubleCheckTest.java",
                    "test",
                    FilePurpose::Test,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/test/javatests/dagger/internal/InstanceFactoryTest.java",
                    "test",
                    FilePurpose::Test,
                    &[],
                    "java",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::Other,
            Some(&ResearchProfile::Testing),
            None,
            candidates,
        );

        assert!(selected
            .iter()
            .take(2)
            .all(|path| path.contains("/javatests/") || path.contains("Test.java")));
        assert!(selected
            .iter()
            .any(|path| path.ends_with("DoubleCheckTest.java")));
        assert!(selected
            .iter()
            .any(|path| path.ends_with("InstanceFactoryTest.java")));
    }

    #[test]
    fn runtime_profile_prefers_public_contracts_and_validation_over_internal_helpers() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Binds.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Lazy.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/validation/BindsMethodValidator.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/validation/ProvidesMethodValidator.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/internal/ProviderOfLazy.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/internal/MapLazyFactory.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::Other,
            Some(&ResearchProfile::Runtime),
            None,
            candidates,
        );

        assert!(selected
            .iter()
            .take(2)
            .any(|path| path.ends_with("Binds.java")));
        assert!(selected
            .iter()
            .take(2)
            .any(|path| path.ends_with("Lazy.java")));
        assert!(selected.iter().any(|path| path.ends_with("Lazy.java")));
        assert!(selected
            .iter()
            .any(|path| path.ends_with("BindsMethodValidator.java")));
        assert!(selected.iter().any(|path| path.contains("/validation/")));
    }

    #[test]
    fn docs_guide_profile_still_keeps_docs_sources_after_selector_hardening() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "docs/configure/theming.mdx",
                    "docs",
                    FilePurpose::Docs,
                    &[],
                    "mdx",
                ),
                scanned_file_with(
                    "docs/configure/styling-and-css.mdx",
                    "docs",
                    FilePurpose::Docs,
                    &[],
                    "mdx",
                ),
                scanned_file_with(
                    "code/core/src/theming/create.ts",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/theming/index.ts",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "typescript",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: vec![
                "code/core/src/theming/create.ts".to_string(),
                "code/core/src/theming/index.ts".to_string(),
            ],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: if file.is_docs_like() {
                    ResearchKeySourceOrigin::DocReference
                } else {
                    ResearchKeySourceOrigin::ScopeSource
                },
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::Other,
            Some(&ResearchProfile::DocsGuide),
            None,
            candidates,
        );

        assert!(selected
            .iter()
            .any(|path| path == "docs/configure/theming.mdx"));
        assert!(selected
            .iter()
            .any(|path| path == "docs/configure/styling-and-css.mdx"));
        assert!(selected
            .iter()
            .any(|path| path.ends_with("/theming/create.ts")));
    }

    #[test]
    fn type_topic_focus_promotes_story_and_preview_families_over_generic_contracts() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "code/renderers/html/src/public-types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/lib/core-webpack/src/types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/router/types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/lib/cli-storybook/src/typings.d.ts",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/csf/story.ts",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/manager-api/modules/stories.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "react",
                ),
                scanned_file_with(
                    "code/core/src/preview-api/modules/store/inferArgTypes.test.ts",
                    "test",
                    FilePurpose::Test,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "docs/api/arg-types.mdx",
                    "docs",
                    FilePurpose::Docs,
                    &[],
                    "mdx",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: vec![
                "code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx".to_string(),
            ],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let mut unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            "Types API",
            "api-domain",
            "API-参考/开发API参考/Types-API.md",
        );
        unit.scope = UnitScope {
            source_ids: vec![
                "source:code/renderers/html/src/public-types.ts".to_string(),
                "source:code/core/src/csf/story.ts".to_string(),
                "source:code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx".to_string(),
            ],
            ..UnitScope::default()
        };
        let doc_reference_citations = vec![
            SourceCitation {
                path: "code/core/src/csf/story.ts".to_string(),
                start_line: 1,
                end_line: 24,
                source_id: Some("source:code/core/src/csf/story.ts".to_string()),
                symbol_id: None,
                note: "文档引用".to_string(),
            },
            SourceCitation {
                path: "code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx".to_string(),
                start_line: 1,
                end_line: 24,
                source_id: Some(
                    "source:code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx"
                        .to_string(),
                ),
                symbol_id: None,
                note: "文档引用".to_string(),
            },
        ];
        let topic_focus = build_unit_topic_focus_hints(&unit, &report, &doc_reference_citations);
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ApiSurface,
            Some(&ResearchProfile::ApiSurface),
            topic_focus.as_ref(),
            candidates,
        );
        let top_four = selected.iter().take(4).cloned().collect::<Vec<_>>();

        assert!(
            top_four.iter().any(|path| path.ends_with("/csf/story.ts")),
            "top_four={top_four:?}"
        );
        assert!(top_four
            .iter()
            .any(|path| path.ends_with("/manager-api/modules/stories.ts")));
        assert!(top_four
            .iter()
            .any(|path| path.ends_with("/preview-web/PreviewWeb.tsx")));
        assert!(!top_four
            .iter()
            .any(|path| path.ends_with("/router/types.ts")));
        assert!(!top_four
            .iter()
            .any(|path| path.ends_with("/lib/core-webpack/src/types.ts")));
    }

    #[test]
    fn theme_topic_focus_prefers_theming_core_over_framework_presets() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "docs/configure/user-interface/theming.mdx",
                    "docs",
                    FilePurpose::Docs,
                    &[],
                    "mdx",
                ),
                scanned_file_with(
                    "docs/configure/styling-and-css.mdx",
                    "docs",
                    FilePurpose::Docs,
                    &[],
                    "mdx",
                ),
                scanned_file_with(
                    "code/core/src/theming/index.ts",
                    "source",
                    FilePurpose::Entry,
                    &["entry-point"],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/theming/create.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/core/src/theming/themes/light.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/frameworks/nextjs/src/preset.ts",
                    "source",
                    FilePurpose::Config,
                    &[],
                    "typescript",
                ),
                scanned_file_with(
                    "code/frameworks/nextjs/src/preview.tsx",
                    "source",
                    FilePurpose::Config,
                    &[],
                    "react",
                ),
                scanned_file_with(
                    "code/core/src/router/types.ts",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "typescript",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: vec!["code/core/src/theming/index.ts".to_string()],
            dependency_hints: Vec::<DependencyHint>::new(),
        };
        let mut unit = KnowledgeUnit::new(
            UnitType::ConfigDoc,
            "颜色和字体系统",
            "theme-domain",
            "配置参考/主题和外观/颜色和字体系统.md",
        );
        unit.scope = UnitScope {
            source_ids: vec![
                "source:docs/configure/user-interface/theming.mdx".to_string(),
                "source:docs/configure/styling-and-css.mdx".to_string(),
                "source:code/core/src/theming/create.ts".to_string(),
            ],
            ..UnitScope::default()
        };
        let topic_focus = build_unit_topic_focus_hints(&unit, &report, &[]);
        let candidates = report
            .files
            .iter()
            .map(|file| ResearchKeySourceCandidate {
                path: file.path.clone(),
                origin: ResearchKeySourceOrigin::ScopeSource,
            })
            .collect::<Vec<_>>();

        let selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ConfigSurface,
            Some(&ResearchProfile::ConfigSurface),
            topic_focus.as_ref(),
            candidates,
        );
        let top_three = selected.iter().take(3).cloned().collect::<Vec<_>>();

        assert!(top_three
            .iter()
            .any(|path| path.contains("/core/src/theming/")));
        assert!(!top_three
            .iter()
            .all(|path| path.contains("/frameworks/nextjs/")));
        assert!(!top_three
            .iter()
            .any(|path| path.ends_with("/router/types.ts")));
    }

    #[test]
    fn generic_title_focus_separates_runtime_compiler_and_hilt_api_spines() {
        let report = ScanReport {
            root: "repo".to_string(),
            files: vec![
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Module.java",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Component.java",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-runtime/main/java/dagger/Subcomponent.java",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/base/SourceFileGenerator.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-compiler/main/java/dagger/internal/codegen/processingstep/BindingMethodProcessingStep.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "hilt-android/main/java/dagger/hilt/android/AndroidEntryPoint.java",
                    "source",
                    FilePurpose::Type,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessor.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessingStep.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
                scanned_file_with(
                    "dagger-android-processor/main/java/dagger/android/processor/AndroidProcessor.java",
                    "source",
                    FilePurpose::Service,
                    &[],
                    "java",
                ),
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::<DependencyHint>::new(),
        };

        let mut compiler_unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            "编译时API",
            "api-domain",
            "API-参考/编译时API.md",
        );
        compiler_unit.scope = UnitScope {
            source_ids: vec![
                "source:dagger-compiler/main/java/dagger/internal/codegen/ComponentProcessor.java"
                    .to_string(),
                "source:dagger-compiler/main/java/dagger/internal/codegen/base/SourceFileGenerator.java"
                    .to_string(),
            ],
            ..UnitScope::default()
        };
        let compiler_focus = build_unit_topic_focus_hints(&compiler_unit, &report, &[]);
        assert!(compiler_focus.is_some(), "compiler_focus should exist");
        let compiler_focus_ref = compiler_focus.as_ref();
        let compiler_selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ApiSurface,
            Some(&ResearchProfile::ApiSurface),
            compiler_focus_ref,
            report
                .files
                .iter()
                .map(|file| ResearchKeySourceCandidate {
                    path: file.path.clone(),
                    origin: ResearchKeySourceOrigin::ScopeSource,
                })
                .collect(),
        );
        let compiler_top_three = compiler_selected
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>();
        assert!(
            compiler_top_three
                .iter()
                .any(|path| path.contains("dagger-compiler/")),
            "compiler_top_three={compiler_top_three:?}"
        );
        assert!(!compiler_top_three
            .iter()
            .any(|path| path.ends_with("/AndroidEntryPoint.java")));

        let mut hilt_unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            "Hilt API",
            "api-domain",
            "API-参考/Hilt-API.md",
        );
        hilt_unit.scope = UnitScope {
            source_ids: vec![
                "source:hilt-android/main/java/dagger/hilt/android/AndroidEntryPoint.java"
                    .to_string(),
                "source:hilt-compiler/main/java/dagger/hilt/android/processor/internal/androidentrypoint/AndroidEntryPointProcessor.java"
                    .to_string(),
            ],
            ..UnitScope::default()
        };
        let hilt_focus = build_unit_topic_focus_hints(&hilt_unit, &report, &[]);
        assert!(hilt_focus.is_some(), "hilt_focus should exist");
        let hilt_selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ApiSurface,
            Some(&ResearchProfile::ApiSurface),
            hilt_focus.as_ref(),
            report
                .files
                .iter()
                .map(|file| ResearchKeySourceCandidate {
                    path: file.path.clone(),
                    origin: ResearchKeySourceOrigin::ScopeSource,
                })
                .collect(),
        );
        let hilt_top_three = hilt_selected.iter().take(3).cloned().collect::<Vec<_>>();
        assert!(
            hilt_top_three.iter().any(|path| path.contains("hilt-")),
            "hilt_top_three={hilt_top_three:?}"
        );
        assert!(!hilt_top_three
            .iter()
            .any(|path| path.ends_with("/ComponentProcessor.java")));

        let mut runtime_unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            "运行时API",
            "api-domain",
            "API-参考/运行时API.md",
        );
        runtime_unit.scope = UnitScope {
            source_ids: vec![
                "source:dagger-runtime/main/java/dagger/Module.java".to_string(),
                "source:dagger-runtime/main/java/dagger/Component.java".to_string(),
            ],
            ..UnitScope::default()
        };
        let runtime_focus = build_unit_topic_focus_hints(&runtime_unit, &report, &[]);
        assert!(runtime_focus.is_some(), "runtime_focus should exist");
        let runtime_selected = select_structural_key_sources(
            &report,
            ResearchPageKind::ApiSurface,
            Some(&ResearchProfile::ApiSurface),
            runtime_focus.as_ref(),
            report
                .files
                .iter()
                .map(|file| ResearchKeySourceCandidate {
                    path: file.path.clone(),
                    origin: ResearchKeySourceOrigin::ScopeSource,
                })
                .collect(),
        );
        let runtime_top_three = runtime_selected.iter().take(3).cloned().collect::<Vec<_>>();
        assert!(
            runtime_top_three
                .iter()
                .any(|path| path.contains("dagger-runtime/")),
            "runtime_top_three={runtime_top_three:?}"
        );
        assert!(!runtime_top_three
            .iter()
            .any(|path| path.contains("dagger-compiler/")));
    }

    #[test]
    fn example_tutorial_profile_does_not_penalize_sample_sources() {
        let sample = scanned_file_with(
            "test-storybooks/external-docs/.storybook/main.cjs",
            "config",
            FilePurpose::Config,
            &["entry-point"],
            "javascript",
        );
        let score_without_example = structural_key_source_score(
            ResearchPageKind::Other,
            Some(&ResearchProfile::ConfigSurface),
            None,
            &sample,
            &[ResearchKeySourceOrigin::ScopeSource],
        );
        let score_with_example = structural_key_source_score(
            ResearchPageKind::Other,
            Some(&ResearchProfile::ExampleTutorial),
            None,
            &sample,
            &[ResearchKeySourceOrigin::ScopeSource],
        );

        assert!(score_with_example > score_without_example);
    }

    #[test]
    fn default_api_and_overview_skeletons_use_reference_outline() {
        let overview_unit =
            KnowledgeUnit::new(UnitType::Overview, "项目概述", "system", "项目概述.md");
        let api_unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            "Android API",
            "api-domain",
            "API-参考/Android-API.md",
        );

        let overview_sections = build_default_section_plan(&overview_unit, None, &[]);
        let api_sections =
            build_default_section_plan(&api_unit, Some(&ResearchProfile::ApiSurface), &[]);

        assert_eq!(overview_sections[1].title, "目录");
        assert_eq!(overview_sections[2].title, "简介");
        assert_eq!(overview_sections[3].title, "项目结构");
        assert_eq!(api_sections[2].title, "简介");
        assert_eq!(api_sections[3].title, "项目结构");
        assert!(api_sections
            .iter()
            .any(|section| section.title == "详细组件分析"));
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

    fn scanned_file_with(
        path: &str,
        kind: &str,
        purpose: FilePurpose,
        tags: &[&str],
        language: &str,
    ) -> ScannedFile {
        ScannedFile {
            id: format!("source:{path}"),
            path: path.to_string(),
            language: language.to_string(),
            kind: kind.to_string(),
            purpose,
            fingerprint: "fp".to_string(),
            size: 0,
            tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
        }
    }

    fn module_context(module_id: &str, key_sources: Vec<String>) -> ModuleContext {
        ModuleContext {
            module_id: module_id.to_string(),
            role_hints: Vec::new(),
            public_surface: Vec::new(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
            key_sources,
            graph_hotspots: Vec::new(),
            communities: Vec::new(),
            cycle_warnings: Vec::new(),
            capability_topics: Vec::new(),
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
