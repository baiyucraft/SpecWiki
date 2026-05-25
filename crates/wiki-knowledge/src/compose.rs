use std::collections::{BTreeMap, BTreeSet};

use crate::domain::compose::{ComposeSectionDraft, DiagramDraft, PageDraft};
use crate::domain::research::{
    canonical_reference_outline_title, DiagramSuggestion, DomainResearch, EvidenceCluster,
    KeySourceCluster, PageDiagramDigest, PageDigest, PageSectionDigest, PlannedSection,
    ProjectionDigestStatus, ProjectionDigestStatusReason, ProjectionDigestStatusReasonKind,
    ResearchPageSeed, ResearchProfile, SectionGroundingRef, SkeletonProfile, SourceCitation,
    SystemResearch, UnitResearch,
};
use wiki_model::domain::knowledge::{DecompositionProfile, KnowledgeTree, KnowledgeUnit, UnitType};
use wiki_model::domain::stable_id::stable_id;

/// 最低 citation 密度——整页不低于此数。
const MIN_PAGE_CITATIONS: usize = 14;
/// 最低 section citation 密度。
const MIN_SECTION_CITATIONS: usize = 3;
/// 子页 digest 最多向父页传播的 citation 数量。
const MAX_DIGEST_CITATIONS: usize = 12;
/// fallback 图表最多展开的 digest 节点数。
const MAX_DIGEST_DIAGRAM_NODES: usize = 8;

/// 统一后的 compose 输入合同。
#[derive(Debug, Clone)]
pub struct ComposePageContract {
    /// 最终页面稳定 ID。
    pub page_id: String,
    /// 当前 contract 对应的 KnowledgeUnit。
    pub unit_id: String,
    /// 页面标题。
    pub title: String,
    /// `.wiki/` 下的稳定相对路径。
    pub relative_path: String,
    /// 页面级摘要；digest 与导语都消费它。
    pub summary: String,
    /// planner 给出的拆分画像。
    pub decomposition_profile: Option<DecompositionProfile>,
    /// research 画像；影响 section 和 citation 风格。
    pub research_profile: Option<ResearchProfile>,
    /// 统一 section plan，所有页面类型都走这一入口。
    pub section_plan: Vec<PlannedSection>,
    /// research 产出的稳定骨架画像。
    pub skeleton_profile: Option<SkeletonProfile>,
    /// section 级 grounding 合同。
    pub section_grounding_refs: Vec<SectionGroundingRef>,
    /// 当前页声明的关键来源簇。
    pub key_source_clusters: Vec<KeySourceCluster>,
    /// 当前页可直接消费的证据簇。
    pub evidence_clusters: Vec<EvidenceCluster>,
    /// 当前页可直接消费的图建议。
    pub diagram_suggestions: Vec<DiagramSuggestion>,
    /// 子页 digest 汇总；父页和高层页通过它做 rollup。
    pub child_digest_rollup: Vec<PageDigest>,
    /// 子页 section 级 citation 摘要。
    pub child_section_citation_digest: Vec<PageSectionDigest>,
    /// 子页图摘要。
    pub child_diagram_digest: Vec<PageDiagramDigest>,
    /// 子页贡献出来的关键源码集合。
    pub child_key_sources: Vec<String>,
    /// 子页 readiness 摘要；避免父页在未就绪时误装配。
    pub child_readiness: Vec<ComposeChildReadiness>,
    /// 当前 parent contract 期望消费的直接 child unit 集合。
    pub expected_child_unit_ids: Vec<String>,
    /// 当前 compose 阶段仍未就绪的直接 child unit 集合。
    pub missing_child_unit_ids: Vec<String>,
}

/// 子页 contract 的 readiness 摘要。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeChildReadiness {
    /// 对应 child digest 的稳定 ID。
    pub digest_id: String,
    /// 对应 child unit 的稳定 ID。
    pub unit_id: String,
    /// child 当前已经到达的 compose/readiness 阶段。
    pub readiness_stage: String,
}

#[derive(Debug, Clone)]
struct ComposePageSeedOverlay {
    summary: String,
    section_plan: Vec<PlannedSection>,
    skeleton_profile: Option<SkeletonProfile>,
    section_grounding_refs: Vec<SectionGroundingRef>,
    key_source_clusters: Vec<KeySourceCluster>,
    evidence_clusters: Vec<EvidenceCluster>,
    diagram_suggestions: Vec<DiagramSuggestion>,
}

// ─── Leaf pages ─────────────────────────────────────────────

/// 叶子页面 compose：消费 UnitResearch，按 section_plan 展开正文。
pub fn compose_leaf_page(unit: &KnowledgeUnit, research: &UnitResearch) -> (PageDraft, PageDigest) {
    compose_page_from_contract(&build_compose_page_contract(unit, research, &[], None))
}

// ─── Parent pages ───────────────────────────────────────────

/// 父页面 compose：消费 UnitResearch + 子页 PageDigest[]。
pub fn compose_parent_page(
    unit: &KnowledgeUnit,
    research: &UnitResearch,
    child_digests: &[PageDigest],
) -> (PageDraft, PageDigest) {
    compose_page_from_contract(&build_compose_page_contract(
        unit,
        research,
        child_digests,
        None,
    ))
}

// ─── Index pages ────────────────────────────────────────────

/// 域索引页面 compose：消费 DomainResearch + 域内子页 PageDigest[]。
pub fn compose_index_page(
    unit: &KnowledgeUnit,
    domain_research: &DomainResearch,
    child_digests: &[PageDigest],
) -> PageDraft {
    compose_seed_backed_page(
        unit,
        &domain_research.compose_seed,
        child_digests,
        Some(domain_research.domain_summary.as_str()),
        None,
        None,
    )
    .0
}

// ─── System pages ───────────────────────────────────────────

/// 系统页面 compose（Overview / Architecture）：消费 SystemResearch + 域索引 PageDigest[]。
pub fn compose_system_page(
    unit: &KnowledgeUnit,
    system_research: &SystemResearch,
    domain_digests: &[PageDigest],
) -> PageDraft {
    let seed = match unit.unit_type {
        UnitType::Architecture => &system_research.architecture_seed,
        _ => &system_research.overview_seed,
    };
    compose_seed_backed_page(
        unit,
        seed,
        domain_digests,
        Some(system_research.description.as_str()),
        None,
        None,
    )
    .0
}

/// 把 unit research 与可选 seed overlay 合并成统一 compose contract。
pub fn build_compose_page_contract(
    unit: &KnowledgeUnit,
    research: &UnitResearch,
    child_digests: &[PageDigest],
    compose_seed: Option<&ResearchPageSeed>,
) -> ComposePageContract {
    let overlay = merge_seed_overlay(research, compose_seed);
    build_compose_page_contract_from_parts(
        unit,
        primary_or_fallback_text(&research.summary, &overlay.summary),
        research.decomposition_profile.clone(),
        research.research_profile.clone(),
        overlay.section_plan,
        overlay.skeleton_profile,
        overlay.section_grounding_refs,
        overlay.key_source_clusters,
        overlay.evidence_clusters,
        overlay.diagram_suggestions,
        child_digests,
    )
}

