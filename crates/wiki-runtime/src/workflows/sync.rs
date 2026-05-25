//! `sync` 负责把用户对 `.wiki/*.md` 的外部修改同步回 WikiState、metadata 与 formal artifacts。
//! 当前 declared writeback 维持 page-level atomic：只要页面存在非法 drift，该页 declared truth 就不提交。

use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::metadata_mapper::{export_metadata, ExportContext};
use crate::domain::state::{WikiPageState, WikiSectionState};
use crate::domain::steering::{load_steering_config_with_mode, SteeringLoadMode};
use crate::generation::managed_sections::{
    content_hash, parse_wiki_page, PageBlock, ParsedWikiPage,
};
use crate::generation::sections::section_titles_for_page_type;
use crate::repo::git::{current_branch, current_commit};
use crate::storage::cache_store::{
    read_page_context_cache, read_page_generation_cache, write_module_tree_cache, write_scan_cache,
};
use crate::storage::knowledge_artifacts::{
    knowledge_artifacts_exist, load_knowledge_artifacts, persist_knowledge_artifacts,
    restore_runtime_cache_from_artifacts, PersistKnowledgeArtifactsInput,
};
use crate::storage::metadata_store::write_metadata;
use crate::storage::sqlite_store;
use crate::storage::state_store::{load_or_rebuild_state, write_state};
use crate::storage::wiki_fs::resolve_page_path;
use crate::workflows::init::current_timestamp;
use wiki_index::fingerprint::fingerprint_bytes;
use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::scan_repo_with_boundary;
use wiki_model::domain::knowledge_artifact::{
    validate_declared_record_snapshot, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordKind,
    DeclaredKnowledgeRecordStatus, DeclaredKnowledgeRelation, DeclaredKnowledgeRelationKind,
    DeclaredKnowledgeScope, DeclaredKnowledgeScopeKind, KnowledgeHealthRecommendedAction,
    KnowledgeHealthSeverity, KnowledgeHealthSignal, KnowledgeHealthSignalKind,
};
use wiki_model::domain::stable_id::stable_id;

const DECLARED_START_PREFIX: &str = "<!-- wiki:declared";
const DECLARED_END_MARKER: &str = "<!-- wiki:declared:end -->";

/// `sync` 的分类结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncResultKind {
    DeclaredWriteback,
    MetadataOnly,
    IllegalDrift,
}

impl SyncResultKind {
    fn recommended_action(self) -> &'static str {
        match self {
            Self::DeclaredWriteback => "update",
            Self::MetadataOnly => "none",
            Self::IllegalDrift => "rebuild",
        }
    }
}

/// 单页 sync 结果。
#[derive(Debug, Clone, Serialize)]
pub struct SyncPageOutcome {
    /// 受影响页面的稳定标识。
    pub page_id: String,
    /// 页面在 `.wiki/` 下的相对路径。
    pub path: String,
    /// 本页本次同步的正式分类。
    pub result_kind: SyncResultKind,
    /// 本次 page-scoped declared snapshot diff 命中的 record ids。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_record_ids: Vec<String>,
    /// 本次 declared lifecycle 变化触发的 stale units。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stale_unit_ids: Vec<String>,
    /// 本次 declared lifecycle 变化触发的 stale projections。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stale_projection_ids: Vec<String>,
    /// 本页被判为 metadata-only / declared_writeback / illegal_drift 的原因。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
    /// 宿主应采取的下一步动作。
    pub recommended_action: String,
}

/// `sync` 的输出报告。
#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    /// 同步后的运行时状态。
    pub state: String,
    /// 本次被识别为变化的页面路径列表。
    pub synced_pages: Vec<String>,
    /// 本次变更的分类结果。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub page_outcomes: Vec<SyncPageOutcome>,
    /// 同步过程中产生的警告信息。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct SyncPageAnalysis {
    result_kind: SyncResultKind,
    reasons: Vec<String>,
    declared_records: Vec<DeclaredKnowledgeRecord>,
    affected_declared_record_ids: Vec<String>,
    stale_unit_ids: Vec<String>,
    stale_projection_ids: Vec<String>,
    health_signals: Vec<KnowledgeHealthSignal>,
}

