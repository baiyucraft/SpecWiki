//! init workflow 负责按全量链路生成第一版 Repo Wiki runtime。
//! 它串联扫描、模块树、页面规划、渲染、状态写盘和 metadata 导出。

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use std::time::SystemTime;

use crate::debug_trace;
use crate::domain::checkpoint::{compute_facts_input_hash, PipelineRuntimeSummary, PipelineStage};
use crate::domain::context::PageContext;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::runtime_profile::{LlmExecutionMode, RuntimeSummaryProjection};
use crate::domain::state::{assemble_state, PageBuildResult};
use crate::domain::steering::LlmCacheMode;
use crate::domain::steering::{
    check_user_config_file, ensure_default_user_config_file, load_steering_config_with_mode,
    SteeringLoadMode,
};
use crate::generation::context::{build_module_contexts_with_graph, build_repo_context_with_graph};
use crate::generation::renderer::render_page_draft;
use crate::llm::{LlmRuntime, LlmService};
use crate::repo::git::{current_branch, current_commit};
use crate::storage::cache_store::{
    ensure_cache_dir, ensure_page_cache_dirs, write_page_context_cache,
    write_page_generation_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::knowledge_artifacts::{persist_knowledge_artifacts, PersistKnowledgeArtifactsInput};
use crate::storage::metadata_store::metadata_exists;
use crate::storage::metadata_store::write_metadata;
use crate::storage::sqlite::runtime_store::SqliteRuntimeStore;
use crate::storage::sqlite_store;
use crate::storage::state_store::{write_facts_snapshot, write_state};
use crate::storage::wiki_fs::{remove_runtime_with_cache_mode, write_page};
use crate::workflows::page_render::{
    finalize_pipeline_runtime, load_runtime_summary_for_repo, persist_runtime_blocker_for_repo,
    plan_runtime_knowledge_tree, run_compose_pipeline_with_action,
};
use crate::workflows::progress::{
    NoopProgressSink, ProgressSink, SharedProgressSink, WorkflowProgressEvent, WorkflowReporter,
};
use crate::workflows::release_scope::{
    persist_index_only_release_scope, v0_1_index_only_enabled, INDEX_ONLY_RUNTIME_STATE,
};
use crate::workflows::research_provider::select_runtime_research_provider;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use wiki_index::hierarchy::build_module_tree_with_graph_and_assist;
use wiki_index::scanner::scan_repo_with_boundary_and_assist;
use wiki_index::symbol_graph::{analyze_symbol_graph, build_graph_summary, resolve_symbol_graph};
use wiki_knowledge::domain::compose::PageDraft;

/// `init` 会全量生成 Repo Wiki 运行时。
/// 它是当前最完整的一条链路：扫描 -> 模块树 -> 页面 -> WikiState -> metadata/cache。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InitReport {
    /// 当前命令是否真正完成了初始化流程。
    pub initialized: bool,
    /// 初始化结束后的 runtime 状态。
    /// `v0.1.0 index-only` 收敛路径会显式返回 `index_only`。
    pub state: String,
    /// 本次初始化实际写出的页面路径集合。
    /// `v0.1.0 index-only` 收敛路径不会产出页面。
    pub generated_pages: Vec<String>,
    /// 当前 workflow 成功结束后的 runtime 摘要。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_summary: Option<RuntimeSummaryProjection>,
    /// 当前长流程真实采用的执行路径。
    pub llm_execution_mode: LlmExecutionMode,
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
    run_init_with_progress_and_llm_as_with_mode(
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
    run_init_with_progress_and_llm_as_with_mode(
        action,
        repo_root,
        progress_sink,
        _llm_service,
        SteeringLoadMode::Production,
    )
}

pub fn run_init_with_progress_and_llm_as_with_mode<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    _llm_service: Option<&'a mut dyn LlmService>,
    steering_mode: SteeringLoadMode,
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
    let index_only_release = v0_1_index_only_enabled(action);

    // 按 deterministic pipeline 的顺序串起整条生成链。
    reporter.phase("user_config", "检查用户配置");
    ensure_default_user_config_file()?;
    check_user_config_file()?;
    let steering = load_steering_config_with_mode(repo_root, steering_mode);
    let should_preserve_incomplete = !index_only_release
        && should_preserve_incomplete_init_runtime(action, repo_root, steering.llm.cache_mode)?;
    if index_only_release || !should_preserve_incomplete {
        remove_runtime_with_cache_mode(repo_root, steering.llm.cache_mode)?;
    }
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
    if index_only_release {
        persist_index_only_release_scope(repo_root)?;
        reporter.phase(
            "v0_1_index_only_short_circuit",
            "v0.1.0 index-only 收敛：跳过 knowledge/page runtime",
        );
        return Ok(InitReport {
            initialized: true,
            state: INDEX_ONLY_RUNTIME_STATE.to_string(),
            generated_pages: Vec::new(),
            runtime_summary: None,
            llm_execution_mode: match llm_runtime.selected_path() {
                Some(crate::llm::SelectedLlmPath::ProviderApi) => LlmExecutionMode::ProviderDirect,
                Some(crate::llm::SelectedLlmPath::AgentBridge) => LlmExecutionMode::AgentBridge,
                None => LlmExecutionMode::DeterministicOnly,
            },
        });
    }
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
    let _digests = pipeline.digests;
    let unit_researches = pipeline.unit_researches;
    let knowledge_tree = pipeline.knowledge_tree;
    let pages = pipeline.planned_pages;
    let pages_by_id: BTreeMap<String, _> =
        pages.iter().map(|p| (p.id.clone(), p.clone())).collect();

    ensure_cache_dir(repo_root)?;
    ensure_page_cache_dirs(repo_root)?;

    // ── Render & Write ──
    let page_total = page_drafts.len();
    reporter.counted("render_pages", "渲染页面", 0, page_total);

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
    let generated_at = current_timestamp();
    let mut ancestor_ids_by_page = BTreeMap::new();

    for (index, draft) in page_drafts.iter().enumerate() {
        let rendered = render_page_draft(draft);

        write_page(repo_root, &draft.relative_path, &rendered.content)?;
        let page_path = format!(".wiki/{}", draft.relative_path);
        generated_pages.push(page_path);

        let content_hash = wiki_index::fingerprint::fingerprint_bytes(rendered.content.as_bytes());

        let planned_page = find_or_build_planned_page(draft, &pages_by_id);
        let page_context = build_minimal_page_context(
            draft,
            &planned_page,
            &knowledge_tree,
            &_digests,
            &unit_researches,
        );
        let input_hash = wiki_index::fingerprint::fingerprint_bytes(
            format!("{}:{}", draft.page_id, draft.citation_count).as_bytes(),
        );

        let ancestor_ids = ancestor_ids_for_page(&planned_page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(planned_page.id.clone(), ancestor_ids.clone());

        let summary = _digests
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

    // page_id 去重——知识树中极端情况可能产生路径冲突
    {
        let mut seen = std::collections::HashSet::new();
        page_results.retain(|r| seen.insert(r.page.id.clone()));
    }

    // 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。
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
    let page_digests = _digests.values().cloned().collect::<Vec<_>>();
    let runtime_gates = runtime_store.read_unit_runtime_gates()?;
    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: action,
        generated_at: &generated_at,
        facts_input_hash: &facts_input_hash,
        metadata: &metadata,
        knowledge_tree: &knowledge_tree,
        research_summaries: &research_summaries,
        page_digests: &page_digests,
        runtime_gates: &runtime_gates,
    })?;
    let runtime_summary =
        load_runtime_summary_for_repo(repo_root)?.map(RuntimeSummaryProjection::from_summary);
    runtime_store.clear_pipeline_checkpoint()?;

    Ok(InitReport {
        initialized: true,
        state: state.dirty_state.status,
        generated_pages,
        runtime_summary,
        llm_execution_mode: match llm_runtime.selected_path() {
            Some(crate::llm::SelectedLlmPath::ProviderApi) => LlmExecutionMode::ProviderDirect,
            Some(crate::llm::SelectedLlmPath::AgentBridge) => LlmExecutionMode::AgentBridge,
            None => LlmExecutionMode::DeterministicOnly,
        },
    })
}

