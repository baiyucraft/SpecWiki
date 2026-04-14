//! update workflow 负责把 `stale` runtime 增量刷新回 `fresh`。
//! 它优先局部重建受影响页面，并在必要时回退到 init 或 rebuild。

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use serde::Serialize;

use crate::debug_trace;
use crate::domain::change_set::{plan_runtime_changes_with_mode, ChangePlan, FallbackMode};
use crate::domain::checkpoint::{compute_facts_input_hash, PipelineStage};
use crate::domain::metadata::DirtyState;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::runtime_profile::{LlmExecutionMode, RuntimeSummaryProjection};
use crate::domain::state::{assemble_state_from_pages, build_page_state, PageBuildResult};
use crate::domain::steering::{load_steering_config_with_mode, SteeringLoadMode};
use crate::generation::context::{build_module_contexts_with_graph, build_repo_context_with_graph};
use crate::generation::managed_sections::{merge_sections, parse_wiki_page, ManagedSectionBlock};
use crate::generation::renderer::{assemble_page_from_merge, render_page_draft};
use crate::generation::sections::section_titles_for_page_type;
use crate::llm::{LlmRuntime, LlmService};
use crate::repo::git::{current_branch, current_commit};
use crate::storage::cache_store::{
    remove_page_caches, write_page_context_cache, write_page_generation_cache,
    PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::knowledge_artifacts::{
    load_knowledge_artifacts, persist_knowledge_artifacts, restore_runtime_cache_from_artifacts,
    KnowledgeArtifactSnapshot, PersistKnowledgeArtifactsInput,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::sqlite::{index_store::SqliteIndexStore, runtime_store::SqliteRuntimeStore};
use crate::storage::sqlite_store;
use crate::storage::state_store::{
    facts_snapshot_ready, write_facts_snapshot, write_facts_snapshot_for_files, write_state,
};
use crate::storage::wiki_fs::{resolve_page_path, write_page};
use crate::workflows::init::{
    ancestor_ids_for_page, build_minimal_page_context, current_timestamp, page_provenance,
    run_init_with_progress_and_llm_as_with_mode, source_paths_for_page,
};
use crate::workflows::page_render::{
    finalize_pipeline_runtime, load_runtime_summary_for_repo, persist_runtime_blocker_for_repo,
    plan_runtime_knowledge_tree, run_compose_pipeline_with_action,
    run_scoped_compose_pipeline_for_update,
};
use crate::workflows::progress::{
    NoopProgressSink, ProgressSink, SharedProgressSink, WorkflowProgressEvent, WorkflowReporter,
};
use crate::workflows::rebuild::run_rebuild_with_progress_and_llm_as_with_mode;
use crate::workflows::release_scope::project_external_runtime_state;
use crate::workflows::research_provider::select_runtime_research_provider;
use wiki_index::fingerprint::fingerprint_bytes;
use wiki_index::hierarchy::build_module_tree_with_graph_and_assist;
use wiki_index::scanner::scan_repo_with_boundary_and_assist;
use wiki_index::store::IndexQueryStore;
use wiki_index::symbol_graph::resolve::{
    build_import_resolution_context, collect_import_target_files,
};
use wiki_index::symbol_graph::{
    analyze_symbol_graph, build_graph_summary, resolve_symbol_graph, ResolvedGraphSnapshot,
};
use wiki_index::symbols::{ParsedSymbolsSnapshot, SymbolTable};
use wiki_model::domain::update_scope::{AffectedKnowledgeScope, ScopeEscalationLevel};

/// 小范围符号变更仍走 scoped graph refresh；超过阈值直接回退全量图刷新，避免增量拼接丢边。
const LOCAL_UPDATE_MAX_FILES: usize = 16;

/// `update` 当前会优先走增量 runtime。
/// 只有 runtime 缺失或已损坏时，才回退到 init / rebuild。
#[derive(Debug, Clone, Serialize)]
pub struct UpdateReport {
    /// update 开始前看到的 runtime 状态。
    pub previous_state: String,
    /// update 完成后的 runtime 状态。
    pub state: String,
    /// 本次 update 实际触达的页面路径集合。
    pub updated_pages: Vec<String>,
    /// 本次 update 命中的知识范围摘要。
    pub affected_knowledge_scope: AffectedKnowledgeScope,
    /// 当前 workflow 终态对应的 runtime 摘要。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_summary: Option<RuntimeSummaryProjection>,
    /// 当前长流程真实采用的执行路径。
    pub llm_execution_mode: LlmExecutionMode,
}

/// 更新 Repo Wiki。
/// `fresh` 时直接 no-op；`stale` 时走增量 apply；`missing / needs_rebuild` 时回退。
///
/// # 参数
/// - `repo_root`：要更新的本地代码目录。
///
/// # 返回
/// - 成功时返回更新前状态、更新后状态以及本次更新的页面列表。
///
/// # 错误
/// - 当变化规划、页面重生成或 runtime 落盘失败时返回错误。
pub fn run_update(repo_root: &Path) -> io::Result<UpdateReport> {
    let mut sink = NoopProgressSink;
    run_update_with_progress_as("update", repo_root, &mut sink)
}

pub fn run_update_with_progress_as(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &mut dyn ProgressSink,
) -> io::Result<UpdateReport> {
    run_update_with_progress_and_llm_as_with_mode(
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
/// - `repo_root`：待更新 Wiki 的仓库根目录。
/// - `progress_sink`：接收阶段进度的下游。
/// - `llm_service`：可选的 LLM 桥接服务；不可用时自动回退 deterministic。
///
/// # 返回
/// - 成功时返回 update 报告。
pub fn run_update_with_progress_and_llm_as<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    llm_service: Option<&'a mut dyn LlmService>,
) -> io::Result<UpdateReport> {
    run_update_with_progress_and_llm_as_with_mode(
        action,
        repo_root,
        progress_sink,
        llm_service,
        SteeringLoadMode::Production,
    )
}

pub fn run_update_with_progress_and_llm_as_with_mode<'a>(
    action: &'static str,
    repo_root: &Path,
    progress_sink: &'a mut dyn ProgressSink,
    llm_service: Option<&'a mut dyn LlmService>,
    steering_mode: SteeringLoadMode,
) -> io::Result<UpdateReport> {
    let started_at = Instant::now();
    let steering = load_steering_config_with_mode(repo_root, steering_mode);
    debug_trace::begin_session(action, repo_root, &steering.debug)?;
    WorkflowReporter::from_started_at(action, progress_sink, started_at)
        .phase("plan_changes", "规划增量变更");
    let mut plan = plan_runtime_changes_with_mode(repo_root, steering_mode)?;
    if plan.needs_rebuild_reason.as_deref() == Some("cache_missing")
        && restore_runtime_cache_from_artifacts(repo_root)?
    {
        WorkflowReporter::from_started_at(action, progress_sink, started_at)
            .phase("plan_changes", "基于正式产物恢复 runtime cache");
        plan = plan_runtime_changes_with_mode(repo_root, steering_mode)?;
    }
    let previous_state =
        project_external_runtime_state(repo_root, plan.state(), facts_snapshot_ready(repo_root)?);

    match plan.fallback_mode {
        FallbackMode::Init => {
            WorkflowReporter::from_started_at(action, progress_sink, started_at)
                .phase("plan_changes", "runtime 缺失，回退到 init");
            let init = run_init_with_progress_and_llm_as_with_mode(
                action,
                repo_root,
                progress_sink,
                llm_service,
                steering_mode,
            )?;
            return Ok(UpdateReport {
                previous_state,
                state: init.state,
                updated_pages: init.generated_pages,
                affected_knowledge_scope: AffectedKnowledgeScope::default(),
                runtime_summary: init.runtime_summary,
                llm_execution_mode: init.llm_execution_mode,
            });
        }
        FallbackMode::Rebuild => {
            WorkflowReporter::from_started_at(action, progress_sink, started_at)
                .phase("plan_changes", "runtime 缺失或损坏，回退到 rebuild");
            let rebuild = run_rebuild_with_progress_and_llm_as_with_mode(
                action,
                repo_root,
                progress_sink,
                llm_service,
                steering_mode,
            )?;
            return Ok(UpdateReport {
                previous_state,
                state: rebuild.state,
                updated_pages: rebuild.updated_pages,
                affected_knowledge_scope: AffectedKnowledgeScope::default(),
                runtime_summary: rebuild.runtime_summary,
                llm_execution_mode: rebuild.llm_execution_mode,
            });
        }
        FallbackMode::None => {}
    }

    if plan.change_set.is_empty() && plan.affected_knowledge_scope.is_empty() {
        return Ok(UpdateReport {
            previous_state,
            state: "fresh".to_string(),
            updated_pages: Vec::new(),
            affected_knowledge_scope: plan.affected_knowledge_scope.clone(),
            runtime_summary: load_runtime_summary_for_repo(repo_root)?
                .map(RuntimeSummaryProjection::from_summary),
            llm_execution_mode: LlmExecutionMode::DeterministicOnly,
        });
    }

    let shared_sink = Rc::new(RefCell::new(progress_sink));
    let mut reporter_sink = SharedProgressSink::new(shared_sink.clone());
    let mut reporter = WorkflowReporter::from_started_at(action, &mut reporter_sink, started_at);
    reporter.phase("plan_changes", "应用增量变更");
    let (updated_pages, llm_execution_mode) = apply_incremental_update(
        repo_root,
        &plan,
        &mut reporter,
        llm_service,
        action,
        started_at,
        shared_sink.clone(),
        steering_mode,
    )?;

    Ok(UpdateReport {
        previous_state,
        state: "fresh".to_string(),
        updated_pages,
        affected_knowledge_scope: plan.affected_knowledge_scope.clone(),
        runtime_summary: load_runtime_summary_for_repo(repo_root)?
            .map(RuntimeSummaryProjection::from_summary),
        llm_execution_mode,
    })
}

fn merge_page_digests_for_update(
    knowledge_tree: &crate::domain::knowledge::KnowledgeTree,
    previous_artifacts: Option<&KnowledgeArtifactSnapshot>,
    fresh_digests: &BTreeMap<String, wiki_knowledge::domain::research::PageDigest>,
) -> BTreeMap<String, wiki_knowledge::domain::research::PageDigest> {
    let mut merged = previous_artifacts
        .map(|artifacts| {
            artifacts
                .page_digests
                .iter()
                .map(|digest| (digest.unit_id.clone(), digest.clone()))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let current_unit_ids = knowledge_tree
        .units
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    merged.retain(|unit_id, _| current_unit_ids.contains(unit_id));
    merged.extend(
        fresh_digests
            .iter()
            .map(|(unit_id, digest)| (unit_id.clone(), digest.clone())),
    );
    merged
}

fn merge_research_summaries_for_update(
    knowledge_tree: &crate::domain::knowledge::KnowledgeTree,
    previous_artifacts: Option<&KnowledgeArtifactSnapshot>,
    fresh_unit_researches: &BTreeMap<String, wiki_knowledge::domain::research::UnitResearch>,
) -> Vec<wiki_model::domain::knowledge_artifact::KnowledgeResearchSummary> {
    let mut merged = previous_artifacts
        .map(|artifacts| {
            artifacts
                .research_summaries
                .iter()
                .map(|summary| (summary.unit_id.clone(), summary.clone()))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let current_unit_ids = knowledge_tree
        .units
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    merged.retain(|unit_id, _| current_unit_ids.contains(unit_id));
    for unit in knowledge_tree.units.values() {
        if let Some(research) = fresh_unit_researches.get(&unit.id) {
            merged.insert(unit.id.clone(), research.to_artifact_summary(unit));
        }
    }
    merged.into_values().collect()
}

fn merge_declared_records_for_update(
    knowledge_tree: &crate::domain::knowledge::KnowledgeTree,
    previous_artifacts: Option<&KnowledgeArtifactSnapshot>,
) -> Vec<wiki_model::domain::knowledge_artifact::DeclaredKnowledgeRecord> {
    let current_unit_ids = knowledge_tree
        .units
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();

    previous_artifacts
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
        .unwrap_or_default()
}

fn apply_incremental_update<'a>(
    repo_root: &Path,
    plan: &ChangePlan,
    reporter: &mut WorkflowReporter<'_>,
    llm_service: Option<&'a mut dyn LlmService>,
    action: &'static str,
    started_at: Instant,
    shared_sink: Rc<RefCell<&'a mut dyn ProgressSink>>,
    steering_mode: SteeringLoadMode,
) -> io::Result<(Vec<String>, LlmExecutionMode)> {
    // 增量路径保持 symbols/edges 按文件刷新，但 graph-derived 视图整体重算。
    let previous_state = plan
        .previous_state
        .as_ref()
        .ok_or_else(|| io::Error::other("incremental update requires previous wiki state"))?;
    let steering = load_steering_config_with_mode(repo_root, steering_mode);
    let mut llm_runtime = LlmRuntime::new(repo_root, &steering.llm, llm_service);
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
    if llm_runtime.service_available() {
        reporter.phase("llm_uncertainty_gate", "执行 LLM 不确定性判断");
    }
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report = scan_repo_with_boundary_and_assist(
        repo_root,
        ignore_paths,
        include_paths,
        Some(&mut llm_runtime as &mut dyn wiki_index::assist::FactsAssist),
    )?;
    let changed_symbol_paths = plan.affected_set.graph_refresh_sources.clone();
    let dirty_symbol_paths = plan
        .affected_set
        .graph_refresh_sources
        .iter()
        .chain(plan.change_set.removed_sources.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let parse_total =
        wiki_index::symbols::symbol_parse_file_count(&scan_report, &changed_symbol_paths);
    reporter.counted("parse_symbols", "解析受影响源码符号", 0, parse_total);
    let changed_symbol_snapshot = if changed_symbol_paths.is_empty() {
        ParsedSymbolsSnapshot::default()
    } else {
        wiki_index::symbols::parse_symbols_for_paths_with_progress(
            repo_root,
            &scan_report,
            &changed_symbol_paths,
            &mut |processed, total| {
                reporter.counted(
                    "parse_symbols",
                    format!("解析受影响源码符号 {processed}/{total}"),
                    processed,
                    total,
                );
            },
        )?
    };
    let working_set = load_incremental_working_set(
        repo_root,
        &scan_report,
        plan,
        &changed_symbol_snapshot,
        &dirty_symbol_paths,
    )?;
    reporter.phase("plan_changes", working_set.note.clone());
    let graph_refresh_strategy = working_set.strategy;
    let persisted_symbols = working_set.persisted_symbols;
    let persisted_edges = working_set.persisted_edges;
    let (full_symbol_snapshot, full_resolved_graph, changed_resolved_graph) =
        match graph_refresh_strategy {
            GraphRefreshStrategy::Full => {
                let symbol_total = wiki_index::symbols::symbol_parse_file_count(&scan_report, &[]);
                reporter.counted("parse_symbols", "解析源码符号", 0, symbol_total);
                let full_symbol_snapshot = wiki_index::symbols::parse_symbols_with_progress(
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
                let full_resolved_graph =
                    resolve_symbol_graph(repo_root, &scan_report, &full_symbol_snapshot)?;
                (
                    full_symbol_snapshot,
                    full_resolved_graph,
                    ResolvedGraphSnapshot::default(),
                )
            }
            GraphRefreshStrategy::Scoped => {
                let full_symbol_snapshot = merge_symbol_snapshots(
                    &persisted_symbols,
                    &dirty_symbol_paths,
                    &changed_symbol_snapshot,
                );
                let resolution_snapshot =
                    build_resolution_snapshot(&changed_symbol_snapshot, &full_symbol_snapshot);
                reporter.phase("resolve_symbol_graph", "解析符号关系");
                let changed_resolved_graph = if changed_symbol_paths.is_empty() {
                    ResolvedGraphSnapshot::default()
                } else {
                    resolve_symbol_graph(repo_root, &scan_report, &resolution_snapshot)?
                };
                let full_resolved_graph = merge_resolved_graphs(
                    &persisted_symbols,
                    &persisted_edges,
                    &dirty_symbol_paths,
                    &changed_resolved_graph,
                );
                (
                    full_symbol_snapshot,
                    full_resolved_graph,
                    changed_resolved_graph,
                )
            }
        };
    reporter.phase("analyze_symbol_graph", "分析符号图");
    let analysis = analyze_symbol_graph(&full_symbol_snapshot, &full_resolved_graph);
    let graph_summary = build_graph_summary(
        &scan_report,
        &full_symbol_snapshot,
        &full_resolved_graph,
        &analysis,
    );
    reporter.phase("build_module_tree", "构建模块树");
    let module_tree = build_module_tree_with_graph_and_assist(
        &scan_report,
        &graph_summary,
        Some(&mut llm_runtime),
    );
    reporter.phase("write_facts_snapshot", "提交 facts snapshot");
    match graph_refresh_strategy {
        GraphRefreshStrategy::Full => write_facts_snapshot(
            repo_root,
            &scan_report,
            &module_tree,
            &full_symbol_snapshot.symbols,
            &full_resolved_graph,
            &analysis,
        )?,
        GraphRefreshStrategy::Scoped => write_facts_snapshot_for_files(
            repo_root,
            &scan_report,
            &module_tree,
            &dirty_symbol_paths,
            &changed_symbol_snapshot.symbols,
            &changed_resolved_graph,
            &analysis,
        )?,
    };
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
    let planned_knowledge_tree = plan.knowledge_tree.clone().unwrap_or_else(|| {
        plan_runtime_knowledge_tree(
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &graph_summary,
            &steering,
        )
    });
    let update_page_targets = if matches!(
        plan.affected_knowledge_scope.escalation.level,
        ScopeEscalationLevel::LocalRefresh
    ) {
        plan.affected_set.affected_page_ids.clone()
    } else {
        wiki_knowledge::plan_pages_from_knowledge_tree(&planned_knowledge_tree)
            .into_iter()
            .map(|page| page.id)
            .collect::<Vec<_>>()
    };
    let previous_artifacts = load_knowledge_artifacts(repo_root).ok();
    let pipeline = if let Some(previous_artifacts) = previous_artifacts.as_ref() {
        run_scoped_compose_pipeline_for_update(
            action,
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &full_symbol_snapshot,
            &full_resolved_graph,
            &analysis,
            &graph_summary,
            &steering,
            &planned_knowledge_tree,
            &previous_artifacts.page_digests,
            &plan.affected_knowledge_scope,
            &update_page_targets,
            research_provider_impl.as_ref(),
        )?
    } else {
        reporter.phase(
            "compose",
            "formal artifacts 缺失，回退到全量 knowledge pipeline",
        );
        run_compose_pipeline_with_action(
            action,
            repo_root,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
            &full_symbol_snapshot,
            &full_resolved_graph,
            &analysis,
            &graph_summary,
            &steering,
            research_provider_impl.as_ref(),
        )?
    };
    drop(research_provider);
    let page_drafts = pipeline.page_drafts;
    let digests = merge_page_digests_for_update(
        &pipeline.knowledge_tree,
        previous_artifacts.as_ref(),
        &pipeline.digests,
    );
    let unit_researches = pipeline.unit_researches;
    let knowledge_tree = pipeline.knowledge_tree;
    let research_summaries = merge_research_summaries_for_update(
        &knowledge_tree,
        previous_artifacts.as_ref(),
        &unit_researches,
    );
    let declared_records =
        merge_declared_records_for_update(&knowledge_tree, previous_artifacts.as_ref());
    let drafts_by_page_id = page_drafts
        .iter()
        .map(|draft| (draft.page_id.clone(), draft))
        .collect::<BTreeMap<_, _>>();
    let explicit_affected_page_ids = plan
        .affected_set
        .affected_page_ids
        .iter()
        .cloned()
        .chain(update_page_targets.iter().cloned())
        .collect::<BTreeSet<_>>();
    let explicit_removed_page_ids = plan
        .affected_set
        .removed_page_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let previous_pages = previous_state
        .pages
        .iter()
        .map(|page| (page.page_id.clone(), page))
        .collect::<BTreeMap<_, _>>();

    let current_page_ids = pipeline
        .planned_pages
        .iter()
        .map(|page| page.id.clone())
        .collect::<BTreeSet<_>>();
    let removed_page_ids = previous_pages
        .keys()
        .filter(|page_id| !current_page_ids.contains(*page_id))
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut ancestor_ids_by_page = BTreeMap::new();
    let mut next_pages = Vec::new();
    let mut touched_paths = BTreeSet::new();
    let page_total = explicit_affected_page_ids.len();
    let mut rendered_pages = 0usize;

    reporter.counted("render_pages", "渲染页面", 0, page_total);

    for planned_page in &pipeline.planned_pages {
        let ancestor_ids = ancestor_ids_for_page(planned_page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(planned_page.id.clone(), ancestor_ids.clone());
        let current_page_path = format!(".wiki/{}", planned_page.relative_path);
        let previous_page = previous_pages.get(&planned_page.id).copied();
        let should_rerender = explicit_affected_page_ids.contains(&planned_page.id)
            || explicit_removed_page_ids.contains(&planned_page.id)
            || previous_page
                .map(|page| page.path != current_page_path)
                .unwrap_or(true);

        if !should_rerender {
            if let Some(previous_page) = previous_page {
                next_pages.push(previous_page.clone());
                continue;
            }
        }

        let draft = drafts_by_page_id
            .get(&planned_page.id)
            .copied()
            .ok_or_else(|| {
                io::Error::other(format!(
                    "missing scoped draft for affected page {}",
                    planned_page.id
                ))
            })?;
        let rendered = render_page_draft(draft);
        let page_context = build_minimal_page_context(
            draft,
            planned_page,
            &knowledge_tree,
            &digests,
            &unit_researches,
        );
        let input_hash = wiki_index::fingerprint::fingerprint_bytes(
            format!("{}:{}", draft.page_id, draft.citation_count).as_bytes(),
        );

        let final_content = merge_user_sections_into_page(
            repo_root,
            previous_page.map(|page| page.path.as_str()),
            planned_page,
            &rendered.sections,
            &rendered.content,
        );

        let content_hash = fingerprint_bytes(final_content.as_bytes());

        write_page(repo_root, &planned_page.relative_path, &final_content)?;
        if let Some(previous_page) = previous_page {
            if previous_page.path != current_page_path {
                let previous_disk_path = resolve_page_path(repo_root, &previous_page.path);
                if previous_disk_path.exists() {
                    fs::remove_file(previous_disk_path)?;
                }
                touched_paths.insert(previous_page.path.clone());
            }
        }

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

        next_pages.push(build_page_state(&PageBuildResult {
            page: planned_page.clone(),
            context: page_context.clone(),
            summary,
            input_hash,
            content_hash,
            source_paths: source_paths_for_page(&scan_report, &page_context),
            ancestor_ids,
            provenance: page_provenance(&planned_page, &page_context, &scan_report),
            sections: rendered.sections,
        }));
        touched_paths.insert(current_page_path);
        rendered_pages += 1;
        reporter.counted(
            "render_pages",
            format!("渲染页面 {rendered_pages}/{page_total}"),
            rendered_pages,
            page_total,
        );
    }

    for removed_page_id in removed_page_ids {
        if let Some(previous_page) = previous_pages.get(&removed_page_id) {
            let disk_path = resolve_page_path(repo_root, &previous_page.path);
            if disk_path.exists() {
                fs::remove_file(disk_path)?;
            }
            remove_page_caches(repo_root, &removed_page_id)?;
            touched_paths.insert(previous_page.path.clone());
        }
    }

    // page_id 去重
    {
        let mut seen = std::collections::HashSet::new();
        next_pages.retain(|p| seen.insert(p.page_id.clone()));
    }

    let generated_at = current_timestamp();
    let next_state = assemble_state_from_pages(
        &next_pages,
        &scan_report,
        &module_tree,
        &generated_at,
        DirtyState::fresh(),
    );

    reporter.phase("write_state", "写入运行时状态");
    write_state(repo_root, &next_state)?;

    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at: generated_at.clone(),
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&next_state, &export_context);
    reporter.phase("write_metadata", "写入元数据");
    write_metadata(repo_root, &metadata)?;
    finalize_pipeline_runtime(repo_root, action, next_pages.len())?;
    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    let facts_input_hash = compute_facts_input_hash(&scan_report, &module_tree);
    let page_digests = digests.values().cloned().collect::<Vec<_>>();
    let runtime_gates = runtime_store.read_unit_runtime_gates()?;
    let health_signals = Vec::new();
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
    runtime_store.clear_pipeline_checkpoint()?;

    Ok((
        touched_paths.into_iter().collect(),
        match llm_runtime.selected_path() {
            Some(crate::llm::SelectedLlmPath::ProviderApi) => LlmExecutionMode::ProviderDirect,
            Some(crate::llm::SelectedLlmPath::AgentBridge) => LlmExecutionMode::AgentBridge,
            None => LlmExecutionMode::DeterministicOnly,
        },
    ))
}

/// 从磁盘旧页面中解析 user sections，与新生成的 managed sections 合并。
/// 如果旧页面不存在或没有 user sections，直接返回新生成的内容。
fn merge_user_sections_into_page(
    repo_root: &Path,
    previous_page_path: Option<&str>,
    planned_page: &wiki_knowledge::PlannedPage,
    new_sections: &[crate::generation::sections::SectionDraft],
    new_content: &str,
) -> String {
    let page_path = resolve_page_path(
        repo_root,
        previous_page_path.unwrap_or(&format!(".wiki/{}", planned_page.relative_path)),
    );
    let old_content = match fs::read_to_string(&page_path) {
        Ok(c) => c,
        Err(_) => return new_content.to_string(),
    };

    let known_titles = section_titles_for_page_type(&planned_page.page_type);
    let known_titles_ref: Vec<&str> = known_titles.iter().copied().collect();
    let old_parsed = parse_wiki_page(&old_content, &known_titles_ref);

    // 检查旧页面是否有 user sections
    let has_user_sections = old_parsed
        .blocks
        .iter()
        .any(|b| matches!(b, crate::generation::managed_sections::PageBlock::User(_)));

    if !has_user_sections {
        return new_content.to_string();
    }

    // 把新 section drafts 转成 ManagedSectionBlock
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
    assemble_page_from_merge(&planned_page.title, &merge_plan)
}

fn merge_symbol_snapshots(
    persisted_symbols: &[wiki_index::symbols::SymbolNode],
    dirty_symbol_paths: &[String],
    changed_symbol_snapshot: &ParsedSymbolsSnapshot,
) -> ParsedSymbolsSnapshot {
    let dirty_symbol_paths = dirty_symbol_paths.iter().cloned().collect::<BTreeSet<_>>();
    let mut symbols = persisted_symbols
        .iter()
        .filter(|symbol| !dirty_symbol_paths.contains(&symbol.file_path))
        .cloned()
        .collect::<Vec<_>>();
    symbols.extend(changed_symbol_snapshot.symbols.iter().cloned());
    symbols.sort_by(|left, right| {
        left.file_path
            .cmp(&right.file_path)
            .then(left.start_line.cmp(&right.start_line))
            .then(left.end_line.cmp(&right.end_line))
            .then(left.symbol_id.cmp(&right.symbol_id))
    });
    symbols.dedup_by(|left, right| left.symbol_id == right.symbol_id);

    ParsedSymbolsSnapshot {
        files: BTreeMap::new(),
        symbol_table: SymbolTable::from_symbols(&symbols),
        symbols,
        diagnostics: changed_symbol_snapshot.diagnostics.clone(),
    }
}

fn build_resolution_snapshot(
    changed_symbol_snapshot: &ParsedSymbolsSnapshot,
    full_symbol_snapshot: &ParsedSymbolsSnapshot,
) -> ParsedSymbolsSnapshot {
    ParsedSymbolsSnapshot {
        files: changed_symbol_snapshot.files.clone(),
        symbols: full_symbol_snapshot.symbols.clone(),
        diagnostics: changed_symbol_snapshot.diagnostics.clone(),
        symbol_table: full_symbol_snapshot.symbol_table.clone(),
    }
}

fn merge_resolved_graphs(
    persisted_symbols: &[wiki_index::symbols::SymbolNode],
    persisted_edges: &[wiki_index::symbol_graph::ResolvedSymbolEdge],
    dirty_symbol_paths: &[String],
    changed_resolved_graph: &ResolvedGraphSnapshot,
) -> ResolvedGraphSnapshot {
    let dirty_symbol_paths = dirty_symbol_paths.iter().cloned().collect::<BTreeSet<_>>();
    let persisted_symbol_paths = persisted_symbols
        .iter()
        .map(|symbol| (symbol.symbol_id.clone(), symbol.file_path.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut merged_edges = persisted_edges
        .iter()
        .filter(|edge| {
            let source_dirty = persisted_symbol_paths
                .get(&edge.source_id)
                .map(|path| dirty_symbol_paths.contains(path))
                .unwrap_or(false);
            let target_dirty = persisted_symbol_paths
                .get(&edge.target_id)
                .map(|path| dirty_symbol_paths.contains(path))
                .unwrap_or(false);
            !(source_dirty || target_dirty)
        })
        .cloned()
        .collect::<Vec<_>>();
    merged_edges.extend(changed_resolved_graph.edges.iter().cloned());
    merged_edges.sort_by(|left, right| {
        left.source_id
            .cmp(&right.source_id)
            .then(left.target_id.cmp(&right.target_id))
            .then(left.edge_type.cmp(&right.edge_type))
            .then(left.edge_id.cmp(&right.edge_id))
    });
    merged_edges.dedup_by(|left, right| left.edge_id == right.edge_id);

    ResolvedGraphSnapshot {
        edges: merged_edges,
        diagnostics: changed_resolved_graph.diagnostics.clone(),
    }
}

#[derive(Clone, Copy)]
enum GraphRefreshStrategy {
    /// 仅刷新 dirty file 及其一跳邻接范围，保留现有的 scoped symbol/edge 持久化路径。
    Scoped,
    /// 直接重建完整 symbol graph，再以全量快照覆盖 index 真相表。
    Full,
}

struct IncrementalWorkingSet {
    persisted_symbols: Vec<wiki_index::symbols::SymbolNode>,
    persisted_edges: Vec<wiki_index::symbol_graph::ResolvedSymbolEdge>,
    note: String,
    strategy: GraphRefreshStrategy,
}

fn load_incremental_working_set(
    repo_root: &Path,
    scan_report: &wiki_index::scanner::ScanReport,
    plan: &ChangePlan,
    changed_symbol_snapshot: &ParsedSymbolsSnapshot,
    dirty_symbol_paths: &[String],
) -> io::Result<IncrementalWorkingSet> {
    let index_store = SqliteIndexStore::new(repo_root);
    let total_symbol_files = index_store.count_symbol_files()?;
    let mut workset_paths = dirty_symbol_paths.iter().cloned().collect::<BTreeSet<_>>();
    workset_paths.extend(plan.affected_set.graph_refresh_sources.iter().cloned());

    if !changed_symbol_snapshot.files.is_empty() {
        let import_context = build_import_resolution_context(repo_root, scan_report)?;
        let selected_files = dirty_symbol_paths.iter().cloned().collect::<BTreeSet<_>>();
        workset_paths.extend(collect_import_target_files(
            changed_symbol_snapshot,
            &import_context,
            &selected_files,
        ));
    }

    let seed_paths = workset_paths.iter().cloned().collect::<Vec<_>>();
    workset_paths.extend(index_store.list_adjacent_symbol_files(&seed_paths)?);

    if total_symbol_files > 0 && workset_paths.len() <= LOCAL_UPDATE_MAX_FILES {
        let scoped_paths = workset_paths.into_iter().collect::<Vec<_>>();
        return Ok(IncrementalWorkingSet {
            persisted_symbols: index_store.list_symbols()?,
            persisted_edges: index_store.list_edges()?,
            note: format!(
                "使用局部 symbol/edge 工作集（{} 个文件），在局部 parse/resolve 工作集上刷新变更，并基于全量 persisted graph 重算 analysis",
                scoped_paths.len()
            ),
            strategy: GraphRefreshStrategy::Scoped,
        });
    }

    let note = if total_symbol_files == 0 {
        "symbol 图状态缺失，回退到全量 symbol/edge 读取".to_string()
    } else if workset_paths.len() > LOCAL_UPDATE_MAX_FILES {
        format!(
            "局部工作集 {} 个文件超过阈值 {}，回退到全量 symbol/edge 读取",
            workset_paths.len(),
            LOCAL_UPDATE_MAX_FILES
        )
    } else {
        format!(
            "局部工作集仅覆盖 {}/{} 个 symbol 文件，回退到全量 symbol/edge 读取",
            workset_paths.len(),
            total_symbol_files
        )
    };

    Ok(IncrementalWorkingSet {
        persisted_symbols: index_store.list_symbols()?,
        persisted_edges: index_store.list_edges()?,
        note,
        strategy: GraphRefreshStrategy::Full,
    })
}
