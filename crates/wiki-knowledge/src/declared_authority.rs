//! declared authority 负责按 kind + canonical scope 对 replacement graph 求唯一 head。

use std::collections::{BTreeMap, BTreeSet};

use wiki_model::domain::knowledge_artifact::{
    validate_declared_record_snapshot, DeclaredAuthoringState, DeclaredAuthorityDecision,
    DeclaredAuthorityReasonKind, DeclaredAuthorityState, DeclaredGovernanceEvent,
    DeclaredGovernanceEventKind, DeclaredKnowledgeRecord, DeclaredKnowledgeRecordStatus,
    DeclaredKnowledgeRelationKind, KnowledgeConflictRecord,
};
use wiki_model::domain::stable_id::stable_id;

/// 对完整 declared snapshot 求稳定的 group-level authority decisions。
pub fn evaluate_declared_authority(
    records: &[DeclaredKnowledgeRecord],
) -> Result<Vec<DeclaredAuthorityDecision>, String> {
    validate_declared_record_snapshot(records)?;
    let mut groups = BTreeMap::<(String, String), Vec<&DeclaredKnowledgeRecord>>::new();
    for record in records {
        groups
            .entry((
                record.record_kind.as_str().to_string(),
                record.scope.canonical_key(),
            ))
            .or_default()
            .push(record);
    }

    let mut decisions = Vec::new();
    for ((kind, scope_key), mut members) in groups {
        members.sort_by(|left, right| left.record_id.cmp(&right.record_id));
        let by_authoring = members
            .iter()
            .map(|record| (record.authoring_id.as_str(), *record))
            .collect::<BTreeMap<_, _>>();
        let mut outgoing = BTreeMap::<String, BTreeSet<String>>::new();
        for record in &members {
            for relation in &record.relations {
                let Some(target) = relation.target_record_ref.as_ref() else {
                    continue;
                };
                match relation.relation_kind {
                    DeclaredKnowledgeRelationKind::ReplacedBy => {
                        outgoing
                            .entry(record.authoring_id.clone())
                            .or_default()
                            .insert(target.clone());
                    }
                    DeclaredKnowledgeRelationKind::Supersedes => {
                        outgoing
                            .entry(target.clone())
                            .or_default()
                            .insert(record.authoring_id.clone());
                    }
                    DeclaredKnowledgeRelationKind::Deprecated => {}
                }
            }
        }

        let all_deprecated = members
            .iter()
            .all(|record| record.status == DeclaredKnowledgeRecordStatus::Deprecated);
        let terminal = members
            .iter()
            .filter(|record| {
                record.status != DeclaredKnowledgeRecordStatus::Deprecated
                    && outgoing
                        .get(&record.authoring_id)
                        .is_none_or(BTreeSet::is_empty)
            })
            .copied()
            .collect::<Vec<_>>();
        let valid_heads = terminal
            .iter()
            .filter(|record| {
                matches!(
                    record.status,
                    DeclaredKnowledgeRecordStatus::Active | DeclaredKnowledgeRecordStatus::Replaced
                )
            })
            .copied()
            .collect::<Vec<_>>();

        let (authority_state, reason_kind, heads) = if all_deprecated {
            (
                DeclaredAuthorityState::None,
                DeclaredAuthorityReasonKind::ExplicitlyDeprecated,
                Vec::new(),
            )
        } else if valid_heads.len() == 1
            && valid_heads[0].authoring_state == DeclaredAuthoringState::Missing
        {
            (
                DeclaredAuthorityState::Conflict,
                DeclaredAuthorityReasonKind::AuthorityAuthoringMissing,
                valid_heads,
            )
        } else if valid_heads.len() == 1 && terminal.len() == 1 {
            (
                DeclaredAuthorityState::Unique,
                DeclaredAuthorityReasonKind::UniqueHead,
                valid_heads,
            )
        } else if valid_heads.len() > 1 {
            (
                DeclaredAuthorityState::Conflict,
                DeclaredAuthorityReasonKind::ParallelHeads,
                valid_heads,
            )
        } else {
            (
                DeclaredAuthorityState::Conflict,
                DeclaredAuthorityReasonKind::LifecycleHeadAmbiguity,
                if terminal.is_empty() {
                    members.clone()
                } else {
                    terminal
                },
            )
        };

        let mut decision = DeclaredAuthorityDecision {
            authority_group_id: stable_id("declared-authority", format!("{kind}:{scope_key}")),
            record_kind: members[0].record_kind,
            scope: members[0].scope.clone(),
            authority_state,
            head_record_refs: heads
                .iter()
                .map(|record| record.record_id.clone())
                .collect(),
            member_record_refs: members
                .iter()
                .map(|record| record.record_id.clone())
                .collect(),
            reason_kind,
            evidence_refs: members
                .iter()
                .map(|record| record.source_ref.clone())
                .collect(),
            evaluated_declared_snapshot_id: String::new(),
        };
        decision.canonicalize();
        decision.validate()?;
        decisions.push(decision);

        debug_assert!(heads
            .iter()
            .all(|head| by_authoring.contains_key(head.authoring_id.as_str())));
    }
    decisions.sort_by(|left, right| left.authority_group_id.cmp(&right.authority_group_id));
    Ok(decisions)
}

