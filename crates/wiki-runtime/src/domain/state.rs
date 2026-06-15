//! 状态层负责持久化 Repo Wiki runtime 的页面、源码、模块与关系事实。
//! 它位于页面生成之后，服务 `status / update / query / sync` 的统一读取与回写。

use std::collections::BTreeMap;

use serde::Serialize;

use crate::domain::context::PageContext;
use crate::domain::metadata::{DirtyState, WikiMetadata};
use crate::domain::module_tree::ModuleTree;
use crate::domain::relation::WikiRelation;
use crate::generation::sections::SectionDraft;
use crate::storage::wiki_fs::is_official_page_path;
use wiki_index::fingerprint::fingerprint_bytes;
use wiki_index::scanner::ScanReport;
use wiki_knowledge::PlannedPage;

pub use wiki_model::domain::state::{
    BuildState, SourceState, WikiPageState, WikiSectionState, WikiState,
};
/// 单个页面的构建产物，用于装配 WikiState。
pub struct PageBuildResult {
    /// planner 产出的页面定义。
    pub page: PlannedPage,
    /// 当前页面的上下文输入。
    pub context: PageContext,
    /// 当前页面对外暴露的摘要文本。
    pub summary: String,
    /// 当前页面输入事实的稳定摘要。
    pub input_hash: String,
    /// 当前页面最终 Markdown 的内容摘要。
    pub content_hash: String,
    /// 当前页面关联的源码路径集合。
    pub source_paths: Vec<String>,
    /// 当前页面在页面树中的祖先链。
    pub ancestor_ids: Vec<String>,
    /// 当前页面的 provenance 线索集合。
    pub provenance: Vec<String>,
    /// 当前页面的 section 草稿集合。
    pub sections: Vec<SectionDraft>,
}

/// 从 init pipeline 的构建结果装配完整 WikiState。
///
/// # 参数
/// - `page_results`：本轮页面构建结果集合。
/// - `scan_report`：当前仓库扫描结果。
/// - `module_tree`：当前仓库模块树。
/// - `generated_at`：本轮构建完成时间。
///
/// # 返回
/// - 返回可直接持久化的完整 `WikiState`。
pub fn assemble_state(
    page_results: &[PageBuildResult],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    generated_at: &str,
) -> WikiState {
    let pages = page_results
        .iter()
        .map(build_page_state)
        .collect::<Vec<_>>();

    assemble_state_from_pages(
        &pages,
        scan_report,
        module_tree,
        generated_at,
        DirtyState::fresh(),
    )
}

/// 用已知页面状态重建完整 WikiState。
/// update 会复用旧页面状态与新页面状态混合后，再通过这里装配最新运行时。
///
/// # 参数
/// - `pages`：已经确认的页面状态集合。
/// - `scan_report`：当前仓库扫描结果。
/// - `module_tree`：当前仓库模块树。
/// - `generated_at`：本轮构建完成时间。
/// - `dirty_state`：要写入状态层的外部脏状态摘要。
///
/// # 返回
/// - 返回基于给定页面状态重新装配出的完整 `WikiState`。
pub fn assemble_state_from_pages(
    pages: &[WikiPageState],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    generated_at: &str,
    dirty_state: DirtyState,
) -> WikiState {
    let pages = pages
        .iter()
        .filter(|page| is_official_page_path(&page.path))
        .cloned()
        .collect::<Vec<_>>();
    WikiState {
        sources: build_source_states(&pages, scan_report, module_tree),
        modules: module_tree.modules.clone(),
        relations: build_relations(&pages, module_tree),
        dirty_state,
        build_state: BuildState {
            generated_at: generated_at.to_string(),
            page_count: pages.len(),
            module_count: module_tree.modules.len(),
        },
        pages,
    }
}

/// 从 WikiMetadata 重建 WikiState 的回退路径。
/// 当 wiki-state.json 丢失但 wiki.metadata.json 存在时使用。
///
/// # 参数
/// - `metadata`：正式索引层导出的 WikiMetadata。
///
/// # 返回
/// - 返回足以支撑 `status / query` 理解正式索引的状态视图。
pub fn rebuild_state_from_metadata(metadata: &WikiMetadata) -> WikiState {
    let pages = metadata
        .wiki_items
        .iter()
        .filter(|item| is_official_page_path(&item.path))
        .map(|item| WikiPageState {
            page_id: item.id.clone(),
            title: item.title.clone(),
            path: item.path.clone(),
            page_type: item.item_type.clone(),
            parent_id: item.parent_id.clone(),
            ancestor_ids: item.ancestor_ids.clone(),
            input_hash: item.content_hash.clone(),
            content_hash: item.content_hash.clone(),
            source_ids: Vec::new(),
            source_paths: item.source_files.clone(),
            module_ids: item.module_ids.clone(),
            summary: item.summary.clone(),
            provenance: item.provenance.clone(),
            section_anchors: Vec::new(),
            sections: Vec::new(),
        })
        .collect::<Vec<_>>();

    let sources = metadata
        .source_files
        .iter()
        .map(|source| SourceState {
            source_id: source.id.clone(),
            path: source.path.clone(),
            fingerprint: source.fingerprint.clone(),
            page_ids: source
                .wiki_item_ids
                .iter()
                .filter(|page_id| pages.iter().any(|page| &page.page_id == *page_id))
                .cloned()
                .collect(),
            module_ids: source.module_ids.clone(),
        })
        .collect();

    let page_count = pages.len();

    WikiState {
        pages,
        sources,
        modules: metadata.modules.clone(),
        relations: metadata.relations.clone(),
        dirty_state: metadata.dirty_state.clone(),
        build_state: BuildState {
            generated_at: metadata.generated_at.clone(),
            page_count,
            module_count: metadata.modules.len(),
        },
    }
}

