use std::fs;
use std::io;
use std::path::Path;

use crate::domain::metadata::WikiMetadata;

pub fn read_metadata(repo_root: &Path) -> io::Result<WikiMetadata> {
    let json = fs::read_to_string(repo_root.join(".wiki/wiki.metadata.json"))?;

    serde_json::from_str(&json).map_err(|err| io::Error::other(err.to_string()))
}

pub fn write_metadata(repo_root: &Path, metadata: &WikiMetadata) -> io::Result<()> {
    let wiki_root = repo_root.join(".wiki");
    fs::create_dir_all(&wiki_root)?;

    let json = serde_json::to_string_pretty(metadata)
        .map_err(|err| io::Error::other(err.to_string()))?;

    fs::write(wiki_root.join("wiki.metadata.json"), json)
}
