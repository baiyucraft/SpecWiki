use crate::domain::context::PageContext;

/// 生成项目概述页的标准章节集合。
///
/// # 参数
/// - `context`：项目概述页对应的页面上下文。
///
/// # 返回
/// - 返回概述页应写出的章节列表。
pub fn overview_sections(context: &PageContext) -> Vec<String> {
    vec![
        "## 简介\n\n由 codebuddy-wiki 自动生成的仓库概览。".to_string(),
        format!("## 项目事实\n\n{}", bullet_lines(&context.facts)),
        format!("## 关键信息\n\n{}", bullet_lines(&context.summary_inputs)),
    ]
}

/// 生成系统架构页的标准章节集合。
///
/// # 参数
/// - `context`：系统架构页对应的页面上下文。
///
/// # 返回
/// - 返回架构页应写出的章节列表。
pub fn architecture_sections(context: &PageContext) -> Vec<String> {
    vec![
        "## 架构概览\n\n该页面用于说明仓库的模块层级与主要关系。".to_string(),
        format!("## 模块结构\n\n{}", bullet_lines(&context.facts)),
        format!("## 关系摘要\n\n{}", bullet_lines(&context.summary_inputs)),
    ]
}

/// 生成模块页的标准章节集合。
///
/// # 参数
/// - `context`：模块页对应的页面上下文。
///
/// # 返回
/// - 返回模块页应写出的章节列表。
pub fn module_sections(context: &PageContext) -> Vec<String> {
    vec![
        "## 模块说明\n\n该页面围绕单个模块整理其边界、入口和依赖。".to_string(),
        format!("## 模块事实\n\n{}", bullet_lines(&context.facts)),
        format!("## 关键摘要\n\n{}", bullet_lines(&context.summary_inputs)),
    ]
}

/// 把字符串列表渲染成 Markdown 项目符号列表。
///
/// # 参数
/// - `lines`：要渲染成列表的字符串集合。
///
/// # 返回
/// - 返回 Markdown 项目符号列表文本；如果为空则返回 `- 无`。
fn bullet_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return "- 无".to_string();
    }

    lines.iter()
        .map(|line| format!("- {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
