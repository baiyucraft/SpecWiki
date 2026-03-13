//! 页面规划层负责把模块树与上下文转换成稳定页面计划。
//! 它服务于 `init / update` 的主链路，只决定“生成哪些页”和“每页依赖什么”。

use std::collections::{BTreeMap, BTreeSet};

use crate::domain::context::{ModuleContext, RepoContext, TopicSeed};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
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
    /// 页面类型，如 `overview / architecture / family-index / family-child / family-leaf-doc / module / topic`。
    pub page_type: String,
    /// 父页面 ID，用于恢复页面树结构。
    pub parent_id: Option<String>,
    /// 页面作用域标签，帮助后续区分 repository / architecture / module 页面。
    pub scope: String,
    /// 当前页面直接对应的知识单元 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 当前页面直接对应的知识单元类型。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// 当前页面归属的知识域 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
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
}

#[derive(Debug, Clone)]
struct TopicCandidate {
    seed: TopicSeed,
    parent_page_id: String,
    parent_scope: String,
    scope: String,
    priority: usize,
}

#[derive(Debug, Clone)]
struct FamilyCandidate {
    family_kind: String,
    family_key: String,
    family_directory: String,
    title: String,
    source_ids: Vec<String>,
    module_ids: Vec<String>,
    relation_ids: Vec<String>,
    children: Vec<FamilyChildCandidate>,
}

#[derive(Debug, Clone)]
struct FamilyChildCandidate {
    family_kind: String,
    family_key: String,
    family_directory: String,
    title: String,
    source_ids: Vec<String>,
    module_ids: Vec<String>,
    relation_ids: Vec<String>,
    leaves: Vec<FamilyLeafCandidate>,
}

#[derive(Debug, Clone)]
struct FamilyLeafCandidate {
    family_kind: String,
    child_key: String,
    leaf_key: String,
    family_directory: String,
    child_title: String,
    title: String,
    source_ids: Vec<String>,
    module_ids: Vec<String>,
    relation_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RepoArchetype {
    DocsPlatform,
}

#[derive(Debug, Clone)]
struct RepoArchetypeProfile {
    overview_path: String,
    architecture_path: String,
    architecture_title: String,
    detection_groups: Vec<SignalGroupSpec>,
    families: Vec<FamilyProfile>,
    root_docs: Vec<RootDocProfile>,
}

#[derive(Debug, Clone)]
struct SignalGroupSpec {
    selectors: Vec<FileSelector>,
}

#[derive(Debug, Clone)]
struct FamilyProfile {
    family_kind: &'static str,
    family_key: &'static str,
    family_directory: &'static str,
    title: &'static str,
    summary: &'static str,
    sort_key: usize,
    children: Vec<FamilyChildProfile>,
}

#[derive(Debug, Clone)]
struct FamilyChildProfile {
    family_key: &'static str,
    title: &'static str,
    summary: &'static str,
    selectors: Vec<FileSelector>,
}

#[derive(Debug, Clone)]
struct RootDocProfile {
    topic_key: &'static str,
    title: &'static str,
    relative_path: &'static str,
    selectors: Vec<FileSelector>,
}

#[derive(Debug, Clone)]
struct FileSelector {
    mode: FileSelectorMode,
    patterns: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
enum FileSelectorMode {
    Prefix,
    Contains,
    Name,
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
    let archetype = detect_repo_archetype(report);
    let archetype_profile = repo_archetype_profile(archetype);
    let overview_id = stable_id("page", "overview");
    let architecture_id = stable_id("page", "architecture");
    let overview_path = overview_page_path(archetype_profile.as_ref());
    let architecture_path = architecture_page_path(archetype_profile.as_ref());
    let architecture_title = architecture_page_title(archetype_profile.as_ref());

    // 概述页总是根页面，后续其他页面默认挂到它下面。
    let mut pages = vec![PlannedPage {
        id: overview_id.clone(),
        title: "项目概述".to_string(),
        relative_path: overview_path,
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
        unit_id: None,
        unit_type: None,
        domain_id: None,
    }];

    // 架构页与概述页并列存在，但在层级上作为概述页的直接子页面。
    pages.push(PlannedPage {
        id: architecture_id.clone(),
        title: architecture_title,
        relative_path: architecture_path,
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
        unit_id: None,
        unit_type: None,
        domain_id: None,
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
            unit_id: None,
            unit_type: None,
            domain_id: None,
        });
        workflow_page_id = Some(workflow_id);
    }

    // 第一遍：决定哪些模块生成独立页面，哪些被合并。
    let family_candidates = discover_family_candidates(report, module_tree, archetype);
    let family_covered_module_ids = family_covered_module_ids(&family_candidates);
    let mut merged_ids = compute_merged_modules(&candidates, report, steering);
    for module in &candidates {
        if family_covered_module_ids.contains(&module.id) {
            merged_ids.insert(module.id.clone());
        }
    }

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
            unit_id: None,
            unit_type: None,
            domain_id: None,
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

    let mut family_priority = 60usize;
    for candidate in family_candidates {
        let family_index = build_family_index_page(&candidate, &overview_id, family_priority);
        family_priority += 1;
        let family_page_id = family_index.id.clone();
        pages.push(family_index);
        for child in candidate.children {
            let family_child = build_family_child_page(
                &child,
                &family_page_id,
                120 + family_priority,
            );
            family_priority += 1;
            let family_child_id = family_child.id.clone();
            pages.push(family_child);
            for leaf in child.leaves {
                pages.push(build_family_leaf_page(
                    &leaf,
                    &family_child_id,
                    180 + family_priority,
                ));
                family_priority += 1;
            }
        }
    }

    pages.extend(discover_root_doc_pages(
        report,
        module_tree,
        archetype_profile.as_ref(),
        &overview_id,
        220,
    ));

    let topic_candidates = topic_candidates(
        report,
        repo_context,
        module_contexts,
        module_tree,
        &has_page,
        &family_covered_module_ids,
        &architecture_id,
        workflow_page_id.as_deref(),
    );
    for (index, topic) in topic_candidates.into_iter().enumerate() {
        pages.push(build_topic_page(topic, 100 + index));
    }

