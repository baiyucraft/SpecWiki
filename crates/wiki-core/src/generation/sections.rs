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
        "topic" => vec!["主题说明", "关键证据", "结构图", "关联模块"],
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
        "topic" => topic_section_templates(context),
        _ => vec![(
            "简介".to_string(),
            "由 codebuddy-wiki 自动生成。".to_string(),
        )],
    };

    templates
        .into_iter()
        .map(|(title, content)| {
            let content = merged_section_content(&title, &content, enrichment);
            SectionDraft {
                section_id: section_id_for_title(&page.id, &title),
                content: append_supporting_blocks(&title, &content, context),
                title,
                managed: true,
                source_ids: context.source_ids.clone(),
                relation_ids: context.relation_ids.clone(),
            }
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

fn topic_section_templates(context: &PageContext) -> Vec<(String, String)> {
    vec![
        ("主题说明".to_string(), topic_intro(context)),
        ("关键证据".to_string(), topic_evidence_section(context)),
        ("结构图".to_string(), topic_diagram_section(context)),
        ("关联模块".to_string(), topic_related_section(context)),
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
    if let Some(summary) = context
        .research_result
        .as_ref()
        .map(|result| result.summary.trim())
        .filter(|summary| !summary.is_empty())
    {
        return summary.to_string();
    }

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
    if let Some(dossier) = context.module_dossiers.first() {
        let mut blocks = Vec::new();
        if !dossier.key_sources.is_empty() {
            blocks.push(grouped_bullet_block("关键源码", &dossier.key_sources).unwrap());
        }
        let snippets = dossier
            .source_snippets
            .iter()
            .take(3)
            .map(|snippet| {
                format!(
                    "- `{}`:{}-{}",
                    snippet.path, snippet.start_line, snippet.end_line
                )
            })
            .collect::<Vec<_>>();
        if let Some(block) = grouped_bullet_block("源码片段", &snippets) {
            blocks.push(block);
        }
        if !blocks.is_empty() {
            return blocks.join("\n\n");
        }
    }

    prefixed_bullets(
        &context.summary_inputs,
        "源码",
        "当前没有筛出高信号的关键源码文件。",
    )
}

fn dependency_section(context: &PageContext) -> String {
    if let Some(dossier) = context.module_dossiers.first() {
        if !dossier.cross_module_edges.is_empty() {
            return bullet_lines(&dossier.cross_module_edges);
        }
    }

    prefixed_bullets(
        &context.summary_inputs,
        "依赖",
        "当前未识别出稳定的跨模块依赖。",
    )
}

fn module_fact_section(context: &PageContext) -> String {
    let mut blocks = vec![bullet_lines(&context.facts)];
    if let Some(result) = &context.research_result {
        if let Some(block) = grouped_bullet_block("关键要点", &result.key_points) {
            blocks.push(block);
        }
        if let Some(block) = grouped_bullet_block("待确认点", &result.open_questions) {
            blocks.push(block);
        }
    }

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
    if !context.child_rollups.is_empty() {
        let lines = context
            .child_rollups
            .iter()
            .map(|rollup| {
                if rollup.summary.trim().is_empty() {
                    format!("{}（{}）", rollup.title, rollup.page_type)
                } else {
                    format!("{}：{}", rollup.title, rollup.summary.trim())
                }
            })
            .collect::<Vec<_>>();
        return bullet_lines(&lines);
    }

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

fn topic_intro(context: &PageContext) -> String {
    if let Some(summary) = context
        .research_result
        .as_ref()
        .map(|result| result.summary.trim())
        .filter(|summary| !summary.is_empty())
    {
        return summary.to_string();
    }

    let topic_title = fact_value(&context.facts, "主题标题").unwrap_or("当前主题");
    let topic_kind = fact_value(&context.facts, "主题类别").unwrap_or("topic");
    let source_count = fact_value(&context.facts, "关联源码数").unwrap_or("0");
    let topic_summary = prefixed_values(&context.summary_inputs, "主题摘要");

    let mut sentences = vec![format!(
        "`{topic_title}` 页面聚焦一个 {topic_kind} 类型的稳定专题。"
    )];
    if let Some(summary) = topic_summary.first() {
        sentences.push(summary.clone());
    }
    sentences.push(format!(
        "当前主题直接关联 {source_count} 份源码或配置线索。"
    ));

    paragraph_lines(&sentences, "该页面用于解释一个稳定专题。")
}

fn topic_evidence_section(context: &PageContext) -> String {
    if let Some(result) = &context.research_result {
        let lines = result
            .evidence_rollup
            .iter()
            .map(|group| {
                let items = group
                    .items
                    .iter()
                    .take(4)
                    .map(|item| item.path.clone())
                    .collect::<Vec<_>>();
                format!("{}：{}", group.title, items.join("、"))
            })
            .filter(|line| !line.ends_with('：'))
            .collect::<Vec<_>>();
        if !lines.is_empty() {
            return bullet_lines(&lines);
        }
    }

    if let Some(dossier) = &context.topic_dossier {
        let lines = dossier
            .evidence_rollup
            .iter()
            .map(|group| {
                let items = group
                    .items
                    .iter()
                    .take(4)
                    .map(|item| item.path.clone())
                    .collect::<Vec<_>>();
                format!("{}：{}", group.title, items.join("、"))
            })
            .filter(|line| !line.ends_with('：'))
            .collect::<Vec<_>>();
        if !lines.is_empty() {
            return bullet_lines(&lines);
        }
    }

    let source_paths = prefixed_values(&context.summary_inputs, "关键源码");
    if source_paths.is_empty() {
        "当前主题还没有额外的关键源码摘要。".to_string()
    } else {
        grouped_bullet_block("关键源码", &source_paths).unwrap_or_else(|| "- 无".to_string())
    }
}

fn topic_diagram_section(context: &PageContext) -> String {
    if let Some(result) = &context.research_result {
        if !result.diagram_rollup.is_empty() {
            let lines = result
                .diagram_rollup
                .iter()
                .map(|diagram| {
                    if diagram.summary.trim().is_empty() {
                        format!("{}（{}）", diagram.title, diagram.diagram_type)
                    } else {
                        format!("{}：{}", diagram.title, diagram.summary.trim())
                    }
                })
                .collect::<Vec<_>>();
            return bullet_lines(&lines);
        }
    }

    if context
        .diagram_inputs
        .iter()
        .any(|diagram| diagram.section_title == "结构图")
    {
        "当前专题页包含 deterministic 结构图，用于解释主题内部的关系。".to_string()
    } else {
        "当前事实不足以生成稳定结构图。".to_string()
    }
}

fn topic_related_section(context: &PageContext) -> String {
    if let Some(dossier) = &context.topic_dossier {
        if !dossier.child_page_rollup.is_empty() {
            let lines = dossier
                .child_page_rollup
                .iter()
                .map(|rollup| {
                    if rollup.summary.trim().is_empty() {
                        rollup.title.clone()
                    } else {
                        format!("{}：{}", rollup.title, rollup.summary.trim())
                    }
                })
                .collect::<Vec<_>>();
            return bullet_lines(&lines);
        }
    }

    let related_modules = prefixed_values(&context.summary_inputs, "关联模块");
    if related_modules.is_empty() {
        "当前专题页没有额外的关联模块线索。".to_string()
    } else {
        grouped_bullet_block("关联模块", &related_modules).unwrap_or_else(|| "- 无".to_string())
    }
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

fn append_supporting_blocks(title: &str, content: &str, context: &PageContext) -> String {
    let evidence_blocks = context
        .evidence_groups
        .iter()
        .filter(|group| group.section_title == title)
        .filter_map(render_evidence_group)
        .collect::<Vec<_>>();
    let diagram_blocks = context
        .diagram_inputs
        .iter()
        .filter(|diagram| diagram.section_title == title)
        .filter_map(render_diagram_block)
        .collect::<Vec<_>>();

    let mut blocks = Vec::new();
    if !content.trim().is_empty() {
        blocks.push(content.trim().to_string());
    }
    blocks.extend(evidence_blocks);
    blocks.extend(diagram_blocks);

    if blocks.is_empty() {
        content.to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn render_evidence_group(group: &crate::domain::context::PageEvidenceGroup) -> Option<String> {
    if group.items.is_empty() {
        return None;
    }

    let mut lines = vec![format!("**{}**", group.title)];
    if !group.summary.trim().is_empty() {
        lines.push(group.summary.trim().to_string());
    }
    lines.extend(group.items.iter().take(8).map(|item| {
        if item.note.trim().is_empty() {
            format!("- `{}`", item.path)
        } else {
            format!("- `{}`: {}", item.path, item.note.trim())
        }
    }));

    Some(lines.join("\n"))
}

fn render_diagram_block(diagram: &crate::domain::context::PageDiagramInput) -> Option<String> {
    if diagram.nodes.is_empty() || diagram.edges.is_empty() {
        return None;
    }

    let mut lines = vec![format!("**{}**", diagram.title)];
    if !diagram.summary.trim().is_empty() {
        lines.push(diagram.summary.trim().to_string());
    }
    lines.push("```mermaid".to_string());
    lines.push(match diagram.diagram_type.as_str() {
        "flow" => "flowchart LR".to_string(),
        _ => "graph LR".to_string(),
    });
    lines.extend(diagram.nodes.iter().map(|node| {
        format!(
            "    {}[\"{}\"]",
            sanitize_mermaid_id(&node.node_id),
            node.label
        )
    }));
    lines.extend(diagram.edges.iter().map(|edge| {
        let source = sanitize_mermaid_id(&edge.source);
        let target = sanitize_mermaid_id(&edge.target);
        if let Some(label) = &edge.label {
            format!("    {source} -->|{}| {target}", label)
        } else {
            format!("    {source} --> {target}")
        }
    }));
    lines.push("```".to_string());

    Some(lines.join("\n"))
}

fn sanitize_mermaid_id(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => character,
            _ => '_',
        })
        .collect()
}