/// 直接按统一 contract 产出最终 `PageDraft + PageDigest`。
///
/// 这里是 9.8 之后的正式 compose 入口：leaf、parent、domain index 和 system page
/// 最终都要收敛到这一层，而不是再各自维护固定骨架分支。
pub fn compose_page_from_contract(contract: &ComposePageContract) -> (PageDraft, PageDigest) {
    let mut sections = Vec::new();
    let mut total_citations = 0;

    for planned in &contract.section_plan {
        let (section, citation_count) = compose_section_from_contract(contract, planned);
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
            gather_missing_citations(&contract.evidence_clusters, &existing_citations, deficit);
        if !extra_citations.is_empty() {
            total_citations += extra_citations.len();
            sections.push(ComposeSectionDraft {
                section_key: "source-references".to_string(),
                title: "源码参考".to_string(),
                content: format_citation_section(&extra_citations),
                citations: extra_citations,
                managed: true,
                preserve_source_markdown: false,
            });
        }
    }

    let draft = PageDraft {
        page_id: contract.page_id.clone(),
        unit_id: contract.unit_id.clone(),
        title: contract.title.clone(),
        relative_path: contract.relative_path.clone(),
        sections,
        diagrams: build_contract_diagram_drafts(contract),
        citation_count: total_citations,
    };

    let digest = PageDigest {
        digest_id: stable_id("digest", &contract.unit_id),
        unit_id: contract.unit_id.clone(),
        page_id: contract.page_id.clone(),
        title: contract.title.clone(),
        decomposition_profile: contract.decomposition_profile.clone(),
        research_profile: contract.research_profile.clone(),
        summary: contract.summary.clone(),
        key_topics: contract_digest_key_topics(contract),
        key_sources: contract_digest_key_sources(contract),
        planned_key_sources: contract_planned_key_sources(contract),
        grounded_key_sources: contract_grounded_key_sources(contract, &draft),
        skeleton_profile: contract.skeleton_profile.clone(),
        section_grounding_refs: contract.section_grounding_refs.clone(),
        citations: contract_digest_citations(contract),
        section_digests: Vec::new(),
        diagram_digests: Vec::new(),
        projection_status: contract_projection_status(contract),
        status_reasons: contract_projection_status_reasons(contract),
        readiness_stage: contract_digest_readiness(contract),
    };

    (draft, digest)
}

fn compose_seed_backed_page(
    unit: &KnowledgeUnit,
    seed: &ResearchPageSeed,
    child_digests: &[PageDigest],
    summary_fallback: Option<&str>,
    decomposition_profile: Option<DecompositionProfile>,
    research_profile: Option<ResearchProfile>,
) -> (PageDraft, PageDigest) {
    let contract = build_compose_page_contract_from_parts(
        unit,
        primary_or_fallback_text(seed.summary.as_str(), summary_fallback.unwrap_or_default()),
        decomposition_profile,
        research_profile,
        seed.section_plan.clone(),
        seed.skeleton_profile.clone(),
        seed.section_grounding_refs.clone(),
        seed.key_source_clusters.clone(),
        seed.evidence_clusters.clone(),
        seed.diagram_suggestions.clone(),
        child_digests,
    );
    compose_page_from_contract(&contract)
}

fn build_compose_page_contract_from_parts(
    unit: &KnowledgeUnit,
    summary: String,
    decomposition_profile: Option<DecompositionProfile>,
    research_profile: Option<ResearchProfile>,
    section_plan: Vec<PlannedSection>,
    skeleton_profile: Option<SkeletonProfile>,
    section_grounding_refs: Vec<SectionGroundingRef>,
    key_source_clusters: Vec<KeySourceCluster>,
    evidence_clusters: Vec<EvidenceCluster>,
    diagram_suggestions: Vec<DiagramSuggestion>,
    child_digests: &[PageDigest],
) -> ComposePageContract {
    let missing_child_unit_ids = collect_missing_child_unit_ids(unit, child_digests);

    ComposePageContract {
        page_id: stable_id("page", &unit.relative_path),
        unit_id: unit.id.clone(),
        title: unit.title.clone(),
        relative_path: unit.relative_path.clone(),
        summary,
        decomposition_profile,
        research_profile,
        section_plan,
        skeleton_profile,
        section_grounding_refs,
        key_source_clusters,
        evidence_clusters,
        diagram_suggestions,
        child_digest_rollup: child_digests.to_vec(),
        child_section_citation_digest: child_digests
            .iter()
            .flat_map(|digest| digest.section_digests.clone())
            .collect(),
        child_diagram_digest: child_digests
            .iter()
            .flat_map(|digest| digest.diagram_digests.clone())
            .collect(),
        child_key_sources: merge_digest_key_sources(std::iter::empty(), child_digests),
        child_readiness: child_digests
            .iter()
            .map(|digest| ComposeChildReadiness {
                digest_id: digest.digest_id.clone(),
                unit_id: digest.unit_id.clone(),
                readiness_stage: digest.readiness_stage.clone(),
            })
            .collect(),
        expected_child_unit_ids: unit.child_unit_ids.clone(),
        missing_child_unit_ids,
    }
}

fn collect_missing_child_unit_ids(
    unit: &KnowledgeUnit,
    child_digests: &[PageDigest],
) -> Vec<String> {
    let ready_child_ids = child_digests
        .iter()
        .map(|digest| digest.unit_id.as_str())
        .collect::<BTreeSet<_>>();

    unit.child_unit_ids
        .iter()
        .filter(|child_id| !ready_child_ids.contains(child_id.as_str()))
        .cloned()
        .collect()
}

fn merge_seed_overlay(
    research: &UnitResearch,
    compose_seed: Option<&ResearchPageSeed>,
) -> ComposePageSeedOverlay {
    let Some(seed) = compose_seed else {
        return ComposePageSeedOverlay {
            summary: research.summary.clone(),
            section_plan: research.section_plan.clone(),
            skeleton_profile: research.skeleton_profile.clone(),
            section_grounding_refs: research.section_grounding_refs.clone(),
            key_source_clusters: research.key_source_clusters.clone(),
            evidence_clusters: research.evidence_clusters.clone(),
            diagram_suggestions: research.diagram_suggestions.clone(),
        };
    };

    ComposePageSeedOverlay {
        summary: primary_or_fallback_text(&research.summary, &seed.summary),
        section_plan: merge_planned_sections(&seed.section_plan, &research.section_plan),
        skeleton_profile: research
            .skeleton_profile
            .clone()
            .or_else(|| seed.skeleton_profile.clone()),
        section_grounding_refs: merge_section_groundings(
            &seed.section_grounding_refs,
            &research.section_grounding_refs,
        ),
        key_source_clusters: merge_key_source_clusters(
            &seed.key_source_clusters,
            &research.key_source_clusters,
        ),
        evidence_clusters: merge_evidence_clusters(
            &seed.evidence_clusters,
            &research.evidence_clusters,
        ),
        diagram_suggestions: merge_diagram_suggestions(
            &seed.diagram_suggestions,
            &research.diagram_suggestions,
        ),
    }
}

