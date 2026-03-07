use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::repo::scanner::ScanReport;

pub fn ensure_cache_dir(repo_root: &Path) -> io::Result<()> {
    fs::create_dir_all(cache_dir(repo_root))
}

pub fn cache_dir(repo_root: &Path) -> PathBuf {
    repo_root.join(".wiki").join(".cache")
}

pub fn scan_cache_path(repo_root: &Path) -> PathBuf {
    cache_dir(repo_root).join("repo-scan.json")
}

pub fn has_cache_layout(repo_root: &Path) -> bool {
    cache_dir(repo_root).exists() && scan_cache_path(repo_root).exists()
}

pub fn write_scan_cache(repo_root: &Path, report: &ScanReport) -> io::Result<()> {
    ensure_cache_dir(repo_root)?;
    let json = serde_json::to_string_pretty(report)
        .map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(scan_cache_path(repo_root), json)
}