    pages
}

fn topic_candidates(
    report: &ScanReport,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    module_tree: &ModuleTree,
    has_module_page: &BTreeSet<String>,
    family_covered_module_ids: &BTreeSet<String>,
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
            if topic_is_covered_by_family(seed, module, report, family_covered_module_ids) {
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

    for (seed, parent_kind) in discover_archetype_topic_seeds(
        report,
        repo_context,
        module_tree,
        workflow_page_id.is_some(),
    ) {
        let scope = topic_scope(&seed);
        let (parent_page_id, parent_scope, priority) = match parent_kind {
            "workflow" => (
                workflow_page_id.unwrap_or(architecture_page_id).to_string(),
                "workflow".to_string(),
                6,
            ),
            _ => (
                architecture_page_id.to_string(),
                "architecture".to_string(),
                5,
            ),
        };
        let dedupe_key = format!("{parent_scope}:{}:{}", seed.topic_kind, seed.topic_key);
        if !seen.insert(dedupe_key) {
            continue;
        }
        candidates.push(TopicCandidate {
            seed,
            parent_page_id,
            parent_scope,
            scope,
            priority,
        });
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

fn topic_is_covered_by_family(
    seed: &TopicSeed,
    module: &ModuleNode,
    _report: &ScanReport,
    family_covered_module_ids: &BTreeSet<String>,
) -> bool {
    if seed.topic_kind != "module-capability" {
        return false;
    }

    family_covered_module_ids.contains(&module.id)
}

fn discover_archetype_topic_seeds(
    report: &ScanReport,
    repo_context: &RepoContext,
    module_tree: &ModuleTree,
    workflow_page_present: bool,
) -> Vec<(TopicSeed, &'static str)> {
    let mut topics = Vec::new();

    if let Some(seed) = web_request_topic_seed(report, repo_context, module_tree) {
        topics.push((seed, "architecture"));
    }
    if let Some(seed) = config_runtime_topic_seed(report, repo_context) {
        topics.push((seed, "architecture"));
    }
    if let Some(seed) = cli_entry_topic_seed(report, module_tree) {
        topics.push((seed, "architecture"));
    } else if let Some(seed) = library_api_topic_seed(report, module_tree) {
        topics.push((seed, "architecture"));
    }
    if workflow_page_present {
        if let Some(seed) = ops_runtime_topic_seed(report) {
            topics.push((seed, "workflow"));
        }
    }

    topics
}

fn web_request_topic_seed(
    report: &ScanReport,
    repo_context: &RepoContext,
    module_tree: &ModuleTree,
) -> Option<TopicSeed> {
    let relevant = report
        .files
        .iter()
        .filter(|file| {
            matches!(
                file.purpose,
                crate::repo::scanner::FilePurpose::Entry
                    | crate::repo::scanner::FilePurpose::Router
                    | crate::repo::scanner::FilePurpose::Controller
                    | crate::repo::scanner::FilePurpose::Handler
                    | crate::repo::scanner::FilePurpose::Middleware
            ) && file.is_substantive_source()
        })
        .take(8)
        .collect::<Vec<_>>();
    let has_router = relevant.iter().any(|file| {
        matches!(
            file.purpose,
            crate::repo::scanner::FilePurpose::Router | crate::repo::scanner::FilePurpose::Handler
        )
    });
    if relevant.len() < 3 || !has_router {
        return None;
    }

    Some(TopicSeed {
        topic_kind: "repo-archetype".to_string(),
        topic_key: "request-lifecycle".to_string(),
        title: "请求处理链".to_string(),
        summary: "该专题聚焦入口、路由、中间件与处理函数之间的稳定请求链。".to_string(),
        source_ids: relevant.iter().map(|file| file.id.clone()).collect(),
        source_paths: relevant.iter().map(|file| file.path.clone()).collect(),
        module_ids: repo_context
            .top_modules
            .iter()
            .filter(|module_id| module_tree.module_by_id(module_id).is_some())
            .cloned()
            .collect(),
        relation_ids: Vec::new(),
    })
}

fn config_runtime_topic_seed(report: &ScanReport, repo_context: &RepoContext) -> Option<TopicSeed> {
    let mut paths = report
        .config_files
        .iter()
        .take(6)
        .cloned()
        .collect::<Vec<_>>();
    paths.extend(repo_context.key_entry_points.iter().take(2).cloned());
    paths.sort();
    paths.dedup();
    if paths.len() < 2 {
        return None;
    }

    Some(TopicSeed {
        topic_kind: "repo-archetype".to_string(),
        topic_key: "config-runtime".to_string(),
        title: "配置与运行时".to_string(),
        summary: "该专题聚焦配置文件、运行时入口和环境切换的稳定边界。".to_string(),
        source_ids: paths
            .iter()
            .filter_map(|path| {
                report
                    .files
                    .iter()
                    .find(|file| file.path == *path)
                    .map(|file| file.id.clone())
            })
            .collect(),
        source_paths: paths,
        module_ids: repo_context.top_modules.clone(),
        relation_ids: Vec::new(),
    })
}

fn cli_entry_topic_seed(report: &ScanReport, module_tree: &ModuleTree) -> Option<TopicSeed> {
    let cli_modules = module_tree
        .modules
        .iter()
        .filter(|module| module.kind == "cli-tool")
        .collect::<Vec<_>>();
    if cli_modules.is_empty() {
        return None;
    }

    let mut source_paths = cli_modules
        .iter()
        .flat_map(|module| module.entry_points.iter().cloned())
        .collect::<Vec<_>>();
    if source_paths.is_empty() {
        source_paths.extend(
            report
                .entry_points
                .iter()
                .filter(|path| {
                    path.contains("main") || path.contains("cli") || path.contains("command")
                })
                .take(6)
                .cloned(),
        );
    }
    source_paths.sort();
    source_paths.dedup();
    if source_paths.is_empty() {
        return None;
    }

    Some(TopicSeed {
        topic_kind: "repo-archetype".to_string(),
        topic_key: "cli-entry".to_string(),
        title: "命令入口与执行流".to_string(),
        summary: "该专题聚焦命令入口、参数解析和执行主链。".to_string(),
        source_ids: source_paths
            .iter()
            .filter_map(|path| {
                report
                    .files
                    .iter()
                    .find(|file| file.path == *path)
                    .map(|file| file.id.clone())
            })
            .collect(),
        source_paths,
        module_ids: cli_modules.iter().map(|module| module.id.clone()).collect(),
        relation_ids: Vec::new(),
    })
}

fn library_api_topic_seed(report: &ScanReport, module_tree: &ModuleTree) -> Option<TopicSeed> {
    let relevant = report
        .files
        .iter()
        .filter(|file| {
            matches!(
                file.purpose,
                crate::repo::scanner::FilePurpose::Library
                    | crate::repo::scanner::FilePurpose::Domain
                    | crate::repo::scanner::FilePurpose::Model
                    | crate::repo::scanner::FilePurpose::Service
            ) && file.is_substantive_source()
        })
        .take(8)
        .collect::<Vec<_>>();
    if relevant.len() < 3 {
        return None;
    }

    Some(TopicSeed {
        topic_kind: "repo-archetype".to_string(),
        topic_key: "core-api-models".to_string(),
        title: "核心 API 与数据模型".to_string(),
        summary: "该专题聚焦对外 API、核心领域模型与主要实现边界。".to_string(),
        source_ids: relevant.iter().map(|file| file.id.clone()).collect(),
        source_paths: relevant.iter().map(|file| file.path.clone()).collect(),
        module_ids: module_tree
            .modules
            .iter()
            .filter(|module| module.kind == "library")
            .map(|module| module.id.clone())
            .collect(),
        relation_ids: Vec::new(),
    })
}

fn ops_runtime_topic_seed(report: &ScanReport) -> Option<TopicSeed> {
    let relevant = report
        .files
        .iter()
        .filter(|file| {
            file.path.contains("docker")
                || file.path.contains("k8s")
                || file.path.contains("helm")
                || file.path.starts_with(".github/workflows/")
                || file.path.starts_with(".gitlab/")
        })
        .take(8)
        .collect::<Vec<_>>();
    if relevant.len() < 2 {
        return None;
    }

    Some(TopicSeed {
        topic_kind: "repo-archetype".to_string(),
        topic_key: "ops-runtime".to_string(),
        title: "部署与环境".to_string(),
        summary: "该专题聚焦镜像、部署、CI/CD 与环境编排相关事实。".to_string(),
        source_ids: relevant.iter().map(|file| file.id.clone()).collect(),
        source_paths: relevant.iter().map(|file| file.path.clone()).collect(),
        module_ids: Vec::new(),
        relation_ids: Vec::new(),
    })
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
        unit_id: None,
        unit_type: None,
        domain_id: None,
    }
}

fn discover_family_candidates(
    report: &ScanReport,
    module_tree: &ModuleTree,
    archetype: Option<RepoArchetype>,
) -> Vec<FamilyCandidate> {
    let Some(profile) = repo_archetype_profile(archetype) else {
        return Vec::new();
    };

    let mut candidates = profile
        .families
        .iter()
        .filter_map(|family| build_family_candidate(report, module_tree, family))
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        family_sort_key(&profile, &left.family_kind)
            .cmp(&family_sort_key(&profile, &right.family_kind))
            .then(left.title.cmp(&right.title))
            .then(left.family_key.cmp(&right.family_key))
    });
    candidates
}

fn detect_repo_archetype(report: &ScanReport) -> Option<RepoArchetype> {
    for archetype in [RepoArchetype::DocsPlatform] {
        let Some(profile) = repo_archetype_profile(Some(archetype)) else {
            continue;
        };
        if profile
            .detection_groups
            .iter()
            .all(|group| group_matches(report, group))
        {
            return Some(archetype);
        }
    }

    None
}

fn family_covered_module_ids(families: &[FamilyCandidate]) -> BTreeSet<String> {
    families
        .iter()
        .flat_map(|family| family.module_ids.iter().cloned())
        .collect()
}

fn repo_archetype_profile(archetype: Option<RepoArchetype>) -> Option<RepoArchetypeProfile> {
    match archetype {
        Some(RepoArchetype::DocsPlatform) => Some(docs_platform_profile()),
        None => None,
    }
}

fn docs_platform_profile() -> RepoArchetypeProfile {
    RepoArchetypeProfile {
        overview_path: "项目概述/项目概述.md".to_string(),
        architecture_path: "项目概述/技术架构/技术架构.md".to_string(),
        architecture_title: "技术架构".to_string(),
        detection_groups: vec![
            SignalGroupSpec {
                selectors: vec![prefixes(&["docs/"])],
            },
            SignalGroupSpec {
                selectors: vec![
                    prefixes(&["code/addons/", "code/frameworks/", "code/builders/", "code/presets/"]),
                    prefixes(&["packages/"]),
                ],
            },
            SignalGroupSpec {
                selectors: vec![
                    prefixes(&["docs/api/", "docs/get-started/", "docs/configure/"]),
                    names(&["main.ts", "preview.ts", "manager.ts", "manager.tsx", "public-types.ts"]),
                ],
            },
        ],
        families: docs_platform_family_profiles(),
        root_docs: vec![
            RootDocProfile {
                topic_key: "quick-start",
                title: "快速开始",
                relative_path: "快速开始.md",
                selectors: vec![
                    prefixes(&["docs/get-started/", "docs/configure/"]),
                    names(&["README.md"]),
                    contains(&["getting-started", "quick-start"]),
                ],
            },
            RootDocProfile {
                topic_key: "contributing",
                title: "贡献指南",
                relative_path: "贡献指南.md",
                selectors: vec![
                    names(&["CONTRIBUTING.md", "CODE_OF_CONDUCT.md"]),
                    prefixes(&[".github/", "scripts/"]),
                    contains(&["contribut", "release", "changeset"]),
                ],
            },
        ],
    }
}

fn docs_platform_family_profiles() -> Vec<FamilyProfile> {
    vec![
        FamilyProfile {
            family_kind: "concept",
            family_key: "concept",
            family_directory: "核心概念",
            title: "核心概念",
            summary: "该 family 聚焦文档平台仓库中的 stories、CSF、preview/manager 与全局状态模型。",
            sort_key: 0,
            children: vec![
                child_profile(
                    "stories",
                    "组件故事（Stories）",
                    "该子页聚焦 story、portable stories 与故事组织约定。",
                    vec![
                        prefixes(&["docs/get-started/"]),
                        prefixes(&["docs/api/portable-stories/"]),
                        contains(&["stories.ts", ".stories.", "portable-stories"]),
                    ],
                ),
                child_profile(
                    "csf",
                    "CSF格式规范",
                    "该子页聚焦 CSF、story annotations 与 story store 规范。",
                    vec![
                        prefixes(&["docs/api/csf/"]),
                        contains(&["/preview-api/modules/store/csf/", "csf-factory", "processCSFFile", "composeConfigs"]),
                    ],
                ),
                child_profile(
                    "decorators-and-globals",
                    "装饰器和全局状态",
                    "该子页聚焦 decorators、globals 与 addons hooks 的运行时约定。",
                    vec![
                        contains(&["/preview-api/modules/addons/", "/preview/globals/", "/toolbar/", "globals.ts", "make-decorator"]),
                        names(&["preview.ts", "manager.tsx"]),
                    ],
                ),
                child_profile(
                    "preview-and-manager",
                    "预览和管理界面",
                    "该子页聚焦 preview、manager、channel 与 manager-api 的核心实现。",
                    vec![
                        contains(&["/manager-api/", "/preview-api/", "/channels/", "/manager/", "/preview/"]),
                        names(&["main.ts", "preview.ts", "manager.ts", "manager.tsx"]),
                    ],
                ),
            ],
        },
        FamilyProfile {
            family_kind: "addon",
            family_key: "addon",
            family_directory: "插件系统",
            title: "插件系统",
            summary: "该 family 聚焦插件系统、扩展入口与插件能力边界。",
            sort_key: 1,
            children: vec![
                child_profile("overview", "Addons概览", "该子页聚焦插件系统的文档入口与全局概览。", vec![prefixes(&["docs/addons/"])]),
                child_profile("a11y", "A11y Addon（可访问性测试）", "该子页聚焦可访问性插件的 manager / preview / preset 与测试入口。", vec![prefixes(&["code/addons/a11y/", "docs/writing-tests/", "docs/_snippets/addon-a11y"])]),
                child_profile("docs", "Docs Addon（文档生成）", "该子页聚焦文档插件、Doc Blocks 与自动文档生成能力。", vec![prefixes(&["code/addons/docs/", "docs/writing-docs/"])]),
                child_profile("links", "Links Addon（故事导航）", "该子页聚焦故事导航与链接型插件能力。", vec![prefixes(&["code/addons/links/"])]),
                child_profile("themes", "Themes Addon（主题切换）", "该子页聚焦主题切换、装饰器与外观扩展。", vec![prefixes(&["code/addons/themes/", "docs/configure/user-interface/theming.mdx"])]),
                child_profile("tooling", "辅助工具 Addons（扩展工具）", "该子页聚焦 pseudo-states、onboarding 等辅助型扩展。", vec![prefixes(&["code/addons/pseudo-states/", "code/addons/onboarding/"])]),
                child_profile("vitest", "Vitest Addon（测试集成）", "该子页聚焦测试面板和 Vitest 插件能力。", vec![prefixes(&["code/addons/vitest/", "docs/writing-tests/integrations/vitest-addon/"])]),
            ],
        },
        FamilyProfile {
            family_kind: "framework",
            family_key: "framework",
            family_directory: "多框架支持",
            title: "多框架支持",
            summary: "该 family 聚焦多框架适配器、renderers 与运行时入口。",
            sort_key: 2,
            children: vec![
                child_profile("angular", "Angular框架支持", "该子页聚焦 Angular 适配器、builder 与 compodoc 集成。", vec![prefixes(&["code/frameworks/angular/", "docs/configure/integration/frameworks.mdx", "docs/_snippets/angular-"])]),
                child_profile("html", "HTML框架支持", "该子页聚焦 HTML renderer 与 html-vite 适配路径。", vec![prefixes(&["code/frameworks/html-vite/", "code/renderers/html/"])]),
                child_profile("react", "React框架支持", "该子页聚焦 React / Next.js 适配器与 React renderer。", vec![prefixes(&["code/frameworks/react-vite/", "code/frameworks/react-webpack5/", "code/frameworks/nextjs/", "code/frameworks/nextjs-vite/", "code/renderers/react/"])]),
                child_profile("svelte", "Svelte框架支持", "该子页聚焦 Svelte / SvelteKit 适配器与 Svelte renderer。", vec![prefixes(&["code/frameworks/svelte-vite/", "code/frameworks/sveltekit/", "code/renderers/svelte/"])]),
                child_profile("vue3", "Vue 3框架支持", "该子页聚焦 Vue 3 适配器与 Vue 3 renderer。", vec![prefixes(&["code/frameworks/vue3-vite/", "code/renderers/vue3/"])]),
                child_profile("web-components", "Web Components框架支持", "该子页聚焦 Web Components 适配器与 renderer。", vec![prefixes(&["code/frameworks/web-components-vite/", "code/renderers/web-components/"])]),
            ],
        },
        FamilyProfile {
            family_kind: "builder",
            family_key: "builder",
            family_directory: "构建系统",
            title: "构建系统",
            summary: "该 family 聚焦 builders、presets 与构建配置边界。",
            sort_key: 3,
            children: vec![
                child_profile("vite", "Vite构建器详解", "该子页聚焦 builder-vite 的配置、插件与 dev server 入口。", vec![prefixes(&["code/builders/builder-vite/", "docs/builders/vite.mdx"])]),
                child_profile("webpack", "Webpack构建器详解", "该子页聚焦 builder-webpack5 与 webpack preview 入口。", vec![prefixes(&["code/builders/builder-webpack5/", "docs/builders/webpack.mdx"])]),
                child_profile("presets", "预设配置", "该子页聚焦 presets、common preset 与 framework preset 配置面。", vec![prefixes(&["code/presets/", "code/core/src/core-server/presets/", "docs/addons/writing-presets.mdx"])]),
            ],
        },
        FamilyProfile {
            family_kind: "api",
            family_key: "api",
            family_directory: "API参考",
            title: "API参考",
            summary: "该 family 聚焦 CLI、开发 API 与类型定义入口。",
            sort_key: 4,
            children: vec![
                child_profile("cli", "CLI命令参考", "该子页聚焦 CLI、dispatcher 与 create 命令入口。", vec![contains(&["/cli-sb/", "/src/bin/"]), prefixes(&["docs/api/"]), names(&["cli-options.mdx"])]),
                child_profile("dev-api", "开发API参考", "该子页聚焦 addons、manager-api、preview-api 与公开开发 API。", vec![contains(&["/manager-api/", "/preview-api/", "/addons/", "get-addon-annotations", "get-addon-names"]), prefixes(&["docs/api/"])]),
                child_profile("types", "类型定义参考", "该子页聚焦 public-types、types 与 typings 定义面。", vec![names(&["public-types.ts", "types.ts", "typings.d.ts"]), contains(&["/src/types.ts", "/src/typings.d.ts"])]),
            ],
        },
        FamilyProfile {
            family_kind: "config",
            family_key: "config",
            family_directory: "API参考/配置API参考",
            title: "配置API参考",
            summary: "该 family 聚焦 main / preview / manager / preset 配置入口。",
            sort_key: 5,
            children: vec![
                child_profile("main", "main.js配置", "该子页聚焦 main 配置、stories 匹配与 addons 注册入口。", vec![names(&["main.ts", "main.js"]), prefixes(&["docs/api/main-config/"]), contains(&["load-main-config"])]),
                child_profile("preview", "preview.js配置", "该子页聚焦 preview 配置、全局参数、decorators 与 preview 注解。", vec![names(&["preview.ts", "preview.js"]), contains(&["process-preview-annotation", "preview-annotations"]), prefixes(&["docs/api/main-config/"])]),
                child_profile("manager", "manager.js配置", "该子页聚焦 manager 入口、manager-head 与 UI 配置面。", vec![names(&["manager.ts", "manager.js", "manager.tsx"]), prefixes(&["docs/api/main-config/"]), contains(&["manager-head", "/manager/"])]),
                child_profile("builder-config", "构建器配置", "该子页聚焦 builder 与 build-config 的配置入口。", vec![names(&["build-config.ts"]), prefixes(&["code/builders/", "docs/builders/"])]),
                child_profile("preset-config", "预设配置", "该子页聚焦 preset.ts、preset.js 与 common preset 体系。", vec![names(&["preset.ts", "preset.js"]), prefixes(&["code/presets/", "docs/addons/"]), contains(&["common-preset", "preview-preset"])]),
            ],
        },
        FamilyProfile {
            family_kind: "theme",
            family_key: "theme",
            family_directory: "主题和外观",
            title: "主题和外观",
            summary: "该 family 聚焦主题系统、外观配置与 UI 定制能力。",
            sort_key: 6,
            children: vec![
                child_profile("theme-system", "主题系统概览", "该子页聚焦 theme、backgrounds、toolbar 与 UI 外观能力。", vec![contains(&["theme", "backgrounds", "toolbar"]), prefixes(&["docs/configure/user-interface/", "docs/sharing/"])]),
                child_profile("custom-theme", "自定义主题开发", "该子页聚焦自定义主题、theme switcher 与主题装饰器扩展。", vec![prefixes(&["code/addons/themes/"]), contains(&["theme-switcher"])]),
            ],
        },
        FamilyProfile {
            family_kind: "advanced",
            family_key: "advanced",
            family_directory: "高级功能",
            title: "高级功能",
            summary: "该 family 聚焦高阶扩展、工具集成、性能观测与自定义渲染能力。",
            sort_key: 7,
            children: vec![
                child_profile("presets", "预设配置", "该子页聚焦 preset、common preset 与高阶扩展配置。", vec![prefixes(&["code/presets/", "docs/addons/"]), contains(&["writing-presets", "common-preset", "preview-preset"])]),
                child_profile("custom-renderers", "自定义渲染器", "该子页聚焦 renderers、自定义 renderer 扩展与框架渲染接入点。", vec![prefixes(&["code/renderers/"]), contains(&["renderer", "renderToCanvas"])]),
                child_profile("tooling", "工具集成", "该子页聚焦 codemod、ESLint、IDE、CI 与外围工具接入。", vec![prefixes(&["scripts/", ".github/", ".circleci/", "code/lib/"]), contains(&["codemod", "eslint", "ide", "workflow", "chromatic", "telejson"])]),
                child_profile("performance", "性能监控", "该子页聚焦性能测量、bench、telemetry 与运行时观测点。", vec![contains(&["performance", "benchmark", "bench", "telemetry", "profiler"]), prefixes(&["code/core/", "scripts/bench/"])]),
            ],
        },
        FamilyProfile {
            family_kind: "troubleshooting",
            family_key: "troubleshooting",
            family_directory: "故障排除",
            title: "故障排除",
            summary: "该 family 聚焦调试、配置问题与运行时错误定位。",
            sort_key: 8,
            children: vec![
                child_profile("faq", "调试工具和技巧", "该子页聚焦调试、错误处理与本地排查入口。", vec![contains(&["ERRORS.md", "error", "debug", "faq"]), prefixes(&["docs/"])]),
                child_profile("runtime-errors", "运行时错误", "该子页聚焦 preview/manager/core-server 运行时错误与边界处理。", vec![contains(&["Error", "errors", "error-boundary", "preview-errors", "server-errors"]), prefixes(&["code/core/src/"])]),
                child_profile("config-errors", "配置错误", "该子页聚焦配置装载、main/preview 冲突与 builder 配置问题。", vec![contains(&["load-main-config", "setup-addon-in-config", "sync-main-preview-addons", "vite-config", "iframe-webpack.config"]), prefixes(&["docs/configure/"])]),
            ],
        },
        FamilyProfile {
            family_kind: "testing",
            family_key: "testing",
            family_directory: "测试框架",
            title: "测试框架",
            summary: "该 family 聚焦 Vitest、Playwright、组件测试与可访问性测试能力。",
            sort_key: 9,
            children: vec![
                child_profile("vitest", "Vitest集成", "该子页聚焦 addon-vitest、vitest 配置与 test provider 能力。", vec![contains(&["vitest", "test-provider", "vitest-plugin"]), prefixes(&["docs/writing-tests/"])]),
                child_profile("playwright", "Playwright测试", "该子页聚焦 Playwright 组件测试与 portable stories 测试入口。", vec![contains(&["playwright", "component-testing.spec", "portable-stories-playwright"]), prefixes(&["docs/writing-tests/"])]),
                child_profile("component-testing", "组件测试", "该子页聚焦 component-testing 模块与测试运行时。", vec![contains(&["component-testing", "portable-stories", "component-tests"]), prefixes(&["docs/writing-tests/"])]),
                child_profile("a11y", "可访问性测试", "该子页聚焦 addon-a11y、test-runner a11y 与可访问性测试入口。", vec![contains(&["a11y", "accessibility", "axe-playwright"]), prefixes(&["docs/writing-tests/"])]),
            ],
        },
        FamilyProfile {
            family_kind: "ops",
            family_key: "ops",
            family_directory: "部署和CI_CD",
            title: "部署和CI_CD",
            summary: "该 family 聚焦发布、Chromatic 与 CI/CD 流水线。",
            sort_key: 10,
            children: vec![
                child_profile("chromatic", "Chromatic集成", "该子页聚焦 chromatic 配置、自动化脚本与发布集成。", vec![contains(&["chromatic", "ui-review", "build-storybook"]), prefixes(&["docs/sharing/"])]),
                child_profile("ci-cd", "CI_CD集成", "该子页聚焦 GitHub Actions、CircleCI 与测试流水线入口。", vec![prefixes(&[".github/", ".circleci/", "scripts/ci/", "scripts/ecosystem-ci/"]), contains(&["workflow", "e2e-tests-build", "test-runner-build", "chromatic.ts"])]),
                child_profile("static-deploy", "静态部署", "该子页聚焦 build-static、静态发布与产物部署流程。", vec![contains(&["build-static", "build-storybook"]), prefixes(&["docs/sharing/"])]),
            ],
        },
    ]
}

fn family_sort_key(profile: &RepoArchetypeProfile, family_kind: &str) -> usize {
    profile
        .families
        .iter()
        .find(|family| family.family_kind == family_kind)
        .map(|family| family.sort_key)
        .unwrap_or(99)
}

fn build_family_candidate(
    report: &ScanReport,
    module_tree: &ModuleTree,
    family: &FamilyProfile,
) -> Option<FamilyCandidate> {
    let mut children = family
        .children
        .iter()
        .filter_map(|child| build_family_child_candidate(report, module_tree, family, child))
        .collect::<Vec<_>>();
    children.retain(|child| !child.source_ids.is_empty());
    if children.is_empty() {
        return None;
    }

    children.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.family_key.cmp(&right.family_key))
    });

    Some(FamilyCandidate {
        family_kind: family.family_kind.to_string(),
        family_key: family.family_key.to_string(),
        family_directory: family.family_directory.to_string(),
        title: family.title.to_string(),
        source_ids: dedupe_vec(children.iter().flat_map(|child| child.source_ids.clone()).collect()),
        module_ids: dedupe_vec(children.iter().flat_map(|child| child.module_ids.clone()).collect()),
        relation_ids: Vec::new(),
        children,
    })
}

