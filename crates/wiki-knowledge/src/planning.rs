use std::collections::{BTreeMap, BTreeSet};

use crate::domain::context::{ModuleContext, RepoContext};
use wiki_model::domain::knowledge::{
    ApiSurface, CollapseGuardDecision, CollapseGuardReason, ConfigSurface, DecompositionProfile,
    DocsAnchor, DomainEvidence, DomainType, KnowledgeDomain, KnowledgeTree, KnowledgeUnit,
    PlannerSignalBundle, PlannerSignalKind, UnitScope, UnitType,
};
use wiki_model::domain::module_tree::{ModuleNode, ModuleTree};
use wiki_model::domain::stable_id::stable_id;
#[derive(Debug, Clone, Default)]
pub struct KnowledgePlannerConfig {
    pub force_domains: Vec<String>,
    pub suppress_domains: Vec<String>,
    pub unit_weight_threshold: f32,
    pub max_units_per_domain: usize,
}

use wiki_index::scanner::ScanReport;
use wiki_index::symbol_graph::GraphSummary;

// ─── Domain discovery ───────────────────────────────────────

/// 从 Facts 层的 ModuleTree + ScanReport + GraphSummary + RepoContext 中发现知识域。
/// 每种 DomainType 有独立的启发式规则，按证据强度排序。
pub fn discover_knowledge_domains(
    report: &ScanReport,
    module_tree: &ModuleTree,
    _repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    _graph_summary: &GraphSummary,
    planner_config: &KnowledgePlannerConfig,
) -> Vec<KnowledgeDomain> {
    let mut domains = Vec::new();

    let module_index: BTreeMap<&str, &ModuleNode> = module_tree
        .modules
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();

    let module_context_index: BTreeMap<&str, &ModuleContext> = module_contexts
        .iter()
        .map(|c| (c.module_id.as_str(), c))
        .collect();

    let docs_files = collect_docs_files(report);
    let concept_docs = filter_docs_for_domain(&docs_files, DomainType::ConceptGuide);
    let api_docs = filter_docs_for_domain(&docs_files, DomainType::ApiReference);
    let config_docs = filter_docs_for_domain(&docs_files, DomainType::ConfigReference);
    let troubleshooting_docs = filter_docs_for_domain(&docs_files, DomainType::Troubleshooting);
    let compiler_docs = filter_docs_for_domain(&docs_files, DomainType::CompilerToolchain);
    let config_files = collect_config_files(report);
    let test_files = collect_test_files(report);

    let top_modules = collect_top_modules(module_tree);

    // CoreRuntime
    if let Some(domain) = discover_core_runtime(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // Framework
    for domain in discover_frameworks(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // PlatformBinding
    for domain in discover_platform_bindings(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // CompilerToolchain
    if let Some(domain) =
        discover_compiler_toolchain(&top_modules, &module_index, report, &compiler_docs)
    {
        domains.push(domain);
    }

    // PluginEcosystem
    if let Some(domain) = discover_plugin_ecosystem(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // MultiFramework
    if let Some(domain) = discover_multi_framework(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // TestingInfra
    if let Some(domain) = discover_testing_infra(&top_modules, &module_index, report, &test_files) {
        domains.push(domain);
    }

    // BuildSystem
    if let Some(domain) = discover_build_system(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // ThemeSystem
    if let Some(domain) = discover_theme_system(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // DevTooling
    if let Some(domain) = discover_dev_tooling(&top_modules, &module_index, report) {
        domains.push(domain);
    }

    // ConceptGuide
    if let Some(domain) = discover_concept_guide(report, &concept_docs) {
        domains.push(domain);
    }

    // ApiReference
    if let Some(domain) =
        discover_api_reference(&top_modules, &module_context_index, report, &api_docs)
    {
        domains.push(domain);
    }

    // ConfigReference
    if let Some(domain) = discover_config_reference(report, &config_files, &config_docs) {
        domains.push(domain);
    }

    // Troubleshooting
    if let Some(domain) = discover_troubleshooting(report, &troubleshooting_docs) {
        domains.push(domain);
    }

    // Steering overrides
    apply_steering_overrides(&mut domains, planner_config);
    prune_overlapping_structural_domain_modules(&mut domains, &module_index, report);

    // 兜底：将未被任何 ModuleDoc 产生域覆盖的模块收入 CoreRuntime 域
    {
        let module_doc_domain_types = [
            DomainType::CoreRuntime,
            DomainType::Framework,
            DomainType::PlatformBinding,
            DomainType::CompilerToolchain,
            DomainType::PluginEcosystem,
            DomainType::MultiFramework,
            DomainType::TestingInfra,
            DomainType::ApiReference,
            DomainType::BuildSystem,
            DomainType::ThemeSystem,
            DomainType::DevTooling,
        ];
        let covered_module_ids: BTreeSet<&str> = domains
            .iter()
            .filter(|d| module_doc_domain_types.contains(&d.domain_type))
            .flat_map(|d| d.source_modules.iter().map(String::as_str))
            .collect();
        let uncovered: Vec<&ModuleNode> = top_modules
            .iter()
            .filter(|m| !covered_module_ids.contains(m.id.as_str()))
            .copied()
            .collect();
        if !uncovered.is_empty() {
            let mut fallback = KnowledgeDomain::new(DomainType::CoreRuntime, "核心模块");
            fallback.evidence = DomainEvidence {
                matched_modules: uncovered.iter().map(|m| m.id.clone()).collect(),
                ..Default::default()
            };
            fallback.source_modules = uncovered.iter().map(|m| m.id.clone()).collect();
            fallback.source_files = collect_module_files(report, &uncovered);
            fallback.estimated_depth = estimate_depth(&uncovered);
            domains.push(fallback);
        }
    }

    domains
}

fn prune_overlapping_structural_domain_modules(
    domains: &mut [KnowledgeDomain],
    module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) {
    let mut module_to_domains: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (domain_index, domain) in domains.iter().enumerate() {
        if !domain_supports_structural_module_ownership(&domain.domain_type) {
            continue;
        }
        for module_id in &domain.source_modules {
            module_to_domains
                .entry(module_id.clone())
                .or_default()
                .push(domain_index);
        }
    }

    if !module_to_domains.values().any(|indices| indices.len() > 1) {
        return;
    }

    let source_path_by_id: BTreeMap<&str, &str> = report
        .files
        .iter()
        .map(|file| (file.id.as_str(), file.path.as_str()))
        .collect();
    let mut preferred_domain_by_module = BTreeMap::new();
    for (module_id, domain_indices) in &module_to_domains {
        if domain_indices.len() < 2 {
            continue;
        }
        let Some(module) = module_index.get(module_id.as_str()).copied() else {
            continue;
        };
        let preferred = choose_primary_structural_domain(module, domain_indices, domains);
        preferred_domain_by_module.insert(module_id.clone(), preferred);
    }

    for (module_id, preferred_domain) in preferred_domain_by_module {
        let Some(module) = module_index.get(module_id.as_str()).copied() else {
            continue;
        };
        let removed_paths: BTreeSet<&str> = module
            .source_ids
            .iter()
            .filter_map(|source_id| source_path_by_id.get(source_id.as_str()).copied())
            .collect();
        let Some(domain_indices) = module_to_domains.get(&module_id) else {
            continue;
        };
        for domain_index in domain_indices {
            if *domain_index == preferred_domain {
                continue;
            }
            let domain = &mut domains[*domain_index];
            domain
                .source_modules
                .retain(|candidate| candidate != &module_id);
            domain
                .evidence
                .matched_modules
                .retain(|candidate| candidate != &module_id);
            if !removed_paths.is_empty() {
                domain
                    .source_files
                    .retain(|path| !removed_paths.contains(path.as_str()));
            }
        }
    }

    for domain in domains.iter_mut() {
        sort_and_dedup_strings(&mut domain.source_modules);
        sort_and_dedup_strings(&mut domain.evidence.matched_modules);
        sort_and_dedup_strings(&mut domain.source_files);
        let remaining_modules: Vec<&ModuleNode> = domain
            .source_modules
            .iter()
            .filter_map(|module_id| module_index.get(module_id.as_str()).copied())
            .collect();
        domain.estimated_depth = if !remaining_modules.is_empty() {
            estimate_depth(&remaining_modules)
        } else if !domain.source_files.is_empty() {
            estimate_file_depth(&domain.source_files)
        } else {
            1
        };
    }
}

fn domain_supports_structural_module_ownership(domain_type: &DomainType) -> bool {
    matches!(
        domain_type,
        DomainType::CoreRuntime
            | DomainType::Framework
            | DomainType::PlatformBinding
            | DomainType::CompilerToolchain
            | DomainType::PluginEcosystem
            | DomainType::TestingInfra
            | DomainType::BuildSystem
            | DomainType::MultiFramework
            | DomainType::ThemeSystem
            | DomainType::DevTooling
    )
}

fn choose_primary_structural_domain(
    module: &ModuleNode,
    domain_indices: &[usize],
    domains: &[KnowledgeDomain],
) -> usize {
    let mut best_index = *domain_indices
        .first()
        .expect("overlapping module should have at least one domain");
    let mut best_score = structural_domain_fit_score(module, &domains[best_index]);

    for domain_index in domain_indices.iter().copied().skip(1) {
        let score = structural_domain_fit_score(module, &domains[domain_index]);
        if score > best_score {
            best_index = domain_index;
            best_score = score;
        }
    }

    best_index
}

fn structural_domain_fit_score(module: &ModuleNode, domain: &KnowledgeDomain) -> (usize, usize) {
    let name_tokens = tokenize_for_domain_match(&module.name);
    let path_tokens = module
        .root_paths
        .iter()
        .flat_map(|path| tokenize_for_domain_match(path))
        .collect::<Vec<_>>();
    let tag_tokens = module
        .tags
        .iter()
        .flat_map(|tag| tokenize_for_domain_match(tag))
        .collect::<Vec<_>>();
    let domain_keywords = domain_match_keywords(domain);

    let mut score = 0usize;
    for keyword in &domain_keywords {
        if tokens_match_keyword(&name_tokens, keyword) {
            score += 5;
        }
        if tokens_match_keyword(&path_tokens, keyword) {
            score += 3;
        }
        if tokens_match_keyword(&tag_tokens, keyword) {
            score += 2;
        }
    }

    score += match domain.domain_type {
        DomainType::BuildSystem if module.kind == "build-tool" => 4,
        DomainType::DevTooling if module.kind == "cli-tool" => 4,
        DomainType::TestingInfra
            if module
                .tags
                .iter()
                .any(|tag| tag == "test" || tag == "testing") =>
        {
            4
        }
        DomainType::CoreRuntime
            if module
                .tags
                .iter()
                .any(|tag| tag == "core" || tag == "runtime") =>
        {
            4
        }
        _ => 0,
    };

    (score, structural_domain_priority(&domain.domain_type))
}

fn structural_domain_priority(domain_type: &DomainType) -> usize {
    match domain_type {
        DomainType::Framework => 10,
        DomainType::CompilerToolchain => 9,
        DomainType::PlatformBinding => 8,
        DomainType::PluginEcosystem => 7,
        DomainType::MultiFramework => 6,
        DomainType::ThemeSystem => 5,
        DomainType::BuildSystem => 4,
        DomainType::TestingInfra => 3,
        DomainType::DevTooling => 2,
        DomainType::CoreRuntime => 1,
        _ => 0,
    }
}

fn domain_match_keywords(domain: &KnowledgeDomain) -> Vec<String> {
    let mut keywords = match domain.domain_type {
        DomainType::CoreRuntime => vec![
            "core", "runtime", "engine", "kernel", "base", "lib", "internal",
        ],
        DomainType::Framework => vec!["framework"],
        DomainType::PlatformBinding => vec!["android", "ios", "web", "desktop", "wasm"],
        DomainType::CompilerToolchain => vec![
            "compiler",
            "codegen",
            "processor",
            "processing",
            "annotation",
            "transform",
            "babel",
            "swc",
            "spi",
        ],
        DomainType::PluginEcosystem => vec!["addon", "plugin", "extension", "preset", "decorator"],
        DomainType::TestingInfra => vec![
            "test",
            "testing",
            "spec",
            "benchmark",
            "performance",
            "monitor",
            "timing",
            "profile",
        ],
        DomainType::BuildSystem => vec![
            "builder",
            "build",
            "webpack",
            "vite",
            "rollup",
            "esbuild",
            "turbopack",
            "bundler",
            "nx",
        ],
        DomainType::MultiFramework => vec![
            "framework",
            "renderer",
            "react",
            "vue",
            "angular",
            "svelte",
            "preact",
            "html",
            "server",
        ],
        DomainType::ThemeSystem => vec!["theme", "theming", "style", "css", "design", "token"],
        DomainType::DevTooling => vec![
            "cli", "tool", "tools", "script", "scripts", "lint", "format",
        ],
        _ => Vec::new(),
    }
    .into_iter()
    .map(|value| value.to_string())
    .collect::<Vec<_>>();

    keywords.extend(tokenize_for_domain_match(&domain.label));
    for pattern in &domain.evidence.matched_file_patterns {
        keywords.extend(tokenize_for_domain_match(pattern));
    }

    keywords.retain(|keyword| {
        !keyword.is_empty()
            && !matches!(
                keyword.as_str(),
                "page" | "pages" | "guide" | "guides" | "domain" | "module" | "modules"
            )
    });
    keywords.sort();
    keywords.dedup();
    keywords
}

fn tokenize_for_domain_match(value: &str) -> Vec<String> {
    normalize_path(value)
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_ascii_lowercase())
        .collect()
}

fn tokens_match_keyword(tokens: &[String], keyword: &str) -> bool {
    tokens.iter().any(|token| {
        if keyword.len() <= 3 {
            token == keyword
        } else {
            token == keyword || token.contains(keyword)
        }
    })
}

// ─── Individual domain detectors ────────────────────────────

fn discover_core_runtime(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let core_keywords = [
        "core", "runtime", "engine", "kernel", "base", "lib", "internal",
    ];
    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            core_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.kind == "library"
                || m.tags.iter().any(|t| t == "core" || t == "runtime")
        })
        .copied()
        .collect();

    if matched.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心运行时");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/core/**".into(), "**/runtime/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = collect_module_files(report, &matched);
    domain.estimated_depth = if matched.is_empty() {
        estimate_file_depth(&domain.source_files)
    } else {
        estimate_depth(&matched)
    };
    Some(domain)
}

fn discover_frameworks(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Vec<KnowledgeDomain> {
    let framework_patterns = [
        ("hilt", "Hilt"),
        ("spring", "Spring"),
        ("express", "Express"),
        ("actix", "Actix"),
        ("axum", "Axum"),
        ("gin", "Gin"),
        ("chi", "Chi"),
        ("django", "Django"),
        ("flask", "Flask"),
        ("fastapi", "FastAPI"),
        ("nestjs", "NestJS"),
    ];

    let mut results = Vec::new();

    for (pattern, label) in &framework_patterns {
        let matched: Vec<&ModuleNode> = top_modules
            .iter()
            .filter(|m| {
                let name_lower = m.name.to_ascii_lowercase();
                name_lower.contains(pattern)
                    || m.root_paths
                        .iter()
                        .any(|p| p.to_ascii_lowercase().contains(pattern))
            })
            .copied()
            .collect();

        if matched.is_empty() {
            continue;
        }

        let mut domain = KnowledgeDomain::new(DomainType::Framework, format!("框架集成：{label}"));
        domain.evidence = DomainEvidence {
            matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
            matched_file_patterns: vec![format!("**/{pattern}/**")],
            ..Default::default()
        };
        domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
        domain.source_files = collect_module_files(report, &matched);
        domain.estimated_depth = estimate_depth(&matched);
        results.push(domain);
    }

    results
}

fn discover_platform_bindings(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Vec<KnowledgeDomain> {
    let platform_patterns = [
        ("android", "Android"),
        ("ios", "iOS"),
        ("web", "Web"),
        ("desktop", "Desktop"),
        ("wasm", "WASM"),
    ];

    let mut results = Vec::new();

    for (pattern, label) in &platform_patterns {
        let matched: Vec<&ModuleNode> = top_modules
            .iter()
            .filter(|m| {
                let name_lower = m.name.to_ascii_lowercase();
                name_lower.contains(pattern)
                    || m.tags.iter().any(|t| t.to_ascii_lowercase() == *pattern)
            })
            .copied()
            .collect();

        if matched.is_empty() {
            continue;
        }

        let mut domain =
            KnowledgeDomain::new(DomainType::PlatformBinding, format!("平台绑定：{label}"));
        domain.evidence = DomainEvidence {
            matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
            matched_file_patterns: vec![format!("**/{pattern}/**")],
            ..Default::default()
        };
        domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
        domain.source_files = collect_module_files(report, &matched);
        domain.estimated_depth = estimate_depth(&matched);
        results.push(domain);
    }

    results
}

fn discover_compiler_toolchain(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
    docs_files: &[String],
) -> Option<KnowledgeDomain> {
    let compiler_keywords = [
        "compiler",
        "codegen",
        "transpiler",
        "processor",
        "spi",
        "annotation",
        "transform",
        "babel",
        "swc",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            compiler_keywords.iter().any(|kw| name_lower.contains(kw))
        })
        .copied()
        .collect();

    if matched.is_empty() && docs_files.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::CompilerToolchain, "编译工具链");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/compiler/**".into(), "**/codegen/**".into()],
        docs_anchors: build_docs_anchors(docs_files),
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = collect_module_files(report, &matched);
    domain.source_files.extend(docs_files.iter().cloned());
    sort_and_dedup_strings(&mut domain.source_files);
    domain.estimated_depth = estimate_depth(&matched);
    Some(domain)
}

fn discover_plugin_ecosystem(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let plugin_keywords = ["addon", "plugin", "extension", "preset", "decorator"];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            plugin_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.tags.iter().any(|t| {
                    let t_lower = t.to_ascii_lowercase();
                    plugin_keywords.iter().any(|kw| t_lower.contains(kw))
                })
                || m.root_paths.iter().any(|path| {
                    let lower = path.to_ascii_lowercase();
                    plugin_keywords.iter().any(|kw| lower.contains(kw))
                })
        })
        .copied()
        .collect();

    let file_hits = report
        .files
        .iter()
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            plugin_keywords.iter().any(|kw| lower.contains(kw))
        })
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();

    if matched.is_empty() && file_hits.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::PluginEcosystem, "插件生态");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/addon*/**".into(), "**/plugin*/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = if matched.is_empty() {
        file_hits.iter().take(64).cloned().collect()
    } else {
        let mut files = collect_module_files(report, &matched);
        files.extend(file_hits);
        files.sort();
        files.dedup();
        files
    };
    domain.estimated_depth = if matched.is_empty() {
        2
    } else {
        estimate_depth(&matched)
    };
    Some(domain)
}

fn discover_multi_framework(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let renderer_keywords = [
        "renderer",
        "react",
        "vue",
        "angular",
        "svelte",
        "preact",
        "web-component",
        "html",
        "server",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            (name_lower.contains("renderer") || name_lower.contains("framework"))
                && renderer_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.root_paths.iter().any(|path| {
                    let lower = path.to_ascii_lowercase();
                    lower.contains("framework")
                        && renderer_keywords.iter().any(|kw| lower.contains(kw))
                })
        })
        .copied()
        .collect();

    // Also check if multiple framework-related modules exist even without "renderer" prefix
    let framework_modules: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            ["react", "vue", "angular", "svelte", "preact"]
                .iter()
                .any(|fw| {
                    name_lower.contains(fw)
                        || m.root_paths
                            .iter()
                            .any(|path| path.to_ascii_lowercase().contains(fw))
                })
        })
        .copied()
        .collect();

    let framework_file_hits: BTreeSet<&str> = report
        .files
        .iter()
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            lower.contains("framework")
                && ["react", "vue", "angular", "svelte", "preact"]
                    .iter()
                    .any(|fw| lower.contains(fw))
        })
        .map(|file| file.id.as_str())
        .collect();

    let all_matched: BTreeSet<&str> = matched
        .iter()
        .chain(framework_modules.iter())
        .map(|m| m.id.as_str())
        .collect();

    if all_matched.len() < 2 && framework_file_hits.len() < 2 {
        return None;
    }

    let all_modules: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| all_matched.contains(m.id.as_str()))
        .copied()
        .collect();

    let mut domain = KnowledgeDomain::new(DomainType::MultiFramework, "多框架支持");
    domain.evidence = DomainEvidence {
        matched_modules: all_modules.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/renderer-*/**".into()],
        ..Default::default()
    };
    domain.source_modules = all_modules.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &all_modules);
    source_files.extend(
        report
            .files
            .iter()
            .filter(|file| framework_file_hits.contains(file.id.as_str()))
            .map(|file| file.path.clone()),
    );
    source_files.sort();
    source_files.dedup();
    domain.source_files = source_files;
    domain.estimated_depth = if all_modules.is_empty() {
        2
    } else {
        estimate_depth(&all_modules)
    };
    Some(domain)
}

fn discover_testing_infra(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    _report: &ScanReport,
    test_files: &[String],
) -> Option<KnowledgeDomain> {
    let test_keywords = [
        "test",
        "testing",
        "spec",
        "e2e",
        "integration-test",
        "benchmark",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            (test_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.tags.iter().any(|t| t == "test" || t == "testing")
                || m.kind == "test-suite")
                && !m.root_paths.iter().any(|path| {
                    let lower = normalize_path(path).to_ascii_lowercase();
                    lower.contains("/test-storybooks/")
                        || lower.contains("/kitchen-sink/")
                        || lower == "test-storybooks"
                        || lower == "kitchen-sink"
                        || lower.ends_with("/test-storybooks")
                        || lower.ends_with("/kitchen-sink")
                })
        })
        .copied()
        .collect();

    let has_significant_tests = !matched.is_empty() || !test_files.is_empty();

    if !has_significant_tests {
        return None;
    }

    let performance_signal_keywords = ["benchmark", "performance", "monitor", "timing", "profile"];
    let performance_signal_files: Vec<String> = _report
        .files
        .iter()
        .filter(|file| {
            let lower = normalize_path(&file.path).to_ascii_lowercase();
            performance_signal_keywords
                .iter()
                .any(|keyword| lower.contains(keyword))
        })
        .map(|file| file.path.clone())
        .collect();
    let testing_docs_signal_keywords = [
        "writing-tests",
        "vitest",
        "playwright",
        "test-runner",
        "stories-in-unit-tests",
        "stories-in-end-to-end-tests",
        "interaction-testing",
        "snapshot-testing",
        "visual-testing",
        "accessibility-testing",
        "in-ci",
    ];
    let testing_docs_signal_files: Vec<String> = _report
        .files
        .iter()
        .filter(|file| is_markdown_path(&file.path))
        .filter(|file| {
            let lower = normalize_path(&file.path).to_ascii_lowercase();
            testing_docs_signal_keywords
                .iter()
                .any(|keyword| lower.contains(keyword))
        })
        .map(|file| file.path.clone())
        .collect();
    let testing_support_signal_keywords = [
        "hiltandroidtest",
        "hiltandroidrule",
        "testinstallin",
        "customtestapplication",
        "testinjector",
        "skiptestinjection",
        "bindvalue",
        "uninstallmodules",
    ];
    let testing_support_signal_files: Vec<String> = _report
        .files
        .iter()
        .filter(|file| {
            let lower = normalize_path(&file.path).to_ascii_lowercase();
            (lower.contains("hilt-android-testing/")
                && !lower.contains("/test/")
                && !lower.contains("/tests/")
                && !lower.contains("/javatests/"))
                || testing_support_signal_keywords
                    .iter()
                    .any(|keyword| lower.contains(keyword))
        })
        .map(|file| file.path.clone())
        .collect();

    let mut domain = KnowledgeDomain::new(DomainType::TestingInfra, "测试基础设施");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec![
            "**/*test*/**".into(),
            "**/*.test.*".into(),
            "**/*.spec.*".into(),
            "**/*benchmark*/**".into(),
            "**/*monitor*/**".into(),
            "**/*timing*/**".into(),
        ],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = test_files.to_vec();
    domain.source_files.extend(performance_signal_files);
    domain.source_files.extend(testing_docs_signal_files);
    domain.source_files.extend(testing_support_signal_files);
    sort_and_dedup_strings(&mut domain.source_files);
    domain.estimated_depth = if matched.len() > 2 { 2 } else { 1 };
    Some(domain)
}