/// 基于页面计划、页面上下文和当前扫描结果计算稳定输入指纹。
/// 这里显式把源码 fingerprint 纳入签名，让局部源码内容变化也会驱动相关页面重生成。
///
/// # 参数
/// - `page`：当前页面计划。
/// - `context`：当前页面的上下文输入。
/// - `scan_report`：当前仓库扫描结果。
///
/// # 返回
/// - 返回页面输入事实的稳定 hash。
pub fn compute_page_input_hash(
    page: &PlannedPage,
    context: &PageContext,
    scan_report: &ScanReport,
) -> String {
    #[derive(Serialize)]
    struct PageInputSignature<'a> {
        page_id: &'a str,
        title: &'a str,
        relative_path: &'a str,
        page_type: &'a str,
        generation_mode: &'a str,
        parent_id: &'a Option<String>,
        scope: &'a str,
        unit_id: &'a Option<String>,
        unit_type: &'a Option<String>,
        domain_id: &'a Option<String>,
        source_ids: &'a [String],
        module_ids: &'a [String],
        relation_ids: &'a [String],
        facts: &'a [String],
        summary_inputs: &'a [String],
        hints: &'a [String],
        child_summaries: &'a [String],
        evidence_groups: &'a [crate::domain::context::PageEvidenceGroup],
        diagram_inputs: &'a [crate::domain::context::PageDiagramInput],
        source_fingerprints: Vec<(&'a str, &'a str)>,
    }

    let fingerprint_index = scan_report
        .files
        .iter()
        .map(|file| (file.id.as_str(), file.fingerprint.as_str()))
        .collect::<BTreeMap<_, _>>();
    let source_fingerprints = context
        .source_ids
        .iter()
        .filter_map(|source_id| {
            fingerprint_index
                .get(source_id.as_str())
                .map(|fingerprint| (source_id.as_str(), *fingerprint))
        })
        .collect::<Vec<_>>();

    let signature = PageInputSignature {
        page_id: &page.id,
        title: &page.title,
        relative_path: &page.relative_path,
        page_type: &page.page_type,
        generation_mode: &page.generation_mode,
        parent_id: &page.parent_id,
        scope: &page.scope,
        unit_id: &page.unit_id,
        unit_type: &page.unit_type,
        domain_id: &page.domain_id,
        source_ids: &context.source_ids,
        module_ids: &page.module_ids,
        relation_ids: &page.relation_ids,
        facts: &context.facts,
        summary_inputs: &context.summary_inputs,
        hints: &context.hints,
        child_summaries: &context.child_summaries,
        evidence_groups: &context.evidence_groups,
        diagram_inputs: &context.diagram_inputs,
        source_fingerprints,
    };

    let serialized = serde_json::to_vec(&signature).unwrap_or_default();
    fingerprint_bytes(&serialized)
}

/// 把单个页面构建结果转换成可持久化的页面状态。
///
/// # 参数
/// - `result`：单个页面的构建产物。
///
/// # 返回
/// - 返回可写入 `WikiState` 的 `WikiPageState`。
pub fn build_page_state(result: &PageBuildResult) -> WikiPageState {
    let sections = result
        .sections
        .iter()
        .map(|section| WikiSectionState {
            section_id: section.section_id.clone(),
            title: section.title.clone(),
            managed: section.managed,
            content_hash: fingerprint_bytes(section.content.as_bytes()),
            generated_content_hash: if section.managed {
                Some(fingerprint_bytes(section.content.as_bytes()))
            } else {
                None
            },
            anchor_after_section_id: None,
            anchor_before_section_id: None,
            source_ids: section.source_ids.clone(),
            relation_ids: section.relation_ids.clone(),
        })
        .collect::<Vec<_>>();
    let section_anchors = sections
        .iter()
        .filter(|section| section.managed)
        .map(|section| section.section_id.clone())
        .collect::<Vec<_>>();

    WikiPageState {
        page_id: result.page.id.clone(),
        title: result.page.title.clone(),
        path: format!(".wiki/{}", result.page.relative_path),
        page_type: result.page.page_type.clone(),
        parent_id: result.page.parent_id.clone(),
        ancestor_ids: result.ancestor_ids.clone(),
        input_hash: result.input_hash.clone(),
        content_hash: result.content_hash.clone(),
        source_ids: result.context.source_ids.clone(),
        source_paths: result.source_paths.clone(),
        module_ids: result.page.module_ids.clone(),
        summary: result.summary.clone(),
        provenance: result.provenance.clone(),
        section_anchors,
        sections,
    }
}

fn build_source_states(
    pages: &[WikiPageState],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
) -> Vec<SourceState> {
    scan_report
        .files
        .iter()
        .map(|file| {
            let page_ids = pages
                .iter()
                .filter(|page| page.source_ids.iter().any(|sid| sid == &file.id))
                .map(|page| page.page_id.clone())
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
        .collect()
}

fn build_relations(pages: &[WikiPageState], module_tree: &ModuleTree) -> Vec<WikiRelation> {
    let mut relations = Vec::new();

    for page in pages {
        if let Some(parent_id) = &page.parent_id {
            relations.push(WikiRelation {
                source_id: parent_id.clone(),
                target_id: page.page_id.clone(),
                relation_type: "PARENT_CHILD".to_string(),
                evidence: vec![page.path.clone()],
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

    relations
}
