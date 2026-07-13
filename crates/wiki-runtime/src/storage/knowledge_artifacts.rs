use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::domain::checkpoint::{PipelineRuntimeSummary, UnitRuntimeGate};
use crate::domain::context::PageContext;
use crate::domain::metadata::WikiMetadata;
use crate::domain::runtime_profile::{
    FusionReadiness, LayerReadiness, RestoredLevel, RuntimeReadiness,
};
use crate::domain::state::{rebuild_state_from_metadata, WikiSectionState, WikiState};
use crate::generation::managed_sections::{content_hash, parse_wiki_page, PageBlock};
use crate::generation::sections::SectionDraft;
use crate::storage::cache_store::{
    ensure_cache_dir, ensure_page_cache_dirs, write_module_tree_cache, write_page_context_cache,
    write_page_generation_cache, write_scan_cache, PageContextCacheEntry, PageGenerationCacheEntry,
};
use crate::storage::metadata_store::{metadata_exists, metadata_path, read_metadata};
use crate::storage::sqlite::runtime_store::SqliteRuntimeStore;
use crate::storage::sqlite_store;
use crate::storage::state_store::{mark_level1_restored_mirror, write_state};
use crate::storage::wiki_fs::{remove_cache_db, resolve_page_path, wiki_root};
use wiki_index::fingerprint::fingerprint_bytes;
use wiki_index::scanner::{FilePurpose, ScanReport, ScannedFile};
use wiki_knowledge::domain::compose::PageDraft;
use wiki_knowledge::domain::research::{
    validate_page_digest_snapshot, PageDigest, ProjectionDigestStatus, UnitResearch,
};
use wiki_knowledge::plan_pages_from_knowledge_tree;
use wiki_model::domain::knowledge::{
    KnowledgeDomain, KnowledgeTree, KnowledgeUnit, KnowledgeUnitStatus,
};
use wiki_model::domain::knowledge_artifact::{
    validate_conflict_record_snapshot, validate_declared_record_snapshot,
    validate_research_summary_snapshot, CommittedSnapshotManifest, DeclaredKnowledgeRecord,
    DeclaredKnowledgeRecordStatus, DeclaredKnowledgeRelationKind, KnowledgeConflictKind,
    KnowledgeConflictRecord, KnowledgeConflictStatus, KnowledgeHealthRecommendedAction,
    KnowledgeHealthSeverity, KnowledgeHealthSignal, KnowledgeHealthSignalKind,
    KnowledgeResearchSummary, KnowledgeResearchSummaryStatus, KnowledgeRuntimeGateRecord,
};
use wiki_model::domain::module_tree::ModuleTree;
use wiki_model::domain::projection::{
    ProjectionDigest, ProjectionDigestStatus as ModelProjectionDigestStatus,
    ProjectionStatusReason, ProjectionStatusReasonKind,
};
use wiki_model::domain::source_citation::SourceCitation;

const ARTIFACT_SCHEMA_VERSION: &str = "1";
const DECLARED_START_PREFIX: &str = "<!-- wiki:declared";
const DECLARED_END_MARKER: &str = "<!-- wiki:declared:end -->";

#[derive(Debug, Clone)]
pub struct KnowledgeArtifactSnapshot {
    pub domains: Vec<KnowledgeDomain>,
    pub units: Vec<KnowledgeUnit>,
    pub knowledge_tree: KnowledgeTree,
    pub declared_records: Vec<DeclaredKnowledgeRecord>,
    pub conflict_records: Vec<KnowledgeConflictRecord>,
    pub research_summaries: Vec<KnowledgeResearchSummary>,
    pub page_digests: Vec<PageDigest>,
    pub projection_digests: Vec<ProjectionDigest>,
    pub runtime_gates: Vec<KnowledgeRuntimeGateRecord>,
    pub health_signals: Vec<KnowledgeHealthSignal>,
    pub snapshot_manifest: CommittedSnapshotManifest,
}

#[derive(Debug, Clone)]
pub struct RestoreOutcome {
    pub attempted: bool,
    pub restored_cache: bool,
    pub restored_level: RestoredLevel,
    pub readiness: RuntimeReadiness,
    pub reason: Option<String>,
    pub snapshot_id: Option<String>,
}

impl RestoreOutcome {
    fn not_attempted(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            attempted: false,
            restored_cache: false,
            restored_level: RestoredLevel::None,
            readiness: RuntimeReadiness::missing(reason.clone()),
            reason: Some(reason),
            snapshot_id: None,
        }
    }

    fn blocked(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            attempted: true,
            restored_cache: false,
            restored_level: RestoredLevel::None,
            readiness: RuntimeReadiness {
                index: LayerReadiness::Blocked,
                knowledge: LayerReadiness::Blocked,
                projection: LayerReadiness::Conflict,
                fusion: FusionReadiness::Blocked,
                restored_level: RestoredLevel::None,
                snapshot_id: None,
                reasons: vec![reason.clone()],
            },
            reason: Some(reason),
            snapshot_id: None,
        }
    }

    fn restored(snapshot_id: Option<String>) -> Self {
        Self {
            attempted: true,
            restored_cache: true,
            restored_level: RestoredLevel::Level1,
            readiness: RuntimeReadiness::level1(snapshot_id.clone(), Vec::new()),
            reason: None,
            snapshot_id,
        }
    }
}

pub struct PersistKnowledgeArtifactsInput<'a> {
    pub repo_root: &'a Path,
    pub workflow_action: &'a str,
    pub generated_at: &'a str,
    pub facts_input_hash: &'a str,
    pub metadata: &'a WikiMetadata,
    pub knowledge_tree: &'a KnowledgeTree,
    pub declared_records: &'a [DeclaredKnowledgeRecord],
    pub research_summaries: &'a [KnowledgeResearchSummary],
    pub page_digests: &'a [PageDigest],
    pub runtime_gates: &'a [UnitRuntimeGate],
    pub health_signals: &'a [KnowledgeHealthSignal],
}

pub fn knowledge_root(repo_root: &Path) -> PathBuf {
    wiki_root(repo_root).join(".knowledge")
}

fn derived_root(repo_root: &Path) -> PathBuf {
    knowledge_root(repo_root).join("derived")
}

fn runtime_root(repo_root: &Path) -> PathBuf {
    knowledge_root(repo_root).join("runtime")
}

fn declared_root(repo_root: &Path) -> PathBuf {
    knowledge_root(repo_root).join("declared")
}

fn knowledge_domains_path(repo_root: &Path) -> PathBuf {
    derived_root(repo_root).join("knowledge-domains.json")
}

fn knowledge_units_path(repo_root: &Path) -> PathBuf {
    derived_root(repo_root).join("knowledge-units.jsonl")
}

fn knowledge_tree_path(repo_root: &Path) -> PathBuf {
    derived_root(repo_root).join("knowledge-tree.json")
}

fn research_summaries_path(repo_root: &Path) -> PathBuf {
    derived_root(repo_root).join("research-summaries.jsonl")
}

fn declared_records_path(repo_root: &Path) -> PathBuf {
    declared_root(repo_root).join("records.jsonl")
}

fn page_digests_path(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("page-digests.jsonl")
}

fn projection_digests_path(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("projection-digests.jsonl")
}

fn conflict_records_path(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("conflict-records.jsonl")
}

fn runtime_gates_path(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("runtime-gates.jsonl")
}

fn health_signals_path(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("health-signals.jsonl")
}

fn snapshots_root(repo_root: &Path) -> PathBuf {
    runtime_root(repo_root).join("snapshots")
}

fn snapshot_manifest_path(repo_root: &Path, snapshot_id: &str) -> PathBuf {
    snapshots_root(repo_root)
        .join(snapshot_id)
        .join("manifest.yaml")
}

