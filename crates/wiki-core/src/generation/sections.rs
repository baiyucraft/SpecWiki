//! section 生成层负责把页面上下文拆成稳定章节草稿。
//! 它不做写盘，只负责为 renderer、state 和 cache 提供可复用的 section 粒度产物。

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::domain::context::{
    PageComposePlan, PageComposeSection, PageContext, PageResearchSectionPlan,
};
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

#[derive(Debug, Clone, Copy)]
struct SectionSlot {
    key: &'static str,
    title: &'static str,
}

/// 返回某类页面的稳定 section 标题模板。
///
/// # 参数
/// - `page_type`：页面类型，如 `overview / architecture / family-index / family-child / family-leaf-doc / module / workflow`。
///
/// # 返回
/// - 返回该页面类型对应的稳定 section 标题顺序。
pub fn section_titles_for_page_type(page_type: &str) -> Vec<&'static str> {
    section_slots_for_page_type(page_type)
        .into_iter()
        .map(|slot| slot.title)
        .collect()
}

pub fn section_key_for_title(page_type: &str, title: &str) -> String {
    let normalized_title = normalize_requested_section_title(page_type, title);
    section_slots_for_page_type(page_type)
        .into_iter()
        .find(|slot| slot.title == normalized_title)
        .map(|slot| slot.key.to_string())
        .unwrap_or_else(|| slug_key(title))
}

pub fn section_title_for_key(page_type: &str, section_key: &str) -> Option<&'static str> {
    section_slots_for_page_type(page_type)
        .into_iter()
        .find(|slot| slot.key == section_key)
        .map(|slot| slot.title)
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

pub fn section_id_for_key(page_id: &str, section_key: &str) -> String {
    stable_id("section", &format!("{page_id}:{section_key}"))
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
    let compose_plan = build_page_compose_plan(page, context);
    build_section_drafts_from_compose_plan(page, context, &compose_plan)
}

/// 在 renderer 前显式生成 compose 计划。
pub fn build_page_compose_plan(page: &PlannedPage, context: &PageContext) -> PageComposePlan {
    let templates = section_templates_for_page(page.page_type.as_str(), context);
    let sections = ordered_section_templates(page.page_type.as_str(), &templates, context)
        .into_iter()
        .map(|(section_key, title, content, plan)| {
            let content =
                append_supporting_blocks(&section_key, &title, &content, context, plan.as_ref());
            PageComposeSection {
                section_key,
                section_title: title,
                content,
                evidence_refs: plan
                    .as_ref()
                    .map(|plan| plan.evidence_refs.clone())
                    .unwrap_or_default(),
                diagram_refs: plan
                    .as_ref()
                    .map(|plan| plan.diagram_refs.clone())
                    .unwrap_or_default(),
                child_refs: plan
                    .as_ref()
                    .map(|plan| plan.child_refs.clone())
                    .unwrap_or_default(),
            }
        })
        .collect();

    PageComposePlan {
        page_positioning: String::new(),
        summary: String::new(),
        sections,
    }
}

/// 把 compose 计划转成稳定的 section 草稿。
pub fn build_section_drafts_from_compose_plan(
    page: &PlannedPage,
    context: &PageContext,
    compose_plan: &PageComposePlan,
) -> Vec<SectionDraft> {
    compose_plan
        .sections
        .iter()
        .map(|section| SectionDraft {
            section_id: section_id_for_key(&page.id, &section.section_key),
            title: section.section_title.clone(),
            managed: true,
            source_ids: context.source_ids.clone(),
            relation_ids: context.relation_ids.clone(),
            content: section.content.clone(),
        })
        .collect()
}

