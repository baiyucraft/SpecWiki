//! managed section 内核负责页面级 parse / merge。
//! 它定义 marker 协议、区段类型、解析模式和合并计划，
//! 供 sync / update / rebuild 共享同一套页面语义。

use serde::{Deserialize, Serialize};

use crate::repo::fingerprint::fingerprint_bytes;

// ---------------------------------------------------------------------------
// Marker 协议常量
// ---------------------------------------------------------------------------

/// managed section 开始标记前缀。
pub const MARKER_START_PREFIX: &str = "<!-- wiki:managed:start";
/// managed section 结束标记前缀。
pub const MARKER_END_PREFIX: &str = "<!-- wiki:managed:end";
/// marker 协议当前版本号。
pub const MARKER_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// 区段类型
// ---------------------------------------------------------------------------

/// 页面中的一个区段，可以是 runtime 托管区段或用户手写区段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageBlock {
    /// runtime 托管区段，由生成器产出并以 marker 包裹。
    Managed(ManagedSectionBlock),
    /// 用户手写区段，位于 managed blocks 之间。
    User(UserSectionBlock),
}

/// runtime 托管区段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedSectionBlock {
    /// 稳定 section ID，与 state / cache 对齐。
    pub section_id: String,
    /// section 展示标题。
    pub title: String,
    /// marker 协议版本。
    pub version: u32,
    /// marker 之间的完整 Markdown 正文（不含 marker 行本身）。
    pub body: String,
}

/// 用户手写区段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSectionBlock {
    /// 运行时分配的临时 ID，仅用于当次 merge 的锚点引用。
    pub id: String,
    /// 用户区段的完整 Markdown 正文。
    pub body: String,
    /// 该 user section 前方最近的 managed section ID。
    pub anchor_after_section_id: Option<String>,
    /// 该 user section 后方最近的 managed section ID。
    pub anchor_before_section_id: Option<String>,
}

// ---------------------------------------------------------------------------
// 解析模式与解析结果
// ---------------------------------------------------------------------------

/// 页面解析模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageParseMode {
    /// 页面包含 managed marker，直接按 marker 解析。
    ManagedMarkers,
    /// legacy 页面无 marker，按已知 section 标题做 best-effort 迁移。
    LegacyHeadings,
}

