use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde_json::Value;

use crate::domain::stable_id::stable_id;
use crate::repo::parsers::{analyze_manifests, normalize_dependency_target};
use crate::repo::scanner::ScanReport;
use crate::repo::symbol_graph::{GraphDiagnostic, ResolvedGraphSnapshot, ResolvedSymbolEdge};
use crate::repo::symbols::{ParsedSymbolsSnapshot, RawImportCapture, SymbolNode, SymbolTable};

/// `SuffixIndex` 为 import resolution 提供近似 O(1) 的 suffix/path 查找。
#[derive(Debug, Clone, Default)]
pub struct SuffixIndex {
    exact: BTreeMap<String, Vec<String>>,
    insensitive: BTreeMap<String, Vec<String>>,
    dir_members: BTreeMap<String, Vec<String>>,
}

impl SuffixIndex {
    /// 基于仓库内源码路径构建 suffix 索引。
    pub fn build(file_paths: &[String]) -> Self {
        let mut index = Self::default();

        for file_path in file_paths {
            let normalized = normalize_repo_path(file_path);
            let parts = normalized.split('/').collect::<Vec<_>>();

            for offset in 0..parts.len() {
                let suffix = parts[offset..].join("/");
                index.exact.entry(suffix.clone()).or_default().push(file_path.clone());
                index
                    .insensitive
                    .entry(suffix.to_ascii_lowercase())
                    .or_default()
                    .push(file_path.clone());
            }

            let file_path_obj = Path::new(&normalized);
            let extension = file_path_obj
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| format!(".{ext}"))
                .unwrap_or_default();
            if let Some(parent) = file_path_obj.parent() {
                let dir = normalize_repo_path(&parent.to_string_lossy());
                if !dir.is_empty() && !extension.is_empty() {
                    let dir_parts = dir.split('/').collect::<Vec<_>>();
                    for offset in 0..dir_parts.len() {
                        let dir_suffix = dir_parts[offset..].join("/");
                        index
                            .dir_members
                            .entry(format!("{dir_suffix}:{extension}"))
                            .or_default()
                            .push(file_path.clone());
                    }
                }
            }
        }

        for values in index.exact.values_mut() {
            values.sort();
            values.dedup();
        }
        for values in index.insensitive.values_mut() {
            values.sort();
            values.dedup();
        }
        for values in index.dir_members.values_mut() {
            values.sort();
            values.dedup();
        }

        index
    }

    /// 按精确 suffix 取候选文件。
    pub fn lookup(&self, suffix: &str) -> Vec<String> {
        self.exact.get(suffix).cloned().unwrap_or_default()
    }

    /// 按大小写不敏感 suffix 取候选文件。
    pub fn lookup_insensitive(&self, suffix: &str) -> Vec<String> {
        self.insensitive
            .get(&suffix.to_ascii_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    /// 按目录 suffix 和扩展名取候选文件。
    pub fn lookup_dir(&self, dir_suffix: &str, extension: &str) -> Vec<String> {
        self.dir_members
            .get(&format!("{}:{extension}", dir_suffix))
            .cloned()
            .unwrap_or_default()
    }
}

/// `tsconfig` 路径别名的一条 rewrite 规则。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TsconfigAlias {
    alias_prefix: String,
    target_prefix: String,
}

/// TypeScript import resolution 所需的 `tsconfig` 配置。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TsconfigPaths {
    base_url: String,
    aliases: Vec<TsconfigAlias>,
}

/// 单轮 import resolution 复用的上下文。
#[derive(Debug, Clone, Default)]
pub struct ImportResolutionContext {
    /// 仓库内全部源码路径集合。
    pub all_source_paths: BTreeSet<String>,
    /// 归一化后的源码路径列表。
    pub normalized_file_list: Vec<String>,
    /// suffix 查找索引。
    pub suffix_index: SuffixIndex,
    /// `current_file + raw_import` 的 resolve cache。
    pub resolve_cache: RefCell<BTreeMap<String, Vec<String>>>,
    /// manifest 和 workspace 派生出的内部别名映射。
    pub import_aliases: BTreeMap<String, String>,
    /// TypeScript path aliases。
    tsconfig_paths: TsconfigPaths,
    /// `go.mod` 声明的 module path。
    go_module_path: Option<String>,
    /// Composer 的 PSR-4 namespace 到目录映射。
    composer_psr4: BTreeMap<String, String>,
    /// Swift target 名到源码目录映射。
    swift_targets: BTreeMap<String, String>,
}

