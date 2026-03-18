use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::stable_id::stable_id;
use crate::llm::{FilePurposeAssistInput, LlmRuntime};
use crate::repo::detectors::detect_tech_hints;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::parsers::{
    analyze_manifests, extract_source_aliases as parse_source_aliases,
    extract_source_dependency_targets, normalize_dependency_target,
};

/// `FilePurpose` 是扫描阶段给每个文件打上的稳定角色标签。
/// 它优先由 deterministic 的路径/文件名规则给出，供 hierarchy / planner / context 复用。
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FilePurpose {
    Entry,
    Router,
    Controller,
    Handler,
    Service,
    Model,
    Repository,
    Domain,
    Agent,
    Library,
    Middleware,
    Plugin,
    #[default]
    Utility,
    Helper,
    Constant,
    Type,
    Page,
    Component,
    Widget,
    Layout,
    Config,
    Migration,
    Test,
    Docs,
}

impl FilePurpose {
    /// 维持旧运行时分层需要的粗粒度 family。
    pub fn family(&self) -> &'static str {
        match self {
            FilePurpose::Config => "config",
            FilePurpose::Docs => "docs",
            _ => "source",
        }
    }

    /// 高信号角色会显著影响关键源码和模块评分。
    pub fn signal_weight(&self) -> i32 {
        match self {
            FilePurpose::Entry => 120,
            FilePurpose::Router | FilePurpose::Controller | FilePurpose::Handler => 95,
            FilePurpose::Service | FilePurpose::Agent => 80,
            FilePurpose::Repository | FilePurpose::Domain | FilePurpose::Model => 65,
            FilePurpose::Page
            | FilePurpose::Layout
            | FilePurpose::Component
            | FilePurpose::Widget => 55,
            FilePurpose::Library | FilePurpose::Middleware | FilePurpose::Plugin => 45,
            FilePurpose::Migration => 20,
            FilePurpose::Utility => 10,
            FilePurpose::Helper | FilePurpose::Constant | FilePurpose::Type => -10,
            FilePurpose::Config => -20,
            FilePurpose::Test | FilePurpose::Docs => -50,
        }
    }

    /// 低信号角色会被 hierarchy / planner / context 统一降权。
    pub fn is_low_signal(&self) -> bool {
        matches!(
            self,
            FilePurpose::Test
                | FilePurpose::Docs
                | FilePurpose::Config
                | FilePurpose::Helper
                | FilePurpose::Constant
                | FilePurpose::Type
        )
    }

    /// 这些角色变化更容易触发模块边界或页面规划变化。
    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            FilePurpose::Entry
                | FilePurpose::Router
                | FilePurpose::Layout
                | FilePurpose::Config
                | FilePurpose::Migration
        )
    }
}

/// `ScannedFile` 是扫描阶段最原子的事实记录。
/// 后续模块拆分、页面规划和 metadata 映射都会围绕这些字段工作，
/// 所以这里尽量只保存“可重复计算的事实”，不混入解释性判断。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScannedFile {
    /// 稳定 ID，后续 source -> module/page 映射依赖它来建立关系。
    pub id: String,
    /// 相对仓库根目录的路径，统一使用 `/` 作为分隔符。
    pub path: String,
    /// 语言是后续技术栈判断和依赖线索提取的基础。
    pub language: String,
    /// kind 用于区分源码、配置、文档、资源等不同角色。
    pub kind: String,
    /// `purpose` 表达比 `kind` 更细的稳定文件角色。
    #[serde(default)]
    pub purpose: FilePurpose,
    /// 指纹用于 update/status 阶段判断源码是否变化。
    pub fingerprint: String,
    /// 文件大小目前主要用于调试和后续评分，不参与业务决策。
    pub size: usize,
    /// tags 放扫描阶段就能稳定得到的标签，例如 `entry-point`。
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct PendingFilePurposeCandidate {
    index: usize,
    input: FilePurposeAssistInput,
}

impl ScannedFile {
    /// 兼容旧粗分类的配置文件判断。
    pub fn is_config_like(&self) -> bool {
        self.kind == "config" || self.purpose == FilePurpose::Config
    }

    /// 兼容旧粗分类的文档文件判断。
    pub fn is_docs_like(&self) -> bool {
        self.kind == "docs" || self.purpose == FilePurpose::Docs
    }

    /// 资源文件仍然沿用旧 `kind` 判定。
    pub fn is_asset_like(&self) -> bool {
        self.kind == "asset"
    }

    /// 入口相关文件会同时驱动模块晋升和 key source 选择。
    pub fn is_entry_like(&self) -> bool {
        self.tags.iter().any(|tag| tag == "entry-point")
            || matches!(self.purpose, FilePurpose::Entry | FilePurpose::Router)
    }

    /// 测试文件不应主导模块判断。
    pub fn is_test_like(&self) -> bool {
        self.tags.iter().any(|tag| tag == "test-file") || self.purpose == FilePurpose::Test
    }

    /// 下游统一消费的低信号判断。
    pub fn is_low_signal(&self) -> bool {
        self.is_asset_like() || self.is_docs_like() || self.purpose.is_low_signal()
    }

