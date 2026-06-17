//! `projection` 收口 page / section / digest 的稳定投影合同。
//! 它只承载跨 crate 共享的 section ownership、binding、digest 和 sync 结果，不承载 Markdown 正文。

use serde::{Deserialize, Serialize};

/// `SectionOwnership` 是页面 section 的机器可判定 ownership。
/// 它跨 metadata、runtime parser、knowledge validation 和 sync result 共用。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SectionOwnership {
    /// 人可编辑并可通过 declared authoring 合法回写 declared knowledge。
    DeclaredManaged,
    /// runtime 从 derived knowledge 生成，人工漂移不得回写 derived truth。
    DerivedManaged,
    /// runtime 生成的静态投影片段，人工改动视为 drift。
    ProjectionStatic,
    /// 人维护的自由内容，runtime 只允许 metadata-only 记录。
    ManualUnmanaged,
    /// 外部文件或系统引用，runtime 只维护引用和摘要。
    ExternalRef,
}

impl SectionOwnership {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredManaged => "declared_managed",
            Self::DerivedManaged => "derived_managed",
            Self::ProjectionStatic => "projection_static",
            Self::ManualUnmanaged => "manual_unmanaged",
            Self::ExternalRef => "external_ref",
        }
    }
}

/// `ProjectionDigestStatus` 表示 projection recovery anchor 的状态。
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionDigestStatus {
    #[default]
    Ready,
    Stale,
    Conflict,
    Blocked,
}

impl ProjectionDigestStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Stale => "stale",
            Self::Conflict => "conflict",
            Self::Blocked => "blocked",
        }
    }
}

/// `ProjectionStatusReasonKind` 收敛 projection binding 失败或降级的原因。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionStatusReasonKind {
    MarkerMissing,
    MarkerMalformed,
    MetadataBindingMismatch,
    PageSnapshotMismatch,
    SectionHashMismatch,
    KnowledgeRefMissing,
}

impl ProjectionStatusReasonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MarkerMissing => "marker_missing",
            Self::MarkerMalformed => "marker_malformed",
            Self::MetadataBindingMismatch => "metadata_binding_mismatch",
            Self::PageSnapshotMismatch => "page_snapshot_mismatch",
            Self::SectionHashMismatch => "section_hash_mismatch",
            Self::KnowledgeRefMissing => "knowledge_ref_missing",
        }
    }
}

/// `ProjectionStatusReason` 是 projection 状态降级的 machine-readable 解释。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProjectionStatusReason {
    pub reason_kind: Option<ProjectionStatusReasonKind>,
    #[serde(default)]
    pub reason_message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
}

impl ProjectionStatusReason {
    pub fn canonicalize(&mut self) {
        self.reason_message = self.reason_message.trim().to_string();
        self.upstream_ref = self
            .upstream_ref
            .take()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }
}

/// `SyncResultKind` 是 sync/writeback 的共享结果分类。
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SyncResultKind {
    DeclaredWriteback,
    MetadataOnly,
    IllegalDrift,
    Conflict,
    Stale,
}

impl SyncResultKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredWriteback => "declared_writeback",
            Self::MetadataOnly => "metadata_only",
            Self::IllegalDrift => "illegal_drift",
            Self::Conflict => "conflict",
            Self::Stale => "stale",
        }
    }
}

/// `ProjectionContractError` 是 projection 合同校验的稳定错误类别。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionContractError {
    MissingSectionId,
    MissingOwner { section_id: String },
    MissingBindingRefs { section_id: String },
    MissingInputHash { section_id: String },
    MissingContentHash { section_id: String },
    MissingProjectionId,
    MissingPageId { projection_id: String },
    MissingProjectionBinding { projection_id: String },
    MissingStatusReason { projection_id: String },
    InvalidStatusReason { projection_id: String },
}

/// `SectionBinding` 记录 section 与 knowledge/source/projection 的最小正式绑定。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SectionBinding {
    pub section_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_kind: Option<SectionOwnership>,
    #[serde(default)]
    pub knowledge_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub input_hash: String,
    pub content_hash: String,
    pub projection_status: ProjectionDigestStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection_digest_ref: Option<String>,
}

/// `ProjectionBinding` 记录 projection 与 page/section/knowledge/snapshot 的恢复锚点。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProjectionBinding {
    pub projection_id: String,
    pub page_id: String,
    #[serde(default)]
    pub section_ids: Vec<String>,
    #[serde(default)]
    pub knowledge_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub snapshot_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest_ref: Option<String>,
}

impl ProjectionBinding {
    pub fn canonicalize(&mut self) {
        self.projection_id = self.projection_id.trim().to_string();
        self.page_id = self.page_id.trim().to_string();
        self.section_ids = sorted_unique(&self.section_ids);
        self.knowledge_refs = sorted_unique(&self.knowledge_refs);
        self.source_refs = sorted_unique(&self.source_refs);
        self.snapshot_id = self.snapshot_id.trim().to_string();
        self.digest_ref = self
            .digest_ref
            .take()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }

