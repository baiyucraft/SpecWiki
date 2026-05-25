# storybook Reference 对比报告

生成页面：220 页
reference 页面：176 页
命中对比：176 页
缺失对比：0 页
总体对齐率：100%
运行模式：reuse (cache_mode=preserve)
LLM usage：requests=0, total_tokens=0, page_research=0, page_enrichment=0

## 95% 验收口径

- overall_match_rate：176/176 = 100%
- missing pages：0
- collapsed pages：0
- low-fidelity matched pages：176
- extra generated pages：165
- provider-backed page research requests：0
- budget stopped pages：0
- stalled pages：0
- invalid output pages：0
- provider failed pages：0

## 覆盖统计

- 专题页覆盖：generated 6 / reference 8（repo-archetype=0）
- evidence 落页：generated 219 / reference 176
- citation 密度：generated 69.73 / reference 77.72
- 图表达覆盖：generated 220 / reference 176
- 主章节骨架短板：112 页
- 英文 raw docs 命名残留：matched 0 / extra 0
- page research 请求：0
- page enrichment 请求：0
- stop reason 分布：completed(205)
- research session 聚合：turns=0, tool_calls=0, delta_section=856, delta_evidence=204, delta_diagram=74, child_digest=0
- 已规划专题类型：专题页(6)
- 高频缺失专题：无

## Decomposition 命中

- generated：runtime(83)、compiler-pipeline(18)、api-surface(82)、config-surface(140)、docs-guide(220)、testing(101)、example-tutorial(84)、troubleshooting(124)、integration-platform(149)
- reference：runtime(168)、compiler-pipeline(60)、api-surface(125)、config-surface(168)、docs-guide(176)、testing(120)、example-tutorial(127)、troubleshooting(176)、integration-platform(136)
- 高频缺口：troubleshooting(107)、example-tutorial(71)、compiler-pipeline(44)、api-surface(22)、runtime(19)、testing(14)

## 项目结论

- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- docs-backed 页面主章节骨架仍偏离 reference，存在英文原始 docs heading 或泛化模板回退
- 解释层正文密度仍低于 reference
- 仍存在明显 page collapse，同一生成页承担多个 reference 页面

## 多页折叠现象

- 多框架支持/多框架支持.md 被 6 个 reference 页面共享映射
- 插件生态/pseudo-states.md 被 4 个 reference 页面共享映射
- 核心模块/code.md 被 6 个 reference 页面共享映射
- API-参考/开发API参考/Preview-API.md 被 2 个 reference 页面共享映射
- API-参考/开发API参考.md 被 2 个 reference 页面共享映射
- 构建系统/构建系统.md 被 8 个 reference 页面共享映射
- API-参考/组件故事-Stories.md 被 11 个 reference 页面共享映射
- 配置参考/主题和外观.md 被 7 个 reference 页面共享映射
- 主题系统/themes.md 被 3 个 reference 页面共享映射
- API-参考/Angular框架支持.md 被 2 个 reference 页面共享映射
- 多框架支持/svelte-vite.md 被 3 个 reference 页面共享映射
- 构建系统/vue3-vite.md 被 2 个 reference 页面共享映射
- 插件生态/插件生态.md 被 4 个 reference 页面共享映射
- 插件生态/a11y.md 被 3 个 reference 页面共享映射
- 概念指南/README.md 被 2 个 reference 页面共享映射
- 概念指南/addons.md 被 2 个 reference 页面共享映射
- 配置参考/配置参考.md 被 2 个 reference 页面共享映射
- 插件生态/addons.md 被 2 个 reference 页面共享映射
- 核心模块/核心模块.md 被 6 个 reference 页面共享映射
- 构建系统/builder-vite.md 被 2 个 reference 页面共享映射
- 系统架构.md 被 12 个 reference 页面共享映射
- 概念指南/writing-docs.md 被 3 个 reference 页面共享映射
- 项目概述.md 被 50 个 reference 页面共享映射

## 额外生成页面

