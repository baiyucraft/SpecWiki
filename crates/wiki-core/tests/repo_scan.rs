use std::path::PathBuf;

use wiki_core::repo::scanner::scan_repo;

#[test]
fn scan_repo_discovers_files_and_detected_stack() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/minimal-node-repo");
    let report = scan_repo(&fixture).unwrap();

    assert!(report.files.iter().any(|f| f.path.ends_with("package.json")));
    assert!(report
        .detected_topics
        .iter()
        .any(|topic| topic == "frontend"));
}
