//! Steering 配置集成测试。
//! 验证 `scan.ignore/include`、legacy ignore 兼容读取、模块提升/降级和合并阈值。

use std::fs;

use tempfile::TempDir;

use wiki_core::domain::steering::load_steering_config;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::{scan_repo, scan_repo_with_boundary};

fn make_repo_with_steering(yaml: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::write(dir.path().join("src/app.ts"), "export function app() {}\n").unwrap();
    fs::write(dir.path().join("src/utils.ts"), "export function u() {}\n").unwrap();
    fs::write(dir.path().join("src/types.ts"), "export type T = string;\n").unwrap();

    fs::create_dir_all(dir.path().join("docs")).unwrap();
    fs::write(dir.path().join("docs/guide.md"), "# Guide\n").unwrap();
    fs::write(dir.path().join("docs/keep.md"), "# Keep\n").unwrap();
    fs::create_dir_all(dir.path().join("benchmarks")).unwrap();
    fs::write(dir.path().join("benchmarks/bench.ts"), "// bench\n").unwrap();

    fs::create_dir_all(dir.path().join("scripts")).unwrap();
    fs::write(dir.path().join("scripts/deploy.sh"), "#!/bin/bash\n").unwrap();

    fs::create_dir_all(dir.path().join(".wiki")).unwrap();
    fs::write(dir.path().join(".wiki/wiki.steering.yaml"), yaml).unwrap();

    dir
}

#[test]
fn scan_ignore_excludes_directories() {
    let repo = make_repo_with_steering(
        r#"
version: 1
scan:
  ignore:
    - "benchmarks/**"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        !report
            .files
            .iter()
            .any(|f| f.path.starts_with("benchmarks/")),
        "benchmarks/ should be excluded by scan.ignore"
    );
    assert!(
        report.files.iter().any(|f| f.path.starts_with("src/")),
        "src/ should not be excluded"
    );
}

#[test]
fn scan_include_restores_ignored_paths() {
    let repo = make_repo_with_steering(
        r#"
version: 1
scan:
  ignore:
    - "docs/**"
  include:
    - "docs/keep.md"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        report.files.iter().any(|f| f.path == "docs/keep.md"),
        "scan.include should restore docs/keep.md"
    );
    assert!(
        !report.files.iter().any(|f| f.path == "docs/guide.md"),
        "docs/guide.md should remain excluded"
    );
}

#[test]
fn legacy_ignore_schema_is_still_respected() {
    let repo = make_repo_with_steering(
        r#"
version: 1
ignore:
  global:
    - "docs/**"
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();

    assert!(
        !report.files.iter().any(|f| f.path.starts_with("docs/")),
        "legacy ignore.global should be normalized into scan.ignore"
    );
}

#[test]
fn builtin_ignore_rules_still_apply() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name":"test","version":"1.0.0"}"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;\n").unwrap();
    fs::create_dir_all(dir.path().join(".next/cache")).unwrap();
    fs::write(dir.path().join(".next/cache/data.json"), "{}").unwrap();
    fs::create_dir_all(dir.path().join("__pycache__")).unwrap();
    fs::write(dir.path().join("__pycache__/mod.pyc"), "bytes").unwrap();

    let report = scan_repo(dir.path(), &[]).unwrap();

    assert!(
        !report.files.iter().any(|f| f.path.contains(".next/")),
        ".next/ should be excluded by builtin rules"
    );
    assert!(
        !report.files.iter().any(|f| f.path.contains("__pycache__/")),
        "__pycache__/ should be excluded by builtin rules"
    );
}

#[test]
fn steering_promote_keeps_small_module_page() {
    let repo = make_repo_with_steering(
        r#"
version: 1
modules:
  promote:
    - path: "scripts"
      reason: "keep scripts visible"
merge_threshold: 3
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let has_scripts_page = pages
        .iter()
        .any(|p| p.page_type == "module" && p.relative_path.contains("scripts"));
    let scripts_is_module = tree
        .modules
        .iter()
        .any(|m| m.root_paths.iter().any(|rp| rp == "scripts"));

    if scripts_is_module {
        assert!(
            has_scripts_page,
            "promoted scripts should have independent page"
        );
    }
}

#[test]
fn steering_demote_merges_module() {
    let repo = make_repo_with_steering(
        r#"
version: 1
modules:
  demote:
    - path: "src"
      reason: "merge into parent"
merge_threshold: 100
"#,
    );

    let steering = load_steering_config(repo.path());
    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let has_src_page = pages.iter().any(|p| {
        p.page_type == "module"
            && p.module_ids.iter().any(|id| {
                tree.module_by_id(id)
                    .map(|m| m.root_paths.iter().any(|rp| rp == "src"))
                    .unwrap_or(false)
            })
    });
    let src_is_module = tree
        .modules
        .iter()
        .any(|m| m.root_paths.iter().any(|rp| rp == "src") && m.parent_id.is_some());

    if src_is_module {
        assert!(
            !has_src_page,
            "demoted src should not have independent page"
        );
    }
}

#[test]
fn merge_threshold_controls_merge_behavior() {
    let repo = make_repo_with_steering(
        r#"
version: 1
merge_threshold: 100
"#,
    );

    let steering = load_steering_config(repo.path());
    assert_eq!(steering.merge_threshold, 100);

    let (ignore_paths, include_paths) = steering.scan_boundary();
    let report = scan_repo_with_boundary(repo.path(), ignore_paths, include_paths).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &steering);

    let module_pages: Vec<_> = pages.iter().filter(|p| p.page_type == "module").collect();
    let default_steering = wiki_core::domain::steering::SteeringConfig::default();
    let default_pages = plan_pages(&report, &tree, &repo_ctx, &mod_ctxs, &default_steering);
    let default_module_pages: Vec<_> = default_pages
        .iter()
        .filter(|p| p.page_type == "module")
        .collect();

    assert!(
        module_pages.len() <= default_module_pages.len(),
        "high threshold should produce fewer or equal module pages: {} vs {}",
        module_pages.len(),
        default_module_pages.len()
    );
}
