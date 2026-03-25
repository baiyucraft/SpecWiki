//! `symbol_graph` 负责把 parser 产出的 raw symbol facts 继续收敛成图事实。
//! 这一层位于 `repo::symbols` 之后、workflow 编排之前，承载 resolve / analyze 的共享模型。

pub mod analyze;
pub mod models;
mod pipeline;
pub mod resolve;

pub use models::{
    CommunityMember, CommunityNode, CycleSummary, GraphAnalysisSnapshot, GraphDiagnostic,
    GraphSummary, ProcessNode, ProcessStep, ResolvedGraphSnapshot, ResolvedSymbolEdge,
    SymbolGraphSnapshot,
};
pub use pipeline::{
    analyze_symbol_graph, assemble_symbol_graph_snapshot, build_graph_summary, resolve_symbol_graph,
};

