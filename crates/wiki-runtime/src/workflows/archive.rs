//! Archive workflow 组合只读治理判断与可恢复文件操作。

use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Serialize;
use time::{format_description::well_known::Rfc3339, Date, OffsetDateTime};
use wiki_model::domain::governance::{
    ArchiveCheckpoint, ArchiveErrorKind, ArchiveMode, ArchiveOperationManifest,
    ArchiveOperationStatus, ArchiveOutcome, ArchiveReport, ArchiveStep, ArchiveStepStatus,
    GovernanceArtifactRef, GovernanceGateStatus,
};

use crate::storage::archive_fs::{
    archive_failure, ArchiveFs, ParentPatch, ARCHIVE_ALGORITHM_VERSION, ARCHIVE_SCHEMA_VERSION,
};
use crate::workflows::governance::GovernanceService;

#[derive(Debug, Clone)]
pub struct ArchivePlanOptions {
    pub now: OffsetDateTime,
    pub operation_id: String,
}

#[derive(Debug, Clone)]
pub struct ArchiveService {
    repo_root: PathBuf,
}

#[derive(Serialize)]
struct DigestInput<'a> {
    schema_version: &'a str,
    policy_version: &'a str,
    algorithm_version: &'a str,
    change_id: &'a str,
    source_path: &'a str,
    target_path: &'a str,
    target_absent: bool,
    artifact_hash_summary: &'a [wiki_model::domain::governance::ArchiveArtifactHash],
    parent_diff: &'a Option<wiki_model::domain::governance::ArchiveParentDiff>,
    validation: &'a wiki_model::domain::governance::GovernanceValidationResult,
}

impl ArchiveService {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub fn plan(&self, change_id: &str) -> io::Result<ArchiveReport> {
        let now = OffsetDateTime::now_utc();
        let operation_id = format!("{}-{}", now.unix_timestamp_nanos(), change_id);
        self.plan_with_options(change_id, ArchivePlanOptions { now, operation_id })
    }

    pub fn plan_with_options(
        &self,
        change_id: &str,
        options: ArchivePlanOptions,
    ) -> io::Result<ArchiveReport> {
        self.plan_internal(change_id, options, ArchiveMode::DryRun)
    }

