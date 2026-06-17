//! 渲染层负责把页面上下文和 section 草稿组装成最终 Markdown。
//! 它只关心表达形式，不参与模块树、变化规划和状态判断。

use crate::domain::context::PageComposePlan;
use crate::domain::context::PageContext;
use crate::domain::stable_id::stable_id;
use crate::generation::managed_sections::{
    render_page_with_markers, ManagedSectionBlock, PageBlock, PageMergePlan,
};
use crate::generation::sections::{
    build_page_compose_plan, build_section_drafts_from_compose_plan, SectionDraft,
};
use wiki_knowledge::domain::compose::{ComposeSectionDraft, DiagramDraft, PageDraft};
use wiki_knowledge::PagePlan;

/// `RenderedPage` 是页面渲染层的标准输出。
/// 它同时返回 section 草稿和最终 Markdown，供缓存和状态层复用。
#[derive(Debug, Clone)]
pub struct RenderedPage {
    /// 当前页面渲染出的稳定 section 草稿集合。
    pub sections: Vec<SectionDraft>,
    /// 当前页面最终组装完成的 Markdown 文本。
    pub content: String,
}

/// 把 `PageContext` 渲染成最终 Markdown 文本。
/// 渲染层只做页面表达，不再回头参与结构分析。
///
/// # 参数
/// - `page`：当前页面计划。
/// - `context`：当前页面对应的渲染上下文。
///
/// # 返回
/// - 返回可直接写入 `.wiki/*.md` 的 Markdown 文本。
pub fn render_page(page: &PagePlan, context: &PageContext) -> String {
    render_page_bundle(page, context).content
}

/// 生成页面级 section 草稿并组装成带 managed marker 的最终 Markdown。
///
/// # 参数
/// - `page`：当前页面计划。
/// - `context`：当前页面渲染上下文。
///
/// # 返回
/// - 返回同时包含 section 草稿和整页 Markdown 的渲染结果。
pub fn render_page_bundle(page: &PagePlan, context: &PageContext) -> RenderedPage {
    let compose_plan = build_page_compose_plan(page, context);
    render_page_bundle_with_compose(page, context, &compose_plan)
}

/// 基于显式 compose plan 组装页面。
pub fn render_page_bundle_with_compose(
    page: &PagePlan,
    context: &PageContext,
    compose_plan: &PageComposePlan,
) -> RenderedPage {
    let sections = build_section_drafts_from_compose_plan(page, context, compose_plan);
    let content = assemble_page(page, &sections);
    RenderedPage { sections, content }
}

/// 构建 compose plan 并直接渲染页面。
pub fn render_page_bundle_via_compose(
    page: &PagePlan,
    context: &PageContext,
) -> (PageComposePlan, RenderedPage) {
    let compose_plan = build_page_compose_plan(page, context);
    let rendered_page = render_page_bundle_with_compose(page, context, &compose_plan);
    (compose_plan, rendered_page)
}

/// 把 section 草稿转换成 managed blocks，用于 merge 和渲染。
pub fn drafts_to_managed_blocks(sections: &[SectionDraft]) -> Vec<ManagedSectionBlock> {
    sections
        .iter()
        .map(|s| {
            ManagedSectionBlock::generated(
                s.section_id.clone(),
                s.title.clone(),
                s.content.clone(),
            )
        })
        .collect()
}

/// 按稳定 section 顺序把草稿组装成带 managed marker 的整页 Markdown。
///
/// # 参数
/// - `page`：当前页面计划。
/// - `sections`：已经生成好的 section 草稿集合。
///
/// # 返回
/// - 返回可直接写入页面文件的整页 Markdown（含 managed marker）。
pub fn assemble_page(page: &PagePlan, sections: &[SectionDraft]) -> String {
    let blocks: Vec<PageBlock> = sections
        .iter()
        .map(|s| {
            PageBlock::Managed(ManagedSectionBlock::generated(
                s.section_id.clone(),
                s.title.clone(),
                s.content.clone(),
            ))
        })
        .collect();

    render_page_with_markers(&page.title, &blocks)
}

