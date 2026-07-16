use tempfile::tempdir;
use wiki_runtime::domain::checkpoint::UnitRuntimeGate;
use wiki_runtime::domain::knowledge::{DomainType, KnowledgeDomain, KnowledgeUnit, UnitType};
use wiki_runtime::storage::sqlite_store;

#[test]
fn unit_runtime_gate_crud_roundtrips_and_clears() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    let conn = sqlite_store::open_db(repo_root).unwrap();

    let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心运行时");
    let unit = KnowledgeUnit::new(
        UnitType::ModuleDoc,
        "运行时",
        domain.id.clone(),
        "核心运行时/运行时.md",
    );
    sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
    sqlite_store::write_knowledge_units(&conn, std::slice::from_ref(&unit)).unwrap();

    sqlite_store::write_unit_runtime_gate(
        &conn,
        &UnitRuntimeGate {
            unit_id: unit.id.clone(),
            unit_type: unit.unit_type.as_str().to_string(),
            research_status: "ready".to_string(),
            compose_status: "blocked".to_string(),
            assemble_status: "pending".to_string(),
            last_ready_stage: Some("research_unit".to_string()),
            blocked_reason: Some("missing child rollup".to_string()),
            missing_dependencies: vec!["unit:child-a".to_string()],
            updated_at: "123".to_string(),
        },
    )
    .unwrap();

    let gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    assert_eq!(gates.len(), 1);
    assert_eq!(gates[0].unit_id, unit.id);
    assert_eq!(gates[0].compose_status, "blocked");
    assert_eq!(
        gates[0].missing_dependencies,
        vec!["unit:child-a".to_string()]
    );

    sqlite_store::clear_unit_runtime_gates(&conn).unwrap();
    assert!(sqlite_store::read_unit_runtime_gates(&conn)
        .unwrap()
        .is_empty());
}
