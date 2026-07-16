//! managed section parse / merge 内核的单元测试。
//! 覆盖 marker 解析、typed diagnostics、user section 锚点恢复和 managed drift 检测。

use wiki_model::domain::projection::{
    ProjectionDigestStatus, SectionBinding, SectionOwnership, SyncResultKind,
};
use wiki_runtime::generation::managed_sections::*;

fn managed(section_id: &str, title: &str, body: &str) -> ManagedSectionBlock {
    ManagedSectionBlock::generated(section_id.to_string(), title.to_string(), body.to_string())
}

fn binding_for(block: &ManagedSectionBlock) -> SectionBinding {
    SectionBinding {
        section_id: block.section_id.clone(),
        owner_kind: Some(block.owner_kind),
        knowledge_refs: block.knowledge_refs.clone(),
        source_refs: block.source_refs.clone(),
        input_hash: block.input_hash.clone(),
        content_hash: block.content_hash.clone(),
        projection_status: ProjectionDigestStatus::Ready,
        projection_digest_ref: block.projection_digest_ref.clone(),
    }
}

// ---------------------------------------------------------------------------
// Marker 解析
// ---------------------------------------------------------------------------

#[test]
fn parse_marker_without_owner_reports_typed_diagnostic() {
    let content = r#"# Page

<!-- wiki:managed:start id=section:intro title="Intro" version=2 -->
## Intro

body
<!-- wiki:managed:end id=section:intro -->"#;

    let parsed = parse_wiki_page(content, &SectionBindingIndex::default());

    assert!(parsed.diagnostics.iter().any(|diagnostic| {
        matches!(diagnostic.kind, PageParseDiagnosticKind::MarkerMissingOwner)
    }));
    assert!(parsed.managed_blocks().is_empty());
}

#[test]
fn render_parse_roundtrip_preserves_section_binding() {
    let body_hash = content_hash("generated body");
    let block = ManagedSectionBlock {
        section_id: "section:intro".to_string(),
        owner_kind: SectionOwnership::DerivedManaged,
        title: "简介".to_string(),
        version: MARKER_VERSION,
        body: "generated body".to_string(),
        knowledge_refs: vec!["unit:repo".to_string()],
        source_refs: vec!["src/lib.rs".to_string()],
        input_hash: "input-hash".to_string(),
        content_hash: body_hash.clone(),
        generated_content_hash: Some(body_hash),
        projection_digest_ref: Some("projection:repo".to_string()),
    };

    let rendered = render_page_with_markers("Repo", &[PageBlock::Managed(block.clone())]);
    let parsed = parse_wiki_page(
        &rendered,
        &SectionBindingIndex::from_blocks(std::slice::from_ref(&block)),
    );
    let roundtripped = parsed.managed_blocks();

    assert!(parsed.diagnostics.is_empty());
    assert_eq!(roundtripped.len(), 1);
    assert_eq!(roundtripped[0].owner_kind, block.owner_kind);
    assert_eq!(
        roundtripped[0].projection_digest_ref,
        block.projection_digest_ref
    );
    assert_eq!(roundtripped[0].knowledge_refs, block.knowledge_refs);
    assert_eq!(roundtripped[0].source_refs, block.source_refs);
    assert_eq!(roundtripped[0].input_hash, block.input_hash);
    assert_eq!(roundtripped[0].content_hash, block.content_hash);
    assert_eq!(
        roundtripped[0].generated_content_hash,
        block.generated_content_hash
    );
}

#[test]
fn classify_section_drift_returns_all_contract_kinds() {
    let declared = ManagedSectionBlock {
        owner_kind: SectionOwnership::DeclaredManaged,
        body: "edited declared body".to_string(),
        content_hash: content_hash("baseline declared body"),
        source_refs: vec!["manual".to_string()],
        ..managed("section:declared", "Declared", "baseline declared body")
    };
    assert_eq!(
        classify_section_drift(&declared, &binding_for(&declared)).kind,
        SyncResultKind::DeclaredWriteback
    );

    let manual = ManagedSectionBlock {
        owner_kind: SectionOwnership::ManualUnmanaged,
        body: "manual edit".to_string(),
        content_hash: content_hash("manual baseline"),
        source_refs: vec!["manual".to_string()],
        ..managed("section:manual", "Manual", "manual baseline")
    };
    assert_eq!(
        classify_section_drift(&manual, &binding_for(&manual)).kind,
        SyncResultKind::MetadataOnly
    );

    let derived = ManagedSectionBlock {
        body: "edited derived body".to_string(),
        content_hash: content_hash("generated derived body"),
        source_refs: vec!["src/lib.rs".to_string()],
        ..managed("section:derived", "Derived", "generated derived body")
    };
    assert_eq!(
        classify_section_drift(&derived, &binding_for(&derived)).kind,
        SyncResultKind::IllegalDrift
    );

    let owner_mismatch = managed("section:owner", "Owner", "body");
    let mut mismatched_binding = binding_for(&owner_mismatch);
    mismatched_binding.owner_kind = Some(SectionOwnership::DeclaredManaged);
    assert_eq!(
        classify_section_drift(&owner_mismatch, &mismatched_binding).kind,
        SyncResultKind::Conflict
    );

    let stale = ManagedSectionBlock {
        input_hash: "current-input".to_string(),
        source_refs: vec!["src/lib.rs".to_string()],
        ..managed("section:stale", "Stale", "body")
    };
    let mut stale_binding = binding_for(&stale);
    stale_binding.input_hash = "previous-input".to_string();
    assert_eq!(
        classify_section_drift(&stale, &stale_binding).kind,
        SyncResultKind::Stale
    );
}

