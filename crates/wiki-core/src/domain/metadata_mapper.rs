use crate::domain::metadata::{SourceFileRecord, WikiMetadata};
use crate::domain::state::WikiState;
use crate::domain::wiki_item::WikiItem;

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
        .collect();

    let source_files = state
        .sources
        .iter()
        .map(|source| SourceFileRecord {
            id: source.source_id.clone(),
            path: source.path.clone(),
            fingerprint: source.fingerprint.clone(),
            wiki_item_ids: source.page_ids.clone(),
            module_ids: source.module_ids.clone(),
        })
        .collect();

    WikiMetadata {
        schema_version: context.schema_version.clone(),
        language: context.language.clone(),
        repo_root: context.repo_root.clone(),
        branch: context.branch.clone(),
        generated_at: context.generated_at.clone(),
        last_indexed_commit: context.last_indexed_commit.clone(),
        modules: state.modules.clone(),
        wiki_items,
        relations: state.relations.clone(),
        source_files,
        dirty_state: state.dirty_state.clone(),
    }
}
