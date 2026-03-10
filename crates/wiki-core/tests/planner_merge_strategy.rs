//! Planner 合并策略和父子关系单元测试。
//! 覆盖小模块合并、有子模块不合并、steering promote/demote 覆盖、嵌套父子关系分配。

use wiki_core::domain::context::{ModuleContext, RepoContext};
use wiki_core::domain::module_tree::{ModuleNode, ModuleTree};
use wiki_core::domain::stable_id::stable_id;
use wiki_core::domain::steering::{ModuleOverride, SteeringConfig};
use wiki_core::generation::planner::{module_page_weight, plan_pages};
use wiki_core::repo::scanner::{FilePurpose, ScanReport, ScannedFile};

/// 构造一个带有多层模块的合成仓库，用于测试合并和父子关系。
///
/// 模块结构：
/// - root (.)
///   - big (5 source files, workspace member) → 应保留独立页面
///   - small (2 source files, no children) → 应被合并
///   - group (has children, 1 source file)
///     - group/child-a (4 source files) → 应保留独立页面
///     - group/child-b (1 source file, no children) → 应被合并到 group
fn make_test_data() -> (ScanReport, ModuleTree, RepoContext, Vec<ModuleContext>) {
    let root_id = stable_id("module", ".");
    let big_id = stable_id("module", "big");
    let small_id = stable_id("module", "small");
    let group_id = stable_id("module", "group");
    let child_a_id = stable_id("module", "group/child-a");
    let child_b_id = stable_id("module", "group/child-b");

    let mut files = Vec::new();
    let mut file_counter = 0u32;

    let mut make_source = |dir: &str, count: usize| -> Vec<String> {
        let mut ids = Vec::new();
        for i in 0..count {
            let path = format!("{dir}/src_{i}.rs");
            let id = stable_id("file", &path);
            files.push(ScannedFile {
                id: id.clone(),
                path,
                language: "rust".to_string(),
                kind: "source".to_string(),
                purpose: FilePurpose::Library,
                fingerprint: format!("fp{file_counter}"),
                size: 100,
                tags: vec![],
            });
            ids.push(id);
            file_counter += 1;
        }
        ids
    };

    let big_source_ids = make_source("big", 5);
    let small_source_ids = make_source("small", 2);
    let group_source_ids = make_source("group", 1);
    let child_a_source_ids = make_source("group/child-a", 4);
    let child_b_source_ids = make_source("group/child-b", 1);

    let all_source_ids: Vec<String> = files.iter().map(|f| f.id.clone()).collect();

    let report = ScanReport {
        root: "/test-repo".to_string(),
        files,
        tech_hints: vec!["rust".to_string()],
        workspace_roots: vec![".".to_string(), "big".to_string()],
        config_files: vec![],
        entry_points: vec!["big/src_0.rs".to_string()],
        dependency_hints: vec![],
    };

    let root_module = ModuleNode {
        id: root_id.clone(),
        name: "test-repo".to_string(),
        kind: "repository".to_string(),
        root_paths: vec![".".to_string()],
        source_ids: all_source_ids,
        parent_id: None,
        child_ids: vec![big_id.clone(), small_id.clone(), group_id.clone()],
        entry_points: vec!["big/src_0.rs".to_string()],
        tags: vec!["rust".to_string()],
    };

    let big_module = ModuleNode {
        id: big_id.clone(),
        name: "big".to_string(),
        kind: "library".to_string(),
        root_paths: vec!["big".to_string()],
        source_ids: big_source_ids,
        parent_id: Some(root_id.clone()),
        child_ids: vec![],
        entry_points: vec!["big/src_0.rs".to_string()],
        tags: vec!["rust".to_string()],
    };

    let small_module = ModuleNode {
        id: small_id.clone(),
        name: "small".to_string(),
        kind: "module".to_string(),
        root_paths: vec!["small".to_string()],
        source_ids: small_source_ids,
        parent_id: Some(root_id.clone()),
        child_ids: vec![],
        entry_points: vec![],
        tags: vec![],
    };

    let group_module = ModuleNode {
        id: group_id.clone(),
        name: "group".to_string(),
        kind: "module-group".to_string(),
        root_paths: vec!["group".to_string()],
        source_ids: group_source_ids,
        parent_id: Some(root_id.clone()),
        child_ids: vec![child_a_id.clone(), child_b_id.clone()],
        entry_points: vec![],
        tags: vec![],
    };

    let child_a_module = ModuleNode {
        id: child_a_id.clone(),
        name: "child-a".to_string(),
        kind: "library".to_string(),
        root_paths: vec!["group/child-a".to_string()],
        source_ids: child_a_source_ids,
        parent_id: Some(group_id.clone()),
        child_ids: vec![],
        entry_points: vec![],
        tags: vec!["rust".to_string()],
    };

    let child_b_module = ModuleNode {
        id: child_b_id.clone(),
        name: "child-b".to_string(),
        kind: "module".to_string(),
        root_paths: vec!["group/child-b".to_string()],
        source_ids: child_b_source_ids,
        parent_id: Some(group_id.clone()),
        child_ids: vec![],
        entry_points: vec![],
        tags: vec![],
    };

    let module_tree = ModuleTree {
        root_modules: vec![root_id.clone()],
        modules: vec![
            root_module,
            big_module,
            small_module,
            group_module,
            child_a_module,
            child_b_module,
        ],
        cross_module_edges: vec![],
        architecture_hints: vec![],
    };

    let repo_context = RepoContext {
        repo_summary_inputs: vec![],
        top_modules: vec![big_id.clone(), small_id.clone(), group_id.clone()],
        key_entry_points: vec![],
        global_relations: vec![],
        tech_stack: vec!["rust".to_string()],
    };

    let module_contexts = vec![
        ModuleContext {
            module_id: big_id,
            role_hints: vec!["library".to_string()],
            public_surface: vec![],
            dependencies: vec![],
            dependents: vec![],
            key_sources: vec![],
        },
        ModuleContext {
            module_id: small_id,
            role_hints: vec![],
            public_surface: vec![],
            dependencies: vec![],
            dependents: vec![],
            key_sources: vec![],
        },
        ModuleContext {
            module_id: group_id,
            role_hints: vec![],
            public_surface: vec![],
            dependencies: vec![],
            dependents: vec![],
            key_sources: vec![],
        },
        ModuleContext {
            module_id: child_a_id,
            role_hints: vec!["library".to_string()],
            public_surface: vec![],
            dependencies: vec![],
            dependents: vec![],
            key_sources: vec![],
        },
        ModuleContext {
            module_id: child_b_id,
            role_hints: vec![],
            public_surface: vec![],
            dependencies: vec![],
            dependents: vec![],
            key_sources: vec![],
        },
    ];

    (report, module_tree, repo_context, module_contexts)
}