pub fn compose_contract_page_for_unit(
    unit: &KnowledgeUnit,
    system_research: Option<&SystemResearch>,
    domain_research: Option<&DomainResearch>,
    unit_research: Option<&UnitResearch>,
    child_digests: &[PageDigest],
) -> Option<(PageDraft, PageDigest)> {
    let compose_seed = match unit.unit_type {
        UnitType::Overview | UnitType::Architecture => {
            system_research.map(|research| match unit.unit_type {
                UnitType::Architecture => &research.architecture_seed,
                _ => &research.overview_seed,
            })
        }
        UnitType::DomainIndex => domain_research.map(|research| &research.compose_seed),
        _ => None,
    };

    unit_research.map(|research| {
        compose_page_from_contract(&build_compose_page_contract(
            unit,
            research,
            child_digests,
            compose_seed,
        ))
    })
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

        if let Some((draft, digest)) = compose_contract_page_for_unit(
            unit,
            Some(system_research),
            domain_researches.get(&unit.domain_id),
            unit_researches.get(&unit.id),
            &child_digests,
        ) {
            digests.insert(unit.id.clone(), digest);
            drafts.push(draft);
        }
    }

    (drafts, digests)
}

// ─── Section compose helpers ────────────────────────────────

fn compose_section_from_contract(
    contract: &ComposePageContract,
    planned: &PlannedSection,
) -> (ComposeSectionDraft, usize) {
    let mut content_parts = Vec::new();
    let mut section_citations = Vec::new();
    let grounding = contract
        .section_grounding_refs
        .iter()
        .find(|grounding| grounding.section_key == planned.section_key);
    let preserves_reference_markdown =
        planned.preserve_source_markdown || preserves_reference_markdown(&planned.section_summary);
    let reference_outline_contract = uses_reference_outline_contract(planned);
    let section_clusters = collect_section_evidence_clusters(contract, planned, grounding);
    let key_source_clusters =
        collect_section_key_source_clusters(contract, &section_clusters, grounding);
    let section_diagram_titles = collect_section_diagram_titles(contract, grounding);
    let child_section_digests = collect_section_child_section_digests(contract, planned, grounding);
    let child_digests = collect_section_child_digests(contract, planned, grounding);

    if !reference_outline_contract && !planned.intent.trim().is_empty() {
        content_parts.push(format!("**本节目标**：{}", planned.intent.trim()));
    }
    if !planned.section_summary.trim().is_empty() {
        content_parts.push(planned.section_summary.trim().to_string());
    }

    if !preserves_reference_markdown
        && !reference_outline_contract
        && !key_source_clusters.is_empty()
    {
        content_parts.push("**关键源码**".to_string());
        for cluster in &key_source_clusters {
            content_parts.push(render_key_source_cluster_summary(cluster));
        }
    }

    if !preserves_reference_markdown && !reference_outline_contract && !section_clusters.is_empty()
    {
        content_parts.push("**证据焦点**".to_string());
    }
    for cluster in &section_clusters {
        if !preserves_reference_markdown && !reference_outline_contract {
            content_parts.push(format!(
                "- **{}**：{}",
                cluster.label,
                summarize_evidence_cluster(cluster)
            ));
        }
        section_citations.extend(cluster.citations.iter().cloned());
    }
    section_citations.extend(collect_key_source_cluster_citations(
        contract,
        &key_source_clusters,
        &section_citations,
    ));
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

    if !preserves_reference_markdown
        && !reference_outline_contract
        && !section_diagram_titles.is_empty()
    {
        content_parts.push(format!(
            "**图示线索**：{}",
            section_diagram_titles
                .into_iter()
                .map(|title| format!("`{title}`"))
                .collect::<Vec<_>>()
                .join("、")
        ));
    }

    if !child_section_digests.is_empty() {
        content_parts.push("**子页章节摘要**".to_string());
        for digest in &child_section_digests {
            content_parts.push(format!("- **{}**：{}", digest.title, digest.summary));
            section_citations.extend(digest.citations.iter().cloned());
        }
    } else if !child_digests.is_empty() {
        content_parts.push("**子页摘要**".to_string());
        for digest in &child_digests {
            content_parts.push(render_child_digest_summary(digest));
        }
        section_citations.extend(collect_child_digest_citations_from_refs(
            &child_digests,
            MAX_DIGEST_CITATIONS,
        ));
    }

    if !preserves_reference_markdown && section_citations.len() < MIN_SECTION_CITATIONS {
        let deficit = MIN_SECTION_CITATIONS - section_citations.len();
        let extra =
            gather_missing_citations(&contract.evidence_clusters, &section_citations, deficit);
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
        || canonical_reference_outline_title(planned.title.as_str()).is_some()
}

fn should_keep_section(section: &ComposeSectionDraft) -> bool {
    !section.content.trim().is_empty()
        || !section.citations.is_empty()
        || section.section_key == "toc"
}

fn primary_or_fallback_text(primary: &str, fallback: &str) -> String {
    if !primary.trim().is_empty() {
        primary.trim().to_string()
    } else {
        fallback.trim().to_string()
    }
}

fn merge_planned_sections(
    seed_sections: &[PlannedSection],
    research_sections: &[PlannedSection],
) -> Vec<PlannedSection> {
    let mut merged = seed_sections.to_vec();
    for section in research_sections {
        if let Some(existing) = merged.iter_mut().find(|candidate| {
            candidate.section_key == section.section_key || candidate.title == section.title
        }) {
            if !section.intent.trim().is_empty() {
                existing.intent = section.intent.clone();
            }
            if !section.section_summary.trim().is_empty() {
                existing.section_summary = section.section_summary.clone();
            }
            existing.child_digest_slot |= section.child_digest_slot;
            existing.preserve_source_markdown |= section.preserve_source_markdown;
            merge_unique_strings(
                &mut existing.evidence_cluster_keys,
                section.evidence_cluster_keys.iter().cloned(),
            );
        } else {
            merged.push(section.clone());
        }
    }
    if merged.is_empty() {
        research_sections.to_vec()
    } else {
        merged
    }
}

fn merge_section_groundings(
    seed_groundings: &[SectionGroundingRef],
    research_groundings: &[SectionGroundingRef],
) -> Vec<SectionGroundingRef> {
    let mut merged = seed_groundings.to_vec();
    for grounding in research_groundings {
        if let Some(existing) = merged
            .iter_mut()
            .find(|candidate| candidate.section_key == grounding.section_key)
        {
            merge_unique_strings(
                &mut existing.key_source_cluster_keys,
                grounding.key_source_cluster_keys.iter().cloned(),
            );
            merge_unique_strings(
                &mut existing.evidence_cluster_keys,
                grounding.evidence_cluster_keys.iter().cloned(),
            );
            merge_unique_strings(
                &mut existing.child_digest_refs,
                grounding.child_digest_refs.iter().cloned(),
            );
            merge_unique_strings(
                &mut existing.diagram_refs,
                grounding.diagram_refs.iter().cloned(),
            );
        } else {
            merged.push(grounding.clone());
        }
    }
    merged
}

fn merge_key_source_clusters(
    seed_clusters: &[KeySourceCluster],
    research_clusters: &[KeySourceCluster],
) -> Vec<KeySourceCluster> {
    let mut merged = seed_clusters.to_vec();
    for cluster in research_clusters {
        if let Some(existing) = merged
            .iter_mut()
            .find(|candidate| candidate.cluster_key == cluster.cluster_key)
        {
            merge_unique_strings(
                &mut existing.source_paths,
                cluster.source_paths.iter().cloned(),
            );
            merge_unique_strings(
                &mut existing.evidence_cluster_keys,
                cluster.evidence_cluster_keys.iter().cloned(),
            );
            if existing.label.trim().is_empty() && !cluster.label.trim().is_empty() {
                existing.label = cluster.label.clone();
            }
        } else {
            merged.push(cluster.clone());
        }
    }
    merged
}

fn merge_evidence_clusters(
    seed_clusters: &[EvidenceCluster],
    research_clusters: &[EvidenceCluster],
) -> Vec<EvidenceCluster> {
    let mut merged = seed_clusters.to_vec();
    for cluster in research_clusters {
        if let Some(existing) = merged
            .iter_mut()
            .find(|candidate| candidate.cluster_key == cluster.cluster_key)
        {
            let existing_keys = existing
                .citations
                .iter()
                .map(citation_identity)
                .collect::<std::collections::BTreeSet<_>>();
            existing.citations.extend(
                cluster
                    .citations
                    .iter()
                    .filter(|citation| !existing_keys.contains(&citation_identity(citation)))
                    .cloned(),
            );
            if existing.label.trim().is_empty() && !cluster.label.trim().is_empty() {
                existing.label = cluster.label.clone();
            }
        } else {
            merged.push(cluster.clone());
        }
    }
    merged
}

fn merge_diagram_suggestions(
    seed_diagrams: &[DiagramSuggestion],
    research_diagrams: &[DiagramSuggestion],
) -> Vec<DiagramSuggestion> {
    let mut merged = seed_diagrams.to_vec();
    for diagram in research_diagrams {
        if !merged.iter().any(|candidate| {
            candidate.title == diagram.title && candidate.diagram_type == diagram.diagram_type
        }) {
            merged.push(diagram.clone());
        }
    }
    merged
}

fn merge_unique_strings<I>(target: &mut Vec<String>, incoming: I)
where
    I: IntoIterator<Item = String>,
{
    for value in incoming {
        if !target.iter().any(|existing| existing == &value) {
            target.push(value);
        }
    }
}

fn collect_section_evidence_clusters<'a>(
    contract: &'a ComposePageContract,
    planned: &PlannedSection,
    grounding: Option<&SectionGroundingRef>,
) -> Vec<&'a EvidenceCluster> {
    let mut cluster_keys = planned.evidence_cluster_keys.clone();
    if let Some(grounding) = grounding {
        merge_unique_strings(
            &mut cluster_keys,
            grounding.evidence_cluster_keys.iter().cloned(),
        );
    }
    cluster_keys
        .iter()
        .filter_map(|cluster_key| {
            contract
                .evidence_clusters
                .iter()
                .find(|cluster| cluster.cluster_key == *cluster_key)
        })
        .collect()
}

