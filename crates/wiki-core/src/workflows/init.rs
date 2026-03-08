use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::domain::context::PageContext;
use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::{assemble_state, PageBuildResult};
use crate::generation::context::{build_module_contexts, build_page_context, build_repo_context};
use crate::generation::planner::plan_pages;
use crate::generation::renderer::render_page;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::{ensure_cache_dir, write_module_tree_cache, write_scan_cache};
use crate::storage::metadata_store::write_metadata;
use crate::storage::state_store::write_state;
use crate::storage::wiki_fs::{remove_runtime, write_page};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `init` 会全量生成 Repo Wiki 运行时。
/// 它是当前最完整的一条链路：扫描 -> 模块树 -> 页面 -> WikiState -> metadata/cache。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InitReport {
    pub initialized: bool,
    pub state: String,
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
    if !repo_root.exists() || !repo_root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repo root must be an existing directory",
        ));
    }

    remove_runtime(repo_root)?;

    // 按 deterministic pipeline 的顺序串起整条生成链。
    let scan_report = scan_repo(repo_root)?;
    let module_tree = build_module_tree(&scan_report);
    let repo_context = build_repo_context(&scan_report, &module_tree);
    let module_contexts = build_module_contexts(&scan_report, &module_tree);
    let pages = plan_pages(&scan_report, &module_tree, &repo_context, &module_contexts);

    ensure_cache_dir(repo_root)?;
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    let mut page_results = Vec::new();
    let mut generated_pages = Vec::new();
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
        let content = render_page(page, &page_context);
        write_page(repo_root, &page.relative_path, &content)?;
        let page_path = format!(".wiki/{}", page.relative_path);
        generated_pages.push(page_path);
        let ancestor_ids = ancestor_ids_for_page(page, &ancestor_ids_by_page);
        ancestor_ids_by_page.insert(page.id.clone(), ancestor_ids.clone());

        page_results.push(PageBuildResult {
            page: page.clone(),
            context: page_context.clone(),
            content_hash: crate::repo::fingerprint::fingerprint_bytes(content.as_bytes()),
            source_paths: source_paths_for_page(&scan_report, &page_context),
            ancestor_ids,
            provenance: page_provenance(page, &page_context, &scan_report),
        });
    }

    // 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。
    let state = assemble_state(&page_results, &scan_report, &module_tree, &generated_at);
    write_state(repo_root, &state)?;

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

    Ok(InitReport {
        initialized: true,
        state: state.dirty_state.status,
        generated_pages,
    })
}

/// 根据页面上下文里的 `source_ids` 反查源码路径。
fn source_paths_for_page(scan_report: &crate::repo::scanner::ScanReport, page_context: &PageContext) -> Vec<String> {
    scan_report
        .files
        .iter()
        .filter(|file| page_context.source_ids.contains(&file.id))
        .map(|file| file.path.clone())
        .collect()
}

/// 页面祖先链会直接写入状态层，方便 query 和后续 runtime 读取。
fn ancestor_ids_for_page(
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
fn page_provenance(
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
