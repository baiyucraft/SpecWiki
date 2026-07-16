use tempfile::tempdir;

use crate::core_scenario_support::{
    create_formal_restore_pair, dispatch, insert_declared_blocks, replace_declared_blocks,
    route_results, route_tags, snapshot_manifest_path, write_core_scenario_repo, write_repo_file,
};
use wiki_runtime::storage::knowledge_artifacts::load_knowledge_artifacts;
use wiki_runtime::storage::metadata_store::read_metadata;

const DECLARED_KNOWLEDGE_BLOCKS: &str = concat!(
    "\n<!-- wiki:declared id=payments-policy kind=policy scope=module:payments status=active source=manual -->\n",
    "payments changes require contract tests.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=query-convention kind=convention scope=repo status=active source=manual -->\n",
    "query results require source evidence.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=payment-pitfall kind=pitfall scope=source:packages/payments/src/shared.ts status=active source=manual -->\n",
    "payment rounding must avoid floating point drift.\n",
    "<!-- wiki:declared:end -->\n",
);

const DECLARED_CONFLICT_BLOCKS: &str = concat!(
    "\n<!-- wiki:declared id=runtime-policy-a kind=policy scope=repo status=active source=manual -->\n",
    "formal artifact must be written before query.\n",
    "<!-- wiki:declared:end -->\n",
    "\n<!-- wiki:declared id=runtime-policy-b kind=policy scope=repo status=active source=manual -->\n",
    "formal artifact must be independently approved before query.\n",
    "<!-- wiki:declared:end -->\n",
);

const DECLARED_RESOLVED_BLOCK: &str = concat!(
    "\n<!-- wiki:declared id=runtime-policy-a kind=policy scope=repo status=active source=manual -->\n",
    "formal artifact must be written before query.\n",
    "<!-- wiki:declared:end -->\n",
);

#[test]
fn canonical_core_scenarios_stay_within_public_contract() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_core_scenario_repo(repo_root);

    let init = dispatch(repo_root, "init", None);
    assert_eq!(init["state"], "fresh");
    assert!(repo_root.join(".wiki/wiki.metadata.json").exists());
    assert!(repo_root.join(".wiki/.knowledge").exists());

    let initial_status = dispatch(repo_root, "status", None);
    assert_eq!(initial_status["state"], "fresh");
    assert_eq!(initial_status["recommended_action"], "none");

    let symbol_query = dispatch(repo_root, "query", Some("handleCheckout"));
    assert!(symbol_query.get("route_groups").is_some());
    assert!(symbol_query.get("answer").is_some());
    assert!(symbol_query.get("query_trust").is_some());
    assert!(symbol_query.get("recommended_action").is_some());
    for internal_field in [
        "matched_symbols",
        "matched_sources",
        "matched_modules",
        "matched_symbol_edges",
        "matches",
        "provenance_summary",
    ] {
        assert!(
            symbol_query.get(internal_field).is_none(),
            "internal query field leaked: {internal_field}"
        );
    }
    let symbol_routes = route_tags(&symbol_query);
    assert!(symbol_routes.contains("index_symbol_hit"));
    assert!(symbol_routes.contains("index_graph_hit"));
    for result in route_results(&symbol_query).into_iter().filter(|result| {
        matches!(
            result["route_tag"].as_str(),
            Some("index_symbol_hit" | "index_graph_hit")
        )
    }) {
        assert!(result.get("provenance").is_some());
        assert!(result["source_refs"]
            .as_array()
            .is_some_and(|refs| !refs.is_empty()));
    }

    let module_query = dispatch(repo_root, "query", Some("payments"));
    let module_routes = route_tags(&module_query);
    assert!(module_routes.contains("index_path_hit"));
    assert!(module_routes.contains("index_module_hit"));

    write_repo_file(
        repo_root,
        "packages/payments/src/shared.ts",
        "export function finalizePayment() { return false; }\n",
    );
    let stale_status = dispatch(repo_root, "status", None);
    assert_eq!(stale_status["state"], "needs_update");
    assert_eq!(stale_status["recommended_action"], "update");
    assert!(stale_status["affected_knowledge_scope"]["direct_unit_ids"]
        .as_array()
        .is_some_and(|ids| !ids.is_empty()));

    let update = dispatch(repo_root, "update", None);
    assert_eq!(update["state"], "fresh");
    assert_eq!(update["previous_state"], "stale");
    assert!(update["updated_pages"]
        .as_array()
        .is_some_and(|pages| !pages.is_empty()));
    let final_status = dispatch(repo_root, "status", None);
    assert_eq!(final_status["state"], "fresh");
    assert_eq!(final_status["recommended_action"], "none");
}

