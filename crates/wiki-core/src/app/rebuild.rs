use serde::Serialize;
use std::io;
use std::path::Path;

use crate::app::init::run_init;

#[derive(Debug, Clone, Serialize)]
pub struct RebuildReport {
    pub state: String,
    pub updated_pages: Vec<String>,
}

pub fn run_rebuild(repo_root: &Path) -> io::Result<RebuildReport> {
    let init = run_init(repo_root)?;

    Ok(RebuildReport {
        state: init.state,
        updated_pages: init.generated_pages,
    })
}
