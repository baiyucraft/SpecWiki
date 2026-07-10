//! `.spec` 治理证据的只读文件系统适配器。
//!
//! 本模块负责发现、规范化和哈希 evidence，不解释治理 readiness，也不写入 `.spec`。

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use wiki_model::domain::governance::{
    GovernanceArtifactRef, GovernanceArtifactStatus, GovernanceLocation,
};

const DEFAULT_MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;

const ARTIFACTS: [(&str, &str); 8] = [
    ("split", "split.md"),
    ("proposal", "proposal.md"),
    ("design", "design.md"),
    ("cases", "system-tests.md"),
    ("tasks", "tasks.md"),
    ("review-report", "review-report.md"),
    ("test-report", "test-report.md"),
    ("metadata", "meta.yaml"),
];

/// 治理 evidence 的只读发现端口。
pub trait GovernanceEvidenceStore {
    /// 读取当前仓库的 live evidence；单个 change 损坏会进入 `failures`，不会中止其它 change。
    fn discover_repo(&self) -> io::Result<GovernanceEvidenceSnapshot>;
}

/// `.spec` 中一次确定性发现得到的规范化快照。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceEvidenceSnapshot {
    /// `.spec` 根是否存在且启用。
    pub enabled: bool,
    /// 所有已知 evidence 文件内容与位置共同形成的稳定哈希。
    pub fingerprint: Option<String>,
    /// 成功解析的 active changes，按 id 排序。
    pub active_changes: Vec<GovernanceChangeEvidence>,
    /// 成功解析的 archived changes，按 id 和路径排序。
    pub archived_changes: Vec<GovernanceChangeEvidence>,
    /// 已知 artifact 的无正文引用，包含 missing/empty 状态。
    pub artifacts: Vec<GovernanceArtifactRef>,
    /// 无法可靠解析的局部 evidence；调用方据此产生 conflict issue。
    pub failures: Vec<GovernanceEvidenceFailure>,
}

/// 单个 active 或 archived change 的规范化 evidence。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceChangeEvidence {
    /// 来自目录名的 change id；metadata id 由 policy 单独核对。
    pub id: String,
    /// Change 位于 active 或 archive tree。
    pub location: GovernanceLocation,
    /// Change 目录的仓库相对路径。
    pub relative_path: String,
    /// 已解析 metadata。
    pub metadata: GovernanceMetadataEvidence,
    /// 已解析 review report frontmatter；不存在或损坏时为 `None`。
    pub review_report: Option<GovernanceReviewEvidence>,
    /// 已解析 test report frontmatter；不存在或损坏时为 `None`。
    pub test_report: Option<GovernanceTestEvidence>,
    /// `split.md` 中被显式勾选 archived 的 child ids。
    pub split_archived_children: BTreeSet<String>,
}

/// `meta.yaml` 中 policy 所需的规范化字段。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceMetadataEvidence {
    /// Metadata 声明的 change id。
    pub id: String,
    /// UniSpec lifecycle stage。
    pub stage: String,
    /// `single-change` 或 `multi-change`；缺失由 policy 判断一致性。
    #[serde(default)]
    pub delivery_shape: Option<String>,
    /// Parent/child 协作 metadata。
    #[serde(default)]
    pub multi_change: Option<GovernanceMultiChangeEvidence>,
}

/// `multiChange` 的 parent/child 公共证据。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceMultiChangeEvidence {
    /// `parent` 或 `child`。
    pub role: String,
    /// Parent 声明的 child 列表。
    #[serde(default)]
    pub children: Vec<GovernanceChildEvidence>,
    /// Child 所属 parent id。
    #[serde(default)]
    pub parent: Option<String>,
    /// Child 在 parent 中的顺序。
    #[serde(default)]
    pub order: Option<u32>,
    /// Child 声明的同级依赖。
    #[serde(default)]
    pub depends_on: Vec<String>,
}

/// Parent metadata 中的 child 引用与 archive marker。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceChildEvidence {
    /// Child change id。
    pub id: String,
    /// Child 执行顺序。
    pub order: u32,
    /// Child 的同级依赖。
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// 归档后写回的固定 marker。
    #[serde(default)]
    pub archive_status: Option<String>,
    /// 归档时间证据。
    #[serde(default)]
    pub archived_at: Option<String>,
    /// 仓库相对 archive 目录。
    #[serde(default)]
    pub archived_to: Option<String>,
}

