use std::fs;

use tempfile::tempdir;
use wiki_core::workflows::{init::run_init, query::run_query, rebuild::run_rebuild, sync::run_sync};

#[test]
fn sync_detects_manual_markdown_changes() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    fs::write(&overview, "# 项目概述\n\n自定义说明\n").unwrap();

    let result = run_sync(repo_root).unwrap();
    assert_eq!(result.state, "fresh");
    assert!(result
        .synced_pages
        .iter()
        .any(|path| path.ends_with("项目概述.md")));

    let query = run_query(repo_root, "项目概述").unwrap();
    assert_eq!(query.term, "项目概述");
    assert!(!query.matched_pages.is_empty());
    assert!(!query.matched_modules.is_empty());
    assert!(!query.matched_sources.is_empty());
    assert!(!query.matches.is_empty());
    assert!(query
        .matches
        .iter()
        .any(|page| page.reasons.iter().any(|reason| reason == "页面标题匹配")));
    assert!(query
        .matches
        .iter()
        .all(|page| page.match_mode == "structure"));

    let rebuild = run_rebuild(repo_root).unwrap();
    assert_eq!(rebuild.state, "fresh");
    assert!(!rebuild.updated_pages.is_empty());
}

#[test]
fn query_falls_back_to_markdown_and_returns_empty_result() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    let custom_phrase = "仅在 Markdown 中出现的唯一短语";
    fs::write(&overview, format!("# 项目概述\n\n{custom_phrase}\n")).unwrap();
    run_sync(repo_root).unwrap();

    let markdown_query = run_query(repo_root, custom_phrase).unwrap();
    assert_eq!(markdown_query.matches.len(), 1);
    assert_eq!(markdown_query.matches[0].match_mode, "fallback_markdown");
    assert!(markdown_query.matches[0].summary.contains("Markdown 内容匹配"));

    let empty_query = run_query(repo_root, "definitely-no-query-hit").unwrap();
    assert!(empty_query.matches.is_empty());
    assert!(empty_query.matched_pages.is_empty());
    assert!(empty_query.matched_modules.is_empty());
    assert!(empty_query.matched_sources.is_empty());
    assert!(empty_query.matched_relations.is_empty());
}
