//! section 生成层负责把页面上下文拆成稳定章节草稿。
//! 它不做写盘，只负责为 renderer、state 和 cache 提供可复用的 section 粒度产物。

use serde::{Deserialize, Serialize};

use crate::domain::context::PageContext;
use crate::domain::stable_id::stable_id;
use crate::generation::planner::PlannedPage;
use crate::llm::PageEnrichmentResult;

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
    build_section_drafts_with_enrichment(page, context, None)
}

/// 为当前页面构建可选 LLM 增强后的稳定 section 草稿集合。
pub fn build_section_drafts_with_enrichment(
    page: &PlannedPage,
    context: &PageContext,
    enrichment: Option<&PageEnrichmentResult>,
) -> Vec<SectionDraft> {
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
            content: merged_section_content(&title, &content, enrichment),
            title,
            managed: true,
            source_ids: context.source_ids.clone(),
            relation_ids: context.relation_ids.clone(),
        })
        .collect()
}

fn overview_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        ("简介".to_string(), overview_intro(context)),
        ("项目事实".to_string(), overview_project_facts(context)),
        ("技术栈".to_string(), tech_stack_section(context)),
        ("入口与构建".to_string(), entry_and_build_section(context)),
        ("关键信息".to_string(), overview_key_insights(context)),
    ]
}

fn architecture_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        ("架构概览".to_string(), architecture_overview(context)),
        (
            "模块结构".to_string(),
            architecture_module_structure(context),
        ),
        ("跨模块关系".to_string(), relation_section(context)),
        ("架构提示".to_string(), architecture_hints_section(context)),
    ]
}

fn module_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        ("模块说明".to_string(), module_intro(context)),
        ("关键源码".to_string(), source_section(context)),
        ("依赖关系".to_string(), dependency_section(context)),
        ("模块事实".to_string(), module_fact_section(context)),
        ("子模块概述".to_string(), child_module_section(context)),
    ]
}

fn workflow_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        ("工作流概述".to_string(), workflow_overview(context)),
        (
            "构建流程".to_string(),
            prefixed_bullets(&context.facts, "构建", "当前未检测到稳定的构建流程线索。"),
        ),
        (
            "CI/CD 配置".to_string(),
            prefixed_bullets(&context.facts, "CI", "当前未检测到 CI/CD 配置。"),
        ),
        (
            "容器化".to_string(),
            prefixed_bullets(&context.facts, "容器", "当前未检测到容器化配置。"),
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

fn paragraph_lines(lines: &[String], empty: &str) -> String {
    if lines.is_empty() {
        empty.to_string()
    } else {
        lines.join(" ")
    }
}

/// 从事实列表中筛选以指定前缀开头的条目。
fn filter_prefixed(lines: &[String], prefix: &str) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .cloned()
        .collect()
}

