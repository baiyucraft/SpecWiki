//! section 生成层负责把页面上下文拆成稳定章节草稿。
//! 它不做写盘，只负责为 renderer、state 和 cache 提供可复用的 section 粒度产物。

use serde::{Deserialize, Serialize};

use crate::domain::context::PageContext;
use crate::domain::stable_id::stable_id;
use crate::generation::planner::PlannedPage;

/// `SectionDraft` 是页面组装前的标准章节草稿。
/// 迭代 3 之后，页面重生成将围绕这个粒度做局部替换和缓存复用。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SectionDraft {
    /// 页面内稳定 section ID，用于状态层和缓存层复用。
    pub section_id: String,
    /// section 展示标题，会被组装成 Markdown 二级标题。
    pub title: String,
    /// 当前 section 是否属于 runtime 托管区段。
    pub managed: bool,
    /// 当前 section 依赖的源码 ID 集合。
    pub source_ids: Vec<String>,
    /// 当前 section 依赖的关系 ID 集合。
    pub relation_ids: Vec<String>,
    /// 当前 section 的 Markdown 正文草稿。
    pub content: String,
}

/// 返回某类页面的稳定 section 标题模板。
///
/// # 参数
/// - `page_type`：页面类型，如 `overview / architecture / module / workflow`。
///
/// # 返回
/// - 返回该页面类型对应的稳定 section 标题顺序。
pub fn section_titles_for_page_type(page_type: &str) -> Vec<&'static str> {
    match page_type {
        "overview" => vec!["简介", "项目事实", "技术栈", "入口与构建", "关键信息"],
        "architecture" => vec!["架构概览", "模块结构", "跨模块关系", "架构提示"],
        "module" => vec!["模块说明", "关键源码", "依赖关系", "模块事实", "子模块概述"],
        "workflow" => vec!["工作流概述", "构建流程", "CI/CD 配置", "容器化"],
        _ => vec!["简介"],
    }
}

/// 基于页面 ID 和标题生成稳定 section ID。
///
/// # 参数
/// - `page_id`：页面稳定 ID。
/// - `title`：section 标题。
///
/// # 返回
/// - 返回稳定的 section ID。
pub fn section_id_for_title(page_id: &str, title: &str) -> String {
    stable_id("section", &format!("{page_id}:{title}"))
}

/// 为当前页面构建稳定的 section 草稿集合。
///
/// # 参数
/// - `page`：当前页面计划。
/// - `context`：当前页面上下文。
///
/// # 返回
/// - 返回按稳定顺序排列的 section 草稿集合。
pub fn build_section_drafts(page: &PlannedPage, context: &PageContext) -> Vec<SectionDraft> {
    let templates = match page.page_type.as_str() {
        "overview" => overview_section_templates(context),
        "architecture" => architecture_section_templates(context),
        "module" => module_section_templates(context),
        "workflow" => workflow_section_templates(context),
        _ => vec![(
            "简介".to_string(),
            "由 codebuddy-wiki 自动生成。".to_string(),
        )],
    };

    templates
        .into_iter()
        .map(|(title, content)| SectionDraft {
            section_id: section_id_for_title(&page.id, &title),
            title,
            managed: true,
            source_ids: context.source_ids.clone(),
            relation_ids: context.relation_ids.clone(),
            content,
        })
        .collect()
}

fn overview_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        (
            "简介".to_string(),
            "由 codebuddy-wiki 自动生成的仓库概览。".to_string(),
        ),
        ("项目事实".to_string(), bullet_lines(&context.facts)),
        (
            "技术栈".to_string(),
            bullet_lines(&filter_prefixed(&context.facts, "技术栈")),
        ),
        (
            "入口与构建".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "入口")),
        ),
        (
            "关键信息".to_string(),
            bullet_lines(&context.summary_inputs),
        ),
    ]
}

fn architecture_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        (
            "架构概览".to_string(),
            "该页面用于说明仓库的模块层级与主要关系。".to_string(),
        ),
        ("模块结构".to_string(), bullet_lines(&context.facts)),
        (
            "跨模块关系".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "关系")),
        ),
        (
            "架构提示".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "架构")),
        ),
    ]
}

fn module_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        (
            "模块说明".to_string(),
            "该页面围绕单个模块整理其边界、入口和依赖。".to_string(),
        ),
        (
            "关键源码".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "源码")),
        ),
        (
            "依赖关系".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "依赖")),
        ),
        (
            "模块事实".to_string(),
            bullet_lines(&merge_lines(
                &context.facts,
                &filter_prefixed(&context.summary_inputs, "图热点"),
                &filter_prefixed(&context.summary_inputs, "社区"),
                &filter_prefixed(&context.summary_inputs, "循环"),
            )),
        ),
        (
            "子模块概述".to_string(),
            bullet_lines(&filter_prefixed(&context.summary_inputs, "子模块")),
        ),
    ]
}

fn workflow_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        (
            "工作流概述".to_string(),
            "该页面描述仓库的构建、CI/CD 和部署配置。".to_string(),
        ),
        (
            "构建流程".to_string(),
            bullet_lines(&filter_prefixed(&context.facts, "构建")),
        ),
        (
            "CI/CD 配置".to_string(),
            bullet_lines(&filter_prefixed(&context.facts, "CI")),
        ),
        (
            "容器化".to_string(),
            bullet_lines(&filter_prefixed(&context.facts, "容器")),
        ),
    ]
}

/// 把字符串列表渲染成 Markdown 项目符号列表。
fn bullet_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return "- 无".to_string();
    }

    lines
        .iter()
        .map(|line| format!("- {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// 从事实列表中筛选以指定前缀开头的条目。
fn filter_prefixed(lines: &[String], prefix: &str) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .cloned()
        .collect()
}

fn merge_lines(groups: &[String], extra_a: &[String], extra_b: &[String], extra_c: &[String]) -> Vec<String> {
    groups
        .iter()
        .chain(extra_a.iter())
        .chain(extra_b.iter())
        .chain(extra_c.iter())
        .cloned()
        .collect()
}