fn build_family_child_candidate(
    report: &ScanReport,
    module_tree: &ModuleTree,
    family: &FamilyProfile,
    child: &FamilyChildProfile,
) -> Option<FamilyChildCandidate> {
    let files = select_files(report, &child.selectors);
    if files.is_empty() {
        return None;
    }
    let source_ids = files.iter().map(|file| file.id.clone()).collect::<Vec<_>>();
    let paths = files.iter().map(|file| file.path.clone()).collect::<Vec<_>>();
    let leaves = build_family_leaf_candidates(report, module_tree, family, child, &files);
    Some(FamilyChildCandidate {
        family_kind: family.family_kind.to_string(),
        family_key: child.family_key.to_string(),
        family_directory: family.family_directory.to_string(),
        title: child.title.to_string(),
        source_ids,
        module_ids: collect_module_ids_for_paths(module_tree, &paths),
        relation_ids: Vec::new(),
        leaves,
    })
}

fn build_family_leaf_candidates(
    _report: &ScanReport,
    module_tree: &ModuleTree,
    family: &FamilyProfile,
    child: &FamilyChildProfile,
    files: &[&crate::repo::scanner::ScannedFile],
) -> Vec<FamilyLeafCandidate> {
    let mut merged_groups = BTreeMap::<String, (String, String, Vec<String>, Vec<String>)>::new();
    for (leaf_key, title, summary, paths, source_ids) in cluster_leaf_groups_from_docs(files)
        .into_iter()
        .chain(cluster_leaf_groups_from_surfaces(files))
    {
        let entry = merged_groups
            .entry(leaf_key)
            .or_insert_with(|| (title, summary, Vec::new(), Vec::new()));
        entry.2.extend(paths);
        entry.3.extend(source_ids);
    }

    let groups = merged_groups
        .into_iter()
        .map(|(leaf_key, (title, summary, paths, source_ids))| {
            (
                leaf_key,
                title,
                summary,
                dedupe_vec(paths),
                dedupe_vec(source_ids),
            )
        })
        .filter(|(leaf_key, title, _, paths, source_ids)| {
            !leaf_key.is_empty()
                && !is_generic_leaf_key(leaf_key)
                && !leaf_title_duplicates_child(child.title, title)
                && paths.len() >= 2
                && source_ids.len() >= 2
        })
        .take(6)
        .collect::<Vec<_>>();

    if groups.len() < 2 {
        return Vec::new();
    }

    groups
        .into_iter()
        .map(|(leaf_key, title, _summary, paths, source_ids)| FamilyLeafCandidate {
            family_kind: family.family_kind.to_string(),
            child_key: child.family_key.to_string(),
            leaf_key,
            family_directory: family.family_directory.to_string(),
            child_title: child.title.to_string(),
            title,
            module_ids: collect_module_ids_for_paths(module_tree, &paths),
            relation_ids: Vec::new(),
            source_ids,
        })
        .collect()
}

