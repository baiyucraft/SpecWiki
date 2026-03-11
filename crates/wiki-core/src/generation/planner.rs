//! 页面规划层负责把模块树与上下文转换成稳定页面计划。
//! 它服务于 `init / update` 的主链路，只决定“生成哪些页”和“每页依赖什么”。

use std::collections::{BTreeMap, BTreeSet};

use crate::domain::context::{ModuleContext, RepoContext, TopicSeed};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::stable_id::stable_id;
use crate::domain::steering::SteeringConfig;
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::GraphSummary;

/// 模块"页面权重"评分，供合并策略消费。
/// 权重越高，越应该生成独立页面。
///
/// 评分因素：
/// - 源码文件数量（非 test 文件）
/// - 是否有子模块
/// - 是否是 workspace 成员
/// - 是否有入口文件
/// - 模块 kind 是否为核心类型
pub fn module_page_weight(module: &ModuleNode, report: &ScanReport) -> u32 {
    let mut weight: u32 = 0;

    // 源码文件数量（只计非 test 文件）
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
        .count() as u32;
    weight += source_count;

    // 有子模块 +3
    if !module.child_ids.is_empty() {
        weight += 3;
    }

    // workspace 成员 +2
    let is_workspace_member = module
        .root_paths
        .first()
        .map(|rp| report.workspace_roots.iter().any(|wr| wr == rp))
        .unwrap_or(false);
    if is_workspace_member {
        weight += 2;
    }

    // 有入口文件 +2
    if !module.entry_points.is_empty() {
        weight += 2;
    }

    // 核心 kind 类型 +1
    if matches!(
        module.kind.as_str(),
        "library" | "cli-tool" | "backend-service" | "frontend-app"
    ) {
        weight += 1;
    }

    weight
}

/// `PlannedPage` 描述”要生成什么页面”。
/// 这一层只决定页面结构和依赖范围，还不负责具体写出 Markdown 内容。
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PlannedPage {
    /// 页面稳定 ID，会被状态层、缓存层和关系层引用。
    pub id: String,
    /// 页面标题，最终渲染为一级标题。
    pub title: String,
    /// 页面在 `.wiki/` 下的稳定相对路径。
    pub relative_path: String,
    /// 页面类型，如 `overview / architecture / module`。
    pub page_type: String,
    /// 父页面 ID，用于恢复页面树结构。
    pub parent_id: Option<String>,
    /// 页面作用域标签，帮助后续区分 repository / architecture / module 页面。
    pub scope: String,
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
    /// 被合并到本页面的子模块 ID 集合（低权重模块不生成独立页面时记录在此）。
    #[serde(default)]
    pub merged_module_ids: Vec<String>,
    /// `topic` 页面使用的稳定主题类别。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_kind: Option<String>,
    /// `topic` 页面使用的稳定主题键。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_key: Option<String>,
    /// `topic` 页面可直接复用的稳定主题摘要。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_summary: Option<String>,
}

#[derive(Debug, Clone)]
struct TopicCandidate {
    seed: TopicSeed,
    parent_page_id: String,
    parent_scope: String,
    scope: String,
    priority: usize,
}

/// 根据模块树、上下文和 steering 配置规划页面集合。
/// 生成项目概述、系统架构、以及经过合并策略筛选后的模块页。
///
/// # 参数
/// - `report`：仓库扫描报告。
/// - `module_tree`：当前仓库的模块树。
/// - `repo_context`：仓库级页面上下文。
/// - `module_contexts`：模块级上下文集合。
/// - `steering`：steering 配置，控制合并阈值和模块提升/降级。
///
/// # 返回
/// - 返回当前仓库需要生成的页面计划列表。
pub fn plan_pages(
    report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    steering: &SteeringConfig,
) -> Vec<PlannedPage> {
    plan_pages_with_graph(
        report,
        module_tree,
        repo_context,
        module_contexts,
        steering,
        &GraphSummary::default(),
    )
}