fn should_preserve_incomplete_init_runtime(
    action: &str,
    repo_root: &Path,
    cache_mode: LlmCacheMode,
) -> io::Result<bool> {
    if action != "init" || cache_mode == LlmCacheMode::Clear {
        return Ok(false);
    }

    if metadata_exists(repo_root) {
        return Ok(false);
    }

    let db_path = sqlite_store::db_path(repo_root);
    if !db_path.exists() {
        return Ok(false);
    }

    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    let Some(raw_summary) = runtime_store.runtime_meta_get("pipeline_runtime_summary")? else {
        // preserve-resume 的首要目标是避免在 timeout 后立即清理 `.wiki/.cache`，
        // 否则 Windows 上旧进程尚未释放句柄时会把第二次 init 直接打成 os error 32。
        // 只有在 metadata/markdown 都还没出现、且 summary 尚未成型时，才允许这个最宽松的保留窗口。
        return Ok(count_existing_markdown_pages(repo_root.join(".wiki").as_path())? == 0);
    };
    let runtime_summary: PipelineRuntimeSummary = serde_json::from_str(&raw_summary)
        .map_err(|error| io::Error::other(format!("deserialize runtime summary: {error}")))?;
    if runtime_summary.workflow_action != action {
        return Ok(false);
    }
    if !matches!(
        runtime_summary.runtime_state.as_str(),
        "researching" | "compose_pending" | "compose_complete" | "interrupted"
    ) {
        return Ok(false);
    }

    // metadata 还没产出时，即使已经写出部分 markdown，也可能只是 assemble 尚未收尾。
    Ok(!runtime_store.read_unit_runtime_gates()?.is_empty())
}

