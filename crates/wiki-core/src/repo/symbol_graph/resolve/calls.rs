use std::collections::{BTreeMap, BTreeSet};

use crate::domain::stable_id::stable_id;
use crate::repo::symbol_graph::{GraphDiagnostic, ResolvedGraphSnapshot, ResolvedSymbolEdge};
use crate::repo::symbols::{ParsedSymbolsSnapshot, RawCallCapture, SymbolNode};

/// 基于 parsed calls 和已解析 import edges 构建稳定 `CALLS` edges。
///
/// # 参数
/// - `snapshot`：parser 阶段输出的符号与 raw calls。
/// - `import_graph`：已完成的 `IMPORTS` edges，用于收窄 imported candidates。
///
/// # 返回
/// - 返回带置信度和 reason 的 `CALLS` edges，以及 unresolved diagnostics。
pub fn resolve_calls(
    snapshot: &ParsedSymbolsSnapshot,
    import_graph: &ResolvedGraphSnapshot,
) -> ResolvedGraphSnapshot {
    let symbols_by_id = snapshot
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let symbols_by_file = build_symbols_by_file(snapshot);
    let imported_symbols = imported_symbols_by_file(snapshot, import_graph);

    let mut edges = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();

    for parsed_file in snapshot.files.values() {
        for capture in &parsed_file.calls {
            if is_builtin_or_noise(capture) {
                continue;
            }

            let Some(source_symbol) = capture_source_symbol(capture, &symbols_by_id, &symbols_by_file)
            else {
                diagnostics.push(call_diagnostic(
                    capture,
                    "missing_source_symbol",
                    "call capture cannot be attached to any source symbol",
                ));
                continue;
            };

            let (targets, confidence, reason) = resolve_call_targets(
                capture,
                snapshot,
                &symbols_by_id,
                &imported_symbols,
            );
            if targets.is_empty() {
                diagnostics.push(call_diagnostic(
                    capture,
                    "unresolved_call",
                    "call target was not resolved to any stable symbol",
                ));
                continue;
            }

            for target in targets {
                let edge_id = stable_id(
                    "edge",
                    format!(
                        "CALLS:{}:{}:{}:{}",
                        source_symbol.symbol_id, target.symbol_id, capture.line, capture.called_name
                    ),
                );
                if seen.insert(edge_id.clone()) {
                    edges.push(ResolvedSymbolEdge {
                        edge_id,
                        source_id: source_symbol.symbol_id.clone(),
                        target_id: target.symbol_id.clone(),
                        edge_type: "CALLS".to_string(),
                        confidence,
                        reason: reason.clone(),
                    });
                }
            }
        }
    }

    ResolvedGraphSnapshot { edges, diagnostics }
}

fn resolve_call_targets<'a>(
    capture: &RawCallCapture,
    snapshot: &'a ParsedSymbolsSnapshot,
    symbols_by_id: &'a BTreeMap<String, &'a SymbolNode>,
    imported_symbols: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
) -> (Vec<&'a SymbolNode>, f64, String) {
    let same_file = snapshot
        .symbol_table
        .lookup_exact(&capture.file_path, &capture.called_name)
        .into_iter()
        .filter_map(|symbol_id| symbols_by_id.get(&symbol_id).copied())
        .collect::<Vec<_>>();
    if !same_file.is_empty() {
        let confidence = match capture.receiver_text.as_deref() {
            Some("this" | "self" | "super") => 0.98,
            Some(_) => 0.96,
            None => 0.95,
        };
        return (same_file, confidence, "same-file".to_string());
    }

    let imported = imported_symbols
        .get(&capture.file_path)
        .into_iter()
        .flatten()
        .copied()
        .filter(|symbol| symbol.name == capture.called_name)
        .collect::<Vec<_>>();
    if !imported.is_empty() {
        return (imported, 0.85, "import-resolved".to_string());
    }

    let global = snapshot
        .symbol_table
        .lookup_global(&capture.called_name)
        .into_iter()
        .filter_map(|symbol_id| symbols_by_id.get(&symbol_id).copied())
        .collect::<Vec<_>>();
    if global.len() == 1 {
        return (global, 0.55, "fuzzy-global".to_string());
    }

    let exported_global = global
        .into_iter()
        .filter(|symbol| symbol.is_exported)
        .collect::<Vec<_>>();
    if exported_global.len() == 1 {
        return (exported_global, 0.6, "fuzzy-global-exported".to_string());
    }

    (Vec::new(), 0.0, String::new())
}

fn capture_source_symbol<'a>(
    capture: &RawCallCapture,
    symbols_by_id: &'a BTreeMap<String, &'a SymbolNode>,
    symbols_by_file: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
) -> Option<&'a SymbolNode> {
    capture
        .source_symbol_id
        .as_ref()
        .and_then(|symbol_id| symbols_by_id.get(symbol_id).copied())
        .or_else(|| file_owner_symbol(symbols_by_file, &capture.file_path))
}

fn imported_symbols_by_file<'a>(
    snapshot: &'a ParsedSymbolsSnapshot,
    import_graph: &ResolvedGraphSnapshot,
) -> BTreeMap<String, Vec<&'a SymbolNode>> {
    let symbols_by_id = snapshot
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let mut imported = BTreeMap::<String, Vec<&SymbolNode>>::new();

    for edge in &import_graph.edges {
        if edge.edge_type != "IMPORTS" {
            continue;
        }
        let Some(source_symbol) = symbols_by_id.get(&edge.source_id).copied() else {
            continue;
        };
        let Some(target_symbol) = symbols_by_id.get(&edge.target_id).copied() else {
            continue;
        };
        imported
            .entry(source_symbol.file_path.clone())
            .or_default()
            .push(target_symbol);
    }

    for symbols in imported.values_mut() {
        symbols.sort_by(|left, right| left.symbol_id.cmp(&right.symbol_id));
        symbols.dedup_by(|left, right| left.symbol_id == right.symbol_id);
    }

    imported
}

fn build_symbols_by_file(snapshot: &ParsedSymbolsSnapshot) -> BTreeMap<String, Vec<&SymbolNode>> {
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
    symbols_by_file
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

fn call_diagnostic(capture: &RawCallCapture, kind: &str, message: &str) -> GraphDiagnostic {
    GraphDiagnostic {
        stage: "resolve_calls".to_string(),
        kind: kind.to_string(),
        message: format!("{}:{} -> {}", capture.file_path, capture.line, message),
        file_path: Some(capture.file_path.clone()),
        symbol_id: capture.source_symbol_id.clone(),
    }
}

fn is_builtin_or_noise(capture: &RawCallCapture) -> bool {
    const BUILTIN_NAMES: &[&str] = &[
        "assert",
        "assert_eq",
        "clone",
        "debug",
        "eprintln",
        "expect",
        "filter",
        "for_each",
        "forEach",
        "len",
        "log",
        "map",
        "pop",
        "print",
        "println",
        "push",
        "reduce",
        "to_string",
        "unwrap",
    ];
    const NOISE_RECEIVERS: &[&str] = &["console", "logger", "log", "tracing"];

    BUILTIN_NAMES.iter().any(|name| *name == capture.called_name)
        || capture
            .receiver_text
            .as_deref()
            .map(|receiver| NOISE_RECEIVERS.iter().any(|candidate| *candidate == receiver))
            .unwrap_or(false)
}
