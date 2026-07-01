use std::collections::{BTreeMap, BTreeSet};

use crate::symbol_graph::{GraphDiagnostic, ResolvedGraphSnapshot, ResolvedSymbolEdge};
use crate::symbols::{ParsedSymbolsSnapshot, RawHeritageCapture, SymbolNode};
use wiki_model::domain::stable_id::stable_id;

/// 基于 parsed heritage 和已解析 import edges 构建 `EXTENDS / IMPLEMENTS`。
///
/// # 参数
/// - `snapshot`：parser 阶段产出的符号与 raw heritage captures。
/// - `import_graph`：已完成的 `IMPORTS` edges，用于收窄跨文件目标。
///
/// # 返回
/// - 返回稳定的 heritage edges 与 unresolved diagnostics。
pub fn resolve_heritage(
    snapshot: &ParsedSymbolsSnapshot,
    import_graph: &ResolvedGraphSnapshot,
) -> ResolvedGraphSnapshot {
    let symbols_by_id = snapshot
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let imported_symbols = imported_symbols_by_file(snapshot, import_graph);

    let mut edges = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen = BTreeSet::new();

    for parsed_file in snapshot.files.values() {
        for capture in &parsed_file.heritage {
            let Some(owner_symbol) = capture_owner_symbol(capture, snapshot, &symbols_by_id) else {
                diagnostics.push(heritage_diagnostic(
                    capture,
                    "missing_owner_symbol",
                    "heritage capture cannot be attached to any owner symbol",
                ));
                continue;
            };

            let (targets, confidence, reason) =
                resolve_heritage_targets(capture, snapshot, &symbols_by_id, &imported_symbols);
            if targets.is_empty() {
                diagnostics.push(heritage_diagnostic(
                    capture,
                    "unresolved_heritage",
                    "heritage target was not resolved to any stable symbol",
                ));
                continue;
            }

            let edge_type = if capture.relation_kind == "implements" {
                "IMPLEMENTS"
            } else {
                "EXTENDS"
            };

            for target in targets {
                let edge_id = stable_id(
                    "edge",
                    format!(
                        "{edge_type}:{}:{}:{}:{}",
                        owner_symbol.symbol_id, target.symbol_id, capture.line, capture.target_name
                    ),
                );
                if seen.insert(edge_id.clone()) {
                    edges.push(ResolvedSymbolEdge {
                        edge_id,
                        source_id: owner_symbol.symbol_id.clone(),
                        target_id: target.symbol_id.clone(),
                        edge_type: edge_type.to_string(),
                        confidence,
                        reason: reason.clone(),
                    });
                }
            }
        }
    }

    ResolvedGraphSnapshot { edges, diagnostics }
}

fn resolve_heritage_targets<'a>(
    capture: &RawHeritageCapture,
    snapshot: &'a ParsedSymbolsSnapshot,
    symbols_by_id: &'a BTreeMap<String, &'a SymbolNode>,
    imported_symbols: &'a BTreeMap<String, Vec<&'a SymbolNode>>,
) -> (Vec<&'a SymbolNode>, f64, String) {
    let same_file = snapshot
        .symbol_table
        .lookup_exact(&capture.file_path, &capture.target_name)
        .into_iter()
        .filter_map(|symbol_id| symbols_by_id.get(&symbol_id).copied())
        .collect::<Vec<_>>();
    if !same_file.is_empty() {
        return (same_file, 0.95, "same-file".to_string());
    }

    let imported = imported_symbols
        .get(&capture.file_path)
        .into_iter()
        .flatten()
        .copied()
        .filter(|symbol| symbol.name == capture.target_name)
        .collect::<Vec<_>>();
    if !imported.is_empty() {
        return (imported, 0.9, "import-resolved".to_string());
    }

    let global = snapshot
        .symbol_table
        .lookup_global(&capture.target_name)
        .into_iter()
        .filter_map(|symbol_id| symbols_by_id.get(&symbol_id).copied())
        .collect::<Vec<_>>();
    if global.len() == 1 {
        return (global, 0.75, "fuzzy-global".to_string());
    }

    let exported_global = global
        .into_iter()
        .filter(|symbol| symbol.is_exported)
        .collect::<Vec<_>>();
    if exported_global.len() == 1 {
        return (exported_global, 0.8, "fuzzy-global-exported".to_string());
    }

    (Vec::new(), 0.0, String::new())
}

fn capture_owner_symbol<'a>(
    capture: &RawHeritageCapture,
    snapshot: &'a ParsedSymbolsSnapshot,
    symbols_by_id: &'a BTreeMap<String, &'a SymbolNode>,
) -> Option<&'a SymbolNode> {
    capture
        .owner_symbol_id
        .as_ref()
        .and_then(|symbol_id| symbols_by_id.get(symbol_id).copied())
        .or_else(|| {
            snapshot
                .symbol_table
                .lookup_exact(&capture.file_path, &capture.owner_name)
                .into_iter()
                .find_map(|symbol_id| symbols_by_id.get(&symbol_id).copied())
        })
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

fn heritage_diagnostic(capture: &RawHeritageCapture, kind: &str, message: &str) -> GraphDiagnostic {
    GraphDiagnostic {
        stage: "resolve_heritage".to_string(),
        kind: kind.to_string(),
        message: format!(
            "{}:{}:{} -> {}",
            capture.file_path, capture.line, capture.target_name, message
        ),
        file_path: Some(capture.file_path.clone()),
        symbol_id: capture.owner_symbol_id.clone(),
    }
}
