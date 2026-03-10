use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use tree_sitter::{Node, Parser, Query, QueryCursor, StreamingIterator, Tree};

use crate::domain::stable_id::stable_id;
use crate::repo::scanner::{ScanReport, ScannedFile};

use super::models::{
    ParsedFileSymbols, ParsedSymbolsSnapshot, SymbolNode, SymbolParseDiagnostic, SymbolTable,
};
use super::registry::{resolve_symbol_language, ResolvedSymbolLanguage};

/// 单批解析预算。当前只用它约束顺序处理的分块边界。
pub const CHUNK_BYTE_BUDGET: usize = 20 * 1024 * 1024;
/// 超过这个大小的单文件先直接跳过，避免解析器被极端文件拖慢。
pub const MAX_FILE_BYTES: usize = 512 * 1024;

/// 全量解析 scan report 里的可支持源码文件。
pub fn parse_symbols(repo_root: &Path, scan_report: &ScanReport) -> io::Result<ParsedSymbolsSnapshot> {
    parse_symbols_for_paths(repo_root, scan_report, &[])
}

/// 只对指定文件路径做符号解析；空数组表示全量。
pub fn parse_symbols_for_paths(
    repo_root: &Path,
    scan_report: &ScanReport,
    target_paths: &[String],
) -> io::Result<ParsedSymbolsSnapshot> {
    let target_set = (!target_paths.is_empty()).then(|| {
        target_paths
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
    });
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

    let mut snapshot = ParsedSymbolsSnapshot::default();
    let mut chunk_bytes = 0usize;

    for file in candidates {
        if chunk_bytes > 0 && chunk_bytes + file.size > CHUNK_BYTE_BUDGET {
            chunk_bytes = 0;
        }
        chunk_bytes += file.size.min(CHUNK_BYTE_BUDGET);

        let parsed = parse_file_symbols(repo_root, file);
        snapshot.diagnostics.extend(parsed.diagnostics.iter().cloned());
        snapshot.symbols.extend(parsed.symbols.iter().cloned());
        snapshot.files.insert(file.path.clone(), parsed);
    }

    snapshot.symbols.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.start_line.cmp(&right.start_line))
            .then(left.symbol_id.cmp(&right.symbol_id))
    });
    snapshot
        .symbols
        .dedup_by(|left, right| left.symbol_id == right.symbol_id);
    snapshot.symbol_table = SymbolTable::from_symbols(&snapshot.symbols);
    Ok(snapshot)
}

fn parse_file_symbols(repo_root: &Path, file: &ScannedFile) -> ParsedFileSymbols {
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

    let tree = match parse_tree(&resolved_language, &source) {
        Ok(tree) => tree,
        Err(diagnostic) => {
            parsed.diagnostics.push(diagnostic.with_file(&file.path));
            return parsed;
        }
    };

    if tree.root_node().has_error() {
        parsed.diagnostics.push(diagnostic(
            file,
            resolved_language.effective_language,
            "parse_error",
            "tree-sitter reported syntax errors; file skipped".to_string(),
        ));
        return parsed;
    }

    let language = resolved_language.language();
    let query = match Query::new(&language, resolved_language.query_source) {
        Ok(query) => query,
        Err(error) => {
            parsed.diagnostics.push(diagnostic(
                file,
                resolved_language.effective_language,
                "query_error",
                error.to_string(),
            ));
            return parsed;
        }
    };

    parsed.symbols = collect_definition_symbols(
        &file.path,
        resolved_language.effective_language,
        source.as_bytes(),
        &tree,
        &query,
    );
    parsed
}

fn parse_tree(
    resolved_language: &ResolvedSymbolLanguage,
    source: &str,
) -> Result<Tree, SymbolParseDiagnostic> {
    let mut parser = Parser::new();
    let language = resolved_language.language();
    parser
        .set_language(&language)
        .map_err(|error| SymbolParseDiagnostic {
            file_path: String::new(),
            language: resolved_language.effective_language.to_string(),
            kind: "set_language_failed".to_string(),
            message: error.to_string(),
        })?;
    parser
        .parse(source, None)
        .ok_or_else(|| SymbolParseDiagnostic {
            file_path: String::new(),
            language: resolved_language.effective_language.to_string(),
            kind: "parse_failed".to_string(),
            message: "tree-sitter returned no syntax tree".to_string(),
        })
}

fn collect_definition_symbols(
    file_path: &str,
    language: &str,
    source: &[u8],
    tree: &Tree,
    query: &Query,
) -> Vec<SymbolNode> {
    let mut symbols = Vec::new();
    let mut seen_symbol_ids = BTreeSet::new();
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source);
    matches.advance();

    while let Some(query_match) = matches.get() {
        if let Some(symbol) =
            build_symbol_from_match(file_path, language, source, capture_names, query_match)
        {
            if seen_symbol_ids.insert(symbol.symbol_id.clone()) {
                symbols.push(symbol);
            }
        }
        matches.advance();
    }

    symbols
}

