use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::repo::detectors::detect_topics;
use crate::repo::fingerprint::fingerprint_bytes;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScannedFile {
    pub path: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScanReport {
    pub root: String,
    pub files: Vec<ScannedFile>,
    pub detected_topics: Vec<String>,
}

pub fn scan_repo(root: &Path) -> io::Result<ScanReport> {
    let mut files = Vec::new();
    visit_dir(root, root, &mut files)?;

    let paths = files.iter().map(|file| file.path.clone()).collect::<Vec<_>>();

    Ok(ScanReport {
        root: root.to_string_lossy().to_string(),
        files,
        detected_topics: detect_topics(&paths),
    })
}

fn visit_dir(root: &Path, dir: &Path, files: &mut Vec<ScannedFile>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            if should_ignore_dir(&path) {
                continue;
            }

            visit_dir(root, &path, files)?;
            continue;
        }

        let relative = make_relative(root, &path);
        let bytes = fs::read(&path)?;

        files.push(ScannedFile {
            path: relative,
            fingerprint: fingerprint_bytes(&bytes),
        });
    }

    Ok(())
}

fn should_ignore_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | "node_modules" | "target" | ".wiki")
    )
}

fn make_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