    /// 对模块规划有意义的“真实源码”定义。
    pub fn is_substantive_source(&self) -> bool {
        self.kind == "source" && !self.is_test_like() && !self.is_low_signal()
    }
}

/// `DependencyHint` 不是完整语义依赖图，只是后续模块关系推断的启发式线索。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependencyHint {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub confidence: String,
}

/// `ScanReport` 是仓库扫描阶段的统一输出。
/// 它是 deterministic pipeline 的起点，后面的 decomposition / context / generation
/// 都应该基于它继续加工，而不是重新扫文件系统。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScanReport {
    pub root: String,
    pub files: Vec<ScannedFile>,
    pub tech_hints: Vec<String>,
    pub workspace_roots: Vec<String>,
    pub config_files: Vec<String>,
    pub entry_points: Vec<String>,
    pub dependency_hints: Vec<DependencyHint>,
}

/// 扫描本地代码目录并产出基础事实。
/// 这里刻意不引入 LLM，也不做复杂推理，只建立稳定、可复用的事实层。
///
/// # 参数
/// - `root`：要扫描的本地代码目录。
/// - `extra_ignore_paths`：额外忽略路径列表（来自 steering 配置），
///   支持 glob 风格的 `dir/**` 模式和精确目录名匹配。
///
/// # 返回
/// - 成功时返回完整的扫描报告，包含文件、技术栈线索、入口和依赖线索。
///
/// # 错误
/// - 当目录不可读、文件读取失败或遍历过程中出现 I/O 错误时返回错误。
pub fn scan_repo(root: &Path, extra_ignore_paths: &[String]) -> io::Result<ScanReport> {
    scan_repo_with_boundary(root, extra_ignore_paths, &[])
}

/// 带 include / ignore 边界的扫描入口。
/// `scan.include` 会对白名单路径恢复被忽略目录或文件。
pub fn scan_repo_with_boundary(
    root: &Path,
    extra_ignore_paths: &[String],
    extra_include_paths: &[String],
) -> io::Result<ScanReport> {
    scan_repo_with_boundary_and_llm(root, extra_ignore_paths, extra_include_paths, None)
}

/// 带可选 LLM Uncertainty Gate 的扫描入口。
pub fn scan_repo_with_boundary_and_llm(
    root: &Path,
    extra_ignore_paths: &[String],
    extra_include_paths: &[String],
    llm_runtime: Option<&mut LlmRuntime<'_, '_>>,
) -> io::Result<ScanReport> {
    let mut files = Vec::new();
    let mut pending_file_purposes = Vec::new();
    // 先做一次快速扫描，收集根目录下的 manifest 文件，
    // 用于判断仓库类型（如 Go 仓库）和 workspace 成员白名单。
    let root_manifests = discover_root_manifests(root);
    let is_go_repo = root_manifests.iter().any(|m| m == "go.mod");
    let mut llm_runtime = llm_runtime;
    visit_dir(
        root,
        root,
        &mut files,
        &mut pending_file_purposes,
        &root_manifests,
        is_go_repo,
        extra_ignore_paths,
        extra_include_paths,
    )?;
    apply_llm_file_purpose_overrides(
        &mut files,
        pending_file_purposes,
        llm_runtime.as_deref_mut(),
    )?;
    let manifest_analysis = analyze_manifests(root, &files);

    // 这些聚合字段会被 decomposition 和 workflow 直接消费，
    // 因此在扫描阶段就顺手整理出来，避免后续每一层都重复遍历。
    let paths = files
        .iter()
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    // `config_files / entry_points` 会进入 status/update 的结构漂移判断，
    // 这里必须坚持 deterministic 规则，不能被 LLM purpose 覆盖污染。
    let config_files = files
        .iter()
        .filter(|file| is_structural_config_file(&file.path, &file.kind))
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    let entry_points = files
        .iter()
        .filter(|file| is_structural_entry_point(&file.path, &file.kind))
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    let mut tech_hints = detect_tech_hints(&paths)
        .into_iter()
        .collect::<BTreeSet<_>>();
    tech_hints.extend(manifest_analysis.tech_hints.iter().cloned());

    let mut workspace_roots = discover_workspace_roots(&config_files)
        .into_iter()
        .collect::<BTreeSet<_>>();
    workspace_roots.extend(manifest_analysis.workspace_roots.iter().cloned());

    let mut resolved_entry_points = entry_points.into_iter().collect::<BTreeSet<_>>();
    resolved_entry_points.extend(manifest_analysis.entry_points.iter().cloned());

    let mut import_aliases = manifest_analysis.import_aliases;
    import_aliases.extend(collect_source_aliases(root, &files));

    let mut dependency_hints = manifest_analysis.dependency_hints;
    dependency_hints.extend(collect_dependency_hints(root, &files, &import_aliases));
    dependency_hints.extend(collect_service_api_hints(root, &files));
    dependency_hints.extend(collect_infrastructure_dependency_hints(root, &files));
    dependency_hints = dedupe_dependency_hints(dependency_hints);

    Ok(ScanReport {
        root: root.to_string_lossy().to_string(),
        tech_hints: tech_hints.into_iter().collect(),
        workspace_roots: workspace_roots.into_iter().collect(),
        config_files,
        entry_points: resolved_entry_points.into_iter().collect(),
        dependency_hints,
        files,
    })
}

