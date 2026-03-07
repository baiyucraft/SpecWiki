use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreCommand {
    pub action: String,
    #[serde(rename = "repoRoot")]
    pub repo_root: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreResponse {
    pub ok: bool,
    pub error: Option<String>,
}

impl CoreResponse {
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
        }
    }
}