#[derive(Debug, Clone)]
struct DeclaredBlock {
    explicit_id: Option<String>,
    record_kind: DeclaredKnowledgeRecordKind,
    scope: DeclaredKnowledgeScope,
    status: DeclaredKnowledgeRecordStatus,
    relations: Vec<DeclaredKnowledgeRelation>,
    source_ref: String,
    body: String,
}

/// parser-first sync：解析页面区段结构，回写 section 状态、summary、metadata 与 formal declared artifacts。
pub fn run_sync(repo_root: &Path) -> io::Result<SyncReport> {
    run_sync_with_mode(repo_root, SteeringLoadMode::Production)
}

pub fn run_sync_with_mode(
    repo_root: &Path,
    steering_mode: SteeringLoadMode,
) -> io::Result<SyncReport> {
    let mut wiki_state = load_or_rebuild_state(repo_root)?;
    let mut synced_pages = Vec::new();
    let mut all_warnings = Vec::new();
    let mut page_outcomes = Vec::new();
    let mut declared_writeback_records = HashMap::<String, Vec<DeclaredKnowledgeRecord>>::new();
    let mut custom_health_signals = Vec::new();
    let previous_declared_records_by_page = if knowledge_artifacts_exist(repo_root) {
        let mut records_by_page = HashMap::<String, Vec<DeclaredKnowledgeRecord>>::new();
        for record in load_knowledge_artifacts(repo_root)?.declared_records {
            records_by_page
                .entry(record.page_id.clone())
                .or_default()
                .push(record);
        }
        records_by_page
    } else {
        HashMap::new()
    };

    for page in &mut wiki_state.pages {
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = match fs::read_to_string(&page_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let new_hash = fingerprint_bytes(content.as_bytes());
        if page.content_hash == new_hash {
            continue;
        }

        let known_titles = section_titles_for_page_type(&page.page_type);
        let known_titles_ref: Vec<&str> = known_titles.iter().copied().collect();
        let parsed = parse_wiki_page(&content, &known_titles_ref);
        let generated_hash_map = load_generated_hashes(repo_root, page);
        let unit_id = read_page_context_cache(repo_root, &page.page_id)
            .ok()
            .and_then(|entry| entry.context.unit_id);
        let previous_page_declared_records = previous_declared_records_by_page
            .get(&page.page_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let analysis = analyze_sync_page(
            page,
            &parsed,
            &generated_hash_map,
            unit_id.as_deref(),
            previous_page_declared_records,
        );
        let new_sections = build_section_states_from_parsed(&parsed, &generated_hash_map);

        for reason in &analysis.reasons {
            all_warnings.push(format!("[{}] {}", page.path, reason));
        }

        page.content_hash = new_hash;
        if !parsed.title.trim().is_empty() {
            page.title = parsed.title.trim().to_string();
        }
        page.sections = new_sections;
        page.section_anchors = page
            .sections
            .iter()
            .filter(|section| section.managed)
            .map(|section| section.section_id.clone())
            .collect();
        page.summary = extract_summary_from_parsed(&parsed);

        if analysis.result_kind == SyncResultKind::DeclaredWriteback {
            declared_writeback_records
                .insert(page.page_id.clone(), analysis.declared_records.clone());
        }
        custom_health_signals.extend(analysis.health_signals.clone());
        synced_pages.push(page.path.clone());
        page_outcomes.push(SyncPageOutcome {
            page_id: page.page_id.clone(),
            path: page.path.clone(),
            result_kind: analysis.result_kind,
            declared_record_ids: analysis.affected_declared_record_ids,
            stale_unit_ids: analysis.stale_unit_ids,
            stale_projection_ids: analysis.stale_projection_ids,
            reasons: analysis.reasons,
            recommended_action: analysis.result_kind.recommended_action().to_string(),
        });
    }

    let generated_at = current_timestamp();
    wiki_state.dirty_state = crate::domain::metadata::DirtyState::fresh();
    wiki_state.build_state.generated_at = generated_at.clone();
    write_state(repo_root, &wiki_state)?;

    let export_context = ExportContext {
        schema_version: "1".to_string(),
        language: "zh".to_string(),
        repo_root: repo_root.to_string_lossy().to_string(),
        branch: current_branch(repo_root),
        generated_at: generated_at.clone(),
        last_indexed_commit: current_commit(repo_root),
    };
    let metadata = export_metadata(&wiki_state, &export_context);
    write_metadata(repo_root, &metadata)?;

    let steering = load_steering_config_with_mode(repo_root, steering_mode);
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths)?;
    let module_tree = build_module_tree(&scan_report);
    write_scan_cache(repo_root, &scan_report)?;
    write_module_tree_cache(repo_root, &module_tree)?;

    if knowledge_artifacts_exist(repo_root) {
        persist_sync_artifacts(
            repo_root,
            &metadata,
            &generated_at,
            &declared_writeback_records,
            &custom_health_signals,
        )?;
        let _ = restore_runtime_cache_from_artifacts(repo_root)?;
    }

    Ok(SyncReport {
        state: "fresh".to_string(),
        synced_pages,
        page_outcomes,
        warnings: all_warnings,
    })
}

fn persist_sync_artifacts(
    repo_root: &Path,
    metadata: &crate::domain::metadata::WikiMetadata,
    generated_at: &str,
    declared_writeback_records: &HashMap<String, Vec<DeclaredKnowledgeRecord>>,
    custom_health_signals: &[KnowledgeHealthSignal],
) -> io::Result<()> {
    let artifacts = load_knowledge_artifacts(repo_root)?;
    let mut declared_records = artifacts
        .declared_records
        .into_iter()
        .filter(|record| !declared_writeback_records.contains_key(&record.page_id))
        .collect::<Vec<_>>();
    for records in declared_writeback_records.values() {
        declared_records.extend(records.iter().cloned());
    }
    validate_declared_record_snapshot(&declared_records)
        .map_err(|error| io::Error::other(format!("validate sync declared snapshot: {error}")))?;

    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_gates = sqlite_store::read_unit_runtime_gates(&conn)?;
    persist_knowledge_artifacts(PersistKnowledgeArtifactsInput {
        repo_root,
        workflow_action: "sync",
        generated_at,
        facts_input_hash: &artifacts.recovery_manifest.facts_input_hash,
        metadata,
        knowledge_tree: &artifacts.knowledge_tree,
        declared_records: &declared_records,
        research_summaries: &artifacts.research_summaries,
        page_digests: &artifacts.page_digests,
        runtime_gates: &runtime_gates,
        health_signals: custom_health_signals,
    })
}

fn analyze_sync_page(
    page: &WikiPageState,
    parsed: &ParsedWikiPage,
    generated_hash_map: &HashMap<String, String>,
    unit_id: Option<&str>,
    previous_page_declared_records: &[DeclaredKnowledgeRecord],
) -> SyncPageAnalysis {
    let mut reasons = parsed.warnings.clone();
    let mut declared_records = Vec::new();
    let mut seen_authoring_ids = BTreeSet::new();
    let mut illegal_drift = !parsed.warnings.is_empty();

    for block in &parsed.blocks {
        let PageBlock::Managed(managed) = block else {
            continue;
        };
        let parse_result = parse_declared_blocks(&managed.body);
        let (declared_blocks, cleaned_body) = match parse_result {
            Ok(result) => result,
            Err(parse_error) => {
                reasons.push(parse_error.to_string());
                illegal_drift = true;
                continue;
            }
        };

        let Some(generated_hash) = generated_hash_map.get(&managed.section_id) else {
            reasons.push(format!(
                "managed section '{}' 缺少 generated baseline，无法确认 writeback 合法性",
                managed.section_id
            ));
            illegal_drift = true;
            continue;
        };

        let cleaned_hash = content_hash(&cleaned_body);
        if cleaned_hash != *generated_hash {
            reasons.push(format!(
                "managed section '{}' 存在非 declared 的正文漂移",
                managed.section_id
            ));
            illegal_drift = true;
            continue;
        }

        for declared in declared_blocks {
            match materialize_declared_record(
                page,
                managed.section_id.as_str(),
                unit_id,
                &declared,
                &mut seen_authoring_ids,
            ) {
                Ok(record) => declared_records.push(record),
                Err(error) => {
                    reasons.push(error.to_string());
                    illegal_drift = true;
                }
            }
        }
    }

    let snapshot_diff =
        diff_page_declared_snapshot(previous_page_declared_records, &declared_records);
    let has_declared_lifecycle_change = !snapshot_diff.affected_declared_record_ids.is_empty();
    let result_kind = if illegal_drift {
        SyncResultKind::IllegalDrift
    } else if has_declared_lifecycle_change {
        SyncResultKind::DeclaredWriteback
    } else {
        SyncResultKind::MetadataOnly
    };
    let committed_declared_record_ids = if result_kind == SyncResultKind::DeclaredWriteback {
        snapshot_diff.affected_declared_record_ids.clone()
    } else {
        Vec::new()
    };
    let stale_unit_ids = if result_kind == SyncResultKind::DeclaredWriteback {
        let mut unit_ids = snapshot_diff.stale_unit_ids;
        if unit_ids.is_empty() {
            if let Some(unit_id) = unit_id {
                unit_ids.push(unit_id.to_string());
            }
        }
        unit_ids
    } else {
        Vec::new()
    };
    let stale_projection_ids = if result_kind == SyncResultKind::DeclaredWriteback {
        let mut projection_ids = snapshot_diff.stale_projection_ids;
        if projection_ids.is_empty() {
            projection_ids.push(page.page_id.clone());
        }
        projection_ids
    } else {
        Vec::new()
    };
    let health_signals = build_sync_health_signals(
        page,
        unit_id,
        result_kind,
        has_declared_lifecycle_change,
        &reasons,
    );

    SyncPageAnalysis {
        result_kind,
        reasons,
        declared_records,
        affected_declared_record_ids: committed_declared_record_ids,
        stale_unit_ids,
        stale_projection_ids,
        health_signals,
    }
}

#[derive(Debug, Default)]
struct PageDeclaredSnapshotDiff {
    affected_declared_record_ids: Vec<String>,
    stale_unit_ids: Vec<String>,
    stale_projection_ids: Vec<String>,
}

fn diff_page_declared_snapshot(
    previous_records: &[DeclaredKnowledgeRecord],
    current_records: &[DeclaredKnowledgeRecord],
) -> PageDeclaredSnapshotDiff {
    let previous_by_id = previous_records
        .iter()
        .cloned()
        .map(|record| (record.record_id.clone(), record))
        .collect::<BTreeMap<_, _>>();
    let current_by_id = current_records
        .iter()
        .cloned()
        .map(|record| (record.record_id.clone(), record))
        .collect::<BTreeMap<_, _>>();

    let mut affected_declared_record_ids = BTreeSet::new();
    let mut stale_unit_ids = BTreeSet::new();
    let mut stale_projection_ids = BTreeSet::new();

    for (record_id, previous_record) in &previous_by_id {
        match current_by_id.get(record_id) {
            Some(current_record)
                if declared_records_match_for_snapshot(current_record, previous_record) => {}
            Some(current_record) => {
                affected_declared_record_ids.insert(record_id.clone());
                collect_stale_targets_for_declared_record(
                    previous_record,
                    &mut stale_unit_ids,
                    &mut stale_projection_ids,
                );
                collect_stale_targets_for_declared_record(
                    current_record,
                    &mut stale_unit_ids,
                    &mut stale_projection_ids,
                );
            }
            None => {
                affected_declared_record_ids.insert(record_id.clone());
                collect_stale_targets_for_declared_record(
                    previous_record,
                    &mut stale_unit_ids,
                    &mut stale_projection_ids,
                );
            }
        }
    }

    for (record_id, current_record) in &current_by_id {
        if previous_by_id.contains_key(record_id) {
            continue;
        }
        affected_declared_record_ids.insert(record_id.clone());
        collect_stale_targets_for_declared_record(
            current_record,
            &mut stale_unit_ids,
            &mut stale_projection_ids,
        );
    }

    PageDeclaredSnapshotDiff {
        affected_declared_record_ids: affected_declared_record_ids.into_iter().collect(),
        stale_unit_ids: stale_unit_ids.into_iter().collect(),
        stale_projection_ids: stale_projection_ids.into_iter().collect(),
    }
}

fn declared_records_match_for_snapshot(
    current_record: &DeclaredKnowledgeRecord,
    previous_record: &DeclaredKnowledgeRecord,
) -> bool {
    let mut current = current_record.clone();
    let mut previous = previous_record.clone();
    current.updated_at.clear();
    previous.updated_at.clear();
    current == previous
}

fn collect_stale_targets_for_declared_record(
    record: &DeclaredKnowledgeRecord,
    stale_unit_ids: &mut BTreeSet<String>,
    stale_projection_ids: &mut BTreeSet<String>,
) {
    for unit_id in &record.unit_refs {
        stale_unit_ids.insert(unit_id.clone());
    }
    for projection_ref in &record.projection_refs {
        if let Some(page_id) = projection_ref
            .strip_prefix("page:")
            .and_then(|value| value.split(":section:").next())
        {
            stale_projection_ids.insert(page_id.to_string());
        }
    }
    if !record.page_id.is_empty() {
        stale_projection_ids.insert(record.page_id.clone());
    }
}

fn build_sync_health_signals(
    page: &WikiPageState,
    unit_id: Option<&str>,
    result_kind: SyncResultKind,
    has_declared_lifecycle_change: bool,
    reasons: &[String],
) -> Vec<KnowledgeHealthSignal> {
    let mut signals = Vec::new();
    if has_declared_lifecycle_change && result_kind != SyncResultKind::IllegalDrift {
        let target_ref = unit_id
            .map(|value| format!("unit:{value}"))
            .unwrap_or_else(|| format!("page:{}", page.page_id));
        signals.push(KnowledgeHealthSignal {
            signal_id: stable_id("health", format!("declared-sync:{}", target_ref)),
            signal_kind: KnowledgeHealthSignalKind::DeclaredDerivedDivergence,
            severity: KnowledgeHealthSeverity::Warning,
            target_ref,
            recommended_action: KnowledgeHealthRecommendedAction::Update,
            reason:
                "declared writeback 已写入 formal artifacts，等待 update 刷新 derived/projection"
                    .to_string(),
        });
    }
    if result_kind == SyncResultKind::IllegalDrift {
        signals.push(KnowledgeHealthSignal {
            signal_id: stable_id("health", format!("illegal-drift:{}", page.page_id)),
            signal_kind: KnowledgeHealthSignalKind::IllegalDrift,
            severity: KnowledgeHealthSeverity::Error,
            target_ref: format!("page:{}", page.page_id),
            recommended_action: KnowledgeHealthRecommendedAction::Rebuild,
            reason: if reasons.is_empty() {
                "managed truth 漂移且无法映射为合法 declared writeback".to_string()
            } else {
                reasons.join("; ")
            },
        });
    }
    signals
}

fn materialize_declared_record(
    page: &WikiPageState,
    section_id: &str,
    unit_id: Option<&str>,
    block: &DeclaredBlock,
    seen_authoring_ids: &mut BTreeSet<String>,
) -> io::Result<DeclaredKnowledgeRecord> {
    let authoring_id = match block.explicit_id.as_deref() {
        Some(explicit_id) => format!("marker:{explicit_id}"),
        None => format!(
            "page:{}:section:{}:kind:{}:scope:{}",
            page.page_id,
            section_id,
            block.record_kind.as_str(),
            block.scope.canonical_key()
        ),
    };
    if !seen_authoring_ids.insert(authoring_id.clone()) {
        return Err(io::Error::other(format!(
            "declared authoring identity 冲突，若需同页同 scope 并存多条记录，必须显式提供不同 id: {}",
            authoring_id
        )));
    }

    let mut record = DeclaredKnowledgeRecord {
        record_id: DeclaredKnowledgeRecord::record_id_from_authoring_id(&authoring_id),
        authoring_id,
        record_kind: block.record_kind,
        scope: block.scope.clone(),
        status: block.status,
        relations: block.relations.clone(),
        source_ref: block.source_ref.clone(),
        updated_at: current_timestamp(),
        unit_refs: unit_id
            .map(|value| vec![value.to_string()])
            .unwrap_or_default(),
        projection_refs: vec![format!("page:{}:section:{}", page.page_id, section_id)],
        page_id: page.page_id.clone(),
        section_id: section_id.to_string(),
        ordinal: 0,
        body: block.body.clone(),
    };
    record.canonicalize();
    record
        .validate_lifecycle()
        .map_err(|error| io::Error::other(format!("declared lifecycle 非法: {error}")))?;
    Ok(record)
}

fn parse_declared_blocks(body: &str) -> io::Result<(Vec<DeclaredBlock>, String)> {
    let lines = body.lines().collect::<Vec<_>>();
    let mut cleaned_lines = Vec::new();
    let mut blocks = Vec::new();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index];
        if line.trim_start().starts_with(DECLARED_START_PREFIX) {
            let attrs = parse_declared_start_marker(line)?;
            index += 1;
            let mut block_lines = Vec::new();
            let mut found_end = false;
            while index < lines.len() {
                if lines[index].trim() == DECLARED_END_MARKER {
                    found_end = true;
                    index += 1;
                    break;
                }
                block_lines.push(lines[index]);
                index += 1;
            }
            if !found_end {
                return Err(io::Error::other("declared block 缺少结束 marker"));
            }
            let scope = parse_declared_scope(attrs.scope.as_str())?;
            let explicit_id = attrs
                .id
                .map(|id| canonical_marker_id(id.as_str()))
                .transpose()?;
            let (status, relations) = resolve_declared_lifecycle(
                attrs.status.as_deref(),
                attrs.deprecated,
                attrs.replaced_by.as_slice(),
                attrs.supersedes.as_slice(),
            )?;
            blocks.push(DeclaredBlock {
                explicit_id,
                record_kind: parse_declared_kind(attrs.kind.as_str())?,
                scope,
                status,
                relations,
                source_ref: attrs.source,
                body: normalize_text(&block_lines.join("\n")),
            });
            continue;
        }

        if line.contains("wiki:declared:end") {
            return Err(io::Error::other("declared end marker 缺少起始 marker"));
        }

        cleaned_lines.push(line);
        index += 1;
    }

    Ok((blocks, normalize_text(&cleaned_lines.join("\n"))))
}

