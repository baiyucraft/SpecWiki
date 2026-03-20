use std::collections::BTreeMap;

use crate::domain::compose::{ComposeSectionDraft, DiagramDraft, PageDraft};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use crate::domain::research::{
    DiagramSuggestion, DomainResearch, EvidenceCluster, PageDigest, PlannedSection, SourceCitation,
    SystemResearch, UnitResearch,
};
use crate::domain::stable_id::stable_id;

/// 最低 citation 密度——整页不低于此数。
const MIN_PAGE_CITATIONS: usize = 14;
/// 最低 section citation 密度。
const MIN_SECTION_CITATIONS: usize = 3;
/// 子页 digest 最多向父页传播的 citation 数量。
const MAX_DIGEST_CITATIONS: usize = 12;
/// fallback 图表最多展开的 digest 节点数。
const MAX_DIGEST_DIAGRAM_NODES: usize = 8;

// ─── Leaf pages ─────────────────────────────────────────────

/// 叶子页面 compose：消费 UnitResearch，按 section_plan 展开正文。
pub fn compose_leaf_page(unit: &KnowledgeUnit, research: &UnitResearch) -> (PageDraft, PageDigest) {
    let page_id = stable_id("page", &unit.relative_path);
    let mut sections = Vec::new();
    let mut total_citations = 0;

    for planned in &research.section_plan {
        let (section, citation_count) = compose_section(planned, &research.evidence_clusters, &[]);
        if should_keep_section(&section) {
            total_citations += citation_count;
            sections.push(section);
        }
    }

    // Citation 密度保证
    if total_citations < MIN_PAGE_CITATIONS {
        let deficit = MIN_PAGE_CITATIONS - total_citations;
        let existing_citations = sections
            .iter()
            .flat_map(|section| section.citations.iter().cloned())
            .collect::<Vec<_>>();
        let extra_citations =
            gather_missing_citations(&research.evidence_clusters, &existing_citations, deficit);
        if !extra_citations.is_empty() {
            let extra_section = ComposeSectionDraft {
                section_key: "source-references".to_string(),
                title: "源码参考".to_string(),
                content: format_citation_section(&extra_citations),
                citations: extra_citations.clone(),
                managed: true,
                preserve_source_markdown: false,
            };
            total_citations += extra_citations.len();
            sections.push(extra_section);
        }
    }

    let draft = PageDraft {
        page_id: page_id.clone(),
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: build_diagram_drafts(&research.diagram_suggestions),
        citation_count: total_citations,
    };

    let digest = PageDigest {
        digest_id: stable_id("digest", &unit.id),
        unit_id: unit.id.clone(),
        page_id,
        title: unit.title.clone(),
        decomposition_profile: research.decomposition_profile.clone(),
        research_profile: research.research_profile.clone(),
        summary: research.summary.clone(),
        key_topics: research
            .section_plan
            .iter()
            .map(|s| s.title.clone())
            .collect(),
        key_sources: research.key_sources.iter().take(5).cloned().collect(),
        citations: digest_citations_from_research(research, &[]),
        section_digests: Vec::new(),
        diagram_digests: Vec::new(),
        readiness_stage: "compose_ready".to_string(),
    };

    (draft, digest)
}

// ─── Parent pages ───────────────────────────────────────────

/// 父页面 compose：消费 UnitResearch + 子页 PageDigest[]。
pub fn compose_parent_page(
    unit: &KnowledgeUnit,
    research: &UnitResearch,
    child_digests: &[PageDigest],
) -> (PageDraft, PageDigest) {
    let page_id = stable_id("page", &unit.relative_path);
    let mut sections = Vec::new();
    let mut total_citations = 0;

    for planned in &research.section_plan {
        let (section, citation_count) =
            compose_section(planned, &research.evidence_clusters, child_digests);
        if should_keep_section(&section) {
            total_citations += citation_count;
            sections.push(section);
        }
    }

    if total_citations < MIN_PAGE_CITATIONS {
        let deficit = MIN_PAGE_CITATIONS - total_citations;
        let existing_citations = sections
            .iter()
            .flat_map(|section| section.citations.iter().cloned())
            .collect::<Vec<_>>();
        let extra_citations =
            gather_missing_citations(&research.evidence_clusters, &existing_citations, deficit);
        if !extra_citations.is_empty() {
            let extra_section = ComposeSectionDraft {
                section_key: "source-references".to_string(),
                title: "源码参考".to_string(),
                content: format_citation_section(&extra_citations),
                citations: extra_citations.clone(),
                managed: true,
                preserve_source_markdown: false,
            };
            total_citations += extra_citations.len();
            sections.push(extra_section);
        }
    }

    let draft = PageDraft {
        page_id: page_id.clone(),
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: build_diagram_drafts(&research.diagram_suggestions),
        citation_count: total_citations,
    };

    let digest = PageDigest {
        digest_id: stable_id("digest", &unit.id),
        unit_id: unit.id.clone(),
        page_id,
        title: unit.title.clone(),
        decomposition_profile: research.decomposition_profile.clone(),
        research_profile: research.research_profile.clone(),
        summary: research.summary.clone(),
        key_topics: child_digests.iter().map(|d| d.title.clone()).collect(),
        key_sources: merge_digest_key_sources(
            research.key_sources.iter().take(5).cloned(),
            child_digests,
        ),
        citations: digest_citations_from_research(research, child_digests),
        section_digests: Vec::new(),
        diagram_digests: Vec::new(),
        readiness_stage: "compose_ready".to_string(),
    };

    (draft, digest)
}

// ─── Index pages ────────────────────────────────────────────

