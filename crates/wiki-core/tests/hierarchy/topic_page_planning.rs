//! 9.4 之后，页面规划的验收重点转为 Knowledge Planning 层。
//! 这组测试覆盖 storybook / dagger archetype 的知识域发现与知识单元规划。

use std::collections::BTreeSet;
use std::fs;
use tempfile::TempDir;
use wiki_core::domain::knowledge::{DecompositionProfile, DomainType, UnitType};
use wiki_core::domain::steering::SteeringConfig;
use wiki_core::generation::context::{build_module_contexts, build_repo_context};
use wiki_core::generation::knowledge_planner::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use wiki_core::repo::hierarchy::build_module_tree;
use wiki_core::repo::scanner::scan_repo;
use wiki_core::repo::symbol_graph::GraphSummary;
use wiki_core::storage::sqlite_store;

fn assert_units_have_resolvable_parents(units: &[wiki_core::domain::knowledge::KnowledgeUnit]) {
    let ids: BTreeSet<&str> = units.iter().map(|unit| unit.id.as_str()).collect();
    for unit in units {
        if let Some(parent_id) = unit.parent_unit_id.as_deref() {
            assert!(
                ids.contains(parent_id),
                "unit `{}` points to missing parent `{}`; units={:?}",
                unit.title,
                parent_id,
                units
                    .iter()
                    .map(|candidate| {
                        format!(
                            "{}:{} -> {:?}",
                            candidate.title, candidate.id, candidate.parent_unit_id
                        )
                    })
                    .collect::<Vec<_>>()
            );
        }
    }
}

fn make_storybook_like_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/react/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/angular/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/vue3/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/builders/vite/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/docs/docs")).unwrap();
    fs::create_dir_all(repo.path().join("code/renderers/react")).unwrap();
    fs::create_dir_all(repo.path().join("test-storybooks/kitchen-sink/react/src")).unwrap();
    fs::create_dir_all(repo.path().join("docs/get-started")).unwrap();
    fs::create_dir_all(repo.path().join("docs/get-started/advanced")).unwrap();
    fs::create_dir_all(repo.path().join("docs/api")).unwrap();
    fs::create_dir_all(repo.path().join("docs/configure")).unwrap();
    fs::create_dir_all(repo.path().join("docs/_snippets")).unwrap();
    fs::create_dir_all(repo.path().join("docs/troubleshooting")).unwrap();
    fs::create_dir_all(repo.path().join("themes/default")).unwrap();
    fs::create_dir_all(repo.path().join("tests/e2e")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"storybook-like","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/index.ts"),
        "export const addon = true;
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/frameworks/react/src/index.ts"),
        "export const reactRenderer = true;
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/frameworks/angular/src/framework-preset-angular.ts"),
        "export const angularPreset = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/frameworks/angular/src/zone.js"),
        "export const zoneSupport = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/frameworks/vue3/src/index.ts"),
        "export const vueRenderer = true;
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/builders/vite/src/index.ts"),
        "export const viteBuilder = true;
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/public-types.ts"),
        "export type StorybookApi = { run(): void };
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/addons.ts"),
        "export const addonsApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/addon-types.ts"),
        "export type AddonTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/preview-web.ts"),
        "export const previewApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/preview.ts"),
        "export const previewAnnotations = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/store.ts"),
        "export const storeApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/story-store.ts"),
        "export const storyStoreApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/frameworks.ts"),
        "export const frameworkTypes = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/renderer-to-framework.ts"),
        "export const rendererMap = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/builders.ts"),
        "export const builderTypes = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/builder.ts"),
        "export const builderApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/story.ts"),
        "export const storyApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/csf.ts"),
        "export const csfApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/typings.d.ts"),
        "export interface StorybookTypes {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/main.ts"),
        "export const main = {};
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/ERRORS.md"),
        "# Internal Errors
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/docs/docs/docspage.md"),
        "# Docspage\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/renderers/react/README.md"),
        "# React Renderer Package\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("test-storybooks/kitchen-sink/react/src/index.ts"),
        "export const exampleApp = true;
",
    )
    .unwrap();
    fs::write(
        repo.path().join("themes/default/theme.css"),
        ":root { color: red; }
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/get-started/index.md"),
        "# Get Started
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/get-started/advanced/setup.mdx"),
        "# Setup
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/api/index.md"),
        "# API
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/api/addon-api.md"),
        "# Addon API
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/configure/index.md"),
        "# Configure
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/configure/main-js.md"),
        "# Main JS
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/_snippets/example.md"),
        "# Example Snippet
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/troubleshooting/README.md"),
        "# Troubleshooting
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/troubleshooting/common-errors.md"),
        "# Common Errors
",
    )
    .unwrap();
    fs::write(
        repo.path().join("tests/e2e/smoke.test.ts"),
        "test('smoke', () => {});
",
    )
    .unwrap();
    repo
}