fn count_existing_markdown_pages(wiki_root: &Path) -> io::Result<usize> {
    if !wiki_root.exists() {
        return Ok(0);
    }

    let mut count = 0usize;
    for entry in std::fs::read_dir(wiki_root)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            if entry.file_name() == ".cache" {
                continue;
            }
            count += count_existing_markdown_pages(&path)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            count += 1;
        }
    }

    Ok(count)
}

/// 根据页面上下文里的 `source_ids` 反查源码路径。
pub(crate) fn source_paths_for_page(
    scan_report: &wiki_index::scanner::ScanReport,
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
    page: &wiki_knowledge::PlannedPage,
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
    page: &wiki_knowledge::PlannedPage,
    page_context: &PageContext,
    scan_report: &wiki_index::scanner::ScanReport,
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
        provenance.insert(format!("scope:{}", page.scope), ());
    }

    if page.page_type == "domain-index" || page.scope == "domain" || page.scope == "unit" {
        provenance.insert(format!("scope:{}", page.scope), ());
    }

    provenance.into_keys().collect()
}

/// 从 pages_by_id 中查找对应的 PlannedPage，找不到时基于 PageDraft 构建。
pub(crate) fn find_or_build_planned_page(
    draft: &PageDraft,
    pages_by_id: &BTreeMap<String, wiki_knowledge::PlannedPage>,
) -> wiki_knowledge::PlannedPage {
    if let Some(page) = pages_by_id.get(&draft.page_id) {
        return page.clone();
    }
    wiki_knowledge::PlannedPage {
        id: draft.page_id.clone(),
        title: draft.title.clone(),
        page_type: "module".to_string(),
        relative_path: draft.relative_path.clone(),
        parent_id: None,
        source_ids: Vec::new(),
        module_ids: Vec::new(),
        merged_module_ids: Vec::new(),
        relation_ids: Vec::new(),
        scope: "unit".to_string(),
        priority: 0,
        generation_mode: "compose".to_string(),
        unit_id: Some(draft.unit_id.clone()),
        unit_type: None,
        domain_id: None,
    }
}