/// 构建 import resolution 需要复用的路径和配置上下文。
///
/// # 参数
/// - `repo_root`：仓库根目录。
/// - `scan_report`：当前轮扫描结果。
///
/// # 返回
/// - 返回供 `resolve_imports` 复用的 lookup 上下文。
///
/// # 错误
/// - 当读取配置文件失败且属于真实 I/O 错误时返回错误。
pub fn build_import_resolution_context(
    repo_root: &Path,
    scan_report: &ScanReport,
) -> io::Result<ImportResolutionContext> {
    let source_paths = scan_report
        .files
        .iter()
        .filter(|file| file.kind == "source")
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();
    let manifest_analysis = analyze_manifests(repo_root, &scan_report.files);
    let tsconfig_paths = load_tsconfig_paths(repo_root)?;
    let go_module_path = load_go_module_path(repo_root)?;
    let composer_psr4 = load_composer_psr4(repo_root)?;
    let swift_targets = load_swift_targets(repo_root)?;

    Ok(ImportResolutionContext {
        all_source_paths: source_paths.iter().cloned().collect(),
        normalized_file_list: source_paths.iter().map(|path| normalize_repo_path(path)).collect(),
        suffix_index: SuffixIndex::build(&source_paths),
        resolve_cache: RefCell::new(BTreeMap::new()),
        import_aliases: manifest_analysis.import_aliases,
        tsconfig_paths,
        go_module_path,
        composer_psr4,
        swift_targets,
    })
}

/// 把 parser 阶段的 raw import captures 解析成稳定 `IMPORTS` edges。
///
/// # 参数
/// - `snapshot`：parser 阶段产出的 definitions 与 raw import captures。
/// - `context`：预构建的 import resolution 上下文。
///
/// # 返回
/// - 返回稳定 `IMPORTS` edges 与 unresolved diagnostics。
pub fn resolve_imports(
    snapshot: &ParsedSymbolsSnapshot,
    context: &ImportResolutionContext,
) -> ResolvedGraphSnapshot {
    let symbols_by_id = snapshot
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let mut symbols_by_file = BTreeMap::<String, Vec<&SymbolNode>>::new();
    for symbol in &snapshot.symbols {
        symbols_by_file
            .entry(symbol.file_path.clone())
            .or_default()
            .push(symbol);
    }
    for symbols in symbols_by_file.values_mut() {
        symbols.sort_by(|left, right| {
            left.start_line
                .cmp(&right.start_line)
                .then(left.end_line.cmp(&right.end_line))
                .then(left.symbol_id.cmp(&right.symbol_id))
        });
    }

    let mut edges = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();

    for parsed_file in snapshot.files.values() {
        for capture in &parsed_file.imports {
            let source_symbols =
                import_source_symbols(capture, &symbols_by_id, &symbols_by_file, &snapshot.symbol_table);
            if source_symbols.is_empty() {
                diagnostics.push(unresolved_import_diagnostic(
                    capture,
                    "missing_source_symbol",
                    "import capture cannot be attached to any source symbol",
                ));
                continue;
            }

            let resolved_files = resolve_import_capture(context, capture);
            if resolved_files.is_empty() {
                diagnostics.push(unresolved_import_diagnostic(
                    capture,
                    "unresolved_import",
                    "import target did not match any repository source file",
                ));
                continue;
            }

            let mut target_symbols = Vec::<(&SymbolNode, String)>::new();
            for target_file in resolved_files {
                for target in target_symbols_for_file(&target_file, &symbols_by_file) {
                    target_symbols.push((target, target_file.clone()));
                }
            }
            if target_symbols.is_empty() {
                diagnostics.push(unresolved_import_diagnostic(
                    capture,
                    "missing_target_symbol",
                    "import target resolved to file(s) without symbols",
                ));
                continue;
            }

            for source_symbol in &source_symbols {
                for (target_symbol, target_file) in &target_symbols {
                    let reason = if capture.source_symbol_id.is_some() {
                        format!("resolved-import:{}:{}", capture.raw_path, target_file)
                    } else {
                        format!("resolved-import:file-owner:{}:{}", capture.raw_path, target_file)
                    };
                    let edge_id = stable_id(
                        "edge",
                        format!(
                            "IMPORTS:{}:{}:{}:{}",
                            source_symbol.symbol_id, target_symbol.symbol_id, capture.line, capture.raw_path
                        ),
                    );
                    if seen.insert(edge_id.clone()) {
                        edges.push(ResolvedSymbolEdge {
                            edge_id,
                            source_id: source_symbol.symbol_id.clone(),
                            target_id: target_symbol.symbol_id.clone(),
                            edge_type: "IMPORTS".to_string(),
                            confidence: 1.0,
                            reason,
                        });
                    }
                }
            }
        }
    }

    ResolvedGraphSnapshot { edges, diagnostics }
}