fn collect_section_key_source_clusters<'a>(
    contract: &'a ComposePageContract,
    section_clusters: &[&EvidenceCluster],
    grounding: Option<&SectionGroundingRef>,
) -> Vec<&'a KeySourceCluster> {
    let mut cluster_keys = grounding
        .map(|grounding| grounding.key_source_cluster_keys.clone())
        .unwrap_or_default();
    if cluster_keys.is_empty() {
        for cluster in &contract.key_source_clusters {
            if cluster.evidence_cluster_keys.iter().any(|key| {
                section_clusters
                    .iter()
                    .any(|section_cluster| section_cluster.cluster_key == *key)
            }) {
                cluster_keys.push(cluster.cluster_key.clone());
            }
        }
    }
    cluster_keys
        .iter()
        .filter_map(|cluster_key| {
            contract
                .key_source_clusters
                .iter()
                .find(|cluster| cluster.cluster_key == *cluster_key)
        })
        .collect()
}

fn collect_key_source_cluster_citations(
    contract: &ComposePageContract,
    key_source_clusters: &[&KeySourceCluster],
    existing: &[SourceCitation],
) -> Vec<SourceCitation> {
    let existing_keys = existing
        .iter()
        .map(citation_identity)
        .collect::<std::collections::BTreeSet<_>>();

    key_source_clusters
        .iter()
        .flat_map(|cluster| {
            contract
                .evidence_clusters
                .iter()
                .filter(move |evidence_cluster| {
                    cluster
                        .evidence_cluster_keys
                        .iter()
                        .any(|key| key == &evidence_cluster.cluster_key)
                })
                .flat_map(|cluster| cluster.citations.iter().cloned())
        })
        .filter(|citation| !existing_keys.contains(&citation_identity(citation)))
        .collect()
}

fn collect_section_diagram_titles(
    contract: &ComposePageContract,
    grounding: Option<&SectionGroundingRef>,
) -> Vec<String> {
    let Some(grounding) = grounding else {
        return Vec::new();
    };
    let mut titles = Vec::new();
    for diagram_ref in &grounding.diagram_refs {
        if let Some(diagram) = contract.diagram_suggestions.iter().find(|diagram| {
            diagram.title == *diagram_ref
                || stable_id(
                    "diagram",
                    format!("{}:{}", diagram.diagram_type, diagram.title),
                ) == *diagram_ref
        }) {
            if !titles.iter().any(|existing| existing == &diagram.title) {
                titles.push(diagram.title.clone());
            }
            continue;
        }
        if let Some(diagram) = contract
            .child_diagram_digest
            .iter()
            .find(|diagram| diagram.digest_id == *diagram_ref || diagram.title == *diagram_ref)
        {
            if !titles.iter().any(|existing| existing == &diagram.title) {
                titles.push(diagram.title.clone());
            }
        }
    }
    titles
}

fn collect_section_child_digests<'a>(
    contract: &'a ComposePageContract,
    planned: &PlannedSection,
    grounding: Option<&SectionGroundingRef>,
) -> Vec<&'a PageDigest> {
    let references = grounding
        .map(|grounding| grounding.child_digest_refs.clone())
        .unwrap_or_default();
    if references.is_empty() {
        return planned
            .child_digest_slot
            .then(|| contract.child_digest_rollup.iter().collect())
            .unwrap_or_default();
    }
    contract
        .child_digest_rollup
        .iter()
        .filter(|digest| {
            references.iter().any(|reference| {
                digest.digest_id == *reference
                    || digest.unit_id == *reference
                    || digest.page_id == *reference
                    || digest.title == *reference
            })
        })
        .collect()
}

fn collect_section_child_section_digests<'a>(
    contract: &'a ComposePageContract,
    planned: &PlannedSection,
    grounding: Option<&SectionGroundingRef>,
) -> Vec<&'a PageSectionDigest> {
    let references = grounding
        .map(|grounding| grounding.child_digest_refs.clone())
        .unwrap_or_default();
    if references.is_empty() {
        return planned
            .child_digest_slot
            .then(|| contract.child_section_citation_digest.iter().collect())
            .unwrap_or_default();
    }
    contract
        .child_section_citation_digest
        .iter()
        .filter(|digest| {
            references.iter().any(|reference| {
                digest.digest_id == *reference
                    || digest.section_key == *reference
                    || digest.title == *reference
            })
        })
        .collect()
}

