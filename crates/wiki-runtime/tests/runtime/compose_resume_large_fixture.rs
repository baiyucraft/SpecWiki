use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::rc::Rc;

use tempfile::tempdir;
use wiki_index::hierarchy::build_module_tree_with_graph;
use wiki_index::scanner::{scan_repo_with_boundary, ScanReport};
use wiki_index::symbol_graph::{
    analyze_symbol_graph, build_graph_summary, resolve_symbol_graph, GraphAnalysisSnapshot,
    GraphSummary, ResolvedGraphSnapshot,
};
use wiki_index::symbols::{parse_symbols, ParsedSymbolsSnapshot};
use wiki_knowledge::domain::research::{DomainResearch, PageDigest, SystemResearch, UnitResearch};
use wiki_knowledge::research::{ResearchDataSource, ResearchProvider, StructuralResearchProvider};
use wiki_knowledge::{ModuleContext, RepoContext};
use wiki_model::domain::update_scope::AffectedKnowledgeScope;
use wiki_runtime::domain::knowledge::{
    DomainType, KnowledgeDomain, KnowledgeTree, KnowledgeUnit, UnitType,
};
use wiki_runtime::domain::module_tree::ModuleTree;
use wiki_runtime::domain::steering::{
    load_steering_config_with_mode, SteeringConfig, SteeringLoadMode,
};
use wiki_runtime::generation::context::{
    build_module_contexts_with_graph, build_repo_context_with_graph,
};
use wiki_runtime::storage::metadata_store::read_metadata;
use wiki_runtime::storage::sqlite::runtime_store::SqliteRuntimeStore;
use wiki_runtime::storage::sqlite_store;
use wiki_runtime::workflows::init::run_init_with_progress_and_llm_as_with_mode;
use wiki_runtime::workflows::page_render::{
    load_runtime_summary_for_repo, run_scoped_compose_pipeline_for_update,
};
use wiki_runtime::workflows::progress::NoopProgressSink;

struct RuntimeInputs {
    scan_report: ScanReport,
    module_tree: ModuleTree,
    repo_context: RepoContext,
    module_contexts: Vec<ModuleContext>,
    symbol_snapshot: ParsedSymbolsSnapshot,
    resolved_graph: ResolvedGraphSnapshot,
    graph_analysis: GraphAnalysisSnapshot,
    graph_summary: GraphSummary,
    steering: SteeringConfig,
    knowledge_tree: KnowledgeTree,
}

