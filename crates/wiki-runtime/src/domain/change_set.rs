//! 变化规划层负责比较当前仓库与最近一次 runtime 的差异。
//! 它为 `status / update / rebuild` 提供统一的 change set、affected set 和回退决策。

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::module_tree::ModuleTree;
use crate::domain::state::{WikiPageState, WikiState};
use crate::domain::steering::{load_steering_config_with_mode, SteeringLoadMode};
use crate::generation::context::{build_module_contexts, build_repo_context};
use crate::generation::sections::{section_id_for_title, section_titles_for_page_type};
use crate::storage::cache_store::{
    missing_incremental_cache_components, read_module_tree_cache, read_scan_cache,
};
use crate::storage::knowledge_artifacts::{load_knowledge_artifacts, KnowledgeArtifactSnapshot};
use crate::storage::metadata_store::metadata_exists;
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::sqlite_store;
use crate::storage::state_store::{load_or_rebuild_state, read_state};
use crate::storage::wiki_fs::{page_exists, wiki_root};
use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::{scan_repo_with_boundary, ScanReport, ScannedFile};
use wiki_index::store::IndexQueryStore;
use wiki_index::symbol_graph::GraphSummary;
use wiki_knowledge::planning::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use wiki_knowledge::{plan_pages_from_knowledge_tree, PagePlan};
use wiki_model::domain::knowledge::{KnowledgeTree, KnowledgeUnit};
use wiki_model::domain::update_scope::{
    AffectedKnowledgeScope, ScopeEscalation, ScopeEscalationLevel,
};

/// `ChangeSet` 描述当前仓库相对最近一次 runtime 的源码变化集合。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeSet {
    /// 最近一次 runtime 中不存在、当前扫描新出现的源码路径。
    pub added_sources: Vec<String>,
    /// 最近一次 runtime 中已存在，但 fingerprint 已变化的源码路径。
    pub modified_sources: Vec<String>,
    /// 最近一次 runtime 中存在、当前扫描已消失的源码路径。
    pub removed_sources: Vec<String>,
    /// 会影响模块树、页面规划或入口识别的结构性源码路径。
    pub structural_sources: Vec<String>,
    /// 通过上一轮 `IMPORTS` edges 命中的一跳 graph 依赖文件。
    pub graph_dependent_sources: Vec<String>,
    /// 是否需要重新计算模块树与页面计划，而不只是局部重渲染。
    pub requires_replan: bool,
    /// 当前 runtime 是否已经损坏到必须回退 full rebuild。
    pub requires_rebuild: bool,
}

/// `AffectedSet` 描述这次变化最终会影响哪些模块、页面和 section。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AffectedSet {
    /// 受这次变化直接或间接影响的模块 ID 集合。
    pub affected_module_ids: Vec<String>,
    /// 本次 update 需要重新生成的页面 ID 集合。
    pub affected_page_ids: Vec<String>,
    /// 每个受影响页面里需要重渲染的稳定 section ID 集合。
    pub affected_section_ids_by_page: BTreeMap<String, Vec<String>>,
    /// 相比上一轮页面计划已经消失、需要删盘的页面 ID 集合。
    pub removed_page_ids: Vec<String>,
    /// 本轮 symbol/edge refresh 需要重新解析的源码路径集合。
    #[serde(default)]
    pub graph_refresh_sources: Vec<String>,
}

/// 增量 runtime 的后续动作选择。
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub enum FallbackMode {
    /// 当前 runtime 足以继续走增量路径。
    None,
    /// 当前 runtime 正式索引缺失，需要回到 init。
    Init,
    /// 当前 runtime 存在关键缺失或损坏，必须强制全量重建。
    Rebuild,
}

/// `ChangePlan` 是 status/update/rebuild 共享的变化规划结果。
pub struct ChangePlan {
    /// 最近一次 runtime 的状态快照；当 runtime 缺失时为空。
    pub previous_state: Option<WikiState>,
    /// 基于当前仓库重新扫描得到的最新扫描结果。
    pub scan_report: Option<ScanReport>,
    /// 当前判定应使用的模块树；局部变化时可能直接复用旧树。
    pub module_tree: Option<ModuleTree>,
    /// 当前模块树和 planner 产出的最新页面计划。
    pub planned_pages: Vec<PagePlan>,
    /// 当前 facts/index 规划得到的最新 knowledge tree。
    pub knowledge_tree: Option<KnowledgeTree>,
    /// 当前仓库相对上一轮 runtime 的源码变化集合。
    pub change_set: ChangeSet,
    /// 变化集合映射得到的受影响知识范围。
    pub affected_knowledge_scope: AffectedKnowledgeScope,
    /// 变化集合映射得到的受影响模块、页面与 section。
    pub affected_set: AffectedSet,
    /// status/update 后续应该采取的回退动作。
    pub fallback_mode: FallbackMode,
    /// 当状态升级为 `needs_rebuild` 时，对外返回的原因文本。
    pub needs_rebuild_reason: Option<String>,
}

impl ChangeSet {
    /// 当前 change set 是否为空。
    ///
    /// # 返回
    /// - 仅当本次没有新增、修改、删除、结构变化，也不要求 replan/rebuild 时返回 `true`。
    pub fn is_empty(&self) -> bool {
        self.added_sources.is_empty()
            && self.modified_sources.is_empty()
            && self.removed_sources.is_empty()
            && self.structural_sources.is_empty()
            && self.graph_dependent_sources.is_empty()
            && !self.requires_replan
            && !self.requires_rebuild
    }

