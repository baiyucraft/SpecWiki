use std::fs;

use tempfile::tempdir;
use wiki_model::domain::governance::GovernanceReadiness;
use wiki_model::domain::query::QueryRouteTag;
use wiki_runtime::domain::runtime_profile::{QueryTrust, RecommendedAction};
use wiki_runtime::domain::steering::SteeringLoadMode;
use wiki_runtime::storage::state_store::read_state;
use wiki_runtime::workflows::governance::GovernanceService;
use wiki_runtime::workflows::init::run_init_with_progress_and_llm_as_with_mode;
use wiki_runtime::workflows::progress::NoopProgressSink;
use wiki_runtime::workflows::query::run_query;
use wiki_runtime::workflows::status::run_status;
use wiki_runtime::workflows::update::run_update_with_progress_and_llm_as_with_mode;

fn write_ready_change(root: &std::path::Path, body: &str) {
    let change_dir = root.join(".spec/changes/demo-change");
    fs::create_dir_all(&change_dir).unwrap();
    fs::create_dir_all(root.join(".spec/archive")).unwrap();
    fs::write(
        change_dir.join("meta.yaml"),
        "id: demo-change\nstage: design\ndeliveryShape: single-change\n",
    )
    .unwrap();
    fs::write(change_dir.join("proposal.md"), body).unwrap();
    fs::write(change_dir.join("design.md"), "# design\n").unwrap();
}

fn run_development_init(root: &std::path::Path) {
    let mut sink = NoopProgressSink;
    run_init_with_progress_and_llm_as_with_mode(
        "init",
        root,
        &mut sink,
        None,
        SteeringLoadMode::Development,
    )
    .unwrap();
}

fn run_development_update(root: &std::path::Path) -> wiki_runtime::workflows::update::UpdateReport {
    let mut sink = NoopProgressSink;
    run_update_with_progress_and_llm_as_with_mode(
        "update",
        root,
        &mut sink,
        None,
        SteeringLoadMode::Development,
    )
    .unwrap()
}

#[test]
fn governance_service_tracks_live_and_cached_freshness() {
    let repo = tempdir().unwrap();
    let service = GovernanceService::new(repo.path());
    assert_eq!(
        service.status().unwrap().readiness,
        GovernanceReadiness::NotEnabled
    );

    write_ready_change(repo.path(), "# first\n");
    assert_eq!(
        service.status().unwrap().readiness,
        GovernanceReadiness::Stale
    );
    assert_eq!(
        service.refresh().unwrap().readiness,
        GovernanceReadiness::Ready
    );
    assert_eq!(
        service.status().unwrap().readiness,
        GovernanceReadiness::Ready
    );

    fs::write(
        repo.path().join(".spec/changes/demo-change/proposal.md"),
        "# second\n",
    )
    .unwrap();
    assert_eq!(
        service.status().unwrap().readiness,
        GovernanceReadiness::Stale
    );
    assert!(service.validate_change("demo-change").unwrap().valid);
}

#[test]
fn governance_service_queries_only_fresh_structured_refs() {
    let repo = tempdir().unwrap();
    write_ready_change(repo.path(), "# body-only-secret\n");
    let service = GovernanceService::new(repo.path());
    service.refresh().unwrap();

    let results = service.query_refs("demo-change", 10).unwrap();
    assert!(results
        .iter()
        .any(|result| result.route_tag == QueryRouteTag::GovernanceSummaryHit));
    assert!(results
        .iter()
        .any(|result| result.route_tag == QueryRouteTag::GovernanceEvidenceRef));
    assert!(results
        .iter()
        .all(|result| !result.label.contains("body-only-secret")));

    fs::write(
        repo.path().join(".spec/changes/demo-change/proposal.md"),
        "# changed-secret\n",
    )
    .unwrap();
    assert!(service.query_refs("demo-change", 10).unwrap().is_empty());
}

#[test]
fn blocked_governance_without_cache_returns_live_diagnostic_refs() {
    let repo = tempdir().unwrap();
    write_ready_change(repo.path(), "# proposal\n");
    fs::remove_file(repo.path().join(".spec/changes/demo-change/design.md")).unwrap();

    let service = GovernanceService::new(repo.path());
    assert_eq!(
        service.status().unwrap().readiness,
        GovernanceReadiness::Blocked
    );
    let refs = service.query_refs("demo-change", 10).unwrap();
    assert!(refs
        .iter()
        .any(|result| result.route_tag == QueryRouteTag::GovernanceEvidenceRef));
    assert!(refs.iter().any(|result| !result.source_refs.is_empty()));
    assert!(refs.iter().all(|result| result
        .source_refs
        .iter()
        .all(|source| !source.diagnostics.is_empty())));
}

