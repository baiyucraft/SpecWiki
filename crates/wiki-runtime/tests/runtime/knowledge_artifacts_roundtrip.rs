use std::fs;

use tempfile::tempdir;
use wiki_runtime::storage::knowledge_artifacts::{
    knowledge_artifacts_exist, load_declared_records, load_health_signals,
    load_knowledge_artifacts, persist_knowledge_artifacts, restore_runtime_cache_from_artifacts,
    PersistKnowledgeArtifactsInput,
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
            record_id: "declared-policy-repo-overview".to_string(),
            record_kind:
                wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordKind::Policy,
            scope_ref: "repo".to_string(),
            status: wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecordStatus::Active,
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
    assert!(!research_summary.summary_status.is_empty());
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
        vec!["declared-policy-repo-overview".to_string()]
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