/// 递归遍历目录，把每个可分析文件整理成 `ScannedFile`。
/// 这里的职责很单纯：过滤无关目录/文件，读取字节，补足基础分类信息。
///
/// # 参数
/// - `root`：仓库根目录，用于计算相对路径。
/// - `dir`：当前递归扫描到的目录。
/// - `files`：收集扫描结果的输出数组。
///
/// # 返回
/// - 成功时把当前目录及子目录里的可分析文件追加到 `files`。
///
/// # 错误
/// - 当目录遍历、文件类型判断或文件读取失败时返回错误。
fn visit_dir(
    root: &Path,
    dir: &Path,
    files: &mut Vec<ScannedFile>,
    pending_file_purposes: &mut Vec<PendingFilePurposeCandidate>,
    root_manifests: &[String],
    is_go_repo: bool,
    extra_ignore_paths: &[String],
    extra_include_paths: &[String],
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            let explicitly_included = should_include_path(root, &path, extra_include_paths);
            if should_ignore_dir(&path)
                && !explicitly_included
                && !should_descend_for_include(root, &path, extra_include_paths)
            {
                continue;
            }

            // vendor 目录在 Go 仓库中保留，其他仓库排除
            if path.file_name().and_then(|n| n.to_str()) == Some("vendor")
                && !is_go_repo
                && !explicitly_included
                && !should_descend_for_include(root, &path, extra_include_paths)
            {
                continue;
            }

            // 嵌套仓库检测：子目录含 .git 且不属于 workspace 成员时跳过
            if is_nested_repo(&path, root_manifests)
                && !explicitly_included
                && !should_descend_for_include(root, &path, extra_include_paths)
            {
                continue;
            }

            // steering 配置的额外忽略路径
            if should_ignore_by_extra_paths(root, &path, extra_ignore_paths)
                && !explicitly_included
                && !should_descend_for_include(root, &path, extra_include_paths)
            {
                continue;
            }

            visit_dir(
                root,
                &path,
                files,
                pending_file_purposes,
                root_manifests,
                is_go_repo,
                extra_ignore_paths,
                extra_include_paths,
            )?;
            continue;
        }

        // 运行产物、日志和二进制不参与 Repo Wiki 分析。
        // 它们会污染模块判断，还会让“关键源码”落到无关文件上。
        let explicitly_included = should_include_path(root, &path, extra_include_paths);
        if should_ignore_file(&path) && !explicitly_included {
            continue;
        }

        let relative = make_relative(root, &path);
        if should_ignore_relative_path(&relative, extra_ignore_paths) && !explicitly_included {
            continue;
        }
        let bytes = fs::read(&path)?;
        let kind = classify_file_kind(&relative);
        let language = detect_language(&relative);
        let purpose = classify_file_purpose(&relative, &kind);
        if purpose == FilePurpose::Utility {
            let preview = build_file_purpose_preview(&bytes);
            if !preview.is_empty() {
                pending_file_purposes.push(PendingFilePurposeCandidate {
                    index: files.len(),
                    input: FilePurposeAssistInput {
                        path: relative.clone(),
                        kind: kind.clone(),
                        language: language.clone(),
                        file_size: bytes.len(),
                        deterministic: "utility".to_string(),
                        preview,
                    },
                });
            }
        }
        let tags = detect_tags(&relative, purpose);

        files.push(ScannedFile {
            id: stable_id("source", &relative),
            path: relative,
            language,
            kind,
            purpose,
            fingerprint: fingerprint_bytes(&bytes),
            size: bytes.len(),
            tags,
        });
    }

    Ok(())
}