fn unresolved_import_diagnostic(
    capture: &RawImportCapture,
    kind: &str,
    message: &str,
) -> GraphDiagnostic {
    GraphDiagnostic {
        stage: "resolve_imports".to_string(),
        kind: kind.to_string(),
        message: format!("{}:{} -> {}", capture.file_path, capture.line, message),
        file_path: Some(capture.file_path.clone()),
        symbol_id: capture.source_symbol_id.clone(),
    }
}

fn import_source_symbols<'a>(
    capture: &RawImportCapture,
    symbols_by_id: &'a BTreeMap<String, &'a SymbolNode>,
    symbols_by_file: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
    symbol_table: &'a SymbolTable,
) -> Vec<&'a SymbolNode> {
    if let Some(source_symbol_id) = &capture.source_symbol_id {
        return symbols_by_id
            .get(source_symbol_id)
            .copied()
            .into_iter()
            .collect();
    }

    let file_owner = file_owner_symbol(symbols_by_file, &capture.file_path);
    if file_owner.is_some() {
        return file_owner.into_iter().collect();
    }

    // 文件内如果没有 owner，但恰好只有一个同文件定义，也保守回落到它。
    symbol_table
        .file_index
        .get(&capture.file_path)
        .and_then(|names| {
            let mut candidates = names
                .values()
                .flat_map(|ids| ids.iter())
                .filter_map(|symbol_id| symbols_by_id.get(symbol_id).copied())
                .collect::<Vec<_>>();
            candidates.sort_by(|left, right| left.start_line.cmp(&right.start_line));
            (candidates.len() == 1).then_some(candidates)
        })
        .unwrap_or_default()
}

fn target_symbols_for_file<'a>(
    file_path: &str,
    symbols_by_file: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
) -> Vec<&'a SymbolNode> {
    let Some(candidates) = symbols_by_file.get(file_path) else {
        return Vec::new();
    };
    let exported = candidates
        .iter()
        .copied()
        .filter(|symbol| symbol.is_exported)
        .collect::<Vec<_>>();
    if !exported.is_empty() {
        return exported;
    }

    file_owner_symbol(symbols_by_file, file_path)
        .into_iter()
        .collect()
}

fn file_owner_symbol<'a>(
    symbols_by_file: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
    file_path: &str,
) -> Option<&'a SymbolNode> {
    let candidates = symbols_by_file.get(file_path)?;
    candidates
        .iter()
        .copied()
        .find(|symbol| symbol.is_exported)
        .or_else(|| candidates.first().copied())
}