#[test]
fn parse_page_with_managed_markers() {
    let content = r#"# 项目概述

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

由 spec-wiki 自动生成的仓库概览。
<!-- wiki:managed:end id=section-aaa -->

<!-- wiki:managed:start id=section-bbb owner=derived_managed title="项目事实" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 项目事实

- Rust 项目
<!-- wiki:managed:end id=section-bbb -->"#;

    let parsed = parse_with_markers(content);
    assert_eq!(parsed.title, "项目概述");
    assert_eq!(parsed.parse_mode, PageParseMode::ManagedMarkers);
    assert_eq!(parsed.blocks.len(), 2);
    assert!(parsed.warnings.is_empty());

    match &parsed.blocks[0] {
        PageBlock::Managed(m) => {
            assert_eq!(m.section_id, "section-aaa");
            assert_eq!(m.title, "简介");
            assert!(m.body.contains("自动生成"));
        }
        _ => panic!("expected managed block"),
    }

    match &parsed.blocks[1] {
        PageBlock::Managed(m) => {
            assert_eq!(m.section_id, "section-bbb");
            assert_eq!(m.title, "项目事实");
        }
        _ => panic!("expected managed block"),
    }
}

#[test]
fn parse_page_with_user_section_between_managed() {
    let content = r#"# 项目概述

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

自动生成内容。
<!-- wiki:managed:end id=section-aaa -->

## 手工笔记

这是用户手写的内容。

<!-- wiki:managed:start id=section-bbb owner=derived_managed title="项目事实" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 项目事实

- 事实 1
<!-- wiki:managed:end id=section-bbb -->"#;

    let parsed = parse_with_markers(content);
    assert_eq!(parsed.blocks.len(), 3);

    match &parsed.blocks[0] {
        PageBlock::Managed(m) => assert_eq!(m.section_id, "section-aaa"),
        _ => panic!("expected managed block"),
    }

    match &parsed.blocks[1] {
        PageBlock::User(u) => {
            assert!(u.body.contains("手工笔记"));
            assert!(u.body.contains("用户手写"));
            assert_eq!(u.anchor_after_section_id.as_deref(), Some("section-aaa"));
            assert_eq!(u.anchor_before_section_id.as_deref(), Some("section-bbb"));
        }
        _ => panic!("expected user block"),
    }

    match &parsed.blocks[2] {
        PageBlock::Managed(m) => assert_eq!(m.section_id, "section-bbb"),
        _ => panic!("expected managed block"),
    }
}

#[test]
fn parse_page_with_trailing_user_section() {
    let content = r#"# 测试

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

内容。
<!-- wiki:managed:end id=section-aaa -->

## 用户追加

追加在末尾的内容。"#;

    let parsed = parse_with_markers(content);
    assert_eq!(parsed.blocks.len(), 2);

    match &parsed.blocks[1] {
        PageBlock::User(u) => {
            assert!(u.body.contains("用户追加"));
            assert_eq!(u.anchor_after_section_id.as_deref(), Some("section-aaa"));
            assert_eq!(u.anchor_before_section_id, None);
        }
        _ => panic!("expected user block"),
    }
}

#[test]
fn parse_page_missing_end_marker_warns() {
    let content = r#"# 测试

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

没有结束 marker 的内容。"#;

    let parsed = parse_with_markers(content);
    assert_eq!(parsed.blocks.len(), 1);
    assert!(!parsed.warnings.is_empty());
    assert!(parsed.warnings[0].contains("缺少结束 marker"));
}

// ---------------------------------------------------------------------------
// 统一解析入口
// ---------------------------------------------------------------------------

#[test]
fn parse_wiki_page_auto_detects_markers() {
    let with_markers = r#"# 测试

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

内容。
<!-- wiki:managed:end id=section-aaa -->"#;

    let parsed = parse_wiki_page(with_markers, &SectionBindingIndex::default());
    assert_eq!(parsed.parse_mode, PageParseMode::ManagedMarkers);
}

#[test]
fn parse_wiki_page_without_markers_is_unmanaged_only() {
    let without_markers = r#"# 测试

## 简介

内容。"#;

    let parsed = parse_wiki_page(without_markers, &SectionBindingIndex::default());
    assert_eq!(parsed.parse_mode, PageParseMode::UnmanagedOnly);
    assert!(parsed.blocks.is_empty());
    assert!(parsed
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.kind == PageParseDiagnosticKind::MarkerMissing));
}

