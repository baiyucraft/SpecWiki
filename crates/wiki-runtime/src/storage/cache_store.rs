//! cache 存储层负责 `.wiki/.cache/` 下各类运行时缓存的读写与失效。
//! 所有缓存通过 SQLite 统一存储，不再使用散落的 JSON 文件。

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::context::PageContext;
use crate::domain::module_tree::ModuleTree;
use crate::domain::state::WikiState;
use crate::generation::sections::SectionDraft;
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::sqlite_store;
use wiki_index::scanner::ScanReport;
use wiki_index::store::IndexSnapshotStore;

/// 确保 `.wiki/.cache/` 存在。
pub fn ensure_cache_dir(repo_root: &Path) -> io::Result<()> {
    fs::create_dir_all(cache_dir(repo_root))
}

/// 返回 cache 根目录。
pub fn cache_dir(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join(".cache")
}

/// `PageContextCacheEntry` 让 update 可以按页面复用上下文输入签名。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageContextCacheEntry {
    pub page_id: String,
    pub input_hash: String,
    pub context: PageContext,
}

/// `PageGenerationCacheEntry` 保存按页面拆分的 section 级生成结果。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageGenerationCacheEntry {
    pub page_id: String,
    pub input_hash: String,
    pub content_hash: String,
    pub sections: Vec<SectionDraft>,
}

/// 检查 cache layout 是否完整（DB 存在且可打开）。
pub fn has_cache_layout(repo_root: &Path) -> bool {
    sqlite_store::db_exists(repo_root) && sqlite_store::open_db_readonly(repo_root).is_ok()
}

/// 写入扫描缓存到 SQLite scan_cache。
pub fn write_scan_cache(repo_root: &Path, report: &ScanReport) -> io::Result<()> {
    let store = SqliteIndexStore::new(repo_root);
    store.write_scan_report(report)
}

/// 从 SQLite scan_cache 读取扫描缓存。
pub fn read_scan_cache(repo_root: &Path) -> io::Result<ScanReport> {
    let store = SqliteIndexStore::new(repo_root);
    store
        .read_scan_report()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "repo-scan not found in DB"))
}

/// 写入模块树缓存到 SQLite scan_cache。
pub fn write_module_tree_cache(repo_root: &Path, module_tree: &ModuleTree) -> io::Result<()> {
    let store = SqliteIndexStore::new(repo_root);
    store.write_module_tree(module_tree)
}

/// 从 SQLite scan_cache 读取模块树缓存。
pub fn read_module_tree_cache(repo_root: &Path) -> io::Result<ModuleTree> {
    let store = SqliteIndexStore::new(repo_root);
    store
        .read_module_tree()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "module-tree not found in DB"))
}

/// 确保 DB 已初始化（替代原来的 ensure_page_cache_dirs）。
pub fn ensure_page_cache_dirs(repo_root: &Path) -> io::Result<()> {
    sqlite_store::open_db(repo_root)?;
    Ok(())
}

/// 写入单页上下文缓存到 SQLite。
pub fn write_page_context_cache(repo_root: &Path, entry: &PageContextCacheEntry) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    let context_json =
        serde_json::to_string(&entry.context).map_err(|e| io::Error::other(e.to_string()))?;
    sqlite_store::write_page_context(&conn, &entry.page_id, &entry.input_hash, &context_json)
}

/// 从 SQLite 读取单页上下文缓存。
pub fn read_page_context_cache(
    repo_root: &Path,
    page_id: &str,
) -> io::Result<PageContextCacheEntry> {
    let conn = sqlite_store::open_db_readonly(repo_root)?;
    match sqlite_store::read_page_context(&conn, page_id)? {
        Some((input_hash, context_json)) => {
            let context: PageContext =
                serde_json::from_str(&context_json).map_err(|e| io::Error::other(e.to_string()))?;
            Ok(PageContextCacheEntry {
                page_id: page_id.to_string(),
                input_hash,
                context,
            })
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("page context cache not found: {page_id}"),
        )),
    }
}

/// 写入单页生成缓存到 SQLite。
pub fn write_page_generation_cache(
    repo_root: &Path,
    entry: &PageGenerationCacheEntry,
) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    let sections_json =
        serde_json::to_string(&entry.sections).map_err(|e| io::Error::other(e.to_string()))?;
    sqlite_store::write_page_generation(
        &conn,
        &entry.page_id,
        &entry.input_hash,
        &entry.content_hash,
        &sections_json,
    )
}