/// `review-report.md` 的机器可读 frontmatter。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceReviewEvidence {
    /// `pass`、`fail` 或 `partial`。
    pub review_result: String,
    /// `full` 或 `partial`。
    pub scope: String,
}

/// `test-report.md` 的机器可读 frontmatter。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceTestEvidence {
    /// `pass`、`fail` 或 `skipped`。
    pub verification_result: String,
    /// `full` 或 `partial`。
    pub scope: String,
}

/// 单个 evidence 读取或解析失败，不包含文件正文。
#[derive(Debug, Clone, Serialize)]
pub struct GovernanceEvidenceFailure {
    /// 稳定机器规则 id。
    pub rule_id: String,
    /// 能定位到 change 时提供目录 id。
    pub change_id: Option<String>,
    /// 失败 evidence 的仓库相对路径。
    pub relative_path: String,
    /// 面向开发者的诊断信息。
    pub message: String,
}

/// 从仓库根读取 `.spec/changes` 和 `.spec/archive` 的实现。
#[derive(Debug, Clone)]
pub struct FsGovernanceEvidenceStore {
    repo_root: PathBuf,
    max_file_bytes: u64,
}

impl FsGovernanceEvidenceStore {
    /// 创建使用默认 4 MiB 单文件解析上限的 store。
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
        }
    }

    /// 覆盖单文件解析上限，主要用于受控环境与边界测试。
    pub fn with_max_file_bytes(mut self, max_file_bytes: u64) -> Self {
        self.max_file_bytes = max_file_bytes;
        self
    }

    /// 读取当前仓库的 live governance evidence。
    pub fn discover_repo(&self) -> io::Result<GovernanceEvidenceSnapshot> {
        <Self as GovernanceEvidenceStore>::discover_repo(self)
    }

    fn discover_tree(
        &self,
        root: &Path,
        location: GovernanceLocation,
        repo_canonical: &Path,
        changes: &mut Vec<GovernanceChangeEvidence>,
        artifacts: &mut Vec<GovernanceArtifactRef>,
        failures: &mut Vec<GovernanceEvidenceFailure>,
    ) {
        let mut entries = match fs::read_dir(root) {
            Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<_>>(),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return,
            Err(error) => {
                failures.push(GovernanceEvidenceFailure {
                    rule_id: "evidence.tree.read".to_string(),
                    change_id: None,
                    relative_path: relative_path(&self.repo_root, root),
                    message: error.to_string(),
                });
                return;
            }
        };
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_symlink() {
                failures.push(GovernanceEvidenceFailure {
                    rule_id: "evidence.path.escape".to_string(),
                    change_id: None,
                    relative_path: relative_path(&self.repo_root, &entry.path()),
                    message: "change directories must not be symbolic links".to_string(),
                });
                continue;
            }
            if !file_type.is_dir() {
                continue;
            }
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            let Some(path_id) = change_id_from_entry(&entry_name, location) else {
                failures.push(GovernanceEvidenceFailure {
                    rule_id: "evidence.archive.path".to_string(),
                    change_id: None,
                    relative_path: relative_path(&self.repo_root, &entry.path()),
                    message: "archive directory must use YYYY-MM-DD-<change-id>".to_string(),
                });
                continue;
            };

            if let Some(change) = self.discover_change(
                &entry.path(),
                path_id,
                location,
                repo_canonical,
                artifacts,
                failures,
            ) {
                changes.push(change);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn discover_change(
        &self,
        change_dir: &Path,
        path_id: String,
        location: GovernanceLocation,
        repo_canonical: &Path,
        all_artifacts: &mut Vec<GovernanceArtifactRef>,
        failures: &mut Vec<GovernanceEvidenceFailure>,
    ) -> Option<GovernanceChangeEvidence> {
        let mut metadata_content = None;
        let mut review_content = None;
        let mut test_content = None;
        let mut split_content = None;

        for (kind, file_name) in ARTIFACTS {
            let path = change_dir.join(file_name);
            let inspected = self.inspect_file(&path, &path_id, repo_canonical, failures);
            let artifact = GovernanceArtifactRef {
                change_id: path_id.clone(),
                kind: kind.to_string(),
                relative_path: relative_path(&self.repo_root, &path),
                status: inspected.status,
                content_hash: inspected.content_hash,
            };
            match kind {
                "metadata" => metadata_content = inspected.content,
                "review-report" => review_content = inspected.content,
                "test-report" => test_content = inspected.content,
                "split" => split_content = inspected.content,
                _ => {}
            }
            all_artifacts.push(artifact);
        }

        let metadata_path = relative_path(&self.repo_root, &change_dir.join("meta.yaml"));
        let metadata: GovernanceMetadataEvidence = match metadata_content {
            Some(content) => match serde_yaml::from_str(&content) {
                Ok(metadata) => metadata,
                Err(error) => {
                    failures.push(GovernanceEvidenceFailure {
                        rule_id: "evidence.metadata.parse".to_string(),
                        change_id: Some(path_id),
                        relative_path: metadata_path,
                        message: error.to_string(),
                    });
                    return None;
                }
            },
            None => {
                failures.push(GovernanceEvidenceFailure {
                    rule_id: "evidence.metadata.parse".to_string(),
                    change_id: Some(path_id),
                    relative_path: metadata_path,
                    message: "metadata is missing, empty, unreadable, or too large".to_string(),
                });
                return None;
            }
        };

        let review_report = review_content.and_then(|content| {
            parse_review_report(&content).map_or_else(
                |message| {
                    failures.push(GovernanceEvidenceFailure {
                        rule_id: "report.review.parse".to_string(),
                        change_id: Some(path_id.clone()),
                        relative_path: relative_path(
                            &self.repo_root,
                            &change_dir.join("review-report.md"),
                        ),
                        message,
                    });
                    None
                },
                Some,
            )
        });
        let test_report = test_content.and_then(|content| {
            parse_test_report(&content).map_or_else(
                |message| {
                    failures.push(GovernanceEvidenceFailure {
                        rule_id: "report.verification.parse".to_string(),
                        change_id: Some(path_id.clone()),
                        relative_path: relative_path(
                            &self.repo_root,
                            &change_dir.join("test-report.md"),
                        ),
                        message,
                    });
                    None
                },
                Some,
            )
        });

        let split_archived_children = if let Some(content) = split_content.as_deref() {
            metadata
                .multi_change
                .as_ref()
                .map(|multi| {
                    multi
                        .children
                        .iter()
                        .filter(|child| split_marks_archived(content, &child.id))
                        .map(|child| child.id.clone())
                        .collect()
                })
                .unwrap_or_default()
        } else {
            BTreeSet::new()
        };

        Some(GovernanceChangeEvidence {
            id: path_id,
            location,
            relative_path: relative_path(&self.repo_root, change_dir),
            metadata,
            review_report,
            test_report,
            split_archived_children,
        })
    }

    fn inspect_file(
        &self,
        path: &Path,
        change_id: &str,
        repo_canonical: &Path,
        failures: &mut Vec<GovernanceEvidenceFailure>,
    ) -> InspectedFile {
        let relative = relative_path(&self.repo_root, path);
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return InspectedFile::missing()
            }
            Err(error) => {
                failures.push(file_failure(
                    "evidence.artifact.read",
                    change_id,
                    &relative,
                    error,
                ));
                return InspectedFile::missing();
            }
        };
        if metadata.file_type().is_symlink() {
            match fs::canonicalize(path) {
                Ok(target) if target.starts_with(repo_canonical) => {}
                _ => {
                    failures.push(GovernanceEvidenceFailure {
                        rule_id: "evidence.path.escape".to_string(),
                        change_id: Some(change_id.to_string()),
                        relative_path: relative,
                        message: "symlink target escapes repository root".to_string(),
                    });
                    return InspectedFile::missing();
                }
            }
        }
        if !metadata.is_file() && !metadata.file_type().is_symlink() {
            return InspectedFile::missing();
        }

        let content_hash = match hash_file(path) {
            Ok(hash) => Some(hash),
            Err(error) => {
                failures.push(file_failure(
                    "evidence.artifact.read",
                    change_id,
                    &relative,
                    error,
                ));
                return InspectedFile::missing();
            }
        };
        if metadata.len() > self.max_file_bytes {
            failures.push(GovernanceEvidenceFailure {
                rule_id: "evidence.artifact.too_large".to_string(),
                change_id: Some(change_id.to_string()),
                relative_path: relative,
                message: format!(
                    "artifact has {} bytes; limit is {}",
                    metadata.len(),
                    self.max_file_bytes
                ),
            });
            return InspectedFile {
                status: GovernanceArtifactStatus::Present,
                content_hash,
                content: None,
            };
        }

        match fs::read_to_string(path) {
            Ok(content) if content.trim().is_empty() => InspectedFile {
                status: GovernanceArtifactStatus::Empty,
                content_hash,
                content: Some(content),
            },
            Ok(content) => InspectedFile {
                status: GovernanceArtifactStatus::Present,
                content_hash,
                content: Some(content),
            },
            Err(error) => {
                failures.push(file_failure(
                    "evidence.artifact.read",
                    change_id,
                    &relative,
                    error,
                ));
                InspectedFile::missing()
            }
        }
    }
}