// ---------------------------------------------------------------------------
// Merge
// ---------------------------------------------------------------------------

#[test]
fn merge_preserves_user_section_between_managed() {
    // 旧页面有 user section 在两个 managed 之间
    let old_content = r#"# 测试

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

旧简介。
<!-- wiki:managed:end id=section-aaa -->

## 手工笔记

用户内容。

<!-- wiki:managed:start id=section-bbb owner=derived_managed title="事实" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 事实

旧事实。
<!-- wiki:managed:end id=section-bbb -->"#;

    let old_parsed = parse_with_markers(old_content);

    let new_managed = vec![
        managed("section-aaa", "简介", "新简介内容。"),
        managed("section-bbb", "事实", "新事实内容。"),
    ];

    let plan = merge_sections(&new_managed, &old_parsed);
    assert_eq!(plan.blocks.len(), 3);
    assert!(plan.warnings.is_empty());

    // 验证 user section 被保留在中间
    match &plan.blocks[1] {
        PageBlock::User(u) => {
            assert!(u.body.contains("手工笔记"));
        }
        _ => panic!("expected user block preserved in middle"),
    }

    // 验证 managed sections 是新内容
    match &plan.blocks[0] {
        PageBlock::Managed(m) => assert!(m.body.contains("新简介")),
        _ => panic!("expected managed block"),
    }
    match &plan.blocks[2] {
        PageBlock::Managed(m) => assert!(m.body.contains("新事实")),
        _ => panic!("expected managed block"),
    }
}

#[test]
fn merge_appends_orphan_user_section_with_warning() {
    // 旧页面有 user section 锚定在 section-ccc 之后，但新页面没有 section-ccc
    let old_content = r#"# 测试

<!-- wiki:managed:start id=section-ccc owner=derived_managed title="旧区段" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 旧区段

旧内容。
<!-- wiki:managed:end id=section-ccc -->

## 用户笔记

用户内容。"#;

    let old_parsed = parse_with_markers(old_content);

    // 新 managed 完全不同
    let new_managed = vec![managed("section-ddd", "新区段", "新内容。")];

    let plan = merge_sections(&new_managed, &old_parsed);

    // user section 应该被追加到末尾
    let user_count = plan
        .blocks
        .iter()
        .filter(|b| matches!(b, PageBlock::User(_)))
        .count();
    assert_eq!(user_count, 1);

    // 应该有锚点丢失的 warning
    assert!(!plan.warnings.is_empty());
}

#[test]
fn merge_no_user_sections_returns_only_managed() {
    let old_content = r#"# 测试

<!-- wiki:managed:start id=section-aaa owner=derived_managed title="简介" version=2 knowledge="" source="" input-hash= content-hash= generated-content-hash= projection="" -->
## 简介

旧内容。
<!-- wiki:managed:end id=section-aaa -->"#;

    let old_parsed = parse_with_markers(old_content);

    let new_managed = vec![managed("section-aaa", "简介", "新内容。")];

    let plan = merge_sections(&new_managed, &old_parsed);
    assert_eq!(plan.blocks.len(), 1);
    assert!(plan.warnings.is_empty());
}

// ---------------------------------------------------------------------------
// Marker 渲染
// ---------------------------------------------------------------------------

#[test]
fn render_managed_block_produces_valid_markers() {
    let output = render_managed_block("section-abc", "简介", "自动生成内容。");
    assert!(output.contains("<!-- wiki:managed:start id=section-abc"));
    assert!(output.contains("<!-- wiki:managed:end id=section-abc -->"));
    assert!(output.contains("## 简介"));
    assert!(output.contains("自动生成内容。"));
}

#[test]
fn render_page_with_markers_roundtrips() {
    let blocks = vec![
        PageBlock::Managed(managed("section-aaa", "简介", "内容 A。")),
        PageBlock::User(UserSectionBlock {
            id: "user-001".to_string(),
            body: "## 手工笔记\n\n用户内容。".to_string(),
            anchor_after_section_id: Some("section-aaa".to_string()),
            anchor_before_section_id: Some("section-bbb".to_string()),
        }),
        PageBlock::Managed(managed("section-bbb", "事实", "内容 B。")),
    ];

    let rendered = render_page_with_markers("测试页面", &blocks);

    // 重新解析应该得到相同结构
    let parsed = parse_with_markers(&rendered);
    assert_eq!(parsed.title, "测试页面");
    assert_eq!(parsed.blocks.len(), 3);
    assert!(parsed.warnings.is_empty());
}

// ---------------------------------------------------------------------------
// content_hash
// ---------------------------------------------------------------------------

#[test]
fn content_hash_is_stable() {
    let h1 = content_hash("hello world");
    let h2 = content_hash("hello world");
    assert_eq!(h1, h2);

    let h3 = content_hash("different");
    assert_ne!(h1, h3);
}
