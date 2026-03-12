use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::domain::context::{
    ChildPageRollup, ModuleContext, ModuleDossier, PageContext, PageDiagramEdge,
    PageDiagramInput, PageDiagramNode, PageEvidenceGroup, PageEvidenceItem,
    PageResearchDiagramRollup, PageResearchEvidenceGroup, PageResearchEvidenceItem,
    RepoContext, SourceSnippet, TopicDossier, TopicSeed,
};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::stable_id::stable_id;
use crate::generation::planner::PlannedPage;
use crate::repo::hierarchy::discover_root_topic_seeds;
use crate::repo::scanner::{FilePurpose, ScanReport};
use crate::repo::symbol_graph::GraphSummary;

/// 构建仓库级上下文。
/// 这一步把扫描事实和模块树压缩成“项目概述 / 系统架构”真正会用到的信息。
///
/// # 参数
/// - `report`：仓库扫描报告。
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 返回供仓库级页面使用的 `RepoContext`。
pub fn build_repo_context(report: &ScanReport, module_tree: &ModuleTree) -> RepoContext {
    build_repo_context_with_graph(report, module_tree, &GraphSummary::default())
}

/// 构建消费 graph summary 的仓库级上下文。
pub fn build_repo_context_with_graph(
    report: &ScanReport,
    module_tree: &ModuleTree,
    graph_summary: &GraphSummary,
) -> RepoContext {
    RepoContext {
        repo_summary_inputs: vec![
            format!("仓库根路径：{}", report.root),
            format!("技术栈：{}", join_or_default(&report.tech_hints, "未识别")),
            format!("文件数量：{}", report.files.len()),
            format!(
                "模块数量：{}",
                module_tree
                    .modules
                    .iter()
                    .filter(|module| module.parent_id.is_some())
                    .count()
            ),
        ],
        top_modules: top_modules(module_tree)
            .into_iter()
            .map(|module| module.id.clone())
            .collect(),
        key_entry_points: report.entry_points.clone(),
        global_relations: module_tree
            .cross_module_edges
            .iter()
            .map(|edge| format!("{} -> {}", edge.source, edge.target))
            .collect(),
        tech_stack: report.tech_hints.clone(),
        graph_hotspots: graph_summary
            .module_call_hotspots
            .values()
            .flat_map(|items| items.iter().cloned())
            .take(12)
            .collect(),
        detected_processes: graph_summary.detected_processes.clone(),
        community_labels: graph_summary
            .communities_by_module
            .values()
            .flat_map(|items| items.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        cycle_warnings: graph_summary.cycle_warnings.clone(),
        root_topics: discover_root_topic_seeds(report, graph_summary),
        process_topics: discover_process_topic_seeds(report, graph_summary),
    }
}

/// 为每个模块构建独立上下文。
/// 模块页后续只消费这里的结果，不直接重新遍历仓库。
///
/// # 参数
/// - `report`：仓库扫描报告。
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 返回每个模块对应的上下文对象列表。
pub fn build_module_contexts(report: &ScanReport, module_tree: &ModuleTree) -> Vec<ModuleContext> {
    build_module_contexts_with_graph(report, module_tree, &GraphSummary::default())
}

/// 为每个模块构建消费 graph summary 的独立上下文。
pub fn build_module_contexts_with_graph(
    report: &ScanReport,
    module_tree: &ModuleTree,
    graph_summary: &GraphSummary,
) -> Vec<ModuleContext> {
    module_tree
        .modules
        .iter()
        .map(|module| {
            let subtree_ids = collect_module_subtree_ids(module_tree, &module.id);
            let dependencies = module_tree
                .cross_module_edges
                .iter()
                .filter(|edge| {
                    subtree_ids.contains(&edge.source) && !subtree_ids.contains(&edge.target)
                })
                .map(|edge| edge.target.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let dependents = module_tree
                .cross_module_edges
                .iter()
                .filter(|edge| {
                    subtree_ids.contains(&edge.target) && !subtree_ids.contains(&edge.source)
                })
                .map(|edge| edge.source.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let key_sources = select_key_sources(report, module);
            let module_root = module
                .root_paths
                .first()
                .cloned()
                .unwrap_or_else(|| ".".to_string());

            ModuleContext {
                module_id: module.id.clone(),
                role_hints: module.tags.clone(),
                public_surface: module.entry_points.clone(),
                dependencies,
                dependents,
                key_sources,
                graph_hotspots: graph_summary
                    .module_call_hotspots
                    .get(&module_root)
                    .cloned()
                    .unwrap_or_default(),
                communities: graph_summary
                    .communities_by_module
                    .get(&module_root)
                    .cloned()
                    .unwrap_or_default(),
                cycle_warnings: graph_summary.cycle_warnings.clone(),
                capability_topics: discover_module_capability_topics(report, module, graph_summary),
            }
        })
        .collect()
}

fn discover_process_topic_seeds(
    report: &ScanReport,
    graph_summary: &GraphSummary,
) -> Vec<TopicSeed> {
    let source_by_path = report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.id.clone()))
        .collect::<BTreeMap<_, _>>();
    let entry_source_ids = report
        .entry_points
        .iter()
        .filter_map(|path| source_by_path.get(path.as_str()).cloned())
        .collect::<Vec<_>>();

    graph_summary
        .detected_processes
        .iter()
        .enumerate()
        .map(|(index, process)| {
            let title = process
                .split(':')
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or("流程主题");
            TopicSeed {
                topic_kind: "process".to_string(),
                topic_key: format!("process-{}", slug_key(title)),
                title: format!("流程主题：{title}"),
                summary: process.clone(),
                source_ids: entry_source_ids.clone(),
                source_paths: report.entry_points.iter().take(6).cloned().collect(),
                module_ids: Vec::new(),
                relation_ids: vec![format!("process:{index}")],
            }
        })
        .collect()
}

fn discover_module_capability_topics(
    report: &ScanReport,
    module: &ModuleNode,
    graph_summary: &GraphSummary,
) -> Vec<TopicSeed> {
    if module.source_ids.len() < 4 {
        return Vec::new();
    }

    let mut by_purpose =
        BTreeMap::<String, (FilePurpose, Vec<&crate::repo::scanner::ScannedFile>)>::new();
    for file in report
        .files
        .iter()
        .filter(|file| module.source_ids.contains(&file.id))
        .filter(|file| file.is_substantive_source())
    {
        by_purpose
            .entry(format!("{:?}", file.purpose))
            .or_insert_with(|| (file.purpose, Vec::new()))
            .1
            .push(file);
    }

    let Some((purpose, files)) = by_purpose
        .into_iter()
        .map(|(_, entry)| entry)
        .filter(|(purpose, files)| purpose.signal_weight() >= 45 && files.len() >= 2)
        .max_by(|left, right| {
            left.1
                .len()
                .cmp(&right.1.len())
                .then(left.0.signal_weight().cmp(&right.0.signal_weight()))
        })
    else {
        return Vec::new();
    };

    let mut selected = files
        .into_iter()
        .map(|file| (file.path.len(), file.path.clone(), file.id.clone()))
        .collect::<Vec<_>>();
    selected.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let selected = selected.into_iter().take(4).collect::<Vec<_>>();
    let module_root = module
        .root_paths
        .first()
        .cloned()
        .unwrap_or_else(|| module.name.clone());
    let hotspots = graph_summary
        .module_call_hotspots
        .get(&module_root)
        .cloned()
        .unwrap_or_default();
    let summary = if hotspots.is_empty() {
        format!(
            "该主题聚焦 `{}` 模块中的{}能力。",
            module.name,
            file_purpose_label(purpose)
        )
    } else {
        format!(
            "该主题聚焦 `{}` 模块中的{}能力，相关热点包括 {}。",
            module.name,
            file_purpose_label(purpose),
            hotspots.into_iter().take(2).collect::<Vec<_>>().join("、")
        )
    };

    vec![TopicSeed {
        topic_kind: "module-capability".to_string(),
        topic_key: format!("{}-{}", slug_key(&module.id), file_purpose_key(purpose)),
        title: format!("{}能力：{}", module.name, file_purpose_label(purpose)),
        summary,
        source_ids: selected.iter().map(|(_, _, id)| id.clone()).collect(),
        source_paths: selected.iter().map(|(_, path, _)| path.clone()).collect(),
        module_ids: vec![module.id.clone()],
        relation_ids: Vec::new(),
    }]
}

fn collect_module_subtree_ids(module_tree: &ModuleTree, module_id: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    collect_module_subtree_ids_recursive(module_tree, module_id, &mut ids);
    ids
}

fn collect_module_subtree_ids_recursive(
    module_tree: &ModuleTree,
    module_id: &str,
    ids: &mut BTreeSet<String>,
) {
    if !ids.insert(module_id.to_string()) {
        return;
    }

    let Some(module) = module_tree.module_by_id(module_id) else {
        return;
    };

    for child_id in &module.child_ids {
        collect_module_subtree_ids_recursive(module_tree, child_id, ids);
    }
}

fn select_key_sources(report: &ScanReport, module: &ModuleNode) -> Vec<String> {
    let source_ids = module.source_ids.iter().cloned().collect::<BTreeSet<_>>();
    let dependency_evidence = report
        .dependency_hints
        .iter()
        .flat_map(|hint| [hint.from.clone(), hint.to.clone()])
        .collect::<BTreeSet<_>>();

    let mut candidates = report
        .files
        .iter()
        .filter(|file| source_ids.contains(&file.id))
        .filter_map(|file| {
            let score = key_source_score(file, module, &dependency_evidence);
            (score > -1000).then(|| (score, file.path.clone()))
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.len().cmp(&right.1.len()))
            .then_with(|| left.1.cmp(&right.1))
    });

    candidates
        .into_iter()
        .map(|(_, path)| path)
        .take(8)
        .collect()
}