impl GovernanceEvidenceStore for FsGovernanceEvidenceStore {
    fn discover_repo(&self) -> io::Result<GovernanceEvidenceSnapshot> {
        let spec_root = self.repo_root.join(".spec");
        if !spec_root.exists() {
            return Ok(GovernanceEvidenceSnapshot {
                enabled: false,
                fingerprint: None,
                active_changes: Vec::new(),
                archived_changes: Vec::new(),
                artifacts: Vec::new(),
                failures: Vec::new(),
            });
        }

        let repo_canonical = fs::canonicalize(&self.repo_root)?;
        match fs::canonicalize(&spec_root) {
            Ok(path) if path.starts_with(&repo_canonical) && path.is_dir() => {}
            Ok(_) => {
                let failure = GovernanceEvidenceFailure {
                    rule_id: "evidence.path.escape".to_string(),
                    change_id: None,
                    relative_path: ".spec".to_string(),
                    message: ".spec root must be a directory inside the repository".to_string(),
                };
                return Ok(GovernanceEvidenceSnapshot {
                    enabled: true,
                    fingerprint: Some(compute_fingerprint(&[], std::slice::from_ref(&failure))),
                    active_changes: Vec::new(),
                    archived_changes: Vec::new(),
                    artifacts: Vec::new(),
                    failures: vec![failure],
                });
            }
            Err(error) => {
                let failure = GovernanceEvidenceFailure {
                    rule_id: "evidence.tree.read".to_string(),
                    change_id: None,
                    relative_path: ".spec".to_string(),
                    message: error.to_string(),
                };
                return Ok(GovernanceEvidenceSnapshot {
                    enabled: true,
                    fingerprint: Some(compute_fingerprint(&[], std::slice::from_ref(&failure))),
                    active_changes: Vec::new(),
                    archived_changes: Vec::new(),
                    artifacts: Vec::new(),
                    failures: vec![failure],
                });
            }
        }
        let mut active_changes = Vec::new();
        let mut archived_changes = Vec::new();
        let mut artifacts = Vec::new();
        let mut failures = Vec::new();
        self.discover_tree(
            &spec_root.join("changes"),
            GovernanceLocation::Active,
            &repo_canonical,
            &mut active_changes,
            &mut artifacts,
            &mut failures,
        );
        self.discover_tree(
            &spec_root.join("archive"),
            GovernanceLocation::Archived,
            &repo_canonical,
            &mut archived_changes,
            &mut artifacts,
            &mut failures,
        );

        active_changes.sort_by(|left, right| left.id.cmp(&right.id));
        archived_changes.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });
        artifacts.sort_by(|left, right| {
            left.change_id
                .cmp(&right.change_id)
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });
        failures.sort_by(|left, right| {
            left.change_id
                .cmp(&right.change_id)
                .then_with(|| left.rule_id.cmp(&right.rule_id))
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });
        let fingerprint = Some(compute_fingerprint(&artifacts, &failures));

        Ok(GovernanceEvidenceSnapshot {
            enabled: true,
            fingerprint,
            active_changes,
            archived_changes,
            artifacts,
            failures,
        })
    }
}

