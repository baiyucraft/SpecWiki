use std::collections::BTreeSet;

use crate::domain::context::{ModuleContext, RepoContext};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::stable_id::stable_id;
use crate::repo::scanner::ScanReport;

/// `PlannedPage` 描述“要生成什么页面”。
/// 这一层只决定页面结构和依赖范围，还不负责具体写出 Markdown 内容。
#[derive(Debug, Clone)]
pub struct PlannedPage {
    pub id: String,
    pub title: String,
    pub relative_path: String,
    pub page_type: String,
    pub parent_id: Option<String>,
    pub scope: String,
    pub source_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub relation_ids: Vec<String>,
    pub generation_mode: String,
    pub priority: usize,
}

/// 根据模块树和上下文规划页面集合。
/// 当前固定生成：项目概述、系统架构、以及每个业务模块对应的模块页。
///
/// # 参数
/// - `report`：仓库扫描报告。
/// - `module_tree`：当前仓库的模块树。
/// - `repo_context`：仓库级页面上下文。
/// - `module_contexts`：模块级上下文集合。
///
/// # 返回
/// - 返回当前仓库需要生成的页面计划列表。
pub fn plan_pages(
    report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
) -> Vec<PlannedPage> {
    let overview_id = stable_id("page", "overview");
    let architecture_id = stable_id("page", "architecture");

    // 概述页总是根页面，后续其他页面默认挂到它下面。
    let mut pages = vec![PlannedPage {
        id: overview_id.clone(),
        title: "项目概述".to_string(),
        relative_path: "项目概述.md".to_string(),
        page_type: "overview".to_string(),
        parent_id: None,
        scope: "repository".to_string(),
        source_ids: report.files.iter().map(|file| file.id.clone()).collect(),
        module_ids: repo_context.top_modules.clone(),
        relation_ids: module_tree
            .cross_module_edges
            .iter()
            .map(|edge| edge.id.clone())
            .collect(),
        generation_mode: "deterministic".to_string(),
        priority: 0,
    }];

    // 架构页与概述页并列存在，但在层级上作为概述页的直接子页面。
    pages.push(PlannedPage {
        id: architecture_id.clone(),
        title: "系统架构".to_string(),
        relative_path: "系统架构.md".to_string(),
        page_type: "architecture".to_string(),
        parent_id: Some(overview_id.clone()),
        scope: "architecture".to_string(),
        source_ids: report.files.iter().map(|file| file.id.clone()).collect(),
        module_ids: repo_context.top_modules.clone(),
        relation_ids: module_tree
            .cross_module_edges
            .iter()
            .map(|edge| edge.id.clone())
            .collect(),
        generation_mode: "deterministic".to_string(),
        priority: 1,
    });

    let modules_to_render = modules_to_render(module_tree);
    let module_context_index = module_contexts
        .iter()
        .map(|context| (context.module_id.clone(), context))
        .collect::<std::collections::BTreeMap<_, _>>();

    // 模块页规划时会保留页面层级，这样后续如果出现嵌套模块，可以自然落成目录结构。
    for (index, module) in modules_to_render.into_iter().enumerate() {
        let path = module_page_path(module_tree, module);
        let parent_id = module
            .parent_id
            .as_ref()
            .and_then(|parent_module_id| {
                if module_tree.root_modules.contains(parent_module_id) {
                    Some(overview_id.clone())
                } else {
                    Some(stable_id("page", &module_page_path(
                        module_tree,
                        module_tree
                            .module_by_id(parent_module_id)
                            .expect("parent module should exist"),
                    )))
                }
            })
            .or_else(|| Some(overview_id.clone()));

        let source_ids = module.source_ids.clone();
        let relation_ids = module_tree
            .cross_module_edges
            .iter()
            .filter(|edge| edge.source == module.id || edge.target == module.id)
            .map(|edge| edge.id.clone())
            .collect::<Vec<_>>();
        let title = format!("模块：{}", module.name);
        let summary_hint = module_context_index
            .get(&module.id)
            .map(|context| context.role_hints.join("、"))
            .filter(|hint| !hint.is_empty())
            .unwrap_or_else(|| "模块".to_string());

        pages.push(PlannedPage {
            id: stable_id("page", &path),
            title,
            relative_path: path,
            page_type: "module".to_string(),
            parent_id,
            scope: format!("module:{}", module.id),
            source_ids,
            module_ids: vec![module.id.clone()],
            relation_ids,
            generation_mode: format!("deterministic:{summary_hint}"),
            priority: 10 + index,
        });
    }

    pages
}

/// 如果存在真正的业务子模块，就只为这些子模块生成模块页；
/// 否则退回到根模块，至少保证最小仓库也能拿到一个模块页。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 返回应该被实际渲染为模块页的模块列表。
fn modules_to_render<'a>(module_tree: &'a ModuleTree) -> Vec<&'a ModuleNode> {
    let non_root_modules = module_tree.non_root_modules();

    if non_root_modules.is_empty() {
        return module_tree
            .root_modules
            .iter()
            .filter_map(|module_id| module_tree.module_by_id(module_id))
            .collect();
    }

    non_root_modules
}

/// 根据模块祖先链生成模块页路径。
/// 这样后续支持子模块时，目录结构也能自然跟着模块树展开。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
/// - `module`：当前要生成页面路径的模块。
///
/// # 返回
/// - 返回当前模块页在 `.wiki/` 下的相对路径。
fn module_page_path(module_tree: &ModuleTree, module: &ModuleNode) -> String {
    let mut segments = module_ancestry(module_tree, module);
    segments.push(slugify(&module.name));
    format!("核心模块/{}.md", segments.join("/"))
}

/// 收集模块的祖先名称，用来构造嵌套页面路径。
///
/// # 参数
/// - `module_tree`：当前仓库的模块树。
/// - `module`：当前模块节点。
///
/// # 返回
/// - 返回当前模块所有祖先模块的 slug 列表。
fn module_ancestry(module_tree: &ModuleTree, module: &ModuleNode) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current_parent = module.parent_id.clone();

    while let Some(parent_id) = current_parent {
        if module_tree.root_modules.contains(&parent_id) {
            break;
        }

        let Some(parent) = module_tree.module_by_id(&parent_id) else {
            break;
        };

        segments.push(slugify(&parent.name));
        current_parent = parent.parent_id.clone();
    }

    segments.reverse();
    segments
}

/// 文件名规范化，保证模块页路径在 Windows 上也可安全落盘。
///
/// # 参数
/// - `value`：待规范化的原始名称。
///
/// # 返回
/// - 返回适合当作页面文件名的安全 slug。
fn slugify(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| match character {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            ' ' => '-',
            other => other,
        })
        .collect::<String>();

    let parts = slug
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<BTreeSet<_>>();

    if parts.is_empty() {
        "module".to_string()
    } else {
        parts.into_iter().collect::<Vec<_>>().join("-")
    }
}
