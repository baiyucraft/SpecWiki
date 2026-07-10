use wiki_model::domain::governance::{
    GovernanceArtifactRef, GovernanceArtifactStatus, GovernanceBlockingIssue,
    GovernanceChangeSummary, GovernanceGateStatus, GovernanceGateSummary, GovernanceIssueSeverity,
    GovernanceLocation, GovernanceReadiness, GovernanceRecommendedAction, GovernanceRuleResult,
    GovernanceSummary, GovernanceValidationResult,
};

fn governance_artifact() -> GovernanceArtifactRef {
    GovernanceArtifactRef {
        change_id: "governance-isolation".to_string(),
        kind: "proposal".to_string(),
        relative_path: ".spec/changes/governance-isolation/proposal.md".to_string(),
        status: GovernanceArtifactStatus::Present,
        content_hash: Some("blake3:proposal".to_string()),
    }
}

fn governance_issue() -> GovernanceBlockingIssue {
    let artifact = governance_artifact();
    let issue = GovernanceBlockingIssue {
        rule_id: "governance.warning.fixture".to_string(),
        severity: GovernanceIssueSeverity::Warning,
        message: "fixture warning".to_string(),
        change_id: Some("governance-isolation".to_string()),
        artifact_ref: Some(artifact.clone()),
        recommended_action: GovernanceRecommendedAction::ReviewGovernance,
    };

    issue
}

fn governance_summary() -> GovernanceSummary {
    GovernanceSummary {
        readiness: GovernanceReadiness::Ready,
        fingerprint: Some("blake3:evidence".to_string()),
        active_count: 1,
        archived_count: 0,
        issues: vec![governance_issue()],
        recommended_action: GovernanceRecommendedAction::ReviewGovernance,
    }
}

#[test]
fn governance_contract_serializes_closed_product_dto() {
    let summary = governance_summary();
    let value = serde_json::to_value(&summary).unwrap();

    assert_eq!(value["readiness"], "ready");
    assert_eq!(value["issues"][0]["severity"], "warning");
    assert_eq!(value["recommended_action"], "review_governance");
    assert!(value.get("governance_readiness").is_none());

    let roundtrip: GovernanceSummary = serde_json::from_value(value).unwrap();
    assert_eq!(roundtrip, summary);
}

#[test]
fn governance_contract_serializes_change_and_artifact_refs() {
    let change = GovernanceChangeSummary {
        id: "governance-isolation".to_string(),
        location: GovernanceLocation::Active,
        stage: "implementation".to_string(),
        role: Some("child".to_string()),
        parent: Some("governance".to_string()),
        order: Some(1),
        depends_on: vec!["governance-model".to_string()],
        gate: GovernanceGateSummary {
            artifact_gate: GovernanceGateStatus::Passed,
            review_gate: GovernanceGateStatus::Pending,
            verification_gate: GovernanceGateStatus::NotApplicable,
            archive_readiness: GovernanceGateStatus::Pending,
        },
    };

    let change_value = serde_json::to_value(&change).unwrap();
    assert_eq!(change_value["location"], "active");
    assert_eq!(change_value["gate"]["artifact_gate"], "passed");
    assert_eq!(
        serde_json::from_value::<GovernanceChangeSummary>(change_value).unwrap(),
        change
    );

    let artifact = governance_artifact();
    let artifact_value = serde_json::to_value(&artifact).unwrap();
    assert_eq!(artifact_value["status"], "present");
    assert_eq!(
        serde_json::from_value::<GovernanceArtifactRef>(artifact_value).unwrap(),
        artifact
    );
}

#[test]
fn governance_contract_roundtrips_validation_result() {
    let issue = governance_issue();
    let result = GovernanceValidationResult {
        valid: false,
        readiness: GovernanceReadiness::Blocked,
        rule_results: vec![GovernanceRuleResult {
            rule_id: "governance.artifact.required".to_string(),
            passed: false,
            severity: GovernanceIssueSeverity::Blocking,
            message: Some("proposal.md is missing".to_string()),
            artifact_ref: issue.artifact_ref.clone(),
        }],
        issues: vec![issue],
    };

    let value = serde_json::to_value(&result).unwrap();
    assert_eq!(value["readiness"], "blocked");
    assert_eq!(value["rule_results"][0]["severity"], "blocking");
    assert!(!value["rule_results"][0]["passed"].as_bool().unwrap());
    assert_eq!(
        serde_json::from_value::<GovernanceValidationResult>(value).unwrap(),
        result
    );
}

#[test]
fn governance_contract_rejects_unknown_closed_enum_values() {
    assert!(serde_json::from_str::<GovernanceReadiness>(r#""unknown""#).is_err());
    assert!(serde_json::from_str::<GovernanceIssueSeverity>(r#""fatal""#).is_err());
    assert!(serde_json::from_str::<GovernanceLocation>(r#""deleted""#).is_err());
    assert!(serde_json::from_str::<GovernanceArtifactStatus>(r#""invalid""#).is_err());
    assert!(serde_json::from_str::<GovernanceGateStatus>(r#""ready""#).is_err());
    assert!(serde_json::from_str::<GovernanceRecommendedAction>(r#""rebuild""#).is_err());
}
