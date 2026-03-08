use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::storage::metadata_store::read_metadata;
use crate::storage::wiki_fs::resolve_page_path;

/// `QueryMatch` 描述一个命中的页面，以及它为什么命中。
#[derive(Debug, Clone, Serialize)]
pub struct QueryMatch {
    pub page_id: String,
    pub title: String,
    pub path: String,
    pub item_type: String,
    pub module_ids: Vec<String>,
    pub source_files: Vec<String>,
    pub summary: String,
}

/// `QueryModuleMatch` 是 query 返回的模块视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryModuleMatch {
    pub module_id: String,
    pub name: String,
    pub root_paths: Vec<String>,
}

/// `QuerySourceMatch` 是 query 返回的源码视图。
#[derive(Debug, Clone, Serialize)]
pub struct QuerySourceMatch {
    pub source_id: String,
    pub path: String,
    pub module_ids: Vec<String>,
}

/// `QueryReport` 是当前对 Agent 最友好的结构化返回。
#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    pub term: String,
    pub matched_pages: Vec<String>,
    pub matched_modules: Vec<QueryModuleMatch>,
    pub matched_sources: Vec<QuerySourceMatch>,
    pub matches: Vec<QueryMatch>,
}

/// 执行关键词查询。
/// 当前实现是 metadata + Markdown 内容的混合检索，还不是完整语义搜索。
///
/// # 参数
/// - `repo_root`：要查询的本地代码目录。
/// - `term`：要匹配的关键词。
///
/// # 返回
/// - 成功时返回匹配到的页面、模块、源码以及命中原因摘要。
///
/// # 错误
/// - 当 metadata 或页面文件读取失败时返回错误。
pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    let metadata = read_metadata(repo_root)?;
    let needle = term.trim().to_lowercase();
    let mut matched_pages = Vec::new();
    let mut matched_modules = BTreeMap::new();
    let mut matched_sources = BTreeMap::new();
    let mut matches = Vec::new();

    if needle.is_empty() {
        return Ok(QueryReport {
            term: term.to_string(),
            matched_pages,
            matched_modules: Vec::new(),
            matched_sources: Vec::new(),
            matches,
        });
    }

    // 这些索引表是为了让后续页面命中时，能顺手把关联模块和源码一起带出来。
    let module_index = metadata
        .modules
        .iter()
        .map(|module| (module.id.clone(), module))
        .collect::<BTreeMap<_, _>>();
    let source_index = metadata
        .source_files
        .iter()
        .map(|source| (source.path.clone(), source))
        .collect::<BTreeMap<_, _>>();

    for item in &metadata.wiki_items {
        let page_path = resolve_page_path(repo_root, &item.path);
        let content = fs::read_to_string(&page_path).unwrap_or_default();
        let mut reasons = Vec::new();

        // 当前“命中原因”是显式保留下来的。
        // 这样 Agent 或调用方能看出结果来自标题、路径、内容还是模块/源码关系。
        if item.title.to_lowercase().contains(&needle) {
            reasons.push("标题匹配");
        }

        if item.path.to_lowercase().contains(&needle) {
            reasons.push("路径匹配");
        }

        if content.to_lowercase().contains(&needle) {
            reasons.push("内容匹配");
        }

        if item
            .module_ids
            .iter()
            .filter_map(|module_id| module_index.get(module_id))
            .any(|module| module.name.to_lowercase().contains(&needle))
        {
            reasons.push("模块匹配");
        }

        if item
            .source_files
            .iter()
            .any(|source| source.to_lowercase().contains(&needle))
        {
            reasons.push("源码匹配");
        }

        if !reasons.is_empty() {
            matched_pages.push(item.path.clone());

            for module_id in &item.module_ids {
                if let Some(module) = module_index.get(module_id) {
                    matched_modules
                        .entry(module.id.clone())
                        .or_insert(QueryModuleMatch {
                            module_id: module.id.clone(),
                            name: module.name.clone(),
                            root_paths: module.root_paths.clone(),
                        });
                }
            }

            for source_path in &item.source_files {
                if let Some(source) = source_index.get(source_path) {
                    matched_sources
                        .entry(source.id.clone())
                        .or_insert(QuerySourceMatch {
                            source_id: source.id.clone(),
                            path: source.path.clone(),
                            module_ids: source.module_ids.clone(),
                        });
                }
            }

            matches.push(QueryMatch {
                page_id: item.id.clone(),
                title: item.title.clone(),
                path: item.path.clone(),
                item_type: item.item_type.clone(),
                module_ids: item.module_ids.clone(),
                source_files: item.source_files.clone(),
                summary: reasons.join("、"),
            });
        }
    }

    Ok(QueryReport {
        term: term.to_string(),
        matched_pages,
        matched_modules: matched_modules.into_values().collect(),
        matched_sources: matched_sources.into_values().collect(),
        matches,
    })
}