fn current_snapshot_manifest_path(repo_root: &Path) -> io::Result<(String, PathBuf)> {
    let metadata = read_metadata(repo_root)?;
    let snapshot_id = metadata
        .current_snapshot_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "metadata current_snapshot_id is missing",
            )
        })?;
    let snapshot_path = Path::new(&snapshot_id);
    if snapshot_path.components().count() != 1
        || !matches!(
            snapshot_path.components().next(),
            Some(std::path::Component::Normal(_))
        )
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "metadata current_snapshot_id is invalid",
        ));
    }
    Ok((
        snapshot_id.clone(),
        snapshot_manifest_path(repo_root, &snapshot_id),
    ))
}

pub fn knowledge_artifacts_exist(repo_root: &Path) -> bool {
    knowledge_domains_path(repo_root).exists()
        && knowledge_units_path(repo_root).exists()
        && knowledge_tree_path(repo_root).exists()
        && declared_records_path(repo_root).exists()
        && research_summaries_path(repo_root).exists()
        && page_digests_path(repo_root).exists()
        && projection_digests_path(repo_root).exists()
        && conflict_records_path(repo_root).exists()
        && runtime_gates_path(repo_root).exists()
        && health_signals_path(repo_root).exists()
        && current_snapshot_manifest_path(repo_root)
            .map(|(_, path)| path.is_file())
            .unwrap_or(false)
}

pub fn persist_knowledge_artifacts(input: PersistKnowledgeArtifactsInput<'_>) -> io::Result<()> {
    fs::create_dir_all(derived_root(input.repo_root))?;
    fs::create_dir_all(runtime_root(input.repo_root))?;
    fs::create_dir_all(declared_root(input.repo_root))?;

    let runtime_gates = input
        .runtime_gates
        .iter()
        .map(|gate| KnowledgeRuntimeGateRecord {
            unit_id: gate.unit_id.clone(),
            unit_type: gate.unit_type.clone(),
            research_status: gate.research_status.clone(),
            compose_status: gate.compose_status.clone(),
            assemble_status: gate.assemble_status.clone(),
            last_ready_stage: gate.last_ready_stage.clone(),
            blocked_reason: gate.blocked_reason.clone(),
            missing_dependencies: gate.missing_dependencies.clone(),
            updated_at: gate.updated_at.clone(),
        })
        .collect::<Vec<_>>();

    let metadata_json = serde_json::to_vec_pretty(input.metadata)
        .map_err(|error| io::Error::other(format!("serialize metadata for artifact: {error}")))?;
    let metadata_hash = fingerprint_bytes(&metadata_json);

    let domains = input
        .knowledge_tree
        .domains
        .values()
        .cloned()
        .collect::<Vec<_>>();
    let conflict_records = derive_declared_conflicts(input.declared_records, input.generated_at);
    let units = materialize_unit_contracts(
        input.knowledge_tree,
        input.declared_records,
        input.research_summaries,
        input.page_digests,
        &runtime_gates,
        input.generated_at,
    );
    validate_research_summary_snapshot(input.research_summaries).map_err(|error| {
        io::Error::other(format!("validate research summary snapshot: {error}"))
    })?;
    validate_declared_record_snapshot(input.declared_records)
        .map_err(|error| io::Error::other(format!("validate declared snapshot: {error}")))?;
    validate_conflict_record_snapshot(&conflict_records, input.declared_records)
        .map_err(|error| io::Error::other(format!("validate conflict snapshot: {error}")))?;
    validate_page_digest_snapshot(input.page_digests)
        .map_err(|error| io::Error::other(format!("validate page digest snapshot: {error}")))?;
    validate_page_digests_match_metadata(input.metadata, input.page_digests).map_err(|error| {
        io::Error::other(format!("validate page digest metadata binding: {error}"))
    })?;
    let projection_digests = build_projection_digests_from_page_digests(input.page_digests)?;
    let mut health_signals = build_minimal_health_signals(
        &domains,
        &units,
        &conflict_records,
        input.research_summaries,
        input.declared_records,
        input.page_digests,
    );
    let mut health_by_id = health_signals
        .drain(..)
        .map(|signal| (signal.signal_id.clone(), signal))
        .collect::<BTreeMap<_, _>>();
    for signal in input.health_signals {
        health_by_id.insert(signal.signal_id.clone(), signal.clone());
    }
    let health_signals = health_by_id.into_values().collect::<Vec<_>>();
    let declared_snapshot_id = compute_declared_snapshot_id(input.declared_records)?;
    let knowledge_snapshot_id = compute_knowledge_snapshot_id(
        input.knowledge_tree,
        input.declared_records,
        &conflict_records,
        input.research_summaries,
        input.page_digests,
        &runtime_gates,
        &health_signals,
        input.facts_input_hash,
    )?;
    let snapshot_id = compute_committed_snapshot_id(input.facts_input_hash);
    let page_hashes = input
        .metadata
        .wiki_items
        .iter()
        .map(|item| (item.id.clone(), item.content_hash.clone()))
        .collect::<BTreeMap<_, _>>();
    let projection_digest_refs = projection_digests
        .iter()
        .map(|digest| digest.projection_id.clone())
        .collect::<Vec<_>>();
    let runtime_gate_refs = runtime_gates
        .iter()
        .map(|gate| gate.unit_id.clone())
        .collect::<Vec<_>>();
    let projection_snapshot_id = fingerprint_bytes(
        serde_json::to_vec(&projection_digests)
            .map_err(|error| io::Error::other(format!("serialize projection snapshot: {error}")))?
            .as_slice(),
    );
    let manifest = CommittedSnapshotManifest {
        schema_version: ARTIFACT_SCHEMA_VERSION.to_string(),
        snapshot_id: snapshot_id.clone(),
        repo_root: input.repo_root.to_string_lossy().to_string(),
        workflow_action: input.workflow_action.to_string(),
        generated_at: input.generated_at.to_string(),
        facts_input_hash: input.facts_input_hash.to_string(),
        graph_snapshot_id: input.facts_input_hash.to_string(),
        knowledge_snapshot_id,
        declared_snapshot_id,
        projection_snapshot_id,
        metadata_hash,
        page_hashes,
        projection_digest_refs,
        runtime_gate_refs,
        status: "committed".to_string(),
        page_count: input.metadata.wiki_items.len(),
        unit_count: units.len(),
    };

    write_json(&knowledge_domains_path(input.repo_root), &domains)?;
    write_json_lines(&knowledge_units_path(input.repo_root), &units)?;
    write_json(&knowledge_tree_path(input.repo_root), input.knowledge_tree)?;
    write_json_lines(
        &declared_records_path(input.repo_root),
        input.declared_records,
    )?;
    write_json_lines(
        &research_summaries_path(input.repo_root),
        input.research_summaries,
    )?;
    write_json_lines(&page_digests_path(input.repo_root), input.page_digests)?;
    write_json_lines(
        &projection_digests_path(input.repo_root),
        &projection_digests,
    )?;
    write_json_lines(&conflict_records_path(input.repo_root), &conflict_records)?;
    write_json_lines(&runtime_gates_path(input.repo_root), &runtime_gates)?;
    write_json_lines(&health_signals_path(input.repo_root), &health_signals)?;
    write_yaml(
        &snapshot_manifest_path(input.repo_root, &snapshot_id),
        &manifest,
    )
}

pub fn load_knowledge_artifacts(repo_root: &Path) -> io::Result<KnowledgeArtifactSnapshot> {
    Ok(KnowledgeArtifactSnapshot {
        domains: read_json(&knowledge_domains_path(repo_root))?,
        units: read_json_lines(&knowledge_units_path(repo_root))?,
        knowledge_tree: read_json(&knowledge_tree_path(repo_root))?,
        declared_records: read_json_lines(&declared_records_path(repo_root))?,
        conflict_records: read_json_lines(&conflict_records_path(repo_root))?,
        research_summaries: read_json_lines(&research_summaries_path(repo_root))?,
        page_digests: read_json_lines(&page_digests_path(repo_root))?,
        projection_digests: read_json_lines(&projection_digests_path(repo_root))?,
        runtime_gates: read_json_lines(&runtime_gates_path(repo_root))?,
        health_signals: read_json_lines(&health_signals_path(repo_root))?,
        snapshot_manifest: read_snapshot_manifest(repo_root)?,
    })
}

