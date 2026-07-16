use std::fs;

use tempfile::tempdir;
use wiki_model::domain::knowledge_artifact::{
    DeclaredAuthoringState, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordStatus,
};
use wiki_model::domain::projection::{
    PageLinkRef, ProjectionAction, ProjectionEligibility, ProjectionLifecycle, SectionOwnership,
};
use wiki_runtime::generation::managed_sections::{parse_wiki_page, SectionBindingIndex};
use wiki_runtime::storage::knowledge_artifacts::load_knowledge_artifacts;
use wiki_runtime::storage::wiki_fs::resolve_page_path;
use wiki_runtime::workflows::projection_governance::reconcile_retiring_page;
use wiki_runtime::workflows::{init::run_init, status::run_status, update::run_update};

use super::test_support::force_full_runtime;

#[test]
fn projection_removal_blocks_on_protected_sections_and_manual_links() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("src/runtime")).unwrap();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"projection-protection"}"#,
    )
    .unwrap();
    for index in 0..8 {
        fs::write(
            repo_root.join(format!("src/runtime/module_{index}.ts")),
            format!("export const value_{index} = {index};\n"),
        )
        .unwrap();
    }

    run_init(repo_root).unwrap();
    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let retiring = artifacts
        .projection_decisions
        .iter()
        .find(|decision| decision.eligibility == ProjectionEligibility::Selected)
        .expect("fixture must contain a selected leaf page");
    let page_path = resolve_page_path(repo_root, &format!(".wiki/{}", retiring.relative_path));
    let original = fs::read_to_string(&page_path).unwrap();
    let clean = parse_wiki_page(&original, &SectionBindingIndex::default());
    assert_eq!(
        reconcile_retiring_page(&retiring.page_id, &clean, &[], &[]).action,
        ProjectionAction::Remove
    );
    let authority = DeclaredKnowledgeRecord {
        record_id: "declared-protected".to_string(),
        authoring_id: "protected".to_string(),
        page_id: retiring.page_id.clone(),
        status: DeclaredKnowledgeRecordStatus::Active,
        authoring_state: DeclaredAuthoringState::Bound,
        ..DeclaredKnowledgeRecord::default()
    };
    assert_eq!(
        reconcile_retiring_page(
            &retiring.page_id,
            &clean,
            std::slice::from_ref(&authority),
            &[]
        )
        .action,
        ProjectionAction::Block
    );
    for owner in [
        SectionOwnership::ManualUnmanaged,
        SectionOwnership::DerivedManaged,
    ] {
        let inbound = PageLinkRef {
            source_page_id: "source-page".to_string(),
            source_section_id: None,
            source_owner: owner,
            target_page_id: retiring.page_id.clone(),
            target_path: format!(".wiki/{}", retiring.relative_path),
            content_hash: "source-content".to_string(),
        };
        assert_eq!(
            reconcile_retiring_page(
                &retiring.page_id,
                &clean,
                &[],
                std::slice::from_ref(&inbound)
            )
            .action,
            ProjectionAction::Block
        );
    }
    fs::write(
        &page_path,
        format!("{original}\n\n## Manual runbook\n\nKeep this operator note.\n"),
    )
    .unwrap();
    fs::write(
        repo_root.join(".wiki/config.yaml"),
        format!(
            "version: 1\npages:\n  exclude:\n    - unit_ref: {}\n  max_projected_leaf_pages_per_domain: 5\n",
            retiring.unit_ref
        ),
    )
    .unwrap();
    fs::write(
        repo_root.join("src/runtime/module_0.ts"),
        "export const value_0 = 100;\n",
    )
    .unwrap();

    run_update(repo_root).unwrap();

    assert!(
        page_path.is_file(),
        "manual content must block page removal"
    );
    assert!(
        fs::read_to_string(&page_path)
            .unwrap()
            .contains("Keep this operator note."),
        "runtime must not rewrite protected manual text"
    );
    let after = load_knowledge_artifacts(repo_root).unwrap();
    let retiring_after = after
        .projection_decisions
        .iter()
        .find(|decision| decision.page_id == retiring.page_id)
        .unwrap();
    assert_eq!(retiring_after.lifecycle, ProjectionLifecycle::Retiring);
    assert_eq!(retiring_after.action, ProjectionAction::Block);
    let status = serde_json::to_value(run_status(repo_root).unwrap()).unwrap();
    assert_eq!(status["recommended_action"], "review_governance");
    assert_ne!(status["readiness"]["projection"], "ready");
}

#[test]
fn clean_projection_demotion_removes_page_without_dangling_links() {
    let (_env_lock, _index_only) = force_full_runtime();
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();

    fs::create_dir_all(repo_root.join("src/runtime")).unwrap();
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"projection-clean-demotion"}"#,
    )
    .unwrap();
    for index in 0..8 {
        fs::write(
            repo_root.join(format!("src/runtime/module_{index}.ts")),
            format!("export const value_{index} = {index};\n"),
        )
        .unwrap();
    }

    run_init(repo_root).unwrap();
    let before = load_knowledge_artifacts(repo_root).unwrap();
    let retiring = before
        .projection_decisions
        .iter()
        .find(|decision| decision.eligibility == ProjectionEligibility::Selected)
        .unwrap()
        .clone();
    let page_path = resolve_page_path(repo_root, &format!(".wiki/{}", retiring.relative_path));
    fs::write(
        repo_root.join(".wiki/config.yaml"),
        format!(
            "version: 1\npages:\n  exclude:\n    - unit_ref: {}\n  max_projected_leaf_pages_per_domain: 5\n",
            retiring.unit_ref
        ),
    )
    .unwrap();
    fs::write(
        repo_root.join("src/runtime/module_0.ts"),
        "export const value_0 = 100;\n",
    )
    .unwrap();

    run_update(repo_root).unwrap();

    assert!(!page_path.exists());
    let after = load_knowledge_artifacts(repo_root).unwrap();
    let retired = after
        .projection_decisions
        .iter()
        .find(|decision| decision.page_id == retiring.page_id)
        .unwrap();
    assert_eq!(retired.lifecycle, ProjectionLifecycle::Retired);
    assert_eq!(retired.action, ProjectionAction::Remove);
    assert!(after
        .page_link_refs
        .iter()
        .all(|link_ref| link_ref.target_page_id != retiring.page_id));
}
