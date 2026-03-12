//! init workflow 负责按全量链路生成第一版 Repo Wiki runtime。
//! 它串联扫描、模块树、页面规划、渲染、状态写盘和 metadata 导出。

use std::collections::BTreeMap;
use std::cell::RefCell;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use std::time::SystemTime;

use crate::debug_trace;
use crate::domain::context::PageContext;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::{assemble_state, PageBuildResult};
use crate::domain::steering::load_steering_config;
use crate::generation::context::{build_module_contexts_with_graph, build_repo_context_with_graph};
use crate::generation::planner::plan_pages_with_graph;
use crate::llm::{LlmRuntime, LlmService};
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
use crate::storage::wiki_fs::{remove_runtime_with_cache_mode, write_page};
use crate::workflows::page_render::prepare_page_artifacts_with_llm;
use crate::workflows::progress::{
    NoopProgressSink, ProgressSink, SharedProgressSink, WorkflowProgressEvent, WorkflowReporter,
};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `init` 会全量生成 Repo Wiki 运行时。
/// 它是当前最完整的一条链路：扫描 -> 模块树 -> 页面 -> WikiState -> metadata/cache。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InitReport {
    /// 当前命令是否真正完成了初始化流程。
    pub initialized: bool,
    /// 初始化结束后的 runtime 状态，正常情况为 `fresh`。
    pub state: String,
    /// 本次初始化实际写出的页面路径集合。
    pub generated_pages: Vec<String>,
}

/// 初始化 `.wiki/` 运行时。
/// 当前只要求 `repo_root` 是本地目录，Git 只是可选元信息来源。
///
/// # 参数
/// - `repo_root`：要生成 Repo Wiki 的本地代码目录。
///
/// # 返回
/// - 成功时返回初始化结果，包含最终状态和本次生成的页面列表。
///
/// # 错误
/// - 当目录不存在、不是目录、文件不可读或运行时无法写入时返回错误。
pub fn run_init(repo_root: &Path) -> io::Result<InitReport> {
    let mut sink = NoopProgressSink;
    run_init_with_progress_as("init", repo_root, &mut sink)
}

/// 允许 transport 以指定 action 名称执行 init 主链。
/// `update` 回退到 init 时会复用这条链，但对外仍暴露为 update 进度。
pub fn run_init_with_progress_as(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &mut dyn ProgressSink,
) -> io::Result<InitReport> {
    run_init_with_progress_and_llm_as(action, repo_root, progress_sink, None)
}

/// 在保留旧进度接口的同时，允许 transport 注入可选 LLM 桥接。
///
/// # 参数
/// - `action`：当前 workflow 的对外动作名。
/// - `repo_root`：待生成 Wiki 的仓库根目录。
/// - `progress_sink`：接收阶段进度的下游。
/// - `_llm_service`：可选的 LLM 桥接服务；不可用时自动回退 deterministic。
///
/// # 返回
/// - 成功时返回 init 报告。
pub fn run_init_with_progress_and_llm_as<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    _llm_service: Option<&'a mut dyn LlmService>,
) -> io::Result<InitReport> {
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

    // 按 deterministic pipeline 的顺序串起整条生成链。
    let steering = load_steering_config(repo_root);
    remove_runtime_with_cache_mode(repo_root, steering.llm.cache_mode)?;
    debug_trace::begin_session(action, repo_root, &steering.debug)?;
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
    let pages = plan_pages_with_graph(
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
        &steering,
        Some(&mut llm_runtime),
        llm_enrichment_enabled.then_some(&mut llm_progress as &mut dyn FnMut(usize, usize)),
    );

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
    let generated_at = current_timestamp();
    let mut ancestor_ids_by_page = BTreeMap::new();

    for (index, artifact) in prepared_pages.into_iter().enumerate() {
        write_page(
            repo_root,
            &artifact.page.relative_path,
            &artifact.rendered_page.content,
        )?;
        let page_path = format!(".wiki/{}", artifact.page.relative_path);
        generated_pages.push(page_path);
        let ancestor_ids = ancestor_ids_for_page(&artifact.page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(artifact.page.id.clone(), ancestor_ids.clone());
        let content_hash =
            crate::repo::fingerprint::fingerprint_bytes(artifact.rendered_page.content.as_bytes());

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

    // 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。
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

    Ok(InitReport {
        initialized: true,
        state: state.dirty_state.status,
        generated_pages,
    })
}

/// 根据页面上下文里的 `source_ids` 反查源码路径。
pub(crate) fn source_paths_for_page(
    scan_report: &crate::repo::scanner::ScanReport,
    page_context: &PageContext,
) -> Vec<String> {
    scan_report
        .files
        .iter()
        .filter(|file| page_context.source_ids.contains(&file.id))
        .map(|file| file.path.clone())
        .collect()
}

/// 页面祖先链会直接写入状态层，方便 query 和后续 runtime 读取。
pub(crate) fn ancestor_ids_for_page(
    page: &crate::generation::planner::PlannedPage,
    ancestor_ids_by_page: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let Some(parent_id) = &page.parent_id else {
        return Vec::new();
    };

    let mut ancestor_ids = ancestor_ids_by_page
        .get(parent_id)
        .cloned()
        .unwrap_or_default();
    ancestor_ids.push(parent_id.clone());
    ancestor_ids
}

/// provenance 保留最小可追溯线索，供 query 和后续 runtime 使用。
pub(crate) fn page_provenance(
    page: &crate::generation::planner::PlannedPage,
    page_context: &PageContext,
    scan_report: &crate::repo::scanner::ScanReport,
) -> Vec<String> {
    let mut provenance = BTreeMap::new();

    for module_id in &page.module_ids {
        provenance.insert(format!("module:{module_id}"), ());
    }

    for relation_id in &page.relation_ids {
        provenance.insert(format!("relation:{relation_id}"), ());
    }

    for source_path in source_paths_for_page(scan_report, page_context) {
        provenance.insert(format!("source:{source_path}"), ());
    }

    if page.page_type == "architecture" {
        provenance.insert("scope:architecture".to_string(), ());
    }

    if page.page_type == "overview" {
        provenance.insert("scope:repository".to_string(), ());
    }

    if page.page_type == "topic" {
        if let Some(topic_kind) = &page.topic_kind {
            provenance.insert(format!("topic:{topic_kind}"), ());
        }
        if let Some(topic_key) = &page.topic_key {
            provenance.insert(format!("topic-key:{topic_key}"), ());
        }
    }

    provenance.into_keys().collect()
}

/// 统一生成 RFC3339 时间戳。
/// 如果格式化失败，再退回 Unix 时间，避免时间字段导致整个流程报错。
pub(crate) fn current_timestamp() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| duration.as_secs().to_string())
                .unwrap_or_else(|_| "0".to_string())
        })
}
