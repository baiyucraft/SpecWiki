use wiki_knowledge::declared_authority::{
    evaluate_declared_authority, reconcile_declared_governance_events,
};
use wiki_model::domain::knowledge_artifact::{
    validate_declared_record_snapshot, DeclaredAuthorityState, DeclaredGovernanceEventKind,
    DeclaredKnowledgeRecord, DeclaredKnowledgeRecordKind, DeclaredKnowledgeRecordStatus,
    DeclaredKnowledgeRelation, DeclaredKnowledgeRelationKind, DeclaredKnowledgeScope,
    DeclaredKnowledgeScopeKind, KnowledgeConflictRecord,
};

fn record(
    id: &str,
    status: DeclaredKnowledgeRecordStatus,
    relations: Vec<DeclaredKnowledgeRelation>,
) -> DeclaredKnowledgeRecord {
    DeclaredKnowledgeRecord {
        record_id: id.to_string(),
        authoring_id: id.to_string(),
        record_kind: DeclaredKnowledgeRecordKind::Policy,
        scope: DeclaredKnowledgeScope {
            kind: DeclaredKnowledgeScopeKind::Repo,
            r#ref: "repo".to_string(),
            selectors: Vec::new(),
        },
        status,
        relations,
        source_ref: format!("page:overview#{id}"),
        page_id: "page-overview".to_string(),
        section_id: "section-policy".to_string(),
        body: id.to_string(),
        ..DeclaredKnowledgeRecord::default()
    }
}

fn relation(
    relation_kind: DeclaredKnowledgeRelationKind,
    target: Option<&str>,
) -> DeclaredKnowledgeRelation {
    DeclaredKnowledgeRelation {
        relation_kind,
        target_record_ref: target.map(str::to_string),
    }
}

#[test]
fn declared_snapshot_accepts_canonical_multi_generation_replacement_chain() {
    let records = vec![
        record(
            "a",
            DeclaredKnowledgeRecordStatus::Superseded,
            vec![relation(
                DeclaredKnowledgeRelationKind::ReplacedBy,
                Some("b"),
            )],
        ),
        record(
            "b",
            DeclaredKnowledgeRecordStatus::Superseded,
            vec![
                relation(DeclaredKnowledgeRelationKind::Supersedes, Some("a")),
                relation(DeclaredKnowledgeRelationKind::ReplacedBy, Some("c")),
            ],
        ),
        record(
            "c",
            DeclaredKnowledgeRecordStatus::Replaced,
            vec![relation(
                DeclaredKnowledgeRelationKind::Supersedes,
                Some("b"),
            )],
        ),
    ];

    let validation = validate_declared_record_snapshot(&records);
    assert!(
        validation.is_ok(),
        "A -> B -> C must be a valid replacement chain with C as the authority head: {validation:?}"
    );

    let decisions = evaluate_declared_authority(&records).expect("chain authority");
    assert_eq!(decisions.len(), 1);
    assert_eq!(decisions[0].authority_state, DeclaredAuthorityState::Unique);
    assert_eq!(decisions[0].head_record_refs, vec!["c".to_string()]);
}

#[test]
fn authority_evaluator_distinguishes_explicit_none_from_parallel_conflict() {
    let deprecated = vec![record(
        "a",
        DeclaredKnowledgeRecordStatus::Deprecated,
        vec![relation(DeclaredKnowledgeRelationKind::Deprecated, None)],
    )];
    let none = evaluate_declared_authority(&deprecated).expect("deprecated authority");
    assert_eq!(none[0].authority_state, DeclaredAuthorityState::None);
    assert!(none[0].head_record_refs.is_empty());

    let parallel = vec![
        record("a", DeclaredKnowledgeRecordStatus::Active, Vec::new()),
        record("b", DeclaredKnowledgeRecordStatus::Active, Vec::new()),
    ];
    let conflict = evaluate_declared_authority(&parallel).expect("parallel authority");
    assert_eq!(
        conflict[0].authority_state,
        DeclaredAuthorityState::Conflict
    );
    assert_eq!(conflict[0].head_record_refs.len(), 2);
}

#[test]
fn governance_event_history_is_append_only_and_noop_sync_is_idempotent() {
    let unique_records = vec![record(
        "a",
        DeclaredKnowledgeRecordStatus::Active,
        Vec::new(),
    )];
    let conflict_records = vec![
        unique_records[0].clone(),
        record("b", DeclaredKnowledgeRecordStatus::Active, Vec::new()),
    ];
    let unique = evaluate_declared_authority(&unique_records).unwrap();
    let conflict = evaluate_declared_authority(&conflict_records).unwrap();
    let open_conflict = KnowledgeConflictRecord {
        conflict_id: "conflict-repo-policy".to_string(),
        scope: unique_records[0].scope.clone(),
        record_ids: vec!["a".to_string(), "b".to_string()],
        authoring_ids: vec!["a".to_string(), "b".to_string()],
        reason: "parallel heads".to_string(),
        ..KnowledgeConflictRecord::default()
    };

    let opened = reconcile_declared_governance_events(
        &[],
        &unique,
        &conflict,
        &unique_records,
        &conflict_records,
        &[],
        std::slice::from_ref(&open_conflict),
        "snapshot-1",
        "snapshot-2",
        "2026-07-16T00:00:00Z",
    )
    .unwrap();
    let resolved = reconcile_declared_governance_events(
        &opened,
        &conflict,
        &unique,
        &conflict_records,
        &unique_records,
        std::slice::from_ref(&open_conflict),
        &[],
        "snapshot-2",
        "snapshot-3",
        "2026-07-16T00:01:00Z",
    )
    .unwrap();
    let noop = reconcile_declared_governance_events(
        &resolved,
        &unique,
        &unique,
        &unique_records,
        &unique_records,
        &[],
        &[],
        "snapshot-3",
        "snapshot-3",
        "2026-07-16T00:02:00Z",
    )
    .unwrap();
    assert_eq!(noop, resolved);
    let reopened = reconcile_declared_governance_events(
        &noop,
        &unique,
        &conflict,
        &unique_records,
        &conflict_records,
        &[],
        std::slice::from_ref(&open_conflict),
        "snapshot-3",
        "snapshot-4",
        "2026-07-16T00:03:00Z",
    )
    .unwrap();

    let conflict_events = reopened
        .iter()
        .filter_map(|event| match event.event_kind {
            DeclaredGovernanceEventKind::ConflictOpened
            | DeclaredGovernanceEventKind::ConflictResolved => Some(event.event_kind),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        conflict_events,
        vec![
            DeclaredGovernanceEventKind::ConflictOpened,
            DeclaredGovernanceEventKind::ConflictResolved,
            DeclaredGovernanceEventKind::ConflictOpened,
        ]
    );
    assert_eq!(
        reopened
            .iter()
            .map(|event| event.event_id.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        reopened.len()
    );
}
