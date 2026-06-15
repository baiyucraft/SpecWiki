use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;
use wiki_runtime::domain::steering::SteeringLoadMode;
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::workflows::progress::NoopProgressSink;
use wiki_runtime::workflows::{
    init::run_init_with_progress_and_llm_as_with_mode, query::run_query,
};

fn run_init_in_development(repo_root: &Path) {
    let mut sink = NoopProgressSink;
    run_init_with_progress_and_llm_as_with_mode(
        "init",
        repo_root,
        &mut sink,
        None,
        SteeringLoadMode::Development,
    )
    .unwrap();
}

#[test]
fn baseline_fixture_generates_hierarchical_wiki_and_structured_query_results() {
    let fixture = copy_fixture_to_temp("baseline-hierarchy-repo");
    let repo_root = fixture.path();

    run_init_in_development(repo_root);

    assert!(repo_root.join(".wiki/INDEX.md").exists());
    assert!(repo_root.join(".wiki/00-项目总览/00-系统架构.md").exists());

    let metadata = read_metadata(repo_root).unwrap();
    assert!(metadata
        .wiki_items
        .iter()
        .all(|item| wiki_model::domain::knowledge::is_official_wiki_relative_path(&item.path)));
    assert!(
        metadata
            .wiki_items
            .iter()
            .any(|item| item.item_type == "module"),
        "should have at least one module-type page"
    );

    let auth_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.title.contains("auth"))
        .expect("auth page should be exported to metadata");
    let architecture_item = metadata
        .wiki_items
        .iter()
        .find(|item| item.path == ".wiki/00-项目总览/00-系统架构.md")
        .expect("architecture page should be exported to metadata");
    assert!(!auth_item.module_ids.is_empty());
    assert!(auth_item
        .source_files
        .iter()
        .any(|path| path == "packages/domain/auth/src/index.ts"));
    assert!(architecture_item.parent_id.is_some());
    assert!(!architecture_item.ancestor_ids.is_empty());

    let query = run_query(repo_root, "auth").unwrap();
    assert!(query.matched_modules.iter().any(|module| {
        module
            .root_paths
            .iter()
            .any(|root| root == "packages/domain/auth")
    }));
}

#[test]
fn mixed_local_fixture_filters_low_signal_key_sources_and_exports_relations() {
    let fixture = copy_fixture_to_temp("mixed-local-repo");
    let repo_root = fixture.path();

    run_init_in_development(repo_root);

    let metadata = read_metadata(repo_root).unwrap();
    assert!(
        metadata
            .wiki_items
            .iter()
            .any(|item| item.title.contains("web")),
        "web page should be exported"
    );
    assert!(
        metadata
            .wiki_items
            .iter()
            .any(|item| item.title.contains("spider")),
        "spider page should be exported"
    );

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
    assert!(!query.matched_relations.is_empty());
    assert!(query
        .matched_relations
        .iter()
        .any(|relation| relation.relation_type == "DEPENDS_ON"));
    assert!(query
        .matched_relations
        .iter()
        .all(|relation| !relation.reasons.is_empty()));
}

fn copy_fixture_to_temp(name: &str) -> tempfile::TempDir {
    let source_root = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{name}")),
        PathBuf::from(format!("crates/wiki-runtime/tests/fixtures/{name}")),
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
