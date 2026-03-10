//! managed section parse / merge 内核的单元测试。
//! 覆盖 marker 解析、legacy heading 迁移、user section 锚点恢复和 managed drift 检测。

use wiki_core::generation::managed_sections::*;
use wiki_core::generation::sections::section_titles_for_page_type;

// ---------------------------------------------------------------------------
// Marker 解析
// ---------------------------------------------------------------------------

#[test]
fn parse_page_with_managed_markers() {
    let content = r#"# 项目概述

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

由 codebuddy-wiki 自动生成的仓库概览。
<!-- wiki:managed:end id=section-aaa -->

<!-- wiki:managed:start id=section-bbb title="项目事实" version=1 -->
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

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

自动生成内容。
<!-- wiki:managed:end id=section-aaa -->

## 手工笔记

这是用户手写的内容。

<!-- wiki:managed:start id=section-bbb title="项目事实" version=1 -->
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

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
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

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

没有结束 marker 的内容。"#;

    let parsed = parse_with_markers(content);
    assert_eq!(parsed.blocks.len(), 1);
    assert!(!parsed.warnings.is_empty());
    assert!(parsed.warnings[0].contains("缺少结束 marker"));
}

// ---------------------------------------------------------------------------
// Legacy heading 迁移
// ---------------------------------------------------------------------------

#[test]
fn parse_legacy_overview_page() {
    let content = r#"# 项目概述

## 简介

这是简介内容。

## 项目事实

- 事实 1
- 事实 2

## 关键信息

关键信息内容。"#;

    let known = section_titles_for_page_type("overview");
    let known_ref: Vec<&str> = known.iter().copied().collect();
    let parsed = parse_with_legacy_headings(content, &known_ref);

    assert_eq!(parsed.title, "项目概述");
    assert_eq!(parsed.parse_mode, PageParseMode::LegacyHeadings);
    assert_eq!(parsed.blocks.len(), 3);

    // 所有已知标题都应该被识别为 managed
    for block in &parsed.blocks {
        match block {
            PageBlock::Managed(m) => {
                assert!(known_ref.contains(&m.title.as_str()));
            }
            _ => panic!("expected all blocks to be managed for known titles"),
        }
    }
}

#[test]
fn parse_legacy_page_with_user_section() {
    let content = r#"# 项目概述

## 简介

简介内容。

## 手工笔记

用户手写的内容。

## 项目事实

- 事实 1

## 关键信息

关键信息。"#;

    let known = section_titles_for_page_type("overview");
    let known_ref: Vec<&str> = known.iter().copied().collect();
    let parsed = parse_with_legacy_headings(content, &known_ref);

    assert_eq!(parsed.blocks.len(), 4);

    // 第二个应该是 user section（手工笔记）
    match &parsed.blocks[1] {
        PageBlock::User(u) => {
            assert!(u.body.contains("手工笔记"));
        }
        _ => panic!("expected user block for unknown heading"),
    }
}

#[test]
fn parse_legacy_page_no_known_titles_warns() {
    let content = r#"# 随便

## 未知标题

一些内容。"#;

    let known = section_titles_for_page_type("overview");
    let known_ref: Vec<&str> = known.iter().copied().collect();
    let parsed = parse_with_legacy_headings(content, &known_ref);

    // 没有匹配到任何 managed section，应该有 warning
    let has_managed = parsed
        .blocks
        .iter()
        .any(|b| matches!(b, PageBlock::Managed(_)));
    if !has_managed {
        assert!(!parsed.warnings.is_empty());
    }
}

// ---------------------------------------------------------------------------
// 统一解析入口
// ---------------------------------------------------------------------------

#[test]
fn parse_wiki_page_auto_detects_markers() {
    let with_markers = r#"# 测试

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

内容。
<!-- wiki:managed:end id=section-aaa -->"#;

    let parsed = parse_wiki_page(with_markers, &["简介"]);
    assert_eq!(parsed.parse_mode, PageParseMode::ManagedMarkers);
}

#[test]
fn parse_wiki_page_auto_detects_legacy() {
    let without_markers = r#"# 测试

## 简介

内容。"#;

    let parsed = parse_wiki_page(without_markers, &["简介"]);
    assert_eq!(parsed.parse_mode, PageParseMode::LegacyHeadings);
}

// ---------------------------------------------------------------------------
// Merge
// ---------------------------------------------------------------------------

#[test]
fn merge_preserves_user_section_between_managed() {
    // 旧页面有 user section 在两个 managed 之间
    let old_content = r#"# 测试

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

旧简介。
<!-- wiki:managed:end id=section-aaa -->

## 手工笔记

用户内容。

<!-- wiki:managed:start id=section-bbb title="事实" version=1 -->
## 事实

旧事实。
<!-- wiki:managed:end id=section-bbb -->"#;

    let old_parsed = parse_with_markers(old_content);

    let new_managed = vec![
        ManagedSectionBlock {
            section_id: "section-aaa".to_string(),
            title: "简介".to_string(),
            version: 1,
            body: "新简介内容。".to_string(),
        },
        ManagedSectionBlock {
            section_id: "section-bbb".to_string(),
            title: "事实".to_string(),
            version: 1,
            body: "新事实内容。".to_string(),
        },
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

<!-- wiki:managed:start id=section-ccc title="旧区段" version=1 -->
## 旧区段

旧内容。
<!-- wiki:managed:end id=section-ccc -->

## 用户笔记

用户内容。"#;

    let old_parsed = parse_with_markers(old_content);

    // 新 managed 完全不同
    let new_managed = vec![ManagedSectionBlock {
        section_id: "section-ddd".to_string(),
        title: "新区段".to_string(),
        version: 1,
        body: "新内容。".to_string(),
    }];

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

<!-- wiki:managed:start id=section-aaa title="简介" version=1 -->
## 简介

旧内容。
<!-- wiki:managed:end id=section-aaa -->"#;

    let old_parsed = parse_with_markers(old_content);

    let new_managed = vec![ManagedSectionBlock {
        section_id: "section-aaa".to_string(),
        title: "简介".to_string(),
        version: 1,
        body: "新内容。".to_string(),
    }];

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
        PageBlock::Managed(ManagedSectionBlock {
            section_id: "section-aaa".to_string(),
            title: "简介".to_string(),
            version: 1,
            body: "内容 A。".to_string(),
        }),
        PageBlock::User(UserSectionBlock {
            id: "user-001".to_string(),
            body: "## 手工笔记\n\n用户内容。".to_string(),
            anchor_after_section_id: Some("section-aaa".to_string()),
            anchor_before_section_id: Some("section-bbb".to_string()),
        }),
        PageBlock::Managed(ManagedSectionBlock {
            section_id: "section-bbb".to_string(),
            title: "事实".to_string(),
            version: 1,
            body: "内容 B。".to_string(),
        }),
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
