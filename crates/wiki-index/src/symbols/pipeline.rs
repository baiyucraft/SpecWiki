use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;

use regex::Regex;
use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator, Tree};

use wiki_model::domain::stable_id::stable_id;
use crate::scanner::{ScanReport, ScannedFile};

use super::models::{
    ParsedFileSymbols, ParsedSymbolsSnapshot, RawCallCapture, RawHeritageCapture, RawImportCapture,
    SymbolNode, SymbolParseDiagnostic, SymbolTable,
};
use super::registry::{resolve_embedded_language, resolve_symbol_language, ResolvedSymbolLanguage};

/// 单批解析预算。当前只用它约束顺序处理的分块边界。
pub const CHUNK_BYTE_BUDGET: usize = 20 * 1024 * 1024;
/// 超过这个大小的单文件先直接跳过，避免解析器被极端文件拖慢。
pub const MAX_FILE_BYTES: usize = 512 * 1024;

#[derive(Clone)]
struct ParseUnit {
    content: String,
    resolved_language: ResolvedSymbolLanguage,
    line_offset: usize,
    source_label: String,
}

struct ParsedUnitArtifacts {
    resolved_language: ResolvedSymbolLanguage,
    line_offset: usize,
    source_bytes: Vec<u8>,
    tree: Tree,
}

#[derive(Default)]
struct ParseWorkerContext {
    parser_cache: HashMap<usize, Parser>,
    query_cache: HashMap<usize, Query>,
}

/// 全量解析 scan report 里的可支持源码文件。
pub fn parse_symbols(
    repo_root: &Path,
    scan_report: &ScanReport,
) -> io::Result<ParsedSymbolsSnapshot> {
    let mut on_progress = |_processed: usize, _total: usize| {};
    parse_symbols_with_progress(repo_root, scan_report, &mut on_progress)
}

/// 只对指定文件路径做符号解析；空数组表示全量。
pub fn parse_symbols_for_paths(
    repo_root: &Path,
    scan_report: &ScanReport,
    target_paths: &[String],
) -> io::Result<ParsedSymbolsSnapshot> {
    let mut on_progress = |_processed: usize, _total: usize| {};
    parse_symbols_for_paths_with_progress(repo_root, scan_report, target_paths, &mut on_progress)
}

pub fn parse_symbols_with_progress<F>(
    repo_root: &Path,
    scan_report: &ScanReport,
    on_progress: &mut F,
) -> io::Result<ParsedSymbolsSnapshot>
where
    F: FnMut(usize, usize),
{
    parse_symbols_for_paths_with_progress(repo_root, scan_report, &[], on_progress)
}

pub fn parse_symbols_for_paths_with_progress<F>(
    repo_root: &Path,
    scan_report: &ScanReport,
    target_paths: &[String],
    on_progress: &mut F,
) -> io::Result<ParsedSymbolsSnapshot>
where
    F: FnMut(usize, usize),
{
    let candidates = collect_parse_candidates(scan_report, target_paths);
    let mut snapshot = ParsedSymbolsSnapshot::default();
    let total = candidates.len();
    let mut processed = 0usize;

    for chunk in chunk_parse_candidates(&candidates) {
        for parsed in parse_file_chunk(repo_root, &chunk) {
            processed += 1;
            snapshot
                .diagnostics
                .extend(parsed.diagnostics.iter().cloned());
            snapshot.symbols.extend(parsed.symbols.iter().cloned());
            snapshot.files.insert(parsed.file_path.clone(), parsed);
            on_progress(processed, total);
        }
    }

    snapshot.symbols.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.start_line.cmp(&right.start_line))
            .then(left.end_line.cmp(&right.end_line))
            .then(left.symbol_id.cmp(&right.symbol_id))
    });
    snapshot
        .symbols
        .dedup_by(|left, right| left.symbol_id == right.symbol_id);
    snapshot.symbol_table = SymbolTable::from_symbols(&snapshot.symbols);
    Ok(snapshot)
}

pub fn symbol_parse_file_count(scan_report: &ScanReport, target_paths: &[String]) -> usize {
    collect_parse_candidates(scan_report, target_paths).len()
}

