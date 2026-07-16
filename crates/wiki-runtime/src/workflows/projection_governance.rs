//! Projection governance 对 retiring page 执行只读保护预检。

use wiki_model::domain::knowledge_artifact::{
    DeclaredAuthoringState, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordStatus,
};
use wiki_model::domain::projection::{PageLinkRef, ProjectionAction, SectionOwnership};

use crate::generation::managed_sections::{PageBlock, ParsedWikiPage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionProtectionKind {
    ParseInvalid,
    ManualContent,
    DeclaredAuthority,
    ManualInboundLink,
    ManagedDanglingLink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionProtectionBlocker {
    pub kind: ProjectionProtectionKind,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionRemovalPreflight {
    pub action: ProjectionAction,
    pub blockers: Vec<ProjectionProtectionBlocker>,
}

impl ProjectionRemovalPreflight {
    pub fn is_blocked(&self) -> bool {
        self.action == ProjectionAction::Block
    }
}

pub fn reconcile_retiring_page(
    target_page_id: &str,
    parsed: &ParsedWikiPage,
    declared_records: &[DeclaredKnowledgeRecord],
    inbound_links: &[PageLinkRef],
) -> ProjectionRemovalPreflight {
    let mut blockers = Vec::new();
    if !parsed.diagnostics.is_empty() {
        blockers.push(ProjectionProtectionBlocker {
            kind: ProjectionProtectionKind::ParseInvalid,
            evidence_ref: format!("page:{target_page_id}:parse"),
        });
    }
    for block in &parsed.blocks {
        match block {
            PageBlock::User(block) if !block.body.trim().is_empty() => {
                blockers.push(ProjectionProtectionBlocker {
                    kind: ProjectionProtectionKind::ManualContent,
                    evidence_ref: format!("page:{target_page_id}:manual:{}", block.id),
                });
            }
            PageBlock::Managed(block)
                if block.owner_kind == SectionOwnership::ManualUnmanaged
                    && !block.body.trim().is_empty() =>
            {
                blockers.push(ProjectionProtectionBlocker {
                    kind: ProjectionProtectionKind::ManualContent,
                    evidence_ref: format!("page:{target_page_id}:manual:{}", block.section_id),
                });
            }
            _ => {}
        }
    }
    for record in declared_records.iter().filter(|record| {
        record.page_id == target_page_id
            && matches!(
                record.status,
                DeclaredKnowledgeRecordStatus::Active | DeclaredKnowledgeRecordStatus::Replaced
            )
            && record.authoring_state != DeclaredAuthoringState::Detached
    }) {
        blockers.push(ProjectionProtectionBlocker {
            kind: ProjectionProtectionKind::DeclaredAuthority,
            evidence_ref: format!("declared:{}", record.record_id),
        });
    }
    for link_ref in inbound_links
        .iter()
        .filter(|link_ref| link_ref.target_page_id == target_page_id)
    {
        let kind = if link_ref.source_owner == SectionOwnership::ManualUnmanaged {
            ProjectionProtectionKind::ManualInboundLink
        } else {
            ProjectionProtectionKind::ManagedDanglingLink
        };
        blockers.push(ProjectionProtectionBlocker {
            kind,
            evidence_ref: format!(
                "page-link:{}:{}:{}",
                link_ref.source_page_id,
                link_ref.source_section_id.as_deref().unwrap_or("manual"),
                link_ref.target_page_id
            ),
        });
    }
    blockers.sort_by(|left, right| left.evidence_ref.cmp(&right.evidence_ref));
    blockers.dedup();
    ProjectionRemovalPreflight {
        action: if blockers.is_empty() {
            ProjectionAction::Remove
        } else {
            ProjectionAction::Block
        },
        blockers,
    }
}
