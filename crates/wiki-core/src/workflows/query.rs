use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;

use crate::domain::module_tree::ModuleNode;
use crate::domain::relation::WikiRelation;
use crate::domain::state::{SourceState, WikiPageState, WikiState};
use crate::storage::state_store::load_or_rebuild_state;
use crate::storage::wiki_fs::resolve_page_path;

/// `QueryMatch` 描述一个命中的页面，以及它为什么命中。
#[derive(Debug, Clone, Serialize)]
pub struct QueryMatch {
    pub page_id: String,
    pub title: String,
    pub path: String,
    pub item_type: String,
    pub module_ids: Vec<String>,
    pub source_files: Vec<String>,
    pub reasons: Vec<String>,
    pub provenance: Vec<String>,
    pub summary: String,
    pub match_mode: String,
    #[serde(default)]
    pub context_pack: QueryContextPack,
}

/// `QueryContextPack` 让 Agent 不需要二次查询就能获得页面的关联上下文。
#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryContextPack {
    pub module_summaries: Vec<String>,
    pub key_source_paths: Vec<String>,
    pub relation_evidence: Vec<String>,
}

/// `QueryModuleMatch` 是 query 返回的模块视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryModuleMatch {
    pub module_id: String,
    pub name: String,
    pub kind: String,
    pub root_paths: Vec<String>,
    pub tags: Vec<String>,
    pub reasons: Vec<String>,
}

/// `QuerySourceMatch` 是 query 返回的源码视图。
#[derive(Debug, Clone, Serialize)]
pub struct QuerySourceMatch {
    pub source_id: String,
    pub path: String,
    pub module_ids: Vec<String>,
    pub reasons: Vec<String>,
}

/// `QueryRelationMatch` 是 query 返回的关系视图。
#[derive(Debug, Clone, Serialize)]
pub struct QueryRelationMatch {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: String,
    pub evidence: Vec<String>,
    pub reasons: Vec<String>,
}

/// `QueryReport` 是当前对 Agent 最友好的结构化返回。
#[derive(Debug, Clone, Serialize)]
pub struct QueryReport {
    pub term: String,
    pub matched_pages: Vec<String>,
    pub matched_modules: Vec<QueryModuleMatch>,
    pub matched_sources: Vec<QuerySourceMatch>,
    pub matched_relations: Vec<QueryRelationMatch>,
    pub matches: Vec<QueryMatch>,
    #[serde(default)]
    pub provenance_summary: String,
}

#[derive(Default)]
struct PageMatchState {
    reasons: BTreeSet<String>,
    provenance: BTreeSet<String>,
    match_mode: String,
}

#[derive(Default)]
struct RelatedMatchState {
    reasons: BTreeSet<String>,
}

