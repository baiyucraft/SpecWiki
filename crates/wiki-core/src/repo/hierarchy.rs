use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::domain::module_tree::{ModuleNode, ModuleTree, RelationEdge};
use crate::domain::stable_id::stable_id;
use crate::repo::detectors::detect_tech_hints;
use crate::repo::scanner::{DependencyHint, ScanReport, ScannedFile};

/// 模块根路径发现结果会被后续建树和页面规划共用。
/// 这里把"显式根路径"和"为递归层级补出的祖先根路径"分开保存，避免后续再重复推断。
struct ModuleRootDiscovery {
    explicit_roots: BTreeSet<String>,
    parent_by_root: BTreeMap<String, String>,
    child_roots_by_parent: BTreeMap<String, Vec<String>>,
    ordered_roots: Vec<String>,
}

/// 基于扫描结果构建模块树。
/// 这一层的目标不是做"完美架构分析"，而是把仓库稳定地切成可用于页面规划的模块层级。
///
/// # 参数
/// - `report`：仓库扫描阶段生成的扫描报告。
///
/// # 返回
/// - 返回供页面规划、metadata 和 query 使用的模块树。
pub fn build_module_tree(report: &ScanReport) -> ModuleTree {
    let root_name = repo_name_from_root(&report.root);
    let root_id = stable_id("module", &report.root);
    let discovery = discover_module_roots(report);

    let root_module = ModuleNode {
        id: root_id.clone(),
        name: root_name,
        kind: "repository".to_string(),
        root_paths: vec![".".to_string()],
        source_ids: report.files.iter().map(|file| file.id.clone()).collect(),
        parent_id: None,
        child_ids: child_ids_for_parent(".", &discovery.child_roots_by_parent),
        entry_points: report.entry_points.clone(),
        tags: report.tech_hints.clone(),
    };

    let mut modules = vec![root_module];

    for root_path in &discovery.ordered_roots {
        modules.push(build_module_node(
            report,
            root_path,
            &root_id,
            &discovery.explicit_roots,
            &discovery.parent_by_root,
            &discovery.child_roots_by_parent,
        ));
    }

    let cross_module_edges = build_cross_module_edges(report, &modules);
    let architecture_hints = build_architecture_hints(report, &modules, &cross_module_edges);

    ModuleTree {
        root_modules: vec![root_id],
        modules,
        cross_module_edges,
        architecture_hints,
    }
}

/// 发现所有显式模块根路径，并补齐递归层级需要的祖先节点。
/// 这样像 `packages/domain/auth` 这类路径会自然形成 `packages -> packages/domain -> packages/domain/auth`。
fn discover_module_roots(report: &ScanReport) -> ModuleRootDiscovery {
    let explicit_roots = discover_explicit_module_roots(report);
    let mut all_roots = explicit_roots.clone();

    for root_path in &explicit_roots {
        for ancestor in ancestor_roots(root_path) {
            all_roots.insert(ancestor);
        }
    }

    let mut parent_by_root = BTreeMap::new();
    let mut child_roots_by_parent = BTreeMap::new();

    for root_path in &all_roots {
        let parent_root =
            nearest_parent_root(root_path, &all_roots).unwrap_or_else(|| ".".to_string());
        parent_by_root.insert(root_path.clone(), parent_root.clone());
        child_roots_by_parent
            .entry(parent_root)
            .or_insert_with(Vec::new)
            .push(root_path.clone());
    }

    for children in child_roots_by_parent.values_mut() {
        children.sort();
    }

    let mut ordered_roots = Vec::new();
    collect_preorder_roots(".", &child_roots_by_parent, &mut ordered_roots);

    ModuleRootDiscovery {
        explicit_roots,
        parent_by_root,
        child_roots_by_parent,
        ordered_roots,
    }
}

/// 显式模块根路径来自 workspace/member、固定边界和混合仓库的顶层目录识别。
fn discover_explicit_module_roots(report: &ScanReport) -> BTreeSet<String> {
    let mut roots = BTreeSet::new();

    for workspace_root in &report.workspace_roots {
        if workspace_root != "." {
            roots.insert(workspace_root.clone());
        }
    }

    for file in &report.files {
        if let Some(candidate) = top_level_boundary(&file.path).filter(|candidate| candidate != ".")
        {
            roots.insert(candidate);
        }
    }

    for candidate in discover_meaningful_top_level_roots(report) {
        if candidate != "." {
            roots.insert(candidate);
        }
    }

    roots
}

