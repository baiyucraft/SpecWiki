//! 9.4 之后，页面规划的验收重点转为 Knowledge Planning 层。
//! 这组测试覆盖 storybook / dagger archetype 的知识域发现与知识单元规划。

use std::fs;
use tempfile::TempDir;
use wiki_core::domain::knowledge::{DomainType, UnitType};
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::knowledge_planner::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::repo::symbol_graph::GraphSummary;

fn make_storybook_like_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/react/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/vue3/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/builders/vite/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src")).unwrap();
    fs::create_dir_all(repo.path().join("docs/get-started")).unwrap();
    fs::create_dir_all(repo.path().join("docs/api")).unwrap();
    fs::create_dir_all(repo.path().join("docs/configure")).unwrap();
    fs::create_dir_all(repo.path().join("themes/default")).unwrap();
    fs::create_dir_all(repo.path().join("tests/e2e")).unwrap();
    fs::write(repo.path().join("package.json"), r#"{"name":"storybook-like","private":true}"#).unwrap();
    fs::write(repo.path().join("code/addons/a11y/src/index.ts"), "export const addon = true;
").unwrap();
    fs::write(repo.path().join("code/frameworks/react/src/index.ts"), "export const reactRenderer = true;
").unwrap();
    fs::write(repo.path().join("code/frameworks/vue3/src/index.ts"), "export const vueRenderer = true;
").unwrap();
    fs::write(repo.path().join("code/builders/vite/src/index.ts"), "export const viteBuilder = true;
").unwrap();
    fs::write(repo.path().join("code/core/src/public-types.ts"), "export type StorybookApi = { run(): void };
").unwrap();
    fs::write(repo.path().join("code/core/src/main.ts"), "export const main = {};
").unwrap();
    fs::write(repo.path().join("themes/default/theme.css"), ":root { color: red; }
").unwrap();
    fs::write(repo.path().join("docs/get-started/index.md"), "# Get Started
").unwrap();
    fs::write(repo.path().join("docs/api/index.md"), "# API
").unwrap();
    fs::write(repo.path().join("docs/configure/index.md"), "# Configure
").unwrap();
    fs::write(repo.path().join("tests/e2e/smoke.test.ts"), "test('smoke', () => {});
").unwrap();
    repo
}

fn make_dagger_like_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("dagger-runtime/src/main/java/runtime")).unwrap();
    fs::create_dir_all(repo.path().join("dagger-android/src/main/java/android")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-core/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("dagger-compiler/src/main/java/compiler")).unwrap();
    fs::create_dir_all(repo.path().join("docs/guide")).unwrap();
    fs::create_dir_all(repo.path().join("api")).unwrap();
    fs::create_dir_all(repo.path().join("tests/integration")).unwrap();
    fs::write(repo.path().join("settings.gradle"), "include ':dagger-runtime', ':dagger-android', ':hilt-core', ':dagger-compiler'
").unwrap();
    fs::write(repo.path().join("dagger-runtime/src/main/java/runtime/CoreRuntime.java"), "class CoreRuntime {}
").unwrap();
    fs::write(repo.path().join("dagger-android/src/main/java/android/AndroidBinding.java"), "class AndroidBinding {}
").unwrap();
    fs::write(repo.path().join("hilt-core/src/main/java/hilt/HiltEntry.java"), "class HiltEntry {}
").unwrap();
    fs::write(repo.path().join("dagger-compiler/src/main/java/compiler/Codegen.java"), "class Codegen {}
").unwrap();
    fs::write(repo.path().join("api/PublicApi.java"), "public class PublicApi {}
").unwrap();
    fs::write(repo.path().join("docs/guide/index.md"), "# Concepts
").unwrap();
    fs::write(repo.path().join("tests/integration/runtime_test.java"), "class RuntimeTest {}
").unwrap();
    repo
}

fn discovered_domain_types(repo: &TempDir) -> Vec<DomainType> {
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let domains = discover_knowledge_domains(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &GraphSummary::default(),
        &SteeringConfig::default(),
    );
    domains.into_iter().map(|domain| domain.domain_type).collect()
}

#[test]
fn storybook_archetype_discovers_expected_domains() {
    let repo = make_storybook_like_repo();
    let domains = discovered_domain_types(&repo);

    assert!(domains.contains(&DomainType::PluginEcosystem));
    assert!(domains.contains(&DomainType::MultiFramework));
    assert!(domains.contains(&DomainType::BuildSystem));
    assert!(domains.contains(&DomainType::TestingInfra));
    assert!(domains.contains(&DomainType::ApiReference));
    assert!(domains.contains(&DomainType::ConfigReference));
    assert!(domains.contains(&DomainType::ConceptGuide));
    assert!(domains.contains(&DomainType::ThemeSystem));
}

#[test]
fn dagger_archetype_discovers_expected_domains() {
    let repo = make_dagger_like_repo();
    let domains = discovered_domain_types(&repo);

    assert!(domains.contains(&DomainType::CoreRuntime));
    assert!(domains.contains(&DomainType::Framework));
    assert!(domains.contains(&DomainType::PlatformBinding));
    assert!(domains.contains(&DomainType::CompilerToolchain));
    assert!(domains.contains(&DomainType::TestingInfra));
    assert!(domains.contains(&DomainType::ApiReference));
    assert!(domains.contains(&DomainType::ConceptGuide));
}

#[test]
fn knowledge_units_include_domain_index_and_leaf_units() {
    let repo = make_storybook_like_repo();
    let report = scan_repo(repo.path(), &[]).unwrap();
    let tree = build_module_tree(&report);
    let repo_ctx = build_repo_context(&report, &tree);
    let mod_ctxs = build_module_contexts(&report, &tree);
    let domains = discover_knowledge_domains(
        &report,
        &tree,
        &repo_ctx,
        &mod_ctxs,
        &GraphSummary::default(),
        &SteeringConfig::default(),
    );
    let units = plan_knowledge_units(&domains, &tree, &report, &mod_ctxs, &SteeringConfig::default());
    let knowledge_tree = build_knowledge_tree(domains, units);

    assert!(knowledge_tree.units.values().any(|unit| unit.unit_type == UnitType::DomainIndex));
    assert!(knowledge_tree.units.values().any(|unit| matches!(unit.unit_type, UnitType::ApiDoc | UnitType::ConceptGuide | UnitType::ModuleDoc)));
    assert!(!knowledge_tree.processing_order.is_empty());
}
