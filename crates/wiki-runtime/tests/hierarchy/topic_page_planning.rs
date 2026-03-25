//! 9.4 之后，页面规划的验收重点转为 Knowledge Planning 层。
//! 这组测试覆盖 storybook / dagger archetype 的知识域发现与知识单元规划。

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use tempfile::TempDir;
use wiki_runtime::domain::knowledge::{DecompositionProfile, DomainType, UnitType};
use wiki_runtime::domain::steering::SteeringConfig;
use wiki_runtime::generation::context::{build_module_contexts, build_repo_context};
use wiki_knowledge::planning::{
    build_knowledge_tree, discover_knowledge_domains, plan_knowledge_units,
};
use wiki_index::hierarchy::build_module_tree;
use wiki_index::scanner::scan_repo;
use wiki_index::symbol_graph::GraphSummary;
use wiki_runtime::storage::sqlite_store;

fn default_planner_config() -> wiki_knowledge::KnowledgePlannerConfig {
    SteeringConfig::default().knowledge_planner_config()
}

fn assert_units_have_resolvable_parents(units: &[wiki_runtime::domain::knowledge::KnowledgeUnit]) {
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

fn assert_unit_has_diagnostic_bundle(
    unit: &wiki_runtime::domain::knowledge::KnowledgeUnit,
    expected_kind: wiki_runtime::domain::knowledge::PlannerSignalKind,
) {
    let bundle = unit
        .planner_signal_bundles
        .iter()
        .find(|bundle| bundle.kind == expected_kind)
        .unwrap_or_else(|| {
            panic!(
                "unit `{}` missing planner signal bundle {:?}: {:?}",
                unit.title, expected_kind, unit.planner_signal_bundles
            )
        });
    assert!(
        !bundle.matched_keywords.is_empty(),
        "unit `{}` bundle {:?} should retain matched keywords",
        unit.title,
        expected_kind
    );
    assert!(
        !bundle.matched_source_ids.is_empty(),
        "unit `{}` bundle {:?} should retain matched source ids",
        unit.title,
        expected_kind
    );
    assert!(
        !bundle.matched_paths.is_empty(),
        "unit `{}` bundle {:?} should retain matched paths",
        unit.title,
        expected_kind
    );
}

fn make_storybook_like_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".storybook")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/react/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/angular/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/frameworks/vue3/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/builders/vite/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/builders/vite")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/docs/docs")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/themes/src/decorators")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/themes/template/stories")).unwrap();
    fs::create_dir_all(repo.path().join("code/renderers/react")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/template/stories")).unwrap();
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
        repo.path().join(".storybook/main.ts"),
        "export const defineMainConfig = { stories: [], addons: [], framework: '@storybook/react-vite' };\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".storybook/preview.tsx"),
        "export const previewAnnotations = ['docs']; export const themeProvider = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".storybook/manager.tsx"),
        "export const managerLayout = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/index.ts"),
        "export const addon = true;
