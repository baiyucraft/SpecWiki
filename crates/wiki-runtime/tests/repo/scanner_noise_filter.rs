use std::path::PathBuf;

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
fn scanner_excludes_nested_repo_with_dot_git() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    assert!(
        !report.files.iter().any(|f| f.path.contains("nested-repo/")),
        "files from nested repo (with .git) should be excluded"
    );
}

#[test]
fn scanner_excludes_fixture_directories() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    assert!(
        !report
            .files
            .iter()
            .any(|f| f.path.contains("tests/fixtures/")),
        "files from tests/fixtures/ should be excluded"
    );
}

#[test]
fn scanner_excludes_non_code_artifact_directories() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    assert!(
        !report.files.iter().any(|f| f.path.starts_with(".spec/")),
        "files from .spec/ should be excluded"
    );
    assert!(
        !report.files.iter().any(|f| f.path.starts_with(".github/")),
        "files from .github/ should be excluded"
    );
    assert!(
        !report.files.iter().any(|f| f.path.starts_with("coverage/")),
        "files from coverage/ should be excluded"
    );
}

#[test]
fn scanner_retains_core_source_files() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    assert!(
        report.files.iter().any(|f| f.path == "src/index.ts"),
        "core source file src/index.ts should be retained"
    );
    assert!(
        report.files.iter().any(|f| f.path == "src/utils.ts"),
        "core source file src/utils.ts should be retained"
    );
}

#[test]
fn scanner_tags_test_path_files() {
    let report = scan_repo(&fixture_path(), &[]).unwrap();
    let test_file = report.files.iter().find(|f| f.path.contains("tests/unit/"));
    assert!(
        test_file.is_some(),
        "test file in tests/unit/ should be scanned"
    );
    assert!(
        test_file.unwrap().tags.iter().any(|t| t == "test-file"),
        "test file should have test-file tag"
    );
}
