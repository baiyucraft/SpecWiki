//! managed section 内核负责页面级 parse / merge。
//! 它定义 marker 协议、区段类型、解析诊断和合并计划，
//! 供 sync / update / rebuild 共享同一套页面语义。

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use wiki_index::fingerprint::fingerprint_bytes;
use wiki_model::domain::projection::{SectionBinding, SectionOwnership, SyncResultKind};

// ---------------------------------------------------------------------------
// Marker 协议常量
// ---------------------------------------------------------------------------

pub const MARKER_START_PREFIX: &str = "<!-- wiki:managed:start";
pub const MARKER_END_PREFIX: &str = "<!-- wiki:managed:end";
pub const MARKER_VERSION: u32 = 2;

// ---------------------------------------------------------------------------
// 区段类型
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageBlock {
    Managed(ManagedSectionBlock),
    User(UserSectionBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedSectionBlock {
    pub section_id: String,
    pub owner_kind: SectionOwnership,
    pub title: String,
    pub version: u32,
    pub body: String,
    #[serde(default)]
    pub knowledge_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub input_hash: String,
    #[serde(default)]
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection_digest_ref: Option<String>,
}

impl ManagedSectionBlock {
    pub fn generated(section_id: String, title: String, body: String) -> Self {
        let body_hash = content_hash(&body);
        Self {
            section_id,
            owner_kind: SectionOwnership::DerivedManaged,
            title,
            version: MARKER_VERSION,
            body,
            knowledge_refs: Vec::new(),
            source_refs: Vec::new(),
            input_hash: String::new(),
            content_hash: body_hash.clone(),
            generated_content_hash: Some(body_hash),
            projection_digest_ref: None,
        }
    }

    pub fn section_binding(&self) -> SectionBinding {
        SectionBinding {
            section_id: self.section_id.clone(),
            owner_kind: Some(self.owner_kind),
            knowledge_refs: self.knowledge_refs.clone(),
            source_refs: self.source_refs.clone(),
            input_hash: self.input_hash.clone(),
            content_hash: self.content_hash.clone(),
            projection_status: Default::default(),
            projection_digest_ref: self.projection_digest_ref.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSectionBlock {
    pub id: String,
    pub body: String,
    pub anchor_after_section_id: Option<String>,
    pub anchor_before_section_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SectionBindingIndex {
    bindings: BTreeMap<String, SectionBinding>,
}

impl SectionBindingIndex {
    pub fn from_bindings(bindings: impl IntoIterator<Item = SectionBinding>) -> Self {
        Self {
            bindings: bindings
                .into_iter()
                .map(|binding| (binding.section_id.clone(), binding))
                .collect(),
        }
    }

    pub fn from_blocks(blocks: &[ManagedSectionBlock]) -> Self {
        Self::from_bindings(blocks.iter().map(ManagedSectionBlock::section_binding))
    }

    pub fn get(&self, section_id: &str) -> Option<&SectionBinding> {
        self.bindings.get(section_id)
    }
}

// ---------------------------------------------------------------------------
// 解析模式与解析结果
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageParseMode {
    ManagedMarkers,
    UnmanagedOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageParseDiagnosticKind {
    MarkerMissing,
    MarkerMalformed,
    MarkerMissingId,
    MarkerMissingOwner,
    MarkerVersionUnsupported,
    MarkerEndMismatch,
    MetadataBindingMismatch,
    HashMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageParseDiagnostic {
    pub kind: PageParseDiagnosticKind,
    pub section_id: Option<String>,
    pub message: String,
}

impl PageParseDiagnostic {
    fn new(
        kind: PageParseDiagnosticKind,
        section_id: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            section_id,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedWikiPage {
    pub title: String,
    pub blocks: Vec<PageBlock>,
    pub parse_mode: PageParseMode,
    pub warnings: Vec<String>,
    pub diagnostics: Vec<PageParseDiagnostic>,
}

impl ParsedWikiPage {
    pub fn managed_blocks(&self) -> Vec<&ManagedSectionBlock> {
        self.blocks
            .iter()
            .filter_map(|block| match block {
                PageBlock::Managed(managed) => Some(managed),
                PageBlock::User(_) => None,
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct PageMergePlan {
    pub blocks: Vec<PageBlock>,
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Marker 渲染
// ---------------------------------------------------------------------------

pub fn render_managed_block(section_id: &str, title: &str, body: &str) -> String {
    render_managed_section_block(&ManagedSectionBlock::generated(
        section_id.to_string(),
        title.to_string(),
        body.to_string(),
    ))
}

fn render_managed_section_block(block: &ManagedSectionBlock) -> String {
    let start = format!(
        "{} id={} owner={} title=\"{}\" version={} knowledge=\"{}\" source=\"{}\" input-hash={} content-hash={} generated-content-hash={} projection=\"{}\" -->",
        MARKER_START_PREFIX,
        block.section_id,
        block.owner_kind.as_str(),
        block.title,
        block.version,
        block.knowledge_refs.join(","),
        block.source_refs.join(","),
        block.input_hash,
        block.content_hash,
        block.generated_content_hash.as_deref().unwrap_or_default(),
        block.projection_digest_ref.as_deref().unwrap_or_default()
    );
    let end = format!("{} id={} -->", MARKER_END_PREFIX, block.section_id);
    if block.title.trim().is_empty() {
        format!("{start}\n{}\n{end}", block.body)
    } else {
        format!("{start}\n## {}\n\n{}\n{end}", block.title, block.body)
    }
}

pub fn render_page_with_markers(title: &str, blocks: &[PageBlock]) -> String {
    let mut parts = vec![format!("# {title}")];
    for block in blocks {
        match block {
            PageBlock::Managed(managed) => parts.push(render_managed_section_block(managed)),
            PageBlock::User(user) => parts.push(user.body.clone()),
        }
    }
    parts.join("\n\n")
}

// ---------------------------------------------------------------------------
// Marker 解析
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct ManagedStartMarker {
    section_id: String,
    owner_kind: SectionOwnership,
    title: String,
    version: u32,
    knowledge_refs: Vec<String>,
    source_refs: Vec<String>,
    input_hash: String,
    content_hash: String,
    generated_content_hash: Option<String>,
    projection_digest_ref: Option<String>,
}

fn parse_start_marker(line: &str) -> Result<Option<ManagedStartMarker>, PageParseDiagnostic> {
    let trimmed = line.trim();
    if !trimmed.starts_with(MARKER_START_PREFIX) {
        return Ok(None);
    }
    if !trimmed.ends_with("-->") {
        return Err(PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerMalformed,
            None,
            "managed start marker 非法",
        ));
    }

    let attrs = &trimmed[MARKER_START_PREFIX.len()..]
        .trim_end_matches("-->")
        .trim();
    let id = extract_attr(attrs, "id").ok_or_else(|| {
        PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerMissingId,
            None,
            "managed start marker 缺少 id",
        )
    })?;
    let owner = extract_attr(attrs, "owner").ok_or_else(|| {
        PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerMissingOwner,
            Some(id.clone()),
            "managed start marker 缺少 owner",
        )
    })?;
    let owner_kind = parse_owner_kind(owner.as_str()).ok_or_else(|| {
        PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerMalformed,
            Some(id.clone()),
            format!("unsupported section owner: {owner}"),
        )
    })?;
    let version = extract_attr(attrs, "version")
        .ok_or_else(|| {
            PageParseDiagnostic::new(
                PageParseDiagnosticKind::MarkerVersionUnsupported,
                Some(id.clone()),
                "managed start marker 缺少 version",
            )
        })?
        .parse::<u32>()
        .map_err(|_| {
            PageParseDiagnostic::new(
                PageParseDiagnosticKind::MarkerVersionUnsupported,
                Some(id.clone()),
                "managed start marker version 非法",
            )
        })?;
    if version != MARKER_VERSION {
        return Err(PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerVersionUnsupported,
            Some(id.clone()),
            format!("unsupported marker version: {version}"),
        ));
    }

    Ok(Some(ManagedStartMarker {
        section_id: id,
        owner_kind,
        title: extract_quoted_attr(attrs, "title").unwrap_or_default(),
        version,
        knowledge_refs: parse_list_attr(extract_quoted_attr(attrs, "knowledge")),
        source_refs: parse_list_attr(extract_quoted_attr(attrs, "source")),
        input_hash: extract_attr(attrs, "input-hash").unwrap_or_default(),
        content_hash: extract_attr(attrs, "content-hash").unwrap_or_default(),
        generated_content_hash: extract_attr(attrs, "generated-content-hash")
            .filter(|value| !value.trim().is_empty()),
        projection_digest_ref: extract_quoted_attr(attrs, "projection")
            .filter(|value| !value.trim().is_empty()),
    }))
}

fn parse_owner_kind(value: &str) -> Option<SectionOwnership> {
    match value {
        "declared_managed" => Some(SectionOwnership::DeclaredManaged),
        "derived_managed" => Some(SectionOwnership::DerivedManaged),
        "projection_static" => Some(SectionOwnership::ProjectionStatic),
        "manual_unmanaged" => Some(SectionOwnership::ManualUnmanaged),
        "external_ref" => Some(SectionOwnership::ExternalRef),
        _ => None,
    }
}

fn parse_list_attr(value: Option<String>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

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

fn extract_attr(attrs: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    for token in attrs.split_whitespace() {
        if let Some(value) = token.strip_prefix(&prefix) {
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

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

pub fn has_managed_markers(content: &str) -> bool {
    content.contains(MARKER_START_PREFIX)
}

pub fn parse_with_markers(content: &str) -> ParsedWikiPage {
    parse_with_markers_and_bindings(content, &SectionBindingIndex::default())
}

pub fn parse_with_markers_and_bindings(
    content: &str,
    binding_index: &SectionBindingIndex,
) -> ParsedWikiPage {
    let lines: Vec<&str> = content.lines().collect();
    let mut title = String::new();
    let mut blocks: Vec<PageBlock> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut diagnostics: Vec<PageParseDiagnostic> = Vec::new();
    let mut user_buf: Vec<&str> = Vec::new();
    let mut last_managed_id: Option<String> = None;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if title.is_empty() {
            if let Some(h1) = line.strip_prefix("# ") {
                title = h1.trim().to_string();
                i += 1;
                continue;
            }
        }

        let marker = match parse_start_marker(line) {
            Ok(Some(marker)) => Some(marker),
            Ok(None) => None,
            Err(diagnostic) => {
                warnings.push(diagnostic.message.clone());
                diagnostics.push(diagnostic);
                i += 1;
                continue;
            }
        };

        if let Some(marker) = marker {
            flush_user_buf(
                &mut user_buf,
                &mut blocks,
                &last_managed_id,
                &Some(marker.section_id.clone()),
            );

            let mut body_lines: Vec<&str> = Vec::new();
            i += 1;
            let mut found_end = false;
            while i < lines.len() {
                if let Some(end_id) = parse_end_marker(lines[i]) {
                    found_end = true;
                    if end_id != marker.section_id {
                        let diagnostic = PageParseDiagnostic::new(
                            PageParseDiagnosticKind::MarkerEndMismatch,
                            Some(marker.section_id.clone()),
                            format!(
                                "managed section '{}' end marker id mismatch: {}",
                                marker.section_id, end_id
                            ),
                        );
                        warnings.push(diagnostic.message.clone());
                        diagnostics.push(diagnostic);
                    }
                    i += 1;
                    break;
                }
                body_lines.push(lines[i]);
                i += 1;
            }
            if !found_end {
                let diagnostic = PageParseDiagnostic::new(
                    PageParseDiagnosticKind::MarkerEndMismatch,
                    Some(marker.section_id.clone()),
                    format!("managed section '{}' 缺少结束 marker", marker.section_id),
                );
                warnings.push(diagnostic.message.clone());
                diagnostics.push(diagnostic);
            }

            let body = strip_leading_heading(&body_lines);
            let computed_content_hash = content_hash(&body);
            if !marker.content_hash.trim().is_empty()
                && marker.content_hash != computed_content_hash
            {
                diagnostics.push(PageParseDiagnostic::new(
                    PageParseDiagnosticKind::HashMismatch,
                    Some(marker.section_id.clone()),
                    format!(
                        "managed section '{}' content hash mismatch",
                        marker.section_id
                    ),
                ));
            }
            if let Some(binding) = binding_index.get(&marker.section_id) {
                if binding.owner_kind != Some(marker.owner_kind) {
                    diagnostics.push(PageParseDiagnostic::new(
                        PageParseDiagnosticKind::MetadataBindingMismatch,
                        Some(marker.section_id.clone()),
                        format!(
                            "managed section '{}' owner 与 metadata binding 不一致",
                            marker.section_id
                        ),
                    ));
                }
            }
            let content_hash = if marker.content_hash.trim().is_empty() {
                computed_content_hash
            } else {
                marker.content_hash
            };

            blocks.push(PageBlock::Managed(ManagedSectionBlock {
                section_id: marker.section_id.clone(),
                owner_kind: marker.owner_kind,
                title: marker.title,
                version: marker.version,
                body,
                knowledge_refs: marker.knowledge_refs,
                source_refs: marker.source_refs,
                input_hash: marker.input_hash,
                content_hash,
                generated_content_hash: marker.generated_content_hash,
                projection_digest_ref: marker.projection_digest_ref,
            }));
            last_managed_id = Some(marker.section_id);
            continue;
        }

        user_buf.push(line);
        i += 1;
    }

    flush_user_buf(&mut user_buf, &mut blocks, &last_managed_id, &None);

    ParsedWikiPage {
        title,
        blocks,
        parse_mode: PageParseMode::ManagedMarkers,
        warnings,
        diagnostics,
    }
}

pub fn parse_wiki_page(content: &str, binding_index: &SectionBindingIndex) -> ParsedWikiPage {
    parse_wiki_page_with_bindings(content, binding_index)
}

pub fn parse_wiki_page_with_bindings(
    content: &str,
    binding_index: &SectionBindingIndex,
) -> ParsedWikiPage {
    if has_managed_markers(content) {
        parse_with_markers_and_bindings(content, binding_index)
    } else {
        let title = content
            .lines()
            .find_map(|line| line.strip_prefix("# ").map(|h1| h1.trim().to_string()))
            .unwrap_or_default();
        let diagnostic = PageParseDiagnostic::new(
            PageParseDiagnosticKind::MarkerMissing,
            None,
            "页面缺少 managed marker",
        );
        ParsedWikiPage {
            title,
            blocks: Vec::new(),
            parse_mode: PageParseMode::UnmanagedOnly,
            warnings: vec![diagnostic.message.clone()],
            diagnostics: vec![diagnostic],
        }
    }
}

// ---------------------------------------------------------------------------
// Drift 分类与 Merge
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionDriftClassification {
    pub kind: SyncResultKind,
    pub reasons: Vec<String>,
}

pub fn classify_section_drift(
    parsed: &ManagedSectionBlock,
    binding: &SectionBinding,
) -> SectionDriftClassification {
    if binding.owner_kind != Some(parsed.owner_kind) {
        return SectionDriftClassification {
            kind: SyncResultKind::Conflict,
            reasons: vec![format!("section '{}' owner mismatch", parsed.section_id)],
        };
    }
    if !binding.input_hash.is_empty() && parsed.input_hash != binding.input_hash {
        return SectionDriftClassification {
            kind: SyncResultKind::Stale,
            reasons: vec![format!("section '{}' input hash stale", parsed.section_id)],
        };
    }
    let current_hash = content_hash(&parsed.body);
    match parsed.owner_kind {
        SectionOwnership::DeclaredManaged if current_hash != binding.content_hash => {
            SectionDriftClassification {
                kind: SyncResultKind::DeclaredWriteback,
                reasons: Vec::new(),
            }
        }
        SectionOwnership::ManualUnmanaged => SectionDriftClassification {
            kind: SyncResultKind::MetadataOnly,
            reasons: Vec::new(),
        },
        SectionOwnership::DerivedManaged | SectionOwnership::ProjectionStatic
            if current_hash != binding.content_hash =>
        {
            SectionDriftClassification {
                kind: SyncResultKind::IllegalDrift,
                reasons: vec![format!(
                    "section '{}' illegal generated drift",
                    parsed.section_id
                )],
            }
        }
        _ => SectionDriftClassification {
            kind: SyncResultKind::MetadataOnly,
            reasons: Vec::new(),
        },
    }
}

pub fn merge_sections(
    new_managed: &[ManagedSectionBlock],
    old_parsed: &ParsedWikiPage,
) -> PageMergePlan {
    let mut result_blocks: Vec<PageBlock> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let user_sections: Vec<&UserSectionBlock> = old_parsed
        .blocks
        .iter()
        .filter_map(|b| match b {
            PageBlock::User(u) => Some(u),
            PageBlock::Managed(_) => None,
        })
        .collect();
    let new_managed_ids: Vec<&str> = new_managed.iter().map(|m| m.section_id.as_str()).collect();

    for managed in new_managed {
        result_blocks.push(PageBlock::Managed(managed.clone()));
        for user in &user_sections {
            if user.anchor_after_section_id.as_deref() == Some(&managed.section_id) {
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

    for user in &user_sections {
        let already_inserted = result_blocks.iter().any(|b| match b {
            PageBlock::User(u) => u.id == user.id,
            PageBlock::Managed(_) => false,
        });
        if already_inserted {
            continue;
        }
        if let Some(before_id) = &user.anchor_before_section_id {
            if let Some(pos) = result_blocks.iter().position(|b| match b {
                PageBlock::Managed(m) => m.section_id == *before_id,
                PageBlock::User(_) => false,
            }) {
                result_blocks.insert(pos, PageBlock::User((*user).clone()));
                continue;
            }
        }
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

pub fn content_hash(body: &str) -> String {
    fingerprint_bytes(body.as_bytes())
}

// ---------------------------------------------------------------------------
// 内部辅助
// ---------------------------------------------------------------------------

fn strip_leading_heading(lines: &[&str]) -> String {
    let mut start = 0;
    while start < lines.len() && lines[start].trim().is_empty() {
        start += 1;
    }
    if start < lines.len() && lines[start].starts_with("## ") {
        start += 1;
    }
    while start < lines.len() && lines[start].trim().is_empty() {
        start += 1;
    }
    let mut end = lines.len();
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    lines[start..end].join("\n")
}

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

#[cfg(test)]
mod tests {
    use super::{parse_with_markers, render_managed_block};

    #[test]
    fn render_managed_block_omits_heading_for_empty_title() {
        let rendered = render_managed_block("section:preamble", "", "<cite>\nbody\n</cite>");

        assert!(!rendered.contains("\n## \n"));
        assert!(rendered.contains("<cite>"));
    }

    #[test]
    fn parse_with_markers_preserves_empty_title_preamble() {
        let page = "# 示例页\n\n<!-- wiki:managed:start id=section:preamble owner=derived_managed title=\"\" version=2 knowledge=\"\" source=\"\" input-hash= content-hash= generated-content-hash= projection=\"\" -->\n<cite>\nbody\n</cite>\n<!-- wiki:managed:end id=section:preamble -->";
        let parsed = parse_with_markers(page);

        assert_eq!(parsed.blocks.len(), 1);
        let block = match &parsed.blocks[0] {
            super::PageBlock::Managed(block) => block,
            super::PageBlock::User(_) => panic!("expected managed block"),
        };
        assert!(block.title.is_empty());
        assert!(block.body.contains("<cite>"));
    }
}
