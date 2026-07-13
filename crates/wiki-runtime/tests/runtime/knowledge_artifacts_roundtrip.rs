use std::fs;

use tempfile::tempdir;
use wiki_knowledge::domain::research::{
    ProjectionDigestStatus, ProjectionDigestStatusReason, ProjectionDigestStatusReasonKind,
};
use wiki_model::domain::knowledge_artifact::KnowledgeResearchSummaryStatus;
use wiki_model::domain::projection::ProjectionDigest;
use wiki_runtime::storage::knowledge_artifacts::{
    knowledge_artifacts_exist, load_conflict_records, load_declared_records, load_health_signals,
    load_knowledge_artifacts, persist_knowledge_artifacts, restore_runtime_cache_from_artifacts,
    PersistKnowledgeArtifactsInput,
};
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::workflows::{init::run_init, sync::run_sync};

use super::test_support::force_full_runtime;

const DECLARED_RUNTIME_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract kind=policy scope=repo status=active source=manual -->\n",
    "当前仓库必须先写 formal artifact，再谈 query。\n",
    "<!-- wiki:declared:end -->\n"
);

const DECLARED_RUNTIME_CHANGED_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=repo-runtime-contract kind=policy scope=repo status=deprecated deprecated=true source=manual -->\n",
    "当前仓库必须先写 formal artifact，再谈 query。\n",
    "<!-- wiki:declared:end -->\n"
);

fn set_declared_blocks_in_first_managed_section(
    repo_root: &std::path::Path,
    declared_blocks: &str,
) {
    let overview_path = repo_root.join(".wiki/INDEX.md");
    mark_first_managed_section_declared(&overview_path);
    let content = fs::read_to_string(&overview_path).unwrap();
    let marker = "<!-- wiki:managed:end";
    let pos = content
        .find(marker)
        .expect("should have managed end marker");

    let mut new_content = content[..pos]
        .replace(DECLARED_RUNTIME_BLOCK, "")
        .replace(DECLARED_RUNTIME_CHANGED_BLOCK, "");
    new_content.push_str(declared_blocks);
    new_content.push_str(&content[pos..]);
    fs::write(&overview_path, &new_content).unwrap();
}

fn mark_first_managed_section_declared(page_path: &std::path::Path) {
    let content = fs::read_to_string(page_path).unwrap();
    let marker = "<!-- wiki:managed:start";
    let start = content
        .find(marker)
        .expect("should have managed start marker");
    let end = content[start..].find('\n').unwrap() + start;
    let line = &content[start..end];
    let declared_line = line.replace("owner=derived_managed", "owner=declared_managed");
    if line == declared_line {
        assert!(line.contains("owner=declared_managed"));
        return;
    }

    let mut new_content = content[..start].to_string();
    new_content.push_str(&declared_line);
    new_content.push_str(&content[end..]);
    fs::write(page_path, &new_content).unwrap();
}