fn parse_declared_kind(value: &str) -> io::Result<DeclaredKnowledgeRecordKind> {
    match value {
        "policy" => Ok(DeclaredKnowledgeRecordKind::Policy),
        "convention" => Ok(DeclaredKnowledgeRecordKind::Convention),
        "pitfall" => Ok(DeclaredKnowledgeRecordKind::Pitfall),
        "decision" => Ok(DeclaredKnowledgeRecordKind::Decision),
        _ => Err(io::Error::other(format!(
            "unsupported declared kind: {value}"
        ))),
    }
}

fn parse_declared_status(value: &str) -> io::Result<DeclaredKnowledgeRecordStatus> {
    match value {
        "active" => Ok(DeclaredKnowledgeRecordStatus::Active),
        "deprecated" => Ok(DeclaredKnowledgeRecordStatus::Deprecated),
        "superseded" => Ok(DeclaredKnowledgeRecordStatus::Superseded),
        "replaced" => Ok(DeclaredKnowledgeRecordStatus::Replaced),
        _ => Err(io::Error::other(format!(
            "unsupported declared status: {value}"
        ))),
    }
}

fn parse_declared_scope(value: &str) -> io::Result<DeclaredKnowledgeScope> {
    let raw = value.trim();
    if raw.is_empty() {
        return Err(io::Error::other("declared marker 的 scope 不能为空"));
    }

    let (head, selectors) = raw
        .split_once('?')
        .map(|(scope, selector_text)| {
            (
                scope,
                selector_text
                    .split(',')
                    .map(|selector| selector.trim().to_string())
                    .filter(|selector| !selector.is_empty())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or((raw, Vec::new()));
    let (kind, scope_ref) = head
        .split_once(':')
        .map(|(kind, scope_ref)| (kind.trim(), scope_ref.trim()))
        .unwrap_or(("repo", head.trim()));

    let scope_kind = match kind {
        "repo" => DeclaredKnowledgeScopeKind::Repo,
        "domain" => DeclaredKnowledgeScopeKind::Domain,
        "unit" => DeclaredKnowledgeScopeKind::Unit,
        "module" => DeclaredKnowledgeScopeKind::Module,
        "page" => DeclaredKnowledgeScopeKind::Page,
        "source" => DeclaredKnowledgeScopeKind::Source,
        _ => {
            return Err(io::Error::other(format!(
                "unsupported declared scope kind: {kind}"
            )))
        }
    };
    let default_ref = if scope_kind == DeclaredKnowledgeScopeKind::Repo && scope_ref.is_empty() {
        "repo"
    } else {
        scope_ref
    };
    let mut scope = DeclaredKnowledgeScope {
        kind: scope_kind,
        r#ref: default_ref.to_string(),
        selectors,
    };
    scope.canonicalize();
    if scope.r#ref.is_empty() {
        return Err(io::Error::other("declared scope 缺少 ref"));
    }
    Ok(scope)
}

struct DeclaredMarkerAttrs {
    id: Option<String>,
    kind: String,
    scope: String,
    status: Option<String>,
    deprecated: bool,
    replaced_by: Vec<String>,
    supersedes: Vec<String>,
    source: String,
}

fn parse_declared_start_marker(line: &str) -> io::Result<DeclaredMarkerAttrs> {
    let trimmed = line.trim();
    if !trimmed.starts_with(DECLARED_START_PREFIX) || !trimmed.ends_with("-->") {
        return Err(io::Error::other("declared start marker 非法"));
    }
    let attrs = trimmed
        .trim_start_matches(DECLARED_START_PREFIX)
        .trim_end_matches("-->")
        .trim();
    let id = extract_attr(attrs, "id");
    let kind =
        extract_attr(attrs, "kind").ok_or_else(|| io::Error::other("declared marker 缺少 kind"))?;
    let scope = extract_attr(attrs, "scope")
        .ok_or_else(|| io::Error::other("declared marker 缺少 scope"))?;
    let status = extract_attr(attrs, "status");
    let deprecated = extract_attr(attrs, "deprecated")
        .map(|value| parse_bool_attr("deprecated", value.as_str()))
        .transpose()?
        .unwrap_or(false);
    let replaced_by = parse_relation_targets(extract_attr(attrs, "replaced_by"))?;
    let supersedes = parse_relation_targets(extract_attr(attrs, "supersedes"))?;
    let source = extract_attr(attrs, "source")
        .ok_or_else(|| io::Error::other("declared marker 缺少 source"))?;

    Ok(DeclaredMarkerAttrs {
        id,
        kind,
        scope,
        status,
        deprecated,
        replaced_by,
        supersedes,
        source,
    })
}

fn parse_bool_attr(key: &str, value: &str) -> io::Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(io::Error::other(format!("{key} 必须是 true/false"))),
    }
}

