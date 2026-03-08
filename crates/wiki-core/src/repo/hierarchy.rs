use std::collections::{BTreeMap, BTreeSet};

use crate::domain::module_tree::{ModuleNode, ModuleTree, RelationEdge};
use crate::domain::stable_id::stable_id;
use crate::repo::detectors::detect_tech_hints;
use crate::repo::scanner::{DependencyHint, ScanReport, ScannedFile};

/// 基于扫描结果构建模块树。
/// 这一层的目标不是做“完美架构分析”，而是把仓库稳定地切成可用于页面规划的模块层级。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
///
/// # 返回
/// - 返回供页面规划、metadata 和 query 使用的模块树。
pub fn build_module_tree(report: &ScanReport) -> ModuleTree {
    let root_name = repo_name_from_root(&report.root);
    let root_id = stable_id("module", &report.root);
    let child_roots = discover_child_module_roots(report);

    let mut modules = Vec::new();
    let mut child_ids = Vec::new();

    for child_root in &child_roots {
        let node = build_child_module(report, child_root, &root_id);
        child_ids.push(node.id.clone());
        modules.push(node);
    }

    let root_module = ModuleNode {
        id: root_id.clone(),
        name: root_name,
        kind: "repository".to_string(),
        root_paths: vec![".".to_string()],
        source_ids: report.files.iter().map(|file| file.id.clone()).collect(),
        parent_id: None,
        child_ids,
        entry_points: report.entry_points.clone(),
        tags: report.tech_hints.clone(),
    };

    modules.insert(0, root_module);

    let cross_module_edges = build_cross_module_edges(report, &modules);
    let architecture_hints = build_architecture_hints(report, &modules, &cross_module_edges);

    ModuleTree {
        root_modules: vec![root_id],
        modules,
        cross_module_edges,
        architecture_hints,
    }
}

/// 发现子模块根路径。
/// 顺序上先信任显式 workspace/member，再补固定目录约定，最后用启发式兜底顶层业务目录。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
///
/// # 返回
/// - 返回应被提升为子模块的根路径列表。
fn discover_child_module_roots(report: &ScanReport) -> Vec<String> {
    let mut roots = BTreeSet::new();

    for workspace_root in &report.workspace_roots {
        if workspace_root != "." {
            roots.insert(workspace_root.clone());
        }
    }

    for file in &report.files {
        if let Some(candidate) = top_level_boundary(&file.path) {
            roots.insert(candidate);
        }
    }

    // 除了显式 workspace/member 之外，还要兜底识别“有明显业务含义的顶层目录”。
    // 这是混合仓库的关键：像 web、spider、nginx 这类目录不能因为不在固定白名单里就被吞进根模块。
    for candidate in discover_meaningful_top_level_roots(report) {
        roots.insert(candidate);
    }

    if roots.is_empty() && !report.files.is_empty() {
        roots.insert(".".to_string());
    }

    roots.into_iter().collect()
}

/// 固定目录约定主要服务 monorepo 结构。
/// 它能快速识别 `crates/wiki-core`、`agents/codebuddy` 这类明确成员。
///
/// # 参数
/// - `path`：仓库内相对路径。
///
/// # 返回
/// - 如果路径命中固定目录约定，则返回对应的模块根路径。
fn top_level_boundary(path: &str) -> Option<String> {
    let mut segments = path.split('/').collect::<Vec<_>>();
    if segments.len() < 2 {
        return None;
    }

    let group = segments.remove(0);
    let name = segments.remove(0);

    match group {
        "crates" | "agents" | "apps" | "services" | "libs" | "packages" => {
            Some(format!("{group}/{name}"))
        }
        "src" => Some(".".to_string()),
        _ => None,
    }
}

/// 这一组统计字段专门服务“顶层目录是否应晋升为模块”的评分。
#[derive(Default)]
struct TopLevelRootStats {
    total_files: usize,
    source_files: usize,
    config_files: usize,
    entry_points: usize,
    languages: BTreeSet<String>,
    tags: BTreeSet<String>,
}

/// 兜底识别“看起来就是一个独立子系统”的顶层目录。
/// 这一步专门解决混合仓库场景：目录没有出现在固定白名单里，但明明是独立模块。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
///
/// # 返回
/// - 返回通过启发式评分后应被提升为模块的顶层目录列表。
fn discover_meaningful_top_level_roots(report: &ScanReport) -> Vec<String> {
    let mut stats_by_root = BTreeMap::new();

    for file in &report.files {
        let Some(root_path) = top_level_segment(&file.path) else {
            continue;
        };

        if should_skip_top_level_root(root_path) {
            continue;
        }

        let stats = stats_by_root
            .entry(root_path.to_string())
            .or_insert_with(TopLevelRootStats::default);
        observe_top_level_file(stats, file);
    }

    stats_by_root
        .into_iter()
        .filter_map(|(root_path, stats)| stats.should_promote().then_some(root_path))
        .collect()
}

