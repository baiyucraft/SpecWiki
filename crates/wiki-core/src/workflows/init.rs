use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::domain::context::PageContext;
use crate::domain::metadata::{DirtyState, SourceFileRecord, WikiMetadata};
use crate::domain::module_tree::ModuleTree;
use crate::domain::relation::WikiRelation;
use crate::domain::wiki_item::WikiItem;
use crate::generation::context::{build_module_contexts, build_page_context, build_repo_context};
use crate::generation::planner::plan_pages;
use crate::generation::renderer::render_page;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree;
use crate::repo::scanner::scan_repo;
use crate::storage::cache_store::{ensure_cache_dir, write_module_tree_cache, write_scan_cache};
use crate::storage::metadata_store::write_metadata;
use crate::storage::wiki_fs::{remove_runtime, write_page};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `init` 会全量生成 Repo Wiki 运行时。
/// 它是当前最完整的一条链路：扫描 -> 模块树 -> 页面 -> metadata/cache。
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

    // 这里按 deterministic pipeline 的顺序串起整条生成链。
    let scan_report = scan_repo(repo_root)?;
    let module_tree = build_module_tree(&scan_report);
    let repo_context = build_repo_context(&scan_report, &module_tree);
    let module_contexts = build_module_contexts(&scan_report, &module_tree);
    let pages = plan_pages(&scan_report, &module_tree, &repo_context, &module_contexts);

    ensure_cache_dir(repo_root)?;
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    let mut wiki_items = Vec::new();
    let mut relations = Vec::new();
    let mut generated_pages = Vec::new();
    let generated_at = current_timestamp();

    for page in &pages {
        // `PageContext` 是渲染层真正消费的输入；
        // 页面写盘和 metadata 建立都围绕这一步生成的事实组织。
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
        generated_pages.push(page_path.clone());

        wiki_items.push(WikiItem {
            id: page.id.clone(),
            title: page.title.clone(),
            path: page_path.clone(),
            item_type: page.page_type.clone(),
            parent_id: page.parent_id.clone(),
            module_ids: page.module_ids.clone(),
            source_files: source_paths_for_page(&scan_report, &page_context),
            content_hash: crate::repo::fingerprint::fingerprint_bytes(content.as_bytes()),
            summary: page_context.summary_inputs.join("；"),
        });

        if let Some(parent_id) = &page.parent_id {
            relations.push(WikiRelation {
                source_id: parent_id.clone(),
                target_id: page.id.clone(),
                relation_type: "PARENT_CHILD".to_string(),
                evidence: vec![page_path.clone()],
            });
        }
    }

    for edge in &module_tree.cross_module_edges {
        relations.push(WikiRelation {
            source_id: edge.source.clone(),
            target_id: edge.target.clone(),
            relation_type: edge.relation_type.clone(),
            evidence: edge.evidence.clone(),
        });
    }

    let metadata = WikiMetadata {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at,
        last_indexed_commit: current_commit(repo_root),
        modules: module_tree.modules.clone(),
        wiki_items,
        relations,
        source_files: scan_report
            .files
            .iter()
            .map(|file| SourceFileRecord {
                id: file.id.clone(),
                path: file.path.clone(),
                fingerprint: file.fingerprint.clone(),
                wiki_item_ids: pages_for_source(file.id.as_str(), &pages),
                module_ids: modules_for_source(file.id.as_str(), &module_tree),
            })
            .collect(),
        dirty_state: DirtyState::fresh(),
    };

    write_metadata(repo_root, &metadata)?;

    Ok(InitReport {
        initialized: true,
        state: metadata.dirty_state.status,
        generated_pages,
    })
}

/// 根据页面上下文里的 `source_ids` 反查源码路径。
///
/// # 参数
/// - `scan_report`：本次扫描得到的源码清单。
/// - `page_context`：当前页面对应的上下文对象。
///
/// # 返回
/// - 返回当前页面实际关联的源码路径列表。
fn source_paths_for_page(scan_report: &crate::repo::scanner::ScanReport, page_context: &PageContext) -> Vec<String> {
    scan_report
        .files
        .iter()
        .filter(|file| page_context.source_ids.contains(&file.id))
        .map(|file| file.path.clone())
        .collect()
}

/// 建立 source -> page 的反向映射。
///
/// # 参数
/// - `source_id`：需要查询的源码稳定 ID。
/// - `pages`：本次规划出的页面集合。
///
/// # 返回
/// - 返回引用该源码的页面 ID 列表。
fn pages_for_source(source_id: &str, pages: &[crate::generation::planner::PlannedPage]) -> Vec<String> {
    pages
        .iter()
        .filter(|page| page.source_ids.iter().any(|page_source_id| page_source_id == source_id))
        .map(|page| page.id.clone())
        .collect()
}

/// 建立 source -> module 的反向映射。
///
/// # 参数
/// - `source_id`：需要查询的源码稳定 ID。
/// - `module_tree`：当前仓库的模块树。
///
/// # 返回
/// - 返回包含该源码的模块 ID 列表。
fn modules_for_source(source_id: &str, module_tree: &ModuleTree) -> Vec<String> {
    module_tree
        .modules
        .iter()
        .filter(|module| module.source_ids.iter().any(|module_source_id| module_source_id == source_id))
        .map(|module| module.id.clone())
        .collect()
}

/// 统一生成 RFC3339 时间戳。
/// 如果格式化失败，再退回 Unix 时间，避免时间字段导致整个流程报错。
///
/// # 返回
/// - 返回可写入 metadata 的时间字符串。
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