fn collect_parse_candidates<'a>(
    scan_report: &'a ScanReport,
    target_paths: &[String],
) -> Vec<&'a ScannedFile> {
    let target_set =
        (!target_paths.is_empty()).then(|| target_paths.iter().cloned().collect::<BTreeSet<_>>());
    let mut candidates = scan_report
        .files
        .iter()
        .filter(|file| file.kind == "source")
        .filter(|file| {
            target_set
                .as_ref()
                .map(|paths| paths.contains(&file.path))
                .unwrap_or(true)
        })
        .filter(|file| resolve_symbol_language(&file.path, &file.language).is_some())
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.path.cmp(&right.path));
    candidates
}

fn chunk_parse_candidates<'a>(candidates: &[&'a ScannedFile]) -> Vec<Vec<&'a ScannedFile>> {
    let mut chunks = Vec::new();
    let mut current = Vec::new();
    let mut chunk_bytes = 0usize;

    for file in candidates {
        if chunk_bytes > 0 && chunk_bytes + file.size > CHUNK_BYTE_BUDGET {
            chunks.push(current);
            current = Vec::new();
            chunk_bytes = 0;
        }

        chunk_bytes += file.size.min(CHUNK_BYTE_BUDGET);
        current.push(*file);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn parse_file_chunk(repo_root: &Path, files: &[&ScannedFile]) -> Vec<ParsedFileSymbols> {
    let worker_count = configured_worker_count(files.len());
    if worker_count <= 1 || files.len() <= 1 {
        let mut context = ParseWorkerContext::default();
        return files
            .iter()
            .map(|file| parse_file_symbols(repo_root, file, &mut context))
            .collect();
    }

    let next_index = AtomicUsize::new(0);
    let results = Mutex::new(Vec::<(usize, ParsedFileSymbols)>::with_capacity(
        files.len(),
    ));

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                let mut context = ParseWorkerContext::default();
                loop {
                    let index = next_index.fetch_add(1, Ordering::Relaxed);
                    if index >= files.len() {
                        break;
                    }

                    let parsed = parse_file_symbols(repo_root, files[index], &mut context);
                    results.lock().unwrap().push((index, parsed));
                }
            });
        }
    });

    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, parsed)| parsed).collect()
}

fn configured_worker_count(file_count: usize) -> usize {
    let configured = std::env::var("WIKI_SYMBOL_PARSE_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0);
    let default = thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .min(4);

    configured.unwrap_or(default).clamp(1, file_count.max(1))
}

fn parse_file_symbols(
    repo_root: &Path,
    file: &ScannedFile,
    parse_context: &mut ParseWorkerContext,
) -> ParsedFileSymbols {
    let mut parsed = ParsedFileSymbols {
        file_path: file.path.clone(),
        language: file.language.clone(),
        ..ParsedFileSymbols::default()
    };

    let Some(resolved_language) = resolve_symbol_language(&file.path, &file.language) else {
        return parsed;
    };
    parsed.language = resolved_language.effective_language.to_string();

    if file.size > MAX_FILE_BYTES {
        parsed.diagnostics.push(diagnostic(
            file,
            resolved_language.effective_language,
            "file_too_large",
            format!(
                "file size {} exceeds max parse budget {} bytes",
                file.size, MAX_FILE_BYTES
            ),
        ));
        return parsed;
    }

    let absolute_path = repo_root.join(&file.path);
    let source = match fs::read_to_string(&absolute_path) {
        Ok(source) => source,
        Err(error) => {
            parsed.diagnostics.push(diagnostic(
                file,
                resolved_language.effective_language,
                "read_failed",
                error.to_string(),
            ));
            return parsed;
        }
    };

    let units = build_parse_units(file, &source);
    if let Some(first_unit) = units.first() {
        parsed.language = first_unit.resolved_language.effective_language.to_string();
    }
    if units.is_empty() {
        parsed.diagnostics.push(diagnostic(
            file,
            resolved_language.effective_language,
            "wrapper_without_script",
            "wrapper source contains no parseable script block".to_string(),
        ));
        return parsed;
    }

    let mut artifacts = Vec::new();
    for unit in &units {
        match build_unit_artifacts(file, unit, parse_context) {
            Ok(artifact) => {
                let query = parse_context
                    .query_for(artifact.resolved_language)
                    .expect("query should exist after artifact build");
                parsed.symbols.extend(collect_definition_symbols(
                    &file.path,
                    artifact.resolved_language.effective_language,
                    &artifact.source_bytes,
                    &artifact.tree,
                    query,
                    artifact.line_offset,
                ));
                artifacts.push(artifact);
            }
            Err(diagnostic) => parsed.diagnostics.push(
                diagnostic
                    .with_file(&file.path)
                    .with_message_prefix(&format!("{}: ", unit.source_label)),
            ),
        }
    }

    parsed.symbols.sort_by(|left, right| {
        left.start_line
            .cmp(&right.start_line)
            .then(left.end_line.cmp(&right.end_line))
            .then(left.symbol_id.cmp(&right.symbol_id))
    });
    parsed
        .symbols
        .dedup_by(|left, right| left.symbol_id == right.symbol_id);

    let captured_symbols = parsed.symbols.clone();
    for artifact in &artifacts {
        let query = parse_context
            .query_for(artifact.resolved_language)
            .expect("query should exist after artifact build");
        let (imports, calls, heritage) = collect_raw_captures(
            &file.path,
            artifact.resolved_language.effective_language,
            &artifact.source_bytes,
            &artifact.tree,
            query,
            artifact.line_offset,
            &captured_symbols,
        );
        parsed.imports.extend(imports);
        parsed.calls.extend(calls);
        parsed.heritage.extend(heritage);
    }

    parsed.imports.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.line.cmp(&right.line))
            .then(left.raw_path.cmp(&right.raw_path))
            .then(left.source_symbol_id.cmp(&right.source_symbol_id))
    });
    parsed.imports.dedup();

    parsed.calls.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.line.cmp(&right.line))
            .then(left.called_name.cmp(&right.called_name))
            .then(left.source_symbol_id.cmp(&right.source_symbol_id))
    });
    parsed.calls.dedup();

    parsed.heritage.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.line.cmp(&right.line))
            .then(left.owner_name.cmp(&right.owner_name))
            .then(left.target_name.cmp(&right.target_name))
            .then(left.relation_kind.cmp(&right.relation_kind))
    });
    parsed.heritage.dedup();

    parsed
}