/// 为显式模块根路径补齐所有祖先目录，供递归模块树使用。
fn ancestor_roots(root_path: &str) -> Vec<String> {
    let segments = root_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut roots = Vec::new();

    for end in 1..segments.len() {
        roots.push(segments[..end].join("/"));
    }

    roots
}

/// 当前模块的父模块总是取"最长可匹配祖先根路径"。
/// 这样 `packages/domain/auth` 会挂到 `packages/domain`，而不是直接挂到 `packages` 或根模块。
fn nearest_parent_root(root_path: &str, all_roots: &BTreeSet<String>) -> Option<String> {
    ancestor_roots(root_path)
        .into_iter()
        .filter(|candidate| all_roots.contains(candidate))
        .max_by_key(|candidate| candidate.len())
}

/// 预先按先序遍历收集模块根路径，保证父节点总在子节点之前出现。
fn collect_preorder_roots(
    parent_root: &str,
    child_roots_by_parent: &BTreeMap<String, Vec<String>>,
    ordered_roots: &mut Vec<String>,
) {
    let Some(children) = child_roots_by_parent.get(parent_root) else {
        return;
    };

    for child_root in children {
        ordered_roots.push(child_root.clone());
        collect_preorder_roots(child_root, child_roots_by_parent, ordered_roots);
    }
}

/// 根据模块根路径构建模块节点。
/// 这里会同时补齐页面规划需要的 `kind / tags / entry_points / source_ids / child_ids`。
fn build_module_node(
    report: &ScanReport,
    root_path: &str,
    root_id: &str,
    explicit_roots: &BTreeSet<String>,
    parent_by_root: &BTreeMap<String, String>,
    child_roots_by_parent: &BTreeMap<String, Vec<String>>,
) -> ModuleNode {
    let module_name = module_name_from_root(root_path);
    let module_id = stable_id("module", root_path);
    let source_files = files_for_root(report, root_path);
    let tags = module_tags(&source_files, report, root_path);
    let child_ids = child_ids_for_parent(root_path, child_roots_by_parent);
    let has_children = !child_ids.is_empty();
    let parent_id = parent_by_root.get(root_path).map(|parent_root| {
        if parent_root == "." {
            root_id.to_string()
        } else {
            stable_id("module", parent_root)
        }
    });

    ModuleNode {
        id: module_id,
        name: module_name,
        kind: module_kind(
            report,
            root_path,
            &tags,
            explicit_roots.contains(root_path),
            has_children,
            &source_files,
        ),
        root_paths: vec![root_path.to_string()],
        source_ids: source_files.iter().map(|file| file.id.clone()).collect(),
        parent_id,
        child_ids,
        entry_points: report
            .entry_points
            .iter()
            .filter(|path| path_belongs_to_root(path, root_path))
            .cloned()
            .collect(),
        tags,
    }
}

/// 把某个父模块下的子根路径列表转成稳定模块 ID。
fn child_ids_for_parent(
    parent_root: &str,
    child_roots_by_parent: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    child_roots_by_parent
        .get(parent_root)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|child_root| stable_id("module", child_root))
        .collect()
}

/// 固定目录约定主要服务 monorepo 结构。
/// 它能快速识别 `crates/wiki-core`、`agents/codebuddy` 这类明确成员。
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

/// 这一组统计字段专门服务"顶层目录是否应晋升为模块"的评分。
#[derive(Default)]
struct TopLevelRootStats {
    total_files: usize,
    source_files: usize,
    config_files: usize,
    entry_points: usize,
    has_subdirs: bool,
    test_files: usize,
    languages: BTreeSet<String>,
    tags: BTreeSet<String>,
}

/// 兜底识别"看起来就是一个独立子系统"的顶层目录。
/// 这一步专门解决混合仓库场景：目录没有出现在固定白名单里，但明明是独立模块。
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
        .filter_map(|(root_path, stats)| {
            // 单文件模块抑制：只含 1 个文件且无子目录的候选节点不提升为独立模块
            if stats.total_files <= 1 && !stats.has_subdirs {
                return None;
            }
            stats.should_promote().then_some(root_path)
        })
        .collect()
}