#[test]
fn small_module_merged_when_below_threshold() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default(); // merge_threshold = 3
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    // "small" has 2 source files, no children → should be merged
    assert!(
        !pages.iter().any(|p| p.page_type == "module"
            && p.module_ids
                .iter()
                .any(|id| *id == stable_id("module", "small"))),
        "small module should NOT have an independent page"
    );

    // "small" should appear in some page's merged_module_ids
    let small_id = stable_id("module", "small");
    assert!(
        pages
            .iter()
            .any(|p| p.merged_module_ids.contains(&small_id)),
        "small module should be listed in a parent page's merged_module_ids"
    );
}

#[test]
fn big_module_keeps_independent_page() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default();
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let big_id = stable_id("module", "big");
    assert!(
        pages
            .iter()
            .any(|p| p.page_type == "module" && p.module_ids.contains(&big_id)),
        "big module should have an independent page"
    );
}

#[test]
fn module_with_children_not_merged_even_if_low_source_count() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default();
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    // "group" has only 1 source file but has children → should NOT be merged
    let group_id = stable_id("module", "group");
    assert!(
        pages
            .iter()
            .any(|p| p.page_type == "module" && p.module_ids.contains(&group_id)),
        "group module with children should keep independent page"
    );
}

#[test]
fn child_b_merged_into_group_page() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default();
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let group_id = stable_id("module", "group");
    let child_b_id = stable_id("module", "group/child-b");

    let group_page = pages
        .iter()
        .find(|p| p.page_type == "module" && p.module_ids.contains(&group_id))
        .expect("group page should exist");

    assert!(
        group_page.merged_module_ids.contains(&child_b_id),
        "child-b should be merged into group page"
    );
}