fn make_dagger_like_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("dagger-runtime/src/main/java/runtime")).unwrap();
    fs::create_dir_all(repo.path().join("dagger-android/src/main/java/android")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-core/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-android/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("dagger-compiler/src/main/java/compiler")).unwrap();
    fs::create_dir_all(repo.path().join("docs/guide")).unwrap();
    fs::create_dir_all(repo.path().join("docs/examples")).unwrap();
    fs::create_dir_all(repo.path().join("docs/troubleshooting")).unwrap();
    fs::create_dir_all(repo.path().join("api")).unwrap();
    fs::create_dir_all(repo.path().join("tests/integration")).unwrap();
    fs::write(
        repo.path().join("settings.gradle"),
        "include ':dagger-runtime', ':dagger-android', ':hilt-core', ':dagger-compiler'
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/CoreRuntime.java"),
        "class CoreRuntime {}
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/MembersInjector.java"),
        "class MembersInjector {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/BindsInstanceFactory.java"),
        "class BindsInstanceFactory {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/Subcomponent.java"),
        "class Subcomponent {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/Producer.java"),
        "class Producer {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/Produces.java"),
        "class Produces {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/ProductionComponent.java"),
        "class ProductionComponent {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/ProducerMonitor.java"),
        "class ProducerMonitor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/ProducerTimingRecorder.java"),
        "class ProducerTimingRecorder {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-android/src/main/java/android/AndroidBinding.java"),
        "class AndroidBinding {}
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/HiltEntry.java"),
        "class HiltEntry {}
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/InstallIn.java"),
        "class InstallIn {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/EntryPoint.java"),
        "class EntryPoint {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/GeneratesRootInput.java"),
        "class GeneratesRootInput {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android/src/main/java/hilt/HiltAndroidApp.java"),
        "class HiltAndroidApp {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android/src/main/java/hilt/AndroidEntryPoint.java"),
        "class AndroidEntryPoint {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/Codegen.java"),
        "class Codegen {}
",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ProvidesMethodValidator.java"),
        "class ProvidesMethodValidator {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/BindsMethodValidator.java"),
        "class BindsMethodValidator {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/PublicApi.java"),
        "public class PublicApi {}
",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/addons.ts"),
        "export const addonsApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/addon-types.ts"),
        "export type AddonTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/preview-web.ts"),
        "export const previewApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/store.ts"),
        "export const storeApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/frameworks.ts"),
        "export const frameworkTypes = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/builders.ts"),
        "export const builderTypes = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/story.ts"),
        "export const storyApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/typings.d.ts"),
        "export interface StorybookTypes {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/MembersInjectorApi.java"),
        "public class MembersInjectorApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/BindsInstanceApi.java"),
        "public class BindsInstanceApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/SubcomponentApi.java"),
        "public class SubcomponentApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/ProducerApi.java"),
        "public class ProducerApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/ProducesApi.java"),
        "public class ProducesApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/ProvidesApi.java"),
        "public class ProvidesApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/BindsApi.java"),
        "public class BindsApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/TimingMonitor.java"),
        "public class TimingMonitor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("api/BenchmarkDebugApi.java"),
        "public class BenchmarkDebugApi {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/guide/index.md"),
        "# Concepts
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/examples/quick-start.md"),
        "# Quick Start
",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/troubleshooting/debugging.md"),
        "# Debugging
",
    )
    .unwrap();
    fs::write(
        repo.path().join("tests/integration/runtime_test.java"),
        "class RuntimeTest {}
",
    )
    .unwrap();
    repo
}

fn make_localized_docs_overlay_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("docs/get-started")).unwrap();
    fs::create_dir_all(repo.path().join("docs/api")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/react/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/builders/vite/src")).unwrap();
    fs::create_dir_all(repo.path().join("localized/zh/content/概念指南")).unwrap();
    fs::create_dir_all(repo.path().join("localized/zh/content/api")).unwrap();
    fs::create_dir_all(repo.path().join("localized/zh/content/配置参考")).unwrap();
    fs::create_dir_all(repo.path().join("localized/zh/content/故障排查")).unwrap();
    fs::create_dir_all(repo.path().join("localized/zh/content/项目概述")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"localized-docs-overlay","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/get-started/index.md"),
        "# Get Started\n",
    )
    .unwrap();
    fs::write(repo.path().join("docs/api/addon-api.md"), "# Addon API\n").unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/index.ts"),
        "export const addon = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/frameworks/react/src/index.ts"),
        "export const react = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/builders/vite/src/index.ts"),
        "export const vite = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("localized/zh/content/概念指南/快速开始.md"),
        "# 快速开始\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("localized/zh/content/api/插件接口.md"),
        "# 插件接口\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("localized/zh/content/配置参考/主配置.md"),
        "# 主配置\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("localized/zh/content/故障排查/常见错误.md"),
        "# 常见错误\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("localized/zh/content/项目概述/框架集成.md"),
        "# 框架集成\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("localized/zh/content/项目概述/构建流程.md"),
        "# 构建流程\n",
    )
    .unwrap();
    repo
}

fn make_hidden_generated_docs_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("docs/get-started")).unwrap();
    fs::create_dir_all(repo.path().join(".qoder/repowiki/zh/content/概念指南")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"hidden-generated-docs","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/get-started/index.md"),
        "# Get Started\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join(".qoder/repowiki/zh/content/概念指南/快速开始.md"),
        "# 快速开始\n",
    )
    .unwrap();
    repo
}