#[test]
fn knowledge_artifacts_roundtrip_preserves_declared_and_health_records() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-roundtrip-demo"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("src.ts"),
        "export const runtimeContract = true;\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();

    let before = load_knowledge_artifacts(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let conn = sqlite_store::open_db(repo_root).unwrap();
    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    let digest = before
        .page_digests
        .first()
        .expect("init should persist at least one page digest");
    let unit_id = digest.unit_id.clone();
    let generated_at = "2026-04-14T12:00:00Z".to_string();

    let declared_records = vec![
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord {
            record_id: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id("marker:repo-runtime-contract"),
            authoring_id: "marker:repo-runtime-contract".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScope {
                kind: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScopeKind::Repo,
                r#ref: "repo".to_string(),
                selectors: vec!["formal".to_string(), "runtime".to_string()],
            },
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: generated_at.clone(),
            unit_refs: vec![unit_id.clone()],
            projection_refs: vec![format!("page:{}:section:intro", digest.page_id)],
            page_id: digest.page_id.clone(),
            section_id: "section:intro".to_string(),
            ordinal: 0,
            body: "运行时约束必须经 formal artifacts 落盘。".to_string(),
        },
    ];
    let health_signals =
        vec![wiki_model::domain::knowledge_artifact::KnowledgeHealthSignal {
        signal_id: "health-projection-stale-demo".to_string(),
        signal_kind:
            wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::ProjectionStale,
        severity: wiki_model::domain::knowledge_artifact::KnowledgeHealthSeverity::Warning,
        target_ref: format!("unit:{unit_id}"),
        recommended_action:
            wiki_model::domain::knowledge_artifact::KnowledgeHealthRecommendedAction::Update,
        reason: "projection digest pending refresh".to_string(),
    }];

    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: "update",
        generated_at: &generated_at,
        facts_input_hash: &before.snapshot_manifest.facts_input_hash,
        metadata: &metadata,
        knowledge_tree: &before.knowledge_tree,
        declared_records: &declared_records,
        research_summaries: &before.research_summaries,
        page_digests: &before.page_digests,
        runtime_gates: &runtime_gates,
        health_signals: &health_signals,
    })
    .unwrap();

    assert!(knowledge_artifacts_exist(repo_root));

    let after = load_knowledge_artifacts(repo_root).unwrap();
    assert!(!after.projection_digests.is_empty());
    assert!(after
        .projection_digests
        .iter()
        .all(|digest: &ProjectionDigest| digest.validate().is_ok()));
    assert!(repo_root
        .join(".wiki/.knowledge/runtime/projection-digests.jsonl")
        .exists());
    let projection_refs = after
        .projection_digests
        .iter()
        .map(|digest| digest.projection_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        after.snapshot_manifest.projection_digest_refs,
        projection_refs
    );
    let projection_snapshot_id = wiki_index::fingerprint::fingerprint_bytes(
        serde_json::to_vec(&after.projection_digests)
            .unwrap()
            .as_slice(),
    );
    assert_eq!(
        after.snapshot_manifest.projection_snapshot_id,
        projection_snapshot_id
    );
    assert_eq!(after.declared_records, declared_records);
    assert_eq!(after.health_signals, health_signals);
    assert_eq!(load_declared_records(repo_root).unwrap(), declared_records);
    assert_eq!(load_health_signals(repo_root).unwrap(), health_signals);
    let research_summary = after
        .research_summaries
        .iter()
        .find(|summary| summary.unit_id == unit_id)
        .expect("target unit should keep research summary");
    assert_eq!(
        research_summary.summary_status,
        KnowledgeResearchSummaryStatus::Ready
    );
    assert_eq!(
        research_summary
            .source_refs
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>(),
        research_summary
            .key_sources
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>()
    );
    assert!(!research_summary.citation_refs.is_empty());

    let unit = after
        .units
        .iter()
        .find(|unit| unit.id == unit_id)
        .expect("target unit should survive roundtrip");
    let expected_research_ref = format!("research-summary:{unit_id}");
    assert_eq!(
        unit.declared_record_refs,
        vec![wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id(
            "marker:repo-runtime-contract"
        )]
    );
    assert_eq!(
        unit.derived_research_ref.as_deref(),
        Some(expected_research_ref.as_str())
    );
    assert!(unit
        .projection_refs
        .iter()
        .any(|value| value == &format!("page:{}", digest.page_id)));
    assert_eq!(
        unit.status,
        wiki_model::domain::knowledge::KnowledgeUnitStatus::Active
    );
    assert_eq!(unit.updated_at, generated_at);

    drop(conn);
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();
    assert!(
        restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn artifact_loader_selects_manifest_from_metadata_snapshot_pointer() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"snapshot-pointer-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const current = true;\n").unwrap();

    run_init(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let current_snapshot_id = metadata.current_snapshot_id.unwrap();
    let current = load_knowledge_artifacts(repo_root)
        .unwrap()
        .snapshot_manifest;
    assert_eq!(current.snapshot_id, current_snapshot_id);

    let mut stale = current.clone();
    stale.snapshot_id = "zzzz-stale-snapshot".to_string();
    stale.metadata_hash = "stale-metadata-hash".to_string();
    let stale_path =
        repo_root.join(".wiki/.knowledge/runtime/snapshots/zzzz-stale-snapshot/manifest.yaml");
    fs::create_dir_all(stale_path.parent().unwrap()).unwrap();
    fs::write(&stale_path, serde_yaml::to_string(&stale).unwrap()).unwrap();

    let loaded = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(loaded.snapshot_manifest.snapshot_id, current_snapshot_id);
}

#[test]
fn artifact_loader_rejects_manifest_identity_mismatch() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"snapshot-identity-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const current = true;\n").unwrap();

    run_init(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let current_snapshot_id = metadata.current_snapshot_id.unwrap();
    let manifest_path = repo_root
        .join(".wiki/.knowledge/runtime/snapshots")
        .join(&current_snapshot_id)
        .join("manifest.yaml");
    let mut manifest: wiki_model::domain::knowledge_artifact::CommittedSnapshotManifest =
        serde_yaml::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
    manifest.snapshot_id = "different-snapshot".to_string();
    fs::write(&manifest_path, serde_yaml::to_string(&manifest).unwrap()).unwrap();

    let error = load_knowledge_artifacts(repo_root).unwrap_err();
    assert!(error.to_string().contains("snapshot id mismatch"));
}