/// 这里只取路径的第一段，因为我们判断的是“顶层目录是否可以成为模块”。
///
/// # 参数
/// - `path`：仓库内相对路径。
///
/// # 返回
/// - 如果路径存在顶层目录段，则返回该目录名。
fn top_level_segment(path: &str) -> Option<&str> {
    path.split('/').next().filter(|segment| !segment.is_empty())
}

/// 顶层目录如果本身就是缓存/依赖/编辑器目录，就不参与模块评分。
///
/// # 参数
/// - `root_path`：顶层目录名。
///
/// # 返回
/// - 如果该目录应被顶层模块评分逻辑跳过，则返回 `true`。
fn should_skip_top_level_root(root_path: &str) -> bool {
    matches!(
        root_path,
        ".wiki"
            | ".git"
            | ".hg"
            | ".svn"
            | "node_modules"
            | "target"
            | "dist"
            | "logs"
            | "log"
            | "temp"
            | "tmp"
            | "__pycache__"
            | ".idea"
            | ".vscode"
            | ".cursor"
            | ".venv"
            | "venv"
    )
}

/// 把单个文件提供的信号累积到顶层目录统计里。
/// 这里收集的是入口、源码、配置和技术类型等“能稳定反映子系统形态”的信息。
///
/// # 参数
/// - `stats`：当前顶层目录的累计统计信息。
/// - `file`：要纳入统计的文件记录。
///
/// # 返回
/// - 该函数直接更新 `stats`，不单独返回结果。
fn observe_top_level_file(stats: &mut TopLevelRootStats, file: &ScannedFile) {
    stats.total_files += 1;

    if file.kind == "source" {
        stats.source_files += 1;
    }

    if file.kind == "config" {
        stats.config_files += 1;
    }

    if file.tags.iter().any(|tag| tag == "entry-point") {
        stats.entry_points += 1;
    }

    if !matches!(
        file.language.as_str(),
        "json" | "markdown" | "yaml" | "text" | "asset"
    ) {
        stats.languages.insert(file.language.clone());
    }

    if matches!(
        file.language.as_str(),
        "python" | "rust" | "java" | "csharp" | "kotlin" | "php" | "swift"
    ) {
        stats.tags.insert("backend".to_string());
    }

    if matches!(
        file.language.as_str(),
        "typescript" | "javascript" | "react" | "vue" | "svelte"
    ) {
        stats.tags.insert("frontend".to_string());
    }

    if file.path.ends_with("nginx.conf") {
        stats.tags.insert("infrastructure".to_string());
    }
}

impl TopLevelRootStats {
    /// 模块晋升评分是刻意保守的。
    /// 只要目录同时具备入口、源码、配置或基础设施信号中的几项，就应先被提升为模块。
    fn should_promote(&self) -> bool {
        let mut score = 0;

        if self.entry_points > 0 {
            score += 3;
        }

        if self.source_files >= 3 {
            score += 2;
        } else if self.source_files > 0 {
            score += 1;
        }

        if self.config_files > 0 {
            score += 1;
        }

        if self.tags.contains("frontend") || self.tags.contains("backend") {
            score += 1;
        }

        if self.tags.contains("infrastructure") {
            score += 2;
        }

        if !self.languages.is_empty() {
            score += 1;
        }

        // 这里不是要做“绝对精确”的模块判定，而是给层级拆分一个稳定及格线：
        // 只要一个顶层目录同时具备入口、源码、配置或基础设施信号中的几项，就应该先被提升为模块。
        score >= 3
    }
}

/// 根据模块根路径构建子模块节点。
/// 这里会同时补齐页面规划需要的 `kind / tags / entry_points / source_ids`。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `root_path`：当前子模块的根路径。
/// - `parent_id`：父模块的稳定 ID。
///
/// # 返回
/// - 返回带完整基础信息的子模块节点。
fn build_child_module(report: &ScanReport, root_path: &str, parent_id: &str) -> ModuleNode {
    let module_name = module_name_from_root(root_path);
    let module_id = stable_id("module", root_path);
    let source_files = files_for_root(report, root_path);
    let tags = module_tags(&source_files, report, root_path);

    ModuleNode {
        id: module_id,
        name: module_name,
        kind: module_kind(report, root_path, &tags),
        root_paths: vec![root_path.to_string()],
        source_ids: source_files.iter().map(|file| file.id.clone()).collect(),
        parent_id: Some(parent_id.to_string()),
        child_ids: Vec::new(),
        entry_points: report
            .entry_points
            .iter()
            .filter(|path| path_belongs_to_root(path, root_path))
            .cloned()
            .collect(),
        tags,
    }
}