    /// 返回这次变化涉及的全部源码路径。
    ///
    /// # 返回
    /// - 返回去重后的脏源码路径集合，供 `status` 和 `update` 对外展示。
    pub fn dirty_sources(&self) -> Vec<String> {
        let mut dirty = BTreeSet::new();
        dirty.extend(self.added_sources.iter().cloned());
        dirty.extend(self.modified_sources.iter().cloned());
        dirty.extend(self.removed_sources.iter().cloned());
        dirty.extend(self.structural_sources.iter().cloned());
        dirty.into_iter().collect()
    }

    /// 返回需要重跑 symbol/edge refresh 的源码路径。
    ///
    /// # 返回
    /// - 返回新增、修改和 graph 一跳依赖扩散后的源码路径，不包含已删除路径。
    pub fn symbol_refresh_sources(&self) -> Vec<String> {
        let mut dirty = BTreeSet::new();
        dirty.extend(self.added_sources.iter().cloned());
        dirty.extend(self.modified_sources.iter().cloned());
        dirty.extend(self.graph_dependent_sources.iter().cloned());
        dirty.into_iter().collect()
    }
}

impl AffectedSet {
    /// 当前 affected set 是否为空。
    ///
    /// # 返回
    /// - 仅当没有受影响模块、页面、section 和待删除页面时返回 `true`。
    pub fn is_empty(&self) -> bool {
        self.affected_module_ids.is_empty()
            && self.affected_page_ids.is_empty()
            && self.affected_section_ids_by_page.is_empty()
            && self.removed_page_ids.is_empty()
            && self.graph_refresh_sources.is_empty()
    }

    /// 返回本次需要触达的全部页面 ID。
    ///
    /// # 返回
    /// - 返回需要重建或删除的全部页面 ID，供 workflow 进一步映射成页面路径。
    pub fn dirty_page_ids(&self) -> Vec<String> {
        let mut page_ids = BTreeSet::new();
        page_ids.extend(self.affected_page_ids.iter().cloned());
        page_ids.extend(self.removed_page_ids.iter().cloned());
        page_ids.into_iter().collect()
    }
}

impl ChangePlan {
    /// 返回对外可见的 runtime 状态字符串。
    ///
    /// # 返回
    /// - 返回 `fresh / stale / missing / needs_rebuild` 之一。
    pub fn state(&self) -> &'static str {
        match self.fallback_mode {
            FallbackMode::Init => "missing",
            FallbackMode::Rebuild => "needs_rebuild",
            FallbackMode::None => {
                if self.change_set.is_empty() {
                    "fresh"
                } else {
                    "stale"
                }
            }
        }
    }

    /// 把受影响页面 ID 映射成对外返回的页面路径。
    ///
    /// # 返回
    /// - 返回 `.wiki/*.md` 形式的受影响页面路径集合。
    pub fn dirty_page_paths(&self) -> Vec<String> {
        let mut page_paths = BTreeSet::new();

        if let Some(state) = &self.previous_state {
            let old_page_index = state
                .pages
                .iter()
                .map(|page| (page.page_id.as_str(), page))
                .collect::<BTreeMap<_, _>>();

            for page_id in self.affected_set.dirty_page_ids() {
                if let Some(page) = self
                    .planned_pages
                    .iter()
                    .find(|planned| planned.id == page_id)
                {
                    page_paths.insert(format!(".wiki/{}", page.relative_path));
                    continue;
                }

                if let Some(page) = old_page_index.get(page_id.as_str()) {
                    page_paths.insert(page.path.clone());
                }
            }
        }

        page_paths.into_iter().collect()
    }
}

/// 读取最近一次 runtime，并为 `status / update / rebuild` 规划后续动作。
///
/// # 参数
/// - `repo_root`：要检查或更新的本地代码目录。
///
/// # 返回
/// - 成功时返回统一的 `ChangePlan`，包含变化集合、受影响集合和回退动作。
///
/// # 错误
/// - 当 runtime 状态、metadata、cache 或当前仓库扫描无法读取时返回错误。
pub fn plan_runtime_changes(repo_root: &Path) -> io::Result<ChangePlan> {
    plan_runtime_changes_with_mode(repo_root, SteeringLoadMode::Production)
}

