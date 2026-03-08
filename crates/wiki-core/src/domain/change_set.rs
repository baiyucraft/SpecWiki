use serde::{Deserialize, Serialize};

/// `ChangeSet` 用来描述一次比较后得到的变化集合。
/// 当前使用还比较轻，后续增量更新会更依赖它。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeSet {
    pub dirty_sources: Vec<String>,
    pub dirty_pages: Vec<String>,
}
