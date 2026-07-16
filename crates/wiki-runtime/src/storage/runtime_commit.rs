//! Repo-local runtime file commit journal。
//!
//! Reference: `storage/archive_fs.rs` 的 fs2 single-writer lock 与 atomic replace。
//! 本模块面向 `.wiki` page/formal pointer 重新设计 phase/recovery，属于改写而非直接迁移。

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeCommitPhase {
    PlanPersisted,
    Staged,
    Applied,
    PointerCommitted,
    Finalized,
}

impl RuntimeCommitPhase {
    pub const ALL: [Self; 5] = [
        Self::PlanPersisted,
        Self::Staged,
        Self::Applied,
        Self::PointerCommitted,
        Self::Finalized,
    ];
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeCommitWrite {
    pub relative_path: String,
    pub content: Vec<u8>,
    #[serde(default)]
    pub commit_pointer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeCommitPlan {
    pub schema_version: String,
    pub operation_id: String,
    pub base_snapshot_id: Option<String>,
    pub target_snapshot_id: String,
    pub writes: Vec<RuntimeCommitWrite>,
    pub removals: Vec<String>,
    pub base_hashes: BTreeMap<String, Option<String>>,
}

impl RuntimeCommitPlan {
    pub fn capture(
        repo_root: &Path,
        operation_id: impl Into<String>,
        base_snapshot_id: Option<String>,
        target_snapshot_id: impl Into<String>,
        mut writes: Vec<RuntimeCommitWrite>,
        mut removals: Vec<String>,
    ) -> io::Result<Self> {
        let operation_id = operation_id.into();
        validate_operation_id(&operation_id)?;
        writes.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        removals.sort();
        removals.dedup();
        let mut paths = writes
            .iter()
            .map(|write| write.relative_path.clone())
            .chain(removals.iter().cloned())
            .collect::<Vec<_>>();
        paths.sort();
        paths.dedup();
        let mut base_hashes = BTreeMap::new();
        for relative_path in paths {
            validate_runtime_path(&relative_path)?;
            let path = repo_root.join(&relative_path);
            let hash = path.is_file().then(|| hash_file(&path)).transpose()?;
            base_hashes.insert(relative_path, hash);
        }
        Ok(Self {
            schema_version: "runtime-commit-v1".to_string(),
            operation_id,
            base_snapshot_id,
            target_snapshot_id: target_snapshot_id.into(),
            writes,
            removals,
            base_hashes,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RuntimeCommitState {
    phase: RuntimeCommitPhase,
}

pub fn execute_runtime_commit(
    repo_root: &Path,
    plan: &RuntimeCommitPlan,
    fail_after: Option<RuntimeCommitPhase>,
) -> io::Result<()> {
    validate_plan(plan)?;
    let _lock = acquire_lock(repo_root)?;
    recover_incomplete_locked(repo_root)?;
    verify_base_hashes(repo_root, plan)?;
    let operation_root = operation_root(repo_root, &plan.operation_id);
    fs::create_dir_all(operation_root.join("staging"))?;
    fs::create_dir_all(operation_root.join("before"))?;
    write_new_json(&operation_root.join("plan.json"), plan)?;
    snapshot_before(repo_root, plan, &operation_root)?;
    write_state(&operation_root, RuntimeCommitPhase::PlanPersisted)?;
    fail_if_requested(fail_after, RuntimeCommitPhase::PlanPersisted)?;

    stage_writes(plan, &operation_root)?;
    write_state(&operation_root, RuntimeCommitPhase::Staged)?;
    fail_if_requested(fail_after, RuntimeCommitPhase::Staged)?;

    apply_non_pointer(repo_root, plan, &operation_root)?;
    write_state(&operation_root, RuntimeCommitPhase::Applied)?;
    fail_if_requested(fail_after, RuntimeCommitPhase::Applied)?;

    apply_pointer(repo_root, plan, &operation_root)?;
    write_state(&operation_root, RuntimeCommitPhase::PointerCommitted)?;
    fail_if_requested(fail_after, RuntimeCommitPhase::PointerCommitted)?;

    write_state(&operation_root, RuntimeCommitPhase::Finalized)?;
    fail_if_requested(fail_after, RuntimeCommitPhase::Finalized)?;
    fs::remove_dir_all(operation_root)
}

pub fn recover_runtime_commits(repo_root: &Path) -> io::Result<()> {
    if !commits_root(repo_root).is_dir() {
        return Ok(());
    }
    let _lock = acquire_lock(repo_root)?;
    recover_incomplete_locked(repo_root)
}

pub fn has_incomplete_runtime_commit(repo_root: &Path) -> bool {
    let root = commits_root(repo_root);
    fs::read_dir(root)
        .map(|entries| {
            entries.filter_map(Result::ok).any(|entry| {
                entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false)
                    && entry.path().join("plan.json").is_file()
            })
        })
        .unwrap_or(false)
}

fn recover_incomplete_locked(repo_root: &Path) -> io::Result<()> {
    let root = commits_root(repo_root);
    if !root.is_dir() {
        return Ok(());
    }
    let mut operations = fs::read_dir(&root)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
        .collect::<Vec<_>>();
    operations.sort_by_key(|entry| entry.file_name());
    for entry in operations {
        let operation_root = entry.path();
        if !operation_root.join("plan.json").is_file() {
            continue;
        }
        let plan: RuntimeCommitPlan = read_json(&operation_root.join("plan.json"))?;
        validate_plan(&plan)?;
        let state: RuntimeCommitState = read_json(&operation_root.join("state.json"))?;
        let pointer_matches = pointer_matches_staging(repo_root, &plan, &operation_root)?;
        if state.phase >= RuntimeCommitPhase::PointerCommitted || pointer_matches {
            apply_non_pointer(repo_root, &plan, &operation_root)?;
            apply_pointer(repo_root, &plan, &operation_root)?;
        } else {
            rollback(repo_root, &plan, &operation_root)?;
        }
        fs::remove_dir_all(operation_root)?;
    }
    Ok(())
}

fn commits_root(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki/.cache/runtime-commits")
}

fn operation_root(repo_root: &Path, operation_id: &str) -> PathBuf {
    commits_root(repo_root).join(operation_id)
}

struct RuntimeCommitLock(File);

impl Drop for RuntimeCommitLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

fn acquire_lock(repo_root: &Path) -> io::Result<RuntimeCommitLock> {
    let root = commits_root(repo_root);
    fs::create_dir_all(&root)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(root.join(".lock"))?;
    file.try_lock_exclusive()
        .map_err(|error| io::Error::new(io::ErrorKind::WouldBlock, error))?;
    Ok(RuntimeCommitLock(file))
}

fn validate_plan(plan: &RuntimeCommitPlan) -> io::Result<()> {
    if plan.schema_version != "runtime-commit-v1"
        || plan.target_snapshot_id.trim().is_empty()
        || plan
            .writes
            .iter()
            .filter(|write| write.commit_pointer)
            .count()
            != 1
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid runtime commit plan identity or pointer",
        ));
    }
    validate_operation_id(&plan.operation_id)?;
    for path in plan
        .writes
        .iter()
        .map(|write| write.relative_path.as_str())
        .chain(plan.removals.iter().map(String::as_str))
    {
        validate_runtime_path(path)?;
    }
    Ok(())
}

fn validate_operation_id(operation_id: &str) -> io::Result<()> {
    let mut components = Path::new(operation_id).components();
    let is_single_normal =
        matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none();
    if operation_id.trim().is_empty()
        || !is_single_normal
        || operation_id.contains(['/', '\\'])
        || operation_id == ".lock"
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "runtime commit operation id must be a single path component",
        ));
    }
    Ok(())
}