fn write_large_source_fixture(repo_root: &Path) {
    fs::write(
        repo_root.join("package.json"),
        r#"{"name":"large-resume-fixture","private":true,"workspaces":["packages/*"]}"#,
    )
    .unwrap();
    for index in 0..48 {
        let module_dir = repo_root.join(format!("packages/module-{index:02}"));
        fs::create_dir_all(&module_dir).unwrap();
        fs::write(
            module_dir.join("package.json"),
            format!(r#"{{"name":"@fixture/module-{index:02}","version":"1.0.0"}}"#),
        )
        .unwrap();
        fs::write(
            module_dir.join("index.ts"),
            format!("export function module{index:02}() {{ return {index}; }}\n"),
        )
        .unwrap();
    }
}

fn build_runtime_inputs(repo_root: &Path) -> RuntimeInputs {
    let steering = load_steering_config_with_mode(repo_root, SteeringLoadMode::Development);
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let scan_report =
        scan_repo_with_boundary(repo_root, ignore_paths, include_paths).expect("scan fixture");
    let symbol_snapshot = parse_symbols(repo_root, &scan_report).expect("parse fixture symbols");
    let resolved_graph =
        resolve_symbol_graph(repo_root, &scan_report, &symbol_snapshot).expect("resolve graph");
    let graph_analysis = analyze_symbol_graph(&symbol_snapshot, &resolved_graph);
    let graph_summary = build_graph_summary(
        &scan_report,
        &symbol_snapshot,
        &resolved_graph,
        &graph_analysis,
    );
    let module_tree = build_module_tree_with_graph(&scan_report, &graph_summary);
    let repo_context = build_repo_context_with_graph(&scan_report, &module_tree, &graph_summary);
    let module_contexts =
        build_module_contexts_with_graph(&scan_report, &module_tree, &graph_summary);
    let knowledge_tree = build_large_knowledge_tree();
    RuntimeInputs {
        scan_report,
        module_tree,
        repo_context,
        module_contexts,
        symbol_snapshot,
        resolved_graph,
        graph_analysis,
        graph_summary,
        steering,
        knowledge_tree,
    }
}

fn build_large_knowledge_tree() -> KnowledgeTree {
    let domain = KnowledgeDomain::new(DomainType::CoreRuntime, "Large Fixture");
    let mut units = (0..48)
        .map(|index| {
            let mut unit = KnowledgeUnit::new(
                UnitType::ModuleDoc,
                format!("Module {index:02}"),
                domain.id.clone(),
                format!("large-fixture/module-{index:02}.md"),
            );
            unit.priority = (48 - index) as f32;
            unit
        })
        .collect::<Vec<_>>();
    let overview_unit_id = units[0].id.clone();
    let mut tree = KnowledgeTree::new(overview_unit_id);
    tree.add_domain(domain);
    for unit in units.drain(..) {
        tree.add_unit(unit);
    }
    tree.build_processing_order();
    tree
}

struct CountingProvider {
    inner: StructuralResearchProvider,
    fail_at_unit_call: Rc<Cell<Option<usize>>>,
    total_unit_calls: Rc<Cell<usize>>,
    unit_calls: Rc<RefCell<Vec<String>>>,
}

impl CountingProvider {
    fn failing_at(unit_call: usize) -> Self {
        Self {
            inner: StructuralResearchProvider,
            fail_at_unit_call: Rc::new(Cell::new(Some(unit_call))),
            total_unit_calls: Rc::new(Cell::new(0)),
            unit_calls: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn resume(&self) {
        self.fail_at_unit_call.set(None);
    }
}

impl ResearchProvider for CountingProvider {
    fn research_system(&self, ds: &ResearchDataSource) -> io::Result<SystemResearch> {
        self.inner.research_system(ds)
    }

    fn research_domain(
        &self,
        domain: &KnowledgeDomain,
        ds: &ResearchDataSource,
    ) -> io::Result<DomainResearch> {
        self.inner.research_domain(domain, ds)
    }

    fn research_unit(
        &self,
        unit: &KnowledgeUnit,
        ds: &ResearchDataSource,
        child_digests: &[PageDigest],
    ) -> io::Result<UnitResearch> {
        let next_call = self.total_unit_calls.get() + 1;
        self.total_unit_calls.set(next_call);
        self.unit_calls.borrow_mut().push(unit.id.clone());
        if self.fail_at_unit_call.get() == Some(next_call) {
            return Err(io::Error::other(format!(
                "deterministic interruption at unit {}",
                unit.id
            )));
        }
        self.inner.research_unit(unit, ds, child_digests)
    }
}

fn run_compose(
    action: &str,
    repo_root: &Path,
    inputs: &RuntimeInputs,
    provider: &dyn ResearchProvider,
) -> io::Result<wiki_runtime::workflows::page_render::ComposePipelineOutput> {
    let affected_scope = AffectedKnowledgeScope {
        direct_unit_ids: inputs.knowledge_tree.processing_order.clone(),
        ..AffectedKnowledgeScope::default()
    };
    run_scoped_compose_pipeline_for_update(
        action,
        repo_root,
        &inputs.scan_report,
        &inputs.module_tree,
        &inputs.repo_context,
        &inputs.module_contexts,
        &inputs.symbol_snapshot,
        &inputs.resolved_graph,
        &inputs.graph_analysis,
        &inputs.graph_summary,
        &inputs.steering,
        &inputs.knowledge_tree,
        &[],
        &[],
        &affected_scope,
        &[],
        provider,
    )
}

fn assert_no_durable_session_fields(repo_root: &Path) {
    let mut pending = vec![repo_root.join(".wiki")];
    while let Some(path) = pending.pop() {
        if path.is_dir() {
            pending.extend(
                fs::read_dir(path)
                    .unwrap()
                    .map(|entry| entry.unwrap().path()),
            );
            continue;
        }
        let bytes = fs::read(&path).unwrap();
        let content = String::from_utf8_lossy(&bytes);
        for forbidden in [
            "session_summary",
            "recent_turns",
            "tool_artifact_refs",
            "durable-session-must-not-resume",
        ] {
            assert!(
                !content.contains(forbidden),
                "request-local provider state leaked into {}",
                path.display()
            );
        }
    }
}

#[test]
fn large_fixture_resumes_completed_units_once_and_clears_public_checkpoint() {
    let fixture = tempdir().unwrap();
    write_large_source_fixture(fixture.path());
    let inputs = build_runtime_inputs(fixture.path());
    let unit_count = inputs.knowledge_tree.units.len();
    assert!(
        (32..=64).contains(&unit_count),
        "fixture must plan 32-64 units, planned {unit_count}"
    );

    let provider = CountingProvider::failing_at(17);
    let first = run_compose("init", fixture.path(), &inputs, &provider);
    assert!(first.is_err());
    let interrupted_calls = provider.unit_calls.borrow().clone();
    assert_eq!(interrupted_calls.len(), 17);
    let interrupted_unit = interrupted_calls[16].clone();

    provider.resume();
    let resumed = run_compose("init", fixture.path(), &inputs, &provider).unwrap();
    assert_eq!(resumed.digests.len(), unit_count);
    let calls = provider.unit_calls.borrow();
    let counts = calls.iter().fold(BTreeMap::new(), |mut counts, unit_id| {
        *counts.entry(unit_id.clone()).or_insert(0usize) += 1;
        counts
    });
    for unit_id in interrupted_calls.iter().take(16) {
        assert_eq!(
            counts.get(unit_id),
            Some(&1),
            "completed unit was recomputed"
        );
    }
    assert_eq!(counts.get(&interrupted_unit), Some(&2));
    assert_eq!(counts.values().sum::<usize>(), unit_count + 1);
    drop(calls);

    let large_summary = load_runtime_summary_for_repo(fixture.path())
        .unwrap()
        .expect("large compose summary");
    assert_eq!(large_summary.runtime_state, "compose_complete");
    assert_eq!(large_summary.researched_units, unit_count);
    assert_eq!(large_summary.composed_units, unit_count);
    let large_conn = sqlite_store::open_db(fixture.path()).unwrap();
    let large_store = SqliteRuntimeStore::new(&large_conn);
    assert_eq!(
        large_store.read_unit_runtime_gates().unwrap().len(),
        unit_count
    );
    assert!(large_store.read_pipeline_checkpoint().unwrap().is_some());
    drop(large_conn);

    let mut sink = NoopProgressSink;
    let report = run_init_with_progress_and_llm_as_with_mode(
        "init",
        fixture.path(),
        &mut sink,
        None,
        SteeringLoadMode::Development,
    )
    .unwrap();
    assert!(report.initialized);

    let conn = sqlite_store::open_db(fixture.path()).unwrap();
    let runtime_store = SqliteRuntimeStore::new(&conn);
    assert!(runtime_store.read_pipeline_checkpoint().unwrap().is_none());
    let summary = load_runtime_summary_for_repo(fixture.path())
        .unwrap()
        .expect("completed runtime summary");
    assert_eq!(summary.runtime_state, "completed");
    assert_eq!(summary.assembled_pages, report.generated_pages.len());
    let gates = runtime_store.read_unit_runtime_gates().unwrap();
    assert_eq!(summary.researched_units, gates.len());
    assert_eq!(summary.composed_units, gates.len());
    assert!(gates.iter().all(|gate| {
        gate.research_status == "ready"
            && gate.compose_status == "done"
            && gate.assemble_status == "done"
    }));

    let metadata = read_metadata(fixture.path()).unwrap();
    let snapshot_id = metadata.current_snapshot_id.expect("formal snapshot id");
    assert!(fixture
        .path()
        .join(".wiki/.knowledge/runtime/snapshots")
        .join(snapshot_id)
        .join("manifest.yaml")
        .is_file());
    assert_no_durable_session_fields(fixture.path());
}

#[test]
fn large_fixture_rejects_completed_units_when_resume_identity_changes() {
    let fixture = tempdir().unwrap();
    write_large_source_fixture(fixture.path());
    let inputs = build_runtime_inputs(fixture.path());
    assert!((32..=64).contains(&inputs.knowledge_tree.units.len()));

    let initial = CountingProvider::failing_at(17);
    assert!(run_compose("init", fixture.path(), &inputs, &initial).is_err());
    let changed_action = CountingProvider::failing_at(usize::MAX);
    assert!(run_compose("rebuild", fixture.path(), &inputs, &changed_action).is_ok());
    let changed_calls = changed_action.unit_calls.borrow();
    assert_eq!(changed_calls.len(), inputs.knowledge_tree.units.len());
    let changed_counts = changed_calls
        .iter()
        .fold(BTreeMap::new(), |mut counts, unit_id| {
            *counts.entry(unit_id).or_insert(0usize) += 1;
            counts
        });
    assert_eq!(changed_counts.len(), inputs.knowledge_tree.units.len());
    assert!(changed_counts.values().all(|count| *count == 1));
}
