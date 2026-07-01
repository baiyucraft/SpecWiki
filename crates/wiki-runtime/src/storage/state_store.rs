use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use crate::domain::state::{rebuild_state_from_metadata, WikiState};
use crate::storage::metadata_store::read_metadata;
use crate::storage::sqlite::index_store::SqliteIndexStore;
use crate::storage::sqlite::runtime_store::SqliteRuntimeStore;
use crate::storage::sqlite_store;
use wiki_index::scanner::ScanReport;
use wiki_index::store::{
    FolderRecord, GraphPhaseStatus, GraphReadinessStatus, GraphSnapshot, IndexSnapshotStore,
    SourceFileRecord,
};
use wiki_index::symbol_graph::{GraphAnalysisSnapshot, GraphDiagnostic, ResolvedGraphSnapshot};
use wiki_index::symbols::{GraphPhase, ReferenceKind, SourceRange, SymbolNode, UnresolvedRef};
use wiki_model::domain::module_tree::ModuleTree;

const GRAPH_SNAPSHOT_ORIGIN_KEY: &str = "graph_snapshot_origin";
const GRAPH_SNAPSHOT_ID_KEY: &str = "graph_snapshot_id";
const GRAPH_ORIGIN_SOURCE_REBUILD: &str = "source_rebuild";
const GRAPH_ORIGIN_LEVEL1_RESTORED_MIRROR: &str = "level1_restored_mirror";

/// 写入 facts snapshot 到 SQLite index 存储。
/// 这条路径只负责 facts/index substrate，不携带 `WikiState` 或页面层写入。
pub fn write_facts_snapshot(
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.write_scan_report(scan_report)?;
    index_store.write_module_tree(module_tree)?;
    let graph_snapshot = build_graph_snapshot(scan_report, symbols, resolved_graph, analysis);
    index_store.replace_graph_snapshot(&graph_snapshot)?;
    mark_source_graph_snapshot(repo_root)
}

