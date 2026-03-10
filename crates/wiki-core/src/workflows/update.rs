//! update workflow 负责把 `stale` runtime 增量刷新回 `fresh`。
//! 它优先局部重建受影响页面，并在必要时回退到 init 或 rebuild。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;

use crate::domain::change_set::{plan_runtime_changes, ChangePlan, FallbackMode};
use crate::domain::metadata::DirtyState;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::steering::load_steering_config;
use crate::domain::state::{
    assemble_state_from_pages, build_page_state, compute_page_input_hash, PageBuildResult,
};
use crate::generation::context::{
    build_module_contexts_with_graph, build_page_context, build_repo_context_with_graph,
};
use crate::generation::managed_sections::{merge_sections, parse_wiki_page, ManagedSectionBlock};
use crate::generation::renderer::{assemble_page_from_merge, render_page_bundle};
use crate::generation::sections::section_titles_for_page_type;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree_with_graph;
use crate::repo::symbol_graph::{
    analyze_symbol_graph, build_graph_summary, resolve_symbol_graph, ResolvedGraphSnapshot,
};
use crate::repo::symbols::{parse_symbols_for_paths, ParsedSymbolsSnapshot, SymbolTable};
use crate::storage::cache_store::{
    remove_page_caches, write_module_tree_cache, write_page_context_cache,
    write_page_generation_cache, write_scan_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::sqlite_store;
use crate::storage::state_store::write_state_with_symbol_graph_for_files;
use crate::storage::wiki_fs::{resolve_page_path, write_page};
use crate::workflows::init::{
    ancestor_ids_for_page, current_timestamp, page_provenance, run_init, source_paths_for_page,
};
use crate::workflows::rebuild::run_rebuild;

/// `update` 当前会优先走增量 runtime。
/// 只有 runtime 缺失或已损坏时，才回退到 init / rebuild。
#[derive(Debug, Clone, Serialize)]
pub struct UpdateReport {
    /// update 开始前看到的 runtime 状态。
    pub previous_state: String,
    /// update 完成后的 runtime 状态，正常情况为 `fresh`。
    pub state: String,
    /// 本次 update 实际触达的页面路径集合。
    pub updated_pages: Vec<String>,
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
    let plan = plan_runtime_changes(repo_root)?;
    let previous_state = plan.state().to_string();

    match plan.fallback_mode {
        FallbackMode::Init => {
            let init = run_init(repo_root)?;
            return Ok(UpdateReport {
                previous_state,
                state: init.state,
                updated_pages: init.generated_pages,
            });
        }
        FallbackMode::Rebuild => {
            let rebuild = run_rebuild(repo_root)?;
            return Ok(UpdateReport {
                previous_state,
                state: rebuild.state,
                updated_pages: rebuild.updated_pages,
            });
        }
        FallbackMode::None => {}
    }

    if plan.change_set.is_empty() {
        return Ok(UpdateReport {
            previous_state,
            state: "fresh".to_string(),
            updated_pages: Vec::new(),
        });
    }

    let updated_pages = apply_incremental_update(repo_root, &plan)?;

    Ok(UpdateReport {
        previous_state,
        state: "fresh".to_string(),
        updated_pages,
    })
}

fn apply_incremental_update(repo_root: &Path, plan: &ChangePlan) -> io::Result<Vec<String>> {
    // 增量路径保持 symbols/edges 按文件刷新，但 graph-derived 视图整体重算。
    let previous_state = plan
        .previous_state
        .as_ref()
        .ok_or_else(|| io::Error::other("incremental update requires previous wiki state"))?;
    let scan_report = plan
        .scan_report
        .as_ref()
        .ok_or_else(|| io::Error::other("incremental update requires current scan report"))?;
    let steering = load_steering_config(repo_root);
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
    let changed_symbol_snapshot = if changed_symbol_paths.is_empty() {
        ParsedSymbolsSnapshot::default()
    } else {
        parse_symbols_for_paths(repo_root, scan_report, &changed_symbol_paths)?
    };
    let persisted_symbols = sqlite_store::list_symbols(repo_root)?;
    let persisted_edges = sqlite_store::list_edges(repo_root)?;
    let full_symbol_snapshot = merge_symbol_snapshots(
        &persisted_symbols,
        &dirty_symbol_paths,
        &changed_symbol_snapshot,
    );
    let resolution_snapshot = build_resolution_snapshot(&changed_symbol_snapshot, &full_symbol_snapshot);
    let changed_resolved_graph = if changed_symbol_paths.is_empty() {
        ResolvedGraphSnapshot::default()
    } else {
        resolve_symbol_graph(repo_root, scan_report, &resolution_snapshot)?
    };
    let full_resolved_graph =
        merge_resolved_graphs(&persisted_symbols, &persisted_edges, &dirty_symbol_paths, &changed_resolved_graph);
    let analysis = analyze_symbol_graph(&full_symbol_snapshot, &full_resolved_graph);
    let graph_summary =
        build_graph_summary(scan_report, &full_symbol_snapshot, &full_resolved_graph, &analysis);
    let module_tree = build_module_tree_with_graph(scan_report, &graph_summary);
    let repo_context = build_repo_context_with_graph(scan_report, &module_tree, &graph_summary);
    let module_contexts =
        build_module_contexts_with_graph(scan_report, &module_tree, &graph_summary);
    let pages = crate::generation::planner::plan_pages_with_graph(
        scan_report,
        &module_tree,
        &repo_context,
        &module_contexts,
        &steering,
        &graph_summary,
    );
    let explicit_affected_page_ids = plan
        .affected_set
        .affected_page_ids
        .iter()
        .cloned()
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
    let current_page_ids = pages
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

    for planned_page in &pages {
        let ancestor_ids = ancestor_ids_for_page(planned_page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(planned_page.id.clone(), ancestor_ids.clone());
        let current_page_path = format!(".wiki/{}", planned_page.relative_path);
        let page_context = build_page_context(
            planned_page,
            scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
        );
        let input_hash = compute_page_input_hash(planned_page, &page_context, scan_report);
        let previous_page = previous_pages.get(&planned_page.id).copied();
        let should_rerender = match previous_page {
            Some(previous_page)
                if !explicit_affected_page_ids.contains(&planned_page.id)
                    && !explicit_removed_page_ids.contains(&planned_page.id)
                    && previous_page.input_hash == input_hash
                    && previous_page.path == current_page_path =>
            {
                false
            }
            Some(_) | None => true,
        };

        if !should_rerender {
            if let Some(previous_page) = previous_page {
                next_pages.push(previous_page.clone());
                continue;
            }
        }

        let rendered_page = render_page_bundle(planned_page, &page_context);

        // 尝试从磁盘读取旧页面，解析出 user sections 并 merge 回新页面
        let final_content = merge_user_sections_into_page(
            repo_root,
            previous_page.map(|page| page.path.as_str()),
            planned_page,
            &rendered_page.sections,
            &rendered_page.content,
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
                sections: rendered_page.sections.clone(),
            },
        )?;

        next_pages.push(build_page_state(&PageBuildResult {
            page: planned_page.clone(),
            context: page_context.clone(),
            input_hash,
            content_hash,
            source_paths: source_paths_for_page(scan_report, &page_context),
            ancestor_ids,
            provenance: page_provenance(planned_page, &page_context, scan_report),
            sections: rendered_page.sections,
        }));
        touched_paths.insert(current_page_path);
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

    let generated_at = current_timestamp();
    let next_state = assemble_state_from_pages(
        &next_pages,
        scan_report,
        &module_tree,
        &generated_at,
        DirtyState::fresh(),
    );

    write_scan_cache(repo_root, scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;
    write_state_with_symbol_graph_for_files(
        repo_root,
        &next_state,
        &dirty_symbol_paths,
        &changed_symbol_snapshot.symbols,
        &changed_resolved_graph,
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
    let metadata = export_metadata(&next_state, &export_context);
    write_metadata(repo_root, &metadata)?;

    Ok(touched_paths.into_iter().collect())
}

/// 从磁盘旧页面中解析 user sections，与新生成的 managed sections 合并。
/// 如果旧页面不存在或没有 user sections，直接返回新生成的内容。
fn merge_user_sections_into_page(
    repo_root: &Path,
    previous_page_path: Option<&str>,
    planned_page: &crate::generation::planner::PlannedPage,
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
    persisted_symbols: &[crate::repo::symbols::SymbolNode],
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
    persisted_symbols: &[crate::repo::symbols::SymbolNode],
    persisted_edges: &[crate::repo::symbol_graph::ResolvedSymbolEdge],
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
