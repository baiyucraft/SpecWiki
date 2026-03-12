//! rebuild workflow 负责强制全量重建 Repo Wiki runtime。
//! 迭代 5 改为：忽略旧 generation cache，但对同 page_id 页面复用已同步的 user sections。

use serde::Serialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use crate::debug_trace;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::{assemble_state, PageBuildResult};
use crate::domain::steering::load_steering_config;
use crate::generation::context::{build_module_contexts_with_graph, build_repo_context_with_graph};
use crate::generation::managed_sections::{
    merge_sections, parse_wiki_page, ManagedSectionBlock, PageBlock,
};
use crate::generation::renderer::assemble_page_from_merge;
use crate::generation::sections::section_titles_for_page_type;
use crate::llm::{LlmRuntime, LlmService};
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree_with_graph_and_llm;
use crate::repo::scanner::scan_repo_with_boundary_and_llm;
use crate::repo::symbol_graph::{analyze_symbol_graph, build_graph_summary, resolve_symbol_graph};
use crate::storage::cache_store::{
    ensure_cache_dir, ensure_page_cache_dirs, write_module_tree_cache, write_page_context_cache,
    write_page_generation_cache, write_scan_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::state_store::write_state_with_symbol_graph;
use crate::storage::wiki_fs::{resolve_page_path, write_page};
use crate::workflows::init::{
    ancestor_ids_for_page, current_timestamp, page_provenance, source_paths_for_page,
};
use crate::workflows::page_render::prepare_page_artifacts_with_llm;
use crate::workflows::progress::{
    NoopProgressSink, ProgressSink, SharedProgressSink, WorkflowProgressEvent, WorkflowReporter,
};

/// `rebuild` 是显式的"强制重建"入口。
/// 迭代 5 之后，rebuild 会保留同 page_id 页面中已同步的 user sections。
#[derive(Debug, Clone, Serialize)]
pub struct RebuildReport {
    pub state: String,
    pub updated_pages: Vec<String>,
    /// rebuild 过程中产生的警告信息。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// 强制全量重建 Repo Wiki，保留同页 user sections。
pub fn run_rebuild(repo_root: &Path) -> io::Result<RebuildReport> {
    let mut sink = NoopProgressSink;
    run_rebuild_with_progress_as("rebuild", repo_root, &mut sink)
}

/// 允许 transport 以指定 action 名称执行 rebuild 主链。
pub fn run_rebuild_with_progress_as(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &mut dyn ProgressSink,
) -> io::Result<RebuildReport> {
    run_rebuild_with_progress_and_llm_as(action, repo_root, progress_sink, None)
}

/// 在保留旧进度接口的同时，允许 transport 注入可选 LLM 桥接。
///
/// # 参数
/// - `action`：当前 workflow 的对外动作名。
/// - `repo_root`：待重建 Wiki 的仓库根目录。
/// - `progress_sink`：接收阶段进度的下游。
/// - `_llm_service`：可选的 LLM 桥接服务；不可用时自动回退 deterministic。
///
/// # 返回
/// - 成功时返回 rebuild 报告。
pub fn run_rebuild_with_progress_and_llm_as<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    _llm_service: Option<&'a mut dyn LlmService>,
) -> io::Result<RebuildReport> {
    if !repo_root.exists() || !repo_root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repo root must be an existing directory",
        ));
    }

    let shared_sink = Rc::new(RefCell::new(progress_sink));
    let started_at = Instant::now();
    let mut reporter_sink = SharedProgressSink::new(shared_sink.clone());
    let mut reporter = WorkflowReporter::from_started_at(action, &mut reporter_sink, started_at);

    let steering = load_steering_config(repo_root);

    // 在清理前，读取旧页面的磁盘内容用于 user section 恢复
    let old_page_contents = read_old_page_contents(repo_root);

    // 清理旧 runtime
    reporter.phase("clear_runtime", "清理旧运行时");
    crate::storage::wiki_fs::remove_runtime_with_cache_mode(repo_root, steering.llm.cache_mode)?;
    debug_trace::begin_session(action, repo_root, &steering.debug)?;

    // 全量 pipeline
    let mut llm_runtime = LlmRuntime::new(repo_root, &steering.llm, _llm_service);
    let usage_sink = shared_sink.clone();
    llm_runtime.set_usage_reporter(Some(Box::new(move |usage| {
        usage_sink.borrow_mut().report(WorkflowProgressEvent {
            action: action.to_string(),
            phase: "llm_usage".to_string(),
            message: format!(
                "LLM usage 已更新：{} requests / {} tokens",
                usage.request_count, usage.total_tokens
            ),
            elapsed_ms: started_at.elapsed().as_millis() as u64,
            processed: None,
            total: None,
            usage: Some(usage),
        });
    })));
    let (ignore_paths, include_paths) = steering.scan_boundary();
    if llm_runtime.service_available() {
        reporter.phase("llm_uncertainty_gate", "执行 LLM 不确定性判断");
    }
    reporter.phase("scan", "扫描仓库源码");
    let scan_report = scan_repo_with_boundary_and_llm(
        repo_root,
        ignore_paths,
        include_paths,
        Some(&mut llm_runtime),
    )?;
    let symbol_total = crate::repo::symbols::symbol_parse_file_count(&scan_report, &[]);
    reporter.counted("parse_symbols", "解析源码符号", 0, symbol_total);
    let symbol_snapshot = crate::repo::symbols::parse_symbols_with_progress(
        repo_root,
        &scan_report,
        &mut |processed, total| {
            reporter.counted(
                "parse_symbols",
                format!("解析源码符号 {processed}/{total}"),
                processed,
                total,
            );
        },
    )?;
    reporter.phase("resolve_symbol_graph", "解析符号关系");
    let resolved_graph = resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot)?;
    reporter.phase("analyze_symbol_graph", "分析符号图");
    let analysis = analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
    let graph_summary =
        build_graph_summary(&scan_report, &symbol_snapshot, &resolved_graph, &analysis);
    reporter.phase("build_module_tree", "构建模块树");
    let module_tree =
        build_module_tree_with_graph_and_llm(&scan_report, &graph_summary, Some(&mut llm_runtime));
    reporter.phase("build_contexts", "构建页面上下文");
    let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
    let module_contexts =
        build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);
    reporter.phase("plan_pages", "规划 Wiki 页面");
    let pages = crate::generation::planner::plan_pages_with_graph(
        &scan_report,
        &module_tree,
        &repo_context,
        &module_contexts,
        &steering,
        &graph_summary,
    );

    ensure_cache_dir(repo_root)?;
    ensure_page_cache_dirs(repo_root)?;
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    let page_total = pages.len();

    reporter.counted("render_pages", "渲染页面", 0, page_total);
    let llm_enrichment_enabled = llm_runtime.enrichment_enabled();
    if llm_enrichment_enabled {
        reporter.phase("llm_enrichment", "生成页面增强内容");
    }
    let mut llm_progress = |processed: usize, total: usize| {
        reporter.counted(
            "llm_enrichment",
            format!("生成页面增强内容 {processed}/{total}"),
            processed,
            total,
        );
    };
    let prepared_pages = prepare_page_artifacts_with_llm(
        &pages,
        &scan_report,
        &module_tree,
        &repo_context,
        &module_contexts,
        &symbol_snapshot,
        &resolved_graph,
        &analysis,
        &steering,
        Some(&mut llm_runtime),
        llm_enrichment_enabled.then_some(&mut llm_progress as &mut dyn FnMut(usize, usize)),
    );

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
    let mut all_warnings = Vec::new();
    let generated_at = current_timestamp();
    let mut ancestor_ids_by_page = BTreeMap::new();

    for (index, artifact) in prepared_pages.into_iter().enumerate() {
        // 尝试从旧页面恢复 user sections
        let final_content = match old_page_contents.get(&artifact.page.id) {
            Some(old_content) => merge_old_user_sections(
                &artifact.page,
                &artifact.rendered_page.sections,
                &artifact.rendered_page.content,
                old_content,
                &mut all_warnings,
            ),
            None => artifact.rendered_page.content.clone(),
        };

        write_page(repo_root, &artifact.page.relative_path, &final_content)?;
        let page_path = format!(".wiki/{}", artifact.page.relative_path);
        generated_pages.push(page_path);
        let ancestor_ids = ancestor_ids_for_page(&artifact.page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(artifact.page.id.clone(), ancestor_ids.clone());
        let content_hash = fingerprint_bytes(final_content.as_bytes());

        write_page_context_cache(
            repo_root,
            &PageContextCacheEntry {
                page_id: artifact.page.id.clone(),
                input_hash: artifact.input_hash.clone(),
                context: artifact.page_context.clone(),
            },
        )?;
        write_page_generation_cache(
            repo_root,
            &PageGenerationCacheEntry {
                page_id: artifact.page.id.clone(),
                input_hash: artifact.input_hash.clone(),
                content_hash: content_hash.clone(),
                sections: artifact.rendered_page.sections.clone(),
            },
        )?;

        page_results.push(PageBuildResult {
            page: artifact.page.clone(),
            context: artifact.page_context.clone(),
            summary: artifact.page_summary.clone(),
            input_hash: artifact.input_hash,
            content_hash,
            source_paths: source_paths_for_page(&scan_report, &artifact.page_context),
            ancestor_ids,
            provenance: page_provenance(&artifact.page, &artifact.page_context, &scan_report),
            sections: artifact.rendered_page.sections,
        });

        reporter.counted(
            "render_pages",
            format!("渲染页面 {}/{}", index + 1, page_total),
            index + 1,
            page_total,
        );
    }

    let state = assemble_state(&page_results, &scan_report, &module_tree, &generated_at);
    reporter.phase("write_state", "写入运行时状态");
    write_state_with_symbol_graph(
        repo_root,
        &state,
        &symbol_snapshot.symbols,
        &resolved_graph,
        &analysis,
    )?;

    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at,
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&state, &export_context);
    reporter.phase("write_metadata", "写入元数据");
    write_metadata(repo_root, &metadata)?;

    Ok(RebuildReport {
        state: state.dirty_state.status,
        updated_pages: generated_pages,
        warnings: all_warnings,
    })
}