/// 找出归属于某个模块根路径的全部文件。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `root_path`：当前模块根路径。
///
/// # 返回
/// - 返回所有属于该模块根路径的文件引用列表。
fn files_for_root<'a>(report: &'a ScanReport, root_path: &str) -> Vec<&'a ScannedFile> {
    report
        .files
        .iter()
        .filter(|file| path_belongs_to_root(&file.path, root_path))
        .collect()
}

/// 模块标签不再继承整个仓库的技术栈，而是根据模块自己的文件重新判断。
/// 这样 `web` 不会错误带上 `backend`，`spider` 也不会带上 `frontend`。
///
/// # 参数
/// - `source_files`：当前模块包含的文件集合。
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `root_path`：当前模块根路径。
///
/// # 返回
/// - 返回当前模块自己的标签列表。
fn module_tags(source_files: &[&ScannedFile], report: &ScanReport, root_path: &str) -> Vec<String> {
    let mut tags = BTreeSet::new();
    let paths = source_files
        .iter()
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();

    for tag in detect_tech_hints(&paths) {
        tags.insert(tag.clone());
    }

    if source_files.iter().any(|file| {
        matches!(
            file.language.as_str(),
            "python" | "rust" | "java" | "csharp" | "kotlin" | "php" | "swift"
        )
    }) {
        tags.insert("backend".to_string());
    }

    if source_files.iter().any(|file| {
        matches!(
            file.language.as_str(),
            "typescript" | "javascript" | "react" | "vue" | "svelte" | "css" | "html"
        )
    }) {
        tags.insert("frontend".to_string());
    }

    if source_files
        .iter()
        .any(|file| file.path.ends_with("nginx.conf"))
    {
        tags.insert("infrastructure".to_string());
    }

    if report
        .entry_points
        .iter()
        .any(|path| path_belongs_to_root(path, root_path))
    {
        tags.insert("entry".to_string());
    }

    if source_files.iter().any(|file| file.kind == "config") {
        tags.insert("config".to_string());
    }

    tags.into_iter().collect()
}

/// `kind` 是页面和后续 query 的粗粒度角色说明。
/// 先把“前端 / 后端 / 基础设施”区分出来，比统一叫 workspace-member 更有用。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `root_path`：当前模块根路径。
/// - `tags`：当前模块已经识别出的标签列表。
///
/// # 返回
/// - 返回适合落到 metadata 和页面中的模块角色类型。
fn module_kind(report: &ScanReport, root_path: &str, tags: &[String]) -> String {
    if root_path == "." {
        return "application".to_string();
    }

    if tags.iter().any(|tag| tag == "infrastructure") {
        return "infrastructure".to_string();
    }

    if tags.iter().any(|tag| tag == "frontend") {
        return "frontend-app".to_string();
    }

    if tags.iter().any(|tag| tag == "backend") {
        return "backend-service".to_string();
    }

    if report.workspace_roots.iter().any(|workspace_root| workspace_root == root_path) {
        return "workspace-member".to_string();
    }

    "module".to_string()
}

/// 判断一个路径是否属于某个模块根路径。
/// 根模块 `"."` 只兜底接住“没有被任何子模块显式吃掉的路径”。
///
/// # 参数
/// - `path`：待判断的仓库内相对路径。
/// - `root_path`：模块根路径。
///
/// # 返回
/// - 如果路径属于该模块，则返回 `true`。
fn path_belongs_to_root(path: &str, root_path: &str) -> bool {
    if root_path == "." {
        return !matches!(
            top_level_boundary(path).as_deref(),
            Some(boundary) if boundary != "."
        );
    }

    path == root_path || path.starts_with(&format!("{root_path}/"))
}

/// 把文件级依赖线索折叠成模块级关系。
/// 这一层只关心“模块之间是否有证据表明存在依赖”。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `modules`：当前仓库的模块节点列表。
///
/// # 返回
/// - 返回去重后的跨模块关系边列表。
fn build_cross_module_edges(report: &ScanReport, modules: &[ModuleNode]) -> Vec<RelationEdge> {
    let mut edges = BTreeMap::new();

    for dependency in &report.dependency_hints {
        if let Some(edge) = map_dependency_to_module_edge(dependency, modules) {
            edges.entry(edge.id.clone()).or_insert(edge);
        }
    }

    edges.into_values().collect()
}