pub fn load_declared_records(repo_root: &Path) -> io::Result<Vec<DeclaredKnowledgeRecord>> {
    read_json_lines(&declared_records_path(repo_root))
}

pub fn load_health_signals(repo_root: &Path) -> io::Result<Vec<KnowledgeHealthSignal>> {
    read_json_lines(&health_signals_path(repo_root))
}

fn build_projection_digests_from_page_digests(
    page_digests: &[PageDigest],
) -> io::Result<Vec<ProjectionDigest>> {
    let mut projection_digests = Vec::new();
    for page_digest in page_digests {
        let status = match page_digest.projection_status {
            ProjectionDigestStatus::Ready => ModelProjectionDigestStatus::Ready,
            ProjectionDigestStatus::Stale => ModelProjectionDigestStatus::Stale,
            ProjectionDigestStatus::Blocked => ModelProjectionDigestStatus::Blocked,
        };
        let status_reasons = page_digest
            .status_reasons
            .iter()
            .map(|reason| ProjectionStatusReason {
                reason_kind: reason.reason_kind.map(|kind| match kind {
                    wiki_knowledge::domain::research::ProjectionDigestStatusReasonKind::MissingPageOutput => {
                        ProjectionStatusReasonKind::PageSnapshotMismatch
                    }
                    wiki_knowledge::domain::research::ProjectionDigestStatusReasonKind::PageSnapshotMismatch => {
                        ProjectionStatusReasonKind::PageSnapshotMismatch
                    }
                    wiki_knowledge::domain::research::ProjectionDigestStatusReasonKind::DeclaredOrDerivedChanged => {
                        ProjectionStatusReasonKind::SectionHashMismatch
                    }
                    wiki_knowledge::domain::research::ProjectionDigestStatusReasonKind::BlockedByResearch => {
                        ProjectionStatusReasonKind::KnowledgeRefMissing
                    }
                }),
                reason_message: reason.reason_message.clone(),
                upstream_ref: reason.upstream_ref.clone(),
            })
            .collect::<Vec<_>>();
        let mut projection_digest = ProjectionDigest {
            projection_id: page_digest.digest_id.clone(),
            page_id: page_digest.page_id.clone(),
            section_ids: page_digest
                .section_digests
                .iter()
                .map(|section| section.section_key.clone())
                .collect(),
            knowledge_refs: vec![page_digest.unit_id.clone()],
            source_refs: page_digest.key_sources.clone(),
            input_hash: page_digest.digest_id.clone(),
            renderer_version: if page_digest.readiness_stage.trim().is_empty() {
                "page-compose-digest-derived".to_string()
            } else {
                page_digest.readiness_stage.clone()
            },
            content_digest: page_digest.digest_id.clone(),
            section_hashes: page_digest
                .section_digests
                .iter()
                .map(|section| (section.section_key.clone(), section.digest_id.clone()))
                .collect(),
            status,
            status_reasons,
        };
        projection_digest.canonicalize();
        projection_digest.validate().map_err(|error| {
            io::Error::other(format!(
                "validate projection digest '{}': {:?}",
                projection_digest.projection_id, error
            ))
        })?;
        projection_digests.push(projection_digest);
    }
    Ok(projection_digests)
}

fn validate_projection_digest_snapshot_binding(
    artifacts: &KnowledgeArtifactSnapshot,
) -> io::Result<()> {
    for digest in &artifacts.projection_digests {
        digest.validate().map_err(|error| {
            io::Error::other(format!(
                "validate model projection digest '{}': {:?}",
                digest.projection_id, error
            ))
        })?;
    }
    let refs = artifacts
        .projection_digests
        .iter()
        .map(|digest| digest.projection_id.clone())
        .collect::<Vec<_>>();
    if artifacts.snapshot_manifest.projection_digest_refs != refs {
        return Err(io::Error::other("projection digest refs mismatch"));
    }
    let projection_snapshot_id = fingerprint_bytes(
        serde_json::to_vec(&artifacts.projection_digests)
            .map_err(|error| {
                io::Error::other(format!("serialize model projection digests: {error}"))
            })?
            .as_slice(),
    );
    if artifacts.snapshot_manifest.projection_snapshot_id != projection_snapshot_id {
        return Err(io::Error::other("projection snapshot id mismatch"));
    }
    Ok(())
}

pub fn load_conflict_records(repo_root: &Path) -> io::Result<Vec<KnowledgeConflictRecord>> {
    read_json_lines(&conflict_records_path(repo_root))
}

pub fn restore_runtime_cache_from_artifacts(repo_root: &Path) -> io::Result<RestoreOutcome> {
    if !metadata_exists(repo_root) || !knowledge_artifacts_exist(repo_root) {
        return Ok(RestoreOutcome::not_attempted("restore_artifacts_missing"));
    }

    let metadata = read_metadata(repo_root)?;
    let metadata_json = fs::read(metadata_path(repo_root))?;
    let artifacts = load_knowledge_artifacts(repo_root)?;
    let metadata_hash = fingerprint_bytes(&metadata_json);
    if artifacts.snapshot_manifest.metadata_hash != metadata_hash {
        return Ok(RestoreOutcome::blocked("metadata_hash_mismatch"));
    }
    if !pages_match_metadata_snapshot(repo_root, &metadata)? {
        return Ok(RestoreOutcome::blocked("page_hash_mismatch"));
    }
    let declared_snapshot_id = compute_declared_snapshot_id(&artifacts.declared_records)?;
    if artifacts.snapshot_manifest.declared_snapshot_id != declared_snapshot_id {
        return Ok(RestoreOutcome::blocked("declared_snapshot_mismatch"));
    }
    if validate_research_summary_snapshot(&artifacts.research_summaries).is_err() {
        return Ok(RestoreOutcome::blocked("research_summary_mismatch"));
    }
    if validate_conflict_record_snapshot(&artifacts.conflict_records, &artifacts.declared_records)
        .is_err()
    {
        return Ok(RestoreOutcome::blocked("conflict_snapshot_mismatch"));
    }
    if validate_page_digest_snapshot(&artifacts.page_digests).is_err() {
        return Ok(RestoreOutcome::blocked("projection_digest_mismatch"));
    }
    if validate_page_digests_match_metadata(&metadata, &artifacts.page_digests).is_err() {
        return Ok(RestoreOutcome::blocked(
            "projection_digest_metadata_mismatch",
        ));
    }
    if validate_projection_digest_snapshot_binding(&artifacts).is_err() {
        return Ok(RestoreOutcome::blocked("model_projection_digest_mismatch"));
    }
    let knowledge_snapshot_id = compute_knowledge_snapshot_id(
        &artifacts.knowledge_tree,
        &artifacts.declared_records,
        &artifacts.conflict_records,
        &artifacts.research_summaries,
        &artifacts.page_digests,
        &artifacts.runtime_gates,
        &artifacts.health_signals,
        &artifacts.snapshot_manifest.facts_input_hash,
    )?;
    if artifacts.snapshot_manifest.knowledge_snapshot_id != knowledge_snapshot_id {
        return Ok(RestoreOutcome::blocked("knowledge_snapshot_mismatch"));
    }

    let preserved_llm_cache = sqlite_store::load_all_llm_cache(repo_root).unwrap_or_default();
    ensure_cache_dir(repo_root)?;
    remove_cache_db(repo_root)?;
    ensure_page_cache_dirs(repo_root)?;
    if !preserved_llm_cache.is_empty() {
        sqlite_store::restore_llm_cache(repo_root, &preserved_llm_cache)?;
    }

    let rebuilt_state = rebuild_state_with_sections(repo_root, &metadata)?;
    write_state(repo_root, &rebuilt_state)?;

    let scan_report = rebuild_scan_report(repo_root, &metadata);
    let module_tree = rebuild_module_tree(&metadata);
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    runtime_store.clear_unit_runtime_gates()?;
    sqlite_store::clear_page_digests(&conn)?;
    conn.execute("DELETE FROM knowledge_units", [])
        .map_err(|error| io::Error::other(format!("clear knowledge_units for restore: {error}")))?;
    conn.execute("DELETE FROM knowledge_domains", [])
        .map_err(|error| {
            io::Error::other(format!("clear knowledge_domains for restore: {error}"))
        })?;

    sqlite_store::write_knowledge_domains(&conn, &artifacts.domains)?;
    sqlite_store::write_knowledge_units(&conn, &artifacts.units)?;
    for digest in &artifacts.page_digests {
        let digest_json = serde_json::to_string(digest)
            .map_err(|error| io::Error::other(format!("serialize digest for restore: {error}")))?;
        sqlite_store::write_page_digest(
            &conn,
            &digest.unit_id,
            &digest_json,
            Some(&fingerprint_bytes(digest_json.as_bytes())),
        )?;
    }

    for gate in &artifacts.runtime_gates {
        runtime_store.write_unit_runtime_gate(&UnitRuntimeGate {
            unit_id: gate.unit_id.clone(),
            unit_type: gate.unit_type.clone(),
            research_status: gate.research_status.clone(),
            compose_status: gate.compose_status.clone(),
            assemble_status: gate.assemble_status.clone(),
            last_ready_stage: gate.last_ready_stage.clone(),
            blocked_reason: gate.blocked_reason.clone(),
            missing_dependencies: gate.missing_dependencies.clone(),
            updated_at: gate.updated_at.clone(),
        })?;
    }

    let runtime_summary = rebuild_runtime_summary(&artifacts);
    runtime_store.runtime_meta_set(
        "pipeline_runtime_summary",
        &serde_json::to_string(&runtime_summary)
            .map_err(|error| io::Error::other(format!("serialize runtime summary: {error}")))?,
    )?;
    mark_level1_restored_mirror(repo_root, &artifacts.snapshot_manifest.snapshot_id)?;

    restore_page_caches(
        repo_root,
        &rebuilt_state,
        &artifacts.knowledge_tree,
        &artifacts.page_digests,
        &artifacts.research_summaries,
    )?;

    Ok(RestoreOutcome::restored(Some(
        artifacts.snapshot_manifest.snapshot_id.clone(),
    )))
}