/// 在清理前读取旧 WikiState 中每个页面的磁盘内容，按 page_id 索引。
fn read_old_page_contents(repo_root: &Path) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    if let Ok(old_state) = crate::storage::state_store::load_or_rebuild_state(repo_root) {
        for page in &old_state.pages {
            let page_path = resolve_page_path(repo_root, &page.path);
            if let Ok(content) = std::fs::read_to_string(&page_path) {
                map.insert(page.page_id.clone(), content);
            }
        }
    }
    map
}

/// 从旧页面内容中解析 user sections，与新 managed sections 合并。
fn merge_old_user_sections(
    planned_page: &crate::generation::planner::PlannedPage,
    new_sections: &[crate::generation::sections::SectionDraft],
    new_content: &str,
    old_content: &str,
    warnings: &mut Vec<String>,
) -> String {
    let known_titles = section_titles_for_page_type(&planned_page.page_type);
    let known_titles_ref: Vec<&str> = known_titles.iter().copied().collect();
    let old_parsed = parse_wiki_page(old_content, &known_titles_ref);

    let has_user_sections = old_parsed
        .blocks
        .iter()
        .any(|b| matches!(b, PageBlock::User(_)));

    if !has_user_sections {
        return new_content.to_string();
    }

    let new_managed: Vec<ManagedSectionBlock> = new_sections
        .iter()
        .map(|s| ManagedSectionBlock {
            section_id: s.section_id.clone(),
            title: s.title.clone(),
            version: crate::generation::managed_sections::MARKER_VERSION,
            body: s.content.clone(),
        })
        .collect();

    let merge_plan = merge_sections(&new_managed, &old_parsed);
    for w in &merge_plan.warnings {
        warnings.push(format!("[{}] {}", planned_page.relative_path, w));
    }
    assemble_page_from_merge(&planned_page.title, &merge_plan)
}