fn cluster_leaf_groups_from_docs(
    files: &[&crate::repo::scanner::ScannedFile],
) -> Vec<(String, String, String, Vec<String>, Vec<String>)> {
    let mut groups = BTreeMap::<String, (String, String, Vec<String>, Vec<String>)>::new();

    for file in files.iter().copied().filter(|file| is_doc_leaf_path(&file.path)) {
        let Some((leaf_key, title)) = doc_leaf_cluster_descriptor(&file.path) else {
            continue;
        };
        let summary = format!("该叶子页聚焦 `{}` 相关的文档簇与入口。", title);
        let entry = groups
            .entry(leaf_key)
            .or_insert_with(|| (title, summary, Vec::new(), Vec::new()));
        entry.2.push(file.path.clone());
        entry.3.push(file.id.clone());
    }

    groups
        .into_iter()
        .filter(|(_, (_, _, paths, source_ids))| paths.len() >= 2 && source_ids.len() >= 2)
        .map(|(leaf_key, (title, summary, paths, source_ids))| {
            (leaf_key, title, summary, dedupe_vec(paths), dedupe_vec(source_ids))
        })
        .take(8)
        .collect()
}

fn cluster_leaf_groups_from_surfaces(
    files: &[&crate::repo::scanner::ScannedFile],
) -> Vec<(String, String, String, Vec<String>, Vec<String>)> {
    let mut groups = BTreeMap::<String, (String, String, Vec<String>, Vec<String>)>::new();

    for file in files {
        let Some((surface_kind, cluster_key, title)) = surface_leaf_cluster_descriptor(&file.path)
        else {
            continue;
        };
        let leaf_key = format!("{}-{}", surface_kind, cluster_key);
        let summary = format!("该叶子页聚焦 `{}` 的{}入口与关键实现。", title, surface_kind);
        let entry = groups
            .entry(leaf_key)
            .or_insert_with(|| (title, summary, Vec::new(), Vec::new()));
        entry.2.push(file.path.clone());
        entry.3.push(file.id.clone());
    }

    groups
        .into_iter()
        .filter(|(_, (_, _, paths, source_ids))| paths.len() >= 2 && source_ids.len() >= 2)
        .map(|(leaf_key, (title, summary, paths, source_ids))| {
            (leaf_key, title, summary, dedupe_vec(paths), dedupe_vec(source_ids))
        })
        .take(6)
        .collect()
}

