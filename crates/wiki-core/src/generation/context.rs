use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::context::{ModuleContext, PageContext, RepoContext};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::generation::planner::PlannedPage;
use crate::repo::scanner::ScanReport;

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
    RepoContext {
        repo_summary_inputs: vec![
            format!("仓库根路径：{}", report.root),
            format!("技术栈：{}", join_or_default(&report.tech_hints, "未识别")),
            format!("文件数量：{}", report.files.len()),
            format!("模块数量：{}", module_tree.modules.len()),
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
    module_tree
        .modules
        .iter()
        .map(|module| {
            let subtree_ids = collect_module_subtree_ids(module_tree, &module.id);
            let dependencies = module_tree
                .cross_module_edges
                .iter()
                .filter(|edge| subtree_ids.contains(&edge.source) && !subtree_ids.contains(&edge.target))
                .map(|edge| edge.target.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let dependents = module_tree
                .cross_module_edges
                .iter()
                .filter(|edge| subtree_ids.contains(&edge.target) && !subtree_ids.contains(&edge.source))
                .map(|edge| edge.source.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let key_sources = select_key_sources(report, module);

            ModuleContext {
                module_id: module.id.clone(),
                role_hints: module.tags.clone(),
                public_surface: module.entry_points.clone(),
                dependencies,
                dependents,
                key_sources,
            }
        })
        .collect()
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
    if is_low_signal_key_source(file.path.as_str(), file.kind.as_str()) {
        return -1000;
    }

    let mut score = 0;
    let file_name = Path::new(&file.path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if module.entry_points.contains(&file.path) || file.tags.iter().any(|tag| tag == "entry-point") {
        score += 100;
    }

    if dependency_evidence.contains(&file.path) {
        score += 70;
    }

    match file.kind.as_str() {
        "source" => score += 40,
        "config" => score += 10,
        "docs" => score -= 30,
        "asset" => score -= 50,
        _ => {}
    }

    match file.language.as_str() {
        "typescript"
        | "javascript"
        | "react"
        | "vue"
        | "svelte"
        | "python"
        | "rust"
        | "java"
        | "csharp"
        | "kotlin"
        | "php"
        | "swift" => score += 25,
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
        if file.kind == "config" {
            score += 35;
        }

        if matches!(file.language.as_str(), "html" | "css" | "markdown" | "text") {
            score -= 35;
        }

        if file.path.contains("/html/") {
            score -= 50;
        }
    }

    if file.path.contains("/src/") || file.path.contains("/app/") || file.path.contains("/modules/") {
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

fn is_low_signal_key_source(path: &str, kind: &str) -> bool {
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
        "pnpm-lock.yaml" | "package-lock.json" | "yarn.lock" | "cargo.lock" | "readme" | "readme.md"
    ) {
        return true;
    }

    if path.contains("/openspec/") {
        return true;
    }

    if path.contains("/docs/") {
        return true;
    }

    kind == "docs" || kind == "asset"
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
    _report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
) -> PageContext {
    let module_context_index = module_contexts
        .iter()
        .map(|context| (context.module_id.clone(), context))
        .collect::<BTreeMap<_, _>>();

    // `facts` 是页面必须稳定出现的硬信息，
    // `summary_inputs` 则是用来组织段落的补充说明输入。
    let mut facts = Vec::new();
    let mut summary_inputs = Vec::new();

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
            summary_inputs.extend(
                repo_context
                    .key_entry_points
                    .iter()
                    .map(|entry| format!("入口：{entry}")),
            );
        }
        "architecture" => {
            facts.extend(module_tree.architecture_hints.clone());
            facts.extend(render_module_tree_lines(module_tree, 0, &module_tree.root_modules));
            summary_inputs.extend(
                module_tree
                    .cross_module_edges
                    .iter()
                    .map(|edge| {
                        format!(
                            "跨模块关系：{} -> {}",
                            module_name(module_tree, &edge.source),
                            module_name(module_tree, &edge.target)
                        )
                    }),
            );
        }
        "module" => {
            for module_id in &page.module_ids {
                if let Some(module) = module_tree.module_by_id(module_id) {
                    facts.push(format!("模块名称：{}", module.name));
                    facts.push(format!(
                        "模块根路径：{}",
                        join_or_default(&module.root_paths, ".")
                    ));
                    facts.push(format!(
                        "入口文件：{}",
                        join_or_default(&module.entry_points, "无")
                    ));

                    if let Some(context) = module_context_index.get(module_id) {
                        summary_inputs.push(format!(
                            "模块角色：{}",
                            join_or_default(&context.role_hints, "未标注")
                        ));
                        summary_inputs.push(format!(
                            "关键源码：{}",
                            join_or_default(&context.key_sources, "无")
                        ));
                        summary_inputs.push(format!(
                            "依赖模块：{}",
                            join_or_default(
                                &context
                                    .dependencies
                                    .iter()
                                    .map(|dependency| module_name(module_tree, dependency))
                                    .collect::<Vec<_>>(),
                                "无"
                            )
                        ));
                        summary_inputs.push(format!(
                            "被依赖模块：{}",
                            join_or_default(
                                &context
                                    .dependents
                                    .iter()
                                    .map(|dependent| module_name(module_tree, dependent))
                                    .collect::<Vec<_>>(),
                                "无"
                            )
                        ));
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

    PageContext {
        page_id: page.id.clone(),
        page_type: page.page_type.clone(),
        scope: page.scope.clone(),
        source_ids: page.source_ids.clone(),
        module_ids: page.module_ids.clone(),
        relation_ids: page.relation_ids.clone(),
        facts,
        summary_inputs,
    }
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
        .filter(|module| module.parent_id.as_deref() == module_tree.root_modules.first().map(String::as_str))
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
            let indent = "  ".repeat(depth);
            lines.push(format!("{indent}- {}", module.name));
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