fn rebuild_runtime_summary(artifacts: &KnowledgeArtifactSnapshot) -> PipelineRuntimeSummary {
    let blocked_units = artifacts
        .runtime_gates
        .iter()
        .filter(|gate| gate.blocked_reason.is_some() || !gate.missing_dependencies.is_empty())
        .map(|gate| gate.unit_id.clone())
        .collect::<Vec<_>>();
    let assembled_pages = artifacts
        .runtime_gates
        .iter()
        .filter(|gate| gate.assemble_status == "done")
        .count();
    let runtime_state = if !blocked_units.is_empty() {
        "interrupted".to_string()
    } else if !artifacts.runtime_gates.is_empty()
        && assembled_pages == artifacts.runtime_gates.len()
    {
        "completed".to_string()
    } else {
        "runtime_incomplete".to_string()
    };
    PipelineRuntimeSummary {
        facts_input_hash: artifacts.snapshot_manifest.facts_input_hash.clone(),
        workflow_action: artifacts.snapshot_manifest.workflow_action.clone(),
        runtime_state,
        researched_units: artifacts
            .runtime_gates
            .iter()
            .filter(|gate| gate.research_status == "ready")
            .count(),
        compose_ready_units: artifacts
            .runtime_gates
            .iter()
            .filter(|gate| gate.compose_status == "ready" || gate.compose_status == "done")
            .count(),
        composed_units: artifacts
            .runtime_gates
            .iter()
            .filter(|gate| gate.compose_status == "done")
            .count(),
        assembled_pages,
        blocked_units,
        last_ready_stage: None,
        last_interrupted_stage: None,
        summary_reason: None,
        current_research_unit_id: None,
        current_research_unit_type: None,
        current_research_started_at: None,
        last_researched_unit_id: None,
        last_research_elapsed_ms: None,
    }
}

fn materialize_unit_contracts(
    knowledge_tree: &KnowledgeTree,
    declared_records: &[DeclaredKnowledgeRecord],
    research_summaries: &[KnowledgeResearchSummary],
    page_digests: &[PageDigest],
    runtime_gates: &[KnowledgeRuntimeGateRecord],
    generated_at: &str,
) -> Vec<KnowledgeUnit> {
    let research_by_unit = research_summaries
        .iter()
        .map(|summary| (summary.unit_id.clone(), summary))
        .collect::<BTreeMap<_, _>>();
    let digest_by_unit = page_digests
        .iter()
        .map(|digest| (digest.unit_id.clone(), digest))
        .collect::<BTreeMap<_, _>>();
    let gate_by_unit = runtime_gates
        .iter()
        .map(|gate| (gate.unit_id.clone(), gate))
        .collect::<BTreeMap<_, _>>();
    let mut declared_refs_by_unit = BTreeMap::<String, BTreeSet<String>>::new();
    let mut projection_refs_by_unit = BTreeMap::<String, BTreeSet<String>>::new();

    for record in declared_records {
        for unit_id in &record.unit_refs {
            declared_refs_by_unit
                .entry(unit_id.clone())
                .or_default()
                .insert(record.record_id.clone());
            projection_refs_by_unit
                .entry(unit_id.clone())
                .or_default()
                .extend(record.projection_refs.iter().cloned());
        }
    }

    knowledge_tree
        .units
        .values()
        .cloned()
        .map(|mut unit| {
            let digest = digest_by_unit.get(&unit.id).copied();
            let research = research_by_unit.get(&unit.id).copied();
            let gate = gate_by_unit.get(&unit.id).copied();
            let mut projection_refs = projection_refs_by_unit.remove(&unit.id).unwrap_or_default();
            if let Some(digest) = digest {
                projection_refs.insert(format!("page:{}", digest.page_id));
            }

            unit.declared_record_refs = declared_refs_by_unit
                .remove(&unit.id)
                .unwrap_or_default()
                .into_iter()
                .collect();
            unit.derived_research_ref =
                research.map(|summary| format!("research-summary:{}", summary.unit_id));
            unit.projection_refs = projection_refs.into_iter().collect();
            unit.source_refs = materialize_source_refs(&unit, digest);
            unit.citation_refs = materialize_citation_refs(digest);
            unit.updated_at = generated_at.to_string();

            let (status, invalidation_reason) = derive_unit_status(research, digest, gate);
            unit.status = status;
            unit.invalidation_reason = invalidation_reason;
            unit
        })
        .collect()
}

fn materialize_source_refs(unit: &KnowledgeUnit, digest: Option<&PageDigest>) -> Vec<String> {
    if !unit.scope.source_ids.is_empty() {
        return unit
            .scope
            .source_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
    }

    digest
        .map(|digest| {
            digest
                .key_sources
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        })
        .unwrap_or_default()
}

fn materialize_citation_refs(digest: Option<&PageDigest>) -> Vec<String> {
    let mut refs = BTreeSet::new();
    let Some(digest) = digest else {
        return Vec::new();
    };

    for citation in &digest.citations {
        refs.insert(citation_ref(citation));
    }
    for section in &digest.section_digests {
        for citation in &section.citations {
            refs.insert(citation_ref(citation));
        }
    }
    refs.into_iter().collect()
}

fn citation_ref(citation: &SourceCitation) -> String {
    citation.source_id.clone().unwrap_or_else(|| {
        format!(
            "{}:{}-{}",
            citation.path, citation.start_line, citation.end_line
        )
    })
}