fn make_config_noise_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".storybook")).unwrap();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::create_dir_all(repo.path().join("docs/configure")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"config-noise","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join(".storybook/main.ts"),
        "export default {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".storybook/preview.ts"),
        "export const preview = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("src/preview.ts"),
        "export const preview = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("src/AboutPage.tsx"),
        "export const AboutPage = () => null;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("src/config-test.ts"),
        "export const noisy = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("src/addon-locations.png"),
        "not really a png but enough for scan\n",
    )
    .unwrap();
    fs::write(repo.path().join("docs/configure/main-js.md"), "# Main JS\n").unwrap();
    repo
}

fn make_package_readme_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("packages/core/src")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"package-readme","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("packages/core/README.md"),
        "# Core Package\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("packages/core/src/internal.md"),
        "# Internal Notes\n",
    )
    .unwrap();
    repo
}

fn make_dense_raw_config_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("buildSrc")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"dense-config","private":true}"#,
    )
    .unwrap();
    fs::write(repo.path().join(".bazelrc"), "build --config=ci\n").unwrap();
    fs::write(repo.path().join(".bazelversion"), "7.0.0\n").unwrap();
    fs::write(repo.path().join(".gitignore"), "node_modules/\n").unwrap();
    fs::write(
        repo.path().join("settings.gradle.kts"),
        "rootProject.name = \"dense\"\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("gradle.properties"),
        "org.gradle.jvmargs=-Xmx2g\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("buildSrc/build.gradle.kts"),
        "plugins { kotlin(\"jvm\") }\n",
    )
    .unwrap();
    repo
}

fn make_shadowed_docs_build_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("buildSrc")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"shadowed-docs-build","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("settings.gradle.kts"),
        "rootProject.name = \"shadowed\"\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("buildSrc/build.gradle.kts"),
        "plugins { kotlin(\"jvm\") }\n",
    )
    .unwrap();
    fs::write(repo.path().join("buildSrc/README.md"), "# BuildSrc\n").unwrap();
    repo
}

fn make_fallback_overlap_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("java/src")).unwrap();
    fs::create_dir_all(repo.path().join("javatests/src")).unwrap();
    fs::create_dir_all(repo.path().join("gradle/plugins")).unwrap();
    fs::create_dir_all(repo.path().join("examples/basic")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"fallback-overlap","private":true}"#,
    )
    .unwrap();
    fs::write(
        repo.path().join("settings.gradle.kts"),
        "rootProject.name = \"fallback-overlap\"\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("java/src/addons.ts"),
        "export const addonsApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("java/src/public-types.ts"),
        "export type PublicTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("java/src/store.ts"),
        "export const storeApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("javatests/src/runtime.test.ts"),
        "test('runtime', () => {});\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("gradle/plugins/build.gradle.kts"),
        "plugins { kotlin(\"jvm\") }\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("examples/basic/demo.ts"),
        "export const demo = true;\n",
    )
    .unwrap();
    repo
}

fn make_large_testing_signal_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("tests/unit")).unwrap();
    fs::create_dir_all(repo.path().join("tests/monitoring")).unwrap();
    fs::write(
        repo.path().join("package.json"),
        r#"{"name":"large-testing-signal","private":true}"#,
    )
    .unwrap();
    for index in 0..55 {
        fs::write(
            repo.path()
                .join(format!("tests/unit/basic_test_{index:02}.ts")),
            "test('basic', () => {});\n",
        )
        .unwrap();
    }
    fs::write(
        repo.path()
            .join("tests/monitoring/TimingProductionComponentMonitorTest.ts"),
        "test('monitor', () => {});\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("tests/monitoring/BenchmarkProfileTest.ts"),
        "test('profile', () => {});\n",
    )
    .unwrap();
    repo
}

fn make_hilt_signal_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join("hilt-core/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-compiler/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-android/src/main/java/hilt")).unwrap();
    fs::create_dir_all(repo.path().join("hilt-android-testing/src/main/java/hilt")).unwrap();
    fs::write(
        repo.path().join("settings.gradle"),
        "include ':hilt-core', ':hilt-compiler', ':hilt-android', ':hilt-android-testing'\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/GeneratesRootInput.java"),
        "class GeneratesRootInput {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/GeneratedComponent.java"),
        "class GeneratedComponent {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/GeneratedComponentManager.java"),
        "class GeneratedComponentManager {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-core/src/main/java/hilt/InstallIn.java"),
        "class InstallIn {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-compiler/src/main/java/hilt/RootProcessor.java"),
        "class RootProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-compiler/src/main/java/hilt/ComponentTreeDepsProcessor.java"),
        "class ComponentTreeDepsProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-compiler/src/main/java/hilt/AndroidEntryPointProcessingStep.java"),
        "class AndroidEntryPointProcessingStep {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-compiler/src/main/java/hilt/BindValueProcessor.java"),
        "class BindValueProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-compiler/src/main/java/hilt/TestInjectorGenerator.java"),
        "class TestInjectorGenerator {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android/src/main/java/hilt/HiltAndroidApp.java"),
        "class HiltAndroidApp {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android/src/main/java/hilt/AndroidEntryPoint.java"),
        "class AndroidEntryPoint {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android-testing/src/main/java/hilt/UninstallModules.java"),
        "class UninstallModules {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android-testing/src/main/java/hilt/TestInjector.java"),
        "class TestInjector {}\n",
    )
    .unwrap();
    repo
}

