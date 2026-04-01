use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::symbol_graph::{CommunityMember, CommunityNode, ResolvedGraphSnapshot};
use crate::symbols::ParsedSymbolsSnapshot;
use wiki_model::domain::stable_id::stable_id;

const COMMUNITY_EDGE_CONFIDENCE_THRESHOLD: f64 = 0.55;
const LARGE_GRAPH_NODE_THRESHOLD: usize = 10_000;
const LARGE_GRAPH_MIN_DEGREE: usize = 2;
const LABEL_PROPAGATION_MAX_ITERATIONS: usize = 24;

/// 基于高质量 `CALLS / EXTENDS / IMPLEMENTS` 边做 community 检测。
/// 当前优先使用加权标签传播近似聚类；若结果退化，再回退到 deterministic 连通分量。
pub fn detect_communities(
    parsed_symbols: &ParsedSymbolsSnapshot,
    resolved_graph: &ResolvedGraphSnapshot,
) -> (Vec<CommunityNode>, Vec<CommunityMember>) {
    let filtered_edges = resolved_graph
        .edges
        .iter()
        .filter(|edge| {
            matches!(edge.edge_type.as_str(), "CALLS" | "EXTENDS" | "IMPLEMENTS")
                && edge.confidence >= COMMUNITY_EDGE_CONFIDENCE_THRESHOLD
        })
        .collect::<Vec<_>>();
    let original_adjacency = build_weighted_adjacency(&filtered_edges);
    if original_adjacency.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let denoised_adjacency = if original_adjacency.len() > LARGE_GRAPH_NODE_THRESHOLD {
        prune_sparse_nodes(&original_adjacency, LARGE_GRAPH_MIN_DEGREE)
    } else {
        original_adjacency.clone()
    };
    let analysis_adjacency = if denoised_adjacency.is_empty() {
        original_adjacency.clone()
    } else {
        denoised_adjacency
    };

    let preferred_groups = label_propagation_groups(&analysis_adjacency);
    let component_groups = connected_component_groups(&analysis_adjacency);
    let groups = if is_meaningful_groups(&preferred_groups) {
        preferred_groups
    } else {
        component_groups
    };

    build_communities(parsed_symbols, &filtered_edges, groups)
}

fn build_weighted_adjacency(
    edges: &[&crate::symbol_graph::ResolvedSymbolEdge],
) -> BTreeMap<String, BTreeMap<String, f64>> {
    let mut adjacency = BTreeMap::<String, BTreeMap<String, f64>>::new();

    for edge in edges {
        *adjacency
            .entry(edge.source_id.clone())
            .or_default()
            .entry(edge.target_id.clone())
            .or_default() += edge.confidence;
        *adjacency
            .entry(edge.target_id.clone())
            .or_default()
            .entry(edge.source_id.clone())
            .or_default() += edge.confidence;
    }

    adjacency
}

fn prune_sparse_nodes(
    adjacency: &BTreeMap<String, BTreeMap<String, f64>>,
    min_degree: usize,
) -> BTreeMap<String, BTreeMap<String, f64>> {
    let retained = adjacency
        .iter()
        .filter(|(_, neighbors)| neighbors.len() >= min_degree)
        .map(|(node_id, _)| node_id.clone())
        .collect::<BTreeSet<_>>();

    adjacency
        .iter()
        .filter(|(node_id, _)| retained.contains(*node_id))
        .map(|(node_id, neighbors)| {
            let filtered = neighbors
                .iter()
                .filter(|(neighbor_id, _)| retained.contains(*neighbor_id))
                .map(|(neighbor_id, weight)| (neighbor_id.clone(), *weight))
                .collect::<BTreeMap<_, _>>();
            (node_id.clone(), filtered)
        })
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .collect()
}

