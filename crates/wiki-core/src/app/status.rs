use serde::Serialize;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use crate::repo::scanner::scan_repo;
use crate::storage::metadata_store::read_metadata;

#[derive(Debug, Clone, Serialize)]
pub struct StatusReport {
    pub state: String,
    pub dirty_sources: Vec<String>,
}

pub fn run_status(repo_root: &Path) -> io::Result<StatusReport> {
    let metadata = read_metadata(repo_root)?;
    let scan_report = scan_repo(repo_root)?;

    let current = scan_report
        .files
        .iter()
        .map(|file| (file.path.clone(), file.fingerprint.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut dirty_sources = Vec::new();

    for source in &metadata.source_files {
        match current.get(&source.path) {
            Some(fingerprint) if fingerprint == &source.fingerprint => {}
            _ => dirty_sources.push(source.path.clone()),
        }
    }

    let state = if dirty_sources.is_empty() {
        "fresh"
    } else {
        "stale"
    };

    Ok(StatusReport {
        state: state.to_string(),
        dirty_sources,
    })
}
