use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::Path;

use crate::scanner::ScanReport;
use crate::symbols::{ParsedSymbolsSnapshot, SymbolNode};

use super::analyze::{detect_communities, detect_cycles, detect_processes};
use super::models::{
    GraphAnalysisSnapshot, GraphSummary, ProcessNode, ProcessStep, ResolvedGraphSnapshot,
    ResolvedSymbolEdge, SymbolGraphSnapshot,
};
use super::resolve::{
    build_import_resolution_context, resolve_calls, resolve_heritage, resolve_imports,
};

/// 把 parser、resolve 和 analyze 三阶段结果装配成统一 graph 快照。
///
/// # 参数
/// - `parsed_symbols`：parser 阶段产出的 definitions 与 raw captures。
/// - `resolved_graph`：resolve 阶段产出的稳定 edges。
/// - `analysis`：基于 symbol graph 的派生视图。
/// - `summary`：供 hierarchy/context/planner 直接消费的聚合摘要。
///
/// # 返回
/// - 返回面向 workflow 编排的统一 symbol graph 快照。
pub fn assemble_symbol_graph_snapshot(
    parsed_symbols: ParsedSymbolsSnapshot,
    resolved_graph: ResolvedGraphSnapshot,
    analysis: GraphAnalysisSnapshot,
    summary: GraphSummary,
) -> SymbolGraphSnapshot {
    SymbolGraphSnapshot {
        parsed_symbols,
        resolved_graph,
        analysis,
        summary,
    }
}

/// 执行 iter 8 当前已具备的 symbol resolution 主链。
///
/// # 参数
/// - `repo_root`：仓库根目录。
/// - `scan_report`：当前轮扫描结果。
/// - `parsed_symbols`：parser 阶段产出的 definitions 与 raw captures。
///
/// # 返回
/// - 返回合并后的 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges。
///
/// # 错误
/// - 当构建 import resolution 上下文需要读取配置文件且发生 I/O 错误时返回错误。
pub fn resolve_symbol_graph(
    repo_root: &Path,
    scan_report: &ScanReport,
    parsed_symbols: &ParsedSymbolsSnapshot,
) -> io::Result<ResolvedGraphSnapshot> {
    let import_context = build_import_resolution_context(repo_root, scan_report)?;
    let imports = resolve_imports(parsed_symbols, &import_context);
    let calls = resolve_calls(parsed_symbols, &imports);
    let heritage = resolve_heritage(parsed_symbols, &imports);

    Ok(merge_resolved_graphs([imports, calls, heritage]))
}