/// 这里只取路径的第一段，因为我们判断的是"顶层目录是否可以成为模块"。
fn top_level_segment(path: &str) -> Option<&str> {
    path.split('/').next().filter(|segment| !segment.is_empty())
}

/// 顶层目录如果本身就是缓存/依赖/编辑器目录，就不参与模块评分。
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
fn observe_top_level_file(stats: &mut TopLevelRootStats, file: &ScannedFile) {
    stats.total_files += 1;

    // 检测是否有子目录（路径段数 > 2 说明不是直接在顶层目录下）
    let depth = file.path.split('/').count();
    if depth > 2 {
        stats.has_subdirs = true;
    }

    // test-file 标记的文件单独计数，用于评分降权
    if file.is_test_like() {
        stats.test_files += 1;
    }

    if file.kind == "source" && !file.is_low_signal() {
        stats.source_files += 1;
    }

    if file.is_config_like() {
        stats.config_files += 1;
    }

    if file.is_entry_like() {
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
    /// test-file 标记的文件在评分中被降权，避免测试产物主导模块提升。
    fn should_promote(&self) -> bool {
        let mut score = 0;

        if self.entry_points > 0 {
            score += 3;
        }

        // 非 test 源码文件数量用于评分，test 文件不计入
        let non_test_source_files = self.source_files.saturating_sub(self.test_files);
        if non_test_source_files >= 3 {
            score += 2;
        } else if non_test_source_files > 0 {
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

        score >= 3
    }
}

/// 找出归属于某个模块根路径的全部文件。
fn files_for_root<'a>(report: &'a ScanReport, root_path: &str) -> Vec<&'a ScannedFile> {
    report
        .files
        .iter()
        .filter(|file| path_belongs_to_root(&file.path, root_path))
        .collect()
}

/// 模块标签不再继承整个仓库的技术栈，而是根据模块自己的文件重新判断。
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

    if source_files.iter().any(|file| file.is_config_like()) {
        tags.insert("config".to_string());
    }

    tags.into_iter().collect()
}

