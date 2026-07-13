//! Archive 写侧文件系统适配器。

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};

use fs2::FileExt;
use serde::Serialize;
use serde_yaml::{Mapping, Value};
use wiki_model::domain::governance::{
    ArchiveArtifactHash, ArchiveCheckpoint, ArchiveErrorKind, ArchiveOperationManifest,
    ArchiveOperationStatus, ArchiveOutcome, ArchiveParentDiff,
};

pub const ARCHIVE_SCHEMA_VERSION: &str = "archive-operation-v1";
pub const ARCHIVE_ALGORITHM_VERSION: &str = "archive-precondition-v1";

#[derive(Debug)]
pub struct ArchiveFailure {
    pub kind: ArchiveErrorKind,
    message: String,
}

impl std::fmt::Display for ArchiveFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ArchiveFailure {}

pub fn archive_failure(kind: ArchiveErrorKind, message: impl Into<String>) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        ArchiveFailure {
            kind,
            message: message.into(),
        },
    )
}

pub fn archive_failure_kind(error: &io::Error) -> Option<ArchiveErrorKind> {
    error
        .get_ref()
        .and_then(|source| source.downcast_ref::<ArchiveFailure>())
        .map(|failure| failure.kind)
}

#[derive(Debug, Clone)]
pub struct ParentPatch {
    pub diff: ArchiveParentDiff,
    pub meta_after: String,
    pub split_after: String,
}

pub struct ArchiveLock {
    file: File,
}

impl Drop for ArchiveLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

#[derive(Debug, Clone)]
pub struct ArchiveFs {
    repo_root: PathBuf,
}

impl ArchiveFs {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn operation_root(&self, operation_id: &str) -> PathBuf {
        self.repo_root
            .join(".spec/.runtime/archive-operations")
            .join(operation_id)
    }