/// 写入 scan/module snapshot，并按文件刷新 symbols / edges。
/// scoped update 仍然整体重算 graph-derived 结果，但 symbol/edge 按工作集覆盖。
pub fn write_facts_snapshot_for_files(
    repo_root: &Path,
    scan_report: &ScanReport,
    module_tree: &ModuleTree,
    file_paths: &[String],
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> io::Result<()> {
    let index_store = SqliteIndexStore::new(repo_root);
    index_store.write_scan_report(scan_report)?;
    index_store.write_module_tree(module_tree)?;
    let graph_snapshot = build_graph_snapshot(scan_report, symbols, resolved_graph, analysis);
    index_store.replace_graph_snapshot_for_files(file_paths, &graph_snapshot)?;
    mark_source_graph_snapshot(repo_root)
}

fn build_graph_snapshot(
    scan_report: &ScanReport,
    symbols: &[SymbolNode],
    resolved_graph: &ResolvedGraphSnapshot,
    analysis: &GraphAnalysisSnapshot,
) -> GraphSnapshot {
    let source_fingerprint = scan_report
        .files
        .iter()
        .map(|file| format!("{}:{}", file.path, file.fingerprint))
        .collect::<Vec<_>>()
        .join("|");
    let snapshot_id =
        wiki_model::domain::stable_id::stable_id("graph-snapshot", &source_fingerprint);
    let files = scan_report
        .files
        .iter()
        .filter(|file| file.kind == "source")
        .map(|file| SourceFileRecord {
            file_id: file.id.clone(),
            path: file.path.clone(),
            language: file.language.clone(),
            kind: file.kind.clone(),
            fingerprint: file.fingerprint.clone(),
            size: file.size as u64,
            indexed_at: "source-snapshot".to_string(),
            diagnostics: Vec::new(),
        })
        .collect::<Vec<_>>();
    let mut folder_paths = BTreeSet::new();
    for file in &files {
        let mut path = std::path::Path::new(&file.path);
        while let Some(parent) = path.parent() {
            let parent_text = parent.to_string_lossy().replace('\\', "/");
            if parent_text.is_empty() {
                break;
            }
            folder_paths.insert(parent_text);
            path = parent;
        }
    }
    let folders = folder_paths
        .into_iter()
        .map(|path| FolderRecord {
            folder_id: wiki_model::domain::stable_id::stable_id("folder", &path),
            parent_id: std::path::Path::new(&path)
                .parent()
                .map(|parent| parent.to_string_lossy().replace('\\', "/"))
                .filter(|parent| !parent.is_empty())
                .map(|parent| wiki_model::domain::stable_id::stable_id("folder", parent)),
            path,
        })
        .collect::<Vec<_>>();
    let unresolved_refs = resolved_graph
        .diagnostics
        .iter()
        .filter_map(diagnostic_to_unresolved_ref)
        .collect::<Vec<_>>();
    GraphSnapshot {
        snapshot_id,
        source_fingerprint,
        files,
        folders,
        symbols: symbols.to_vec(),
        edges: resolved_graph.edges.clone(),
        raw_imports: Vec::new(),
        raw_calls: Vec::new(),
        raw_heritage: Vec::new(),
        unresolved_refs,
        analysis: analysis.clone(),
        phase_statuses: vec![
            phase_status(GraphPhase::Scan),
            phase_status(GraphPhase::Structure),
            phase_status(GraphPhase::Parse),
            phase_status(GraphPhase::ResolveImports),
            phase_status(GraphPhase::ResolveCalls),
            phase_status(GraphPhase::ResolveHeritage),
            phase_status(GraphPhase::AnalyzeCommunities),
            phase_status(GraphPhase::AnalyzeProcesses),
            phase_status(GraphPhase::BuildFts),
        ],
    }
}

fn phase_status(phase: GraphPhase) -> GraphPhaseStatus {
    GraphPhaseStatus {
        phase,
        status: "ready".to_string(),
        input_fingerprint: None,
        output_fingerprint: None,
        started_at: None,
        completed_at: None,
        diagnostics: Vec::new(),
    }
}

fn diagnostic_to_unresolved_ref(diagnostic: &GraphDiagnostic) -> Option<UnresolvedRef> {
    let file_path = diagnostic.file_path.clone()?;
    let resolver_phase = match diagnostic.stage.as_str() {
        "resolve_imports" => GraphPhase::ResolveImports,
        "resolve_calls" => GraphPhase::ResolveCalls,
        "resolve_heritage" => GraphPhase::ResolveHeritage,
        _ => return None,
    };
    let reference_kind = match resolver_phase {
        GraphPhase::ResolveImports => ReferenceKind::Import,
        GraphPhase::ResolveCalls => ReferenceKind::Call,
        GraphPhase::ResolveHeritage => ReferenceKind::Heritage,
        _ => ReferenceKind::Import,
    };
    let reference_name = diagnostic
        .message
        .split(" -> ")
        .next()
        .and_then(|prefix| prefix.rsplit(':').next())
        .filter(|value| !value.is_empty())
        .map(normalize_reference_name)
        .unwrap_or_else(|| diagnostic.kind.clone());
    let file_id = format!("file:{file_path}");
    Some(UnresolvedRef {
        unresolved_ref_id: wiki_model::domain::stable_id::stable_id(
            "unresolved",
            format!("{}:{}:{}", diagnostic.stage, file_path, diagnostic.message),
        ),
        capture_id: wiki_model::domain::stable_id::stable_id(
            "capture",
            format!("{}:{}:{}", diagnostic.stage, file_path, diagnostic.message),
        ),
        file_id: file_id.clone(),
        resolver_phase,
        reference_kind,
        reference_name,
        target_hint: None,
        range: SourceRange::new(file_id, file_path, 0, 0, 0, 0),
        candidates: Vec::new(),
        reason: diagnostic.kind.clone(),
        diagnostics: vec![diagnostic.message.clone()],
    })
}

fn normalize_reference_name(value: &str) -> String {
    let normalized = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .replace('\\', "/");
    let trimmed = normalized
        .trim_start_matches("./")
        .trim_start_matches("../");
    let leaf = trimmed.rsplit('/').next().unwrap_or(trimmed);
    leaf.rsplit_once('.')
        .map(|(stem, _)| stem)
        .filter(|stem| !stem.is_empty())
        .unwrap_or(leaf)
        .to_string()
}

/// runtime mirror 是否已经达到可诊断/恢复消费的最小条件。
pub fn runtime_mirror_ready(repo_root: &Path) -> io::Result<bool> {
    if !sqlite_store::db_exists(repo_root) {
        return Ok(false);
    }

    let index_store = SqliteIndexStore::new(repo_root);
    let Some(scan_report) = index_store.read_scan_report()? else {
        return Ok(false);
    };
    let Some(module_tree) = index_store.read_module_tree()? else {
        return Ok(false);
    };

    let sources = index_store.list_sources()?;
    let modules = index_store.list_modules()?;
    Ok((scan_report.files.is_empty() || !sources.is_empty())
        && (module_tree.modules.is_empty() || !modules.is_empty()))
}

/// index graph 是否来自当前源码 build/update/rebuild，而不是 Level 1 restore mirror。
pub fn index_graph_ready(repo_root: &Path) -> io::Result<bool> {
    Ok(matches!(
        sqlite_store::read_graph_readiness(repo_root)?.status,
        GraphReadinessStatus::Ready
    ))
}

/// 旧内部判断名保留为 mirror ready，避免内部 cache 恢复路径被迫一次性重写。
pub fn facts_snapshot_ready(repo_root: &Path) -> io::Result<bool> {
    runtime_mirror_ready(repo_root)
}

fn mark_source_graph_snapshot(repo_root: &Path) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    runtime_store.runtime_meta_set(GRAPH_SNAPSHOT_ORIGIN_KEY, GRAPH_ORIGIN_SOURCE_REBUILD)?;
    runtime_store.runtime_meta_set(GRAPH_SNAPSHOT_ID_KEY, GRAPH_ORIGIN_SOURCE_REBUILD)
}

