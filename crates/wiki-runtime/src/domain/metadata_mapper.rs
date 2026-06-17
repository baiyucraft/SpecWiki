use crate::domain::metadata::{MetadataReverseRefs, MetadataSectionBinding, SourceFileRecord, WikiMetadata};
use crate::domain::state::WikiState;
use crate::domain::wiki_item::WikiItem;
use crate::storage::wiki_fs::is_official_page_path;
use std::collections::{BTreeMap, BTreeSet};

/// `ExportContext` 承载 WikiState 中不包含的外部展示字段。
/// MetadataMapper 在导出时用它补齐 repo_root、branch 等信息。
pub struct ExportContext {
    pub schema_version: String,
    pub language: String,
    pub repo_root: String,
    pub branch: String,
    pub generated_at: String,
    pub last_indexed_commit: String,
}

/// 将 WikiState 映射为 WikiMetadata（外部导出格式）。
/// 所有 workflow 必须通过此函数生成 wiki.metadata.json，不再直接手工拼装。
pub fn export_metadata(state: &WikiState, context: &ExportContext) -> WikiMetadata {
    let wiki_items = state
        .pages
        .iter()
        .filter(|page| is_official_page_path(&page.path))
        .map(|page| WikiItem {
            id: page.page_id.clone(),
            title: page.title.clone(),
            path: page.path.clone(),
            item_type: page.page_type.clone(),
            parent_id: page.parent_id.clone(),
            ancestor_ids: page.ancestor_ids.clone(),
            module_ids: page.module_ids.clone(),
            source_files: page.source_paths.clone(),
            content_hash: page.content_hash.clone(),
            summary: page.summary.clone(),
            provenance: page.provenance.clone(),
        })
        .collect::<Vec<_>>();
    let official_page_ids = wiki_items
        .iter()
        .map(|item| item.id.as_str())
        .collect::<BTreeSet<_>>();

    let source_files = state
        .sources
        .iter()
        .map(|source| SourceFileRecord {
            id: source.source_id.clone(),
            path: source.path.clone(),
            fingerprint: source.fingerprint.clone(),
            wiki_item_ids: source
                .page_ids
                .iter()
                .filter(|page_id| official_page_ids.contains(page_id.as_str()))
                .cloned()
                .collect(),
            module_ids: source.module_ids.clone(),
        })
        .collect();
    let sections = state
        .pages
        .iter()
        .filter(|page| is_official_page_path(&page.path))
        .flat_map(|page| {
            page.sections.iter().map(|section| MetadataSectionBinding {
                section_id: section.section_id.clone(),
                page_id: page.page_id.clone(),
                title: section.title.clone(),
                owner_kind: section.owner_kind,
                knowledge_refs: section.knowledge_refs.clone(),
                source_refs: section.source_ids.clone(),
                input_hash: section.input_hash.clone(),
                content_hash: section.content_hash.clone(),
                generated_content_hash: section.generated_content_hash.clone(),
                projection_digest_ref: section.projection_digest_ref.clone(),
            })
        })
        .collect::<Vec<_>>();
    let reverse_refs = build_reverse_refs(&sections);

    WikiMetadata {
        schema_version: context.schema_version.clone(),
        current_snapshot_id: None,
        language: context.language.clone(),
        repo_root: context.repo_root.clone(),
        branch: context.branch.clone(),
        generated_at: context.generated_at.clone(),
        last_indexed_commit: context.last_indexed_commit.clone(),
        modules: state.modules.clone(),
        wiki_items,
        relations: state.relations.clone(),
        source_files,
        sections,
        reverse_refs,
        dirty_state: state.dirty_state.clone(),
    }
}

fn build_reverse_refs(sections: &[MetadataSectionBinding]) -> MetadataReverseRefs {
    let mut knowledge_to_sections = BTreeMap::<String, Vec<String>>::new();
    let mut source_to_sections = BTreeMap::<String, Vec<String>>::new();
    let mut projection_to_sections = BTreeMap::<String, Vec<String>>::new();

    for section in sections {
        for knowledge_ref in &section.knowledge_refs {
            knowledge_to_sections
                .entry(knowledge_ref.clone())
                .or_default()
                .push(section.section_id.clone());
        }
        for source_ref in &section.source_refs {
            source_to_sections
                .entry(source_ref.clone())
                .or_default()
                .push(section.section_id.clone());
        }
        if let Some(projection_ref) = section.projection_digest_ref.as_ref() {
            projection_to_sections
                .entry(projection_ref.clone())
                .or_default()
                .push(section.section_id.clone());
        }
    }

    for refs in knowledge_to_sections
        .values_mut()
        .chain(source_to_sections.values_mut())
        .chain(projection_to_sections.values_mut())
    {
        refs.sort();
        refs.dedup();
    }

    MetadataReverseRefs {
        knowledge_to_sections,
        source_to_sections,
        projection_to_sections,
    }
}