#[test]
fn restore_refuses_page_snapshot_drift_even_when_artifacts_exist() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-page-drift-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();
    let overview_path = repo_root.join(".wiki/INDEX.md");
    let original = fs::read_to_string(&overview_path).unwrap();
    fs::write(&overview_path, format!("{original}\n<!-- drift -->\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn restore_refuses_declared_page_drift_instead_of_rebuilding_truth_from_page() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-declared-page-drift-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();
    set_declared_blocks_in_first_managed_section(repo_root, DECLARED_RUNTIME_BLOCK);
    let sync = run_sync(repo_root).unwrap();
    assert_eq!(
        serde_json::to_value(sync).unwrap()["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(artifacts.declared_records.len(), 1);
    assert_eq!(
        artifacts.declared_records[0].status,
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active
    );

    set_declared_blocks_in_first_managed_section(repo_root, DECLARED_RUNTIME_CHANGED_BLOCK);
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );

    let persisted = load_knowledge_artifacts(repo_root).unwrap();
    assert_eq!(
        persisted.declared_records[0].status,
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active
    );
}

#[test]
fn restore_refuses_invalid_research_summary_snapshot() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-invalid-research-summary-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();

    let mut artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let target = artifacts
        .research_summaries
        .first_mut()
        .expect("init should persist at least one research summary");
    target.summary_status = KnowledgeResearchSummaryStatus::Blocked;
    target.status_reasons = Vec::new();

    let research_path = repo_root.join(".wiki/.knowledge/derived/research-summaries.jsonl");
    let research_jsonl = artifacts
        .research_summaries
        .iter()
        .map(|summary| serde_json::to_string(summary).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&research_path, format!("{research_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn restore_refuses_invalid_projection_digest_snapshot() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-invalid-projection-digest-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();

    let mut artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let target = artifacts
        .page_digests
        .first_mut()
        .expect("init should persist at least one page digest");
    target.projection_status = ProjectionDigestStatus::Blocked;
    target.status_reasons = Vec::new();

    let digest_path = repo_root.join(".wiki/.knowledge/runtime/page-digests.jsonl");
    let digest_jsonl = artifacts
        .page_digests
        .iter()
        .map(|digest| serde_json::to_string(digest).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&digest_path, format!("{digest_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn restore_refuses_invalid_model_projection_digest_snapshot() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-invalid-model-projection-digest-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();

    let mut artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let target = artifacts
        .projection_digests
        .first_mut()
        .expect("init should persist at least one model projection digest");
    target.status = wiki_model::domain::projection::ProjectionDigestStatus::Blocked;
    target.status_reasons = Vec::new();

    let digest_path = repo_root.join(".wiki/.knowledge/runtime/projection-digests.jsonl");
    let digest_jsonl = artifacts
        .projection_digests
        .iter()
        .map(|digest| serde_json::to_string(digest).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&digest_path, format!("{digest_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn restore_refuses_projection_digest_metadata_mismatch() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-projection-metadata-mismatch-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();

    let mut artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let target = artifacts
        .page_digests
        .first_mut()
        .expect("init should persist at least one page digest");
    target.projection_status = ProjectionDigestStatus::Stale;
    target.status_reasons = vec![ProjectionDigestStatusReason {
        reason_kind: Some(ProjectionDigestStatusReasonKind::PageSnapshotMismatch),
        reason_message: "page digest 与 metadata 标题不一致".to_string(),
        upstream_ref: Some(format!("page:{}", target.page_id)),
    }];
    target.title.push_str("-mismatch");

    let digest_path = repo_root.join(".wiki/.knowledge/runtime/page-digests.jsonl");
    let digest_jsonl = artifacts
        .page_digests
        .iter()
        .map(|digest| serde_json::to_string(digest).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&digest_path, format!("{digest_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn artifact_roundtrip_persists_declared_conflict_records() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-conflict-roundtrip-demo"}"#,
    )
    .unwrap();
    fs::write(
        repo_root.join("src.ts"),
        "export const runtimeContract = true;\n",
    )
    .unwrap();

    run_init(repo_root).unwrap();

    let before = load_knowledge_artifacts(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let conn = sqlite_store::open_db(repo_root).unwrap();
    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    let digest = before
        .page_digests
        .first()
        .expect("init should persist at least one page digest");
    let unit_id = digest.unit_id.clone();
    let generated_at = "2026-04-15T12:00:00Z".to_string();

    let declared_records = vec![
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord {
            record_id: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id("marker:repo-runtime-contract"),
            authoring_id: "marker:repo-runtime-contract".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScope {
                kind: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScopeKind::Repo,
                r#ref: "repo".to_string(),
                selectors: Vec::new(),
            },
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: generated_at.clone(),
            unit_refs: vec![unit_id.clone()],
            projection_refs: vec![format!("page:{}:section:intro", digest.page_id)],
            page_id: digest.page_id.clone(),
            section_id: "section:intro".to_string(),
            ordinal: 0,
            body: "第一条并行 policy".to_string(),
        },
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord {
            record_id: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id("marker:repo-runtime-contract-v2"),
            authoring_id: "marker:repo-runtime-contract-v2".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScope {
                kind: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScopeKind::Repo,
                r#ref: "repo".to_string(),
                selectors: Vec::new(),
            },
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: generated_at.clone(),
            unit_refs: vec![unit_id.clone()],
            projection_refs: vec![format!("page:{}:section:intro", digest.page_id)],
            page_id: digest.page_id.clone(),
            section_id: "section:intro".to_string(),
            ordinal: 1,
            body: "第二条并行 policy".to_string(),
        },
    ];

    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: "update",
        generated_at: &generated_at,
        facts_input_hash: &before.snapshot_manifest.facts_input_hash,
        metadata: &metadata,
        knowledge_tree: &before.knowledge_tree,
        declared_records: &declared_records,
        research_summaries: &before.research_summaries,
        page_digests: &before.page_digests,
        runtime_gates: &runtime_gates,
        health_signals: &[],
    })
    .unwrap();

    drop(conn);
    let conflicts = load_conflict_records(repo_root).unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(
        conflicts[0].conflict_kind.as_str(),
        "parallel_active_declared"
    );
    assert!(
        restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}

#[test]
fn restore_refuses_invalid_conflict_snapshot() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"artifact-invalid-conflict-demo"}"#,
    )
    .unwrap();
    fs::write(repo_root.join("src.ts"), "export const restore = true;\n").unwrap();

    run_init(repo_root).unwrap();

    let before = load_knowledge_artifacts(repo_root).unwrap();
    let metadata = read_metadata(repo_root).unwrap();
    let conn = sqlite_store::open_db(repo_root).unwrap();
    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    drop(conn);
    let digest = before.page_digests.first().unwrap();
    let unit_id = digest.unit_id.clone();
    let generated_at = "2026-04-15T12:30:00Z".to_string();
    let declared_records = vec![
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord {
            record_id: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id("marker:repo-runtime-contract"),
            authoring_id: "marker:repo-runtime-contract".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScope {
                kind: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScopeKind::Repo,
                r#ref: "repo".to_string(),
                selectors: Vec::new(),
            },
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: generated_at.clone(),
            unit_refs: vec![unit_id.clone()],
            projection_refs: vec![format!("page:{}:section:intro", digest.page_id)],
            page_id: digest.page_id.clone(),
            section_id: "section:intro".to_string(),
            ordinal: 0,
            body: "第一条并行 policy".to_string(),
        },
        wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord {
            record_id: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord::record_id_from_authoring_id("marker:repo-runtime-contract-v2"),
            authoring_id: "marker:repo-runtime-contract-v2".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScope {
                kind: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeScopeKind::Repo,
                r#ref: "repo".to_string(),
                selectors: Vec::new(),
            },
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
            relations: Vec::new(),
            source_ref: "manual".to_string(),
            updated_at: generated_at.clone(),
            unit_refs: vec![unit_id.clone()],
            projection_refs: vec![format!("page:{}:section:intro", digest.page_id)],
            page_id: digest.page_id.clone(),
            section_id: "section:intro".to_string(),
            ordinal: 1,
            body: "第二条并行 policy".to_string(),
        },
    ];

    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: "update",
        generated_at: &generated_at,
        facts_input_hash: &before.snapshot_manifest.facts_input_hash,
        metadata: &metadata,
        knowledge_tree: &before.knowledge_tree,
        declared_records: &declared_records,
        research_summaries: &before.research_summaries,
        page_digests: &before.page_digests,
        runtime_gates: &runtime_gates,
        health_signals: &[],
    })
    .unwrap();

    let mut artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let target = artifacts
        .conflict_records
        .first_mut()
        .expect("conflict snapshot should exist");
    target.record_ids = vec![
        "missing-record".to_string(),
        "another-missing-record".to_string(),
    ];

    let conflict_path = repo_root.join(".wiki/.knowledge/runtime/conflict-records.jsonl");
    let conflict_jsonl = artifacts
        .conflict_records
        .iter()
        .map(|conflict| serde_json::to_string(conflict).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&conflict_path, format!("{conflict_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(
        !restore_runtime_cache_from_artifacts(repo_root)
            .unwrap()
            .restored_cache
    );
}