fn derive_unit_status(
    research_summary: Option<&KnowledgeResearchSummary>,
    page_digest: Option<&PageDigest>,
    gate: Option<&KnowledgeRuntimeGateRecord>,
) -> (KnowledgeUnitStatus, Option<String>) {
    if let Some(gate) = gate {
        if gate.blocked_reason.is_some() || !gate.missing_dependencies.is_empty() {
            return (
                KnowledgeUnitStatus::Blocked,
                gate.blocked_reason.clone().or_else(|| {
                    (!gate.missing_dependencies.is_empty())
                        .then(|| "missing_dependencies".to_string())
                }),
            );
        }
    }

    let Some(research_summary) = research_summary else {
        return (
            KnowledgeUnitStatus::Stale,
            Some("missing_research_summary".to_string()),
        );
    };
    match research_summary.summary_status {
        KnowledgeResearchSummaryStatus::Blocked => {
            return (
                KnowledgeUnitStatus::Blocked,
                Some(research_status_invalidation_reason(
                    "research_blocked",
                    research_summary,
                )),
            );
        }
        KnowledgeResearchSummaryStatus::Degraded => {
            return (
                KnowledgeUnitStatus::Stale,
                Some(research_status_invalidation_reason(
                    "research_degraded",
                    research_summary,
                )),
            );
        }
        KnowledgeResearchSummaryStatus::Ready => {}
    }
    let Some(page_digest) = page_digest else {
        return (
            KnowledgeUnitStatus::Stale,
            Some("missing_projection_digest".to_string()),
        );
    };

    match page_digest.projection_status {
        ProjectionDigestStatus::Ready => {}
        ProjectionDigestStatus::Stale => {
            return (
                KnowledgeUnitStatus::Stale,
                Some(projection_status_invalidation_reason(
                    "projection_stale",
                    page_digest,
                )),
            );
        }
        ProjectionDigestStatus::Blocked => {
            return (
                KnowledgeUnitStatus::Blocked,
                Some(projection_status_invalidation_reason(
                    "projection_blocked",
                    page_digest,
                )),
            );
        }
    }

    (KnowledgeUnitStatus::Active, None)
}

fn build_minimal_health_signals(
    domains: &[KnowledgeDomain],
    units: &[KnowledgeUnit],
    conflict_records: &[KnowledgeConflictRecord],
    research_summaries: &[KnowledgeResearchSummary],
    declared_records: &[DeclaredKnowledgeRecord],
    page_digests: &[PageDigest],
) -> Vec<KnowledgeHealthSignal> {
    let domain_ids = domains
        .iter()
        .map(|domain| domain.id.clone())
        .collect::<BTreeSet<_>>();
    let research_by_unit = research_summaries
        .iter()
        .map(|summary| (summary.unit_id.clone(), summary))
        .collect::<BTreeMap<_, _>>();
    let digest_by_unit = page_digests
        .iter()
        .map(|digest| (digest.unit_id.clone(), digest))
        .collect::<BTreeMap<_, _>>();
    let mut signals = Vec::new();

    for unit in units {
        let target_ref = format!("unit:{}", unit.id);
        let research_summary = research_by_unit.get(&unit.id).copied();
        let projection_digest = digest_by_unit.get(&unit.id).copied();
        let domain_missing = !matches!(
            unit.unit_type,
            wiki_model::domain::knowledge::UnitType::Overview
                | wiki_model::domain::knowledge::UnitType::Architecture
        ) && (unit.domain_id.trim().is_empty()
            || !domain_ids.contains(&unit.domain_id));
        let source_missing = unit.source_refs.is_empty();
        let projection_missing = unit.projection_refs.is_empty();

        if domain_missing || (source_missing && projection_missing) {
            let severity = KnowledgeHealthSeverity::Error;
            let action = if domain_missing {
                KnowledgeHealthRecommendedAction::Rebuild
            } else {
                KnowledgeHealthRecommendedAction::Update
            };
            let reason = if domain_missing {
                "unit missing valid domain binding".to_string()
            } else {
                "unit missing source and projection binding".to_string()
            };
            signals.push(build_health_signal(
                "orphan",
                wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::OrphanUnit,
                severity,
                &target_ref,
                action,
                &reason,
            ));
        }

        if source_missing || unit.citation_refs.is_empty() {
            let reason = if source_missing {
                "unit missing source refs".to_string()
            } else {
                "unit missing citation refs".to_string()
            };
            signals.push(build_health_signal(
                "provenance",
                wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::MissingProvenance,
                KnowledgeHealthSeverity::Warning,
                &target_ref,
                KnowledgeHealthRecommendedAction::Review,
                &reason,
            ));
        }

        if unit.status == KnowledgeUnitStatus::Stale
            && unit.invalidation_reason.as_deref() == Some("missing_research_summary")
        {
            signals.push(build_health_signal(
                "derived",
                wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::DerivedStale,
                KnowledgeHealthSeverity::Warning,
                &target_ref,
                KnowledgeHealthRecommendedAction::Update,
                "unit research summary is stale or missing",
            ));
        }
        if let Some(summary) = research_summary {
            match summary.summary_status {
                KnowledgeResearchSummaryStatus::Degraded => {
                    signals.push(build_health_signal(
                        "derived",
                        wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::DerivedStale,
                        KnowledgeHealthSeverity::Warning,
                        &target_ref,
                        KnowledgeHealthRecommendedAction::Update,
                        &format!(
                            "unit research summary degraded: {}",
                            first_research_reason_message(summary)
                        ),
                    ));
                }
                KnowledgeResearchSummaryStatus::Blocked => {
                    signals.push(build_health_signal(
                        "derived",
                        wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::DerivedStale,
                        KnowledgeHealthSeverity::Error,
                        &target_ref,
                        KnowledgeHealthRecommendedAction::Update,
                        &format!(
                            "unit research summary blocked: {}",
                            first_research_reason_message(summary)
                        ),
                    ));
                }
                KnowledgeResearchSummaryStatus::Ready => {}
            }
        }

        if let Some((severity, reason)) = projection_health_signal(unit, projection_digest) {
            signals.push(build_health_signal(
                "projection",
                wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::ProjectionStale,
                severity,
                &target_ref,
                KnowledgeHealthRecommendedAction::Update,
                &reason,
            ));
        }

        if !unit.declared_record_refs.is_empty() && unit.derived_research_ref.is_none() {
            signals.push(build_health_signal(
                "declared-derived",
                wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind::DeclaredDerivedDivergence,
                KnowledgeHealthSeverity::Warning,
                &target_ref,
                KnowledgeHealthRecommendedAction::Update,
                "declared records exist without a matching derived research summary",
            ));
        }
    }

    for record in declared_records {
        if record.status == DeclaredKnowledgeRecordStatus::Active {
            continue;
        }
        let target_ref = format!("declared:{}", record.record_id);
        signals.push(build_health_signal(
            "declared-lifecycle",
            KnowledgeHealthSignalKind::DeclaredLifecycle,
            KnowledgeHealthSeverity::Info,
            &target_ref,
            KnowledgeHealthRecommendedAction::None,
            &format!(
                "declared record '{}' lifecycle={}",
                record.record_id,
                record.status.as_str()
            ),
        ));
    }

    for conflict in conflict_records {
        signals.push(build_health_signal(
            "governance-conflict",
            KnowledgeHealthSignalKind::GovernanceConflict,
            conflict.severity,
            &format!("conflict:{}", conflict.conflict_id),
            KnowledgeHealthRecommendedAction::Review,
            &conflict.reason,
        ));
    }

    signals
}

fn research_status_invalidation_reason(prefix: &str, summary: &KnowledgeResearchSummary) -> String {
    summary
        .status_reasons
        .first()
        .and_then(|reason| reason.reason_kind)
        .map(|reason_kind| format!("{prefix}:{}", reason_kind.as_str()))
        .unwrap_or_else(|| prefix.to_string())
}

fn first_research_reason_message(summary: &KnowledgeResearchSummary) -> String {
    summary
        .status_reasons
        .first()
        .map(|reason| reason.reason_message.clone())
        .unwrap_or_else(|| summary.summary_status.as_str().to_string())
}

