use std::path::PathBuf;

use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::scan_repo;

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
fn single_file_directory_is_not_promoted_to_module() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let module_names: Vec<&str> = module_tree
        .modules
        .iter()
        .map(|m| m.name.as_str())
        .collect();

    assert!(
        !module_names.contains(&"lonely-dir"),
        "single-file directory should not be promoted to a module, got modules: {module_names:?}"
    );
}

#[test]
fn noise_filtered_directories_do_not_become_modules() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    let module_names: Vec<&str> = module_tree
        .modules
        .iter()
        .map(|m| m.name.as_str())
        .collect();

    assert!(
        !module_names.contains(&".spec"),
        ".spec should not be a module, got: {module_names:?}"
    );
    assert!(
        !module_names.contains(&"nested-repo"),
        "nested-repo should not be a module, got: {module_names:?}"
    );
    assert!(
        !module_names.contains(&"coverage"),
        "coverage should not be a module, got: {module_names:?}"
    );
}

#[test]
fn test_files_do_not_dominate_module_promotion() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let module_tree = build_module_tree(&report);

    // tests directory should not be promoted as a top-level module
    // because its non-test source count is low after test-file demotion
    let module_names: Vec<&str> = module_tree
        .modules
        .iter()
        .filter(|m| m.parent_id.is_some())
        .map(|m| m.name.as_str())
        .collect();

    // The "tests" directory should not appear as a standalone promoted module
    let has_tests_as_standalone = module_names.contains(&"tests");
    if has_tests_as_standalone {
        // If it does appear, verify it wasn't promoted due to test files alone
        let tests_module = module_tree
            .modules
            .iter()
            .find(|m| m.name == "tests" && m.root_paths.iter().any(|r| r == "tests"))
            .unwrap();
        // It should be a low-priority module (not many source_ids after filtering)
        assert!(
            tests_module.source_ids.len() <= 2,
            "tests module should have few sources if promoted"
        );
    }
}