fn resolve_import_capture(
    context: &ImportResolutionContext,
    capture: &RawImportCapture,
) -> Vec<String> {
    let cache_key = format!("{}::{}", capture.file_path, capture.raw_path);
    if let Some(cached) = context.resolve_cache.borrow().get(&cache_key) {
        return cached.clone();
    }

    let mut candidates = rewrite_tsconfig_alias(context, &capture.raw_path)
        .into_iter()
        .chain(rewrite_rust_import(capture).into_iter())
        .chain(
            normalize_dependency_target(
                &capture.file_path,
                &capture.raw_path,
                &capture.language,
                &context.import_aliases,
            )
            .into_iter(),
        )
        .chain(rewrite_go_module_import(context, &capture.raw_path).into_iter())
        .chain(rewrite_php_psr4_import(context, &capture.raw_path).into_iter())
        .chain(rewrite_swift_target_import(context, &capture.raw_path).into_iter())
        .map(|candidate| normalize_repo_path(&candidate))
        .collect::<Vec<_>>();

    if candidates.is_empty()
        && (capture.raw_path.starts_with("./") || capture.raw_path.starts_with("../"))
    {
        candidates.push(resolve_relative_repo_path(&capture.file_path, &capture.raw_path));
    }

    let mut resolved = Vec::new();
    let mut seen = BTreeSet::new();
    for candidate in candidates {
        for file_path in resolve_candidate_to_files(context, &candidate, &capture.language) {
            if seen.insert(file_path.clone()) {
                resolved.push(file_path);
            }
        }
    }

    context
        .resolve_cache
        .borrow_mut()
        .insert(cache_key, resolved.clone());
    resolved
}

fn rewrite_tsconfig_alias(context: &ImportResolutionContext, raw_path: &str) -> Option<String> {
    if raw_path.starts_with('.') {
        return None;
    }

    for alias in &context.tsconfig_paths.aliases {
        if raw_path == alias.alias_prefix
            || raw_path.starts_with(&format!("{}/", alias.alias_prefix.trim_end_matches('/')))
        {
            let remainder = raw_path
                .strip_prefix(&alias.alias_prefix)
                .or_else(|| raw_path.strip_prefix(alias.alias_prefix.trim_end_matches('/')))
                .unwrap_or_default()
                .trim_start_matches('/');
            let candidate = if context.tsconfig_paths.base_url == "." {
                join_repo_paths(&alias.target_prefix, remainder)
            } else {
                join_repo_paths(
                    &join_repo_paths(&context.tsconfig_paths.base_url, &alias.target_prefix),
                    remainder,
                )
            };
            return Some(candidate);
        }
    }

    None
}

fn rewrite_go_module_import(context: &ImportResolutionContext, raw_path: &str) -> Option<String> {
    let module_path = context.go_module_path.as_ref()?;
    let relative = raw_path.strip_prefix(module_path)?.trim_start_matches('/');
    (!relative.is_empty()).then(|| relative.to_string())
}

fn rewrite_rust_import(capture: &RawImportCapture) -> Option<String> {
    if capture.language != "rust" {
        return None;
    }

    if let Some(relative) = capture.raw_path.strip_prefix("crate/") {
        let crate_root = rust_crate_root(&capture.file_path)?;
        return Some(join_repo_paths(&crate_root, relative));
    }

    if let Some(relative) = capture.raw_path.strip_prefix("self/") {
        let parent = Path::new(&capture.file_path)
            .parent()
            .map(|path| normalize_repo_path(&path.to_string_lossy()))
            .unwrap_or_default();
        return Some(join_repo_paths(&parent, relative));
    }

    if let Some(relative) = capture.raw_path.strip_prefix("super/") {
        let parent = Path::new(&capture.file_path)
            .parent()
            .and_then(|path| path.parent())
            .map(|path| normalize_repo_path(&path.to_string_lossy()))
            .unwrap_or_default();
        return Some(join_repo_paths(&parent, relative));
    }

    None
}

