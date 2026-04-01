use std::fs;
use std::path::Path;

use tempfile::tempdir;
use wiki_index::hierarchy::build_module_tree_with_graph;
use wiki_index::scanner::scan_repo;
use wiki_index::symbol_graph::{
    analyze_symbol_graph, build_graph_summary, resolve_symbol_graph, ResolvedGraphSnapshot,
    ResolvedSymbolEdge,
};
use wiki_index::symbols::{parse_symbols, ParsedSymbolsSnapshot, SymbolNode, SymbolTable};
use wiki_runtime::generation::context::build_repo_context_with_graph;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::workflows::init::run_init;

fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn write_graph_demo_repo(repo_root: &Path) {
    write_repo_file(repo_root, "package.json", r#"{"name":"graph-demo"}"#);
    write_repo_file(
        repo_root,
        "src/shared.ts",
        "export function finalizePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "src/service.ts",
        concat!(
            "import { finalizePayment } from \"./shared\";\n",
            "export function runPayment() {\n",
            "  return finalizePayment();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "src/controller.ts",
        concat!(
            "import { runPayment } from \"./service\";\n",
            "export function handleCheckout() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "src/cycle_a.ts",
        concat!(
            "import { bounceB } from \"./cycle_b\";\n",
            "export function bounceA() {\n",
            "  return bounceB();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "src/cycle_b.ts",
        concat!(
            "import { bounceA } from \"./cycle_a\";\n",
            "export function bounceB() {\n",
            "  return bounceA();\n",
            "}\n",
        ),
    );
}

#[test]
fn analyze_symbol_graph_detects_communities_processes_cycles_and_persists_them() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_demo_repo(repo_root);

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed_symbols = parse_symbols(repo_root, &scan_report).unwrap();
    let resolved_graph = resolve_symbol_graph(repo_root, &scan_report, &parsed_symbols).unwrap();
    let analysis = analyze_symbol_graph(&parsed_symbols, &resolved_graph);

    assert!(
        !analysis.communities.is_empty(),
        "expected at least one detected community"
    );
    assert!(
        analysis
            .processes
            .iter()
            .any(|process| process.label.contains("handleCheckout")),
        "expected a process rooted at handleCheckout, got {:#?}",
        analysis.processes
    );
    assert!(
        analysis
            .cycles
            .iter()
            .any(|cycle| cycle.warning.contains("bounceA") || cycle.warning.contains("bounceB")),
        "expected cycle warning for bounceA/bounceB, got {:#?}",
        analysis.cycles
    );

    run_init(repo_root).unwrap();

    assert!(
        !sqlite_store::list_communities(repo_root)
            .unwrap()
            .is_empty(),
        "expected persisted communities"
    );
    assert!(
        !sqlite_store::list_processes(repo_root).unwrap().is_empty(),
        "expected persisted processes"
    );
    assert!(
        !sqlite_store::list_process_steps(repo_root)
            .unwrap()
            .is_empty(),
        "expected persisted process steps"
    );
}

#[test]
fn graph_summary_drives_cross_module_edges_and_repo_context() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"monorepo","workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/core/package.json",
        r#"{"name":"@demo/core"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/web/package.json",
        r#"{"name":"@demo/web"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/core/src/shared.ts",
        "export function finalizePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "packages/core/src/service.ts",
        concat!(
            "import { finalizePayment } from \"./shared\";\n",
            "export function runPayment() {\n",
            "  return finalizePayment();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "packages/web/src/handler.ts",
        concat!(
            "import { runPayment } from \"../../core/src/service\";\n",
            "export function handleCheckout() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    );

    let scan_report = scan_repo(repo_root, &[]).unwrap();
    let parsed_symbols = parse_symbols(repo_root, &scan_report).unwrap();
    let resolved_graph = resolve_symbol_graph(repo_root, &scan_report, &parsed_symbols).unwrap();
    let analysis = analyze_symbol_graph(&parsed_symbols, &resolved_graph);
    let graph_summary =
        build_graph_summary(&scan_report, &parsed_symbols, &resolved_graph, &analysis);
    let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
    let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);

    assert_eq!(
        graph_summary
            .module_dependency_hints
            .get("packages/web")
            .cloned()
            .unwrap_or_default(),
        vec!["packages/core".to_string()]
    );
    assert!(
        module_tree.cross_module_edges.iter().any(|edge| {
            edge.relation_type == "GRAPH_DEPENDS_ON"
                && edge.evidence.contains(&"graph:packages/web".to_string())
                && edge.evidence.contains(&"graph:packages/core".to_string())
        }),
        "expected GRAPH_DEPENDS_ON edge, got {:#?}",
        module_tree.cross_module_edges
    );
    assert!(
        repo_context
            .detected_processes
            .iter()
            .any(|process| process.contains("handleCheckout")),
        "expected graph-derived process in repo context, got {:#?}",
        repo_context.detected_processes
    );
}

#[test]
fn community_detection_prefers_weighted_clusters_over_single_connected_component() {
    let symbols = vec![
        test_symbol("a1", "alpha_one"),
        test_symbol("a2", "alpha_two"),
        test_symbol("a3", "alpha_three"),
        test_symbol("b1", "beta_one"),
        test_symbol("b2", "beta_two"),
        test_symbol("b3", "beta_three"),
    ];
    let parsed_symbols = ParsedSymbolsSnapshot {
        symbol_table: SymbolTable::from_symbols(&symbols),
        symbols,
        ..ParsedSymbolsSnapshot::default()
    };
    let resolved_graph = ResolvedGraphSnapshot {
        edges: vec![
            weighted_call("a1", "a2", 0.95),
            weighted_call("a2", "a3", 0.95),
            weighted_call("a1", "a3", 0.95),
            weighted_call("b1", "b2", 0.95),
            weighted_call("b2", "b3", 0.95),
            weighted_call("b1", "b3", 0.95),
            weighted_call("a3", "b1", 0.60),
        ],
        diagnostics: Vec::new(),
    };

    let analysis = analyze_symbol_graph(&parsed_symbols, &resolved_graph);
    let memberships = analysis
        .community_members
        .iter()
        .map(|member| (member.symbol_id.as_str(), member.community_id.as_str()))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(
        analysis.communities.len() >= 2,
        "expected weighted clustering to split weakly connected groups, got {:#?}",
        analysis.communities
    );
    assert_eq!(memberships.get("a1"), memberships.get("a2"));
    assert_eq!(memberships.get("a2"), memberships.get("a3"));
    assert_eq!(memberships.get("b1"), memberships.get("b2"));
    assert_eq!(memberships.get("b2"), memberships.get("b3"));
    assert_ne!(memberships.get("a1"), memberships.get("b1"));
}

fn test_symbol(symbol_id: &str, name: &str) -> SymbolNode {
    SymbolNode {
        symbol_id: symbol_id.to_string(),
        name: name.to_string(),
        label: "function".to_string(),
        file_path: format!("src/{symbol_id}.ts"),
        start_line: 1,
        end_line: 1,
        is_exported: true,
        language: "typescript".to_string(),
    }
}

fn weighted_call(source_id: &str, target_id: &str, confidence: f64) -> ResolvedSymbolEdge {
    ResolvedSymbolEdge {
        edge_id: format!("edge:{source_id}:{target_id}"),
        source_id: source_id.to_string(),
        target_id: target_id.to_string(),
        edge_type: "CALLS".to_string(),
        confidence,
        reason: "test-call".to_string(),
    }
}
