use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::repo::language_processors::LanguageProcessorManager;
use crate::repo::scanner::{DependencyHint, ScannedFile};

/// `ManifestAnalysis` 汇总了结构化解析阶段得到的附加信号。
/// 扫描层会把这些结果和文件系统事实合并，再交给模块树构建阶段使用。
#[derive(Debug, Default)]
pub struct ManifestAnalysis {
    /// 通过 manifest 显式声明出来的工作区/成员目录。
    pub workspace_roots: Vec<String>,
    /// 通过 manifest 明确声明或推断出来的入口文件。
    pub entry_points: Vec<String>,
    /// 通过 manifest 直接得到的模块依赖线索。
    pub dependency_hints: Vec<DependencyHint>,
    /// 供源码 import/use/import-from 解析使用的“内部别名 -> 模块根路径”映射。
    pub import_aliases: BTreeMap<String, String>,
    /// manifest 本身能稳定给出的技术栈提示。
    pub tech_hints: Vec<String>,
}

/// 中间依赖种子只在 manifest 解析阶段存在。
/// 等所有别名收集完后，再统一折叠成正式的 `DependencyHint`。
#[derive(Debug)]
struct ManifestDependencySeed {
    from_path: String,
    target_name: String,
    kind: String,
}

#[derive(Debug, Default)]
struct ManifestCollector {
    workspace_roots: BTreeSet<String>,
    entry_points: BTreeSet<String>,
    dependency_hints: Vec<DependencyHint>,
    import_aliases: BTreeMap<String, String>,
    tech_hints: BTreeSet<String>,
    dependency_seeds: Vec<ManifestDependencySeed>,
}