    pub fn validate(&self) -> Result<(), ProjectionContractError> {
        let mut normalized = self.clone();
        normalized.canonicalize();
        if normalized.projection_id.is_empty() {
            return Err(ProjectionContractError::MissingProjectionId);
        }
        if normalized.page_id.is_empty() {
            return Err(ProjectionContractError::MissingPageId {
                projection_id: normalized.projection_id,
            });
        }
        if normalized.section_ids.is_empty() && normalized.knowledge_refs.is_empty() {
            return Err(ProjectionContractError::MissingProjectionBinding {
                projection_id: normalized.projection_id,
            });
        }
        Ok(())
    }
}

impl SectionBinding {
    pub fn canonicalize(&mut self) {
        self.section_id = self.section_id.trim().to_string();
        self.knowledge_refs = sorted_unique(&self.knowledge_refs);
        self.source_refs = sorted_unique(&self.source_refs);
        self.input_hash = self.input_hash.trim().to_string();
        self.content_hash = self.content_hash.trim().to_string();
        self.projection_digest_ref = self
            .projection_digest_ref
            .take()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }

    pub fn validate(&self) -> Result<(), ProjectionContractError> {
        let mut normalized = self.clone();
        normalized.canonicalize();
        if normalized.section_id.is_empty() {
            return Err(ProjectionContractError::MissingSectionId);
        }
        if normalized.owner_kind.is_none() {
            return Err(ProjectionContractError::MissingOwner {
                section_id: normalized.section_id,
            });
        }
        if normalized.knowledge_refs.is_empty() && normalized.source_refs.is_empty() {
            return Err(ProjectionContractError::MissingBindingRefs {
                section_id: normalized.section_id,
            });
        }
        if normalized.input_hash.is_empty() {
            return Err(ProjectionContractError::MissingInputHash {
                section_id: normalized.section_id,
            });
        }
        if normalized.content_hash.is_empty() {
            return Err(ProjectionContractError::MissingContentHash {
                section_id: normalized.section_id,
            });
        }
        Ok(())
    }
}

/// `ProjectionDigest` 是持久化 projection/runtime recovery anchor。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProjectionDigest {
    pub projection_id: String,
    pub page_id: String,
    #[serde(default)]
    pub section_ids: Vec<String>,
    #[serde(default)]
    pub knowledge_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub input_hash: String,
    pub renderer_version: String,
    pub content_digest: String,
    #[serde(default)]
    pub section_hashes: Vec<(String, String)>,
    pub status: ProjectionDigestStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reasons: Vec<ProjectionStatusReason>,
}

impl ProjectionDigest {
    pub fn canonicalize(&mut self) {
        self.projection_id = self.projection_id.trim().to_string();
        self.page_id = self.page_id.trim().to_string();
        self.section_ids = sorted_unique(&self.section_ids);
        self.knowledge_refs = sorted_unique(&self.knowledge_refs);
        self.source_refs = sorted_unique(&self.source_refs);
        self.input_hash = self.input_hash.trim().to_string();
        self.renderer_version = self.renderer_version.trim().to_string();
        self.content_digest = self.content_digest.trim().to_string();
        self.section_hashes = self
            .section_hashes
            .iter()
            .map(|(section_id, hash)| (section_id.trim().to_string(), hash.trim().to_string()))
            .filter(|(section_id, hash)| !section_id.is_empty() && !hash.is_empty())
            .collect();
        self.section_hashes.sort();
        self.section_hashes.dedup_by(|left, right| left.0 == right.0);
        for reason in &mut self.status_reasons {
            reason.canonicalize();
        }
        self.status_reasons.sort_by(|left, right| {
            (
                left.reason_kind
                    .map(|kind| kind.as_str())
                    .unwrap_or_default(),
                left.reason_message.as_str(),
                left.upstream_ref.as_deref().unwrap_or_default(),
            )
                .cmp(&(
                    right
                        .reason_kind
                        .map(|kind| kind.as_str())
                        .unwrap_or_default(),
                    right.reason_message.as_str(),
                    right.upstream_ref.as_deref().unwrap_or_default(),
                ))
        });
        self.status_reasons.dedup();
    }

    pub fn validate(&self) -> Result<(), ProjectionContractError> {
        let mut normalized = self.clone();
        normalized.canonicalize();
        if normalized.projection_id.is_empty() {
            return Err(ProjectionContractError::MissingProjectionId);
        }
        if normalized.page_id.is_empty() {
            return Err(ProjectionContractError::MissingPageId {
                projection_id: normalized.projection_id,
            });
        }
        match normalized.status {
            ProjectionDigestStatus::Ready => {
                if !normalized.status_reasons.is_empty() {
                    return Err(ProjectionContractError::InvalidStatusReason {
                        projection_id: normalized.projection_id,
                    });
                }
            }
            ProjectionDigestStatus::Stale
            | ProjectionDigestStatus::Conflict
            | ProjectionDigestStatus::Blocked => {
                if normalized.status_reasons.is_empty() {
                    return Err(ProjectionContractError::MissingStatusReason {
                        projection_id: normalized.projection_id,
                    });
                }
            }
        }
        Ok(())
    }
}

fn sorted_unique(values: &[String]) -> Vec<String> {
    let mut sorted = values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    sorted.sort();
    sorted.dedup();
    sorted
}
