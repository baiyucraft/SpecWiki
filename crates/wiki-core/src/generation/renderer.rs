use crate::domain::context::PageContext;
use crate::generation::planner::PlannedPage;
use crate::generation::sections::{
    architecture_sections, module_sections, overview_sections,
};

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
    let mut lines = vec![format!("# {}", page.title)];

    match page.page_type.as_str() {
        "overview" => lines.extend(overview_sections(context)),
        "architecture" => lines.extend(architecture_sections(context)),
        "module" => lines.extend(module_sections(context)),
        _ => {
            lines.push("## 简介\n\n由 codebuddy-wiki 自动生成。".to_string());
        }
    }

    lines.join("\n\n")
}