fn build_file_purpose_preview(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .lines()
        .take(24)
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn apply_llm_file_purpose_overrides(
    files: &mut [ScannedFile],
    pending_file_purposes: Vec<PendingFilePurposeCandidate>,
    llm_runtime: Option<&mut LlmRuntime<'_, '_>>,
) -> io::Result<()> {
    let Some(llm_runtime) = llm_runtime else {
        return Ok(());
    };
    if pending_file_purposes.is_empty() {
        return Ok(());
    }

    let inputs = pending_file_purposes
        .iter()
        .map(|candidate| candidate.input.clone())
        .collect::<Vec<_>>();
    let resolved = llm_runtime.classify_file_purposes(&inputs)?;

    for (candidate, purpose) in pending_file_purposes.into_iter().zip(resolved.into_iter()) {
        let Some(purpose) = purpose else {
            continue;
        };
        let Some(file) = files.get_mut(candidate.index) else {
            continue;
        };
        file.purpose = purpose;
        file.tags = detect_tags(&file.path, purpose);
    }

    Ok(())
}

/// 目录级过滤主要用来剔除依赖、缓存和运行时目录。
/// 这些目录一旦进入扫描，会显著干扰“顶层模块”判断。
///
/// # 参数
/// - `path`：当前要判断的目录路径。
///
/// # 返回
/// - 如果目录应被扫描阶段忽略，则返回 `true`。
fn should_ignore_dir(path: &Path) -> bool {
    let dir_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    // 依赖、缓存、运行时、编辑器目录
    if matches!(
        dir_name,
        ".git"
            | ".hg"
            | ".svn"
            | "node_modules"
            | "target"
            | ".wiki"
            | "dist"
            | "logs"
            | "log"
            | "temp"
            | "tmp"
            | "__pycache__"
            | ".idea"
            | ".vscode"
            | ".cursor"
            | ".qoder"
            | ".codex"
            | ".serena"
            | ".venv"
            | "venv"
            // per-language 默认忽略（安全地无条件排除）
            | ".next"
            | ".nuxt"
            | ".gradle"
    ) {
        return true;
    }

    // Python egg-info 目录（*.egg-info）
    if dir_name.ends_with(".egg-info") {
        return true;
    }

    // fixture / test-data / mock 目录整体排除，
    // 这些目录的内容不应参与模块发现和页面生成。
    if matches!(
        dir_name,
        "fixtures"
            | "__fixtures__"
            | "test-data"
            | "testdata"
            | "test_data"
            | "mock-data"
            | "mocks"
            | "__mocks__"
    ) {
        return true;
    }

    // 非代码产物目录排除
    if matches!(
        dir_name,
        "openspec" | ".github" | ".gitlab" | ".circleci" | ".husky" | "coverage" | ".nyc_output"
    ) {
        return true;
    }

    false
}

/// 检查目录是否匹配 steering 配置的额外忽略路径。
/// 支持精确目录名匹配和 `dir/**` glob 前缀匹配。
fn should_ignore_by_extra_paths(root: &Path, dir: &Path, extra_ignore_paths: &[String]) -> bool {
    if extra_ignore_paths.is_empty() {
        return false;
    }

    let relative = dir
        .strip_prefix(root)
        .ok()
        .and_then(|p| p.to_str())
        .map(|s| s.replace('\\', "/"))
        .unwrap_or_default();

    if relative.is_empty() {
        return false;
    }

    should_ignore_relative_path(&relative, extra_ignore_paths)
}

fn should_ignore_relative_path(relative: &str, patterns: &[String]) -> bool {
    patterns
        .iter()
        .any(|pattern| path_matches_boundary_pattern(relative, pattern))
}

fn should_include_path(root: &Path, path: &Path, include_paths: &[String]) -> bool {
    if include_paths.is_empty() {
        return false;
    }

    let relative = make_relative(root, path);
    if relative.is_empty() {
        return false;
    }

    include_paths
        .iter()
        .any(|pattern| path_matches_boundary_pattern(&relative, pattern))
}

fn should_descend_for_include(root: &Path, dir: &Path, include_paths: &[String]) -> bool {
    if include_paths.is_empty() {
        return false;
    }

    let relative = make_relative(root, dir);
    if relative.is_empty() {
        return false;
    }

    include_paths.iter().any(|pattern| {
        let normalized = normalize_boundary_pattern(pattern);
        normalized == relative || normalized.starts_with(&format!("{relative}/"))
    })
}

fn path_matches_boundary_pattern(relative: &str, pattern: &str) -> bool {
    let normalized = normalize_boundary_pattern(pattern);
    if normalized.is_empty() {
        return false;
    }

    if let Some(prefix) = normalized.strip_suffix("/**") {
        return relative == prefix || relative.starts_with(&format!("{prefix}/"));
    }

    relative == normalized || relative.starts_with(&format!("{normalized}/"))
}

fn normalize_boundary_pattern(pattern: &str) -> String {
    pattern
        .trim()
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_string()
}

/// 文件级过滤负责剔除日志、二进制和编译副产物。
/// 这些内容既不会成为页面重点，也容易污染“关键源码”列表。
///
/// # 参数
/// - `path`：当前要判断的文件路径。
///
/// # 返回
/// - 如果文件应被扫描阶段忽略，则返回 `true`。
fn should_ignore_file(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if file_name.contains(".log.") || file_name.ends_with(".log") {
        return true;
    }

    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("log" | "pid" | "pyc" | "pyo" | "exe" | "dll" | "so" | "dylib" | "class")
    )
}

/// 把绝对路径转换成统一的仓库内相对路径。
/// 后续所有存储结构都以这个相对路径为准，避免平台差异。
///
/// # 参数
/// - `root`：仓库根目录。
/// - `path`：要转换的绝对路径。
///
/// # 返回
/// - 返回统一使用 `/` 分隔的仓库内相对路径。
fn make_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// 这里的 kind 只服务运行时分层，不追求语言语义精确。
/// 重点是尽早把“配置/文档/资源/源码”分开，减少后续判断的歧义。
///
/// # 参数
/// - `path`：仓库内相对路径。
///
/// # 返回
/// - 返回该文件的粗粒度角色分类。
fn classify_file_kind(path: &str) -> String {
    if matches!(
        path,
        "package.json"
            | "tsconfig.json"
            | "Cargo.toml"
            | "pnpm-workspace.yaml"
            | "pyproject.toml"
            | "requirements.txt"
            | "Pipfile"
    ) || path.ends_with("/package.json")
        || path.ends_with("/tsconfig.json")
        || path.ends_with("/Cargo.toml")
        || path.ends_with("/pyproject.toml")
        || path.ends_with("/requirements.txt")
        || path.ends_with("/Pipfile")
        || path.ends_with("/config.yaml")
        || path.ends_with("/config.yml")
        || path.ends_with("/nginx.conf")
        || path.ends_with(".conf")
        || path.ends_with(".ini")
    {
        return "config".to_string();
    }

    if path.ends_with(".md") || path.ends_with(".mdx") {
        return "docs".to_string();
    }

    if matches!(
        Path::new(path).extension().and_then(|ext| ext.to_str()),
        Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "ico")
    ) {
        return "asset".to_string();
    }

    "source".to_string()
}