/// 根据 graph summary 增强页面规划。
pub fn plan_pages_with_graph(
    report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    steering: &SteeringConfig,
    graph_summary: &GraphSummary,
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
        merged_module_ids: vec![],
        topic_kind: None,
        topic_key: None,
        topic_summary: None,
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
        merged_module_ids: vec![],
        topic_kind: None,
        topic_key: None,
        topic_summary: None,
    });

    let candidates = modules_to_render(module_tree);
    let mut workflow_page_id = None::<String>;

    // 当仓库存在工作流线索时，生成 workflow 页面。
    if has_workflow_clues(report, graph_summary) {
        let workflow_id = stable_id("page", "workflow");
        let mut workflow_source_ids: Vec<String> = report
            .files
            .iter()
            .filter(|f| is_workflow_file(&f.path))
            .map(|f| f.id.clone())
            .collect();
        if workflow_source_ids.is_empty() && !graph_summary.detected_processes.is_empty() {
            workflow_source_ids = report
                .files
                .iter()
                .filter(|file| file.kind == "source")
                .take(12)
                .map(|file| file.id.clone())
                .collect();
        }

        pages.push(PlannedPage {
            id: workflow_id.clone(),
            title: "工作流与部署".to_string(),
            relative_path: "工作流与部署.md".to_string(),
            page_type: "workflow".to_string(),
            parent_id: Some(overview_id.clone()),
            scope: "workflow".to_string(),
            source_ids: workflow_source_ids,
            module_ids: repo_context.top_modules.clone(),
            relation_ids: vec![],
            generation_mode: if graph_summary.detected_processes.is_empty() {
                "deterministic".to_string()
            } else {
                "deterministic:graph-workflow".to_string()
            },
            priority: 2,
            merged_module_ids: vec![],
            topic_kind: None,
            topic_key: None,
            topic_summary: None,
        });
        workflow_page_id = Some(workflow_id);
    }

    // 第一遍：决定哪些模块生成独立页面，哪些被合并。
    let merged_ids = compute_merged_modules(&candidates, report, steering);

    // 构建"模块 ID → 拥有独立页面的最近祖先页面 ID"索引，
    // 用于被合并模块的子模块查找父页面。
    let has_page: BTreeSet<String> = candidates
        .iter()
        .filter(|m| !merged_ids.contains(&m.id))
        .map(|m| m.id.clone())
        .collect();

    let module_context_index: BTreeMap<String, &ModuleContext> = module_contexts
        .iter()
        .map(|context| (context.module_id.clone(), context))
        .collect();

    // 第二遍：为有独立页面的模块生成 PlannedPage，同时收集被合并模块。
    let mut merged_into: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (index, module) in candidates.iter().enumerate() {
        if merged_ids.contains(&module.id) {
            // 被合并模块：找到最近的有独立页面的祖先，记录到其 merged_module_ids。
            let target_page_module_id = find_nearest_page_ancestor(module, module_tree, &has_page);
            let target = target_page_module_id.unwrap_or_else(|| "__overview__".to_string());
            merged_into
                .entry(target)
                .or_default()
                .push(module.id.clone());
            continue;
        }

        let page_id = module_page_id(module);
        let path = module_page_path(module);

        // 父子关系按模块树层级分配：
        // - 顶层模块（父模块是根模块）→ parent_id = overview
        // - 嵌套模块 → parent_id = 父模块的页面 ID（如果父模块有独立页面）
        // - 父模块被合并 → 向上查找最近的有独立页面的祖先模块
        let parent_id = resolve_parent_page_id(module, module_tree, &has_page, &overview_id);

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
            id: page_id,
            title,
            relative_path: path,
            page_type: "module".to_string(),
            parent_id: Some(parent_id),
            scope: format!("module:{}", module.id),
            source_ids,
            module_ids: vec![module.id.clone()],
            relation_ids,
            generation_mode: format!("deterministic:{summary_hint}"),
            priority: 10 + index,
            merged_module_ids: vec![],
            topic_kind: None,
            topic_key: None,
            topic_summary: None,
        });
    }

    // 第三遍：把被合并模块的 ID 写入对应父页面的 merged_module_ids。
    for page in &mut pages {
        // 模块页：按模块 ID 查找
        if page.page_type == "module" {
            if let Some(module_id) = page.module_ids.first() {
                if let Some(merged) = merged_into.remove(module_id) {
                    page.merged_module_ids = merged;
                }
            }
        }
    }
    // 没有找到模块页归属的合并模块，挂到 overview 页。
    if let Some(orphan_merged) = merged_into.remove("__overview__") {
        if let Some(overview_page) = pages.iter_mut().find(|p| p.page_type == "overview") {
            overview_page.merged_module_ids.extend(orphan_merged);
        }
    }
    // 其余未归属的也挂到 overview
    for (_, merged) in merged_into {
        if let Some(overview_page) = pages.iter_mut().find(|p| p.page_type == "overview") {
            overview_page.merged_module_ids.extend(merged);
        }
    }

    let topic_candidates = topic_candidates(
        repo_context,
        module_contexts,
        module_tree,
        &has_page,
        &architecture_id,
        workflow_page_id.as_deref(),
    );
    for (index, topic) in topic_candidates.into_iter().enumerate() {
        pages.push(build_topic_page(topic, 100 + index));
    }

    pages
}