/// 域索引页面 compose：消费 DomainResearch + 域内子页 PageDigest[]。
pub fn compose_index_page(
    unit: &KnowledgeUnit,
    domain_research: &DomainResearch,
    child_digests: &[PageDigest],
) -> PageDraft {
    let page_id = stable_id("page", &unit.relative_path);

    let mut sections = vec![
        ComposeSectionDraft {
            section_key: "intro".to_string(),
            title: "简介".to_string(),
            content: domain_research.domain_summary.clone(),
            citations: collect_child_digest_citations(child_digests, 3),
            managed: true,
            preserve_source_markdown: false,
        },
        ComposeSectionDraft {
            section_key: "structure".to_string(),
            title: "项目结构".to_string(),
            content: domain_research.internal_structure.clone(),
            citations: collect_child_digest_citations(child_digests, 3),
            managed: true,
            preserve_source_markdown: false,
        },
    ];

    if !child_digests.is_empty() {
        let child_content = child_digests
            .iter()
            .map(render_child_digest_summary)
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ComposeSectionDraft {
            section_key: "components".to_string(),
            title: "核心组件".to_string(),
            content: child_content,
            citations: collect_child_digest_citations(child_digests, 8),
            managed: true,
            preserve_source_markdown: false,
        });
    }

    let relationship_lines = if domain_research.relationships.is_empty() {
        build_digest_relationship_lines(child_digests)
    } else {
        domain_research
            .relationships
            .iter()
            .map(|relationship| format!("- {relationship}"))
            .collect::<Vec<_>>()
    };
    if !relationship_lines.is_empty() {
        sections.push(ComposeSectionDraft {
            section_key: "dependencies".to_string(),
            title: "依赖关系分析".to_string(),
            content: relationship_lines.join("\n"),
            citations: collect_child_digest_citations(child_digests, 8),
            managed: true,
            preserve_source_markdown: false,
        });
    }

    sections.push(ComposeSectionDraft {
        section_key: "conclusion".to_string(),
        title: "结论".to_string(),
        content: build_domain_conclusion(domain_research, child_digests),
        citations: collect_child_digest_citations(child_digests, 4),
        managed: true,
        preserve_source_markdown: false,
    });

    let citation_count = sections
        .iter()
        .map(|section| dedup_compose_citations(section.citations.clone()).len())
        .sum();

    PageDraft {
        page_id,
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: build_index_page_diagrams(unit, domain_research, child_digests),
        citation_count,
    }
}

// ─── System pages ───────────────────────────────────────────

/// 系统页面 compose（Overview / Architecture）：消费 SystemResearch + 域索引 PageDigest[]。
pub fn compose_system_page(
    unit: &KnowledgeUnit,
    system_research: &SystemResearch,
    domain_digests: &[PageDigest],
) -> PageDraft {
    let page_id = stable_id("page", &unit.relative_path);

    let sections = match unit.unit_type {
        UnitType::Overview => compose_overview_sections(system_research, domain_digests),
        UnitType::Architecture => compose_architecture_sections(system_research, domain_digests),
        _ => vec![ComposeSectionDraft {
            section_key: "content".to_string(),
            title: unit.title.clone(),
            content: system_research.description.clone(),
            citations: Vec::new(),
            managed: true,
            preserve_source_markdown: false,
        }],
    };

    let citation_count = sections
        .iter()
        .map(|section| dedup_compose_citations(section.citations.clone()).len())
        .sum();

    PageDraft {
        page_id,
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: build_system_page_diagrams(unit, system_research, domain_digests),
        citation_count,
    }
}

fn compose_overview_sections(
    sr: &SystemResearch,
    domain_digests: &[PageDigest],
) -> Vec<ComposeSectionDraft> {
    let mut sections = vec![
        ComposeSectionDraft {
            section_key: "intro".to_string(),
            title: "简介".to_string(),
            content: format!(
                "{}\n\n**项目类型**: {}\n**目标用户**: {}",
                sr.description,
                sr.project_type,
                sr.target_users.join(", ")
            ),
            citations: collect_child_digest_citations(domain_digests, 4),
            managed: true,
            preserve_source_markdown: false,
        },
        ComposeSectionDraft {
            section_key: "architecture".to_string(),
            title: "架构总览".to_string(),
            content: format!(
                "**架构模式**: {}\n\n{}\n\n**技术栈**\n{}",
                sr.architecture_pattern,
                sr.system_boundary,
                render_bullet_list(&sr.tech_stack, "- 未识别")
            ),
            citations: collect_child_digest_citations(domain_digests, 4),
            managed: true,
            preserve_source_markdown: false,
        },
    ];

    if !domain_digests.is_empty() {
        let structure_content = domain_digests
            .iter()
            .map(render_child_digest_summary)
            .collect::<Vec<_>>()
            .join("\n");
        sections.push(ComposeSectionDraft {
            section_key: "structure".to_string(),
            title: "项目结构".to_string(),
            content: structure_content,
            citations: collect_child_digest_citations(domain_digests, 10),
            managed: true,
            preserve_source_markdown: false,
        });

        let component_content = domain_digests
            .iter()
            .take(MAX_DIGEST_DIAGRAM_NODES)
            .map(render_digest_component_summary)
            .collect::<Vec<_>>()
            .join("\n");
        sections.push(ComposeSectionDraft {
            section_key: "components".to_string(),
            title: "核心组件".to_string(),
            content: component_content,
            citations: collect_child_digest_citations(domain_digests, 10),
            managed: true,
            preserve_source_markdown: false,
        });

        let dependency_lines = build_digest_relationship_lines(domain_digests);
        if !dependency_lines.is_empty() {
            sections.push(ComposeSectionDraft {
                section_key: "dependencies".to_string(),
                title: "依赖关系分析".to_string(),
                content: dependency_lines.join("\n"),
                citations: collect_child_digest_citations(domain_digests, 10),
                managed: true,
                preserve_source_markdown: false,
            });
        }
    }

    sections.push(ComposeSectionDraft {
        section_key: "conclusion".to_string(),
        title: "结论".to_string(),
        content: build_system_conclusion(sr, domain_digests),
        citations: collect_child_digest_citations(domain_digests, 4),
        managed: true,
        preserve_source_markdown: false,
    });

    sections
}

