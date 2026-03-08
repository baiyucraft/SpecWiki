use serde::{Deserialize, Serialize};

use crate::domain::metadata::DirtyState;
use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;

/// `WikiPageState` 是内部运行时使用的页面状态模型。
/// 相比 `WikiItem`，这里更偏 update/query 所需的内部映射信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiPageState {
    pub page_id: String,
    pub path: String,
    pub content_hash: String,
    pub source_ids: Vec<String>,
    pub source_paths: Vec<String>,
    pub module_ids: Vec<String>,
}

/// `SourceState` 是内部运行时的源码状态模型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceState {
    pub source_id: String,
    pub path: String,
    pub fingerprint: String,
    pub page_ids: Vec<String>,
    pub module_ids: Vec<String>,
}

/// `BuildState` 记录一次构建完成后的整体摘要。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildState {
    pub generated_at: String,
    pub page_count: usize,
    pub module_count: usize,
}

/// `WikiState` 是内部标准化状态模型。
/// 当前还没有完整替代 `WikiMetadata`，但后续增量更新会更多依赖它。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiState {
    pub pages: Vec<WikiPageState>,
    pub sources: Vec<SourceState>,
    pub modules: Vec<ModuleNode>,
    pub relations: Vec<WikiRelation>,
    pub dirty_state: DirtyState,
    pub build_state: BuildState,
}