fn is_doc_leaf_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    (lower.starts_with("docs/") || lower.contains("/docs/"))
        && (lower.ends_with(".md") || lower.ends_with(".mdx"))
}

fn doc_leaf_cluster_descriptor(path: &str) -> Option<(String, String)> {
    let segments = path.split('/').collect::<Vec<_>>();
    let docs_index = segments
        .iter()
        .position(|segment| segment.eq_ignore_ascii_case("docs"))?;
    let directories = segments
        .iter()
        .skip(docs_index + 1)
        .take(segments.len().saturating_sub(docs_index + 2))
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let cluster_segment = directories
        .iter()
        .rev()
        .find(|segment| !is_generic_doc_directory(segment))?;
    let leaf_key = slug_key(cluster_segment);
    if leaf_key.is_empty() || is_generic_leaf_key(&leaf_key) {
        return None;
    }
    Some((leaf_key, humanize_leaf_title(cluster_segment)))
}

fn is_generic_doc_directory(segment: &str) -> bool {
    matches!(
        segment.to_ascii_lowercase().as_str(),
        "api"
            | "apis"
            | "addons"
            | "docs"
            | "doc"
            | "guide"
            | "guides"
            | "essentials"
            | "tutorial"
            | "tutorials"
            | "snippets"
            | "examples"
            | "reference"
            | "references"
            | "writing-tests"
            | "get-started"
    )
}

