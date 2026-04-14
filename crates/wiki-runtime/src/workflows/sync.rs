//! `sync` 负责把用户对 `.wiki/*.md` 的外部修改同步回 WikiState、metadata 与 formal artifacts。
//! 当前只允许最小 declared block writeback；其余 managed truth 漂移统一进入非法分类。

use serde::Serialize;
use std::collections::HashMap;
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
    DeclaredKnowledgeRecord, DeclaredKnowledgeRecordKind, DeclaredKnowledgeRecordStatus,
    KnowledgeHealthRecommendedAction, KnowledgeHealthSeverity, KnowledgeHealthSignal,
    KnowledgeHealthSignalKind,
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
    pub page_id: String,
    pub path: String,
    pub result_kind: SyncResultKind,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_record_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
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
    health_signals: Vec<KnowledgeHealthSignal>,
}

#[derive(Debug, Clone)]
struct DeclaredBlock {
    record_kind: DeclaredKnowledgeRecordKind,
    scope_ref: String,
    status: DeclaredKnowledgeRecordStatus,
    source_ref: String,
    body: String,
    ordinal: usize,
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
        let analysis = analyze_sync_page(page, &parsed, &generated_hash_map, unit_id.as_deref());
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
            declared_record_ids: analysis
                .declared_records
                .iter()
                .map(|record| record.record_id.clone())
                .collect(),
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
) -> SyncPageAnalysis {
    let mut reasons = parsed.warnings.clone();
    let mut declared_records = Vec::new();
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
            declared_records.push(materialize_declared_record(
                page,
                managed.section_id.as_str(),
                unit_id,
                &declared,
            ));
        }
    }

    let result_kind = if illegal_drift {
        SyncResultKind::IllegalDrift
    } else if !declared_records.is_empty() {
        SyncResultKind::DeclaredWriteback
    } else {
        SyncResultKind::MetadataOnly
    };
    let health_signals = build_sync_health_signals(page, unit_id, result_kind, &reasons);

    SyncPageAnalysis {
        result_kind,
        reasons,
        declared_records,
        health_signals,
    }
}

fn build_sync_health_signals(
    page: &WikiPageState,
    unit_id: Option<&str>,
    result_kind: SyncResultKind,
    reasons: &[String],
) -> Vec<KnowledgeHealthSignal> {
    match result_kind {
        SyncResultKind::MetadataOnly => Vec::new(),
        SyncResultKind::DeclaredWriteback => {
            let target_ref = unit_id
                .map(|value| format!("unit:{value}"))
                .unwrap_or_else(|| format!("page:{}", page.page_id));
            vec![KnowledgeHealthSignal {
                signal_id: stable_id("health", format!("declared-sync:{}", target_ref)),
                signal_kind: KnowledgeHealthSignalKind::DeclaredDerivedDivergence,
                severity: KnowledgeHealthSeverity::Warning,
                target_ref,
                recommended_action: KnowledgeHealthRecommendedAction::Update,
                reason: "declared writeback 已写入 formal artifacts，等待 update 刷新 derived/projection".to_string(),
            }]
        }
        SyncResultKind::IllegalDrift => vec![KnowledgeHealthSignal {
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
        }],
    }
}

fn materialize_declared_record(
    page: &WikiPageState,
    section_id: &str,
    unit_id: Option<&str>,
    block: &DeclaredBlock,
) -> DeclaredKnowledgeRecord {
    let record_id = stable_id(
        "declared",
        format!(
            "{}:{}:{}:{}:{}",
            block.record_kind.as_str(),
            block.scope_ref,
            page.page_id,
            section_id,
            block.ordinal
        ),
    );
    DeclaredKnowledgeRecord {
        record_id,
        record_kind: block.record_kind,
        scope_ref: block.scope_ref.clone(),
        status: block.status,
        source_ref: block.source_ref.clone(),
        updated_at: current_timestamp(),
        unit_refs: unit_id
            .map(|value| vec![value.to_string()])
            .unwrap_or_default(),
        projection_refs: vec![format!("page:{}:section:{}", page.page_id, section_id)],
        page_id: page.page_id.clone(),
        section_id: section_id.to_string(),
        ordinal: block.ordinal,
        body: block.body.clone(),
    }
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
            blocks.push(DeclaredBlock {
                record_kind: parse_declared_kind(attrs.kind.as_str())?,
                scope_ref: attrs.scope,
                status: parse_declared_status(attrs.status.as_str())?,
                source_ref: attrs.source,
                body: normalize_text(&block_lines.join("\n")),
                ordinal: blocks.len(),
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
        "superseded" => Ok(DeclaredKnowledgeRecordStatus::Superseded),
        "removed" => Ok(DeclaredKnowledgeRecordStatus::Removed),
        _ => Err(io::Error::other(format!(
            "unsupported declared status: {value}"
        ))),
    }
}

struct DeclaredMarkerAttrs {
    kind: String,
    scope: String,
    status: String,
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
    let kind =
        extract_attr(attrs, "kind").ok_or_else(|| io::Error::other("declared marker 缺少 kind"))?;
    let scope = extract_attr(attrs, "scope")
        .ok_or_else(|| io::Error::other("declared marker 缺少 scope"))?;
    let status = extract_attr(attrs, "status")
        .ok_or_else(|| io::Error::other("declared marker 缺少 status"))?;
    let source = extract_attr(attrs, "source")
        .ok_or_else(|| io::Error::other("declared marker 缺少 source"))?;

    Ok(DeclaredMarkerAttrs {
        kind,
        scope,
        status,
        source,
    })
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