fn compose_architecture_sections(
    sr: &SystemResearch,
    domain_digests: &[PageDigest],
) -> Vec<ComposeSectionDraft> {
    let mut sections = vec![ComposeSectionDraft {
        section_key: "architecture".to_string(),
        title: "架构总览".to_string(),
        content: format!(
            "**架构模式**: {}\n\n{}",
            sr.architecture_pattern, sr.system_boundary
        ),
        citations: collect_child_digest_citations(domain_digests, 4),
        managed: true,
        preserve_source_markdown: false,
    }];

    if !domain_digests.is_empty() {
        let content = domain_digests
            .iter()
            .take(MAX_DIGEST_DIAGRAM_NODES)
            .map(render_digest_component_summary)
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ComposeSectionDraft {
            section_key: "components".to_string(),
            title: "核心组件".to_string(),
            content,
            citations: collect_child_digest_citations(domain_digests, 10),
            managed: true,
            preserve_source_markdown: false,
        });

        let dependency_lines = build_digest_relationship_lines(domain_digests);
        if !dependency_lines.is_empty() {
            sections.push(ComposeSectionDraft {
                section_key: "dependencies".to_string(),
                title: "依赖关系分析".to_string(),
                content: dependency_lines.join("\n"),
                citations: collect_child_digest_citations(domain_digests, 10),
                managed: true,
                preserve_source_markdown: false,
            });
        }
    }

    sections.push(ComposeSectionDraft {
        section_key: "conclusion".to_string(),
        title: "结论".to_string(),
        content: build_system_conclusion(sr, domain_digests),
        citations: collect_child_digest_citations(domain_digests, 4),
        managed: true,
        preserve_source_markdown: false,
    });

    sections
}

