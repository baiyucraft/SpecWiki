use tempfile::tempdir;
use wiki_runtime::domain::checkpoint::{PipelineRuntimeSummary, UnitRuntimeGate};
use wiki_runtime::domain::context::PageContext;
use wiki_runtime::domain::knowledge::{DomainType, KnowledgeDomain, KnowledgeUnit, UnitType};
use wiki_runtime::domain::metadata::DirtyState;
use wiki_runtime::domain::state::{BuildState, WikiPageState, WikiSectionState, WikiState};
use wiki_runtime::generation::sections::SectionDraft;
use wiki_runtime::storage::cache_store::{
    missing_incremental_cache_components, write_page_context_cache, write_page_generation_cache,
    PageContextCacheEntry, PageGenerationCacheEntry,
};
use wiki_runtime::storage::sqlite_store;

fn sample_section(page_id: &str, title: &str) -> WikiSectionState {
    WikiSectionState {
        section_id: format!("section:{page_id}:{title}"),
        title: title.to_string(),
        managed: true,
        owner_kind: Some(wiki_model::domain::projection::SectionOwnership::DerivedManaged),
        knowledge_refs: Vec::new(),
        content_hash: "content-hash".to_string(),
        input_hash: "input-hash".to_string(),
        generated_content_hash: Some("content-hash".to_string()),
        projection_digest_ref: Some(format!("projection:{page_id}")),
        anchor_after_section_id: None,
        anchor_before_section_id: None,
        source_ids: Vec::new(),
        relation_ids: Vec::new(),
    }
}

fn sample_page(page_id: &str, title: &str, page_type: &str, parent_id: Option<&str>) -> WikiPageState {
    WikiPageState {
        page_id: page_id.to_string(),
        title: title.to_string(),
        path: format!(".wiki/{title}.md"),
        page_type: page_type.to_string(),
        parent_id: parent_id.map(str::to_string),
        ancestor_ids: parent_id.into_iter().map(str::to_string).collect(),
        input_hash: "input-hash".to_string(),
        content_hash: "content-hash".to_string(),
        source_ids: Vec::new(),
        source_paths: Vec::new(),
        module_ids: Vec::new(),
        summary: format!("{title} 摘要"),
        provenance: Vec::new(),
        section_anchors: vec!["intro".to_string()],
        sections: vec![sample_section(page_id, "简介")],
    }
}

fn sample_state() -> WikiState {
    WikiState {
        pages: vec![
            sample_page("page-parent", "核心模块", "domain-index", None),
            sample_page("page-child", "运行时", "module", Some("page-parent")),
        ],
        sources: Vec::new(),
        modules: Vec::new(),
        relations: Vec::new(),
        dirty_state: DirtyState::fresh(),
        build_state: BuildState {
            generated_at: "2026-03-19T00:00:00Z".to_string(),
            page_count: 2,
            module_count: 0,
        },
    }
}

fn sample_context(page_id: &str, unit_id: &str, page_type: &str) -> PageContext {
    PageContext {
        page_id: page_id.to_string(),
        page_type: page_type.to_string(),
        scope: if page_type == "domain-index" {
            "domain".to_string()
        } else {
            "unit".to_string()
        },
        unit_id: Some(unit_id.to_string()),
        unit_type: Some(if page_type == "domain-index" {
            "domain_index".to_string()
        } else {
            "module_doc".to_string()
        }),
        domain_id: Some("domain-runtime".to_string()),
        source_ids: Vec::new(),
        module_ids: Vec::new(),
        relation_ids: Vec::new(),
        facts: Vec::new(),
        summary_inputs: Vec::new(),
        hints: Vec::new(),
        child_summaries: Vec::new(),
        child_unit_ids: Vec::new(),
        child_page_ids: Vec::new(),
        child_digest_ids: Vec::new(),
        readiness_status: String::new(),
        citation_digest_refs: Vec::new(),
        diagram_digest_refs: Vec::new(),
        evidence_groups: Vec::new(),
        diagram_inputs: Vec::new(),
    }
}