fn projection_status_invalidation_reason(prefix: &str, digest: &PageDigest) -> String {
    digest
        .status_reasons
        .first()
        .and_then(|reason| reason.reason_kind)
        .map(|reason_kind| format!("{prefix}:{}", reason_kind.as_str()))
        .unwrap_or_else(|| prefix.to_string())
}

fn first_projection_reason_message(digest: &PageDigest) -> String {
    digest
        .status_reasons
        .first()
        .map(|reason| reason.reason_message.clone())
        .unwrap_or_else(|| digest.projection_status.as_str().to_string())
}

fn projection_health_signal(
    unit: &KnowledgeUnit,
    digest: Option<&PageDigest>,
) -> Option<(KnowledgeHealthSeverity, String)> {
    match unit.invalidation_reason.as_deref() {
        Some("missing_projection_digest") => Some((
            KnowledgeHealthSeverity::Warning,
            "unit projection digest is stale or missing".to_string(),
        )),
        Some(reason) if reason.starts_with("projection_stale") => Some((
            KnowledgeHealthSeverity::Warning,
            format!(
                "unit projection digest is stale: {}",
                digest
                    .map(first_projection_reason_message)
                    .unwrap_or_else(|| reason.to_string())
            ),
        )),
        Some(reason) if reason.starts_with("projection_blocked") => Some((
            KnowledgeHealthSeverity::Error,
            format!(
                "unit projection digest is blocked: {}",
                digest
                    .map(first_projection_reason_message)
                    .unwrap_or_else(|| reason.to_string())
            ),
        )),
        _ => None,
    }
}

fn compute_declared_snapshot_id(records: &[DeclaredKnowledgeRecord]) -> io::Result<String> {
    let snapshot = serde_json::to_vec(records)
        .map_err(|error| io::Error::other(format!("serialize declared snapshot: {error}")))?;
    Ok(fingerprint_bytes(&snapshot))
}

pub fn compute_committed_snapshot_id(facts_input_hash: &str) -> String {
    fingerprint_bytes(format!("committed-snapshot:{facts_input_hash}").as_bytes())
}

fn compute_knowledge_snapshot_id(
    knowledge_tree: &KnowledgeTree,
    declared_records: &[DeclaredKnowledgeRecord],
    conflict_records: &[KnowledgeConflictRecord],
    research_summaries: &[KnowledgeResearchSummary],
    page_digests: &[PageDigest],
    runtime_gates: &[KnowledgeRuntimeGateRecord],
    health_signals: &[KnowledgeHealthSignal],
    facts_input_hash: &str,
) -> io::Result<String> {
    let snapshot_seed = serde_json::to_vec(&(
        knowledge_tree,
        declared_records,
        conflict_records,
        research_summaries,
        page_digests,
        runtime_gates,
        health_signals,
        facts_input_hash,
    ))
    .map_err(|error| io::Error::other(format!("serialize knowledge snapshot: {error}")))?;
    Ok(fingerprint_bytes(&snapshot_seed))
}

fn derive_declared_conflicts(
    declared_records: &[DeclaredKnowledgeRecord],
    detected_at: &str,
) -> Vec<KnowledgeConflictRecord> {
    let mut grouped = BTreeMap::<(String, String), Vec<&DeclaredKnowledgeRecord>>::new();
    for record in declared_records {
        grouped
            .entry((
                record.record_kind.as_str().to_string(),
                record.scope.canonical_key(),
            ))
            .or_default()
            .push(record);
    }

    let mut conflicts = Vec::new();
    for ((_kind, _scope_key), records) in grouped {
        let active_records = records
            .iter()
            .filter(|record| record.status == DeclaredKnowledgeRecordStatus::Active)
            .copied()
            .collect::<Vec<_>>();
        if active_records.len() > 1 {
            conflicts.push(build_declared_conflict_record(
                KnowledgeConflictKind::ParallelActiveDeclared,
                &active_records,
                "同一 kind + canonical scope 下存在多条 active declared records",
                detected_at,
            ));
            continue;
        }

        let head_records = lifecycle_head_records(&records);
        if head_records.len() > 1 {
            conflicts.push(build_declared_conflict_record(
                KnowledgeConflictKind::LifecycleHeadAmbiguity,
                &head_records,
                "declared lifecycle 无法推出唯一 authoritative head",
                detected_at,
            ));
        }
    }

    conflicts.sort_by(|left, right| left.conflict_id.cmp(&right.conflict_id));
    conflicts
}

fn lifecycle_head_records<'a>(
    records: &'a [&DeclaredKnowledgeRecord],
) -> Vec<&'a DeclaredKnowledgeRecord> {
    let mut outgoing = BTreeMap::<String, BTreeSet<String>>::new();

    for record in records {
        for relation in &record.relations {
            match relation.relation_kind {
                DeclaredKnowledgeRelationKind::ReplacedBy => {
                    if let Some(target) = relation.target_record_ref.as_ref() {
                        outgoing
                            .entry(record.authoring_id.clone())
                            .or_default()
                            .insert(target.clone());
                    }
                }
                DeclaredKnowledgeRelationKind::Supersedes => {
                    if let Some(target) = relation.target_record_ref.as_ref() {
                        outgoing
                            .entry(target.clone())
                            .or_default()
                            .insert(record.authoring_id.clone());
                    }
                }
                DeclaredKnowledgeRelationKind::Deprecated => {}
            }
        }
    }

    records
        .iter()
        .copied()
        .filter(|record| {
            matches!(
                record.status,
                DeclaredKnowledgeRecordStatus::Active | DeclaredKnowledgeRecordStatus::Replaced
            ) && outgoing
                .get(record.authoring_id.as_str())
                .map(|targets| targets.is_empty())
                .unwrap_or(true)
        })
        .collect()
}

fn build_declared_conflict_record(
    conflict_kind: KnowledgeConflictKind,
    records: &[&DeclaredKnowledgeRecord],
    reason: &str,
    detected_at: &str,
) -> KnowledgeConflictRecord {
    let scope = records
        .first()
        .map(|record| record.scope.clone())
        .unwrap_or_default();
    let scope_key = scope.canonical_key();
    let mut conflict = KnowledgeConflictRecord {
        conflict_id: crate::domain::stable_id::stable_id(
            "conflict",
            format!("{}:{scope_key}", conflict_kind.as_str()),
        ),
        conflict_kind,
        status: KnowledgeConflictStatus::Open,
        severity: KnowledgeHealthSeverity::Warning,
        scope,
        record_ids: records
            .iter()
            .map(|record| record.record_id.clone())
            .collect(),
        authoring_ids: records
            .iter()
            .map(|record| record.authoring_id.clone())
            .collect(),
        unit_refs: records
            .iter()
            .flat_map(|record| record.unit_refs.iter().cloned())
            .collect(),
        projection_refs: records
            .iter()
            .flat_map(|record| record.projection_refs.iter().cloned())
            .collect(),
        reason: reason.to_string(),
        detected_at: detected_at.to_string(),
    };
    conflict.canonicalize();
    conflict
}

fn pages_match_metadata_snapshot(repo_root: &Path, metadata: &WikiMetadata) -> io::Result<bool> {
    for item in &metadata.wiki_items {
        if !crate::storage::wiki_fs::is_official_page_path(&item.path) {
            return Ok(false);
        }
        let page_path = resolve_page_path(repo_root, &item.path);
        if !page_path.exists() {
            return Ok(false);
        }
        let content = fs::read_to_string(&page_path)?;
        if fingerprint_bytes(content.as_bytes()) != item.content_hash {
            return Ok(false);
        }
    }
    Ok(true)
}

fn validate_page_digests_match_metadata(
    metadata: &WikiMetadata,
    digests: &[PageDigest],
) -> Result<(), String> {
    let pages_by_id = metadata
        .wiki_items
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<BTreeMap<_, _>>();

    for digest in digests {
        let Some(page) = pages_by_id.get(digest.page_id.as_str()) else {
            return Err(format!(
                "page digest '{}' 引用了不存在的 page '{}'",
                digest.digest_id, digest.page_id
            ));
        };
        if page.title.trim() != digest.title.trim() {
            return Err(format!(
                "page digest '{}' 与 metadata page '{}' title 不一致",
                digest.digest_id, digest.page_id
            ));
        }
    }

    Ok(())
}