fn topic_candidates(
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    module_tree: &ModuleTree,
    has_module_page: &BTreeSet<String>,
    architecture_page_id: &str,
    workflow_page_id: Option<&str>,
) -> Vec<TopicCandidate> {
    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();

    for seed in &repo_context.root_topics {
        if !should_keep_root_topic(seed) {
            continue;
        }
        let dedupe_key = format!("architecture:{}:{}", seed.topic_kind, seed.topic_key);
        if !seen.insert(dedupe_key) {
            continue;
        }
        candidates.push(TopicCandidate {
            seed: seed.clone(),
            parent_page_id: architecture_page_id.to_string(),
            parent_scope: "architecture".to_string(),
            scope: topic_scope(seed),
            priority: 3,
        });
    }

    if let Some(workflow_page_id) = workflow_page_id {
        let total_process_topics = repo_context.process_topics.len();
        for seed in &repo_context.process_topics {
            if !should_keep_process_topic(seed, total_process_topics) {
                continue;
            }
            let dedupe_key = format!("workflow:{}:{}", seed.topic_kind, seed.topic_key);
            if !seen.insert(dedupe_key) {
                continue;
            }
            candidates.push(TopicCandidate {
                seed: seed.clone(),
                parent_page_id: workflow_page_id.to_string(),
                parent_scope: "workflow".to_string(),
                scope: topic_scope(seed),
                priority: 4,
            });
        }
    }

    for context in module_contexts {
        if !has_module_page.contains(&context.module_id) {
            continue;
        }
        let Some(module) = module_tree.module_by_id(&context.module_id) else {
            continue;
        };

        for seed in &context.capability_topics {
            if !should_keep_module_topic(seed, module) {
                continue;
            }
            let dedupe_key = format!("module:{}:{}", context.module_id, seed.topic_key);
            if !seen.insert(dedupe_key) {
                continue;
            }
            candidates.push(TopicCandidate {
                seed: seed.clone(),
                parent_page_id: module_page_id(module),
                parent_scope: format!("module:{}", context.module_id),
                scope: topic_scope(seed),
                priority: 20,
            });
        }
    }

    candidates.sort_by(|left, right| {
        left.priority.cmp(&right.priority).then_with(|| {
            left.seed
                .title
                .cmp(&right.seed.title)
                .then(left.seed.topic_key.cmp(&right.seed.topic_key))
        })
    });
    candidates
}

fn should_keep_root_topic(seed: &TopicSeed) -> bool {
    seed.source_ids.len() >= 3 || seed.source_paths.len() >= 3
}

fn should_keep_process_topic(seed: &TopicSeed, process_topic_count: usize) -> bool {
    process_topic_count > 1 || count_process_steps(&seed.summary) >= 4
}