fn make_dagger_signal_repo() -> TempDir {
    let repo = make_dagger_like_repo();
    fs::create_dir_all(repo.path().join("dagger-spi/src/main/java/spi")).unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ComponentProcessor.java"),
        "class ComponentProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ComponentDescriptor.java"),
        "class ComponentDescriptor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/InjectProcessingStep.java"),
        "class InjectProcessingStep {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/SourceFileGenerator.java"),
        "class SourceFileGenerator {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/BindingGraph.java"),
        "class BindingGraph {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ValidationReport.java"),
        "class ValidationReport {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/DiagnosticReporter.java"),
        "class DiagnosticReporter {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/IncrementalProcessor.java"),
        "class IncrementalProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ClearableCache.java"),
        "class ClearableCache {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-spi/src/main/java/spi/BindingGraphPlugin.java"),
        "class BindingGraphPlugin {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-spi/src/main/java/spi/ServiceLoaderPlugin.java"),
        "class ServiceLoaderPlugin {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/InjectFactory.java"),
        "class InjectFactory {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/MembersInjector.java"),
        "class MembersInjector {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/LazyProviderFactory.java"),
        "class LazyProviderFactory {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/BindsInstanceFactory.java"),
        "class BindsInstanceFactory {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/IntoSet.java"),
        "class IntoSet {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-runtime/src/main/java/runtime/IntoMap.java"),
        "class IntoMap {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("tests/integration/BindingGraphSubjectTest.java"),
        "class BindingGraphSubjectTest {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("tests/integration/PerformanceMonitorTest.java"),
        "class PerformanceMonitorTest {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("tests/integration/BenchmarkProfileTest.java"),
        "class BenchmarkProfileTest {}\n",
    )
    .unwrap();
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
    domains
        .into_iter()
        .map(|domain| domain.domain_type)
        .collect()
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
    assert!(domains.contains(&DomainType::Troubleshooting));
    assert!(domains.contains(&DomainType::ThemeSystem));
}

#[test]
fn overlapping_structural_domains_keep_only_best_fitting_module_owner() {
    let repo = make_dagger_like_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let paths = units
        .iter()
        .map(|unit| unit.relative_path.clone())
        .collect::<Vec<_>>();

    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "框架集成-Hilt/hilt-android.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "平台绑定-Android/hilt-android.md"),
        "{paths:?}"
    );
    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "框架集成-Hilt/hilt-core.md"),
        "{paths:?}"
    );
    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "编译工具链/dagger-compiler.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心运行时/hilt-core.md"),
        "{paths:?}"
    );
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
    assert!(domains.contains(&DomainType::Troubleshooting));
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let knowledge_tree = build_knowledge_tree(domains, units);

    assert!(knowledge_tree
        .units
        .values()
        .any(|unit| unit.unit_type == UnitType::DomainIndex));
    assert!(knowledge_tree.units.values().any(|unit| matches!(
        unit.unit_type,
        UnitType::ApiDoc
            | UnitType::ConceptGuide
            | UnitType::ModuleDoc
            | UnitType::ConfigDoc
            | UnitType::ExampleDoc
    )));
    assert!(knowledge_tree
        .units
        .values()
        .any(|unit| { unit.decomposition_profile == Some(DecompositionProfile::ApiSurface) }));
    assert!(knowledge_tree
        .units
        .values()
        .any(|unit| { unit.decomposition_profile == Some(DecompositionProfile::ConfigSurface) }));
    assert!(knowledge_tree
        .units
        .values()
        .any(|unit| { unit.decomposition_profile == Some(DecompositionProfile::ExampleTutorial) }));
    assert!(!knowledge_tree.processing_order.is_empty());
}