fn render_bullet_list(items: &[String], empty_line: &str) -> String {
    if items.is_empty() {
        empty_line.to_string()
    } else {
        items
            .iter()
            .map(|item| format!("- {item}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn render_digest_component_summary(digest: &PageDigest) -> String {
    let topics = digest
        .key_topics
        .iter()
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    if topics.is_empty() {
        format!("- **{}**：{}", digest.title, digest.summary.trim())
    } else {
        format!(
            "- **{}**：{} 关注 {}。",
            digest.title,
            digest.summary.trim(),
            topics.join("、")
        )
    }
}

fn build_digest_relationship_lines(digests: &[PageDigest]) -> Vec<String> {
    digests
        .iter()
        .take(MAX_DIGEST_DIAGRAM_NODES)
        .map(|digest| {
            let topics = digest
                .key_topics
                .iter()
                .take(3)
                .cloned()
                .collect::<Vec<_>>();
            if topics.is_empty() {
                format!("- `{}` 承接 {}。", digest.title, digest.summary.trim())
            } else {
                format!(
                    "- `{}` 负责 {}，并与 {} 形成协作。",
                    digest.title,
                    digest.summary.trim(),
                    topics.join("、")
                )
            }
        })
        .collect()
}

fn build_system_conclusion(sr: &SystemResearch, domain_digests: &[PageDigest]) -> String {
    let domain_titles = domain_digests
        .iter()
        .take(4)
        .map(|digest| digest.title.clone())
        .collect::<Vec<_>>();
    if domain_titles.is_empty() {
        format!(
            "当前仓库以 `{}` 架构组织，核心关注点集中在 {}。",
            sr.architecture_pattern, sr.description
        )
    } else {
        format!(
            "当前仓库以 `{}` 架构组织，核心知识域集中在 {}，共同支撑 {}。",
            sr.architecture_pattern,
            domain_titles.join("、"),
            sr.project_name
        )
    }
}

fn build_domain_conclusion(
    domain_research: &DomainResearch,
    child_digests: &[PageDigest],
) -> String {
    let titles = child_digests
        .iter()
        .take(4)
        .map(|digest| digest.title.clone())
        .collect::<Vec<_>>();
    if titles.is_empty() {
        domain_research.domain_summary.clone()
    } else {
        format!(
            "{} 当前重点页面包括 {}。",
            domain_research.domain_summary,
            titles.join("、")
        )
    }
}

fn build_system_page_diagrams(
    unit: &KnowledgeUnit,
    system_research: &SystemResearch,
    domain_digests: &[PageDigest],
) -> Vec<DiagramDraft> {
    build_digest_star_diagram(
        unit.title.as_str(),
        "知识域关系图",
        format!(
            "展示 {} 与核心知识域之间的关系。",
            system_research.project_name
        ),
        domain_digests,
    )
    .into_iter()
    .collect()
}

fn build_index_page_diagrams(
    unit: &KnowledgeUnit,
    domain_research: &DomainResearch,
    child_digests: &[PageDigest],
) -> Vec<DiagramDraft> {
    if let Some(suggestion) = domain_research.diagram_suggestion.as_ref() {
        let diagrams = build_diagram_drafts(std::slice::from_ref(suggestion));
        if !diagrams.is_empty() {
            return diagrams;
        }
    }

    build_digest_star_diagram(
        unit.title.as_str(),
        "核心组件关系图",
        format!("展示 {} 域内页面之间的承接关系。", unit.title),
        child_digests,
    )
    .into_iter()
    .collect()
}

fn build_digest_star_diagram(
    center_label: &str,
    title: &str,
    description: String,
    digests: &[PageDigest],
) -> Option<DiagramDraft> {
    if digests.is_empty() {
        return None;
    }

    let limited = digests
        .iter()
        .take(MAX_DIGEST_DIAGRAM_NODES)
        .collect::<Vec<_>>();
    let suggestion = DiagramSuggestion {
        diagram_type: "dependency".to_string(),
        title: title.to_string(),
        description,
        nodes: std::iter::once(crate::domain::research::DiagramNodeSuggestion {
            node_id: "center".to_string(),
            label: center_label.to_string(),
        })
        .chain(
            limited
                .iter()
                .map(|digest| crate::domain::research::DiagramNodeSuggestion {
                    node_id: stable_id("node", &digest.title),
                    label: digest.title.clone(),
                }),
        )
        .collect(),
        edges: limited
            .iter()
            .map(|digest| crate::domain::research::DiagramEdgeSuggestion {
                source: "center".to_string(),
                target: stable_id("node", &digest.title),
                label: digest
                    .key_topics
                    .first()
                    .cloned()
                    .filter(|topic| !topic.trim().is_empty()),
            })
            .collect(),
    };

    diagram_suggestion_to_draft(&suggestion)
}

// ─── Full tree compose ──────────────────────────────────────

/// 按 processing_order 叶子优先 compose 整棵知识树。
pub fn compose_knowledge_tree(
    tree: &KnowledgeTree,
    system_research: &SystemResearch,
    domain_researches: &BTreeMap<String, DomainResearch>,
    unit_researches: &BTreeMap<String, UnitResearch>,
) -> (Vec<PageDraft>, BTreeMap<String, PageDigest>) {
    let mut drafts = Vec::new();
    let mut digests: BTreeMap<String, PageDigest> = BTreeMap::new();

    for unit_id in &tree.processing_order {
        let Some(unit) = tree.get_unit(unit_id) else {
            continue;
        };

        let child_digests: Vec<PageDigest> = unit
            .child_unit_ids
            .iter()
            .filter_map(|cid| digests.get(cid).cloned())
            .collect();

        match unit.unit_type {
            UnitType::Overview | UnitType::Architecture => {
                let domain_digests: Vec<PageDigest> = tree
                    .units
                    .values()
                    .filter(|u| u.unit_type == UnitType::DomainIndex)
                    .filter_map(|u| digests.get(&u.id).cloned())
                    .collect();

                let draft = compose_system_page(unit, system_research, &domain_digests);
                let digest = PageDigest {
                    digest_id: stable_id("digest", &unit.id),
                    unit_id: unit.id.clone(),
                    page_id: draft.page_id.clone(),
                    title: unit.title.clone(),
                    decomposition_profile: unit.decomposition_profile.clone(),
                    research_profile: None,
                    summary: system_research.description.clone(),
                    key_topics: system_research.key_domains.clone(),
                    key_sources: merge_digest_key_sources(std::iter::empty(), &domain_digests),
                    citations: collect_child_digest_citations(
                        &domain_digests,
                        MAX_DIGEST_CITATIONS,
                    ),
                    section_digests: Vec::new(),
                    diagram_digests: Vec::new(),
                    readiness_stage: "compose_ready".to_string(),
                };
                digests.insert(unit.id.clone(), digest);
                drafts.push(draft);
            }
            UnitType::DomainIndex => {
                if let Some(dr) = domain_researches.get(&unit.domain_id) {
                    let draft = compose_index_page(unit, dr, &child_digests);
                    let digest = PageDigest {
                        digest_id: stable_id("digest", &unit.id),
                        unit_id: unit.id.clone(),
                        page_id: draft.page_id.clone(),
                        title: unit.title.clone(),
                        decomposition_profile: unit.decomposition_profile.clone(),
                        research_profile: None,
                        summary: dr.domain_summary.clone(),
                        key_topics: child_digests.iter().map(|d| d.title.clone()).collect(),
                        key_sources: merge_digest_key_sources(std::iter::empty(), &child_digests),
                        citations: collect_child_digest_citations(
                            &child_digests,
                            MAX_DIGEST_CITATIONS,
                        ),
                        section_digests: Vec::new(),
                        diagram_digests: Vec::new(),
                        readiness_stage: "compose_ready".to_string(),
                    };
                    digests.insert(unit.id.clone(), digest);
                    drafts.push(draft);
                }
            }
            _ => {
                if let Some(ur) = unit_researches.get(&unit.id) {
                    let (draft, digest) = if unit.is_leaf() {
                        compose_leaf_page(unit, ur)
                    } else {
                        compose_parent_page(unit, ur, &child_digests)
                    };
                    digests.insert(unit.id.clone(), digest);
                    drafts.push(draft);
                }
            }
        }
    }

    (drafts, digests)
}

// ─── Section compose helpers ────────────────────────────────

fn compose_section(
    planned: &PlannedSection,
    evidence_clusters: &[EvidenceCluster],
    child_digests: &[PageDigest],
) -> (ComposeSectionDraft, usize) {
    let mut content_parts = Vec::new();
    let mut section_citations = Vec::new();
    let preserves_reference_markdown =
        planned.preserve_source_markdown || preserves_reference_markdown(&planned.section_summary);
    let reference_outline_contract = uses_reference_outline_contract(planned);

    if !reference_outline_contract && !planned.intent.trim().is_empty() {
        content_parts.push(format!("**本节目标**：{}", planned.intent.trim()));
    }
    if !planned.section_summary.trim().is_empty() {
        content_parts.push(planned.section_summary.trim().to_string());
    }

    let section_clusters = planned
        .evidence_cluster_keys
        .iter()
        .filter_map(|cluster_key| {
            evidence_clusters
                .iter()
                .find(|cluster| cluster.cluster_key == *cluster_key)
        })
        .collect::<Vec<_>>();

    // 填充 evidence
    if !preserves_reference_markdown && !reference_outline_contract && !section_clusters.is_empty()
    {
        content_parts.push("**证据焦点**".to_string());
    }
    for cluster_key in &planned.evidence_cluster_keys {
        if let Some(cluster) = evidence_clusters
            .iter()
            .find(|c| c.cluster_key == *cluster_key)
        {
            if !preserves_reference_markdown && !reference_outline_contract {
                content_parts.push(format!(
                    "- **{}**：{}",
                    cluster.label,
                    summarize_evidence_cluster(cluster)
                ));
            }
            section_citations.extend(cluster.citations.iter().cloned());
        }
    }
    if !preserves_reference_markdown
        && reference_outline_contract
        && planned.section_key != "preamble"
        && planned.section_key != "toc"
        && !section_clusters.is_empty()
    {
        let digest = render_reference_outline_evidence_digest(&section_clusters);
        if !digest.is_empty() {
            content_parts.push(digest);
        }
    }

    // 填充 child digests
    if planned.child_digest_slot && !child_digests.is_empty() {
        content_parts.push("**子页摘要**".to_string());
        for digest in child_digests {
            content_parts.push(render_child_digest_summary(digest));
        }
        section_citations.extend(collect_child_digest_citations(
            child_digests,
            MAX_DIGEST_CITATIONS,
        ));
    }

    // Citation 密度保证——section 级别
    if !preserves_reference_markdown && section_citations.len() < MIN_SECTION_CITATIONS {
        let deficit = MIN_SECTION_CITATIONS - section_citations.len();
        let extra = gather_missing_citations(evidence_clusters, &section_citations, deficit);
        if !reference_outline_contract {
            for citation in &extra {
                content_parts.push(format!("补充参考：{}。", citation.path));
            }
        }
        section_citations.extend(extra);
    }
    section_citations = dedup_compose_citations(section_citations);

    let citation_count = section_citations.len();

    let section = ComposeSectionDraft {
        section_key: planned.section_key.clone(),
        title: planned.title.clone(),
        content: content_parts.join("\n"),
        citations: section_citations,
        managed: true,
        preserve_source_markdown: planned.preserve_source_markdown,
    };

    (section, citation_count)
}

fn preserves_reference_markdown(content: &str) -> bool {
    let normalized = content.trim();
    normalized.contains("file://")
        || normalized.contains("<cite>")
        || normalized.contains("章节来源")
        || normalized.contains("图表来源")
}

fn uses_reference_outline_contract(planned: &PlannedSection) -> bool {
    planned.section_key == "preamble"
        || planned.section_key == "toc"
        || matches!(
            planned.title.as_str(),
            "目录"
                | "简介"
                | "项目结构"
                | "核心组件"
                | "架构总览"
                | "详细组件分析"
                | "依赖关系分析"
                | "性能考量"
                | "故障排查指南"
                | "结论"
                | "附录"
        )
}

fn should_keep_section(section: &ComposeSectionDraft) -> bool {
    !section.content.trim().is_empty()
        || !section.citations.is_empty()
        || section.section_key == "toc"
}

fn summarize_evidence_cluster(cluster: &EvidenceCluster) -> String {
    let noted = cluster
        .citations
        .iter()
        .filter_map(|citation| {
            let note = citation.note.trim();
            (!note.is_empty()).then_some(note.to_string())
        })
        .take(2)
        .collect::<Vec<_>>();
    let paths = cluster
        .citations
        .iter()
        .map(|citation| citation.path.clone())
        .collect::<Vec<_>>();
    let path_summary = summarize_paths(&paths);

    let mut parts = Vec::new();
    if !noted.is_empty() {
        parts.push(noted.join("；"));
    }
    if !path_summary.is_empty() {
        parts.push(format!("重点入口：{path_summary}"));
    }

    if parts.is_empty() {
        format!("覆盖 {} 条源码引用。", cluster.citations.len())
    } else {
        parts.join("；")
    }
}

fn render_reference_outline_evidence_digest(section_clusters: &[&EvidenceCluster]) -> String {
    let summaries = section_clusters
        .iter()
        .take(2)
        .map(|cluster| {
            let summary = summarize_evidence_cluster(cluster);
            if cluster.label.trim().is_empty() {
                summary
            } else {
                format!("{}：{}", cluster.label, summary)
            }
        })
        .filter(|summary| !summary.trim().is_empty())
        .collect::<Vec<_>>();

    if summaries.is_empty() {
        String::new()
    } else {
        format!("相关实现重点：{}。", summaries.join("；"))
    }
}

fn summarize_paths(paths: &[String]) -> String {
    let mut unique = Vec::new();
    for path in paths {
        if !unique.iter().any(|existing: &String| existing == path) {
            unique.push(path.clone());
        }
        if unique.len() == 3 {
            break;
        }
    }

    match unique.len() {
        0 => String::new(),
        1..=2 => unique
            .iter()
            .map(|path| format!("`{path}`"))
            .collect::<Vec<_>>()
            .join("、"),
        _ => format!(
            "{} 等 {} 处文件",
            unique
                .iter()
                .map(|path| format!("`{path}`"))
                .collect::<Vec<_>>()
                .join("、"),
            paths.len()
        ),
    }
}

fn render_child_digest_summary(digest: &PageDigest) -> String {
    let mut line = format!("- **{}**：{}", digest.title, digest.summary);
    let source_summary = summarize_paths(&digest.key_sources);
    if !source_summary.is_empty() {
        line.push_str(&format!(" 关键入口：{source_summary}。"));
    }
    line
}

fn digest_citations_from_research(
    research: &UnitResearch,
    child_digests: &[PageDigest],
) -> Vec<SourceCitation> {
    let mut citations = research
        .evidence_clusters
        .iter()
        .flat_map(|cluster| cluster.citations.iter().cloned())
        .collect::<Vec<_>>();
    citations.extend(collect_child_digest_citations(
        child_digests,
        MAX_DIGEST_CITATIONS,
    ));
    dedup_compose_citations(citations)
        .into_iter()
        .take(MAX_DIGEST_CITATIONS)
        .collect()
}

fn collect_child_digest_citations(
    child_digests: &[PageDigest],
    limit: usize,
) -> Vec<SourceCitation> {
    if limit == 0 {
        return Vec::new();
    }

    let mut citations = Vec::new();
    for digest in child_digests {
        if let Some(citation) = digest.citations.first() {
            citations.push(citation.clone());
        }
    }
    for digest in child_digests {
        for citation in digest.citations.iter().skip(1) {
            citations.push(citation.clone());
        }
    }

    dedup_compose_citations(citations)
        .into_iter()
        .take(limit)
        .collect()
}

fn merge_digest_key_sources<I>(own_sources: I, child_digests: &[PageDigest]) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut seen = std::collections::BTreeSet::new();
    let mut merged = Vec::new();

    for source in own_sources {
        if seen.insert(source.clone()) {
            merged.push(source);
        }
    }
    for digest in child_digests {
        for source in &digest.key_sources {
            if seen.insert(source.clone()) {
                merged.push(source.clone());
            }
        }
    }

    merged.truncate(5);
    merged
}

fn build_diagram_drafts(suggestions: &[DiagramSuggestion]) -> Vec<DiagramDraft> {
    suggestions
        .iter()
        .filter_map(diagram_suggestion_to_draft)
        .collect()
}

fn diagram_suggestion_to_draft(suggestion: &DiagramSuggestion) -> Option<DiagramDraft> {
    if suggestion.nodes.is_empty() || suggestion.edges.is_empty() {
        return None;
    }

    let header = match suggestion.diagram_type.as_str() {
        "flow" | "process" => "flowchart LR",
        _ => "graph LR",
    };
    let mut lines = vec![header.to_string()];
    for node in &suggestion.nodes {
        lines.push(format!(
            "    {}[\"{}\"]",
            mermaid_id(&node.node_id),
            mermaid_label(&node.label)
        ));
    }
    for edge in &suggestion.edges {
        let source = mermaid_id(&edge.source);
        let target = mermaid_id(&edge.target);
        if let Some(label) = edge.label.as_ref().filter(|label| !label.trim().is_empty()) {
            lines.push(format!(
                "    {source} -->|{}| {target}",
                mermaid_label(label)
            ));
        } else {
            lines.push(format!("    {source} --> {target}"));
        }
    }

    Some(DiagramDraft {
        diagram_id: stable_id(
            "diagram",
            format!("{}:{}", suggestion.diagram_type, suggestion.title),
        ),
        diagram_type: suggestion.diagram_type.clone(),
        title: suggestion.title.clone(),
        description: suggestion.description.clone(),
        content: lines.join("\n"),
    })
}

fn mermaid_id(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "node".to_string()
    } else {
        out
    }
}

fn mermaid_label(value: &str) -> String {
    value.replace('"', "\\\"")
}

fn gather_missing_citations(
    evidence_clusters: &[EvidenceCluster],
    existing: &[SourceCitation],
    count: usize,
) -> Vec<SourceCitation> {
    let existing = existing
        .iter()
        .map(citation_identity)
        .collect::<std::collections::BTreeSet<_>>();
    evidence_clusters
        .iter()
        .flat_map(|cluster| cluster.citations.iter())
        .filter(|citation| !existing.contains(&citation_identity(citation)))
        .take(count)
        .cloned()
        .collect()
}

fn dedup_compose_citations(citations: Vec<SourceCitation>) -> Vec<SourceCitation> {
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for citation in citations {
        let key = citation_identity(&citation);
        if seen.insert(key) {
            deduped.push(citation);
        }
    }
    deduped
}

fn citation_identity(citation: &SourceCitation) -> String {
    format!(
        "{}:{}:{}:{}",
        citation.path, citation.start_line, citation.end_line, citation.note
    )
}

fn format_citation_section(citations: &[SourceCitation]) -> String {
    let mut lines = vec!["证据汇总与源码出处如下：".to_string(), String::new()];
    lines.push("**证据**".to_string());
    lines.extend(citations.iter().map(|citation| {
        let line_span = format!("L{}-L{}", citation.start_line, citation.end_line);
        format!(
            "- [`{}`](file://{}#{}) `{}`",
            citation.path, citation.path, line_span, line_span
        )
    }));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::domain::knowledge::{KnowledgeDomain, KnowledgeTree, KnowledgeUnit};

    use super::*;

    #[test]
    fn compose_parent_page_consumes_child_digests_and_keeps_citation_density() {
        let mut unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "父模块",
            "domain-mod",
            "核心模块/parent.md",
        );
        unit.child_unit_ids = vec!["child-unit".to_string()];

        let research = UnitResearch {
            unit_id: unit.id.clone(),
            decomposition_profile: unit.decomposition_profile.clone(),
            research_profile: None,
            positioning: "父页".to_string(),
            summary: "父模块摘要".to_string(),
            section_plan: vec![
                PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "说明父模块定位".to_string(),
                    section_summary: "先交代父模块的职责边界，再说明它消费哪些子能力。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "children".to_string(),
                    title: "子单元".to_string(),
                    intent: "汇总子页".to_string(),
                    section_summary: "把子页职责、差异和阅读顺序收回到父页。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: true,
                    preserve_source_markdown: false,
                },
            ],
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["src/parent.ts".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let child_digests = vec![PageDigest {
            digest_id: "digest-child-unit".to_string(),
            unit_id: "child-unit".to_string(),
            page_id: "page-child".to_string(),
            title: "子模块".to_string(),
            decomposition_profile: None,
            research_profile: None,
            summary: "这是子模块摘要".to_string(),
            key_topics: vec!["topic".to_string()],
            key_sources: vec!["src/child.ts".to_string()],
            citations: vec![SourceCitation {
                path: "src/child.ts".to_string(),
                start_line: 3,
                end_line: 12,
                source_id: Some("source-child".to_string()),
                symbol_id: None,
                note: "子模块入口".to_string(),
            }],
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let (draft, digest) = compose_parent_page(&unit, &research, &child_digests);
        let child_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "children")
            .unwrap();

        assert!(child_section.content.contains("这是子模块摘要"));
        assert!(child_section.content.contains("src/child.ts"));
        assert!(child_section
            .content
            .contains("把子页职责、差异和阅读顺序收回到父页"));
        assert!(child_section.content.contains("**证据焦点**"));
        assert!(child_section
            .citations
            .iter()
            .any(|citation| citation.path == "src/child.ts"));
        assert!(draft.citation_count >= 10);
        assert_eq!(digest.title, "父模块");
        assert!(digest
            .citations
            .iter()
            .any(|citation| citation.path == "src/child.ts"));
    }

    #[test]
    fn compose_system_page_consumes_child_digest_citations_and_key_sources() {
        let unit = KnowledgeUnit::new(
            UnitType::Overview,
            "项目概述",
            "domain-system",
            "项目概述.md",
        );
        let system_research = SystemResearch {
            project_name: "demo".to_string(),
            description: "项目概览".to_string(),
            project_type: "monorepo".to_string(),
            target_users: vec!["开发者".to_string()],
            system_boundary: "系统边界".to_string(),
            tech_stack: vec!["TypeScript".to_string()],
            architecture_pattern: "分层".to_string(),
            key_domains: vec!["核心模块".to_string()],
            input_hash: String::new(),
        };
        let domain_digests = vec![PageDigest {
            digest_id: "digest-domain-runtime".to_string(),
            unit_id: "domain-runtime".to_string(),
            page_id: "page-runtime".to_string(),
            title: "核心模块".to_string(),
            decomposition_profile: None,
            research_profile: None,
            summary: "覆盖核心运行时和扩展点".to_string(),
            key_topics: vec!["运行时".to_string()],
            key_sources: vec!["src/runtime.ts".to_string(), "src/store.ts".to_string()],
            citations: vec![SourceCitation {
                path: "src/runtime.ts".to_string(),
                start_line: 10,
                end_line: 42,
                source_id: Some("source-runtime".to_string()),
                symbol_id: None,
                note: "运行时主入口".to_string(),
            }],
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let draft = compose_system_page(&unit, &system_research, &domain_digests);
        let structure_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "structure")
            .unwrap();

        assert!(structure_section.content.contains("src/runtime.ts"));
        assert!(structure_section
            .citations
            .iter()
            .any(|citation| citation.path == "src/runtime.ts"));
        assert!(draft
            .sections
            .iter()
            .any(|section| section.title == "依赖关系分析"));
        assert!(draft.sections.iter().any(|section| section.title == "结论"));
        assert_eq!(draft.diagrams.len(), 1);
        assert!(draft.citation_count > 0);
    }

    #[test]
    fn compose_index_page_adds_relationship_section_and_fallback_diagram() {
        let unit = KnowledgeUnit::new(
            UnitType::DomainIndex,
            "核心模块",
            "domain-runtime",
            "核心模块/核心模块.md",
        );
        let domain_research = DomainResearch {
            domain_id: "domain-runtime".to_string(),
            domain_summary: "覆盖核心运行时与扩展点。".to_string(),
            internal_structure: "按入口、扩展点和共享状态组织。".to_string(),
            key_modules: Vec::new(),
            key_apis: Vec::new(),
            relationships: Vec::new(),
            diagram_suggestion: None,
            input_hash: String::new(),
        };
        let child_digests = vec![PageDigest {
            digest_id: "digest-unit-runtime".to_string(),
            unit_id: "unit-runtime".to_string(),
            page_id: "page-runtime".to_string(),
            title: "运行时".to_string(),
            decomposition_profile: None,
            research_profile: None,
            summary: "负责主渲染流程。".to_string(),
            key_topics: vec!["渲染".to_string(), "状态同步".to_string()],
            key_sources: vec!["src/runtime.ts".to_string()],
            citations: vec![SourceCitation {
                path: "src/runtime.ts".to_string(),
                start_line: 1,
                end_line: 24,
                source_id: Some("source-runtime".to_string()),
                symbol_id: None,
                note: "运行时入口".to_string(),
            }],
            section_digests: Vec::new(),
            diagram_digests: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let draft = compose_index_page(&unit, &domain_research, &child_digests);

        assert!(draft
            .sections
            .iter()
            .any(|section| section.title == "核心组件"));
        assert!(draft
            .sections
            .iter()
            .any(|section| section.title == "依赖关系分析"));
        assert!(draft.sections.iter().any(|section| section.title == "结论"));
        assert_eq!(draft.diagrams.len(), 1);
        assert!(draft.diagrams[0].content.contains("graph LR"));
    }

    #[test]
    fn compose_leaf_page_materializes_diagram_suggestions() {
        let unit = KnowledgeUnit::new(
            UnitType::ModuleDoc,
            "运行时",
            "domain-runtime",
            "核心运行时/runtime.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            decomposition_profile: unit.decomposition_profile.clone(),
            research_profile: None,
            positioning: "运行时定位".to_string(),
            summary: "运行时摘要".to_string(),
            section_plan: vec![PlannedSection {
                section_key: "overview".to_string(),
                title: "概述".to_string(),
                intent: "说明运行时主线".to_string(),
                section_summary: "需要把运行时入口、状态变化和关键调度关系串起来。".to_string(),
                evidence_cluster_keys: vec!["cluster-a".to_string()],
                child_digest_slot: false,
                preserve_source_markdown: false,
            }],
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: vec![crate::domain::research::DiagramSuggestion {
                diagram_type: "flow".to_string(),
                title: "运行时流程".to_string(),
                description: "流程图".to_string(),
                nodes: vec![
                    crate::domain::research::DiagramNodeSuggestion {
                        node_id: "start".to_string(),
                        label: "Start".to_string(),
                    },
                    crate::domain::research::DiagramNodeSuggestion {
                        node_id: "end".to_string(),
                        label: "End".to_string(),
                    },
                ],
                edges: vec![crate::domain::research::DiagramEdgeSuggestion {
                    source: "start".to_string(),
                    target: "end".to_string(),
                    label: Some("next".to_string()),
                }],
            }],
            key_sources: vec!["src/runtime.rs".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, _) = compose_leaf_page(&unit, &research);

        assert_eq!(draft.diagrams.len(), 1);
        assert_eq!(draft.diagrams[0].description, "流程图");
        assert!(draft.diagrams[0].content.contains("flowchart LR"));
        assert!(draft.diagrams[0].content.contains("start -->|next| end"));
    }

    #[test]
    fn docs_backed_sections_keep_reference_markdown_without_generic_filler() {
        let unit = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "可访问性测试",
            "domain-docs",
            "概念指南/可访问性测试.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            decomposition_profile: unit.decomposition_profile.clone(),
            research_profile: None,
            positioning: String::new(),
            summary: String::new(),
            section_plan: vec![PlannedSection {
                section_key: "preamble".to_string(),
                title: String::new(),
                intent: String::new(),
                section_summary: "<cite>\n- [index.ts](file://src/index.ts)\n</cite>".to_string(),
                evidence_cluster_keys: vec!["cluster-a".to_string()],
                child_digest_slot: false,
                preserve_source_markdown: true,
            }],
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["src/index.ts".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, _) = compose_leaf_page(&unit, &research);
        let section = &draft.sections[0];

        assert!(section.content.contains("<cite>"));
        assert!(!section.content.contains("**证据焦点**"));
        assert!(!section.content.contains("补充参考"));
    }

    #[test]
    fn reference_outline_sections_omit_generic_filler_and_keep_toc_slot() {
        let unit = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "Accessibility Testing",
            "domain-docs",
            "概念指南/Accessibility-Testing.md",
        );
        let research = UnitResearch {
            unit_id: unit.id.clone(),
            decomposition_profile: unit.decomposition_profile.clone(),
            research_profile: None,
            positioning: String::new(),
            summary: String::new(),
            section_plan: vec![
                PlannedSection {
                    section_key: "preamble".to_string(),
                    title: String::new(),
                    intent: String::new(),
                    section_summary: String::new(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "toc".to_string(),
                    title: "目录".to_string(),
                    intent: "给出本页章节导航".to_string(),
                    section_summary: String::new(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "intro".to_string(),
                    title: "简介".to_string(),
                    intent: "说明定位".to_string(),
                    section_summary: "这里是简介。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
            ],
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["docs/topic.md".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, _) = compose_leaf_page(&unit, &research);
        let preamble = draft
            .sections
            .iter()
            .find(|section| section.section_key == "preamble")
            .unwrap();
        let toc = draft
            .sections
            .iter()
            .find(|section| section.section_key == "toc")
            .unwrap();
        let intro = draft
            .sections
            .iter()
            .find(|section| section.section_key == "intro")
            .unwrap();

        assert!(preamble.content.is_empty());
        assert_eq!(toc.content, "");
        assert!(intro.content.contains("这里是简介"));
        assert!(intro.content.contains("相关实现重点"));
        assert!(!intro.content.contains("**本节目标**"));
        assert!(!intro.content.contains("**证据焦点**"));
        assert!(!intro.content.contains("补充参考"));
        assert!(intro.citations.len() >= 2);
    }

    #[test]
    fn compose_knowledge_tree_is_leaf_first() {
        let domain = KnowledgeDomain::new(
            crate::domain::knowledge::DomainType::ConceptGuide,
            "核心概念",
        );
        let mut overview = KnowledgeUnit::new(UnitType::Overview, "项目概述", "", "项目概述.md");
        let mut domain_index = KnowledgeUnit::new(
            UnitType::DomainIndex,
            "核心概念",
            domain.id.clone(),
            "核心概念/核心概念.md",
        );
        let child = KnowledgeUnit::new(
            UnitType::ConceptGuide,
            "依赖注入基础",
            domain.id.clone(),
            "核心概念/依赖注入基础.md",
        );

        domain_index.child_unit_ids = vec![child.id.clone()];
        overview.child_unit_ids = vec![domain_index.id.clone()];

        let mut tree = KnowledgeTree::new(overview.id.clone());
        tree.add_domain(domain.clone());
        tree.add_unit(overview.clone());
        tree.add_unit(domain_index.clone());
        tree.add_unit(child.clone());
        tree.build_processing_order();

        let system_research = SystemResearch {
            project_name: "demo".to_string(),
            description: "demo".to_string(),
            project_type: "library".to_string(),
            target_users: vec!["dev".to_string()],
            system_boundary: "repo".to_string(),
            tech_stack: vec!["rust".to_string()],
            architecture_pattern: "multi-module".to_string(),
            key_domains: vec!["核心概念".to_string()],
            input_hash: String::new(),
        };

        let mut domain_researches = BTreeMap::new();
        domain_researches.insert(
            domain.id.clone(),
            DomainResearch {
                domain_id: domain.id.clone(),
                domain_summary: "域摘要".to_string(),
                internal_structure: "结构".to_string(),
                key_modules: Vec::new(),
                key_apis: Vec::new(),
                relationships: Vec::new(),
                diagram_suggestion: None,
                input_hash: String::new(),
            },
        );

        let mut unit_researches = BTreeMap::new();
        unit_researches.insert(
            child.id.clone(),
            UnitResearch {
                unit_id: child.id.clone(),
                decomposition_profile: child.decomposition_profile.clone(),
                research_profile: None,
                positioning: "leaf".to_string(),
                summary: "leaf summary".to_string(),
                section_plan: vec![PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "内容".to_string(),
                    section_summary: "围绕依赖注入基础建立阅读入口。".to_string(),
                    evidence_cluster_keys: vec!["cluster-leaf".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                }],
                evidence_clusters: vec![dense_cluster("cluster-leaf")],
                diagram_suggestions: Vec::new(),
                key_sources: vec!["src/leaf.ts".to_string()],
                provider_stop_reason: None,
                provider_session_stats: None,
                input_hash: String::new(),
            },
        );

        let (drafts, digests) = compose_knowledge_tree(
            &tree,
            &system_research,
            &domain_researches,
            &unit_researches,
        );

        assert_eq!(
            drafts.first().map(|draft| draft.unit_id.as_str()),
            Some(child.id.as_str())
        );
        assert!(digests.contains_key(&child.id));
        assert!(digests.contains_key(&domain_index.id));
        assert!(digests.contains_key(&overview.id));
    }

    fn dense_cluster(cluster_key: &str) -> EvidenceCluster {
        EvidenceCluster {
            cluster_key: cluster_key.to_string(),
            label: "关键源码".to_string(),
            citations: (1..=6)
                .map(|line| SourceCitation {
                    path: "src/demo.ts".to_string(),
                    start_line: line,
                    end_line: line + 1,
                    source_id: Some("source-demo".to_string()),
                    symbol_id: None,
                    note: String::new(),
                })
                .collect(),
        }
    }
}
