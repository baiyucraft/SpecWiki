use wiki_runtime::domain::metadata::WikiMetadata;
use wiki_runtime::domain::metadata_mapper::{export_metadata, ExportContext};
use wiki_runtime::domain::state::{
    BuildState, SourceState, WikiPageState, WikiSectionState, WikiState,
};
use wiki_runtime::domain::metadata::DirtyState;

#[test]
fn metadata_roundtrip_keeps_dirty_state() {
    let metadata = WikiMetadata::sample();
    let json = serde_json::to_string_pretty(&metadata).unwrap();
    let decoded: WikiMetadata = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.dirty_state.status, metadata.dirty_state.status);
    assert_eq!(
        decoded.dirty_state.needs_rebuild_reason,
        metadata.dirty_state.needs_rebuild_reason
    );
    assert_eq!(decoded.wiki_items.len(), 1);
    assert_eq!(decoded.source_files.len(), 1);
}

#[test]
fn metadata_mapper_builds_section_reverse_refs() {
    let state = WikiState {
        pages: vec![WikiPageState {
            page_id: "page:repo".to_string(),
            title: "Repo".to_string(),
            path: ".wiki/INDEX.md".to_string(),
            page_type: "index".to_string(),
            parent_id: None,
            ancestor_ids: Vec::new(),
            input_hash: "input".to_string(),
            content_hash: "page-hash".to_string(),
            source_ids: vec!["source:lib".to_string()],
            source_paths: vec!["src/lib.rs".to_string()],
            module_ids: Vec::new(),
            summary: "summary".to_string(),
            provenance: Vec::new(),
            section_anchors: vec!["section:intro".to_string()],
            sections: vec![WikiSectionState {
                section_id: "section:intro".to_string(),
                title: "Intro".to_string(),
                managed: true,
                content_hash: "section-hash".to_string(),
                generated_content_hash: Some("section-hash".to_string()),
                anchor_after_section_id: None,
                anchor_before_section_id: None,
                source_ids: vec!["source:lib".to_string()],
                relation_ids: Vec::new(),
                owner_kind: Some(wiki_model::domain::projection::SectionOwnership::DerivedManaged),
                knowledge_refs: vec!["unit:repo".to_string()],
                projection_digest_ref: Some("projection:repo".to_string()),
                input_hash: "section-input".to_string(),
            }],
        }],
        sources: vec![SourceState {
            source_id: "source:lib".to_string(),
            path: "src/lib.rs".to_string(),
            fingerprint: "source-hash".to_string(),
            page_ids: vec!["page:repo".to_string()],
            module_ids: Vec::new(),
        }],
        modules: Vec::new(),
        relations: Vec::new(),
        dirty_state: DirtyState::fresh(),
        build_state: BuildState {
            generated_at: "2026-06-17T00:00:00Z".to_string(),
            page_count: 1,
            module_count: 0,
        },
    };
    let metadata = export_metadata(
        &state,
        &ExportContext {
            schema_version: "1".to_string(),
            language: "zh".to_string(),
            repo_root: ".".to_string(),
            branch: "main".to_string(),
            generated_at: "2026-06-17T00:00:00Z".to_string(),
            last_indexed_commit: "HEAD".to_string(),
        },
    );

    let section = metadata.section("section:intro").expect("section binding");
    assert_eq!(section.knowledge_refs, vec!["unit:repo"]);
    assert_eq!(
        metadata.reverse_refs.knowledge_to_sections["unit:repo"],
        vec!["section:intro"]
    );
    assert_eq!(
        metadata.reverse_refs.projection_to_sections["projection:repo"],
        vec!["section:intro"]
    );
}