fn build_parse_units(file: &ScannedFile, source: &str) -> Vec<ParseUnit> {
    match file.language.as_str() {
        "vue" | "svelte" => build_wrapper_parse_units(source),
        _ => resolve_symbol_language(&file.path, &file.language)
            .map(|resolved_language| {
                vec![ParseUnit {
                    content: source.to_string(),
                    resolved_language,
                    line_offset: 0,
                    source_label: file.language.clone(),
                }]
            })
            .unwrap_or_default(),
    }
}

fn build_wrapper_parse_units(source: &str) -> Vec<ParseUnit> {
    let script_regex = wrapper_script_regex();
    let lang_regex = wrapper_lang_regex();

    script_regex
        .captures_iter(source)
        .filter_map(|captures| {
            let attrs = captures
                .name("attrs")
                .map(|match_| match_.as_str())
                .unwrap_or_default();
            let body = captures.name("body")?;
            let line_offset = source[..body.start()]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count();
            let requested_language = lang_regex
                .captures(attrs)
                .and_then(|match_| {
                    match_
                        .name("lang")
                        .map(|lang| lang.as_str().to_ascii_lowercase())
                })
                .unwrap_or_else(|| "javascript".to_string());
            let effective_language = match requested_language.as_str() {
                "ts" | "typescript" | "tsx" => "typescript",
                "jsx" | "js" | "javascript" => "javascript",
                _ => "javascript",
            };
            let resolved_language = resolve_embedded_language(effective_language)?;
            Some(ParseUnit {
                content: body.as_str().to_string(),
                resolved_language,
                line_offset,
                source_label: requested_language,
            })
        })
        .collect()
}

fn build_unit_artifacts(
    file: &ScannedFile,
    unit: &ParseUnit,
    parse_context: &mut ParseWorkerContext,
) -> Result<ParsedUnitArtifacts, SymbolParseDiagnostic> {
    let tree = match parse_tree(&unit.resolved_language, &unit.content, parse_context) {
        Ok(tree) => tree,
        Err(diagnostic) => return Err(diagnostic),
    };

    if tree.root_node().has_error() {
        return Err(diagnostic(
            file,
            unit.resolved_language.effective_language,
            "parse_error",
            format!(
                "{}: tree-sitter reported syntax errors; parse unit skipped",
                unit.source_label
            ),
        ));
    }

    if let Err(error) = parse_context.query_for(unit.resolved_language) {
        return Err(diagnostic(
            file,
            unit.resolved_language.effective_language,
            "query_error",
            format!("{}: {error}", unit.source_label),
        ));
    }

    Ok(ParsedUnitArtifacts {
        resolved_language: unit.resolved_language,
        line_offset: unit.line_offset,
        source_bytes: unit.content.as_bytes().to_vec(),
        tree,
    })
}