fn write_runtime_fixture(
    repo_root: &std::path::Path,
    include_runtime_summary: bool,
    include_runtime_gates: bool,
    include_parent_contract: bool,
) {
    let mut conn = sqlite_store::open_db(repo_root).unwrap();
    sqlite_store::scan_cache_set(&conn, "repo-scan", "{}").unwrap();
    sqlite_store::scan_cache_set(&conn, "module-tree", "{}").unwrap();
    sqlite_store::replace_state_rows(&mut conn, &sample_state()).unwrap();

    let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心模块");
    let mut parent_unit = KnowledgeUnit::new(
        UnitType::DomainIndex,
        "核心模块",
        domain.id.clone(),
        "核心模块/核心模块.md",
    );
    let child_unit = KnowledgeUnit::new(
        UnitType::ModuleDoc,
        "运行时",
        domain.id.clone(),
        "核心模块/运行时.md",
    );
    parent_unit.child_unit_ids.push(child_unit.id.clone());
    sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
    sqlite_store::write_knowledge_units(&conn, &[parent_unit.clone(), child_unit.clone()]).unwrap();

    let mut parent_context = sample_context("page-parent", &parent_unit.id, "domain-index");
    if include_parent_contract {
        parent_context.child_summaries.push("运行时：负责主流程".to_string());
        parent_context.child_unit_ids.push(child_unit.id.clone());
        parent_context.child_page_ids.push("page-child".to_string());
        parent_context.child_digest_ids.push("digest-child".to_string());
        parent_context.readiness_status = "compose_ready".to_string();
        parent_context
            .citation_digest_refs
            .push("section-runtime".to_string());
    } else {
        parent_context.readiness_status = "compose_ready".to_string();
    }
    let child_context = sample_context("page-child", &child_unit.id, "module");

    write_page_context_cache(
        repo_root,
        &PageContextCacheEntry {
            page_id: "page-parent".to_string(),
            input_hash: "input-hash".to_string(),
            context: parent_context,
        },
    )
    .unwrap();
    write_page_context_cache(
        repo_root,
        &PageContextCacheEntry {
            page_id: "page-child".to_string(),
            input_hash: "input-hash".to_string(),
            context: child_context,
        },
    )
    .unwrap();
    for page_id in ["page-parent", "page-child"] {
        write_page_generation_cache(
            repo_root,
            &PageGenerationCacheEntry {
                page_id: page_id.to_string(),
                input_hash: "input-hash".to_string(),
                content_hash: "content-hash".to_string(),
                sections: vec![SectionDraft {
                    section_id: format!("section:{page_id}:intro"),
                    title: "简介".to_string(),
                    managed: true,
                    source_ids: Vec::new(),
                    relation_ids: Vec::new(),
                    content: "内容".to_string(),
                }],
            },
        )
        .unwrap();
    }

    if include_runtime_summary {
        let summary = serde_json::to_string(&PipelineRuntimeSummary {
            facts_input_hash: "facts-hash".to_string(),
            workflow_action: "init".to_string(),
            runtime_state: "compose_complete".to_string(),
            researched_units: 2,
            compose_ready_units: 2,
            composed_units: 2,
            assembled_pages: 2,
            blocked_units: Vec::new(),
            last_ready_stage: Some("compose_unit".to_string()),
            last_interrupted_stage: None,
            summary_reason: None,
            current_research_unit_id: None,
            current_research_unit_type: None,
            current_research_started_at: None,
            last_researched_unit_id: None,
            last_research_elapsed_ms: None,
        })
        .unwrap();
        sqlite_store::runtime_meta_set(&conn, "pipeline_runtime_summary", &summary).unwrap();
    }

    if include_runtime_gates {
        for unit in [parent_unit, child_unit] {
            sqlite_store::write_unit_runtime_gate(
                &conn,
                &UnitRuntimeGate {
                    unit_id: unit.id.clone(),
                    unit_type: unit.unit_type.as_str().to_string(),
                    research_status: "ready".to_string(),
                    compose_status: "ready".to_string(),
                    assemble_status: "pending".to_string(),
                    last_ready_stage: Some("compose_unit".to_string()),
                    blocked_reason: None,
                    missing_dependencies: Vec::new(),
                    updated_at: "123".to_string(),
                },
            )
            .unwrap();
        }
    }
}

#[test]
fn sqlite_runtime_gates_round_trip() {
    let temp = tempdir().unwrap();
    let conn = sqlite_store::open_db(temp.path()).unwrap();
    let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "核心模块");
    let unit = KnowledgeUnit::new(
        UnitType::ModuleDoc,
        "运行时",
        domain.id.clone(),
        "核心模块/运行时.md",
    );
    sqlite_store::write_knowledge_domains(&conn, &[domain]).unwrap();
    sqlite_store::write_knowledge_units(&conn, std::slice::from_ref(&unit)).unwrap();

    let gate = UnitRuntimeGate {
        unit_id: unit.id.clone(),
        unit_type: unit.unit_type.as_str().to_string(),
        research_status: "ready".to_string(),
        compose_status: "blocked".to_string(),
        assemble_status: "pending".to_string(),
        last_ready_stage: Some("research_unit".to_string()),
        blocked_reason: Some("missing child digest".to_string()),
        missing_dependencies: vec!["digest-child".to_string()],
        updated_at: "42".to_string(),
    };
    sqlite_store::write_unit_runtime_gate(&conn, &gate).unwrap();

    let loaded = sqlite_store::read_unit_runtime_gates(&conn).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].unit_id, unit.id);
    assert_eq!(loaded[0].compose_status, "blocked");
    assert_eq!(loaded[0].missing_dependencies, vec!["digest-child".to_string()]);

    sqlite_store::clear_unit_runtime_gates(&conn).unwrap();
    assert!(sqlite_store::read_unit_runtime_gates(&conn).unwrap().is_empty());
}

#[test]
fn cache_integrity_accepts_runtime_summary_gates_and_parent_contract() {
    let temp = tempdir().unwrap();
    write_runtime_fixture(temp.path(), true, true, true);

    let missing = missing_incremental_cache_components(temp.path(), &sample_state());

    assert!(missing.is_empty(), "unexpected missing components: {missing:?}");
}

#[test]
fn cache_integrity_reports_missing_runtime_summary_gates_and_parent_contract() {
    let temp = tempdir().unwrap();
    write_runtime_fixture(temp.path(), false, false, false);

    let missing = missing_incremental_cache_components(temp.path(), &sample_state());

    assert!(missing.contains(&"pipeline-runtime-summary".to_string()));
    assert!(missing.contains(&"unit-runtime-gates".to_string()));
    assert!(missing.contains(&"page-context-child-contract:page-parent".to_string()));
    assert!(missing.iter().any(|item| item.starts_with("unit-runtime-gate:")));
}