fn rewrite_php_psr4_import(context: &ImportResolutionContext, raw_path: &str) -> Option<String> {
    let canonical = raw_path.replace('\\', "/");
    context
        .composer_psr4
        .iter()
        .find_map(|(namespace_prefix, directory)| {
            let prefix = namespace_prefix.replace('\\', "/");
            let trimmed_prefix = prefix.trim_end_matches('/');
            (canonical == trimmed_prefix || canonical.starts_with(&format!("{trimmed_prefix}/")))
                .then(|| canonical.strip_prefix(trimmed_prefix).unwrap_or_default())
                .map(|suffix| join_repo_paths(directory, suffix.trim_start_matches('/')))
        })
}

fn rewrite_swift_target_import(context: &ImportResolutionContext, raw_path: &str) -> Option<String> {
    context.swift_targets.get(raw_path).cloned()
}

fn resolve_candidate_to_files(
    context: &ImportResolutionContext,
    candidate: &str,
    language: &str,
) -> Vec<String> {
    let candidate = normalize_repo_path(candidate);
    let mut resolved = Vec::new();
    let mut seen = BTreeSet::new();

    if context.all_source_paths.contains(&candidate) {
        resolved.push(candidate.clone());
        seen.insert(candidate.clone());
    }

    for extension in preferred_extensions(language) {
        let candidate_with_extension = if extension.starts_with('/') {
            format!("{candidate}{extension}")
        } else {
            format!("{candidate}{extension}")
        };
        for file_path in lookup_suffix(context, &candidate_with_extension) {
            if seen.insert(file_path.clone()) {
                resolved.push(file_path);
            }
        }
    }

    if resolved.is_empty() {
        for extension in preferred_directory_extensions(language) {
            for file_path in context.suffix_index.lookup_dir(&candidate, extension) {
                if seen.insert(file_path.clone()) {
                    resolved.push(file_path);
                }
            }
        }
    }

    if resolved.is_empty() && language == "rust" {
        if let Some((parent_candidate, _)) = candidate.rsplit_once('/') {
            for extension in preferred_extensions(language) {
                let candidate_with_extension = format!("{parent_candidate}{extension}");
                for file_path in lookup_suffix(context, &candidate_with_extension) {
                    if seen.insert(file_path.clone()) {
                        resolved.push(file_path);
                    }
                }
            }
        }
    }

    if resolved.is_empty() {
        for file_path in lookup_suffix(context, &candidate) {
            if seen.insert(file_path.clone()) {
                resolved.push(file_path);
            }
        }
    }

    resolved
}

fn lookup_suffix(context: &ImportResolutionContext, suffix: &str) -> Vec<String> {
    let mut results = context.suffix_index.lookup(suffix);
    if results.is_empty() {
        results = context.suffix_index.lookup_insensitive(suffix);
    }
    results
}

fn preferred_extensions(language: &str) -> &'static [&'static str] {
    match language {
        "javascript" | "typescript" => &[
            "",
            ".ts",
            ".tsx",
            ".js",
            ".jsx",
            ".vue",
            ".svelte",
            "/index.ts",
            "/index.tsx",
            "/index.js",
            "/index.jsx",
        ],
        "rust" => &["", ".rs", "/mod.rs"],
        "java" => &["", ".java"],
        "kotlin" => &["", ".kt", ".kts"],
        "go" => &["", ".go"],
        "php" => &["", ".php", ".phtml"],
        "swift" => &["", ".swift"],
        "python" => &["", ".py", "/__init__.py"],
        "csharp" => &["", ".cs"],
        _ => &[""],
    }
}

fn preferred_directory_extensions(language: &str) -> &'static [&'static str] {
    match language {
        "go" => &[".go"],
        "swift" => &[".swift"],
        _ => &[],
    }
}

