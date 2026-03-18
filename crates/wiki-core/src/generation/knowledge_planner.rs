use std::collections::{BTreeMap, BTreeSet};

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{
    ApiSurface, ConfigSurface, DecompositionProfile, DocsAnchor, DomainEvidence, DomainType,
    KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitScope, UnitType,
};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::steering::SteeringConfig;
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::GraphSummary;

// ─── Domain discovery ───────────────────────────────────────

/// 从 Facts 层的 ModuleTree + ScanReport + GraphSummary + RepoContext 中发现知识域。
/// 每种 DomainType 有独立的启发式规则，按证据强度排序。
pub fn discover_knowledge_domains(
    report: &ScanReport,
    module_tree: &ModuleTree,
    _repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    _graph_summary: &GraphSummary,
    steering: &SteeringConfig,
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
    apply_steering_overrides(&mut domains, steering);
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
        .map(|file| file.id.clone())
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
            test_keywords.iter().any(|kw| name_lower.contains(kw))
                || m.tags.iter().any(|t| t == "test" || t == "testing")
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

    if matched.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::DevTooling, "开发工具");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/cli/**".into(), "**/scripts/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = collect_module_files(report, &matched);
    domain.estimated_depth = estimate_depth(&matched);
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

fn apply_steering_overrides(domains: &mut Vec<KnowledgeDomain>, steering: &SteeringConfig) {
    let suppress = &steering.knowledge.suppress_domains;
    if !suppress.is_empty() {
        domains.retain(|d| {
            !suppress.iter().any(|s| {
                s.eq_ignore_ascii_case(d.domain_type.as_str()) || s.eq_ignore_ascii_case(&d.label)
            })
        });
    }

    let force = &steering.knowledge.force_domains;
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
    steering: &SteeringConfig,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();
    let weight_threshold = steering.knowledge.unit_weight_threshold;
    let max_per_domain = steering.knowledge.max_units_per_domain;
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
    prune_shadowed_docs_backed_units(&mut units);
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
        DomainType::PlatformBinding
        | DomainType::BuildSystem
        | DomainType::ThemeSystem
        | DomainType::DevTooling => plan_module_doc_units(
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
        DomainType::PluginEcosystem | DomainType::MultiFramework => plan_module_doc_units(
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

        let module_name = sanitize_path_segment(&module.name);
        let mut unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            &module.name,
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

    if domain_matches_keywords(
        domain,
        &["hilt", "hiltandroidapp", "androidentrypoint", "installin"],
    ) {
        units.extend(plan_domain_signal_family_units(
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
        ));
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
    if domain.label != "核心模块" && domain_has_annotation_di_signal(domain) {
        units.extend(plan_domain_signal_family_units(
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
        ));
        units.extend(plan_domain_signal_family_units(
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
        ));
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
    if domain_has_annotation_di_signal(domain) {
        units.extend(plan_domain_signal_family_units(
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
        ));
        units.extend(plan_domain_signal_leaf_units(
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
        ));
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
    units.extend(plan_domain_signal_family_units(
        domain,
        report,
        domain_label,
        "测试策略与最佳实践",
        UnitType::TestDoc,
        DecompositionProfile::Testing,
        0.72,
        &[
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
        ],
    ));
    units
}

fn plan_concept_guide_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    plan_docs_backed_units_for_domain(domain, report, domain_label, |profile| {
        !matches!(
            profile,
            DecompositionProfile::ApiSurface
                | DecompositionProfile::ConfigSurface
                | DecompositionProfile::Troubleshooting
        )
    })
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
                title: "插件API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.74,
                keywords: &["addon", "addons", "addon-types"],
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
                title: "Store API",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.72,
                keywords: &["store", "story-store", "usestorybookstate"],
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
                title: "工具类型定义",
                unit_type: UnitType::ApiDoc,
                profile: DecompositionProfile::ApiSurface,
                min_hits: 1,
                priority: 0.7,
                keywords: &["builder", "builders", "cli", "tool"],
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
    if domain_has_annotation_di_signal(domain) {
        signal_leaves.extend([
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
    units.extend(plan_domain_signal_leaf_units(
        domain,
        report,
        domain_label,
        &signal_leaves,
    ));

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
    let mut family_source_ids = BTreeSet::new();
    let family_relative_path = format!("{domain_label}/{}.md", sanitize_path_segment(family_title));
    let family_id = crate::domain::stable_id::stable_id(
        "unit",
        format!(
            "{}:{}:{}",
            family_unit_type.as_str(),
            &domain.id,
            family_title
        ),
    );

    for child in children {
        let matched_source_ids = matching_signal_source_ids(&candidate_files, child.keywords);
        if matched_source_ids.len() < child.min_hits {
            continue;
        }
        family_source_ids.extend(matched_source_ids.iter().cloned());
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
            module_ids: domain.source_modules.clone(),
            source_ids: matched_source_ids,
            ..Default::default()
        };
        unit.priority = child.priority;
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
    family.decomposition_profile = Some(family_profile);
    family.scope = UnitScope {
        module_ids: domain.source_modules.clone(),
        source_ids: family_source_ids.into_iter().collect(),
        ..Default::default()
    };
    family.priority = family_priority;

    let mut units = vec![family];
    units.extend(child_units);
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
        let matched_source_ids = matching_signal_source_ids(&candidate_files, leaf.keywords);
        if matched_source_ids.len() < leaf.min_hits {
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
            module_ids: domain.source_modules.clone(),
            source_ids: matched_source_ids,
            ..Default::default()
        };
        unit.priority = leaf.priority;
        units.push(unit);
    }

    units
}

fn collect_domain_signal_candidates<'a>(
    domain: &KnowledgeDomain,
    report: &'a ScanReport,
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    let source_paths: BTreeSet<&str> = domain.source_files.iter().map(String::as_str).collect();
    report
        .files
        .iter()
        .filter(|file| source_paths.contains(file.path.as_str()))
        .filter(|file| match domain.domain_type {
            DomainType::ConfigReference => !file.is_asset_like(),
            _ => {
                !is_markdown_path(&file.path)
                    && !file.is_asset_like()
                    && !file.is_config_like()
                    && !file.is_docs_like()
            }
        })
        .collect()
}

fn domain_matches_keywords(domain: &KnowledgeDomain, keywords: &[&str]) -> bool {
    let label = domain.label.to_ascii_lowercase();
    if keywords.iter().any(|keyword| label.contains(keyword)) {
        return true;
    }

    domain.source_files.iter().any(|path| {
        let lower = normalize_path(path).to_ascii_lowercase();
        keywords.iter().any(|keyword| lower.contains(keyword))
    })
}

fn domain_has_annotation_di_signal(domain: &KnowledgeDomain) -> bool {
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
    domain.source_files.iter().any(|path| {
        let lower = normalize_path(path).to_ascii_lowercase();
        signals.iter().any(|signal| lower.contains(signal))
    })
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

fn matching_signal_source_ids(
    files: &[&crate::repo::scanner::ScannedFile],
    keywords: &[&str],
) -> Vec<String> {
    files
        .iter()
        .filter(|file| {
            let lower = normalize_path(&file.path).to_ascii_lowercase();
            keywords.iter().any(|keyword| lower.contains(keyword))
        })
        .map(|file| file.id.clone())
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
            source_ids: report
                .files
                .iter()
                .filter(|f| f.path == surface.file_path)
                .map(|f| f.id.clone())
                .collect(),
            config_surfaces: vec![surface.clone()],
            ..Default::default()
        };
        unit.priority = 0.6;
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

        let title = docs_unit_title(path);
        let mut unit = KnowledgeUnit::new(
            unit_type,
            &title,
            &domain.id,
            docs_output_relative_path(domain_label, path, &title),
        );
        unit.decomposition_profile = Some(profile);
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
        units.push(unit);
    }

    assign_docs_unit_parents(&mut units);
    dedup_units_by_relative_path(units)
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

fn dedup_units_by_relative_path(units: Vec<KnowledgeUnit>) -> Vec<KnowledgeUnit> {
    let mut seen = BTreeSet::new();
    units
        .into_iter()
        .filter(|unit| seen.insert(unit.relative_path.clone()))
        .collect()
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
            | "main.mjs"
            | "preview.js"
            | "preview.ts"
            | "preview.mjs"
            | "manager.js"
            | "manager.ts"
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
            | "installin.java"
            | "entrypoint.java"
            | "definecomponent.java"
            | "generatesrootinput.java"
            | "hiltandroidapp.java"
            | "androidentrypoint.java"
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