fn key_source_score(
    file: &crate::repo::scanner::ScannedFile,
    module: &ModuleNode,
    dependency_evidence: &BTreeSet<String>,
) -> i32 {
    if is_low_signal_key_source(file.path.as_str(), file) {
        return -1000;
    }

    // fixture 路径惩罚：这些文件不应出现在关键源码中
    if is_fixture_path(&file.path) {
        return -1000;
    }

    let mut score = 0;

    // test 路径降权：不完全排除，但显著降低优先级
    if is_test_source_path(&file.path) {
        score -= 50;
    }

    // test-file tag 降权：消费 scanner 阶段的标记
    if file.is_test_like() {
        score -= 30;
    }
    let file_name = Path::new(&file.path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if module.entry_points.contains(&file.path) || file.is_entry_like() {
        score += 100;
    }

    if dependency_evidence.contains(&file.path) {
        score += 70;
    }

    score += file.purpose.signal_weight();

    match file.language.as_str() {
        "typescript" | "javascript" | "react" | "vue" | "svelte" | "python" | "rust" | "java"
        | "csharp" | "kotlin" | "php" | "swift" => score += 25,
        "html" | "css" => score += 5,
        _ => {}
    }

    if module.tags.iter().any(|tag| tag == "frontend")
        && matches!(
            file.language.as_str(),
            "typescript" | "javascript" | "react" | "vue" | "svelte"
        )
    {
        score += 15;
    }

    if module.tags.iter().any(|tag| tag == "backend")
        && matches!(
            file.language.as_str(),
            "python" | "rust" | "java" | "csharp" | "kotlin" | "php" | "swift"
        )
    {
        score += 15;
    }

    if module.tags.iter().any(|tag| tag == "infrastructure") {
        if file.is_config_like() {
            score += 35;
        }

        if file.is_docs_like()
            || file.is_asset_like()
            || matches!(file.language.as_str(), "html" | "css" | "markdown" | "text")
        {
            score -= 35;
        }

        if file.path.contains("/html/") {
            score -= 50;
        }
    }

    if file.path.contains("/src/") || file.path.contains("/app/") || file.path.contains("/modules/")
    {
        score += 15;
    }

    if matches!(
        file_name,
        "app.py"
            | "main.py"
            | "main.ts"
            | "main.js"
            | "main.rs"
            | "nginx.conf"
            | "package.json"
            | "Cargo.toml"
            | "pyproject.toml"
            | "config.yaml"
            | "config.yml"
    ) {
        score += 20;
    }

    if file.path.ends_with(".d.ts") {
        score -= 20;
    }

    score
}

fn is_low_signal_key_source(path: &str, file: &crate::repo::scanner::ScannedFile) -> bool {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if file_name.contains(".log.") || file_name.ends_with(".log") {
        return true;
    }

    if matches!(
        file_name.as_str(),
        "pnpm-lock.yaml"
            | "package-lock.json"
            | "yarn.lock"
            | "cargo.lock"
            | "readme"
            | "readme.md"
    ) {
        return true;
    }

    if path.contains("/openspec/") {
        return true;
    }

    if path.contains("/docs/") {
        return true;
    }

    file.is_low_signal()
}

/// fixture 路径检测：这些目录下的文件不应出现在关键源码列表中。
fn is_fixture_path(path: &str) -> bool {
    let segments: Vec<&str> = path.split('/').collect();
    segments.iter().any(|seg| {
        matches!(
            *seg,
            "fixtures"
                | "__fixtures__"
                | "test-data"
                | "testdata"
                | "test_data"
                | "mock-data"
                | "mocks"
                | "__mocks__"
        )
    })
}

/// test 源码路径检测：用于关键源码评分降权。
fn is_test_source_path(path: &str) -> bool {
    let segments: Vec<&str> = path.split('/').collect();
    segments
        .iter()
        .any(|seg| matches!(*seg, "tests" | "test" | "spec" | "__tests__" | "__test__"))
}

/// 把 `PlannedPage` 进一步转换成渲染器可直接消费的 `PageContext`。
/// 这里是“结构化事实 -> 页面输入”的最后一道转换层。
///
/// # 参数
/// - `page`：当前要生成的页面计划。
/// - `_report`：仓库扫描报告；当前保留这个参数是为了后续扩展。
/// - `module_tree`：当前仓库的模块树。
/// - `repo_context`：仓库级页面上下文。
/// - `module_contexts`：模块级上下文集合。
///
/// # 返回
/// - 返回可直接交给渲染器使用的页面上下文。
pub fn build_page_context(
    page: &PlannedPage,
    report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
) -> PageContext {
    build_page_context_with_inputs(
        page,
        report,
        module_tree,
        repo_context,
        module_contexts,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

/// 构建带页面 hints 和 child summaries 的页面上下文。
/// 这条路径服务迭代 9 的页面增强输入，但 deterministic 渲染仍然只消费稳定 facts。
pub fn build_page_context_with_inputs(
    page: &PlannedPage,
    report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    hints: Vec<String>,
    child_summaries: Vec<String>,
    child_rollups: Vec<ChildPageRollup>,
) -> PageContext {
    let module_context_index = module_contexts
        .iter()
        .map(|context| (context.module_id.clone(), context))
        .collect::<BTreeMap<_, _>>();

    // `facts` 是页面必须稳定出现的硬信息，
    // `summary_inputs` 则是用来组织段落的补充说明输入。
    let mut facts = Vec::new();
    let mut summary_inputs = Vec::new();
    let mut evidence_groups = Vec::new();
    let mut diagram_inputs = Vec::new();

    match page.page_type.as_str() {
        "overview" => {
            facts.extend(repo_context.repo_summary_inputs.clone());
            facts.push(format!(
                "顶层模块：{}",
                join_or_default(
                    &top_modules(module_tree)
                        .into_iter()
                        .map(|module| module.name.clone())
                        .collect::<Vec<_>>(),
                    "无"
                )
            ));
            // 技术栈事实（供"技术栈" section 消费）
            for tech in &repo_context.tech_stack {
                facts.push(format!("技术栈：{tech}"));
            }
            for source in select_repo_core_sources(report) {
                summary_inputs.push(format!("核心源码：{source}"));
            }
            for hotspot in repo_context.graph_hotspots.iter().take(8) {
                summary_inputs.push(format!("图热点：{hotspot}"));
            }
            for process in repo_context.detected_processes.iter().take(6) {
                summary_inputs.push(format!("流程：{process}"));
            }
            for community in repo_context.community_labels.iter().take(8) {
                summary_inputs.push(format!("社区：{community}"));
            }
            for warning in repo_context.cycle_warnings.iter().take(6) {
                summary_inputs.push(format!("循环：{warning}"));
            }
            // 入口与构建事实（供"入口与构建" section 消费）
            for entry in select_display_paths(&repo_context.key_entry_points, 10) {
                summary_inputs.push(format!("入口：{entry}"));
            }
            // 构建命令线索
            for config in &report.config_files {
                let name = config.rsplit('/').next().unwrap_or(config);
                if matches!(name, "Makefile" | "makefile" | "GNUmakefile") {
                    summary_inputs.push(format!("入口：构建工具 {name}"));
                }
            }
            if let Some(group) = build_evidence_group(
                &page.id,
                "关键信息",
                "repo-core-sources",
                "关键来源",
                "这些高信号源码共同支撑仓库概述中的核心判断。",
                select_repo_core_source_records(report)
                    .into_iter()
                    .map(|(source_id, path)| (Some(source_id), path, String::new()))
                    .collect(),
            ) {
                evidence_groups.push(group);
            }
            if let Some(diagram) = build_structure_diagram_for_modules(
                &page.id,
                "关键信息",
                "顶层结构图",
                module_tree,
                &top_level_module_ids(module_tree),
            ) {
                diagram_inputs.push(diagram);
            }
        }
        "architecture" => {
            facts.extend(module_tree.architecture_hints.clone());
            facts.extend(render_module_tree_lines(
                module_tree,
                0,
                &top_level_module_ids(module_tree),
            ));
            // 跨模块关系（供"跨模块关系" section 消费）
            for edge in &module_tree.cross_module_edges {
                summary_inputs.push(format!(
                    "关系：{} -> {} ({})",
                    module_name(module_tree, &edge.source),
                    module_name(module_tree, &edge.target),
                    edge.relation_type
                ));
            }
            // 架构提示（供"架构提示" section 消费）
            for hint in &module_tree.architecture_hints {
                summary_inputs.push(format!("架构：{hint}"));
            }
            if let Some(group) = build_evidence_group(
                &page.id,
                "模块结构",
                "architecture-sources",
                "架构关键来源",
                "这些源码和入口共同支撑当前架构拆分与模块边界。",
                architecture_evidence_items(module_tree, &module_context_index),
            ) {
                evidence_groups.push(group);
            }
            if let Some(diagram) = build_structure_diagram_for_modules(
                &page.id,
                "模块结构",
                "模块层级图",
                module_tree,
                &top_level_module_ids(module_tree),
            ) {
                diagram_inputs.push(diagram);
            }
            if let Some(diagram) = build_dependency_diagram_for_modules(
                &page.id,
                "跨模块关系",
                "模块依赖图",
                module_tree,
                &top_level_module_ids(module_tree),
            ) {
                diagram_inputs.push(diagram);
            }
        }
        "module" => {
            for module_id in &page.module_ids {
                if let Some(module) = module_tree.module_by_id(module_id) {
                    facts.push(format!("模块名称：{}", module.name));
                    facts.push(format!("模块类型：{}", module.kind));
                    facts.push(format!(
                        "模块根路径：{}",
                        join_or_default(&module.root_paths, ".")
                    ));
                    facts.push(format!(
                        "入口文件：{}",
                        join_or_default(&module.entry_points, "无")
                    ));
                    facts.push(format!("源文件数：{}", module.source_ids.len()));

                    if let Some(context) = module_context_index.get(module_id) {
                        summary_inputs.push(format!(
                            "模块角色：{}",
                            join_or_default(&context.role_hints, "未标注")
                        ));
                        // 关键源码（供"关键源码" section 消费）
                        for src in &context.key_sources {
                            summary_inputs.push(format!("源码：{src}"));
                        }
                        // 依赖关系（供"依赖关系" section 消费）
                        for dep in &context.dependencies {
                            summary_inputs
                                .push(format!("依赖：→ {}", module_name(module_tree, dep)));
                        }
                        for dep in &context.dependents {
                            summary_inputs
                                .push(format!("依赖：← {}", module_name(module_tree, dep)));
                        }
                        for hotspot in context.graph_hotspots.iter().take(6) {
                            summary_inputs.push(format!("图热点：{hotspot}"));
                        }
                        for community in context.communities.iter().take(6) {
                            summary_inputs.push(format!("社区：{community}"));
                        }
                        for warning in context.cycle_warnings.iter().take(6) {
                            summary_inputs.push(format!("循环：{warning}"));
                        }
                        if let Some(group) = build_evidence_group(
                            &page.id,
                            "关键源码",
                            &format!("module-key-sources:{module_id}"),
                            "关键来源",
                            "这些高信号源码最能代表当前模块的职责和公开表面。",
                            context
                                .key_sources
                                .iter()
                                .map(|path| {
                                    (
                                        source_id_for_path(report, path),
                                        path.clone(),
                                        String::new(),
                                    )
                                })
                                .collect(),
                        ) {
                            evidence_groups.push(group);
                        }
                    }
                    if let Some(diagram) = build_dependency_diagram_for_modules(
                        &page.id,
                        "依赖关系",
                        "模块依赖图",
                        module_tree,
                        std::slice::from_ref(module_id),
                    ) {
                        diagram_inputs.push(diagram);
                    }
                    if let Some(diagram) = build_structure_diagram_for_modules(
                        &page.id,
                        "子模块概述",
                        "子模块结构图",
                        module_tree,
                        std::slice::from_ref(module_id),
                    ) {
                        diagram_inputs.push(diagram);
                    }
                }
            }
            // 子模块概述（供"子模块概述" section 消费）
            for merged_id in &page.merged_module_ids {
                if let Some(merged_module) = module_tree.module_by_id(merged_id) {
                    summary_inputs.push(format!(
                        "子模块：{} ({}，{} 个文件)",
                        merged_module.name,
                        merged_module.kind,
                        merged_module.source_ids.len()
                    ));
                }
            }
        }
        "workflow" => {
            facts.push("页面类型：工作流与部署".to_string());
            for process in &repo_context.detected_processes {
                facts.push(format!("构建：检测到流程 {process}"));
            }
            for warning in &repo_context.cycle_warnings {
                facts.push(format!("构建：循环提示 {warning}"));
            }
            // 按类别分类工作流文件
            for file in &report.files {
                let name = file.path.rsplit('/').next().unwrap_or(&file.path);
                if file.path.starts_with(".github/workflows/") {
                    facts.push(format!("CI：GitHub Actions - {}", file.path));
                } else if file.path.starts_with(".gitlab/") || name == ".gitlab-ci.yml" {
                    facts.push(format!("CI：GitLab CI - {}", file.path));
                } else if file.path.starts_with(".circleci/") {
                    facts.push(format!("CI：CircleCI - {}", file.path));
                } else if name == ".travis.yml" {
                    facts.push(format!("CI：Travis CI - {}", file.path));
                } else if name == "Jenkinsfile" {
                    facts.push(format!("CI：Jenkins - {}", file.path));
                } else if matches!(name, "Makefile" | "makefile" | "GNUmakefile") {
                    facts.push(format!("构建：{}", file.path));
                } else if name == "Dockerfile" || name.starts_with("docker-compose") {
                    facts.push(format!("容器：{}", file.path));
                }
            }
            let workflow_items = report
                .files
                .iter()
                .filter(|file| is_workflow_support_file(file.path.as_str()))
                .map(|file| (Some(file.id.clone()), file.path.clone(), String::new()))
                .collect::<Vec<_>>();
            if let Some(group) = build_evidence_group(
                &page.id,
                "构建流程",
                "workflow-files",
                "工作流来源",
                "这些配置文件直接支撑构建、CI/CD 或容器化说明。",
                workflow_items,
            ) {
                evidence_groups.push(group);
            }
            if let Some(diagram) = repo_context.detected_processes.first().and_then(|process| {
                build_process_diagram(&page.id, "工作流概述", "流程图", process)
            }) {
                diagram_inputs.push(diagram);
            }
        }
        "topic" => {
            let topic_seed = lookup_topic_seed(page, repo_context, module_contexts);
            let topic_summary = page
                .topic_summary
                .as_ref()
                .filter(|summary| !summary.is_empty())
                .cloned()
                .or_else(|| topic_seed.as_ref().map(|seed| seed.summary.clone()))
                .unwrap_or_else(|| "当前主题由 deterministic planner 生成。".to_string());
            let topic_kind = page
                .topic_kind
                .clone()
                .or_else(|| topic_seed.as_ref().map(|seed| seed.topic_kind.clone()))
                .unwrap_or_else(|| "topic".to_string());

            facts.push(format!("主题类别：{topic_kind}"));
            facts.push(format!("主题标题：{}", page.title));
            facts.push(format!("关联源码数：{}", page.source_ids.len()));
            summary_inputs.push(format!("主题摘要：{topic_summary}"));
            for module_id in &page.module_ids {
                summary_inputs.push(format!("关联模块：{}", module_name(module_tree, module_id)));
            }
            for source_path in topic_seed
                .as_ref()
                .map(|seed| seed.source_paths.clone())
                .unwrap_or_default()
                .into_iter()
                .take(6)
            {
                summary_inputs.push(format!("关键源码：{source_path}"));
            }
            if let Some(seed) = topic_seed {
                if let Some(group) = build_evidence_group(
                    &page.id,
                    "关键证据",
                    &format!("topic-evidence:{}", seed.topic_key),
                    "关键来源",
                    "这些 evidence 直接支撑当前专题页的主题判断。",
                    seed.source_paths
                        .iter()
                        .zip(
                            seed.source_ids
                                .iter()
                                .cloned()
                                .chain(std::iter::repeat(String::new())),
                        )
                        .take(8)
                        .map(|(path, source_id)| {
                            (
                                (!source_id.is_empty()).then_some(source_id),
                                path.clone(),
                                String::new(),
                            )
                        })
                        .collect(),
                ) {
                    evidence_groups.push(group);
                }
                if seed.topic_kind == "process" {
                    if let Some(diagram) =
                        build_process_diagram(&page.id, "结构图", "流程图", &seed.summary)
                    {
                        diagram_inputs.push(diagram);
                    }
                } else if !page.module_ids.is_empty() {
                    if let Some(diagram) = build_dependency_diagram_for_modules(
                        &page.id,
                        "关联模块",
                        "关联模块图",
                        module_tree,
                        &page.module_ids,
                    ) {
                        diagram_inputs.push(diagram);
                    }
                }
            }
        }
        _ => {
            facts.push(format!("页面类型：{}", page.page_type));
            summary_inputs.push("由 codebuddy-wiki 自动生成。".to_string());
        }
    }

    if summary_inputs.is_empty() {
        summary_inputs.push("由 codebuddy-wiki 自动生成。".to_string());
    }
    facts = dedupe_lines(facts);
    summary_inputs = dedupe_lines(summary_inputs);
    let module_dossiers = build_module_dossiers(
        page,
        report,
        module_tree,
        &module_context_index,
        &child_rollups,
        &evidence_groups,
        &diagram_inputs,
    );
    let topic_dossier = build_topic_dossier(
        page,
        report,
        repo_context,
        module_contexts,
        &child_rollups,
        &evidence_groups,
        &diagram_inputs,
    );

    PageContext {
        page_id: page.id.clone(),
        page_type: page.page_type.clone(),
        scope: page.scope.clone(),
        source_ids: page.source_ids.clone(),
        module_ids: page.module_ids.clone(),
        relation_ids: page.relation_ids.clone(),
        facts,
        summary_inputs,
        hints,
        child_summaries,
        child_rollups,
        evidence_groups,
        diagram_inputs,
        module_dossiers,
        topic_dossier,
        research_result: None,
        research_session: None,
    }
}

fn build_module_dossiers(
    page: &PlannedPage,
    report: &ScanReport,
    module_tree: &ModuleTree,
    module_context_index: &BTreeMap<String, &ModuleContext>,
    child_rollups: &[ChildPageRollup],
    evidence_groups: &[PageEvidenceGroup],
    diagram_inputs: &[PageDiagramInput],
) -> Vec<ModuleDossier> {
    page.module_ids
        .iter()
        .filter_map(|module_id| {
            let module = module_tree.module_by_id(module_id)?;
            let context = module_context_index.get(module_id)?;
            let key_sources = context.key_sources.clone();
            let title = format!("{} 研究包", module.name);

            Some(ModuleDossier {
                dossier_id: stable_id("module-dossier", module_id),
                module_id: module_id.clone(),
                title,
                key_sources: key_sources.clone(),
                key_symbols: Vec::new(),
                source_snippets: key_sources
                    .iter()
                    .filter_map(|path| read_source_snippet(report, path))
                    .collect(),
                cross_module_edges: context
                    .dependencies
                    .iter()
                    .map(|dep| format!("依赖 -> {}", module_name(module_tree, dep)))
                    .chain(
                        context
                            .dependents
                            .iter()
                            .map(|dep| format!("被依赖 <- {}", module_name(module_tree, dep))),
                    )
                    .collect(),
                process_candidates: context
                    .graph_hotspots
                    .iter()
                    .take(4)
                    .cloned()
                    .collect(),
                evidence_rollup: evidence_groups_to_research_rollup(evidence_groups),
                diagram_rollup: diagram_inputs_to_research_rollup(diagram_inputs),
                child_page_rollup: child_rollups.to_vec(),
            })
        })
        .collect()
}

fn build_topic_dossier(
    page: &PlannedPage,
    report: &ScanReport,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    child_rollups: &[ChildPageRollup],
    evidence_groups: &[PageEvidenceGroup],
    diagram_inputs: &[PageDiagramInput],
) -> Option<TopicDossier> {
    if page.page_type != "topic" {
        return None;
    }

    let seed = lookup_topic_seed(page, repo_context, module_contexts)?;
    let key_sources = seed.source_paths.clone();
    Some(TopicDossier {
        dossier_id: stable_id("topic-dossier", &seed.topic_key),
        topic_key: seed.topic_key.clone(),
        topic_kind: seed.topic_kind.clone(),
        title: seed.title.clone(),
        summary: seed.summary.clone(),
        key_sources: key_sources.clone(),
        key_symbols: Vec::new(),
        source_snippets: key_sources
            .iter()
            .filter_map(|path| read_source_snippet(report, path))
            .collect(),
        evidence_rollup: evidence_groups_to_research_rollup(evidence_groups),
        diagram_rollup: diagram_inputs_to_research_rollup(diagram_inputs),
        child_page_rollup: child_rollups.to_vec(),
    })
}

fn evidence_groups_to_research_rollup(
    evidence_groups: &[PageEvidenceGroup],
) -> Vec<PageResearchEvidenceGroup> {
    evidence_groups
        .iter()
        .map(|group| PageResearchEvidenceGroup {
            group_key: group.group_id.clone(),
            title: group.title.clone(),
            items: group
                .items
                .iter()
                .map(|item| PageResearchEvidenceItem {
                    source_id: item.source_id.clone(),
                    path: item.path.clone(),
                    start_line: 0,
                    end_line: 0,
                    note: item.note.clone(),
                })
                .collect(),
        })
        .collect()
}

fn diagram_inputs_to_research_rollup(
    diagram_inputs: &[PageDiagramInput],
) -> Vec<PageResearchDiagramRollup> {
    diagram_inputs
        .iter()
        .map(|diagram| PageResearchDiagramRollup {
            diagram_key: diagram.diagram_id.clone(),
            diagram_type: normalize_diagram_type(&diagram.diagram_type),
            title: diagram.title.clone(),
            summary: diagram.summary.clone(),
        })
        .collect()
}

fn normalize_diagram_type(diagram_type: &str) -> String {
    match diagram_type {
        "dependency" => "module_dependency".to_string(),
        "structure" => "hierarchy".to_string(),
        "flow" => "process".to_string(),
        other => other.to_string(),
    }
}

fn read_source_snippet(report: &ScanReport, path: &str) -> Option<SourceSnippet> {
    let source_id = source_id_for_path(report, path);
    let absolute_path = Path::new(&report.root).join(path);
    let content = fs::read_to_string(&absolute_path).ok()?;
    let lines = content.lines().collect::<Vec<_>>();
    let snippet_lines = lines
        .iter()
        .take(24)
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<_>>();
    let content = snippet_lines.join("\n").trim().to_string();
    if content.is_empty() {
        return None;
    }

    Some(SourceSnippet {
        source_id,
        path: path.to_string(),
        start_line: 1,
        end_line: snippet_lines.len(),
        content,
    })
}

fn lookup_topic_seed(
    page: &PlannedPage,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
) -> Option<TopicSeed> {
    let topic_key = page.topic_key.as_deref()?;

    repo_context
        .root_topics
        .iter()
        .chain(repo_context.process_topics.iter())
        .find(|seed| seed.topic_key == topic_key)
        .cloned()
        .or_else(|| {
            module_contexts
                .iter()
                .flat_map(|context| context.capability_topics.iter())
                .find(|seed| seed.topic_key == topic_key)
                .cloned()
        })
}

fn source_id_for_path(report: &ScanReport, path: &str) -> Option<String> {
    report
        .files
        .iter()
        .find(|file| file.path == path)
        .map(|file| file.id.clone())
}

fn architecture_evidence_items(
    module_tree: &ModuleTree,
    module_context_index: &BTreeMap<String, &ModuleContext>,
) -> Vec<(Option<String>, String, String)> {
    top_level_module_ids(module_tree)
        .into_iter()
        .filter_map(|module_id| {
            let context = module_context_index.get(&module_id)?;
            let key_source = context.key_sources.first()?.clone();
            Some((
                None,
                key_source,
                format!("对应模块 {}", module_name(module_tree, &module_id)),
            ))
        })
        .take(8)
        .collect()
}

fn build_evidence_group(
    page_id: &str,
    section_title: &str,
    group_key: &str,
    title: &str,
    summary: &str,
    items: Vec<(Option<String>, String, String)>,
) -> Option<PageEvidenceGroup> {
    let mut deduped = Vec::new();
    let mut seen = BTreeSet::new();

    for (source_id, path, note) in items {
        let normalized = path.trim().to_string();
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        deduped.push(PageEvidenceItem {
            evidence_id: stable_id(
                "evidence",
                &format!("{page_id}:{section_title}:{group_key}:{normalized}"),
            ),
            label: normalized.clone(),
            path: normalized,
            source_id,
            note,
        });
    }

    (!deduped.is_empty()).then(|| PageEvidenceGroup {
        group_id: stable_id(
            "evidence-group",
            &format!("{page_id}:{section_title}:{group_key}"),
        ),
        section_title: section_title.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        items: deduped,
    })
}

fn build_structure_diagram_for_modules(
    page_id: &str,
    section_title: &str,
    title: &str,
    module_tree: &ModuleTree,
    module_ids: &[String],
) -> Option<PageDiagramInput> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen_nodes = BTreeSet::new();

    for module_id in module_ids {
        let Some(module) = module_tree.module_by_id(module_id) else {
            continue;
        };
        if seen_nodes.insert(module.id.clone()) {
            nodes.push(PageDiagramNode {
                node_id: module.id.clone(),
                label: module.name.clone(),
            });
        }

        for child_id in &module.child_ids {
            let Some(child) = module_tree.module_by_id(child_id) else {
                continue;
            };
            if seen_nodes.insert(child.id.clone()) {
                nodes.push(PageDiagramNode {
                    node_id: child.id.clone(),
                    label: child.name.clone(),
                });
            }
            edges.push(PageDiagramEdge {
                source: module.id.clone(),
                target: child.id.clone(),
                label: Some("contains".to_string()),
            });
        }
    }

    (!nodes.is_empty() && !edges.is_empty()).then(|| PageDiagramInput {
        diagram_id: stable_id("diagram", &format!("{page_id}:{section_title}:structure")),
        section_title: section_title.to_string(),
        diagram_type: "structure".to_string(),
        title: title.to_string(),
        summary: "图结构来自模块树父子关系。".to_string(),
        nodes,
        edges,
    })
}

fn build_dependency_diagram_for_modules(
    page_id: &str,
    section_title: &str,
    title: &str,
    module_tree: &ModuleTree,
    focus_module_ids: &[String],
) -> Option<PageDiagramInput> {
    let focus = focus_module_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen_nodes = BTreeSet::new();

    for edge in &module_tree.cross_module_edges {
        if !focus.is_empty() && !focus.contains(&edge.source) && !focus.contains(&edge.target) {
            continue;
        }
        let Some(source) = module_tree.module_by_id(&edge.source) else {
            continue;
        };
        let Some(target) = module_tree.module_by_id(&edge.target) else {
            continue;
        };
        if seen_nodes.insert(source.id.clone()) {
            nodes.push(PageDiagramNode {
                node_id: source.id.clone(),
                label: source.name.clone(),
            });
        }
        if seen_nodes.insert(target.id.clone()) {
            nodes.push(PageDiagramNode {
                node_id: target.id.clone(),
                label: target.name.clone(),
            });
        }
        edges.push(PageDiagramEdge {
            source: source.id.clone(),
            target: target.id.clone(),
            label: Some(edge.relation_type.clone()),
        });
    }

    (!nodes.is_empty() && !edges.is_empty()).then(|| PageDiagramInput {
        diagram_id: stable_id("diagram", &format!("{page_id}:{section_title}:dependency")),
        section_title: section_title.to_string(),
        diagram_type: "dependency".to_string(),
        title: title.to_string(),
        summary: "图结构来自 cross-module edges。".to_string(),
        nodes,
        edges,
    })
}

fn build_process_diagram(
    page_id: &str,
    section_title: &str,
    title: &str,
    process_summary: &str,
) -> Option<PageDiagramInput> {
    let steps = parse_process_steps(process_summary);
    if steps.len() < 3 {
        return None;
    }

    let nodes = steps
        .iter()
        .enumerate()
        .map(|(index, step)| PageDiagramNode {
            node_id: stable_id(
                "diagram-node",
                &format!("{page_id}:{section_title}:{index}:{step}"),
            ),
            label: step.clone(),
        })
        .collect::<Vec<_>>();
    let edges = nodes
        .windows(2)
        .map(|pair| PageDiagramEdge {
            source: pair[0].node_id.clone(),
            target: pair[1].node_id.clone(),
            label: None,
        })
        .collect::<Vec<_>>();

    Some(PageDiagramInput {
        diagram_id: stable_id("diagram", &format!("{page_id}:{section_title}:flow")),
        section_title: section_title.to_string(),
        diagram_type: "flow".to_string(),
        title: title.to_string(),
        summary: process_summary.to_string(),
        nodes,
        edges,
    })
}

fn parse_process_steps(process_summary: &str) -> Vec<String> {
    let trace = process_summary.split(':').nth(1).unwrap_or(process_summary);
    trace
        .split("->")
        .map(str::trim)
        .filter(|step| !step.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_workflow_support_file(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    path.starts_with(".github/workflows/")
        || path.starts_with(".gitlab/")
        || path.starts_with(".circleci/")
        || matches!(
            name,
            "Makefile"
                | "makefile"
                | "GNUmakefile"
                | "Dockerfile"
                | "docker-compose.yml"
                | "docker-compose.yaml"
                | "Jenkinsfile"
                | ".gitlab-ci.yml"
                | ".travis.yml"
        )
}

fn file_purpose_label(purpose: FilePurpose) -> &'static str {
    match purpose {
        FilePurpose::Entry => "入口编排",
        FilePurpose::Router => "路由",
        FilePurpose::Controller | FilePurpose::Handler => "处理流程",
        FilePurpose::Service => "服务协作",
        FilePurpose::Repository => "数据访问",
        FilePurpose::Domain | FilePurpose::Model => "领域建模",
        FilePurpose::Middleware => "中间件",
        FilePurpose::Component | FilePurpose::Widget | FilePurpose::Page | FilePurpose::Layout => {
            "界面组成"
        }
        FilePurpose::Library | FilePurpose::Plugin => "扩展机制",
        _ => "核心实现",
    }
}

fn file_purpose_key(purpose: FilePurpose) -> &'static str {
    match purpose {
        FilePurpose::Entry => "entry",
        FilePurpose::Router => "router",
        FilePurpose::Controller => "controller",
        FilePurpose::Handler => "handler",
        FilePurpose::Service => "service",
        FilePurpose::Repository => "repository",
        FilePurpose::Domain => "domain",
        FilePurpose::Model => "model",
        FilePurpose::Middleware => "middleware",
        FilePurpose::Library => "library",
        FilePurpose::Plugin => "plugin",
        FilePurpose::Component => "component",
        FilePurpose::Widget => "widget",
        FilePurpose::Page => "page",
        FilePurpose::Layout => "layout",
        _ => "core",
    }
}

fn slug_key(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => character,
            ' ' | '/' | '\\' | ':' => '-',
            _ if character.is_ascii_alphanumeric() => character,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_ascii_lowercase()
}

/// 找到顶层业务模块。
/// 这里的“顶层”指父级直接是根仓库模块，而不是所有非根模块。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 返回所有顶层业务模块的引用列表。
fn top_modules<'a>(module_tree: &'a ModuleTree) -> Vec<&'a ModuleNode> {
    module_tree
        .modules
        .iter()
        .filter(|module| {
            module.parent_id.as_deref() == module_tree.root_modules.first().map(String::as_str)
        })
        .collect()
}

