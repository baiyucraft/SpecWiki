use wiki_model::domain::query::{
    QueryConfidence, QueryProvenance, QueryRefKind, QueryResultDto, QueryRouteTag,
    QuerySourceRef, RecommendedAction,
};

#[test]
fn query_result_dto_serializes_public_contract_fields() {
    let result = QueryResultDto {
        route_tag: QueryRouteTag::IndexPathHit,
        ref_kind: QueryRefKind::SourcePath,
        ref_id: "src/lib.rs".to_string(),
        label: "src/lib.rs".to_string(),
        score: 0.91,
        provenance: QueryProvenance {
            layer: "index".to_string(),
            state: Some("ready".to_string()),
            reason: None,
        },
        confidence: QueryConfidence::High,
        recommended_action: RecommendedAction::OpenSourceRef,
        source_refs: vec![QuerySourceRef {
            ref_kind: QueryRefKind::SourcePath,
            ref_id: "src/lib.rs".to_string(),
            label: Some("src/lib.rs".to_string()),
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
}

#[test]
fn query_route_tag_and_ref_kind_reject_unknown_values() {
    let route_err = serde_json::from_str::<QueryRouteTag>(r#""index_hit""#).unwrap_err();
    assert!(route_err.to_string().contains("unknown variant"));

    let ref_kind_err = serde_json::from_str::<QueryRefKind>(r#""mystery_ref""#).unwrap_err();
    assert!(ref_kind_err.to_string().contains("unknown variant"));
}
