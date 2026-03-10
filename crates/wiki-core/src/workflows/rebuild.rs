//! rebuild workflow 负责强制全量重建 Repo Wiki runtime。
//! 迭代 5 改为：忽略旧 generation cache，但对同 page_id 页面复用已同步的 user sections。

use serde::Serialize;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::{assemble_state, compute_page_input_hash, PageBuildResult};
use crate::domain::steering::load_steering_config;
use crate::generation::context::{
    build_module_contexts_with_graph, build_page_context, build_repo_context_with_graph,
};
use crate::generation::managed_sections::{
    merge_sections, parse_wiki_page, ManagedSectionBlock, PageBlock,
};
use crate::generation::renderer::{assemble_page_from_merge, render_page_bundle};
use crate::generation::sections::section_titles_for_page_type;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree_with_graph;
use crate::repo::scanner::scan_repo_with_boundary;
use crate::repo::symbol_graph::{
    analyze_symbol_graph, build_graph_summary, resolve_symbol_graph,
};
use crate::repo::symbols::parse_symbols;
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
    if !repo_root.exists() || !repo_root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repo root must be an existing directory",
        ));
    }

    // 在清理前，读取旧页面的磁盘内容用于 user section 恢复
    let old_page_contents = read_old_page_contents(repo_root);

    // 清理旧 runtime
    crate::storage::wiki_fs::remove_runtime(repo_root)?;

    // 全量 pipeline
    let steering = load_steering_config(repo_root);
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths)?;
    let symbol_snapshot = parse_symbols(repo_root, &scan_report)?;
    let resolved_graph = resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot)?;
    let analysis = analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
    let graph_summary =
        build_graph_summary(&scan_report, &symbol_snapshot, &resolved_graph, &analysis);
    let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
    let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
    let module_contexts =
        build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);
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

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
    let mut all_warnings = Vec::new();
    let generated_at = current_timestamp();
    let mut ancestor_ids_by_page = BTreeMap::new();

    for page in &pages {
        let page_context = build_page_context(
            page,
            &scan_report,
            &module_tree,
            &repo_context,
            &module_contexts,
        );
        let input_hash = compute_page_input_hash(page, &page_context, &scan_report);
        let rendered_page = render_page_bundle(page, &page_context);

        // 尝试从旧页面恢复 user sections
        let final_content = match old_page_contents.get(&page.id) {
            Some(old_content) => merge_old_user_sections(
                page,
                &rendered_page.sections,
                &rendered_page.content,
                old_content,
                &mut all_warnings,
            ),
            None => rendered_page.content.clone(),
        };

        write_page(repo_root, &page.relative_path, &final_content)?;
        let page_path = format!(".wiki/{}", page.relative_path);
        generated_pages.push(page_path);
        let ancestor_ids = ancestor_ids_for_page(page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(page.id.clone(), ancestor_ids.clone());
        let content_hash = fingerprint_bytes(final_content.as_bytes());

        write_page_context_cache(
            repo_root,
            &PageContextCacheEntry {
                page_id: page.id.clone(),
                input_hash: input_hash.clone(),
                context: page_context.clone(),
            },
        )?;
        write_page_generation_cache(
            repo_root,
            &PageGenerationCacheEntry {
                page_id: page.id.clone(),
                input_hash: input_hash.clone(),
                content_hash: content_hash.clone(),
                sections: rendered_page.sections.clone(),
            },
        )?;

        page_results.push(PageBuildResult {
            page: page.clone(),
            context: page_context.clone(),
            input_hash,
            content_hash,
            source_paths: source_paths_for_page(&scan_report, &page_context),
            ancestor_ids,
            provenance: page_provenance(page, &page_context, &scan_report),
            sections: rendered_page.sections,
        });
    }

    let state = assemble_state(&page_results, &scan_report, &module_tree, &generated_at);
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