/// `kind` 是页面和 query 的粗粒度角色说明。
/// 综合 manifest 类型、入口文件、目录结构、模块标签和目录名/manifest 关键词判断。
/// 优先级：module-group 结构判断 > manifest 声明 > 入口文件 > 目录结构 > 标签 > 默认值。
///
/// # 参数
/// - `report`：仓库扫描报告，用于读取 manifest 和 workspace 信息。
/// - `root_path`：模块根路径。
/// - `tags`：模块标签列表。
/// - `is_explicit_root`：是否为显式模块根路径（workspace 成员或固定边界）。
/// - `has_children`：是否有子模块。
/// - `source_files`：归属于该模块的文件列表。
///
/// # 返回
/// - 返回模块的 kind 字符串，如 `library`、`cli-tool`、`infrastructure` 等。
fn module_kind(
    report: &ScanReport,
    root_path: &str,
    tags: &[String],
    is_explicit_root: bool,
    has_children: bool,
    source_files: &[&ScannedFile],
) -> String {
    if root_path == "." {
        return "application".to_string();
    }

    // 组节点判断保持不变
    if has_children && (!is_explicit_root || !has_structural_manifest(report, root_path)) {
        return "module-group".to_string();
    }

    // 收集 manifest 信号
    let manifest_signal = detect_manifest_signal(report, root_path);

    // 1. manifest 声明优先
    if let Some(kind) = kind_from_manifest(&manifest_signal, root_path) {
        return kind;
    }

    // 2. 入口文件信号
    let has_main_entry = source_files.iter().any(|f| {
        let file_name = Path::new(&f.path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        matches!(
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
        ) || f.is_entry_like()
    });

    // 3. 纯基础设施检测：没有应用源码，只有基础设施文件
    let has_app_source = source_files.iter().any(|f| {
        f.is_substantive_source()
            && !matches!(
                f.language.as_str(),
                "yaml" | "toml" | "json" | "text" | "config" | "markdown"
            )
    });
    let has_infra_files = source_files.iter().any(|f| {
        let name = Path::new(&f.path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        name == "Dockerfile"
            || name == "docker-compose.yml"
            || name == "docker-compose.yaml"
            || name == "nginx.conf"
            || name.ends_with(".sh")
    });

    if !has_app_source && has_infra_files {
        return "infrastructure".to_string();
    }

    // 4. 目录名/manifest 关键词覆盖标签判断
    let is_agent_or_adapter = root_path.split('/').any(|seg| {
        matches!(
            seg.to_ascii_lowercase().as_str(),
            "agent"
                | "agents"
                | "adapter"
                | "adapters"
                | "plugin"
                | "plugins"
                | "connector"
                | "connectors"
                | "bridge"
        )
    });

    // 如果目录名暗示 agent/adapter/plugin，不要被 frontend/backend 标签误导
    if is_agent_or_adapter {
        if has_main_entry {
            return "cli-tool".to_string();
        }
        return "module".to_string();
    }

    // 5. 标签兜底
    if tags.iter().any(|tag| tag == "infrastructure") {
        return "infrastructure".to_string();
    }

    if tags.iter().any(|tag| tag == "frontend") && !tags.iter().any(|tag| tag == "backend") {
        return "frontend-app".to_string();
    }

    if tags.iter().any(|tag| tag == "backend") && !tags.iter().any(|tag| tag == "frontend") {
        return "backend-service".to_string();
    }

    // 前后端标签同时存在时，不做武断分类
    if tags.iter().any(|tag| tag == "frontend") && tags.iter().any(|tag| tag == "backend") {
        return "module".to_string();
    }

    if report
        .workspace_roots
        .iter()
        .any(|workspace_root| workspace_root == root_path)
    {
        return "workspace-member".to_string();
    }

    "module".to_string()
}

/// manifest 信号聚合，用于 module kind 多维判断。
/// 每个字段对应一种 manifest 声明的存在性，供 `kind_from_manifest()` 消费。
struct ManifestSignal {
    has_cargo_lib: bool,
    has_cargo_bin: bool,
    has_package_json_bin: bool,
    has_package_json_main: bool,
    has_go_mod: bool,
}

/// 从 manifest 文件内容中提取轻量级信号。
/// 只做字符串匹配，不做完整 TOML/JSON 解析。
fn detect_manifest_signal(report: &ScanReport, root_path: &str) -> ManifestSignal {
    let repo_root = Path::new(&report.root);
    let module_dir = if root_path == "." {
        repo_root.to_path_buf()
    } else {
        repo_root.join(root_path)
    };

    let mut signal = ManifestSignal {
        has_cargo_lib: false,
        has_cargo_bin: false,
        has_package_json_bin: false,
        has_package_json_main: false,
        has_go_mod: false,
    };

    // Cargo.toml 检查
    if let Ok(content) = std::fs::read_to_string(module_dir.join("Cargo.toml")) {
        signal.has_cargo_lib = has_cargo_lib_section(&content);
        signal.has_cargo_bin = has_cargo_bin_section(&content);
    }

    // package.json 检查
    if let Ok(content) = std::fs::read_to_string(module_dir.join("package.json")) {
        signal.has_package_json_bin = has_package_json_bin(&content);
        signal.has_package_json_main = has_package_json_main(&content);
    }

    // go.mod 检查
    signal.has_go_mod = module_dir.join("go.mod").is_file();

    signal
}

/// 基于 manifest 信号推断 kind。
fn kind_from_manifest(signal: &ManifestSignal, root_path: &str) -> Option<String> {
    // Cargo.toml: [lib] 且无 [[bin]] → library；有 [[bin]] → cli-tool
    if signal.has_cargo_lib && !signal.has_cargo_bin {
        return Some("library".to_string());
    }
    if signal.has_cargo_bin {
        return Some("cli-tool".to_string());
    }

    // package.json: bin 字段 → cli-tool
    if signal.has_package_json_bin {
        return Some("cli-tool".to_string());
    }

    // go.mod: 检查是否有 cmd/ 目录暗示 CLI
    if signal.has_go_mod {
        let dir_name = root_path.split('/').next_back().unwrap_or_default();
        if dir_name == "cmd" || root_path.contains("/cmd/") {
            return Some("cli-tool".to_string());
        }
        // go.mod 本身不足以判断 library vs application，交给后续信号
    }

    None
}

/// 检查 Cargo.toml 是否包含 `[lib]` section。
fn has_cargo_lib_section(content: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "[lib]" || trimmed.starts_with("[lib]")
    })
}

