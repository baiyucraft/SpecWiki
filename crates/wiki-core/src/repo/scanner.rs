use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::stable_id::stable_id;
use crate::repo::detectors::detect_tech_hints;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::parsers::{
    analyze_manifests, extract_source_aliases as parse_source_aliases,
    extract_source_dependency_targets,
    normalize_dependency_target,
};

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
    /// 指纹用于 update/status 阶段判断源码是否变化。
    pub fingerprint: String,
    /// 文件大小目前主要用于调试和后续评分，不参与业务决策。
    pub size: usize,
    /// tags 放扫描阶段就能稳定得到的标签，例如 `entry-point`。
    pub tags: Vec<String>,
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
///
/// # 返回
/// - 成功时返回完整的扫描报告，包含文件、技术栈线索、入口和依赖线索。
///
/// # 错误
/// - 当目录不可读、文件读取失败或遍历过程中出现 I/O 错误时返回错误。
pub fn scan_repo(root: &Path) -> io::Result<ScanReport> {
    let mut files = Vec::new();
    visit_dir(root, root, &mut files)?;
    let manifest_analysis = analyze_manifests(root, &files);

    // 这些聚合字段会被 decomposition 和 workflow 直接消费，
    // 因此在扫描阶段就顺手整理出来，避免后续每一层都重复遍历。
    let paths = files.iter().map(|file| file.path.clone()).collect::<Vec<_>>();
    let config_files = files
        .iter()
        .filter(|file| file.kind == "config")
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    let entry_points = files
        .iter()
        .filter(|file| file.tags.iter().any(|tag| tag == "entry-point"))
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
    dependency_hints.extend(collect_dependency_hints(
        root,
        &files,
        &import_aliases,
    ));
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
fn visit_dir(root: &Path, dir: &Path, files: &mut Vec<ScannedFile>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            if should_ignore_dir(&path) {
                continue;
            }

            visit_dir(root, &path, files)?;
            continue;
        }

        // 运行产物、日志和二进制不参与 Repo Wiki 分析。
        // 它们会污染模块判断，还会让“关键源码”落到无关文件上。
        if should_ignore_file(&path) {
            continue;
        }

        let relative = make_relative(root, &path);
        let bytes = fs::read(&path)?;
        let kind = classify_file_kind(&relative);
        let language = detect_language(&relative);
        let tags = detect_tags(&relative, &kind);

        files.push(ScannedFile {
            id: stable_id("source", &relative),
            path: relative,
            language,
            kind,
            fingerprint: fingerprint_bytes(&bytes),
            size: bytes.len(),
            tags,
        });
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
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(
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
                | ".venv"
                | "venv"
        )
    )
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

    if path.ends_with(".md") {
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
fn detect_tags(path: &str, kind: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if kind == "config" {
        tags.push("config".to_string());
    }

    if matches!(
        path,
        "src/main.rs"
            | "src/lib.rs"
            | "src/index.ts"
            | "src/index.js"
            | "index.ts"
            | "index.js"
            | "src/Main.java"
            | "src/Program.cs"
    ) || matches!(
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
            roots.insert(if parent.is_empty() { ".".to_string() } else { parent });
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
        if !matches!(
            file.language.as_str(),
            "java" | "kotlin" | "csharp" | "php"
        ) {
            continue;
        }

        let absolute_path = root.join(&file.path);
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            continue;
        };

        for alias in parse_source_aliases(&file.language, &content) {
            aliases.entry(alias).or_insert_with(|| source_root_for_alias(&file.path));
        }
    }

    aliases
}

fn source_root_for_alias(source_path: &str) -> String {
    let mut segments = source_path.split('/').filter(|segment| !segment.is_empty());
    let first = segments.next().unwrap_or(".");

    if matches!(first, "crates" | "agents" | "apps" | "services" | "libs" | "packages") {
        let second = segments.next().unwrap_or(".");
        return format!("{first}/{second}");
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
