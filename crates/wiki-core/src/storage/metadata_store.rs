use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::domain::metadata::WikiMetadata;

pub fn read_metadata(repo_root: &Path) -> io::Result<WikiMetadata> {
    let json = fs::read_to_string(metadata_path(repo_root))?;

    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

pub fn write_metadata(repo_root: &Path, metadata: &WikiMetadata) -> io::Result<()> {
    let wiki_root = repo_root.join(".wiki");
    fs::create_dir_all(&wiki_root)?;

    let json = serde_json::to_string_pretty(metadata)
        .map_err(|err| io::Error::other(err.to_string()))?;

    fs::write(metadata_path(repo_root), json)
}

pub fn metadata_path(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join("wiki.metadata.json")
}

pub fn metadata_exists(repo_root: &Path) -> bool {
    metadata_path(repo_root).exists()
}
