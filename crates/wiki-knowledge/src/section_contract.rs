use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct SectionContractSlot {
    pub key: &'static str,
    pub title: &'static str,
}

pub fn section_contract_slots_for_page_type(page_type: &str) -> Vec<SectionContractSlot> {
    section_slots_for_page_type(page_type)
}

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

fn section_slots_for_page_type(page_type: &str) -> Vec<SectionContractSlot> {
    match page_type {
        "overview" => vec![
            SectionContractSlot {
                key: "intro",
                title: "简介",
            },
            SectionContractSlot {
                key: "facts",
                title: "项目事实",
            },
            SectionContractSlot {
                key: "tech-stack",
                title: "技术栈",
            },
            SectionContractSlot {
                key: "entry-build",
                title: "入口与构建",
            },
            SectionContractSlot {
                key: "key-insights",
                title: "关键信息",
            },
        ],
        "architecture" => vec![
            SectionContractSlot {
                key: "overview",
                title: "架构概览",
            },
            SectionContractSlot {
                key: "module-structure",
                title: "模块结构",
            },
            SectionContractSlot {
                key: "cross-module-relations",
                title: "跨模块关系",
            },
            SectionContractSlot {
                key: "architecture-hints",
                title: "架构提示",
            },
        ],
        "module" => vec![
            SectionContractSlot {
                key: "toc",
                title: "目录",
            },
            SectionContractSlot {
                key: "module-intro",
                title: "简介",
            },
            SectionContractSlot {
                key: "child-overview",
                title: "项目结构",
            },
            SectionContractSlot {
                key: "key-sources",
                title: "核心组件",
            },
            SectionContractSlot {
                key: "module-architecture",
                title: "架构总览",
            },
            SectionContractSlot {
                key: "module-facts",
                title: "详细组件分析",
            },
            SectionContractSlot {
                key: "dependencies",
                title: "依赖关系分析",
            },
            SectionContractSlot {
                key: "module-performance",
                title: "性能考量",
            },
            SectionContractSlot {
                key: "module-troubleshooting",
                title: "故障排查指南",
            },
            SectionContractSlot {
                key: "module-conclusion",
                title: "结论",
            },
            SectionContractSlot {
                key: "module-appendix",
                title: "附录",
            },
        ],
        "workflow" => vec![
            SectionContractSlot {
                key: "workflow-overview",
                title: "工作流概述",
            },
            SectionContractSlot {
                key: "build-process",
                title: "构建流程",
            },
            SectionContractSlot {
                key: "ci-cd",
                title: "CI/CD 配置",
            },
            SectionContractSlot {
                key: "containerization",
                title: "容器化",
            },
        ],
        "topic" => vec![
            SectionContractSlot {
                key: "toc",
                title: "目录",
            },
            SectionContractSlot {
                key: "topic-intro",
                title: "简介",
            },
            SectionContractSlot {
                key: "topic-structure",
                title: "项目结构",
            },
            SectionContractSlot {
                key: "topic-evidence",
                title: "核心组件",
            },
            SectionContractSlot {
                key: "topic-diagram",
                title: "架构总览",
            },
            SectionContractSlot {
                key: "topic-related",
                title: "详细组件分析",
            },
            SectionContractSlot {
                key: "topic-dependencies",
                title: "依赖关系分析",
            },
            SectionContractSlot {
                key: "topic-performance",
                title: "性能考量",
            },
            SectionContractSlot {
                key: "topic-troubleshooting",
                title: "故障排查指南",
            },
            SectionContractSlot {
                key: "topic-conclusion",
                title: "结论",
            },
            SectionContractSlot {
                key: "topic-appendix",
                title: "附录",
            },
        ],
        "family-index" => vec![
            SectionContractSlot {
                key: "toc",
                title: "目录",
            },
            SectionContractSlot {
                key: "family-overview",
                title: "简介",
            },
            SectionContractSlot {
                key: "family-children",
                title: "项目结构",
            },
            SectionContractSlot {
                key: "family-scope",
                title: "核心组件",
            },
            SectionContractSlot {
                key: "family-architecture",
                title: "架构总览",
            },
            SectionContractSlot {
                key: "family-evidence",
                title: "详细组件分析",
            },
            SectionContractSlot {
                key: "family-related",
                title: "依赖关系分析",
            },
            SectionContractSlot {
                key: "family-performance",
                title: "性能考量",
            },
            SectionContractSlot {
                key: "family-troubleshooting",
                title: "故障排查指南",
            },
            SectionContractSlot {
                key: "family-conclusion",
                title: "结论",
            },
            SectionContractSlot {
                key: "family-appendix",
                title: "附录",
            },
        ],
        "family-child" => vec![
            SectionContractSlot {
                key: "toc",
                title: "目录",
            },
            SectionContractSlot {
                key: "family-child-intro",
                title: "简介",
            },
            SectionContractSlot {
                key: "family-child-structure",
                title: "项目结构",
            },
            SectionContractSlot {
                key: "family-child-surfaces",
                title: "核心组件",
            },
            SectionContractSlot {
                key: "family-child-architecture",
                title: "架构总览",
            },
            SectionContractSlot {
                key: "family-child-sources",
                title: "详细组件分析",
            },
            SectionContractSlot {
                key: "family-child-related",
                title: "依赖关系分析",
            },
            SectionContractSlot {
                key: "family-child-performance",
                title: "性能考量",
            },
            SectionContractSlot {
                key: "family-child-troubleshooting",
                title: "故障排查指南",
            },
            SectionContractSlot {
                key: "family-child-conclusion",
                title: "结论",
            },
            SectionContractSlot {
                key: "family-child-appendix",
                title: "附录",
            },
        ],
        "family-leaf-doc" => vec![
            SectionContractSlot {
                key: "toc",
                title: "目录",
            },
            SectionContractSlot {
                key: "family-leaf-intro",
                title: "简介",
            },
            SectionContractSlot {
                key: "family-leaf-structure",
                title: "项目结构",
            },
            SectionContractSlot {
                key: "family-leaf-surfaces",
                title: "核心组件",
            },
            SectionContractSlot {
                key: "family-leaf-architecture",
                title: "架构总览",
            },
            SectionContractSlot {
                key: "family-leaf-sources",
                title: "详细组件分析",
            },
            SectionContractSlot {
                key: "family-leaf-related",
                title: "依赖关系分析",
            },
            SectionContractSlot {
                key: "family-leaf-performance",
                title: "性能考量",
            },
            SectionContractSlot {
                key: "family-leaf-troubleshooting",
                title: "故障排查指南",
            },
            SectionContractSlot {
                key: "family-leaf-conclusion",
                title: "结论",
            },
            SectionContractSlot {
                key: "family-leaf-appendix",
                title: "附录",
            },
        ],
        _ => vec![SectionContractSlot {
            key: "intro",
            title: "简介",
        }],
    }
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