fn validate_runtime_path(relative_path: &str) -> io::Result<()> {
    let path = Path::new(relative_path);
    if path.is_absolute()
        || !relative_path.replace('\\', "/").starts_with(".wiki/")
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "runtime commit path must stay under .wiki",
        ));
    }
    Ok(())
}

fn verify_base_hashes(repo_root: &Path, plan: &RuntimeCommitPlan) -> io::Result<()> {
    for (relative_path, expected) in &plan.base_hashes {
        let path = repo_root.join(relative_path);
        let actual = path.is_file().then(|| hash_file(&path)).transpose()?;
        if &actual != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("runtime commit base hash mismatch: {relative_path}"),
            ));
        }
    }
    Ok(())
}

fn snapshot_before(
    repo_root: &Path,
    plan: &RuntimeCommitPlan,
    operation_root: &Path,
) -> io::Result<()> {
    for (index, (relative_path, hash)) in plan.base_hashes.iter().enumerate() {
        if hash.is_some() {
            fs::copy(
                repo_root.join(relative_path),
                operation_root
                    .join("before")
                    .join(format!("{index:06}.bin")),
            )?;
        }
    }
    Ok(())
}

fn stage_writes(plan: &RuntimeCommitPlan, operation_root: &Path) -> io::Result<()> {
    for (index, write) in plan.writes.iter().enumerate() {
        write_new(
            &operation_root
                .join("staging")
                .join(format!("{index:06}.bin")),
            &write.content,
        )?;
    }
    Ok(())
}

