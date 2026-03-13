use std::collections::{BTreeMap, BTreeSet};

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::knowledge::{
    ApiSurface, ConfigSurface, DocsAnchor, DomainEvidence, DomainType, KnowledgeDomain,
    KnowledgeTree, KnowledgeUnit, UnitScope, UnitType,
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
    if let Some(domain) = discover_compiler_toolchain(&top_modules, &module_index, report) {
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
    if let Some(domain) = discover_testing_infra(&top_modules, &module_index, report, &test_files)
    {
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
    if let Some(domain) = discover_concept_guide(report, &docs_files) {
        domains.push(domain);
    }

    // ApiReference
    if let Some(domain) =
        discover_api_reference(&top_modules, &module_context_index, report)
    {
        domains.push(domain);
    }

    // ConfigReference
    if let Some(domain) = discover_config_reference(report, &config_files) {
        domains.push(domain);
    }

    // Steering overrides
    apply_steering_overrides(&mut domains, steering);

    // 兜底：将未被任何 ModuleDoc 产生域覆盖的模块收入 CoreRuntime 域
    {
        let module_doc_domain_types = [
            DomainType::CoreRuntime,
            DomainType::Framework,
            DomainType::PlatformBinding,
            DomainType::CompilerToolchain,
            DomainType::PluginEcosystem,
            DomainType::MultiFramework,
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
            core_keywords
                .iter()
                .any(|kw| name_lower.contains(kw))
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
    domain.estimated_depth = estimate_depth(&matched);
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

        let mut domain = KnowledgeDomain::new(
            DomainType::Framework,
            format!("框架集成：{label}"),
        );
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

        let mut domain = KnowledgeDomain::new(
            DomainType::PlatformBinding,
            format!("平台绑定：{label}"),
        );
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
) -> Option<KnowledgeDomain> {
    let compiler_keywords = [
        "compiler", "codegen", "transpiler", "processor", "spi", "annotation",
        "transform", "babel", "swc",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            compiler_keywords
                .iter()
                .any(|kw| name_lower.contains(kw))
        })
        .copied()
        .collect();

    if matched.is_empty() {
        return None;
    }

    let mut domain = KnowledgeDomain::new(DomainType::CompilerToolchain, "编译工具链");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec!["**/compiler/**".into(), "**/codegen/**".into()],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = collect_module_files(report, &matched);
    domain.estimated_depth = estimate_depth(&matched);
    Some(domain)
}

fn discover_plugin_ecosystem(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let plugin_keywords = [
        "addon", "plugin", "extension", "preset", "decorator",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            plugin_keywords
                .iter()
                .any(|kw| name_lower.contains(kw))
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
    domain.estimated_depth = if matched.is_empty() { 2 } else { estimate_depth(&matched) };
    Some(domain)
}

fn discover_multi_framework(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let renderer_keywords = [
        "renderer", "react", "vue", "angular", "svelte", "preact", "web-component",
        "html", "server",
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
                        || m.root_paths.iter().any(|path| path.to_ascii_lowercase().contains(fw))
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
    domain.estimated_depth = if all_modules.is_empty() { 2 } else { estimate_depth(&all_modules) };
    Some(domain)
}

fn discover_testing_infra(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    _report: &ScanReport,
    test_files: &[String],
) -> Option<KnowledgeDomain> {
    let test_keywords = [
        "test", "testing", "spec", "e2e", "integration-test", "benchmark",
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

    let mut domain = KnowledgeDomain::new(DomainType::TestingInfra, "测试基础设施");
    domain.evidence = DomainEvidence {
        matched_modules: matched.iter().map(|m| m.id.clone()).collect(),
        matched_file_patterns: vec![
            "**/*test*/**".into(),
            "**/*.test.*".into(),
            "**/*.spec.*".into(),
        ],
        ..Default::default()
    };
    domain.source_modules = matched.iter().map(|m| m.id.clone()).collect();
    domain.source_files = test_files.iter().take(50).cloned().collect();
    domain.estimated_depth = if matched.len() > 2 { 2 } else { 1 };
    Some(domain)
}

fn discover_build_system(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let build_keywords = [
        "builder", "bundler", "build", "webpack", "rollup", "vite", "esbuild",
        "turbopack", "nx",
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
    domain.estimated_depth = if matched.is_empty() { 1 } else { estimate_depth(&matched) };
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
            theme_keywords
                .iter()
                .any(|kw| name_lower.contains(kw))
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
    domain.estimated_depth = if matched.is_empty() { 1 } else { estimate_depth(&matched) };
    Some(domain)
}

fn discover_dev_tooling(
    top_modules: &[&ModuleNode],
    _module_index: &BTreeMap<&str, &ModuleNode>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let tooling_keywords = [
        "cli", "dev-tool", "devtool", "tool", "scripts", "linter", "formatter",
    ];

    let matched: Vec<&ModuleNode> = top_modules
        .iter()
        .filter(|m| {
            let name_lower = m.name.to_ascii_lowercase();
            tooling_keywords
                .iter()
                .any(|kw| name_lower.contains(kw))
                || m.kind == "cli-tool"
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

fn discover_concept_guide(
    _report: &ScanReport,
    docs_files: &[String],
) -> Option<KnowledgeDomain> {
    if docs_files.is_empty() {
        return None;
    }

    let docs_anchors: Vec<DocsAnchor> = docs_files
        .iter()
        .take(20)
        .map(|path| {
            let heading = path
                .rsplit('/')
                .next()
                .unwrap_or(path)
                .trim_end_matches(".md")
                .trim_end_matches(".mdx")
                .to_string();
            DocsAnchor {
                file_path: path.clone(),
                heading,
                level: 1,
                links: Vec::new(),
            }
        })
        .collect();

    let mut domain = KnowledgeDomain::new(DomainType::ConceptGuide, "概念指南");
    domain.evidence = DomainEvidence {
        matched_file_patterns: vec!["docs/**/*.md".into(), "**/*.mdx".into()],
        docs_anchors,
        ..Default::default()
    };
    domain.source_files = docs_files.iter().take(50).cloned().collect();
    domain.estimated_depth = 1;
    Some(domain)
}

fn discover_api_reference(
    top_modules: &[&ModuleNode],
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    report: &ScanReport,
) -> Option<KnowledgeDomain> {
    let mut api_modules = Vec::new();
    let mut api_surfaces = Vec::new();

    for module in top_modules {
        if let Some(ctx) = module_context_index.get(module.id.as_str()) {
            if ctx.public_surface.len() >= 2 {
                api_modules.push(*module);
                api_surfaces.push(ApiSurface {
                    module_id: module.id.clone(),
                    exported_symbols: ctx.public_surface.clone(),
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
        })
        .collect();

    if api_modules.is_empty() && api_file_hits.is_empty() {
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
        public_api_surfaces: api_surfaces,
        ..Default::default()
    };
    domain.source_modules = api_modules.iter().map(|m| m.id.clone()).collect();
    let mut source_files = collect_module_files(report, &api_modules);
    source_files.extend(api_file_hits.iter().map(|file| file.path.clone()));
    source_files.sort();
    source_files.dedup();
    domain.source_files = source_files;
    domain.estimated_depth = 1;
    Some(domain)
}

fn discover_config_reference(
    _report: &ScanReport,
    config_files: &[String],
) -> Option<KnowledgeDomain> {
    if config_files.len() < 2 {
        return None;
    }

    let config_surfaces: Vec<ConfigSurface> = config_files
        .iter()
        .take(15)
        .map(|path| ConfigSurface {
            file_path: path.clone(),
            keys: Vec::new(),
        })
        .collect();

    let mut domain = KnowledgeDomain::new(DomainType::ConfigReference, "配置参考");
    domain.evidence = DomainEvidence {
        matched_file_patterns: vec![
            "**/*.config.*".into(),
            "**/config/**".into(),
        ],
        config_surfaces,
        ..Default::default()
    };
    domain.source_files = config_files.iter().take(30).cloned().collect();
    domain.estimated_depth = 1;
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
        let already_present = domains.iter().any(|d| {
            d.domain_type.as_str().eq_ignore_ascii_case(forced_type_str)
        });
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

    let module_context_index: BTreeMap<&str, &ModuleContext> = module_contexts
        .iter()
        .map(|c| (c.module_id.as_str(), c))
        .collect();

    // 1. Overview unit（全局唯一）
    let overview = KnowledgeUnit::new(
        UnitType::Overview,
        "项目概述",
        "system",
        "项目概述.md",
    );
    units.push(overview);

    // 2. Architecture unit（全局唯一）
    let arch = KnowledgeUnit::new(
        UnitType::Architecture,
        "系统架构",
        "system",
        "系统架构.md",
    );
    units.push(arch);

    // 3. 按域生成 DomainIndex + 域内单元
    for domain in domains {
        let domain_label = sanitize_path_segment(&domain.label);
        let domain_index = KnowledgeUnit::new(
            UnitType::DomainIndex,
            &domain.label,
            &domain.id,
            format!("{domain_label}/{domain_label}.md"),
        );
        let domain_index_id = domain_index.id.clone();
        units.push(domain_index);

        let mut domain_units = plan_units_for_domain(
            domain,
            module_tree,
            report,
            &module_context_index,
            &domain_label,
        );

        // Steering: 过滤低权重单元
        if weight_threshold > 0.0 {
            domain_units.retain(|u| u.priority >= weight_threshold);
        }

        // Steering: 限制域内单元数
        if max_per_domain > 0 && domain_units.len() > max_per_domain {
            domain_units.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
            domain_units.truncate(max_per_domain);
        }

        for mut unit in domain_units {
            unit.parent_unit_id = Some(domain_index_id.clone());
            units.push(unit);
        }
    }

    // 4. 建立父子关系
    establish_parent_child_links(&mut units);

    units
}

fn plan_units_for_domain(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    report: &ScanReport,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    match domain.domain_type {
        DomainType::CoreRuntime
        | DomainType::Framework
        | DomainType::PlatformBinding
        | DomainType::BuildSystem
        | DomainType::ThemeSystem
        | DomainType::DevTooling => {
            plan_module_doc_units(domain, module_tree, report, module_context_index, domain_label)
        }
        DomainType::CompilerToolchain => {
            plan_module_doc_units(domain, module_tree, report, module_context_index, domain_label)
        }
        DomainType::PluginEcosystem | DomainType::MultiFramework => {
            plan_module_doc_units(domain, module_tree, report, module_context_index, domain_label)
        }
        DomainType::TestingInfra => {
            plan_test_doc_units(domain, module_tree, report, domain_label)
        }
        DomainType::ConceptGuide => plan_concept_guide_units(domain, report, domain_label),
        DomainType::ApiReference => {
            plan_api_doc_units(domain, module_tree, module_context_index, domain_label)
        }
        DomainType::ConfigReference => plan_config_doc_units(domain, report, domain_label),
        DomainType::Troubleshooting => Vec::new(),
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

fn plan_concept_guide_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    domain
        .source_files
        .iter()
        .filter(|path| path.ends_with(".md") || path.ends_with(".mdx"))
        .take(20)
        .map(|path| {
            let file_name = path
                .rsplit('/')
                .next()
                .unwrap_or(path)
                .trim_end_matches(".md")
                .trim_end_matches(".mdx");
            let safe_name = sanitize_path_segment(file_name);
            let mut unit = KnowledgeUnit::new(
                UnitType::ConceptGuide,
                file_name,
                &domain.id,
                format!("{domain_label}/{safe_name}.md"),
            );
            unit.scope = UnitScope {
                source_ids: report
                    .files
                    .iter()
                    .filter(|f| f.path == *path)
                    .map(|f| f.id.clone())
                    .collect(),
                docs_anchors: vec![DocsAnchor {
                    file_path: path.clone(),
                    heading: file_name.to_string(),
                    level: 1,
                    links: Vec::new(),
                }],
                ..Default::default()
            };
            unit.priority = 0.6;
            unit
        })
        .collect()
}

fn plan_api_doc_units(
    domain: &KnowledgeDomain,
    module_tree: &ModuleTree,
    module_context_index: &BTreeMap<&str, &ModuleContext>,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    let mut units = Vec::new();

    for module_id in &domain.source_modules {
        let Some(module) = module_tree.module_by_id(module_id) else {
            continue;
        };
        let Some(ctx) = module_context_index.get(module_id.as_str()) else {
            continue;
        };

        if ctx.public_surface.is_empty() {
            continue;
        }

        let module_name = sanitize_path_segment(&module.name);
        let mut unit = KnowledgeUnit::new(
            UnitType::ApiDoc,
            format!("API：{}", module.name),
            &domain.id,
            format!("{domain_label}/{module_name}.md"),
        );
        unit.scope = UnitScope {
            module_ids: vec![module_id.clone()],
            source_ids: module.source_ids.clone(),
            api_surfaces: vec![ApiSurface {
                module_id: module_id.clone(),
                exported_symbols: ctx.public_surface.clone(),
            }],
            ..Default::default()
        };
        unit.priority = 0.7;
        units.push(unit);
    }

    units
}

fn plan_config_doc_units(
    domain: &KnowledgeDomain,
    report: &ScanReport,
    domain_label: &str,
) -> Vec<KnowledgeUnit> {
    if domain.source_files.is_empty() {
        return Vec::new();
    }

    let mut unit = KnowledgeUnit::new(
        UnitType::ConfigDoc,
        "配置参考",
        &domain.id,
        format!("{domain_label}/配置参考.md"),
    );
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
        config_surfaces: domain
            .evidence
            .config_surfaces
            .clone(),
        ..Default::default()
    };
    unit.priority = 0.5;
    vec![unit]
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
        .filter(|m| {
            m.parent_id.is_some()
                || root_ids.contains(m.id.as_str())
        })
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
        .map(|f| f.path.clone())
        .collect()
}

fn collect_config_files(report: &ScanReport) -> Vec<String> {
    let mut configs: Vec<String> = report.config_files.clone();
    configs.extend(
        report
            .files
            .iter()
            .filter(|f| {
                f.is_config_like()
                    || f.path.to_ascii_lowercase().contains("config")
                    || f.path.to_ascii_lowercase().contains("configure")
                    || f.path.to_ascii_lowercase().contains("settings")
                    || f.path.to_ascii_lowercase().contains("main.")
                    || f.path.to_ascii_lowercase().contains("preview.")
            })
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
