use wiki_core::domain::metadata::WikiMetadata;

#[test]
fn metadata_roundtrip_keeps_dirty_state() {
    let metadata = WikiMetadata::sample();
    let json = serde_json::to_string_pretty(&metadata).unwrap();
    let decoded: WikiMetadata = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.dirty_state.status, metadata.dirty_state.status);
    assert_eq!(decoded.wiki_items.len(), 1);
    assert_eq!(decoded.source_files.len(), 1);
}
