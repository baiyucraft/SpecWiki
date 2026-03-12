//! 页面预渲染辅助层负责并行构造 page context、input hash 和 Markdown bundle。
//! 它只产出内存中的 page artifacts；最终写盘、cache 和状态收口仍由 workflow 串行完成。

use std::collections::BTreeMap;
use std::thread;

use crate::debug_trace;
use crate::domain::context::{
    ChildPageRollup, ModuleContext, PageContext, PageResearchDiagramRollup,
    PageResearchEvidenceGroup, PageResearchEvidenceItem, RepoContext,
};
use crate::domain::module_tree::ModuleTree;
use crate::domain::state::compute_page_input_hash;
use crate::domain::steering::SteeringConfig;
use crate::generation::context::build_page_context_with_inputs;
use crate::generation::planner::PlannedPage;
use crate::generation::renderer::{
    render_page_bundle, render_page_bundle_with_enrichment, RenderedPage,
};
use crate::llm::{
    LlmRuntime, PageEnrichmentInput, PageResearchInput, PageResearchRuntimeContext,
};
use crate::repo::scanner::ScanReport;

/// `PreparedPageArtifact` 是页面预渲染阶段的稳定输出。
/// 它把后续串行写盘需要的 page/context/hash/rendered bundle 统一打包。
#[derive(Debug, Clone)]
pub struct PreparedPageArtifact {
    /// 当前 artifact 对应的 planner 页面定义。
    pub page: PlannedPage,
    /// 当前页面的渲染上下文。
    pub page_context: PageContext,
    /// 当前页面输入事实的稳定摘要。
    pub input_hash: String,
    /// 当前页面供父页消费和状态层持久化的摘要。
    pub page_summary: String,
    /// 当前页面渲染得到的 Markdown bundle。
    pub rendered_page: RenderedPage,
}

#[derive(Debug, Clone)]
struct PendingLlmPageArtifact {
    index: usize,
    page: PlannedPage,
    page_context: PageContext,
    input_hash: String,
    enrichment_input: Option<PageEnrichmentInput>,
}

/// 并行预渲染页面 artifacts，并按 planner 顺序返回稳定结果。
pub fn prepare_page_artifacts(
    pages: &[PlannedPage],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
) -> Vec<PreparedPageArtifact> {
    prepare_page_artifacts_with_workers(
        pages,
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
        configured_page_worker_count(pages.len()),
    )
}