#[derive(Debug)]
struct InspectedFile {
    status: GovernanceArtifactStatus,
    content_hash: Option<String>,
    content: Option<String>,
}

impl InspectedFile {
    fn missing() -> Self {
        Self {
            status: GovernanceArtifactStatus::Missing,
            content_hash: None,
            content: None,
        }
    }
}

#[derive(Deserialize)]
struct ReviewFrontmatter {
    #[serde(rename = "review-result")]
    review_result: String,
    scope: String,
}

#[derive(Deserialize)]
struct TestFrontmatter {
    #[serde(rename = "verification-result")]
    verification_result: String,
    scope: String,
}

fn parse_review_report(content: &str) -> Result<GovernanceReviewEvidence, String> {
    let frontmatter = extract_frontmatter(content)?;
    let parsed: ReviewFrontmatter =
        serde_yaml::from_str(frontmatter).map_err(|error| error.to_string())?;
    if !matches!(parsed.review_result.as_str(), "pass" | "fail" | "partial") {
        return Err("review-result must be pass, fail, or partial".to_string());
    }
    validate_scope(&parsed.scope)?;
    Ok(GovernanceReviewEvidence {
        review_result: parsed.review_result,
        scope: parsed.scope,
    })
}

fn parse_test_report(content: &str) -> Result<GovernanceTestEvidence, String> {
    let frontmatter = extract_frontmatter(content)?;
    let parsed: TestFrontmatter =
        serde_yaml::from_str(frontmatter).map_err(|error| error.to_string())?;
    if !matches!(
        parsed.verification_result.as_str(),
        "pass" | "fail" | "skipped"
    ) {
        return Err("verification-result must be pass, fail, or skipped".to_string());
    }
    validate_scope(&parsed.scope)?;
    Ok(GovernanceTestEvidence {
        verification_result: parsed.verification_result,
        scope: parsed.scope,
    })
}