fn surface_leaf_descriptor(path: &str) -> Option<(&'static str, String)> {
    let name = path.rsplit('/').next()?;
    let lower = name.to_ascii_lowercase();
    let stem = name
        .trim_end_matches(".d.ts")
        .trim_end_matches(".tsx")
        .trim_end_matches(".ts")
        .trim_end_matches(".js")
        .trim_end_matches(".jsx")
        .trim_end_matches(".json");
    if stem.is_empty() {
        return None;
    }

    if lower.contains("config") || matches!(lower.as_str(), "main.ts" | "main.js" | "preview.ts" | "preview.js" | "manager.ts" | "manager.js" | "preset.ts" | "preset.js" | "build-config.ts") {
        return Some(("config", stem.to_string()));
    }
    if lower.contains("types") || lower.contains("typing") || lower.ends_with(".d.ts") {
        return Some(("type", stem.to_string()));
    }
    if matches!(lower.as_str(), "index.ts" | "index.js" | "public-types.ts") || path.contains("/src/") {
        return Some(("api", stem.to_string()));
    }

    None
}

fn surface_leaf_cluster_descriptor(path: &str) -> Option<(&'static str, String, String)> {
    if is_doc_leaf_path(path) || is_low_signal_leaf_source(path) {
        return None;
    }

    let (surface_kind, stem) = surface_leaf_descriptor(path)?;
    let cluster = semantic_surface_cluster_key(&stem)?;
    let leaf_key = slug_key(&cluster);
    if leaf_key.is_empty() || is_generic_leaf_key(&leaf_key) {
        return None;
    }
    Some((surface_kind, leaf_key, humanize_leaf_title(&cluster)))
}

fn semantic_surface_cluster_key(stem: &str) -> Option<String> {
    let tokens = leaf_tokens(stem);
    if tokens.is_empty() {
        return None;
    }

    if tokens.starts_with(&["main".to_string(), "config".to_string()]) {
        return Some("main-config".to_string());
    }
    if tokens.starts_with(&["preview".to_string(), "config".to_string()]) {
        return Some("preview-config".to_string());
    }
    if tokens.starts_with(&["manager".to_string(), "config".to_string()]) {
        return Some("manager-config".to_string());
    }
    if tokens.starts_with(&["builder".to_string()]) && tokens.len() >= 2 {
        return Some(format!("builder-{}", tokens[1]));
    }
    if tokens.starts_with(&["addon".to_string()]) && tokens.len() >= 2 {
        return Some(format!("addon-{}", tokens[1]));
    }
    if tokens.starts_with(&["addons".to_string()]) && tokens.len() >= 2 {
        return Some(format!("addons-{}", tokens[1]));
    }
    if tokens.starts_with(&["web".to_string(), "components".to_string()]) {
        return Some("web-components".to_string());
    }
    if matches!(
        tokens[0].as_str(),
        "preview"
            | "store"
            | "types"
            | "hooks"
            | "decorators"
            | "csf"
            | "cli"
            | "react"
            | "vue"
            | "angular"
            | "svelte"
            | "html"
            | "webpack"
            | "vite"
            | "themes"
            | "links"
            | "a11y"
            | "vitest"
            | "playwright"
    ) {
        return Some(tokens[0].clone());
    }
    if tokens.len() >= 2
        && matches!(
            tokens[1].as_str(),
            "api" | "config" | "types" | "type" | "builder" | "framework" | "renderer"
        )
    {
        return Some(format!("{}-{}", tokens[0], tokens[1]));
    }

    Some(tokens[0].clone())
}