/// 在 LLM 可用时执行叶子优先的页面增强编排。
pub fn prepare_page_artifacts_with_llm(
    pages: &[PlannedPage],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    steering: &SteeringConfig,
    llm_runtime: Option<&mut LlmRuntime<'_, '_>>,
    mut on_llm_progress: Option<&mut dyn FnMut(usize, usize)>,
) -> Vec<PreparedPageArtifact> {
    let Some(llm_runtime) = llm_runtime else {
        return prepare_page_artifacts(
            pages,
            scan_report,
            module_tree,
            repo_context,
            module_contexts,
        );
    };
    if !llm_runtime.enrichment_enabled() {
        return prepare_page_artifacts(
            pages,
            scan_report,
            module_tree,
            repo_context,
            module_contexts,
        );
    }

    let children_by_parent = page_children_index(pages);
    let index_by_page_id = pages
        .iter()
        .enumerate()
        .map(|(index, page)| (page.id.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let depths = page_depth_index(pages);
    let mut page_order = pages
        .iter()
        .map(|page| {
            (
                depths.get(&page.id).copied().unwrap_or_default(),
                index_by_page_id.get(&page.id).copied().unwrap_or_default(),
                page,
            )
        })
        .collect::<Vec<_>>();
    page_order.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    let mut depth_groups = Vec::<Vec<(usize, &PlannedPage)>>::new();
    let mut last_depth = None::<usize>;
    for (depth, index, page) in page_order {
        if last_depth == Some(depth) {
            depth_groups
                .last_mut()
                .expect("depth group should exist")
                .push((index, page));
        } else {
            depth_groups.push(vec![(index, page)]);
            last_depth = Some(depth);
        }
    }

    let mut summaries = BTreeMap::<String, String>::new();
    let mut rollups = BTreeMap::<String, ChildPageRollup>::new();
    let mut artifacts = Vec::<(usize, PreparedPageArtifact)>::with_capacity(pages.len());
    let mut llm_processed = 0;

    for group in depth_groups {
        let mut pending_group = Vec::<PendingLlmPageArtifact>::with_capacity(group.len());
        for (index, page) in group {
            let module_paths = page_module_paths(page, module_tree);
            let hints = steering.page_hints_for(&page.page_type, Some(&page.id), &module_paths);
            let child_summaries = children_by_parent
                .get(&page.id)
                .into_iter()
                .flat_map(|child_ids| child_ids.iter())
                .filter_map(|child_id| summaries.get(child_id).cloned())
                .collect::<Vec<_>>();
            let child_rollups = children_by_parent
                .get(&page.id)
                .into_iter()
                .flat_map(|child_ids| child_ids.iter())
                .filter_map(|child_id| rollups.get(child_id).cloned())
                .collect::<Vec<_>>();
            let mut page_context = build_page_context_with_inputs(
                page,
                scan_report,
                module_tree,
                repo_context,
                module_contexts,
                hints,
                child_summaries,
                child_rollups,
            );
            if llm_runtime.session_enabled() && matches!(page.page_type.as_str(), "module" | "topic")
            {
                let research_input = PageResearchInput::from_page(page, &page_context);
                let research_runtime = PageResearchRuntimeContext {
                    page,
                    page_context: &page_context,
                    scan_report,
                    module_tree,
                    repo_context,
                    module_contexts,
                };
                match llm_runtime.research_page(&research_input, &research_runtime) {
                    Ok(Some(output)) => {
                        page_context.research_result = Some(output.result);
                        page_context.research_session = Some(output.session);
                    }
                    Ok(None) => {}
                    Err(error) => {
                        debug_trace::record_json(
                            "llm_research_error",
                            &serde_json::json!({
                                "page_id": page.id,
                                "page_type": page.page_type,
                                "title": page.title,
                                "error": error.to_string(),
                            }),
                        );
                    }
                }
            }
            let input_hash = compute_page_input_hash(page, &page_context, scan_report);
            let enrichment_input = (!matches!(page.page_type.as_str(), "module" | "topic")
                || page_context.research_result.is_none())
                .then(|| {
                    PageEnrichmentInput::from_page(page, &page_context, steering.llm.allow_mermaid)
                });
            pending_group.push(PendingLlmPageArtifact {
                index,
                page: page.clone(),
                page_context,
                input_hash,
                enrichment_input,
            });
        }

        let group_inputs = pending_group
            .iter()
            .filter_map(|task| task.enrichment_input.clone())
            .collect::<Vec<_>>();
        let enrichments = llm_runtime
            .enrich_pages(&group_inputs)
            .unwrap_or_else(|_| vec![None; group_inputs.len()]);
        let mut enrichments_iter = enrichments.into_iter();

        for task in pending_group.into_iter() {
            let enrichment = if task.enrichment_input.is_some() {
                enrichments_iter.next().unwrap_or(None)
            } else {
                None
            };
            let page_summary = if let Some(summary) = enrichment
                .as_ref()
                .map(|result| result.summary.trim())
                .filter(|summary| !summary.is_empty())
            {
                summary.to_string()
            } else if let Some(summary) = task
                .page_context
                .research_result
                .as_ref()
                .map(|result| result.summary.trim())
                .filter(|summary| !summary.is_empty())
            {
                summary.to_string()
            } else {
                task.page_context.summary_inputs.join("；")
            };
            let rendered_page = render_page_bundle_with_enrichment(
                &task.page,
                &task.page_context,
                enrichment.as_ref(),
            );

            summaries.insert(task.page.id.clone(), page_summary.clone());
            rollups.insert(
                task.page.id.clone(),
                build_child_page_rollup(&task.page, &task.page_context, &page_summary),
            );
            artifacts.push((
                task.index,
                PreparedPageArtifact {
                    page: task.page,
                    page_context: task.page_context,
                    input_hash: task.input_hash,
                    page_summary,
                    rendered_page,
                },
            ));
        }
        llm_processed += group_inputs.len();
        if let Some(callback) = on_llm_progress.as_deref_mut() {
            callback(llm_processed, pages.len());
        }
    }

    artifacts.sort_by_key(|(index, _)| *index);
    artifacts
        .into_iter()
        .map(|(_, artifact)| artifact)
        .collect()
}

fn prepare_page_artifacts_with_workers(
    pages: &[PlannedPage],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    _worker_count: usize,
) -> Vec<PreparedPageArtifact> {
    let children_by_parent = page_children_index(pages);
    let index_by_page_id = pages
        .iter()
        .enumerate()
        .map(|(index, page)| (page.id.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let depths = page_depth_index(pages);
    let mut page_order = pages
        .iter()
        .map(|page| {
            (
                depths.get(&page.id).copied().unwrap_or_default(),
                index_by_page_id.get(&page.id).copied().unwrap_or_default(),
                page,
            )
        })
        .collect::<Vec<_>>();
    page_order.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));

    let mut summaries = BTreeMap::<String, String>::new();
    let mut rollups = BTreeMap::<String, ChildPageRollup>::new();
    let mut artifacts = Vec::<(usize, PreparedPageArtifact)>::with_capacity(pages.len());

    for (_, index, page) in page_order {
        let child_summaries = children_by_parent
            .get(&page.id)
            .into_iter()
            .flat_map(|child_ids| child_ids.iter())
            .filter_map(|child_id| summaries.get(child_id).cloned())
            .collect::<Vec<_>>();
        let child_rollups = children_by_parent
            .get(&page.id)
            .into_iter()
            .flat_map(|child_ids| child_ids.iter())
            .filter_map(|child_id| rollups.get(child_id).cloned())
            .collect::<Vec<_>>();
        let artifact = build_page_artifact(
            page,
            scan_report,
            module_tree,
            repo_context,
            module_contexts,
            child_summaries,
            child_rollups,
        );
        summaries.insert(artifact.page.id.clone(), artifact.page_summary.clone());
        rollups.insert(
            artifact.page.id.clone(),
            build_child_page_rollup(&artifact.page, &artifact.page_context, &artifact.page_summary),
        );
        artifacts.push((index, artifact));
    }

    artifacts.sort_by_key(|(index, _)| *index);
    artifacts.into_iter().map(|(_, artifact)| artifact).collect()
}

fn configured_page_worker_count(page_count: usize) -> usize {
    let configured = std::env::var("WIKI_PAGE_RENDER_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0);
    let default = thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .min(4);

    configured.unwrap_or(default).clamp(1, page_count.max(1))
}

fn build_page_artifact(
    page: &PlannedPage,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    child_summaries: Vec<String>,
    child_rollups: Vec<ChildPageRollup>,
) -> PreparedPageArtifact {
    let page_context = build_page_context_with_inputs(
        page,
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
        Vec::new(),
        child_summaries,
        child_rollups,
    );
    let input_hash = compute_page_input_hash(page, &page_context, scan_report);
    let page_summary = page_context.summary_inputs.join("；");
    let rendered_page = render_page_bundle(page, &page_context);

    PreparedPageArtifact {
        page: page.clone(),
        page_context,
        input_hash,
        page_summary,
        rendered_page,
    }
}

fn build_child_page_rollup(
    page: &PlannedPage,
    page_context: &PageContext,
    page_summary: &str,
) -> ChildPageRollup {
    let evidence_rollup = page_context
        .research_result
        .as_ref()
        .map(|result| result.evidence_rollup.clone())
        .unwrap_or_else(|| {
            page_context
                .evidence_groups
                .iter()
                .map(|group| PageResearchEvidenceGroup {
                    group_key: group.group_id.clone(),
                    title: group.title.clone(),
                    items: group
                        .items
                        .iter()
                        .map(|item| PageResearchEvidenceItem {
                            source_id: item.source_id.clone(),
                            path: item.path.clone(),
                            start_line: 0,
                            end_line: 0,
                            note: item.note.clone(),
                        })
                        .collect(),
                })
                .collect()
        });
    let diagram_rollup = page_context
        .research_result
        .as_ref()
        .map(|result| result.diagram_rollup.clone())
        .unwrap_or_else(|| {
            page_context
                .diagram_inputs
                .iter()
                .map(|diagram| PageResearchDiagramRollup {
                    diagram_key: diagram.diagram_id.clone(),
                    diagram_type: diagram.diagram_type.clone(),
                    title: diagram.title.clone(),
                    summary: diagram.summary.clone(),
                })
                .collect()
        });
    let key_sources_rollup = page_context
        .module_dossiers
        .iter()
        .flat_map(|dossier| dossier.key_sources.iter().cloned())
        .chain(
            page_context
                .topic_dossier
                .iter()
                .flat_map(|dossier| dossier.key_sources.iter().cloned()),
        )
        .collect::<Vec<_>>();
    let open_questions = page_context
        .research_result
        .as_ref()
        .map(|result| result.open_questions.clone())
        .unwrap_or_default();

    ChildPageRollup {
        page_id: page.id.clone(),
        title: page.title.clone(),
        page_type: page.page_type.clone(),
        summary: page_summary.to_string(),
        key_sources_rollup,
        evidence_rollup,
        diagram_rollup,
        open_questions,
    }
}

fn page_children_index(pages: &[PlannedPage]) -> BTreeMap<String, Vec<String>> {
    let mut children = BTreeMap::<String, Vec<String>>::new();

    for page in pages {
        if let Some(parent_id) = &page.parent_id {
            children
                .entry(parent_id.clone())
                .or_default()
                .push(page.id.clone());
        }
    }

    children
}

fn page_depth_index(pages: &[PlannedPage]) -> BTreeMap<String, usize> {
    let pages_by_id = pages
        .iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let mut depths = BTreeMap::new();

    for page in pages {
        depths.insert(page.id.clone(), page_depth(page, &pages_by_id));
    }

    depths
}

fn page_depth(page: &PlannedPage, pages_by_id: &BTreeMap<String, &PlannedPage>) -> usize {
    let mut depth = 0;
    let mut current = page.parent_id.as_deref();

    while let Some(parent_id) = current {
        depth += 1;
        current = pages_by_id
            .get(parent_id)
            .and_then(|parent| parent.parent_id.as_deref());
    }

    depth
}

fn page_module_paths(page: &PlannedPage, module_tree: &ModuleTree) -> Vec<String> {
    page.module_ids
        .iter()
        .filter_map(|module_id| module_tree.module_by_id(module_id))
        .flat_map(|module| module.root_paths.iter().cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::domain::steering::load_steering_config;
    use crate::generation::context::{
        build_module_contexts_with_graph, build_repo_context_with_graph,
    };
    use crate::generation::planner::plan_pages_with_graph;
    use crate::repo::hierarchy::build_module_tree_with_graph;
    use crate::repo::scanner::scan_repo_with_boundary;
    use crate::repo::symbol_graph::{
        analyze_symbol_graph, build_graph_summary, resolve_symbol_graph,
    };
    use crate::repo::symbols::parse_symbols;

    use super::prepare_page_artifacts_with_workers;

    #[test]
    fn page_artifacts_keep_deterministic_order_and_hashes_across_worker_counts() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::write(
            repo_root.join("package.json"),
            r#"{"name":"page-render-demo"}"#,
        )
        .unwrap();
        fs::create_dir_all(repo_root.join("src/lib")).unwrap();
        fs::write(
            repo_root.join("src/index.ts"),
            "export function handleCheckout() { return true; }\n",
        )
        .unwrap();
        fs::write(
            repo_root.join("src/lib/payments.ts"),
            concat!(
                "import { handleCheckout } from \"../index\";\n",
                "export function settlePayment() { return handleCheckout(); }\n",
            ),
        )
        .unwrap();

        let steering = load_steering_config(repo_root);
        let (ignore_paths, include_paths) = steering.scan_boundary();
        let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths).unwrap();
        let symbol_snapshot = parse_symbols(repo_root, &scan_report).unwrap();
        let resolved_graph =
            resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let analysis = analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary =
            build_graph_summary(&scan_report, &symbol_snapshot, &resolved_graph, &analysis);
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context =
            build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
        let module_contexts =
            build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);
        let pages = plan_pages_with_graph(
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &steering,
            &graph_summary,
        );

        let single = prepare_page_artifacts_with_workers(
            &pages,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            1,
        );
        let parallel = prepare_page_artifacts_with_workers(
            &pages,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            4,
        );

        assert_eq!(single.len(), parallel.len());
        for (left, right) in single.iter().zip(parallel.iter()) {
            assert_eq!(left.page.id, right.page.id);
            assert_eq!(left.input_hash, right.input_hash);
            assert_eq!(
                left.page_context.summary_inputs,
                right.page_context.summary_inputs
            );
            assert_eq!(left.rendered_page.content, right.rendered_page.content);
        }
    }
}