#[test]
fn docs_backed_units_route_into_generic_domains_and_preserve_parent_chain() {
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    let addon_api = units
        .iter()
        .find(|unit| unit.title == "Addon Api")
        .expect("addon api docs unit should exist");
    assert_eq!(addon_api.unit_type, UnitType::ApiDoc);
    assert_eq!(
        addon_api.decomposition_profile,
        Some(DecompositionProfile::ApiSurface)
    );
    assert_eq!(addon_api.relative_path, "API-参考/api/Addon-Api.md");

    let main_js = units
        .iter()
        .find(|unit| unit.title == "Main Js")
        .expect("main js docs unit should exist");
    assert_eq!(main_js.unit_type, UnitType::ConfigDoc);
    assert_eq!(
        main_js.decomposition_profile,
        Some(DecompositionProfile::ConfigSurface)
    );
    assert_eq!(main_js.relative_path, "配置参考/configure/Main-Js.md");

    let troubleshooting = units
        .iter()
        .find(|unit| unit.title == "Troubleshooting")
        .expect("troubleshooting readme unit should exist");
    let common_errors = units
        .iter()
        .find(|unit| unit.title == "Common Errors")
        .expect("troubleshooting child unit should exist");
    assert_eq!(troubleshooting.unit_type, UnitType::TroubleshootDoc);
    assert_eq!(common_errors.unit_type, UnitType::TroubleshootDoc);
    assert_eq!(
        common_errors.parent_unit_id.as_deref(),
        Some(troubleshooting.id.as_str())
    );

    let get_started = units
        .iter()
        .find(|unit| unit.title == "Get Started")
        .expect("get started index unit should exist");
    let setup = units
        .iter()
        .find(|unit| unit.title == "Setup")
        .expect("nested setup unit should exist");
    assert_eq!(
        setup.parent_unit_id.as_deref(),
        Some(get_started.id.as_str())
    );
    assert_eq!(
        setup.relative_path,
        "概念指南/get-started/advanced/Setup.md"
    );
    assert_units_have_resolvable_parents(&units);
    assert!(
        !units
            .iter()
            .any(|unit| unit.title.eq_ignore_ascii_case("react")),
        "example sandbox modules should not become top-level knowledge units"
    );
    assert!(
        !units.iter().any(|unit| unit.title == "Example Snippet"),
        "docs/_snippets should stay as evidence assets instead of wiki pages"
    );
    assert!(
        !units.iter().any(|unit| unit.title == "ERRORS"),
        "markdown files under code roots should not become docs-backed wiki pages"
    );
    assert!(
        !units.iter().any(|unit| unit.title == "Docspage"),
        "nested docs roots under code/ should stay out of primary docs corpus"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.title == "React Renderer Package"),
        "package README under code/ should not become docs-backed wiki pages"
    );
    assert!(
        !units.iter().any(|unit| unit.title == "API：core"),
        "docs-backed api corpus should suppress blanket module api pages"
    );
}

#[test]
fn storybook_api_signal_units_cover_developer_api_topics_without_module_api_sprawl() {
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    for expected_title in [
        "插件API",
        "CSF API",
        "Preview API",
        "Store API",
        "API类型定义",
        "工具类型定义",
        "框架类型定义",
        "Angular框架支持",
        "组件故事（Stories）",
    ] {
        assert!(
            units.iter().any(|unit| {
                unit.title == expected_title
                    && unit.unit_type == UnitType::ApiDoc
                    && unit.decomposition_profile == Some(DecompositionProfile::ApiSurface)
            }),
            "missing storybook api signal unit: {expected_title}; api units={:?}",
            units
                .iter()
                .filter(|unit| {
                    unit.unit_type == UnitType::ApiDoc
                        && unit.decomposition_profile == Some(DecompositionProfile::ApiSurface)
                })
                .map(|unit| unit.title.clone())
                .collect::<Vec<_>>()
        );
    }

    assert!(
        !units.iter().any(|unit| unit.title == "API：core"),
        "docs-backed api corpus should suppress generic module api pages"
    );
    assert!(
        !units.iter().any(|unit| unit.title == "API：react"),
        "docs-backed api corpus should suppress generic framework module api pages"
    );
    for unexpected_title in [
        "@Inject 注解详解",
        "绑定与提供者模式",
        "运行时API",
        "异步处理与生产者",
        "@Provides 与 @Binds 注解详解",
        "SPI扩展机制",
    ] {
        assert!(
            !units.iter().any(|unit| unit.title == unexpected_title),
            "storybook should not materialize annotation-di signal unit: {unexpected_title}"
        );
    }
}

#[test]
fn nested_api_reference_docs_keep_section_index_but_suppress_deep_leaf_pages() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(repo.path().join("docs/api/main-config")).unwrap();
    fs::create_dir_all(repo.path().join("docs/api/portable-stories")).unwrap();
    fs::write(
        repo.path().join("docs/api/main-config/main-config.mdx"),
        "# Main Config\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/api/main-config/main-config-babel.mdx"),
        "# Main Config Babel\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/api/portable-stories/portable-stories.mdx"),
        "# Portable Stories\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/api/portable-stories/portable-stories-vitest.mdx"),
        "# Portable Stories Vitest\n",
    )
    .unwrap();

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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units
        .iter()
        .any(|unit| unit.relative_path == "API-参考/api/main-config/Main-Config.md"));
    assert!(units
        .iter()
        .any(|unit| unit.relative_path == "API-参考/api/portable-stories/Portable-Stories.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "API-参考/api/main-config/Main-Config-Babel.md"));
    assert!(!units.iter().any(|unit| {
        unit.relative_path == "API-参考/api/portable-stories/Portable-Stories-Vitest.md"
    }));
}

#[test]
fn docs_guide_skips_meta_reference_noise_pages() {
    let repo = make_storybook_like_repo();
    fs::write(repo.path().join("CHANGELOG.md"), "# Changelog\n").unwrap();
    fs::write(repo.path().join("CONTRIBUTING.md"), "# Contributing\n").unwrap();
    fs::write(
        repo.path().join("CODE-OF-CONDUCT.md"),
        "# Code Of Conduct\n",
    )
    .unwrap();
    fs::write(repo.path().join("SECURITY.md"), "# Security\n").unwrap();
    fs::create_dir_all(repo.path().join("docs/releases")).unwrap();
    fs::create_dir_all(repo.path().join("docs/contribute/documentation")).unwrap();
    fs::write(repo.path().join("docs/releases/roadmap.md"), "# Roadmap\n").unwrap();
    fs::write(
        repo.path().join("docs/releases/upgrading.md"),
        "# Upgrading\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/contribute/how-to-reproduce.md"),
        "# How To Reproduce\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/contribute/documentation/updates.md"),
        "# Documentation Updates\n",
    )
    .unwrap();

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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    for unexpected_title in [
        "CHANGELOG",
        "CODE OF CONDUCT",
        "Security",
        "Roadmap",
        "Upgrading",
        "How To Reproduce",
        "Documentation Updates",
    ] {
        assert!(
            !units.iter().any(|unit| unit.title == unexpected_title),
            "meta docs should stay as evidence only: {unexpected_title}"
        );
    }
    assert!(
        units.iter().any(|unit| {
            unit.title == "Contributing"
                || unit.title == "CONTRIBUTING"
                || unit.relative_path.ends_with("/CONTRIBUTING.md")
                || unit.relative_path == "概念指南/CONTRIBUTING.md"
        }),
        "top-level CONTRIBUTING guide should remain docs-backed"
    );
}

