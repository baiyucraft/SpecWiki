use wiki_model::domain::query::{
    QueryConfidence, QueryProvenance, QueryProvenanceLayer, QueryProvenanceState,
    QueryRankingBasis, QueryRefKind, QueryResultDto, QueryRouteGroup, QueryRouteTag,
    QueryScoreDirection, QuerySourceRef, RecommendedAction,
};

#[test]
fn query_result_dto_serializes_public_contract_fields() {
    let result = QueryResultDto {
        route_tag: QueryRouteTag::IndexPathHit,
        ref_kind: QueryRefKind::SourcePath,
        ref_id: "src/lib.rs".to_string(),
        label: "src/lib.rs".to_string(),
        rank: 1,
        score: Some(0.91),
        provenance: QueryProvenance {
            layer: QueryProvenanceLayer::Index,
            state: QueryProvenanceState::Ready,
            reason: None,
        },
        confidence: QueryConfidence::High,
        recommended_action: RecommendedAction::OpenSourceRef,
        source_refs: vec![QuerySourceRef {
            ref_kind: QueryRefKind::SourcePath,
            ref_id: "src/lib.rs".to_string(),
            label: Some("src/lib.rs".to_string()),
            path: Some("src/lib.rs".to_string()),
            start_line: Some(1),
            end_line: Some(3),
            provenance: vec!["index:files_fts".to_string()],
            diagnostics: Vec::new(),
        }],
    };

    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value["route_tag"], "index_path_hit");
    assert_eq!(value["ref_kind"], "source_path");
    assert_eq!(value["ref_id"], "src/lib.rs");
    assert_eq!(value["label"], "src/lib.rs");
    assert_eq!(value["score"], 0.91);
    assert_eq!(value["provenance"]["layer"], "index");
    assert_eq!(value["provenance"]["state"], "ready");
    assert_eq!(value["confidence"], "high");
    assert_eq!(value["recommended_action"], "open_source_ref");
    assert_eq!(value["source_refs"][0]["ref_kind"], "source_path");
    assert_eq!(value["source_refs"][0]["ref_id"], "src/lib.rs");
    assert_eq!(value["source_refs"][0]["path"], "src/lib.rs");
    assert_eq!(value["source_refs"][0]["start_line"], 1);
    assert_eq!(value["source_refs"][0]["end_line"], 3);
    assert_eq!(value["source_refs"][0]["provenance"][0], "index:files_fts");
}

#[test]
fn query_route_tag_and_ref_kind_reject_unknown_values() {
    let route_err = serde_json::from_str::<QueryRouteTag>(r#""index_hit""#).unwrap_err();
    assert!(route_err.to_string().contains("unknown variant"));

    let ref_kind_err = serde_json::from_str::<QueryRefKind>(r#""mystery_ref""#).unwrap_err();
    assert!(ref_kind_err.to_string().contains("unknown variant"));
}

#[test]
fn query_group_serializes_ranking_and_module_contract() {
    let result = QueryResultDto {
        route_tag: QueryRouteTag::IndexModuleHit,
        ref_kind: QueryRefKind::SourceModule,
        ref_id: "module:payments".to_string(),
        label: "payments".to_string(),
        rank: 1,
        score: None,
        provenance: QueryProvenance {
            layer: QueryProvenanceLayer::Index,
            state: QueryProvenanceState::Ready,
            reason: Some("module name matched".to_string()),
        },
        confidence: QueryConfidence::High,
        recommended_action: RecommendedAction::OpenSourceRef,
        source_refs: Vec::new(),
    };
    let group = QueryRouteGroup {
        route_tag: QueryRouteTag::IndexModuleHit,
        ranking_basis: QueryRankingBasis::StructuralMatch,
        score_direction: QueryScoreDirection::None,
        total_count: 1,
        returned_count: 1,
        truncated: false,
        results: vec![result],
    };

    let value = serde_json::to_value(group).unwrap();
    assert_eq!(value["route_tag"], "index_module_hit");
    assert_eq!(value["ranking_basis"], "structural_match");
    assert_eq!(value["score_direction"], "none");
    assert_eq!(value["total_count"], 1);
    assert_eq!(value["returned_count"], 1);
    assert_eq!(value["truncated"], false);
    assert_eq!(value["results"][0]["ref_kind"], "source_module");
    assert_eq!(value["results"][0]["rank"], 1);
    assert!(value["results"][0].get("score").is_none());
    assert_eq!(value["results"][0]["provenance"]["layer"], "index");
    assert_eq!(value["results"][0]["provenance"]["state"], "ready");
}