fn parse_tree(
    resolved_language: &ResolvedSymbolLanguage,
    source: &str,
    parse_context: &mut ParseWorkerContext,
) -> Result<Tree, SymbolParseDiagnostic> {
    parse_context
        .parser_for(*resolved_language)
        .map_err(|error| SymbolParseDiagnostic {
            file_path: String::new(),
            language: resolved_language.effective_language.to_string(),
            kind: "set_language_failed".to_string(),
            message: error,
        })?;
    parse_context
        .parser_for(*resolved_language)
        .expect("parser should exist after cache lookup")
        .parse(source, None)
        .ok_or_else(|| SymbolParseDiagnostic {
            file_path: String::new(),
            language: resolved_language.effective_language.to_string(),
            kind: "parse_failed".to_string(),
            message: "tree-sitter returned no syntax tree".to_string(),
        })
}

impl ParseWorkerContext {
    fn parser_for(
        &mut self,
        resolved_language: ResolvedSymbolLanguage,
    ) -> Result<&mut Parser, String> {
        let cache_key = resolved_language.cache_key();
        if !self.parser_cache.contains_key(&cache_key) {
            let mut parser = Parser::new();
            let language = resolved_language.language();
            parser
                .set_language(&language)
                .map_err(|error| error.to_string())?;
            self.parser_cache.insert(cache_key, parser);
        }

        self.parser_cache
            .get_mut(&cache_key)
            .ok_or_else(|| "parser cache lookup failed".to_string())
    }

    fn query_for(&mut self, resolved_language: ResolvedSymbolLanguage) -> Result<&Query, String> {
        let cache_key = resolved_language.cache_key();
        if !self.query_cache.contains_key(&cache_key) {
            let language = resolved_language.language();
            let query = Query::new(&language, resolved_language.query_source)
                .map_err(|error| error.to_string())?;
            self.query_cache.insert(cache_key, query);
        }

        self.query_cache
            .get(&cache_key)
            .ok_or_else(|| "query cache lookup failed".to_string())
    }
}

fn wrapper_script_regex() -> &'static Regex {
    static SCRIPT_REGEX: OnceLock<Regex> = OnceLock::new();
    SCRIPT_REGEX.get_or_init(|| {
        Regex::new(r#"(?is)<script(?P<attrs>[^>]*)>(?P<body>.*?)</script>"#).unwrap()
    })
}

fn wrapper_lang_regex() -> &'static Regex {
    static LANG_REGEX: OnceLock<Regex> = OnceLock::new();
    LANG_REGEX.get_or_init(|| Regex::new(r#"lang\s*=\s*["'](?P<lang>[^"']+)["']"#).unwrap())
}

fn collect_definition_symbols(
    file_path: &str,
    language: &str,
    source: &[u8],
    tree: &Tree,
    query: &Query,
    line_offset: usize,
) -> Vec<SymbolNode> {
    let mut symbols = Vec::new();
    let mut seen_symbol_ids = BTreeSet::new();
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source);
    matches.advance();

    while let Some(query_match) = matches.get() {
        if let Some(symbol) = build_symbol_from_match(
            file_path,
            language,
            source,
            capture_names,
            query_match,
            line_offset,
        ) {
            if seen_symbol_ids.insert(symbol.symbol_id.clone()) {
                symbols.push(symbol);
            }
        }
        matches.advance();
    }

    symbols
}

