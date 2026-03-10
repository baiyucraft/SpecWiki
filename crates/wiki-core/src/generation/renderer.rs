//! 渲染层负责把页面上下文和 section 草稿组装成最终 Markdown。
//! 它只关心表达形式，不参与模块树、变化规划和状态判断。

use crate::domain::context::PageContext;
use crate::generation::managed_sections::{
    render_page_with_markers, ManagedSectionBlock, PageBlock, PageMergePlan,
};
use crate::generation::planner::PlannedPage;
use crate::generation::sections::{build_section_drafts, SectionDraft};

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
pub fn render_page(page: &PlannedPage, context: &PageContext) -> String {
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
pub fn render_page_bundle(page: &PlannedPage, context: &PageContext) -> RenderedPage {
    let sections = build_section_drafts(page, context);
    let content = assemble_page(page, &sections);

    RenderedPage { sections, content }
}

/// 把 section 草稿转换成 managed blocks，用于 merge 和渲染。
pub fn drafts_to_managed_blocks(sections: &[SectionDraft]) -> Vec<ManagedSectionBlock> {
    sections
        .iter()
        .map(|s| ManagedSectionBlock {
            section_id: s.section_id.clone(),
            title: s.title.clone(),
            version: crate::generation::managed_sections::MARKER_VERSION,
            body: s.content.clone(),
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
pub fn assemble_page(page: &PlannedPage, sections: &[SectionDraft]) -> String {
    let blocks: Vec<PageBlock> = sections
        .iter()
        .map(|s| {
            PageBlock::Managed(ManagedSectionBlock {
                section_id: s.section_id.clone(),
                title: s.title.clone(),
                version: crate::generation::managed_sections::MARKER_VERSION,
                body: s.content.clone(),
            })
        })
        .collect();

    render_page_with_markers(&page.title, &blocks)
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