/// 通过模块 ID 取展示名称，避免页面里泄漏稳定 ID。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
/// - `module_id`：待查找的模块稳定 ID。
///
/// # 返回
/// - 返回适合页面展示的模块名称。
fn module_name(module_tree: &ModuleTree, module_id: &str) -> String {
    module_tree
        .module_by_id(module_id)
        .map(|module| module.name.clone())
        .unwrap_or_else(|| module_id.to_string())
}

fn top_level_module_ids(module_tree: &ModuleTree) -> Vec<String> {
    module_tree
        .root_modules
        .iter()
        .filter_map(|module_id| module_tree.module_by_id(module_id))
        .flat_map(|module| module.child_ids.iter().cloned())
        .collect()
}

/// 把模块树递归展开成可写入 Markdown 的缩进列表。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
/// - `depth`：当前递归深度。
/// - `module_ids`：当前层级要展开的模块 ID 列表。
///
/// # 返回
/// - 返回可直接写入 Markdown 的缩进列表行集合。
fn render_module_tree_lines(
    module_tree: &ModuleTree,
    depth: usize,
    module_ids: &[String],
) -> Vec<String> {
    let mut lines = Vec::new();

    for module_id in module_ids {
        if let Some(module) = module_tree.module_by_id(module_id) {
            lines.push(format!("模块树：{depth}:{}", module.name));
            lines.extend(render_module_tree_lines(
                module_tree,
                depth + 1,
                &module.child_ids,
            ));
        }
    }

    lines
}

