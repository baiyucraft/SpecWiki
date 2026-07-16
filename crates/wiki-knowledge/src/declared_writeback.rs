//! declared writeback 的 knowledge-owned 语义校验入口。
//! runtime 只能提交结构化 candidate，本模块负责归一化 declared record 并产出 patch。

use wiki_model::domain::knowledge_artifact::{
    DeclaredAuthoringState, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordKind,
    DeclaredKnowledgeRecordStatus, DeclaredKnowledgeRelation, DeclaredKnowledgeScope,
};

#[derive(Debug, Clone, Default)]
pub struct DeclaredSnapshot {
    pub records: Vec<DeclaredKnowledgeRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredAuthoringCandidate {
    pub authoring_id: String,
    pub record_kind: DeclaredKnowledgeRecordKind,
    pub scope: DeclaredKnowledgeScope,
    pub status: DeclaredKnowledgeRecordStatus,
    pub relations: Vec<DeclaredKnowledgeRelation>,
    pub source_ref: String,
    pub page_id: String,
    pub section_id: String,
    pub body: String,
    pub baseline_hash: String,
    pub current_hash: String,
    pub marker_version: u32,
    pub metadata_binding_ref: Option<String>,
    pub unit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredRecordPatch {
    pub record: DeclaredKnowledgeRecord,
    pub affected_projection_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DeclaredWritebackDecision {
    Accepted(DeclaredRecordPatch),
    Rejected {
        reason: String,
        evidence_refs: Vec<String>,
    },
    Conflict {
        reason: String,
        conflicting_record_refs: Vec<String>,
        evidence_refs: Vec<String>,
    },
}

pub fn validate_declared_writeback(
    candidate: &DeclaredAuthoringCandidate,
    snapshot: &DeclaredSnapshot,
) -> DeclaredWritebackDecision {
    if candidate.authoring_id.trim().is_empty() {
        return DeclaredWritebackDecision::Rejected {
            reason: "declared authoring_id 不能为空".to_string(),
            evidence_refs: candidate_evidence_refs(candidate),
        };
    }
    if candidate.body.trim().is_empty() {
        return DeclaredWritebackDecision::Rejected {
            reason: "declared body 不能为空".to_string(),
            evidence_refs: candidate_evidence_refs(candidate),
        };
    }

    let mut record = DeclaredKnowledgeRecord {
        record_id: DeclaredKnowledgeRecord::record_id_from_authoring_id(&candidate.authoring_id),
        authoring_id: candidate.authoring_id.trim().to_string(),
        record_kind: candidate.record_kind,
        scope: candidate.scope.clone(),
        status: candidate.status,
        authoring_state: DeclaredAuthoringState::Bound,
        relations: candidate.relations.clone(),
        source_ref: candidate.source_ref.trim().to_string(),
        updated_at: String::new(),
        unit_refs: candidate.unit_refs.clone(),
        projection_refs: vec![projection_ref(candidate)],
        page_id: candidate.page_id.clone(),
        section_id: candidate.section_id.clone(),
        ordinal: 0,
        body: candidate.body.trim().to_string(),
    };
    record.canonicalize();

    if let Err(error) = record.validate_lifecycle() {
        return DeclaredWritebackDecision::Rejected {
            reason: error,
            evidence_refs: candidate_evidence_refs(candidate),
        };
    }

    if let Some(existing) = snapshot
        .records
        .iter()
        .find(|existing| existing.record_id == record.record_id)
    {
        let mut normalized_existing = existing.clone();
        normalized_existing.canonicalize();
        if normalized_existing.record_kind != record.record_kind
            || normalized_existing.scope.canonical_key() != record.scope.canonical_key()
        {
            return DeclaredWritebackDecision::Conflict {
                reason: "declared writeback 与现有 record kind/scope 冲突".to_string(),
                conflicting_record_refs: vec![normalized_existing.record_id],
                evidence_refs: candidate_evidence_refs(candidate),
            };
        }
    }

    DeclaredWritebackDecision::Accepted(DeclaredRecordPatch {
        affected_projection_refs: record.projection_refs.clone(),
        record,
    })
}

fn projection_ref(candidate: &DeclaredAuthoringCandidate) -> String {
    let page_ref = candidate
        .page_id
        .strip_prefix("page:")
        .unwrap_or(candidate.page_id.as_str());
    format!("page:{}:section:{}", page_ref, candidate.section_id)
}

fn candidate_evidence_refs(candidate: &DeclaredAuthoringCandidate) -> Vec<String> {
    let mut refs = vec![projection_ref(candidate)];
    if let Some(binding_ref) = candidate.metadata_binding_ref.as_ref() {
        if !binding_ref.trim().is_empty() {
            refs.push(binding_ref.trim().to_string());
        }
    }
    refs
}
