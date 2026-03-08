use std::path::PathBuf;

use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;

#[test]
fn hierarchy_planning_generates_architecture_and_module_pages() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-node-repo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/minimal-node-repo"),
        PathBuf::from("tests/fixtures/minimal-node-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for hierarchy planning tests");

    let report = scan_repo(&fixture).unwrap();
    let module_tree = build_module_tree(&report);
    let repo_context = build_repo_context(&report, &module_tree);
    let module_contexts = build_module_contexts(&report, &module_tree);
    let pages = plan_pages(&report, &module_tree, &repo_context, &module_contexts);

    assert!(module_tree.modules.len() >= 2);
    assert!(pages.iter().any(|page| page.page_type == "overview"));
    assert!(pages.iter().any(|page| page.page_type == "architecture"));
    assert!(pages.iter().any(|page| page.page_type == "module"));
}

#[test]
fn hierarchy_planning_promotes_mixed_top_level_directories_to_modules() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mixed-local-repo"),
        PathBuf::from("crates/wiki-core/tests/fixtures/mixed-local-repo"),
        PathBuf::from("tests/fixtures/mixed-local-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for mixed repo hierarchy tests");

    let report = scan_repo(&fixture).unwrap();
    let module_tree = build_module_tree(&report);
    let repo_context = build_repo_context(&report, &module_tree);
    let module_contexts = build_module_contexts(&report, &module_tree);
    let pages = plan_pages(&report, &module_tree, &repo_context, &module_contexts);

    let module_names = module_tree
        .non_root_modules()
        .into_iter()
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

    assert!(module_names.iter().any(|name| name == "web"));
    assert!(module_names.iter().any(|name| name == "spider"));
    assert!(module_names.iter().any(|name| name == "nginx"));

    assert!(pages
        .iter()
        .any(|page| page.relative_path.ends_with("核心模块/web.md")));
    assert!(pages
        .iter()
        .any(|page| page.relative_path.ends_with("核心模块/spider.md")));
    assert!(pages
        .iter()
        .any(|page| page.relative_path.ends_with("核心模块/nginx.md")));
}

#[test]
fn hierarchy_planning_builds_cross_module_edges_from_manifest_and_rust_parsers() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rust-workspace"),
        PathBuf::from("crates/wiki-core/tests/fixtures/rust-workspace"),
        PathBuf::from("tests/fixtures/rust-workspace"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for rust workspace hierarchy tests");

    let report = scan_repo(&fixture).unwrap();
    let module_tree = build_module_tree(&report);

    let module_names = module_tree
        .non_root_modules()
        .into_iter()
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

    assert!(module_names.iter().any(|name| name == "api"));
    assert!(module_names.iter().any(|name| name == "core"));
    assert!(module_tree.cross_module_edges.iter().any(|edge| {
        let source_name = module_tree
            .module_by_id(&edge.source)
            .map(|module| module.name.as_str())
            .unwrap_or_default();
        let target_name = module_tree
            .module_by_id(&edge.target)
            .map(|module| module.name.as_str())
            .unwrap_or_default();

        source_name == "api" && target_name == "core"
    }));
}

#[test]
fn hierarchy_planning_builds_cross_module_edges_from_java_package_aliases() {
    let fixture = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/java-modules"),
        PathBuf::from("crates/wiki-core/tests/fixtures/java-modules"),
        PathBuf::from("tests/fixtures/java-modules"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist for java hierarchy tests");

    let report = scan_repo(&fixture).unwrap();
    let module_tree = build_module_tree(&report);

    assert!(module_tree.cross_module_edges.iter().any(|edge| {
        let source_name = module_tree
            .module_by_id(&edge.source)
            .map(|module| module.name.as_str())
            .unwrap_or_default();
        let target_name = module_tree
            .module_by_id(&edge.target)
            .map(|module| module.name.as_str())
            .unwrap_or_default();

        source_name == "api" && target_name == "core"
    }));
}