/// 文件角色优先由 deterministic 路径 / 文件名规则决定。
/// 这里借鉴 deepwiki-rs 的 rule-based 顺序，但只保留当前 core 真正消费的角色集合。
fn classify_file_purpose(path: &str, kind: &str) -> FilePurpose {
    let lower_path = path.to_ascii_lowercase();
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let stem = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let segments = lower_path.split('/').collect::<Vec<_>>();

    if kind == "docs" {
        return FilePurpose::Docs;
    }

    if kind == "config" {
        return FilePurpose::Config;
    }

    if is_test_path(path) {
        return FilePurpose::Test;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "migrations" | "migration"))
        || file_name.contains("migration")
        || file_name.starts_with("v") && file_name.contains("__")
    {
        return FilePurpose::Migration;
    }

    if matches!(
        file_name.as_str(),
        "main.rs"
            | "main.go"
            | "main.py"
            | "main.ts"
            | "main.js"
            | "main.kt"
            | "main.swift"
            | "program.cs"
            | "main.java"
            | "__main__.py"
            | "app.ts"
            | "app.js"
            | "app.py"
            | "server.ts"
            | "server.js"
    ) || matches!(
        lower_path.as_str(),
        "src/main.rs"
            | "src/index.ts"
            | "src/index.js"
            | "src/main.ts"
            | "src/main.js"
            | "src/main.py"
            | "src/lib.rs"
            | "index.ts"
            | "index.js"
    ) {
        return FilePurpose::Entry;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "layouts" | "layout"))
        || stem == "layout"
    {
        return FilePurpose::Layout;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "pages" | "page" | "views" | "view"))
        || stem.ends_with(".page")
        || stem == "page"
    {
        return FilePurpose::Page;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "widgets" | "widget"))
    {
        return FilePurpose::Widget;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "components" | "component"))
        || file_name.ends_with(".component.tsx")
        || file_name.ends_with(".component.jsx")
    {
        return FilePurpose::Component;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "routers" | "router" | "routes" | "route"))
        || stem.contains("router")
        || stem.contains("route")
    {
        return FilePurpose::Router;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "controllers" | "controller"))
        || stem.contains("controller")
    {
        return FilePurpose::Controller;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "handlers" | "handler"))
        || stem.contains("handler")
        || stem.contains("command")
    {
        return FilePurpose::Handler;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "services" | "service" | "usecases" | "usecase"))
        || stem.contains("service")
    {
        return FilePurpose::Service;
    }

    if segments.iter().any(|segment| {
        matches!(
            *segment,
            "repositories" | "repository" | "repos" | "repo" | "daos" | "dao" | "stores" | "store"
        )
    }) || stem.contains("repository")
        || stem.contains("repo")
        || stem.contains("dao")
    {
        return FilePurpose::Repository;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "domain" | "domains"))
    {
        return FilePurpose::Domain;
    }

    if segments.iter().any(|segment| {
        matches!(
            *segment,
            "models" | "model" | "entities" | "entity" | "schemas" | "schema"
        )
    }) || stem.contains("model")
        || stem.contains("entity")
    {
        return FilePurpose::Model;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "agents" | "agent"))
    {
        return FilePurpose::Agent;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "middlewares" | "middleware"))
        || stem.contains("middleware")
    {
        return FilePurpose::Middleware;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "plugins" | "plugin"))
        || stem.contains("plugin")
    {
        return FilePurpose::Plugin;
    }

    if matches!(file_name.as_str(), "lib.rs" | "mod.rs")
        || segments
            .iter()
            .any(|segment| matches!(*segment, "libs" | "lib" | "shared"))
    {
        return FilePurpose::Library;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "helpers" | "helper"))
        || stem.contains("helper")
    {
        return FilePurpose::Helper;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "constants" | "constant" | "consts"))
        || stem.contains("constant")
        || stem.contains("const")
    {
        return FilePurpose::Constant;
    }

    if segments.iter().any(|segment| {
        matches!(
            *segment,
            "types"
                | "type"
                | "interfaces"
                | "interface"
                | "contracts"
                | "contract"
                | "dtos"
                | "dto"
        )
    }) || stem.contains("types")
        || stem.ends_with(".d")
    {
        return FilePurpose::Type;
    }

    if segments
        .iter()
        .any(|segment| matches!(*segment, "utils" | "util" | "common"))
        || stem.contains("util")
    {
        return FilePurpose::Utility;
    }

    FilePurpose::Utility
}

