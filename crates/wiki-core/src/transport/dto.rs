use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreCommand {
    pub action: String,
    #[serde(rename = "repoRoot")]
    pub repo_root: Option<String>,
    pub term: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub data: Option<Value>,
}

impl CoreResponse {
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            data: None,
        }
    }

    pub fn success(data: Value) -> Self {
        Self {
            ok: true,
            error: None,
            data: Some(data),
        }
    }
}