    pub fn snapshot_tree(&self, relative: &str) -> io::Result<Vec<ArchiveArtifactHash>> {
        validate_relative_path(relative)?;
        let root = self.repo_root.join(relative);
        let canonical_repo = fs::canonicalize(&self.repo_root)?;
        let canonical_root = fs::canonicalize(&root)?;
        if !canonical_root.starts_with(&canonical_repo) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "archive source escapes repository",
            ));
        }
        let mut result = Vec::new();
        self.walk_tree(&root, &root, &mut result)?;
        result.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(result)
    }

    fn walk_tree(
        &self,
        root: &Path,
        current: &Path,
        output: &mut Vec<ArchiveArtifactHash>,
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unsafe archive source entry: {}", path.display()),
                ));
            }
            let relative = path.strip_prefix(root).map_err(io::Error::other)?;
            let relative = relative.to_string_lossy().replace('\\', "/");
            if metadata.is_dir() {
                self.walk_tree(root, &path, output)?;
            } else if metadata.is_file() {
                output.push(ArchiveArtifactHash {
                    relative_path: relative,
                    file_type: "file".to_string(),
                    size: metadata.len(),
                    content_hash: hash_file(&path)?,
                });
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unsafe archive source entry: {}", path.display()),
                ));
            }
        }
        Ok(())
    }

    pub fn build_parent_patch(
        &self,
        parent_id: &str,
        child_id: &str,
        archived_at: &str,
        archived_to: &str,
    ) -> io::Result<ParentPatch> {
        let meta_path = format!(".spec/changes/{parent_id}/meta.yaml");
        let split_path = format!(".spec/changes/{parent_id}/split.md");
        let meta_before = fs::read_to_string(self.repo_root.join(&meta_path))?;
        let split_before = fs::read_to_string(self.repo_root.join(&split_path))?;
        let meta_after = patch_parent_meta(&meta_before, child_id, archived_at, archived_to)?;
        let split_after = patch_parent_split(&split_before, child_id)?;
        Ok(ParentPatch {
            diff: ArchiveParentDiff {
                parent_id: parent_id.to_string(),
                meta_path,
                split_path,
                archived_at: archived_at.to_string(),
                archived_to: archived_to.to_string(),
                meta_before_hash: hash_bytes(meta_before.as_bytes()),
                meta_after_hash: hash_bytes(meta_after.as_bytes()),
                split_before_hash: hash_bytes(split_before.as_bytes()),
                split_after_hash: hash_bytes(split_after.as_bytes()),
            },
            meta_after,
            split_after,
        })
    }

    pub fn compute_digest<T: Serialize>(&self, value: &T) -> io::Result<String> {
        let bytes = serde_json::to_vec(value).map_err(io::Error::other)?;
        Ok(hash_bytes(&bytes))
    }

    pub fn acquire_lock(
        &self,
        change_id: &str,
        parent_id: Option<&str>,
    ) -> io::Result<ArchiveLock> {
        let mutation_owner = parent_id.unwrap_or(change_id);
        let key = hash_bytes(format!("archive-mutation-set\0{mutation_owner}").as_bytes());
        let dir = self.repo_root.join(".spec/.runtime/archive-locks");
        fs::create_dir_all(&dir)?;
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(dir.join(format!("{key}.lock")))?;
        file.try_lock_exclusive()
            .map_err(|error| io::Error::new(io::ErrorKind::WouldBlock, error))?;
        Ok(ArchiveLock { file })
    }

    pub fn persist_plan(
        &self,
        manifest: &ArchiveOperationManifest,
        patch: Option<&ParentPatch>,
    ) -> io::Result<PathBuf> {
        let root = self.operation_root(&manifest.operation_id);
        fs::create_dir_all(root.join("checkpoints"))?;
        fs::create_dir_all(root.join("staging"))?;
        write_new_json(&root.join("plan.json"), manifest)?;
        if let Some(patch) = patch {
            write_new(
                &root.join("staging/parent-meta.after"),
                patch.meta_after.as_bytes(),
            )?;
            write_new(
                &root.join("staging/parent-split.after"),
                patch.split_after.as_bytes(),
            )?;
            fs::copy(
                self.repo_root.join(&patch.diff.meta_path),
                root.join("staging/parent-meta.before"),
            )?;
            fs::copy(
                self.repo_root.join(&patch.diff.split_path),
                root.join("staging/parent-split.before"),
            )?;
        }
        Ok(root)
    }

    pub fn append_checkpoint(
        &self,
        operation_id: &str,
        checkpoint: &ArchiveCheckpoint,
    ) -> io::Result<()> {
        let path = self.operation_root(operation_id).join("checkpoints").join(
            format!(
                "{:06}-{:02}-{:?}.json",
                checkpoint.sequence, checkpoint.attempt, checkpoint.step
            )
            .to_ascii_lowercase(),
        );
        write_new_json(&path, checkpoint)
    }

    pub fn write_result(&self, manifest: &ArchiveOperationManifest) -> io::Result<()> {
        write_new_json(
            &self
                .operation_root(&manifest.operation_id)
                .join("result.json"),
            manifest,
        )
    }

    pub fn read_plan(&self, operation_id: &str) -> io::Result<ArchiveOperationManifest> {
        let content = fs::read_to_string(self.operation_root(operation_id).join("plan.json"))
            .map_err(|error| {
                archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    format!("read archive plan: {error}"),
                )
            })?;
        let plan: ArchiveOperationManifest = serde_json::from_str(&content).map_err(|error| {
            archive_failure(
                ArchiveErrorKind::ManifestInvalid,
                format!("parse archive plan: {error}"),
            )
        })?;
        if plan.schema_version != ARCHIVE_SCHEMA_VERSION
            || plan.algorithm_version != ARCHIVE_ALGORITHM_VERSION
            || plan.operation_id != operation_id
            || plan.change_id.trim().is_empty()
        {
            return Err(archive_failure(
                ArchiveErrorKind::ManifestInvalid,
                "invalid archive operation manifest identity or version",
            ));
        }
        validate_relative_path(&plan.source_path)?;
        validate_relative_path(&plan.target_path)?;
        if let Some(operation_root) = plan.operation_root.as_deref() {
            validate_relative_path(operation_root)?;
        }
        Ok(plan)
    }

    pub fn read_result(
        &self,
        operation_id: &str,
        plan: &ArchiveOperationManifest,
    ) -> io::Result<ArchiveOperationManifest> {
        let content = fs::read_to_string(self.operation_root(operation_id).join("result.json"))
            .map_err(|error| {
                archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    format!("read archive result: {error}"),
                )
            })?;
        let result: ArchiveOperationManifest = serde_json::from_str(&content).map_err(|error| {
            archive_failure(
                ArchiveErrorKind::ManifestInvalid,
                format!("parse archive result: {error}"),
            )
        })?;
        if result.schema_version != plan.schema_version
            || result.algorithm_version != plan.algorithm_version
            || result.operation_id != plan.operation_id
            || result.change_id != plan.change_id
            || result.source_path != plan.source_path
            || result.target_path != plan.target_path
            || result.precondition_digest != plan.precondition_digest
            || result.outcome != ArchiveOutcome::Completed
            || result.status != ArchiveOperationStatus::Completed
        {
            return Err(archive_failure(
                ArchiveErrorKind::ManifestInvalid,
                "archive result does not match immutable plan",
            ));
        }
        Ok(result)
    }

    pub fn read_checkpoints(&self, operation_id: &str) -> io::Result<Vec<ArchiveCheckpoint>> {
        let root = self.operation_root(operation_id).join("checkpoints");
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut checkpoints = Vec::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if !entry.file_type()?.is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("json")
            {
                continue;
            }
            let content = fs::read_to_string(entry.path()).map_err(|error| {
                archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    format!("read archive checkpoint: {error}"),
                )
            })?;
            checkpoints.push(serde_json::from_str::<ArchiveCheckpoint>(&content).map_err(
                |error| {
                    archive_failure(
                        ArchiveErrorKind::ManifestInvalid,
                        format!("parse archive checkpoint: {error}"),
                    )
                },
            )?);
        }
        checkpoints.sort_by_key(|checkpoint| checkpoint.sequence);
        for pair in checkpoints.windows(2) {
            if pair[0].sequence >= pair[1].sequence {
                return Err(archive_failure(
                    ArchiveErrorKind::ManifestInvalid,
                    "archive checkpoint sequence must be strictly increasing",
                ));
            }
        }
        Ok(checkpoints)
    }

    pub fn discover_incomplete(&self, change_id: &str) -> io::Result<Vec<String>> {
        Ok(self
            .discover_operations(change_id)?
            .into_iter()
            .filter(|operation| !self.operation_root(operation).join("result.json").exists())
            .collect())
    }

    pub fn discover_completed(&self, change_id: &str) -> io::Result<Vec<String>> {
        let mut completed = Vec::new();
        for operation in self.discover_operations(change_id)? {
            if self.operation_root(&operation).join("result.json").exists() {
                let plan = self.read_plan(&operation)?;
                self.read_result(&operation, &plan)?;
                completed.push(operation);
            }
        }
        Ok(completed)
    }

    fn discover_operations(&self, change_id: &str) -> io::Result<Vec<String>> {
        let root = self.repo_root.join(".spec/.runtime/archive-operations");
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let operation_id = entry.file_name().to_string_lossy().to_string();
            let plan = self.read_plan(&operation_id).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid archive operation manifest {operation_id}: {error}"),
                )
            })?;
            if plan.change_id == change_id {
                found.push(operation_id);
            }
        }
        found.sort();
        Ok(found)
    }

    pub fn rename_source(&self, source: &str, target: &str) -> io::Result<()> {
        validate_relative_path(source)?;
        validate_relative_path(target)?;
        let source = self.repo_root.join(source);
        let target = self.repo_root.join(target);
        if target.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "archive target already exists",
            ));
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(source, target)
    }

    pub fn replace_parent_files(&self, patch: &ParentPatch) -> io::Result<()> {
        if hash_bytes(patch.meta_after.as_bytes()) != patch.diff.meta_after_hash
            || hash_bytes(patch.split_after.as_bytes()) != patch.diff.split_after_hash
        {
            return Err(archive_failure(
                ArchiveErrorKind::ManifestInvalid,
                "archive parent staging hash does not match immutable plan",
            ));
        }
        let meta_path = self.repo_root.join(&patch.diff.meta_path);
        let split_path = self.repo_root.join(&patch.diff.split_path);
        let meta_current = hash_file(&meta_path)?;
        let split_current = hash_file(&split_path)?;
        if meta_current != patch.diff.meta_before_hash && meta_current != patch.diff.meta_after_hash
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "parent meta has unknown external changes",
            ));
        }
        if split_current != patch.diff.split_before_hash
            && split_current != patch.diff.split_after_hash
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "parent split has unknown external changes",
            ));
        }
        if meta_current != patch.diff.meta_after_hash {
            write_atomic(&meta_path, patch.meta_after.as_bytes())?;
        }
        if split_current != patch.diff.split_after_hash {
            write_atomic(&split_path, patch.split_after.as_bytes())?;
        }
        Ok(())
    }
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn validate_relative_path(value: &str) -> io::Result<()> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "archive path must be repo-relative without traversal",
        ));
    }
    Ok(())
}

