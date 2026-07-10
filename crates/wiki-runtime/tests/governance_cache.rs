use serde_json::json;
use tempfile::tempdir;
use wiki_runtime::storage::sqlite::governance_store::{
    GovernanceCacheSnapshot, SqliteGovernanceCache,
};
use wiki_runtime::storage::sqlite_store;

fn snapshot(fingerprint: &str, label: &str) -> GovernanceCacheSnapshot {
    GovernanceCacheSnapshot {
        schema_version: "1".to_string(),
        policy_version: "unispec-0.1.0".to_string(),
        evidence_fingerprint: fingerprint.to_string(),
        summary: json!({"readiness": "ready", "label": label}),
        changes: vec![json!({"id": "demo", "label": label})],
        artifact_refs: vec![json!({"change_id": "demo", "path": "proposal.md"})],
        issues: Vec::new(),
    }
}

#[test]
fn governance_cache_is_fingerprint_and_version_bound() {
    let repo = tempdir().unwrap();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let first = snapshot("fp-1", "first");

    SqliteGovernanceCache::new(&mut conn)
        .replace_snapshot(&first)
        .unwrap();

    let loaded = SqliteGovernanceCache::new(&mut conn)
        .read_snapshot("fp-1", "1", "unispec-0.1.0")
        .unwrap()
        .unwrap();
    assert_eq!(loaded, first);
    assert!(SqliteGovernanceCache::new(&mut conn)
        .read_snapshot("fp-2", "1", "unispec-0.1.0")
        .unwrap()
        .is_none());
    assert!(SqliteGovernanceCache::new(&mut conn)
        .read_snapshot("fp-1", "2", "unispec-0.1.0")
        .unwrap()
        .is_none());
    assert!(SqliteGovernanceCache::new(&mut conn)
        .read_snapshot("fp-1", "1", "policy-2")
        .unwrap()
        .is_none());
}

#[test]
fn governance_cache_failed_replace_keeps_previous_snapshot_and_runtime_tables() {
    let repo = tempdir().unwrap();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    sqlite_store::runtime_meta_set(&conn, "unrelated", "preserved").unwrap();

    let first = snapshot("fp-1", "first");
    SqliteGovernanceCache::new(&mut conn)
        .replace_snapshot(&first)
        .unwrap();

    conn.execute_batch(
        "CREATE TRIGGER fail_governance_changes_insert
         BEFORE INSERT ON governance_changes
         BEGIN
             SELECT RAISE(ABORT, 'forced governance cache failure');
         END;",
    )
    .unwrap();

    let second = snapshot("fp-2", "second");
    assert!(SqliteGovernanceCache::new(&mut conn)
        .replace_snapshot(&second)
        .is_err());
    conn.execute_batch("DROP TRIGGER fail_governance_changes_insert;")
        .unwrap();

    let loaded = SqliteGovernanceCache::new(&mut conn)
        .read_snapshot("fp-1", "1", "unispec-0.1.0")
        .unwrap()
        .unwrap();
    assert_eq!(loaded, first);
    assert_eq!(
        sqlite_store::runtime_meta_get(&conn, "unrelated").unwrap(),
        Some("preserved".to_string())
    );
}
