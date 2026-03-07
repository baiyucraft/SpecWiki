use serde::Serialize;
use std::io;
use std::path::Path;

use crate::app::init::run_init;
use crate::app::status::run_status;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateReport {
    pub previous_state: String,
    pub state: String,
    pub updated_pages: Vec<String>,
}

pub fn run_update(repo_root: &Path) -> io::Result<UpdateReport> {
    let status = run_status(repo_root)?;

    if status.state == "fresh" {
        return Ok(UpdateReport {
            previous_state: status.state.clone(),
            state: status.state,
            updated_pages: Vec::new(),
        });
    }

    let init = run_init(repo_root)?;

    Ok(UpdateReport {
        previous_state: status.state,
        state: init.state,
        updated_pages: init.generated_pages,
    })
}