- API-参考/API-参考.md (API 参考, 153 行)
- API-参考/api/Arg-Types.md (Arg Types, 138 行)
- API-参考/api/Cli-Options.md (Cli Options, 139 行)
- API-参考/api/New-Frameworks.md (New Frameworks, 140 行)
- API-参考/api/Parameters.md (Parameters, 137 行)
- API-参考/api/main-config/Main-Config.md (Main Config, 138 行)
- 主题系统/主题系统.md (主题系统, 143 行)
- 多框架支持/angular.md (angular, 147 行)
- 多框架支持/create-react-app.md (create-react-app, 120 行)
- 多框架支持/html-vite.md (html-vite, 161 行)
- 多框架支持/preact-vite.md (preact-vite, 134 行)
- 多框架支持/preact.md (preact, 137 行)
- 多框架支持/react-dom-shim.md (react-dom-shim, 143 行)
- 多框架支持/react-native-web-vite.md (react-native-web-vite, 168 行)
- 多框架支持/react-vite.md (react-vite, 159 行)
- 多框架支持/react-vitest-3.md (react-vitest-3, 148 行)
- 多框架支持/react-webpack.md (react-webpack, 136 行)
- 多框架支持/react-webpack5.md (react-webpack5, 174 行)
- 多框架支持/react.md (react, 178 行)
- 多框架支持/server-webpack5.md (server-webpack5, 134 行)
- 多框架支持/svelte.md (svelte, 135 行)
- 多框架支持/sveltekit.md (sveltekit, 167 行)
- 多框架支持/vue3.md (vue3, 177 行)
- 平台绑定-Web/web-components-vite.md (web-components-vite, 139 行)
- 平台绑定-Web/平台绑定-Web.md (平台绑定：Web, 150 行)
- 开发工具/cli-sb.md (cli-sb, 109 行)
- 开发工具/cli-storybook.md (cli-storybook, 157 行)
- 开发工具/ember-cli.md (ember-cli, 136 行)
- 开发工具/开发工具.md (开发工具, 159 行)
- 插件生态/csf-plugin.md (csf-plugin, 137 行)
- 插件生态/docs.md (docs, 82 行)
- 插件生态/eslint-plugin-local-rules.md (eslint-plugin-local-rules, 122 行)
- 插件生态/onboarding.md (onboarding, 142 行)
- 故障排除/Faq.md (Faq, 137 行)
- 构建系统/nextjs-vite.md (nextjs-vite, 152 行)
- 构建系统/server-webpack.md (server-webpack, 144 行)
- 构建系统/vitest.md (vitest, 160 行)
- 核心模块/codemod.md (codemod, 145 行)
- 核心模块/docs.md (docs, 105 行)
- 核心模块/ember.md (ember, 139 行)
- 核心模块/external-docs.md (external-docs, 137 行)
- 核心模块/html.md (html, 137 行)
- 核心模块/nextjs.md (nextjs, 145 行)
- 核心模块/portable-stories-kitchen-sink.md (portable-stories-kitchen-sink, 144 行)
- 核心模块/server-kitchen-sink.md (server-kitchen-sink, 141 行)
- 核心模块/server.md (server, 126 行)
- 核心模块/standalone-preview.md (standalone-preview, 109 行)
- 核心模块/test-storybooks.md (test-storybooks, 138 行)
- 核心模块/yarn-pnp.md (yarn-pnp, 148 行)
- 核心模块/yarn.md (.yarn, 116 行)
- 核心运行时/core-webpack.md (core-webpack, 149 行)
- 核心运行时/lib.md (lib, 142 行)
- 概念指南/CHANGELOG-Prerelease.md (CHANGELOG Prerelease, 137 行)
- 概念指南/CHANGELOG-V1-5.md (CHANGELOG V1 5, 136 行)
- 概念指南/CHANGELOG-V6.md (CHANGELOG V6, 138 行)
- 概念指南/CONTRIBUTING-Old.md (CONTRIBUTING Old, 138 行)
- 概念指南/CONTRIBUTING.md (CONTRIBUTING, 140 行)
- 概念指南/MIGRATION.md (MIGRATION, 138 行)
- 概念指南/addons/Addon-Knowledge-Base.md (Addon Knowledge Base, 162 行)
- 概念指南/addons/Addon-Migration-Guide.md (Addon Migration Guide, 140 行)
- 概念指南/addons/Addon-Types.md (Addon Types, 138 行)
- 概念指南/addons/Addons-Api.md (Addons Api, 139 行)
- 概念指南/addons/Install-Addons.md (Install Addons, 138 行)
- 概念指南/addons/Integration-Catalog.md (Integration Catalog, 146 行)
- 概念指南/addons/Writing-Addons.md (Writing Addons, 147 行)
- 概念指南/addons/Writing-Presets.md (Writing Presets, 138 行)
- 概念指南/builders/Builder-Api.md (Builder Api, 138 行)
- 概念指南/builders/Vite.md (Vite, 140 行)
- 概念指南/builders/Webpack.md (Webpack, 148 行)
- 概念指南/contribute/Framework.md (Framework, 139 行)
- 概念指南/essentials.md (Essentials, 220 行)
- 概念指南/essentials/Actions.md (Actions, 138 行)
- 概念指南/essentials/Controls.md (Controls, 143 行)
- 概念指南/essentials/Highlight.md (Highlight, 137 行)
- 概念指南/essentials/Measure-And-Outline.md (Measure And Outline, 137 行)
- 概念指南/essentials/Themes.md (Themes, 140 行)
- 概念指南/essentials/Toolbars-And-Globals.md (Toolbars And Globals, 138 行)
- 概念指南/essentials/Viewport.md (Viewport, 137 行)
- 概念指南/get-started/Browse-Stories.md (Browse Stories, 138 行)
- 概念指南/get-started/Conclusion.md (Conclusion, 161 行)
- 概念指南/get-started/Install.md (Install, 140 行)
- 概念指南/get-started/Setup.md (Setup, 138 行)
- 概念指南/get-started/Whats-A-Story.md (Whats A Story, 139 行)
- 概念指南/get-started/Why-Storybook.md (Why Storybook, 136 行)
- 概念指南/get-started/frameworks.md (Frameworks, 213 行)
- 概念指南/get-started/frameworks/Angular.md (Angular, 140 行)
- 概念指南/get-started/frameworks/Nextjs-Vite.md (Nextjs Vite, 143 行)
- 概念指南/get-started/frameworks/Nextjs.md (Nextjs, 138 行)
- 概念指南/get-started/frameworks/Preact-Vite.md (Preact Vite, 149 行)
- 概念指南/get-started/frameworks/React-Native-Web-Vite.md (React Native Web Vite, 142 行)
- 概念指南/get-started/frameworks/React-Vite.md (React Vite, 139 行)
- 概念指南/get-started/frameworks/React-Webpack5.md (React Webpack5, 138 行)
- 概念指南/get-started/frameworks/Svelte-Vite.md (Svelte Vite, 143 行)
- 概念指南/get-started/frameworks/Sveltekit.md (Sveltekit, 171 行)
- 概念指南/get-started/frameworks/Vue3-Vite.md (Vue3 Vite, 148 行)
- 概念指南/get-started/frameworks/Web-Components-Vite.md (Web Components Vite, 140 行)
- 概念指南/scripts/eslint-plugin-local-rules.md (Eslint Plugin Local Rules, 105 行)
- 概念指南/sharing.md (Sharing, 201 行)
- 概念指南/sharing/Design-Integrations.md (Design Integrations, 137 行)
- 概念指南/sharing/Embed.md (Embed, 138 行)
- 概念指南/sharing/Package-Composition.md (Package Composition, 138 行)
- 概念指南/sharing/Publish-Storybook.md (Publish Storybook, 138 行)
- 概念指南/sharing/Storybook-Composition.md (Storybook Composition, 137 行)
- 概念指南/test-storybooks/ember-cli.md (Ember Cli, 172 行)
- 概念指南/test-storybooks/external-docs.md (External Docs, 178 行)
- 概念指南/test-storybooks/external-docs/pages.md (Pages, 163 行)
- 概念指南/test-storybooks/server-kitchen-sink.md (Server Kitchen Sink, 166 行)
- 概念指南/test-storybooks/standalone-preview.md (Standalone Preview, 172 行)
- 概念指南/writing-docs/Autodocs.md (Autodocs, 146 行)
- 概念指南/writing-docs/Build-Documentation.md (Build Documentation, 138 行)
- 概念指南/writing-docs/Code-Panel.md (Code Panel, 138 行)
- 概念指南/writing-docs/Doc-Blocks.md (Doc Blocks, 138 行)
- 概念指南/writing-docs/Mdx.md (Mdx, 138 行)
- 概念指南/writing-stories.md (Writing Stories, 215 行)
- 概念指南/writing-stories/Args.md (Args, 138 行)
- 概念指南/writing-stories/Build-Pages-With-Storybook.md (Build Pages With Storybook, 139 行)
- 概念指南/writing-stories/Decorators.md (Decorators, 138 行)
- 概念指南/writing-stories/Loaders.md (Loaders, 138 行)
- 概念指南/writing-stories/Naming-Components-And-Hierarchy.md (Naming Components And Hierarchy, 139 行)
- 概念指南/writing-stories/Parameters.md (Parameters, 139 行)
- 概念指南/writing-stories/Play-Function.md (Play Function, 140 行)
- 概念指南/writing-stories/Stories-For-Multiple-Components.md (Stories For Multiple Components, 139 行)
- 概念指南/writing-stories/Tags.md (Tags, 137 行)
- 概念指南/writing-stories/Typescript.md (Typescript, 138 行)
- 概念指南/writing-stories/mocking-data-and-modules.md (Mocking Data And Modules, 141 行)
- 概念指南/writing-stories/mocking-data-and-modules/Mocking-Modules.md (Mocking Modules, 138 行)
- 概念指南/writing-stories/mocking-data-and-modules/Mocking-Network-Requests.md (Mocking Network Requests, 141 行)
- 概念指南/writing-stories/mocking-data-and-modules/Mocking-Providers.md (Mocking Providers, 162 行)
- 概念指南/writing-tests.md (Writing Tests, 213 行)
- 概念指南/writing-tests/Accessibility-Testing.md (Accessibility Testing, 140 行)
- 概念指南/writing-tests/In-Ci.md (In Ci, 138 行)
- 概念指南/writing-tests/Interaction-Testing.md (Interaction Testing, 141 行)
- 概念指南/writing-tests/Snapshot-Testing.md (Snapshot Testing, 140 行)
- 概念指南/writing-tests/Test-Coverage.md (Test Coverage, 140 行)
- 概念指南/writing-tests/Visual-Testing.md (Visual Testing, 140 行)
- 概念指南/writing-tests/integrations.md (Integrations, 162 行)
- 概念指南/writing-tests/integrations/Stories-In-End-To-End-Tests.md (Stories In End To End Tests, 138 行)
- 概念指南/writing-tests/integrations/Stories-In-Unit-Tests.md (Stories In Unit Tests, 144 行)
- 概念指南/writing-tests/integrations/Test-Runner.md (Test Runner, 138 行)
- 概念指南/writing-tests/integrations/vitest-addon.md (Vitest Addon, 182 行)
- 概念指南/writing-tests/integrations/vitest-addon/Migration-Guide.md (Migration Guide, 140 行)
- 概念指南/概念指南.md (概念指南, 132 行)
- 测试基础设施/test-storybooks.md (测试：test-storybooks, 132 行)
- 测试基础设施/测试基础设施.md (测试基础设施, 150 行)
- 测试基础设施/测试策略与最佳实践.md (测试策略与最佳实践, 172 行)
- 测试基础设施/测试策略与最佳实践/单元测试.md (单元测试, 132 行)
- 配置参考/Preview.md (Preview, 106 行)
- 配置参考/addons/Configure-Addons.md (Configure Addons, 141 行)
- 配置参考/configure.md (Configure, 212 行)
- 配置参考/configure/Environment-Variables.md (Environment Variables, 140 行)
- 配置参考/configure/Story-Layout.md (Story Layout, 141 行)
- 配置参考/configure/Story-Rendering.md (Story Rendering, 137 行)
- 配置参考/configure/Styling-And-Css.md (Styling And Css, 163 行)
- 配置参考/configure/Telemetry.md (Telemetry, 139 行)
- 配置参考/configure/Webpack.md (Webpack, 173 行)
- 配置参考/configure/integration/Compilers.md (Compilers, 140 行)
- 配置参考/configure/integration/Eslint-Plugin.md (Eslint Plugin, 138 行)
- 配置参考/configure/integration/Frameworks-Feature-Support.md (Frameworks Feature Support, 138 行)
- 配置参考/configure/integration/Frameworks.md (Frameworks, 140 行)
- 配置参考/configure/integration/Images-And-Assets.md (Images And Assets, 138 行)
- 配置参考/configure/integration/Typescript.md (Typescript, 138 行)
- 配置参考/configure/user-interface/Features-And-Behavior.md (Features And Behavior, 140 行)
- 配置参考/configure/user-interface/Sidebar-And-Urls.md (Sidebar And Urls, 140 行)
- 配置参考/configure/user-interface/Storybook-Addons.md (Storybook Addons, 140 行)
- 配置参考/configure/user-interface/Theming.md (Theming, 138 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考/API参考.md | 多框架支持/多框架支持.md | 368/178 | 101/44 | 0/5 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js |
| API参考/CLI命令参考.md | 核心运行时/core.md | 280/303 | 82/217 | 0/5 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、migration.md、get-storybook-configuration.ts、index.js、initiate.ts、package.js、cli-step.ts、node.js |
| API参考/开发API参考/Addon API.md | API-参考/开发API参考/插件API.md | 294/540 | 113/454 | 0/5 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、index.test.ts、addons.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md |
| API参考/开发API参考/CSF API.md | API-参考/开发API参考/CSF-API.md | 277/540 | 131/454 | 0/5 | 7/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts |
| API参考/开发API参考/Decorators API.md | 插件生态/pseudo-states.md | 248/170 | 92/50 | 13/12 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts |
| API参考/开发API参考/Hooks API.md | 核心模块/code.md | 343/285 | 105/196 | 0/5 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：hooks.test.js、hooks.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts |
| API参考/开发API参考/Preview API.md | API-参考/开发API参考/Preview-API.md | 231/540 | 108/454 | 0/5 | 7/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preview-api.ts、previewweb.ts、webview.ts、simulate-pageload.ts、preview-api.spec.ts |
| API参考/开发API参考/Store API.md | API-参考/开发API参考/Store-API.md | 397/540 | 148/454 | 0/5 | 10/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts |
| API参考/开发API参考/Types API.md | API-参考/api.md | 273/203 | 82/52 | 0/13 | 4/1 | 图表少于 reference；缺少关键文件提及：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts |
| API参考/开发API参考/开发API参考.md | API-参考/开发API参考.md | 318/581 | 107/460 | 0/6 | 7/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts |
| API参考/类型定义参考/API类型定义.md | API-参考/类型定义参考/API类型定义.md | 284/540 | 122/454 | 0/5 | 5/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts |
| API参考/类型定义参考/工具类型定义.md | API-参考/类型定义参考/工具类型定义.md | 296/540 | 172/454 | 16/5 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：types.ts、root.ts、store.ts、hooks.ts、features.ts |
| API参考/类型定义参考/插件类型定义.md | 核心运行时/核心运行时.md | 430/153 | 214/34 | 17/5 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：codegen-set-addon-channel.ts、addons.ts、addons.test.js、typings.d.ts |
| API参考/类型定义参考/构建器类型定义.md | 构建系统/构建系统.md | 324/165 | 240/42 | 17/5 | 9/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js、vite.conf |
| API参考/类型定义参考/核心类型定义.md | 插件生态/pseudo-states.md | 358/170 | 255/50 | 18/12 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts |
| API参考/类型定义参考/框架类型定义.md | API-参考/类型定义参考/框架类型定义.md | 337/540 | 139/454 | 17/5 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts |
| API参考/类型定义参考/类型定义参考.md | API-参考/类型定义参考.md | 480/580 | 362/460 | 22/6 | 12/1 | 图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts |
| API参考/配置API参考/main.js配置.md | 配置参考/configure/integration.md | 407/169 | 130/55 | 0/15 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、storybook-composition.md、main.js |
| API参考/配置API参考/manager.js配置.md | 配置参考/configure/user-interface.md | 254/146 | 100/55 | 16/6 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md |
| API参考/配置API参考/preview.js配置.md | API-参考/开发API参考.md | 351/581 | 127/460 | 19/6 | 9/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js、builder-api-preview-exports.md |
| API参考/配置API参考/构建器配置.md | 概念指南/builders.md | 245/191 | 94/52 | 0/13 | 5/1 | 图表少于 reference；缺少关键文件提及：main.ts、package.js、build.ts、vite-config.ts、base-webpack.config.ts、builder.ts、merge-webpack-config.test.ts、storybook-addons-preset-webpackfinal.md |
| API参考/配置API参考/配置API参考.md | API-参考/组件故事-Stories.md | 260/574 | 71/448 | 0/12 | 3/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts |
| API参考/配置API参考/预设配置.md | 构建系统/构建系统.md | 282/165 | 108/42 | 0/5 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：presets.test.ts、presets.ts、build-static.ts、load.ts、configfile.ts |
| 主题和外观/主题和外观.md | 配置参考/主题和外观.md | 359/197 | 132/49 | 0/15 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts |
| 主题和外观/主题系统概览.md | 配置参考/主题和外观/主题系统概览.md | 302/124 | 113/53 | 0/5 | 6/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js |
| 主题和外观/布局和间距设计.md | 主题系统/themes.md | 185/137 | 64/51 | 0/5 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js |
| 主题和外观/自定义主题开发.md | 配置参考/主题和外观/自定义主题开发.md | 372/124 | 123/53 | 0/5 | 5/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、index.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts |
| 主题和外观/颜色和字体系统.md | 配置参考/主题和外观/颜色和字体系统.md | 244/124 | 90/53 | 13/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、manager.js、preset.js、preview.js、accessibilityrulemaps.ts、a11yrunner.test.ts |
| 多框架支持/Angular框架支持.md | API-参考/Angular框架支持.md | 287/540 | 119/454 | 0/5 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、decorators.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、tsconfig.js、zone.js |
| 多框架支持/HTML框架支持.md | 核心模块/code.md | 215/285 | 89/196 | 0/5 | 4/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：build-config.ts、package.js、sandbox-templates.ts、render.ts |
| 多框架支持/React框架支持.md | 多框架支持/多框架支持.md | 349/178 | 225/44 | 16/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js、tsconfig.js |
| 多框架支持/Svelte框架支持.md | 多框架支持/svelte-vite.md | 355/180 | 233/94 | 15/5 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts |
| 多框架支持/Vue 3框架支持.md | 构建系统/vue3-vite.md | 279/147 | 115/61 | 15/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts |
| 多框架支持/Web Components框架支持.md | 平台绑定-Web/web-components.md | 300/142 | 223/56 | 13/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts |
| 多框架支持/多框架支持.md | 多框架支持/多框架支持.md | 259/178 | 87/44 | 0/5 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、package.js |
| 快速开始.md | API-参考/组件故事-Stories.md | 264/574 | 120/448 | 0/12 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js |
| 插件系统/Addons概览.md | 插件生态/插件生态.md | 276/170 | 97/44 | 16/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、package.js |
| 插件系统/插件系统.md | 插件生态/插件生态.md | 356/170 | 107/44 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js |
| 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md | 插件生态/a11y.md | 335/158 | 174/72 | 17/5 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、visionsimulator.ts、constants.ts、params.ts、preview.ts、types.ts |
| 插件系统/核心Addons详解/Actions Addon（动作记录）.md | 插件生态/pseudo-states.md | 354/170 | 162/50 | 0/12 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、manager.ts、actiondisplay.ts、actionoptions.ts |
| 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md | 概念指南/essentials/Backgrounds.md | 355/138 | 126/45 | 0/12 | 5/1 | 内容明显短于 reference；解释性段落明显不足；图表少于 reference；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts |
| 插件系统/核心Addons详解/Controls Addon（参数控制）.md | 概念指南/README.md | 285/219 | 131/61 | 0/12 | 7/1 | 图表少于 reference；缺少关键文件提及：changelog.md、argcontrol.ts、constants.ts、preview.ts、app.ts、panel.ts、addon-controls.spec.ts、arg-types.md |
| 插件系统/核心Addons详解/Docs Addon（文档生成）.md | 插件生态/pseudo-states.md | 239/170 | 86/50 | 0/12 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md |
| 插件系统/核心Addons详解/Links Addon（故事导航）.md | 插件生态/links.md | 362/145 | 125/59 | 0/5 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、preview.ts、utils.test.ts、utils.ts、decorator.stories.ts |
| 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md | 概念指南/addons.md | 399/212 | 153/52 | 0/13 | 6/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：tabs.hooks.ts、common-manager.ts、common-preset.ts、layout.ts、shortcuts.ts、tool.ts、constants.ts、manager.ts |
| 插件系统/核心Addons详解/Themes Addon（主题切换）.md | 配置参考/主题和外观.md | 279/197 | 120/49 | 0/15 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md |
| 插件系统/核心Addons详解/核心Addons详解.md | 配置参考/配置参考.md | 342/145 | 88/38 | 0/5 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.yarnrc.yml、manager.ts、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts、addon-toolbars.spec.ts |
| 插件系统/第三方Addons集成.md | 概念指南/addons.md | 246/212 | 98/52 | 20/13 | 6/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、package.js、index.ts |
| 插件系统/自定义Addons开发.md | 插件生态/addons.md | 304/140 | 108/54 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts |
| 故障排除/兼容性问题.md | 概念指南/get-started.md | 260/167 | 80/54 | 0/9 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：common.md、ember.md、frameworks-overview.md、nextjs.md、preact.md、react-vue-angular.md、renderers.md、svelte.md |
| 故障排除/安装问题.md | 开发工具/create-storybook.md | 350/152 | 106/66 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts |
| 故障排除/故障排除.md | 故障排除/故障排除.md | 348/85 | 106/30 | 0/5 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：main.ts、readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts |
| 故障排除/构建和性能问题.md | 构建系统/构建系统.md | 312/165 | 98/42 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、vite-server.ts、base-webpack.config.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts |
| 故障排除/调试工具和技巧.md | 核心模块/核心模块.md | 295/176 | 121/44 | 0/5 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、context-in-play-function.md、context-in-play-function.ts |
| 故障排除/运行时错误.md | 核心模块/核心模块.md | 386/176 | 108/44 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：globalerrormodal.ts、decoratorfunction.ts、errorformatter.ts、instrumenter.ts、types.ts、manager-errors.ts、errors.stories.ts、preview-errors.ts |
| 故障排除/配置错误.md | 配置参考/配置参考.md | 299/145 | 92/38 | 0/5 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts |
| 构建系统/Vite构建器详解.md | 构建系统/builder-vite.md | 300/185 | 120/74 | 0/10 | 9/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts |
| 构建系统/Webpack构建器详解.md | 构建系统/构建系统.md | 339/165 | 138/42 | 17/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、custom-webpack-preset.ts、preview-preset.ts、iframe-webpack.config.ts、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts |
| 构建系统/构建器概览.md | 构建系统/builder-webpack5.md | 321/200 | 113/114 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf |
| 构建系统/构建系统.md | 构建系统/构建系统.md | 291/165 | 103/42 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts、preview-preset.ts |
| 构建系统/构建配置优化.md | 核心模块/code.md | 264/285 | 70/196 | 0/5 | 4/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、build.ts、chromatic.config.js、build-config.ts、standalone.ts、tsconfig.js、package.js |
| 核心概念/Addons系统架构/Addons开发指南.md | 插件生态/插件生态.md | 584/170 | 390/44 | 23/5 | 13/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts |
| 核心概念/Addons系统架构/Addons架构设计.md | 系统架构.md | 321/166 | 146/41 | 0/4 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、channel.ts |
| 核心概念/Addons系统架构/Addons系统架构.md | 系统架构.md | 356/166 | 156/41 | 22/4 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md |
| 核心概念/Addons系统架构/Addons通信机制.md | 系统架构.md | 290/166 | 131/41 | 0/4 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：index.test.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts |
| 核心概念/Addons系统架构/Addons配置管理.md | 系统架构.md | 273/166 | 85/41 | 0/4 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：main.js、package.js、preset.ts、presets.ts |
| 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md | 插件生态/a11y.md | 362/158 | 125/72 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ycontext.test.ts、details.ts、report.stories.ts |
| 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md | 系统架构.md | 302/166 | 103/41 | 0/4 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、migration.md、constants.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts、addons.ts |
| 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md | 系统架构.md | 374/166 | 115/41 | 0/4 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts |
| 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md | 系统架构.md | 282/166 | 95/41 | 0/4 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts |
| 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md | 系统架构.md | 348/166 | 127/41 | 0/4 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、manager.ts、manifest.ts、mdx-react-shim.ts、types.ts |
| 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md | 主题系统/themes.md | 293/137 | 127/51 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、helpers.ts、provider.decorator.ts、preview.ts、types.ts、themes.md |
| 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md | 系统架构.md | 290/166 | 128/41 | 0/4 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、manager.ts、preview.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts |
| 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md | 系统架构.md | 257/166 | 98/41 | 0/4 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、addon-viewport.spec.ts |
| 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md | 系统架构.md | 309/166 | 113/41 | 7/4 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts |
| 核心概念/CSF格式规范.md | 概念指南/writing-docs.md | 302/201 | 115/50 | 0/13 | 5/1 | 图表少于 reference；缺少关键文件提及：inject-decorator.ts、csffile.test.ts、csffile.ts、readme.md、build-config.ts、csf-2-to-3.ts、project.js、csf-component.md |
| 核心概念/主题和参数系统.md | 配置参考/主题和外观.md | 356/197 | 126/49 | 0/15 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts |
| 核心概念/核心概念.md | API-参考/组件故事-Stories.md | 388/574 | 139/448 | 16/12 | 7/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md |
| 核心概念/组件故事（Stories）.md | API-参考/组件故事-Stories.md | 244/574 | 85/448 | 0/12 | 4/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.stories.ts、canvas.stories.ts、story.stories.ts、csffile.ts、getstorysortparameter.ts、stories.ts、readme-store.md、package.js |
| 核心概念/装饰器和全局状态.md | API-参考/组件故事-Stories.md | 307/574 | 119/448 | 0/12 | 7/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js |
| 核心概念/预览和管理界面.md | 核心模块/code.md | 417/285 | 154/196 | 0/5 | 7/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts |
| 测试框架/Playwright测试.md | 配置参考/主题和外观.md | 382/197 | 118/49 | 0/15 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、chromatic.config.js、manager.spec.ts、navigation.spec.ts、preview-api.spec.ts、util.ts、playwright.config.ts |
| 测试框架/Vitest集成.md | API-参考/组件故事-Stories.md | 250/574 | 64/448 | 0/12 | 3/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storybook.setup.ts、component-tests.spec.ts、package.js、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts、playwright.config.ts |
| 测试框架/可访问性测试.md | 插件生态/a11y.md | 300/158 | 109/72 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、preview.ts、types.ts、utils.ts |
| 测试框架/测试框架.md | API-参考/组件故事-Stories.md | 361/574 | 136/448 | 0/12 | 8/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：chromatic.config.js、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts |
| 测试框架/组件测试.md | API-参考/组件故事-Stories.md | 317/574 | 121/448 | 20/12 | 8/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storybook.setup.ts、readme.md、extract.ts、test-fn.stories.ts、mocker-runtime.js、sb-module-mocking.spec.ts、vitest-setup.ts、vitest.config.storybook.ts |
| 测试框架/视觉回归测试.md | 配置参考/主题和外观.md | 328/197 | 135/49 | 0/15 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、chromatic.config.js、component-tests.spec.ts、playwright.config.ts、vitest.config.storybook.ts、node.js |
| 贡献指南.md | 概念指南/README.md | 228/219 | 77/61 | 0/12 | 3/1 | 图表少于 reference；缺少关键文件提及：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、security.md、package.js、playwright.config.ts、vitest.config.ts |
| 部署和CI_CD/CI_CD集成.md | 开发工具/scripts.md | 317/324 | 96/201 | 0/12 | 5/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、executors.ts、helpers.ts、parameters.ts、types.ts |
| 部署和CI_CD/Chromatic集成.md | 配置参考/Main.md | 242/106 | 123/46 | 18/12 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：ischromatic.ts、chromatic.config.js、get-chromatic-version.ts、package.js |
| 部署和CI_CD/环境配置.md | API-参考/组件故事-Stories.md | 299/574 | 97/448 | 17/12 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、codecov.yml、dependabot.yml、nx.js、package.js |
| 部署和CI_CD/部署和CI_CD.md | 核心模块/storybook.md | 264/141 | 105/48 | 0/7 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、readme.md、chromatic.config.js、package.js、project.js |
| 部署和CI_CD/静态部署.md | API-参考/组件故事-Stories.md | 201/574 | 63/448 | 0/12 | 4/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、manager.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts |
| 项目概述/使用场景和案例.md | 概念指南/Docs.md | 271/213 | 100/52 | 0/13 | 6/1 | 图表少于 reference；缺少关键文件提及：main.ts、preview.ts、readme.md、package.js |
| 项目概述/基本概念.md | 项目概述.md | 280/224 | 80/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：button.stories.ts、storybook-addons.md |
| 项目概述/技术架构/Monorepo设计.md | 项目概述.md | 249/224 | 78/51 | 0/6 | 3/1 | 图表少于 reference；缺少关键文件提及：package.js、project.js、nx.js、check-package.ts、prepare-sandbox.ts |
| 项目概述/技术架构/技术架构.md | 项目概述.md | 289/224 | 141/51 | 0/6 | 8/1 | 解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、manager-stores.ts、next.js、index.js、core.js |
| 项目概述/技术架构/插件系统架构/插件架构设计.md | 项目概述.md | 346/224 | 158/51 | 17/6 | 8/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md |
| 项目概述/技术架构/插件系统架构/插件注册机制.md | 项目概述.md | 263/224 | 100/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md |
| 项目概述/技术架构/插件系统架构/插件生命周期管理.md | 项目概述.md | 266/224 | 94/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：manager.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts、postinstalladdon.ts |
| 项目概述/技术架构/插件系统架构/插件系统架构.md | 插件生态/插件生态.md | 315/170 | 127/44 | 0/5 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md、storybook-addons-root-preset.md |
| 项目概述/技术架构/插件系统架构/插件通信协议.md | 项目概述.md | 312/224 | 136/51 | 0/6 | 7/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：index.test.ts、types.ts |
| 项目概述/技术架构/构建系统架构/Vite构建器.md | 构建系统/builder-vite.md | 283/185 | 127/74 | 0/10 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、vite-config.ts、vite-server.ts、vite.conf |
| 项目概述/技术架构/构建系统架构/Webpack构建器.md | 项目概述.md | 242/224 | 84/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：package.js、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js |
| 项目概述/技术架构/构建系统架构/构建器架构设计.md | 构建系统/构建系统.md | 363/165 | 146/42 | 0/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts、frameworkdetectionservice.ts |
| 项目概述/技术架构/构建系统架构/构建系统架构.md | 构建系统/builders.md | 244/130 | 105/44 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts |
| 项目概述/技术架构/核心引擎架构/CLI分发器.md | 项目概述.md | 308/224 | 152/51 | 0/6 | 9/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、upgrade.ts、node.js |
| 项目概述/技术架构/核心引擎架构/全局设置.md | 项目概述.md | 360/224 | 143/51 | 18/6 | 7/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts、util.ts |
| 项目概述/技术架构/核心引擎架构/核心引擎架构.md | 项目概述.md | 272/224 | 122/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：dispatcher.js、build-config.ts、package.js、tsconfig.js |
| 项目概述/技术架构/核心引擎架构/核心服务器.md | 项目概述.md | 334/224 | 131/51 | 0/6 | 6/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：build-config.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts、storybook-builder-api-dev-server.md、storybook-builder-api-interface.md |
| 项目概述/技术架构/核心引擎架构/版本管理.md | 项目概述.md | 359/224 | 137/51 | 0/6 | 8/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts |
| 项目概述/技术架构/框架适配器架构/Angular框架适配器.md | 项目概述.md | 355/224 | 245/51 | 17/6 | 8/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：build-config.ts、package.js、project.js、error-handler.ts、run-compodoc.ts、standalone-options.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts |
| 项目概述/技术架构/框架适配器架构/HTML框架适配器.md | 项目概述.md | 245/224 | 99/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：.stories.ts、project.js、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js、types.ts |
| 项目概述/技术架构/框架适配器架构/React框架适配器.md | 项目概述.md | 326/224 | 239/51 | 17/6 | 9/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、types.ts、public-types.ts、tsconfig.js |
| 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md | 多框架支持/svelte-vite.md | 287/180 | 131/94 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、utils.ts、portable-stories.ts、public-types.ts |
| 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md | 构建系统/vue3-vite.md | 305/147 | 212/61 | 15/5 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、vue-docgen.ts、types.ts、preview.ts、public-types.ts、render.ts、tsconfig.js |
| 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md | 项目概述.md | 478/224 | 315/51 | 17/6 | 9/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：...conf、package.js、types.ts、framework-api.ts、globals.ts、public-types.ts |
| 项目概述/技术架构/框架适配器架构/框架适配器架构.md | 项目概述.md | 297/224 | 136/51 | 0/6 | 8/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：package.js、react-docgen.ts、types.ts、next.js |
| 项目概述/技术架构/渲染系统架构/HTML渲染器.md | 核心模块/核心模块.md | 250/176 | 108/44 | 14/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、globals.ts、portable-stories.ts、public-types.ts、types.ts |
| 项目概述/技术架构/渲染系统架构/React渲染器.md | 项目概述.md | 238/224 | 91/51 | 12/6 | 4/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：docsrenderer.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts |
| 项目概述/技术架构/渲染系统架构/Svelte渲染器.md | 核心模块/核心模块.md | 319/176 | 149/44 | 0/5 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、mount.ts、portable-stories.ts、public-types.ts、types.ts |
| 项目概述/技术架构/渲染系统架构/Vue3渲染器.md | 项目概述.md | 217/224 | 83/51 | 0/6 | 6/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、render.ts |
| 项目概述/技术架构/渲染系统架构/渲染系统架构.md | 项目概述.md | 267/224 | 130/51 | 0/6 | 7/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：manager.ts、package.js、manager-stores.ts、globals.ts |
| 项目概述/核心特性/主题定制系统/主题切换机制.md | 项目概述.md | 303/224 | 101/51 | 0/6 | 6/1 | 图表少于 reference；缺少关键文件提及：manager.ts、create.ts、utils.ts |
| 项目概述/核心特性/主题定制系统/主题定制系统.md | 项目概述.md | 275/224 | 143/51 | 0/6 | 6/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、create.ts、types.ts |
| 项目概述/核心特性/主题定制系统/主题扩展开发.md | 项目概述.md | 343/224 | 119/51 | 0/6 | 6/1 | 图表少于 reference；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts |
| 项目概述/核心特性/主题定制系统/字体系统.md | 项目概述.md | 221/224 | 69/51 | 0/6 | 3/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts |
| 项目概述/核心特性/主题定制系统/布局系统.md | 项目概述.md | 314/224 | 121/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts、layout.stories.ts |
| 项目概述/核心特性/主题定制系统/颜色系统.md | 项目概述.md | 192/224 | 52/51 | 0/6 | 3/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts |
| 项目概述/核心特性/多框架支持/Angular框架支持.md | API-参考/Angular框架支持.md | 302/540 | 137/454 | 22/5 | 8/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、config.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、preset-options.ts、tsconfig.js |
| 项目概述/核心特性/多框架支持/Next.js框架支持.md | 核心模块/frameworks.md | 237/154 | 109/68 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-nextjs.spec.ts、types.ts、package.js、decorator.ts、next.config.js |
| 项目概述/核心特性/多框架支持/React框架支持.md | 多框架支持/多框架支持.md | 296/178 | 107/44 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、react-docgen.ts、preview.ts、public-types.ts、types.ts |
| 项目概述/核心特性/多框架支持/Svelte框架支持.md | 多框架支持/svelte-vite.md | 299/180 | 106/94 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts |
| 项目概述/核心特性/多框架支持/Vue框架支持.md | 项目概述.md | 234/224 | 129/51 | 12/6 | 6/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js |
| 项目概述/核心特性/多框架支持/Web Components框架支持.md | 项目概述.md | 268/224 | 110/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：button.js、button.stories.js、build-config.ts、package.js |
| 项目概述/核心特性/多框架支持/其他框架支持.md | 项目概述.md | 309/224 | 119/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：package.js、frameworks.js |
| 项目概述/核心特性/多框架支持/多框架支持.md | 多框架支持/多框架支持.md | 494/178 | 123/44 | 0/5 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、preview-prod.ts、preview.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts |
| 项目概述/核心特性/插件系统架构/Addon API设计.md | 项目概述.md | 262/224 | 98/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：params.ts、types.ts、docsrenderer.ts、manager.ts |
| 项目概述/核心特性/插件系统架构/Manager API.md | 项目概述.md | 335/224 | 124/51 | 17/6 | 7/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.js、manager.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md |
| 项目概述/核心特性/插件系统架构/Preview API.md | API-参考/开发API参考/Preview-API.md | 267/540 | 101/454 | 15/5 | 5/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js |
| 项目概述/核心特性/插件系统架构/插件生命周期管理.md | 项目概述.md | 322/224 | 119/51 | 0/6 | 8/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md |
| 项目概述/核心特性/插件系统架构/插件系统架构.md | 项目概述.md | 417/224 | 167/51 | 0/6 | 10/1 | 解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、manager.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts、hooks.ts |
| 项目概述/核心特性/插件系统架构/通信机制.md | 项目概述.md | 281/224 | 128/51 | 0/6 | 7/1 | 解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：geteventsourceurl.ts、types.ts、addons.ts、app.ts |
| 项目概述/核心特性/文档生成功能/Doc Blocks API.md | 项目概述.md | 389/224 | 121/51 | 0/6 | 3/1 | 图表少于 reference；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts |
| 项目概述/核心特性/文档生成功能/MDX文档编写.md | 概念指南/writing-docs.md | 269/201 | 121/50 | 19/13 | 8/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md |
| 项目概述/核心特性/文档生成功能/文档生成功能.md | 概念指南/writing-docs.md | 341/201 | 129/50 | 17/13 | 7/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts |
| 项目概述/核心特性/文档生成功能/自动文档系统.md | 项目概述.md | 267/224 | 118/51 | 0/6 | 7/1 | 图表少于 reference；缺少关键文件提及：docspage.md、mdx.md、csf-plugin.ts、package.js、shared.ts |
| 项目概述/核心特性/文档生成功能/自定义文档页面.md | 项目概述.md | 260/224 | 110/51 | 18/6 | 6/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.ts、metaof.md、title.md、create.ts |
| 项目概述/核心特性/核心特性.md | 项目概述.md | 349/224 | 138/51 | 0/6 | 7/1 | 解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts、render-components-manifest.ts、menu.ts |
| 项目概述/核心特性/测试支持功能/Playwright测试.md | 项目概述.md | 373/224 | 157/51 | 0/6 | 8/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js |
| 项目概述/核心特性/测试支持功能/Vitest集成.md | 项目概述.md | 291/224 | 114/51 | 23/6 | 7/1 | 图表少于 reference；evidence block 少于 reference；缺少关键文件提及：.spec.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts、vitest.shared.ts |
| 项目概述/核心特性/测试支持功能/可访问性测试.md | 项目概述.md | 311/224 | 136/51 | 0/6 | 6/1 | 解释性段落明显不足；图表少于 reference；缺少关键文件提及：package.js、constants.ts、manager.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf |
| 项目概述/核心特性/测试支持功能/测试支持功能.md | 项目概述.md | 338/224 | 133/51 | 21/6 | 7/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：.spec.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts |
| 项目概述/核心特性/测试支持功能/组件测试.md | 项目概述.md | 266/224 | 106/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts、jest.config.js |
| 项目概述/核心特性/组件开发环境/交互式调试.md | 项目概述.md | 264/224 | 110/51 | 0/6 | 7/1 | 图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.manager.ts、manager.ts、constants.ts、panel.ts |
| 项目概述/核心特性/组件开发环境/实时预览.md | 项目概述.md | 311/224 | 108/51 | 0/6 | 5/1 | 图表少于 reference；缺少关键文件提及：manager.ts、story.ts、geteventsourceurl.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts、storystore.ts、render.ts |
| 项目概述/核心特性/组件开发环境/开发服务器.md | 项目概述.md | 303/224 | 124/51 | 0/6 | 6/1 | 图表少于 reference；缺少关键文件提及：logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts、middleware.ts |
| 项目概述/核心特性/组件开发环境/组件开发环境.md | 项目概述.md | 312/224 | 139/51 | 21/6 | 8/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、types.ts、package.js |
| 项目概述/核心特性/组件开发环境/组件生命周期.md | 项目概述.md | 319/224 | 122/51 | 16/6 | 6/1 | 图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts |
| 项目概述/项目概述.md | 项目概述.md | 312/224 | 131/51 | 21/6 | 8/1 | 解释性段落明显不足；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributing.md、package.js、tsconfig.js |
| 高级功能/工具集成/CI_CD集成.md | 系统架构.md | 299/166 | 127/41 | 20/4 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：config.generated.yml、config.yml、chromatic.config.js、package.js、ghp-github-action.md、test-runner-local-build-workflow.md、in-ci.md、common-jobs.ts |
| 高级功能/工具集成/Codemod工具.md | 核心模块/核心模块.md | 286/176 | 114/44 | 18/5 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts |
| 高级功能/工具集成/ESLint集成.md | 插件生态/eslint-plugin.md | 269/173 | 69/59 | 0/12 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts |
| 高级功能/工具集成/IDE配置优化.md | 配置参考/主题和外观.md | 339/197 | 105/49 | 15/15 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、main.ts、manager.ts、preview.ts、storybook.setup.ts、vitest.config.storybook.ts、prettier.conf、prettier.config.js |
| 高级功能/工具集成/工具集成.md | API-参考/组件故事-Stories.md | 313/574 | 105/448 | 0/12 | 6/1 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、package.js、prettier.conf、tsconfig.js、prettier.config.js、codemod.ts |
| 高级功能/工具集成/构建工具优化.md | 构建系统/构建系统.md | 287/165 | 93/42 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、virtual-module-mapping.ts、environments-support.ts、generate-bundle.ts |
| 高级功能/性能监控.md | 核心模块/code.md | 340/285 | 165/196 | 0/5 | 8/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、types.ts、utils.ts、bench.ts、upload-bench.ts |
| 高级功能/扩展开发/Addon开发.md | 插件生态/addons.md | 327/140 | 120/54 | 0/5 | 8/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preview.ts |
| 高级功能/扩展开发/主题开发.md | 主题系统/themes.md | 320/137 | 149/51 | 20/5 | 9/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js |
| 高级功能/扩展开发/扩展开发.md | 配置参考/主题和外观.md | 243/197 | 106/49 | 0/15 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、preview.ts |
| 高级功能/扩展开发/构建器扩展.md | 构建系统/构建系统.md | 276/165 | 105/42 | 0/5 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、builder.ts、storybook-builder-api-interface.md、storybook-main-versioned-webpack.md、builder-api.md、vite.conf |
| 高级功能/扩展开发/渲染器扩展.md | 核心模块/核心模块.md | 333/176 | 127/44 | 0/5 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts |
| 高级功能/扩展开发/预设开发.md | 核心模块/code.md | 292/285 | 123/196 | 0/5 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js |
| 高级功能/自定义渲染器.md | 多框架支持/renderers.md | 324/140 | 98/54 | 0/5 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preview.ts、render.ts、types.ts |
| 高级功能/预设配置.md | 插件生态/presets.md | 288/132 | 110/46 | 0/5 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：presets.ts、package.js、types.ts、index.js |
| 高级功能/高级功能.md | 多框架支持/多框架支持.md | 375/178 | 139/44 | 0/5 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、package.js、build.ts、postinstalladdon.ts、prettier.conf |

## 逐文件详情

### API参考/API参考.md

- reference 标题：API参考
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：136
- 页面类型：other / other
- 行数：368 / 178
- 段落行数：101 / 44
- Evidence：0 / 5
- Mermaid：4 / 1
- 文件提及重合：main.ts、index.ts、preview.js
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js

### API参考/CLI命令参考.md

- reference 标题：CLI命令参考
- 生成页：核心运行时/core.md（core）
- 匹配分数：124
- 页面类型：other / other
- 行数：280 / 303
- 段落行数：82 / 217
- Evidence：0 / 5
- Mermaid：4 / 1
- 文件提及重合：core.ts、dispatcher.ts
- reference 关键文件未覆盖：changelog.md、migration.md、get-storybook-configuration.ts、index.js、initiate.ts、package.js、cli-step.ts、node.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、migration.md、get-storybook-configuration.ts、index.js、initiate.ts、package.js、cli-step.ts、node.js

### API参考/开发API参考/Addon API.md

- reference 标题：插件API
- 生成页：API-参考/开发API参考/插件API.md（插件API）
- 匹配分数：254
- 页面类型：other / other
- 行数：294 / 540
- 段落行数：113 / 454
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、types.ts、index.test.ts、addons.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、index.test.ts、addons.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md

### API参考/开发API参考/CSF API.md

- reference 标题：CSF API
- 生成页：API-参考/开发API参考/CSF-API.md（CSF API）
- 匹配分数：270
- 页面类型：other / other
- 行数：277 / 540
- 段落行数：131 / 454
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts

### API参考/开发API参考/Decorators API.md

- reference 标题：装饰器API
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- 匹配分数：120
- 页面类型：other / other
- 行数：248 / 170
- 段落行数：92 / 50
- Evidence：13 / 12
- Mermaid：5 / 1
- 文件提及重合：preview.ts、index.ts
- reference 关键文件未覆盖：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts

### API参考/开发API参考/Hooks API.md

- reference 标题：Hooks API
- 生成页：核心模块/code.md（code）
- 匹配分数：68
- 页面类型：other / module
- 行数：343 / 285
- 段落行数：105 / 196
- Evidence：0 / 5
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：hooks.test.js、hooks.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：hooks.test.js、hooks.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts

### API参考/开发API参考/Preview API.md

- reference 标题：Preview API
- 生成页：API-参考/开发API参考/Preview-API.md（Preview API）
- 匹配分数：268
- 页面类型：other / other
- 行数：231 / 540
- 段落行数：108 / 454
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：preview-api.ts、previewweb.ts、webview.ts、simulate-pageload.ts、preview-api.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preview-api.ts、previewweb.ts、webview.ts、simulate-pageload.ts、preview-api.spec.ts

### API参考/开发API参考/Store API.md

- reference 标题：Store API
- 生成页：API-参考/开发API参考/Store-API.md（Store API）
- 匹配分数：268
- 页面类型：other / other
- 行数：397 / 540
- 段落行数：148 / 454
- Evidence：0 / 5
- Mermaid：10 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts

### API参考/开发API参考/Types API.md

- reference 标题：Types API
- 生成页：API-参考/api.md（Api）
- 匹配分数：80
- 页面类型：other / other
- 行数：273 / 203
- 段落行数：82 / 52
- Evidence：0 / 13
- Mermaid：4 / 1
- 文件提及重合：arg-types.md
- reference 关键文件未覆盖：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts
- 结论：图表少于 reference；缺少关键文件提及：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts

### API参考/开发API参考/开发API参考.md

- reference 标题：开发API参考
- 生成页：API-参考/开发API参考.md（开发API参考）
- 匹配分数：292
- 页面类型：other / other
- 行数：318 / 581
- 段落行数：107 / 460
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、manager.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts

### API参考/类型定义参考/API类型定义.md

- reference 标题：API类型定义
- 生成页：API-参考/类型定义参考/API类型定义.md（API类型定义）
- 匹配分数：252
- 页面类型：other / other
- 行数：284 / 540
- 段落行数：122 / 454
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts

### API参考/类型定义参考/工具类型定义.md

- reference 标题：工具类型定义
- 生成页：API-参考/类型定义参考/工具类型定义.md（工具类型定义）
- 匹配分数：264
- 页面类型：other / other
- 行数：296 / 540
- 段落行数：172 / 454
- Evidence：16 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：types.ts、root.ts、store.ts、hooks.ts、features.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：types.ts、root.ts、store.ts、hooks.ts、features.ts

### API参考/类型定义参考/插件类型定义.md

- reference 标题：插件类型定义
- 生成页：核心运行时/核心运行时.md（核心运行时）
- 匹配分数：78
- 页面类型：other / other
- 行数：430 / 153
- 段落行数：214 / 34
- Evidence：17 / 5
- Mermaid：8 / 1
- 文件提及重合：types.ts、main.ts
- reference 关键文件未覆盖：codegen-set-addon-channel.ts、addons.ts、addons.test.js、typings.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：codegen-set-addon-channel.ts、addons.ts、addons.test.js、typings.d.ts

### API参考/类型定义参考/构建器类型定义.md

- reference 标题：构建器类型定义
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：104
- 页面类型：other / other
- 行数：324 / 165
- 段落行数：240 / 42
- Evidence：17 / 5
- Mermaid：9 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js、vite.conf
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js、vite.conf

### API参考/类型定义参考/核心类型定义.md

- reference 标题：核心类型定义
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- 匹配分数：84
- 页面类型：other / other
- 行数：358 / 170
- 段落行数：255 / 50
- Evidence：18 / 12
- Mermaid：8 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts

### API参考/类型定义参考/框架类型定义.md

- reference 标题：框架类型定义
- 生成页：API-参考/类型定义参考/框架类型定义.md（框架类型定义）
- 匹配分数：276
- 页面类型：other / other
- 行数：337 / 540
- 段落行数：139 / 454
- Evidence：17 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts

### API参考/类型定义参考/类型定义参考.md

- reference 标题：类型定义参考
- 生成页：API-参考/类型定义参考.md（类型定义参考）
- 匹配分数：310
- 页面类型：other / other
- 行数：480 / 580
- 段落行数：362 / 460
- Evidence：22 / 6
- Mermaid：12 / 1
- 文件提及重合：types.ts、typings.d.ts
- reference 关键文件未覆盖：main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts
- 结论：图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts

### API参考/配置API参考/main.js配置.md

- reference 标题：main.js配置
- 生成页：配置参考/configure/integration.md（Integration）
- 匹配分数：126
- 页面类型：other / other
- 行数：407 / 169
- 段落行数：130 / 55
- Evidence：0 / 15
- Mermaid：8 / 1
- 文件提及重合：main.ts、index.md、frameworks.md、images-and-assets.md
- reference 关键文件未覆盖：main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、storybook-composition.md、main.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、storybook-composition.md、main.js

### API参考/配置API参考/manager.js配置.md

- reference 标题：manager.js配置
- 生成页：配置参考/configure/user-interface.md（User Interface）
- 匹配分数：84
- 页面类型：other / other
- 行数：254 / 146
- 段落行数：100 / 55
- Evidence：16 / 6
- Mermaid：7 / 1
- 文件提及重合：features-and-behavior.md、manager.js
- reference 关键文件未覆盖：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md

### API参考/配置API参考/preview.js配置.md

- reference 标题：preview.js配置
- 生成页：API-参考/开发API参考.md（开发API参考）
- 匹配分数：92
- 页面类型：other / other
- 行数：351 / 581
- 段落行数：127 / 460
- Evidence：19 / 6
- Mermaid：9 / 1
- 文件提及重合：preview.ts、preview.js
- reference 关键文件未覆盖：basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js、builder-api-preview-exports.md
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js、builder-api-preview-exports.md

### API参考/配置API参考/构建器配置.md

- reference 标题：构建器配置
- 生成页：概念指南/builders.md（Builders）
- 匹配分数：70
- 页面类型：other / other
- 行数：245 / 191
- 段落行数：94 / 52
- Evidence：0 / 13
- Mermaid：5 / 1
- 文件提及重合：vite.md
- reference 关键文件未覆盖：main.ts、package.js、build.ts、vite-config.ts、base-webpack.config.ts、builder.ts、merge-webpack-config.test.ts、storybook-addons-preset-webpackfinal.md
- 结论：图表少于 reference；缺少关键文件提及：main.ts、package.js、build.ts、vite-config.ts、base-webpack.config.ts、builder.ts、merge-webpack-config.test.ts、storybook-addons-preset-webpackfinal.md

### API参考/配置API参考/配置API参考.md

- reference 标题：配置API参考
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：68
- 页面类型：other / other
- 行数：260 / 574
- 段落行数：71 / 448
- Evidence：0 / 12
- Mermaid：3 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts

### API参考/配置API参考/预设配置.md

- reference 标题：预设配置
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：110
- 页面类型：other / other
- 行数：282 / 165
- 段落行数：108 / 42
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：preset.ts、preset.js
- reference 关键文件未覆盖：presets.test.ts、presets.ts、build-static.ts、load.ts、configfile.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：presets.test.ts、presets.ts、build-static.ts、load.ts、configfile.ts

### 主题和外观/主题和外观.md

- reference 标题：主题和外观
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：338
- 页面类型：topic / topic
- 行数：359 / 197
- 段落行数：132 / 49
- Evidence：0 / 15
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts

### 主题和外观/主题系统概览.md

- reference 标题：主题系统概览
- 生成页：配置参考/主题和外观/主题系统概览.md（主题系统概览）
- 匹配分数：328
- 页面类型：topic / topic
- 行数：302 / 124
- 段落行数：113 / 53
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js

### 主题和外观/布局和间距设计.md

- reference 标题：布局和间距设计
- 生成页：主题系统/themes.md（themes）
- 匹配分数：124
- 页面类型：topic / topic
- 行数：185 / 137
- 段落行数：64 / 51
- Evidence：0 / 5
- Mermaid：3 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js

### 主题和外观/自定义主题开发.md

- reference 标题：自定义主题开发
- 生成页：配置参考/主题和外观/自定义主题开发.md（自定义主题开发）
- 匹配分数：380
- 页面类型：topic / topic
- 行数：372 / 124
- 段落行数：123 / 53
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：package.js
- reference 关键文件未覆盖：constants.ts、index.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、index.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts

### 主题和外观/颜色和字体系统.md

- reference 标题：颜色和字体系统
- 生成页：配置参考/主题和外观/颜色和字体系统.md（颜色和字体系统）
- 匹配分数：338
- 页面类型：topic / topic
- 行数：244 / 124
- 段落行数：90 / 53
- Evidence：13 / 5
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：filemock.js、stylemock.js、build-config.ts、manager.js、preset.js、preview.js、accessibilityrulemaps.ts、a11yrunner.test.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、manager.js、preset.js、preview.js、accessibilityrulemaps.ts、a11yrunner.test.ts

### 多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：API-参考/Angular框架支持.md（Angular框架支持）
- 匹配分数：276
- 页面类型：other / other
- 行数：287 / 540
- 段落行数：119 / 454
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、decorators.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、tsconfig.js、zone.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、decorators.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、tsconfig.js、zone.js

### 多框架支持/HTML框架支持.md

- reference 标题：HTML框架支持
- 生成页：核心模块/code.md（code）
- 匹配分数：86
- 页面类型：other / module
- 行数：215 / 285
- 段落行数：89 / 196
- Evidence：0 / 5
- Mermaid：4 / 2
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：build-config.ts、package.js、sandbox-templates.ts、render.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：build-config.ts、package.js、sandbox-templates.ts、render.ts

### 多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：190
- 页面类型：other / other
- 行数：349 / 178
- 段落行数：225 / 44
- Evidence：16 / 5
- Mermaid：7 / 1
- 文件提及重合：main.ts、index.ts、preset.ts
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、package.js、tsconfig.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js、tsconfig.js

### 多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- 匹配分数：164
- 页面类型：other / other
- 行数：355 / 180
- 段落行数：233 / 94
- Evidence：15 / 5
- Mermaid：8 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts

### 多框架支持/Vue 3框架支持.md

- reference 标题：Vue 3框架支持
- 生成页：构建系统/vue3-vite.md（vue3-vite）
- 匹配分数：160
- 页面类型：other / other
- 行数：279 / 147
- 段落行数：115 / 61
- Evidence：15 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts、preset.ts、vite-plugin.ts
- reference 关键文件未覆盖：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts

### 多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：平台绑定-Web/web-components.md（web-components）
- 匹配分数：194
- 页面类型：module / module
- 行数：300 / 142
- 段落行数：223 / 56
- Evidence：13 / 5
- Mermaid：7 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts

### 多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：568
- 页面类型：other / other
- 行数：259 / 178
- 段落行数：87 / 44
- Evidence：0 / 5
- Mermaid：3 / 1
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：110
- 页面类型：other / other
- 行数：264 / 574
- 段落行数：120 / 448
- Evidence：0 / 12
- Mermaid：6 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js

### 插件系统/Addons概览.md

- reference 标题：Addons概览
- 生成页：插件生态/插件生态.md（插件生态）
- 匹配分数：128
- 页面类型：other / other
- 行数：276 / 170
- 段落行数：97 / 44
- Evidence：16 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、package.js

### 插件系统/插件系统.md

- reference 标题：插件系统
- 生成页：插件生态/插件生态.md（插件生态）
- 匹配分数：172
- 页面类型：other / other
- 行数：356 / 170
- 段落行数：107 / 44
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js

### 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md

- reference 标题：A11y Addon（可访问性测试）
- 生成页：插件生态/a11y.md（a11y）
- 匹配分数：158
- 页面类型：other / other
- 行数：335 / 158
- 段落行数：174 / 72
- Evidence：17 / 5
- Mermaid：7 / 1
- 文件提及重合：a11ypanel.ts、a11ycontext.ts、index.ts、manager.ts
- reference 关键文件未覆盖：package.js、a11yrunner.ts、a11yrunnerutils.ts、visionsimulator.ts、constants.ts、params.ts、preview.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、visionsimulator.ts、constants.ts、params.ts、preview.ts、types.ts

### 插件系统/核心Addons详解/Actions Addon（动作记录）.md

- reference 标题：Actions Addon（动作记录）
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- 匹配分数：82
- 页面类型：other / other
- 行数：354 / 170
- 段落行数：162 / 50
- Evidence：0 / 12
- Mermaid：9 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、manager.ts、actiondisplay.ts、actionoptions.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、manager.ts、actiondisplay.ts、actionoptions.ts

### 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md

- reference 标题：Backgrounds Addon（背景设置）
- 生成页：概念指南/essentials/Backgrounds.md（Backgrounds）
- 匹配分数：86
- 页面类型：other / other
- 行数：355 / 138
- 段落行数：126 / 45
- Evidence：0 / 12
- Mermaid：5 / 1
- 文件提及重合：backgrounds.md
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；图表少于 reference；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts

### 插件系统/核心Addons详解/Controls Addon（参数控制）.md

- reference 标题：Controls Addon（参数控制）
- 生成页：概念指南/README.md（README）
- 匹配分数：100
- 页面类型：other / other
- 行数：285 / 219
- 段落行数：131 / 61
- Evidence：0 / 12
- Mermaid：7 / 1
- 文件提及重合：migration.md、readme.md
- reference 关键文件未覆盖：changelog.md、argcontrol.ts、constants.ts、preview.ts、app.ts、panel.ts、addon-controls.spec.ts、arg-types.md
- 结论：图表少于 reference；缺少关键文件提及：changelog.md、argcontrol.ts、constants.ts、preview.ts、app.ts、panel.ts、addon-controls.spec.ts、arg-types.md

### 插件系统/核心Addons详解/Docs Addon（文档生成）.md

- reference 标题：Docs Addon（文档生成）
- 生成页：插件生态/pseudo-states.md（pseudo-states）
- 匹配分数：80
- 页面类型：other / other
- 行数：239 / 170
- 段落行数：86 / 50
- Evidence：0 / 12
- Mermaid：5 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md

### 插件系统/核心Addons详解/Links Addon（故事导航）.md

- reference 标题：Links Addon（故事导航）
- 生成页：插件生态/links.md（links）
- 匹配分数：156
- 页面类型：other / other
- 行数：362 / 145
- 段落行数：125 / 59
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：index.ts、manager.ts、link.test.ts、link.ts
- reference 关键文件未覆盖：.test.ts、readme.md、package.js、constants.ts、preview.ts、utils.test.ts、utils.ts、decorator.stories.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、preview.ts、utils.test.ts、utils.ts、decorator.stories.ts

### 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md

- reference 标题：Outline/Measure/Toolbar Addons（辅助工具）
- 生成页：概念指南/addons.md（Addons）
- 匹配分数：86
- 页面类型：other / other
- 行数：399 / 212
- 段落行数：153 / 52
- Evidence：0 / 13
- Mermaid：6 / 1
- 文件提及重合：writing-addons.md
- reference 关键文件未覆盖：tabs.hooks.ts、common-manager.ts、common-preset.ts、layout.ts、shortcuts.ts、tool.ts、constants.ts、manager.ts
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：tabs.hooks.ts、common-manager.ts、common-preset.ts、layout.ts、shortcuts.ts、tool.ts、constants.ts、manager.ts

### 插件系统/核心Addons详解/Themes Addon（主题切换）.md

- reference 标题：Themes Addon（主题切换）
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：252
- 页面类型：topic / topic
- 行数：279 / 197
- 段落行数：120 / 49
- Evidence：0 / 15
- Mermaid：8 / 1
- 文件提及重合：build-config.ts、package.js、project.js
- reference 关键文件未覆盖：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md

### 插件系统/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：配置参考/配置参考.md（配置参考）
- 匹配分数：100
- 页面类型：other / other
- 行数：342 / 145
- 段落行数：88 / 38
- Evidence：0 / 5
- Mermaid：3 / 1
- 文件提及重合：main.ts、preview.ts、package.js
- reference 关键文件未覆盖：.yarnrc.yml、manager.ts、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts、addon-toolbars.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.yarnrc.yml、manager.ts、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts、addon-toolbars.spec.ts

### 插件系统/第三方Addons集成.md

- reference 标题：第三方Addons集成
- 生成页：概念指南/addons.md（Addons）
- 匹配分数：180
- 页面类型：other / other
- 行数：246 / 212
- 段落行数：98 / 52
- Evidence：20 / 13
- Mermaid：6 / 1
- 文件提及重合：addon-types.md、index.md、writing-addons.md
- reference 关键文件未覆盖：main.ts、package.js、index.ts
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、package.js、index.ts

### 插件系统/自定义Addons开发.md

- reference 标题：自定义Addons开发
- 生成页：插件生态/addons.md（addons）
- 匹配分数：110
- 页面类型：other / other
- 行数：304 / 140
- 段落行数：108 / 54
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：manager.ts、index.ts
- reference 关键文件未覆盖：main.ts、preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts

### 故障排除/兼容性问题.md

- reference 标题：兼容性问题
- 生成页：概念指南/get-started.md（Get Started）
- 匹配分数：94
- 页面类型：other / other
- 行数：260 / 167
- 段落行数：80 / 54
- Evidence：0 / 9
- Mermaid：6 / 1
- 文件提及重合：angular.md、next.js、node.js
- reference 关键文件未覆盖：common.md、ember.md、frameworks-overview.md、nextjs.md、preact.md、react-vue-angular.md、renderers.md、svelte.md
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：common.md、ember.md、frameworks-overview.md、nextjs.md、preact.md、react-vue-angular.md、renderers.md、svelte.md

### 故障排除/安装问题.md

- reference 标题：安装问题
- 生成页：开发工具/create-storybook.md（create-storybook）
- 匹配分数：126
- 页面类型：other / other
- 行数：350 / 152
- 段落行数：106 / 66
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：finalizationcommand.ts、preflightcheckcommand.ts
- reference 关键文件未覆盖：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：故障排除/故障排除.md（故障排除）
- 匹配分数：494
- 页面类型：other / other
- 行数：348 / 85
- 段落行数：106 / 30
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：main.ts、readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts

### 故障排除/构建和性能问题.md

- reference 标题：构建和性能问题
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：118
- 页面类型：other / other
- 行数：312 / 165
- 段落行数：98 / 42
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、vite-server.ts、base-webpack.config.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、vite-server.ts、base-webpack.config.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts

### 故障排除/调试工具和技巧.md

- reference 标题：调试工具和技巧
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：68
- 页面类型：other / module
- 行数：295 / 176
- 段落行数：121 / 44
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：main.ts、util.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、context-in-play-function.md、context-in-play-function.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、context-in-play-function.md、context-in-play-function.ts

### 故障排除/运行时错误.md

- reference 标题：运行时错误
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：74
- 页面类型：other / module
- 行数：386 / 176
- 段落行数：108 / 44
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：next.js、main.js
- reference 关键文件未覆盖：globalerrormodal.ts、decoratorfunction.ts、errorformatter.ts、instrumenter.ts、types.ts、manager-errors.ts、errors.stories.ts、preview-errors.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：globalerrormodal.ts、decoratorfunction.ts、errorformatter.ts、instrumenter.ts、types.ts、manager-errors.ts、errors.stories.ts、preview-errors.ts

### 故障排除/配置错误.md

- reference 标题：配置错误
- 生成页：配置参考/配置参考.md（配置参考）
- 匹配分数：74
- 页面类型：other / other
- 行数：299 / 145
- 段落行数：92 / 38
- Evidence：0 / 5
- Mermaid：4 / 1
- 文件提及重合：main.ts、preview.ts、package.js
- reference 关键文件未覆盖：manager.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts

### 构建系统/Vite构建器详解.md

- reference 标题：Vite构建器详解
- 生成页：构建系统/builder-vite.md（builder-vite）
- 匹配分数：198
- 页面类型：other / other
- 行数：300 / 185
- 段落行数：120 / 74
- Evidence：0 / 10
- Mermaid：9 / 1
- 文件提及重合：index.ts、code-generator-plugin.ts、csf-plugin.ts
- reference 关键文件未覆盖：package.js、build.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts

### 构建系统/Webpack构建器详解.md

- reference 标题：Webpack构建器详解
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：90
- 页面类型：other / other
- 行数：339 / 165
- 段落行数：138 / 42
- Evidence：17 / 5
- Mermaid：7 / 1
- 文件提及重合：index.ts、next.js
- reference 关键文件未覆盖：package.js、custom-webpack-preset.ts、preview-preset.ts、iframe-webpack.config.ts、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、custom-webpack-preset.ts、preview-preset.ts、iframe-webpack.config.ts、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts

### 构建系统/构建器概览.md

- reference 标题：构建器概览
- 生成页：构建系统/builder-webpack5.md（builder-webpack5）
- 匹配分数：114
- 页面类型：other / other
- 行数：321 / 200
- 段落行数：113 / 114
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf

### 构建系统/构建系统.md

- reference 标题：构建系统
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：614
- 页面类型：other / other
- 行数：291 / 165
- 段落行数：103 / 42
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts、preview-preset.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts、preview-preset.ts

### 构建系统/构建配置优化.md

- reference 标题：构建配置优化
- 生成页：核心模块/code.md（code）
- 匹配分数：128
- 页面类型：other / module
- 行数：264 / 285
- 段落行数：70 / 196
- Evidence：0 / 5
- Mermaid：4 / 2
- 文件提及重合：main.ts、core.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、build.ts、chromatic.config.js、build-config.ts、standalone.ts、tsconfig.js、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、build.ts、chromatic.config.js、build-config.ts、standalone.ts、tsconfig.js、package.js

### 核心概念/Addons系统架构/Addons开发指南.md

- reference 标题：Addons开发指南
- 生成页：插件生态/插件生态.md（插件生态）
- 匹配分数：142
- 页面类型：architecture / other
- 行数：584 / 170
- 段落行数：390 / 44
- Evidence：23 / 5
- Mermaid：13 / 1
- 文件提及重合：manager.ts、preview.ts、index.ts、index.js、manager.js、preview.js
- reference 关键文件未覆盖：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts

### 核心概念/Addons系统架构/Addons架构设计.md

- reference 标题：Addons架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：110
- 页面类型：architecture / architecture
- 行数：321 / 166
- 段落行数：146 / 41
- Evidence：0 / 4
- Mermaid：8 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、channel.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、channel.ts

### 核心概念/Addons系统架构/Addons系统架构.md

- reference 标题：Addons系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：130
- 页面类型：architecture / architecture
- 行数：356 / 166
- 段落行数：156 / 41
- Evidence：22 / 4
- Mermaid：7 / 1
- 文件提及重合：index.md
- reference 关键文件未覆盖：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md

### 核心概念/Addons系统架构/Addons通信机制.md

- reference 标题：Addons通信机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：154
- 页面类型：architecture / architecture
- 行数：290 / 166
- 段落行数：131 / 41
- Evidence：0 / 4
- Mermaid：7 / 1
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：index.test.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：index.test.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts

### 核心概念/Addons系统架构/Addons配置管理.md

- reference 标题：Addons配置管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：114
- 页面类型：architecture / architecture
- 行数：273 / 166
- 段落行数：85 / 41
- Evidence：0 / 4
- Mermaid：4 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：main.js、package.js、preset.ts、presets.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：main.js、package.js、preset.ts、presets.ts

### 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md

- reference 标题：A11y Addon
- 生成页：插件生态/a11y.md（a11y）
- 匹配分数：158
- 页面类型：architecture / other
- 行数：362 / 158
- 段落行数：125 / 72
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：a11ypanel.ts、a11ycontext.ts、index.ts、manager.ts
- reference 关键文件未覆盖：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ycontext.test.ts、details.ts、report.stories.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ycontext.test.ts、details.ts、report.stories.ts

### 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md

- reference 标题：Actions Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：108
- 页面类型：architecture / architecture
- 行数：302 / 166
- 段落行数：103 / 41
- Evidence：0 / 4
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：changelog.md、migration.md、constants.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts、addons.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：changelog.md、migration.md、constants.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts、addons.ts

### 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md

- reference 标题：Backgrounds Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：102
- 页面类型：architecture / architecture
- 行数：374 / 166
- 段落行数：115 / 41
- Evidence：0 / 4
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts

### 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md

- reference 标题：Controls Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：282 / 166
- 段落行数：95 / 41
- Evidence：0 / 4
- Mermaid：4 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts

### 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md

- reference 标题：Docs Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：110
- 页面类型：architecture / architecture
- 行数：348 / 166
- 段落行数：127 / 41
- Evidence：0 / 4
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、manager.ts、manifest.ts、mdx-react-shim.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、manager.ts、manifest.ts、mdx-react-shim.ts、types.ts

### 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md

- reference 标题：Themes Addon
- 生成页：主题系统/themes.md（themes）
- 匹配分数：202
- 页面类型：architecture / topic
- 行数：293 / 137
- 段落行数：127 / 51
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts、manager.ts、theme-switcher.ts
- reference 关键文件未覆盖：constants.ts、helpers.ts、provider.decorator.ts、preview.ts、types.ts、themes.md
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、helpers.ts、provider.decorator.ts、preview.ts、types.ts、themes.md

### 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md

- reference 标题：工具栏插件（Toolbars Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：110
- 页面类型：architecture / architecture
- 行数：290 / 166
- 段落行数：128 / 41
- Evidence：0 / 4
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、manager.ts、preview.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、manager.ts、preview.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts

### 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md

- reference 标题：视口插件（Viewport Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：118
- 页面类型：architecture / architecture
- 行数：257 / 166
- 段落行数：98 / 41
- Evidence：0 / 4
- Mermaid：6 / 1
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、addon-viewport.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、addon-viewport.spec.ts

### 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：系统架构.md（系统架构）
- 匹配分数：106
- 页面类型：architecture / architecture
- 行数：309 / 166
- 段落行数：113 / 41
- Evidence：7 / 4
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts

### 核心概念/CSF格式规范.md

- reference 标题：CSF格式规范
- 生成页：概念指南/writing-docs.md（Writing Docs）
- 匹配分数：74
- 页面类型：other / other
- 行数：302 / 201
- 段落行数：115 / 50
- Evidence：0 / 13
- Mermaid：5 / 1
- 文件提及重合：index.md、mdx.md
- reference 关键文件未覆盖：inject-decorator.ts、csffile.test.ts、csffile.ts、readme.md、build-config.ts、csf-2-to-3.ts、project.js、csf-component.md
- 结论：图表少于 reference；缺少关键文件提及：inject-decorator.ts、csffile.test.ts、csffile.ts、readme.md、build-config.ts、csf-2-to-3.ts、project.js、csf-component.md

### 核心概念/主题和参数系统.md

- reference 标题：主题和参数系统
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：112
- 页面类型：topic / topic
- 行数：356 / 197
- 段落行数：126 / 49
- Evidence：0 / 15
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：106
- 页面类型：other / other
- 行数：388 / 574
- 段落行数：139 / 448
- Evidence：16 / 12
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、index.ts、index.js
- reference 关键文件未覆盖：manager.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md

### 核心概念/组件故事（Stories）.md

- reference 标题：组件故事（Stories）
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：260
- 页面类型：other / other
- 行数：244 / 574
- 段落行数：85 / 448
- Evidence：0 / 12
- Mermaid：4 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：.stories.ts、canvas.stories.ts、story.stories.ts、csffile.ts、getstorysortparameter.ts、stories.ts、readme-store.md、package.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.stories.ts、canvas.stories.ts、story.stories.ts、csffile.ts、getstorysortparameter.ts、stories.ts、readme-store.md、package.js

### 核心概念/装饰器和全局状态.md

- reference 标题：装饰器和全局状态
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：102
- 页面类型：other / other
- 行数：307 / 574
- 段落行数：119 / 448
- Evidence：0 / 12
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js

### 核心概念/预览和管理界面.md

- reference 标题：预览和管理界面
- 生成页：核心模块/code.md（code）
- 匹配分数：80
- 页面类型：other / module
- 行数：417 / 285
- 段落行数：154 / 196
- Evidence：0 / 5
- Mermaid：7 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts

### 测试框架/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：92
- 页面类型：other / topic
- 行数：382 / 197
- 段落行数：118 / 49
- Evidence：0 / 15
- Mermaid：5 / 1
- 文件提及重合：package.js、vitest.config.ts
- reference 关键文件未覆盖：main.ts、preview.ts、chromatic.config.js、manager.spec.ts、navigation.spec.ts、preview-api.spec.ts、util.ts、playwright.config.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、chromatic.config.js、manager.spec.ts、navigation.spec.ts、preview-api.spec.ts、util.ts、playwright.config.ts

### 测试框架/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：82
- 页面类型：other / other
- 行数：250 / 574
- 段落行数：64 / 448
- Evidence：0 / 12
- Mermaid：3 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：storybook.setup.ts、component-tests.spec.ts、package.js、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts、playwright.config.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storybook.setup.ts、component-tests.spec.ts、package.js、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts、playwright.config.ts

### 测试框架/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：插件生态/a11y.md（a11y）
- 匹配分数：100
- 页面类型：other / other
- 行数：300 / 158
- 段落行数：109 / 72
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts、manager.ts
- reference 关键文件未覆盖：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、preview.ts、types.ts、utils.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、preview.ts、types.ts、utils.ts

### 测试框架/测试框架.md

- reference 标题：测试框架
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：88
- 页面类型：other / other
- 行数：361 / 574
- 段落行数：136 / 448
- Evidence：0 / 12
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：chromatic.config.js、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：chromatic.config.js、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.config.ts、vitest.helpers.ts

### 测试框架/组件测试.md

- reference 标题：组件测试
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：78
- 页面类型：other / other
- 行数：317 / 574
- 段落行数：121 / 448
- Evidence：20 / 12
- Mermaid：8 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：storybook.setup.ts、readme.md、extract.ts、test-fn.stories.ts、mocker-runtime.js、sb-module-mocking.spec.ts、vitest-setup.ts、vitest.config.storybook.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：storybook.setup.ts、readme.md、extract.ts、test-fn.stories.ts、mocker-runtime.js、sb-module-mocking.spec.ts、vitest-setup.ts、vitest.config.storybook.ts

### 测试框架/视觉回归测试.md

- reference 标题：视觉回归测试
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：88
- 页面类型：other / topic
- 行数：328 / 197
- 段落行数：135 / 49
- Evidence：0 / 15
- Mermaid：8 / 1
- 文件提及重合：package.js、vitest.config.ts
- reference 关键文件未覆盖：main.ts、preview.ts、chromatic.config.js、component-tests.spec.ts、playwright.config.ts、vitest.config.storybook.ts、node.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、chromatic.config.js、component-tests.spec.ts、playwright.config.ts、vitest.config.storybook.ts、node.js

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：概念指南/README.md（README）
- 匹配分数：68
- 页面类型：other / other
- 行数：228 / 219
- 段落行数：77 / 61
- Evidence：0 / 12
- Mermaid：3 / 1
- 文件提及重合：contributing.md
- reference 关键文件未覆盖：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、security.md、package.js、playwright.config.ts、vitest.config.ts
- 结论：图表少于 reference；缺少关键文件提及：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、security.md、package.js、playwright.config.ts、vitest.config.ts

### 部署和CI_CD/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：开发工具/scripts.md（scripts）
- 匹配分数：66
- 页面类型：workflow / other
- 行数：317 / 324
- 段落行数：96 / 201
- Evidence：0 / 12
- Mermaid：5 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：config.generated.yml、config.yml、common-jobs.ts、executors.ts、helpers.ts、parameters.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、executors.ts、helpers.ts、parameters.ts、types.ts

### 部署和CI_CD/Chromatic集成.md

- reference 标题：Chromatic集成
- 生成页：配置参考/Main.md（Main）
- 匹配分数：94
- 页面类型：workflow / other
- 行数：242 / 106
- 段落行数：123 / 46
- Evidence：18 / 12
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：ischromatic.ts、chromatic.config.js、get-chromatic-version.ts、package.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：ischromatic.ts、chromatic.config.js、get-chromatic-version.ts、package.js

### 部署和CI_CD/环境配置.md

- reference 标题：环境配置
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：66
- 页面类型：workflow / other
- 行数：299 / 574
- 段落行数：97 / 448
- Evidence：17 / 12
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：config.yml、.yarnrc.yml、vitest.config.ts、codecov.yml、dependabot.yml、nx.js、package.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、codecov.yml、dependabot.yml、nx.js、package.js

### 部署和CI_CD/部署和CI_CD.md

- reference 标题：部署和CI/CD
- 生成页：核心模块/storybook.md（storybook）
- 匹配分数：112
- 页面类型：workflow / module
- 行数：264 / 141
- 段落行数：105 / 48
- Evidence：0 / 7
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：config.generated.yml、config.yml、distribution-config.yaml、readme.md、chromatic.config.js、package.js、project.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、readme.md、chromatic.config.js、package.js、project.js

### 部署和CI_CD/静态部署.md

- reference 标题：静态部署
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：80
- 页面类型：workflow / other
- 行数：201 / 574
- 段落行数：63 / 448
- Evidence：0 / 12
- Mermaid：4 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：config.yml、manager.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、manager.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts

### 项目概述/使用场景和案例.md

- reference 标题：使用场景和案例
- 生成页：概念指南/Docs.md（Docs）
- 匹配分数：228
- 页面类型：overview / other
- 行数：271 / 213
- 段落行数：100 / 52
- Evidence：0 / 13
- Mermaid：6 / 1
- 文件提及重合：index.md
- reference 关键文件未覆盖：main.ts、preview.ts、readme.md、package.js
- 结论：图表少于 reference；缺少关键文件提及：main.ts、preview.ts、readme.md、package.js

### 项目概述/基本概念.md

- reference 标题：基本概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：182
- 页面类型：overview / overview
- 行数：280 / 224
- 段落行数：80 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：main.ts、preview.ts、readme.md、index.md
- reference 关键文件未覆盖：button.stories.ts、storybook-addons.md
- 结论：图表少于 reference；缺少关键文件提及：button.stories.ts、storybook-addons.md

### 项目概述/技术架构/Monorepo设计.md

- reference 标题：Monorepo设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 页面类型：overview / overview
- 行数：249 / 224
- 段落行数：78 / 51
- Evidence：0 / 6
- Mermaid：3 / 1
- 文件提及重合：build-package.ts
- reference 关键文件未覆盖：package.js、project.js、nx.js、check-package.ts、prepare-sandbox.ts
- 结论：图表少于 reference；缺少关键文件提及：package.js、project.js、nx.js、check-package.ts、prepare-sandbox.ts

### 项目概述/技术架构/技术架构.md

- reference 标题：技术架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：136
- 页面类型：overview / overview
- 行数：289 / 224
- 段落行数：141 / 51
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：index.ts、preset.ts、preview.ts
- reference 关键文件未覆盖：dispatcher.js、package.js、dispatcher.ts、manager-stores.ts、next.js、index.js、core.js
- 结论：解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、manager-stores.ts、next.js、index.js、core.js

### 项目概述/技术架构/插件系统架构/插件架构设计.md

- reference 标题：插件架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：346 / 224
- 段落行数：158 / 51
- Evidence：17 / 6
- Mermaid：8 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md

### 项目概述/技术架构/插件系统架构/插件注册机制.md

- reference 标题：插件注册机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 页面类型：overview / overview
- 行数：263 / 224
- 段落行数：100 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：preview.js、preset.js
- reference 关键文件未覆盖：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md
- 结论：图表少于 reference；缺少关键文件提及：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md

### 项目概述/技术架构/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：130
- 页面类型：overview / overview
- 行数：266 / 224
- 段落行数：94 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts、postinstalladdon.ts
- 结论：图表少于 reference；缺少关键文件提及：manager.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts、postinstalladdon.ts

### 项目概述/技术架构/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：插件生态/插件生态.md（插件生态）
- 匹配分数：146
- 页面类型：overview / other
- 行数：315 / 170
- 段落行数：127 / 44
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：index.ts、preset.js
- reference 关键文件未覆盖：package.js、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md、storybook-addons-root-preset.md
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md、storybook-addons-root-preset.md

### 项目概述/技术架构/插件系统架构/插件通信协议.md

- reference 标题：插件通信协议
- 生成页：项目概述.md（项目概述）
- 匹配分数：154
- 页面类型：overview / overview
- 行数：312 / 224
- 段落行数：136 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：index.test.ts、types.ts
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：index.test.ts、types.ts

### 项目概述/技术架构/构建系统架构/Vite构建器.md

- reference 标题：Vite构建器
- 生成页：构建系统/builder-vite.md（builder-vite）
- 匹配分数：208
- 页面类型：overview / other
- 行数：283 / 185
- 段落行数：127 / 74
- Evidence：0 / 10
- Mermaid：7 / 1
- 文件提及重合：index.ts、csf-plugin.ts、preset.ts
- reference 关键文件未覆盖：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、vite-config.ts、vite-server.ts、vite.conf
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、vite-config.ts、vite-server.ts、vite.conf

### 项目概述/技术架构/构建系统架构/Webpack构建器.md

- reference 标题：Webpack构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：242 / 224
- 段落行数：84 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js
- 结论：图表少于 reference；缺少关键文件提及：package.js、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js

### 项目概述/技术架构/构建系统架构/构建器架构设计.md

- reference 标题：构建器架构设计
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：146
- 页面类型：overview / other
- 行数：363 / 165
- 段落行数：146 / 42
- Evidence：0 / 5
- Mermaid：7 / 1
- 文件提及重合：index.ts、preset.js
- reference 关键文件未覆盖：package.js、build.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts、frameworkdetectionservice.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts、frameworkdetectionservice.ts

### 项目概述/技术架构/构建系统架构/构建系统架构.md

- reference 标题：构建系统架构
- 生成页：构建系统/builders.md（builders）
- 匹配分数：146
- 页面类型：overview / other
- 行数：244 / 130
- 段落行数：105 / 44
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、build.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、build.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts

### 项目概述/技术架构/核心引擎架构/CLI分发器.md

- reference 标题：CLI分发器
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：overview / overview
- 行数：308 / 224
- 段落行数：152 / 51
- Evidence：0 / 6
- Mermaid：9 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、upgrade.ts、node.js
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、upgrade.ts、node.js

### 项目概述/技术架构/核心引擎架构/全局设置.md

- reference 标题：全局设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 页面类型：overview / overview
- 行数：360 / 224
- 段落行数：143 / 51
- Evidence：18 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts、util.ts
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts、util.ts

### 项目概述/技术架构/核心引擎架构/核心引擎架构.md

- reference 标题：核心引擎架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：overview / overview
- 行数：272 / 224
- 段落行数：122 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：readme.md
- reference 关键文件未覆盖：dispatcher.js、build-config.ts、package.js、tsconfig.js
- 结论：图表少于 reference；缺少关键文件提及：dispatcher.js、build-config.ts、package.js、tsconfig.js

### 项目概述/技术架构/核心引擎架构/核心服务器.md

- reference 标题：核心服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 页面类型：overview / overview
- 行数：334 / 224
- 段落行数：131 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：preview.ts、index.ts
- reference 关键文件未覆盖：build-config.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts、storybook-builder-api-dev-server.md、storybook-builder-api-interface.md
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：build-config.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts、storybook-builder-api-dev-server.md、storybook-builder-api-interface.md

### 项目概述/技术架构/核心引擎架构/版本管理.md

- reference 标题：版本管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：359 / 224
- 段落行数：137 / 51
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts

### 项目概述/技术架构/框架适配器架构/Angular框架适配器.md

- reference 标题：Angular框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：164
- 页面类型：overview / overview
- 行数：355 / 224
- 段落行数：245 / 51
- Evidence：17 / 6
- Mermaid：8 / 1
- 文件提及重合：readme.md、index.ts、preset.ts、main.ts、preview.ts
- reference 关键文件未覆盖：build-config.ts、package.js、project.js、error-handler.ts、run-compodoc.ts、standalone-options.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：build-config.ts、package.js、project.js、error-handler.ts、run-compodoc.ts、standalone-options.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts

### 项目概述/技术架构/框架适配器架构/HTML框架适配器.md

- reference 标题：HTML框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：114
- 页面类型：overview / overview
- 行数：245 / 224
- 段落行数：99 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：.stories.ts、project.js、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js、types.ts
- 结论：图表少于 reference；缺少关键文件提及：.stories.ts、project.js、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js、types.ts

### 项目概述/技术架构/框架适配器架构/React框架适配器.md

- reference 标题：React框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 页面类型：overview / overview
- 行数：326 / 224
- 段落行数：239 / 51
- Evidence：17 / 6
- Mermaid：9 / 1
- 文件提及重合：main.ts、index.ts、preset.ts、preview.ts
- reference 关键文件未覆盖：package.js、types.ts、public-types.ts、tsconfig.js
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js、types.ts、public-types.ts、tsconfig.js

### 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md

- reference 标题：Svelte框架适配器
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- 匹配分数：166
- 页面类型：overview / other
- 行数：287 / 180
- 段落行数：131 / 94
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、utils.ts、portable-stories.ts、public-types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、utils.ts、portable-stories.ts、public-types.ts

### 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md

- reference 标题：Vue 3框架适配器
- 生成页：构建系统/vue3-vite.md（vue3-vite）
- 匹配分数：158
- 页面类型：overview / other
- 行数：305 / 147
- 段落行数：212 / 61
- Evidence：15 / 5
- Mermaid：7 / 1
- 文件提及重合：index.ts、vue-component-meta.ts、vue-template.ts、preset.ts、vite-plugin.ts
- reference 关键文件未覆盖：package.js、vue-docgen.ts、types.ts、preview.ts、public-types.ts、render.ts、tsconfig.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、vue-docgen.ts、types.ts、preview.ts、public-types.ts、render.ts、tsconfig.js

### 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md

- reference 标题：Web Components框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：164
- 页面类型：overview / overview
- 行数：478 / 224
- 段落行数：315 / 51
- Evidence：17 / 6
- Mermaid：9 / 1
- 文件提及重合：main.ts、index.ts、preview.ts
- reference 关键文件未覆盖：...conf、package.js、types.ts、framework-api.ts、globals.ts、public-types.ts
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：...conf、package.js、types.ts、framework-api.ts、globals.ts、public-types.ts

### 项目概述/技术架构/框架适配器架构/框架适配器架构.md

- reference 标题：框架适配器架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：152
- 页面类型：overview / overview
- 行数：297 / 224
- 段落行数：136 / 51
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：index.ts、preset.ts、main.ts
- reference 关键文件未覆盖：package.js、react-docgen.ts、types.ts、next.js
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：package.js、react-docgen.ts、types.ts、next.js

### 项目概述/技术架构/渲染系统架构/HTML渲染器.md

- reference 标题：HTML渲染器
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：168
- 页面类型：overview / module
- 行数：250 / 176
- 段落行数：108 / 44
- Evidence：14 / 5
- Mermaid：6 / 1
- 文件提及重合：entry-preview.ts、index.ts、render.ts
- reference 关键文件未覆盖：package.js、globals.ts、portable-stories.ts、public-types.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、globals.ts、portable-stories.ts、public-types.ts、types.ts

### 项目概述/技术架构/渲染系统架构/React渲染器.md

- reference 标题：React渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：238 / 224
- 段落行数：91 / 51
- Evidence：12 / 6
- Mermaid：4 / 1
- 文件提及重合：preset.ts
- reference 关键文件未覆盖：docsrenderer.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：docsrenderer.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts

### 项目概述/技术架构/渲染系统架构/Svelte渲染器.md

- reference 标题：Svelte渲染器
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：126
- 页面类型：overview / module
- 行数：319 / 176
- 段落行数：149 / 44
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：index.ts、preset.ts、render.ts、preset.js
- reference 关键文件未覆盖：.ts、framework-svelte.spec.ts、package.js、globals.ts、mount.ts、portable-stories.ts、public-types.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、mount.ts、portable-stories.ts、public-types.ts、types.ts

### 项目概述/技术架构/渲染系统架构/Vue3渲染器.md

- reference 标题：Vue3渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：217 / 224
- 段落行数：83 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：entry-preview.ts、index.ts
- reference 关键文件未覆盖：package.js、render.ts
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、render.ts

### 项目概述/技术架构/渲染系统架构/渲染系统架构.md

- reference 标题：渲染系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：130
- 页面类型：overview / overview
- 行数：267 / 224
- 段落行数：130 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、package.js、manager-stores.ts、globals.ts
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：manager.ts、package.js、manager-stores.ts、globals.ts

### 项目概述/核心特性/主题定制系统/主题切换机制.md

- reference 标题：主题切换机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 页面类型：overview / overview
- 行数：303 / 224
- 段落行数：101 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、create.ts、utils.ts
- 结论：图表少于 reference；缺少关键文件提及：manager.ts、create.ts、utils.ts

### 项目概述/核心特性/主题定制系统/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：overview / overview
- 行数：275 / 224
- 段落行数：143 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、storybook.setup.ts、create.ts、types.ts
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、create.ts、types.ts

### 项目概述/核心特性/主题定制系统/主题扩展开发.md

- reference 标题：主题扩展开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：136
- 页面类型：overview / overview
- 行数：343 / 224
- 段落行数：119 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts
- 结论：图表少于 reference；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts

### 项目概述/核心特性/主题定制系统/字体系统.md

- reference 标题：字体系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：221 / 224
- 段落行数：69 / 51
- Evidence：0 / 6
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：constants.ts、base.ts、global.ts、dark.ts、light.ts
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts

### 项目概述/核心特性/主题定制系统/布局系统.md

- reference 标题：布局系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：314 / 224
- 段落行数：121 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：preview.ts
- reference 关键文件未覆盖：theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts、layout.stories.ts
- 结论：图表少于 reference；缺少关键文件提及：theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts、layout.stories.ts

### 项目概述/核心特性/主题定制系统/颜色系统.md

- reference 标题：颜色系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：192 / 224
- 段落行数：52 / 51
- Evidence：0 / 6
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、colors.ts、dark.ts、light.ts
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts

### 项目概述/核心特性/多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：API-参考/Angular框架支持.md（Angular框架支持）
- 匹配分数：278
- 页面类型：overview / other
- 行数：302 / 540
- 段落行数：137 / 454
- Evidence：22 / 5
- Mermaid：8 / 1
- 文件提及重合：preview.ts、index.ts
- reference 关键文件未覆盖：package.js、config.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、preset-options.ts、tsconfig.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、config.ts、public-types.ts、types.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts、preset-options.ts、tsconfig.js

### 项目概述/核心特性/多框架支持/Next.js框架支持.md

- reference 标题：Next.js框架支持
- 生成页：核心模块/frameworks.md（frameworks）
- 匹配分数：174
- 页面类型：overview / module
- 行数：237 / 154
- 段落行数：109 / 68
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts、preview.ts、next.js
- reference 关键文件未覆盖：framework-nextjs.spec.ts、types.ts、package.js、decorator.ts、next.config.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-nextjs.spec.ts、types.ts、package.js、decorator.ts、next.config.js

### 项目概述/核心特性/多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：190
- 页面类型：overview / other
- 行数：296 / 178
- 段落行数：107 / 44
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：main.ts、index.ts、preset.ts
- reference 关键文件未覆盖：package.js、react-docgen.ts、preview.ts、public-types.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少关键文件提及：package.js、react-docgen.ts、preview.ts、public-types.ts、types.ts

### 项目概述/核心特性/多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- 匹配分数：160
- 页面类型：overview / other
- 行数：299 / 180
- 段落行数：106 / 94
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts

### 项目概述/核心特性/多框架支持/Vue框架支持.md

- reference 标题：Vue框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：142
- 页面类型：overview / overview
- 行数：234 / 224
- 段落行数：129 / 51
- Evidence：12 / 6
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：package.js

### 项目概述/核心特性/多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 页面类型：overview / overview
- 行数：268 / 224
- 段落行数：110 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、index.ts、preset.js、preset.ts
- reference 关键文件未覆盖：button.js、button.stories.js、build-config.ts、package.js
- 结论：图表少于 reference；缺少关键文件提及：button.js、button.stories.js、build-config.ts、package.js

### 项目概述/核心特性/多框架支持/其他框架支持.md

- reference 标题：其他框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：142
- 页面类型：overview / overview
- 行数：309 / 224
- 段落行数：119 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、frameworks.js
- 结论：图表少于 reference；缺少关键文件提及：package.js、frameworks.js

### 项目概述/核心特性/多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：516
- 页面类型：overview / other
- 行数：494 / 178
- 段落行数：123 / 44
- Evidence：0 / 5
- Mermaid：4 / 1
- 文件提及重合：index.ts、preset.ts、main.ts
- reference 关键文件未覆盖：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、preview-prod.ts、preview.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、preview-prod.ts、preview.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts

### 项目概述/核心特性/插件系统架构/Addon API设计.md

- reference 标题：Addon API设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：140
- 页面类型：overview / overview
- 行数：262 / 224
- 段落行数：98 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：index.ts、preset.ts、manager.js、preview.js、preview.ts
- reference 关键文件未覆盖：params.ts、types.ts、docsrenderer.ts、manager.ts
- 结论：图表少于 reference；缺少关键文件提及：params.ts、types.ts、docsrenderer.ts、manager.ts

### 项目概述/核心特性/插件系统架构/Manager API.md

- reference 标题：Manager API
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：335 / 224
- 段落行数：124 / 51
- Evidence：17 / 6
- Mermaid：7 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：main.js、manager.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.js、manager.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md

### 项目概述/核心特性/插件系统架构/Preview API.md

- reference 标题：Preview API
- 生成页：API-参考/开发API参考/Preview-API.md（Preview API）
- 匹配分数：270
- 页面类型：overview / other
- 行数：267 / 540
- 段落行数：101 / 454
- Evidence：15 / 5
- Mermaid：5 / 1
- 文件提及重合：preview.ts
- reference 关键文件未覆盖：main.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js

### 项目概述/核心特性/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：322 / 224
- 段落行数：119 / 51
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：preset.js
- reference 关键文件未覆盖：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md

### 项目概述/核心特性/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：162
- 页面类型：overview / overview
- 行数：417 / 224
- 段落行数：167 / 51
- Evidence：0 / 6
- Mermaid：10 / 1
- 文件提及重合：preview.ts、main.ts、index.ts
- reference 关键文件未覆盖：package.js、manager.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts、hooks.ts
- 结论：解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、manager.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts、hooks.ts

### 项目概述/核心特性/插件系统架构/通信机制.md

- reference 标题：通信机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：158
- 页面类型：overview / overview
- 行数：281 / 224
- 段落行数：128 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：geteventsourceurl.ts、types.ts、addons.ts、app.ts
- 结论：解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：geteventsourceurl.ts、types.ts、addons.ts、app.ts

### 项目概述/核心特性/文档生成功能/Doc Blocks API.md

- reference 标题：Doc Blocks API
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 页面类型：overview / overview
- 行数：389 / 224
- 段落行数：121 / 51
- Evidence：0 / 6
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts
- 结论：图表少于 reference；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts

### 项目概述/核心特性/文档生成功能/MDX文档编写.md

- reference 标题：MDX文档编写
- 生成页：概念指南/writing-docs.md（Writing Docs）
- 匹配分数：216
- 页面类型：overview / other
- 行数：269 / 201
- 段落行数：121 / 50
- Evidence：19 / 13
- Mermaid：8 / 1
- 文件提及重合：build-documentation.md、doc-blocks.md、index.md、mdx.md
- reference 关键文件未覆盖：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md

### 项目概述/核心特性/文档生成功能/文档生成功能.md

- reference 标题：文档生成功能
- 生成页：概念指南/writing-docs.md（Writing Docs）
- 匹配分数：172
- 页面类型：overview / other
- 行数：341 / 201
- 段落行数：129 / 50
- Evidence：17 / 13
- Mermaid：7 / 1
- 文件提及重合：autodocs.md、index.md、mdx.md
- reference 关键文件未覆盖：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts

### 项目概述/核心特性/文档生成功能/自动文档系统.md

- reference 标题：自动文档系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 页面类型：overview / overview
- 行数：267 / 224
- 段落行数：118 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、readme.md、index.ts
- reference 关键文件未覆盖：docspage.md、mdx.md、csf-plugin.ts、package.js、shared.ts
- 结论：图表少于 reference；缺少关键文件提及：docspage.md、mdx.md、csf-plugin.ts、package.js、shared.ts

### 项目概述/核心特性/文档生成功能/自定义文档页面.md

- reference 标题：自定义文档页面
- 生成页：项目概述.md（项目概述）
- 匹配分数：140
- 页面类型：overview / overview
- 行数：260 / 224
- 段落行数：110 / 51
- Evidence：18 / 6
- Mermaid：6 / 1
- 文件提及重合：main.ts、preview.ts、index.ts、index.md
- reference 关键文件未覆盖：manager.ts、metaof.md、title.md、create.ts
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.ts、metaof.md、title.md、create.ts

### 项目概述/核心特性/核心特性.md

- reference 标题：核心特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：184
- 页面类型：overview / overview
- 行数：349 / 224
- 段落行数：138 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、preset.ts、faq.md
- reference 关键文件未覆盖：manager.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts、render-components-manifest.ts、menu.ts
- 结论：解释性段落明显不足；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：manager.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts、render-components-manifest.ts、menu.ts

### 项目概述/核心特性/测试支持功能/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：373 / 224
- 段落行数：157 / 51
- Evidence：0 / 6
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js

### 项目概述/核心特性/测试支持功能/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：overview / overview
- 行数：291 / 224
- 段落行数：114 / 51
- Evidence：23 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：.spec.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts、vitest.shared.ts
- 结论：图表少于 reference；evidence block 少于 reference；缺少关键文件提及：.spec.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts、vitest.shared.ts

### 项目概述/核心特性/测试支持功能/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 页面类型：overview / overview
- 行数：311 / 224
- 段落行数：136 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、constants.ts、manager.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf
- 结论：解释性段落明显不足；图表少于 reference；缺少关键文件提及：package.js、constants.ts、manager.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf

### 项目概述/核心特性/测试支持功能/测试支持功能.md

- reference 标题：测试支持功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 页面类型：overview / overview
- 行数：338 / 224
- 段落行数：133 / 51
- Evidence：21 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：.spec.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：.spec.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts

### 项目概述/核心特性/测试支持功能/组件测试.md

- reference 标题：组件测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：overview / overview
- 行数：266 / 224
- 段落行数：106 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts、jest.config.js
- 结论：图表少于 reference；缺少关键文件提及：storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts、jest.config.js

### 项目概述/核心特性/组件开发环境/交互式调试.md

- reference 标题：交互式调试
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：overview / overview
- 行数：264 / 224
- 段落行数：110 / 51
- Evidence：0 / 6
- Mermaid：7 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：.manager.ts、manager.ts、constants.ts、panel.ts
- 结论：图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.manager.ts、manager.ts、constants.ts、panel.ts

### 项目概述/核心特性/组件开发环境/实时预览.md

- reference 标题：实时预览
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：overview / overview
- 行数：311 / 224
- 段落行数：108 / 51
- Evidence：0 / 6
- Mermaid：5 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、story.ts、geteventsourceurl.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts、storystore.ts、render.ts
- 结论：图表少于 reference；缺少关键文件提及：manager.ts、story.ts、geteventsourceurl.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts、storystore.ts、render.ts

### 项目概述/核心特性/组件开发环境/开发服务器.md

- reference 标题：开发服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：114
- 页面类型：overview / overview
- 行数：303 / 224
- 段落行数：124 / 51
- Evidence：0 / 6
- Mermaid：6 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts、middleware.ts
- 结论：图表少于 reference；缺少关键文件提及：logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts、middleware.ts

### 项目概述/核心特性/组件开发环境/组件开发环境.md

- reference 标题：组件开发环境
- 生成页：项目概述.md（项目概述）
- 匹配分数：136
- 页面类型：overview / overview
- 行数：312 / 224
- 段落行数：139 / 51
- Evidence：21 / 6
- Mermaid：8 / 1
- 文件提及重合：main.ts、preview.ts、index.ts
- reference 关键文件未覆盖：manager.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、types.ts、package.js
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：manager.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、types.ts、package.js

### 项目概述/核心特性/组件开发环境/组件生命周期.md

- reference 标题：组件生命周期
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 页面类型：overview / overview
- 行数：319 / 224
- 段落行数：122 / 51
- Evidence：16 / 6
- Mermaid：6 / 1
- 文件提及重合：readme.md、index.ts、main.ts
- reference 关键文件未覆盖：store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts
- 结论：图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：416
- 页面类型：overview / overview
- 行数：312 / 224
- 段落行数：131 / 51
- Evidence：21 / 6
- Mermaid：8 / 1
- 文件提及重合：readme.md、main.ts、index.md
- reference 关键文件未覆盖：contributing.md、package.js、tsconfig.js
- 结论：解释性段落明显不足；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：contributing.md、package.js、tsconfig.js

### 高级功能/工具集成/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：系统架构.md（系统架构）
- 匹配分数：72
- 页面类型：other / architecture
- 行数：299 / 166
- 段落行数：127 / 41
- Evidence：20 / 4
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：config.generated.yml、config.yml、chromatic.config.js、package.js、ghp-github-action.md、test-runner-local-build-workflow.md、in-ci.md、common-jobs.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少关键文件提及：config.generated.yml、config.yml、chromatic.config.js、package.js、ghp-github-action.md、test-runner-local-build-workflow.md、in-ci.md、common-jobs.ts

### 高级功能/工具集成/Codemod工具.md

- reference 标题：Codemod工具
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：126
- 页面类型：other / module
- 行数：286 / 176
- 段落行数：114 / 44
- Evidence：18 / 5
- Mermaid：5 / 1
- 文件提及重合：index.test.ts、index.ts
- reference 关键文件未覆盖：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts

### 高级功能/工具集成/ESLint集成.md

- reference 标题：ESLint集成
- 生成页：插件生态/eslint-plugin.md（eslint-plugin）
- 匹配分数：206
- 页面类型：other / other
- 行数：269 / 173
- 段落行数：69 / 59
- Evidence：0 / 12
- Mermaid：4 / 1
- 文件提及重合：csf.ts、recommended.ts、index.ts、csf-component.ts
- reference 关键文件未覆盖：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts

### 高级功能/工具集成/IDE配置优化.md

- reference 标题：IDE配置优化
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：88
- 页面类型：other / topic
- 行数：339 / 197
- 段落行数：105 / 49
- Evidence：15 / 15
- Mermaid：4 / 1
- 文件提及重合：tsconfig.js、vitest.config.ts、package.js
- reference 关键文件未覆盖：.eslintrc.js、main.ts、manager.ts、preview.ts、storybook.setup.ts、vitest.config.storybook.ts、prettier.conf、prettier.config.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、main.ts、manager.ts、preview.ts、storybook.setup.ts、vitest.config.storybook.ts、prettier.conf、prettier.config.js

### 高级功能/工具集成/工具集成.md

- reference 标题：工具集成
- 生成页：API-参考/组件故事-Stories.md（组件故事（Stories））
- 匹配分数：108
- 页面类型：other / other
- 行数：313 / 574
- 段落行数：105 / 448
- Evidence：0 / 12
- Mermaid：6 / 1
- 文件提及重合：main.ts、preview.ts
- reference 关键文件未覆盖：config.yml、.eslintrc.js、manager.ts、package.js、prettier.conf、tsconfig.js、prettier.config.js、codemod.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、package.js、prettier.conf、tsconfig.js、prettier.config.js、codemod.ts

### 高级功能/工具集成/构建工具优化.md

- reference 标题：构建工具优化
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：116
- 页面类型：other / other
- 行数：287 / 165
- 段落行数：93 / 42
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：main.ts、package.js、vite-config.ts、vite-server.ts、virtual-module-mapping.ts、environments-support.ts、generate-bundle.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、virtual-module-mapping.ts、environments-support.ts、generate-bundle.ts

### 高级功能/性能监控.md

- reference 标题：性能监控
- 生成页：核心模块/code.md（code）
- 匹配分数：76
- 页面类型：other / module
- 行数：340 / 285
- 段落行数：165 / 196
- Evidence：0 / 5
- Mermaid：8 / 2
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、types.ts、utils.ts、bench.ts、upload-bench.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、types.ts、utils.ts、bench.ts、upload-bench.ts

### 高级功能/扩展开发/Addon开发.md

- reference 标题：Addon开发
- 生成页：插件生态/addons.md（addons）
- 匹配分数：120
- 页面类型：other / other
- 行数：327 / 140
- 段落行数：120 / 54
- Evidence：0 / 5
- Mermaid：8 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、preview.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preview.ts

### 高级功能/扩展开发/主题开发.md

- reference 标题：主题开发
- 生成页：主题系统/themes.md（themes）
- 匹配分数：206
- 页面类型：topic / topic
- 行数：320 / 137
- 段落行数：149 / 51
- Evidence：20 / 5
- Mermaid：9 / 1
- 文件提及重合：manager.ts、class-name.decorator.ts、index.ts
- reference 关键文件未覆盖：main.ts、preview.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、preview.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js

### 高级功能/扩展开发/扩展开发.md

- reference 标题：扩展开发
- 生成页：配置参考/主题和外观.md（主题和外观）
- 匹配分数：78
- 页面类型：other / topic
- 行数：243 / 197
- 段落行数：106 / 49
- Evidence：0 / 15
- Mermaid：7 / 1
- 文件提及重合：package.js
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：main.ts、manager.ts、preview.ts

### 高级功能/扩展开发/构建器扩展.md

- reference 标题：构建器扩展
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：120
- 页面类型：other / other
- 行数：276 / 165
- 段落行数：105 / 42
- Evidence：0 / 5
- Mermaid：5 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、types.ts、builder.ts、storybook-builder-api-interface.md、storybook-main-versioned-webpack.md、builder-api.md、vite.conf
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、types.ts、builder.ts、storybook-builder-api-interface.md、storybook-main-versioned-webpack.md、builder-api.md、vite.conf

### 高级功能/扩展开发/渲染器扩展.md

- reference 标题：渲染器扩展
- 生成页：核心模块/核心模块.md（核心模块）
- 匹配分数：142
- 页面类型：other / module
- 行数：333 / 176
- 段落行数：127 / 44
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：render.ts、index.ts、preset.ts、entry-preview.ts
- reference 关键文件未覆盖：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts

### 高级功能/扩展开发/预设开发.md

- reference 标题：预设开发
- 生成页：核心模块/code.md（code）
- 匹配分数：84
- 页面类型：other / module
- 行数：292 / 285
- 段落行数：123 / 196
- Evidence：0 / 5
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js

### 高级功能/自定义渲染器.md

- reference 标题：自定义渲染器
- 生成页：多框架支持/renderers.md（renderers）
- 匹配分数：264
- 页面类型：other / other
- 行数：324 / 140
- 段落行数：98 / 54
- Evidence：0 / 5
- Mermaid：4 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、preview.ts、render.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：package.js、preview.ts、render.ts、types.ts

### 高级功能/预设配置.md

- reference 标题：预设配置
- 生成页：插件生态/presets.md（presets）
- 匹配分数：178
- 页面类型：other / other
- 行数：288 / 132
- 段落行数：110 / 46
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：preset.js、index.ts
- reference 关键文件未覆盖：presets.ts、package.js、types.ts、index.js
- 结论：章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：presets.ts、package.js、types.ts、index.js

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：136
- 页面类型：other / other
- 行数：375 / 178
- 段落行数：139 / 44
- Evidence：0 / 5
- Mermaid：6 / 1
- 文件提及重合：main.ts、preset.ts、index.ts
- reference 关键文件未覆盖：.eslintrc.js、manager.ts、preview.ts、package.js、build.ts、postinstalladdon.ts、prettier.conf
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；主章节骨架偏离 reference；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、package.js、build.ts、postinstalladdon.ts、prettier.conf