#[test]
fn declared_knowledge_and_structured_conflict_are_traceable() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_core_scenario_repo(repo_root);
    dispatch(repo_root, "init", None);
    insert_declared_blocks(repo_root, DECLARED_KNOWLEDGE_BLOCKS);

    let sync = dispatch(repo_root, "sync", None);
    assert_eq!(
        sync["page_outcomes"][0]["result_kind"],
        "declared_writeback"
    );
    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let kinds = artifacts
        .declared_records
        .iter()
        .map(|record| record.record_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        kinds,
        std::collections::BTreeSet::from(["policy", "convention", "pitfall"])
    );
    let declared_update = dispatch(repo_root, "update", None);
    assert_eq!(declared_update["state"], "fresh");
    for term in ["contract tests", "source evidence", "floating point drift"] {
        let query = dispatch(repo_root, "query", Some(term));
        assert!(route_tags(&query).contains("knowledge_declared_hit"));
    }

    let invalid_fixture = tempdir().unwrap();
    write_core_scenario_repo(invalid_fixture.path());
    dispatch(invalid_fixture.path(), "init", None);
    let duplicate_blocks = concat!(
        "\n<!-- wiki:declared id=duplicate-policy kind=policy scope=repo status=active source=manual -->\n",
        "first duplicate.\n<!-- wiki:declared:end -->\n",
        "\n<!-- wiki:declared id=duplicate-policy kind=policy scope=repo status=active source=manual -->\n",
        "second duplicate.\n<!-- wiki:declared:end -->\n",
    );
    insert_declared_blocks(invalid_fixture.path(), duplicate_blocks);
    let invalid_sync = dispatch(invalid_fixture.path(), "sync", None);
    assert_eq!(
        invalid_sync["page_outcomes"][0]["result_kind"],
        "illegal_drift"
    );
    assert!(load_knowledge_artifacts(invalid_fixture.path())
        .unwrap()
        .declared_records
        .is_empty());

    let conflict_fixture = tempdir().unwrap();
    write_core_scenario_repo(conflict_fixture.path());
    dispatch(conflict_fixture.path(), "init", None);
    insert_declared_blocks(conflict_fixture.path(), DECLARED_CONFLICT_BLOCKS);
    dispatch(conflict_fixture.path(), "sync", None);
    let conflict_artifacts = load_knowledge_artifacts(conflict_fixture.path()).unwrap();
    assert_eq!(conflict_artifacts.conflict_records.len(), 1);
    let conflict = &conflict_artifacts.conflict_records[0];
    assert_eq!(conflict.conflict_kind.as_str(), "parallel_active_declared");
    assert_eq!(conflict.record_ids.len(), 2);
    assert_eq!(conflict.authoring_ids.len(), 2);
    assert!(!conflict.unit_refs.is_empty());
    assert!(!conflict.projection_refs.is_empty());
    dispatch(conflict_fixture.path(), "sync", None);
    assert_eq!(
        load_knowledge_artifacts(conflict_fixture.path())
            .unwrap()
            .conflict_records
            .len(),
        1
    );

    let conflict_query = dispatch(conflict_fixture.path(), "query", Some("formal artifact"));
    assert_eq!(conflict_query["answer"]["answer_mode"], "degraded");
    assert_eq!(conflict_query["answer"]["answer_trust"], "constrained");
    assert_eq!(conflict_query["recommended_action"], "review_governance");
    assert_eq!(
        conflict_query["answer"]["recommended_action"],
        "review_governance"
    );
    assert!(conflict_query["answer"]["supporting_refs"]
        .as_array()
        .is_some_and(|refs| !refs.is_empty()));

    replace_declared_blocks(
        conflict_fixture.path(),
        DECLARED_CONFLICT_BLOCKS,
        DECLARED_RESOLVED_BLOCK,
    );
    dispatch(conflict_fixture.path(), "sync", None);
    let resolved = load_knowledge_artifacts(conflict_fixture.path()).unwrap();
    assert!(resolved.conflict_records.is_empty());
}