fn collect_raw_captures(
    file_path: &str,
    language: &str,
    source: &[u8],
    tree: &Tree,
    query: &Query,
    line_offset: usize,
    symbols: &[SymbolNode],
) -> (
    Vec<RawImportCapture>,
    Vec<RawCallCapture>,
    Vec<RawHeritageCapture>,
) {
    let mut imports = Vec::new();
    let mut calls = Vec::new();
    let mut heritage = Vec::new();

    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source);
    matches.advance();

    while let Some(query_match) = matches.get() {
        let mut import_node = None;
        let mut import_sources = Vec::new();
        let mut call_node = None;
        let mut call_name = None;
        let mut call_receiver = None;
        let mut heritage_node = None;
        let mut heritage_kind = None;
        let mut heritage_owner = None;
        let mut heritage_targets = Vec::new();

        for capture in query_match.captures {
            let Some(capture_name) = capture_names.get(capture.index as usize).copied() else {
                continue;
            };

            match capture_name {
                "import" => import_node = Some(capture.node),
                "import.source" => import_sources.push(capture.node),
                "call" => call_node = Some(capture.node),
                "call.name" => call_name = Some(capture.node),
                "call.receiver" => call_receiver = Some(capture.node),
                "heritage.extends" => {
                    heritage_node = Some(capture.node);
                    heritage_kind = Some("extends");
                }
                "heritage.implements" => {
                    heritage_node = Some(capture.node);
                    heritage_kind = Some("implements");
                }
                "heritage.owner" => heritage_owner = Some(capture.node),
                "heritage.target" => heritage_targets.push(capture.node),
                _ => {}
            }
        }

        if let Some(import_node) = import_node {
            let line = import_node.start_position().row + line_offset + 1;
            let source_symbol_id = find_enclosing_symbol_id(symbols, line);
            let source_text = import_node
                .utf8_text(source)
                .ok()
                .map(normalize_symbol_name)
                .unwrap_or_default();

            for import_source in import_sources {
                let raw_path = import_source
                    .utf8_text(source)
                    .ok()
                    .map(clean_import_text)
                    .unwrap_or_default();
                if raw_path.is_empty() {
                    continue;
                }
                imports.push(RawImportCapture {
                    file_path: file_path.to_string(),
                    raw_path,
                    line,
                    language: language.to_string(),
                    source_symbol_id: source_symbol_id.clone(),
                    source_text: source_text.clone(),
                });
            }
        }

        if let (Some(call_node), Some(call_name)) = (call_node, call_name) {
            let called_name = call_name
                .utf8_text(source)
                .ok()
                .map(normalize_symbol_name)
                .unwrap_or_default();
            if !called_name.is_empty() {
                let line = call_node.start_position().row + line_offset + 1;
                calls.push(RawCallCapture {
                    file_path: file_path.to_string(),
                    called_name,
                    line,
                    language: language.to_string(),
                    source_symbol_id: find_enclosing_symbol_id(symbols, line),
                    receiver_text: call_receiver
                        .and_then(|node| node.utf8_text(source).ok())
                        .map(normalize_symbol_name)
                        .filter(|text| !text.is_empty()),
                    source_text: call_node
                        .utf8_text(source)
                        .ok()
                        .map(normalize_symbol_name)
                        .unwrap_or_default(),
                });
            }
        }

        if let (Some(heritage_node), Some(heritage_kind), Some(heritage_owner)) =
            (heritage_node, heritage_kind, heritage_owner)
        {
            let line = heritage_node.start_position().row + line_offset + 1;
            let owner_name = heritage_owner
                .utf8_text(source)
                .ok()
                .map(normalize_symbol_name)
                .unwrap_or_default();
            let owner_symbol_id = find_symbol_id_by_name_and_line(symbols, &owner_name, line)
                .or_else(|| find_enclosing_symbol_id(symbols, line));
            let source_text = heritage_node
                .utf8_text(source)
                .ok()
                .map(normalize_symbol_name)
                .unwrap_or_default();

            for target_node in heritage_targets {
                let target_name = target_node
                    .utf8_text(source)
                    .ok()
                    .map(normalize_symbol_name)
                    .unwrap_or_default();
                if target_name.is_empty() {
                    continue;
                }
                heritage.push(RawHeritageCapture {
                    file_path: file_path.to_string(),
                    line,
                    language: language.to_string(),
                    owner_name: owner_name.clone(),
                    owner_symbol_id: owner_symbol_id.clone(),
                    target_name,
                    relation_kind: heritage_kind.to_string(),
                    source_text: source_text.clone(),
                });
            }
        }

        matches.advance();
    }

    (imports, calls, heritage)
}