",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/preset.ts"),
        "export const addonPreset = true;\n",
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
        repo.path().join("code/frameworks/angular/src/preset.ts"),
        "export const frameworkPreset = true;\n",
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
        repo.path().join("code/builders/vite/build-config.ts"),
        "export const viteBuildConfig = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/builders/vite/vite-config.ts"),
        "export const viteConfigSurface = true;\n",
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
        repo.path().join("code/core/src/decorators.ts"),
        "export const decoratorsApi = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/decorators.test.ts"),
        "test('decorators', () => {});\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/core/template/stories/decorators.stories.ts"),
        "export const decoratorsStory = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/hooks.ts"),
        "export const hooksApi = true;\n",
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
    fs::create_dir_all(repo.path().join("code/core/src/channels")).unwrap();
    fs::write(
        repo.path().join("code/core/src/channels/types.ts"),
        "export type ChannelEvent = { id: string };\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/types.ts"),
        "export type A11yApi = { enabled: boolean };\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/themes/src/decorators/provider.decorator.tsx"),
        "export const themeDecorator = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/themes/template/stories/decorators.stories.ts"),
        "export const themeDecoratorStory = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/builders/vite/src/types.ts"),
        "export type ViteBuilderOptions = { vite: boolean };\n",
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
            .join("dagger-android/src/main/java/android/AndroidInjector.java"),
        "class AndroidInjector {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-android/src/main/java/android/HasAndroidInjector.java"),
        "class HasAndroidInjector {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-android/src/main/java/android/AndroidInjection.java"),
        "class AndroidInjection {}\n",
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
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/ComponentProcessor.java"),
        "class ComponentProcessor {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("dagger-compiler/src/main/java/compiler/BindingGraphFactory.java"),
        "class BindingGraphFactory {}\n",
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
            .join("dagger-compiler/src/main/java/compiler/ValidationReport.java"),
        "class ValidationReport {}\n",
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
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        .find(|unit| unit.relative_path == "故障排除/troubleshooting.md")
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
fn docs_backed_units_keep_surface_cluster_diagnostics() {
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    for title in ["Addon Api", "Main Js", "Get Started"] {
        let unit = units
            .iter()
            .find(|unit| unit.title == title)
            .unwrap_or_else(|| panic!("docs-backed unit should exist: {title}"));
        assert_unit_has_diagnostic_bundle(
            unit,
            wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster,
        );
    }
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    for expected_title in [
        "插件API",
        "Decorators API",
        "CSF API",
        "Preview API",
        "Hooks API",
        "Store API",
        "API类型定义",
        "插件类型定义",
        "工具类型定义",
        "构建器类型定义",
        "核心类型定义",
        "框架类型定义",
        "main.js配置",
        "preview.js配置",
        "manager.js配置",
        "构建器配置",
        "预设配置",
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
    assert!(units.iter().any(|unit| {
        unit.title == "配置API参考"
            && unit.unit_type == UnitType::ApiDoc
            && unit.child_unit_ids.len() >= 3
    }));
    for (title, expected_path) in [
        ("preview.js配置", ".storybook/preview.tsx"),
        ("manager.js配置", ".storybook/manager.tsx"),
    ] {
        let unit = units
            .iter()
            .find(|unit| unit.title == title)
            .unwrap_or_else(|| panic!("missing config api unit: {title}"));
        let source_paths = unit
            .scope
            .source_ids
            .iter()
            .filter_map(|source_id| {
                report
                    .files
                    .iter()
                    .find(|file| &file.id == source_id)
                    .map(|file| file.path.as_str())
            })
            .collect::<Vec<_>>();
        assert!(
            source_paths.iter().any(|path| *path == expected_path),
            "{title} should retain {expected_path}; source_paths={source_paths:?}"
        );
    }
    let decorators_unit = units
        .iter()
        .find(|unit| unit.title == "Decorators API")
        .expect("missing decorators api unit");
    let decorators_sources = decorators_unit
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .collect::<Vec<_>>();
    assert!(
        decorators_sources
            .iter()
            .any(|path| *path == "code/core/src/decorators.ts"),
        "Decorators API should keep core implementation source; sources={decorators_sources:?}"
    );
    assert!(
        decorators_sources.len() <= 8,
        "Decorators API should trim source scope; sources={decorators_sources:?}"
    );
    assert!(
        decorators_sources
            .iter()
            .all(|path| !path.contains(".stories.") && !path.contains("/template/")),
        "Decorators API should prefer implementation sources over template stories; sources={decorators_sources:?}"
    );

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
fn types_api_prefers_topic_spine_over_generic_public_contracts() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(repo.path().join("code/core/src/csf")).unwrap();
    fs::create_dir_all(
        repo.path()
            .join("code/core/src/preview-api/modules/preview-web"),
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/manager-api/modules")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/docs-tools/argTypes")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/preview-api/modules/store")).unwrap();
    fs::create_dir_all(repo.path().join("code/renderers/html/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/lib/core-webpack/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/lib/cli-storybook/src")).unwrap();
    fs::write(repo.path().join("docs/api/arg-types.mdx"), "# Arg Types\n").unwrap();
    fs::write(
        repo.path().join("code/core/src/csf/story.ts"),
        "export type ComponentAnnotations = {}; export type StoryObj = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx"),
        "export class PreviewWeb {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/core/src/manager-api/modules/stories.ts"),
        "export const getCurrentStoryData = () => {};\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/core/src/docs-tools/argTypes/types.ts"),
        "export type ArgTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/core/src/preview-api/modules/store/args.test.ts"),
        "test('args', () => {});\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/renderers/html/src/public-types.ts"),
        "export type HtmlRendererTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/lib/core-webpack/src/types.ts"),
        "export type WebpackTypes = {};\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/lib/cli-storybook/src/typings.d.ts"),
        "export interface WindowTypes {}\n",
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    let unit = units
        .iter()
        .find(|unit| unit.title == "Types API")
        .expect("Types API should exist");
    let source_paths = unit
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .collect::<Vec<_>>();
    for expected_path in [
        "code/core/src/csf/story.ts",
        "code/core/src/preview-api/modules/preview-web/PreviewWeb.tsx",
        "code/core/src/manager-api/modules/stories.ts",
        "code/core/src/docs-tools/argTypes/types.ts",
    ] {
        assert!(
            source_paths.iter().any(|path| *path == expected_path),
            "Types API should keep topic spine source {expected_path}; sources={source_paths:?}"
        );
    }
    for unexpected_path in [
        "code/renderers/html/src/public-types.ts",
        "code/lib/core-webpack/src/types.ts",
        "code/lib/cli-storybook/src/typings.d.ts",
    ] {
        assert!(
            !source_paths.iter().any(|path| *path == unexpected_path),
            "Types API should not be dominated by generic contract source {unexpected_path}; sources={source_paths:?}"
        );
    }
    let collapse_guard = unit
        .collapse_guard
        .as_ref()
        .expect("Types API should retain scope refinement diagnostics");
    assert_eq!(
        collapse_guard.reason,
        wiki_runtime::domain::knowledge::CollapseGuardReason::TopicScopeRefinement
    );
    assert!(
        collapse_guard
            .collapsed_paths
            .iter()
            .any(|path| path.ends_with("public-types.ts")),
        "Types API should record collapsed generic contracts: {:?}",
        collapse_guard.collapsed_paths
    );
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        "Android API",
        "Hilt API",
        "编译时API",
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
    for (title, expected_paths) in [
        (
            "Android API",
            vec![
                "dagger-android/src/main/java/android/AndroidInjector.java",
                "dagger-android/src/main/java/android/AndroidInjection.java",
            ],
        ),
        (
            "编译时API",
            vec![
                "dagger-compiler/src/main/java/compiler/ComponentProcessor.java",
                "dagger-compiler/src/main/java/compiler/BindingGraphFactory.java",
            ],
        ),
    ] {
        let unit = units
            .iter()
            .find(|unit| unit.title == title)
            .unwrap_or_else(|| panic!("missing dagger api unit: {title}"));
        let source_paths = unit
            .scope
            .source_ids
            .iter()
            .filter_map(|source_id| {
                report
                    .files
                    .iter()
                    .find(|file| &file.id == source_id)
                    .map(|file| file.path.as_str())
            })
            .collect::<Vec<_>>();
        for expected_path in expected_paths {
            assert!(
                source_paths.iter().any(|path| *path == expected_path),
                "{title} should retain {expected_path}; source_paths={source_paths:?}"
            );
        }
        assert!(
            source_paths.len() <= 8,
            "{title} should keep a focused source scope; source_paths={source_paths:?}"
        );
        assert!(
            source_paths
                .iter()
                .all(|path| !path.contains("buildSrc/") && !path.contains("/test/")),
            "{title} should prefer implementation sources over build/test noise; source_paths={source_paths:?}"
        );
    }
}

