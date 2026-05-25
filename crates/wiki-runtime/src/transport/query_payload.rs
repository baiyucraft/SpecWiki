use std::collections::BTreeSet;

use serde::Serialize;

use crate::domain::runtime_profile::{
    AnswerEnvelope, QueryMode, QueryTrust, RecommendedAction,
};
use crate::workflows::query::QueryReport;

const PAGE_HIT_LIMIT: usize = 4;
const SYMBOL_HIT_LIMIT: usize = 4;
const SOURCE_HIT_LIMIT: usize = 2;
const MODULE_HIT_LIMIT: usize = 2;
const EDGE_HIT_LIMIT: usize = 2;
const TOTAL_HIT_LIMIT: usize = 10;
const FOCUS_LIMIT: usize = 6;

#[derive(Debug, Clone, Serialize)]
pub struct ExternalQueryReport {
    pub term: String,
    pub runtime_state: String,
    pub query_mode: QueryMode,
    pub query_trust: QueryTrust,
    pub recommended_action: RecommendedAction,
    pub matched_pages: Vec<String>,
    pub provenance_summary: String,
    pub answer: AnswerEnvelope,
    pub summary: ExternalQuerySummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<ExternalQueryHit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalQuerySummary {
    pub text: String,
    pub counts: ExternalQueryCounts,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalQueryCounts {
    pub pages: usize,
    pub modules: usize,
    pub sources: usize,
    pub symbols: usize,
    pub edges: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalQueryHit {
    pub hit_type: String,
    pub title: String,
    pub location: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_end: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<String>,
}

pub fn map_query_report(report: QueryReport) -> ExternalQueryReport {
    let summary = build_summary(&report);
    let hits = build_hits(&report);

    ExternalQueryReport {
        term: report.term,
        runtime_state: report.runtime_state,
        query_mode: report.query_mode,
        query_trust: report.query_trust,
        recommended_action: report.recommended_action,
        matched_pages: report.matched_pages,
        provenance_summary: report.provenance_summary,
        answer: report.answer,
        summary,
        hits,
    }
}

fn build_summary(report: &QueryReport) -> ExternalQuerySummary {
    let counts = ExternalQueryCounts {
        pages: report.matches.len(),
        modules: report.matched_modules.len(),
        sources: report.matched_sources.len(),
        symbols: report.matched_symbols.len(),
        edges: report.matched_symbol_edges.len(),
    };
    let focus = collect_focus(report);
    let text = build_summary_text(&counts, &focus);

    ExternalQuerySummary {
        text,
        counts,
        focus,
    }
}

fn collect_focus(report: &QueryReport) -> Vec<String> {
    let mut focus = BTreeSet::new();

    for path in report.matches.iter().map(|page| page.path.as_str()) {
        if focus.len() >= FOCUS_LIMIT {
            break;
        }
        focus.insert(path.to_string());
    }

    if focus.len() < FOCUS_LIMIT {
        for path in report
            .matched_symbols
            .iter()
            .map(|symbol| symbol.file_path.as_str())
        {
            if focus.len() >= FOCUS_LIMIT {
                break;
            }
            focus.insert(path.to_string());
        }
    }

    if focus.len() < FOCUS_LIMIT {
        for path in report
            .matched_sources
            .iter()
            .map(|source| source.path.as_str())
        {
            if focus.len() >= FOCUS_LIMIT {
                break;
            }
            focus.insert(path.to_string());
        }
    }

    if focus.len() < FOCUS_LIMIT {
        for root in report
            .matched_modules
            .iter()
            .flat_map(|module| module.root_paths.iter())
        {
            if focus.len() >= FOCUS_LIMIT {
                break;
            }
            focus.insert(root.clone());
        }
    }

    focus.into_iter().collect()
}

fn build_summary_text(counts: &ExternalQueryCounts, focus: &[String]) -> String {
    let mut parts = Vec::new();
    if counts.pages > 0 {
        parts.push(format!("命中 {} 个页面", counts.pages));
    }
    if counts.symbols > 0 {
        parts.push(format!("命中 {} 个符号", counts.symbols));
    }
    if counts.sources > 0 {
        parts.push(format!("命中 {} 个源码文件", counts.sources));
    }
    if counts.modules > 0 {
        parts.push(format!("命中 {} 个模块", counts.modules));
    }
    if counts.edges > 0 {
        parts.push(format!("扩展 {} 条调用边", counts.edges));
    }

    let focus_text = if focus.is_empty() {
        String::new()
    } else {
        format!("优先看 {}", focus.join("、"))
    };

    if parts.is_empty() {
        if focus_text.is_empty() {
            "没有命中页面、符号、源码或图上下文。".to_string()
        } else {
            format!("没有结构化命中。{}。", focus_text)
        }
    } else if focus_text.is_empty() {
        format!("{}。", parts.join("、"))
    } else {
        format!("{}。{}。", parts.join("、"), focus_text)
    }
}

fn build_hits(report: &QueryReport) -> Vec<ExternalQueryHit> {
    let mut hits = Vec::new();
    let mut seen = BTreeSet::new();

    for page in report.matches.iter().take(PAGE_HIT_LIMIT) {
        push_hit(
            &mut hits,
            &mut seen,
            ExternalQueryHit {
                hit_type: "page".to_string(),
                title: page.title.clone(),
                location: page.path.clone(),
                summary: page.summary.clone(),
                line_start: None,
                line_end: None,
                reasons: page.reasons.clone(),
                provenance: page.provenance.clone(),
            },
        );
    }

    for symbol in report.matched_symbols.iter().take(SYMBOL_HIT_LIMIT) {
        push_hit(
            &mut hits,
            &mut seen,
            ExternalQueryHit {
                hit_type: "symbol".to_string(),
                title: symbol.name.clone(),
                location: symbol_location(symbol),
                summary: format!("{} · {}", symbol.label, symbol.language),
                line_start: (symbol.start_line > 0).then_some(symbol.start_line),
                line_end: (symbol.end_line > 0).then_some(symbol.end_line),
                reasons: symbol.reasons.clone(),
                provenance: Vec::new(),
            },
        );
    }

    for source in report.matched_sources.iter().take(SOURCE_HIT_LIMIT) {
        push_hit(
            &mut hits,
            &mut seen,
            ExternalQueryHit {
                hit_type: "source".to_string(),
                title: source
                    .path
                    .rsplit('/')
                    .next()
                    .unwrap_or(source.path.as_str())
                    .to_string(),
                location: source.path.clone(),
                summary: "源码命中".to_string(),
                line_start: None,
                line_end: None,
                reasons: source.reasons.clone(),
                provenance: Vec::new(),
            },
        );
    }

    for module in report.matched_modules.iter().take(MODULE_HIT_LIMIT) {
        let location = module
            .root_paths
            .first()
            .cloned()
            .unwrap_or_else(|| module.name.clone());
        push_hit(
            &mut hits,
            &mut seen,
            ExternalQueryHit {
                hit_type: "module".to_string(),
                title: module.name.clone(),
                location,
                summary: format!("{} 模块", module.kind),
                line_start: None,
                line_end: None,
                reasons: module.reasons.clone(),
                provenance: Vec::new(),
            },
        );
    }

    for edge in report.matched_symbol_edges.iter().take(EDGE_HIT_LIMIT) {
        let traversal = if edge.traversal_modes.is_empty() {
            edge.edge_type.clone()
        } else {
            format!("{} · {}", edge.edge_type, edge.traversal_modes.join("/"))
        };
        push_hit(
            &mut hits,
            &mut seen,
            ExternalQueryHit {
                hit_type: "call_edge".to_string(),
                title: format!("{} -> {}", edge.source_symbol, edge.target_symbol),
                location: traversal,
                summary: edge.reason.clone(),
                line_start: None,
                line_end: None,
                reasons: edge.reasons.clone(),
                provenance: edge.provenance.clone(),
            },
        );
    }

    hits.truncate(TOTAL_HIT_LIMIT);
    hits
}

fn push_hit(hits: &mut Vec<ExternalQueryHit>, seen: &mut BTreeSet<String>, hit: ExternalQueryHit) {
    if hits.len() >= TOTAL_HIT_LIMIT {
        return;
    }
    let key = format!("{}|{}|{}", hit.hit_type, hit.title, hit.location);
    if seen.insert(key) {
        hits.push(hit);
    }
}

fn symbol_location(symbol: &crate::workflows::query::QuerySymbolMatch) -> String {
    if symbol.start_line > 0 {
        format!("{}:{}", symbol.file_path, symbol.start_line)
    } else {
        symbol.file_path.clone()
    }
}