pub fn mark_level1_restored_mirror(repo_root: &Path, snapshot_id: &str) -> io::Result<()> {
    let conn = sqlite_store::open_db(repo_root)?;
    let runtime_store = SqliteRuntimeStore::new(&conn);
    runtime_store.runtime_meta_set(
        GRAPH_SNAPSHOT_ORIGIN_KEY,
        GRAPH_ORIGIN_LEVEL1_RESTORED_MIRROR,
    )?;
    runtime_store.runtime_meta_set(GRAPH_SNAPSHOT_ID_KEY, snapshot_id)
}

/// 写入 WikiState 到 SQLite 关系型状态表。
pub fn write_state(repo_root: &Path, state: &WikiState) -> io::Result<()> {
    let mut conn = sqlite_store::open_db(repo_root)?;
    sqlite_store::replace_state_rows(&mut conn, state)
}

/// 从 SQLite 关系型状态表读取 WikiState。
pub fn read_state(repo_root: &Path) -> io::Result<WikiState> {
    let index_store = SqliteIndexStore::new(repo_root);
    let module_tree = index_store.read_module_tree()?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "module-tree not found in sqlite index snapshot",
        )
    })?;
    let conn = sqlite_store::open_db_readonly(repo_root)?;
    sqlite_store::load_state_rows(&conn, &module_tree)
}

/// 优先从 SQLite 读 WikiState，不存在时从 WikiMetadata 重建，都不存在时返回错误。
/// DB 损坏时输出 warning 并回退到 metadata 重建。
pub fn load_or_rebuild_state(repo_root: &Path) -> io::Result<WikiState> {
    match read_state(repo_root) {
        Ok(state) => return Ok(state),
        Err(e) => {
            if sqlite_store::db_exists(repo_root) {
                eprintln!("[warn] failed to read WikiState from DB, falling back to metadata: {e}");
            }
        }
    }

    let metadata = read_metadata(repo_root)?;
    Ok(rebuild_state_from_metadata(&metadata))
}