fn prefixed_values(lines: &[String], prefix: &str) -> Vec<String> {
    filter_prefixed(lines, prefix)
        .into_iter()
        .map(|line| {
            line.strip_prefix(&format!("{prefix}："))
                .unwrap_or(line.as_str())
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect()
}

fn prefixed_bullets(lines: &[String], prefix: &str, empty: &str) -> String {
    let values = prefixed_values(lines, prefix);
    if values.is_empty() {
        empty.to_string()
    } else {
        bullet_lines(&values)
    }
}

fn fact_value<'a>(facts: &'a [String], key: &str) -> Option<&'a str> {
    facts.iter().find_map(|line| {
        line.strip_prefix(&format!("{key}："))
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

fn grouped_bullet_block(title: &str, lines: &[String]) -> Option<String> {
    (!lines.is_empty()).then(|| format!("{title}：\n{}", bullet_lines(lines)))
}

fn overview_intro(context: &PageContext) -> String {
    let repo_root = fact_value(&context.facts, "仓库根路径").unwrap_or("当前仓库");
    let tech_stack = prefixed_values(&context.facts, "技术栈");
    let module_count = fact_value(&context.facts, "模块数量").unwrap_or("未识别");
    let top_modules = fact_value(&context.facts, "顶层模块").unwrap_or("未识别");

    let mut sentences = vec![format!("该页面概览 `{repo_root}` 的整体结构与主要知识面。")];
    if !tech_stack.is_empty() {
        sentences.push(format!(
            "当前识别到的技术栈包括 {}。",
            tech_stack.join("、")
        ));
    }
    if module_count != "未识别" || top_modules != "未识别" {
        sentences.push(format!(
            "仓库当前拆分出 {module_count} 个模块，顶层关注点主要集中在 {top_modules}。"
        ));
    }

    paragraph_lines(&sentences, "该页面概览仓库的整体结构。")
}

fn overview_project_facts(context: &PageContext) -> String {
    let facts = context
        .facts
        .iter()
        .filter(|line| !line.starts_with("技术栈："))
        .cloned()
        .collect::<Vec<_>>();
    bullet_lines(&facts)
}

fn tech_stack_section(context: &PageContext) -> String {
    prefixed_bullets(&context.facts, "技术栈", "当前未识别出稳定的技术栈线索。")
}

fn entry_and_build_section(context: &PageContext) -> String {
    prefixed_bullets(
        &context.summary_inputs,
        "入口",
        "当前未检测到稳定的入口或构建线索。",
    )
}

fn overview_key_insights(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "核心源码",
        &prefixed_values(&context.summary_inputs, "核心源码"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "关键调用热点",
        &prefixed_values(&context.summary_inputs, "图热点"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "核心流程",
        &prefixed_values(&context.summary_inputs, "流程"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "社区聚类",
        &prefixed_values(&context.summary_inputs, "社区"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "循环提示",
        &prefixed_values(&context.summary_inputs, "循环"),
    ) {
        blocks.push(block);
    }

    if blocks.is_empty() {
        "当前没有额外的图分析补充信息。".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn architecture_overview(context: &PageContext) -> String {
    let module_count = fact_value(&context.facts, "模块数量").unwrap_or("未识别");
    let relation_count = fact_value(&context.facts, "跨模块关系").unwrap_or("0");
    let top_modules = fact_value(&context.facts, "顶层模块").unwrap_or("未识别");
    let mut sentences = vec![format!(
        "该页面总结仓库的模块边界与协作方式，当前识别出 {module_count} 个模块。"
    )];
    sentences.push(format!("顶层结构主要由 {top_modules} 组成。"));
    if relation_count == "0" {
        sentences.push("当前还没有检测到稳定的跨模块依赖。".to_string());
    } else {
        sentences.push(format!("当前检测到 {relation_count} 条跨模块关系。"));
    }

    paragraph_lines(&sentences, "该页面用于说明仓库的模块层级与主要关系。")
}

fn architecture_module_structure(context: &PageContext) -> String {
    let summary_facts = context
        .facts
        .iter()
        .filter(|line| !line.starts_with("模块树："))
        .cloned()
        .collect::<Vec<_>>();
    let tree_lines = prefixed_values(&context.facts, "模块树");
    let mut blocks = Vec::new();

    if !summary_facts.is_empty() {
        blocks.push(bullet_lines(&summary_facts));
    }
    if !tree_lines.is_empty() {
        blocks.push(render_module_tree_markdown(&tree_lines));
    }

    if blocks.is_empty() {
        "- 无".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn relation_section(context: &PageContext) -> String {
    prefixed_bullets(
        &context.summary_inputs,
        "关系",
        "当前未检测到稳定的跨模块关系。",
    )
}

fn architecture_hints_section(context: &PageContext) -> String {
    prefixed_bullets(&context.summary_inputs, "架构", "当前没有额外的架构提示。")
}

fn module_intro(context: &PageContext) -> String {
    let module_name = fact_value(&context.facts, "模块名称").unwrap_or("当前模块");
    let module_kind = fact_value(&context.facts, "模块类型").unwrap_or("module");
    let module_root = fact_value(&context.facts, "模块根路径").unwrap_or(".");
    let entry = fact_value(&context.facts, "入口文件").unwrap_or("无");
    let roles = prefixed_values(&context.summary_inputs, "模块角色");

    let mut sentences = vec![format!(
        "`{module_name}` 是一个 {module_kind} 模块，根路径位于 `{module_root}`。"
    )];
    if !roles.is_empty() && roles.first().is_some_and(|role| role != "未标注") {
        sentences.push(format!("当前角色线索表明它主要承担 {}。", roles.join("、")));
    }
    if entry != "无" {
        sentences.push(format!("当前识别到的公开入口包括 {entry}。"));
    } else {
        sentences.push("当前未识别到稳定的入口文件。".to_string());
    }

    paragraph_lines(&sentences, "该页面围绕单个模块整理其边界、入口和依赖。")
}

fn source_section(context: &PageContext) -> String {
    prefixed_bullets(
        &context.summary_inputs,
        "源码",
        "当前没有筛出高信号的关键源码文件。",
    )
}

fn dependency_section(context: &PageContext) -> String {
    prefixed_bullets(
        &context.summary_inputs,
        "依赖",
        "当前未识别出稳定的跨模块依赖。",
    )
}

fn module_fact_section(context: &PageContext) -> String {
    let mut blocks = vec![bullet_lines(&context.facts)];

    if let Some(block) = grouped_bullet_block(
        "关键调用热点",
        &prefixed_values(&context.summary_inputs, "图热点"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "社区聚类",
        &prefixed_values(&context.summary_inputs, "社区"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "循环提示",
        &prefixed_values(&context.summary_inputs, "循环"),
    ) {
        blocks.push(block);
    }

    blocks.join("\n\n")
}

fn child_module_section(context: &PageContext) -> String {
    prefixed_bullets(
        &context.summary_inputs,
        "子模块",
        "当前没有独立展开的子模块。",
    )
}

fn workflow_overview(context: &PageContext) -> String {
    let builds = prefixed_values(&context.facts, "构建");
    let ci = prefixed_values(&context.facts, "CI");
    let containers = prefixed_values(&context.facts, "容器");

    let mut sentences = vec!["该页面汇总仓库的构建、交付与部署相关事实。".to_string()];
    if !builds.is_empty() {
        sentences.push(format!("当前检测到 {} 条构建流程线索。", builds.len()));
    }
    if !ci.is_empty() {
        sentences.push(format!("CI/CD 主要由 {} 支撑。", ci.join("、")));
    }
    if !containers.is_empty() {
        sentences.push(format!("同时存在容器化线索：{}。", containers.join("、")));
    }

    paragraph_lines(&sentences, "该页面描述仓库的构建、CI/CD 和部署配置。")
}

fn render_module_tree_markdown(lines: &[String]) -> String {
    lines
        .iter()
        .map(|line| {
            let (depth, title) = line
                .split_once(':')
                .and_then(|(depth, title)| depth.parse::<usize>().ok().map(|depth| (depth, title)))
                .unwrap_or((0, line.as_str()));
            format!("{}- {}", "  ".repeat(depth), title.trim())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn merged_section_content(
    title: &str,
    deterministic: &str,
    enrichment: Option<&PageEnrichmentResult>,
) -> String {
    let Some(enrichment) = enrichment else {
        return deterministic.to_string();
    };

    let mut content = enrichment
        .section_overrides
        .get(title)
        .cloned()
        .unwrap_or_else(|| deterministic.to_string());

    if let Some(mermaid) = enrichment.mermaid_blocks.get(title) {
        if !content.is_empty() {
            content.push_str("\n\n");
        }
        content.push_str("```mermaid\n");
        content.push_str(mermaid);
        content.push_str("\n```");
    }

    if content.trim().is_empty() {
        deterministic.to_string()
    } else {
        content
    }
}
