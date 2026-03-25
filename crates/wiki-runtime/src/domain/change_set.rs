//! 变化规划层负责比较当前仓库与最近一次 runtime 的差异。
//! 它为 `status / update / rebuild` 提供统一的 change set、affected set 和回退决策。

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::module_tree::ModuleTree;
use crate::domain::state::{WikiPageState, WikiState};
use crate::domain::steering::load_steering_config;
use crate::generation::context::{build_module_contexts, build_repo_context};
use wiki_knowledge::planning::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use wiki_knowledge::{plan_pages_from_knowledge_tree, PlannedPage};
use crate::generation::sections::{section_id_for_title, section_titles_for_page_type};
use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::{scan_repo_with_boundary, ScanReport, ScannedFile};
use wiki_index::symbol_graph::GraphSummary;
use crate::storage::cache_store::{
    missing_incremental_cache_components, read_module_tree_cache, read_scan_cache,
};
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::metadata_store::metadata_exists;
use crate::storage::sqlite_store;
use wiki_index::store::IndexQueryStore;
use crate::storage::state_store::{load_or_rebuild_state, read_state};
use crate::storage::wiki_fs::{page_exists, wiki_root};

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
    pub planned_pages: Vec<PlannedPage>,
    /// 当前仓库相对上一轮 runtime 的源码变化集合。
    pub change_set: ChangeSet,
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
    if !wiki_root(repo_root).exists() || !metadata_exists(repo_root) {
        return Ok(ChangePlan {
            previous_state: None,
            scan_report: None,
            module_tree: None,
            planned_pages: Vec::new(),
            change_set: ChangeSet::default(),
            affected_set: AffectedSet::default(),
            fallback_mode: FallbackMode::Init,
            needs_rebuild_reason: None,
        });
    }

    let previous_state = load_or_rebuild_state(repo_root)?;
    let steering = load_steering_config(repo_root);
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
            change_set: ChangeSet {
                requires_rebuild: true,
                ..ChangeSet::default()
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
            change_set: ChangeSet {
                requires_rebuild: true,
                ..ChangeSet::default()
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
    let affected_set = build_affected_set(
        &previous_state,
        previous_module_tree.as_ref(),
        &current_module_tree,
        &planned_pages,
        &change_set,
    );

    Ok(ChangePlan {
        previous_state: Some(previous_state),
        scan_report: Some(scan_report),
        module_tree: Some(current_module_tree),
        planned_pages,
        change_set,
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

fn build_affected_set(
    previous_state: &WikiState,
    previous_module_tree: Option<&ModuleTree>,
    current_module_tree: &ModuleTree,
    planned_pages: &[PlannedPage],
    change_set: &ChangeSet,
) -> AffectedSet {
    let dirty_source_paths = change_set
        .dirty_sources()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let previous_pages = previous_state
        .pages
        .iter()
        .map(|page| (page.page_id.clone(), page))
        .collect::<BTreeMap<_, _>>();
    let current_pages = planned_pages
        .iter()
        .map(|page| (page.id.clone(), page))
        .collect::<BTreeMap<_, _>>();

    let mut affected_module_ids = BTreeSet::new();
    let mut affected_page_ids = BTreeSet::new();
    let mut removed_page_ids = BTreeSet::new();

    for path in &dirty_source_paths {
        if let Some(module_id) = best_module_id_for_path(path, current_module_tree) {
            affected_module_ids.insert(module_id);
        }

        if let Some(previous_tree) = previous_module_tree {
            if let Some(module_id) = best_module_id_for_path(path, previous_tree) {
                affected_module_ids.insert(module_id);
            }
        }
    }

    if let Some(previous_tree) = previous_module_tree {
        let previous_modules = previous_tree
            .modules
            .iter()
            .map(|module| (module.id.clone(), module))
            .collect::<BTreeMap<_, _>>();
        let current_modules = current_module_tree
            .modules
            .iter()
            .map(|module| (module.id.clone(), module))
            .collect::<BTreeMap<_, _>>();

        for module_id in previous_modules.keys() {
            if !current_modules.contains_key(module_id) {
                affected_module_ids.insert(module_id.clone());
            }
        }

        for (module_id, current_module) in &current_modules {
            match previous_modules.get(module_id) {
                Some(previous_module) if *previous_module == *current_module => {}
                _ => {
                    affected_module_ids.insert(module_id.clone());
                }
            }
        }
    }

    for page in previous_state.pages.iter().filter(|page| {
        page.source_paths
            .iter()
            .any(|path| dirty_source_paths.contains(path))
    }) {
        affected_page_ids.insert(page.page_id.clone());
        affected_module_ids.extend(page.module_ids.iter().cloned());
    }

    if change_set.requires_replan {
        let previous_page_ids = previous_pages.keys().cloned().collect::<BTreeSet<_>>();
        let current_page_ids = current_pages.keys().cloned().collect::<BTreeSet<_>>();

        for page_id in previous_page_ids.difference(&current_page_ids) {
            removed_page_ids.insert(page_id.clone());
            if let Some(parent_id) = previous_pages
                .get(page_id)
                .and_then(|page| page.parent_id.clone())
            {
                affected_page_ids.insert(parent_id);
            }
        }

        for page_id in current_page_ids.difference(&previous_page_ids) {
            affected_page_ids.insert(page_id.clone());
        }

        for planned_page in planned_pages {
            if matches!(planned_page.page_type.as_str(), "overview" | "architecture") {
                affected_page_ids.insert(planned_page.id.clone());
            }

            if let Some(previous_page) = previous_pages.get(&planned_page.id) {
                if page_plan_changed(previous_page, planned_page) {
                    affected_page_ids.insert(planned_page.id.clone());
                }
            }

            if planned_page
                .module_ids
                .iter()
                .any(|module_id| affected_module_ids.contains(module_id))
            {
                affected_page_ids.insert(planned_page.id.clone());
            }
        }
    } else if !affected_module_ids.is_empty() {
        for page in &previous_state.pages {
            if page
                .module_ids
                .iter()
                .any(|module_id| affected_module_ids.contains(module_id))
            {
                affected_page_ids.insert(page.page_id.clone());
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

fn page_plan_changed(previous_page: &WikiPageState, planned_page: &PlannedPage) -> bool {
    previous_page.title != planned_page.title
        || previous_page.path != format!(".wiki/{}", planned_page.relative_path)
        || previous_page.page_type != planned_page.page_type
        || previous_page.parent_id != planned_page.parent_id
        || previous_page.source_ids != planned_page.source_ids
        || previous_page.module_ids != planned_page.module_ids
}

fn predicted_section_ids_for_page(page: &PlannedPage) -> Vec<String> {
    section_titles_for_page_type(&page.page_type)
        .into_iter()
        .map(|title| section_id_for_title(&page.id, title))
        .collect()
}

fn expand_affected_page_ancestors(
    previous_pages: &BTreeMap<String, &WikiPageState>,
    current_pages: &BTreeMap<String, &PlannedPage>,
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












