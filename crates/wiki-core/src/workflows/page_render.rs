//! 页面预渲染辅助层负责并行构造 page context、input hash 和 Markdown bundle。
//! 它只产出内存中的 page artifacts；最终写盘、cache 和状态收口仍由 workflow 串行完成。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

use crate::domain::context::{ModuleContext, PageContext, RepoContext};
use crate::domain::module_tree::ModuleTree;
use crate::domain::state::compute_page_input_hash;
use crate::generation::context::build_page_context;
use crate::generation::planner::PlannedPage;
use crate::generation::renderer::{render_page_bundle, RenderedPage};
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
    /// 当前页面渲染得到的 Markdown bundle。
    pub rendered_page: RenderedPage,
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

fn prepare_page_artifacts_with_workers(
    pages: &[PlannedPage],
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    repo_context: &RepoContext,
    module_contexts: &[ModuleContext],
    worker_count: usize,
) -> Vec<PreparedPageArtifact> {
    if worker_count <= 1 || pages.len() <= 1 {
        return pages
            .iter()
            .map(|page| build_page_artifact(page, scan_report, module_tree, repo_context, module_contexts))
            .collect();
    }

    let next_index = AtomicUsize::new(0);
    let results = Mutex::new(Vec::<(usize, PreparedPageArtifact)>::with_capacity(pages.len()));

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| loop {
                let index = next_index.fetch_add(1, Ordering::Relaxed);
                if index >= pages.len() {
                    break;
                }

                let artifact = build_page_artifact(
                    &pages[index],
                    scan_report,
                    module_tree,
                    repo_context,
                    module_contexts,
                );
                results.lock().unwrap().push((index, artifact));
            });
        }
    });

    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, artifact)| artifact).collect()
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
) -> PreparedPageArtifact {
    let page_context = build_page_context(
        page,
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
    );
    let input_hash = compute_page_input_hash(page, &page_context, scan_report);
    let rendered_page = render_page_bundle(page, &page_context);

    PreparedPageArtifact {
        page: page.clone(),
        page_context,
        input_hash,
        rendered_page,
    }
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
        fs::write(repo_root.join("package.json"), r#"{"name":"page-render-demo"}"#).unwrap();
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
        let resolved_graph = resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).unwrap();
        let analysis = analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
        let graph_summary =
            build_graph_summary(&scan_report, &symbol_snapshot, &resolved_graph, &analysis);
        let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
        let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
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
            assert_eq!(left.page_context.summary_inputs, right.page_context.summary_inputs);
            assert_eq!(left.rendered_page.content, right.rendered_page.content);
        }
    }
}