fn build_health_signal(
    namespace: &str,
    signal_kind: wiki_model::domain::knowledge_artifact::KnowledgeHealthSignalKind,
    severity: KnowledgeHealthSeverity,
    target_ref: &str,
    recommended_action: KnowledgeHealthRecommendedAction,
    reason: &str,
) -> KnowledgeHealthSignal {
    let signal_id = fingerprint_bytes(
        format!(
            "{namespace}:{}:{}:{}:{}",
            signal_kind.as_str(),
            severity.as_str(),
            target_ref,
            reason
        )
        .as_bytes(),
    );
    KnowledgeHealthSignal {
        signal_id,
        signal_kind,
        severity,
        target_ref: target_ref.to_string(),
        recommended_action,
        reason: reason.to_string(),
    }
}

fn restore_page_caches(
    repo_root: &Path,
    rebuilt_state: &WikiState,
    knowledge_tree: &KnowledgeTree,
    page_digests: &[PageDigest],
    research_summaries: &[KnowledgeResearchSummary],
) -> io::Result<()> {
    let planned_pages = plan_pages_from_knowledge_tree(knowledge_tree)
        .into_iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let digest_by_unit = page_digests
        .iter()
        .map(|digest| (digest.unit_id.clone(), digest.clone()))
        .collect::<BTreeMap<_, _>>();
    let research_by_unit = research_summaries
        .iter()
        .map(|summary| {
            let mut research = UnitResearch::default();
            research.unit_id = summary.unit_id.clone();
            research.input_hash = summary.input_hash.clone();
            (summary.unit_id.clone(), research)
        })
        .collect::<BTreeMap<_, _>>();

    for page in &rebuilt_state.pages {
        if !crate::storage::wiki_fs::is_official_page_path(&page.path) {
            continue;
        }
        let Some(planned_page) = planned_pages.get(&page.page_id) else {
            continue;
        };
        let Some(unit_id) = planned_page.unit_id.as_ref() else {
            continue;
        };
        let Some(unit) = knowledge_tree.get_unit(unit_id) else {
            continue;
        };
        let managed_sections = load_managed_sections(repo_root, page)?;
        let draft = PageDraft {
            page_id: page.page_id.clone(),
            unit_id: unit.id.clone(),
            title: planned_page.title.clone(),
            relative_path: planned_page.relative_path.clone(),
            sections: Vec::new(),
            diagrams: Vec::new(),
            citation_count: 0,
        };
        let context = build_restored_page_context(
            unit,
            planned_page,
            &draft,
            knowledge_tree,
            &digest_by_unit,
            &research_by_unit,
            page,
        );
        write_page_context_cache(
            repo_root,
            &PageContextCacheEntry {
                page_id: page.page_id.clone(),
                input_hash: page.input_hash.clone(),
                context,
            },
        )?;
        write_page_generation_cache(
            repo_root,
            &PageGenerationCacheEntry {
                page_id: page.page_id.clone(),
                input_hash: page.input_hash.clone(),
                content_hash: page.content_hash.clone(),
                sections: managed_sections,
            },
        )?;
    }

    Ok(())
}

fn build_restored_page_context(
    unit: &KnowledgeUnit,
    planned_page: &wiki_knowledge::PagePlan,
    draft: &PageDraft,
    knowledge_tree: &KnowledgeTree,
    digest_by_unit: &BTreeMap<String, PageDigest>,
    research_by_unit: &BTreeMap<String, UnitResearch>,
    page: &wiki_model::domain::state::WikiPageState,
) -> PageContext {
    let mut ctx = PageContext::default();
    let child_digests = unit
        .child_unit_ids
        .iter()
        .filter_map(|child_id| digest_by_unit.get(child_id))
        .cloned()
        .collect::<Vec<_>>();

    ctx.page_id = planned_page.id.clone();
    ctx.page_type = planned_page.page_type.clone();
    ctx.scope = planned_page.scope.clone();
    ctx.unit_id = planned_page.unit_id.clone();
    ctx.unit_type = planned_page.unit_type.clone();
    ctx.domain_id = planned_page.domain_id.clone();
    ctx.source_ids = unit.scope.source_ids.clone();
    ctx.module_ids = unit.scope.module_ids.clone();
    ctx.relation_ids = unit.scope.relation_ids.clone();
    ctx.facts = vec![
        format!("unit={}", unit.id),
        format!("unit_type={}", unit.unit_type.as_str()),
        format!("relative_path={}", unit.relative_path),
    ];
    ctx.summary_inputs = vec![draft.title.clone()];
    ctx.child_summaries = child_digests
        .iter()
        .map(|digest| format!("{}: {}", digest.title, digest.summary))
        .collect();
    ctx.child_unit_ids = child_digests
        .iter()
        .map(|digest| digest.unit_id.clone())
        .collect();
    ctx.child_page_ids = child_digests
        .iter()
        .map(|digest| digest.page_id.clone())
        .collect();
    ctx.child_digest_ids = child_digests
        .iter()
        .map(|digest| digest.digest_id.clone())
        .collect();
    ctx.missing_child_unit_ids = unit
        .child_unit_ids
        .iter()
        .filter(|child_id| {
            !ctx.child_unit_ids
                .iter()
                .any(|existing| existing == *child_id)
        })
        .cloned()
        .collect();
    ctx.readiness_status = if ctx.missing_child_unit_ids.is_empty() {
        "compose_ready".to_string()
    } else {
        "waiting_children".to_string()
    };
    if let Some(unit_research) = research_by_unit.get(&unit.id) {
        ctx.has_unit_research_contract = true;
        if !unit_research.input_hash.trim().is_empty() {
            ctx.unit_research_input_hash = Some(unit_research.input_hash.clone());
        }
    }
    ctx.citation_digest_refs = child_digests
        .iter()
        .flat_map(|digest| {
            digest
                .section_digests
                .iter()
                .filter(|section| !section.citations.is_empty())
                .map(|section| section.digest_id.clone())
        })
        .collect();
    ctx.diagram_digest_refs = child_digests
        .iter()
        .flat_map(|digest| {
            digest
                .diagram_digests
                .iter()
                .map(|diagram| diagram.digest_id.clone())
        })
        .collect();
    if ctx.source_ids.is_empty() {
        ctx.source_ids = page.source_ids.clone();
    }
    if ctx.module_ids.is_empty() {
        ctx.module_ids = page.module_ids.clone();
    }
    if knowledge_tree.get_unit(&unit.id).is_none() {
        ctx.readiness_status = "restored".to_string();
    }
    ctx
}

fn rebuild_state_with_sections(repo_root: &Path, metadata: &WikiMetadata) -> io::Result<WikiState> {
    let mut state = rebuild_state_from_metadata(metadata);
    let source_ids_by_page = build_source_ids_by_page(metadata);

    for page in &mut state.pages {
        page.source_ids = source_ids_by_page
            .get(&page.page_id)
            .cloned()
            .unwrap_or_default();
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = fs::read_to_string(&page_path)?;
        page.content_hash = fingerprint_bytes(content.as_bytes());
        let managed_sections = parse_page_sections(page.page_type.as_str(), &content);
        page.summary = extract_summary_from_managed_sections(&managed_sections);
        page.section_anchors = managed_sections
            .iter()
            .filter(|section| section.managed)
            .map(|section| section.section_id.clone())
            .collect();
        page.sections = managed_sections;
    }

    Ok(state)
}

