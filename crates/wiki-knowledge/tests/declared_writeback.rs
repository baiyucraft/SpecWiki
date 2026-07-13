use wiki_knowledge::declared_writeback::{
    validate_declared_writeback, DeclaredAuthoringCandidate, DeclaredSnapshot,
    DeclaredWritebackDecision,
};
use wiki_model::domain::knowledge_artifact::{
    DeclaredKnowledgeRecordKind, DeclaredKnowledgeRecordStatus, DeclaredKnowledgeScope,
    DeclaredKnowledgeScopeKind,
};

fn empty_declared_snapshot() -> DeclaredSnapshot {
    DeclaredSnapshot::default()
}

fn candidate() -> DeclaredAuthoringCandidate {
    DeclaredAuthoringCandidate {
        authoring_id: "marker:runtime-policy".to_string(),
        record_kind: DeclaredKnowledgeRecordKind::Policy,
        scope: DeclaredKnowledgeScope {
            kind: DeclaredKnowledgeScopeKind::Module,
            r#ref: "runtime".to_string(),
            selectors: Vec::new(),
        },
        status: DeclaredKnowledgeRecordStatus::Active,
        relations: Vec::new(),
        source_ref: "manual".to_string(),
        page_id: "page:runtime".to_string(),
        section_id: "section:runtime-policy".to_string(),
        body: "runtime contract".to_string(),
        baseline_hash: "old".to_string(),
        current_hash: "new".to_string(),
        marker_version: 2,
        metadata_binding_ref: Some("binding:runtime-policy".to_string()),
        unit_refs: vec!["unit:runtime".to_string()],
    }
}

#[test]
fn validate_declared_candidate_normalizes_scope_and_lifecycle() {
    let decision = validate_declared_writeback(&candidate(), &empty_declared_snapshot());

    let DeclaredWritebackDecision::Accepted(patch) = decision else {
        panic!("expected accepted declared writeback");
    };
    assert_eq!(patch.record.authoring_id, "marker:runtime-policy");
    assert_eq!(patch.record.scope.canonical_key(), "module:runtime");
    assert_eq!(patch.record.status, DeclaredKnowledgeRecordStatus::Active);
    assert_eq!(
        patch.affected_projection_refs,
        vec!["page:runtime:section:section:runtime-policy"]
    );
}

#[test]
fn validate_declared_candidate_rejects_conflicting_scope() {
    let mut snapshot_record =
        match validate_declared_writeback(&candidate(), &empty_declared_snapshot()) {
            DeclaredWritebackDecision::Accepted(patch) => patch.record,
            _ => panic!("expected accepted seed"),
        };
    snapshot_record.scope.r#ref = "other-runtime".to_string();
    let snapshot = DeclaredSnapshot {
        records: vec![snapshot_record],
    };

    let decision = validate_declared_writeback(&candidate(), &snapshot);

    assert!(matches!(
        decision,
        DeclaredWritebackDecision::Conflict { .. }
    ));
}