/// 基于 resolved graph 执行 community / process / cycle 分析。
pub fn analyze_symbol_graph(
    parsed_symbols: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> GraphAnalysisSnapshot {
    let (communities, community_members) = detect_communities(parsed_symbols, resolved_graph);
    let (processes, process_steps) = detect_processes(parsed_symbols, resolved_graph);
    let cycles = detect_cycles(parsed_symbols, resolved_graph);

    GraphAnalysisSnapshot {
        communities,
        community_members,
        processes,
        process_steps,
        cycles,
        diagnostics: Vec::new(),
    }
}

/// 把 symbol graph 与 analysis 结果压缩成 hierarchy/context/planner 可消费的摘要。
pub fn build_graph_summary(
    scan_report: &ScanReport,
    parsed_symbols: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> GraphSummary {
    let symbols_by_id = parsed_symbols
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let mut module_dependency_hints = BTreeMap::<String, BTreeSet<String>>::new();
    let mut module_call_hotspots = BTreeMap::<String, BTreeSet<String>>::new();
    let mut communities_by_module = BTreeMap::<String, BTreeSet<String>>::new();

    for edge in &resolved_graph.edges {
        let Some(source_symbol) = symbols_by_id.get(&edge.source_id).copied() else {
            continue;
        };
        let Some(target_symbol) = symbols_by_id.get(&edge.target_id).copied() else {
            continue;
        };
        let source_root = summary_root_for_path(scan_report, &source_symbol.file_path);
        let target_root = summary_root_for_path(scan_report, &target_symbol.file_path);

        if source_root != target_root
            && matches!(edge.edge_type.as_str(), "IMPORTS" | "CALLS")
            && edge.confidence >= 0.55
        {
            module_dependency_hints
                .entry(source_root.clone())
                .or_default()
                .insert(target_root.clone());
        }

        if edge.edge_type == "CALLS" && edge.confidence >= 0.75 {
            module_call_hotspots
                .entry(source_root)
                .or_default()
                .insert(format!("{} -> {}", source_symbol.name, target_symbol.name));
        }
    }

    let community_labels = analysis
        .communities
        .iter()
        .map(|community| (community.community_id.clone(), community.label.clone()))
        .collect::<BTreeMap<_, _>>();
    let community_members = analysis
        .community_members
        .iter()
        .map(|member| (member.symbol_id.clone(), member.community_id.clone()))
        .collect::<BTreeMap<_, _>>();
    for symbol in &parsed_symbols.symbols {
        if let Some(community_label) = community_members
            .get(&symbol.symbol_id)
            .and_then(|community_id| community_labels.get(community_id))
        {
            communities_by_module
                .entry(summary_root_for_path(scan_report, &symbol.file_path))
                .or_default()
                .insert(community_label.clone());
        }
    }

    GraphSummary {
        module_dependency_hints: to_sorted_map(module_dependency_hints),
        module_call_hotspots: to_sorted_map(module_call_hotspots),
        communities_by_module: to_sorted_map(communities_by_module),
        detected_processes: summarize_processes(
            &analysis.processes,
            &analysis.process_steps,
            &symbols_by_id,
        ),
        cycle_warnings: analysis
            .cycles
            .iter()
            .map(|cycle| cycle.warning.clone())
            .collect(),
    }
}

fn merge_resolved_graphs<const N: usize>(
    graphs: [ResolvedGraphSnapshot; N],
) -> ResolvedGraphSnapshot {
    let mut edges = Vec::<ResolvedSymbolEdge>::new();
    let mut diagnostics = Vec::new();
    let mut seen_edge_ids = BTreeSet::new();

    for graph in graphs {
        diagnostics.extend(graph.diagnostics);
        for edge in graph.edges {
            if seen_edge_ids.insert(edge.edge_id.clone()) {
                edges.push(edge);
            }
        }
    }

    ResolvedGraphSnapshot { edges, diagnostics }
}

fn summary_root_for_path(scan_report: &ScanReport, file_path: &str) -> String {
    let mut candidate = scan_report
        .workspace_roots
        .iter()
        .filter(|root| *root != "." && path_matches_root(file_path, root))
        .max_by_key(|root| root.len())
        .cloned();

    if candidate.is_none() {
        let segments = file_path.split('/').collect::<Vec<_>>();
        if segments.len() >= 2
            && matches!(
                segments[0],
                "crates" | "agents" | "apps" | "services" | "libs" | "packages"
            )
        {
            candidate = Some(format!("{}/{}", segments[0], segments[1]));
        }
    }

    candidate.unwrap_or_else(|| ".".to_string())
}

fn path_matches_root(path: &str, root: &str) -> bool {
    path == root || path.starts_with(&format!("{root}/"))
}

fn to_sorted_map(input: BTreeMap<String, BTreeSet<String>>) -> BTreeMap<String, Vec<String>> {
    input
        .into_iter()
        .map(|(key, values)| (key, values.into_iter().collect()))
        .collect()
}

fn summarize_processes(
    processes: &[ProcessNode],
    steps: &[ProcessStep],
    symbols_by_id: &BTreeMap<String, &SymbolNode>,
) -> Vec<String> {
    processes
        .iter()
        .map(|process| {
            let trace = steps
                .iter()
                .filter(|step| step.process_id == process.process_id)
                .filter_map(|step| symbols_by_id.get(&step.symbol_id).copied())
                .map(|symbol| symbol.name.clone())
                .collect::<Vec<_>>();
            if trace.is_empty() {
                process.label.clone()
            } else {
                format!("{}: {}", process.label, trace.join(" -> "))
            }
        })
        .collect()
}