/// 将 compose 层的 `PageDraft` 转换为 `RenderedPage`。
/// 直接从 compose 产物生成最终 Markdown。
pub fn render_page_draft(draft: &PageDraft) -> RenderedPage {
    let page_citations = collect_page_citations(draft);
    let diagrams = resolve_diagrams_for_render(draft);
    let mut sections: Vec<SectionDraft> = draft
        .sections
        .iter()
        .map(|cs| {
            let citation_source_ids: Vec<String> = cs
                .citations
                .iter()
                .filter_map(|c| c.source_id.clone())
                .collect();
            SectionDraft {
                section_id: stable_id("section", format!("{}:{}", draft.page_id, cs.section_key)),
                title: cs.title.clone(),
                managed: cs.managed,
                source_ids: citation_source_ids,
                relation_ids: Vec::new(),
                content: render_compose_section_content(cs, &draft.sections),
            }
        })
        .collect();
    prepend_page_cite_block_if_missing(draft, &page_citations, &mut sections);
    append_generated_appendix_if_missing(draft, &page_citations, &mut sections);
    if !diagrams.is_empty() {
        sections.push(render_diagram_section(
            &draft.page_id,
            &diagrams,
            &page_citations,
        ));
    }
    insert_generated_toc_if_missing(draft, &mut sections);
    let content = assemble_page_from_sections(&draft.title, &sections);
    RenderedPage { sections, content }
}

fn prepend_page_cite_block_if_missing(
    draft: &PageDraft,
    page_citations: &[wiki_knowledge::domain::research::SourceCitation],
    sections: &mut Vec<SectionDraft>,
) {
    let has_preamble_cite = draft.sections.iter().any(|section| {
        section.title.trim().is_empty()
            && (!section.citations.is_empty() || preserves_reference_markdown(&section.content))
    });
    if has_preamble_cite {
        return;
    }

    if page_citations.is_empty() {
        return;
    }

    sections.insert(
        0,
        SectionDraft {
            section_id: stable_id("section", format!("{}:page-citations", draft.page_id)),
            title: String::new(),
            managed: true,
            source_ids: page_citations
                .iter()
                .filter_map(|citation| citation.source_id.clone())
                .collect(),
            relation_ids: Vec::new(),
            content: render_cite_block(page_citations),
        },
    );
}

fn append_generated_appendix_if_missing(
    draft: &PageDraft,
    page_citations: &[wiki_knowledge::domain::research::SourceCitation],
    sections: &mut Vec<SectionDraft>,
) {
    let has_appendix = draft
        .sections
        .iter()
        .any(|section| section.title.trim() == "附录");
    if has_appendix || page_citations.is_empty() {
        return;
    }

    sections.push(SectionDraft {
        section_id: stable_id("section", format!("{}:appendix", draft.page_id)),
        title: "附录".to_string(),
        managed: true,
        source_ids: page_citations
            .iter()
            .filter_map(|citation| citation.source_id.clone())
            .collect(),
        relation_ids: Vec::new(),
        content: render_appendix_entries_block(page_citations),
    });
}

fn insert_generated_toc_if_missing(draft: &PageDraft, sections: &mut Vec<SectionDraft>) {
    let has_toc = draft
        .sections
        .iter()
        .any(|section| section.title.trim() == "目录")
        || sections
            .iter()
            .any(|section| section.title.trim() == "目录");
    if has_toc {
        return;
    }

    let visible_titles = sections
        .iter()
        .filter(|section| {
            let title = section.title.trim();
            !title.is_empty() && title != "目录"
        })
        .count();
    if visible_titles < 3 {
        return;
    }

    let content = render_outline_toc_from_rendered_sections(sections);
    if content.is_empty() {
        return;
    }

    let insert_at = usize::from(matches!(
        sections.first(),
        Some(first) if first.title.trim().is_empty()
    ));
    sections.insert(
        insert_at,
        SectionDraft {
            section_id: stable_id("section", format!("{}:generated-toc", draft.page_id)),
            title: "目录".to_string(),
            managed: true,
            source_ids: Vec::new(),
            relation_ids: Vec::new(),
            content,
        },
    );
}