fn apply_non_pointer(
    repo_root: &Path,
    plan: &RuntimeCommitPlan,
    operation_root: &Path,
) -> io::Result<()> {
    for (index, write) in plan.writes.iter().enumerate() {
        if !write.commit_pointer {
            replace_from_staging(repo_root, write, index, operation_root)?;
        }
    }
    for removal in &plan.removals {
        let path = repo_root.join(removal);
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn apply_pointer(
    repo_root: &Path,
    plan: &RuntimeCommitPlan,
    operation_root: &Path,
) -> io::Result<()> {
    for (index, write) in plan.writes.iter().enumerate() {
        if write.commit_pointer {
            replace_from_staging(repo_root, write, index, operation_root)?;
        }
    }
    Ok(())
}

fn replace_from_staging(
    repo_root: &Path,
    write: &RuntimeCommitWrite,
    index: usize,
    operation_root: &Path,
) -> io::Result<()> {
    let staged = operation_root
        .join("staging")
        .join(format!("{index:06}.bin"));
    let content = fs::read(staged)?;
    write_atomic(&repo_root.join(&write.relative_path), &content)
}

fn rollback(repo_root: &Path, plan: &RuntimeCommitPlan, operation_root: &Path) -> io::Result<()> {
    for (index, (relative_path, before_hash)) in plan.base_hashes.iter().enumerate() {
        let path = repo_root.join(relative_path);
        if before_hash.is_some() {
            let content = fs::read(
                operation_root
                    .join("before")
                    .join(format!("{index:06}.bin")),
            )?;
            write_atomic(&path, &content)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn pointer_matches_staging(
    repo_root: &Path,
    plan: &RuntimeCommitPlan,
    operation_root: &Path,
) -> io::Result<bool> {
    let Some((index, pointer)) = plan
        .writes
        .iter()
        .enumerate()
        .find(|(_, write)| write.commit_pointer)
    else {
        return Ok(false);
    };
    let live = repo_root.join(&pointer.relative_path);
    if !live.is_file() {
        return Ok(false);
    }
    let staged = operation_root
        .join("staging")
        .join(format!("{index:06}.bin"));
    if !staged.is_file() {
        return Ok(false);
    }
    Ok(hash_file(&live)? == hash_file(&staged)?)
}

fn fail_if_requested(
    fail_after: Option<RuntimeCommitPhase>,
    phase: RuntimeCommitPhase,
) -> io::Result<()> {
    if fail_after == Some(phase) {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            format!("runtime commit failpoint after {phase:?}"),
        ))
    } else {
        Ok(())
    }
}

fn write_state(operation_root: &Path, phase: RuntimeCommitPhase) -> io::Result<()> {
    write_json_atomic(
        &operation_root.join("state.json"),
        &RuntimeCommitState { phase },
    )
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    serde_json::from_slice(&fs::read(path)?)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn write_new_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    write_new(path, &bytes)
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    write_atomic(path, &bytes)
}

fn write_new(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("runtime-commit-tmp");
    if tmp.exists() {
        fs::remove_file(&tmp)?;
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    #[cfg(windows)]
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(tmp, path)
}

fn hash_file(path: &Path) -> io::Result<String> {
    Ok(blake3::hash(&fs::read(path)?).to_hex().to_string())
}
