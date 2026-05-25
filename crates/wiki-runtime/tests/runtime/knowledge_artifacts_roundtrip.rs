use std::fs;

use tempfile::tempdir;
use wiki_knowledge::domain::research::{
    ProjectionDigestStatus, ProjectionDigestStatusReason, ProjectionDigestStatusReasonKind,
};
use wiki_model::domain::knowledge_artifact::KnowledgeResearchSummaryStatus;
use wiki_runtime::storage::knowledge_artifacts::{
    knowledge_artifacts_exist, load_conflict_records, load_declared_records,
    load_health_signals, load_knowledge_artifacts, persist_knowledge_artifacts,
    restore_runtime_cache_from_artifacts, PersistKnowledgeArtifactsInput,
};
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::workflows::init::run_init;

use super::test_support::force_full_runtime;

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
        facts_input_hash: &before.recovery_manifest.facts_input_hash,
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
    assert!(restore_runtime_cache_from_artifacts(repo_root).unwrap());
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
    let overview_path = repo_root.join(".wiki/项目概述.md");
    let original = fs::read_to_string(&overview_path).unwrap();
    fs::write(&overview_path, format!("{original}\n<!-- drift -->\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(!restore_runtime_cache_from_artifacts(repo_root).unwrap());
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

    assert!(!restore_runtime_cache_from_artifacts(repo_root).unwrap());
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

    assert!(!restore_runtime_cache_from_artifacts(repo_root).unwrap());
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

    assert!(!restore_runtime_cache_from_artifacts(repo_root).unwrap());
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
        facts_input_hash: &before.recovery_manifest.facts_input_hash,
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
    assert_eq!(conflicts[0].conflict_kind.as_str(), "parallel_active_declared");
    assert!(restore_runtime_cache_from_artifacts(repo_root).unwrap());
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
        facts_input_hash: &before.recovery_manifest.facts_input_hash,
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
    target.record_ids = vec!["missing-record".to_string(), "another-missing-record".to_string()];

    let conflict_path = repo_root.join(".wiki/.knowledge/runtime/conflict-records.jsonl");
    let conflict_jsonl = artifacts
        .conflict_records
        .iter()
        .map(|conflict| serde_json::to_string(conflict).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&conflict_path, format!("{conflict_jsonl}\n")).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    assert!(!restore_runtime_cache_from_artifacts(repo_root).unwrap());
}