    fn plan_internal(
        &self,
        change_id: &str,
        options: ArchivePlanOptions,
        mode: ArchiveMode,
    ) -> io::Result<ArchiveReport> {
        let governance = GovernanceService::new(&self.repo_root);
        let report = governance.validate_report(change_id)?;
        let change = governance.inspect_change(change_id)?;
        if !report.validation.valid || change.gate.archive_readiness != GovernanceGateStatus::Passed
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "archive change is not ready",
            ));
        }
        let source_path = format!(".spec/changes/{change_id}");
        let date = options.now.date();
        let target_path = target_path(date, change_id)?;
        let fs_adapter = ArchiveFs::new(&self.repo_root);
        if self.repo_root.join(&target_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "archive target already exists",
            ));
        }
        let artifacts = fs_adapter.snapshot_tree(&source_path)?;
        let archived_at = options.now.format(&Rfc3339).map_err(io::Error::other)?;
        let parent_patch = change
            .parent
            .as_deref()
            .map(|parent| {
                fs_adapter.build_parent_patch(parent, change_id, &archived_at, &target_path)
            })
            .transpose()?;
        let validation = report.validation;
        let parent_diff = parent_patch.as_ref().map(|patch| patch.diff.clone());
        let digest = fs_adapter.compute_digest(&DigestInput {
            schema_version: ARCHIVE_SCHEMA_VERSION,
            policy_version: "unispec-0.1.0",
            algorithm_version: ARCHIVE_ALGORITHM_VERSION,
            change_id,
            source_path: &source_path,
            target_path: &target_path,
            target_absent: true,
            artifact_hash_summary: &artifacts,
            parent_diff: &parent_diff,
            validation: &validation,
        })?;
        let operation_root = format!(".spec/.runtime/archive-operations/{}", options.operation_id);
        let manifest = ArchiveOperationManifest {
            schema_version: ARCHIVE_SCHEMA_VERSION.to_string(),
            policy_version: "unispec-0.1.0".to_string(),
            algorithm_version: ARCHIVE_ALGORITHM_VERSION.to_string(),
            operation_id: options.operation_id,
            change_id: change_id.to_string(),
            mode,
            outcome: ArchiveOutcome::Ready,
            status: ArchiveOperationStatus::Planned,
            step: ArchiveStep::Prepared,
            step_status: ArchiveStepStatus::Pending,
            source_path,
            target_path,
            operation_root: Some(operation_root),
            created_at: archived_at,
            persisted: false,
            resumable: false,
            validation: validation.clone(),
            artifact_hash_summary: artifacts,
            parent_diff,
            precondition_digest: digest,
            completed_steps: Vec::new(),
            failure_step: None,
            recovery_hint: None,
            wiki_sync_issues: Vec::new(),
            evidence_refs: evidence_refs(&validation),
        };
        Ok(ArchiveReport {
            governance: report.governance,
            validation,
            manifest,
        })
    }

    pub fn apply(&self, change_id: &str) -> io::Result<ArchiveReport> {
        let fs_adapter = ArchiveFs::new(&self.repo_root);
        let incomplete = fs_adapter.discover_incomplete(change_id)?;
        if let Some(operation) = incomplete.first() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                format!("archive recovery required: {operation}"),
            ));
        }
        let completed = fs_adapter.discover_completed(change_id)?;
        if let Some(operation) = completed.last() {
            let mut report = self.resume(change_id, operation)?;
            report.manifest.outcome = ArchiveOutcome::AlreadyCompleted;
            return Ok(report);
        }
        let now = OffsetDateTime::now_utc();
        let operation_id = format!("{}-{}", now.unix_timestamp_nanos(), change_id);
        let mut report = self.plan_internal(
            change_id,
            ArchivePlanOptions { now, operation_id },
            ArchiveMode::Apply,
        )?;
        let parent_id = report
            .manifest
            .parent_diff
            .as_ref()
            .map(|diff| diff.parent_id.as_str());
        let _lock = fs_adapter.acquire_lock(change_id, parent_id)?;
        let fresh = self.plan_internal(
            change_id,
            ArchivePlanOptions {
                now,
                operation_id: report.manifest.operation_id.clone(),
            },
            ArchiveMode::Apply,
        )?;
        if fresh.manifest.precondition_digest != report.manifest.precondition_digest {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "archive precondition changed",
            ));
        }
        let patch = report
            .manifest
            .parent_diff
            .as_ref()
            .map(|diff| {
                fs_adapter.build_parent_patch(
                    &diff.parent_id,
                    &report.manifest.change_id,
                    &diff.archived_at,
                    &diff.archived_to,
                )
            })
            .transpose()?;
        report.manifest.persisted = true;
        report.manifest.resumable = true;
        report.manifest.status = ArchiveOperationStatus::Applying;
        fs_adapter.persist_plan(&report.manifest, patch.as_ref())?;
        let mutation_result = (|| -> io::Result<()> {
            checkpoint(&fs_adapter, &report.manifest, 1, ArchiveStep::Prepared)?;
            fs_adapter.rename_source(&report.manifest.source_path, &report.manifest.target_path)?;
            report
                .manifest
                .completed_steps
                .push(ArchiveStep::SourceMoved);
            checkpoint(&fs_adapter, &report.manifest, 2, ArchiveStep::SourceMoved)?;
            if let Some(patch) = patch.as_ref() {
                fs_adapter.replace_parent_files(patch)?;
                report.manifest.completed_steps.extend([
                    ArchiveStep::ParentMetaUpdated,
                    ArchiveStep::ParentSplitUpdated,
                ]);
                checkpoint(
                    &fs_adapter,
                    &report.manifest,
                    3,
                    ArchiveStep::ParentMetaUpdated,
                )?;
                checkpoint(
                    &fs_adapter,
                    &report.manifest,
                    4,
                    ArchiveStep::ParentSplitUpdated,
                )?;
            }
            let final_change = GovernanceService::new(&self.repo_root).inspect_change(change_id)?;
            if !matches!(
                final_change.location,
                wiki_model::domain::governance::GovernanceLocation::Archived
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "archive final verification failed",
                ));
            }
            Ok(())
        })();
        if let Err(error) = mutation_result {
            report.manifest.outcome = ArchiveOutcome::RecoveryRequired;
            report.manifest.status = ArchiveOperationStatus::RecoveryRequired;
            report.manifest.step_status = ArchiveStepStatus::Failed;
            report.manifest.failure_step = Some(
                report
                    .manifest
                    .completed_steps
                    .last()
                    .copied()
                    .unwrap_or(ArchiveStep::Prepared),
            );
            report.manifest.recovery_hint = Some(format!(
                "resume with archive --resume {}",
                report.manifest.operation_id
            ));
            let _ = fs_adapter.append_checkpoint(
                &report.manifest.operation_id,
                &ArchiveCheckpoint {
                    sequence: report.manifest.completed_steps.len() as u64 + 10,
                    attempt: 1,
                    step: report
                        .manifest
                        .failure_step
                        .unwrap_or(ArchiveStep::Prepared),
                    status: ArchiveStepStatus::Failed,
                    created_at: report.manifest.created_at.clone(),
                    message: Some(error.to_string()),
                },
            );
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                format!(
                    "archive recovery required for {}: {error}",
                    report.manifest.operation_id
                ),
            ));
        }
        report.manifest.outcome = ArchiveOutcome::Completed;
        report.manifest.status = ArchiveOperationStatus::Completed;
        report.manifest.step = ArchiveStep::Completed;
        report.manifest.step_status = ArchiveStepStatus::Completed;
        report.manifest.completed_steps.push(ArchiveStep::Completed);
        checkpoint(&fs_adapter, &report.manifest, 5, ArchiveStep::Completed)?;
        fs_adapter.write_result(&report.manifest)?;
        Ok(report)
    }

    pub fn resume(&self, change_id: &str, operation_id: &str) -> io::Result<ArchiveReport> {
        let fs_adapter = ArchiveFs::new(&self.repo_root);
        let plan = fs_adapter.read_plan(operation_id)?;
        let _ = fs_adapter.read_checkpoints(operation_id)?;
        if plan.change_id != change_id {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "archive operation change id mismatch",
            ));
        }
        if fs_adapter
            .operation_root(operation_id)
            .join("result.json")
            .exists()
        {
            let result = fs_adapter.read_result(operation_id, &plan)?;
            let parent_id = plan
                .parent_diff
                .as_ref()
                .map(|diff| diff.parent_id.as_str());
            let _lock = fs_adapter.acquire_lock(change_id, parent_id)?;
            self.verify_completed_state(&plan)?;
            let mut report = self.report_from_plan(result)?;
            report.manifest.outcome = ArchiveOutcome::AlreadyCompleted;
            return Ok(report);
        }
        let parent_id = plan
            .parent_diff
            .as_ref()
            .map(|diff| diff.parent_id.as_str());
        let _lock = fs_adapter.acquire_lock(change_id, parent_id)?;
        let source_exists = self.repo_root.join(&plan.source_path).exists();
        let target_exists = self.repo_root.join(&plan.target_path).exists();
        if source_exists && !target_exists {
            fs_adapter.rename_source(&plan.source_path, &plan.target_path)?;
        } else if source_exists || !target_exists {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "archive filesystem state conflicts with plan",
            ));
        }
        if let Some(patch) = self.parent_patch_from_manifest(&plan)? {
            fs_adapter.replace_parent_files(&patch)?;
        }
        let mut report = self.report_from_plan(plan)?;
        report.manifest.outcome = ArchiveOutcome::Completed;
        report.manifest.status = ArchiveOperationStatus::Completed;
        report.manifest.step = ArchiveStep::Completed;
        report.manifest.step_status = ArchiveStepStatus::Completed;
        fs_adapter.write_result(&report.manifest)?;
        Ok(report)
    }

    fn report_from_plan(&self, manifest: ArchiveOperationManifest) -> io::Result<ArchiveReport> {
        let governance = GovernanceService::new(&self.repo_root).status()?;
        Ok(ArchiveReport {
            validation: manifest.validation.clone(),
            governance,
            manifest,
        })
    }

    fn verify_completed_state(&self, manifest: &ArchiveOperationManifest) -> io::Result<()> {
        if self.repo_root.join(&manifest.source_path).exists()
            || !self.repo_root.join(&manifest.target_path).is_dir()
        {
            return Err(archive_failure(
                ArchiveErrorKind::Conflict,
                "completed archive filesystem state conflicts with manifest",
            ));
        }
        if let Some(diff) = manifest.parent_diff.as_ref() {
            let meta = fs::read(self.repo_root.join(&diff.meta_path))?;
            let split = fs::read(self.repo_root.join(&diff.split_path))?;
            if crate::storage::archive_fs::hash_bytes(&meta) != diff.meta_after_hash
                || crate::storage::archive_fs::hash_bytes(&split) != diff.split_after_hash
            {
                return Err(archive_failure(
                    ArchiveErrorKind::Conflict,
                    "completed archive parent state has changed",
                ));
            }
        }
        let change = GovernanceService::new(&self.repo_root).inspect_change(&manifest.change_id)?;
        if !matches!(
            change.location,
            wiki_model::domain::governance::GovernanceLocation::Archived
        ) {
            return Err(archive_failure(
                ArchiveErrorKind::Conflict,
                "completed archive is not present in governance truth",
            ));
        }
        Ok(())
    }

    fn parent_patch_from_manifest(
        &self,
        manifest: &ArchiveOperationManifest,
    ) -> io::Result<Option<ParentPatch>> {
        let Some(diff) = manifest.parent_diff.as_ref() else {
            return Ok(None);
        };
        let staging = ArchiveFs::new(&self.repo_root)
            .operation_root(&manifest.operation_id)
            .join("staging");
        let meta_after =
            fs::read_to_string(staging.join("parent-meta.after")).map_err(|error| {
                archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    format!("invalid archive operation manifest staging: {error}"),
                )
            })?;
        let split_after =
            fs::read_to_string(staging.join("parent-split.after")).map_err(|error| {
                archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    format!("invalid archive operation manifest staging: {error}"),
                )
            })?;
        Ok(Some(ParentPatch {
            diff: diff.clone(),
            meta_after,
            split_after,
        }))
    }

    pub fn resume_target_for_test(
        &self,
        manifest: &ArchiveOperationManifest,
        _now: OffsetDateTime,
    ) -> io::Result<String> {
        Ok(manifest.target_path.clone())
    }
}

fn target_path(date: Date, change_id: &str) -> io::Result<String> {
    Ok(format!(
        ".spec/archive/{:04}-{:02}-{:02}-{change_id}",
        date.year(),
        u8::from(date.month()),
        date.day()
    ))
}

fn evidence_refs(
    validation: &wiki_model::domain::governance::GovernanceValidationResult,
) -> Vec<GovernanceArtifactRef> {
    validation
        .issues
        .iter()
        .filter_map(|issue| issue.artifact_ref.clone())
        .collect()
}

fn checkpoint(
    fs_adapter: &ArchiveFs,
    manifest: &ArchiveOperationManifest,
    sequence: u64,
    step: ArchiveStep,
) -> io::Result<()> {
    fs_adapter.append_checkpoint(
        &manifest.operation_id,
        &ArchiveCheckpoint {
            sequence,
            attempt: 1,
            step,
            status: ArchiveStepStatus::Completed,
            created_at: manifest.created_at.clone(),
            message: None,
        },
    )
}