/// 从 SQLite 读取单页生成缓存。
pub fn read_page_generation_cache(
    repo_root: &Path,
    page_id: &str,
) -> io::Result<PageGenerationCacheEntry> {
    let conn = sqlite_store::open_db_readonly(repo_root)?;
    match sqlite_store::read_page_generation(&conn, page_id)? {
        Some((input_hash, content_hash, sections_json)) => {
            let sections: Vec<SectionDraft> = serde_json::from_str(&sections_json)
                .map_err(|e| io::Error::other(e.to_string()))?;
            Ok(PageGenerationCacheEntry {
                page_id: page_id.to_string(),
                input_hash,
                content_hash,
                sections,
            })
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("page generation cache not found: {page_id}"),
        )),
    }
}

/// 删除某个页面对应的所有增量缓存。
pub fn remove_page_caches(repo_root: &Path, page_id: &str) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::remove_page_all(&conn, page_id)
}

/// 逐页检查增量 runtime 所需的 cache 组件是否完整。
pub fn missing_incremental_cache_components(repo_root: &Path, state: &WikiState) -> Vec<String> {
    let mut missing = Vec::new();
    let index_store = SqliteIndexStore::new(repo_root);

    if !sqlite_store::db_exists(repo_root) {
        missing.push("wiki-db".to_string());
        return missing;
    }

    let conn = match sqlite_store::open_db_readonly(repo_root) {
        Ok(c) => c,
        Err(_) => {
            missing.push("wiki-db".to_string());
            return missing;
        }
    };

    if !sqlite_store::runtime_tables_exist(&conn).unwrap_or(false) {
        missing.push("runtime-state".to_string());
        return missing;
    }

    if index_store.read_scan_report().ok().flatten().is_none() {
        missing.push("repo-scan".to_string());
    }

    if index_store.read_module_tree().ok().flatten().is_none() {
        missing.push("module-tree".to_string());
    }

    if crate::storage::state_store::read_state(repo_root).is_err() {
        missing.push("wiki-state".to_string());
    }

    if sqlite_store::runtime_meta_get(&conn, "pipeline_runtime_summary")
        .ok()
        .flatten()
        .is_none()
    {
        missing.push("pipeline-runtime-summary".to_string());
    }

    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn).unwrap_or_default();
    if runtime_gates.is_empty() {
        missing.push("unit-runtime-gates".to_string());
    }
    let gated_unit_ids = runtime_gates
        .into_iter()
        .map(|gate| gate.unit_id)
        .collect::<BTreeSet<_>>();
    let parent_page_ids = state
        .pages
        .iter()
        .filter_map(|page| page.parent_id.clone())
        .collect::<BTreeSet<_>>();

    for page in &state.pages {
        if !sqlite_store::page_sections_exist(&conn, &page.page_id).unwrap_or(false) {
            missing.push(format!("page-sections:{}", page.page_id));
        }

        if !sqlite_store::page_context_exists(&conn, &page.page_id).unwrap_or(false) {
            missing.push(format!("page-context:{}", page.page_id));
        }

        if !sqlite_store::page_generation_exists(&conn, &page.page_id).unwrap_or(false) {
            missing.push(format!("page-generation:{}", page.page_id));
        }

        let page_context = match sqlite_store::read_page_context(&conn, &page.page_id) {
            Ok(Some((_input_hash, context_json))) => {
                match serde_json::from_str::<PageContext>(&context_json) {
                    Ok(context) => Some(context),
                    Err(_) => {
                        missing.push(format!("page-context-payload:{}", page.page_id));
                        None
                    }
                }
            }
            Ok(None) => None,
            Err(_) => {
                missing.push(format!("page-context-read:{}", page.page_id));
                None
            }
        };
        let Some(context) = page_context else {
            continue;
        };

        if let Some(unit_id) = context.unit_id.as_deref() {
            if !gated_unit_ids.contains(unit_id) {
                missing.push(format!("unit-runtime-gate:{unit_id}"));
            }
        }

        if parent_page_ids.contains(&page.page_id) {
            if context.readiness_status.is_empty() {
                missing.push(format!("page-context-readiness:{}", page.page_id));
            }

            let has_child_contract = !context.child_summaries.is_empty()
                || !context.child_unit_ids.is_empty()
                || !context.child_page_ids.is_empty()
                || !context.child_digest_ids.is_empty()
                || !context.citation_digest_refs.is_empty()
                || !context.diagram_digest_refs.is_empty();
            if context.readiness_status == "compose_ready" && !has_child_contract {
                missing.push(format!("page-context-child-contract:{}", page.page_id));
            }
        }
    }

    missing
}