fn parse_page_sections(page_type: &str, content: &str) -> Vec<WikiSectionState> {
    let _ = page_type;
    let parsed = parse_wiki_page(
        content,
        &crate::generation::managed_sections::SectionBindingIndex::default(),
    );

    parsed
        .blocks
        .iter()
        .map(|block| match block {
            PageBlock::Managed(managed) => WikiSectionState {
                section_id: managed.section_id.clone(),
                title: managed.title.clone(),
                managed: true,
                owner_kind: Some(managed.owner_kind),
                knowledge_refs: managed.knowledge_refs.clone(),
                content_hash: content_hash(&managed.body),
                input_hash: managed.input_hash.clone(),
                // restore 后写回的 generated baseline 必须剥离 declared block，
                // 否则后续 sync 会把 authoring surface 误当成 managed truth。
                generated_content_hash: Some(content_hash(
                    strip_declared_blocks_from_managed_body(&managed.body).as_str(),
                )),
                projection_digest_ref: managed.projection_digest_ref.clone(),
                anchor_after_section_id: None,
                anchor_before_section_id: None,
                source_ids: Vec::new(),
                relation_ids: Vec::new(),
            },
            PageBlock::User(user) => WikiSectionState {
                section_id: user.id.clone(),
                title: String::new(),
                managed: false,
                owner_kind: Some(wiki_model::domain::projection::SectionOwnership::ManualUnmanaged),
                knowledge_refs: Vec::new(),
                content_hash: content_hash(&user.body),
                input_hash: String::new(),
                generated_content_hash: None,
                projection_digest_ref: None,
                anchor_after_section_id: user.anchor_after_section_id.clone(),
                anchor_before_section_id: user.anchor_before_section_id.clone(),
                source_ids: Vec::new(),
                relation_ids: Vec::new(),
            },
        })
        .collect()
}

fn strip_declared_blocks_from_managed_body(body: &str) -> String {
    let lines = body.lines().collect::<Vec<_>>();
    let mut cleaned_lines = Vec::new();
    let mut in_declared_block = false;

    for line in lines {
        if line.trim_start().starts_with(DECLARED_START_PREFIX) {
            in_declared_block = true;
            continue;
        }
        if in_declared_block {
            if line.trim() == DECLARED_END_MARKER {
                in_declared_block = false;
            }
            continue;
        }
        cleaned_lines.push(line);
    }

    cleaned_lines.join("\n").trim().to_string()
}

fn load_managed_sections(
    repo_root: &Path,
    page: &wiki_model::domain::state::WikiPageState,
) -> io::Result<Vec<SectionDraft>> {
    let content = fs::read_to_string(resolve_page_path(repo_root, &page.path))?;
    let parsed = parse_wiki_page(
        &content,
        &crate::generation::managed_sections::SectionBindingIndex::default(),
    );

    Ok(parsed
        .blocks
        .iter()
        .filter_map(|block| match block {
            PageBlock::Managed(managed) => Some(SectionDraft {
                section_id: managed.section_id.clone(),
                title: managed.title.clone(),
                managed: true,
                source_ids: page.source_ids.clone(),
                relation_ids: Vec::new(),
                content: managed.body.clone(),
            }),
            PageBlock::User(_) => None,
        })
        .collect())
}

fn extract_summary_from_managed_sections(sections: &[WikiSectionState]) -> String {
    sections
        .iter()
        .find(|section| section.managed)
        .map(|section| section.title.clone())
        .unwrap_or_default()
}

fn build_source_ids_by_page(metadata: &WikiMetadata) -> BTreeMap<String, Vec<String>> {
    let mut source_ids_by_page = BTreeMap::new();
    for source in &metadata.source_files {
        for page_id in &source.wiki_item_ids {
            source_ids_by_page
                .entry(page_id.clone())
                .or_insert_with(Vec::new)
                .push(source.id.clone());
        }
    }
    source_ids_by_page
}

fn rebuild_module_tree(metadata: &WikiMetadata) -> ModuleTree {
    ModuleTree {
        root_modules: metadata
            .modules
            .iter()
            .filter(|module| module.parent_id.is_none())
            .map(|module| module.id.clone())
            .collect(),
        modules: metadata.modules.clone(),
        cross_module_edges: Vec::new(),
        architecture_hints: Vec::new(),
    }
}

fn rebuild_scan_report(repo_root: &Path, metadata: &WikiMetadata) -> ScanReport {
    let entry_points = metadata
        .modules
        .iter()
        .flat_map(|module| module.entry_points.iter().cloned())
        .collect::<BTreeSet<_>>();
    let files = metadata
        .source_files
        .iter()
        .map(|source| rebuild_scanned_file(source, &entry_points))
        .collect::<Vec<_>>();

    ScanReport {
        root: repo_root.to_string_lossy().to_string(),
        config_files: files
            .iter()
            .filter(|file| file.kind == "config")
            .map(|file| file.path.clone())
            .collect(),
        entry_points: entry_points.into_iter().collect(),
        files,
        tech_hints: Vec::new(),
        workspace_roots: vec![".".to_string()],
        dependency_hints: Vec::new(),
    }
}

fn rebuild_scanned_file(
    source: &crate::domain::metadata::SourceFileRecord,
    entry_points: &BTreeSet<String>,
) -> ScannedFile {
    let purpose = infer_file_purpose(&source.path);
    let kind = purpose.family().to_string();
    let mut tags = Vec::new();
    if entry_points.contains(&source.path) {
        tags.push("entry-point".to_string());
    }
    if purpose == FilePurpose::Test {
        tags.push("test-file".to_string());
    }

    ScannedFile {
        id: source.id.clone(),
        path: source.path.clone(),
        language: infer_language(&source.path),
        kind,
        purpose,
        fingerprint: source.fingerprint.clone(),
        size: 0,
        tags,
    }
}

fn infer_language(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "go",
        "java" => "java",
        "kt" | "kts" => "kotlin",
        "swift" => "swift",
        "php" => "php",
        "md" | "mdx" => "markdown",
        "json" | "yaml" | "yml" | "toml" => "config",
        _ => "unknown",
    }
    .to_string()
}

fn infer_file_purpose(path: &str) -> FilePurpose {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".md") || lower.ends_with(".mdx") || lower.contains("/docs/") {
        return FilePurpose::Docs;
    }
    if lower.ends_with(".json")
        || lower.ends_with(".yaml")
        || lower.ends_with(".yml")
        || lower.ends_with(".toml")
    {
        return FilePurpose::Config;
    }
    if lower.contains(".test.") || lower.contains(".spec.") || lower.contains("/tests/") {
        return FilePurpose::Test;
    }
    FilePurpose::Utility
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| io::Error::other(format!("serialize {}: {error}", path.display())))?;
    fs::write(path, json)
}

fn write_json_lines<T: Serialize>(path: &Path, values: &[T]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut output = String::new();
    for value in values {
        output.push_str(&serde_json::to_string(value).map_err(|error| {
            io::Error::other(format!("serialize jsonl {}: {error}", path.display()))
        })?);
        output.push('\n');
    }
    fs::write(path, output)
}

fn write_yaml<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let yaml = serde_yaml::to_string(value)
        .map_err(|error| io::Error::other(format!("serialize yaml {}: {error}", path.display())))?;
    fs::write(path, yaml)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    let raw = fs::read_to_string(path)?;
    serde_json::from_str(&raw)
        .map_err(|error| io::Error::other(format!("parse {}: {error}", path.display())))
}

fn read_yaml<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    let raw = fs::read_to_string(path)?;
    serde_yaml::from_str(&raw)
        .map_err(|error| io::Error::other(format!("parse yaml {}: {error}", path.display())))
}

fn read_snapshot_manifest(repo_root: &Path) -> io::Result<CommittedSnapshotManifest> {
    let (snapshot_id, path) = current_snapshot_manifest_path(repo_root)?;
    let manifest: CommittedSnapshotManifest = read_yaml(&path)?;
    if manifest.snapshot_id != snapshot_id {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "committed snapshot id mismatch: metadata={snapshot_id}, manifest={}",
                manifest.snapshot_id
            ),
        ));
    }
    Ok(manifest)
}

fn read_json_lines<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<Vec<T>> {
    let raw = fs::read_to_string(path)?;
    raw.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|error| io::Error::other(format!("parse {}: {error}", path.display())))
        })
        .collect()
}
