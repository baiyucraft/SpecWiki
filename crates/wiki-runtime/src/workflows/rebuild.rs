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
use crate::domain::checkpoint::{compute_facts_input_hash, PipelineStage};
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::runtime_profile::{LlmExecutionMode, RuntimeSummaryProjection};
use crate::domain::state::{assemble_state, PageBuildResult};
use crate::domain::steering::{load_steering_config_with_mode, SteeringLoadMode};
use crate::generation::context::{build_module_contexts_with_graph, build_repo_context_with_graph};
use crate::generation::managed_sections::{
    merge_sections, parse_wiki_page, ManagedSectionBlock, PageBlock,
};
use crate::generation::renderer::{assemble_page_from_merge, render_page_draft};
use crate::generation::sections::section_titles_for_page_type;
use crate::llm::{LlmRuntime, LlmService};
use crate::repo::git::{current_branch, current_commit};
use crate::storage::cache_store::{
    ensure_cache_dir, ensure_page_cache_dirs, write_page_context_cache,
    write_page_generation_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::knowledge_artifacts::{
    load_knowledge_artifacts, persist_knowledge_artifacts, PersistKnowledgeArtifactsInput,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::sqlite::runtime_store::SqliteRuntimeStore;
use crate::storage::sqlite_store;
use crate::storage::state_store::{write_facts_snapshot, write_state};
use crate::storage::wiki_fs::{resolve_page_path, write_page};
use crate::workflows::init::{
    ancestor_ids_for_page, build_minimal_page_context, current_timestamp,
    find_or_build_planned_page, page_provenance, source_paths_for_page,
};
use crate::workflows::page_render::{
    finalize_pipeline_runtime, load_runtime_summary_for_repo, persist_runtime_blocker_for_repo,
    plan_runtime_knowledge_tree, run_compose_pipeline_with_action,
};
use crate::workflows::progress::{
    NoopProgressSink, ProgressSink, SharedProgressSink, WorkflowProgressEvent, WorkflowReporter,
};
use crate::workflows::research_provider::select_runtime_research_provider;
use wiki_index::fingerprint::fingerprint_bytes;
use wiki_index::hierarchy::build_module_tree_with_graph_and_assist;
use wiki_index::scanner::scan_repo_with_boundary_and_assist;
use wiki_index::symbol_graph::{analyze_symbol_graph, build_graph_summary, resolve_symbol_graph};

/// `rebuild` 是显式的"强制重建"入口。
/// 迭代 5 之后，rebuild 会保留同 page_id 页面中已同步的 user sections。
#[derive(Debug, Clone, Serialize)]
pub struct RebuildReport {
    pub state: String,
    pub updated_pages: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_summary: Option<RuntimeSummaryProjection>,
    pub llm_execution_mode: LlmExecutionMode,
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
    run_rebuild_with_progress_and_llm_as_with_mode(
        action,
        repo_root,
        progress_sink,
        None,
        SteeringLoadMode::Production,
    )
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
    run_rebuild_with_progress_and_llm_as_with_mode(
        action,
        repo_root,
        progress_sink,
        _llm_service,
        SteeringLoadMode::Production,
    )
}

pub fn run_rebuild_with_progress_and_llm_as_with_mode<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    _llm_service: Option<&'a mut dyn LlmService>,
    steering_mode: SteeringLoadMode,
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

    let steering = load_steering_config_with_mode(repo_root, steering_mode);

    // 在清理前，读取旧页面的磁盘内容用于 user section 恢复
    let old_page_contents = read_old_page_contents(repo_root);
    let previous_artifacts = load_knowledge_artifacts(repo_root).ok();

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
    let scan_report = scan_repo_with_boundary_and_assist(
        repo_root,
        ignore_paths,
        include_paths,
        Some(&mut llm_runtime),
    )?;
    let symbol_total = wiki_index::symbols::symbol_parse_file_count(&scan_report, &[]);
    reporter.counted("parse_symbols", "解析源码符号", 0, symbol_total);
    let symbol_snapshot = wiki_index::symbols::parse_symbols_with_progress(
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
    let module_tree = build_module_tree_with_graph_and_assist(
        &scan_report,
        &graph_summary,
        Some(&mut llm_runtime),
    );
    reporter.phase("write_facts_snapshot", "提交 facts snapshot");
    write_facts_snapshot(
        repo_root,
        &scan_report,
        &module_tree,
        &symbol_snapshot.symbols,
        &resolved_graph,
        &analysis,
    )?;
    reporter.phase("build_contexts", "构建页面上下文");
    let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
    let module_contexts =
        build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);

    // ── Layer 2-4: Knowledge Planning → Research → Compose ──
    reporter.phase("knowledge_planning", "知识域发现与单元规划");
    reporter.phase("research", "执行分层研究");
    reporter.phase("compose", "组合生成页面内容");
    let research_provider =
        select_runtime_research_provider(&steering, &mut llm_runtime, steering_mode);
    reporter.phase(
        "research_provider",
        format!(
            "{} (mode={})",
            research_provider.summary, research_provider.mode
        ),
    );
    let Some(research_provider_impl) = research_provider.provider.as_ref() else {
        let facts_input_hash = compute_facts_input_hash(&scan_report, &module_tree);
        let knowledge_tree = plan_runtime_knowledge_tree(
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &graph_summary,
            &steering,
        );
        let reason = research_provider
            .blocked_reason
            .as_deref()
            .unwrap_or("provider research blocked by production policy");
        persist_runtime_blocker_for_repo(
            repo_root,
            action,
            &facts_input_hash,
            PipelineStage::ResearchSystem,
            reason,
            Some(&knowledge_tree),
        )?;
        return Err(io::Error::other(reason.to_string()));
    };
    let pipeline = run_compose_pipeline_with_action(
        action,
        repo_root,
        &scan_report,
        &module_tree,
        &repo_context,
        &module_contexts,
        &symbol_snapshot,
        &resolved_graph,
        &analysis,
        &graph_summary,
        &steering,
        research_provider_impl.as_ref(),
    )?;
    drop(research_provider);
    let page_drafts = pipeline.page_drafts;
    let digests = pipeline.digests;
    let unit_researches = pipeline.unit_researches;
    let knowledge_tree = pipeline.knowledge_tree;
    let pages_by_id: BTreeMap<String, _> = pipeline
        .planned_pages
        .iter()
        .map(|p| (p.id.clone(), p.clone()))
        .collect();

    ensure_cache_dir(repo_root)?;
    ensure_page_cache_dirs(repo_root)?;

    let page_total = page_drafts.len();
    reporter.counted("render_pages", "渲染页面", 0, page_total);

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
    let mut all_warnings = Vec::new();
    let generated_at = current_timestamp();
    let mut ancestor_ids_by_page = BTreeMap::new();

    for (index, draft) in page_drafts.iter().enumerate() {
        let rendered = render_page_draft(draft);
        let planned_page = find_or_build_planned_page(draft, &pages_by_id);
        let page_context = build_minimal_page_context(
            draft,
            &planned_page,
            &knowledge_tree,
            &digests,
            &unit_researches,
        );

        let final_content = match old_page_contents.get(&planned_page.id) {
            Some(old_content) => merge_old_user_sections(
                &planned_page,
                &rendered.sections,
                &rendered.content,
                old_content,
                &mut all_warnings,
            ),
            None => rendered.content.clone(),
        };

        write_page(repo_root, &draft.relative_path, &final_content)?;
        let page_path = format!(".wiki/{}", draft.relative_path);
        generated_pages.push(page_path);
        let ancestor_ids = ancestor_ids_for_page(&planned_page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(planned_page.id.clone(), ancestor_ids.clone());
        let content_hash = fingerprint_bytes(final_content.as_bytes());
        let input_hash = wiki_index::fingerprint::fingerprint_bytes(
            format!("{}:{}", draft.page_id, draft.citation_count).as_bytes(),
        );

        let summary = digests
            .get(&draft.unit_id)
            .map(|d| d.summary.clone())
            .unwrap_or_default();

        write_page_context_cache(
            repo_root,
            &PageContextCacheEntry {
                page_id: planned_page.id.clone(),
                input_hash: input_hash.clone(),
                context: page_context.clone(),
            },
        )?;
        write_page_generation_cache(
            repo_root,
            &PageGenerationCacheEntry {
                page_id: planned_page.id.clone(),
                input_hash: input_hash.clone(),
                content_hash: content_hash.clone(),
                sections: rendered.sections.clone(),
            },
        )?;

        page_results.push(PageBuildResult {
            page: planned_page.clone(),
            context: page_context.clone(),
            summary,
            input_hash,
            content_hash,
            source_paths: source_paths_for_page(&scan_report, &page_context),
            ancestor_ids,
            provenance: page_provenance(&planned_page, &page_context, &scan_report),
            sections: rendered.sections,
        });

        reporter.counted(
            "render_pages",
            format!("渲染页面 {}/{}", index + 1, page_total),
            index + 1,
            page_total,
        );
    }

    // page_id 去重
    {
        let mut seen = std::collections::HashSet::new();
        page_results.retain(|r| seen.insert(r.page.id.clone()));
    }

    let state = assemble_state(&page_results, &scan_report, &module_tree, &generated_at);
    reporter.phase("write_state", "写入运行时状态");
    write_state(repo_root, &state)?;

    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at: generated_at.clone(),
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&state, &export_context);
    reporter.phase("write_metadata", "写入元数据");
    write_metadata(repo_root, &metadata)?;
    finalize_pipeline_runtime(repo_root, action, generated_pages.len())?;
    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    let facts_input_hash = compute_facts_input_hash(&scan_report, &module_tree);
    let research_summaries = knowledge_tree
        .units
        .values()
        .filter_map(|unit| {
            unit_researches
                .get(&unit.id)
                .map(|research| research.to_artifact_summary(unit))
        })
        .collect::<Vec<_>>();
    let current_unit_ids = knowledge_tree
        .units
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let declared_records = previous_artifacts
        .as_ref()
        .map(|artifacts| {
            artifacts
                .declared_records
                .iter()
                .filter(|record| {
                    record.unit_refs.is_empty()
                        || record
                            .unit_refs
                            .iter()
                            .any(|unit_id| current_unit_ids.contains(unit_id))
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let health_signals = Vec::new();
    let page_digests = digests.values().cloned().collect::<Vec<_>>();
    let runtime_gates = runtime_store.read_unit_runtime_gates()?;
    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: action,
        generated_at: &generated_at,
        facts_input_hash: &facts_input_hash,
        metadata: &metadata,
        knowledge_tree: &knowledge_tree,
        declared_records: &declared_records,
        research_summaries: &research_summaries,
        page_digests: &page_digests,
        runtime_gates: &runtime_gates,
        health_signals: &health_signals,
    })?;
    let runtime_summary =
        load_runtime_summary_for_repo(repo_root)?.map(RuntimeSummaryProjection::from_summary);
    runtime_store.clear_pipeline_checkpoint()?;

    Ok(RebuildReport {
        state: state.dirty_state.status,
        updated_pages: generated_pages,
        runtime_summary,
        llm_execution_mode: match llm_runtime.selected_path() {
            Some(crate::llm::SelectedLlmPath::ProviderApi) => LlmExecutionMode::ProviderDirect,
            Some(crate::llm::SelectedLlmPath::AgentBridge) => LlmExecutionMode::AgentBridge,
            None => LlmExecutionMode::DeterministicOnly,
        },
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
    planned_page: &wiki_knowledge::PlannedPage,
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