fn extract_frontmatter(content: &str) -> Result<&str, String> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    let rest = normalized
        .strip_prefix("---\n")
        .or_else(|| normalized.strip_prefix("---\r\n"))
        .ok_or_else(|| "report must start with YAML frontmatter".to_string())?;
    let end = rest
        .find("\n---\n")
        .or_else(|| rest.find("\r\n---\r\n"))
        .or_else(|| rest.strip_suffix("\n---").map(|prefix| prefix.len()))
        .ok_or_else(|| "report frontmatter is missing closing delimiter".to_string())?;
    Ok(&rest[..end])
}

fn validate_scope(scope: &str) -> Result<(), String> {
    if matches!(scope, "full" | "partial") {
        Ok(())
    } else {
        Err("scope must be full or partial".to_string())
    }
}

fn split_marks_archived(content: &str, child_id: &str) -> bool {
    let Some(start) = content.find(child_id) else {
        return false;
    };
    let section = &content[start..];
    let end = section
        .find("\n### ")
        .or_else(|| section.find("\n## "))
        .unwrap_or(section.len());
    section[..end].contains("- 归档状态：[x] archived")
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

fn compute_fingerprint(
    artifacts: &[GovernanceArtifactRef],
    failures: &[GovernanceEvidenceFailure],
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"specwiki-governance-evidence-v1\0");
    for artifact in artifacts {
        hasher.update(artifact.relative_path.as_bytes());
        hasher.update(b"\0");
        hasher.update(artifact.kind.as_bytes());
        hasher.update(b"\0");
        hasher.update(format!("{:?}", artifact.status).as_bytes());
        hasher.update(b"\0");
        if let Some(content_hash) = &artifact.content_hash {
            hasher.update(content_hash.as_bytes());
        }
        hasher.update(b"\n");
    }
    for failure in failures {
        hasher.update(failure.rule_id.as_bytes());
        hasher.update(b"\0");
        hasher.update(failure.relative_path.as_bytes());
        hasher.update(b"\n");
    }
    hasher.finalize().to_hex().to_string()
}

fn change_id_from_entry(entry_name: &str, location: GovernanceLocation) -> Option<String> {
    if location == GovernanceLocation::Active {
        return (!entry_name.is_empty()).then(|| entry_name.to_string());
    }
    let bytes = entry_name.as_bytes();
    if bytes.len() <= 11
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
        || !bytes[8..10].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    Some(entry_name[11..].to_string())
}

fn file_failure(
    rule_id: &str,
    change_id: &str,
    relative_path: &str,
    error: io::Error,
) -> GovernanceEvidenceFailure {
    GovernanceEvidenceFailure {
        rule_id: rule_id.to_string(),
        change_id: Some(change_id.to_string()),
        relative_path: relative_path.to_string(),
        message: error.to_string(),
    }
}

fn relative_path(repo_root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(repo_root).ok().or_else(|| {
        fs::canonicalize(repo_root)
            .ok()
            .and_then(|canonical_root| path.strip_prefix(canonical_root).ok())
    });
    relative
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