fn label_propagation_groups(
    adjacency: &BTreeMap<String, BTreeMap<String, f64>>,
) -> Vec<Vec<String>> {
    let mut labels = adjacency
        .keys()
        .map(|node_id| (node_id.clone(), node_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut order = adjacency
        .iter()
        .map(|(node_id, neighbors)| {
            let degree_weight = neighbors.values().sum::<f64>();
            (node_id.clone(), degree_weight, neighbors.len())
        })
        .collect::<Vec<_>>();
    order.sort_by(|left, right| {
        right
            .1
            .partial_cmp(&left.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(right.2.cmp(&left.2))
            .then(left.0.cmp(&right.0))
    });

    for _ in 0..LABEL_PROPAGATION_MAX_ITERATIONS {
        let mut moved = false;

        for (node_id, _, _) in &order {
            let Some(neighbors) = adjacency.get(node_id) else {
                continue;
            };

            let mut label_weights = BTreeMap::<String, f64>::new();
            for (neighbor_id, weight) in neighbors {
                let Some(label) = labels.get(neighbor_id) else {
                    continue;
                };
                *label_weights.entry(label.clone()).or_default() += *weight;
            }

            let current_label = labels
                .get(node_id)
                .cloned()
                .unwrap_or_else(|| node_id.clone());
            let current_weight = label_weights.get(&current_label).copied().unwrap_or(0.0);
            let Some((best_label, best_weight)) =
                label_weights.into_iter().max_by(|left, right| {
                    left.1
                        .partial_cmp(&right.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| right.0.cmp(&left.0))
                })
            else {
                continue;
            };

            if best_weight > current_weight + f64::EPSILON && best_label != current_label {
                labels.insert(node_id.clone(), best_label);
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }

    groups_from_labels(&labels)
}

fn groups_from_labels(labels: &BTreeMap<String, String>) -> Vec<Vec<String>> {
    let mut groups = labels.iter().fold(
        BTreeMap::<String, Vec<String>>::new(),
        |mut acc, (node_id, label)| {
            acc.entry(label.clone()).or_default().push(node_id.clone());
            acc
        },
    );
    let mut grouped = groups
        .values_mut()
        .map(|members| {
            members.sort();
            members.clone()
        })
        .filter(|members| members.len() >= 2)
        .collect::<Vec<_>>();
    grouped.sort();
    grouped
}

fn connected_component_groups(
    adjacency: &BTreeMap<String, BTreeMap<String, f64>>,
) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut seen = BTreeSet::new();

    for node_id in adjacency.keys() {
        if !seen.insert(node_id.clone()) {
            continue;
        }

        let mut queue = VecDeque::from([node_id.clone()]);
        let mut component = BTreeSet::new();
        while let Some(current) = queue.pop_front() {
            if !component.insert(current.clone()) {
                continue;
            }
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor_id in neighbors.keys() {
                    if !component.contains(neighbor_id) {
                        queue.push_back(neighbor_id.clone());
                    }
                    seen.insert(neighbor_id.clone());
                }
            }
        }

        if component.len() >= 2 {
            groups.push(component.into_iter().collect());
        }
    }

    groups.sort();
    groups
}

fn is_meaningful_groups(groups: &[Vec<String>]) -> bool {
    !groups.is_empty() && groups.iter().any(|group| group.len() >= 2)
}

fn build_communities(
    parsed_symbols: &ParsedSymbolsSnapshot,
    filtered_edges: &[&crate::symbol_graph::ResolvedSymbolEdge],
    groups: Vec<Vec<String>>,
) -> (Vec<CommunityNode>, Vec<CommunityMember>) {
    let symbol_names = parsed_symbols
        .symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol.name.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut communities = Vec::new();
    let mut members = Vec::new();

    for component_ids in groups {
        let component_set = component_ids.iter().cloned().collect::<BTreeSet<_>>();
        let internal_edge_weight = filtered_edges
            .iter()
            .filter(|edge| {
                component_set.contains(&edge.source_id) && component_set.contains(&edge.target_id)
            })
            .map(|edge| edge.confidence)
            .sum::<f64>();
        let possible_edges = component_ids.len() * (component_ids.len() - 1);
        let cohesion = if possible_edges == 0 {
            1.0
        } else {
            internal_edge_weight / possible_edges as f64
        };
        let label = component_ids
            .iter()
            .filter_map(|symbol_id| symbol_names.get(symbol_id))
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(" / ");
        let community_id = stable_id("community", component_ids.join("|"));

        communities.push(CommunityNode {
            community_id: community_id.clone(),
            label: if label.is_empty() {
                "community".to_string()
            } else {
                label
            },
            cohesion,
            symbol_count: component_ids.len(),
        });
        members.extend(component_ids.into_iter().map(|symbol_id| CommunityMember {
            community_id: community_id.clone(),
            symbol_id,
        }));
    }

    communities.sort_by(|left, right| left.community_id.cmp(&right.community_id));
    members.sort_by(|left, right| {
        left.community_id
            .cmp(&right.community_id)
            .then(left.symbol_id.cmp(&right.symbol_id))
    });

    (communities, members)
}
