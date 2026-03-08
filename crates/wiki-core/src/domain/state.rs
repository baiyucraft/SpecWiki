use serde::{Deserialize, Serialize};

use crate::domain::context::PageContext;
use crate::domain::metadata::{DirtyState, WikiMetadata};
use crate::domain::module_tree::{ModuleNode, ModuleTree};
use crate::domain::relation::WikiRelation;
use crate::generation::planner::PlannedPage;
use crate::repo::scanner::ScanReport;

/// `WikiPageState` 是内部运行时使用的页面状态模型。
/// 相比 `WikiItem`，这里更偏 update/query 所需的内部映射信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiPageState {
    pub page_id: String,
    pub title: String,
    pub path: String,
    pub page_type: String,
    pub parent_id: Option<String>,
    pub ancestor_ids: Vec<String>,
    pub content_hash: String,
    pub source_ids: Vec<String>,
    pub source_paths: Vec<String>,
    pub module_ids: Vec<String>,
    pub summary: String,
    pub provenance: Vec<String>,
}

/// `SourceState` 是内部运行时的源码状态模型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceState {
    pub source_id: String,
    pub path: String,
    pub fingerprint: String,
    pub page_ids: Vec<String>,
    pub module_ids: Vec<String>,
}

/// `BuildState` 记录一次构建完成后的整体摘要。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildState {
    pub generated_at: String,
    pub page_count: usize,
    pub module_count: usize,
}

/// `WikiState` 是所有 workflow 的内部事实主模型。
/// 它承载页面、源码、模块、关系、脏状态和构建状态的完整映射。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiState {
    pub pages: Vec<WikiPageState>,
    pub sources: Vec<SourceState>,
    pub modules: Vec<ModuleNode>,
    pub relations: Vec<WikiRelation>,
    pub dirty_state: DirtyState,
    pub build_state: BuildState,
}

/// 单个页面的构建产物，用于装配 WikiState。
pub struct PageBuildResult {
    pub page: PlannedPage,
    pub context: PageContext,
    pub content_hash: String,
    pub source_paths: Vec<String>,
    pub ancestor_ids: Vec<String>,
    pub provenance: Vec<String>,
}

/// 从 init pipeline 的构建结果装配完整 WikiState。
pub fn assemble_state(
    page_results: &[PageBuildResult],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    generated_at: &str,
) -> WikiState {
    let pages = page_results
        .iter()
        .map(|result| WikiPageState {
            page_id: result.page.id.clone(),
            title: result.page.title.clone(),
            path: format!(".wiki/{}", result.page.relative_path),
            page_type: result.page.page_type.clone(),
            parent_id: result.page.parent_id.clone(),
            ancestor_ids: result.ancestor_ids.clone(),
            content_hash: result.content_hash.clone(),
            source_ids: result.context.source_ids.clone(),
            source_paths: result.source_paths.clone(),
            module_ids: result.page.module_ids.clone(),
            summary: result.context.summary_inputs.join("；"),
            provenance: result.provenance.clone(),
        })
        .collect::<Vec<_>>();

    let sources = scan_report
        .files
        .iter()
        .map(|file| {
            let page_ids = page_results
                .iter()
                .filter(|result| {
                    result
                        .page
                        .source_ids
                        .iter()
                        .any(|sid| sid == &file.id)
                })
                .map(|result| result.page.id.clone())
                .collect();
            let module_ids = module_tree
                .modules
                .iter()
                .filter(|module| module.source_ids.iter().any(|sid| sid == &file.id))
                .map(|module| module.id.clone())
                .collect();
            SourceState {
                source_id: file.id.clone(),
                path: file.path.clone(),
                fingerprint: file.fingerprint.clone(),
                page_ids,
                module_ids,
            }
        })
        .collect();

    let mut relations = Vec::new();
    for result in page_results {
        if let Some(parent_id) = &result.page.parent_id {
            relations.push(WikiRelation {
                source_id: parent_id.clone(),
                target_id: result.page.id.clone(),
                relation_type: "PARENT_CHILD".to_string(),
                evidence: vec![format!(".wiki/{}", result.page.relative_path)],
            });
        }
    }
    for edge in &module_tree.cross_module_edges {
        relations.push(WikiRelation {
            source_id: edge.source.clone(),
            target_id: edge.target.clone(),
            relation_type: edge.relation_type.clone(),
            evidence: edge.evidence.clone(),
        });
    }

    WikiState {
        pages,
        sources,
        modules: module_tree.modules.clone(),
        relations,
        dirty_state: DirtyState::fresh(),
        build_state: BuildState {
            generated_at: generated_at.to_string(),
            page_count: page_results.len(),
            module_count: module_tree.modules.len(),
        },
    }
}

/// 从 WikiMetadata 重建 WikiState 的回退路径。
/// 当 wiki-state.json 丢失但 wiki.metadata.json 存在时使用。
pub fn rebuild_state_from_metadata(metadata: &WikiMetadata) -> WikiState {
    let pages = metadata
        .wiki_items
        .iter()
        .map(|item| WikiPageState {
            page_id: item.id.clone(),
            title: item.title.clone(),
            path: item.path.clone(),
            page_type: item.item_type.clone(),
            parent_id: item.parent_id.clone(),
            ancestor_ids: item.ancestor_ids.clone(),
            content_hash: item.content_hash.clone(),
            source_ids: Vec::new(),
            source_paths: item.source_files.clone(),
            module_ids: item.module_ids.clone(),
            summary: item.summary.clone(),
            provenance: item.provenance.clone(),
        })
        .collect::<Vec<_>>();

    let sources = metadata
        .source_files
        .iter()
        .map(|source| SourceState {
            source_id: source.id.clone(),
            path: source.path.clone(),
            fingerprint: source.fingerprint.clone(),
            page_ids: source.wiki_item_ids.clone(),
            module_ids: source.module_ids.clone(),
        })
        .collect();

    WikiState {
        pages,
        sources,
        modules: metadata.modules.clone(),
        relations: metadata.relations.clone(),
        dirty_state: metadata.dirty_state.clone(),
        build_state: BuildState {
            generated_at: metadata.generated_at.clone(),
            page_count: metadata.wiki_items.len(),
            module_count: metadata.modules.len(),
        },
    }
}