pub fn patch_parent_meta(
    content: &str,
    child_id: &str,
    archived_at: &str,
    archived_to: &str,
) -> io::Result<String> {
    let mut root: Value = serde_yaml::from_str(content).map_err(io::Error::other)?;
    let children = root
        .get_mut("multiChange")
        .and_then(|value| value.get_mut("children"))
        .and_then(Value::as_sequence_mut)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "parent metadata has no children",
            )
        })?;
    let matching = children
        .iter_mut()
        .filter(|child| child.get("id").and_then(Value::as_str) == Some(child_id))
        .collect::<Vec<_>>();
    if matching.len() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "parent child entry must be unique",
        ));
    }
    let mapping = matching
        .into_iter()
        .next()
        .and_then(Value::as_mapping_mut)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "parent child entry must be mapping",
            )
        })?;
    insert_string(mapping, "archiveStatus", "archived");
    insert_string(mapping, "archivedAt", archived_at);
    insert_string(mapping, "archivedTo", archived_to);
    serde_yaml::to_string(&root).map_err(io::Error::other)
}

fn insert_string(mapping: &mut Mapping, key: &str, value: &str) {
    mapping.insert(
        Value::String(key.to_string()),
        Value::String(value.to_string()),
    );
}

pub fn patch_parent_split(content: &str, child_id: &str) -> io::Result<String> {
    let heading = format!("{child_id}\n");
    let starts = content
        .match_indices(&heading)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if starts.len() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "split child section must be unique",
        ));
    }
    let start = starts[0];
    let tail = &content[start..];
    let end = tail[heading.len()..]
        .find("\n### ")
        .map(|offset| start + heading.len() + offset)
        .unwrap_or(content.len());
    let section = &content[start..end];
    let marker = "- 归档状态：[ ] pending";
    if section.matches(marker).count() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "split archive marker must be unique pending marker",
        ));
    }
    let mut updated = String::with_capacity(content.len());
    updated.push_str(&content[..start]);
    updated.push_str(&section.replacen(marker, "- 归档状态：[x] archived", 1));
    updated.push_str(&content[end..]);
    Ok(updated)
}

fn write_new_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    write_new(path, &bytes)
}

fn write_new(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension(format!(
        "{}.archive-tmp",
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("tmp")
    ));
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