fn parse_relation_targets(value: Option<String>) -> io::Result<Vec<String>> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let mut targets = value
        .split(',')
        .map(|item| canonical_marker_id(item))
        .collect::<io::Result<Vec<_>>>()?;
    targets.sort();
    targets.dedup();
    Ok(targets)
}

fn canonical_marker_id(value: &str) -> io::Result<String> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(io::Error::other("declared id 不能为空"));
    }
    if normalized
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '.' | '_' | '-'))
    {
        return Ok(normalized);
    }
    Err(io::Error::other("declared id 只允许 [a-z0-9._-] 字符集"))
}

fn resolve_declared_lifecycle(
    status: Option<&str>,
    deprecated: bool,
    replaced_by: &[String],
    supersedes: &[String],
) -> io::Result<(
    DeclaredKnowledgeRecordStatus,
    Vec<DeclaredKnowledgeRelation>,
)> {
    if deprecated && (!replaced_by.is_empty() || !supersedes.is_empty()) {
        return Err(io::Error::other(
            "deprecated 不允许与 replaced_by / supersedes 同时出现",
        ));
    }
    if !replaced_by.is_empty() && !supersedes.is_empty() {
        return Err(io::Error::other("replaced_by 与 supersedes 不能同时出现"));
    }

    let explicit_status = status.map(parse_declared_status).transpose()?;
    let inferred_status = if deprecated {
        DeclaredKnowledgeRecordStatus::Deprecated
    } else if !replaced_by.is_empty() {
        DeclaredKnowledgeRecordStatus::Superseded
    } else if !supersedes.is_empty() {
        DeclaredKnowledgeRecordStatus::Replaced
    } else {
        DeclaredKnowledgeRecordStatus::Active
    };

    if let Some(explicit_status) = explicit_status {
        let legal = match explicit_status {
            DeclaredKnowledgeRecordStatus::Active => {
                !deprecated && replaced_by.is_empty() && supersedes.is_empty()
            }
            DeclaredKnowledgeRecordStatus::Deprecated => deprecated,
            DeclaredKnowledgeRecordStatus::Superseded => !replaced_by.is_empty(),
            DeclaredKnowledgeRecordStatus::Replaced => !supersedes.is_empty(),
        };
        if !legal || explicit_status != inferred_status {
            return Err(io::Error::other(
                "declared status 与 lifecycle header 组合不一致",
            ));
        }
    }

    let mut relations = Vec::new();
    if deprecated {
        relations.push(DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::Deprecated,
            target_record_ref: None,
        });
    }
    relations.extend(replaced_by.iter().cloned().map(|target_record_ref| {
        DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::ReplacedBy,
            target_record_ref: Some(format!("marker:{target_record_ref}")),
        }
    }));
    relations.extend(supersedes.iter().cloned().map(|target_record_ref| {
        DeclaredKnowledgeRelation {
            relation_kind: DeclaredKnowledgeRelationKind::Supersedes,
            target_record_ref: Some(format!("marker:{target_record_ref}")),
        }
    }));

    Ok((inferred_status, relations))
}