/// 检查 Cargo.toml 是否包含 `[[bin]]` section。
fn has_cargo_bin_section(content: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "[[bin]]" || trimmed.starts_with("[[bin]]")
    })
}

/// 检查 package.json 是否包含 `"bin"` 字段。
fn has_package_json_bin(content: &str) -> bool {
    content.contains("\"bin\"")
}

/// 检查 package.json 是否包含 `"main"` 字段。
fn has_package_json_main(content: &str) -> bool {
    content.contains("\"main\"")
}

/// 模块根路径若自己没有 manifest，但能稳定承载子模块，则视为组节点。
fn has_structural_manifest(report: &ScanReport, root_path: &str) -> bool {
    report
        .config_files
        .iter()
        .any(|config_path| structural_manifest_root(config_path).as_deref() == Some(root_path))
}

fn structural_manifest_root(config_path: &str) -> Option<String> {
    let file_name = Path::new(config_path)
        .file_name()
        .and_then(|name| name.to_str())?;

    if !matches!(
        file_name,
        "package.json"
            | "Cargo.toml"
            | "pyproject.toml"
            | "requirements.txt"
            | "Pipfile"
            | "nginx.conf"
    ) {
        return None;
    }

    let parent = Path::new(config_path)
        .parent()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| ".".to_string());

    Some(if parent.is_empty() {
        ".".to_string()
    } else {
        parent
    })
}

/// 判断一个路径是否属于某个模块根路径。
/// 根模块 `"."` 只兜底接住"没有被任何子模块显式吃掉的路径"。
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
fn build_cross_module_edges(report: &ScanReport, modules: &[ModuleNode]) -> Vec<RelationEdge> {
    let mut edges: BTreeMap<String, RelationEdge> = BTreeMap::new();

    for dependency in &report.dependency_hints {
        if let Some(edge) = map_dependency_to_module_edge(dependency, modules) {
            match edges.get_mut(&edge.id) {
                Some(existing) => {
                    for evidence in edge.evidence {
                        if !existing.evidence.contains(&evidence) {
                            existing.evidence.push(evidence);
                        }
                    }
                }
                None => {
                    edges.insert(edge.id.clone(), edge);
                }
            }
        }
    }

    edges.into_values().collect()
}

/// 把单条依赖线索映射成模块边。
fn map_dependency_to_module_edge(
    dependency: &DependencyHint,
    modules: &[ModuleNode],
) -> Option<RelationEdge> {
    let source_module = find_best_module_for_path(&dependency.from, modules)?;
    let target_module = find_best_module_for_path(&dependency.to, modules)?;

    if source_module.id == target_module.id {
        return None;
    }

    if source_module.parent_id.is_none() || target_module.parent_id.is_none() {
        return None;
    }

    let edge_seed = format!(
        "{}:{}:{}",
        source_module.id, target_module.id, dependency.kind
    );

    Some(RelationEdge {
        id: stable_id("relation", edge_seed),
        source: source_module.id.clone(),
        target: target_module.id.clone(),
        relation_type: dependency.kind.clone(),
        evidence: vec![dependency.from.clone(), dependency.to.clone()],
    })
}

/// 在所有可匹配模块中，优先选根路径最长的那个。
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
fn build_architecture_hints(
    report: &ScanReport,
    modules: &[ModuleNode],
    edges: &[RelationEdge],
) -> Vec<String> {
    let root_id = modules.first().map(|module| module.id.as_str());
    let top_level_modules = modules
        .iter()
        .filter(|module| module.parent_id.as_deref() == root_id)
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

    let mut hints = Vec::new();
    hints.push(format!(
        "技术栈：{}",
        join_or_default(&report.tech_hints, "未识别")
    ));
    hints.push(format!(
        "模块数量：{}",
        modules
            .iter()
            .filter(|module| module.parent_id.is_some())
            .count()
    ));
    hints.push(format!("跨模块关系：{}", edges.len()));
    hints.push(format!(
        "顶层模块：{}",
        join_or_default(&top_level_modules, "未识别")
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
fn join_or_default(values: &[String], fallback: &str) -> String {
    if values.is_empty() {
        fallback.to_string()
    } else {
        values.join("、")
    }
}
