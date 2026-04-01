use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::symbol_graph::{ProcessNode, ProcessStep, ResolvedGraphSnapshot};
use crate::symbols::{ParsedSymbolsSnapshot, SymbolNode};
use wiki_model::domain::stable_id::stable_id;

/// 基于高置信度 `CALLS` 图检测执行流 processes。
pub fn detect_processes(
    parsed_symbols: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> (Vec<ProcessNode>, Vec<ProcessStep>) {
    let call_edges = resolved_graph
        .edges
        .iter()
        .filter(|edge| edge.edge_type == "CALLS" && edge.confidence >= 0.75)
        .collect::<Vec<_>>();
    let symbols_by_id = parsed_symbols
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = BTreeMap::<String, Vec<String>>::new();
    let mut indegree = BTreeMap::<String, usize>::new();

    for edge in &call_edges {
        outgoing
            .entry(edge.source_id.clone())
            .or_default()
            .push(edge.target_id.clone());
        *indegree.entry(edge.target_id.clone()).or_default() += 1;
        indegree.entry(edge.source_id.clone()).or_default();
    }

    for targets in outgoing.values_mut() {
        targets.sort();
        targets.dedup();
    }

    let mut entry_candidates = indegree
        .keys()
        .filter_map(|symbol_id| symbols_by_id.get(symbol_id).copied())
        .map(|symbol| {
            (
                entry_score(
                    symbol,
                    indegree.get(&symbol.symbol_id).copied().unwrap_or(0),
                    outgoing.get(&symbol.symbol_id).map(Vec::len).unwrap_or(0),
                ),
                symbol,
            )
        })
        .filter(|(score, _)| *score > 0)
        .collect::<Vec<_>>();
    entry_candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then(left.1.file_path.cmp(&right.1.file_path))
            .then(left.1.start_line.cmp(&right.1.start_line))
    });

    let mut processes = Vec::new();
    let mut steps = Vec::new();
    let mut seen_signatures = BTreeSet::new();

    for (_, entry_symbol) in entry_candidates.into_iter().take(8) {
        let trace = bfs_trace(&entry_symbol.symbol_id, &outgoing, 8, 24);
        if trace.len() < 2 {
            continue;
        }

        let endpoint = trace.last().cloned();
        let signature = format!(
            "{}:{}:{}",
            entry_symbol.symbol_id,
            endpoint.clone().unwrap_or_default(),
            trace.join("|")
        );
        if !seen_signatures.insert(signature) {
            continue;
        }
        if is_subset_trace(&trace, &processes, &steps) {
            continue;
        }

        let process_id = stable_id("process", trace.join("|"));
        let label = format!("{} flow", entry_symbol.name);
        let process_type = infer_process_type(entry_symbol);

        processes.push(ProcessNode {
            process_id: process_id.clone(),
            label,
            process_type,
            step_count: trace.len(),
            entry_point_id: Some(entry_symbol.symbol_id.clone()),
            terminal_id: endpoint.clone(),
        });
        steps.extend(
            trace
                .into_iter()
                .enumerate()
                .map(|(index, symbol_id)| ProcessStep {
                    process_id: process_id.clone(),
                    symbol_id,
                    step_order: index,
                }),
        );
    }

    processes.sort_by(|left, right| left.process_id.cmp(&right.process_id));
    steps.sort_by(|left, right| {
        left.process_id
            .cmp(&right.process_id)
            .then(left.step_order.cmp(&right.step_order))
    });

    (processes, steps)
}

fn bfs_trace(
    entry_symbol_id: &str,
    outgoing: &BTreeMap<String, Vec<String>>,
    max_depth: usize,
    max_nodes: usize,
) -> Vec<String> {
    let mut queue = VecDeque::from([(entry_symbol_id.to_string(), 0usize)]);
    let mut visited = BTreeSet::new();
    let mut trace = Vec::new();

    while let Some((current, depth)) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        trace.push(current.clone());
        if trace.len() >= max_nodes || depth >= max_depth {
            continue;
        }
        if let Some(targets) = outgoing.get(&current) {
            for target in targets {
                if !visited.contains(target) {
                    queue.push_back((target.clone(), depth + 1));
                }
            }
        }
    }

    trace
}

fn is_subset_trace(trace: &[String], processes: &[ProcessNode], steps: &[ProcessStep]) -> bool {
    let trace_set = trace.iter().cloned().collect::<BTreeSet<_>>();

    processes.iter().any(|process| {
        let existing = steps
            .iter()
            .filter(|step| step.process_id == process.process_id)
            .map(|step| step.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        !existing.is_empty() && trace_set.is_subset(&existing)
    })
}

fn entry_score(symbol: &SymbolNode, indegree: usize, outgoing_count: usize) -> usize {
    let mut score = 0usize;
    if indegree == 0 {
        score += 4;
    }
    if symbol.is_exported {
        score += 3;
    }
    if outgoing_count > 0 {
        score += 2;
    }
    if matches!(symbol.label.as_str(), "function" | "method") {
        score += 2;
    }
    if matches_name_pattern(
        &symbol.name,
        &["handle", "main", "run", "serve", "start", "sync"],
    ) {
        score += 3;
    }
    score
}

fn infer_process_type(symbol: &SymbolNode) -> String {
    if matches_name_pattern(
        &symbol.name,
        &["handle", "serve", "request", "route", "http", "api"],
    ) {
        return "request-flow".to_string();
    }
    if matches_name_pattern(&symbol.name, &["job", "batch", "sync", "worker", "queue"]) {
        return "batch-job".to_string();
    }
    "call-flow".to_string()
}

fn matches_name_pattern(name: &str, patterns: &[&str]) -> bool {
    let lower = name.to_ascii_lowercase();
    patterns.iter().any(|pattern| lower.contains(pattern))
}
