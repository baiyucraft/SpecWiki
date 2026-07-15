use serde::Serialize;

use crate::domain::runtime_profile::{
    AnswerEnvelope, QueryMode, QueryTrust, RecommendedAction, RuntimeReadiness,
};
use crate::workflows::query::QueryReport;
use wiki_model::domain::governance::GovernanceSummary;
use wiki_model::domain::query::QueryRouteGroup;

/// Query transport 的 canonical 对外合同。
///
/// Workflow 可以保留用于装配和诊断的 rich report，但 transport 只发布 route groups、
/// answer 和运行时状态；消费者不得依赖平铺结果或派生摘要。
#[derive(Debug, Clone, Serialize)]
pub struct ExternalQueryReport {
    pub term: String,
    pub runtime_state: String,
    pub readiness: RuntimeReadiness,
    pub query_mode: QueryMode,
    pub query_trust: QueryTrust,
    pub recommended_action: RecommendedAction,
    pub governance: GovernanceSummary,
    pub route_groups: Vec<QueryRouteGroup>,
    pub answer: AnswerEnvelope,
}

pub fn map_query_report(report: QueryReport) -> ExternalQueryReport {
    ExternalQueryReport {
        term: report.term,
        runtime_state: report.runtime_state,
        readiness: report.readiness,
        query_mode: report.query_mode,
        query_trust: report.query_trust,
        recommended_action: report.recommended_action,
        governance: report.governance,
        route_groups: report.route_groups,
        answer: report.answer,
    }
}
