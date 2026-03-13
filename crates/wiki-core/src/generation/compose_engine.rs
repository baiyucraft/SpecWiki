use std::collections::BTreeMap;

use crate::domain::compose::{ComposeSectionDraft, PageDraft};
use crate::domain::knowledge::{KnowledgeTree, KnowledgeUnit, UnitType};
use crate::domain::research::{
    DomainResearch, EvidenceCluster, PageDigest, PlannedSection, SourceCitation, SystemResearch,
    UnitResearch,
};
use crate::domain::stable_id::stable_id;

/// 最低 citation 密度——整页不低于此数。
const MIN_PAGE_CITATIONS: usize = 10;
/// 最低 section citation 密度。
const MIN_SECTION_CITATIONS: usize = 2;

// ─── Leaf pages ─────────────────────────────────────────────

/// 叶子页面 compose：消费 UnitResearch，按 section_plan 展开正文。
pub fn compose_leaf_page(
    unit: &KnowledgeUnit,
    research: &UnitResearch,
) -> (PageDraft, PageDigest) {
    let page_id = stable_id("page", &unit.relative_path);
    let mut sections = Vec::new();
    let mut total_citations = 0;

    for planned in &research.section_plan {
        let (section, citation_count) =
            compose_section(planned, &research.evidence_clusters, &[]);
        total_citations += citation_count;
        sections.push(section);
    }

    // Citation 密度保证
    if total_citations < MIN_PAGE_CITATIONS {
        let deficit = MIN_PAGE_CITATIONS - total_citations;
        let extra_citations = gather_extra_citations(&research.evidence_clusters, deficit);
        if !extra_citations.is_empty() {
            let extra_section = ComposeSectionDraft {
                section_key: "source-references".to_string(),
                title: "源码参考".to_string(),
                content: format_citation_section(&extra_citations),
                citations: extra_citations.clone(),
                managed: true,
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
        diagrams: Vec::new(),
        citation_count: total_citations,
    };

    let digest = PageDigest {
        unit_id: unit.id.clone(),
        page_id,
        title: unit.title.clone(),
        summary: research.summary.clone(),
        key_topics: research
            .section_plan
            .iter()
            .map(|s| s.title.clone())
            .collect(),
        key_sources: research.key_sources.iter().take(5).cloned().collect(),
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
        total_citations += citation_count;
        sections.push(section);
    }

    if total_citations < MIN_PAGE_CITATIONS {
        let deficit = MIN_PAGE_CITATIONS - total_citations;
        let extra_citations = gather_extra_citations(&research.evidence_clusters, deficit);
        if !extra_citations.is_empty() {
            let extra_section = ComposeSectionDraft {
                section_key: "source-references".to_string(),
                title: "源码参考".to_string(),
                content: format_citation_section(&extra_citations),
                citations: extra_citations.clone(),
                managed: true,
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
        diagrams: Vec::new(),
        citation_count: total_citations,
    };

    let digest = PageDigest {
        unit_id: unit.id.clone(),
        page_id,
        title: unit.title.clone(),
        summary: research.summary.clone(),
        key_topics: child_digests.iter().map(|d| d.title.clone()).collect(),
        key_sources: research.key_sources.iter().take(5).cloned().collect(),
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
            section_key: "overview".to_string(),
            title: "概述".to_string(),
            content: domain_research.domain_summary.clone(),
            citations: Vec::new(),
            managed: true,
        },
        ComposeSectionDraft {
            section_key: "structure".to_string(),
            title: "内部结构".to_string(),
            content: domain_research.internal_structure.clone(),
            citations: Vec::new(),
            managed: true,
        },
    ];

    if !child_digests.is_empty() {
        let child_content = child_digests
            .iter()
            .map(|d| format!("- **{}**：{}", d.title, d.summary))
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ComposeSectionDraft {
            section_key: "pages".to_string(),
            title: "域内页面".to_string(),
            content: child_content,
            citations: Vec::new(),
            managed: true,
        });
    }

    PageDraft {
        page_id,
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: Vec::new(),
        citation_count: 0,
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
        }],
    };

    PageDraft {
        page_id,
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        sections,
        diagrams: Vec::new(),
        citation_count: 0,
    }
}