fn section_slots_for_page_type(page_type: &str) -> Vec<SectionSlot> {
    match page_type {
        "overview" => vec![
            SectionSlot {
                key: "intro",
                title: "简介",
            },
            SectionSlot {
                key: "facts",
                title: "项目事实",
            },
            SectionSlot {
                key: "tech-stack",
                title: "技术栈",
            },
            SectionSlot {
                key: "entry-build",
                title: "入口与构建",
            },
            SectionSlot {
                key: "key-insights",
                title: "关键信息",
            },
        ],
        "architecture" => vec![
            SectionSlot {
                key: "overview",
                title: "架构概览",
            },
            SectionSlot {
                key: "module-structure",
                title: "模块结构",
            },
            SectionSlot {
                key: "cross-module-relations",
                title: "跨模块关系",
            },
            SectionSlot {
                key: "architecture-hints",
                title: "架构提示",
            },
        ],
        "module" => vec![
            SectionSlot {
                key: "toc",
                title: "目录",
            },
            SectionSlot {
                key: "module-intro",
                title: "简介",
            },
            SectionSlot {
                key: "child-overview",
                title: "项目结构",
            },
            SectionSlot {
                key: "key-sources",
                title: "核心组件",
            },
            SectionSlot {
                key: "module-architecture",
                title: "架构总览",
            },
            SectionSlot {
                key: "module-facts",
                title: "详细组件分析",
            },
            SectionSlot {
                key: "dependencies",
                title: "依赖关系分析",
            },
            SectionSlot {
                key: "module-performance",
                title: "性能考量",
            },
            SectionSlot {
                key: "module-troubleshooting",
                title: "故障排查指南",
            },
            SectionSlot {
                key: "module-conclusion",
                title: "结论",
            },
            SectionSlot {
                key: "module-appendix",
                title: "附录",
            },
        ],
        "workflow" => vec![
            SectionSlot {
                key: "workflow-overview",
                title: "工作流概述",
            },
            SectionSlot {
                key: "build-process",
                title: "构建流程",
            },
            SectionSlot {
                key: "ci-cd",
                title: "CI/CD 配置",
            },
            SectionSlot {
                key: "containerization",
                title: "容器化",
            },
        ],
        "topic" => vec![
            SectionSlot {
                key: "toc",
                title: "目录",
            },
            SectionSlot {
                key: "topic-intro",
                title: "简介",
            },
            SectionSlot {
                key: "topic-structure",
                title: "项目结构",
            },
            SectionSlot {
                key: "topic-evidence",
                title: "核心组件",
            },
            SectionSlot {
                key: "topic-diagram",
                title: "架构总览",
            },
            SectionSlot {
                key: "topic-related",
                title: "详细组件分析",
            },
            SectionSlot {
                key: "topic-dependencies",
                title: "依赖关系分析",
            },
            SectionSlot {
                key: "topic-performance",
                title: "性能考量",
            },
            SectionSlot {
                key: "topic-troubleshooting",
                title: "故障排查指南",
            },
            SectionSlot {
                key: "topic-conclusion",
                title: "结论",
            },
            SectionSlot {
                key: "topic-appendix",
                title: "附录",
            },
        ],
        "family-index" => vec![
            SectionSlot {
                key: "toc",
                title: "目录",
            },
            SectionSlot {
                key: "family-overview",
                title: "简介",
            },
            SectionSlot {
                key: "family-children",
                title: "项目结构",
            },
            SectionSlot {
                key: "family-scope",
                title: "核心组件",
            },
            SectionSlot {
                key: "family-architecture",
                title: "架构总览",
            },
            SectionSlot {
                key: "family-evidence",
                title: "详细组件分析",
            },
            SectionSlot {
                key: "family-related",
                title: "依赖关系分析",
            },
            SectionSlot {
                key: "family-performance",
                title: "性能考量",
            },
            SectionSlot {
                key: "family-troubleshooting",
                title: "故障排查指南",
            },
            SectionSlot {
                key: "family-conclusion",
                title: "结论",
            },
            SectionSlot {
                key: "family-appendix",
                title: "附录",
            },
        ],
        "family-child" => vec![
            SectionSlot {
                key: "toc",
                title: "目录",
            },
            SectionSlot {
                key: "family-child-intro",
                title: "简介",
            },
            SectionSlot {
                key: "family-child-structure",
                title: "项目结构",
            },
            SectionSlot {
                key: "family-child-surfaces",
                title: "核心组件",
            },
            SectionSlot {
                key: "family-child-architecture",
                title: "架构总览",
            },
            SectionSlot {
                key: "family-child-sources",
                title: "详细组件分析",
            },
            SectionSlot {
                key: "family-child-related",
                title: "依赖关系分析",
            },
            SectionSlot {
                key: "family-child-performance",
                title: "性能考量",
            },
            SectionSlot {
                key: "family-child-troubleshooting",
                title: "故障排查指南",
            },
            SectionSlot {
                key: "family-child-conclusion",
                title: "结论",
            },
            SectionSlot {
                key: "family-child-appendix",
                title: "附录",
            },
        ],
        "family-leaf-doc" => vec![
            SectionSlot {
                key: "toc",
                title: "目录",
            },
            SectionSlot {
                key: "family-leaf-intro",
                title: "简介",
            },
            SectionSlot {
                key: "family-leaf-structure",
                title: "项目结构",
            },
            SectionSlot {
                key: "family-leaf-surfaces",
                title: "核心组件",
            },
            SectionSlot {
                key: "family-leaf-architecture",
                title: "架构总览",
            },
            SectionSlot {
                key: "family-leaf-sources",
                title: "详细组件分析",
            },
            SectionSlot {
                key: "family-leaf-related",
                title: "依赖关系分析",
            },
            SectionSlot {
                key: "family-leaf-performance",
                title: "性能考量",
            },
            SectionSlot {
                key: "family-leaf-troubleshooting",
                title: "故障排查指南",
            },
            SectionSlot {
                key: "family-leaf-conclusion",
                title: "结论",
            },
            SectionSlot {
                key: "family-leaf-appendix",
                title: "附录",
            },
        ],
        _ => vec![SectionSlot {
            key: "intro",
            title: "简介",
        }],
    }
}

