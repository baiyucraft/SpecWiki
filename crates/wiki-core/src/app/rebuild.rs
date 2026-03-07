use serde::Serialize;
use std::io;
use std::path::Path;

use crate::app::init::run_init;

#[derive(Debug, Clone, Serialize)]
pub struct RebuildReport {
    pub updated_pages: Vec<String>,
}

pub fn run_rebuild(repo_root: &Path) -> io::Result<RebuildReport> {
    run_init(repo_root)?;

    Ok(RebuildReport {
        updated_pages: vec![".wiki/项目概述.md".to_string()],
    })
}