fn is_structural_config_file(path: &str, kind: &str) -> bool {
    matches!(classify_structural_purpose(path, kind), FilePurpose::Config)
}

fn is_structural_entry_point(path: &str, kind: &str) -> bool {
    let purpose = classify_structural_purpose(path, kind);
    !matches!(purpose, FilePurpose::Test)
        && (matches!(purpose, FilePurpose::Entry | FilePurpose::Router)
            || detect_tags(path, purpose)
                .iter()
                .any(|tag| tag == "entry-point"))
}

fn classify_structural_purpose(path: &str, kind: &str) -> FilePurpose {
    if is_structural_config_path(path) {
        return FilePurpose::Config;
    }

    classify_file_purpose(path, kind)
}

fn is_structural_config_path(path: &str) -> bool {
    matches!(
        Path::new(path).file_name().and_then(|name| name.to_str()),
        Some(
            ".gitignore"
                | ".editorconfig"
                | "Makefile"
                | "go.mod"
                | "go.sum"
                | "Cargo.lock"
                | "package-lock.json"
                | "pnpm-lock.yaml"
                | "yarn.lock"
                | "bun.lockb"
                | "docker-compose.yml"
                | "docker-compose.yaml"
                | "wiki.dev.yaml"
                | "wiki.yaml"
        )
    )
}

/// 语言识别目前是轻量启发式。
/// 它主要用在技术栈提示和依赖线索抽取上，不承担完整语法解析职责。
///
/// # 参数
/// - `path`：仓库内相对路径。
///
/// # 返回
/// - 返回该文件对应的语言标签。
fn detect_language(path: &str) -> String {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "rust".to_string(),
        Some("ts") => "typescript".to_string(),
        Some("tsx") | Some("jsx") => "react".to_string(),
        Some("js") | Some("mjs") | Some("cjs") => "javascript".to_string(),
        Some("py") => "python".to_string(),
        Some("go") => "go".to_string(),
        Some("c") | Some("h") => "c".to_string(),
        Some("cc") | Some("cpp") | Some("cxx") | Some("hpp") | Some("hh") | Some("hxx") => {
            "cpp".to_string()
        }
        Some("java") => "java".to_string(),
        Some("cs") | Some("csproj") | Some("sln") | Some("sqlproj") | Some("sql") => {
            "csharp".to_string()
        }
        Some("kt") | Some("kts") => "kotlin".to_string(),
        Some("php") => "php".to_string(),
        Some("swift") => "swift".to_string(),
        Some("vue") => "vue".to_string(),
        Some("svelte") => "svelte".to_string(),
        Some("html") => "html".to_string(),
        Some("css") => "css".to_string(),
        Some("bat") => "batch".to_string(),
        Some("json") => "json".to_string(),
        Some("md") => "markdown".to_string(),
        Some("toml") => "toml".to_string(),
        Some("yaml") | Some("yml") => "yaml".to_string(),
        Some("conf") | Some("ini") => "config".to_string(),
        _ => "text".to_string(),
    }
}

/// tags 是后续模块拆分时最早可用的信号。
/// 比如入口文件标签会直接影响“这个目录是否值得提升为模块”。
///
/// # 参数
/// - `path`：仓库内相对路径。
/// - `kind`：该文件的粗粒度角色分类。
///
/// # 返回
/// - 返回扫描阶段可直接得到的标签列表。
fn detect_tags(path: &str, purpose: FilePurpose) -> Vec<String> {
    let mut tags = Vec::new();
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if purpose == FilePurpose::Config {
        tags.push("config".to_string());
    }

    // test 路径文件降权标记，供 hierarchy 和 generation 层消费
    if is_test_path(path) {
        tags.push("test-file".to_string());
    }

    if purpose == FilePurpose::Entry
        || matches!(
            path,
            "src/main.rs"
                | "src/lib.rs"
                | "src/index.ts"
                | "src/index.js"
                | "index.ts"
                | "index.js"
                | "src/Main.java"
                | "src/Program.cs"
        )
        || matches!(
            file_name,
            "main.ts"
                | "main.js"
                | "main.py"
                | "main.kt"
                | "main.swift"
                | "app.ts"
                | "app.js"
                | "app.py"
                | "index.php"
                | "Program.cs"
                | "Main.java"
                | "__main__.py"
                | "nginx.conf"
        )
    {
        tags.push("entry-point".to_string());
    }

    tags
}

/// workspace roots 是“显式模块边界”的第一来源。
/// 这里优先识别 package/cargo/python 这类 manifest 所在目录。
///
/// # 参数
/// - `config_files`：本次扫描得到的配置文件路径列表。
///
/// # 返回
/// - 返回可作为显式模块边界的目录列表。
fn discover_workspace_roots(config_files: &[String]) -> Vec<String> {
    let mut roots = std::collections::BTreeSet::new();

    for config_file in config_files {
        if matches!(
            Path::new(config_file)
                .file_name()
                .and_then(|name| name.to_str()),
            Some("package.json" | "Cargo.toml" | "pyproject.toml" | "requirements.txt" | "Pipfile")
        ) {
            let parent = Path::new(config_file)
                .parent()
                .map(|path| path.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|| ".".to_string());
            roots.insert(if parent.is_empty() {
                ".".to_string()
            } else {
                parent
            });
        }
    }

    if roots.is_empty() {
        roots.insert(".".to_string());
    }

    roots.into_iter().collect()
}