fn compose_overview_sections(
    sr: &SystemResearch,
    domain_digests: &[PageDigest],
) -> Vec<ComposeSectionDraft> {
    let mut sections = vec![
        ComposeSectionDraft {
            section_key: "overview".to_string(),
            title: "项目概述".to_string(),
            content: format!(
                "{}\n\n**项目类型**: {}\n**目标用户**: {}",
                sr.description,
                sr.project_type,
                sr.target_users.join(", ")
            ),
            citations: Vec::new(),
            managed: true,
        },
        ComposeSectionDraft {
            section_key: "tech-stack".to_string(),
            title: "技术栈".to_string(),
            content: if sr.tech_stack.is_empty() {
                "未识别".to_string()
            } else {
                sr.tech_stack
                    .iter()
                    .map(|t| format!("- {t}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            citations: Vec::new(),
            managed: true,
        },
    ];

    if !domain_digests.is_empty() {
        let content = domain_digests
            .iter()
            .map(|d| format!("- **{}**：{}", d.title, d.summary))
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ComposeSectionDraft {
            section_key: "knowledge-domains".to_string(),
            title: "知识域".to_string(),
            content,
            citations: Vec::new(),
            managed: true,
        });
    }

    sections
}

fn compose_architecture_sections(
    sr: &SystemResearch,
    domain_digests: &[PageDigest],
) -> Vec<ComposeSectionDraft> {
    let mut sections = vec![
        ComposeSectionDraft {
            section_key: "overview".to_string(),
            title: "架构概述".to_string(),
            content: format!(
                "**架构模式**: {}\n\n{}",
                sr.architecture_pattern, sr.system_boundary
            ),
            citations: Vec::new(),
            managed: true,
        },
    ];

    if !domain_digests.is_empty() {
        let content = domain_digests
            .iter()
            .map(|d| format!("- **{}**：{}", d.title, d.summary))
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(ComposeSectionDraft {
            section_key: "domains".to_string(),
            title: "模块域".to_string(),
            content,
            citations: Vec::new(),
            managed: true,
        });
    }

    sections
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

                let draft =
                    compose_system_page(unit, system_research, &domain_digests);
                let digest = PageDigest {
                    unit_id: unit.id.clone(),
                    page_id: draft.page_id.clone(),
                    title: unit.title.clone(),
                    summary: system_research.description.clone(),
                    key_topics: system_research.key_domains.clone(),
                    key_sources: Vec::new(),
                };
                digests.insert(unit.id.clone(), digest);
                drafts.push(draft);
            }
            UnitType::DomainIndex => {
                if let Some(dr) = domain_researches.get(&unit.domain_id) {
                    let draft = compose_index_page(unit, dr, &child_digests);
                    let digest = PageDigest {
                        unit_id: unit.id.clone(),
                        page_id: draft.page_id.clone(),
                        title: unit.title.clone(),
                        summary: dr.domain_summary.clone(),
                        key_topics: child_digests.iter().map(|d| d.title.clone()).collect(),
                        key_sources: Vec::new(),
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

    content_parts.push(planned.intent.clone());

    // 填充 evidence
    for cluster_key in &planned.evidence_cluster_keys {
        if let Some(cluster) = evidence_clusters.iter().find(|c| c.cluster_key == *cluster_key) {
            content_parts.push(format!("\n**{}**:", cluster.label));
            for citation in &cluster.citations {
                content_parts.push(format!(
                    "- `{}` (L{}-L{})",
                    citation.path, citation.start_line, citation.end_line
                ));
                section_citations.push(citation.clone());
            }
        }
    }

    // 填充 child digests
    if planned.child_digest_slot && !child_digests.is_empty() {
        content_parts.push(String::new());
        for digest in child_digests {
            content_parts.push(format!("- **{}**：{}", digest.title, digest.summary));
        }
    }

    // Citation 密度保证——section 级别
    if section_citations.len() < MIN_SECTION_CITATIONS {
        let deficit = MIN_SECTION_CITATIONS - section_citations.len();
        let extra = gather_extra_citations(evidence_clusters, deficit);
        for citation in &extra {
            content_parts.push(format!(
                "- 参考：`{}` (L{}-L{})",
                citation.path, citation.start_line, citation.end_line
            ));
        }
        section_citations.extend(extra);
    }

    let citation_count = section_citations.len();

    let section = ComposeSectionDraft {
        section_key: planned.section_key.clone(),
        title: planned.title.clone(),
        content: content_parts.join("\n"),
        citations: section_citations,
        managed: true,
    };

    (section, citation_count)
}

fn gather_extra_citations(
    evidence_clusters: &[EvidenceCluster],
    count: usize,
) -> Vec<SourceCitation> {
    evidence_clusters
        .iter()
        .flat_map(|c| c.citations.iter())
        .take(count)
        .cloned()
        .collect()
}

fn format_citation_section(citations: &[SourceCitation]) -> String {
    citations
        .iter()
        .map(|c| format!("- `{}` (L{}-L{})", c.path, c.start_line, c.end_line))
        .collect::<Vec<_>>()
        .join("\n")
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
            positioning: "父页".to_string(),
            summary: "父模块摘要".to_string(),
            section_plan: vec![
                PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "说明父模块定位".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                },
                PlannedSection {
                    section_key: "children".to_string(),
                    title: "子单元".to_string(),
                    intent: "汇总子页".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: true,
                },
            ],
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["src/parent.ts".to_string()],
            input_hash: String::new(),
        };

        let child_digests = vec![PageDigest {
            unit_id: "child-unit".to_string(),
            page_id: "page-child".to_string(),
            title: "子模块".to_string(),
            summary: "这是子模块摘要".to_string(),
            key_topics: vec!["topic".to_string()],
            key_sources: vec!["src/child.ts".to_string()],
        }];

        let (draft, digest) = compose_parent_page(&unit, &research, &child_digests);
        let child_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "children")
            .unwrap();

        assert!(child_section.content.contains("这是子模块摘要"));
        assert!(draft.citation_count >= 10);
        assert_eq!(digest.title, "父模块");
    }

    #[test]
    fn compose_knowledge_tree_is_leaf_first() {
        let domain = KnowledgeDomain::new(crate::domain::knowledge::DomainType::ConceptGuide, "核心概念");
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
                positioning: "leaf".to_string(),
                summary: "leaf summary".to_string(),
                section_plan: vec![PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "内容".to_string(),
                    evidence_cluster_keys: vec!["cluster-leaf".to_string()],
                    child_digest_slot: false,
                }],
                evidence_clusters: vec![dense_cluster("cluster-leaf")],
                diagram_suggestions: Vec::new(),
                key_sources: vec!["src/leaf.ts".to_string()],
                input_hash: String::new(),
            },
        );

        let (drafts, digests) =
            compose_knowledge_tree(&tree, &system_research, &domain_researches, &unit_researches);

        assert_eq!(drafts.first().map(|draft| draft.unit_id.as_str()), Some(child.id.as_str()));
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