#[test]
fn dagger_api_units_keep_topic_spines_under_scope_refinement() {
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    for (title, expected_paths) in [
        (
            "运行时API",
            vec![
                "dagger-runtime/src/main/java/runtime/MembersInjector.java",
                "dagger-runtime/src/main/java/runtime/Subcomponent.java",
            ],
        ),
        (
            "编译时API",
            vec![
                "dagger-compiler/src/main/java/compiler/ComponentProcessor.java",
                "dagger-compiler/src/main/java/compiler/BindingGraphFactory.java",
            ],
        ),
        (
            "Hilt API",
            vec![
                "hilt-android/src/main/java/hilt/HiltAndroidApp.java",
                "hilt-android/src/main/java/hilt/AndroidEntryPoint.java",
            ],
        ),
    ] {
        let unit = units
            .iter()
            .find(|unit| unit.title == title)
            .unwrap_or_else(|| panic!("missing dagger refined api unit: {title}"));
        let source_paths = unit
            .scope
            .source_ids
            .iter()
            .filter_map(|source_id| {
                report
                    .files
                    .iter()
                    .find(|file| &file.id == source_id)
                    .map(|file| file.path.as_str())
            })
            .collect::<Vec<_>>();
        for expected_path in expected_paths {
            assert!(
                source_paths.iter().any(|path| *path == expected_path),
                "{title} should keep non-generic implementation spine {expected_path}; sources={source_paths:?}"
            );
        }
        assert!(
            !source_paths.is_empty(),
            "{title} should keep a non-empty scope after refinement"
        );
    }
}

