use wiki_core::domain::compose::{ComposeSectionDraft, DiagramDraft, PageDraft};
use wiki_core::domain::context::PageContext;
use wiki_core::domain::research::SourceCitation;
use wiki_core::generation::planner::PlannedPage;
use wiki_core::generation::renderer::{render_page_bundle, render_page_draft};

fn planned_page(page_type: &str, title: &str, path: &str) -> PlannedPage {
    PlannedPage {
        id: format!("page:{path}"),
        title: title.to_string(),
        relative_path: path.to_string(),
        page_type: page_type.to_string(),
        parent_id: None,
        scope: "unit".to_string(),
        unit_id: Some(format!("unit:{path}")),
        unit_type: Some(page_type.to_string()),
        domain_id: Some("domain:test".to_string()),
        source_ids: vec!["src:a".to_string()],
        module_ids: vec!["module:a".to_string()],
        relation_ids: Vec::new(),
        generation_mode: "test".to_string(),
        priority: 0,
        merged_module_ids: Vec::new(),
    }
}

fn page_context(page_type: &str) -> PageContext {
    PageContext {
        page_id: format!("page:{page_type}"),
        page_type: page_type.to_string(),
        scope: "unit".to_string(),
        unit_id: Some(format!("unit:{page_type}")),
        unit_type: Some(page_type.to_string()),
        domain_id: Some("domain:test".to_string()),
        source_ids: vec!["src:a".to_string()],
        module_ids: vec!["module:a".to_string()],
        relation_ids: Vec::new(),
        facts: vec![
            format!("页面类型：{page_type}"),
            "知识域标题：示例页面".to_string(),
            "知识域类别：concept".to_string(),
        ],
        summary_inputs: vec![
            "关键源码：src/lib.rs".to_string(),
            "知识域摘要：示例摘要".to_string(),
        ],
        hints: Vec::new(),
        child_summaries: vec!["子页：摘要".to_string()],
        evidence_groups: Vec::new(),
        diagram_inputs: Vec::new(),
    }
}

#[test]
fn render_page_bundle_keeps_deterministic_sections() {
    let page = planned_page("module", "模块页", "核心模块/module.md");
    let context = page_context("module");
    let rendered = render_page_bundle(&page, &context);

    assert!(!rendered.sections.is_empty());
    assert!(rendered.content.contains("# 模块页"));
    assert!(rendered.content.contains("## 目录"));
    assert!(rendered.content.contains("## 简介"));
    assert!(rendered.content.contains("## 项目结构"));
    assert!(rendered.content.contains("## 核心组件"));
    assert!(rendered.content.contains("## 架构总览"));
    assert!(rendered.content.contains("## 详细组件分析"));
    assert!(rendered.content.contains("## 依赖关系分析"));
    assert!(rendered.content.contains("## 结论"));
    assert!(rendered.content.contains("## 附录"));
}

#[test]
fn render_family_leaf_bundle_keeps_leaf_sections() {
    let page = planned_page("family-leaf-doc", "叶子页", "知识域/叶子页.md");
    let context = page_context("family-leaf-doc");
    let rendered = render_page_bundle(&page, &context);

    assert!(rendered.content.contains("## 目录"));
    assert!(rendered.content.contains("## 简介"));
    assert!(rendered.content.contains("## 项目结构"));
    assert!(rendered.content.contains("## 核心组件"));
    assert!(rendered.content.contains("## 架构总览"));
    assert!(rendered.content.contains("## 详细组件分析"));
    assert!(rendered.content.contains("## 依赖关系分析"));
    assert!(rendered.content.contains("## 结论"));
    assert!(rendered.content.contains("## 附录"));
}

#[test]
fn render_page_draft_uses_compose_sections_directly() {
    let draft = PageDraft {
        page_id: "page:test".to_string(),
        unit_id: "unit:test".to_string(),
        title: "Compose 页".to_string(),
        relative_path: "Compose 页.md".to_string(),
        sections: vec![
            ComposeSectionDraft {
                section_key: "intro".to_string(),
                title: "简介".to_string(),
                content: "来自 compose 的正文。".to_string(),
                citations: vec![SourceCitation {
                    path: "src/demo.ts".to_string(),
                    start_line: 3,
                    end_line: 12,
                    source_id: Some("src:demo".to_string()),
                    symbol_id: None,
                    note: "入口实现".to_string(),
                }],
                managed: true,
                preserve_source_markdown: false,
            },
            ComposeSectionDraft {
                section_key: "details".to_string(),
                title: "细节".to_string(),
                content: "第二节正文。".to_string(),
                citations: Vec::new(),
                managed: true,
                preserve_source_markdown: false,
            },
        ],
        diagrams: vec![DiagramDraft {
            diagram_id: "diagram:runtime".to_string(),
            diagram_type: "dependency".to_string(),
            title: "运行时依赖".to_string(),
            description: "说明 Repo 与 Provider 之间的运行时依赖。".to_string(),
            content: "graph TD\nRepo-->Provider".to_string(),
        }],
        citation_count: 1,
    };

    let rendered = render_page_draft(&draft);
    assert!(rendered.content.contains("# Compose 页"));
    assert!(rendered.content.contains("<cite>"));
    assert!(rendered.content.contains("## 简介"));
    assert!(rendered.content.contains("来自 compose 的正文。"));
    assert!(rendered.content.contains("**证据**"));
    assert!(rendered.content.contains("(file://src/demo.ts#L3-L12)"));
    assert!(rendered.content.contains("```mermaid"));
    assert!(rendered
        .content
        .contains("说明 Repo 与 Provider 之间的运行时依赖。"));
    assert_eq!(rendered.sections.len(), 4);
}