#[test]
fn status_and_query_expose_the_single_governance_summary() {
    let repo = tempdir().unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"governance-demo"}"#,
    )
    .unwrap();
    fs::write(repo.path().join("src.ts"), "export const ready = true;\n").unwrap();

    let status = run_status(repo.path()).unwrap();
    assert_eq!(status.governance.readiness, GovernanceReadiness::NotEnabled);

    let query = run_query(repo.path(), "").unwrap();
    assert_eq!(query.governance.readiness, GovernanceReadiness::NotEnabled);
    let value = serde_json::to_value(query).unwrap();
    assert!(value.get("governance").is_some());
    assert!(value.get("governance_readiness").is_none());
}

#[test]
fn spec_only_delta_refreshes_governance_without_dirtying_core_runtime() {
    let repo = tempdir().unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"governance-demo"}"#,
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(
        repo.path().join("src/index.ts"),
        "export const ready = true;\n",
    )
    .unwrap();
    write_ready_change(repo.path(), "# first\n");

    run_development_init(repo.path());
    GovernanceService::new(repo.path()).refresh().unwrap();
    let core_before = serde_json::to_value(read_state(repo.path()).unwrap()).unwrap();

    fs::write(
        repo.path().join(".spec/changes/demo-change/proposal.md"),
        "# second\n",
    )
    .unwrap();
    assert_eq!(
        run_status(repo.path()).unwrap().governance.readiness,
        GovernanceReadiness::Stale
    );

    let update = run_development_update(repo.path());
    assert_eq!(update.governance.readiness, GovernanceReadiness::Ready);
    assert!(update.updated_pages.is_empty());
    assert_eq!(
        serde_json::to_value(read_state(repo.path()).unwrap()).unwrap(),
        core_before
    );
}

#[test]
fn query_merges_only_fresh_governance_refs() {
    let repo = tempdir().unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"governance-demo"}"#,
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(
        repo.path().join("src/index.ts"),
        "export const ready = true;\n",
    )
    .unwrap();
    write_ready_change(repo.path(), "# body-only-secret\n");
    run_development_init(repo.path());
    GovernanceService::new(repo.path()).refresh().unwrap();

    let ready = run_query(repo.path(), "demo-change").unwrap();
    assert!(ready
        .results
        .iter()
        .any(|result| result.route_tag == QueryRouteTag::GovernanceSummaryHit));
    assert!(!ready.answer.supporting_refs.is_empty());

    fs::write(
        repo.path().join(".spec/changes/demo-change/proposal.md"),
        "# changed-secret\n",
    )
    .unwrap();
    let stale = run_query(repo.path(), "demo-change").unwrap();
    assert_eq!(stale.governance.readiness, GovernanceReadiness::Stale);
    assert!(stale.results.iter().all(|result| {
        !matches!(
            result.route_tag,
            QueryRouteTag::GovernanceSummaryHit | QueryRouteTag::GovernanceEvidenceRef
        )
    }));
}

#[test]
fn governance_blocker_does_not_degrade_core_query_trust() {
    let repo = tempdir().unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"governance-demo"}"#,
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(
        repo.path().join("src/index.ts"),
        "export const ready = true;\n",
    )
    .unwrap();
    write_ready_change(repo.path(), "# proposal\n");
    run_development_init(repo.path());

    fs::remove_file(repo.path().join(".spec/changes/demo-change/design.md")).unwrap();
    GovernanceService::new(repo.path()).refresh().unwrap();

    let query = run_query(repo.path(), "ready").unwrap();
    assert_eq!(query.governance.readiness, GovernanceReadiness::Blocked);
    assert_eq!(query.query_trust, QueryTrust::Ready);
    assert_eq!(
        query.recommended_action,
        RecommendedAction::ReviewGovernance
    );
    assert_eq!(query.answer.recommended_action, RecommendedAction::None);

    let governance_query = run_query(repo.path(), "demo-change").unwrap();
    assert_eq!(
        governance_query.answer.recommended_action,
        RecommendedAction::ReviewGovernance
    );
    assert!(!governance_query.answer.supporting_refs.is_empty());
}