pub fn plan_runtime_changes_with_mode(
    repo_root: &Path,
    steering_mode: SteeringLoadMode,
) -> io::Result<ChangePlan> {
    if !wiki_root(repo_root).exists() || !metadata_exists(repo_root) {
        return Ok(ChangePlan {
            previous_state: None,
            scan_report: None,
            module_tree: None,
            planned_pages: Vec::new(),
            knowledge_tree: None,
            change_set: ChangeSet::default(),
            affected_knowledge_scope: AffectedKnowledgeScope::default(),
            affected_set: AffectedSet::default(),
            fallback_mode: FallbackMode::Init,
            needs_rebuild_reason: None,
        });
    }

    let previous_state = load_or_rebuild_state(repo_root)?;
    let steering = load_steering_config_with_mode(repo_root, steering_mode);
    let previous_scan = read_scan_cache(repo_root).ok();
    let previous_module_tree = read_module_tree_cache(repo_root).ok();
    let mut missing_cache_components =
        missing_incremental_cache_components(repo_root, &previous_state);

    if previous_scan.is_none() {
        missing_cache_components.push("repo-scan".to_string());
    }

    if previous_module_tree.is_none() {
        missing_cache_components.push("module-tree".to_string());
    }

    if crate::storage::sqlite_store::db_exists(repo_root) && read_state(repo_root).is_err() {
        missing_cache_components.push("wiki-state".to_string());
    }

    missing_cache_components.sort();
    missing_cache_components.dedup();

    let missing_pages = previous_state
        .pages
        .iter()
        .filter(|page| !page_exists(repo_root, &page.path))
        .map(|page| page.path.clone())
        .collect::<Vec<_>>();

    if !missing_pages.is_empty() {
        return Ok(ChangePlan {
            previous_state: Some(previous_state.clone()),
            scan_report: None,
            module_tree: previous_module_tree,
            planned_pages: Vec::new(),
            knowledge_tree: None,
            change_set: ChangeSet {
                requires_rebuild: true,
                ..ChangeSet::default()
            },
            affected_knowledge_scope: AffectedKnowledgeScope {
                escalation: ScopeEscalation {
                    level: ScopeEscalationLevel::RebuildRecommended,
                    reason: "page_missing".to_string(),
                },
                ..AffectedKnowledgeScope::default()
            },
            affected_set: affected_pages_for_missing_paths(&previous_state, &missing_pages),
            fallback_mode: FallbackMode::Rebuild,
            needs_rebuild_reason: Some("page_missing".to_string()),
        });
    }

    if !missing_cache_components.is_empty() {
        return Ok(ChangePlan {
            previous_state: Some(previous_state.clone()),
            scan_report: None,
            module_tree: previous_module_tree,
            planned_pages: Vec::new(),
            knowledge_tree: None,
            change_set: ChangeSet {
                requires_rebuild: true,
                ..ChangeSet::default()
            },
            affected_knowledge_scope: AffectedKnowledgeScope {
                escalation: ScopeEscalation {
                    level: ScopeEscalationLevel::RebuildRecommended,
                    reason: "cache_missing".to_string(),
                },
                ..AffectedKnowledgeScope::default()
            },
            affected_set: affected_pages_for_missing_cache(
                &previous_state,
                &missing_cache_components,
            ),
            fallback_mode: FallbackMode::Rebuild,
            needs_rebuild_reason: Some("cache_missing".to_string()),
        });
    }

    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report = scan_repo_with_boundary(repo_root, ignore_paths, include_paths)?;
    let mut change_set = build_change_set(&previous_state, previous_scan.as_ref(), &scan_report);
    change_set.graph_dependent_sources =
        build_graph_dependent_sources(repo_root, &change_set.dirty_sources())?;
    let current_module_tree = if change_set.requires_replan {
        build_module_tree(&scan_report)
    } else {
        previous_module_tree
            .clone()
            .expect("module tree cache should exist when runtime is incrementally healthy")
    };

    let repo_context = build_repo_context(&scan_report, &current_module_tree);
    let module_contexts = build_module_contexts(&scan_report, &current_module_tree);
    let planner_config = steering.knowledge_planner_config();
    let domains = discover_knowledge_domains(
        &scan_report,
        &current_module_tree,
        &repo_context,
        &module_contexts,
        &GraphSummary::default(),
        &planner_config,
    );
    let units = plan_knowledge_units(
        &domains,
        &current_module_tree,
        &scan_report,
        &module_contexts,
        &planner_config,
    );
    let knowledge_tree = build_knowledge_tree(domains, units);
    let planned_pages = plan_pages_from_knowledge_tree(&knowledge_tree);
    let previous_artifacts = load_knowledge_artifacts(repo_root).ok();
    let affected_knowledge_scope = build_affected_knowledge_scope(
        previous_artifacts.as_ref(),
        &previous_state,
        previous_module_tree.as_ref(),
        &current_module_tree,
        &scan_report,
        &knowledge_tree,
        &planned_pages,
        &change_set,
    );
    let affected_set = build_affected_set(
        &previous_state,
        previous_artifacts.as_ref(),
        &knowledge_tree,
        &planned_pages,
        &affected_knowledge_scope,
        &change_set,
    );

    Ok(ChangePlan {
        previous_state: Some(previous_state),
        scan_report: Some(scan_report),
        module_tree: Some(current_module_tree),
        planned_pages,
        knowledge_tree: Some(knowledge_tree),
        change_set,
        affected_knowledge_scope,
        affected_set,
        fallback_mode: FallbackMode::None,
        needs_rebuild_reason: None,
    })
}

