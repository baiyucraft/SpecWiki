use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;
use tempfile::{tempdir, TempDir};
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::transport::dto::CoreCommand;

pub(crate) fn write_repo_file(repo_root: &Path, relative_path: &str, content: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

pub(crate) fn dispatch(repo_root: &Path, action: &str, term: Option<&str>) -> Value {
    let response = wiki_runtime::transport::cli::dispatch(CoreCommand {
        action: action.to_string(),
        repo_root: Some(repo_root.display().to_string()),
        term: term.map(str::to_string),
        change_id: None,
        archive_mode: None,
        archive_operation_id: None,
        bootstrap: None,
        development_mode: true,
        stream_progress: false,
        llm_bridge: None,
    });
    assert!(response.ok, "{action} failed: {:?}", response.error);
    response
        .data
        .expect("successful transport response has data")
}

pub(crate) fn write_core_scenario_repo(repo_root: &Path) {
    write_repo_file(
        repo_root,
        "package.json",
        r#"{"name":"core-scenario-demo","private":true,"workspaces":["packages/*"]}"#,
    );
    write_repo_file(
        repo_root,
        "packages/payments/package.json",
        r#"{"name":"@demo/payments"}"#,
    );
    write_repo_file(
        repo_root,
        "packages/payments/src/shared.ts",
        "export function finalizePayment() { return true; }\n",
    );
    write_repo_file(
        repo_root,
        "packages/payments/src/service.ts",
        concat!(
            "import { finalizePayment } from \"./shared\";\n",
            "export function runPayment() {\n",
            "  return finalizePayment();\n",
            "}\n",
        ),
    );
    write_repo_file(
        repo_root,
        "packages/payments/src/controller.ts",
        concat!(
            "import { runPayment } from \"./service\";\n",
            "export function handleCheckout() {\n",
            "  return runPayment();\n",
            "}\n",
        ),
    );
}

pub(crate) fn insert_declared_blocks(repo_root: &Path, declared_blocks: &str) {
    let page_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&page_path).unwrap();
    let start = content
        .find("<!-- wiki:managed:start")
        .expect("overview has a managed section");
    let start_line_end = content[start..].find('\n').unwrap() + start;
    let start_line = &content[start..start_line_end];
    let declared_start_line = start_line.replace("owner=derived_managed", "owner=declared_managed");
    let mut declared_content = content[..start].to_string();
    declared_content.push_str(&declared_start_line);
    declared_content.push_str(&content[start_line_end..]);

    let end = declared_content
        .find("<!-- wiki:managed:end")
        .expect("overview has a managed end marker");
    let mut updated = declared_content[..end].to_string();
    updated.push_str(declared_blocks);
    updated.push_str(&declared_content[end..]);
    fs::write(page_path, updated).unwrap();
}

pub(crate) fn replace_declared_blocks(repo_root: &Path, previous: &str, next: &str) {
    let page_path = repo_root.join(".wiki/INDEX.md");
    let content = fs::read_to_string(&page_path).unwrap();
    assert!(content.contains(previous));
    fs::write(page_path, content.replacen(previous, next, 1)).unwrap();
}

pub(crate) fn copy_dir_recursive(source: &Path, target: &Path) {
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            fs::create_dir_all(&target_path).unwrap();
            copy_dir_recursive(&source_path, &target_path);
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(source_path, target_path).unwrap();
        }
    }
}

pub(crate) fn copy_formal_artifacts(source_repo: &Path, target_repo: &Path) {
    let source_wiki = source_repo.join(".wiki");
    let target_wiki = target_repo.join(".wiki");
    fs::create_dir_all(&target_wiki).unwrap();
    copy_dir_recursive(
        &source_wiki.join(".knowledge"),
        &target_wiki.join(".knowledge"),
    );
    fs::copy(
        source_wiki.join("wiki.metadata.json"),
        target_wiki.join("wiki.metadata.json"),
    )
    .unwrap();

    let metadata = read_metadata(source_repo).unwrap();
    for item in metadata.wiki_items {
        let source_path = source_repo.join(&item.path);
        let target_path = target_repo.join(&item.path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::copy(source_path, target_path).unwrap();
    }
}

pub(crate) fn snapshot_manifest_path(repo_root: &Path) -> std::path::PathBuf {
    let metadata = read_metadata(repo_root).unwrap();
    let snapshot_id = metadata
        .current_snapshot_id
        .expect("formal metadata has current snapshot id");
    repo_root
        .join(".wiki/.knowledge/runtime/snapshots")
        .join(snapshot_id)
        .join("manifest.yaml")
}

pub(crate) fn create_formal_restore_pair() -> (TempDir, TempDir) {
    let source_fixture = tempdir().unwrap();
    let restored_fixture = tempdir().unwrap();
    write_core_scenario_repo(source_fixture.path());
    write_core_scenario_repo(restored_fixture.path());
    dispatch(source_fixture.path(), "init", None);
    copy_formal_artifacts(source_fixture.path(), restored_fixture.path());
    (source_fixture, restored_fixture)
}

pub(crate) fn route_results(payload: &Value) -> Vec<&Value> {
    payload["route_groups"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|group| group["results"].as_array().into_iter().flatten())
        .collect()
}

pub(crate) fn route_tags(payload: &Value) -> BTreeSet<&str> {
    route_results(payload)
        .into_iter()
        .filter_map(|result| result["route_tag"].as_str())
        .collect()
}