#[test]
fn storybook_repo_signal_families_materialize_fidelity_target_pages() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(
        repo.path()
            .join("docs/writing-tests/integrations/vitest-addon"),
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("docs/configure/integration")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/telemetry")).unwrap();
    fs::create_dir_all(repo.path().join("docs/_snippets")).unwrap();
    fs::create_dir_all(repo.path().join(".circleci")).unwrap();
    fs::create_dir_all(repo.path().join("code/lib/eslint-plugin/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/docs/src")).unwrap();
    fs::write(repo.path().join("docs/faq.mdx"), "# FAQ\n").unwrap();
    fs::write(
        repo.path()
            .join("docs/writing-tests/integrations/vitest-addon/index.mdx"),
        "# Vitest Addon\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/writing-tests/integrations/vitest-addon/migration-guide.mdx"),
        "# Vitest Migration\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/writing-tests/in-ci.mdx"),
        "# In CI\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/configure/integration/eslint-plugin.mdx"),
        "# ESLint Plugin\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/configure/telemetry.mdx"),
        "# Telemetry\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/_snippets/chromatic-install.md"),
        "# Chromatic Install\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("docs/_snippets/test-runner-local-build-workflow.md"),
        "# Test Runner Workflow\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".circleci/config.yml"),
        "workflows:\n  chromatic:\n    jobs: []\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/chromatic.config.json"),
        "{ \"projectId\": \"storybook-like\" }\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/lib/eslint-plugin/src/index.ts"),
        "export const eslintPlugin = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/telemetry/index.ts"),
        "export const telemetry = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/docs/src/index.ts"),
        "export const docsAddon = true;\n",
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    for expected_path in [
        "故障排除/故障排除.md",
        "测试框架/测试框架.md",
        "测试框架/Vitest集成.md",
        "部署和CI_CD/部署和CI_CD.md",
        "部署和CI_CD/CI_CD集成.md",
        "部署和CI_CD/Chromatic集成.md",
        "高级功能/工具集成/ESLint集成.md",
        "高级功能/性能监控.md",
        "插件系统/核心Addons详解/Docs-Addon.md",
    ] {
        assert!(
            units.iter().any(|unit| unit.relative_path == expected_path),
            "missing storybook fidelity target page: {expected_path}; units={:?}",
            units
                .iter()
                .map(|unit| unit.relative_path.clone())
                .collect::<Vec<_>>()
        );
    }

    let testing_family = units
        .iter()
        .find(|unit| unit.relative_path == "测试框架/测试框架.md")
        .expect("testing family should exist");
    let vitest_unit = units
        .iter()
        .find(|unit| unit.relative_path == "测试框架/Vitest集成.md")
        .expect("vitest page should exist");
    assert!(vitest_unit.parent_unit_id.as_deref() == Some(testing_family.id.as_str()));
    let vitest_signal = vitest_unit
        .planner_signal_bundles
        .iter()
        .find(|bundle| {
            bundle.kind == wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster
        })
        .expect("vitest page should retain surface cluster diagnostics");
    assert!(
        vitest_signal
            .key
            .starts_with("surface_cluster/testing_frameworks/"),
        "Vitest集成 should expose testing capability namespace; key={}",
        vitest_signal.key
    );
    let vitest_sources = vitest_unit
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .collect::<Vec<_>>();
    assert!(
        vitest_sources
            .iter()
            .all(|path| !path.contains("/test-storybooks/") && !path.contains("/template/")),
        "Vitest集成 should stay off sample/template noise; sources={vitest_sources:?}"
    );
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "测试基础设施/test-storybooks.md"));
    assert!(units.iter().any(|unit| {
        unit.relative_path == "测试框架/Vitest集成.md"
            && unit.parent_unit_id.as_deref() == Some(testing_family.id.as_str())
    }));

    let deploy_family = units
        .iter()
        .find(|unit| unit.relative_path == "部署和CI_CD/部署和CI_CD.md")
        .expect("deploy family should exist");
    let deploy_signal = deploy_family
        .planner_signal_bundles
        .iter()
        .find(|bundle| {
            bundle.kind == wiki_runtime::domain::knowledge::PlannerSignalKind::LeafDecomposition
        })
        .expect("deploy family should retain leaf decomposition diagnostics");
    assert!(
        deploy_signal
            .key
            .starts_with("leaf_decomposition/delivery_pipelines/"),
        "部署和CI_CD family should expose delivery capability namespace; key={}",
        deploy_signal.key
    );
    for expected_child in ["部署和CI_CD/CI_CD集成.md", "部署和CI_CD/Chromatic集成.md"] {
        assert!(units.iter().any(|unit| {
            unit.relative_path == expected_child
                && unit.parent_unit_id.as_deref() == Some(deploy_family.id.as_str())
        }));
    }

    let docs_addon = units
        .iter()
        .find(|unit| unit.relative_path == "插件系统/核心Addons详解/Docs-Addon.md")
        .expect("docs addon leaf should exist");
    assert!(!units
        .iter()
        .any(|unit| unit.relative_path == "插件生态/docs.md"));
    assert_eq!(docs_addon.title, "Docs Addon");
    let docs_addon_signal = docs_addon
        .planner_signal_bundles
        .iter()
        .find(|bundle| {
            bundle.kind == wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster
        })
        .expect("docs addon page should retain surface cluster diagnostics");
    assert!(
        docs_addon_signal
            .key
            .starts_with("surface_cluster/addon_ecosystem/"),
        "Docs Addon should expose addon capability namespace; key={}",
        docs_addon_signal.key
    );
    assert!(
        docs_addon
            .scope
            .source_ids
            .iter()
            .filter_map(|source_id| {
                report
                    .files
                    .iter()
                    .find(|file| &file.id == source_id)
                    .map(|file| file.path.as_str())
            })
            .any(|path| path.contains("code/addons/docs/")),
        "Docs Addon should stay grounded on addon sources"
    );
    let troubleshooting_unit = units
        .iter()
        .find(|unit| unit.relative_path == "故障排除/故障排除.md")
        .expect("troubleshooting root should exist");
    assert!(troubleshooting_unit
        .scope
        .docs_anchors
        .iter()
        .any(|anchor| anchor.file_path == "docs/faq.mdx"));
}