/// 把字符串列表拼接成自然语言。
///
/// # 参数
/// - `values`：待拼接的字符串列表。
/// - `fallback`：列表为空时使用的兜底文案。
///
/// # 返回
/// - 返回适合页面展示的中文拼接结果。
fn join_or_default(values: &[String], fallback: &str) -> String {
    if values.is_empty() {
        fallback.to_string()
    } else {
        values.join("、")
    }
}

fn dedupe_lines(lines: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();

    for line in lines {
        let normalized = line.trim().to_string();
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        deduped.push(normalized);
    }

    deduped
}

fn select_display_paths(paths: &[String], limit: usize) -> Vec<String> {
    let mut primary = Vec::new();
    let mut secondary = Vec::new();
    let mut seen = BTreeSet::new();

    for path in paths {
        if !seen.insert(path.clone()) {
            continue;
        }

        let file_name = Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        let is_primary = matches!(
            file_name,
            "main.rs"
                | "main.go"
                | "main.py"
                | "main.ts"
                | "main.js"
                | "main.kt"
                | "main.swift"
                | "Program.cs"
                | "Main.java"
                | "__main__.py"
                | "package.json"
                | "Cargo.toml"
                | "pyproject.toml"
                | "go.mod"
        );

        if is_primary {
            primary.push(path.clone());
        } else {
            secondary.push(path.clone());
        }
    }

    primary.into_iter().chain(secondary).take(limit).collect()
}

fn select_repo_core_sources(report: &ScanReport) -> Vec<String> {
    let mut candidates = report
        .files
        .iter()
        .filter(|file| !file.path.contains('/'))
        .filter(|file| file.is_substantive_source())
        .map(|file| {
            (
                file.purpose.signal_weight(),
                file.path.len(),
                file.path.clone(),
            )
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    candidates
        .into_iter()
        .map(|(_, _, path)| path)
        .take(8)
        .collect()
}

fn select_repo_core_source_records(report: &ScanReport) -> Vec<(String, String)> {
    let selected = select_repo_core_sources(report)
        .into_iter()
        .collect::<BTreeSet<_>>();

    report
        .files
        .iter()
        .filter(|file| selected.contains(&file.path))
        .map(|file| (file.id.clone(), file.path.clone()))
        .collect()
}