fn discover_build_system(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let build_keywords = [
        "builder",
        "bundler",
        "build",
        "gradle",
        "maven",
        "bazel",
        "buck",
        "webpack",
        "rollup",
        "vite",
        "esbuild",
        "turbopack",
        "nx",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            build_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.root_paths.iter().any(|path| {
                    let lower = path.to_ascii_lowercase();
                    build_keywords.iter().any(|kw| lower.contains(kw))
                })
                || m.kind == "build-tool"
        })
        .copied()
        .collect();

    let file_hits: Vec<String> = report
        .files
        .iter()
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            build_keywords.iter().any(|kw| lower.contains(kw))
        })
        .map(|file| file.path.clone())
        .collect();

    if matched.is_empty() && file_hits.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::BuildSystem, "构建系统");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/builder*/**".into(), "**/build/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &matched);
    source_files.extend(file_hits);
    source_files.sort();
    source_files.dedup();
    domain.source_files = source_files;
    domain.estimated_depth = if matched.is_empty() {
        1
    } else {
        estimate_depth(&matched)
    };
    Some(domain)
}

fn discover_theme_system(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let theme_keywords = ["theme", "theming", "style", "css", "design-token"];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            theme_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.root_paths.iter().any(|path| {
                    let lower = path.to_ascii_lowercase();
                    theme_keywords.iter().any(|kw| lower.contains(kw))
                })
        })
        .copied()
        .collect();

    let file_hits: Vec<String> = report
        .files
        .iter()
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            theme_keywords.iter().any(|kw| lower.contains(kw))
        })
        .map(|file| file.path.clone())
        .collect();

    if matched.is_empty() && file_hits.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::ThemeSystem, "主题系统");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/theme*/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &matched);
    source_files.extend(file_hits);
    source_files.sort();
    source_files.dedup();
    domain.source_files = source_files;
    domain.estimated_depth = if matched.is_empty() {
        1
    } else {
        estimate_depth(&matched)
    };
    Some(domain)
}

fn discover_dev_tooling(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let tooling_keywords = [
        "cli",
        "dev-tool",
        "devtool",
        "tool",
        "scripts",
        "linter",
        "formatter",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            tooling_keywords.iter().any(|kw| name_lower.contains(kw)) || m.kind == "cli-tool"
        })
        .copied()
        .collect();

    let tooling_file_keywords = [
        "eslint",
        "chromatic",
        "in-ci",
        "telemetry",
        "workflow",
        "pipeline",
        "deploy",
        "release",
        "circleci",
    ];
    let file_hits: Vec<String> = report
        .files
        .iter()
        .filter(|file| {
            let lower = normalize_path(&file.path).to_ascii_lowercase();
            tooling_keywords
                .iter()
                .chain(tooling_file_keywords.iter())
                .any(|keyword| lower.contains(keyword))
        })
        .map(|file| file.path.clone())
        .collect();

    if matched.is_empty() && file_hits.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::DevTooling, "开发工具");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/cli/**".into(), "**/scripts/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &matched);
    source_files.extend(file_hits);
    sort_and_dedup_strings(&mut source_files);
    domain.source_files = source_files;
    domain.estimated_depth = if matched.is_empty() {
        estimate_file_depth(&domain.source_files)
    } else {
        estimate_depth(&matched)
    };
    Some(domain)
}

fn discover_concept_guide(_report: &ScanReport, docs_files: &[String]) -> Option<KnowledgeDomain> {
    if docs_files.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::ConceptGuide, "概念指南");
    domain.evidence = DomainEvidence {
        matched_file_patterns: vec!["docs/**/*.md".into(), "**/*.mdx".into()],
        docs_anchors: build_docs_anchors(docs_files),
        ..Default::default()
    };
    domain.source_files = docs_files.to_vec();
    domain.estimated_depth = estimate_file_depth(docs_files);
    Some(domain)
}

fn discover_api_reference(
    top_modules: &[&ModuleNode],
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    report: &ScanReport,
    docs_files: &[String],
) -> Option<KnowledgeDomain> {
    let mut api_modules = Vec::new();
    let mut api_surfaces = Vec::new();

    for module in top_modules {
        if let Some(ctx) = module_context_index.get(module.id.as_str()) {
            let explicit_api_symbols = module_explicit_api_symbols(module, report);
            if ctx.public_surface.len() >= 2 || !explicit_api_symbols.is_empty() {
                api_modules.push(*module);
                api_surfaces.push(ApiSurface {
                    module_id: module.id.clone(),
                    exported_symbols: if ctx.public_surface.is_empty() {
                        explicit_api_symbols
                    } else {
                        ctx.public_surface.clone()
                    },
                });
            }
        }
    }

    let api_file_hits: Vec<_> = report
        .files
        .iter()
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            lower.contains("/api/")
                || lower.starts_with("api/")
                || lower.contains("publicapi")
                || lower.ends_with("public-types.ts")
                || lower.ends_with("public-types.tsx")
                || looks_like_runtime_config_entry(&file.path)
                || api_signal_candidate_accepts_config_path(&file.path)
                || looks_like_api_surface_path(&file.path)
        })
        .collect();

    if api_modules.is_empty() && api_file_hits.is_empty() && docs_files.is_empty() {
        return None;
    }

    if api_surfaces.is_empty() && !api_file_hits.is_empty() {
        api_surfaces.push(ApiSurface {
            module_id: "file-surface".to_string(),
            exported_symbols: api_file_hits
                .iter()
                .map(|file| {
                    file.path
                        .rsplit('/')
                        .next()
                        .unwrap_or(file.path.as_str())
                        .to_string()
                })
                .collect(),
        });
    }

    let mut domain = KnowledgeDomain::new(DomainType::ApiReference, "API 参考");
    domain.evidence = DomainEvidence {
        matched_modules: api_modules.iter().map(|m| m.id.clone()).collect(),
        docs_anchors: build_docs_anchors(docs_files),
        public_api_surfaces: api_surfaces,
        ..Default::default()
    };
    domain.source_modules = api_modules.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &api_modules);
    source_files.extend(api_file_hits.iter().map(|file| file.path.clone()));
    source_files.extend(docs_files.iter().cloned());
    sort_and_dedup_strings(&mut source_files);
    domain.source_files = source_files;
    domain.estimated_depth = estimate_file_depth(&domain.source_files);
    Some(domain)
}

fn discover_config_reference(
    _report: &ScanReport,
    config_files: &[String],
    docs_files: &[String],
) -> Option<KnowledgeDomain> {
    let raw_config_files: Vec<String> = config_files
        .iter()
        .filter(|path| !is_markdown_path(path))
        .cloned()
        .collect();
    if raw_config_files.is_empty() && docs_files.is_empty() {
        return None;
    }

    let config_surfaces: Vec<ConfigSurface> = raw_config_files
        .iter()
        .map(|path| ConfigSurface {
            file_path: path.clone(),
            keys: Vec::new(),
        })
        .collect();

    let mut domain = KnowledgeDomain::new(DomainType::ConfigReference, "配置参考");
    domain.evidence = DomainEvidence {
        matched_file_patterns: vec!["**/*.config.*".into(), "**/config/**".into()],
        docs_anchors: build_docs_anchors(docs_files),
        config_surfaces,
        ..Default::default()
    };
    domain.source_files = raw_config_files;
    domain.source_files.extend(docs_files.iter().cloned());
    sort_and_dedup_strings(&mut domain.source_files);
    domain.estimated_depth = estimate_file_depth(&domain.source_files);
    Some(domain)
}

fn discover_troubleshooting(
    _report: &ScanReport,
    docs_files: &[String],
) -> Option<KnowledgeDomain> {
    if docs_files.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::Troubleshooting, "故障排除");
    domain.evidence = DomainEvidence {
        matched_file_patterns: vec![
            "**/troubleshoot*/**".into(),
            "**/faq/**".into(),
            "**/debug/**".into(),
        ],
        docs_anchors: build_docs_anchors(docs_files),
        ..Default::default()
    };
    domain.source_files = docs_files.to_vec();
    domain.estimated_depth = estimate_file_depth(docs_files);
    Some(domain)
}

// ─── Steering overrides ─────────────────────────────────────

fn apply_steering_overrides(domains: &mut Vec<KnowledgeDomain>, planner_config: &KnowledgePlannerConfig) {
    let suppress = &planner_config.suppress_domains;
    if !suppress.is_empty() {
        domains.retain(|d| {
            !suppress.iter().any(|s| {
                s.eq_ignore_ascii_case(d.domain_type.as_str()) || s.eq_ignore_ascii_case(&d.label)
            })
        });
    }

    let force = &planner_config.force_domains;
    for forced_type_str in force {
        let already_present = domains
            .iter()
            .any(|d| d.domain_type.as_str().eq_ignore_ascii_case(forced_type_str));
        if !already_present {
            if let Some(domain_type) = parse_domain_type(forced_type_str) {
                let label = default_domain_label(&domain_type);
                domains.push(KnowledgeDomain::new(domain_type, label));
            }
        }
    }
}

fn parse_domain_type(s: &str) -> Option<DomainType> {
    match s.to_ascii_lowercase().replace('-', "_").as_str() {
        "core_runtime" => Some(DomainType::CoreRuntime),
        "framework" => Some(DomainType::Framework),
        "platform_binding" => Some(DomainType::PlatformBinding),
        "compiler_toolchain" => Some(DomainType::CompilerToolchain),
        "plugin_ecosystem" => Some(DomainType::PluginEcosystem),
        "testing_infra" => Some(DomainType::TestingInfra),
        "build_system" => Some(DomainType::BuildSystem),
        "concept_guide" => Some(DomainType::ConceptGuide),
        "api_reference" => Some(DomainType::ApiReference),
        "config_reference" => Some(DomainType::ConfigReference),
        "troubleshooting" => Some(DomainType::Troubleshooting),
        "multi_framework" => Some(DomainType::MultiFramework),
        "theme_system" => Some(DomainType::ThemeSystem),
        "dev_tooling" => Some(DomainType::DevTooling),
        _ => None,
    }
}

fn default_domain_label(domain_type: &DomainType) -> &'static str {
    match domain_type {
        DomainType::CoreRuntime => "核心运行时",
        DomainType::Framework => "框架集成",
        DomainType::PlatformBinding => "平台绑定",
        DomainType::CompilerToolchain => "编译工具链",
        DomainType::PluginEcosystem => "插件生态",
        DomainType::TestingInfra => "测试基础设施",
        DomainType::BuildSystem => "构建系统",
        DomainType::ConceptGuide => "概念指南",
        DomainType::ApiReference => "API 参考",
        DomainType::ConfigReference => "配置参考",
        DomainType::Troubleshooting => "故障排除",
        DomainType::MultiFramework => "多框架支持",
        DomainType::ThemeSystem => "主题系统",
        DomainType::DevTooling => "开发工具",
    }
}

// ─── Plan knowledge units ───────────────────────────────────