#[test]
fn dagger_units_capture_runtime_compiler_and_integration_profiles() {
    let repo = make_dagger_like_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::Runtime)
            && unit.title.to_ascii_lowercase().contains("runtime")
    }));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::CompilerPipeline)
            && unit.title.to_ascii_lowercase().contains("compiler")
    }));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::IntegrationPlatform)
            && unit.title.to_ascii_lowercase().contains("hilt")
    }));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::Testing)
            && unit.unit_type == UnitType::TestDoc
    }));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::Testing)
            && unit.title == "性能测试与监控"
    }));
    assert!(
        units.iter().any(|unit| {
            unit.decomposition_profile == Some(DecompositionProfile::ExampleTutorial)
                && unit.unit_type == UnitType::ExampleDoc
        }),
        "units: {:?}",
        units
            .iter()
            .map(|unit| format!(
                "{:?}:{:?}:{}",
                unit.unit_type, unit.decomposition_profile, unit.title
            ))
            .collect::<Vec<_>>()
    );
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::Troubleshooting)
            && unit.unit_type == UnitType::TroubleshootDoc
    }));
    for expected_title in [
        "Hilt API",
        "运行时API",
        "异步处理与生产者",
        "@Provides 与 @Binds 注解详解",
        "性能监控与调试",
    ] {
        assert!(
            units.iter().any(|unit| {
                unit.title == expected_title
                    && unit.unit_type == UnitType::ApiDoc
                    && unit.decomposition_profile == Some(DecompositionProfile::ApiSurface)
            }),
            "missing api leaf unit: {expected_title}"
        );
    }
}

#[test]
fn docs_planner_prefers_localized_overlay_corpus_over_raw_docs() {
    let repo = make_localized_docs_overlay_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| unit.title == "快速开始"));
    assert!(units.iter().any(|unit| unit.title == "插件接口"));
    assert!(!units.iter().any(|unit| unit.title == "Get Started"));
    assert!(!units.iter().any(|unit| unit.title == "Addon Api"));
}

#[test]
fn localized_docs_overlay_prunes_shadowed_source_units_and_domain_indexes() {
    let repo = make_localized_docs_overlay_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| unit.title == "快速开始"));
    assert!(units.iter().any(|unit| unit.title == "插件接口"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "插件生态/a11y.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "多框架支持/react.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "构建系统/vite.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "概念指南/概念指南.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "API-参考/API-参考.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "配置参考/配置参考.md"));
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "故障排除/故障排除.md"));
    assert_units_have_resolvable_parents(&units);
}

#[test]
fn hidden_generated_docs_corpus_does_not_become_primary_docs_units() {
    let repo = make_hidden_generated_docs_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| unit.title == "Get Started"));
    assert!(!units.iter().any(|unit| unit.title == "快速开始"));
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path.contains("qoder/repowiki")),
        "hidden generated docs should not leak into final wiki paths: {:?}",
        units
            .iter()
            .map(|unit| unit.relative_path.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn config_planner_filters_source_noise_and_keeps_real_config_entries() {
    let repo = make_config_noise_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| unit.title == "Main"));
    assert!(units.iter().any(|unit| unit.title == "Preview"));
    assert!(units.iter().any(|unit| unit.title == "Main Js"));
    assert!(!units.iter().any(|unit| unit.title == "Package"));
    assert!(!units.iter().any(|unit| unit.title == "AboutPage Tsx"));
    assert!(!units.iter().any(|unit| unit.title == "Config Test"));
    assert!(!units.iter().any(|unit| unit.title == "Addon Locations Png"));
}

