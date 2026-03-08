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
use crate::domain::state::{
    assemble_state_from_pages, build_page_state, compute_page_input_hash, PageBuildResult,
};
use crate::generation::context::{build_module_contexts, build_page_context, build_repo_context};
use crate::generation::renderer::render_page_bundle;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::storage::cache_store::{
    remove_page_caches, write_module_tree_cache, write_page_context_cache,
    write_page_generation_cache, write_scan_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::state_store::write_state;
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
    // 增量路径只重建受影响页面，其余页面状态直接沿用上一轮 runtime。
    let previous_state = plan.previous_state.as_ref().ok_or_else(|| {
        io::Error::other("incremental update requires previous wiki state")
    })?;
    let scan_report = plan
        .scan_report
        .as_ref()
        .ok_or_else(|| io::Error::other("incremental update requires current scan report"))?;
    let module_tree = plan
        .module_tree
        .as_ref()
        .ok_or_else(|| io::Error::other("incremental update requires current module tree"))?;

    let repo_context = build_repo_context(scan_report, module_tree);
    let module_contexts = build_module_contexts(scan_report, module_tree);
    let affected_page_ids = plan
        .affected_set
        .affected_page_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let removed_page_ids = plan
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
    let mut ancestor_ids_by_page = BTreeMap::new();
    let mut next_pages = Vec::new();
    let mut touched_paths = BTreeSet::new();

    for planned_page in &plan.planned_pages {
        let ancestor_ids = ancestor_ids_for_page(planned_page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(planned_page.id.clone(), ancestor_ids.clone());

        if !affected_page_ids.contains(&planned_page.id) {
            if let Some(previous_page) = previous_pages.get(&planned_page.id) {
                next_pages.push((*previous_page).clone());
                continue;
            }
        }

        let page_context = build_page_context(
            planned_page,
            scan_report,
            module_tree,
            &repo_context,
            &module_contexts,
        );
        let input_hash = compute_page_input_hash(planned_page, &page_context, scan_report);
        let rendered_page = render_page_bundle(planned_page, &page_context);
        let content_hash = fingerprint_bytes(rendered_page.content.as_bytes());
        let page_path = format!(".wiki/{}", planned_page.relative_path);

        write_page(repo_root, &planned_page.relative_path, &rendered_page.content)?;
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
        touched_paths.insert(page_path);
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
        module_tree,
        &generated_at,
        DirtyState::fresh(),
    );

    write_scan_cache(repo_root, scan_report)?;
    write_module_tree_cache(repo_root, module_tree)?;
    write_state(repo_root, &next_state)?;

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
