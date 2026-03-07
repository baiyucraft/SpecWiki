use serde::Serialize;
use std::io;
use std::path::Path;

use crate::app::init::run_init;
use crate::app::status::run_status;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateReport {
    pub updated_pages: Vec<String>,
}

pub fn run_update(repo_root: &Path) -> io::Result<UpdateReport> {
    let status = run_status(repo_root)?;

    if status.state == "fresh" {
        return Ok(UpdateReport {
            updated_pages: Vec::new(),
        });
    }

    run_init(repo_root)?;

    Ok(UpdateReport {
        updated_pages: vec![".wiki/项目概述.md".to_string()],
    })
}