fn leaf_tokens(value: &str) -> Vec<String> {
    value
        .replace(['_', '-', '.'], " ")
        .split_whitespace()
        .map(|segment| segment.to_ascii_lowercase())
        .filter(|segment| !segment.is_empty())
        .filter(|segment| {
            !matches!(
                segment.as_str(),
                "test"
                    | "tests"
                    | "spec"
                    | "snapshot"
                    | "stories"
                    | "story"
                    | "example"
                    | "examples"
                    | "basic"
                    | "default"
                    | "index"
            )
        })
        .collect()
}

fn is_low_signal_leaf_source(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains(".test.")
        || lower.contains(".spec.")
        || lower.contains(".stories.")
        || lower.contains(".story.")
        || lower.contains(".snapshot.")
        || lower.contains("/examples/")
        || lower.contains("/example/")
}

fn humanize_leaf_title(value: &str) -> String {
    value
        .replace(['_', '-'], " ")
        .split_whitespace()
        .map(|segment| {
            if segment.chars().all(|character| character.is_ascii_uppercase()) {
                segment.to_string()
            } else if segment.contains('.') {
                segment.to_string()
            } else {
                let mut chars = segment.chars();
                match chars.next() {
                    Some(first) if first.is_ascii_alphabetic() => {
                        format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                    }
                    _ => segment.to_string(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_generic_leaf_key(value: &str) -> bool {
    matches!(
        value,
        "readme"
            | "index"
            | "overview"
            | "intro"
            | "config-index"
            | "api-index"
            | "type-index"
            | "entry"
            | "preview"
            | "manager"
            | "setup"
            | "source"
            | "title"
            | "description"
            | "summary"
            | "helpers"
            | "helper"
            | "utils"
            | "utility"
            | "utilities"
            | "button"
            | "constants"
            | "globals"
            | "types"
            | "typings"
            | "public-types"
            | "main-config"
            | "build-config"
            | "build"
            | "common"
            | "framework"
            | "builders"
            | "builder"
            | "plugin"
            | "plugins"
            | "preset"
            | "presets"
            | "storybook"
            | "codegen"
            | "snippets"
            | "menu"
            | "mockdata"
            | "filesearchlist"
            | "filesearchmodal"
            | "styles"
            | "input"
            | "output"
            | "app"
            | "cookies"
            | "portable"
            | "components"
            | "source-decorator"
            | "compodoc"
            | "webpack"
            | "vite"
            | "actions"
            | "backgrounds"
            | "controls"
            | "a11y"
    )
}

fn leaf_title_duplicates_child(child_title: &str, leaf_title: &str) -> bool {
    let child_slug = slug_key(child_title);
    let leaf_slug = slug_key(leaf_title);
    if child_slug.is_empty() || leaf_slug.is_empty() {
        return false;
    }
    child_slug == leaf_slug
        || child_slug.contains(&leaf_slug)
        || leaf_slug.contains(&child_slug)
}

fn child_profile(
    family_key: &'static str,
    title: &'static str,
    summary: &'static str,
    selectors: Vec<FileSelector>,
) -> FamilyChildProfile {
    FamilyChildProfile {
        family_key,
        title,
        summary,
        selectors,
    }
}

fn prefixes(patterns: &[&'static str]) -> FileSelector {
    FileSelector {
        mode: FileSelectorMode::Prefix,
        patterns: patterns.to_vec(),
    }
}

fn contains(patterns: &[&'static str]) -> FileSelector {
    FileSelector {
        mode: FileSelectorMode::Contains,
        patterns: patterns.to_vec(),
    }
}

fn names(patterns: &[&'static str]) -> FileSelector {
    FileSelector {
        mode: FileSelectorMode::Name,
        patterns: patterns.to_vec(),
    }
}

fn group_matches(report: &ScanReport, group: &SignalGroupSpec) -> bool {
    !select_files(report, &group.selectors).is_empty()
}

fn select_files<'a>(
    report: &'a ScanReport,
    selectors: &[FileSelector],
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    let mut files = Vec::new();
    for selector in selectors {
        files.extend(match selector.mode {
            FileSelectorMode::Prefix => files_under_prefix(report, &selector.patterns),
            FileSelectorMode::Contains => files_matching_substrings_v2(report, &selector.patterns),
            FileSelectorMode::Name => files_matching_name(report, &selector.patterns),
        });
    }
    dedupe_files_by_id_v2(files)
}


fn dedupe_files_by_id_v2<'a>(
    files: Vec<&'a crate::repo::scanner::ScannedFile>,
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for file in files {
        if seen.insert(file.id.clone()) {
            deduped.push(file);
        }
    }
    deduped
}

fn files_matching_substrings_v2<'a>(
    report: &'a ScanReport,
    patterns: &[&str],
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    report
        .files
        .iter()
        .filter(|file| patterns.iter().any(|pattern| file.path.contains(pattern)))
        .collect()
}

fn files_under_prefix<'a>(
    report: &'a ScanReport,
    prefixes: &[&str],
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    report
        .files
        .iter()
        .filter(|file| prefixes.iter().any(|prefix| file.path.starts_with(prefix)))
        .collect()
}

fn files_matching_name<'a>(
    report: &'a ScanReport,
    names: &[&str],
) -> Vec<&'a crate::repo::scanner::ScannedFile> {
    report
        .files
        .iter()
        .filter(|file| names.iter().any(|name| file.path.ends_with(name)))
        .collect()
}

fn collect_module_ids_for_paths(module_tree: &ModuleTree, paths: &[String]) -> Vec<String> {
    module_tree
        .modules
        .iter()
        .filter(|module| {
            module.root_paths.iter().any(|root| {
                paths.iter()
                    .any(|path| path.starts_with(root.as_str()) || root.starts_with(path.as_str()))
            })
        })
        .map(|module| module.id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn dedupe_vec(items: Vec<String>) -> Vec<String> {
    items.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
}

fn family_index_page_id(family: &FamilyCandidate) -> String {
    stable_id("page", &format!("family:index:{}:{}", family.family_kind, family.family_key))
}

fn family_child_page_id(child: &FamilyChildCandidate) -> String {
    stable_id("page", &format!("family:child:{}:{}", child.family_kind, child.family_key))
}

fn family_leaf_page_id(leaf: &FamilyLeafCandidate) -> String {
    stable_id(
        "page",
        &format!(
            "family:leaf:{}:{}:{}",
            leaf.family_kind, leaf.child_key, leaf.leaf_key
        ),
    )
}

fn family_index_page_path(family: &FamilyCandidate) -> String {
    format!(
        "{}/{}.md",
        family.family_directory,
        slugify_segment(&family.title)
    )
}

fn family_child_page_path(child: &FamilyChildCandidate) -> String {
    format!(
        "{}/{}.md",
        child.family_directory,
        slugify_segment(&child.title)
    )
}

fn family_leaf_page_path(leaf: &FamilyLeafCandidate) -> String {
    format!(
        "{}/{}/{}.md",
        leaf.family_directory,
        slugify_segment(&leaf.child_title),
        slugify_segment(&leaf.title)
    )
}

fn overview_page_path(profile: Option<&RepoArchetypeProfile>) -> String {
    profile
        .map(|profile| profile.overview_path.clone())
        .unwrap_or_else(|| "项目概述.md".to_string())
}

fn architecture_page_path(profile: Option<&RepoArchetypeProfile>) -> String {
    profile
        .map(|profile| profile.architecture_path.clone())
        .unwrap_or_else(|| "系统架构.md".to_string())
}

fn architecture_page_title(profile: Option<&RepoArchetypeProfile>) -> String {
    profile
        .map(|profile| profile.architecture_title.clone())
        .unwrap_or_else(|| "系统架构".to_string())
}

fn discover_root_doc_pages(
    report: &ScanReport,
    module_tree: &ModuleTree,
    profile: Option<&RepoArchetypeProfile>,
    overview_id: &str,
    base_priority: usize,
) -> Vec<PlannedPage> {
    let Some(profile) = profile else {
        return Vec::new();
    };

    profile
        .root_docs
        .iter()
        .enumerate()
        .filter_map(|(index, root_doc)| {
            build_root_doc_page(report, module_tree, overview_id, base_priority + index, root_doc)
        })
        .collect()
}

fn build_root_doc_page(
    report: &ScanReport,
    module_tree: &ModuleTree,
    overview_id: &str,
    priority: usize,
    root_doc: &RootDocProfile,
) -> Option<PlannedPage> {
    let files = select_files(report, &root_doc.selectors);
    let files = dedupe_files_by_id_v2(files);
    if files.is_empty() {
        return None;
    }
    let source_ids = files.iter().map(|file| file.id.clone()).collect::<Vec<_>>();
    let paths = files.iter().map(|file| file.path.clone()).collect::<Vec<_>>();

    Some(PlannedPage {
        id: stable_id("page", &format!("root-doc:{}", root_doc.topic_key)),
        title: root_doc.title.to_string(),
        relative_path: root_doc.relative_path.to_string(),
        page_type: "topic".to_string(),
        parent_id: Some(overview_id.to_string()),
        scope: format!("topic:repo-guide:{}", root_doc.topic_key),
        source_ids,
        module_ids: collect_module_ids_for_paths(module_tree, &paths),
        relation_ids: Vec::new(),
        generation_mode: "deterministic:topic:repo-guide".to_string(),
        priority,
        merged_module_ids: Vec::new(),
        unit_id: None,
        unit_type: None,
        domain_id: None,
    })
}

fn build_family_index_page(
    family: &FamilyCandidate,
    overview_id: &str,
    priority: usize,
) -> PlannedPage {
    PlannedPage {
        id: family_index_page_id(family),
        title: family.title.clone(),
        relative_path: family_index_page_path(family),
        page_type: "family-index".to_string(),
        parent_id: Some(overview_id.to_string()),
        scope: format!("family:{}", family.family_key),
        source_ids: family.source_ids.clone(),
        module_ids: family.module_ids.clone(),
        relation_ids: family.relation_ids.clone(),
        generation_mode: format!("deterministic:family-index:{}", family.family_kind),
        priority,
        merged_module_ids: Vec::new(),
        unit_id: None,
        unit_type: None,
        domain_id: None,
    }
}

fn build_family_child_page(
    child: &FamilyChildCandidate,
    parent_id: &str,
    priority: usize,
) -> PlannedPage {
    PlannedPage {
        id: family_child_page_id(child),
        title: child.title.clone(),
        relative_path: family_child_page_path(child),
        page_type: "family-child".to_string(),
        parent_id: Some(parent_id.to_string()),
        scope: format!("family-child:{}:{}", child.family_kind, child.family_key),
        source_ids: child.source_ids.clone(),
        module_ids: child.module_ids.clone(),
        relation_ids: child.relation_ids.clone(),
        generation_mode: format!("deterministic:family-child:{}", child.family_kind),
        priority,
        merged_module_ids: Vec::new(),
        unit_id: None,
        unit_type: None,
        domain_id: None,
    }
}

fn build_family_leaf_page(
    leaf: &FamilyLeafCandidate,
    parent_id: &str,
    priority: usize,
) -> PlannedPage {
    PlannedPage {
        id: family_leaf_page_id(leaf),
        title: leaf.title.clone(),
        relative_path: family_leaf_page_path(leaf),
        page_type: "family-leaf-doc".to_string(),
        parent_id: Some(parent_id.to_string()),
        scope: format!(
            "family-leaf:{}:{}:{}",
            leaf.family_kind, leaf.child_key, leaf.leaf_key
        ),
        source_ids: leaf.source_ids.clone(),
        module_ids: leaf.module_ids.clone(),
        relation_ids: leaf.relation_ids.clone(),
        generation_mode: format!("deterministic:family-leaf-doc:{}", leaf.family_kind),
        priority,
        merged_module_ids: Vec::new(),
        unit_id: None,
        unit_type: None,
        domain_id: None,
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

// ─── Knowledge Tree → PlannedPage 映射 ──────────────────────

/// 从 KnowledgeTree 生成 PlannedPage 列表。
/// 这是 2.0 pipeline 的新入口，用 KnowledgeUnit 统一取代旧的 module/topic/family 三路规划。
pub fn plan_pages_from_knowledge_tree(tree: &KnowledgeTree) -> Vec<PlannedPage> {
    tree.processing_order
        .iter()
        .filter_map(|unit_id| tree.get_unit(unit_id))
        .map(|unit| knowledge_unit_to_planned_page(unit, tree))
        .collect()
}

fn knowledge_unit_to_planned_page(unit: &KnowledgeUnit, tree: &KnowledgeTree) -> PlannedPage {
    let page_type = unit_type_to_page_type(&unit.unit_type);
    let scope = unit_type_to_scope(&unit.unit_type);

    let parent_id = unit.parent_unit_id.as_ref().and_then(|pid| {
        tree.get_unit(pid)
            .map(|parent| stable_id("page", &parent.relative_path))
    });

    PlannedPage {
        id: stable_id("page", &unit.relative_path),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        page_type: page_type.to_string(),
        parent_id,
        scope: scope.to_string(),
        source_ids: unit.scope.source_ids.clone(),
        module_ids: unit.scope.module_ids.clone(),
        relation_ids: unit.scope.relation_ids.clone(),
        generation_mode: "knowledge-tree".to_string(),
        priority: (unit.priority * 100.0) as usize,
        merged_module_ids: Vec::new(),
        unit_id: None,
        unit_type: None,
        domain_id: None,
    }
}

fn unit_type_to_page_type(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview => "overview",
        UnitType::Architecture => "architecture",
        UnitType::DomainIndex => "domain-index",
        UnitType::ModuleDoc => "module",
        UnitType::ApiDoc => "api-reference",
        UnitType::ConfigDoc => "config-reference",
        UnitType::ConceptGuide => "concept-guide",
        UnitType::WorkflowDoc => "workflow",
        UnitType::TroubleshootDoc => "troubleshoot",
        UnitType::IntegrationDoc => "integration",
        UnitType::TestDoc => "test-doc",
        UnitType::ExampleDoc => "example",
    }
}

fn unit_type_to_scope(unit_type: &UnitType) -> &'static str {
    match unit_type {
        UnitType::Overview | UnitType::Architecture => "repository",
        UnitType::DomainIndex => "domain",
        _ => "unit",
    }
}