fn render_key_source_cluster_summary(cluster: &KeySourceCluster) -> String {
    let source_summary = summarize_paths(&cluster.source_paths);
    if source_summary.is_empty() {
        format!("- **{}**：覆盖关键源码簇。", cluster.label)
    } else {
        format!("- **{}**：{}", cluster.label, source_summary)
    }
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

fn contract_digest_key_topics(contract: &ComposePageContract) -> Vec<String> {
    if contract.child_digest_rollup.is_empty() {
        contract
            .section_plan
            .iter()
            .map(|section| section.title.clone())
            .collect()
    } else {
        contract
            .child_digest_rollup
            .iter()
            .map(|digest| digest.title.clone())
            .collect()
    }
}

fn contract_digest_key_sources(contract: &ComposePageContract) -> Vec<String> {
    let own_sources = if contract.key_source_clusters.is_empty() {
        contract
            .evidence_clusters
            .iter()
            .flat_map(|cluster| {
                cluster
                    .citations
                    .iter()
                    .map(|citation| citation.path.clone())
            })
            .collect::<Vec<_>>()
    } else {
        contract
            .key_source_clusters
            .iter()
            .flat_map(|cluster| cluster.source_paths.iter().cloned())
            .collect::<Vec<_>>()
    };
    let mut merged = merge_digest_key_sources(own_sources, &contract.child_digest_rollup);
    if merged.is_empty() {
        merge_unique_strings(&mut merged, contract.child_key_sources.iter().cloned());
    }
    merged
}

fn contract_planned_key_sources(contract: &ComposePageContract) -> Vec<String> {
    let mut planned = contract
        .key_source_clusters
        .iter()
        .flat_map(|cluster| cluster.source_paths.iter().cloned())
        .collect::<Vec<_>>();
    if planned.is_empty() {
        merge_unique_strings(
            &mut planned,
            contract.evidence_clusters.iter().flat_map(|cluster| {
                cluster
                    .citations
                    .iter()
                    .map(|citation| citation.path.clone())
            }),
        );
    }
    planned
}

fn contract_grounded_key_sources(contract: &ComposePageContract, draft: &PageDraft) -> Vec<String> {
    let mut grounded = Vec::new();
    if contract.section_grounding_refs.is_empty() {
        let planned = contract_planned_key_sources(contract);
        merge_unique_strings(
            &mut grounded,
            draft.sections.iter().flat_map(|section| {
                let planned = planned.clone();
                section
                    .citations
                    .iter()
                    .filter(move |citation| planned.iter().any(|source| source == &citation.path))
                    .map(|citation| citation.path.clone())
            }),
        );
        return grounded;
    }

    for grounding in &contract.section_grounding_refs {
        if draft
            .sections
            .iter()
            .find(|section| section.section_key == grounding.section_key)
            .is_none()
        {
            continue;
        }
        merge_unique_strings(
            &mut grounded,
            grounding
                .key_source_cluster_keys
                .iter()
                .filter_map(|cluster_key| {
                    contract
                        .key_source_clusters
                        .iter()
                        .find(|cluster| cluster.cluster_key == *cluster_key)
                })
                .flat_map(|cluster| cluster.source_paths.iter().cloned()),
        );
    }
    grounded
}

fn contract_digest_citations(contract: &ComposePageContract) -> Vec<SourceCitation> {
    let mut citations = contract
        .evidence_clusters
        .iter()
        .flat_map(|cluster| cluster.citations.iter().cloned())
        .collect::<Vec<_>>();
    citations.extend(collect_child_digest_citations(
        &contract.child_digest_rollup,
        MAX_DIGEST_CITATIONS,
    ));
    dedup_compose_citations(citations)
        .into_iter()
        .take(MAX_DIGEST_CITATIONS)
        .collect()
}

fn contract_digest_readiness(contract: &ComposePageContract) -> String {
    if !contract.missing_child_unit_ids.is_empty()
        || contract
            .child_readiness
            .iter()
            .any(|readiness| readiness.readiness_stage != "compose_ready")
    {
        "waiting_children".to_string()
    } else {
        "compose_ready".to_string()
    }
}

fn contract_projection_status(contract: &ComposePageContract) -> ProjectionDigestStatus {
    if contract_digest_readiness(contract) == "compose_ready" {
        ProjectionDigestStatus::Ready
    } else {
        ProjectionDigestStatus::Stale
    }
}

fn contract_projection_status_reasons(
    contract: &ComposePageContract,
) -> Vec<ProjectionDigestStatusReason> {
    if contract_digest_readiness(contract) == "compose_ready" {
        return Vec::new();
    }
    let upstream_ref = if contract.missing_child_unit_ids.is_empty() {
        None
    } else {
        Some(format!(
            "missing_child_units:{}",
            contract.missing_child_unit_ids.join(",")
        ))
    };
    vec![ProjectionDigestStatusReason {
        reason_kind: Some(ProjectionDigestStatusReasonKind::DeclaredOrDerivedChanged),
        reason_message: "compose contract 依赖的 child projection 尚未全部就绪".to_string(),
        upstream_ref,
    }]
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

fn collect_child_digest_citations_from_refs(
    child_digests: &[&PageDigest],
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

fn build_contract_diagram_drafts(contract: &ComposePageContract) -> Vec<DiagramDraft> {
    let diagrams = build_diagram_drafts(&contract.diagram_suggestions);
    if !diagrams.is_empty() {
        return diagrams;
    }
    build_digest_star_diagram(
        contract.title.as_str(),
        "页面关系图",
        format!("展示 {} 与直接子页之间的关系。", contract.title),
        &contract.child_digest_rollup,
    )
    .into_iter()
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

    use wiki_model::domain::knowledge::{KnowledgeDomain, KnowledgeTree, KnowledgeUnit};

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
            skeleton_profile: None,
            key_source_clusters: Vec::new(),
            section_grounding_refs: Vec::new(),
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
            planned_key_sources: Vec::new(),
            grounded_key_sources: Vec::new(),
            skeleton_profile: None,
            section_grounding_refs: Vec::new(),
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
            projection_status: ProjectionDigestStatus::Ready,
            status_reasons: Vec::new(),
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
            overview_seed: crate::domain::research::ResearchPageSeed {
                summary: "围绕关键知识域建立总览。".to_string(),
                positioning: String::new(),
                section_plan: vec![
                    PlannedSection {
                        section_key: "domain-map".to_string(),
                        title: "知识域概览".to_string(),
                        intent: "用显式 child digest 收拢核心知识域".to_string(),
                        section_summary: "先交代系统如何被切分成稳定知识域。".to_string(),
                        evidence_cluster_keys: vec!["overview-cluster".to_string()],
                        child_digest_slot: true,
                        preserve_source_markdown: false,
                    },
                    PlannedSection {
                        section_key: "domain-architecture".to_string(),
                        title: "架构焦点".to_string(),
                        intent: "说明高层关系图与关键实现入口".to_string(),
                        section_summary: "再把关键源码与图示线索压到同一节。".to_string(),
                        evidence_cluster_keys: vec!["overview-cluster".to_string()],
                        child_digest_slot: false,
                        preserve_source_markdown: false,
                    },
                ],
                skeleton_profile: Some(crate::domain::research::SkeletonProfile {
                    profile_key: "overview".to_string(),
                    seed_sections: vec![
                        crate::domain::research::SkeletonSection {
                            section_key: "domain-map".to_string(),
                            title: "知识域概览".to_string(),
                        },
                        crate::domain::research::SkeletonSection {
                            section_key: "domain-architecture".to_string(),
                            title: "架构焦点".to_string(),
                        },
                    ],
                }),
                key_source_clusters: vec![crate::domain::research::KeySourceCluster {
                    cluster_key: "cluster-runtime".to_string(),
                    label: "运行时入口".to_string(),
                    source_paths: vec!["src/runtime.ts".to_string()],
                    evidence_cluster_keys: vec!["overview-cluster".to_string()],
                }],
                section_grounding_refs: vec![
                    crate::domain::research::SectionGroundingRef {
                        section_key: "domain-map".to_string(),
                        key_source_cluster_keys: vec!["cluster-runtime".to_string()],
                        evidence_cluster_keys: vec!["overview-cluster".to_string()],
                        child_digest_refs: vec!["digest-domain-runtime".to_string()],
                        diagram_refs: Vec::new(),
                    },
                    crate::domain::research::SectionGroundingRef {
                        section_key: "domain-architecture".to_string(),
                        key_source_cluster_keys: vec!["cluster-runtime".to_string()],
                        evidence_cluster_keys: vec!["overview-cluster".to_string()],
                        child_digest_refs: Vec::new(),
                        diagram_refs: vec!["domain-graph".to_string()],
                    },
                ],
                evidence_clusters: vec![EvidenceCluster {
                    cluster_key: "overview-cluster".to_string(),
                    label: "总览证据".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/runtime.ts".to_string(),
                        start_line: 10,
                        end_line: 42,
                        source_id: Some("source-runtime".to_string()),
                        symbol_id: None,
                        note: "运行时主入口".to_string(),
                    }],
                }],
                diagram_suggestions: vec![crate::domain::research::DiagramSuggestion {
                    diagram_type: "dependency".to_string(),
                    title: "domain-graph".to_string(),
                    description: "知识域关系图".to_string(),
                    nodes: vec![
                        crate::domain::research::DiagramNodeSuggestion {
                            node_id: "overview".to_string(),
                            label: "项目概述".to_string(),
                        },
                        crate::domain::research::DiagramNodeSuggestion {
                            node_id: "runtime".to_string(),
                            label: "核心模块".to_string(),
                        },
                    ],
                    edges: vec![crate::domain::research::DiagramEdgeSuggestion {
                        source: "overview".to_string(),
                        target: "runtime".to_string(),
                        label: Some("rollup".to_string()),
                    }],
                }],
            },
            architecture_seed: crate::domain::research::ResearchPageSeed::default(),
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
            planned_key_sources: Vec::new(),
            grounded_key_sources: Vec::new(),
            skeleton_profile: None,
            section_grounding_refs: Vec::new(),
            citations: vec![SourceCitation {
                path: "src/runtime.ts".to_string(),
                start_line: 10,
                end_line: 42,
                source_id: Some("source-runtime".to_string()),
                symbol_id: None,
                note: "运行时主入口".to_string(),
            }],
            section_digests: vec![PageSectionDigest {
                digest_id: "digest-domain-runtime:behavior".to_string(),
                section_key: "behavior".to_string(),
                title: "行为总览".to_string(),
                summary: "覆盖主流程与扩展点。".to_string(),
                key_sources: vec!["src/runtime.ts".to_string()],
                citations: vec![SourceCitation {
                    path: "src/runtime.ts".to_string(),
                    start_line: 10,
                    end_line: 42,
                    source_id: Some("source-runtime".to_string()),
                    symbol_id: None,
                    note: "运行时主入口".to_string(),
                }],
            }],
            diagram_digests: vec![PageDiagramDigest {
                digest_id: "domain-graph".to_string(),
                diagram_type: "dependency".to_string(),
                title: "domain-graph".to_string(),
                summary: "子页关系图".to_string(),
            }],
            projection_status: ProjectionDigestStatus::Ready,
            status_reasons: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let draft = compose_system_page(&unit, &system_research, &domain_digests);
        let structure_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "domain-map")
            .unwrap();
        let architecture_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "domain-architecture")
            .unwrap();

        assert!(structure_section.content.contains("src/runtime.ts"));
        assert!(structure_section.content.contains("覆盖核心运行时和扩展点"));
        assert!(structure_section
            .citations
            .iter()
            .any(|citation| citation.path == "src/runtime.ts"));
        assert!(architecture_section.content.contains("图示线索"));
        assert!(draft
            .sections
            .iter()
            .any(|section| section.title == "知识域概览"));
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
            compose_seed: crate::domain::research::ResearchPageSeed {
                summary: "围绕核心模块建立域级导航。".to_string(),
                positioning: String::new(),
                section_plan: vec![
                    PlannedSection {
                        section_key: "domain-overview".to_string(),
                        title: "域概览".to_string(),
                        intent: "交代该域的范围与重点".to_string(),
                        section_summary: "先解释运行时域负责什么。".to_string(),
                        evidence_cluster_keys: vec!["domain-cluster".to_string()],
                        child_digest_slot: false,
                        preserve_source_markdown: false,
                    },
                    PlannedSection {
                        section_key: "capability-rollup".to_string(),
                        title: "能力汇总".to_string(),
                        intent: "通过 child section digest 汇总关键能力".to_string(),
                        section_summary: "再把子页章节摘要、关键源码和子图输入收回到父页。"
                            .to_string(),
                        evidence_cluster_keys: vec!["domain-cluster".to_string()],
                        child_digest_slot: false,
                        preserve_source_markdown: false,
                    },
                ],
                skeleton_profile: None,
                key_source_clusters: vec![crate::domain::research::KeySourceCluster {
                    cluster_key: "domain-runtime-cluster".to_string(),
                    label: "域入口".to_string(),
                    source_paths: vec!["src/runtime.ts".to_string()],
                    evidence_cluster_keys: vec!["domain-cluster".to_string()],
                }],
                section_grounding_refs: vec![crate::domain::research::SectionGroundingRef {
                    section_key: "capability-rollup".to_string(),
                    key_source_cluster_keys: vec!["domain-runtime-cluster".to_string()],
                    evidence_cluster_keys: vec!["domain-cluster".to_string()],
                    child_digest_refs: vec![
                        "digest-unit-runtime".to_string(),
                        "digest-unit-runtime:runtime-behavior".to_string(),
                    ],
                    diagram_refs: vec!["runtime-child-graph".to_string()],
                }],
                evidence_clusters: vec![EvidenceCluster {
                    cluster_key: "domain-cluster".to_string(),
                    label: "域证据".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/runtime.ts".to_string(),
                        start_line: 1,
                        end_line: 24,
                        source_id: Some("source-runtime".to_string()),
                        symbol_id: None,
                        note: "运行时入口".to_string(),
                    }],
                }],
                diagram_suggestions: Vec::new(),
            },
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
            planned_key_sources: Vec::new(),
            grounded_key_sources: Vec::new(),
            skeleton_profile: None,
            section_grounding_refs: Vec::new(),
            citations: vec![SourceCitation {
                path: "src/runtime.ts".to_string(),
                start_line: 1,
                end_line: 24,
                source_id: Some("source-runtime".to_string()),
                symbol_id: None,
                note: "运行时入口".to_string(),
            }],
            section_digests: vec![PageSectionDigest {
                digest_id: "digest-unit-runtime:runtime-behavior".to_string(),
                section_key: "runtime-behavior".to_string(),
                title: "运行时行为".to_string(),
                summary: "负责主渲染流程。".to_string(),
                key_sources: vec!["src/runtime.ts".to_string()],
                citations: vec![SourceCitation {
                    path: "src/runtime.ts".to_string(),
                    start_line: 1,
                    end_line: 24,
                    source_id: Some("source-runtime".to_string()),
                    symbol_id: None,
                    note: "运行时入口".to_string(),
                }],
            }],
            diagram_digests: vec![PageDiagramDigest {
                digest_id: "runtime-child-graph".to_string(),
                diagram_type: "dependency".to_string(),
                title: "runtime-child-graph".to_string(),
                summary: "运行时子图".to_string(),
            }],
            projection_status: ProjectionDigestStatus::Ready,
            status_reasons: Vec::new(),
            readiness_stage: "compose_ready".to_string(),
        }];

        let draft = compose_index_page(&unit, &domain_research, &child_digests);
        let rollup_section = draft
            .sections
            .iter()
            .find(|section| section.section_key == "capability-rollup")
            .unwrap();

        assert!(draft
            .sections
            .iter()
            .any(|section| section.title == "能力汇总"));
        assert!(rollup_section.content.contains("运行时行为"));
        assert!(rollup_section.content.contains("src/runtime.ts"));
        assert!(rollup_section.content.contains("图示线索"));
        assert_eq!(draft.diagrams.len(), 1);
        assert!(draft.diagrams[0].content.contains("graph LR"));
    }

    #[test]
    fn compose_leaf_contract_grounds_key_sources_into_target_section() {
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
            section_plan: vec![
                PlannedSection {
                    section_key: "overview".to_string(),
                    title: "概述".to_string(),
                    intent: "说明主线".to_string(),
                    section_summary: "总览运行时主线。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "scheduler".to_string(),
                    title: "调度机制".to_string(),
                    intent: "说明关键调度入口".to_string(),
                    section_summary: "重点说明具体入口和证据。".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
            ],
            skeleton_profile: None,
            key_source_clusters: vec![crate::domain::research::KeySourceCluster {
                cluster_key: "scheduler-cluster".to_string(),
                label: "调度入口".to_string(),
                source_paths: vec!["src/runtime/scheduler.ts".to_string()],
                evidence_cluster_keys: vec!["cluster-b".to_string()],
            }],
            section_grounding_refs: vec![crate::domain::research::SectionGroundingRef {
                section_key: "scheduler".to_string(),
                key_source_cluster_keys: vec!["scheduler-cluster".to_string()],
                evidence_cluster_keys: vec!["cluster-b".to_string()],
                child_digest_refs: Vec::new(),
                diagram_refs: Vec::new(),
            }],
            evidence_clusters: vec![
                EvidenceCluster {
                    cluster_key: "cluster-a".to_string(),
                    label: "总览".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/runtime.ts".to_string(),
                        start_line: 1,
                        end_line: 8,
                        source_id: Some("runtime".to_string()),
                        symbol_id: None,
                        note: "运行时入口".to_string(),
                    }],
                },
                EvidenceCluster {
                    cluster_key: "cluster-b".to_string(),
                    label: "调度".to_string(),
                    citations: vec![SourceCitation {
                        path: "src/runtime/scheduler.ts".to_string(),
                        start_line: 20,
                        end_line: 48,
                        source_id: Some("scheduler".to_string()),
                        symbol_id: None,
                        note: "调度主入口".to_string(),
                    }],
                },
            ],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["src/runtime/scheduler.ts".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, digest) = compose_leaf_page(&unit, &research);
        let overview = draft
            .sections
            .iter()
            .find(|section| section.section_key == "overview")
            .unwrap();
        let scheduler = draft
            .sections
            .iter()
            .find(|section| section.section_key == "scheduler")
            .unwrap();

        assert!(!overview.content.contains("**关键源码**"));
        assert!(scheduler.content.contains("src/runtime/scheduler.ts"));
        assert!(scheduler.content.contains("**关键源码**"));
        assert!(scheduler
            .citations
            .iter()
            .any(|citation| citation.path == "src/runtime/scheduler.ts"));
        assert!(digest
            .key_sources
            .iter()
            .any(|source| source == "src/runtime/scheduler.ts"));
    }

    #[test]
    fn compose_leaf_contract_grounds_key_sources_even_without_section_evidence_keys() {
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
                section_key: "scheduler".to_string(),
                title: "调度机制".to_string(),
                intent: "说明关键调度入口".to_string(),
                section_summary: "重点说明具体入口和证据。".to_string(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            }],
            skeleton_profile: None,
            key_source_clusters: vec![crate::domain::research::KeySourceCluster {
                cluster_key: "scheduler-cluster".to_string(),
                label: "调度入口".to_string(),
                source_paths: vec!["src/runtime/scheduler.ts".to_string()],
                evidence_cluster_keys: vec!["cluster-b".to_string()],
            }],
            section_grounding_refs: vec![crate::domain::research::SectionGroundingRef {
                section_key: "scheduler".to_string(),
                key_source_cluster_keys: vec!["scheduler-cluster".to_string()],
                evidence_cluster_keys: Vec::new(),
                child_digest_refs: Vec::new(),
                diagram_refs: Vec::new(),
            }],
            evidence_clusters: vec![EvidenceCluster {
                cluster_key: "cluster-b".to_string(),
                label: "调度".to_string(),
                citations: vec![SourceCitation {
                    path: "src/runtime/scheduler.ts".to_string(),
                    start_line: 20,
                    end_line: 48,
                    source_id: Some("scheduler".to_string()),
                    symbol_id: None,
                    note: "调度主入口".to_string(),
                }],
            }],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["src/runtime/scheduler.ts".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, digest) = compose_leaf_page(&unit, &research);
        let scheduler = draft
            .sections
            .iter()
            .find(|section| section.section_key == "scheduler")
            .unwrap();

        assert!(scheduler
            .citations
            .iter()
            .any(|citation| citation.path == "src/runtime/scheduler.ts"));
        assert!(digest
            .grounded_key_sources
            .iter()
            .any(|source| source == "src/runtime/scheduler.ts"));
    }

    #[test]
    fn grounded_key_sources_exclude_child_only_citations() {
        let contract = ComposePageContract {
            unit_id: "unit-parent".to_string(),
            page_id: "page-parent".to_string(),
            title: "父页".to_string(),
            relative_path: "父页.md".to_string(),
            summary: "父页摘要".to_string(),
            section_plan: vec![PlannedSection {
                section_key: "overview".to_string(),
                title: "概述".to_string(),
                intent: String::new(),
                section_summary: String::new(),
                evidence_cluster_keys: Vec::new(),
                child_digest_slot: false,
                preserve_source_markdown: false,
            }],
            skeleton_profile: None,
            section_grounding_refs: vec![crate::domain::research::SectionGroundingRef {
                section_key: "overview".to_string(),
                key_source_cluster_keys: vec!["own-cluster".to_string()],
                evidence_cluster_keys: Vec::new(),
                child_digest_refs: vec!["digest-child".to_string()],
                diagram_refs: Vec::new(),
            }],
            key_source_clusters: vec![crate::domain::research::KeySourceCluster {
                cluster_key: "own-cluster".to_string(),
                label: "父页入口".to_string(),
                source_paths: vec!["src/parent.ts".to_string()],
                evidence_cluster_keys: vec!["cluster-parent".to_string()],
            }],
            evidence_clusters: vec![EvidenceCluster {
                cluster_key: "cluster-parent".to_string(),
                label: "父页证据".to_string(),
                citations: vec![SourceCitation {
                    path: "src/parent.ts".to_string(),
                    start_line: 1,
                    end_line: 8,
                    source_id: None,
                    symbol_id: None,
                    note: String::new(),
                }],
            }],
            diagram_suggestions: Vec::new(),
            child_digest_rollup: vec![PageDigest {
                digest_id: "digest-child".to_string(),
                unit_id: "unit-child".to_string(),
                page_id: "page-child".to_string(),
                title: "子页".to_string(),
                key_sources: vec!["src/child.ts".to_string()],
                citations: vec![SourceCitation {
                    path: "src/child.ts".to_string(),
                    start_line: 1,
                    end_line: 4,
                    source_id: None,
                    symbol_id: None,
                    note: String::new(),
                }],
                ..PageDigest::default()
            }],
            child_key_sources: vec!["src/child.ts".to_string()],
            child_section_citation_digest: Vec::new(),
            child_diagram_digest: Vec::new(),
            child_readiness: Vec::new(),
            expected_child_unit_ids: vec!["unit-child".to_string()],
            missing_child_unit_ids: Vec::new(),
            decomposition_profile: None,
            research_profile: None,
        };

        let (draft, digest) = compose_page_from_contract(&contract);

        assert!(draft
            .sections
            .iter()
            .flat_map(|section| section.citations.iter())
            .any(|citation| citation.path == "src/child.ts"));
        assert_eq!(digest.grounded_key_sources, vec!["src/parent.ts"]);
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
            skeleton_profile: None,
            key_source_clusters: Vec::new(),
            section_grounding_refs: Vec::new(),
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
            skeleton_profile: None,
            key_source_clusters: Vec::new(),
            section_grounding_refs: Vec::new(),
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
            skeleton_profile: None,
            key_source_clusters: Vec::new(),
            section_grounding_refs: Vec::new(),
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
    fn reference_outline_alias_titles_also_use_reference_contract() {
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
                    title: "引言".to_string(),
                    intent: "说明定位".to_string(),
                    section_summary: "这里是引言。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
                PlannedSection {
                    section_key: "dependencies".to_string(),
                    title: "依赖分析".to_string(),
                    intent: "说明依赖".to_string(),
                    section_summary: "这里是依赖分析。".to_string(),
                    evidence_cluster_keys: vec!["cluster-a".to_string()],
                    child_digest_slot: false,
                    preserve_source_markdown: false,
                },
            ],
            skeleton_profile: None,
            key_source_clusters: Vec::new(),
            section_grounding_refs: Vec::new(),
            evidence_clusters: vec![dense_cluster("cluster-a")],
            diagram_suggestions: Vec::new(),
            key_sources: vec!["docs/topic.md".to_string()],
            provider_stop_reason: None,
            provider_session_stats: None,
            input_hash: String::new(),
        };

        let (draft, _) = compose_leaf_page(&unit, &research);
        let intro = draft
            .sections
            .iter()
            .find(|section| section.section_key == "intro")
            .unwrap();
        let dependencies = draft
            .sections
            .iter()
            .find(|section| section.section_key == "dependencies")
            .unwrap();

        assert!(intro.content.contains("这里是引言"));
        assert!(dependencies.content.contains("这里是依赖分析"));
        assert!(!intro.content.contains("**本节目标**"));
        assert!(!dependencies.content.contains("**本节目标**"));
    }

    #[test]
    fn compose_knowledge_tree_is_leaf_first() {
        let domain = KnowledgeDomain::new(
            wiki_model::domain::knowledge::DomainType::ConceptGuide,
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
            overview_seed: crate::domain::research::ResearchPageSeed {
                summary: "项目总览".to_string(),
                positioning: String::new(),
                section_plan: vec![PlannedSection {
                    section_key: "domain-map".to_string(),
                    title: "知识域概览".to_string(),
                    intent: "总览知识域".to_string(),
                    section_summary: "通过 domain digest 串联项目。".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: true,
                    preserve_source_markdown: false,
                }],
                skeleton_profile: None,
                key_source_clusters: Vec::new(),
                section_grounding_refs: vec![crate::domain::research::SectionGroundingRef {
                    section_key: "domain-map".to_string(),
                    key_source_cluster_keys: Vec::new(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_refs: vec![domain_index.id.clone()],
                    diagram_refs: Vec::new(),
                }],
                evidence_clusters: Vec::new(),
                diagram_suggestions: Vec::new(),
            },
            architecture_seed: crate::domain::research::ResearchPageSeed::default(),
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
                compose_seed: crate::domain::research::ResearchPageSeed {
                    summary: "概念域".to_string(),
                    positioning: String::new(),
                    section_plan: vec![PlannedSection {
                        section_key: "child-rollup".to_string(),
                        title: "能力汇总".to_string(),
                        intent: "收拢叶子页面".to_string(),
                        section_summary: "通过 child digest 汇总依赖注入基础。".to_string(),
                        evidence_cluster_keys: Vec::new(),
                        child_digest_slot: true,
                        preserve_source_markdown: false,
                    }],
                    skeleton_profile: None,
                    key_source_clusters: Vec::new(),
                    section_grounding_refs: Vec::new(),
                    evidence_clusters: Vec::new(),
                    diagram_suggestions: Vec::new(),
                },
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
                skeleton_profile: None,
                key_source_clusters: Vec::new(),
                section_grounding_refs: Vec::new(),
                evidence_clusters: vec![dense_cluster("cluster-leaf")],
                diagram_suggestions: Vec::new(),
                key_sources: vec!["src/leaf.ts".to_string()],
                provider_stop_reason: None,
                provider_session_stats: None,
                input_hash: String::new(),
            },
        );
        unit_researches.insert(
            domain_index.id.clone(),
            UnitResearch {
                unit_id: domain_index.id.clone(),
                decomposition_profile: domain_index.decomposition_profile.clone(),
                research_profile: None,
                positioning: "domain parent".to_string(),
                summary: "domain summary".to_string(),
                section_plan: vec![PlannedSection {
                    section_key: "child-rollup".to_string(),
                    title: "能力汇总".to_string(),
                    intent: "收拢叶子页面".to_string(),
                    section_summary: "通过子页章节 digest 汇总依赖注入基础。".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: true,
                    preserve_source_markdown: false,
                }],
                skeleton_profile: None,
                key_source_clusters: Vec::new(),
                section_grounding_refs: Vec::new(),
                evidence_clusters: vec![dense_cluster("cluster-domain")],
                diagram_suggestions: Vec::new(),
                key_sources: vec!["src/domain.ts".to_string()],
                provider_stop_reason: None,
                provider_session_stats: None,
                input_hash: String::new(),
            },
        );
        unit_researches.insert(
            overview.id.clone(),
            UnitResearch {
                unit_id: overview.id.clone(),
                decomposition_profile: overview.decomposition_profile.clone(),
                research_profile: None,
                positioning: "overview parent".to_string(),
                summary: "overview summary".to_string(),
                section_plan: vec![PlannedSection {
                    section_key: "domain-map".to_string(),
                    title: "知识域概览".to_string(),
                    intent: "总览知识域".to_string(),
                    section_summary: "通过 parent rollup 串联项目。".to_string(),
                    evidence_cluster_keys: Vec::new(),
                    child_digest_slot: true,
                    preserve_source_markdown: false,
                }],
                skeleton_profile: None,
                key_source_clusters: Vec::new(),
                section_grounding_refs: Vec::new(),
                evidence_clusters: vec![dense_cluster("cluster-overview")],
                diagram_suggestions: Vec::new(),
                key_sources: vec!["src/overview.ts".to_string()],
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
        assert!(drafts.iter().any(|draft| draft
            .sections
            .iter()
            .any(|section| section.title == "知识域概览")));
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
