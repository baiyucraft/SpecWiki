use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `CoreCommand` 是 Agent -> core 的最小命令协议。
/// 当前只保留 action / repoRoot / term 三个字段，避免接入层过早复杂化。
#[derive(Debug, Deserialize, Serialize)]
pub struct CoreCommand {
    pub action: String,
    #[serde(rename = "repoRoot")]
    pub repo_root: Option<String>,
    pub term: Option<String>,
}

/// `CoreResponse` 是 core -> Agent 的统一响应协议。
#[derive(Debug, Deserialize, Serialize)]
pub struct CoreResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub data: Option<Value>,
}

impl CoreResponse {
    /// 构造失败响应。
    ///
    /// # 参数
    /// - `error`：要写入协议层的错误文本。
    ///
    /// # 返回
    /// - 返回 `ok = false` 的统一响应对象。
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: Some(error.into()),
            data: None,
        }
    }

    /// 构造成功响应。
    ///
    /// # 参数
    /// - `data`：要写入协议层的 JSON 数据。
    ///
    /// # 返回
    /// - 返回 `ok = true` 的统一响应对象。
    pub fn success(data: Value) -> Self {
        Self {
            ok: true,
            error: None,
            data: Some(data),
        }
    }
}