fn build_symbol_from_match(
    file_path: &str,
    language: &str,
    source: &[u8],
    capture_names: &[&str],
    query_match: &tree_sitter::QueryMatch<'_, '_>,
    line_offset: usize,
) -> Option<SymbolNode> {
    let mut definition_node = None;
    let mut label = None;
    let mut name_node = None;

    for capture in query_match.captures {
        let capture_name = capture_names.get(capture.index as usize).copied()?;
        if let Some(definition_label) = capture_name.strip_prefix("definition.") {
            definition_node = Some(capture.node);
            label = Some(definition_label);
        } else if capture_name == "name" {
            name_node = Some(capture.node);
        }
    }

    let definition_node = definition_node?;
    let label = label?;
    let name = name_node
        .and_then(|node| node.utf8_text(source).ok())
        .map(normalize_symbol_name)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| fallback_symbol_name(label, definition_node, source));
    if name.is_empty() {
        return None;
    }

    let start_line = definition_node.start_position().row + line_offset + 1;
    let end_line = definition_node.end_position().row + line_offset + 1;
    let seed = format!("{file_path}:{label}:{name}:{start_line}");
    let symbol_id = stable_id("symbol", seed);

    Some(SymbolNode {
        symbol_id,
        name: name.clone(),
        label: label.to_string(),
        file_path: file_path.to_string(),
        start_line,
        end_line,
        is_exported: detect_exported(language, &name, definition_node, source),
        language: language.to_string(),
    })
}

fn normalize_symbol_name(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clean_import_text(text: &str) -> String {
    text.trim()
        .trim_matches(['"', '\'', ';', '<', '>'])
        .replace("::", "/")
}

fn fallback_symbol_name(label: &str, node: Node<'_>, source: &[u8]) -> String {
    if label == "constructor" {
        return "constructor".to_string();
    }

    node.utf8_text(source)
        .ok()
        .map(|text| text.lines().next().unwrap_or_default().trim().to_string())
        .unwrap_or_default()
}

fn find_enclosing_symbol_id(symbols: &[SymbolNode], line: usize) -> Option<String> {
    symbols
        .iter()
        .filter(|symbol| symbol.start_line <= line && line <= symbol.end_line)
        .min_by_key(|symbol| (symbol.end_line - symbol.start_line, symbol.start_line))
        .map(|symbol| symbol.symbol_id.clone())
}

fn find_symbol_id_by_name_and_line(
    symbols: &[SymbolNode],
    name: &str,
    line: usize,
) -> Option<String> {
    symbols
        .iter()
        .find(|symbol| symbol.name == name && symbol.start_line <= line && line <= symbol.end_line)
        .map(|symbol| symbol.symbol_id.clone())
        .or_else(|| {
            symbols
                .iter()
                .find(|symbol| symbol.name == name)
                .map(|symbol| symbol.symbol_id.clone())
        })
}

fn detect_exported(language: &str, name: &str, node: Node<'_>, source: &[u8]) -> bool {
    match language {
        "javascript" | "typescript" => {
            ancestor_has_kind(node, "export_statement")
                || node_text_contains_any(node, source, &["export "])
        }
        "python" => !name.starts_with('_'),
        "go" => name
            .chars()
            .next()
            .map(|first| first.is_uppercase())
            .unwrap_or(false),
        "rust" => node_text_contains_any(node, source, &["pub ", "pub("]),
        "java" | "csharp" | "php" => node_text_contains_any(node, source, &["public "]),
        "kotlin" => node_text_contains_any(node, source, &["public ", "public\n"]),
        "swift" => node_text_contains_any(node, source, &["public ", "open "]),
        _ => false,
    }
}

fn ancestor_has_kind(node: Node<'_>, kind: &str) -> bool {
    let mut current = Some(node);
    while let Some(candidate) = current {
        if candidate.kind() == kind {
            return true;
        }
        current = candidate.parent();
    }
    false
}

fn node_text_contains_any(node: Node<'_>, source: &[u8], needles: &[&str]) -> bool {
    let mut current = Some(node);
    while let Some(candidate) = current {
        if let Ok(text) = candidate.utf8_text(source) {
            let text = text.trim_start();
            if needles.iter().any(|needle| text.contains(needle)) {
                return true;
            }
        }
        current = candidate.parent();
    }
    false
}

fn diagnostic(
    file: &ScannedFile,
    language: &str,
    kind: &str,
    message: String,
) -> SymbolParseDiagnostic {
    SymbolParseDiagnostic {
        file_path: file.path.clone(),
        language: language.to_string(),
        kind: kind.to_string(),
        message,
    }
}

trait DiagnosticWithFile {
    fn with_file(self, file_path: &str) -> Self;
    fn with_message_prefix(self, prefix: &str) -> Self;
}

impl DiagnosticWithFile for SymbolParseDiagnostic {
    fn with_file(mut self, file_path: &str) -> Self {
        self.file_path = file_path.to_string();
        self
    }

    fn with_message_prefix(mut self, prefix: &str) -> Self {
        self.message = format!("{prefix}{}", self.message);
        self
    }
}