fn section_templates_for_page(
    page_type: &str,
    context: &PageContext,
) -> Vec<(String, String, String)> {
    match page_type {
        "overview" => overview_section_templates(context),
        "architecture" => architecture_section_templates(context),
        "module" => module_section_templates(context),
        "workflow" => workflow_section_templates(context),
        "topic" => topic_section_templates(context),
        "family-index" => family_index_section_templates(context),
        "family-child" => family_child_section_templates(context),
        "family-leaf-doc" => family_leaf_section_templates(context),
        _ => vec![(
            "intro".to_string(),
            "简介".to_string(),
            "由 codebuddy-wiki 自动生成。".to_string(),
        )],
    }
}

fn reference_outline_templates(
    sections: Vec<(&str, &str, String)>,
) -> Vec<(String, String, String)> {
    let toc_titles = sections
        .iter()
        .filter_map(|(_, title, _)| {
            let title = title.trim();
            (!title.is_empty() && title != "目录").then_some(title.to_string())
        })
        .collect::<Vec<_>>();

    sections
        .into_iter()
        .map(|(key, title, content)| {
            let content = if title == "目录" && content.trim().is_empty() {
                render_static_outline_toc(&toc_titles)
            } else {
                content
            };
            (key.to_string(), title.to_string(), content)
        })
        .collect()
}

fn render_static_outline_toc(titles: &[String]) -> String {
    titles
        .iter()
        .enumerate()
        .map(|(index, title)| format!("{}. [{}](#{title})", index + 1, title))
        .collect::<Vec<_>>()
        .join("\n")
}

fn overview_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    vec![
        (
            "intro".to_string(),
            "简介".to_string(),
            overview_intro(context),
        ),
        (
            "facts".to_string(),
            "项目事实".to_string(),
            overview_project_facts(context),
        ),
        (
            "tech-stack".to_string(),
            "技术栈".to_string(),
            tech_stack_section(context),
        ),
        (
            "entry-build".to_string(),
            "入口与构建".to_string(),
            entry_and_build_section(context),
        ),
        (
            "key-insights".to_string(),
            "关键信息".to_string(),
            overview_key_insights(context),
        ),
    ]
}

fn architecture_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    vec![
        (
            "overview".to_string(),
            "架构概览".to_string(),
            architecture_overview(context),
        ),
        (
            "module-structure".to_string(),
            "模块结构".to_string(),
            architecture_module_structure(context),
        ),
        (
            "cross-module-relations".to_string(),
            "跨模块关系".to_string(),
            relation_section(context),
        ),
        (
            "architecture-hints".to_string(),
            "架构提示".to_string(),
            architecture_hints_section(context),
        ),
    ]
}

fn module_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    reference_outline_templates(vec![
        ("toc", "目录", String::new()),
        ("module-intro", "简介", module_intro(context)),
        (
            "child-overview",
            "项目结构",
            module_structure_section(context),
        ),
        ("key-sources", "核心组件", source_section(context)),
        (
            "module-architecture",
            "架构总览",
            module_architecture_section(context),
        ),
        ("module-facts", "详细组件分析", module_fact_section(context)),
        ("dependencies", "依赖关系分析", dependency_section(context)),
        (
            "module-performance",
            "性能考量",
            performance_considerations_section(context),
        ),
        (
            "module-troubleshooting",
            "故障排查指南",
            troubleshooting_section(context),
        ),
        (
            "module-conclusion",
            "结论",
            module_conclusion_section(context),
        ),
        ("module-appendix", "附录", appendix_section(context)),
    ])
}