/// 按域类型拆分知识单元——每种域走不同的拆分策略。
pub fn plan_knowledge_units(
    domains: &[KnowledgeDomain],
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_contexts: &[ModuleContext],
    planner_config: &KnowledgePlannerConfig,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();
    let weight_threshold = planner_config.unit_weight_threshold;
    let max_per_domain = planner_config.max_units_per_domain;
    let docs_primary_profiles = localized_docs_overlay_profile_keys(report);

    let module_context_index: BTreeMap<&str, &ModuleContext> = module_contexts
        .iter()
        .map(|c| (c.module_id.as_str(), c))
        .collect();

    // 1. Overview unit（全局唯一）
    let overview = KnowledgeUnit::new(UnitType::Overview, "项目概述", "system", "项目概述.md");
    units.push(overview);

    // 2. Architecture unit（全局唯一）
    let arch = KnowledgeUnit::new(UnitType::Architecture, "系统架构", "system", "系统架构.md");
    units.push(arch);

    // 3. 按域生成 DomainIndex + 域内单元
    for domain in domains {
        let domain_label = sanitize_path_segment(&domain.label);
        let mut domain_units = plan_units_for_domain(
            domain,
            module_tree,
            report,
            &module_context_index,
            &domain_label,
        );
        prune_shadowed_source_units(&mut domain_units, &docs_primary_profiles);
        prune_redundant_single_module_units(&mut domain_units, module_tree);

        // Steering: 过滤低权重单元
        if weight_threshold > 0.0 {
            domain_units.retain(|u| u.priority >= weight_threshold);
        }

        // Steering: 限制域内单元数
        if max_per_domain > 0 && domain_units.len() > max_per_domain {
            domain_units.sort_by(|a, b| {
                b.priority
                    .partial_cmp(&a.priority)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            domain_units.truncate(max_per_domain);
        }

        if domain_units.is_empty() {
            continue;
        }

        let domain_index_id = if should_skip_synthetic_domain_index(
            domain,
            &domain_units,
            &docs_primary_profiles,
            &domain_label,
        ) {
            None
        } else {
            let domain_index = KnowledgeUnit::new(
                UnitType::DomainIndex,
                &domain.label,
                &domain.id,
                format!("{domain_label}/{domain_label}.md"),
            );
            let domain_index_id = domain_index.id.clone();
            units.push(domain_index);
            Some(domain_index_id)
        };

        for mut unit in domain_units {
            if unit.parent_unit_id.is_none() {
                unit.parent_unit_id = domain_index_id.clone();
            }
            units.push(unit);
        }
    }

    // 4. 建立父子关系
    let original_parent_by_id = snapshot_parent_links(&units);
    prune_shadowed_docs_backed_units(&mut units);
    let domain_index_by_domain = collect_domain_index_by_domain(&units);
    repair_orphan_parent_links(&mut units, &original_parent_by_id, &domain_index_by_domain);
    establish_parent_child_links(&mut units);

    units
}

fn localized_docs_overlay_profile_keys(report: &ScanReport) -> BTreeSet<String> {
    let docs_paths = collect_docs_files(report);
    let preferred_paths = select_preferred_docs_paths(&docs_paths);
    let localized_paths = preferred_paths
        .iter()
        .filter(|path| docs_path_is_localized(path))
        .cloned()
        .collect::<Vec<_>>();

    if localized_paths.len() < 4 {
        return BTreeSet::new();
    }

    localized_paths
        .iter()
        .map(|path| classify_docs_unit_profile(path).1.as_str().to_string())
        .collect()
}

fn prune_shadowed_source_units(
    units: &mut Vec<KnowledgeUnit>,
    docs_primary_profiles: &BTreeSet<String>,
) {
    if docs_primary_profiles.is_empty() {
        return;
    }

    units.retain(|unit| {
        if !unit.scope.docs_anchors.is_empty() {
            return true;
        }
        if unit.parent_unit_id.is_some() || !unit.child_unit_ids.is_empty() {
            return true;
        }
        !unit_looks_like_raw_source_page(unit)
    });
}

fn should_skip_synthetic_domain_index(
    domain: &KnowledgeDomain,
    domain_units: &[KnowledgeUnit],
    docs_primary_profiles: &BTreeSet<String>,
    domain_label: &str,
) -> bool {
    let docs_primary_skip = !docs_primary_profiles.is_empty()
        && matches!(
            domain.domain_type,
            DomainType::ConceptGuide
                | DomainType::ApiReference
                | DomainType::ConfigReference
                | DomainType::Troubleshooting
        )
        && domain_units
            .iter()
            .all(|unit| !unit.scope.docs_anchors.is_empty());
    let thin_domain_skip = domain_units.len() <= 4
        && !domain_units_have_internal_aggregate_roots(domain_units)
        && domain_units
            .iter()
            .all(|unit| unit.parent_unit_id.is_none() && unit.child_unit_ids.is_empty());

    docs_primary_skip
        || thin_domain_skip
        || domain_units_have_internal_aggregate_roots(domain_units)
        || domain_units_conflict_with_synthetic_domain_index(domain_units, domain_label)
}

fn domain_units_have_internal_aggregate_roots(domain_units: &[KnowledgeUnit]) -> bool {
    let parent_ids: BTreeSet<&str> = domain_units
        .iter()
        .filter_map(|unit| unit.parent_unit_id.as_deref())
        .collect();
    let aggregate_root_count = domain_units
        .iter()
        .filter(|unit| {
            unit.parent_unit_id.is_none()
                && parent_ids.contains(unit.id.as_str())
                && unit.scope.docs_anchors.is_empty()
        })
        .count();
    aggregate_root_count >= 1
}

fn domain_units_conflict_with_synthetic_domain_index(
    domain_units: &[KnowledgeUnit],
    domain_label: &str,
) -> bool {
    let synthetic_relative_path = format!("{domain_label}/{domain_label}.md");
    domain_units
        .iter()
        .any(|unit| unit.relative_path == synthetic_relative_path)
}

fn prune_redundant_single_module_units(units: &mut Vec<KnowledgeUnit>, module_tree: &ModuleTree) {
    if units.len() < 2 {
        return;
    }

    let module_name_by_id: BTreeMap<&str, &str> = module_tree
        .modules
        .iter()
        .map(|module| (module.id.as_str(), module.name.as_str()))
        .collect();
    let mut indexes_to_remove = BTreeSet::new();

    for (index, unit) in units.iter().enumerate() {
        let Some((module_id, module_name)) =
            generic_single_module_unit_identity(unit, &module_name_by_id)
        else {
            continue;
        };
        if !unit_profile_supports_single_module_pruning(unit.decomposition_profile.as_ref()) {
            continue;
        }

        let candidate_sources: BTreeSet<&str> =
            unit.scope.source_ids.iter().map(String::as_str).collect();
        if candidate_sources.len() < 2 {
            continue;
        }

        let mut covered_sources = BTreeSet::new();
        let mut covering_units = 0usize;
        let mut docs_backed_cover = false;
        for (other_index, other) in units.iter().enumerate() {
            if other_index == index
                || !unit_can_cover_single_module_candidate(
                    other,
                    module_id,
                    module_name,
                    unit.decomposition_profile.as_ref(),
                    &module_name_by_id,
                )
            {
                continue;
            }

            let overlap = other
                .scope
                .source_ids
                .iter()
                .map(String::as_str)
                .filter(|source_id| candidate_sources.contains(source_id))
                .collect::<Vec<_>>();
            if overlap.is_empty() {
                continue;
            }

            covering_units += 1;
            if !other.scope.docs_anchors.is_empty() {
                docs_backed_cover = true;
            }
            covered_sources.extend(overlap);
        }

        let coverage_ratio = covered_sources.len() as f32 / candidate_sources.len() as f32;
        let min_covering_units = match unit.decomposition_profile.as_ref() {
            Some(DecompositionProfile::IntegrationPlatform | DecompositionProfile::ApiSurface) => 3,
            _ => 2,
        };
        let aggregate_cover = units.iter().enumerate().any(|(other_index, other)| {
            other_index != index
                && other.parent_unit_id.is_none()
                && !other.child_unit_ids.is_empty()
                && unit_can_cover_single_module_candidate(
                    other,
                    module_id,
                    module_name,
                    unit.decomposition_profile.as_ref(),
                    &module_name_by_id,
                )
        });
        let broad_cover =
            coverage_ratio >= 0.8 && (docs_backed_cover || covering_units >= min_covering_units);
        let aggregate_dense_cover =
            aggregate_cover && coverage_ratio >= 0.65 && covering_units >= 3;
        let docs_dense_cover = docs_backed_cover && coverage_ratio >= 0.6;
        if broad_cover || aggregate_dense_cover || docs_dense_cover {
            indexes_to_remove.insert(index);
        }
    }

    if indexes_to_remove.is_empty() {
        return;
    }

    *units = units
        .drain(..)
        .enumerate()
        .filter_map(|(index, unit)| (!indexes_to_remove.contains(&index)).then_some(unit))
        .collect();
}

fn unit_profile_supports_single_module_pruning(profile: Option<&DecompositionProfile>) -> bool {
    matches!(
        profile,
        Some(
            DecompositionProfile::Runtime
                | DecompositionProfile::CompilerPipeline
                | DecompositionProfile::Testing
                | DecompositionProfile::IntegrationPlatform
                | DecompositionProfile::ApiSurface
        )
    )
}

fn prune_shadowed_docs_backed_units(units: &mut Vec<KnowledgeUnit>) {
    if units.len() < 2 {
        return;
    }

    let structural_titles: BTreeSet<String> = units
        .iter()
        .filter(|unit| unit.scope.docs_anchors.is_empty())
        .map(|unit| normalized_unit_title(&unit.title))
        .collect();

    units.retain(|unit| {
        if unit.scope.docs_anchors.is_empty() {
            return true;
        }
        if !matches!(
            unit.unit_type,
            UnitType::ConceptGuide
                | UnitType::IntegrationDoc
                | UnitType::WorkflowDoc
                | UnitType::TroubleshootDoc
        ) {
            return true;
        }

        let normalized_title = normalized_unit_title(&unit.title);
        if normalized_title.is_empty() || !structural_titles.contains(&normalized_title) {
            return true;
        }

        docs_backed_unit_source_depth(unit) > 2
    });
}

fn normalized_unit_title(title: &str) -> String {
    title
        .chars()
        .filter_map(|character| {
            let lower = character.to_ascii_lowercase();
            (lower.is_ascii_alphanumeric()).then_some(lower)
        })
        .collect()
}

fn docs_backed_unit_source_depth(unit: &KnowledgeUnit) -> usize {
    unit.scope
        .docs_anchors
        .first()
        .map(|anchor| {
            normalize_path(&anchor.file_path)
                .split('/')
                .filter(|segment| !segment.is_empty())
                .count()
        })
        .unwrap_or(usize::MAX)
}

fn generic_single_module_unit_identity<'a>(
    unit: &'a KnowledgeUnit,
    module_name_by_id: &BTreeMap<&'a str, &'a str>,
) -> Option<(&'a str, &'a str)> {
    if !unit.scope.docs_anchors.is_empty() {
        return None;
    }
    let [module_id] = unit.scope.module_ids.as_slice() else {
        return None;
    };
    let module_name = *module_name_by_id.get(module_id.as_str())?;
    unit_is_generic_single_module_page(unit, module_name)
        .then_some((module_id.as_str(), module_name))
}

fn unit_is_generic_single_module_page(unit: &KnowledgeUnit, module_name: &str) -> bool {
    match unit.unit_type {
        UnitType::ModuleDoc => unit.title == module_name,
        UnitType::ApiDoc => unit.title == format!("API：{module_name}"),
        UnitType::TestDoc => unit.title == format!("测试：{module_name}"),
        _ => false,
    }
}

fn unit_can_cover_single_module_candidate<'a>(
    unit: &KnowledgeUnit,
    module_id: &str,
    module_name: &str,
    candidate_profile: Option<&DecompositionProfile>,
    module_name_by_id: &BTreeMap<&'a str, &'a str>,
) -> bool {
    if unit.scope.source_ids.is_empty() {
        return false;
    }
    if unit.decomposition_profile.as_ref() != candidate_profile {
        return false;
    }
    if let Some((other_module_id, _)) = generic_single_module_unit_identity(unit, module_name_by_id)
    {
        return other_module_id != module_id
            || !unit_is_generic_single_module_page(unit, module_name);
    }
    unit.scope.module_ids.is_empty()
        || unit
            .scope
            .module_ids
            .iter()
            .any(|candidate| candidate == module_id)
}

fn plan_units_for_domain(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    match domain.domain_type {
        DomainType::CoreRuntime => plan_runtime_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::Framework => plan_framework_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::PlatformBinding | DomainType::ThemeSystem => plan_module_doc_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::BuildSystem => plan_build_system_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::CompilerToolchain => plan_compiler_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::PluginEcosystem => plan_plugin_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::MultiFramework => plan_module_doc_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::DevTooling => plan_dev_tooling_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::TestingInfra => plan_testing_units(domain, module_tree, report, domain_label),
        DomainType::ConceptGuide => plan_concept_guide_units(domain, report, domain_label),
        DomainType::ApiReference => plan_api_doc_units(
            domain,
            module_tree,
            report,
            module_context_index,
            domain_label,
        ),
        DomainType::ConfigReference => plan_config_doc_units(domain, report, domain_label),
        DomainType::Troubleshooting => plan_troubleshooting_units(domain, report, domain_label),
    }
}

/// 按模块结构拆分——每个 source_module 生成一个 ModuleDoc 单元。
fn plan_module_doc_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();

    for module_id in &domain.source_modules {
        let Some(module) = module_tree.module_by_id(module_id) else {
            continue;
        };

        let module_title = module_unit_title(domain, module);
        let module_name = sanitize_path_segment(&module_title);
        let mut unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            &module_title,
            &domain.id,
            format!("{domain_label}/{module_name}.md"),
        );
        unit.decomposition_profile = Some(module_domain_profile(&domain.domain_type));

        unit.scope = UnitScope {
            module_ids: vec![module_id.clone()],
            source_ids: module.source_ids.clone(),
            ..Default::default()
        };

        if let Some(ctx) = module_context_index.get(module_id.as_str()) {
            unit.scope.relation_ids = ctx
                .dependencies
                .iter()
                .chain(ctx.dependents.iter())
                .cloned()
                .collect();
        }

        unit.priority = compute_module_priority(module, report);
        units.push(unit);
    }

    units
}

fn plan_build_system_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    )
}

fn plugin_addon_capability() -> RepoSignalCapabilitySpec {
    RepoSignalCapabilitySpec {
        capability_key: "addon_ecosystem",
        materializations: vec![RepoSignalMaterializationSpec::Family(
            RepoSignalFamilySpec {
                root_prefix: "插件系统/核心Addons详解",
                family_title: "核心Addons详解",
                family_unit_type: UnitType::ModuleDoc,
                family_profile: DecompositionProfile::IntegrationPlatform,
                family_priority: 0.74,
                leaves: &[
                    DomainSignalLeafSpec {
                        title: "Docs Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 1,
                        priority: 0.72,
                        keywords: &["addons/docs", "docspage", "docsrenderer", "mdx"],
                    },
                    DomainSignalLeafSpec {
                        title: "A11y Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 2,
                        priority: 0.7,
                        keywords: &["addons/a11y", "accessibility", "contrast", "a11y"],
                    },
                    DomainSignalLeafSpec {
                        title: "Actions Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 1,
                        priority: 0.68,
                        keywords: &["actions", "action-logger", "addon-actions"],
                    },
                    DomainSignalLeafSpec {
                        title: "Controls Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 1,
                        priority: 0.68,
                        keywords: &["controls", "argtypes", "args-table"],
                    },
                    DomainSignalLeafSpec {
                        title: "Themes Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 1,
                        priority: 0.68,
                        keywords: &["themes", "themeprovider", "theme-switcher", "theming"],
                    },
                    DomainSignalLeafSpec {
                        title: "Links Addon",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 1,
                        priority: 0.66,
                        keywords: &["addons/links", "linkto", "links"],
                    },
                ],
            },
        )],
    }
}

fn dev_tooling_capabilities() -> Vec<RepoSignalCapabilitySpec> {
    vec![
        RepoSignalCapabilitySpec {
            capability_key: "tooling_integrations",
            materializations: vec![RepoSignalMaterializationSpec::Family(
                RepoSignalFamilySpec {
                    root_prefix: "高级功能/工具集成",
                    family_title: "工具集成",
                    family_unit_type: UnitType::ModuleDoc,
                    family_profile: DecompositionProfile::IntegrationPlatform,
                    family_priority: 0.68,
                    leaves: &[DomainSignalLeafSpec {
                        title: "ESLint集成",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 2,
                        priority: 0.66,
                        keywords: &["eslint", "eslint-plugin", ".eslintrc", "eslint.config"],
                    }],
                },
            )],
        },
        RepoSignalCapabilitySpec {
            capability_key: "operability_observability",
            materializations: vec![RepoSignalMaterializationSpec::Standalone(
                RepoSignalStandaloneSpec {
                    root_prefix: "高级功能",
                    leaves: &[DomainSignalLeafSpec {
                        title: "性能监控",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::IntegrationPlatform,
                        min_hits: 2,
                        priority: 0.64,
                        keywords: &[
                            "telemetry",
                            "performance",
                            "monitor",
                            "disable-telemetry",
                            "enable-crash-reports",
                        ],
                    }],
                },
            )],
        },
        RepoSignalCapabilitySpec {
            capability_key: "delivery_pipelines",
            materializations: vec![RepoSignalMaterializationSpec::Family(
                RepoSignalFamilySpec {
                    root_prefix: "部署和CI_CD",
                    family_title: "部署和CI_CD",
                    family_unit_type: UnitType::WorkflowDoc,
                    family_profile: DecompositionProfile::Testing,
                    family_priority: 0.68,
                    leaves: &[
                        DomainSignalLeafSpec {
                            title: "CI_CD集成",
                            unit_type: UnitType::WorkflowDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &[
                                "in-ci",
                                "circleci",
                                "github/workflows",
                                "pipeline",
                                "workflow",
                            ],
                        },
                        DomainSignalLeafSpec {
                            title: "Chromatic集成",
                            unit_type: UnitType::WorkflowDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &["chromatic", "visual-testing", "visualtest", "prbadge"],
                        },
                    ],
                },
            )],
        },
    ]
}

fn testing_capabilities() -> Vec<RepoSignalCapabilitySpec> {
    vec![
        RepoSignalCapabilitySpec {
            capability_key: "testing_practices",
            materializations: vec![RepoSignalMaterializationSpec::Family(
                RepoSignalFamilySpec {
                    root_prefix: "测试策略与最佳实践",
                    family_title: "测试策略与最佳实践",
                    family_unit_type: UnitType::TestDoc,
                    family_profile: DecompositionProfile::Testing,
                    family_priority: 0.72,
                    leaves: &[
                        DomainSignalLeafSpec {
                            title: "单元测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 3,
                            priority: 0.68,
                            keywords: &["test", "subject", "assert", "junit", "spec"],
                        },
                        DomainSignalLeafSpec {
                            title: "集成测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &[
                                "integration",
                                "functional",
                                "androidtest",
                                "sharedtest",
                                "emulator",
                            ],
                        },
                        DomainSignalLeafSpec {
                            title: "性能测试与监控",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.64,
                            keywords: &["benchmark", "performance", "monitor", "profile", "timing"],
                        },
                        DomainSignalLeafSpec {
                            title: "Android测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.68,
                            keywords: &[
                                "androidtest",
                                "hiltandroidtest",
                                "hiltandroidrule",
                                "testinstallin",
                                "customtestapplication",
                            ],
                        },
                    ],
                },
            )],
        },
        RepoSignalCapabilitySpec {
            capability_key: "testing_frameworks",
            materializations: vec![RepoSignalMaterializationSpec::Family(
                RepoSignalFamilySpec {
                    root_prefix: "测试框架",
                    family_title: "测试框架",
                    family_unit_type: UnitType::TestDoc,
                    family_profile: DecompositionProfile::Testing,
                    family_priority: 0.7,
                    leaves: &[
                        DomainSignalLeafSpec {
                            title: "Vitest集成",
                            unit_type: UnitType::IntegrationDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.69,
                            keywords: &[
                                "vitest",
                                "vitest-addon",
                                "vitestconfig",
                                "storybooktest",
                                "browser-playwright",
                            ],
                        },
                        DomainSignalLeafSpec {
                            title: "Playwright测试",
                            unit_type: UnitType::IntegrationDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.67,
                            keywords: &[
                                "playwright",
                                "test-runner",
                                "stories-in-end-to-end-tests",
                                "browser-playwright",
                            ],
                        },
                        DomainSignalLeafSpec {
                            title: "可访问性测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &["accessibility", "a11y", "axe", "accessibility-testing"],
                        },
                        DomainSignalLeafSpec {
                            title: "组件测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &[
                                "component-test",
                                "storybooktest",
                                "stories-in-unit-tests",
                                "interaction-testing",
                            ],
                        },
                        DomainSignalLeafSpec {
                            title: "视觉回归测试",
                            unit_type: UnitType::TestDoc,
                            profile: DecompositionProfile::Testing,
                            min_hits: 2,
                            priority: 0.66,
                            keywords: &[
                                "visual-testing",
                                "chromatic",
                                "visualtest",
                                "snapshot-testing",
                            ],
                        },
                    ],
                },
            )],
        },
    ]
}

fn plan_plugin_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    );
    let capabilities = vec![plugin_addon_capability()];
    units.extend(plan_repo_signal_capabilities(domain, report, &capabilities));
    units
}

fn plan_dev_tooling_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    );
    let capabilities = dev_tooling_capabilities();
    units.extend(plan_repo_signal_capabilities(domain, report, &capabilities));
    units
}
fn plan_framework_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    );

    if let Some(archetype_signal) = domain_matches_keywords(
        domain,
        report,
        &["hilt", "hiltandroidapp", "androidentrypoint", "installin"],
    ) {
        let mut hilt_units = plan_domain_signal_family_units(
            domain,
            report,
            domain_label,
            "Hilt框架详解",
            UnitType::ModuleDoc,
            DecompositionProfile::IntegrationPlatform,
            0.79,
            &[
                DomainSignalLeafSpec {
                    title: "编译时处理与代码生成",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::IntegrationPlatform,
                    min_hits: 2,
                    priority: 0.76,
                    keywords: &[
                        "processor",
                        "processingstep",
                        "componenttreedeps",
                        "generatesrootinput",
                        "ksp",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "组件树结构与生命周期",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::IntegrationPlatform,
                    min_hits: 2,
                    priority: 0.74,
                    keywords: &[
                        "generatedcomponent",
                        "generatedcomponentmanager",
                        "componenttree",
                        "scope",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "模块安装与依赖管理",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::IntegrationPlatform,
                    min_hits: 2,
                    priority: 0.74,
                    keywords: &[
                        "installin",
                        "uninstallmodules",
                        "aggregateddeps",
                        "bindvalue",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "测试支持与模拟",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::IntegrationPlatform,
                    min_hits: 2,
                    priority: 0.72,
                    keywords: &["testinjector", "bindvalue", "testing", "uninstallmodules"],
                },
                DomainSignalLeafSpec {
                    title: "@HiltAndroidApp与注解使用",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::IntegrationPlatform,
                    min_hits: 2,
                    priority: 0.76,
                    keywords: &[
                        "hiltandroidapp",
                        "androidentrypoint",
                        "entrypoint",
                        "generatesrootinput",
                    ],
                },
            ],
        );
        append_planner_signal_to_units(
            &mut hilt_units,
            make_planner_signal_bundle(
                PlannerSignalKind::RepoArchetype,
                repo_archetype_key("annotation_di_hilt"),
                "注解驱动框架集成信号",
                &archetype_signal,
            ),
        );
        units.extend(hilt_units);
    }

    units
}

fn plan_runtime_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    );
    if domain.label != "核心模块" {
        if let Some(archetype_signal) = domain_has_annotation_di_signal(domain, report) {
            let mut runtime_core_units = plan_domain_signal_family_units(
                domain,
                report,
                domain_label,
                "核心概念",
                UnitType::ModuleDoc,
                DecompositionProfile::Runtime,
                0.8,
                &[
                    DomainSignalLeafSpec {
                        title: "@Inject 注解详解",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::Runtime,
                        min_hits: 2,
                        priority: 0.78,
                        keywords: &["inject", "membersinjector", "injected"],
                    },
                    DomainSignalLeafSpec {
                        title: "绑定与提供者模式",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::Runtime,
                        min_hits: 3,
                        priority: 0.76,
                        keywords: &[
                            "provider",
                            "lazy",
                            "factory",
                            "assisted",
                            "bindsinstance",
                            "multibinds",
                            "intoset",
                            "intomap",
                        ],
                    },
                    DomainSignalLeafSpec {
                        title: "多值绑定高级用法",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::Runtime,
                        min_hits: 2,
                        priority: 0.7,
                        keywords: &[
                            "multibinds",
                            "intoset",
                            "intomap",
                            "mapkey",
                            "elementsintoset",
                        ],
                    },
                ],
            );
            append_planner_signal_to_units(
                &mut runtime_core_units,
                make_planner_signal_bundle(
                    PlannerSignalKind::RepoArchetype,
                    repo_archetype_key("annotation_di_runtime"),
                    "注解驱动运行时信号",
                    &archetype_signal,
                ),
            );
            units.extend(runtime_core_units);

            let mut runtime_advanced_units = plan_domain_signal_family_units(
                domain,
                report,
                domain_label,
                "高级特性与扩展",
                UnitType::ModuleDoc,
                DecompositionProfile::Runtime,
                0.76,
                &[
                    DomainSignalLeafSpec {
                        title: "多值绑定高级用法",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::Runtime,
                        min_hits: 2,
                        priority: 0.72,
                        keywords: &[
                            "multibinds",
                            "intoset",
                            "intomap",
                            "mapkey",
                            "elementsintoset",
                        ],
                    },
                    DomainSignalLeafSpec {
                        title: "延迟初始化与线程安全",
                        unit_type: UnitType::ModuleDoc,
                        profile: DecompositionProfile::Runtime,
                        min_hits: 2,
                        priority: 0.7,
                        keywords: &[
                            "lazy",
                            "provideroflazy",
                            "doublecheck",
                            "singlecheck",
                            "thread",
                        ],
                    },
                ],
            );
            append_planner_signal_to_units(
                &mut runtime_advanced_units,
                make_planner_signal_bundle(
                    PlannerSignalKind::RepoArchetype,
                    repo_archetype_key("annotation_di_runtime"),
                    "注解驱动运行时信号",
                    &archetype_signal,
                ),
            );
            units.extend(runtime_advanced_units);
        }
    }
    units
}