/// 依赖线索提取目前只做启发式。
/// 目标不是还原完整语义图，而是为跨模块关系提供最小可用证据。
///
/// # 参数
/// - `root`：仓库根目录，用于回读源码内容。
/// - `files`：本次扫描得到的文件列表。
/// - `import_aliases`：manifest 阶段提取出的内部依赖别名映射。
///
/// # 返回
/// - 返回可用于模块关系推断的依赖线索集合。
fn collect_dependency_hints(
    root: &Path,
    files: &[ScannedFile],
    import_aliases: &std::collections::BTreeMap<String, String>,
) -> Vec<DependencyHint> {
    let mut hints = Vec::new();

    for file in files {
        if !matches!(
            file.language.as_str(),
            "typescript"
                | "javascript"
                | "rust"
                | "python"
                | "java"
                | "csharp"
                | "kotlin"
                | "php"
                | "swift"
                | "react"
                | "vue"
                | "svelte"
        ) {
            continue;
        }

        let absolute_path = root.join(&file.path);
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            continue;
        };

        for target in extract_source_dependency_targets(&file.language, &content) {
            // 这里只有“仓库内部依赖线索”才会继续往下传。
            // 例如 npm 包名、第三方 Python 包名会被过滤掉，避免它们被误识别成仓库模块。
            if let Some(normalized) =
                normalize_dependency_target(&file.path, &target, &file.language, import_aliases)
            {
                hints.push(DependencyHint {
                    from: file.path.clone(),
                    to: normalized,
                    kind: "DEPENDS_ON".to_string(),
                    confidence: "parsed".to_string(),
                });
            }
        }
    }

    hints
}

/// 前端消费 `/api` 与后端暴露 `/api` 路由是混合仓库里最稳定的跨模块线索之一。
/// 这里不做语义推理，只在“前端显式调用 API”与“后端显式暴露 API”同时存在时记录依赖。
fn collect_service_api_hints(root: &Path, files: &[ScannedFile]) -> Vec<DependencyHint> {
    let mut consumers = Vec::new();
    let mut providers = Vec::new();

    for file in files {
        if !matches!(
            file.language.as_str(),
            "typescript"
                | "javascript"
                | "react"
                | "vue"
                | "svelte"
                | "python"
                | "java"
                | "csharp"
                | "kotlin"
                | "php"
        ) {
            continue;
        }

        let absolute_path = root.join(&file.path);
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            continue;
        };

        if contains_frontend_api_consumer_hint(file, &content) {
            consumers.push(file.path.clone());
        }

        if contains_backend_api_provider_hint(file, &content) {
            providers.push(file.path.clone());
        }
    }

    let mut hints = Vec::new();

    for consumer in &consumers {
        for provider in &providers {
            if consumer == provider {
                continue;
            }

            hints.push(DependencyHint {
                from: consumer.clone(),
                to: provider.clone(),
                kind: "DEPENDS_ON".to_string(),
                confidence: "heuristic".to_string(),
            });
        }
    }

    hints
}

/// 基础设施配置里如果显式引用仓库内的静态产物目录，就把它折叠成模块依赖线索。
fn collect_infrastructure_dependency_hints(
    root: &Path,
    files: &[ScannedFile],
) -> Vec<DependencyHint> {
    let mut hints = Vec::new();

    for file in files {
        if !file.path.ends_with("nginx.conf") {
            continue;
        }

        let absolute_path = root.join(&file.path);
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            continue;
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(target) = parse_nginx_root_target(line)
                .and_then(|target| relative_path_from_config_target(root, &target))
            {
                hints.push(DependencyHint {
                    from: file.path.clone(),
                    to: target,
                    kind: "SERVES_STATIC".to_string(),
                    confidence: "parsed".to_string(),
                });
            }
        }
    }

    hints
}

fn contains_frontend_api_consumer_hint(file: &ScannedFile, content: &str) -> bool {
    if !matches!(
        file.language.as_str(),
        "typescript" | "javascript" | "react" | "vue" | "svelte"
    ) {
        return false;
    }

    let normalized = content.to_ascii_lowercase();

    (file.path.contains("/api/") || normalized.contains("/api"))
        && (normalized.contains("axios")
            || normalized.contains("fetch(")
            || normalized.contains("baseurl")
            || normalized.contains("vite_api_base"))
}

fn contains_backend_api_provider_hint(file: &ScannedFile, content: &str) -> bool {
    if !matches!(
        file.language.as_str(),
        "python" | "javascript" | "typescript" | "java" | "csharp" | "kotlin" | "php"
    ) {
        return false;
    }

    let normalized = content.to_ascii_lowercase();

    normalized.contains("@app.route('/api")
        || normalized.contains("@app.route(\"/api")
        || normalized.contains("app.get('/api")
        || normalized.contains("app.post('/api")
        || normalized.contains("router.get('/api")
        || normalized.contains("router.post('/api")
        || normalized.contains("requestmapping(\"/api")
        || normalized.contains("requestmapping('/api")
        || normalized.contains("map(\"/api")
}