/// 页面解析结果。
#[derive(Debug, Clone)]
pub struct ParsedWikiPage {
    /// 页面一级标题（`# xxx`），视为 runtime 托管内容。
    pub title: String,
    /// 按文件顺序排列的区段序列。
    pub blocks: Vec<PageBlock>,
    /// 本次解析使用的模式。
    pub parse_mode: PageParseMode,
    /// 解析过程中产生的警告信息。
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// 合并计划
// ---------------------------------------------------------------------------

/// 页面合并计划，描述如何把新生成的 managed sections 与已有 user sections 重新组装。
#[derive(Debug, Clone)]
pub struct PageMergePlan {
    /// 合并后的最终区段序列。
    pub blocks: Vec<PageBlock>,
    /// 合并过程中产生的警告信息（如锚点丢失）。
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Marker 渲染
// ---------------------------------------------------------------------------

/// 把一个 managed section 渲染成带 marker 的 Markdown 片段。
pub fn render_managed_block(section_id: &str, title: &str, body: &str) -> String {
    let start = format!(
        "{} id={} title=\"{}\" version={} -->",
        MARKER_START_PREFIX, section_id, title, MARKER_VERSION
    );
    let end = format!("{} id={} -->", MARKER_END_PREFIX, section_id);
    format!("{start}\n## {title}\n\n{body}\n{end}")
}

/// 把整页（标题 + 区段序列）组装成最终 Markdown。
pub fn render_page_with_markers(title: &str, blocks: &[PageBlock]) -> String {
    let mut parts = vec![format!("# {title}")];

    for block in blocks {
        match block {
            PageBlock::Managed(managed) => {
                parts.push(render_managed_block(
                    &managed.section_id,
                    &managed.title,
                    &managed.body,
                ));
            }
            PageBlock::User(user) => {
                parts.push(user.body.clone());
            }
        }
    }

    parts.join("\n\n")
}

// ---------------------------------------------------------------------------
// Marker 解析
// ---------------------------------------------------------------------------

/// 从 marker 开始行中提取属性。
/// 格式：`<!-- wiki:managed:start id=xxx title="yyy" version=1 -->`
fn parse_start_marker(line: &str) -> Option<(String, String, u32)> {
    let trimmed = line.trim();
    if !trimmed.starts_with(MARKER_START_PREFIX) {
        return None;
    }

    let attrs = &trimmed[MARKER_START_PREFIX.len()..]
        .trim_end_matches("-->")
        .trim();
    let id = extract_attr(attrs, "id")?;
    let title = extract_quoted_attr(attrs, "title")?;
    let version = extract_attr(attrs, "version")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(MARKER_VERSION);

    Some((id, title, version))
}

/// 从 marker 结束行中提取 section ID。
/// 格式：`<!-- wiki:managed:end id=xxx -->`
fn parse_end_marker(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with(MARKER_END_PREFIX) {
        return None;
    }

    let attrs = &trimmed[MARKER_END_PREFIX.len()..]
        .trim_end_matches("-->")
        .trim();
    extract_attr(attrs, "id")
}

/// 从属性字符串中提取简单 key=value。
fn extract_attr(attrs: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    for token in attrs.split_whitespace() {
        if let Some(value) = token.strip_prefix(&prefix) {
            // 去掉可能的引号
            let value = value.trim_matches('"');
            return Some(value.to_string());
        }
    }
    None
}

/// 从属性字符串中提取 key="quoted value"。
fn extract_quoted_attr(attrs: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=\"");
    if let Some(start) = attrs.find(&prefix) {
        let rest = &attrs[start + prefix.len()..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

/// 检测页面内容是否包含 managed marker。
pub fn has_managed_markers(content: &str) -> bool {
    content.contains(MARKER_START_PREFIX)
}

/// 以 managed marker 模式解析页面。
pub fn parse_with_markers(content: &str) -> ParsedWikiPage {
    let lines: Vec<&str> = content.lines().collect();
    let mut title = String::new();
    let mut blocks: Vec<PageBlock> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // 收集非 managed 内容的缓冲区
    let mut user_buf: Vec<&str> = Vec::new();
    // 上一个 managed section ID，用于 user section 锚点
    let mut last_managed_id: Option<String> = None;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // 提取页面一级标题
        if title.is_empty() {
            if let Some(h1) = line.strip_prefix("# ") {
                title = h1.trim().to_string();
                i += 1;
                continue;
            }
        }

        // 尝试解析 managed start marker
        if let Some((section_id, section_title, version)) = parse_start_marker(line) {
            // 先把之前积累的 user 内容刷出去
            flush_user_buf(
                &mut user_buf,
                &mut blocks,
                &last_managed_id,
                &Some(section_id.clone()),
            );

            // 收集 managed body 直到 end marker
            let mut body_lines: Vec<&str> = Vec::new();
            i += 1;
            let mut found_end = false;
            while i < lines.len() {
                if let Some(end_id) = parse_end_marker(lines[i]) {
                    if end_id == section_id {
                        found_end = true;
                        i += 1;
                        break;
                    }
                }
                body_lines.push(lines[i]);
                i += 1;
            }

            if !found_end {
                warnings.push(format!("managed section '{section_id}' 缺少结束 marker"));
            }

            // body 中去掉开头的 ## 标题行（如果存在）
            let body = strip_leading_heading(&body_lines);

            blocks.push(PageBlock::Managed(ManagedSectionBlock {
                section_id: section_id.clone(),
                title: section_title,
                version,
                body,
            }));
            last_managed_id = Some(section_id);
            continue;
        }

        user_buf.push(line);
        i += 1;
    }

    // 刷出尾部 user 内容
    flush_user_buf(&mut user_buf, &mut blocks, &last_managed_id, &None);

    ParsedWikiPage {
        title,
        blocks,
        parse_mode: PageParseMode::ManagedMarkers,
        warnings,
    }
}

/// 以 legacy heading 模式解析页面。
/// `known_titles` 是该页面类型的已知 managed section 标题集合。
pub fn parse_with_legacy_headings(content: &str, known_titles: &[&str]) -> ParsedWikiPage {
    let lines: Vec<&str> = content.lines().collect();
    let mut title = String::new();
    let mut blocks: Vec<PageBlock> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // 按 ## 标题切分区段
    let mut current_heading: Option<String> = None;
    let mut current_lines: Vec<&str> = Vec::new();
    let mut last_managed_id: Option<String> = None;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // 提取页面一级标题
        if title.is_empty() {
            if let Some(h1) = line.strip_prefix("# ") {
                title = h1.trim().to_string();
                i += 1;
                continue;
            }
        }

        // 检测 ## 标题
        if let Some(h2) = line.strip_prefix("## ") {
            // 先刷出之前的区段
            if !current_lines.is_empty() || current_heading.is_some() {
                flush_legacy_section(
                    &current_heading,
                    &current_lines,
                    known_titles,
                    &mut blocks,
                    &mut last_managed_id,
                    &mut warnings,
                );
            }
            current_heading = Some(h2.trim().to_string());
            current_lines.clear();
            i += 1;
            continue;
        }

        current_lines.push(line);
        i += 1;
    }

    // 刷出最后一个区段
    if !current_lines.is_empty() || current_heading.is_some() {
        flush_legacy_section(
            &current_heading,
            &current_lines,
            known_titles,
            &mut blocks,
            &mut last_managed_id,
            &mut warnings,
        );
    }

    if !blocks.iter().any(|b| matches!(b, PageBlock::Managed(_))) && !content.trim().is_empty() {
        warnings.push("legacy 页面无法识别任何 managed section 标题".to_string());
    }

    ParsedWikiPage {
        title,
        blocks,
        parse_mode: PageParseMode::LegacyHeadings,
        warnings,
    }
}

/// 统一解析入口：自动检测 marker 模式或 legacy 模式。
pub fn parse_wiki_page(content: &str, known_titles: &[&str]) -> ParsedWikiPage {
    if has_managed_markers(content) {
        parse_with_markers(content)
    } else {
        parse_with_legacy_headings(content, known_titles)
    }
}

// ---------------------------------------------------------------------------
// Merge
// ---------------------------------------------------------------------------

/// 把新生成的 managed sections 与已有 user sections 合并成最终页面。
///
/// # 参数
/// - `new_managed`：本次生成的 managed section 列表（按期望顺序）。
/// - `old_parsed`：上一次 sync 后解析出的页面区段序列。
///
/// # 返回
/// - 合并计划，包含最终区段序列和警告。
pub fn merge_sections(
    new_managed: &[ManagedSectionBlock],
    old_parsed: &ParsedWikiPage,
) -> PageMergePlan {
    let mut result_blocks: Vec<PageBlock> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // 收集旧页面中的 user sections 及其锚点
    let user_sections: Vec<&UserSectionBlock> = old_parsed
        .blocks
        .iter()
        .filter_map(|b| match b {
            PageBlock::User(u) => Some(u),
            _ => None,
        })
        .collect();

    // 新 managed section ID 集合
    let new_managed_ids: Vec<&str> = new_managed.iter().map(|m| m.section_id.as_str()).collect();

    // 按新 managed 顺序插入，在每个 managed section 之后检查是否有 user section 需要回插
    for managed in new_managed {
        result_blocks.push(PageBlock::Managed(managed.clone()));

        // 找到 anchor_after_section_id == 当前 managed section 的 user sections
        for user in &user_sections {
            if user.anchor_after_section_id.as_deref() == Some(&managed.section_id) {
                // 如果 before 锚点也存在于新 managed 集合中，确认位置正确
                let before_ok = match &user.anchor_before_section_id {
                    Some(before_id) => new_managed_ids.contains(&before_id.as_str()),
                    None => true,
                };
                if before_ok {
                    result_blocks.push(PageBlock::User((*user).clone()));
                }
            }
        }
    }

    // 处理只有 before 锚点的 user sections（插在对应 managed section 之前）
    // 以及锚点都丢失的 user sections（追加到末尾）
    for user in &user_sections {
        let already_inserted = result_blocks.iter().any(|b| match b {
            PageBlock::User(u) => u.id == user.id,
            _ => false,
        });
        if already_inserted {
            continue;
        }

        // 尝试 before 锚点
        if let Some(before_id) = &user.anchor_before_section_id {
            if let Some(pos) = result_blocks.iter().position(|b| match b {
                PageBlock::Managed(m) => m.section_id == *before_id,
                _ => false,
            }) {
                result_blocks.insert(pos, PageBlock::User((*user).clone()));
                continue;
            }
        }

        // 锚点都丢失，追加到末尾
        warnings.push(format!(
            "user section '{}' 的锚点已丢失，追加到页面末尾",
            user.id
        ));
        result_blocks.push(PageBlock::User((*user).clone()));
    }

    PageMergePlan {
        blocks: result_blocks,
        warnings,
    }
}

/// 计算区段内容的 hash。
pub fn content_hash(body: &str) -> String {
    fingerprint_bytes(body.as_bytes())
}

// ---------------------------------------------------------------------------
// 内部辅助
// ---------------------------------------------------------------------------

/// 把 body 行中开头的 `## xxx` 标题行去掉，返回纯正文。
fn strip_leading_heading(lines: &[&str]) -> String {
    let mut start = 0;
    // 跳过空行
    while start < lines.len() && lines[start].trim().is_empty() {
        start += 1;
    }
    // 跳过 ## 标题行
    if start < lines.len() && lines[start].starts_with("## ") {
        start += 1;
    }
    // 跳过标题后的空行
    while start < lines.len() && lines[start].trim().is_empty() {
        start += 1;
    }
    // 去掉尾部空行
    let mut end = lines.len();
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    lines[start..end].join("\n")
}

/// 把 user 缓冲区刷成 UserSectionBlock（如果非空）。
fn flush_user_buf(
    buf: &mut Vec<&str>,
    blocks: &mut Vec<PageBlock>,
    last_managed_id: &Option<String>,
    next_managed_id: &Option<String>,
) {
    let text = buf.join("\n").trim().to_string();
    if !text.is_empty() {
        let id = format!(
            "user-{}",
            fingerprint_bytes(text.as_bytes())
                .chars()
                .take(8)
                .collect::<String>()
        );
        blocks.push(PageBlock::User(UserSectionBlock {
            id,
            body: text,
            anchor_after_section_id: last_managed_id.clone(),
            anchor_before_section_id: next_managed_id.clone(),
        }));
    }
    buf.clear();
}

/// legacy 模式下，把一个 heading + body 区段刷成 managed 或 user block。
fn flush_legacy_section(
    heading: &Option<String>,
    lines: &[&str],
    known_titles: &[&str],
    blocks: &mut Vec<PageBlock>,
    last_managed_id: &mut Option<String>,
    _warnings: &mut Vec<String>,
) {
    let body = lines.join("\n").trim().to_string();

    match heading {
        Some(h) if known_titles.contains(&h.as_str()) => {
            // 已知 managed 标题 → 转为 managed block
            // 用标题生成临时 section ID（legacy 迁移后会被正式 ID 替换）
            let section_id = format!(
                "legacy-{}",
                fingerprint_bytes(h.as_bytes())
                    .chars()
                    .take(12)
                    .collect::<String>()
            );
            blocks.push(PageBlock::Managed(ManagedSectionBlock {
                section_id: section_id.clone(),
                title: h.clone(),
                version: MARKER_VERSION,
                body,
            }));
            *last_managed_id = Some(section_id);
        }
        Some(h) => {
            // 未知标题 → user section
            let full_body = if body.is_empty() {
                format!("## {h}")
            } else {
                format!("## {h}\n\n{body}")
            };
            let id = format!(
                "user-{}",
                fingerprint_bytes(full_body.as_bytes())
                    .chars()
                    .take(8)
                    .collect::<String>()
            );
            blocks.push(PageBlock::User(UserSectionBlock {
                id,
                body: full_body,
                anchor_after_section_id: last_managed_id.clone(),
                anchor_before_section_id: None,
            }));
        }
        None => {
            // 标题前的内容 → user section
            if !body.is_empty() {
                let id = format!(
                    "user-{}",
                    fingerprint_bytes(body.as_bytes())
                        .chars()
                        .take(8)
                        .collect::<String>()
                );
                blocks.push(PageBlock::User(UserSectionBlock {
                    id,
                    body,
                    anchor_after_section_id: last_managed_id.clone(),
                    anchor_before_section_id: None,
                }));
            }
        }
    }
}
