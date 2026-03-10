//! `sync` 负责把用户对 `.wiki/*.md` 的外部修改同步回 WikiState 和 metadata。
//! 迭代 5 改为 parser-first：解析页面区段结构，回写 section 状态、summary 和 metadata。

use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::WikiSectionState;
use crate::domain::steering::load_steering_config;
use crate::generation::managed_sections::{
    content_hash, parse_wiki_page, PageBlock, ParsedWikiPage,
};
use crate::generation::sections::section_titles_for_page_type;
use crate::repo::fingerprint::fingerprint_bytes;
use crate::repo::git::{current_branch, current_commit};
use crate::repo::hierarchy::build_module_tree;
use crate::repo::scanner::scan_repo_with_boundary;
use crate::storage::cache_store::{
    read_page_generation_cache, write_module_tree_cache, write_scan_cache,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::state_store::{load_or_rebuild_state, write_state};
use crate::storage::wiki_fs::resolve_page_path;
use crate::workflows::init::current_timestamp;

/// `sync` 的输出报告。
#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    /// 同步后的运行时状态。
    pub state: String,
    /// 本次被识别为变化的页面路径列表。
    pub synced_pages: Vec<String>,
    /// 同步过程中产生的警告信息。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// parser-first sync：解析页面区段结构，回写 section 状态、summary 和 metadata。
///
/// # 参数
/// - `repo_root`：包含 `.wiki/` 运行时的本地代码目录。
///
/// # 返回
/// - 成功时返回同步报告，包含变化页面和警告。
pub fn run_sync(repo_root: &Path) -> io::Result<SyncReport> {
    let mut wiki_state = load_or_rebuild_state(repo_root)?;
    let mut synced_pages = Vec::new();
    let mut all_warnings = Vec::new();

    for page in &mut wiki_state.pages {
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = match fs::read_to_string(&page_path) {
            Ok(c) => c,
            Err(_) => continue, // 页面文件不存在，跳过
        };

        let new_hash = fingerprint_bytes(content.as_bytes());
        let page_changed = page.content_hash != new_hash;

        if !page_changed {
            continue;
        }

        // 确定已知 managed section 标题（用于 legacy 模式）
        let known_titles = section_titles_for_page_type(&page.page_type);
        let known_titles_ref: Vec<&str> = known_titles.iter().copied().collect();

        // 解析页面区段结构
        let parsed = parse_wiki_page(&content, &known_titles_ref);

        // 收集解析警告
        for w in &parsed.warnings {
            all_warnings.push(format!("[{}] {}", page.path, w));
        }

        // 从 generation cache 获取 generated hash 映射（如果有）
        let gen_hash_map = load_generated_hashes(repo_root, &page.page_id);

        // 把解析结果转换成 section 状态
        let new_sections = build_section_states_from_parsed(&parsed, &gen_hash_map);

        // 检测 managed drift
        for section in &new_sections {
            if section.managed {
                if let (Some(gen_hash), ref obs_hash) =
                    (&section.generated_content_hash, &section.content_hash)
                {
                    if gen_hash.as_str() != obs_hash.as_str() {
                        all_warnings.push(format!(
                            "[{}] managed section '{}' 内容已被手工修改 (managed drift)",
                            page.path, section.title
                        ));
                    }
                }
            }
        }

        // 更新页面状态
        page.content_hash = new_hash;
        page.sections = new_sections;
        page.section_anchors = page
            .sections
            .iter()
            .filter(|section| section.managed)
            .map(|section| section.section_id.clone())
            .collect();
        page.summary = extract_summary_from_parsed(&parsed);

        synced_pages.push(page.path.clone());
    }

    // 持久化 WikiState
    let generated_at = current_timestamp();
    wiki_state.dirty_state = crate::domain::metadata::DirtyState::fresh();
    wiki_state.build_state.generated_at = generated_at.clone();
    write_state(repo_root, &wiki_state)?;

    // 导出 metadata
    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at,
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&wiki_state, &export_context);
    write_metadata(repo_root, &metadata)?;

    // 刷新扫描和模块树缓存
    let steering = load_steering_config(repo_root);
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths)?;
    let module_tree = build_module_tree(&scan_report);
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    Ok(SyncReport {
        state: "fresh".to_string(),
        synced_pages,
        warnings: all_warnings,
    })
}

/// 从 page generation cache 加载 section_id → generated_content_hash 映射。
fn load_generated_hashes(
    repo_root: &Path,
    page_id: &str,
) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Ok(cache) = read_page_generation_cache(repo_root, page_id) {
        for section in &cache.sections {
            map.insert(
                section.section_id.clone(),
                fingerprint_bytes(section.content.as_bytes()),
            );
        }
    }
    map
}

/// 把解析出的页面区段转换成 WikiSectionState 集合。
fn build_section_states_from_parsed(
    parsed: &ParsedWikiPage,
    gen_hash_map: &std::collections::HashMap<String, String>,
) -> Vec<WikiSectionState> {
    parsed
        .blocks
        .iter()
        .map(|block| match block {
            PageBlock::Managed(m) => {
                let observed_hash = content_hash(&m.body);
                let generated_hash = gen_hash_map.get(&m.section_id).cloned();
                WikiSectionState {
                    section_id: m.section_id.clone(),
                    title: m.title.clone(),
                    managed: true,
                    content_hash: observed_hash,
                    generated_content_hash: generated_hash,
                    anchor_after_section_id: None,
                    anchor_before_section_id: None,
                    source_ids: Vec::new(),
                    relation_ids: Vec::new(),
                }
            }
            PageBlock::User(u) => WikiSectionState {
                section_id: u.id.clone(),
                title: String::new(),
                managed: false,
                content_hash: content_hash(&u.body),
                generated_content_hash: None,
                anchor_after_section_id: u.anchor_after_section_id.clone(),
                anchor_before_section_id: u.anchor_before_section_id.clone(),
                source_ids: Vec::new(),
                relation_ids: Vec::new(),
            },
        })
        .collect()
}

/// 从解析结果中提取页面摘要。
/// 优先取第一个 managed section 的 body 前 200 字符。
fn extract_summary_from_parsed(parsed: &ParsedWikiPage) -> String {
    for block in &parsed.blocks {
        if let PageBlock::Managed(m) = block {
            let trimmed = m.body.trim();
            if !trimmed.is_empty() {
                let summary: String = trimmed.chars().take(200).collect();
                return summary;
            }
        }
    }
    String::new()
}