#[test]
fn config_planner_adds_theme_signal_units_when_theme_docs_exist() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(repo.path().join("docs/configure/user-interface")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/themes/src/decorators")).unwrap();
    fs::write(
        repo.path()
            .join("docs/configure/user-interface/theming.mdx"),
        "# Theming\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/configure/user-interface/layout.mdx"),
        "# Layout\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/themes/src/decorators/provider.decorator.tsx"),
        "export const ThemeProviderDecorator = true;\n",
    )
    .unwrap();

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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    for expected_title in ["主题系统概览", "自定义主题开发", "颜色和字体系统"] {
        assert!(
            units.iter().any(|unit| {
                unit.title == expected_title
                    && unit.unit_type == UnitType::ConfigDoc
                    && unit.decomposition_profile == Some(DecompositionProfile::ConfigSurface)
            }),
            "missing config theme signal unit: {expected_title}; config units={:?}",
            units
                .iter()
                .filter(|unit| {
                    unit.unit_type == UnitType::ConfigDoc
                        && unit.decomposition_profile == Some(DecompositionProfile::ConfigSurface)
                })
                .map(|unit| unit.title.clone())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn compiler_docs_are_routed_into_compiler_toolchain_units() {
    let repo = make_dagger_like_repo();
    fs::create_dir_all(repo.path().join("docs/compiler")).unwrap();
    fs::write(repo.path().join("docs/compiler/codegen.md"), "# Codegen\n").unwrap();
    fs::write(
        repo.path().join("docs/compiler/validation.md"),
        "# Validation\n",
    )
    .unwrap();

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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(domains
        .iter()
        .any(|domain| domain.domain_type == DomainType::CompilerToolchain
            && domain
                .source_files
                .iter()
                .any(|path| path.ends_with("docs/compiler/codegen.md"))));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::CompilerPipeline)
            && unit
                .scope
                .docs_anchors
                .iter()
                .any(|anchor| anchor.file_path.ends_with("docs/compiler/codegen.md"))
            && unit.relative_path.starts_with("编译工具链/")
    }));
    assert!(units.iter().any(|unit| {
        unit.decomposition_profile == Some(DecompositionProfile::CompilerPipeline)
            && unit
                .scope
                .docs_anchors
                .iter()
                .any(|anchor| anchor.file_path.ends_with("docs/compiler/validation.md"))
    }));
}

#[test]
fn nested_package_readme_stays_docs_backed_but_src_markdown_does_not() {
    let repo = make_package_readme_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(units.iter().any(|unit| unit.title == "Core"));
    assert!(!units.iter().any(|unit| unit.title == "Internal"));
}

#[test]
fn dense_docs_corpus_uses_root_readme_as_evidence_instead_of_standalone_page() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(repo.path().join("docs/examples")).unwrap();
    fs::write(repo.path().join("README.md"), "# Storybook Like\n").unwrap();
    fs::write(repo.path().join("docs/guides.mdx"), "# Guides\n").unwrap();
    fs::write(
        repo.path().join("docs/examples/advanced.mdx"),
        "# Advanced\n",
    )
    .unwrap();

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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "概念指南/README.md"),
        "root README should not stay as a standalone concept page once the repo already has a dense docs corpus"
    );
}

#[test]
fn thin_domains_do_not_emit_extra_synthetic_indexes() {
    let repo = make_shadowed_docs_build_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "构建系统/构建系统.md"),
        "thin build domains should not add a synthetic domain index on top of one or two leaves"
    );
}

#[test]
fn dense_raw_config_surfaces_collapse_into_aggregate_unit() {
    let repo = make_dense_raw_config_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    let config_units = units
        .iter()
        .filter(|unit| unit.unit_type == UnitType::ConfigDoc)
        .collect::<Vec<_>>();
    assert!(
        config_units.iter().any(|unit| unit.title == "配置参考"),
        "dense raw config surfaces should collapse into an aggregate config page: {:?}",
        config_units
            .iter()
            .map(|unit| unit.relative_path.clone())
            .collect::<Vec<_>>()
    );
    assert!(!config_units.iter().any(|unit| unit.title == "Bazelrc"));
    assert!(!config_units.iter().any(|unit| unit.title == "Bazelversion"));
    assert!(!config_units
        .iter()
        .any(|unit| unit.title == "Gradle Properties"));
    assert!(
        !units.iter().any(|unit| {
            unit.unit_type == UnitType::DomainIndex
                && unit.relative_path == "配置参考/配置参考.md"
        }),
        "aggregate config page should suppress the synthetic domain index when they would share the same path"
    );
}

#[test]
fn docs_backed_shadow_pages_are_pruned_when_structural_page_exists() {
    let repo = make_shadowed_docs_build_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "构建系统/buildSrc.md"),
        "structural build page should remain: {:?}",
        units
            .iter()
            .map(|unit| unit.relative_path.clone())
            .collect::<Vec<_>>()
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "概念指南/BuildSrc.md"),
        "docs-backed shadow page should be removed when structural page already owns the topic"
    );
    assert_units_have_resolvable_parents(&units);
}