#[test]
fn steering_promote_overrides_merge() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let mut steering = SteeringConfig::default();
    steering.modules.promote.push(ModuleOverride {
        path: "small".to_string(),
        reason: "force keep".to_string(),
    });

    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let small_id = stable_id("module", "small");
    assert!(
        pages
            .iter()
            .any(|p| p.page_type == "module" && p.module_ids.contains(&small_id)),
        "promoted small module should have an independent page"
    );
}

#[test]
fn steering_demote_forces_merge() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let mut steering = SteeringConfig::default();
    steering.modules.demote.push(ModuleOverride {
        path: "big".to_string(),
        reason: "force merge".to_string(),
    });

    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let big_id = stable_id("module", "big");
    assert!(
        !pages
            .iter()
            .any(|p| p.page_type == "module" && p.module_ids.contains(&big_id)),
        "demoted big module should NOT have an independent page"
    );
    assert!(
        pages.iter().any(|p| p.merged_module_ids.contains(&big_id)),
        "demoted big module should appear in merged_module_ids"
    );
}

#[test]
fn nested_parent_child_follows_module_tree() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default();
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let group_id = stable_id("module", "group");
    let child_a_id = stable_id("module", "group/child-a");

    let group_page = pages
        .iter()
        .find(|p| p.page_type == "module" && p.module_ids.contains(&group_id))
        .expect("group page should exist");

    let child_a_page = pages
        .iter()
        .find(|p| p.page_type == "module" && p.module_ids.contains(&child_a_id))
        .expect("child-a page should exist");

    // child-a's parent_id should point to group's page_id
    assert_eq!(
        child_a_page.parent_id.as_deref(),
        Some(group_page.id.as_str()),
        "child-a page parent should be group page"
    );
}

#[test]
fn top_level_module_parent_is_overview() {
    let (report, tree, repo_ctx, mod_ctxs) = make_test_data();
    let steering = SteeringConfig::default();
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let overview_id = stable_id("page", "overview");
    let big_id = stable_id("module", "big");

    let big_page = pages
        .iter()
        .find(|p| p.page_type == "module" && p.module_ids.contains(&big_id))
        .expect("big page should exist");

    assert_eq!(
        big_page.parent_id.as_deref(),
        Some(overview_id.as_str()),
        "top-level module page parent should be overview"
    );
}

#[test]
fn page_weight_reflects_module_signals() {
    let (report, tree, _, _) = make_test_data();

    let big = tree.module_by_id(&stable_id("module", "big")).unwrap();
    let small = tree.module_by_id(&stable_id("module", "small")).unwrap();
    let group = tree.module_by_id(&stable_id("module", "group")).unwrap();

    let big_weight = module_page_weight(big, &report);
    let small_weight = module_page_weight(small, &report);
    let group_weight = module_page_weight(group, &report);

    // big: 5 source + 2 workspace + 2 entry + 1 kind = 10
    assert!(
        big_weight >= 8,
        "big weight should be high, got {big_weight}"
    );
    // small: 2 source = 2
    assert!(
        small_weight < 3,
        "small weight should be below threshold, got {small_weight}"
    );
    // group: 1 source + 3 children = 4
    assert!(
        group_weight >= 3,
        "group weight should be above threshold due to children, got {group_weight}"
    );
}