fn load_tsconfig_paths(repo_root: &Path) -> io::Result<TsconfigPaths> {
    let candidates = ["tsconfig.json", "tsconfig.app.json", "tsconfig.base.json"];
    let strip_comments =
        Regex::new(r"(?m)//.*$|/\*[\s\S]*?\*/").map_err(|e| io::Error::other(e.to_string()))?;

    for candidate in candidates {
        let path = repo_root.join(candidate);
        if !path.is_file() {
            continue;
        }

        let raw = fs::read_to_string(&path)?;
        let stripped = strip_comments.replace_all(&raw, "");
        let parsed = match serde_json::from_str::<Value>(&stripped) {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        let compiler_options = parsed
            .get("compilerOptions")
            .and_then(|value| value.as_object());
        let Some(compiler_options) = compiler_options else {
            continue;
        };
        let Some(paths) = compiler_options.get("paths").and_then(|value| value.as_object()) else {
            continue;
        };

        let base_url = compiler_options
            .get("baseUrl")
            .and_then(|value| value.as_str())
            .unwrap_or(".")
            .to_string();
        let aliases = paths
            .iter()
            .filter_map(|(alias, targets)| {
                let target = targets.as_array()?.first()?.as_str()?;
                Some(TsconfigAlias {
                    alias_prefix: alias.trim_end_matches("/*").to_string(),
                    target_prefix: target.trim_end_matches("/*").to_string(),
                })
            })
            .collect::<Vec<_>>();
        if !aliases.is_empty() {
            return Ok(TsconfigPaths { base_url, aliases });
        }
    }

    Ok(TsconfigPaths::default())
}

fn load_go_module_path(repo_root: &Path) -> io::Result<Option<String>> {
    let path = repo_root.join("go.mod");
    if !path.is_file() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .find_map(|line| line.strip_prefix("module ").map(|value| value.trim().to_string()))
        .filter(|value| !value.is_empty()))
}

fn load_composer_psr4(repo_root: &Path) -> io::Result<BTreeMap<String, String>> {
    let path = repo_root.join("composer.json");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }

    let content = fs::read_to_string(path)?;
    let parsed = match serde_json::from_str::<Value>(&content) {
        Ok(parsed) => parsed,
        Err(_) => return Ok(BTreeMap::new()),
    };
    let mut mappings = BTreeMap::new();
    for section in ["autoload", "autoload-dev"] {
        let Some(psr4) = parsed
            .get(section)
            .and_then(|section| section.get("psr-4"))
            .and_then(|value| value.as_object())
        else {
            continue;
        };
        for (namespace, directory) in psr4 {
            let Some(directory) = directory.as_str() else {
                continue;
            };
            mappings.insert(
                namespace.trim_end_matches('\\').to_string(),
                normalize_repo_path(directory),
            );
        }
    }

    Ok(mappings)
}

fn load_swift_targets(repo_root: &Path) -> io::Result<BTreeMap<String, String>> {
    let mut targets = BTreeMap::new();
    for source_dir in ["Sources", "Package/Sources", "src"] {
        let dir = repo_root.join(source_dir);
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let target_name = entry.file_name().to_string_lossy().to_string();
                targets.insert(
                    target_name,
                    normalize_repo_path(&PathBuf::from(source_dir).join(entry.file_name()).to_string_lossy()),
                );
            }
        }
    }

    Ok(targets)
}

fn resolve_relative_repo_path(source_path: &str, raw_path: &str) -> String {
    let parent = Path::new(source_path)
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();
    normalize_repo_path(&parent.join(raw_path).to_string_lossy())
}

fn join_repo_paths(left: &str, right: &str) -> String {
    if left.is_empty() {
        return normalize_repo_path(right);
    }
    if right.is_empty() {
        return normalize_repo_path(left);
    }
    normalize_repo_path(&format!(
        "{}/{}",
        left.trim_end_matches('/'),
        right.trim_start_matches('/')
    ))
}

fn normalize_repo_path(path: &str) -> String {
    let path = path.replace('\\', "/");
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

fn rust_crate_root(file_path: &str) -> Option<String> {
    let segments = file_path.split('/').collect::<Vec<_>>();
    let source_index = segments
        .iter()
        .rposition(|segment| matches!(*segment, "src"));
    source_index.map(|index| segments[..=index].join("/"))
}