fn should_keep_module_topic(seed: &TopicSeed, module: &ModuleNode) -> bool {
    if seed.source_ids.len() < 2 || module.source_ids.len() < 4 {
        return false;
    }

    let coverage = seed.source_ids.len() as f32 / module.source_ids.len().max(1) as f32;
    coverage < 0.8
}

fn count_process_steps(summary: &str) -> usize {
    summary
        .split(':')
        .nth(1)
        .map(|trace| trace.split("->").count())
        .unwrap_or(0)
}

fn topic_scope(seed: &TopicSeed) -> String {
    format!("topic:{}:{}", seed.topic_kind, seed.topic_key)
}

fn topic_page_id(seed: &TopicSeed, parent_scope: &str) -> String {
    stable_id(
        "page",
        &format!(
            "topic:{}:{}:{}",
            parent_scope, seed.topic_kind, seed.topic_key
        ),
    )
}

fn topic_page_path(seed: &TopicSeed) -> String {
    let kind = slugify_segment(&seed.topic_kind);
    let key = slugify_segment(&seed.topic_key);
    let title = slugify_segment(&seed.title);
    format!("专题/{kind}/{key}-{title}.md")
}

fn build_topic_page(topic: TopicCandidate, priority: usize) -> PlannedPage {
    PlannedPage {
        id: topic_page_id(&topic.seed, &topic.parent_scope),
        title: topic.seed.title.clone(),
        relative_path: topic_page_path(&topic.seed),
        page_type: "topic".to_string(),
        parent_id: Some(topic.parent_page_id),
        scope: topic.scope,
        source_ids: topic.seed.source_ids.clone(),
        module_ids: topic.seed.module_ids.clone(),
        relation_ids: topic.seed.relation_ids.clone(),
        generation_mode: format!("deterministic:topic:{}", topic.seed.topic_kind),
        priority,
        merged_module_ids: Vec::new(),
        topic_kind: Some(topic.seed.topic_kind.clone()),
        topic_key: Some(topic.seed.topic_key.clone()),
        topic_summary: Some(topic.seed.summary.clone()),
    }
}

/// 决定哪些模块应该被合并（不生成独立页面）。
/// 合并条件：权重 < 阈值 且 无子模块 且 未被 steering promote。
/// steering demote 的模块强制合并。
fn compute_merged_modules(
    candidates: &[&ModuleNode],
    report: &ScanReport,
    steering: &SteeringConfig,
) -> BTreeSet<String> {
    let threshold = steering.merge_threshold;
    let mut merged = BTreeSet::new();

    for module in candidates {
        let root_path = module.root_paths.first().map(|s| s.as_str()).unwrap_or("");

        // steering demote 强制合并
        if steering.is_demoted(root_path) {
            merged.insert(module.id.clone());
            continue;
        }

        // steering promote 强制保留独立页面
        if steering.is_promoted(root_path) {
            continue;
        }

        // 有子模块的模块不合并
        if !module.child_ids.is_empty() {
            continue;
        }

        let weight = module_page_weight(module, report);
        if weight < threshold {
            merged.insert(module.id.clone());
        }
    }

    merged
}

/// 沿模块树向上查找最近的拥有独立页面的祖先模块 ID。
fn find_nearest_page_ancestor(
    module: &ModuleNode,
    module_tree: &ModuleTree,
    has_page: &BTreeSet<String>,
) -> Option<String> {
    let mut current_parent_id = module.parent_id.as_deref();
    while let Some(pid) = current_parent_id {
        if module_tree.root_modules.contains(&pid.to_string()) {
            return None; // 到达根模块，返回 None 表示应挂到 overview
        }
        if has_page.contains(pid) {
            return Some(pid.to_string());
        }
        current_parent_id = module_tree
            .module_by_id(pid)
            .and_then(|m| m.parent_id.as_deref());
    }
    None
}