fn plan_compiler_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_module_doc_units(
        domain,
        module_tree,
        report,
        module_context_index,
        domain_label,
    );
    units.extend(plan_docs_backed_units_for_domain(
        domain,
        report,
        domain_label,
        |profile| matches!(profile, DecompositionProfile::CompilerPipeline),
    ));
    if let Some(archetype_signal) = domain_has_annotation_di_signal(domain, report) {
        let mut compiler_family_units = plan_domain_signal_family_units(
            domain,
            report,
            domain_label,
            "编译时处理机制",
            UnitType::ModuleDoc,
            DecompositionProfile::CompilerPipeline,
            0.82,
            &[
                DomainSignalLeafSpec {
                    title: "注解处理基础",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::CompilerPipeline,
                    min_hits: 2,
                    priority: 0.78,
                    keywords: &[
                        "annotation",
                        "processingstep",
                        "processor",
                        "processingstepsmodule",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "代码生成策略",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::CompilerPipeline,
                    min_hits: 2,
                    priority: 0.78,
                    keywords: &["codegen", "generator", "sourcefile", "factorygenerator"],
                },
                DomainSignalLeafSpec {
                    title: "依赖图构建",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::CompilerPipeline,
                    min_hits: 2,
                    priority: 0.77,
                    keywords: &[
                        "bindinggraph",
                        "componentdescriptor",
                        "componentpath",
                        "dependencyrequest",
                        "graph",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "验证与错误处理",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::CompilerPipeline,
                    min_hits: 2,
                    priority: 0.77,
                    keywords: &[
                        "validator",
                        "validation",
                        "diagnostic",
                        "errormessages",
                        "report",
                    ],
                },
                DomainSignalLeafSpec {
                    title: "增量编译优化",
                    unit_type: UnitType::ModuleDoc,
                    profile: DecompositionProfile::CompilerPipeline,
                    min_hits: 2,
                    priority: 0.72,
                    keywords: &[
                        "incremental",
                        "clearablecache",
                        "cache",
                        "compileroptions",
                        "processingenvironmentcompileroptions",
                        "validationtype",
                        "fastinit",
                    ],
                },
            ],
        );
        append_planner_signal_to_units(
            &mut compiler_family_units,
            make_planner_signal_bundle(
                PlannerSignalKind::RepoArchetype,
                repo_archetype_key("annotation_di_compiler"),
                "注解驱动编译链信号",
                &archetype_signal,
            ),
        );
        units.extend(compiler_family_units);

        let mut compiler_leaf_units = plan_domain_signal_leaf_units(
            domain,
            report,
            domain_label,
            &[DomainSignalLeafSpec {
                title: "SPI扩展机制",
                unit_type: UnitType::ModuleDoc,
                profile: DecompositionProfile::CompilerPipeline,
                min_hits: 2,
                priority: 0.72,
                keywords: &["spi", "bindinggraphplugin", "plugin", "serviceloader"],
            }],
        );
        append_planner_signal_to_units(
            &mut compiler_leaf_units,
            make_planner_signal_bundle(
                PlannerSignalKind::RepoArchetype,
                repo_archetype_key("annotation_di_compiler"),
                "注解驱动编译链信号",
                &archetype_signal,
            ),
        );
        units.extend(compiler_leaf_units);
    }
    units
}

fn plan_test_doc_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();

    for module_id in &domain.source_modules {
        let Some(module) = module_tree.module_by_id(module_id) else {
            continue;
        };

        let module_name = sanitize_path_segment(&module.name);
        let mut unit = KnowledgeUnit::new(
            UnitType::TestDoc,
            format!("测试：{}", module.name),
            &domain.id,
            format!("{domain_label}/{module_name}.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::Testing);
        unit.scope = UnitScope {
            module_ids: vec![module_id.clone()],
            source_ids: module.source_ids.clone(),
            ..Default::default()
        };
        unit.priority = 0.5;
        units.push(unit);
    }

    // If no test modules but test files exist, create a single TestDoc
    if units.is_empty() && !domain.source_files.is_empty() {
        let mut unit = KnowledgeUnit::new(
            UnitType::TestDoc,
            "测试策略",
            &domain.id,
            format!("{domain_label}/测试策略.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::Testing);
        unit.scope = UnitScope {
            source_ids: domain
                .source_files
                .iter()
                .filter_map(|path| {
                    report
                        .files
                        .iter()
                        .find(|f| f.path == *path)
                        .map(|f| f.id.clone())
                })
                .collect(),
            ..Default::default()
        };
        unit.priority = 0.5;
        units.push(unit);
    }

    units
}

fn plan_testing_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_test_doc_units(domain, module_tree, report, domain_label);
    let capabilities = testing_capabilities();
    units.extend(plan_repo_signal_capabilities(domain, report, &capabilities));
    units
}

fn plan_concept_guide_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_docs_backed_units_for_domain(domain, report, domain_label, |profile| {
        !matches!(
            profile,
            DecompositionProfile::ApiSurface
                | DecompositionProfile::ConfigSurface
                | DecompositionProfile::Troubleshooting
        )
    });
    units.extend(plan_raw_example_source_units(domain, report, &units));
    units
}

fn plan_raw_example_source_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    existing_units: &[KnowledgeUnit],
) -> Vec<KnowledgeUnit> {
    let example_doc_count = existing_units
        .iter()
        .filter(|unit| unit.unit_type == UnitType::ExampleDoc)
        .count();
    if example_doc_count >= 2 {
        return Vec::new();
    }

    let Some(readme_path) = repo_root_readme_path(report) else {
        return Vec::new();
    };
    let Some(example_cluster) = select_representative_example_source_cluster(report) else {
        return Vec::new();
    };

    let existing_titles: BTreeSet<&str> = existing_units
        .iter()
        .map(|unit| unit.title.as_str())
        .collect();
    let existing_paths: BTreeSet<&str> = existing_units
        .iter()
        .map(|unit| unit.relative_path.as_str())
        .collect();
    let mut units = Vec::new();

    if !existing_titles.contains("快速开始") && !existing_paths.contains("快速开始.md") {
        let mut onboarding_paths = vec![readme_path.clone()];
        onboarding_paths.extend(example_cluster.reference_paths.iter().take(6).cloned());
        sort_and_dedup_strings(&mut onboarding_paths);

        let mut unit = KnowledgeUnit::new(
            UnitType::ExampleDoc,
            "快速开始",
            &domain.id,
            "快速开始.md".to_string(),
        );
        unit.decomposition_profile = Some(DecompositionProfile::ExampleTutorial);
        unit.scope = UnitScope {
            source_ids: source_ids_for_paths(report, &onboarding_paths),
            docs_anchors: vec![DocsAnchor {
                file_path: normalize_path(&readme_path),
                heading: "快速开始".to_string(),
                level: 1,
                links: Vec::new(),
            }],
            ..Default::default()
        };
        unit.priority = 0.72;
        units.push(unit);
    }

    if existing_titles.contains("基础示例")
        || existing_paths.contains("示例与教程/基础示例.md")
        || example_cluster.reference_paths.is_empty()
    {
        return units;
    }

    let example_family = KnowledgeUnit::new(
        UnitType::ExampleDoc,
        "示例与教程",
        &domain.id,
        "示例与教程/示例与教程.md".to_string(),
    );
    let example_family_id = example_family.id.clone();
    let mut example_family = example_family;
    example_family.decomposition_profile = Some(DecompositionProfile::ExampleTutorial);
    example_family.scope = UnitScope {
        source_ids: source_ids_for_paths(report, &example_cluster.reference_paths),
        ..Default::default()
    };
    example_family.priority = 0.68;
    units.push(example_family);

    let mut example_unit = KnowledgeUnit::new(
        UnitType::ExampleDoc,
        "基础示例",
        &domain.id,
        "示例与教程/基础示例.md".to_string(),
    );
    example_unit.parent_unit_id = Some(example_family_id);
    example_unit.decomposition_profile = Some(DecompositionProfile::ExampleTutorial);
    example_unit.scope = UnitScope {
        source_ids: source_ids_for_paths(report, &example_cluster.reference_paths),
        ..Default::default()
    };
    example_unit.priority = 0.7;
    units.push(example_unit);

    units
}

fn plan_api_doc_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();
    let has_docs_backed_api = domain
        .evidence
        .docs_anchors
        .iter()
        .any(|anchor| !anchor.file_path.trim().is_empty());

    for module_id in &domain.source_modules {
        let Some(module) = module_tree.module_by_id(module_id) else {
            continue;
        };
        let Some(ctx) = module_context_index.get(module_id.as_str()) else {
            continue;
        };

        let explicit_api_symbols = module_explicit_api_symbols(module, report);
        if ctx.public_surface.is_empty() && explicit_api_symbols.is_empty() {
            continue;
        }
        if has_docs_backed_api && !module_has_explicit_api_signal(module) {
            continue;
        }

        let module_name = sanitize_path_segment(&module.name);
        let mut unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            format!("API：{}", module.name),
            &domain.id,
            format!("{domain_label}/{module_name}.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::ApiSurface);
        unit.scope = UnitScope {
            module_ids: vec![module_id.clone()],
            source_ids: module.source_ids.clone(),
            api_surfaces: vec![ApiSurface {
                module_id: module_id.clone(),
                exported_symbols: if ctx.public_surface.is_empty() {
                    explicit_api_symbols
                } else {
                    ctx.public_surface.clone()
                },
            }],
            ..Default::default()
        };
        unit.priority = 0.7;
        units.push(unit);
    }

    units.extend(plan_docs_backed_units_for_domain(
        domain,
        report,
        domain_label,
        |profile| matches!(profile, DecompositionProfile::ApiSurface),
    ));
    units.extend(plan_domain_signal_family_units(
        domain,
        report,
        domain_label,
        "开发API参考",
        UnitType::ApiDoc,
        DecompositionProfile::ApiSurface,
        0.76,
        &[
            DomainSignalLeafSpec {
                title: "Addon API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &["addon", "addons", "addon-types"],
            },
            DomainSignalLeafSpec {
                title: "插件API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &["addon", "addons", "addon-types"],
            },
            DomainSignalLeafSpec {
                title: "Decorators API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["decorator", "decorators", "withdecorator", "make-decorator"],
            },
            DomainSignalLeafSpec {
                title: "CSF API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &[
                    "docs/api/csf",
                    "/store/csf/",
                    "csf",
                    "composeconfigs",
                    "processcsffile",
                    "portable-stories",
                ],
            },
            DomainSignalLeafSpec {
                title: "Preview API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &["preview", "preview-web", "process-preview-annotation"],
            },
            DomainSignalLeafSpec {
                title: "Hooks API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["hooks", "useargs", "useglobals", "usestorybookstate"],
            },
            DomainSignalLeafSpec {
                title: "Store API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["store", "story-store", "usestorybookstate"],
            },
            DomainSignalLeafSpec {
                title: "Types API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["types", "typings", "public-types", "addon-types"],
            },
        ],
    ));
    units.extend(plan_domain_signal_family_units(
        domain,
        report,
        domain_label,
        "类型定义参考",
        UnitType::ApiDoc,
        DecompositionProfile::ApiSurface,
        0.74,
        &[
            DomainSignalLeafSpec {
                title: "API类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["types", "typings", "core-annotations", "story.ts"],
            },
            DomainSignalLeafSpec {
                title: "插件类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["addon-types", "addons", "plugin", "decorator"],
            },
            DomainSignalLeafSpec {
                title: "工具类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["builder", "builders", "cli", "tool"],
            },
            DomainSignalLeafSpec {
                title: "构建器类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["builder", "builders", "build-config", "vite-config"],
            },
            DomainSignalLeafSpec {
                title: "核心类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &[
                    "public-types",
                    "typings",
                    "channels/types",
                    "/core/src/types/",
                ],
            },
            DomainSignalLeafSpec {
                title: "框架类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["framework", "frameworks", "renderer-to-framework"],
            },
        ],
    ));
    units.extend(plan_domain_signal_family_units(
        domain,
        report,
        domain_label,
        "配置API参考",
        UnitType::ApiDoc,
        DecompositionProfile::ApiSurface,
        0.76,
        &[
            DomainSignalLeafSpec {
                title: "main.js配置",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &[".storybook/main", "definemain", "vitefinal", "staticdirs"],
            },
            DomainSignalLeafSpec {
                title: "preview.js配置",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &[
                    ".storybook/preview",
                    "previewannotations",
                    "themeprovider",
                    "docscontext",
                    "sb_theme",
                ],
            },
            DomainSignalLeafSpec {
                title: "manager.js配置",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &[
                    ".storybook/manager",
                    "manager-api",
                    "layout.ts",
                    "addons/register",
                ],
            },
            DomainSignalLeafSpec {
                title: "构建器配置",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.74,
                keywords: &[
                    "builder-vite",
                    "builder-webpack",
                    "build-config",
                    "vite-config",
                    "storybook-config-plugin",
                ],
            },
            DomainSignalLeafSpec {
                title: "预设配置",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.72,
                keywords: &["preset", "presets", "framework-preset", "common-preset"],
            },
        ],
    ));
    units.extend(plan_domain_signal_leaf_units(
        domain,
        report,
        domain_label,
        &[
            DomainSignalLeafSpec {
                title: "Angular框架支持",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["angular", "framework-preset-angular", "zone"],
            },
            DomainSignalLeafSpec {
                title: "组件故事（Stories）",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["story", "stories", "csf"],
            },
        ],
    ));
    let mut signal_leaves = Vec::new();
    if domain_has_annotation_di_signal(domain, report).is_some() {
        signal_leaves.extend([
            DomainSignalLeafSpec {
                title: "Android API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.76,
                keywords: &[
                    "androidinjector",
                    "hasandroidinjector",
                    "androidinjection",
                    "contributesandroidinjector",
                    "daggerappcompatactivity",
                    "androidinjectordescriptor",
                ],
            },
            DomainSignalLeafSpec {
                title: "Hilt API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.76,
                keywords: &[
                    "hiltandroidapp",
                    "androidentrypoint",
                    "installin",
                    "definecomponent",
                    "entrypoint",
                    "generatesrootinput",
                ],
            },
            DomainSignalLeafSpec {
                title: "编译时API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.76,
                keywords: &[
                    "componentprocessor",
                    "delegatecomponentprocessor",
                    "bindinggraphfactory",
                    "sourcefilegenerator",
                    "processingstep",
                    "validationreport",
                ],
            },
            DomainSignalLeafSpec {
                title: "运行时API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.76,
                keywords: &[
                    "membersinjector",
                    "bindsinstance",
                    "subcomponent",
                    "assistedfactory",
                ],
            },
            DomainSignalLeafSpec {
                title: "异步处理与生产者",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.74,
                keywords: &[
                    "producer",
                    "produces",
                    "productioncomponent",
                    "productionmodule",
                ],
            },
            DomainSignalLeafSpec {
                title: "@Provides 与 @Binds 注解详解",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 2,
                priority: 0.76,
                keywords: &[
                    "provides",
                    "binds",
                    "providesmethodvalidator",
                    "bindsmethodvalidator",
                ],
            },
        ]);
    }
    signal_leaves.push(DomainSignalLeafSpec {
        title: "性能监控与调试",
        unit_type: UnitType::ApiDoc,
        profile: DecompositionProfile::ApiSurface,
        min_hits: 2,
        priority: 0.7,
        keywords: &["timing", "monitor", "benchmark", "debug"],
    });
    let mut signal_units =
        plan_domain_signal_leaf_units(domain, report, domain_label, &signal_leaves);
    if let Some(archetype_signal) = domain_has_annotation_di_signal(domain, report) {
        append_planner_signal_to_units(
            &mut signal_units,
            make_planner_signal_bundle(
                PlannerSignalKind::RepoArchetype,
                repo_archetype_key("annotation_di_api_surface"),
                "注解驱动 API surface 信号",
                &archetype_signal,
            ),
        );
    }
    units.extend(signal_units);

    units
}

struct DomainSignalLeafSpec {
    title: &'static str,
    unit_type: UnitType,
    profile: DecompositionProfile,
    min_hits: usize,
    priority: f32,
    keywords: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, Default)]
struct TopicEvidenceProfile {
    primary_terms: &'static [&'static str],
    secondary_terms: &'static [&'static str],
    contrast_terms: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, Default)]
struct LeafScopeRefinementPolicy {
    topic: TopicEvidenceProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GenericContractClass {
    DeclarationHeavy,
    ConfigManifestHeavy,
    ImplementationHeavy,
    ValidationTestHeavy,
}

#[derive(Debug, Clone)]
struct PathTopicEvidence {
    tokens: BTreeSet<String>,
    docs_shared_tokens: BTreeSet<String>,
    classes: BTreeSet<GenericContractClass>,
}

#[derive(Debug, Clone)]
struct ScoredSignalEntry {
    source_id: String,
    path: String,
    topic_score: i32,
    classes: BTreeSet<GenericContractClass>,
}

#[derive(Debug, Default, Clone)]
struct LeafSignalMatch {
    signal: SignalMatch,
    collapse_guard: Option<CollapseGuardDecision>,
}

struct RepoSignalFamilySpec {
    root_prefix: &'static str,
    family_title: &'static str,
    family_unit_type: UnitType,
    family_profile: DecompositionProfile,
    family_priority: f32,
    leaves: &'static [DomainSignalLeafSpec],
}

struct RepoSignalStandaloneSpec {
    root_prefix: &'static str,
    leaves: &'static [DomainSignalLeafSpec],
}

enum RepoSignalMaterializationSpec {
    Family(RepoSignalFamilySpec),
    Standalone(RepoSignalStandaloneSpec),
}

struct RepoSignalCapabilitySpec {
    capability_key: &'static str,
    materializations: Vec<RepoSignalMaterializationSpec>,
}

#[derive(Debug, Default, Clone)]
struct SignalMatch {
    matched_keywords: Vec<String>,
    matched_source_ids: Vec<String>,
    matched_paths: Vec<String>,
}

impl SignalMatch {
    fn source_count(&self) -> usize {
        self.matched_source_ids.len()
    }