fn collect_page_citations(
    draft: &PageDraft,
) -> Vec<wiki_knowledge::domain::research::SourceCitation> {
    dedup_citation_paths(
        &draft
            .sections
            .iter()
            .flat_map(|section| section.citations.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

fn resolve_diagrams_for_render(draft: &PageDraft) -> Vec<DiagramDraft> {
    if !draft.diagrams.is_empty() {
        return draft.diagrams.clone();
    }

    build_outline_diagram_draft(draft).into_iter().collect()
}

fn build_outline_diagram_draft(draft: &PageDraft) -> Option<DiagramDraft> {
    let headings = draft
        .sections
        .iter()
        .filter_map(|section| {
            let title = section.title.trim();
            (!title.is_empty() && title != "目录" && title != "附录").then_some(title)
        })
        .take(6)
        .collect::<Vec<_>>();
    if headings.len() < 3 {
        return None;
    }

    let mut lines = vec!["flowchart LR".to_string()];
    for (index, heading) in headings.iter().enumerate() {
        lines.push(format!(
            "    s{index}[\"{}\"]",
            heading.replace('"', "\\\"")
        ));
        if index > 0 {
            lines.push(format!("    s{} --> s{index}", index - 1));
        }
    }

    Some(DiagramDraft {
        diagram_id: stable_id("diagram", format!("{}:outline", draft.page_id)),
        diagram_type: "outline".to_string(),
        title: "章节结构图".to_string(),
        description: "展示本页主要章节的阅读顺序。".to_string(),
        content: lines.join("\n"),
    })
}

fn render_compose_section_content(
    section: &ComposeSectionDraft,
    all_sections: &[ComposeSectionDraft],
) -> String {
    if section.title.trim().is_empty()
        && section.content.trim().is_empty()
        && !section.citations.is_empty()
    {
        return render_cite_block(&section.citations);
    }
    if section.title == "目录" && section.content.trim().is_empty() {
        return render_outline_toc(all_sections);
    }

    let mut parts = Vec::new();
    if !section.content.trim().is_empty() {
        parts.push(section.content.trim().to_string());
    }

    if !section.preserve_source_markdown && section.title.trim() == "附录" {
        let appendix_citations = if section.citations.is_empty() {
            collect_all_section_citations(all_sections)
        } else {
            dedup_citations(&section.citations)
        };
        let appendix_block = render_appendix_entries_block(&appendix_citations);
        if !appendix_block.is_empty() {
            parts.push(appendix_block);
        }
    }

    let evidence_block =
        if section.preserve_source_markdown || preserves_reference_markdown(&section.content) {
            String::new()
        } else {
            render_citation_evidence_block(&section.citations)
        };
    if !evidence_block.is_empty() {
        parts.push(evidence_block);
    }
    let section_sources_block =
        if section.preserve_source_markdown || preserves_reference_markdown(&section.content) {
            String::new()
        } else {
            render_section_sources_block(&section.citations)
        };
    if !section_sources_block.is_empty() {
        parts.push(section_sources_block);
    }

    parts.join("\n\n")
}

fn render_cite_block(citations: &[wiki_knowledge::domain::research::SourceCitation]) -> String {
    let citations = dedup_citation_paths(citations);
    let mut lines = vec!["<cite>".to_string(), "**本文引用的文件**".to_string()];
    for citation in citations {
        lines.push(format!("- [{}](file://{})", citation.path, citation.path));
    }
    lines.push("</cite>".to_string());
    lines.join("\n")
}

fn dedup_citation_paths(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> Vec<wiki_knowledge::domain::research::SourceCitation> {
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for citation in citations {
        if seen.insert(citation.path.clone()) {
            deduped.push(citation.clone());
        }
    }
    deduped
}

fn render_outline_toc(all_sections: &[ComposeSectionDraft]) -> String {
    let headings = all_sections
        .iter()
        .filter_map(|section| {
            let title = section.title.trim();
            (!title.is_empty() && title != "目录" && title != "结构图").then_some(title)
        })
        .collect::<Vec<_>>();
    headings
        .iter()
        .enumerate()
        .map(|(index, title)| format!("{}. [{}](#{})", index + 1, title, slugify_heading(title)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_outline_toc_from_rendered_sections(all_sections: &[SectionDraft]) -> String {
    let headings = all_sections
        .iter()
        .filter_map(|section| {
            let title = section.title.trim();
            (!title.is_empty() && title != "目录").then_some(title)
        })
        .collect::<Vec<_>>();
    headings
        .iter()
        .enumerate()
        .map(|(index, title)| format!("{}. [{}](#{})", index + 1, title, slugify_heading(title)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn slugify_heading(title: &str) -> String {
    title
        .trim()
        .chars()
        .map(|character| match character {
            'A'..='Z' => character.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => character,
            _ if character.is_alphanumeric() => character,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn preserves_reference_markdown(content: &str) -> bool {
    let normalized = content.trim();
    normalized.contains("file://")
        || normalized.contains("<cite>")
        || normalized.contains("章节来源")
        || normalized.contains("图表来源")
}

fn render_citation_evidence_block(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> String {
    let citations = dedup_citations(citations);
    if citations.is_empty() {
        return String::new();
    }

    let mut lines = vec!["**证据**".to_string()];
    for citation in citations {
        let line_span = format!("L{}-L{}", citation.start_line, citation.end_line);
        let note = if citation.note.trim().is_empty() {
            String::new()
        } else {
            format!(" {}", citation.note.trim())
        };
        lines.push(format!(
            "- [`{}`](file://{}#{}) `{}`{}",
            citation.path, citation.path, line_span, line_span, note
        ));
    }
    lines.join("\n")
}

fn render_section_sources_block(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> String {
    let citations = dedup_citations(citations);
    if citations.is_empty() {
        return String::new();
    }

    let mut lines = vec!["章节来源".to_string()];
    for citation in citations {
        let line_span = format!("L{}-L{}", citation.start_line, citation.end_line);
        lines.push(format!(
            "- [{}](file://{}#{})",
            citation.path, citation.path, line_span
        ));
    }
    lines.join("\n")
}

fn collect_all_section_citations(
    sections: &[ComposeSectionDraft],
) -> Vec<wiki_knowledge::domain::research::SourceCitation> {
    dedup_citations(
        &sections
            .iter()
            .flat_map(|section| section.citations.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

fn render_appendix_entries_block(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> String {
    let citations = dedup_citations(citations);
    if citations.is_empty() {
        return String::new();
    }

    let mut lines = vec!["**关键实现入口**".to_string()];
    for citation in citations.iter().take(8) {
        let line_span = format!("L{}-L{}", citation.start_line, citation.end_line);
        let note = if citation.note.trim().is_empty() {
            "实现入口".to_string()
        } else {
            citation.note.trim().to_string()
        };
        lines.push(format!(
            "- [`{}`](file://{}#{})：{}",
            citation.path, citation.path, line_span, note
        ));
    }
    lines.join("\n")
}

fn dedup_citations(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> Vec<wiki_knowledge::domain::research::SourceCitation> {
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for citation in citations {
        let key = format!(
            "{}:{}:{}:{}",
            citation.path, citation.start_line, citation.end_line, citation.note
        );
        if seen.insert(key) {
            deduped.push(citation.clone());
        }
    }
    deduped
}

fn render_diagram_section(
    page_id: &str,
    diagrams: &[DiagramDraft],
    page_citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> SectionDraft {
    let mut content = Vec::new();
    for diagram in diagrams {
        content.push(render_diagram_block(diagram));
    }
    let diagram_sources = render_diagram_sources_block(page_citations);
    if !diagram_sources.is_empty() {
        content.push(diagram_sources);
    }

    SectionDraft {
        section_id: stable_id("section", format!("{page_id}:diagrams")),
        title: "结构图".to_string(),
        managed: true,
        source_ids: Vec::new(),
        relation_ids: Vec::new(),
        content: content.join("\n\n"),
    }
}

fn render_diagram_block(diagram: &DiagramDraft) -> String {
    let mut parts = vec![format!(
        "### {}\n```mermaid\n{}\n```",
        diagram.title,
        diagram.content.trim()
    )];
    if !diagram.description.trim().is_empty() {
        parts.push(diagram.description.trim().to_string());
    }
    parts.join("\n\n")
}

fn render_diagram_sources_block(
    citations: &[wiki_knowledge::domain::research::SourceCitation],
) -> String {
    let citations = dedup_citation_paths(citations);
    if citations.is_empty() {
        return String::new();
    }

    let mut lines = vec!["图表来源".to_string()];
    for citation in citations.iter().take(8) {
        let line_span = format!("L{}-L{}", citation.start_line, citation.end_line);
        lines.push(format!(
            "- [{}](file://{}#{})",
            citation.path, citation.path, line_span
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{render_compose_section_content, render_page_draft};
    use wiki_knowledge::domain::compose::{ComposeSectionDraft, DiagramDraft, PageDraft};
    use wiki_knowledge::domain::research::SourceCitation;

    #[test]
    fn renderer_avoids_duplicate_evidence_block_for_reference_markdown() {
        let section = ComposeSectionDraft {
            section_key: "overview".to_string(),
            title: "概述".to_string(),
            content: "<cite>\n- [index.ts](file://src/index.ts)\n</cite>".to_string(),
            citations: vec![SourceCitation {
                path: "src/index.ts".to_string(),
                start_line: 1,
                end_line: 10,
                source_id: Some("source:index".to_string()),
                symbol_id: None,
                note: String::new(),
            }],
            managed: true,
            preserve_source_markdown: true,
        };

        let rendered = render_compose_section_content(&section, std::slice::from_ref(&section));

        assert!(rendered.contains("<cite>"));
        assert!(!rendered.contains("**证据**"));
    }

    #[test]
    fn renderer_materializes_cite_and_outline_for_reference_style_sections() {
        let sections = vec![
            ComposeSectionDraft {
                section_key: "preamble".to_string(),
                title: String::new(),
                content: String::new(),
                citations: vec![
                    SourceCitation {
                        path: "src/index.ts".to_string(),
                        start_line: 1,
                        end_line: 10,
                        source_id: Some("source:index".to_string()),
                        symbol_id: None,
                        note: String::new(),
                    },
                    SourceCitation {
                        path: "src/index.ts".to_string(),
                        start_line: 1,
                        end_line: 10,
                        source_id: Some("source:index".to_string()),
                        symbol_id: None,
                        note: String::new(),
                    },
                ],
                managed: true,
                preserve_source_markdown: false,
            },
            ComposeSectionDraft {
                section_key: "toc".to_string(),
                title: "目录".to_string(),
                content: String::new(),
                citations: Vec::new(),
                managed: true,
                preserve_source_markdown: false,
            },
            ComposeSectionDraft {
                section_key: "intro".to_string(),
                title: "简介".to_string(),
                content: "简介正文".to_string(),
                citations: Vec::new(),
                managed: true,
                preserve_source_markdown: false,
            },
        ];

        let cite = render_compose_section_content(&sections[0], &sections);
        let toc = render_compose_section_content(&sections[1], &sections);

        assert!(cite.contains("<cite>"));
        assert!(cite.contains("本文引用的文件"));
        assert_eq!(cite.matches("accessibility").count(), 0);
        assert_eq!(cite.matches("src/index.ts").count(), 2);
        assert!(toc.contains("[简介](#简介)"));
    }

    #[test]
    fn render_page_draft_preserves_reference_outline_sequence_for_docs_backed_pages() {
        let draft = PageDraft {
            page_id: "page:docs-backed".to_string(),
            unit_id: "unit:docs-backed".to_string(),
            title: "可访问性测试".to_string(),
            relative_path: "概念指南/可访问性测试.md".to_string(),
            sections: vec![
                ComposeSectionDraft {
                    section_key: "preamble".to_string(),
                    title: String::new(),
                    content: "<cite>\n- [index.ts](file://src/index.ts)\n</cite>".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/index.ts".to_string(),
                        start_line: 1,
                        end_line: 10,
                        source_id: Some("source:index".to_string()),
                        symbol_id: None,
                        note: String::new(),
                    }],
                    managed: true,
                    preserve_source_markdown: true,
                },
                ComposeSectionDraft {
                    section_key: "toc".to_string(),
                    title: "目录".to_string(),
                    content: "1. [简介](#简介)\n2. [项目结构](#项目结构)".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: true,
                },
                ComposeSectionDraft {
                    section_key: "intro".to_string(),
                    title: "简介".to_string(),
                    content: "这里是简介。".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: true,
                },
            ],
            diagrams: Vec::new(),
            citation_count: 1,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("# 可访问性测试"));
        assert!(rendered.content.contains("<cite>"));
        assert!(rendered.content.contains("## 目录"));
        assert!(rendered.content.contains("## 简介"));
        assert!(!rendered.content.contains("## \n\n<cite>"));

        let cite_index = rendered.content.find("<cite>").unwrap();
        let toc_index = rendered.content.find("## 目录").unwrap();
        let intro_index = rendered.content.find("## 简介").unwrap();
        assert!(cite_index < toc_index);
        assert!(toc_index < intro_index);
    }

    #[test]
    fn render_page_draft_prepends_page_level_cite_block_when_missing() {
        let draft = PageDraft {
            page_id: "page:plugin-api".to_string(),
            unit_id: "unit:plugin-api".to_string(),
            title: "插件API".to_string(),
            relative_path: "API-参考/开发API参考/插件API.md".to_string(),
            sections: vec![ComposeSectionDraft {
                section_key: "overview".to_string(),
                title: "概述".to_string(),
                content: "插件 API 说明".to_string(),
                citations: vec![SourceCitation {
                    path: "code/core/src/manager-api/lib/addons.ts".to_string(),
                    start_line: 56,
                    end_line: 111,
                    source_id: Some("source:addons".to_string()),
                    symbol_id: None,
                    note: String::new(),
                }],
                managed: true,
                preserve_source_markdown: false,
            }],
            diagrams: Vec::new(),
            citation_count: 1,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("<cite>"));
        assert!(rendered.content.contains("**本文引用的文件**"));
        assert!(rendered.content.contains("章节来源"));
        let cite_index = rendered.content.find("<cite>").unwrap();
        let overview_index = rendered.content.find("## 概述").unwrap();
        assert!(cite_index < overview_index);
    }

    #[test]
    fn render_page_draft_inserts_generated_toc_for_multi_section_page() {
        let draft = PageDraft {
            page_id: "page:overview".to_string(),
            unit_id: "unit:overview".to_string(),
            title: "项目概述".to_string(),
            relative_path: "项目概述.md".to_string(),
            sections: vec![
                ComposeSectionDraft {
                    section_key: "intro".to_string(),
                    title: "简介".to_string(),
                    content: "项目简介".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/index.ts".to_string(),
                        start_line: 1,
                        end_line: 12,
                        source_id: Some("source:index".to_string()),
                        symbol_id: None,
                        note: String::new(),
                    }],
                    managed: true,
                    preserve_source_markdown: false,
                },
                ComposeSectionDraft {
                    section_key: "structure".to_string(),
                    title: "项目结构".to_string(),
                    content: "结构说明".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: false,
                },
                ComposeSectionDraft {
                    section_key: "conclusion".to_string(),
                    title: "结论".to_string(),
                    content: "总结。".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: false,
                },
            ],
            diagrams: Vec::new(),
            citation_count: 1,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("## 目录"));
        let cite_index = rendered.content.find("<cite>").unwrap();
        let toc_index = rendered.content.find("## 目录").unwrap();
        let intro_index = rendered.content.find("## 简介").unwrap();
        assert!(cite_index < toc_index);
        assert!(toc_index < intro_index);
        assert!(rendered.content.contains("[项目结构](#项目结构)"));
        assert!(rendered.content.contains("[结论](#结论)"));
    }

    #[test]
    fn render_page_draft_materializes_diagram_sources_from_page_citations() {
        let draft = PageDraft {
            page_id: "page:diagram".to_string(),
            unit_id: "unit:diagram".to_string(),
            title: "架构图".to_string(),
            relative_path: "系统架构.md".to_string(),
            sections: vec![ComposeSectionDraft {
                section_key: "overview".to_string(),
                title: "概述".to_string(),
                content: "架构说明".to_string(),
                citations: vec![SourceCitation {
                    path: "src/graph.ts".to_string(),
                    start_line: 3,
                    end_line: 20,
                    source_id: Some("source:graph".to_string()),
                    symbol_id: None,
                    note: String::new(),
                }],
                managed: true,
                preserve_source_markdown: false,
            }],
            diagrams: vec![DiagramDraft {
                diagram_id: "diagram:graph".to_string(),
                diagram_type: "dependency".to_string(),
                title: "依赖图".to_string(),
                description: "展示关键节点之间的依赖方向。".to_string(),
                content: "graph TD\nA-->B".to_string(),
            }],
            citation_count: 1,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("```mermaid"));
        assert!(rendered.content.contains("展示关键节点之间的依赖方向。"));
        assert!(rendered.content.contains("图表来源"));
        assert!(rendered.content.contains("file://src/graph.ts#L3-L20"));
    }

    #[test]
    fn render_page_draft_generates_outline_diagram_when_missing() {
        let draft = PageDraft {
            page_id: "page:outline".to_string(),
            unit_id: "unit:outline".to_string(),
            title: "概览页".to_string(),
            relative_path: "项目概述.md".to_string(),
            sections: vec![
                ComposeSectionDraft {
                    section_key: "intro".to_string(),
                    title: "简介".to_string(),
                    content: "简介".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: false,
                },
                ComposeSectionDraft {
                    section_key: "structure".to_string(),
                    title: "项目结构".to_string(),
                    content: "结构".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: false,
                },
                ComposeSectionDraft {
                    section_key: "conclusion".to_string(),
                    title: "结论".to_string(),
                    content: "结论".to_string(),
                    citations: Vec::new(),
                    managed: true,
                    preserve_source_markdown: false,
                },
            ],
            diagrams: Vec::new(),
            citation_count: 0,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("## 结构图"));
        assert!(rendered.content.contains("### 章节结构图"));
        assert!(rendered.content.contains("flowchart LR"));
        assert!(rendered.content.contains("展示本页主要章节的阅读顺序。"));
    }

    #[test]
    fn render_page_draft_appends_generated_appendix_when_missing() {
        let draft = PageDraft {
            page_id: "page:appendix".to_string(),
            unit_id: "unit:appendix".to_string(),
            title: "运行时API".to_string(),
            relative_path: "API-参考/运行时API.md".to_string(),
            sections: vec![ComposeSectionDraft {
                section_key: "overview".to_string(),
                title: "简介".to_string(),
                content: "运行时 API 说明".to_string(),
                citations: vec![SourceCitation {
                    path: "src/runtime.ts".to_string(),
                    start_line: 8,
                    end_line: 32,
                    source_id: Some("source:runtime".to_string()),
                    symbol_id: None,
                    note: "运行时入口".to_string(),
                }],
                managed: true,
                preserve_source_markdown: false,
            }],
            diagrams: Vec::new(),
            citation_count: 1,
        };

        let rendered = render_page_draft(&draft);

        assert!(rendered.content.contains("## 附录"));
        assert!(rendered.content.contains("**关键实现入口**"));
        assert!(rendered.content.contains("file://src/runtime.ts#L8-L32"));
    }

    #[test]
    fn appendix_section_collects_page_citations_when_section_has_none() {
        let sections = vec![
            ComposeSectionDraft {
                section_key: "overview".to_string(),
                title: "简介".to_string(),
                content: "概述".to_string(),
                citations: vec![SourceCitation {
                    path: "src/index.ts".to_string(),
                    start_line: 1,
                    end_line: 12,
                    source_id: Some("source:index".to_string()),
                    symbol_id: None,
                    note: "模块入口".to_string(),
                }],
                managed: true,
                preserve_source_markdown: false,
            },
            ComposeSectionDraft {
                section_key: "appendix".to_string(),
                title: "附录".to_string(),
                content: String::new(),
                citations: Vec::new(),
                managed: true,
                preserve_source_markdown: false,
            },
        ];

        let rendered = render_compose_section_content(&sections[1], &sections);

        assert!(rendered.contains("**关键实现入口**"));
        assert!(rendered.contains("file://src/index.ts#L1-L12"));
    }
}

/// 按标题和 section 草稿组装带 managed marker 的整页 Markdown。
fn assemble_page_from_sections(title: &str, sections: &[SectionDraft]) -> String {
    let blocks: Vec<PageBlock> = sections
        .iter()
        .map(|s| {
            PageBlock::Managed(ManagedSectionBlock::generated(
                s.section_id.clone(),
                s.title.clone(),
                s.content.clone(),
            ))
        })
        .collect();
    render_page_with_markers(title, &blocks)
}

/// 基于 merge plan 组装最终页面 Markdown。
/// 这是 update / rebuild 保留 user sections 的统一出口。
///
/// # 参数
/// - `title`：页面一级标题。
/// - `merge_plan`：合并计划，包含 managed + user 区段序列。
///
/// # 返回
/// - 返回可直接写入页面文件的整页 Markdown。
pub fn assemble_page_from_merge(title: &str, merge_plan: &PageMergePlan) -> String {
    render_page_with_markers(title, &merge_plan.blocks)
}