fn workflow_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    vec![
        (
            "workflow-overview".to_string(),
            "工作流概述".to_string(),
            workflow_overview(context),
        ),
        (
            "build-process".to_string(),
            "构建流程".to_string(),
            prefixed_bullets(&context.facts, "构建", "当前未检测到稳定的构建流程线索。"),
        ),
        (
            "ci-cd".to_string(),
            "CI/CD 配置".to_string(),
            prefixed_bullets(&context.facts, "CI", "当前未检测到 CI/CD 配置。"),
        ),
        (
            "containerization".to_string(),
            "容器化".to_string(),
            prefixed_bullets(&context.facts, "容器", "当前未检测到容器化配置。"),
        ),
    ]
}

fn topic_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    reference_outline_templates(vec![
        ("toc", "目录", String::new()),
        ("topic-intro", "简介", topic_intro(context)),
        (
            "topic-structure",
            "项目结构",
            topic_structure_section(context),
        ),
        (
            "topic-evidence",
            "核心组件",
            topic_evidence_section(context),
        ),
        (
            "topic-diagram",
            "架构总览",
            topic_architecture_section(context),
        ),
        (
            "topic-related",
            "详细组件分析",
            topic_detailed_analysis_section(context),
        ),
        (
            "topic-dependencies",
            "依赖关系分析",
            topic_related_section(context),
        ),
        (
            "topic-performance",
            "性能考量",
            performance_considerations_section(context),
        ),
        (
            "topic-troubleshooting",
            "故障排查指南",
            troubleshooting_section(context),
        ),
        (
            "topic-conclusion",
            "结论",
            topic_conclusion_section(context),
        ),
        ("topic-appendix", "附录", appendix_section(context)),
    ])
}

fn family_index_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    reference_outline_templates(vec![
        ("toc", "目录", String::new()),
        ("family-overview", "简介", family_index_intro(context)),
        (
            "family-children",
            "项目结构",
            family_children_section(context),
        ),
        ("family-scope", "核心组件", family_scope_section(context)),
        (
            "family-architecture",
            "架构总览",
            family_architecture_section(context),
        ),
        (
            "family-evidence",
            "详细组件分析",
            family_evidence_section(context),
        ),
        (
            "family-related",
            "依赖关系分析",
            family_related_section(context),
        ),
        (
            "family-performance",
            "性能考量",
            performance_considerations_section(context),
        ),
        (
            "family-troubleshooting",
            "故障排查指南",
            troubleshooting_section(context),
        ),
        (
            "family-conclusion",
            "结论",
            family_conclusion_section(context),
        ),
        ("family-appendix", "附录", appendix_section(context)),
    ])
}

fn family_child_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    reference_outline_templates(vec![
        ("toc", "目录", String::new()),
        ("family-child-intro", "简介", family_child_intro(context)),
        (
            "family-child-structure",
            "项目结构",
            family_children_section(context),
        ),
        (
            "family-child-surfaces",
            "核心组件",
            family_scope_section(context),
        ),
        (
            "family-child-architecture",
            "架构总览",
            family_architecture_section(context),
        ),
        (
            "family-child-sources",
            "详细组件分析",
            family_sources_section(context),
        ),
        (
            "family-child-related",
            "依赖关系分析",
            family_related_section(context),
        ),
        (
            "family-child-performance",
            "性能考量",
            performance_considerations_section(context),
        ),
        (
            "family-child-troubleshooting",
            "故障排查指南",
            troubleshooting_section(context),
        ),
        (
            "family-child-conclusion",
            "结论",
            family_conclusion_section(context),
        ),
        ("family-child-appendix", "附录", appendix_section(context)),
    ])
}

fn family_leaf_section_templates(context: &PageContext) -> Vec<(String, String, String)> {
    reference_outline_templates(vec![
        ("toc", "目录", String::new()),
        ("family-leaf-intro", "简介", family_leaf_intro(context)),
        (
            "family-leaf-structure",
            "项目结构",
            family_children_section(context),
        ),
        (
            "family-leaf-surfaces",
            "核心组件",
            family_leaf_surface_section(context),
        ),
        (
            "family-leaf-architecture",
            "架构总览",
            family_architecture_section(context),
        ),
        (
            "family-leaf-sources",
            "详细组件分析",
            family_sources_section(context),
        ),
        (
            "family-leaf-related",
            "依赖关系分析",
            family_leaf_related_section(context),
        ),
        (
            "family-leaf-performance",
            "性能考量",
            performance_considerations_section(context),
        ),
        (
            "family-leaf-troubleshooting",
            "故障排查指南",
            troubleshooting_section(context),
        ),
        (
            "family-leaf-conclusion",
            "结论",
            family_conclusion_section(context),
        ),
        ("family-leaf-appendix", "附录", appendix_section(context)),
    ])
}