/// 解析各类 manifest，并把“显式结构信号”提升为扫描报告的一部分。
/// 这里优先用结构化解析处理 `package.json`、`Cargo.toml`、`pyproject.toml`，
/// 目的是先把模块树构建所需的硬事实做稳。
///
/// # 参数
/// - `repo_root`：仓库根目录。
/// - `files`：扫描阶段得到的文件列表。
///
/// # 返回
/// - 返回 manifest 补充得到的工作区、入口、依赖和技术栈提示。
pub fn analyze_manifests(repo_root: &Path, files: &[ScannedFile]) -> ManifestAnalysis {
    let mut collector = ManifestCollector::default();

    for file in files {
        let file_name = Path::new(&file.path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        match file_name {
            "package.json" => analyze_package_json(repo_root, file, &mut collector),
            "Cargo.toml" => analyze_cargo_toml(repo_root, file, &mut collector),
            "pyproject.toml" => analyze_pyproject_toml(repo_root, file, &mut collector),
            "requirements.txt" | "Pipfile" => analyze_python_manifest(file, &mut collector),
            "nginx.conf" => analyze_nginx_manifest(file, &mut collector),
            _ => {}
        }
    }

    finalize_manifest_dependencies(&mut collector);

    ManifestAnalysis {
        workspace_roots: collector.workspace_roots.into_iter().collect(),
        entry_points: collector.entry_points.into_iter().collect(),
        dependency_hints: collector.dependency_hints,
        import_aliases: collector.import_aliases,
        tech_hints: collector.tech_hints.into_iter().collect(),
    }
}

/// 从源码文本里提取“可能的内部依赖目标”。
/// 这里优先让 Rust 走 `syn`，其余语言保留轻量解析，但统一收敛到相同输出模型。
///
/// # 参数
/// - `language`：当前文件的语言标签。
/// - `content`：源码文本。
///
/// # 返回
/// - 返回该文件中出现的原始依赖目标列表。
pub fn extract_source_dependency_targets(language: &str, content: &str) -> Vec<String> {
    LanguageProcessorManager::new().extract_dependency_targets(language, content)
}

/// 从源码文本中提取当前文件声明出的内部别名。
/// 这一步主要服务 Java / C# / Kotlin / PHP 这类“依赖目标依赖 package/namespace”的语言。
///
/// # 参数
/// - `language`：当前文件的语言标签。
/// - `content`：源码文本。
///
/// # 返回
/// - 返回该文件声明出的别名列表；当前语言不支持时返回空数组。
pub fn extract_source_aliases(language: &str, content: &str) -> Vec<String> {
    LanguageProcessorManager::new().extract_declared_aliases(language, content)
}

/// 把原始依赖目标规范化为仓库内部可比较的路径或模块根路径。
/// 如果目标明显是第三方依赖，则返回 `None`，避免污染模块树。
///
/// # 参数
/// - `source_path`：当前源码文件的仓库内相对路径。
/// - `target`：原始依赖目标。
/// - `language`：当前文件语言。
/// - `import_aliases`：通过 manifest 收集到的内部别名映射。
///
/// # 返回
/// - 如果目标能稳定映射到仓库内部路径，则返回规范化结果。
pub fn normalize_dependency_target(
    source_path: &str,
    target: &str,
    language: &str,
    import_aliases: &BTreeMap<String, String>,
) -> Option<String> {
    let normalized_target = target
        .trim()
        .trim_matches(['"', '\'', ';', ',', ')', '('])
        .trim();

    if normalized_target.is_empty() {
        return None;
    }

    if normalized_target.starts_with("./") || normalized_target.starts_with("../") {
        return Some(resolve_source_relative_path(source_path, normalized_target));
    }

    if let Some(mapped_root) = resolve_alias_target(normalized_target, import_aliases) {
        return Some(mapped_root);
    }

    match language {
        "rust" => normalize_rust_target(source_path, normalized_target),
        "python" => normalize_python_target(source_path, normalized_target),
        "java" | "kotlin" | "csharp" | "php" => normalize_namespace_target(normalized_target),
        "swift" => Some(normalized_target.to_string()),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    /// npm 包名，也会作为内部别名候选参与模块映射。
    name: Option<String>,
    /// pnpm/npm workspaces 定义，用于显式模块边界发现。
    workspaces: Option<PackageWorkspaces>,
    /// CommonJS 入口声明。
    main: Option<String>,
    /// ESM 入口声明。
    module: Option<String>,
    /// 类型入口声明。
    types: Option<String>,
    /// CLI 可执行入口声明。
    bin: Option<PackageBin>,
    /// 运行时依赖。
    dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "devDependencies")]
    /// 开发依赖；这里仍会作为“仓库内部包引用”线索参与判断。
    dev_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    /// 对等依赖。
    peer_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    /// 可选依赖。
    optional_dependencies: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PackageWorkspaces {
    List(Vec<String>),
    Object {
        packages: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PackageBin {
    Single(String),
    Multiple(BTreeMap<String, String>),
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    /// crate 的 package 元信息。
    package: Option<CargoPackage>,
    /// Cargo workspace 定义。
    workspace: Option<CargoWorkspace>,
    /// 库目标配置。
    lib: Option<CargoTarget>,
    /// 二进制目标配置。
    bin: Option<Vec<CargoTarget>>,
    /// 依赖表；这里只取键名用于模块关系推断。
    dependencies: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CargoWorkspace {
    members: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CargoTarget {
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PyProjectToml {
    project: Option<PyProject>,
}

#[derive(Debug, Deserialize)]
struct PyProject {
    /// Python 项目名，也会参与内部别名映射。
    name: Option<String>,
    /// PEP 621 依赖声明。
    dependencies: Option<Vec<String>>,
    /// 脚本入口声明。
    scripts: Option<BTreeMap<String, String>>,
}

/// `package.json` 提供的是前端/Node 子系统最明确的结构信号。
/// 这里会读取工作区、入口、包名和显式依赖。
fn analyze_package_json(repo_root: &Path, file: &ScannedFile, collector: &mut ManifestCollector) {
    let absolute_path = repo_root.join(&file.path);
    let Ok(content) = fs::read_to_string(&absolute_path) else {
        return;
    };
    let Ok(manifest) = serde_json::from_str::<PackageJson>(&content) else {
        return;
    };

    let manifest_root = manifest_root(&file.path);
    collector.workspace_roots.insert(manifest_root.clone());
    collector.tech_hints.insert("frontend".to_string());

    if let Some(name) = manifest.name {
        record_alias(&mut collector.import_aliases, name, &manifest_root);
    }

    if let Some(workspaces) = manifest.workspaces {
        let patterns = match workspaces {
            PackageWorkspaces::List(patterns) => patterns,
            PackageWorkspaces::Object { packages } => packages,
        };

        for pattern in patterns {
            for workspace_root in expand_workspace_pattern(repo_root, &manifest_root, &pattern) {
                collector.workspace_roots.insert(workspace_root);
            }
        }
    }

    for candidate in [manifest.main, manifest.module, manifest.types]
        .into_iter()
        .flatten()
    {
        if let Some(entry_path) = resolve_manifest_declared_path(repo_root, &manifest_root, &candidate) {
            collector.entry_points.insert(entry_path);
        }
    }

    if let Some(bin) = manifest.bin {
        match bin {
            PackageBin::Single(path) => {
                if let Some(entry_path) =
                    resolve_manifest_declared_path(repo_root, &manifest_root, &path)
                {
                    collector.entry_points.insert(entry_path);
                }
            }
            PackageBin::Multiple(entries) => {
                for path in entries.into_values() {
                    if let Some(entry_path) =
                        resolve_manifest_declared_path(repo_root, &manifest_root, &path)
                    {
                        collector.entry_points.insert(entry_path);
                    }
                }
            }
        }
    }

    for dependency_name in collect_manifest_dependency_names([
        manifest.dependencies,
        manifest.dev_dependencies,
        manifest.peer_dependencies,
        manifest.optional_dependencies,
    ]) {
        collector.dependency_seeds.push(ManifestDependencySeed {
            from_path: file.path.clone(),
            target_name: dependency_name,
            kind: "DEPENDS_ON".to_string(),
        });
    }
}

/// `Cargo.toml` 负责提供 Rust 工作区、crate 名称和显式依赖。
/// 这里优先抽取那些直接影响模块树和跨模块关系的字段。
fn analyze_cargo_toml(repo_root: &Path, file: &ScannedFile, collector: &mut ManifestCollector) {
    let absolute_path = repo_root.join(&file.path);
    let Ok(content) = fs::read_to_string(&absolute_path) else {
        return;
    };
    let Ok(manifest) = toml::from_str::<CargoToml>(&content) else {
        return;
    };

    let manifest_root = manifest_root(&file.path);
    collector.workspace_roots.insert(manifest_root.clone());
    collector.tech_hints.insert("backend".to_string());

    if let Some(package) = manifest.package {
        if let Some(name) = package.name {
            record_alias(&mut collector.import_aliases, name, &manifest_root);
        }
    }

    if let Some(workspace) = manifest.workspace {
        for pattern in workspace.members.unwrap_or_default() {
            for workspace_root in expand_workspace_pattern(repo_root, &manifest_root, &pattern) {
                collector.workspace_roots.insert(workspace_root);
            }
        }
    }

    if let Some(lib) = manifest.lib {
        if let Some(path) = lib.path {
            if let Some(entry_path) = resolve_manifest_declared_path(repo_root, &manifest_root, &path) {
                collector.entry_points.insert(entry_path);
            }
        }
    }

    for target in manifest.bin.unwrap_or_default() {
        if let Some(path) = target.path {
            if let Some(entry_path) = resolve_manifest_declared_path(repo_root, &manifest_root, &path) {
                collector.entry_points.insert(entry_path);
            }
        }
    }

    for fallback in ["src/main.rs", "src/lib.rs"] {
        if let Some(entry_path) = resolve_manifest_declared_path(repo_root, &manifest_root, fallback) {
            collector.entry_points.insert(entry_path);
        }
    }

    for dependency_name in manifest
        .dependencies
        .unwrap_or_default()
        .into_keys()
        .collect::<Vec<_>>()
    {
        collector.dependency_seeds.push(ManifestDependencySeed {
            from_path: file.path.clone(),
            target_name: dependency_name,
            kind: "DEPENDS_ON".to_string(),
        });
    }
}

/// `pyproject.toml` 在 Python 项目里提供项目名和脚本入口。
/// 当前先提取模块树和页面规划最需要的那一部分信息。
fn analyze_pyproject_toml(repo_root: &Path, file: &ScannedFile, collector: &mut ManifestCollector) {
    let absolute_path = repo_root.join(&file.path);
    let Ok(content) = fs::read_to_string(&absolute_path) else {
        return;
    };
    let Ok(manifest) = toml::from_str::<PyProjectToml>(&content) else {
        return;
    };

    let manifest_root = manifest_root(&file.path);
    collector.workspace_roots.insert(manifest_root.clone());
    collector.tech_hints.insert("backend".to_string());

    if let Some(project) = manifest.project {
        if let Some(name) = project.name {
            record_alias(&mut collector.import_aliases, name, &manifest_root);
        }

        for dependency_name in project
            .dependencies
            .unwrap_or_default()
            .into_iter()
            .filter_map(|raw| python_requirement_name(&raw))
        {
            collector.dependency_seeds.push(ManifestDependencySeed {
                from_path: file.path.clone(),
                target_name: dependency_name,
                kind: "DEPENDS_ON".to_string(),
            });
        }

        for script_target in project.scripts.unwrap_or_default().into_values() {
            if let Some(entry_path) = resolve_python_script_path(repo_root, &manifest_root, &script_target) {
                collector.entry_points.insert(entry_path);
            }
        }
    }

    for fallback in ["app.py", "__main__.py"] {
        if let Some(entry_path) = resolve_manifest_declared_path(repo_root, &manifest_root, fallback) {
            collector.entry_points.insert(entry_path);
        }
    }
}

/// `requirements.txt` 和 `Pipfile` 不能提供完整结构，但可以稳定标记 Python 模块根。
fn analyze_python_manifest(file: &ScannedFile, collector: &mut ManifestCollector) {
    let manifest_root = manifest_root(&file.path);
    collector.workspace_roots.insert(manifest_root);
    collector.tech_hints.insert("backend".to_string());
}

/// `nginx.conf` 代表基础设施入口，应该在模块树里被看见。
fn analyze_nginx_manifest(file: &ScannedFile, collector: &mut ManifestCollector) {
    let manifest_root = manifest_root(&file.path);
    collector.workspace_roots.insert(manifest_root);
    collector.tech_hints.insert("infrastructure".to_string());
    collector.entry_points.insert(file.path.clone());
}

/// manifest 里的依赖名只有在能映射到仓库内部别名时，才会成为正式依赖线索。
fn finalize_manifest_dependencies(collector: &mut ManifestCollector) {
    for seed in &collector.dependency_seeds {
        if let Some(target_root) = resolve_alias_target(&seed.target_name, &collector.import_aliases) {
            collector.dependency_hints.push(DependencyHint {
                from: seed.from_path.clone(),
                to: target_root,
                kind: seed.kind.clone(),
                confidence: "manifest".to_string(),
            });
        }
    }
}

/// 把依赖目标按“内部别名 -> 模块根路径”映射回仓库内路径。
/// 这里会先做分隔符归一化，确保 Java/C#/Kotlin/PHP 这类 namespace 风格目标能走同一套匹配逻辑。
fn resolve_alias_target(target: &str, import_aliases: &BTreeMap<String, String>) -> Option<String> {
    let canonical_target = canonical_dependency_target(target);

    import_aliases
        .iter()
        .filter_map(|(alias, root_path)| {
            let canonical_alias = canonical_dependency_target(alias);
            alias_matches_target(&canonical_alias, &canonical_target)
                .then_some((canonical_alias, root_path))
        })
        .max_by_key(|(canonical_alias, _)| canonical_alias.len())
        .map(|(canonical_alias, root_path)| {
            let suffix = canonical_target
                .strip_prefix(canonical_alias.as_str())
                .unwrap_or_default();
            let suffix = suffix.trim_start_matches('/');

            if suffix.is_empty() {
                root_path.clone()
            } else {
                format!("{root_path}/{suffix}")
            }
        })
}

/// Rust 依赖目标需要保留 `crate/self/super` 这种模块级语义。
/// 这里把它们折叠成相对于当前源码文件可比较的路径。
fn normalize_rust_target(source_path: &str, target: &str) -> Option<String> {
    if matches!(
        target.split('/').next(),
        Some("std" | "core" | "alloc" | "proc_macro")
    ) {
        return None;
    }

    if let Some(relative_target) = target.strip_prefix("crate/") {
        return Some(resolve_source_relative_path(source_path, &format!("../{relative_target}")));
    }

    if let Some(relative_target) = target.strip_prefix("self/") {
        return Some(resolve_source_relative_path(source_path, &format!("./{relative_target}")));
    }

    if let Some(relative_target) = target.strip_prefix("super/") {
        return Some(resolve_source_relative_path(source_path, &format!("../{relative_target}")));
    }

    None
}

/// Python 绝对导入默认落到当前顶层模块根，相对导入则保留相对路径语义。
fn normalize_python_target(source_path: &str, target: &str) -> Option<String> {
    if target.starts_with("./") || target.starts_with("../") {
        return Some(resolve_source_relative_path(source_path, target));
    }

    let source_root = source_root_for_alias(source_path);
    Some(format!("{source_root}/{target}"))
}

/// 把 namespace / package 风格目标统一改成 `/` 分隔，便于后续和模块根路径比较。
fn normalize_namespace_target(target: &str) -> Option<String> {
    let normalized = target.replace(['.', '\\', ':'], "/");
    (!normalized.is_empty()).then_some(normalized)
}

/// 按当前源码文件位置解析相对导入目标。
fn resolve_source_relative_path(source_path: &str, target: &str) -> String {
    let source_dir = Path::new(source_path).parent().unwrap_or_else(|| Path::new(""));
    let joined = source_dir.join(target);
    normalize_path_like(&joined.to_string_lossy())
}

/// 从源码路径中推导“最接近真实模块边界”的根路径。
/// 命中 `src/app/lib/modules` 时优先截到这些目录之前，否则再退回到工作区成员或顶层目录。
fn source_root_for_alias(source_path: &str) -> String {
    let segments = source_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if let Some(source_root_end) = segments
        .iter()
        .position(|segment| matches!(*segment, "src" | "app" | "lib" | "modules"))
    {
        let root_segments = &segments[..source_root_end];
        if !root_segments.is_empty() {
            return root_segments.join("/");
        }
    }

    let mut segments = segments.into_iter();
    let first = segments.next().unwrap_or(".");

    if matches!(first, "crates" | "agents" | "apps" | "services" | "libs" | "packages") {
        let second = segments.next().unwrap_or(".");
        if second != "." {
            return format!("{first}/{second}");
        }
    }

    first.to_string()
}

/// 从 manifest 路径推导其所属模块根目录。
fn manifest_root(manifest_path: &str) -> String {
    Path::new(manifest_path)
        .parent()
        .map(|path| normalize_path_like(&path.to_string_lossy()))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| ".".to_string())
}

/// 记录“内部别名 -> 模块根路径”的映射。
/// 一旦某个别名先被更明确的根路径占用，后续重复声明就不覆盖，避免扫描顺序影响结果。
fn record_alias(import_aliases: &mut BTreeMap<String, String>, alias: String, root_path: &str) {
    if alias.is_empty() {
        return;
    }

    import_aliases.entry(alias).or_insert_with(|| root_path.to_string());
}

/// 判断归一化后的别名是否可以匹配当前依赖目标。
fn alias_matches_target(alias: &str, target: &str) -> bool {
    target == alias || target.starts_with(&format!("{alias}/"))
}

/// 把各种 namespace / import 目标统一折叠成 `/` 分隔格式。
fn canonical_dependency_target(target: &str) -> String {
    target.replace(['.', '\\', ':'], "/")
}

/// 根据 manifest 里声明的相对路径，解析出真实存在的仓库内文件路径。
fn resolve_manifest_declared_path(
    repo_root: &Path,
    manifest_root: &str,
    declared_path: &str,
) -> Option<String> {
    let declared_path = declared_path.trim();
    if declared_path.is_empty() {
        return None;
    }

    let manifest_root_path = if manifest_root == "." {
        PathBuf::new()
    } else {
        PathBuf::from(manifest_root)
    };
    let candidate = manifest_root_path.join(declared_path);
    let normalized = normalize_path_like(&candidate.to_string_lossy());

    repo_root.join(&normalized).exists().then_some(normalized)
}

/// 把 `pyproject.toml` 的 `module:function` 脚本入口映射回可能的源码文件。
fn resolve_python_script_path(
    repo_root: &Path,
    manifest_root: &str,
    script_target: &str,
) -> Option<String> {
    let module_target = script_target.split(':').next()?.trim();
    let module_path = module_target.replace('.', "/");

    for candidate in [format!("{module_path}.py"), format!("{module_path}/__init__.py")] {
        if let Some(entry_path) = resolve_manifest_declared_path(repo_root, manifest_root, &candidate) {
            return Some(entry_path);
        }
    }

    None
}

/// 展开 workspace 模式，例如 `packages/*`。
/// 当前只支持简单星号模式，目标是服务模块树边界发现，不做完整 glob 引擎。
fn expand_workspace_pattern(repo_root: &Path, manifest_root: &str, pattern: &str) -> Vec<String> {
    let manifest_root_path = if manifest_root == "." {
        PathBuf::new()
    } else {
        PathBuf::from(manifest_root)
    };
    let pattern = pattern.trim();

    if pattern.is_empty() {
        return Vec::new();
    }

    if !pattern.contains('*') {
        let candidate = manifest_root_path.join(pattern);
        let normalized = normalize_path_like(&candidate.to_string_lossy());
        return repo_root.join(&normalized).is_dir().then_some(normalized).into_iter().collect();
    }

    let prefix = pattern.split('*').next().unwrap_or_default().trim_end_matches('/');
    let base_dir = manifest_root_path.join(prefix);
    let absolute_base_dir = repo_root.join(&base_dir);
    let Ok(entries) = fs::read_dir(&absolute_base_dir) else {
        return Vec::new();
    };

    entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let file_type = entry.file_type().ok()?;
            file_type.is_dir().then(|| {
                normalize_path_like(&base_dir.join(entry.file_name()).to_string_lossy())
            })
        })
        .collect()
}

/// 把多组 manifest 依赖表统一摊平成依赖名列表。
fn collect_manifest_dependency_names<const N: usize>(
    dependency_groups: [Option<BTreeMap<String, String>>; N],
) -> Vec<String> {
    dependency_groups
        .into_iter()
        .flatten()
        .flat_map(|group| group.into_keys())
        .collect()
}

/// 从 Python requirement 声明里提取包名部分，忽略版本和环境标记。
fn python_requirement_name(raw_dependency: &str) -> Option<String> {
    let trimmed = raw_dependency.trim();
    if trimmed.is_empty() {
        return None;
    }

    let end_index = trimmed
        .find([' ', '[', '(', '<', '>', '=', '!', '~', ';'])
        .unwrap_or(trimmed.len());
    let name = trimmed[..end_index].trim();

    (!name.is_empty()).then(|| name.to_string())
}

/// 归一化路径文本，统一分隔符并消解 `.`、`..`。
fn normalize_path_like(path: &str) -> String {
    let mut segments = Vec::new();
    let normalized = path.replace('\\', "/");

    for segment in normalized.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            value => segments.push(value),
        }
    }

    if segments.is_empty() {
        ".".to_string()
    } else {
        segments.join("/")
    }
}