    fn extend(&mut self, other: &SignalMatch) {
        self.matched_keywords = merge_sorted_strings(
            self.matched_keywords.clone(),
            other.matched_keywords.clone(),
        );
        self.matched_source_ids = merge_sorted_strings(
            self.matched_source_ids.clone(),
            other.matched_source_ids.clone(),
        );
        self.matched_paths =
            merge_sorted_strings(self.matched_paths.clone(), other.matched_paths.clone());
    }
}

fn merge_sorted_strings(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    left.into_iter()
        .chain(right)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn lexical_topic_tokens(input: &str) -> BTreeSet<String> {
    let mut tokens = BTreeSet::new();
    let mut current = String::new();
    let mut previous_was_lower_or_digit = false;

    for character in input.chars() {
        if !character.is_ascii_alphanumeric() {
            if !current.is_empty() {
                tokens.insert(current.to_ascii_lowercase());
                current.clear();
            }
            previous_was_lower_or_digit = false;
            continue;
        }

        if character.is_ascii_uppercase() && previous_was_lower_or_digit && !current.is_empty() {
            tokens.insert(current.to_ascii_lowercase());
            current.clear();
        }

        current.push(character);
        previous_was_lower_or_digit = character.is_ascii_lowercase() || character.is_ascii_digit();
    }

    if !current.is_empty() {
        tokens.insert(current.to_ascii_lowercase());
    }

    tokens
}

fn lexical_tokens_for_path(path: &str) -> BTreeSet<String> {
    normalize_path(path)
        .split('/')
        .flat_map(lexical_topic_tokens)
        .collect()
}

fn docs_topic_tokens(text: &str) -> BTreeSet<String> {
    lexical_topic_tokens(text)
}

fn leaf_scope_refinement_policy(leaf: &DomainSignalLeafSpec) -> Option<LeafScopeRefinementPolicy> {
    let topic = match (leaf.profile.clone(), leaf.title) {
        (DecompositionProfile::ApiSurface, "Types API") => TopicEvidenceProfile {
            primary_terms: &[
                "story",
                "stories",
                "component",
                "preview",
                "manager",
                "arg",
                "args",
                "annotations",
                "csf",
            ],
            secondary_terms: &["type", "types", "parameter", "sbtype", "store", "infer"],
            contrast_terms: &["renderer", "framework", "builder", "window"],
        },
        (DecompositionProfile::ConfigSurface, "主题系统概览") => TopicEvidenceProfile {
            primary_terms: &["theme", "themes", "theming"],
            secondary_terms: &["preview", "manager", "switcher", "base", "dark", "light"],
            contrast_terms: &[
                "create",
                "convert",
                "ensure",
                "decorator",
                "provider",
                "a11y",
                "contrast",
                "vision",
                "font",
                "palette",
                "typography",
            ],
        },
        (DecompositionProfile::ConfigSurface, "自定义主题开发") => TopicEvidenceProfile {
            primary_terms: &[
                "create",
                "convert",
                "ensure",
                "decorator",
                "provider",
                "theme",
                "themes",
                "theming",
            ],
            secondary_terms: &["preview", "index", "custom", "utils"],
            contrast_terms: &[
                "a11y",
                "contrast",
                "vision",
                "font",
                "palette",
                "typography",
            ],
        },
        (DecompositionProfile::ConfigSurface, "颜色和字体系统") => TopicEvidenceProfile {
            primary_terms: &[
                "color",
                "font",
                "palette",
                "typography",
                "contrast",
                "vision",
                "a11y",
                "style",
                "css",
            ],
            secondary_terms: &["theme", "themes", "theming", "accessibility"],
            contrast_terms: &[
                "create",
                "convert",
                "ensure",
                "decorator",
                "provider",
                "switcher",
            ],
        },
        _ => return None,
    };

    Some(LeafScopeRefinementPolicy { topic })
}

fn collect_leaf_docs_grounding_terms(
    domain: &KnowledgeDomain,
    leaf: &DomainSignalLeafSpec,
    policy: &LeafScopeRefinementPolicy,
) -> BTreeSet<String> {
    let leaf_terms = lexical_topic_tokens(leaf.title)
        .into_iter()
        .chain(
            policy
                .topic
                .primary_terms
                .iter()
                .map(|term| (*term).to_string()),
        )
        .chain(
            policy
                .topic
                .secondary_terms
                .iter()
                .map(|term| (*term).to_string()),
        )
        .collect::<BTreeSet<_>>();

    let mut docs_terms = BTreeSet::new();
    for anchor in &domain.evidence.docs_anchors {
        let anchor_terms = docs_topic_tokens(&anchor.heading)
            .into_iter()
            .chain(lexical_tokens_for_path(&anchor.file_path))
            .collect::<BTreeSet<_>>();
        if anchor_terms.intersection(&leaf_terms).next().is_some() {
            docs_terms.extend(anchor_terms);
        }
    }

    if docs_terms.is_empty() {
        docs_terms.extend(leaf_terms);
    }

    docs_terms
}

fn classify_generic_contract_classes(path: &str) -> BTreeSet<GenericContractClass> {
    let normalized = normalize_path(path).to_ascii_lowercase();
    let mut classes = BTreeSet::new();
    let file_name = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    let stem = file_name
        .trim_end_matches(".d.ts")
        .trim_end_matches(".tsx")
        .trim_end_matches(".jsx")
        .trim_end_matches(".ts")
        .trim_end_matches(".js")
        .trim_end_matches(".json")
        .trim_end_matches(".md")
        .trim_end_matches(".mdx");

    if signal_path_is_secondary_noise(&normalized) {
        classes.insert(GenericContractClass::ValidationTestHeavy);
    }

    if matches!(
        file_name,
        "package.json"
            | "tsconfig.json"
            | "tsconfig.base.json"
            | "tsconfig.build.json"
            | "tsconfig.test.json"
            | "tsconfig.node.json"
            | "build-config.ts"
            | "build-config.js"
    ) {
        classes.insert(GenericContractClass::ConfigManifestHeavy);
    }

    if normalized.ends_with(".d.ts")
        || matches!(stem, "types" | "typings" | "public-types")
        || normalized.contains("/types/")
        || normalized.contains("/typings/")
    {
        classes.insert(GenericContractClass::DeclarationHeavy);
    }

    if normalized.contains("/src/")
        || normalized.contains("/lib/")
        || normalized.contains("/modules/")
        || normalized.contains("/preview-api/")
        || normalized.contains("/manager-api/")
        || normalized.contains("/theming/")
        || normalized.contains("/addons/")
    {
        classes.insert(GenericContractClass::ImplementationHeavy);
    }

    classes
}

fn build_path_topic_evidence(path: &str, docs_terms: &BTreeSet<String>) -> PathTopicEvidence {
    let tokens = lexical_tokens_for_path(path);
    let docs_shared_tokens = tokens
        .intersection(docs_terms)
        .cloned()
        .collect::<BTreeSet<_>>();

    PathTopicEvidence {
        tokens,
        docs_shared_tokens,
        classes: classify_generic_contract_classes(path),
    }
}

fn score_topic_evidence(evidence: &PathTopicEvidence, policy: &LeafScopeRefinementPolicy) -> i32 {
    let primary_terms = policy
        .topic
        .primary_terms
        .iter()
        .map(|term| (*term).to_string())
        .collect::<BTreeSet<_>>();
    let secondary_terms = policy
        .topic
        .secondary_terms
        .iter()
        .map(|term| (*term).to_string())
        .collect::<BTreeSet<_>>();
    let contrast_terms = policy
        .topic
        .contrast_terms
        .iter()
        .map(|term| (*term).to_string())
        .collect::<BTreeSet<_>>();

    let primary_hits = evidence.tokens.intersection(&primary_terms).count() as i32;
    let secondary_hits = evidence.tokens.intersection(&secondary_terms).count() as i32;
    let docs_hits = evidence.docs_shared_tokens.len() as i32;
    let contrast_hits = evidence.tokens.intersection(&contrast_terms).count() as i32;
    let strong_topic = primary_hits > 0 || docs_hits >= 2;

    let mut score = primary_hits * 10 + secondary_hits * 5 + docs_hits * 4;
    if evidence
        .classes
        .contains(&GenericContractClass::ImplementationHeavy)
    {
        score += 3;
    }
    if evidence
        .classes
        .contains(&GenericContractClass::ValidationTestHeavy)
    {
        score += if strong_topic { 1 } else { -4 };
    }
    if evidence
        .classes
        .contains(&GenericContractClass::DeclarationHeavy)
    {
        score += if strong_topic { -4 } else { -18 };
    }
    if evidence
        .classes
        .contains(&GenericContractClass::ConfigManifestHeavy)
    {
        score += if strong_topic { -6 } else { -20 };
    }
    if contrast_hits > 0 {
        score -= if strong_topic {
            contrast_hits * 3
        } else {
            contrast_hits * 8
        };
    }

    score
}

fn expanded_leaf_signal_candidates<'a>(
    domain: &KnowledgeDomain,
    report: &'a ScanReport,
    candidate_files: &[&'a wiki_index::scanner::ScannedFile],
    policy: &LeafScopeRefinementPolicy,
    docs_terms: &BTreeSet<String>,
) -> Vec<&'a wiki_index::scanner::ScannedFile> {
    let mut candidates = candidate_files
        .iter()
        .copied()
        .map(|file| (file.id.clone(), file))
        .collect::<BTreeMap<_, _>>();

    for file in &report.files {
        if candidates.contains_key(&file.id) || is_markdown_path(&file.path) || file.is_asset_like()
        {
            continue;
        }

        if matches!(domain.domain_type, DomainType::ApiReference)
            && file.is_config_like()
            && !api_signal_candidate_accepts_config_path(&file.path)
        {
            continue;
        }

        let normalized_path = normalize_path(&file.path);
        let evidence = build_path_topic_evidence(&normalized_path, docs_terms);
        if score_topic_evidence(&evidence, policy) <= 0 {
            continue;
        }

        candidates.insert(file.id.clone(), file);
    }

    candidates.into_values().collect()
}

fn refine_signal_match_for_leaf(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    leaf: &DomainSignalLeafSpec,
    candidate_files: &[&wiki_index::scanner::ScannedFile],
    limit: usize,
) -> LeafSignalMatch {
    let Some(policy) = leaf_scope_refinement_policy(leaf) else {
        return LeafSignalMatch {
            signal: trim_signal_match(matching_signal(candidate_files, leaf.keywords), limit),
            collapse_guard: None,
        };
    };

    let docs_terms = collect_leaf_docs_grounding_terms(domain, leaf, &policy);
    let candidate_files =
        expanded_leaf_signal_candidates(domain, report, candidate_files, &policy, &docs_terms);
    let mut all_keywords = BTreeSet::new();
    let mut entries = candidate_files
        .iter()
        .filter_map(|file| {
            let normalized_path = normalize_path(&file.path);
            let lower = normalized_path.to_ascii_lowercase();
            let matched_keywords = leaf
                .keywords
                .iter()
                .filter(|keyword| lower.contains(**keyword))
                .map(|keyword| (*keyword).to_string())
                .collect::<BTreeSet<_>>();
            let evidence = build_path_topic_evidence(&normalized_path, &docs_terms);
            let topic_score = score_topic_evidence(&evidence, &policy);
            if matched_keywords.is_empty() && topic_score <= 0 {
                return None;
            }
            all_keywords.extend(matched_keywords.iter().cloned());
            Some(ScoredSignalEntry {
                source_id: file.id.clone(),
                path: normalized_path,
                topic_score,
                classes: evidence.classes,
            })
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| {
        right
            .topic_score
            .cmp(&left.topic_score)
            .then(left.path.cmp(&right.path))
    });

    let strong_spine_count = entries
        .iter()
        .filter(|entry| {
            entry.topic_score >= 8
                && !entry
                    .classes
                    .contains(&GenericContractClass::ConfigManifestHeavy)
        })
        .count();
    let allow_generic_support = strong_spine_count < 2;

    let mut preferred_entries = Vec::new();
    let mut collapsed_entries = Vec::new();
    for entry in entries {
        let generic_only = entry.topic_score <= 0
            && entry.classes.iter().any(|class| {
                matches!(
                    class,
                    GenericContractClass::DeclarationHeavy
                        | GenericContractClass::ConfigManifestHeavy
                )
            });
        if !allow_generic_support && generic_only {
            collapsed_entries.push(entry);
        } else {
            preferred_entries.push(entry);
        }
    }

    if preferred_entries.is_empty() && !collapsed_entries.is_empty() {
        preferred_entries = collapsed_entries.clone();
        collapsed_entries.clear();
    }

    if limit != 0 {
        preferred_entries.truncate(limit);
    }

    let signal = SignalMatch {
        matched_keywords: all_keywords.into_iter().collect(),
        matched_source_ids: preferred_entries
            .iter()
            .map(|entry| entry.source_id.clone())
            .collect(),
        matched_paths: preferred_entries
            .iter()
            .map(|entry| entry.path.clone())
            .collect(),
    };
    let collapse_guard = (!collapsed_entries.is_empty()).then(|| CollapseGuardDecision {
        reason: CollapseGuardReason::TopicScopeRefinement,
        owner_title: leaf.title.to_string(),
        collapsed_source_ids: collapsed_entries
            .iter()
            .map(|entry| entry.source_id.clone())
            .collect(),
        collapsed_paths: collapsed_entries
            .iter()
            .map(|entry| entry.path.clone())
            .collect(),
        preserved_source_ids: preferred_entries
            .iter()
            .map(|entry| entry.source_id.clone())
            .collect(),
        preserved_paths: preferred_entries
            .iter()
            .map(|entry| entry.path.clone())
            .collect(),
    });

    LeafSignalMatch {
        signal,
        collapse_guard,
    }
}

fn make_planner_signal_bundle(
    kind: PlannerSignalKind,
    key: impl Into<String>,
    label: impl Into<String>,
    signal: &SignalMatch,
) -> PlannerSignalBundle {
    PlannerSignalBundle {
        kind,
        key: key.into(),
        label: label.into(),
        matched_keywords: signal.matched_keywords.clone(),
        matched_source_ids: signal.matched_source_ids.clone(),
        matched_paths: signal.matched_paths.clone(),
    }
}

fn append_planner_signal(unit: &mut KnowledgeUnit, bundle: PlannerSignalBundle) {
    if unit
        .planner_signal_bundles
        .iter()
        .any(|existing| existing.kind == bundle.kind && existing.key == bundle.key)
    {
        return;
    }
    unit.planner_signal_bundles.push(bundle);
}

fn append_planner_signal_to_units(units: &mut [KnowledgeUnit], bundle: PlannerSignalBundle) {
    for unit in units {
        append_planner_signal(unit, bundle.clone());
    }
}

fn surface_cluster_key(title: &str) -> String {
    format!(
        "surface_cluster/{}",
        sanitize_path_segment(title).to_ascii_lowercase()
    )
}

fn repo_signal_surface_cluster_key(capability_key: &str, title: &str) -> String {
    format!(
        "surface_cluster/{}/{}",
        sanitize_path_segment(capability_key).to_ascii_lowercase(),
        sanitize_path_segment(title).to_ascii_lowercase()
    )
}

fn leaf_decomposition_key(title: &str) -> String {
    format!(
        "leaf_decomposition/{}",
        sanitize_path_segment(title).to_ascii_lowercase()
    )
}

fn repo_signal_leaf_decomposition_key(capability_key: &str, title: &str) -> String {
    format!(
        "leaf_decomposition/{}/{}",
        sanitize_path_segment(capability_key).to_ascii_lowercase(),
        sanitize_path_segment(title).to_ascii_lowercase()
    )
}

fn repo_archetype_key(label: &str) -> String {
    format!(
        "repo_archetype/{}",
        sanitize_path_segment(label).to_ascii_lowercase()
    )
}

fn collect_repo_signal_candidates<'a>(
    domain: &KnowledgeDomain,
    report: &'a ScanReport,
) -> Vec<&'a wiki_index::scanner::ScannedFile> {
    let candidate_paths: BTreeSet<&str> = domain.source_files.iter().map(String::as_str).collect();
    report
        .files
        .iter()
        .filter(|file| candidate_paths.contains(file.path.as_str()))
        .collect()
}

fn plan_repo_signal_capabilities(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    capabilities: &[RepoSignalCapabilitySpec],
) -> Vec<KnowledgeUnit> {
    let candidate_files = collect_repo_signal_candidates(domain, report);
    let mut units = Vec::new();

    for capability in capabilities {
        units.extend(plan_repo_signal_capability_units(
            domain,
            capability,
            &candidate_files,
        ));
    }

    units
}

fn plan_repo_signal_capability_units(
    domain: &KnowledgeDomain,
    capability: &RepoSignalCapabilitySpec,
    candidate_files: &[&wiki_index::scanner::ScannedFile],
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();

    for materialization in &capability.materializations {
        match materialization {
            RepoSignalMaterializationSpec::Family(spec) => {
                units.extend(plan_repo_signal_family_units_from_candidates(
                    domain,
                    candidate_files,
                    capability.capability_key,
                    spec.root_prefix,
                    spec.family_title,
                    spec.family_unit_type.clone(),
                    spec.family_profile.clone(),
                    spec.family_priority,
                    spec.leaves,
                ))
            }
            RepoSignalMaterializationSpec::Standalone(spec) => {
                units.extend(plan_repo_signal_leaf_units_from_candidates(
                    domain,
                    candidate_files,
                    capability.capability_key,
                    spec.root_prefix,
                    spec.leaves,
                ))
            }
        }
    }

    units
}

fn plan_domain_signal_family_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
    family_title: &str,
    family_unit_type: UnitType,
    family_profile: DecompositionProfile,
    family_priority: f32,
    children: &[DomainSignalLeafSpec],
) -> Vec<KnowledgeUnit> {
    let candidate_files = collect_domain_signal_candidates(domain, report);
    let mut child_units = Vec::new();
    let mut family_signal = SignalMatch::default();
    let family_relative_path = format!("{domain_label}/{}.md", sanitize_path_segment(family_title));
    let family_id = stable_id(
        "unit",
        format!(
            "{}:{}:{}",
            family_unit_type.as_str(),
            &domain.id,
            family_title
        ),
    );

    for child in children {
        let matched_signal = refine_signal_match_for_leaf(
            domain,
            report,
            child,
            &candidate_files,
            signal_scope_limit(&child.profile, false),
        );
        if matched_signal.signal.source_count() < child.min_hits {
            continue;
        }
        family_signal.extend(&matched_signal.signal);
        let mut unit = KnowledgeUnit::new(
            child.unit_type.clone(),
            child.title,
            &domain.id,
            format!(
                "{domain_label}/{}/{}.md",
                sanitize_path_segment(family_title),
                sanitize_path_segment(child.title)
            ),
        );
        unit.decomposition_profile = Some(child.profile.clone());
        unit.parent_unit_id = Some(family_id.clone());
        unit.scope = UnitScope {
            source_ids: matched_signal.signal.matched_source_ids.clone(),
            ..Default::default()
        };
        unit.priority = child.priority;
        unit.collapse_guard = matched_signal.collapse_guard;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                surface_cluster_key(child.title),
                child.title,
                &matched_signal.signal,
            ),
        );
        child_units.push(unit);
    }

    if child_units.is_empty() {
        return Vec::new();
    }

    let mut family = KnowledgeUnit::new(
        family_unit_type,
        family_title,
        &domain.id,
        family_relative_path,
    );
    family_signal = trim_signal_match(family_signal, signal_scope_limit(&family_profile, true));
    family.decomposition_profile = Some(family_profile);
    family.scope = UnitScope {
        source_ids: family_signal.matched_source_ids.clone(),
        ..Default::default()
    };
    family.priority = family_priority;
    append_planner_signal(
        &mut family,
        make_planner_signal_bundle(
            PlannerSignalKind::LeafDecomposition,
            leaf_decomposition_key(family_title),
            family_title,
            &family_signal,
        ),
    );

    let mut units = vec![family];
    units.extend(child_units);
    units
}

fn plan_repo_signal_family_units_from_candidates(
    domain: &KnowledgeDomain,
    candidate_files: &[&wiki_index::scanner::ScannedFile],
    capability_key: &str,
    root_prefix: &str,
    family_title: &str,
    family_unit_type: UnitType,
    family_profile: DecompositionProfile,
    family_priority: f32,
    children: &[DomainSignalLeafSpec],
) -> Vec<KnowledgeUnit> {
    let mut child_units = Vec::new();
    let mut family_signal = SignalMatch::default();
    let family_relative_path =
        format!("{}/{}.md", root_prefix, sanitize_path_segment(family_title));
    let family_id = stable_id(
        "unit",
        format!(
            "{}:{}:{}",
            family_unit_type.as_str(),
            &domain.id,
            family_title
        ),
    );

    for child in children {
        let matched_signal = matching_signal(&candidate_files, child.keywords);
        if matched_signal.source_count() < child.min_hits {
            continue;
        }
        let matched_signal =
            trim_signal_match(matched_signal, signal_scope_limit(&child.profile, false));
        family_signal.extend(&matched_signal);
        let mut unit = KnowledgeUnit::new(
            child.unit_type.clone(),
            child.title,
            &domain.id,
            format!("{}/{}.md", root_prefix, sanitize_path_segment(child.title)),
        );
        unit.decomposition_profile = Some(child.profile.clone());
        unit.parent_unit_id = Some(family_id.clone());
        unit.scope = UnitScope {
            source_ids: matched_signal.matched_source_ids.clone(),
            ..Default::default()
        };
        unit.priority = child.priority;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                repo_signal_surface_cluster_key(capability_key, child.title),
                child.title,
                &matched_signal,
            ),
        );
        child_units.push(unit);
    }

    if child_units.is_empty() {
        return Vec::new();
    }

    let mut family = KnowledgeUnit::new(
        family_unit_type,
        family_title,
        &domain.id,
        family_relative_path,
    );
    family_signal = trim_signal_match(family_signal, signal_scope_limit(&family_profile, true));
    family.decomposition_profile = Some(family_profile);
    family.scope = UnitScope {
        source_ids: family_signal.matched_source_ids.clone(),
        ..Default::default()
    };
    family.priority = family_priority;
    append_planner_signal(
        &mut family,
        make_planner_signal_bundle(
            PlannerSignalKind::LeafDecomposition,
            repo_signal_leaf_decomposition_key(capability_key, family_title),
            family_title,
            &family_signal,
        ),
    );

    let mut units = vec![family];
    units.extend(child_units);
    units
}

fn plan_repo_signal_leaf_units_from_candidates(
    domain: &KnowledgeDomain,
    candidate_files: &[&wiki_index::scanner::ScannedFile],
    capability_key: &str,
    root_prefix: &str,
    leaves: &[DomainSignalLeafSpec],
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();

    for leaf in leaves {
        let matched_signal = matching_signal(&candidate_files, leaf.keywords);
        if matched_signal.source_count() < leaf.min_hits {
            continue;
        }
        let matched_signal =
            trim_signal_match(matched_signal, signal_scope_limit(&leaf.profile, false));
        let mut unit = KnowledgeUnit::new(
            leaf.unit_type.clone(),
            leaf.title,
            &domain.id,
            format!("{root_prefix}/{}.md", sanitize_path_segment(leaf.title)),
        );
        unit.decomposition_profile = Some(leaf.profile.clone());
        unit.scope = UnitScope {
            source_ids: matched_signal.matched_source_ids.clone(),
            ..Default::default()
        };
        unit.priority = leaf.priority;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                repo_signal_surface_cluster_key(capability_key, leaf.title),
                leaf.title,
                &matched_signal,
            ),
        );
        units.push(unit);
    }

    units
}

