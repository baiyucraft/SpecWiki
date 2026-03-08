use serde::{Deserialize, Serialize};

/// `RelationEdge` 是模块级关系，不是源码级调用图。
/// 它只保留页面展示和 query 需要的最小结构。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub evidence: Vec<String>,
}

/// `ModuleNode` 是层级化仓库理解的核心节点。
/// 它连接了扫描事实、页面规划、query 和 metadata 四条链路。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleNode {
    /// 稳定模块 ID，后续所有页面与关系都通过它引用模块。
    pub id: String,
    /// 对外展示名称，默认来自根路径最后一段。
    pub name: String,
    /// 模块角色，例如 frontend-app / backend-service / infrastructure。
    pub kind: String,
    /// 一个模块可能对应一个或多个根路径，当前实现先以单根路径为主。
    pub root_paths: Vec<String>,
    /// 模块内所有源码文件的稳定 ID。
    pub source_ids: Vec<String>,
    /// 父模块 ID；根仓库模块没有父级。
    pub parent_id: Option<String>,
    /// 子模块 ID；当前主要用于渲染模块树和页面父子关系。
    pub child_ids: Vec<String>,
    /// 模块入口文件，帮助生成概述和架构说明。
    pub entry_points: Vec<String>,
    /// 模块标签，来自该模块内部文件，而不是整个仓库的全局标签。
    pub tags: Vec<String>,
}

/// `ModuleTree` 是 decomposition 阶段的正式输出。
/// 后续 RepoContext、PagePlan 和 metadata 都依赖这棵树继续加工。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleTree {
    pub root_modules: Vec<String>,
    pub modules: Vec<ModuleNode>,
    pub cross_module_edges: Vec<RelationEdge>,
    pub architecture_hints: Vec<String>,
}

impl ModuleTree {
    /// 通过稳定 ID 查模块，便于其他层避免重复维护索引表。
    pub fn module_by_id(&self, module_id: &str) -> Option<&ModuleNode> {
        self.modules.iter().find(|module| module.id == module_id)
    }

    /// 返回所有非根模块。
    /// 页面规划阶段主要围绕这些真正的业务模块生成模块页。
    pub fn non_root_modules(&self) -> Vec<&ModuleNode> {
        self.modules
            .iter()
            .filter(|module| module.parent_id.is_some())
            .collect()
    }
}