/// 基于 PageDraft 和 KnowledgeTree 构建最小 PageContext。
/// 新 pipeline 中 PageContext 主要承载 source_ids 供状态层持久化。
pub(crate) fn build_minimal_page_context(
    draft: &PageDraft,
    planned_page: &wiki_knowledge::PlannedPage,
    knowledge_tree: &crate::domain::knowledge::KnowledgeTree,
    digests: &BTreeMap<String, wiki_knowledge::domain::research::PageDigest>,
    unit_researches: &BTreeMap<String, wiki_knowledge::domain::research::UnitResearch>,
) -> PageContext {
    let mut ctx = PageContext::default();
    if let Some(unit) = knowledge_tree.get_unit(&draft.unit_id) {
        let child_digests = crate::workflows::page_render::collect_compose_input_digests(
            unit,
            knowledge_tree,
            digests,
        );
        ctx.page_id = planned_page.id.clone();
        ctx.page_type = planned_page.page_type.clone();
        ctx.scope = planned_page.scope.clone();
        ctx.unit_id = planned_page.unit_id.clone();
        ctx.unit_type = planned_page.unit_type.clone();
        ctx.domain_id = planned_page.domain_id.clone();
        ctx.source_ids = unit.scope.source_ids.clone();
        ctx.module_ids = unit.scope.module_ids.clone();
        ctx.relation_ids = unit.scope.relation_ids.clone();
        ctx.facts = vec![
            format!("unit={}", unit.id),
            format!("unit_type={}", unit.unit_type.as_str()),
            format!("relative_path={}", unit.relative_path),
        ];
        ctx.summary_inputs = vec![draft.title.clone()];
        ctx.child_summaries = child_digests
            .iter()
            .map(|digest| format!("{}: {}", digest.title, digest.summary))
            .collect();
        ctx.child_unit_ids = child_digests
            .iter()
            .map(|digest| digest.unit_id.clone())
            .collect();
        ctx.child_page_ids = child_digests
            .iter()
            .map(|digest| digest.page_id.clone())
            .collect();
        ctx.child_digest_ids = child_digests
            .iter()
            .map(|digest| digest.digest_id.clone())
            .collect();
        ctx.missing_child_unit_ids = unit
            .child_unit_ids
            .iter()
            .filter(|child_id| {
                !ctx.child_unit_ids
                    .iter()
                    .any(|existing| existing == *child_id)
            })
            .cloned()
            .collect();
        ctx.readiness_status = if ctx.missing_child_unit_ids.is_empty() {
            "compose_ready".to_string()
        } else {
            "waiting_children".to_string()
        };
        if let Some(unit_research) = unit_researches.get(&unit.id) {
            ctx.has_unit_research_contract = true;
            if !unit_research.input_hash.trim().is_empty() {
                ctx.unit_research_input_hash = Some(unit_research.input_hash.clone());
            }
        }
        ctx.citation_digest_refs = child_digests
            .iter()
            .flat_map(|digest| {
                digest
                    .section_digests
                    .iter()
                    .filter(|section| !section.citations.is_empty())
                    .map(|section| section.digest_id.clone())
            })
            .collect();
        ctx.diagram_digest_refs = child_digests
            .iter()
            .flat_map(|digest| {
                digest
                    .diagram_digests
                    .iter()
                    .map(|diagram| diagram.digest_id.clone())
            })
            .collect();
    }
    ctx
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;
    use std::sync::Mutex;

    use tempfile::tempdir;

    use super::{
        build_minimal_page_context, run_init_with_progress_as,
        should_preserve_incomplete_init_runtime,
    };
    use crate::domain::checkpoint::UnitRuntimeGate;
    use crate::domain::knowledge::{
        DomainType, KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitType,
    };
    use crate::domain::steering::{spec_wiki_user_config_path, LlmCacheMode};
    use crate::storage::sqlite_store;
    use crate::workflows::progress::NoopProgressSink;
    use crate::workflows::release_scope::V0_1_INDEX_ONLY_ENV;
    use wiki_knowledge::domain::compose::PageDraft;
    use wiki_knowledge::domain::research::{
        PageDiagramDigest, PageDigest, PageSectionDigest, SourceCitation, UnitResearch,
    };
    use wiki_knowledge::plan_pages_from_knowledge_tree;

    static HOME_ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set_path(key: &'static str, value: &Path) -> Self {
            let previous = std::env::var_os(key);
            std::env::set_var(key, value.as_os_str());
            Self { key, previous }
        }

        fn set_str(key: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(key);
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(previous) = self.previous.as_ref() {
                std::env::set_var(self.key, previous);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn build_minimal_page_context_persists_child_contract_and_unit_identity() {
        let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心模块");
        let mut parent = KnowledgeUnit::new(
            UnitType::DomainIndex,
            "核心模块",
            domain.id.clone(),
            "核心模块/核心模块.md",
        );
        let child = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心模块/运行时.md",
        );
        parent.child_unit_ids.push(child.id.clone());

        let mut tree = KnowledgeTree::new(parent.id.clone());
        tree.add_domain(domain);
        tree.add_unit(parent.clone());
        tree.add_unit(child.clone());
        tree.processing_order = vec![child.id.clone(), parent.id.clone()];

        let planned_page = plan_pages_from_knowledge_tree(&tree)
            .into_iter()
            .find(|page| {
                page.id == crate::domain::stable_id::stable_id("page", &parent.relative_path)
            })
            .expect("parent page should be planned");
        let draft = PageDraft {
            page_id: planned_page.id.clone(),
            unit_id: parent.id.clone(),
            title: parent.title.clone(),
            relative_path: parent.relative_path.clone(),
            sections: Vec::new(),
            diagrams: Vec::new(),
            citation_count: 0,
        };
        let child_digest = PageDigest {
            digest_id: "digest-child-runtime".to_string(),
            unit_id: child.id.clone(),
            page_id: crate::domain::stable_id::stable_id("page", &child.relative_path),
            title: child.title.clone(),
            summary: "负责主运行时流程。".to_string(),
            key_sources: vec!["src/runtime.rs".to_string()],
            section_digests: vec![PageSectionDigest {
                digest_id: "section-runtime".to_string(),
                section_key: "runtime-flow".to_string(),
                title: "运行时流程".to_string(),
                summary: "按阶段推进主流程。".to_string(),
                key_sources: vec!["src/runtime.rs".to_string()],
                citations: vec![SourceCitation {
                    path: "src/runtime.rs".to_string(),
                    start_line: 10,
                    end_line: 24,
                    source_id: Some("source-runtime".to_string()),
                    symbol_id: None,
                    note: "主入口".to_string(),
                }],
            }],
            diagram_digests: vec![PageDiagramDigest {
                digest_id: "diagram-runtime".to_string(),
                diagram_type: "flow".to_string(),
                title: "运行时流".to_string(),
                summary: "阶段关系".to_string(),
            }],
            readiness_stage: "compose_ready".to_string(),
            ..PageDigest::default()
        };
        let digests = BTreeMap::from([(child.id.clone(), child_digest)]);

        let unit_researches = BTreeMap::from([(
            parent.id.clone(),
            UnitResearch {
                unit_id: parent.id.clone(),
                input_hash: "parent-research-hash".to_string(),
                ..UnitResearch::default()
            },
        )]);

        let context =
            build_minimal_page_context(&draft, &planned_page, &tree, &digests, &unit_researches);

        assert_eq!(context.unit_id.as_deref(), Some(parent.id.as_str()));
        assert_eq!(
            context.unit_type.as_deref(),
            Some(parent.unit_type.as_str())
        );
        assert_eq!(
            context.domain_id.as_deref(),
            Some(parent.domain_id.as_str())
        );
        assert_eq!(context.child_unit_ids, vec![child.id.clone()]);
        assert_eq!(
            context.child_page_ids,
            vec![crate::domain::stable_id::stable_id(
                "page",
                &child.relative_path
            )]
        );
        assert_eq!(
            context.child_digest_ids,
            vec!["digest-child-runtime".to_string()]
        );
        assert_eq!(context.readiness_status, "compose_ready");
        assert!(context.has_unit_research_contract);
        assert_eq!(
            context.unit_research_input_hash.as_deref(),
            Some("parent-research-hash")
        );
        assert!(context.missing_child_unit_ids.is_empty());
        assert_eq!(
            context.citation_digest_refs,
            vec!["section-runtime".to_string()]
        );
        assert_eq!(
            context.diagram_digest_refs,
            vec!["diagram-runtime".to_string()]
        );
    }

    #[test]
    fn build_minimal_page_context_marks_missing_child_rollup_as_waiting() {
        let parent = KnowledgeUnit::new(
            UnitType::DomainIndex,
            "核心模块",
            "domain-runtime",
            "核心模块/核心模块.md",
        );
        let mut tree = KnowledgeTree::new(parent.id.clone());
        let child = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心模块/运行时.md",
        );
        let mut parent_with_child = parent.clone();
        parent_with_child.child_unit_ids = vec![child.id.clone()];
        tree.add_unit(parent_with_child.clone());
        tree.add_unit(child.clone());
        let draft = PageDraft {
            page_id: crate::domain::stable_id::stable_id("page", &parent_with_child.relative_path),
            unit_id: parent_with_child.id.clone(),
            title: parent_with_child.title.clone(),
            relative_path: parent_with_child.relative_path.clone(),
            sections: Vec::new(),
            diagrams: Vec::new(),
            citation_count: 0,
        };
        let planned_page = super::find_or_build_planned_page(&draft, &BTreeMap::new());
        let unit_researches = BTreeMap::from([(
            parent_with_child.id.clone(),
            UnitResearch {
                unit_id: parent_with_child.id.clone(),
                input_hash: "missing-child-hash".to_string(),
                ..UnitResearch::default()
            },
        )]);

        let context = build_minimal_page_context(
            &draft,
            &planned_page,
            &tree,
            &BTreeMap::new(),
            &unit_researches,
        );

        assert_eq!(context.readiness_status, "waiting_children");
        assert_eq!(context.missing_child_unit_ids, vec![child.id.clone()]);
        assert!(context.has_unit_research_contract);
    }

    #[test]
    fn incomplete_init_runtime_is_preserved_only_for_narrow_resume_window() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let wiki_root = repo_root.join(".wiki");
        fs::create_dir_all(wiki_root.join(".cache")).unwrap();

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心运行时");
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心运行时/运行时.md",
        );
        sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
        sqlite_store::write_knowledge_units(&conn, &[unit.clone()]).unwrap();
        sqlite_store::runtime_meta_set(
            &conn,
            "pipeline_runtime_summary",
            &serde_json::json!({
                "facts_input_hash": "facts-same",
                "workflow_action": "init",
                "runtime_state": "researching"
            })
            .to_string(),
        )
        .unwrap();
        sqlite_store::write_unit_runtime_gate(
            &conn,
            &UnitRuntimeGate {
                unit_id: unit.id.clone(),
                unit_type: unit.unit_type.as_str().to_string(),
                research_status: "ready".to_string(),
                compose_status: "ready".to_string(),
                assemble_status: "pending".to_string(),
                last_ready_stage: Some("research_unit".to_string()),
                blocked_reason: None,
                missing_dependencies: Vec::new(),
                updated_at: "1".to_string(),
            },
        )
        .unwrap();

        assert!(
            should_preserve_incomplete_init_runtime("init", repo_root, LlmCacheMode::Preserve)
                .unwrap()
        );
        assert!(!should_preserve_incomplete_init_runtime(
            "rebuild",
            repo_root,
            LlmCacheMode::Preserve
        )
        .unwrap());
        assert!(
            !should_preserve_incomplete_init_runtime("init", repo_root, LlmCacheMode::Clear)
                .unwrap()
        );
        sqlite_store::runtime_meta_set(
            &conn,
            "pipeline_runtime_summary",
            &serde_json::json!({
                "facts_input_hash": "facts-same",
                "workflow_action": "rebuild",
                "runtime_state": "researching"
            })
            .to_string(),
        )
        .unwrap();
        assert!(!should_preserve_incomplete_init_runtime(
            "init",
            repo_root,
            LlmCacheMode::Preserve
        )
        .unwrap());
    }

    #[test]
    fn partial_markdown_compose_complete_runtime_is_preserved_for_assemble_resume() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let wiki_root = repo_root.join(".wiki");
        fs::create_dir_all(wiki_root.join(".cache")).unwrap();

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心运行时");
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心运行时/运行时.md",
        );
        sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
        sqlite_store::write_knowledge_units(&conn, &[unit.clone()]).unwrap();
        sqlite_store::runtime_meta_set(
            &conn,
            "pipeline_runtime_summary",
            &serde_json::json!({
                "facts_input_hash": "facts-same",
                "workflow_action": "init",
                "runtime_state": "compose_complete"
            })
            .to_string(),
        )
        .unwrap();
        sqlite_store::write_unit_runtime_gate(
            &conn,
            &UnitRuntimeGate {
                unit_id: unit.id.clone(),
                unit_type: unit.unit_type.as_str().to_string(),
                research_status: "ready".to_string(),
                compose_status: "pending".to_string(),
                assemble_status: "pending".to_string(),
                last_ready_stage: Some("research_unit".to_string()),
                blocked_reason: None,
                missing_dependencies: Vec::new(),
                updated_at: "1".to_string(),
            },
        )
        .unwrap();

        fs::write(wiki_root.join("运行时.md"), "# runtime\n").unwrap();
        assert!(should_preserve_incomplete_init_runtime(
            "init",
            repo_root,
            LlmCacheMode::Preserve
        )
        .unwrap());
    }

    #[test]
    fn init_runtime_is_not_preserved_once_metadata_exists() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let wiki_root = repo_root.join(".wiki");
        fs::create_dir_all(wiki_root.join(".cache")).unwrap();

        let conn = sqlite_store::open_db(repo_root).unwrap();
        let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心运行时");
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            domain.id.clone(),
            "核心运行时/运行时.md",
        );
        sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
        sqlite_store::write_knowledge_units(&conn, &[unit.clone()]).unwrap();
        sqlite_store::runtime_meta_set(
            &conn,
            "pipeline_runtime_summary",
            &serde_json::json!({
                "facts_input_hash": "facts-same",
                "workflow_action": "init",
                "runtime_state": "compose_complete"
            })
            .to_string(),
        )
        .unwrap();
        sqlite_store::write_unit_runtime_gate(
            &conn,
            &UnitRuntimeGate {
                unit_id: unit.id.clone(),
                unit_type: unit.unit_type.as_str().to_string(),
                research_status: "ready".to_string(),
                compose_status: "done".to_string(),
                assemble_status: "pending".to_string(),
                last_ready_stage: Some("compose_leaf".to_string()),
                blocked_reason: None,
                missing_dependencies: Vec::new(),
                updated_at: "1".to_string(),
            },
        )
        .unwrap();

        fs::write(wiki_root.join("wiki.metadata.json"), "{}").unwrap();
        assert!(!should_preserve_incomplete_init_runtime(
            "init",
            repo_root,
            LlmCacheMode::Preserve
        )
        .unwrap());
    }

    #[test]
    fn incomplete_init_runtime_is_preserved_when_only_cache_db_exists() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        fs::create_dir_all(repo_root.join(".wiki/.cache")).unwrap();
        let _conn = sqlite_store::open_db(repo_root).unwrap();

        assert!(
            should_preserve_incomplete_init_runtime("init", repo_root, LlmCacheMode::Preserve)
                .unwrap()
        );
    }

    #[test]
    fn partial_markdown_runtime_is_not_preserved_when_runtime_gates_are_missing() {
        let fixture = tempdir().unwrap();
        let repo_root = fixture.path();
        let wiki_root = repo_root.join(".wiki");
        fs::create_dir_all(wiki_root.join(".cache")).unwrap();

        let conn = sqlite_store::open_db(repo_root).unwrap();
        sqlite_store::runtime_meta_set(
            &conn,
            "pipeline_runtime_summary",
            &serde_json::json!({
                "facts_input_hash": "facts-same",
                "workflow_action": "init",
                "runtime_state": "compose_complete"
            })
            .to_string(),
        )
        .unwrap();
        fs::write(wiki_root.join("运行时.md"), "# runtime\n").unwrap();

        assert!(!should_preserve_incomplete_init_runtime(
            "init",
            repo_root,
            LlmCacheMode::Preserve
        )
        .unwrap());
    }

    #[test]
    fn init_creates_default_user_config_when_missing() {
        let _home_lock = HOME_ENV_LOCK.lock().unwrap();
        let home = tempdir().unwrap();
        let _home_guard = EnvVarGuard::set_path("HOME", home.path());
        let _userprofile_guard = EnvVarGuard::set_path("USERPROFILE", home.path());
        let _index_only_guard = EnvVarGuard::set_str(V0_1_INDEX_ONLY_ENV, "1");

        let repo = tempdir().unwrap();
        fs::write(
            repo.path().join("package.json"),
            r#"{"name":"config-bootstrap","private":true}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.path().join("src")).unwrap();
        fs::write(repo.path().join("src/main.ts"), "export const main = 1;\n").unwrap();

        let mut sink = NoopProgressSink;
        let report = run_init_with_progress_as("init", repo.path(), &mut sink).unwrap();
        let config_path = spec_wiki_user_config_path().expect("expected user config path");
        let content = fs::read_to_string(&config_path).unwrap();

        assert!(report.initialized);
        assert!(config_path.exists());
        assert!(content.contains("debug: {}"));
        assert!(content.contains("llm: {}"));
    }

    #[test]
    fn init_fails_when_user_config_is_invalid() {
        let _home_lock = HOME_ENV_LOCK.lock().unwrap();
        let home = tempdir().unwrap();
        let _home_guard = EnvVarGuard::set_path("HOME", home.path());
        let _userprofile_guard = EnvVarGuard::set_path("USERPROFILE", home.path());
        let _index_only_guard = EnvVarGuard::set_str(V0_1_INDEX_ONLY_ENV, "1");

        let user_dir = home.path().join(".spec-wiki");
        fs::create_dir_all(&user_dir).unwrap();
        fs::write(user_dir.join("config.yaml"), "{{{{not valid yaml").unwrap();

        let repo = tempdir().unwrap();
        fs::write(
            repo.path().join("package.json"),
            r#"{"name":"config-bootstrap","private":true}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.path().join("src")).unwrap();
        fs::write(repo.path().join("src/main.ts"), "export const main = 1;\n").unwrap();

        let mut sink = NoopProgressSink;
        let error = run_init_with_progress_as("init", repo.path(), &mut sink)
            .expect_err("invalid user config should fail init");
        assert!(error.to_string().contains("failed to parse"));
    }
}
