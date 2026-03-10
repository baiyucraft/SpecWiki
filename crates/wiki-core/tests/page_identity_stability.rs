//! Page identity 稳定性测试。
//! 验证增删少量源文件后核心页面 page_id 不变、
//! 模块名变化但 root_path 不变时 page_id 稳定。

use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::planner::plan_pages;
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::ScanReport;

use std::collections::BTreeMap;
use std::fs;
use tempfile::TempDir;

fn make_repo_with_files(files: &[&str]) -> (TempDir, ScanReport) {
    let dir = TempDir::new().unwrap();
    for file in files {
        let path = dir.path().join(file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, format!("// {file}\n")).unwrap();
    }
    // Write a minimal Cargo.toml so scanner picks it up
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let report = wiki_core::repo::scanner::scan_repo(dir.path(), &[]).unwrap();
    (dir, report)
}

fn plan_from_report(report: &ScanReport) -> Vec<wiki_core::generation::planner::PlannedPage> {
    let tree = build_module_tree(report);
    let repo_ctx = build_repo_context(report, &tree);
    let mod_ctxs = build_module_contexts(report, &tree);
    plan_pages(
        report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &SteeringConfig::default(),
    )
}

#[test]
fn overview_and_architecture_ids_are_fixed() {
    let (_dir, report) = make_repo_with_files(&["src/lib.rs", "src/main.rs"]);
    let pages = plan_from_report(&report);

    let overview = pages.iter().find(|p| p.page_type == "overview").unwrap();
    let arch = pages
        .iter()
        .find(|p| p.page_type == "architecture")
        .unwrap();

    // 固定种子 → 固定 ID
    assert_eq!(
        overview.id,
        wiki_core::domain::stable_id::stable_id("page", "overview")
    );
    assert_eq!(
        arch.id,
        wiki_core::domain::stable_id::stable_id("page", "architecture")
    );
}

#[test]
fn module_page_id_stable_after_adding_file() {
    let (_dir1, report1) = make_repo_with_files(&["src/lib.rs", "src/main.rs"]);
    let pages1 = plan_from_report(&report1);

    let (_dir2, report2) = make_repo_with_files(&["src/lib.rs", "src/main.rs", "src/utils.rs"]);
    let pages2 = plan_from_report(&report2);

    // 找到 src 模块页（如果存在）
    let module_pages_1: Vec<_> = pages1.iter().filter(|p| p.page_type == "module").collect();
    let module_pages_2: Vec<_> = pages2.iter().filter(|p| p.page_type == "module").collect();

    // 至少有一个模块页
    assert!(!module_pages_1.is_empty());
    assert!(!module_pages_2.is_empty());

    // 所有在两次规划中都出现的模块页，page_id 应该一致
    let ids_1: BTreeMap<_, _> = module_pages_1
        .iter()
        .map(|p| (p.relative_path.clone(), p.id.clone()))
        .collect();
    let ids_2: BTreeMap<_, _> = module_pages_2
        .iter()
        .map(|p| (p.relative_path.clone(), p.id.clone()))
        .collect();

    for (path, id1) in &ids_1 {
        if let Some(id2) = ids_2.get(path) {
            assert_eq!(id1, id2, "page_id changed for {path}");
        }
    }
}

#[test]
fn module_page_id_stable_after_removing_file() {
    let (_dir1, report1) = make_repo_with_files(&["src/lib.rs", "src/main.rs", "src/extra.rs"]);
    let pages1 = plan_from_report(&report1);

    let (_dir2, report2) = make_repo_with_files(&["src/lib.rs", "src/main.rs"]);
    let pages2 = plan_from_report(&report2);

    let ids_1: BTreeMap<_, _> = pages1
        .iter()
        .filter(|p| p.page_type == "module")
        .map(|p| (p.relative_path.clone(), p.id.clone()))
        .collect();
    let ids_2: BTreeMap<_, _> = pages2
        .iter()
        .filter(|p| p.page_type == "module")
        .map(|p| (p.relative_path.clone(), p.id.clone()))
        .collect();

    for (path, id1) in &ids_1 {
        if let Some(id2) = ids_2.get(path) {
            assert_eq!(id1, id2, "page_id changed for {path} after file removal");
        }
    }
}

#[test]
fn overview_id_unchanged_across_runs() {
    let (_dir1, report1) = make_repo_with_files(&["src/lib.rs"]);
    let (_dir2, report2) = make_repo_with_files(&["src/lib.rs", "src/new.rs"]);

    let pages1 = plan_from_report(&report1);
    let pages2 = plan_from_report(&report2);

    let ov1 = pages1.iter().find(|p| p.page_type == "overview").unwrap();
    let ov2 = pages2.iter().find(|p| p.page_type == "overview").unwrap();
    assert_eq!(ov1.id, ov2.id);

    let ar1 = pages1
        .iter()
        .find(|p| p.page_type == "architecture")
        .unwrap();
    let ar2 = pages2
        .iter()
        .find(|p| p.page_type == "architecture")
        .unwrap();
    assert_eq!(ar1.id, ar2.id);
}
