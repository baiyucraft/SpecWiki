use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::symbols::ParsedSymbolsSnapshot;

/// `ResolvedSymbolEdge` 是 raw capture resolve 后的稳定图边。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct ResolvedSymbolEdge {
    /// 稳定 edge ID，供 SQLite、query 和 graph analysis 复用。
    pub edge_id: String,
    /// 边起点，对应 `symbols.id`。
    pub source_id: String,
    /// 边终点，对应 `symbols.id`。
    pub target_id: String,
    /// 关系类别，例如 `IMPORTS`、`CALLS`、`EXTENDS`、`IMPLEMENTS`。
    pub edge_type: String,
    /// 解析置信度，供 query 和 graph analysis 做降噪。
    pub confidence: f64,
    /// 人类可读的解析原因或 provenance 摘要。
    pub reason: String,
}

/// community 主记录，对应 `communities` 表。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct CommunityNode {
    /// 稳定 community ID。
    pub community_id: String,
    /// 面向 query/planner 的简洁标签。
    pub label: String,
    /// 聚类内部的粗略凝聚度。
    pub cohesion: f64,
    /// 成员符号数量。
    pub symbol_count: usize,
}

/// community 成员映射，对应 `community_members` 表。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommunityMember {
    /// 所属 community。
    pub community_id: String,
    /// 成员符号。
    pub symbol_id: String,
}

/// process 主记录，对应 `processes` 表。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProcessNode {
    /// 稳定 process ID。
    pub process_id: String,
    /// 面向页面和 query 的展示标签。
    pub label: String,
    /// 流程类别，例如 `request-flow`、`batch-job`。
    pub process_type: String,
    /// 步骤数量缓存，避免 query 再次聚合。
    pub step_count: usize,
    /// 推断得到的入口符号。
    pub entry_point_id: Option<String>,
    /// 推断得到的终点符号。
    pub terminal_id: Option<String>,
}

/// process 步骤记录，对应 `process_steps` 表。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProcessStep {
    /// 所属 process。
    pub process_id: String,
    /// 当前步骤对应的符号。
    pub symbol_id: String,
    /// 稳定步骤顺序。
    pub step_order: usize,
}

/// cycle/topology 派生结果，当前保留在内存快照中供 planner/query 消费。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CycleSummary {
    /// 稳定 cycle ID。
    pub cycle_id: String,
    /// 同一强连通分量内的符号集合。
    pub symbol_ids: Vec<String>,
    /// 若做了断边建议，这里记录 edge IDs。
    pub break_edge_ids: Vec<String>,
    /// 可选的拓扑提示。
    pub topo_order: Vec<String>,
    /// 面向用户的简洁 warning。
    pub warning: String,
}

/// graph 阶段的 fail-soft 诊断。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GraphDiagnostic {
    /// 诊断发生的阶段，例如 `resolve_imports`、`detect_processes`。
    pub stage: String,
    /// 诊断类别，供测试和 query 判断。
    pub kind: String,
    /// 可读错误或降级说明。
    pub message: String,
    /// 若问题定位到具体文件，保留文件路径。
    pub file_path: Option<String>,
    /// 若问题定位到具体符号，保留稳定 symbol ID。
    pub symbol_id: Option<String>,
}

/// resolve 阶段输出的稳定图边快照。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct ResolvedGraphSnapshot {
    /// 本轮 resolve 成功产出的稳定边。
    pub edges: Vec<ResolvedSymbolEdge>,
    /// resolve 阶段诊断。
    pub diagnostics: Vec<GraphDiagnostic>,
}

/// analyze 阶段输出的 graph-derived 视图。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct GraphAnalysisSnapshot {
    /// community 主记录。
    pub communities: Vec<CommunityNode>,
    /// community 成员映射。
    pub community_members: Vec<CommunityMember>,
    /// process 主记录。
    pub processes: Vec<ProcessNode>,
    /// process 步骤映射。
    pub process_steps: Vec<ProcessStep>,
    /// cycle / topo 派生结果。
    pub cycles: Vec<CycleSummary>,
    /// analyze 阶段诊断。
    pub diagnostics: Vec<GraphDiagnostic>,
}

/// 面向 hierarchy/context/planner 的聚合摘要。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GraphSummary {
    /// 模块级依赖提示，键为模块根路径。
    pub module_dependency_hints: BTreeMap<String, Vec<String>>,
    /// 模块内高热调用点，键为模块根路径。
    pub module_call_hotspots: BTreeMap<String, Vec<String>>,
    /// 每个模块关联到的 community 标签，键为模块根路径。
    pub communities_by_module: BTreeMap<String, Vec<String>>,
    /// 已检测到的 process labels。
    pub detected_processes: Vec<String>,
    /// cycle/topology 的 warning 文本。
    pub cycle_warnings: Vec<String>,
}

/// 跨 parser / resolve / analyze 的统一 graph 快照。
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct SymbolGraphSnapshot {
    /// parser 阶段产出的 definitions 与 raw captures。
    pub parsed_symbols: ParsedSymbolsSnapshot,
    /// resolve 阶段产出的稳定 symbol graph。
    pub resolved_graph: ResolvedGraphSnapshot,
    /// analyze 阶段的 graph-derived 结果。
    pub analysis: GraphAnalysisSnapshot,
    /// 面向后续模块消费的聚合摘要。
    pub summary: GraphSummary,
}