fn parse_nginx_root_target(line: &str) -> Option<String> {
    let body = line.strip_prefix("root")?.trim();
    let value = body.split(';').next()?.trim();

    (!value.is_empty()).then(|| value.to_string())
}

fn relative_path_from_config_target(repo_root: &Path, target: &str) -> Option<String> {
    let normalized_target = target.trim().trim_matches(['"', '\'']).replace('\\', "/");
    let normalized_root = repo_root.to_string_lossy().replace('\\', "/");

    if let Some(relative) = normalized_target.strip_prefix(&normalized_root) {
        let relative = relative.trim_start_matches('/').to_string();
        return (!relative.is_empty()).then_some(relative);
    }

    if normalized_target.contains("://") {
        return None;
    }

    let repo_relative = normalized_target
        .trim_start_matches("./")
        .trim_start_matches('/');
    let candidate = repo_root.join(repo_relative);
    candidate
        .exists()
        .then(|| repo_relative.replace('\\', "/"))
        .filter(|relative| !relative.is_empty())
}

/// 某些语言的内部别名并不写在 manifest 里，而是写在源码里。
/// 例如 Java/Kotlin 的 `package`、C# 的 `namespace`、PHP 的 `namespace`。
///
/// # 参数
/// - `root`：仓库根目录。
/// - `files`：本次扫描得到的文件列表。
///
/// # 返回
/// - 返回“源码声明别名 -> 模块根路径”的映射表。
fn collect_source_aliases(
    root: &Path,
    files: &[ScannedFile],
) -> std::collections::BTreeMap<String, String> {
    let mut aliases = std::collections::BTreeMap::new();

    for file in files {
        if !matches!(file.language.as_str(), "java" | "kotlin" | "csharp" | "php") {
            continue;
        }

        let absolute_path = root.join(&file.path);
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            continue;
        };

        for alias in parse_source_aliases(&file.language, &content) {
            aliases
                .entry(alias)
                .or_insert_with(|| source_root_for_alias(&file.path));
        }
    }

    aliases
}

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

    if matches!(
        first,
        "crates" | "agents" | "apps" | "services" | "libs" | "packages"
    ) {
        let second = segments.next().unwrap_or(".");
        if second != "." {
            return format!("{first}/{second}");
        }
    }

    first.to_string()
}

/// 扫描阶段会合并 manifest 依赖和源码依赖，需要在这里统一去重。
///
/// # 参数
/// - `hints`：待去重的依赖线索列表。
///
/// # 返回
/// - 返回按 `from/to/kind` 去重后的依赖线索列表。
fn dedupe_dependency_hints(hints: Vec<DependencyHint>) -> Vec<DependencyHint> {
    let mut deduped = BTreeSet::new();
    let mut result = Vec::new();

    for hint in hints {
        let key = format!("{}|{}|{}", hint.from, hint.to, hint.kind);
        if deduped.insert(key) {
            result.push(hint);
        }
    }

    result
}

/// 快速扫描仓库根目录下的 manifest 文件名，
/// 用于判断仓库类型和 workspace 成员白名单。
fn discover_root_manifests(root: &Path) -> Vec<String> {
    let mut manifests = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return manifests;
    };

    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if matches!(
                name,
                "Cargo.toml"
                    | "package.json"
                    | "pyproject.toml"
                    | "go.mod"
                    | "pom.xml"
                    | "build.gradle"
                    | "build.gradle.kts"
                    | "pnpm-workspace.yaml"
            ) {
                manifests.push(name.to_string());
            }
        }
    }

    manifests
}

/// 检测子目录是否为嵌套仓库。
/// 含 `.git` 目录的子目录视为嵌套仓库（workspace 成员除外）。
fn is_nested_repo(dir: &Path, _root_manifests: &[String]) -> bool {
    dir.join(".git").is_dir()
}

/// 判断文件路径是否位于 test / spec 相关目录下。
/// 用于在扫描阶段标记降权信号，供 hierarchy 和 generation 层消费。
fn is_test_path(path: &str) -> bool {
    let segments: Vec<&str> = path.split('/').collect();
    segments
        .iter()
        .any(|seg| matches!(*seg, "tests" | "test" | "spec" | "__tests__" | "__test__"))
}

#[cfg(test)]
mod tests {
    use super::scan_repo;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scan_repo_ignores_hidden_agent_runtime_dirs() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("docs")).unwrap();
        fs::create_dir_all(repo.path().join(".qoder/repowiki/zh/content")).unwrap();
        fs::write(repo.path().join("docs/get-started.md"), "# Get Started\n").unwrap();
        fs::write(
            repo.path().join(".qoder/repowiki/zh/content/快速开始.md"),
            "# 快速开始\n",
        )
        .unwrap();

        let report = scan_repo(repo.path(), &[]).unwrap();
        let paths = report
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert!(paths.contains(&"docs/get-started.md"));
        assert!(
            !paths
                .iter()
                .any(|path| path.starts_with(".qoder/") || path.contains("/repowiki/")),
            "hidden runtime corpus should stay out of scan report: {:?}",
            paths
        );
    }
}
