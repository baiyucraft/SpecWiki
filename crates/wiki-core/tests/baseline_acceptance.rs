use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;
use wiki_core::storage::metadata_store::read_metadata;
use wiki_core::workflows::{init::run_init, query::run_query};

#[test]
fn baseline_fixture_generates_hierarchical_wiki_and_structured_query_results() {
    let fixture = copy_fixture_to_temp("baseline-hierarchy-repo");
    let repo_root = fixture.path();

    run_init(repo_root).unwrap();

    assert!(repo_root.join(".wiki/项目概述.md").exists());
    assert!(repo_root.join(".wiki/系统架构.md").exists());
    assert!(repo_root.join(".wiki/核心模块/packages/domain.md").exists());
    assert!(repo_root.join(".wiki/核心模块/packages/domain/auth.md").exists());
    assert!(repo_root.join(".wiki/核心模块/infra/nginx.md").exists());

    let auth_page = fs::read_to_string(repo_root.join(".wiki/核心模块/packages/domain/auth.md")).unwrap();
    assert!(auth_page.contains("模块说明"));
    assert!(auth_page.contains("模块名称：auth"));

    let metadata = read_metadata(repo_root).unwrap();
    let auth_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path.ends_with("核心模块/packages/domain/auth.md"))
        .expect("auth page should be exported to metadata");
    let architecture_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path.ends_with("系统架构.md"))
        .expect("architecture page should be exported to metadata");
    assert!(!auth_item.ancestor_ids.is_empty());
    assert!(auth_item
        .provenance
        .iter()
        .any(|entry| entry.starts_with("module:")));
    assert!(auth_item
        .provenance
        .iter()
        .any(|entry| entry.starts_with("source:")));
    assert!(!architecture_item.summary.contains("module-"));

    let query = run_query(repo_root, "auth").unwrap();
    assert!(query
        .matched_pages
        .iter()
        .any(|path| path.ends_with("核心模块/packages/domain/auth.md")));
    assert!(query.matched_modules.iter().any(|module| {
        module.root_paths.iter().any(|root| root == "packages/domain/auth")
    }));
    assert!(query.matches.iter().all(|page| page.match_mode == "structure"));
    assert!(query
        .matches
        .iter()
        .any(|page| page.summary.contains("模块名称匹配") || page.summary.contains("关联模块匹配")));
}

#[test]
fn mixed_local_fixture_filters_low_signal_key_sources_and_exports_relations() {
    let fixture = copy_fixture_to_temp("mixed-local-repo");
    let repo_root = fixture.path();

    run_init(repo_root).unwrap();

    let metadata = read_metadata(repo_root).unwrap();
    let web_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path.ends_with("核心模块/web.md"))
        .expect("web page should be exported");
    let spider_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path.ends_with("核心模块/spider.md"))
        .expect("spider page should be exported");
    let nginx_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path.ends_with("核心模块/nginx.md"))
        .expect("nginx page should be exported");

    assert!(web_item.summary.contains("web/src/main.ts"));
    assert!(!web_item.summary.contains("pnpm-lock.yaml"));
    assert!(spider_item.summary.contains("spider/app.py"));
    assert!(!spider_item.summary.contains("app.log.2026-03-07"));
    assert!(nginx_item.summary.contains("依赖模块：web"));

    let module_names = metadata
        .modules
        .iter()
        .map(|module| (module.id.clone(), module.name.clone()))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(metadata.relations.iter().any(|relation| {
        module_names.get(&relation.source_id).map(String::as_str) == Some("web")
            && module_names.get(&relation.target_id).map(String::as_str) == Some("spider")
            && relation.relation_type == "DEPENDS_ON"
    }));
    assert!(metadata.relations.iter().any(|relation| {
        module_names.get(&relation.source_id).map(String::as_str) == Some("conf")
            && module_names.get(&relation.target_id).map(String::as_str) == Some("web")
            && relation.relation_type == "SERVES_STATIC"
    }));

    let query = run_query(repo_root, "api").unwrap();
    assert!(query.matches.iter().all(|page| page.match_mode == "structure"));
    assert!(!query.matched_relations.is_empty());
}

fn copy_fixture_to_temp(name: &str) -> tempfile::TempDir {
    let source_root = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{name}")),
        PathBuf::from(format!("crates/wiki-core/tests/fixtures/{name}")),
        PathBuf::from(format!("tests/fixtures/{name}")),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .expect("fixture repository should exist");
    let temp = tempdir().unwrap();
    copy_dir_recursive(&source_root, temp.path()).unwrap();
    temp
}

fn copy_dir_recursive(source: &Path, target: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&target_path)?;
            copy_dir_recursive(&source_path, &target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &target_path)?;
        }
    }

    Ok(())
}