fn plan_domain_signal_leaf_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
    leaves: &[DomainSignalLeafSpec],
) -> Vec<KnowledgeUnit> {
    let candidate_files = collect_domain_signal_candidates(domain, report);
    let mut units = Vec::new();

    for leaf in leaves {
        let matched_signal = refine_signal_match_for_leaf(
            domain,
            report,
            leaf,
            &candidate_files,
            signal_scope_limit(&leaf.profile, false),
        );
        if matched_signal.signal.source_count() < leaf.min_hits {
            continue;
        }

        let mut unit = KnowledgeUnit::new(
            leaf.unit_type.clone(),
            leaf.title,
            &domain.id,
            format!("{domain_label}/{}.md", sanitize_path_segment(leaf.title)),
        );
        unit.decomposition_profile = Some(leaf.profile.clone());
        unit.scope = UnitScope {
            source_ids: matched_signal.signal.matched_source_ids.clone(),
            ..Default::default()
        };
        unit.priority = leaf.priority;
        unit.collapse_guard = matched_signal.collapse_guard;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                surface_cluster_key(leaf.title),
                leaf.title,
                &matched_signal.signal,
            ),
        );
        units.push(unit);
    }

    units
}

fn collect_domain_signal_candidates<'a>(
    domain: &KnowledgeDomain,
    report: &'a ScanReport,
) -> Vec<&'a wiki_index::scanner::ScannedFile> {
    let source_paths: BTreeSet<&str> = domain.source_files.iter().map(String::as_str).collect();
    report
        .files
        .iter()
        .filter(|file| source_paths.contains(file.path.as_str()))
        .filter(|file| match domain.domain_type {
            DomainType::ConfigReference => !file.is_asset_like(),
            DomainType::ApiReference => {
                !is_markdown_path(&file.path)
                    && !file.is_asset_like()
                    && (!file.is_config_like()
                        || looks_like_runtime_config_entry(&file.path)
                        || api_signal_candidate_accepts_config_path(&file.path))
                    && !file.is_docs_like()
            }
            _ => {
                !is_markdown_path(&file.path)
                    && !file.is_asset_like()
                    && !file.is_config_like()
                    && !file.is_docs_like()
            }
        })
        .collect()
}

fn api_signal_candidate_accepts_config_path(path: &str) -> bool {
    let lower = normalize_path(path).to_ascii_lowercase();
    lower.contains("preset")
        || lower.contains("builder")
        || lower.contains("build-config")
        || lower.contains("vite-config")
}

fn domain_matches_keywords(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    keywords: &[&str],
) -> Option<SignalMatch> {
    let label = domain.label.to_ascii_lowercase();
    let mut matched_keywords = keywords
        .iter()
        .filter(|keyword| label.contains(**keyword))
        .map(|keyword| (*keyword).to_string())
        .collect::<BTreeSet<_>>();
    let mut matched_paths = BTreeSet::new();

    for path in &domain.source_files {
        let lower = normalize_path(path).to_ascii_lowercase();
        for keyword in keywords {
            if lower.contains(keyword) {
                matched_keywords.insert((*keyword).to_string());
                matched_paths.insert(normalize_path(path));
            }
        }
    }

    if matched_keywords.is_empty() {
        return None;
    }

    if matched_paths.is_empty() {
        matched_paths.extend(domain.source_files.iter().map(|path| normalize_path(path)));
    }

    let matched_paths = matched_paths.into_iter().collect::<Vec<_>>();
    Some(SignalMatch {
        matched_keywords: matched_keywords.into_iter().collect(),
        matched_source_ids: source_ids_for_paths(report, &matched_paths),
        matched_paths,
    })
}

fn domain_has_annotation_di_signal(
    domain: &KnowledgeDomain,
    report: &ScanReport,
) -> Option<SignalMatch> {
    let signals = [
        ".java",
        ".kt",
        "/java/",
        "/kotlin/",
        "dagger",
        "hilt",
        "membersinjector",
        "bindsinstance",
        "multibinds",
        "intoset",
        "intomap",
        "productioncomponent",
        "processingstep",
        "bindinggraph",
        "providesmethodvalidator",
        "bindsmethodvalidator",
    ];
    domain_matches_keywords(domain, report, &signals)
}

fn module_has_explicit_api_signal(module: &ModuleNode) -> bool {
    let module_name = module.name.to_ascii_lowercase();
    if ["api", "preview", "store", "types", "csf"]
        .iter()
        .any(|keyword| module_name.contains(keyword))
    {
        return true;
    }

    module.root_paths.iter().any(|path| {
        let lower = normalize_path(path).to_ascii_lowercase();
        ["api", "preview", "store", "types", "csf"]
            .iter()
            .any(|keyword| lower.contains(keyword))
    })
}