fn extract_attr(attrs: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    attrs.split_whitespace().find_map(|token| {
        token
            .strip_prefix(&prefix)
            .map(|value| value.trim_matches('"').to_string())
    })
}

fn normalize_text(input: &str) -> String {
    input.replace("\r\n", "\n").trim().to_string()
}

fn load_generated_hashes(repo_root: &Path, page: &WikiPageState) -> HashMap<String, String> {
    let mut map = page
        .sections
        .iter()
        .filter_map(|section| {
            section
                .managed
                .then(|| {
                    section
                        .generated_content_hash
                        .as_ref()
                        .map(|hash| (section.section_id.clone(), hash.clone()))
                })
                .flatten()
        })
        .collect::<HashMap<_, _>>();

    if map.is_empty() {
        if let Ok(cache) = read_page_generation_cache(repo_root, &page.page_id) {
            for section in &cache.sections {
                map.insert(
                    section.section_id.clone(),
                    fingerprint_bytes(section.content.as_bytes()),
                );
            }
        }
    }

    map
}

fn build_section_states_from_parsed(
    parsed: &ParsedWikiPage,
    gen_hash_map: &HashMap<String, String>,
) -> Vec<WikiSectionState> {
    parsed
        .blocks
        .iter()
        .map(|block| match block {
            PageBlock::Managed(managed) => WikiSectionState {
                section_id: managed.section_id.clone(),
                title: managed.title.clone(),
                managed: true,
                content_hash: content_hash(&managed.body),
                generated_content_hash: gen_hash_map.get(&managed.section_id).cloned(),
                anchor_after_section_id: None,
                anchor_before_section_id: None,
                source_ids: Vec::new(),
                relation_ids: Vec::new(),
            },
            PageBlock::User(user) => WikiSectionState {
                section_id: user.id.clone(),
                title: String::new(),
                managed: false,
                content_hash: content_hash(&user.body),
                generated_content_hash: None,
                anchor_after_section_id: user.anchor_after_section_id.clone(),
                anchor_before_section_id: user.anchor_before_section_id.clone(),
                source_ids: Vec::new(),
                relation_ids: Vec::new(),
            },
        })
        .collect()
}

fn extract_summary_from_parsed(parsed: &ParsedWikiPage) -> String {
    for block in &parsed.blocks {
        if let PageBlock::Managed(managed) = block {
            if let Ok((_declared, cleaned_body)) = parse_declared_blocks(&managed.body) {
                let trimmed = cleaned_body.trim();
                if !trimmed.is_empty() {
                    return trimmed.chars().take(200).collect();
                }
            }

            let trimmed = managed.body.trim();
            if !trimmed.is_empty() {
                return trimmed.chars().take(200).collect();
            }
        }
    }
    String::new()
}