fn build_symbol_from_match(
    file_path: &str,
    language: &str,
    source: &[u8],
    capture_names: &[&str],
    query_match: &tree_sitter::QueryMatch<'_, '_>,
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

    let start_line = definition_node.start_position().row + 1;
    let end_line = definition_node.end_position().row + 1;
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

fn fallback_symbol_name(label: &str, node: Node<'_>, source: &[u8]) -> String {
    if label == "constructor" {
        return "constructor".to_string();
    }

    node.utf8_text(source)
        .ok()
        .map(|text| text.lines().next().unwrap_or_default().trim().to_string())
        .unwrap_or_default()
}

fn detect_exported(language: &str, name: &str, node: Node<'_>, source: &[u8]) -> bool {
    match language {
        "javascript" | "typescript" => {
            ancestor_has_kind(node, "export_statement") || node_text_contains_any(node, source, &["export "])
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
}

impl DiagnosticWithFile for SymbolParseDiagnostic {
    fn with_file(mut self, file_path: &str) -> Self {
        self.file_path = file_path.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::repo::scanner::{FilePurpose, ScannedFile, ScanReport};

    use super::{parse_symbols, MAX_FILE_BYTES};

    #[test]
    fn parse_symbols_builds_stable_ids_for_duplicate_names() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join("src.ts"),
            "export function same() {}\nfunction same() {}\n",
        )
        .unwrap();
        let scan_report = ScanReport {
            root: temp.path().to_string_lossy().to_string(),
            files: vec![ScannedFile {
                id: "source-a".to_string(),
                path: "src.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                purpose: FilePurpose::Utility,
                fingerprint: "fingerprint".to_string(),
                size: 40,
                tags: Vec::new(),
            }],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };

        let snapshot = parse_symbols(temp.path(), &scan_report).unwrap();
        assert_eq!(
            snapshot.symbols.len(),
            2,
            "unexpected snapshot: {snapshot:#?}"
        );
        assert_ne!(snapshot.symbols[0].symbol_id, snapshot.symbols[1].symbol_id);
        assert!(snapshot.symbols.iter().any(|symbol| symbol.is_exported));
        assert!(snapshot
            .symbol_table
            .global_index
            .get("same")
            .is_some_and(|symbol_ids| symbol_ids.len() == 2));
    }

    #[test]
    fn parse_symbols_skips_oversized_file_with_diagnostic() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("huge.ts"), "export function huge() {}\n").unwrap();
        let scan_report = ScanReport {
            root: temp.path().to_string_lossy().to_string(),
            files: vec![ScannedFile {
                id: "source-huge".to_string(),
                path: "huge.ts".to_string(),
                language: "typescript".to_string(),
                kind: "source".to_string(),
                purpose: FilePurpose::Utility,
                fingerprint: "fingerprint".to_string(),
                size: MAX_FILE_BYTES + 1,
                tags: Vec::new(),
            }],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };

        let snapshot = parse_symbols(temp.path(), &scan_report).unwrap();
        assert!(snapshot.symbols.is_empty());
        assert!(snapshot
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.kind == "file_too_large"));
    }

    #[test]
    fn parse_symbols_isolates_parse_failures_per_file() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("good.ts"), "export function ok() {}\n").unwrap();
        std::fs::write(temp.path().join("broken.ts"), "export function broken( {\n").unwrap();
        let scan_report = ScanReport {
            root: temp.path().to_string_lossy().to_string(),
            files: vec![
                ScannedFile {
                    id: "source-bad".to_string(),
                    path: "broken.ts".to_string(),
                    language: "typescript".to_string(),
                    kind: "source".to_string(),
                    purpose: FilePurpose::Utility,
                    fingerprint: "fingerprint-bad".to_string(),
                    size: 28,
                    tags: Vec::new(),
                },
                ScannedFile {
                    id: "source-good".to_string(),
                    path: "good.ts".to_string(),
                    language: "typescript".to_string(),
                    kind: "source".to_string(),
                    purpose: FilePurpose::Utility,
                    fingerprint: "fingerprint-good".to_string(),
                    size: 24,
                    tags: Vec::new(),
                },
            ],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };

        let snapshot = parse_symbols(temp.path(), &scan_report).unwrap();
        assert!(snapshot.symbols.iter().any(|symbol| symbol.name == "ok"));
        assert!(snapshot
            .files
            .get("broken.ts")
            .is_some_and(|parsed| parsed.symbols.is_empty()));
        assert!(snapshot.diagnostics.iter().any(|diagnostic| {
            diagnostic.file_path == "broken.ts" && diagnostic.kind == "parse_error"
        }));
    }

    #[test]
    fn parse_symbols_extracts_kotlin_definitions() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join("App.kt"),
            [
                "interface Greeting",
                "class Greeter",
                "fun useTheme() = true",
                "val themeName = \"light\"",
                "typealias GreetingAlias = Greeting",
                "",
            ]
            .join("\n"),
        )
        .unwrap();
        let scan_report = ScanReport {
            root: temp.path().to_string_lossy().to_string(),
            files: vec![ScannedFile {
                id: "source-kotlin".to_string(),
                path: "App.kt".to_string(),
                language: "kotlin".to_string(),
                kind: "source".to_string(),
                purpose: FilePurpose::Utility,
                fingerprint: "fingerprint-kotlin".to_string(),
                size: 120,
                tags: Vec::new(),
            }],
            tech_hints: Vec::new(),
            workspace_roots: Vec::new(),
            config_files: Vec::new(),
            entry_points: Vec::new(),
            dependency_hints: Vec::new(),
        };

        let snapshot = parse_symbols(temp.path(), &scan_report).unwrap();
        let names = snapshot
            .symbols
            .iter()
            .map(|symbol| (symbol.name.as_str(), symbol.label.as_str()))
            .collect::<Vec<_>>();

        assert!(names.contains(&("Greeting", "interface")));
        assert!(names.contains(&("Greeter", "class")));
        assert!(names.contains(&("useTheme", "function")));
        assert!(names.contains(&("themeName", "property")));
        assert!(names.contains(&("GreetingAlias", "type")));
    }
}