/// 把单条依赖线索映射成模块边。
/// 如果依赖仍然落在同一个模块里，或者一端是根模块，就不会生成跨模块关系。
///
/// # 参数
/// - `dependency`：单条文件级依赖线索。
/// - `modules`：当前仓库的模块节点列表。
///
/// # 返回
/// - 如果该依赖能被提升为有效的跨模块关系，则返回对应关系边。
fn map_dependency_to_module_edge(
    dependency: &DependencyHint,
    modules: &[ModuleNode],
) -> Option<RelationEdge> {
    let source_module = find_best_module_for_path(&dependency.from, modules)?;
    let target_module = find_best_module_for_path(&dependency.to, modules)?;

    if source_module.id == target_module.id {
        return None;
    }

    // 根模块只是仓库容器，不应该成为“跨模块依赖”的一端。
    // 否则像前端引第三方包这类噪音，很容易被错误映射成“web -> repository”。
    if source_module.parent_id.is_none() || target_module.parent_id.is_none() {
        return None;
    }

    let edge_seed = format!("{}:{}:{}", source_module.id, target_module.id, dependency.kind);

    Some(RelationEdge {
        id: stable_id("relation", edge_seed),
        source: source_module.id.clone(),
        target: target_module.id.clone(),
        relation_type: dependency.kind.clone(),
        evidence: vec![dependency.from.clone(), dependency.to.clone()],
    })
}

/// 在所有可匹配模块中，优先选根路径最长的那个。
/// 这样可以避免 `web/src/*` 被误匹配到更宽泛的父级模块。
///
/// # 参数
/// - `path`：待映射的仓库内相对路径。
/// - `modules`：当前仓库的模块节点列表。
///
/// # 返回
/// - 如果找到最匹配的模块，则返回对应模块引用。
fn find_best_module_for_path<'a>(path: &str, modules: &'a [ModuleNode]) -> Option<&'a ModuleNode> {
    modules
        .iter()
        .filter(|module| {
            module
                .root_paths
                .iter()
                .any(|root_path| path_belongs_to_root(path, root_path))
        })
        .max_by_key(|module| {
            module
                .root_paths
                .iter()
                .map(|root_path| root_path.len())
                .max()
                .unwrap_or_default()
        })
}

/// 架构提示是系统架构页的最小输入。
/// 它不追求完整说明，只提供“技术栈 / 顶层模块 / 关系数量 / 关键入口”这些硬事实。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
/// - `modules`：当前仓库的模块节点列表。
/// - `edges`：当前仓库的跨模块关系边列表。
///
/// # 返回
/// - 返回可直接写入系统架构页的提示信息列表。
fn build_architecture_hints(
    report: &ScanReport,
    modules: &[ModuleNode],
    edges: &[RelationEdge],
) -> Vec<String> {
    let mut hints = Vec::new();
    hints.push(format!("技术栈：{}", join_or_default(&report.tech_hints, "未识别")));
    hints.push(format!("模块数量：{}", modules.iter().filter(|module| module.parent_id.is_some()).count()));
    hints.push(format!("跨模块关系：{}", edges.len()));
    hints.push(format!(
        "顶层模块：{}",
        join_or_default(
            &modules
                .iter()
                .filter(|module| module.parent_id.is_some())
                .map(|module| module.name.clone())
                .collect::<Vec<_>>(),
            "未识别"
        )
    ));

    if !report.entry_points.is_empty() {
        hints.push(format!(
            "关键入口：{}",
            join_or_default(&report.entry_points, "无")
        ));
    }

    hints
}

/// 从仓库根路径生成展示名称。
///
/// # 参数
/// - `root`：仓库根目录路径字符串。
///
/// # 返回
/// - 返回用于页面展示的仓库名称。
fn repo_name_from_root(root: &str) -> String {
    root.replace('\\', "/")
        .trim_end_matches('/')
        .split('/')
        .next_back()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("repository")
        .to_string()
}

/// 模块显示名默认取根路径最后一段。
///
/// # 参数
/// - `root_path`：模块根路径。
///
/// # 返回
/// - 返回用于页面展示的模块名称。
fn module_name_from_root(root_path: &str) -> String {
    if root_path == "." {
        return "主模块".to_string();
    }

    root_path
        .split('/')
        .next_back()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("module")
        .to_string()
}

/// 把列表拼成稳定可读的中文文案。
///
/// # 参数
/// - `values`：待拼接的字符串列表。
/// - `fallback`：列表为空时使用的兜底文案。
///
/// # 返回
/// - 返回适合写入页面的中文拼接结果。
fn join_or_default(values: &[String], fallback: &str) -> String {
    if values.is_empty() {
        fallback.to_string()
    } else {
        values.join("、")
    }
}