/// 执行关键词查询。
/// 优先从 WikiState 构建查询索引，WikiState 丢失时从 metadata 重建。
/// 结构化索引是主命中来源，仅在没有结构命中时回退到 Markdown 内容匹配。
pub fn run_query(repo_root: &Path, term: &str) -> io::Result<QueryReport> {
    let state = load_or_rebuild_state(repo_root)?;
    let needle = term.trim().to_lowercase();

    if needle.is_empty() {
        return Ok(QueryReport {
            term: term.to_string(),
            matched_pages: Vec::new(),
            matched_modules: Vec::new(),
            matched_sources: Vec::new(),
            matched_relations: Vec::new(),
            matches: Vec::new(),
            provenance_summary: String::new(),
        });
    }

    let page_index = state
        .pages
        .iter()
        .map(|page| (page.page_id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let module_index = state
        .modules
        .iter()
        .map(|module| (module.id.clone(), module))
        .collect::<BTreeMap<_, _>>();
    let source_index = state
        .sources
        .iter()
        .map(|source| (source.source_id.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let source_path_index = state
        .sources
        .iter()
        .map(|source| (source.path.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let page_ids_by_module = build_page_ids_by_module(&state);
    let source_ids_by_module = build_source_ids_by_module(&state);

    let mut page_matches = BTreeMap::new();
    let mut module_matches = BTreeMap::new();
    let mut source_matches = BTreeMap::new();
    let mut relation_matches = BTreeMap::new();

    collect_page_matches(&state, &needle, &mut page_matches);
    collect_module_matches(
        &state,
        &needle,
        &page_ids_by_module,
        &source_ids_by_module,
        &page_index,
        &source_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
    );
    collect_source_matches(
        &state,
        &needle,
        &page_index,
        &module_index,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
    );
    collect_relation_matches(
        &state,
        &needle,
        &page_index,
        &module_index,
        &source_index,
        &source_path_index,
        &page_ids_by_module,
        &mut page_matches,
        &mut module_matches,
        &mut source_matches,
        &mut relation_matches,
    );

    // 页面命中建立后，再补齐与这些页面相关联的模块和源码，保证返回结果自洽。
    let matched_page_ids = page_matches.keys().cloned().collect::<Vec<_>>();
    for page_id in &matched_page_ids {
        if let Some(page) = page_index.get(page_id) {
            for module_id in &page.module_ids {
                record_related_match(&mut module_matches, module_id, ["关联页面匹配"]);
            }

            for source_path in &page.source_paths {
                if let Some(source) = source_path_index.get(source_path) {
                    record_related_match(&mut source_matches, &source.source_id, ["关联页面匹配"]);
                }
            }
        }
    }

    let had_structural_match = !page_matches.is_empty()
        || !module_matches.is_empty()
        || !source_matches.is_empty()
        || !relation_matches.is_empty();

    if !had_structural_match {
        collect_markdown_fallback_matches(
            repo_root,
            &state,
            &needle,
            &source_path_index,
            &mut page_matches,
            &mut module_matches,
            &mut source_matches,
        );
    }

    let provenance_summary = build_provenance_summary(&page_matches, &module_matches, &source_matches, &relation_matches);

    Ok(QueryReport {
        term: term.to_string(),
        matched_pages: finalize_page_paths(&page_matches, &page_index),
        matched_modules: finalize_module_matches(&module_matches, &module_index),
        matched_sources: finalize_source_matches(&source_matches, &source_index),
        matched_relations: finalize_relation_matches(&relation_matches, &state.relations),
        matches: finalize_page_matches(&page_matches, &page_index, &state, &module_index),
        provenance_summary,
    })
}

fn collect_page_matches(
    state: &WikiState,
    needle: &str,
    page_matches: &mut BTreeMap<String, PageMatchState>,
) {
    for page in &state.pages {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&page.title, needle) {
            reasons.push("页面标题匹配");
        }

        if contains_case_insensitive(&page.path, needle) {
            reasons.push("页面路径匹配");
        }

        if contains_case_insensitive(&page.page_type, needle) {
            reasons.push("页面类型匹配");
        }

        if contains_case_insensitive(&page.summary, needle) {
            reasons.push("页面摘要匹配");
        }

        if !reasons.is_empty() {
            record_page_match(
                page_matches,
                &page.page_id,
                reasons,
                page.provenance.clone(),
                "structure",
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_module_matches(
    state: &WikiState,
    needle: &str,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    source_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_index: &BTreeMap<String, &WikiPageState>,
    source_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for module in &state.modules {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&module.name, needle) {
            reasons.push("模块名称匹配");
        }

        if contains_case_insensitive(&module.kind, needle) {
            reasons.push("模块类型匹配");
        }

        if module
            .root_paths
            .iter()
            .any(|path| contains_case_insensitive(path, needle))
        {
            reasons.push("模块路径匹配");
        }

        if module
            .tags
            .iter()
            .any(|tag| contains_case_insensitive(tag, needle))
        {
            reasons.push("模块标签匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(module_matches, &module.id, reasons.iter().copied());

        if let Some(page_ids) = page_ids_by_module.get(&module.id) {
            for page_id in page_ids {
                if page_index.contains_key(page_id) {
                    record_page_match(
                        page_matches,
                        page_id,
                        ["关联模块匹配"],
                        [format!("module:{}", module.id)],
                        "structure",
                    );
                }
            }
        }

        if let Some(source_ids) = source_ids_by_module.get(&module.id) {
            for source_id in source_ids {
                if source_index.contains_key(source_id) {
                    record_related_match(source_matches, source_id, ["关联模块匹配"]);
                }
            }
        }
    }
}

fn collect_source_matches(
    state: &WikiState,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for source in &state.sources {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&source.path, needle) {
            reasons.push("源码路径匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(source_matches, &source.source_id, reasons.iter().copied());

        for page_id in &source.page_ids {
            if page_index.contains_key(page_id) {
                record_page_match(
                    page_matches,
                    page_id,
                    ["关联源码匹配"],
                    [format!("source:{}", source.path)],
                    "structure",
                );
            }
        }

        for module_id in &source.module_ids {
            if module_index.contains_key(module_id) {
                record_related_match(module_matches, module_id, ["关联源码匹配"]);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_relation_matches(
    state: &WikiState,
    needle: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_index: &BTreeMap<String, &SourceState>,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
    relation_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for relation in &state.relations {
        let mut reasons = Vec::new();

        if contains_case_insensitive(&relation.relation_type, needle) {
            reasons.push("关系类型匹配");
        }

        if relation
            .evidence
            .iter()
            .any(|evidence| contains_case_insensitive(evidence, needle))
        {
            reasons.push("关系证据匹配");
        }

        let source_label = relation_endpoint_label(
            &relation.source_id,
            page_index,
            module_index,
            source_index,
        );
        let target_label = relation_endpoint_label(
            &relation.target_id,
            page_index,
            module_index,
            source_index,
        );

        if contains_case_insensitive(&source_label, needle)
            || contains_case_insensitive(&target_label, needle)
        {
            reasons.push("关系端点匹配");
        }

        if reasons.is_empty() {
            continue;
        }

        record_related_match(
            relation_matches,
            &relation_identity(relation),
            reasons.iter().copied(),
        );
        record_relation_endpoint_match(
            relation,
            &relation.source_id,
            page_index,
            module_index,
            page_ids_by_module,
            page_matches,
            module_matches,
            source_matches,
        );
        record_relation_endpoint_match(
            relation,
            &relation.target_id,
            page_index,
            module_index,
            page_ids_by_module,
            page_matches,
            module_matches,
            source_matches,
        );

        for evidence_path in &relation.evidence {
            if let Some(source) = source_path_index.get(evidence_path) {
                record_related_match(source_matches, &source.source_id, ["关系证据匹配"]);

                for page_id in &source.page_ids {
                    if page_index.contains_key(page_id) {
                        record_page_match(
                            page_matches,
                            page_id,
                            ["关系证据匹配"],
                            [
                                format!("relation:{}", relation.relation_type),
                                format!("source:{}", source.path),
                            ],
                            "structure",
                        );
                    }
                }
            }
        }
    }
}

fn collect_markdown_fallback_matches(
    repo_root: &Path,
    state: &WikiState,
    needle: &str,
    source_path_index: &BTreeMap<String, &SourceState>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    for page in &state.pages {
        let page_path = resolve_page_path(repo_root, &page.path);
        let content = fs::read_to_string(&page_path).unwrap_or_default();
        if !contains_case_insensitive(&content, needle) {
            continue;
        }

        record_page_match(
            page_matches,
            &page.page_id,
            ["Markdown 内容匹配"],
            page.provenance.clone(),
            "fallback_markdown",
        );

        for module_id in &page.module_ids {
            record_related_match(module_matches, module_id, ["Markdown 回退关联"]);
        }

        for source_path in &page.source_paths {
            if let Some(source) = source_path_index.get(source_path) {
                record_related_match(source_matches, &source.source_id, ["Markdown 回退关联"]);
            }
        }
    }
}

fn record_relation_endpoint_match(
    relation: &WikiRelation,
    endpoint_id: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    page_ids_by_module: &BTreeMap<String, Vec<String>>,
    page_matches: &mut BTreeMap<String, PageMatchState>,
    module_matches: &mut BTreeMap<String, RelatedMatchState>,
    source_matches: &mut BTreeMap<String, RelatedMatchState>,
) {
    if page_index.contains_key(endpoint_id) {
        record_page_match(
            page_matches,
            endpoint_id,
            ["关系端点匹配"],
            [format!("relation:{}", relation.relation_type)],
            "structure",
        );
        return;
    }

    if module_index.contains_key(endpoint_id) {
        record_related_match(module_matches, endpoint_id, ["关系端点匹配"]);

        if let Some(page_ids) = page_ids_by_module.get(endpoint_id) {
            for page_id in page_ids {
                if page_index.contains_key(page_id) {
                    record_page_match(
                        page_matches,
                        page_id,
                        ["关系端点匹配"],
                        [format!("relation:{}", relation.relation_type)],
                        "structure",
                    );
                }
            }
        }
        return;
    }

    record_related_match(source_matches, endpoint_id, ["关系端点匹配"]);
}

fn relation_endpoint_label(
    endpoint_id: &str,
    page_index: &BTreeMap<String, &WikiPageState>,
    module_index: &BTreeMap<String, &ModuleNode>,
    source_index: &BTreeMap<String, &SourceState>,
) -> String {
    if let Some(page) = page_index.get(endpoint_id) {
        return format!("{} {}", page.title, page.path);
    }

    if let Some(module) = module_index.get(endpoint_id) {
        return format!("{} {}", module.name, module.root_paths.join(" "));
    }

    if let Some(source) = source_index.get(endpoint_id) {
        return source.path.clone();
    }

    endpoint_id.to_string()
}

fn record_page_match<I, P>(
    page_matches: &mut BTreeMap<String, PageMatchState>,
    page_id: &str,
    reasons: I,
    provenance: P,
    match_mode: &str,
) where
    I: IntoIterator,
    I::Item: AsRef<str>,
    P: IntoIterator,
    P::Item: Into<String>,
{
    let state = page_matches.entry(page_id.to_string()).or_default();

    for reason in reasons {
        state.reasons.insert(reason.as_ref().to_string());
    }

    for provenance_item in provenance {
        state.provenance.insert(provenance_item.into());
    }

    if state.match_mode != "structure" {
        state.match_mode = match_mode.to_string();
    }
}

fn record_related_match<I>(
    matches: &mut BTreeMap<String, RelatedMatchState>,
    id: &str,
    reasons: I,
) where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let state = matches.entry(id.to_string()).or_default();

    for reason in reasons {
        state.reasons.insert(reason.as_ref().to_string());
    }
}

fn build_page_ids_by_module(state: &WikiState) -> BTreeMap<String, Vec<String>> {
    let mut page_ids_by_module = BTreeMap::new();

    for page in &state.pages {
        for module_id in &page.module_ids {
            page_ids_by_module
                .entry(module_id.clone())
                .or_insert_with(Vec::new)
                .push(page.page_id.clone());
        }
    }

    page_ids_by_module
}

fn build_source_ids_by_module(state: &WikiState) -> BTreeMap<String, Vec<String>> {
    let mut source_ids_by_module = BTreeMap::new();

    for source in &state.sources {
        for module_id in &source.module_ids {
            source_ids_by_module
                .entry(module_id.clone())
                .or_insert_with(Vec::new)
                .push(source.source_id.clone());
        }
    }

    source_ids_by_module
}

fn build_provenance_summary(
    page_matches: &BTreeMap<String, PageMatchState>,
    module_matches: &BTreeMap<String, RelatedMatchState>,
    source_matches: &BTreeMap<String, RelatedMatchState>,
    relation_matches: &BTreeMap<String, RelatedMatchState>,
) -> String {
    let mut parts = Vec::new();

    let structural_pages = page_matches
        .values()
        .filter(|s| s.match_mode == "structure")
        .count();
    let fallback_pages = page_matches
        .values()
        .filter(|s| s.match_mode == "fallback_markdown")
        .count();

    if structural_pages > 0 {
        parts.push(format!("结构命中 {} 页", structural_pages));
    }
    if fallback_pages > 0 {
        parts.push(format!("Markdown 回退命中 {} 页", fallback_pages));
    }
    if !module_matches.is_empty() {
        parts.push(format!("关联 {} 模块", module_matches.len()));
    }
    if !source_matches.is_empty() {
        parts.push(format!("关联 {} 源码", source_matches.len()));
    }
    if !relation_matches.is_empty() {
        parts.push(format!("关联 {} 关系", relation_matches.len()));
    }

    parts.join("、")
}

fn build_context_pack(
    page: &WikiPageState,
    state: &WikiState,
    module_index: &BTreeMap<String, &ModuleNode>,
) -> QueryContextPack {
    let module_summaries = page
        .module_ids
        .iter()
        .filter_map(|mid| module_index.get(mid))
        .map(|m| {
            let tags = if m.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", m.tags.join(", "))
            };
            format!("{} ({}){}", m.name, m.kind, tags)
        })
        .collect();

    let key_source_paths = page.source_paths.iter().take(8).cloned().collect();

    let relation_evidence = state
        .relations
        .iter()
        .filter(|r| {
            page.module_ids.contains(&r.source_id) || page.module_ids.contains(&r.target_id)
        })
        .flat_map(|r| {
            std::iter::once(format!(
                "{} -[{}]-> {}",
                r.source_id, r.relation_type, r.target_id
            ))
        })
        .take(10)
        .collect();

    QueryContextPack {
        module_summaries,
        key_source_paths,
        relation_evidence,
    }
}

fn finalize_page_paths(
    page_matches: &BTreeMap<String, PageMatchState>,
    page_index: &BTreeMap<String, &WikiPageState>,
) -> Vec<String> {
    let mut pages = page_matches
        .keys()
        .filter_map(|page_id| page_index.get(page_id).map(|page| page.path.clone()))
        .collect::<Vec<_>>();
    pages.sort();
    pages
}

fn finalize_page_matches(
    page_matches: &BTreeMap<String, PageMatchState>,
    page_index: &BTreeMap<String, &WikiPageState>,
    state: &WikiState,
    module_index: &BTreeMap<String, &ModuleNode>,
) -> Vec<QueryMatch> {
    let mut matches = page_matches
        .iter()
        .filter_map(|(page_id, match_state)| {
            let page = page_index.get(page_id)?;
            let reasons = match_state.reasons.iter().cloned().collect::<Vec<_>>();
            Some(QueryMatch {
                page_id: page.page_id.clone(),
                title: page.title.clone(),
                path: page.path.clone(),
                item_type: page.page_type.clone(),
                module_ids: page.module_ids.clone(),
                source_files: page.source_paths.clone(),
                reasons: reasons.clone(),
                provenance: match_state.provenance.iter().cloned().collect(),
                summary: reasons.join("、"),
                match_mode: match_state.match_mode.clone(),
                context_pack: build_context_pack(page, state, module_index),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.page_id.cmp(&right.page_id))
    });
    matches
}

fn finalize_module_matches(
    module_matches: &BTreeMap<String, RelatedMatchState>,
    module_index: &BTreeMap<String, &ModuleNode>,
) -> Vec<QueryModuleMatch> {
    let mut matches = module_matches
        .iter()
        .filter_map(|(module_id, state)| {
            let module = module_index.get(module_id)?;
            Some(QueryModuleMatch {
                module_id: module.id.clone(),
                name: module.name.clone(),
                kind: module.kind.clone(),
                root_paths: module.root_paths.clone(),
                tags: module.tags.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.root_paths
            .first()
            .cmp(&right.root_paths.first())
            .then(left.module_id.cmp(&right.module_id))
    });
    matches
}

fn finalize_source_matches(
    source_matches: &BTreeMap<String, RelatedMatchState>,
    source_index: &BTreeMap<String, &SourceState>,
) -> Vec<QuerySourceMatch> {
    let mut matches = source_matches
        .iter()
        .filter_map(|(source_id, state)| {
            let source = source_index.get(source_id)?;
            Some(QuerySourceMatch {
                source_id: source.source_id.clone(),
                path: source.path.clone(),
                module_ids: source.module_ids.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| left.path.cmp(&right.path));
    matches
}

fn finalize_relation_matches(
    relation_matches: &BTreeMap<String, RelatedMatchState>,
    relations: &[WikiRelation],
) -> Vec<QueryRelationMatch> {
    let relation_index = relations
        .iter()
        .map(|relation| (relation_identity(relation), relation))
        .collect::<BTreeMap<_, _>>();
    let mut matches = relation_matches
        .iter()
        .filter_map(|(relation_id, state)| {
            let relation = relation_index.get(relation_id)?;
            Some(QueryRelationMatch {
                source_id: relation.source_id.clone(),
                target_id: relation.target_id.clone(),
                relation_type: relation.relation_type.clone(),
                evidence: relation.evidence.clone(),
                reasons: state.reasons.iter().cloned().collect(),
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| {
        left.source_id
            .cmp(&right.source_id)
            .then(left.target_id.cmp(&right.target_id))
            .then(left.relation_type.cmp(&right.relation_type))
    });
    matches
}

fn relation_identity(relation: &WikiRelation) -> String {
    format!(
        "{}|{}|{}|{}",
        relation.source_id,
        relation.target_id,
        relation.relation_type,
        relation.evidence.join("|")
    )
}

fn contains_case_insensitive(value: &str, needle: &str) -> bool {
    value.to_lowercase().contains(needle)
}