/// 为模块页解析父页面 ID。
/// 顶层模块挂到 overview，嵌套模块挂到父模块页面，父模块被合并时向上查找。
fn resolve_parent_page_id(
    module: &ModuleNode,
    module_tree: &ModuleTree,
    has_page: &BTreeSet<String>,
    overview_id: &str,
) -> String {
    let Some(parent_module_id) = &module.parent_id else {
        return overview_id.to_string();
    };

    // 父模块是根模块 → 挂到 overview
    if module_tree.root_modules.contains(parent_module_id) {
        return overview_id.to_string();
    }

    // 父模块有独立页面 → 挂到父模块页面
    if has_page.contains(parent_module_id) {
        let parent_module = module_tree
            .module_by_id(parent_module_id)
            .expect("parent module should exist");
        return module_page_id(parent_module);
    }

    // 父模块被合并 → 向上查找最近的有独立页面的祖先
    let parent_module = module_tree
        .module_by_id(parent_module_id)
        .expect("parent module should exist");
    if let Some(ancestor_id) = find_nearest_page_ancestor(parent_module, module_tree, has_page) {
        let ancestor = module_tree
            .module_by_id(&ancestor_id)
            .expect("ancestor module should exist");
        return module_page_id(ancestor);
    }

    overview_id.to_string()
}

/// 如果存在真正的业务子模块，就只为这些子模块生成模块页；
/// 否则退回到根模块，至少保证最小仓库也能拿到一个模块页。
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

/// 根据模块 root_paths[0] 生成稳定的 page_id。
/// 锚定到归一化后的相对路径，而不是模块名或发现顺序。
fn module_page_id(module: &ModuleNode) -> String {
    let seed = module
        .root_paths
        .first()
        .map(|p| normalize_root_path(p))
        .unwrap_or_else(|| module.name.clone());
    stable_id("page", &format!("module:{seed}"))
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
fn module_page_path(module: &ModuleNode) -> String {
    let relative = module
        .root_paths
        .first()
        .map(|root_path| {
            root_path
                .split('/')
                .filter(|segment| !segment.is_empty() && *segment != ".")
                .map(slugify_segment)
                .collect::<Vec<_>>()
        })
        .filter(|segments| !segments.is_empty())
        .unwrap_or_else(|| vec![slugify_segment(&module.name)]);

    format!("核心模块/{}.md", relative.join("/"))
}

/// 归一化 root_path：统一分隔符为 `/`，去掉前导 `./`。
fn normalize_root_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let normalized = normalized.trim_start_matches("./");
    if normalized.is_empty() {
        ".".to_string()
    } else {
        normalized.to_string()
    }
}

/// 文件名规范化，保证模块页路径在 Windows 上也可安全落盘。
///
/// # 参数
/// - `value`：待规范化的原始名称。
///
/// # 返回
/// - 返回适合当作页面文件名的安全 slug。
fn slugify_segment(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| match character {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            ' ' => '-',
            other => other,
        })
        .collect::<String>();

    let mut normalized = String::new();
    let mut last_was_dash = false;

    for character in slug.chars() {
        if character == '-' {
            if last_was_dash {
                continue;
            }
            last_was_dash = true;
            normalized.push(character);
            continue;
        }

        last_was_dash = false;
        normalized.push(character);
    }

    let normalized = normalized.trim_matches('-');
    if normalized.is_empty() {
        "module".to_string()
    } else {
        normalized.to_string()
    }
}

/// 检测仓库是否存在工作流线索（CI/CD 配置、Makefile、Dockerfile）。
fn has_workflow_clues(report: &ScanReport, graph_summary: &GraphSummary) -> bool {
    report.files.iter().any(|f| is_workflow_file(&f.path))
        || !graph_summary.detected_processes.is_empty()
}

/// 判断文件路径是否属于工作流相关文件。
fn is_workflow_file(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);

    // CI/CD 配置目录
    if path.starts_with(".github/workflows/")
        || path.starts_with(".gitlab/")
        || path.starts_with(".circleci/")
    {
        return true;
    }

    // 顶层工作流文件
    matches!(
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