fn ordered_section_templates(
    _page_type: &str,
    templates: &[(String, String, String)],
    _context: &PageContext,
) -> Vec<(String, String, String, Option<PageResearchSectionPlan>)> {
    templates
        .iter()
        .cloned()
        .map(|(key, title, content)| (key, title, content, None))
        .collect()
}

fn slug_key(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => character,
            ' ' | '/' | '\\' | ':' => '-',
            _ if character.is_ascii_alphanumeric() => character,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_ascii_lowercase()
}

fn normalize_requested_section_title<'a>(page_type: &str, title: &'a str) -> Cow<'a, str> {
    let trimmed = title.trim();
    let normalized = match (page_type, trimmed) {
        ("module", "模块说明") => Some("简介"),
        ("module", "关键源码") => Some("核心组件"),
        ("module", "依赖关系") => Some("依赖关系分析"),
        ("module", "模块事实") => Some("详细组件分析"),
        ("module", "子模块概述") => Some("项目结构"),
        ("topic", "主题说明") => Some("简介"),
        ("topic", "关键证据") => Some("核心组件"),
        ("topic", "结构图") => Some("架构总览"),
        ("topic", "关联模块") => Some("依赖关系分析"),
        ("family-index", "知识域概览") => Some("简介"),
        ("family-index", "Docs / API / 配置面") => Some("核心组件"),
        ("family-index", "子页结构") => Some("项目结构"),
        ("family-index", "关键来源") => Some("详细组件分析"),
        ("family-child", "主题定位") => Some("简介"),
        ("family-child", "API / 配置面") => Some("核心组件"),
        ("family-child", "关键源码") => Some("详细组件分析"),
        ("family-child", "关联结果") => Some("依赖关系分析"),
        ("family-leaf-doc", "叶子主题") => Some("简介"),
        ("family-leaf-doc", "命中面") => Some("核心组件"),
        ("family-leaf-doc", "关键来源") => Some("详细组件分析"),
        ("family-leaf-doc", "上游与关联结果") => Some("依赖关系分析"),
        _ => None,
    };

    normalized
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Borrowed(trimmed))
}

fn canonical_outline_title(title: &str) -> &str {
    match title.trim() {
        "模块说明" | "主题说明" | "知识域概览" | "主题定位" | "叶子主题" | "简介" => {
            "简介"
        }
        "子模块概述" | "子页结构" | "项目结构" | "知识域结构" => "项目结构",
        "关键源码"
        | "关键证据"
        | "Docs / API / 配置面"
        | "API / 配置面"
        | "命中面"
        | "核心组件" => "核心组件",
        "结构图" | "架构概览" | "架构总览" => "架构总览",
        "模块事实" | "关键来源" | "家族来源" | "叶子来源" | "详细组件分析" => {
            "详细组件分析"
        }
        "依赖关系" | "关联模块" | "关联结果" | "上游与关联结果" | "依赖关系分析" => {
            "依赖关系分析"
        }
        "性能考量" => "性能考量",
        "故障排查指南" => "故障排查指南",
        "结论" => "结论",
        "附录" => "附录",
        "目录" => "目录",
        _ => title.trim(),
    }
}