#[test]
fn dagger_examples_and_android_testing_materialize_fidelity_target_pages() {
    let repo = make_dagger_like_repo();
    fs::create_dir_all(repo.path().join("examples/bazel/java/example/hilt")).unwrap();
    fs::create_dir_all(
        repo.path()
            .join("hilt-android-testing/main/java/dagger/hilt/android/testing"),
    )
    .unwrap();
    fs::write(repo.path().join("README.md"), "# Dagger\n").unwrap();
    fs::write(repo.path().join("examples/bazel/BUILD"), "# bazel\n").unwrap();
    fs::write(
        repo.path()
            .join("examples/bazel/java/example/hilt/CoffeeApp.java"),
        "class CoffeeApp {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("examples/bazel/java/example/hilt/HeaterModule.java"),
        "class HeaterModule {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("examples/bazel/java/example/hilt/PumpModule.java"),
        "class PumpModule {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(
            "hilt-android-testing/main/java/dagger/hilt/android/testing/HiltAndroidTest.java",
        ),
        "class HiltAndroidTest {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(
            "hilt-android-testing/main/java/dagger/hilt/android/testing/HiltAndroidRule.java",
        ),
        "class HiltAndroidRule {}\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(
            "hilt-android-testing/main/java/dagger/hilt/android/testing/CustomTestApplication.java",
        ),
        "class CustomTestApplication {}\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("hilt-android-testing/main/java/dagger/hilt/android/testing/TestInstallIn.java"),
        "class TestInstallIn {}\n",
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    for expected_path in [
        "快速开始.md",
        "示例与教程/示例与教程.md",
        "示例与教程/基础示例.md",
        "测试策略与最佳实践/Android测试.md",
    ] {
        assert!(
            units.iter().any(|unit| unit.relative_path == expected_path),
            "missing dagger fidelity target page: {expected_path}; units={:?}",
            units
                .iter()
                .map(|unit| unit.relative_path.clone())
                .collect::<Vec<_>>()
        );
    }

    let quick_start = units
        .iter()
        .find(|unit| unit.relative_path == "快速开始.md")
        .expect("quick start page should exist");
    assert!(quick_start
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .any(|path| path == "README.md"));

    let basic_example = units
        .iter()
        .find(|unit| unit.relative_path == "示例与教程/基础示例.md")
        .expect("basic example page should exist");
    let basic_example_sources = basic_example
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .collect::<Vec<_>>();
    assert!(
        basic_example_sources
            .iter()
            .any(|path| path.starts_with("examples/bazel/")),
        "basic example should stay grounded on representative example sources; sources={basic_example_sources:?}"
    );
    assert!(
        basic_example_sources
            .iter()
            .all(|path| !path.contains("/javatests/")),
        "basic example should not expand into example test noise; sources={basic_example_sources:?}"
    );

    let testing_family = units
        .iter()
        .find(|unit| unit.relative_path == "测试策略与最佳实践/测试策略与最佳实践.md")
        .expect("testing family should exist");
    let android_testing = units
        .iter()
        .find(|unit| unit.relative_path == "测试策略与最佳实践/Android测试.md")
        .expect("android testing page should exist");
    assert_eq!(
        android_testing.parent_unit_id.as_deref(),
        Some(testing_family.id.as_str())
    );
    let android_testing_signal = android_testing
        .planner_signal_bundles
        .iter()
        .find(|bundle| {
            bundle.kind == wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster
        })
        .expect("android testing page should retain surface cluster diagnostics");
    assert!(
        android_testing_signal
            .key
            .starts_with("surface_cluster/testing_practices/"),
        "Android测试 should expose testing capability namespace; key={}",
        android_testing_signal.key
    );
    let android_testing_sources = android_testing
        .scope
        .source_ids
        .iter()
        .filter_map(|source_id| {
            report
                .files
                .iter()
                .find(|file| &file.id == source_id)
                .map(|file| file.path.as_str())
        })
        .collect::<Vec<_>>();
    assert!(
        android_testing_sources
            .iter()
            .any(|path| path.contains("hilt-android-testing/main/java/dagger/hilt/android/testing/")),
        "Android测试 should stay grounded on hilt-android-testing support sources; sources={android_testing_sources:?}"
    );
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
fn theme_signal_units_keep_distinct_topic_spines() {
    let repo = make_storybook_like_repo();
    fs::create_dir_all(repo.path().join("docs/configure/user-interface")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/theming/themes")).unwrap();
    fs::create_dir_all(repo.path().join("code/core/src/theming/tests")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/themes/src/decorators")).unwrap();
    fs::create_dir_all(repo.path().join("code/addons/a11y/src")).unwrap();
    fs::create_dir_all(repo.path().join("code/.storybook")).unwrap();
    fs::write(
        repo.path()
            .join("docs/configure/user-interface/theming.mdx"),
        "# Theming\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("docs/configure/styling-and-css.mdx"),
        "# Styling And Css\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/create.ts"),
        "export const createTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/convert.ts"),
        "export const convertTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/ensure.ts"),
        "export const ensureTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/base.ts"),
        "export const themeBase = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/themes/light.ts"),
        "export const lightTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/core/src/theming/themes/dark.ts"),
        "export const darkTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/themes/src/theme-switcher.tsx"),
        "export const ThemeSwitcher = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/themes/src/decorators/index.ts"),
        "export const withTheme = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/addons/a11y/src/a11yRunner.ts"),
        "export const runA11y = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path()
            .join("code/addons/a11y/src/visionSimulatorFilters.ts"),
        "export const visionSimulator = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/.storybook/preview.tsx"),
        "export const preview = true;\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("code/.storybook/manager.tsx"),
        "export const manager = true;\n",
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    let sources_for = |title: &str| {
        units
            .iter()
            .find(|unit| unit.title == title)
            .unwrap_or_else(|| panic!("missing theme signal unit: {title}"))
            .scope
            .source_ids
            .iter()
            .filter_map(|source_id| {
                report
                    .files
                    .iter()
                    .find(|file| &file.id == source_id)
                    .map(|file| file.path.clone())
            })
            .collect::<Vec<_>>()
    };

    let theme_overview_sources = sources_for("主题系统概览");
    assert!(
        theme_overview_sources
            .iter()
            .any(|path| path == "code/core/src/theming/base.ts"),
        "主题系统概览 should stay on theme system spine; sources={theme_overview_sources:?}"
    );
    assert!(
        theme_overview_sources.iter().any(|path| path == "code/.storybook/preview.tsx")
            || theme_overview_sources
                .iter()
                .any(|path| path == "code/addons/themes/src/theme-switcher.tsx"),
        "主题系统概览 should keep preview/manager integration context; sources={theme_overview_sources:?}"
    );

    let custom_theme_sources = sources_for("自定义主题开发");
    for expected_path in [
        "code/core/src/theming/create.ts",
        "code/core/src/theming/convert.ts",
        "code/core/src/theming/ensure.ts",
    ] {
        assert!(
            custom_theme_sources.iter().any(|path| path == expected_path),
            "自定义主题开发 should keep implementation spine {expected_path}; sources={custom_theme_sources:?}"
        );
    }
    assert!(
        custom_theme_sources.iter().any(|path| {
            path == "code/addons/themes/src/decorators/index.ts"
                || path == "code/addons/themes/src/decorators/provider.decorator.tsx"
        }),
        "自定义主题开发 should keep addon theme decorator context; sources={custom_theme_sources:?}"
    );
    assert!(
        !custom_theme_sources
            .iter()
            .any(|path| path == "code/addons/a11y/src/a11yRunner.ts"),
        "自定义主题开发 should not drift into a11y validation sources; sources={custom_theme_sources:?}"
    );

    let color_font_sources = sources_for("颜色和字体系统");
    assert!(
        color_font_sources
            .iter()
            .any(|path| path == "code/addons/a11y/src/a11yRunner.ts")
            || color_font_sources
                .iter()
                .any(|path| { path == "code/addons/a11y/src/visionSimulatorFilters.ts" }),
        "颜色和字体系统 should keep accessibility validation spine; sources={color_font_sources:?}"
    );
    assert!(
        !color_font_sources
            .iter()
            .any(|path| path == "code/addons/themes/package.json"),
        "颜色和字体系统 should shrink generic package manifests once stronger topic spine exists; sources={color_font_sources:?}"
    );
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );

    let config_units = units
        .iter()
        .filter(|unit| unit.unit_type == UnitType::ConfigDoc)
        .collect::<Vec<_>>();
    let aggregate_unit = config_units
        .iter()
        .find(|unit| unit.title == "配置参考")
        .expect("aggregate config unit should exist");
    assert!(
        config_units.iter().any(|unit| unit.title == "配置参考"),
        "dense raw config surfaces should collapse into an aggregate config page: {:?}",
        config_units
            .iter()
            .map(|unit| unit.relative_path.clone())
            .collect::<Vec<_>>()
    );
    let collapse_guard = aggregate_unit
        .collapse_guard
        .as_ref()
        .expect("aggregate config unit should retain collapse guard");
    assert_eq!(
        collapse_guard.reason,
        wiki_runtime::domain::knowledge::CollapseGuardReason::RawConfigSurfaceAggregation
    );
    for expected_path in [
        "package.json",
        "settings.gradle.kts",
        "gradle.properties",
        "buildSrc/build.gradle.kts",
    ] {
        assert!(
            collapse_guard
                .collapsed_paths
                .iter()
                .any(|path| path == expected_path),
            "collapse guard should retain collapsed path {expected_path}: {:?}",
            collapse_guard.collapsed_paths
        );
    }
    assert!(
        collapse_guard.collapsed_paths.len() >= 4,
        "collapse guard should retain the collapsed raw config candidate set: {:?}",
        collapse_guard.collapsed_paths
    );
    assert!(
        collapse_guard.preserved_paths.is_empty(),
        "dense raw config repo should not preserve standalone/doc-backed config candidates: {:?}",
        collapse_guard.preserved_paths
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
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
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
        &default_planner_config(),
    );
    let units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
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
    assert_unit_has_diagnostic_bundle(
        compiler_family,
        wiki_runtime::domain::knowledge::PlannerSignalKind::LeafDecomposition,
    );
    assert_unit_has_diagnostic_bundle(
        compiler_family,
        wiki_runtime::domain::knowledge::PlannerSignalKind::RepoArchetype,
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
    let compiler_leaf = units
        .iter()
        .find(|unit| unit.title == "注解处理基础")
        .expect("compiler leaf should exist");
    assert_unit_has_diagnostic_bundle(
        compiler_leaf,
        wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster,
    );
    assert_unit_has_diagnostic_bundle(
        compiler_leaf,
        wiki_runtime::domain::knowledge::PlannerSignalKind::RepoArchetype,
    );
    let runtime_leaf = units
        .iter()
        .find(|unit| unit.title == "@Inject 注解详解")
        .expect("runtime leaf should exist");
    assert_unit_has_diagnostic_bundle(
        runtime_leaf,
        wiki_runtime::domain::knowledge::PlannerSignalKind::SurfaceCluster,
    );
    assert_unit_has_diagnostic_bundle(
        runtime_leaf,
        wiki_runtime::domain::knowledge::PlannerSignalKind::RepoArchetype,
    );

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

#[test]
fn signal_decomposition_unit_ids_stay_stable_after_non_signal_noise() {
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
        &default_planner_config(),
    );
    let baseline_units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );
    let baseline_ids = baseline_units
        .iter()
        .filter(|unit| {
            matches!(
                unit.title.as_str(),
                "编译时处理机制" | "注解处理基础" | "核心概念" | "@Inject 注解详解"
            )
        })
        .map(|unit| (unit.relative_path.clone(), unit.id.clone()))
        .collect::<BTreeMap<_, _>>();

    fs::create_dir_all(repo.path().join("docs/changelog")).unwrap();
    fs::write(repo.path().join("docs/changelog/notes.md"), "# Notes\n").unwrap();

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
        &default_planner_config(),
    );
    let updated_units = plan_knowledge_units(
        &domains,
        &tree,
        &report,
        &mod_ctxs,
        &default_planner_config(),
    );
    let updated_ids = updated_units
        .iter()
        .filter(|unit| baseline_ids.contains_key(&unit.relative_path))
        .map(|unit| (unit.relative_path.clone(), unit.id.clone()))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(baseline_ids, updated_ids);
}