fn build_change_set(
    previous_state: &WikiState,
    previous_scan: Option<&ScanReport>,
    current_scan: &ScanReport,
) -> ChangeSet {
    let previous_sources = previous_state
        .sources
        .iter()
        .map(|source| (source.path.clone(), source.fingerprint.clone()))
        .collect::<BTreeMap<_, _>>();
    let current_sources = current_scan
        .files
        .iter()
        .map(|file| (file.path.clone(), file))
        .collect::<BTreeMap<_, _>>();
    let previous_scan_files = previous_scan
        .map(|scan| {
            scan.files
                .iter()
                .map(|file| (file.path.clone(), file))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    let mut added_sources = Vec::new();
    let mut modified_sources = Vec::new();
    let mut removed_sources = Vec::new();

    for (path, previous_fingerprint) in &previous_sources {
        match current_sources.get(path) {
            Some(current_file) if current_file.fingerprint == *previous_fingerprint => {}
            Some(_) => modified_sources.push(path.clone()),
            None => removed_sources.push(path.clone()),
        }
    }

    for path in current_sources.keys() {
        if !previous_sources.contains_key(path) {
            added_sources.push(path.clone());
        }
    }

    let mut structural_sources = BTreeSet::new();

    for path in added_sources
        .iter()
        .chain(modified_sources.iter())
        .chain(removed_sources.iter())
    {
        let current_file = current_sources.get(path.as_str()).copied();
        let previous_file = previous_scan_files.get(path.as_str()).copied();

        if is_structural_file(path, current_file, previous_file) {
            structural_sources.insert(path.clone());
        }
    }

    if let Some(previous_scan) = previous_scan {
        structural_sources.extend(diff_string_sets(
            &previous_scan.workspace_roots,
            &current_scan.workspace_roots,
        ));
        structural_sources.extend(diff_string_sets(
            &previous_scan.entry_points,
            &current_scan.entry_points,
        ));
        structural_sources.extend(diff_string_sets(
            &previous_scan.config_files,
            &current_scan.config_files,
        ));
    }

    let requires_replan =
        !added_sources.is_empty() || !removed_sources.is_empty() || !structural_sources.is_empty();

    ChangeSet {
        added_sources,
        modified_sources,
        removed_sources,
        structural_sources: structural_sources.into_iter().collect(),
        graph_dependent_sources: Vec::new(),
        requires_replan,
        requires_rebuild: false,
    }
}

/// 用 formal knowledge identity 先规划知识范围，再由 runtime 派生投影范围。
fn build_affected_knowledge_scope(
    previous_artifacts: Option<&KnowledgeArtifactSnapshot>,
    previous_state: &WikiState,
    previous_module_tree: Option<&ModuleTree>,
    current_module_tree: &ModuleTree,
    scan_report: &ScanReport,
    knowledge_tree: &KnowledgeTree,
    planned_pages: &[PagePlan],
    change_set: &ChangeSet,
) -> AffectedKnowledgeScope {
    let dirty_source_paths = change_set
        .dirty_sources()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let current_source_ids_by_path = scan_report
        .files
        .iter()
        .map(|file| (file.path.as_str(), file.id.clone()))
        .collect::<BTreeMap<_, _>>();
    let previous_source_ids_by_path = previous_state
        .sources
        .iter()
        .map(|source| (source.path.as_str(), source.source_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let current_dirty_source_ids = dirty_source_paths
        .iter()
        .filter_map(|path| current_source_ids_by_path.get(path.as_str()))
        .cloned()
        .collect::<BTreeSet<_>>();
    let previous_dirty_source_ids = dirty_source_paths
        .iter()
        .filter_map(|path| previous_source_ids_by_path.get(path.as_str()))
        .cloned()
        .collect::<BTreeSet<_>>();
    let dirty_module_ids = collect_dirty_module_ids(
        &dirty_source_paths,
        previous_module_tree,
        current_module_tree,
    );
    let previous_tree = previous_artifacts.map(|artifacts| &artifacts.knowledge_tree);

    let direct_unit_ids = collect_units_for_sources_or_modules(
        knowledge_tree,
        &current_dirty_source_ids,
        &dirty_module_ids,
    );
    let previous_direct_unit_ids = previous_tree
        .map(|tree| {
            collect_units_for_sources_or_modules(
                tree,
                &previous_dirty_source_ids,
                &dirty_module_ids,
            )
        })
        .unwrap_or_default();
    let mut removed_unit_ids = previous_direct_unit_ids
        .iter()
        .filter(|unit_id| knowledge_tree.get_unit(unit_id).is_none())
        .cloned()
        .collect::<BTreeSet<_>>();

    if change_set.requires_replan {
        removed_unit_ids.extend(collect_removed_unit_ids(previous_tree, knowledge_tree));
    }

    let mut propagated_parent_unit_ids = collect_propagated_parent_units(
        knowledge_tree,
        previous_tree,
        &direct_unit_ids,
        &removed_unit_ids,
    );
    for direct_unit_id in &direct_unit_ids {
        propagated_parent_unit_ids.remove(direct_unit_id);
    }

    let mut affected_domain_ids = BTreeSet::new();
    for unit_id in direct_unit_ids
        .iter()
        .chain(propagated_parent_unit_ids.iter())
    {
        if let Some(unit) = knowledge_tree.get_unit(unit_id) {
            affected_domain_ids.insert(unit.domain_id.clone());
        }
    }
    if let Some(previous_tree) = previous_tree {
        for unit_id in &removed_unit_ids {
            if let Some(unit) = previous_tree.get_unit(unit_id) {
                affected_domain_ids.insert(unit.domain_id.clone());
            }
        }
    }

    let active_unit_ids = direct_unit_ids
        .iter()
        .chain(propagated_parent_unit_ids.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut stale_unit_ids = BTreeSet::new();
    let mut health_signal_targets = BTreeSet::new();
    if let Some(previous_artifacts) = previous_artifacts {
        collect_health_recovery_scope(
            previous_artifacts,
            knowledge_tree,
            previous_tree,
            &mut stale_unit_ids,
            &mut health_signal_targets,
        );
    }
    let projection_unit_ids = collect_projection_unit_ids(knowledge_tree, &active_unit_ids);
    let mut projection_target_page_ids = planned_pages
        .iter()
        .filter(|page| {
            page.unit_id
                .as_ref()
                .map(|unit_id| projection_unit_ids.contains(unit_id))
                .unwrap_or(false)
        })
        .map(|page| page.id.clone())
        .collect::<BTreeSet<_>>();
    if let Some(previous_tree) = previous_tree {
        for previous_page in plan_pages_from_knowledge_tree(previous_tree) {
            if previous_page
                .unit_id
                .as_ref()
                .map(|unit_id| removed_unit_ids.contains(unit_id))
                .unwrap_or(false)
            {
                projection_target_page_ids.insert(previous_page.id.clone());
            }
        }
    }
    projection_target_page_ids.extend(
        planned_pages
            .iter()
            .filter(|page| {
                page.unit_id
                    .as_ref()
                    .map(|unit_id| stale_unit_ids.contains(unit_id))
                    .unwrap_or(false)
            })
            .map(|page| page.id.clone()),
    );
    projection_target_page_ids.extend(collect_projection_targets_for_units(
        knowledge_tree,
        previous_tree,
        &stale_unit_ids,
    ));
    let stale_projection_ids = projection_target_page_ids.clone();
    let related_unit_ids = active_unit_ids
        .union(&stale_unit_ids)
        .chain(removed_unit_ids.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut declared_record_ids = BTreeSet::new();
    if let Some(previous_artifacts) = previous_artifacts {
        declared_record_ids = collect_declared_record_ids(
            previous_artifacts,
            &related_unit_ids,
            &projection_target_page_ids,
        );
    }

    let escalation = if change_set.requires_rebuild {
        ScopeEscalation {
            level: ScopeEscalationLevel::RebuildRecommended,
            reason: "runtime_inconsistent".to_string(),
        }
    } else if change_set.requires_replan {
        let level = if affected_domain_ids.len() <= 1 {
            ScopeEscalationLevel::SubtreeReplan
        } else {
            ScopeEscalationLevel::RepoReplan
        };
        let reason = if !change_set.structural_sources.is_empty() {
            format!(
                "structural_change:{}",
                change_set.structural_sources.join(",")
            )
        } else if !change_set.added_sources.is_empty() || !change_set.removed_sources.is_empty() {
            "knowledge_identity_changed".to_string()
        } else {
            "planner_replan_required".to_string()
        };
        ScopeEscalation { level, reason }
    } else {
        ScopeEscalation {
            level: ScopeEscalationLevel::LocalRefresh,
            reason: "direct_unit_refresh".to_string(),
        }
    };

    AffectedKnowledgeScope {
        direct_unit_ids: direct_unit_ids.into_iter().collect(),
        propagated_parent_unit_ids: propagated_parent_unit_ids.into_iter().collect(),
        stale_unit_ids: stale_unit_ids.iter().cloned().collect(),
        removed_unit_ids: removed_unit_ids.into_iter().collect(),
        affected_domain_ids: affected_domain_ids
            .into_iter()
            .chain(collect_domains_for_units(
                knowledge_tree,
                previous_tree,
                &stale_unit_ids,
            ))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        declared_record_ids: declared_record_ids.into_iter().collect(),
        stale_projection_ids: stale_projection_ids.into_iter().collect(),
        projection_target_page_ids: projection_target_page_ids.into_iter().collect(),
        health_signal_targets: health_signal_targets.into_iter().collect(),
        escalation,
    }
}

fn collect_health_recovery_scope(
    previous_artifacts: &KnowledgeArtifactSnapshot,
    knowledge_tree: &KnowledgeTree,
    previous_tree: Option<&KnowledgeTree>,
    stale_unit_ids: &mut BTreeSet<String>,
    health_signal_targets: &mut BTreeSet<String>,
) {
    for signal in &previous_artifacts.health_signals {
        if !matches!(
            signal.recommended_action,
            wiki_model::domain::knowledge_artifact::KnowledgeHealthRecommendedAction::Update
                | wiki_model::domain::knowledge_artifact::KnowledgeHealthRecommendedAction::Rebuild
        ) {
            continue;
        }

        let Some(unit_id) = signal.target_ref.strip_prefix("unit:") else {
            continue;
        };
        if knowledge_tree.get_unit(unit_id).is_none()
            && previous_tree
                .and_then(|tree| tree.get_unit(unit_id))
                .is_none()
        {
            continue;
        }
        stale_unit_ids.insert(unit_id.to_string());
        health_signal_targets.insert(signal.target_ref.clone());
    }
}

fn collect_projection_targets_for_units(
    knowledge_tree: &KnowledgeTree,
    previous_tree: Option<&KnowledgeTree>,
    unit_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut projection_targets = BTreeSet::new();
    for unit_id in unit_ids {
        if let Some(unit) = knowledge_tree
            .get_unit(unit_id)
            .or_else(|| previous_tree.and_then(|tree| tree.get_unit(unit_id)))
        {
            for projection_ref in &unit.projection_refs {
                if let Some(page_id) = projection_ref.strip_prefix("page:") {
                    let page_id = page_id.split(":section:").next().unwrap_or(page_id);
                    projection_targets.insert(page_id.to_string());
                }
            }
        }
    }
    projection_targets
}

fn collect_declared_record_ids(
    previous_artifacts: &KnowledgeArtifactSnapshot,
    related_unit_ids: &BTreeSet<String>,
    projection_target_page_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    previous_artifacts
        .declared_records
        .iter()
        .filter(|record| {
            record
                .unit_refs
                .iter()
                .any(|unit_id| related_unit_ids.contains(unit_id))
                || projection_target_page_ids.contains(&record.page_id)
                || record.projection_refs.iter().any(|projection_ref| {
                    projection_ref
                        .strip_prefix("page:")
                        .and_then(|value| value.split(":section:").next())
                        .map(|page_id| projection_target_page_ids.contains(page_id))
                        .unwrap_or(false)
                })
        })
        .map(|record| record.record_id.clone())
        .collect()
}

fn collect_domains_for_units(
    knowledge_tree: &KnowledgeTree,
    previous_tree: Option<&KnowledgeTree>,
    unit_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    unit_ids
        .iter()
        .filter_map(|unit_id| {
            knowledge_tree
                .get_unit(unit_id)
                .or_else(|| previous_tree.and_then(|tree| tree.get_unit(unit_id)))
                .map(|unit| unit.domain_id.clone())
        })
        .collect()
}

fn collect_projection_unit_ids(
    knowledge_tree: &KnowledgeTree,
    active_unit_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut projection_unit_ids = active_unit_ids.clone();
    let mut queue = active_unit_ids.iter().cloned().collect::<Vec<_>>();
    while let Some(unit_id) = queue.pop() {
        let Some(unit) = knowledge_tree.get_unit(&unit_id) else {
            continue;
        };
        let Some(parent_id) = unit.parent_unit_id.as_ref() else {
            continue;
        };
        if projection_unit_ids.insert(parent_id.clone()) {
            queue.push(parent_id.clone());
        }
    }
    projection_unit_ids
}

fn collect_dirty_module_ids(
    dirty_source_paths: &BTreeSet<String>,
    previous_module_tree: Option<&ModuleTree>,
    current_module_tree: &ModuleTree,
) -> BTreeSet<String> {
    let mut affected_module_ids = BTreeSet::new();
    for path in dirty_source_paths {
        if let Some(module_id) = best_module_id_for_path(path, current_module_tree) {
            affected_module_ids.insert(module_id);
        }

        if let Some(previous_tree) = previous_module_tree {
            if let Some(module_id) = best_module_id_for_path(path, previous_tree) {
                affected_module_ids.insert(module_id);
            }
        }
    }
    affected_module_ids
}

fn collect_units_for_sources_or_modules(
    knowledge_tree: &KnowledgeTree,
    dirty_source_ids: &BTreeSet<String>,
    dirty_module_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let candidate_unit_ids = knowledge_tree
        .units
        .values()
        .filter(|unit| {
            unit.scope
                .source_ids
                .iter()
                .any(|source_id| dirty_source_ids.contains(source_id))
                || (unit.child_unit_ids.is_empty()
                    && unit
                        .scope
                        .module_ids
                        .iter()
                        .any(|module_id| dirty_module_ids.contains(module_id)))
        })
        .map(|unit| unit.id.clone())
        .collect::<BTreeSet<_>>();

    candidate_unit_ids
        .iter()
        .filter(|unit_id| !has_matching_descendant(knowledge_tree, unit_id, &candidate_unit_ids))
        .cloned()
        .collect()
}

fn has_matching_descendant(
    knowledge_tree: &KnowledgeTree,
    unit_id: &str,
    candidate_unit_ids: &BTreeSet<String>,
) -> bool {
    let Some(unit) = knowledge_tree.get_unit(unit_id) else {
        return false;
    };

    let mut queue = unit.child_unit_ids.iter().cloned().collect::<Vec<_>>();
    while let Some(child_unit_id) = queue.pop() {
        if candidate_unit_ids.contains(&child_unit_id) {
            return true;
        }
        if let Some(child_unit) = knowledge_tree.get_unit(&child_unit_id) {
            queue.extend(child_unit.child_unit_ids.iter().cloned());
        }
    }
    false
}

fn collect_removed_unit_ids(
    previous_tree: Option<&KnowledgeTree>,
    current_tree: &KnowledgeTree,
) -> BTreeSet<String> {
    let Some(previous_tree) = previous_tree else {
        return BTreeSet::new();
    };

    previous_tree
        .units
        .keys()
        .filter(|unit_id| current_tree.get_unit(unit_id).is_none())
        .cloned()
        .collect()
}

fn collect_propagated_parent_units(
    current_tree: &KnowledgeTree,
    previous_tree: Option<&KnowledgeTree>,
    direct_unit_ids: &BTreeSet<String>,
    removed_unit_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut propagated = BTreeSet::new();
    let mut queue = direct_unit_ids.iter().cloned().collect::<Vec<_>>();
    let impacted_unit_ids = direct_unit_ids
        .iter()
        .chain(removed_unit_ids.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    while let Some(unit_id) = queue.pop() {
        let Some(unit) = current_tree.get_unit(&unit_id) else {
            continue;
        };
        if !parent_propagation_required(
            unit,
            previous_tree.and_then(|tree| tree.get_unit(&unit.id)),
        ) {
            continue;
        }
        if let Some(parent_id) = unit.parent_unit_id.as_ref() {
            if propagated.insert(parent_id.clone()) {
                queue.push(parent_id.clone());
            }
        }
    }

    let mut contract_changed_parent_queue = collect_child_contract_changed_parent_units(
        current_tree,
        previous_tree,
        &impacted_unit_ids,
    )
    .into_iter()
    .collect::<Vec<_>>();
    while let Some(unit_id) = contract_changed_parent_queue.pop() {
        if propagated.insert(unit_id.clone()) {
            if let Some(parent_id) = current_tree
                .get_unit(&unit_id)
                .and_then(|unit| unit.parent_unit_id.as_ref())
            {
                contract_changed_parent_queue.push(parent_id.clone());
            }
        }
    }

    if let Some(previous_tree) = previous_tree {
        let mut previous_queue = removed_unit_ids.iter().cloned().collect::<Vec<_>>();
        while let Some(unit_id) = previous_queue.pop() {
            let Some(unit) = previous_tree.get_unit(&unit_id) else {
                continue;
            };
            if let Some(parent_id) = unit.parent_unit_id.as_ref() {
                if current_tree.get_unit(parent_id).is_some()
                    && propagated.insert(parent_id.clone())
                {
                    previous_queue.push(parent_id.clone());
                }
            }
        }
    }

    propagated
}

fn collect_child_contract_changed_parent_units(
    current_tree: &KnowledgeTree,
    previous_tree: Option<&KnowledgeTree>,
    impacted_unit_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let Some(previous_tree) = previous_tree else {
        return BTreeSet::new();
    };

    current_tree
        .units
        .values()
        .filter(|unit| !unit.child_unit_ids.is_empty())
        .filter_map(|unit| {
            let previous_unit = previous_tree.get_unit(&unit.id);
            if !parent_propagation_required(unit, previous_unit) {
                return None;
            }

            let current_children = unit.child_unit_ids.iter().cloned().collect::<BTreeSet<_>>();
            let previous_children = previous_unit
                .map(|previous| {
                    previous
                        .child_unit_ids
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default();
            let child_delta = current_children
                .symmetric_difference(&previous_children)
                .cloned()
                .collect::<BTreeSet<_>>();

            child_delta
                .iter()
                .any(|child_unit_id| impacted_unit_ids.contains(child_unit_id))
                .then(|| unit.id.clone())
        })
        .collect()
}

fn parent_propagation_required(
    current_unit: &KnowledgeUnit,
    previous_unit: Option<&KnowledgeUnit>,
) -> bool {
    match previous_unit {
        Some(previous_unit) => {
            previous_unit.child_unit_ids != current_unit.child_unit_ids
                || previous_unit.parent_unit_id != current_unit.parent_unit_id
                || previous_unit.scope.source_ids != current_unit.scope.source_ids
                || previous_unit.scope.module_ids != current_unit.scope.module_ids
                || previous_unit.relative_path != current_unit.relative_path
        }
        None => true,
    }
}

fn build_affected_set(
    previous_state: &WikiState,
    previous_artifacts: Option<&KnowledgeArtifactSnapshot>,
    knowledge_tree: &KnowledgeTree,
    planned_pages: &[PagePlan],
    affected_knowledge_scope: &AffectedKnowledgeScope,
    change_set: &ChangeSet,
) -> AffectedSet {
    let previous_pages = previous_state
        .pages
        .iter()
        .map(|page| (page.page_id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let current_pages = planned_pages
        .iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let previous_planned_pages = previous_artifacts
        .map(|artifacts| plan_pages_from_knowledge_tree(&artifacts.knowledge_tree))
        .unwrap_or_default();
    let previous_planned_by_id = previous_planned_pages
        .iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();

    let current_units = affected_knowledge_scope
        .active_unit_ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let removed_units = affected_knowledge_scope
        .removed_unit_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut affected_module_ids = BTreeSet::new();
    for unit_id in &current_units {
        if let Some(unit) = knowledge_tree.get_unit(unit_id) {
            affected_module_ids.extend(unit.scope.module_ids.iter().cloned());
        }
    }
    if let Some(previous_artifacts) = previous_artifacts {
        for unit_id in &removed_units {
            if let Some(unit) = previous_artifacts.knowledge_tree.get_unit(unit_id) {
                affected_module_ids.extend(unit.scope.module_ids.iter().cloned());
            }
        }
    }

    let mut affected_page_ids = affected_knowledge_scope
        .projection_target_page_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if !matches!(
        affected_knowledge_scope.escalation.level,
        ScopeEscalationLevel::LocalRefresh
    ) {
        affected_page_ids.extend(
            planned_pages
                .iter()
                .filter(|page| matches!(page.page_type.as_str(), "overview" | "architecture"))
                .map(|page| page.id.clone()),
        );
    }
    let mut removed_page_ids = BTreeSet::new();
    for previous_page in &previous_planned_pages {
        if let Some(unit_id) = previous_page.unit_id.as_ref() {
            if removed_units.contains(unit_id) && !current_pages.contains_key(&previous_page.id) {
                removed_page_ids.insert(previous_page.id.clone());
            }
        }
    }

    expand_affected_page_ancestors(&previous_pages, &current_pages, &mut affected_page_ids);
    let mut removed_and_affected = affected_page_ids.clone();
    removed_and_affected.extend(removed_page_ids.iter().cloned());
    expand_affected_page_ancestors(&previous_pages, &current_pages, &mut removed_and_affected);
    affected_page_ids = removed_and_affected
        .into_iter()
        .filter(|page_id| !removed_page_ids.contains(page_id))
        .collect();

    let mut affected_section_ids_by_page = BTreeMap::new();
    for page_id in &affected_page_ids {
        if let Some(page) = previous_pages.get(page_id) {
            affected_section_ids_by_page.insert(page_id.clone(), page.managed_section_anchors());
            continue;
        }

        if let Some(page) = current_pages.get(page_id) {
            affected_section_ids_by_page
                .insert(page_id.clone(), predicted_section_ids_for_page(page));
        }
    }

    for page_id in &removed_page_ids {
        if let Some(page) = previous_pages.get(page_id) {
            affected_section_ids_by_page.insert(page_id.clone(), page.managed_section_anchors());
        } else if let Some(page) = previous_planned_by_id.get(page_id) {
            affected_section_ids_by_page
                .insert(page_id.clone(), predicted_section_ids_for_page(page));
        }
    }

    AffectedSet {
        affected_module_ids: affected_module_ids.into_iter().collect(),
        affected_page_ids: affected_page_ids.into_iter().collect(),
        affected_section_ids_by_page,
        removed_page_ids: removed_page_ids.into_iter().collect(),
        graph_refresh_sources: change_set.symbol_refresh_sources(),
    }
}

fn affected_pages_for_missing_paths(
    previous_state: &WikiState,
    missing_paths: &[String],
) -> AffectedSet {
    let missing_paths = missing_paths.iter().cloned().collect::<BTreeSet<_>>();
    let removed_page_ids = previous_state
        .pages
        .iter()
        .filter(|page| missing_paths.contains(&page.path))
        .map(|page| page.page_id.clone())
        .collect::<Vec<_>>();

    AffectedSet {
        affected_module_ids: Vec::new(),
        affected_page_ids: Vec::new(),
        affected_section_ids_by_page: BTreeMap::new(),
        removed_page_ids,
        graph_refresh_sources: Vec::new(),
    }
}

fn affected_pages_for_missing_cache(
    previous_state: &WikiState,
    missing_components: &[String],
) -> AffectedSet {
    let mut page_ids = BTreeSet::new();

    for component in missing_components {
        if let Some((_, page_id)) = component.split_once(':') {
            page_ids.insert(page_id.to_string());
        }
    }

    if page_ids.is_empty() {
        page_ids.extend(previous_state.pages.iter().map(|page| page.page_id.clone()));
    }

    AffectedSet {
        affected_module_ids: Vec::new(),
        affected_page_ids: page_ids.into_iter().collect(),
        affected_section_ids_by_page: BTreeMap::new(),
        removed_page_ids: Vec::new(),
        graph_refresh_sources: Vec::new(),
    }
}

fn build_graph_dependent_sources(
    repo_root: &Path,
    dirty_source_paths: &[String],
) -> io::Result<Vec<String>> {
    if dirty_source_paths.is_empty() || !sqlite_store::db_exists(repo_root) {
        return Ok(Vec::new());
    }

    let index_store = SqliteIndexStore::new(repo_root);
    let dirty = dirty_source_paths.iter().cloned().collect::<BTreeSet<_>>();
    let symbols = index_store.list_symbols()?;
    let edges = index_store.list_edges()?;
    let symbol_files = symbols
        .into_iter()
        .map(|symbol| (symbol.symbol_id, symbol.file_path))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeSet::new();

    for edge in edges {
        if edge.edge_type != "IMPORTS" {
            continue;
        }
        let Some(source_file) = symbol_files.get(&edge.source_id) else {
            continue;
        };
        let Some(target_file) = symbol_files.get(&edge.target_id) else {
            continue;
        };
        if dirty.contains(target_file.as_str()) && !dirty.contains(source_file.as_str()) {
            dependents.insert(source_file.clone());
        }
    }

    Ok(dependents.into_iter().collect())
}

fn diff_string_sets(previous: &[String], current: &[String]) -> BTreeSet<String> {
    let previous = previous.iter().cloned().collect::<BTreeSet<_>>();
    let current = current.iter().cloned().collect::<BTreeSet<_>>();

    previous
        .difference(&current)
        .chain(current.difference(&previous))
        .cloned()
        .collect()
}

fn is_structural_file(
    path: &str,
    current_file: Option<&ScannedFile>,
    previous_file: Option<&ScannedFile>,
) -> bool {
    if current_file
        .map(|file| file.is_config_like() || file.is_entry_like() || file.purpose.is_structural())
        .unwrap_or(false)
    {
        return true;
    }

    if previous_file
        .map(|file| file.is_config_like() || file.is_entry_like() || file.purpose.is_structural())
        .unwrap_or(false)
    {
        return true;
    }

    matches!(
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default(),
        "package.json"
            | "Cargo.toml"
            | "pnpm-workspace.yaml"
            | "pyproject.toml"
            | "requirements.txt"
            | "Pipfile"
            | "tsconfig.json"
            | "nginx.conf"
    )
}

fn best_module_id_for_path(path: &str, module_tree: &ModuleTree) -> Option<String> {
    module_tree
        .modules
        .iter()
        .filter(|module| {
            module
                .root_paths
                .iter()
                .any(|root| path_matches_root(path, root))
        })
        .max_by_key(|module| {
            module
                .root_paths
                .iter()
                .map(|root| root.len())
                .max()
                .unwrap_or_default()
        })
        .map(|module| module.id.clone())
}

fn path_matches_root(path: &str, root: &str) -> bool {
    if root == "." {
        return true;
    }

    path == root || path.starts_with(&format!("{root}/"))
}

fn predicted_section_ids_for_page(page: &PagePlan) -> Vec<String> {
    section_titles_for_page_type(&page.page_type)
        .into_iter()
        .map(|title| section_id_for_title(&page.id, title))
        .collect()
}

fn expand_affected_page_ancestors(
    previous_pages: &BTreeMap<String, &WikiPageState>,
    current_pages: &BTreeMap<String, &PagePlan>,
    affected_page_ids: &mut BTreeSet<String>,
) {
    let mut queue = affected_page_ids.iter().cloned().collect::<Vec<_>>();
    while let Some(page_id) = queue.pop() {
        let parent_id = current_pages
            .get(&page_id)
            .and_then(|page| page.parent_id.clone())
            .or_else(|| {
                previous_pages
                    .get(&page_id)
                    .and_then(|page| page.parent_id.clone())
            });
        let Some(parent_id) = parent_id else {
            continue;
        };
        if affected_page_ids.insert(parent_id.clone()) {
            queue.push(parent_id);
        }
    }
}
