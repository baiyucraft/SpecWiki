use std::path::PathBuf;

use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::scan_repo;
use wiki_runtime::generation::context::build_module_contexts;

fn fixture_path() -> PathBuf {
    [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/noise-filter-repo"),
        PathBuf::from("crates/wiki-runtime/tests/fixtures/noise-filter-repo"),
        PathBuf::from("tests/fixtures/noise-filter-repo"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("noise-filter-repo fixture should exist")
}

#[test]
fn key_sources_prioritize_core_over_test_files() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);
    let module_contexts = build_module_contexts(&report, &module_tree);

    // Find the root module context (it contains all files)
    let root_context = module_contexts
        .iter()
        .find(|ctx| {
            module_tree
                .modules
                .iter()
                .any(|m| m.id == ctx.module_id && m.parent_id.is_none())
        })
        .expect("root module context should exist");

    // src/ files should appear before tests/ files in key_sources
    let src_positions: Vec<usize> = root_context
        .key_sources
        .iter()
        .enumerate()
        .filter(|(_, path)| path.starts_with("src/"))
        .map(|(i, _)| i)
        .collect();

    let test_positions: Vec<usize> = root_context
        .key_sources
        .iter()
        .enumerate()
        .filter(|(_, path)| path.starts_with("tests/"))
        .map(|(i, _)| i)
        .collect();

    assert!(
        !src_positions.is_empty(),
        "key_sources should contain src/ files, got: {:?}",
        root_context.key_sources
    );

    if !test_positions.is_empty() {
        let max_src = src_positions.iter().max().unwrap();
        let min_test = test_positions.iter().min().unwrap();
        assert!(
            max_src < min_test,
            "src/ files should rank higher than tests/ files in key_sources: {:?}",
            root_context.key_sources
        );
    }
}

#[test]
fn key_sources_exclude_fixture_paths() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);
    let module_contexts = build_module_contexts(&report, &module_tree);

    for ctx in &module_contexts {
        assert!(
            !ctx.key_sources
                .iter()
                .any(|path| path.contains("fixtures/")),
            "fixture files should not appear in key_sources: {:?}",
            ctx.key_sources
        );
    }
}