#[test]
fn formal_artifacts_restore_second_worktree_to_level1() {
    let (source_fixture, restored_fixture) = create_formal_restore_pair();
    let source_repo = source_fixture.path();
    let restored_repo = restored_fixture.path();

    assert!(!restored_repo.join(".wiki/.cache").exists());
    assert_eq!(
        std::fs::read(snapshot_manifest_path(source_repo)).unwrap(),
        std::fs::read(snapshot_manifest_path(restored_repo)).unwrap()
    );
    let status = dispatch(restored_repo, "status", None);
    assert_eq!(status["state"], "fresh", "status = {status:#}");
    assert_eq!(status["readiness"]["knowledge"], "ready");
    assert_eq!(status["readiness"]["projection"], "ready");
    assert_eq!(status["readiness"]["index"], "missing");
    assert_eq!(status["readiness"]["fusion"], "degraded");
    assert_eq!(status["readiness"]["restored_level"], "level1");
    assert_eq!(status["recommended_action"], "rebuild");
    assert!(restored_repo.join(".wiki/.cache/wiki-cache.db").exists());

    let query = dispatch(restored_repo, "query", Some("payments"));
    assert!(route_tags(&query)
        .iter()
        .all(|route_tag| !route_tag.starts_with("index_")));
}

#[test]
fn formal_restore_rejects_page_drift_and_reports_source_drift() {
    let (_page_source_fixture, page_drift_fixture) = create_formal_restore_pair();

    let metadata = read_metadata(page_drift_fixture.path()).unwrap();
    let page_path = page_drift_fixture.path().join(
        &metadata
            .wiki_items
            .first()
            .expect("formal metadata declares pages")
            .path,
    );
    let mut drifted_page = std::fs::read_to_string(&page_path).unwrap();
    drifted_page.push_str("\nmanual drift outside committed snapshot\n");
    std::fs::write(page_path, drifted_page).unwrap();

    let page_status = dispatch(page_drift_fixture.path(), "status", None);
    assert_eq!(page_status["state"], "needs_rebuild");
    assert_eq!(page_status["recommended_action"], "rebuild");
    assert_eq!(page_status["readiness"]["fusion"], "blocked");
    assert!(page_status["readiness"]["reasons"]
        .as_array()
        .is_some_and(|reasons| reasons.iter().any(|reason| reason == "page_hash_mismatch")));

    let (_source_fixture, source_drift_fixture) = create_formal_restore_pair();
    write_repo_file(
        source_drift_fixture.path(),
        "packages/payments/src/shared.ts",
        "export function finalizePayment() { return false; }\n",
    );

    let source_status = dispatch(source_drift_fixture.path(), "status", None);
    assert_eq!(source_status["state"], "needs_update");
    assert_eq!(source_status["recommended_action"], "update");
    assert_eq!(source_status["readiness"]["restored_level"], "level1");
    let update = dispatch(source_drift_fixture.path(), "update", None);
    assert_eq!(update["state"], "fresh");
}