fn section_titles_equivalent(left: &str, right: &str) -> bool {
    let left = left.trim();
    let right = right.trim();
    !left.is_empty()
        && !right.is_empty()
        && (left == right || canonical_outline_title(left) == canonical_outline_title(right))
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
    if let Some(intro) = research_intro(context) {
        return intro;
    }

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
    if let Some(intro) = research_intro(context) {
        return intro;
    }

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
    if let Some(intro) = research_intro(context) {
        return intro;
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

fn module_structure_section(context: &PageContext) -> String {
    let mut lines = Vec::new();
    if let Some(module_root) = fact_value(&context.facts, "模块根路径") {
        lines.push(format!("模块根路径：`{module_root}`"));
    }
    if let Some(entry) = fact_value(&context.facts, "入口文件").filter(|entry| *entry != "无")
    {
        lines.push(format!("入口文件：`{entry}`"));
    }
    if let Some(source_count) = fact_value(&context.facts, "源文件数") {
        lines.push(format!("源文件规模：{source_count}"));
    }
    lines.extend(
        prefixed_values(&context.summary_inputs, "子模块")
            .into_iter()
            .map(|line| format!("子模块：{line}")),
    );

    if lines.is_empty() {
        "当前没有更多可稳定归纳的模块结构线索。".to_string()
    } else {
        bullet_lines(&lines)
    }
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

fn module_architecture_section(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "模块角色",
        &prefixed_values(&context.summary_inputs, "模块角色"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "协作边界",
        &prefixed_values(&context.summary_inputs, "依赖"),
    ) {
        blocks.push(block);
    }

    if blocks.is_empty() {
        "当前未识别出稳定的模块架构边界，可结合关键源码继续展开。".to_string()
    } else {
        blocks.join("\n\n")
    }
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

    blocks.join(
        "

",
    )
}

fn module_conclusion_section(context: &PageContext) -> String {
    let module_name = fact_value(&context.facts, "模块名称").unwrap_or("当前模块");
    let roles = prefixed_values(&context.summary_inputs, "模块角色");
    if roles.is_empty() {
        format!("`{module_name}` 当前主要作为独立模块承接仓库中的一部分核心职责。")
    } else {
        format!(
            "`{module_name}` 当前主要承担 {}，并通过上面的结构和依赖关系与其余模块协作。",
            roles.join("、")
        )
    }
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
    if let Some(intro) = research_intro(context) {
        return intro;
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
    let source_paths = prefixed_values(&context.summary_inputs, "关键源码");
    if source_paths.is_empty() {
        "当前主题还没有额外的关键源码摘要。".to_string()
    } else {
        grouped_bullet_block("关键源码", &source_paths).unwrap_or_else(|| "- 无".to_string())
    }
}

fn topic_structure_section(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "关键实现落点",
        &prefixed_values(&context.summary_inputs, "关键源码"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "关联模块",
        &prefixed_values(&context.summary_inputs, "关联模块"),
    ) {
        blocks.push(block);
    }

    if blocks.is_empty() {
        "当前专题还没有稳定的结构落点可供展开。".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn topic_diagram_section(context: &PageContext) -> String {
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

fn topic_architecture_section(context: &PageContext) -> String {
    let mut blocks = vec![topic_diagram_section(context)];
    let related_modules = prefixed_values(&context.summary_inputs, "关联模块");
    if !related_modules.is_empty() {
        blocks.push(format!(
            "当前主题主要与 {} 保持直接协作。",
            related_modules.join("、")
        ));
    }
    blocks.join("\n\n")
}

fn topic_related_section(context: &PageContext) -> String {
    let related_modules = prefixed_values(&context.summary_inputs, "关联模块");
    if related_modules.is_empty() {
        "当前专题页没有额外的关联模块线索。".to_string()
    } else {
        grouped_bullet_block("关联模块", &related_modules).unwrap_or_else(|| "- 无".to_string())
    }
}

fn topic_detailed_analysis_section(context: &PageContext) -> String {
    let mut blocks = vec![topic_evidence_section(context)];
    let topic_summary = prefixed_values(&context.summary_inputs, "主题摘要");
    if let Some(summary) = topic_summary.first() {
        blocks.push(summary.clone());
    }
    blocks.join("\n\n")
}

fn topic_conclusion_section(context: &PageContext) -> String {
    let topic_title = fact_value(&context.facts, "主题标题").unwrap_or("当前主题");
    let related_modules = prefixed_values(&context.summary_inputs, "关联模块");
    if related_modules.is_empty() {
        format!("`{topic_title}` 已形成可独立阅读的专题单元，可继续结合核心组件章节向下追踪。")
    } else {
        format!(
            "`{topic_title}` 当前与 {} 形成最直接的协作边界，适合作为后续深入阅读的入口。",
            related_modules.join("、")
        )
    }
}

fn family_index_intro(context: &PageContext) -> String {
    if let Some(intro) = research_intro(context) {
        return intro;
    }

    let family_title = fact_value(&context.facts, "知识域标题").unwrap_or("当前知识域");
    let family_kind = fact_value(&context.facts, "知识域类别").unwrap_or("family");
    let source_count = fact_value(&context.facts, "关联源码数").unwrap_or("0");
    let summary = prefixed_values(&context.summary_inputs, "知识域摘要");

    let mut sentences = vec![format!(
        "`{family_title}` 是一个 {family_kind} 类型的知识域索引页。"
    )];
    if let Some(summary) = summary.first() {
        sentences.push(summary.clone());
    }
    sentences.push(format!(
        "当前索引页直接聚合 {source_count} 份源码、文档或配置线索。"
    ));
    paragraph_lines(&sentences, "该页面用于汇总某个稳定知识域。")
}

fn family_scope_section(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "Docs Anchor",
        &prefixed_values(&context.summary_inputs, "Docs Anchor"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "Public API",
        &prefixed_values(&context.summary_inputs, "Public API"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "Config Surface",
        &prefixed_values(&context.summary_inputs, "Config Surface"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "Type Surface",
        &prefixed_values(&context.summary_inputs, "Type Surface"),
    ) {
        blocks.push(block);
    }

    if blocks.is_empty() {
        "当前知识域尚未命中稳定的 docs / API / 配置面。".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn family_children_section(context: &PageContext) -> String {
    if context.child_summaries.is_empty() {
        "当前知识域尚未拆出稳定子页。".to_string()
    } else {
        bullet_lines(&context.child_summaries)
    }
}

fn family_evidence_section(context: &PageContext) -> String {
    family_sources_section(context)
}

fn family_child_intro(context: &PageContext) -> String {
    if let Some(intro) = research_intro(context) {
        return intro;
    }

    let family_title = fact_value(&context.facts, "知识域标题").unwrap_or("当前知识域子页");
    let family_kind = fact_value(&context.facts, "知识域类别").unwrap_or("family");
    let summary = prefixed_values(&context.summary_inputs, "知识域摘要");
    let mut sentences = vec![format!(
        "`{family_title}` 聚焦一个 {family_kind} 主题子域。"
    )];
    if let Some(summary) = summary.first() {
        sentences.push(summary.clone());
    }
    paragraph_lines(&sentences, "该页面用于解释知识域中的单个高信号子主题。")
}

fn family_sources_section(context: &PageContext) -> String {
    let appendix_lines = appendix_reference_lines(context);
    if appendix_lines.is_empty() {
        "当前知识域子页尚未命中高信号源码。".to_string()
    } else {
        bullet_lines(&appendix_lines)
    }
}

fn family_related_section(context: &PageContext) -> String {
    if context.child_summaries.is_empty() {
        "当前知识域子页没有更多关联结果。".to_string()
    } else {
        format!(
            "当前知识域主要通过以下子页或子单元形成协作边界：\n{}",
            bullet_lines(&context.child_summaries)
        )
    }
}

fn family_leaf_intro(context: &PageContext) -> String {
    if let Some(intro) = research_intro(context) {
        return intro;
    }

    let family_title = fact_value(&context.facts, "知识域标题").unwrap_or("当前叶子主题");
    let family_kind = fact_value(&context.facts, "知识域类别").unwrap_or("family");
    let summary = prefixed_values(&context.summary_inputs, "知识域摘要");
    let mut sentences = vec![format!(
        "`{family_title}` 是 `{family_kind}` 知识域下的叶子文档页。"
    )];
    if let Some(summary) = summary.first() {
        sentences.push(summary.clone());
    }
    sentences
        .push("该页优先承接一手 docs / API / 配置 / 类型入口，而不是继续充当索引页。".to_string());
    paragraph_lines(&sentences, "该页面聚焦一个更细粒度的叶子主题。")
}

fn family_leaf_surface_section(_context: &PageContext) -> String {
    "当前叶子主题尚未命中稳定的 docs / API / 配置 / 类型面。".to_string()
}

fn family_leaf_related_section(_context: &PageContext) -> String {
    "当前叶子主题没有更多上游或关联结果。".to_string()
}

fn family_architecture_section(context: &PageContext) -> String {
    let child_count = context.child_summaries.len();
    let surface_count = prefixed_values(&context.summary_inputs, "Docs Anchor").len()
        + prefixed_values(&context.summary_inputs, "Public API").len()
        + prefixed_values(&context.summary_inputs, "Config Surface").len()
        + prefixed_values(&context.summary_inputs, "Type Surface").len();

    match (child_count, surface_count) {
        (0, 0) => "当前知识域尚未沉淀出稳定的骨架信号。".to_string(),
        (0, _) => format!("当前知识域主要围绕 {surface_count} 类高信号 surface 组织内容。"),
        (_, 0) => format!("当前知识域已拆出 {child_count} 个稳定子单元，可继续按页面树向下展开。"),
        _ => format!(
            "当前知识域同时聚合了 {surface_count} 组高信号 surface，并拆出 {child_count} 个稳定子单元承接细节。"
        ),
    }
}

fn family_conclusion_section(context: &PageContext) -> String {
    let family_title = fact_value(&context.facts, "知识域标题").unwrap_or("当前知识域");
    if context.child_summaries.is_empty() {
        format!(
            "`{family_title}` 当前更适合作为单页知识入口，后续可继续沿 docs 或 API surface 深挖。"
        )
    } else {
        format!(
            "`{family_title}` 当前已经形成稳定的知识域页树，推荐优先沿项目结构和详细组件分析两条路径继续阅读。"
        )
    }
}

fn performance_considerations_section(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "关键调用热点",
        &prefixed_values(&context.summary_inputs, "图热点"),
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
        "当前未识别出足以单独展开的性能热点，后续可优先检查调用热点、配置面和构建链。".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn troubleshooting_section(context: &PageContext) -> String {
    let mut blocks = Vec::new();
    if let Some(block) = grouped_bullet_block(
        "循环提示",
        &prefixed_values(&context.summary_inputs, "循环"),
    ) {
        blocks.push(block);
    }
    if let Some(block) = grouped_bullet_block(
        "相关文档入口",
        &prefixed_values(&context.summary_inputs, "Docs Anchor"),
    ) {
        blocks.push(block);
    }

    if blocks.is_empty() {
        "当前未沉淀出专门的排障线索；建议从简介、项目结构和附录中的源码入口开始定位。".to_string()
    } else {
        blocks.join("\n\n")
    }
}

fn appendix_section(context: &PageContext) -> String {
    let appendix_lines = appendix_reference_lines(context);
    if appendix_lines.is_empty() {
        "补充引用和扩展材料会在最终渲染阶段继续汇总到本节。".to_string()
    } else {
        bullet_lines(&appendix_lines)
    }
}

fn appendix_reference_lines(context: &PageContext) -> Vec<String> {
    [
        "源码",
        "关键源码",
        "Docs Anchor",
        "Public API",
        "Config Surface",
        "Type Surface",
        "关联模块",
    ]
    .into_iter()
    .flat_map(|prefix| prefixed_values(&context.summary_inputs, prefix))
    .take(10)
    .collect()
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

fn research_intro(_context: &PageContext) -> Option<String> {
    None
}

fn append_supporting_blocks(
    section_key: &str,
    title: &str,
    content: &str,
    context: &PageContext,
    plan: Option<&PageResearchSectionPlan>,
) -> String {
    let planned_evidence_refs = plan
        .map(|plan| {
            plan.evidence_refs
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let planned_diagram_refs = plan
        .map(|plan| {
            plan.diagram_refs
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let _planned_child_refs = plan
        .map(|plan| {
            plan.child_refs
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let evidence_blocks = context
        .evidence_groups
        .iter()
        .filter(|group| {
            if !planned_evidence_refs.is_empty() {
                return planned_evidence_refs.contains(&group.group_id);
            }
            section_titles_equivalent(&group.section_title, title)
                || group.items.iter().any(|item| {
                    item.section_refs
                        .iter()
                        .any(|reference| reference == section_key)
                })
        })
        .filter_map(render_evidence_group)
        .collect::<Vec<_>>();
    let diagram_blocks = context
        .diagram_inputs
        .iter()
        .filter(|diagram| {
            if !planned_diagram_refs.is_empty() {
                return planned_diagram_refs.contains(&diagram.diagram_id);
            }
            section_titles_equivalent(&diagram.section_title, title)
        })
        .filter_map(render_diagram_block)
        .collect::<Vec<_>>();
    let child_block: Option<String> = None;

    let mut blocks = Vec::new();
    if !content.trim().is_empty() {
        blocks.push(content.trim().to_string());
    }
    if !evidence_blocks.is_empty() {
        blocks.push(format!("章节来源：\n{}", evidence_blocks.join("\n\n")));
    }
    blocks.extend(diagram_blocks);
    if let Some(child_block) = child_block {
        blocks.push(child_block);
    }

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
        let span = if item.start_line > 0 && item.end_line >= item.start_line {
            format!(":{}-{}", item.start_line, item.end_line)
        } else {
            String::new()
        };
        let coarse = if item.coarse_span { " (coarse)" } else { "" };
        if item.note.trim().is_empty() {
            format!("- `{}`{}{coarse}", item.path, span)
        } else {
            format!("- `{}`{}{coarse}: {}", item.path, span, item.note.trim())
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
