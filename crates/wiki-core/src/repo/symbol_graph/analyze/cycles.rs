use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::domain::stable_id::stable_id;
use crate::repo::symbol_graph::{CycleSummary, ResolvedGraphSnapshot};
use crate::repo::symbols::ParsedSymbolsSnapshot;

/// 对高质量非 import 图边执行 Tarjan SCC，生成 cycle summary。
pub fn detect_cycles(
    parsed_symbols: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> Vec<CycleSummary> {
    let edges = resolved_graph
        .edges
        .iter()
        .filter(|edge| {
            matches!(edge.edge_type.as_str(), "CALLS" | "EXTENDS" | "IMPLEMENTS")
                && edge.confidence >= 0.55
        })
        .collect::<Vec<_>>();
    let symbol_names = parsed_symbols
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol.name.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    let mut edge_by_pair = BTreeMap::<(String, String), Vec<(String, f64)>>::new();
    for edge in &edges {
        adjacency
            .entry(edge.source_id.clone())
            .or_default()
            .push(edge.target_id.clone());
        edge_by_pair
            .entry((edge.source_id.clone(), edge.target_id.clone()))
            .or_default()
            .push((edge.edge_id.clone(), edge.confidence));
    }
    for targets in adjacency.values_mut() {
        targets.sort();
        targets.dedup();
    }

    let mut state = TarjanState::default();
    let keys = adjacency.keys().cloned().collect::<Vec<_>>();
    for node in keys {
        if !state.indices.contains_key(&node) {
            strong_connect(&node, &adjacency, &mut state);
        }
    }

    let mut cycles = Vec::new();
    for component in state.components {
        let has_self_loop = component.iter().any(|node| {
            adjacency
                .get(node)
                .map(|targets| targets.contains(node))
                .unwrap_or(false)
        });
        if component.len() < 2 && !has_self_loop {
            continue;
        }

        let component_set = component.iter().cloned().collect::<BTreeSet<_>>();
        let break_edge_ids = edges
            .iter()
            .filter(|edge| {
                component_set.contains(&edge.source_id) && component_set.contains(&edge.target_id)
            })
            .map(|edge| (edge.edge_id.clone(), edge.confidence))
            .min_by(|left, right| {
                left.1
                    .partial_cmp(&right.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(edge_id, _)| vec![edge_id])
            .unwrap_or_default();
        let topo_order = condensed_topo_order(&component_set, &adjacency, &break_edge_ids, &edges);
        let warning = format!(
            "循环依赖：{}",
            component
                .iter()
                .map(|symbol_id| {
                    symbol_names
                        .get(symbol_id)
                        .cloned()
                        .unwrap_or_else(|| symbol_id.clone())
                })
                .collect::<Vec<_>>()
                .join(" -> ")
        );

        cycles.push(CycleSummary {
            cycle_id: stable_id("cycle", component.join("|")),
            symbol_ids: component,
            break_edge_ids,
            topo_order,
            warning,
        });
    }

    cycles.sort_by(|left, right| left.cycle_id.cmp(&right.cycle_id));
    cycles
}

#[derive(Default)]
struct TarjanState {
    index: usize,
    indices: BTreeMap<String, usize>,
    lowlinks: BTreeMap<String, usize>,
    stack: Vec<String>,
    on_stack: BTreeSet<String>,
    components: Vec<Vec<String>>,
}

fn strong_connect(
    node: &str,
    adjacency: &BTreeMap<String, Vec<String>>,
    state: &mut TarjanState,
) {
    state.indices.insert(node.to_string(), state.index);
    state.lowlinks.insert(node.to_string(), state.index);
    state.index += 1;
    state.stack.push(node.to_string());
    state.on_stack.insert(node.to_string());

    if let Some(targets) = adjacency.get(node) {
        for target in targets {
            if !state.indices.contains_key(target) {
                strong_connect(target, adjacency, state);
                let lowlink = *state.lowlinks.get(node).unwrap();
                let target_lowlink = *state.lowlinks.get(target).unwrap();
                state
                    .lowlinks
                    .insert(node.to_string(), lowlink.min(target_lowlink));
            } else if state.on_stack.contains(target) {
                let lowlink = *state.lowlinks.get(node).unwrap();
                let target_index = *state.indices.get(target).unwrap();
                state
                    .lowlinks
                    .insert(node.to_string(), lowlink.min(target_index));
            }
        }
    }

    if state.lowlinks.get(node) == state.indices.get(node) {
        let mut component = Vec::new();
        while let Some(candidate) = state.stack.pop() {
            state.on_stack.remove(&candidate);
            component.push(candidate.clone());
            if candidate == node {
                break;
            }
        }
        component.sort();
        state.components.push(component);
    }
}

fn condensed_topo_order(
    component: &BTreeSet<String>,
    adjacency: &BTreeMap<String, Vec<String>>,
    break_edge_ids: &[String],
    edges: &[&crate::repo::symbol_graph::ResolvedSymbolEdge],
) -> Vec<String> {
    let break_pairs = edges
        .iter()
        .filter(|edge| break_edge_ids.contains(&edge.edge_id))
        .map(|edge| (edge.source_id.clone(), edge.target_id.clone()))
        .collect::<BTreeSet<_>>();
    let mut indegree = BTreeMap::<String, usize>::new();
    let mut local_adjacency = BTreeMap::<String, Vec<String>>::new();

    for node in component {
        indegree.entry(node.clone()).or_default();
        if let Some(targets) = adjacency.get(node) {
            for target in targets {
                if component.contains(target) && !break_pairs.contains(&(node.clone(), target.clone()))
                {
                    local_adjacency
                        .entry(node.clone())
                        .or_default()
                        .push(target.clone());
                    *indegree.entry(target.clone()).or_default() += 1;
                }
            }
        }
    }
    for targets in local_adjacency.values_mut() {
        targets.sort();
        targets.dedup();
    }

    let mut queue = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(node, _)| node.clone())
        .collect::<VecDeque<_>>();
    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        if let Some(targets) = local_adjacency.get(&node) {
            for target in targets {
                if let Some(degree) = indegree.get_mut(target) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(target.clone());
                    }
                }
            }
        }
    }

    if order.len() == component.len() {
        return order;
    }

    component.iter().cloned().collect()
}