fn module_explicit_api_symbols(module: &ModuleNode, report: &ScanReport) -> Vec<String> {
    report
        .files
        .iter()
        .filter(|file| module.source_ids.contains(&file.id))
        .filter(|file| {
            let lower = file.path.to_ascii_lowercase();
            lower.contains("/api/")
                || lower.starts_with("api/")
                || lower.contains("publicapi")
                || lower.ends_with("public-types.ts")
                || lower.ends_with("public-types.tsx")
                || looks_like_api_surface_path(&file.path)
        })
        .map(|file| {
            file.path
                .rsplit('/')
                .next()
                .unwrap_or(file.path.as_str())
                .to_string()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn matching_signal(files: &[&wiki_index::scanner::ScannedFile], keywords: &[&str]) -> SignalMatch {
    let mut matched_keywords = BTreeSet::new();
    let mut ranked_matches = Vec::new();

    for file in files {
        let normalized_path = normalize_path(&file.path);
        let lower = normalized_path.to_ascii_lowercase();
        let file_keywords = keywords
            .iter()
            .filter(|keyword| lower.contains(**keyword))
            .map(|keyword| (*keyword).to_string())
            .collect::<BTreeSet<_>>();
        if file_keywords.is_empty() {
            continue;
        }

        matched_keywords.extend(file_keywords.iter().cloned());
        ranked_matches.push((
            signal_match_score(&lower, &file_keywords),
            signal_match_noise_penalty(&lower),
            normalized_path,
            file.id.clone(),
        ));
    }

    ranked_matches.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut matched_source_ids = Vec::new();
    let mut matched_paths = Vec::new();
    for (_, _, path, source_id) in ranked_matches {
        if !matched_source_ids
            .iter()
            .any(|existing| existing == &source_id)
        {
            matched_source_ids.push(source_id);
        }
        if !matched_paths.iter().any(|existing| existing == &path) {
            matched_paths.push(path);
        }
    }

    SignalMatch {
        matched_keywords: matched_keywords.into_iter().collect(),
        matched_source_ids,
        matched_paths,
    }
}

fn trim_signal_match(signal: SignalMatch, limit: usize) -> SignalMatch {
    let mut clean_entries = Vec::new();
    let mut noisy_entries = Vec::new();
    for (source_id, path) in signal
        .matched_source_ids
        .into_iter()
        .zip(signal.matched_paths.into_iter())
    {
        if signal_path_is_secondary_noise(&path) {
            noisy_entries.push((source_id, path));
        } else {
            clean_entries.push((source_id, path));
        }
    }

    let mut preferred_entries = if clean_entries.len() >= 2 {
        clean_entries
    } else {
        let mut merged = clean_entries;
        merged.extend(noisy_entries);
        merged
    };

    if limit != 0 {
        preferred_entries.truncate(limit);
    }

    let (matched_source_ids, matched_paths): (Vec<_>, Vec<_>) =
        preferred_entries.into_iter().unzip();

    SignalMatch {
        matched_keywords: signal.matched_keywords,
        matched_source_ids,
        matched_paths,
    }
}

fn signal_path_is_secondary_noise(path: &str) -> bool {
    signal_match_noise_penalty(path) >= 4
}

fn signal_scope_limit(profile: &DecompositionProfile, is_family: bool) -> usize {
    match (profile, is_family) {
        (DecompositionProfile::ConfigSurface, true) => 8,
        (DecompositionProfile::ConfigSurface, false) => 6,
        (DecompositionProfile::ApiSurface, true) => 10,
        (DecompositionProfile::CompilerPipeline, true)
        | (DecompositionProfile::IntegrationPlatform, true)
        | (DecompositionProfile::Runtime, true) => 12,
        (DecompositionProfile::CompilerPipeline, false)
        | (DecompositionProfile::IntegrationPlatform, false)
        | (DecompositionProfile::Runtime, false) => 8,
        (_, true) => 10,
        (_, false) => 8,
    }
}

fn signal_match_score(path: &str, matched_keywords: &BTreeSet<String>) -> usize {
    let file_name = path.rsplit('/').next().unwrap_or(path);
    let stem = file_name.split('.').next().unwrap_or(file_name);
    let keyword_score = matched_keywords
        .iter()
        .map(|keyword| {
            let mut score = 4usize;
            if file_name.contains(keyword) {
                score += 2;
            }
            if stem.eq_ignore_ascii_case(keyword) {
                score += 2;
            }
            if path
                .split('/')
                .any(|segment| segment.eq_ignore_ascii_case(keyword))
            {
                score += 1;
            }
            score
        })
        .sum::<usize>();

    keyword_score + signal_match_implementation_bonus(path)
}

fn signal_match_implementation_bonus(path: &str) -> usize {
    let mut bonus = 0usize;
    if path.contains("/src/") || path.contains("/lib/") {
        bonus += 3;
    }
    if path.contains("/main/java/") || path.contains("/main/kotlin/") {
        bonus += 4;
    }
    if path.contains("/internal/") || path.contains("/modules/") {
        bonus += 1;
    }
    bonus
}

fn signal_match_noise_penalty(path: &str) -> usize {
    let mut penalty = 0usize;
    if path.contains(".test.")
        || path.contains(".spec.")
        || path.contains("/test/")
        || path.contains("/tests/")
        || path.contains("/__tests__/")
        || path.contains("/javatests/")
    {
        penalty += 4;
    }
    if path.contains(".stories.") || path.contains("/stories/") {
        penalty += 3;
    }
    if path.contains("/template/") || path.contains("/templates/") {
        penalty += 3;
    }
    if path.contains("/__mocks__/") || path.contains("/fixtures/") {
        penalty += 4;
    }
    if path.contains("/examples/")
        || path.contains("/example/")
        || path.contains("/test-storybooks/")
        || path.contains("/golden/")
        || path.contains("/snapshots/")
    {
        penalty += 4;
    }
    if path.ends_with("/build.gradle")
        || path.ends_with("/build.gradle.kts")
        || path.ends_with("/package.json")
        || path.ends_with("/project.json")
        || path.ends_with("/pom.xml")
        || path.ends_with("/androidmanifest.xml")
        || path.ends_with("/package-info.java")
        || path.ends_with("/build.bazel")
        || path.ends_with("/build.bzl")
        || path.ends_with("/buildsrc/build.gradle.kts")
        || path.ends_with("/settings.gradle.kts")
    {
        penalty += 5;
    }
    if path.contains("/buildsrc/") || path.contains("/build/") {
        penalty += 4;
    }
    if path.ends_with(".patch") || path.ends_with(".diff") {
        penalty += 6;
    }
    if path.ends_with(".xml") || path.ends_with(".json") || path.ends_with(".gradle.kts") {
        penalty += 2;
    }
    penalty
}

fn signal_match_for_paths(
    report: &ScanReport,
    paths: &[String],
    keywords: Vec<String>,
) -> SignalMatch {
    SignalMatch {
        matched_keywords: keywords
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        matched_source_ids: source_ids_for_paths(report, paths),
        matched_paths: paths
            .iter()
            .map(|path| normalize_path(path))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
    }
}

fn signal_keywords_for_path(path: &str) -> Vec<String> {
    normalize_path(path)
        .split('/')
        .flat_map(lexical_topic_tokens)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn plan_config_doc_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    if domain.source_files.is_empty() {
        return Vec::new();
    }

    let mut units = Vec::new();
    let has_docs_backed_config = domain.source_files.iter().any(|path| {
        is_markdown_path(path)
            && matches!(
                classify_docs_unit_profile(path).1,
                DecompositionProfile::ConfigSurface
            )
    });
    let raw_config_surface_count = domain.evidence.config_surfaces.len();
    let raw_config_file_count = domain
        .source_files
        .iter()
        .filter(|path| !is_markdown_path(path))
        .count();
    let standalone_surface_count = domain
        .evidence
        .config_surfaces
        .iter()
        .filter(|surface| config_surface_keeps_standalone_unit(&surface.file_path))
        .count();
    let preserved_candidate_paths = domain
        .evidence
        .config_surfaces
        .iter()
        .filter(|surface| config_surface_keeps_standalone_unit(&surface.file_path))
        .map(|surface| normalize_path(&surface.file_path))
        .chain(domain.source_files.iter().filter_map(|path| {
            (is_markdown_path(path)
                && matches!(
                    classify_docs_unit_profile(path).1,
                    DecompositionProfile::ConfigSurface
                ))
            .then(|| normalize_path(path))
        }))
        .collect::<BTreeSet<_>>();
    let collapsed_candidate_paths = domain
        .source_files
        .iter()
        .filter(|path| !is_markdown_path(path))
        .map(|path| normalize_path(path))
        .filter(|path| !preserved_candidate_paths.contains(path))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let preserved_candidate_paths = preserved_candidate_paths.into_iter().collect::<Vec<_>>();
    let collapse_raw_config_surfaces = !has_docs_backed_config
        && (raw_config_surface_count >= 5
            || raw_config_file_count >= 5
            || (raw_config_surface_count >= 2 && standalone_surface_count == 0));

    for surface in &domain.evidence.config_surfaces {
        if has_docs_backed_config && !config_surface_keeps_standalone_unit(&surface.file_path) {
            continue;
        }
        if config_surface_is_generic_manifest(&surface.file_path)
            && !config_surface_keeps_standalone_unit(&surface.file_path)
        {
            continue;
        }
        if collapse_raw_config_surfaces && !config_surface_keeps_standalone_unit(&surface.file_path)
        {
            continue;
        }
        let title = config_unit_title(&surface.file_path);
        let safe_name = sanitize_path_segment(&title);
        let mut unit = KnowledgeUnit::new(
            UnitType::ConfigDoc,
            &title,
            &domain.id,
            format!("{domain_label}/{safe_name}.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::ConfigSurface);
        unit.scope = UnitScope {
            source_ids: source_ids_for_path(report, &surface.file_path),
            config_surfaces: vec![surface.clone()],
            ..Default::default()
        };
        unit.priority = 0.6;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                surface_cluster_key(&title),
                &title,
                &signal_match_for_paths(
                    report,
                    &[normalize_path(&surface.file_path)],
                    surface
                        .keys
                        .iter()
                        .cloned()
                        .chain(signal_keywords_for_path(&surface.file_path))
                        .collect(),
                ),
            ),
        );
        units.push(unit);
    }

    units.extend(plan_docs_backed_units_for_domain(
        domain,
        report,
        domain_label,
        |profile| matches!(profile, DecompositionProfile::ConfigSurface),
    ));
    units.extend(plan_domain_signal_family_units(
        domain,
        report,
        domain_label,
        "主题和外观",
        UnitType::ConfigDoc,
        DecompositionProfile::ConfigSurface,
        0.72,
        &[
            DomainSignalLeafSpec {
                title: "主题系统概览",
                unit_type: UnitType::ConfigDoc,
                profile: DecompositionProfile::ConfigSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &[
                    "theme",
                    "themes",
                    "theming",
                    "theme-switcher",
                    "getpreferredcolorscheme",
                ],
            },
            DomainSignalLeafSpec {
                title: "自定义主题开发",
                unit_type: UnitType::ConfigDoc,
                profile: DecompositionProfile::ConfigSurface,
                min_hits: 1,
                priority: 0.68,
                keywords: &[
                    "theme",
                    "themes",
                    "theming",
                    "themeprovider",
                    "provider.decorator",
                    "theme-switcher",
                    "create",
                    "convert",
                ],
            },
            DomainSignalLeafSpec {
                title: "颜色和字体系统",
                unit_type: UnitType::ConfigDoc,
                profile: DecompositionProfile::ConfigSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["color", "font", "palette", "css", "style", "theming"],
            },
        ],
    ));

    if units.is_empty() || collapse_raw_config_surfaces {
        units.retain(|unit| {
            unit.scope.docs_anchors.first().is_some()
                || (unit.scope.config_surfaces.is_empty() && !unit.scope.source_ids.is_empty())
                || unit
                    .scope
                    .config_surfaces
                    .iter()
                    .any(|surface| config_surface_keeps_standalone_unit(&surface.file_path))
        });
    }

    if units.is_empty() {
        let mut unit = KnowledgeUnit::new(
            UnitType::ConfigDoc,
            "配置参考",
            &domain.id,
            format!("{domain_label}/配置参考.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::ConfigSurface);
        unit.scope = UnitScope {
            source_ids: domain
                .source_files
                .iter()
                .filter_map(|path| {
                    report
                        .files
                        .iter()
                        .find(|f| f.path == *path)
                        .map(|f| f.id.clone())
                })
                .collect(),
            config_surfaces: domain.evidence.config_surfaces.clone(),
            ..Default::default()
        };
        unit.priority = 0.5;
        if collapse_raw_config_surfaces {
            unit.collapse_guard = Some(CollapseGuardDecision {
                reason: CollapseGuardReason::RawConfigSurfaceAggregation,
                owner_title: unit.title.clone(),
                collapsed_source_ids: source_ids_for_paths(report, &collapsed_candidate_paths),
                collapsed_paths: collapsed_candidate_paths.clone(),
                preserved_source_ids: source_ids_for_paths(report, &preserved_candidate_paths),
                preserved_paths: preserved_candidate_paths.clone(),
            });
        }
        units.push(unit);
    }

    units
}

fn plan_troubleshooting_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = plan_docs_backed_units_for_domain(domain, report, domain_label, |profile| {
        matches!(profile, DecompositionProfile::Troubleshooting)
    });

    if units.is_empty() && !domain.source_files.is_empty() {
        let mut unit = KnowledgeUnit::new(
            UnitType::TroubleshootDoc,
            "故障排除",
            &domain.id,
            format!("{domain_label}/故障排除.md"),
        );
        unit.decomposition_profile = Some(DecompositionProfile::Troubleshooting);
        unit.scope = UnitScope {
            source_ids: source_ids_for_paths(report, &domain.source_files),
            docs_anchors: build_docs_anchors(&domain.source_files),
            ..Default::default()
        };
        unit.priority = 0.6;
        units.push(unit);
    }

    units
}

fn module_domain_profile(domain_type: &DomainType) -> DecompositionProfile {
    match domain_type {
        DomainType::CoreRuntime | DomainType::BuildSystem | DomainType::ThemeSystem => {
            DecompositionProfile::Runtime
        }
        DomainType::CompilerToolchain => DecompositionProfile::CompilerPipeline,
        DomainType::Framework
        | DomainType::PlatformBinding
        | DomainType::PluginEcosystem
        | DomainType::MultiFramework
        | DomainType::DevTooling => DecompositionProfile::IntegrationPlatform,
        DomainType::ApiReference => DecompositionProfile::ApiSurface,
        DomainType::ConfigReference => DecompositionProfile::ConfigSurface,
        DomainType::TestingInfra => DecompositionProfile::Testing,
        DomainType::Troubleshooting => DecompositionProfile::Troubleshooting,
        DomainType::ConceptGuide => DecompositionProfile::DocsGuide,
    }
}

fn classify_docs_unit_profile(path: &str) -> (UnitType, DecompositionProfile) {
    let lower = normalize_path(path).to_ascii_lowercase();
    if lower.contains("/api/") || lower.contains("/api参考") {
        return (UnitType::ApiDoc, DecompositionProfile::ApiSurface);
    }
    if docs_path_has_config_signal(path) {
        return (UnitType::ConfigDoc, DecompositionProfile::ConfigSurface);
    }
    if docs_path_has_compiler_signal(path) {
        return (UnitType::ModuleDoc, DecompositionProfile::CompilerPipeline);
    }
    if lower.contains("/tutorial")
        || lower.contains("/tutorials/")
        || lower.contains("/get-started/")
        || lower.contains("/快速开始")
        || lower.contains("/教程")
        || lower.contains("/示例")
        || lower.contains("/quick-start")
        || lower.contains("/quickstart")
        || lower.contains("/example")
        || lower.contains("/examples")
        || lower.contains("/sample")
    {
        return (UnitType::ExampleDoc, DecompositionProfile::ExampleTutorial);
    }
    if lower.contains("/troubleshoot")
        || lower.contains("/faq")
        || lower.contains("/error")
        || lower.contains("/debug")
        || lower.contains("/pitfall")
        || lower.contains("/故障")
        || lower.contains("/排查")
    {
        return (
            UnitType::TroubleshootDoc,
            DecompositionProfile::Troubleshooting,
        );
    }
    if lower.contains("/workflow")
        || lower.contains("/process")
        || lower.contains("/deploy")
        || lower.contains("/release")
        || lower.contains("/pipeline")
        || lower.contains("/工作流")
        || lower.contains("/发布")
    {
        return (UnitType::WorkflowDoc, DecompositionProfile::DocsGuide);
    }
    if lower.contains("/android")
        || lower.contains("/ios")
        || lower.contains("/web")
        || lower.contains("/integration")
        || lower.contains("/framework")
        || lower.contains("/hilt")
        || lower.contains("/框架")
        || lower.contains("/集成")
    {
        return (
            UnitType::IntegrationDoc,
            DecompositionProfile::IntegrationPlatform,
        );
    }

    (UnitType::ConceptGuide, DecompositionProfile::DocsGuide)
}

fn docs_path_has_config_signal(path: &str) -> bool {
    let normalized = normalize_path(path);
    let lower = normalized.to_ascii_lowercase();
    if lower.contains("/config")
        || lower.contains("/configure")
        || lower.contains("/配置")
        || lower.contains("/配置参考")
        || lower.contains("/settings")
    {
        return true;
    }

    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());
    matches!(
        markdown_stem(file_name),
        "main"
            | "main-js"
            | "main-ts"
            | "preview"
            | "preview-js"
            | "preview-ts"
            | "manager"
            | "manager-js"
            | "manager-ts"
    )
}

fn config_surface_keeps_standalone_unit(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    let file_name = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    matches!(
        file_name
            .trim_end_matches(".tsx")
            .trim_end_matches(".jsx")
            .trim_end_matches(".ts")
            .trim_end_matches(".js")
            .trim_end_matches(".json")
            .trim_end_matches(".yaml")
            .trim_end_matches(".yml")
            .trim_end_matches(".toml"),
        "main"
            | "main-js"
            | "main-ts"
            | "preview"
            | "preview-js"
            | "preview-ts"
            | "manager"
            | "manager-js"
            | "manager-ts"
    )
}

fn config_surface_is_generic_manifest(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    let file_name = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    matches!(
        file_name,
        "package.json"
            | "cargo.toml"
            | "pyproject.toml"
            | "pom.xml"
            | "composer.json"
            | "build.gradle"
            | "build.gradle.kts"
            | "settings.gradle"
            | "settings.gradle.kts"
            | "gradle.properties"
            | "module.bazel"
            | "workspace"
    )
}

fn api_docs_keeps_standalone_unit(path: &str) -> bool {
    let normalized = normalize_path(path);
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let Some(api_index) = segments
        .iter()
        .position(|segment| segment.eq_ignore_ascii_case("api"))
    else {
        return true;
    };
    let tail = &segments[api_index + 1..];
    if tail.len() <= 1 {
        return true;
    }
    if tail.len() > 2 {
        return false;
    }

    let parent = tail[0];
    let file_name = tail[1];
    markdown_stem(file_name).eq_ignore_ascii_case(parent)
}

fn docs_path_is_meta_reference_noise(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let file_name = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    let stem = markdown_stem(file_name).to_ascii_lowercase();
    if [
        "changelog",
        "changelog-prerelease",
        "changelog-v6",
        "changelog-v1-5",
        "contributing-old",
        "code-of-conduct",
        "code_of_conduct",
        "maintainers",
        "security",
        "resolutions",
    ]
    .iter()
    .any(|candidate| stem == *candidate || stem.starts_with(&format!("{candidate}-")))
    {
        return true;
    }

    segments.iter().any(|segment| {
        matches!(
            *segment,
            "contribute"
                | "contributing"
                | "contributor"
                | "release"
                | "releases"
                | "roadmap"
                | "upgrade"
                | "upgrading"
                | "migration"
                | "migrations"
                | "changelog"
                | "security"
                | "maintainers"
        )
    })
}

fn docs_path_is_repo_root_readme(path: &str) -> bool {
    let normalized = normalize_path(path);
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    matches!(segments.as_slice(), [file_name] if is_index_like_stem(markdown_stem(file_name)))
}

fn docs_path_has_compiler_signal(path: &str) -> bool {
    let lower = normalize_path(path).to_ascii_lowercase();
    [
        "/compiler/",
        "/codegen",
        "/annotation",
        "/processor",
        "/processing",
        "/validation",
        "/spi",
        "/编译",
        "/代码生成",
        "/注解处理",
        "/验证",
        "/错误处理",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn docs_unit_title(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let segments: Vec<&str> = normalized.split('/').collect();
    let file_name = segments.last().copied().unwrap_or(path);
    let stem = markdown_stem(file_name);

    let raw = if is_index_like_stem(stem) {
        segments.iter().rev().nth(1).copied().unwrap_or(stem)
    } else {
        stem
    };

    prettify_segment(raw)
}

fn planned_docs_unit_title(path: &str, unit_type: &UnitType, report: &ScanReport) -> String {
    let lower = normalize_path(path).to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());
    let stem = markdown_stem(file_name);

    if matches!(unit_type, UnitType::TroubleshootDoc)
        && (stem.eq_ignore_ascii_case("faq") || is_index_like_stem(stem))
    {
        return "故障排除".to_string();
    }

    if lower.contains("/in-ci")
        || lower.ends_with("/in-ci.mdx")
        || lower.ends_with("/in-ci.md")
        || lower.contains("/ci/")
    {
        return "CI_CD集成".to_string();
    }

    if repo_has_source_keyword(report, "chromatic")
        && (lower.contains("publish-storybook") || lower.contains("/visual-testing"))
    {
        return "Chromatic集成".to_string();
    }

    if let Some(target) = docs_integration_target_title(path) {
        return format!("{target}集成");
    }

    docs_unit_title(path)
}

fn docs_integration_target_title(path: &str) -> Option<String> {
    let normalized = normalize_path(path);
    let lower = normalized.to_ascii_lowercase();
    let segments: Vec<&str> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    let file_name = segments.last().copied()?;
    let stem = markdown_stem(file_name);
    let raw = if is_index_like_stem(stem) {
        segments
            .iter()
            .rev()
            .nth(1)
            .copied()
            .unwrap_or(stem)
            .to_string()
    } else {
        stem.to_string()
    };
    let raw_lower = raw.to_ascii_lowercase();
    let has_integration_context =
        lower.contains("/integrations/") || lower.contains("/integration/");
    let has_suffix_signal = raw_lower.ends_with("-addon")
        || raw_lower.ends_with("_addon")
        || raw_lower.ends_with(".addon")
        || raw_lower.ends_with("-plugin")
        || raw_lower.ends_with("_plugin")
        || raw_lower.ends_with(".plugin");
    if !has_integration_context && !has_suffix_signal {
        return None;
    }

    let tokens = raw
        .split(['-', '_', '.'])
        .filter(|segment| !segment.is_empty())
        .filter(|segment| {
            !matches!(
                segment.to_ascii_lowercase().as_str(),
                "addon" | "plugin" | "integration" | "integrations"
            )
        })
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return None;
    }

    Some(display_title_from_tokens(&tokens))
}

fn display_title_from_tokens(tokens: &[&str]) -> String {
    tokens
        .iter()
        .map(|token| match token.to_ascii_lowercase().as_str() {
            "api" => "API".to_string(),
            "ci" => "CI".to_string(),
            "cd" => "CD".to_string(),
            "cli" => "CLI".to_string(),
            "eslint" => "ESLint".to_string(),
            "mdx" => "MDX".to_string(),
            "vitest" => "Vitest".to_string(),
            other => prettify_segment(other),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn repo_has_source_keyword(report: &ScanReport, keyword: &str) -> bool {
    report.files.iter().any(|file| {
        normalize_path(&file.path)
            .to_ascii_lowercase()
            .contains(&keyword.to_ascii_lowercase())
    })
}

fn module_unit_title(domain: &KnowledgeDomain, module: &ModuleNode) -> String {
    if matches!(domain.domain_type, DomainType::PluginEcosystem) {
        if let Some(addon_name) = addon_module_name(module) {
            return format!(
                "{} Addon",
                display_title_from_tokens(&[addon_name.as_str()])
            );
        }
    }

    module.name.clone()
}

fn addon_module_name(module: &ModuleNode) -> Option<String> {
    module.root_paths.iter().find_map(|path| {
        let normalized = normalize_path(path);
        let segments = normalized
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        let addon_index = segments.iter().position(|segment| {
            matches!(segment.to_ascii_lowercase().as_str(), "addon" | "addons")
        })?;
        segments
            .get(addon_index + 1)
            .map(|segment| segment.to_string())
    })
}

fn config_unit_title(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let file_name = normalized.rsplit('/').next().unwrap_or(path);
    let stem = file_name
        .trim_end_matches(".ts")
        .trim_end_matches(".js")
        .trim_end_matches(".json")
        .trim_end_matches(".yaml")
        .trim_end_matches(".yml")
        .trim_end_matches(".toml");
    prettify_segment(stem)
}

fn prettify_segment(segment: &str) -> String {
    segment
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn establish_parent_child_links(units: &mut Vec<KnowledgeUnit>) {
    let id_to_index: BTreeMap<String, usize> = units
        .iter()
        .enumerate()
        .map(|(i, u)| (u.id.clone(), i))
        .collect();

    let parent_child_pairs: Vec<(String, String)> = units
        .iter()
        .filter_map(|u| {
            u.parent_unit_id
                .as_ref()
                .map(|pid| (pid.clone(), u.id.clone()))
        })
        .collect();

    for (parent_id, child_id) in parent_child_pairs {
        if let Some(&parent_idx) = id_to_index.get(&parent_id) {
            if !units[parent_idx].child_unit_ids.contains(&child_id) {
                units[parent_idx].child_unit_ids.push(child_id);
            }
        }
    }
}

fn filter_docs_for_domain(docs_files: &[String], domain_type: DomainType) -> Vec<String> {
    docs_files
        .iter()
        .filter(|path| match domain_type {
            DomainType::ConceptGuide => !matches!(
                docs_domain_override(path),
                Some(DomainType::ApiReference)
                    | Some(DomainType::ConfigReference)
                    | Some(DomainType::Troubleshooting)
            ),
            _ => docs_domain_override(path).as_ref() == Some(&domain_type),
        })
        .cloned()
        .collect()
}

fn docs_domain_override(path: &str) -> Option<DomainType> {
    match classify_docs_unit_profile(path).1 {
        DecompositionProfile::ApiSurface => Some(DomainType::ApiReference),
        DecompositionProfile::ConfigSurface => Some(DomainType::ConfigReference),
        DecompositionProfile::CompilerPipeline => Some(DomainType::CompilerToolchain),
        DecompositionProfile::Troubleshooting => Some(DomainType::Troubleshooting),
        _ => None,
    }
}

fn plan_docs_backed_units_for_domain<F>(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
    include_profile: F,
) -> Vec<KnowledgeUnit>
where
    F: Fn(&DecompositionProfile) -> bool,
{
    let mut units = Vec::new();
    let preferred_paths = select_preferred_docs_paths(
        &domain
            .source_files
            .iter()
            .filter(|path| is_markdown_path(path))
            .cloned()
            .collect::<Vec<_>>(),
    );
    let dense_docs_corpus = preferred_paths.len() >= 4;

    for path in preferred_paths.iter() {
        let (unit_type, profile) = classify_docs_unit_profile(path);
        if !include_profile(&profile) {
            continue;
        }
        if matches!(profile, DecompositionProfile::ApiSurface)
            && !api_docs_keeps_standalone_unit(path)
        {
            continue;
        }
        if matches!(profile, DecompositionProfile::DocsGuide)
            && docs_path_is_meta_reference_noise(path)
        {
            continue;
        }
        if matches!(profile, DecompositionProfile::DocsGuide)
            && dense_docs_corpus
            && docs_path_is_repo_root_readme(path)
        {
            continue;
        }

        let title = planned_docs_unit_title(path, &unit_type, report);
        let mut unit = KnowledgeUnit::new(
            unit_type,
            &title,
            &domain.id,
            docs_output_relative_path(domain_label, path, &title),
        );
        unit.decomposition_profile = Some(profile.clone());
        unit.scope = UnitScope {
            source_ids: source_ids_for_path(report, path),
            docs_anchors: vec![DocsAnchor {
                file_path: normalize_path(path),
                heading: title.clone(),
                level: 1,
                links: Vec::new(),
            }],
            ..Default::default()
        };
        unit.priority = 0.6;
        append_planner_signal(
            &mut unit,
            make_planner_signal_bundle(
                PlannerSignalKind::SurfaceCluster,
                format!("docs_anchor/{}", profile.as_str()),
                format!("Docs anchor {}", title),
                &signal_match_for_paths(
                    report,
                    &[normalize_path(path)],
                    signal_keywords_for_path(path),
                ),
            ),
        );
        units.push(unit);
    }

    assign_docs_unit_parents(&mut units);
    let original_parent_by_id = snapshot_parent_links(&units);
    let mut units = dedup_units_by_relative_path(units);
    repair_orphan_parent_links(&mut units, &original_parent_by_id, &BTreeMap::new());
    units
}

fn select_preferred_docs_paths(paths: &[String]) -> Vec<String> {
    if paths.len() < 2 {
        return paths.to_vec();
    }

    #[derive(Default)]
    struct DocsCorpusStats {
        paths: Vec<String>,
        localized_paths: usize,
    }

    let mut corpora: BTreeMap<String, DocsCorpusStats> = BTreeMap::new();
    for path in paths {
        let key = docs_corpus_key(path);
        let stats = corpora.entry(key).or_default();
        stats.paths.push(path.clone());
        if docs_path_is_localized(path) {
            stats.localized_paths += 1;
        }
    }

    if corpora.len() < 2 {
        return paths.to_vec();
    }

    // 当仓库里同时存在 raw docs 与本地化/派生 docs 时，优先保留后者，
    // 避免最终 `.wiki` 同时生成两套命名体系。
    let preferred = corpora
        .iter()
        .max_by_key(|(_, stats)| {
            (
                stats.localized_paths > 0,
                stats.localized_paths,
                stats.paths.len(),
            )
        })
        .map(|(key, _)| key.clone());

    let Some(preferred_key) = preferred else {
        return paths.to_vec();
    };
    let Some(preferred_stats) = corpora.get(&preferred_key) else {
        return paths.to_vec();
    };

    if preferred_stats.localized_paths == 0 {
        return paths.to_vec();
    }

    preferred_stats.paths.clone()
}

fn assign_docs_unit_parents(units: &mut [KnowledgeUnit]) {
    let source_to_id: BTreeMap<String, String> = units
        .iter()
        .filter_map(|unit| docs_source_path(unit).map(|path| (path.to_string(), unit.id.clone())))
        .collect();

    for unit in units.iter_mut() {
        let Some(source_path) = docs_source_path(unit) else {
            continue;
        };
        if let Some(parent_source) = find_docs_parent_source(source_path, &source_to_id) {
            unit.parent_unit_id = source_to_id.get(&parent_source).cloned();
        }
    }
}

fn find_docs_parent_source(
    source_path: &str,
    source_to_id: &BTreeMap<String, String>,
) -> Option<String> {
    let normalized = normalize_path(source_path);
    let segments: Vec<&str> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    let file_name = segments.last().copied()?;
    let stem = markdown_stem(file_name);
    let directory_segments = &segments[..segments.len().saturating_sub(1)];

    if !is_index_like_stem(stem) {
        if let Some(candidate) =
            first_docs_index_candidate(directory_segments, source_to_id, &normalized)
        {
            return Some(candidate);
        }
    }

    for ancestor_len in (0..directory_segments.len()).rev() {
        if let Some(candidate) = first_docs_index_candidate(
            &directory_segments[..ancestor_len],
            source_to_id,
            &normalized,
        ) {
            return Some(candidate);
        }
    }

    None
}

fn first_docs_index_candidate(
    directory_segments: &[&str],
    source_to_id: &BTreeMap<String, String>,
    excluded_path: &str,
) -> Option<String> {
    for suffix in [
        "index.md",
        "index.mdx",
        "README.md",
        "README.mdx",
        "readme.md",
        "readme.mdx",
    ] {
        let candidate = if directory_segments.is_empty() {
            suffix.to_string()
        } else {
            format!("{}/{}", directory_segments.join("/"), suffix)
        };
        if candidate != excluded_path && source_to_id.contains_key(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn docs_source_path(unit: &KnowledgeUnit) -> Option<&str> {
    unit.scope
        .docs_anchors
        .first()
        .map(|anchor| anchor.file_path.as_str())
}

fn unit_looks_like_raw_source_page(unit: &KnowledgeUnit) -> bool {
    let normalized = normalize_path(&unit.relative_path);
    let file_name = normalized
        .rsplit('/')
        .next()
        .unwrap_or(unit.relative_path.as_str());
    let stem = markdown_stem(file_name);
    !stem.is_empty()
        && stem
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ' '))
}

fn docs_corpus_key(path: &str) -> String {
    let normalized = normalize_path(path);
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if let Some(index) = segments.iter().position(|segment| {
        matches!(
            segment.to_ascii_lowercase().as_str(),
            "docs" | "doc" | "documentation" | "guide" | "guides" | "wiki" | "content"
        )
    }) {
        return segments[..=index].join("/");
    }

    segments.first().copied().unwrap_or(path).to_string()
}

fn docs_path_has_hidden_prefix(path: &str) -> bool {
    let normalized = normalize_path(path);
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let Some(index) = segments.iter().position(|segment| {
        matches!(
            segment.to_ascii_lowercase().as_str(),
            "docs" | "doc" | "documentation" | "guide" | "guides" | "wiki" | "content"
        )
    }) else {
        return false;
    };

    segments[..index]
        .iter()
        .any(|segment| segment.starts_with('.'))
}

fn docs_path_is_localized(path: &str) -> bool {
    normalize_path(path)
        .chars()
        .any(|character| !character.is_ascii())
}

fn docs_output_relative_path(domain_label: &str, source_path: &str, title: &str) -> String {
    let segments = strip_docs_root_segments(source_path);
    let safe_title = format!("{}.md", sanitize_path_segment(title));
    let output_segments = if segments.is_empty() {
        vec![safe_title]
    } else {
        let file_name = segments.last().expect("segments should not be empty");
        let stem = markdown_stem(file_name);
        if is_index_like_stem(stem) {
            if segments.len() == 1 {
                vec![safe_title]
            } else {
                let mut nested = segments[..segments.len() - 2]
                    .iter()
                    .map(|segment| sanitize_path_segment(segment))
                    .collect::<Vec<_>>();
                nested.push(format!(
                    "{}.md",
                    sanitize_path_segment(&segments[segments.len() - 2])
                ));
                nested
            }
        } else {
            let mut nested = segments[..segments.len() - 1]
                .iter()
                .map(|segment| sanitize_path_segment(segment))
                .collect::<Vec<_>>();
            nested.push(safe_title);
            nested
        }
    };

    format!("{domain_label}/{}", output_segments.join("/"))
}

fn strip_docs_root_segments(path: &str) -> Vec<String> {
    let normalized = normalize_path(path);
    let mut segments: Vec<String> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if let Some(first) = segments.first() {
        let first_lower = first.to_ascii_lowercase();
        if matches!(
            first_lower.as_str(),
            "docs" | "doc" | "documentation" | "guide" | "guides" | "wiki" | "content"
        ) {
            segments.remove(0);
        }
    }
    segments
}

fn source_ids_for_path(report: &ScanReport, path: &str) -> Vec<String> {
    report
        .files
        .iter()
        .filter(|file| file.path == path)
        .map(|file| file.id.clone())
        .collect()
}

fn source_ids_for_paths(report: &ScanReport, paths: &[String]) -> Vec<String> {
    let path_set: BTreeSet<&str> = paths.iter().map(String::as_str).collect();
    report
        .files
        .iter()
        .filter(|file| path_set.contains(file.path.as_str()))
        .map(|file| file.id.clone())
        .collect()
}

fn build_docs_anchors(docs_files: &[String]) -> Vec<DocsAnchor> {
    docs_files
        .iter()
        .map(|path| DocsAnchor {
            file_path: normalize_path(path),
            heading: docs_unit_title(path),
            level: 1,
            links: Vec::new(),
        })
        .collect()
}

#[derive(Clone)]
struct ExampleSourceCluster {
    reference_paths: Vec<String>,
}

fn repo_root_readme_path(report: &ScanReport) -> Option<String> {
    report.files.iter().find_map(|file| {
        let normalized = normalize_path(&file.path);
        matches!(
            normalized.as_str(),
            "README.md" | "README.mdx" | "readme.md" | "readme.mdx"
        )
        .then_some(file.path.clone())
    })
}

fn select_representative_example_source_cluster(
    report: &ScanReport,
) -> Option<ExampleSourceCluster> {
    let mut clusters: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in &report.files {
        if !is_primary_example_source_path(&file.path) {
            continue;
        }
        let Some(cluster_key) = example_source_cluster_key(&file.path) else {
            continue;
        };
        clusters
            .entry(cluster_key)
            .or_default()
            .push(file.path.clone());
    }

    let best_cluster = clusters
        .into_iter()
        .filter_map(|(key, mut paths)| {
            sort_and_dedup_strings(&mut paths);
            let score = example_source_cluster_score(&key, &paths);
            (score > 0).then_some((score, key, paths))
        })
        .max_by(|left, right| left.0.cmp(&right.0).then_with(|| right.1.cmp(&left.1)))?;

    Some(ExampleSourceCluster {
        reference_paths: summarize_example_cluster_paths(&best_cluster.2),
    })
}

fn summarize_example_cluster_paths(paths: &[String]) -> Vec<String> {
    let mut selected = paths
        .iter()
        .filter(|path| is_example_manifest_path(path))
        .cloned()
        .collect::<Vec<_>>();
    selected.extend(
        paths
            .iter()
            .filter(|path| is_example_source_file(path))
            .take(8)
            .cloned(),
    );
    if selected.is_empty() {
        selected.extend(paths.iter().take(8).cloned());
    }
    sort_and_dedup_strings(&mut selected);
    selected
}

fn example_source_cluster_score(cluster_key: &str, paths: &[String]) -> isize {
    let source_count = paths
        .iter()
        .filter(|path| is_example_source_file(path))
        .count() as isize;
    if source_count < 3 {
        return 0;
    }

    let manifest_bonus = paths.iter().any(|path| is_example_manifest_path(path)) as isize * 6;
    let readme_bonus = paths.iter().any(|path| {
        matches!(
            normalize_path(path).rsplit('/').next(),
            Some("README.md" | "README.mdx" | "readme.md" | "readme.mdx")
        )
    }) as isize
        * 2;
    let depth_penalty = cluster_key.split('/').count() as isize;

    source_count * 4 + manifest_bonus + readme_bonus - depth_penalty
}

fn example_source_cluster_key(path: &str) -> Option<String> {
    let normalized = normalize_path(path);
    let segments = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let example_index = segments.iter().position(|segment| {
        matches!(
            segment.to_ascii_lowercase().as_str(),
            "example" | "examples" | "sample" | "samples"
        )
    })?;
    if example_index + 1 >= segments.len() {
        return None;
    }

    let mut end = example_index + 2;
    while end < segments.len() {
        let lower = segments[end].to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "src"
                | "main"
                | "test"
                | "tests"
                | "javatests"
                | "java"
                | "kotlin"
                | "scala"
                | "resources"
                | "app"
        ) {
            break;
        }
        end += 1;
        if end - example_index >= 4 {
            break;
        }
    }

    Some(segments[..end].join("/"))
}

fn is_primary_example_source_path(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    if !(normalized.starts_with("examples/")
        || normalized.contains("/examples/")
        || normalized.starts_with("example/")
        || normalized.contains("/example/"))
    {
        return false;
    }

    !normalized.contains("/test/")
        && !normalized.contains("/tests/")
        && !normalized.contains("/javatests/")
        && !normalized.contains("/fixtures/")
        && !normalized.contains("/docs/")
        && !normalized.ends_with(".png")
        && !normalized.ends_with(".jpg")
        && !normalized.ends_with(".jpeg")
        && !normalized.ends_with(".gif")
        && !normalized.ends_with(".svg")
        && !normalized.ends_with(".snap")
}

fn is_example_manifest_path(path: &str) -> bool {
    matches!(
        normalize_path(path)
            .to_ascii_lowercase()
            .rsplit('/')
            .next()
            .unwrap_or_default(),
        "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "package.json"
            | "cargo.toml"
            | "settings.gradle"
            | "settings.gradle.kts"
            | "build"
    )
}

fn is_example_source_file(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    [
        ".java", ".kt", ".kts", ".groovy", ".scala", ".js", ".jsx", ".ts", ".tsx", ".py", ".rs",
        ".go",
    ]
    .iter()
    .any(|suffix| normalized.ends_with(suffix))
}

fn dedup_units_by_relative_path(mut units: Vec<KnowledgeUnit>) -> Vec<KnowledgeUnit> {
    units.sort_by(|left, right| {
        left.relative_path
            .cmp(&right.relative_path)
            .then_with(|| {
                right
                    .scope
                    .docs_anchors
                    .is_empty()
                    .cmp(&left.scope.docs_anchors.is_empty())
            })
            .then(
                right
                    .scope
                    .source_ids
                    .len()
                    .cmp(&left.scope.source_ids.len()),
            )
            .then(left.title.cmp(&right.title))
    });
    let mut seen = BTreeSet::new();
    units
        .into_iter()
        .filter(|unit| seen.insert(unit.relative_path.clone()))
        .collect()
}

fn snapshot_parent_links(units: &[KnowledgeUnit]) -> BTreeMap<String, Option<String>> {
    units
        .iter()
        .map(|unit| (unit.id.clone(), unit.parent_unit_id.clone()))
        .collect()
}

fn collect_domain_index_by_domain(units: &[KnowledgeUnit]) -> BTreeMap<String, String> {
    units
        .iter()
        .filter(|unit| unit.unit_type == UnitType::DomainIndex)
        .map(|unit| (unit.domain_id.clone(), unit.id.clone()))
        .collect()
}

fn repair_orphan_parent_links(
    units: &mut Vec<KnowledgeUnit>,
    original_parent_by_id: &BTreeMap<String, Option<String>>,
    domain_index_by_domain: &BTreeMap<String, String>,
) {
    let existing_ids: BTreeSet<String> = units.iter().map(|unit| unit.id.clone()).collect();

    for unit in units.iter_mut() {
        // 父引用收敛后统一重建 child_unit_ids，避免保留被裁剪节点留下的旧 child link。
        unit.child_unit_ids.clear();
        unit.parent_unit_id = resolve_surviving_parent(
            unit,
            &existing_ids,
            original_parent_by_id,
            domain_index_by_domain,
        );
    }
}

fn resolve_surviving_parent(
    unit: &KnowledgeUnit,
    existing_ids: &BTreeSet<String>,
    original_parent_by_id: &BTreeMap<String, Option<String>>,
    domain_index_by_domain: &BTreeMap<String, String>,
) -> Option<String> {
    let mut cursor = unit.parent_unit_id.clone();
    let mut visited = BTreeSet::new();

    while let Some(parent_id) = cursor {
        if !visited.insert(parent_id.clone()) {
            break;
        }
        if existing_ids.contains(&parent_id) && parent_id != unit.id {
            return Some(parent_id);
        }
        cursor = original_parent_by_id.get(&parent_id).cloned().flatten();
    }

    if let Some(domain_index_id) = domain_index_by_domain.get(&unit.domain_id) {
        if domain_index_id != &unit.id && existing_ids.contains(domain_index_id) {
            return Some(domain_index_id.clone());
        }
    }

    None
}

fn estimate_file_depth(paths: &[String]) -> u32 {
    let max_depth = paths
        .iter()
        .map(|path| strip_docs_root_segments(path).len())
        .max()
        .unwrap_or(1);
    max_depth.clamp(1, 3) as u32
}

fn sort_and_dedup_strings(paths: &mut Vec<String>) {
    paths.sort();
    paths.dedup();
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn is_markdown_path(path: &str) -> bool {
    path.ends_with(".md") || path.ends_with(".mdx")
}

fn markdown_stem(file_name: &str) -> &str {
    file_name.trim_end_matches(".md").trim_end_matches(".mdx")
}

fn is_index_like_stem(stem: &str) -> bool {
    stem.eq_ignore_ascii_case("index") || stem.eq_ignore_ascii_case("readme")
}

// ─── Build knowledge tree ───────────────────────────────────

/// 从域列表和单元列表构建完整知识树。
pub fn build_knowledge_tree(
    domains: Vec<KnowledgeDomain>,
    units: Vec<KnowledgeUnit>,
) -> KnowledgeTree {
    let overview_id = units
        .iter()
        .find(|u| u.unit_type == UnitType::Overview)
        .map(|u| u.id.clone())
        .unwrap_or_else(|| "overview-missing".to_string());

    let mut tree = KnowledgeTree::new(overview_id);

    for domain in domains {
        tree.add_domain(domain);
    }

    // wire overview -> architecture + domain indices
    let arch_id = units
        .iter()
        .find(|u| u.unit_type == UnitType::Architecture)
        .map(|u| u.id.clone());
    let domain_index_ids: Vec<String> = units
        .iter()
        .filter(|u| u.unit_type == UnitType::DomainIndex)
        .map(|u| u.id.clone())
        .collect();

    for mut unit in units {
        if unit.unit_type == UnitType::Overview {
            if let Some(ref aid) = arch_id {
                if !unit.child_unit_ids.contains(aid) {
                    unit.child_unit_ids.push(aid.clone());
                }
            }
            for did in &domain_index_ids {
                if !unit.child_unit_ids.contains(did) {
                    unit.child_unit_ids.push(did.clone());
                }
            }
        }
        if unit.unit_type == UnitType::Architecture {
            unit.parent_unit_id = Some(tree.overview_unit_id.clone());
        }
        if unit.unit_type == UnitType::DomainIndex {
            unit.parent_unit_id = Some(tree.overview_unit_id.clone());
        }
        tree.add_unit(unit);
    }

    tree.build_processing_order();
    tree
}

// ─── Helpers ────────────────────────────────────────────────

fn collect_top_modules<'a>(module_tree: &'a ModuleTree) -> Vec<&'a ModuleNode> {
    let root_ids: BTreeSet<&str> = module_tree
        .root_modules
        .iter()
        .map(String::as_str)
        .collect();

    module_tree
        .modules
        .iter()
        .filter(|m| m.parent_id.is_some() || root_ids.contains(m.id.as_str()))
        .filter(|module| !is_low_signal_module_candidate(module))
        .collect()
}

fn collect_module_files(report: &ScanReport, modules: &[&ModuleNode]) -> Vec<String> {
    let source_ids: BTreeSet<&str> = modules
        .iter()
        .flat_map(|m| m.source_ids.iter().map(String::as_str))
        .collect();

    report
        .files
        .iter()
        .filter(|f| source_ids.contains(f.id.as_str()))
        .map(|f| f.path.clone())
        .collect()
}

fn collect_docs_files(report: &ScanReport) -> Vec<String> {
    report
        .files
        .iter()
        .filter(|f| f.is_docs_like())
        .filter(|file| is_primary_docs_path(&file.path))
        .filter(|file| !docs_path_has_hidden_prefix(&file.path))
        .filter(|file| !is_low_signal_docs_path(&file.path))
        .map(|f| f.path.clone())
        .collect()
}

fn collect_config_files(report: &ScanReport) -> Vec<String> {
    let mut configs: Vec<String> = report
        .config_files
        .iter()
        .filter(|path| is_primary_config_path(path))
        .cloned()
        .collect();
    configs.extend(
        report
            .files
            .iter()
            .filter(|f| f.is_config_like() || looks_like_runtime_config_entry(&f.path))
            .filter(|f| is_primary_config_path(&f.path))
            .map(|f| f.path.clone()),
    );
    configs.sort();
    configs.dedup();
    configs
}

fn collect_test_files(report: &ScanReport) -> Vec<String> {
    report
        .files
        .iter()
        .filter(|f| f.is_test_like())
        .map(|f| f.path.clone())
        .collect()
}

fn estimate_depth(modules: &[&ModuleNode]) -> u32 {
    if modules.len() > 5 {
        3
    } else if modules.len() > 2 {
        2
    } else {
        1
    }
}

fn compute_module_priority(module: &ModuleNode, report: &ScanReport) -> f32 {
    let source_count = module
        .source_ids
        .iter()
        .filter(|sid| {
            report
                .files
                .iter()
                .find(|f| &f.id == *sid)
                .map(|f| f.is_substantive_source())
                .unwrap_or(false)
        })
        .count();

    let mut priority = 1.0_f32;
    if source_count > 10 {
        priority += 0.5;
    }
    if !module.entry_points.is_empty() {
        priority += 0.3;
    }
    if !module.child_ids.is_empty() {
        priority += 0.2;
    }
    priority
}

fn sanitize_path_segment(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' | '/' | '\\' | ':' => '-',
            _ if c.is_alphanumeric() => c,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn is_low_signal_module_candidate(module: &ModuleNode) -> bool {
    let mut signals = Vec::with_capacity(1 + module.root_paths.len());
    signals.push(module.name.to_ascii_lowercase());
    signals.extend(
        module
            .root_paths
            .iter()
            .map(|path| normalize_path(path).to_ascii_lowercase()),
    );

    signals.iter().any(|value| {
        [
            "/example/",
            "/examples/",
            "/demo/",
            "/demos/",
            "/sample/",
            "/samples/",
            "/fixture/",
            "/fixtures/",
            "/sandbox/",
            "/sandboxes/",
            "/test-storybooks/",
            "/kitchen-sink/",
            "/storybook-static/",
        ]
        .iter()
        .any(|needle| value.contains(needle))
            || matches!(
                value.as_str(),
                "example"
                    | "examples"
                    | "demo"
                    | "demos"
                    | "sample"
                    | "samples"
                    | "fixture"
                    | "fixtures"
                    | "sandbox"
                    | "sandboxes"
                    | "kitchen-sink"
                    | "storybook-static"
            )
    })
}

fn is_low_signal_docs_path(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();
    normalized.contains("/_snippets/")
        || normalized.contains("/snippets/")
        || normalized.contains("/storybook-static/")
        || normalized.contains("/.wiki/")
        || normalized.contains("/__mockdata__/")
        || normalized.contains("/fixtures/")
        || normalized.contains("/template/stories/")
        || normalized.contains("/test-storybooks/")
}

fn is_primary_docs_path(path: &str) -> bool {
    let normalized = normalize_path(path);
    let segments: Vec<&str> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.is_empty() {
        return false;
    }
    if segments.len() == 1 {
        return is_markdown_path(segments[0]);
    }
    if has_primary_docs_root(&segments) {
        if docs_root_is_embedded(&segments) {
            return false;
        }
        return true;
    }

    is_index_like_stem(markdown_stem(segments.last().copied().unwrap_or_default()))
        && !is_embedded_source_markdown_path(&normalized)
}

fn has_primary_docs_root(segments: &[&str]) -> bool {
    segments.iter().enumerate().any(|(index, segment)| {
        if !matches!(
            segment.to_ascii_lowercase().as_str(),
            "docs" | "doc" | "documentation" | "guide" | "guides" | "wiki" | "content"
        ) {
            return false;
        }

        !matches!(
            segments.get(index + 1).map(|value| value.to_ascii_lowercase()),
            Some(next)
                if matches!(
                    next.as_str(),
                    "src" | "lib" | "dist" | "build" | "__tests__" | "test" | "tests"
                )
        )
    })
}

fn docs_root_is_embedded(segments: &[&str]) -> bool {
    segments.iter().enumerate().any(|(index, segment)| {
        if !matches!(
            segment.to_ascii_lowercase().as_str(),
            "docs" | "doc" | "documentation" | "guide" | "guides" | "wiki" | "content"
        ) {
            return false;
        }

        segments[..index].iter().any(|prefix| {
            matches!(
                prefix.to_ascii_lowercase().as_str(),
                "code"
                    | "src"
                    | "lib"
                    | "dist"
                    | "build"
                    | "scripts"
                    | "example"
                    | "examples"
                    | "__tests__"
                    | "test"
                    | "tests"
                    | "main"
                    | "java"
                    | "kotlin"
                    | "swift"
                    | "androidtest"
            )
        })
    })
}

fn is_embedded_source_markdown_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [
        "code/",
        "/src/",
        "/main/",
        "/main/java/",
        "/main/kotlin/",
        "/main/scala/",
        "/main/swift/",
        "/main/objc/",
        "/app/src/",
        "/lib/",
        "/dist/",
        "/scripts/",
        "/test/",
        "/tests/",
        "/__tests__/",
        "/androidtest/",
        "/stories/",
        "/fixtures/",
        "/example/",
        "/examples/",
        "/sample/",
        "/samples/",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn looks_like_runtime_config_entry(path: &str) -> bool {
    let normalized = normalize_path(path);
    let lower = normalized.to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());
    matches!(
        file_name,
        "main.js"
            | "main.ts"
            | "main.jsx"
            | "main.tsx"
            | "main.mjs"
            | "preview.js"
            | "preview.ts"
            | "preview.jsx"
            | "preview.tsx"
            | "preview.mjs"
            | "manager.js"
            | "manager.ts"
            | "manager.jsx"
            | "manager.tsx"
            | "manager.mjs"
    ) && normalized.split('/').any(|segment| {
        matches!(
            segment.to_ascii_lowercase().as_str(),
            ".storybook" | "storybook" | "config" | "configs" | "configure" | "settings"
        )
    })
}

fn is_primary_config_path(path: &str) -> bool {
    let normalized = normalize_path(path);
    let lower = normalized.to_ascii_lowercase();
    if is_low_signal_docs_path(path)
        || lower.contains("/fixtures/")
        || lower.contains("/__tests__/")
        || lower.contains("/stories/")
        || lower.contains(".stories.")
        || lower.contains(".story.")
        || lower.contains(".test.")
        || lower.contains(".spec.")
        || lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".svg")
        || lower.ends_with(".gif")
    {
        return false;
    }

    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());
    if file_name.starts_with('.') {
        return true;
    }
    if matches!(
        file_name,
        "package.json"
            | "tsconfig.json"
            | "settings.gradle"
            | "settings.gradle.kts"
            | "build.gradle"
            | "build.gradle.kts"
            | "cargo.toml"
            | "pyproject.toml"
            | "pom.xml"
            | "androidmanifest.xml"
    ) {
        return true;
    }
    if looks_like_runtime_config_entry(path) {
        return true;
    }
    if file_name.contains("config")
        || file_name.contains("settings")
        || file_name.contains("manifest")
        || file_name.ends_with(".yaml")
        || file_name.ends_with(".yml")
        || file_name.ends_with(".toml")
        || file_name.ends_with(".json")
        || file_name.ends_with(".jsonc")
        || file_name.ends_with(".ini")
        || file_name.ends_with(".conf")
        || file_name.ends_with(".properties")
    {
        return true;
    }

    false
}

fn looks_like_api_surface_path(path: &str) -> bool {
    if looks_like_runtime_config_entry(path) {
        return false;
    }

    let normalized = normalize_path(path);
    let lower = normalized.to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());

    matches!(
        file_name,
        "addons.ts"
            | "addons.js"
            | "addon-types.ts"
            | "addon-types.js"
            | "androidinjector.java"
            | "hasandroidinjector.java"
            | "androidinjection.java"
            | "installin.java"
            | "entrypoint.java"
            | "definecomponent.java"
            | "generatesrootinput.java"
            | "hiltandroidapp.java"
            | "androidentrypoint.java"
            | "componentprocessor.java"
            | "delegatecomponentprocessor.java"
            | "bindinggraphfactory.java"
            | "sourcefilegenerator.java"
            | "validationreport.java"
            | "csf.ts"
            | "csf.js"
            | "preview-web.ts"
            | "preview-web.js"
            | "store.ts"
            | "store.js"
            | "hooks.ts"
            | "hooks.js"
            | "builder.ts"
            | "builder.js"
            | "builders.ts"
            | "builders.js"
            | "frameworks.ts"
            | "frameworks.js"
            | "story.ts"
            | "story.js"
            | "typings.d.ts"
            | "decorators.ts"
            | "decorators.js"
    ) || lower.contains("framework-preset-")
        || lower.contains("/store/csf/")
        || lower.contains("composeconfigs")
        || lower.contains("processcsffile")
}








