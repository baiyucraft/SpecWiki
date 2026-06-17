use wiki_model::domain::projection::{
    ProjectionBinding, ProjectionContractError, ProjectionDigest, ProjectionDigestStatus,
    ProjectionStatusReason, ProjectionStatusReasonKind, SectionBinding, SectionOwnership,
    SyncResultKind,
};

fn sample_section_binding() -> SectionBinding {
    SectionBinding {
        section_id: "section:intro".to_string(),
        owner_kind: Some(SectionOwnership::DerivedManaged),
        knowledge_refs: vec!["unit:repo".to_string()],
        source_refs: vec!["src/lib.rs".to_string()],
        input_hash: "input-hash".to_string(),
        content_hash: "content-hash".to_string(),
        projection_status: ProjectionDigestStatus::Ready,
        projection_digest_ref: Some("projection:repo".to_string()),
    }
}

fn sample_projection_digest() -> ProjectionDigest {
    ProjectionDigest {
        projection_id: "projection:repo".to_string(),
        page_id: "page:repo".to_string(),
        section_ids: vec!["section:intro".to_string()],
        knowledge_refs: vec!["unit:repo".to_string()],
        source_refs: vec!["src/lib.rs".to_string()],
        input_hash: "input-hash".to_string(),
        renderer_version: "renderer:v2".to_string(),
        content_digest: "content-digest".to_string(),
        section_hashes: vec![("section:intro".to_string(), "content-hash".to_string())],
        status: ProjectionDigestStatus::Ready,
        status_reasons: Vec::new(),
    }
}

#[test]
fn section_binding_requires_owner_refs_and_hashes() {
    let valid = sample_section_binding();
    assert!(valid.validate().is_ok());

    let missing_owner = SectionBinding {
        owner_kind: None,
        ..sample_section_binding()
    };
    assert!(matches!(
        missing_owner.validate(),
        Err(ProjectionContractError::MissingOwner { .. })
    ));

    let missing_hash = SectionBinding {
        content_hash: String::new(),
        ..sample_section_binding()
    };
    assert!(matches!(
        missing_hash.validate(),
        Err(ProjectionContractError::MissingContentHash { .. })
    ));

    let missing_refs = SectionBinding {
        knowledge_refs: Vec::new(),
        source_refs: Vec::new(),
        ..sample_section_binding()
    };
    assert!(matches!(
        missing_refs.validate(),
        Err(ProjectionContractError::MissingBindingRefs { .. })
    ));
}

#[test]
fn projection_digest_status_requires_reasons_when_not_ready() {
    let ready = sample_projection_digest();
    assert!(ready.validate().is_ok());

    let blocked_without_reason = ProjectionDigest {
        status: ProjectionDigestStatus::Blocked,
        ..sample_projection_digest()
    };
    assert!(matches!(
        blocked_without_reason.validate(),
        Err(ProjectionContractError::MissingStatusReason { .. })
    ));

    let blocked = ProjectionDigest {
        status: ProjectionDigestStatus::Blocked,
        status_reasons: vec![ProjectionStatusReason {
            reason_kind: Some(ProjectionStatusReasonKind::MetadataBindingMismatch),
            reason_message: "metadata binding mismatch".to_string(),
            upstream_ref: Some("page:repo".to_string()),
        }],
        ..sample_projection_digest()
    };
    assert!(blocked.validate().is_ok());
}

#[test]
fn projection_binding_requires_page_and_recovery_refs() {
    let valid = ProjectionBinding {
        projection_id: "projection:repo".to_string(),
        page_id: "page:repo".to_string(),
        section_ids: vec!["section:intro".to_string()],
        knowledge_refs: vec!["unit:repo".to_string()],
        source_refs: vec!["src/lib.rs".to_string()],
        snapshot_id: "snapshot:one".to_string(),
        digest_ref: Some("projection:repo".to_string()),
    };
    assert!(valid.validate().is_ok());

    let missing_refs = ProjectionBinding {
        section_ids: Vec::new(),
        knowledge_refs: Vec::new(),
        ..valid
    };
    assert!(matches!(
        missing_refs.validate(),
        Err(ProjectionContractError::MissingProjectionBinding { .. })
    ));
}

#[test]
fn projection_contract_enums_use_snake_case_json() {
    let owner = serde_json::to_string(&SectionOwnership::ProjectionStatic).unwrap();
    assert_eq!(owner, "\"projection_static\"");
    let parsed_owner: SectionOwnership = serde_json::from_str(&owner).unwrap();
    assert_eq!(parsed_owner, SectionOwnership::ProjectionStatic);

    let result_kind = serde_json::to_string(&SyncResultKind::DeclaredWriteback).unwrap();
    assert_eq!(result_kind, "\"declared_writeback\"");
    let parsed_kind: SyncResultKind = serde_json::from_str(&result_kind).unwrap();
    assert_eq!(parsed_kind, SyncResultKind::DeclaredWriteback);
}