#[test]
fn localized_docs_overlay_units_still_write_to_sqlite_after_shadow_pruning() {
    let repo = make_localized_docs_overlay_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    assert_units_have_resolvable_parents(&units);

    let conn = sqlite_store::open_db(repo.path()).unwrap();
    sqlite_store::write_knowledge_domains(&conn, &domains).unwrap();
    sqlite_store::write_knowledge_units(&conn, &units).unwrap();
}

#[test]
fn fallback_core_does_not_reclaim_modules_owned_by_api_testing_or_build_domains() {
    let repo = make_fallback_overlap_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );
    let paths = units
        .iter()
        .map(|unit| unit.relative_path.clone())
        .collect::<Vec<_>>();

    assert!(
        units.iter().any(|unit| {
            unit.unit_type == UnitType::ApiDoc && unit.relative_path.starts_with("API-参考/")
        }),
        "{paths:?}"
    );
    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "测试基础设施/javatests.md"),
        "{paths:?}"
    );
    assert!(
        units
            .iter()
            .any(|unit| unit.relative_path == "构建系统/gradle.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/java.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/javatests.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/gradle.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/examples.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "配置参考/Package.md"),
        "{paths:?}"
    );
}

#[test]
fn testing_domain_keeps_broad_signal_corpus_for_large_repos() {
    let repo = make_large_testing_signal_repo();
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
    let testing_domain = domains
        .iter()
        .find(|domain| domain.domain_type == DomainType::TestingInfra)
        .expect("testing domain should exist");
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    assert!(
        testing_domain.source_files.len() >= 57,
        "large testing repos should keep the full signal corpus instead of truncating to a small prefix"
    );
    assert!(units.iter().any(|unit| unit.title == "性能测试与监控"));
}

#[test]
fn hilt_framework_domain_adds_framework_specific_topic_units() {
    let repo = make_hilt_signal_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    let hilt_family = units
        .iter()
        .find(|unit| unit.title == "Hilt框架详解")
        .expect("hilt family unit should exist");
    assert!(units.iter().any(|unit| {
        unit.title == "编译时处理与代码生成"
            && unit.parent_unit_id.as_deref() == Some(hilt_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "@HiltAndroidApp与注解使用"
            && unit.parent_unit_id.as_deref() == Some(hilt_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "模块安装与依赖管理"
            && unit.parent_unit_id.as_deref() == Some(hilt_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "测试支持与模拟"
            && unit.parent_unit_id.as_deref() == Some(hilt_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "组件树结构与生命周期"
            && unit.parent_unit_id.as_deref() == Some(hilt_family.id.as_str())
    }));
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "框架集成-Hilt/hilt-core.md"),
        "generic single-module framework page should be pruned when Hilt topic units already cover it"
    );
}

#[test]
fn signal_decomposition_adds_compiler_runtime_and_testing_topic_units() {
    let repo = make_dagger_signal_repo();
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
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &SteeringConfig::default(),
    );

    let compiler_family = units
        .iter()
        .find(|unit| unit.title == "编译时处理机制")
        .expect("compiler family unit should exist");
    let runtime_core_family = units
        .iter()
        .find(|unit| unit.title == "核心概念")
        .expect("runtime family unit should exist");
    let advanced_family = units
        .iter()
        .find(|unit| unit.title == "高级特性与扩展")
        .expect("advanced runtime family unit should exist");
    let testing_family = units
        .iter()
        .find(|unit| unit.title == "测试策略与最佳实践")
        .expect("testing family unit should exist");
    assert_eq!(
        compiler_family.decomposition_profile,
        Some(DecompositionProfile::CompilerPipeline)
    );
    assert!(units.iter().any(|unit| {
        unit.title == "注解处理基础"
            && unit.parent_unit_id.as_deref() == Some(compiler_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "代码生成策略"
            && unit.parent_unit_id.as_deref() == Some(compiler_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "依赖图构建"
            && unit.parent_unit_id.as_deref() == Some(compiler_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "验证与错误处理"
            && unit.parent_unit_id.as_deref() == Some(compiler_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "增量编译优化"
            && unit.parent_unit_id.as_deref() == Some(compiler_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| unit.title == "SPI扩展机制"));
    assert!(units.iter().any(|unit| {
        unit.title == "@Inject 注解详解"
            && unit.parent_unit_id.as_deref() == Some(runtime_core_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "绑定与提供者模式"
            && unit.parent_unit_id.as_deref() == Some(runtime_core_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "多值绑定高级用法"
            && unit.parent_unit_id.as_deref() == Some(advanced_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "单元测试"
            && unit.parent_unit_id.as_deref() == Some(testing_family.id.as_str())
    }));
    assert!(units.iter().any(|unit| {
        unit.title == "性能测试与监控"
            && unit.parent_unit_id.as_deref() == Some(testing_family.id.as_str())
    }));

    let paths = units
        .iter()
        .map(|unit| unit.relative_path.clone())
        .collect::<Vec<_>>();
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "编译工具链/dagger-compiler.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/核心概念.md"),
        "{paths:?}"
    );
    assert!(
        !units
            .iter()
            .any(|unit| unit.relative_path == "核心模块/高级特性与扩展.md"),
        "{paths:?}"
    );
}