/// 将两个 declared snapshots 的 authority/binding 差异追加为稳定治理事件。
#[allow(clippy::too_many_arguments)]
pub fn reconcile_declared_governance_events(
    previous_events: &[DeclaredGovernanceEvent],
    previous_decisions: &[DeclaredAuthorityDecision],
    current_decisions: &[DeclaredAuthorityDecision],
    previous_records: &[DeclaredKnowledgeRecord],
    current_records: &[DeclaredKnowledgeRecord],
    previous_conflicts: &[KnowledgeConflictRecord],
    current_conflicts: &[KnowledgeConflictRecord],
    previous_declared_snapshot_id: &str,
    current_declared_snapshot_id: &str,
    occurred_at: &str,
) -> Result<Vec<DeclaredGovernanceEvent>, String> {
    let mut events = previous_events.to_vec();
    validate_event_history(&events)?;
    if previous_declared_snapshot_id == current_declared_snapshot_id {
        return Ok(events);
    }

    let previous_by_group = previous_decisions
        .iter()
        .map(|decision| (decision.authority_group_id.as_str(), decision))
        .collect::<BTreeMap<_, _>>();
    let current_by_group = current_decisions
        .iter()
        .map(|decision| (decision.authority_group_id.as_str(), decision))
        .collect::<BTreeMap<_, _>>();
    let group_ids = previous_by_group
        .keys()
        .chain(current_by_group.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    let mut next_sequence = events.last().map_or(1, |event| event.sequence + 1);

    for group_id in group_ids {
        let previous = previous_by_group.get(group_id).copied();
        let current = current_by_group.get(group_id).copied();
        let record_refs = decision_record_refs(previous, current);
        let evidence_refs = decision_evidence_refs(previous, current);
        let before_heads = previous
            .map(|decision| decision.head_record_refs.clone())
            .unwrap_or_default();
        let after_heads = current
            .map(|decision| decision.head_record_refs.clone())
            .unwrap_or_default();

        let previous_conflict_id = conflict_for_group(previous_conflicts, &record_refs);
        let current_conflict_id = conflict_for_group(current_conflicts, &record_refs);
        if previous_conflict_id != current_conflict_id {
            if let Some(conflict_id) = previous_conflict_id {
                events.push(build_event(
                    next_sequence,
                    DeclaredGovernanceEventKind::ConflictResolved,
                    group_id,
                    Some(conflict_id),
                    before_heads.clone(),
                    after_heads.clone(),
                    record_refs.clone(),
                    evidence_refs.clone(),
                    previous_declared_snapshot_id,
                    current_declared_snapshot_id,
                    occurred_at,
                )?);
                next_sequence += 1;
            }
            if let Some(conflict_id) = current_conflict_id {
                let event_kind = DeclaredGovernanceEventKind::ConflictOpened;
                events.push(build_event(
                    next_sequence,
                    event_kind,
                    group_id,
                    Some(conflict_id),
                    before_heads.clone(),
                    after_heads.clone(),
                    record_refs.clone(),
                    evidence_refs.clone(),
                    previous_declared_snapshot_id,
                    current_declared_snapshot_id,
                    occurred_at,
                )?);
                next_sequence += 1;
            }
        }

        if let (Some(previous), Some(current)) = (previous, current) {
            if previous.authority_state != current.authority_state
                || previous.head_record_refs != current.head_record_refs
            {
                events.push(build_event(
                    next_sequence,
                    DeclaredGovernanceEventKind::AuthorityChanged,
                    group_id,
                    None,
                    before_heads,
                    after_heads,
                    record_refs,
                    evidence_refs,
                    previous_declared_snapshot_id,
                    current_declared_snapshot_id,
                    occurred_at,
                )?);
                next_sequence += 1;
            }
        }
    }

    let previous_by_record = previous_records
        .iter()
        .map(|record| (record.record_id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let mut current_records = current_records.iter().collect::<Vec<_>>();
    current_records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    for current in current_records {
        let Some(previous) = previous_by_record.get(current.record_id.as_str()).copied() else {
            continue;
        };
        let event_kind = match (previous.authoring_state, current.authoring_state) {
            (state, DeclaredAuthoringState::Missing)
                if state != DeclaredAuthoringState::Missing =>
            {
                Some(DeclaredGovernanceEventKind::AuthoringMissing)
            }
            (DeclaredAuthoringState::Missing, DeclaredAuthoringState::Bound) => {
                Some(DeclaredGovernanceEventKind::AuthoringRestored)
            }
            (state, DeclaredAuthoringState::Detached)
                if state != DeclaredAuthoringState::Detached =>
            {
                Some(DeclaredGovernanceEventKind::AuthoringDetached)
            }
            _ => None,
        };
        let Some(event_kind) = event_kind else {
            continue;
        };
        let decision = current_decisions
            .iter()
            .chain(previous_decisions.iter())
            .find(|decision| decision.member_record_refs.contains(&current.record_id));
        let group_id = decision
            .map(|decision| decision.authority_group_id.as_str())
            .ok_or_else(|| {
                format!(
                    "declared record '{}' 缺少 authority group",
                    current.record_id
                )
            })?;
        events.push(build_event(
            next_sequence,
            event_kind,
            group_id,
            None,
            Vec::new(),
            Vec::new(),
            vec![current.record_id.clone()],
            vec![previous.source_ref.clone(), current.source_ref.clone()],
            previous_declared_snapshot_id,
            current_declared_snapshot_id,
            occurred_at,
        )?);
        next_sequence += 1;
    }

    validate_event_history(&events)?;
    Ok(events)
}

fn decision_record_refs(
    previous: Option<&DeclaredAuthorityDecision>,
    current: Option<&DeclaredAuthorityDecision>,
) -> Vec<String> {
    previous
        .into_iter()
        .chain(current)
        .flat_map(|decision| decision.member_record_refs.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn decision_evidence_refs(
    previous: Option<&DeclaredAuthorityDecision>,
    current: Option<&DeclaredAuthorityDecision>,
) -> Vec<String> {
    previous
        .into_iter()
        .chain(current)
        .flat_map(|decision| decision.evidence_refs.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn conflict_for_group(
    conflicts: &[KnowledgeConflictRecord],
    record_refs: &[String],
) -> Option<String> {
    conflicts
        .iter()
        .filter(|conflict| {
            conflict
                .record_ids
                .iter()
                .any(|record_id| record_refs.contains(record_id))
        })
        .map(|conflict| conflict.conflict_id.clone())
        .min()
}

#[allow(clippy::too_many_arguments)]
fn build_event(
    sequence: u64,
    event_kind: DeclaredGovernanceEventKind,
    authority_group_id: &str,
    conflict_id: Option<String>,
    before_head_refs: Vec<String>,
    after_head_refs: Vec<String>,
    record_refs: Vec<String>,
    evidence_refs: Vec<String>,
    previous_declared_snapshot_id: &str,
    current_declared_snapshot_id: &str,
    occurred_at: &str,
) -> Result<DeclaredGovernanceEvent, String> {
    let mut event = DeclaredGovernanceEvent {
        event_id: stable_id(
            "declared-governance-event",
            format!(
                "{sequence}:{}:{authority_group_id}:{previous_declared_snapshot_id}:{current_declared_snapshot_id}",
                event_kind.as_str()
            ),
        ),
        sequence,
        event_kind,
        authority_group_id: authority_group_id.to_string(),
        conflict_id,
        before_head_refs,
        after_head_refs,
        record_refs,
        evidence_refs,
        previous_declared_snapshot_id: previous_declared_snapshot_id.to_string(),
        current_declared_snapshot_id: current_declared_snapshot_id.to_string(),
        occurred_at: occurred_at.to_string(),
    };
    event.canonicalize();
    event.validate()?;
    Ok(event)
}

fn validate_event_history(events: &[DeclaredGovernanceEvent]) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for (index, event) in events.iter().enumerate() {
        event.validate()?;
        let expected = index as u64 + 1;
        if event.sequence != expected {
            return Err(format!(
                "declared governance event sequence 非连续: expected={expected}, actual={}",
                event.sequence
            ));
        }
        if !ids.insert(event.event_id.as_str()) {
            return Err(format!(
                "重复 declared governance event_id '{}'",
                event.event_id
            ));
        }
    }
    Ok(())
}
