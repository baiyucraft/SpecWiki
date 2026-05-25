# storybook Reference 对比报告

生成页面：142 页
reference 页面：176 页
命中对比：126 页
缺失对比：50 页
运行模式：warm (cache_mode=preserve)
LLM usage：requests=16, total_tokens=39895, page_research=0, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 2 / reference 8（repo-archetype=0）
- evidence 落页：generated 0 / reference 176
- citation 密度：generated 0 / reference 77.72
- 图表达覆盖：generated 0 / reference 176
- page research 请求：0
- page enrichment 请求：0
- 已规划专题类型：专题页(2)
- 高频缺失专题：无

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 专题页覆盖仍明显不足，很多 reference 主题还没有被 planner 单独承载
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- facts-driven 图输入尚未稳定覆盖到代表性页面
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 概念指南/Preview-API.md 被 2 个 reference 页面共享映射
- 主题系统/themes.md 被 9 个 reference 页面共享映射
- 多框架支持/svelte-vite.md 被 2 个 reference 页面共享映射
- 多框架支持/多框架支持.md 被 2 个 reference 页面共享映射
- API-参考/code.md 被 3 个 reference 页面共享映射
- 平台绑定-Web/builder-webpack5.md 被 3 个 reference 页面共享映射
- 系统架构.md 被 13 个 reference 页面共享映射
- 项目概述.md 被 61 个 reference 页面共享映射
- 核心模块/html.md 被 2 个 reference 页面共享映射

## 额外生成页面

- API-参考/API-参考.md (API 参考, 25 行)
- API-参考/angular.md (API：angular, 24 行)
- API-参考/core.md (API：core, 24 行)
- API-参考/ember-cli.md (API：ember-cli, 24 行)
- API-参考/frameworks.md (API：frameworks, 24 行)
- API-参考/nextjs-vite.md (API：nextjs-vite, 24 行)
- API-参考/nextjs.md (API：nextjs, 23 行)
- API-参考/portable-stories-kitchen-sink.md (API：portable-stories-kitchen-sink, 24 行)
- API-参考/scripts.md (API：scripts, 23 行)
- API-参考/server-kitchen-sink.md (API：server-kitchen-sink, 23 行)
- API-参考/storybook.md (API：storybook, 24 行)
- API-参考/test-storybooks.md (API：test-storybooks, 24 行)
- API-参考/yarn-pnp.md (API：yarn-pnp, 23 行)
- 主题系统/主题系统.md (主题系统, 13 行)
- 多框架支持/angular.md (angular, 32 行)
- 多框架支持/html-vite.md (html-vite, 32 行)
- 多框架支持/preact-vite.md (preact-vite, 32 行)
- 多框架支持/preact.md (preact, 32 行)
- 多框架支持/react-dom-shim.md (react-dom-shim, 29 行)
- 多框架支持/react-native-web-vite.md (react-native-web-vite, 32 行)
- 多框架支持/react-vitest-3.md (react-vitest-3, 32 行)
- 多框架支持/react-webpack.md (react-webpack, 32 行)
- 多框架支持/react.md (react, 32 行)
- 多框架支持/renderers.md (renderers, 32 行)
- 多框架支持/server-webpack5.md (server-webpack5, 32 行)
- 多框架支持/svelte.md (svelte, 32 行)
- 多框架支持/vue3.md (vue3, 32 行)
- 多框架支持/web-components-vite.md (web-components-vite, 32 行)
- 平台绑定-Web/core-webpack.md (core-webpack, 32 行)
- 平台绑定-Web/react-native-web-vite.md (react-native-web-vite, 32 行)
- 平台绑定-Web/react-webpack.md (react-webpack, 32 行)
- 平台绑定-Web/react-webpack5.md (react-webpack5, 32 行)
- 平台绑定-Web/server-webpack.md (server-webpack, 32 行)
- 平台绑定-Web/server-webpack5.md (server-webpack5, 32 行)
- 平台绑定-Web/web-components-vite.md (web-components-vite, 32 行)
- 平台绑定-Web/平台绑定-Web.md (平台绑定：Web, 21 行)
- 开发工具/cli-sb.md (cli-sb, 31 行)
- 开发工具/cli-storybook.md (cli-storybook, 32 行)
- 开发工具/core.md (core, 32 行)
- 开发工具/create-storybook.md (create-storybook, 32 行)
- 开发工具/ember-cli.md (ember-cli, 32 行)
- 开发工具/scripts.md (scripts, 32 行)
- 开发工具/开发工具.md (开发工具, 18 行)
- 插件生态/addons.md (addons, 32 行)
- 插件生态/create-react-app.md (create-react-app, 29 行)
- 插件生态/csf-plugin.md (csf-plugin, 31 行)
- 插件生态/docs.md (docs, 17 行)
- 插件生态/eslint-plugin-local-rules.md (eslint-plugin-local-rules, 28 行)
- 插件生态/links.md (links, 32 行)
- 插件生态/onboarding.md (onboarding, 32 行)
- 插件生态/presets.md (presets, 32 行)
- 插件生态/pseudo-states.md (pseudo-states, 32 行)
- 插件生态/react-webpack.md (react-webpack, 32 行)
- 插件生态/server-webpack.md (server-webpack, 32 行)
- 插件生态/themes.md (themes, 32 行)
- 插件生态/vitest.md (vitest, 32 行)
- 插件生态/插件生态.md (插件生态, 27 行)
- 构建系统/builder-webpack5.md (builder-webpack5, 32 行)
- 构建系统/builders.md (builders, 32 行)
- 构建系统/core-webpack.md (core-webpack, 32 行)
- 构建系统/html-vite.md (html-vite, 32 行)
- 构建系统/nextjs-vite.md (nextjs-vite, 32 行)
- 构建系统/preact-vite.md (preact-vite, 32 行)
- 构建系统/react-native-web-vite.md (react-native-web-vite, 32 行)
- 构建系统/react-vite.md (react-vite, 32 行)
- 构建系统/react-vitest-3.md (react-vitest-3, 32 行)
- 构建系统/react-webpack.md (react-webpack, 32 行)
- 构建系统/react-webpack5.md (react-webpack5, 32 行)
- 构建系统/server-webpack.md (server-webpack, 32 行)
- 构建系统/server-webpack5.md (server-webpack5, 32 行)
- 构建系统/svelte-vite.md (svelte-vite, 32 行)
- 构建系统/vitest.md (vitest, 32 行)
- 构建系统/vue3-vite.md (vue3-vite, 32 行)
- 构建系统/web-components-vite.md (web-components-vite, 32 行)
- 核心模块/code.md (code, 32 行)
- 核心模块/docs.md (docs, 32 行)
- 核心模块/ember.md (ember, 32 行)
- 核心模块/external-docs.md (external-docs, 32 行)
- 核心模块/frameworks.md (frameworks, 32 行)
- 核心模块/nextjs.md (nextjs, 32 行)
- 核心模块/portable-stories-kitchen-sink.md (portable-stories-kitchen-sink, 32 行)
- 核心模块/server-kitchen-sink.md (server-kitchen-sink, 32 行)
- 核心模块/server.md (server, 32 行)
- 核心模块/standalone-preview.md (standalone-preview, 31 行)
- 核心模块/storybook.md (storybook, 32 行)
- 核心模块/test-storybooks.md (test-storybooks, 32 行)
- 核心模块/yarn-pnp.md (yarn-pnp, 32 行)
- 核心模块/yarn.md (.yarn, 27 行)
- 核心模块/核心模块.md (核心模块, 28 行)
- 核心运行时/core-webpack.md (core-webpack, 32 行)
- 核心运行时/core.md (core, 32 行)
- 核心运行时/核心运行时.md (核心运行时, 15 行)
- 框架集成-Gin/csf-plugin.md (csf-plugin, 31 行)
- 框架集成-Gin/eslint-plugin-local-rules.md (eslint-plugin-local-rules, 28 行)
- 框架集成-Gin/eslint-plugin.md (eslint-plugin, 32 行)
- 框架集成-Gin/框架集成-Gin.md (框架集成：Gin, 15 行)
- 概念指南/Addon-API.md (Addon API, 9 行)
- 概念指南/Decorators-API.md (Decorators API, 9 行)
- 概念指南/概念指南.md (概念指南, 32 行)
- 测试基础设施/react-vitest-3.md (测试：react-vitest-3, 20 行)
- 测试基础设施/test-storybooks.md (测试：test-storybooks, 20 行)
- 测试基础设施/vitest.md (测试：vitest, 20 行)
- 测试基础设施/测试基础设施.md (测试基础设施, 15 行)
- 配置参考/配置参考.md (配置参考, 13 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考/API参考.md | 概念指南/API参考.md | 368/9 | 101/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts |
| API参考/CLI命令参考.md | 概念指南/CLI命令参考.md | 280/9 | 82/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、core.ts、dispatcher.ts、get-storybook-configuration.ts、index.js、initiate.ts、package.js |
| API参考/开发API参考/Addon API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/CSF API.md | 概念指南/CSF-API.md | 277/9 | 131/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts |
| API参考/开发API参考/Decorators API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Hooks API.md | 概念指南/Hooks-API.md | 343/9 | 105/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：hooks.test.js、hooks.ts、main.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts |
| API参考/开发API参考/Preview API.md | 概念指南/Preview-API.md | 231/9 | 108/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview-api.ts、previewweb.ts、webview.ts、index.ts、simulate-pageload.ts、preview-api.spec.ts |
| API参考/开发API参考/Store API.md | 概念指南/Store-API.md | 397/9 | 148/2 | 0/0 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts |
| API参考/开发API参考/Types API.md | 概念指南/Types-API.md | 273/9 | 82/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts |
| API参考/开发API参考/开发API参考.md | 概念指南/开发API参考.md | 318/9 | 107/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、index.ts、addons.ts、shortcuts.ts、hooks.ts |
| API参考/类型定义参考/API类型定义.md | 概念指南/API类型定义.md | 284/9 | 122/2 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts |
| API参考/类型定义参考/工具类型定义.md | 概念指南/工具类型定义.md | 296/9 | 172/2 | 16/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、root.ts、store.ts、index.ts、hooks.ts、features.ts |
| API参考/类型定义参考/插件类型定义.md | 概念指南/插件类型定义.md | 430/9 | 214/2 | 17/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：codegen-set-addon-channel.ts、types.ts、addons.ts、addons.test.js、typings.d.ts、main.ts |
| API参考/类型定义参考/构建器类型定义.md | 概念指南/构建器类型定义.md | 324/9 | 240/2 | 17/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js |
| API参考/类型定义参考/核心类型定义.md | 概念指南/核心类型定义.md | 358/9 | 255/2 | 18/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、index.ts、story.ts、make-decorator.ts、decorators.ts |
| API参考/类型定义参考/框架类型定义.md | 概念指南/框架类型定义.md | 337/9 | 139/2 | 17/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts |
| API参考/类型定义参考/类型定义参考.md | 概念指南/类型定义参考.md | 480/9 | 362/2 | 22/0 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、typings.d.ts、main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts |
| API参考/配置API参考/main.js配置.md | 概念指南/main-js配置.md | 407/9 | 130/2 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、index.md、frameworks.md |
| API参考/配置API参考/manager.js配置.md | 概念指南/manager-js配置.md | 254/9 | 100/2 | 16/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md |
| API参考/配置API参考/preview.js配置.md | 概念指南/preview-js配置.md | 351/9 | 127/2 | 19/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js |
| API参考/配置API参考/构建器配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/配置API参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/预设配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 主题和外观/主题和外观.md | 主题系统/themes.md | 359/32 | 132/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts |
| 主题和外观/主题系统概览.md | 主题系统/themes.md | 302/32 | 113/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts、light.ts |
| 主题和外观/布局和间距设计.md | 主题系统/themes.md | 185/32 | 64/5 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js |
| 主题和外观/自定义主题开发.md | 主题系统/themes.md | 372/32 | 123/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts |
| 主题和外观/颜色和字体系统.md | 插件生态/a11y.md | 244/32 | 90/5 | 13/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunnerutils.ts、axerulemappinghelper.ts、constants.ts |
| 多框架支持/Angular框架支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 多框架支持/HTML框架支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 多框架支持/React框架支持.md | 多框架支持/react-webpack5.md | 349/32 | 225/5 | 16/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js |
| 多框架支持/Svelte框架支持.md | 多框架支持/svelte-vite.md | 355/32 | 233/5 | 15/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts |
| 多框架支持/Vue 3框架支持.md | 多框架支持/vue3-vite.md | 279/32 | 115/5 | 15/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts |
| 多框架支持/Web Components框架支持.md | 平台绑定-Web/web-components.md | 300/32 | 223/5 | 13/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts |
| 多框架支持/多框架支持.md | 多框架支持/多框架支持.md | 259/32 | 87/2 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、package.js、index.ts |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/Addons概览.md | API-参考/code.md | 276/24 | 97/4 | 16/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts |
| 插件系统/插件系统.md | API-参考/code.md | 356/24 | 107/4 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js |
| 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Actions Addon（动作记录）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Controls Addon（参数控制）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Docs Addon（文档生成）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Links Addon（故事导航）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Themes Addon（主题切换）.md | 主题系统/themes.md | 279/32 | 120/5 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md |
| 插件系统/核心Addons详解/核心Addons详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/第三方Addons集成.md | API-参考/code.md | 246/24 | 98/4 | 20/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md |
| 插件系统/自定义Addons开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/兼容性问题.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/安装问题.md | 核心运行时/lib.md | 350/32 | 106/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts |
| 故障排除/故障排除.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/构建和性能问题.md | 平台绑定-Web/builder-webpack5.md | 312/32 | 98/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts |
| 故障排除/调试工具和技巧.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/运行时错误.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/配置错误.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 构建系统/Vite构建器详解.md | 构建系统/builder-vite.md | 300/32 | 120/5 | 0/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts |
| 构建系统/Webpack构建器详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 构建系统/构建器概览.md | 平台绑定-Web/builder-webpack5.md | 321/32 | 113/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf |
| 构建系统/构建系统.md | 构建系统/构建系统.md | 291/30 | 103/2 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、index.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts |
| 构建系统/构建配置优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/Addons系统架构/Addons开发指南.md | 系统架构.md | 584/6 | 390/2 | 23/0 | 13/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、main.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts |
| 核心概念/Addons系统架构/Addons架构设计.md | 系统架构.md | 321/6 | 146/2 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts |
| 核心概念/Addons系统架构/Addons系统架构.md | 系统架构.md | 356/6 | 156/2 | 22/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md |
| 核心概念/Addons系统架构/Addons通信机制.md | 系统架构.md | 290/6 | 131/2 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、main.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts |
| 核心概念/Addons系统架构/Addons配置管理.md | 系统架构.md | 273/6 | 85/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、main.ts、package.js、preset.ts、presets.ts |
| 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md | 系统架构.md | 362/6 | 125/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts |
| 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md | 系统架构.md | 302/6 | 103/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts |
| 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md | 系统架构.md | 374/6 | 115/2 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts |
| 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md | 系统架构.md | 282/6 | 95/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts |
| 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md | 系统架构.md | 348/6 | 127/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts |
| 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md | 主题系统/themes.md | 293/32 | 127/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、types.ts、themes.md |
| 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md | 系统架构.md | 290/6 | 128/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts |
| 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md | 系统架构.md | 257/6 | 98/2 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts |
| 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md | 系统架构.md | 309/6 | 113/2 | 7/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts |
| 核心概念/CSF格式规范.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/主题和参数系统.md | 主题系统/themes.md | 356/32 | 126/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts |
| 核心概念/核心概念.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/组件故事（Stories）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/装饰器和全局状态.md | 主题系统/themes.md | 307/32 | 119/5 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、api.md、helpers.ts、hooks.ts、decorator.ts、next.js |
| 核心概念/预览和管理界面.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/Playwright测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/Vitest集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/可访问性测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/测试框架.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/组件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/视觉回归测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/CI_CD集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/Chromatic集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/环境配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/部署和CI_CD.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/静态部署.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 项目概述/使用场景和案例.md | 项目概述.md | 271/28 | 100/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、readme.md、package.js、index.md |
| 项目概述/基本概念.md | 项目概述.md | 280/28 | 80/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md |
| 项目概述/技术架构/Monorepo设计.md | 项目概述.md | 249/28 | 78/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、nx.js、build-package.ts、check-package.ts、prepare-sandbox.ts |
| 项目概述/技术架构/技术架构.md | 项目概述.md | 289/28 | 141/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts |
| 项目概述/技术架构/插件系统架构/插件架构设计.md | 项目概述.md | 346/28 | 158/3 | 17/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md |
| 项目概述/技术架构/插件系统架构/插件注册机制.md | 项目概述.md | 263/28 | 100/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md |
| 项目概述/技术架构/插件系统架构/插件生命周期管理.md | 项目概述.md | 266/28 | 94/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts |
| 项目概述/技术架构/插件系统架构/插件系统架构.md | 项目概述.md | 315/28 | 127/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md |
| 项目概述/技术架构/插件系统架构/插件通信协议.md | 项目概述.md | 312/28 | 136/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、main.ts、types.ts |
| 项目概述/技术架构/构建系统架构/Vite构建器.md | 项目概述.md | 283/28 | 127/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts |
| 项目概述/技术架构/构建系统架构/Webpack构建器.md | 项目概述.md | 242/28 | 84/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js |
| 项目概述/技术架构/构建系统架构/构建器架构设计.md | 项目概述.md | 363/28 | 146/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts |
| 项目概述/技术架构/构建系统架构/构建系统架构.md | 项目概述.md | 244/28 | 105/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts |
| 项目概述/技术架构/核心引擎架构/CLI分发器.md | 项目概述.md | 308/28 | 152/3 | 0/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts |
| 项目概述/技术架构/核心引擎架构/全局设置.md | 项目概述.md | 360/28 | 143/3 | 18/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、main.ts、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts |
| 项目概述/技术架构/核心引擎架构/核心引擎架构.md | 项目概述.md | 272/28 | 122/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js |
| 项目概述/技术架构/核心引擎架构/核心服务器.md | 项目概述.md | 334/28 | 131/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts |
| 项目概述/技术架构/核心引擎架构/版本管理.md | 项目概述.md | 359/28 | 137/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts |
| 项目概述/技术架构/框架适配器架构/Angular框架适配器.md | 项目概述.md | 355/28 | 245/3 | 17/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts |
| 项目概述/技术架构/框架适配器架构/HTML框架适配器.md | 项目概述.md | 245/28 | 99/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js |
| 项目概述/技术架构/框架适配器架构/React框架适配器.md | 项目概述.md | 326/28 | 239/3 | 17/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js |
| 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md | 多框架支持/svelte-vite.md | 287/32 | 131/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts |
| 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md | 项目概述.md | 305/28 | 212/3 | 15/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts |
| 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md | 项目概述.md | 478/28 | 315/3 | 17/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、main.ts、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts |
| 项目概述/技术架构/框架适配器架构/框架适配器架构.md | 项目概述.md | 297/28 | 136/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js、main.ts |
| 项目概述/技术架构/渲染系统架构/HTML渲染器.md | 核心模块/html.md | 250/32 | 108/5 | 14/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts |
| 项目概述/技术架构/渲染系统架构/React渲染器.md | 项目概述.md | 238/28 | 91/3 | 12/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts |
| 项目概述/技术架构/渲染系统架构/Svelte渲染器.md | 项目概述.md | 319/28 | 149/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts |
| 项目概述/技术架构/渲染系统架构/Vue3渲染器.md | 项目概述.md | 217/28 | 83/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts |
| 项目概述/技术架构/渲染系统架构/渲染系统架构.md | 项目概述.md | 267/28 | 130/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts |
| 项目概述/核心特性/主题定制系统/主题切换机制.md | 项目概述.md | 303/28 | 101/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、index.ts、create.ts、utils.ts |
| 项目概述/核心特性/主题定制系统/主题定制系统.md | 项目概述.md | 275/28 | 143/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts |
| 项目概述/核心特性/主题定制系统/主题扩展开发.md | 项目概述.md | 343/28 | 119/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts |
| 项目概述/核心特性/主题定制系统/字体系统.md | 项目概述.md | 221/28 | 69/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts |
| 项目概述/核心特性/主题定制系统/布局系统.md | 项目概述.md | 314/28 | 121/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts |
| 项目概述/核心特性/主题定制系统/颜色系统.md | 项目概述.md | 192/28 | 52/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts |
| 项目概述/核心特性/多框架支持/Angular框架支持.md | 项目概述.md | 302/28 | 137/3 | 22/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts |
| 项目概述/核心特性/多框架支持/Next.js框架支持.md | 项目概述.md | 237/28 | 109/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js |
| 项目概述/核心特性/多框架支持/React框架支持.md | 多框架支持/react-vite.md | 296/32 | 107/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、preview.ts、public-types.ts、types.ts |
| 项目概述/核心特性/多框架支持/Svelte框架支持.md | 多框架支持/sveltekit.md | 299/32 | 106/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、svelte-docgen.ts、types.ts、globals.ts、public-types.ts |
| 项目概述/核心特性/多框架支持/Vue框架支持.md | 项目概述.md | 234/28 | 129/3 | 12/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts |
| 项目概述/核心特性/多框架支持/Web Components框架支持.md | 项目概述.md | 268/28 | 110/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、main.ts、index.ts、package.js、preset.js、preset.ts |
| 项目概述/核心特性/多框架支持/其他框架支持.md | 项目概述.md | 309/28 | 119/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js |
| 项目概述/核心特性/多框架支持/多框架支持.md | 多框架支持/多框架支持.md | 494/32 | 123/2 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts |
| 项目概述/核心特性/插件系统架构/Addon API设计.md | 项目概述.md | 262/28 | 98/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts |
| 项目概述/核心特性/插件系统架构/Manager API.md | 项目概述.md | 335/28 | 124/3 | 17/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts |
| 项目概述/核心特性/插件系统架构/Preview API.md | 概念指南/Preview-API.md | 267/9 | 101/2 | 15/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js |
| 项目概述/核心特性/插件系统架构/插件生命周期管理.md | 项目概述.md | 322/28 | 119/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md |
| 项目概述/核心特性/插件系统架构/插件系统架构.md | 项目概述.md | 417/28 | 167/3 | 0/0 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、main.ts、types.ts、instrumenter.test.ts、addons.ts |
| 项目概述/核心特性/插件系统架构/通信机制.md | 项目概述.md | 281/28 | 128/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts |
| 项目概述/核心特性/文档生成功能/Doc Blocks API.md | 项目概述.md | 389/28 | 121/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts |
| 项目概述/核心特性/文档生成功能/MDX文档编写.md | 项目概述.md | 269/28 | 121/3 | 19/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md |
| 项目概述/核心特性/文档生成功能/文档生成功能.md | 项目概述.md | 341/28 | 129/3 | 17/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts |
| 项目概述/核心特性/文档生成功能/自动文档系统.md | 项目概述.md | 267/28 | 118/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts |
| 项目概述/核心特性/文档生成功能/自定义文档页面.md | 项目概述.md | 260/28 | 110/3 | 18/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md |
| 项目概述/核心特性/核心特性.md | 项目概述.md | 349/28 | 138/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts |
| 项目概述/核心特性/测试支持功能/Playwright测试.md | 项目概述.md | 373/28 | 157/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js |
| 项目概述/核心特性/测试支持功能/Vitest集成.md | 项目概述.md | 291/28 | 114/3 | 23/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、main.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts |
| 项目概述/核心特性/测试支持功能/可访问性测试.md | 项目概述.md | 311/28 | 136/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts |
| 项目概述/核心特性/测试支持功能/测试支持功能.md | 项目概述.md | 338/28 | 133/3 | 21/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、main.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts |
| 项目概述/核心特性/测试支持功能/组件测试.md | 项目概述.md | 266/28 | 106/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts |
| 项目概述/核心特性/组件开发环境/交互式调试.md | 项目概述.md | 264/28 | 110/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts |
| 项目概述/核心特性/组件开发环境/实时预览.md | 项目概述.md | 311/28 | 108/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts |
| 项目概述/核心特性/组件开发环境/开发服务器.md | 项目概述.md | 303/28 | 124/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts |
| 项目概述/核心特性/组件开发环境/组件开发环境.md | 项目概述.md | 312/28 | 139/3 | 21/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts |
| 项目概述/核心特性/组件开发环境/组件生命周期.md | 项目概述.md | 319/28 | 122/3 | 16/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、main.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts |
| 项目概述/项目概述.md | 项目概述.md | 312/28 | 131/3 | 21/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、main.ts、package.js、index.md、tsconfig.js |
| 高级功能/工具集成/CI_CD集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/工具集成/Codemod工具.md | 核心模块/codemod.md | 286/32 | 114/5 | 18/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts |
| 高级功能/工具集成/ESLint集成.md | 插件生态/eslint-plugin.md | 269/32 | 69/5 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts |
| 高级功能/工具集成/IDE配置优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/工具集成/工具集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/工具集成/构建工具优化.md | 平台绑定-Web/builder-webpack5.md | 287/32 | 93/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts |
| 高级功能/性能监控.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/Addon开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/主题开发.md | 主题系统/themes.md | 320/32 | 149/5 | 20/0 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js |
| 高级功能/扩展开发/扩展开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/构建器扩展.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/渲染器扩展.md | 核心模块/html.md | 333/32 | 127/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts |
| 高级功能/扩展开发/预设开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/自定义渲染器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/预设配置.md | 多框架支持/create-react-app.md | 288/29 | 110/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js |
| 高级功能/高级功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |

## 逐文件详情

### API参考/API参考.md

- reference 标题：API参考
- 生成页：概念指南/API参考.md（API参考）
- 匹配分数：240
- 页面类型：other / other
- 行数：368 / 9
- 段落行数：101 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts

### API参考/CLI命令参考.md

- reference 标题：CLI命令参考
- 生成页：概念指南/CLI命令参考.md（CLI命令参考）
- 匹配分数：242
- 页面类型：other / other
- 行数：280 / 9
- 段落行数：82 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、migration.md、core.ts、dispatcher.ts、get-storybook-configuration.ts、index.js、initiate.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、core.ts、dispatcher.ts、get-storybook-configuration.ts、index.js、initiate.ts、package.js

### API参考/开发API参考/Addon API.md

- reference 标题：插件API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/CSF API.md

- reference 标题：CSF API
- 生成页：概念指南/CSF-API.md（CSF API）
- 匹配分数：244
- 页面类型：other / other
- 行数：277 / 9
- 段落行数：131 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、csf-factory-template.test.ts、csf-factory-template.ts、readme.md、core-annotations.ts、csf-factories.test.ts、csf-factories.ts、csf-factory-utils.ts

### API参考/开发API参考/Decorators API.md

- reference 标题：装饰器API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Hooks API.md

- reference 标题：Hooks API
- 生成页：概念指南/Hooks-API.md（Hooks API）
- 匹配分数：244
- 页面类型：other / other
- 行数：343 / 9
- 段落行数：105 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：hooks.test.js、hooks.ts、main.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：hooks.test.js、hooks.ts、main.ts、preview.ts、hooks.test.ts、hooks.stories.ts、order-of-hooks.stories.ts

### API参考/开发API参考/Preview API.md

- reference 标题：Preview API
- 生成页：概念指南/Preview-API.md（Preview API）
- 匹配分数：244
- 页面类型：other / other
- 行数：231 / 9
- 段落行数：108 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：preview-api.ts、previewweb.ts、webview.ts、index.ts、simulate-pageload.ts、preview-api.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview-api.ts、previewweb.ts、webview.ts、index.ts、simulate-pageload.ts、preview-api.spec.ts

### API参考/开发API参考/Store API.md

- reference 标题：Store API
- 生成页：概念指南/Store-API.md（Store API）
- 匹配分数：246
- 页面类型：other / other
- 行数：397 / 9
- 段落行数：148 / 2
- Evidence：0 / 0
- Mermaid：10 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：storyindexstore.ts、store-setup.ts、url.ts、root.ts、store.ts、urlstore.ts、argsstore.ts、globalsstore.ts

### API参考/开发API参考/Types API.md

- reference 标题：Types API
- 生成页：概念指南/Types-API.md（Types API）
- 匹配分数：244
- 页面类型：other / other
- 行数：273 / 9
- 段落行数：82 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：migration.md、story.ts、types.ts、stories.ts、previewweb.ts、args.test.ts、inferargtypes.test.ts、typings.d.ts

### API参考/开发API参考/开发API参考.md

- reference 标题：开发API参考
- 生成页：概念指南/开发API参考.md（开发API参考）
- 匹配分数：244
- 页面类型：other / other
- 行数：318 / 9
- 段落行数：107 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、index.ts、addons.ts、shortcuts.ts、hooks.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、index.ts、addons.ts、shortcuts.ts、hooks.ts

### API参考/类型定义参考/API类型定义.md

- reference 标题：API类型定义
- 生成页：概念指南/API类型定义.md（API类型定义）
- 匹配分数：242
- 页面类型：other / other
- 行数：284 / 9
- 段落行数：122 / 2
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、sendtelemetryerror.ts、typings.d.ts、preview-api.spec.ts

### API参考/类型定义参考/工具类型定义.md

- reference 标题：工具类型定义
- 生成页：概念指南/工具类型定义.md（工具类型定义）
- 匹配分数：240
- 页面类型：other / other
- 行数：296 / 9
- 段落行数：172 / 2
- Evidence：16 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：types.ts、root.ts、store.ts、index.ts、hooks.ts、features.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、root.ts、store.ts、index.ts、hooks.ts、features.ts

### API参考/类型定义参考/插件类型定义.md

- reference 标题：插件类型定义
- 生成页：概念指南/插件类型定义.md（插件类型定义）
- 匹配分数：242
- 页面类型：other / other
- 行数：430 / 9
- 段落行数：214 / 2
- Evidence：17 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：codegen-set-addon-channel.ts、types.ts、addons.ts、addons.test.js、typings.d.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：codegen-set-addon-channel.ts、types.ts、addons.ts、addons.test.js、typings.d.ts、main.ts

### API参考/类型定义参考/构建器类型定义.md

- reference 标题：构建器类型定义
- 生成页：概念指南/构建器类型定义.md（构建器类型定义）
- 匹配分数：240
- 页面类型：other / other
- 行数：324 / 9
- 段落行数：240 / 2
- Evidence：17 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、types.ts、builder.ts、builders.ts、main-config-vite-final.md、builder-api.md、main.js

### API参考/类型定义参考/核心类型定义.md

- reference 标题：核心类型定义
- 生成页：概念指南/核心类型定义.md（核心类型定义）
- 匹配分数：240
- 页面类型：other / other
- 行数：358 / 9
- 段落行数：255 / 2
- Evidence：18 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、index.ts、story.ts、make-decorator.ts、decorators.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、index.ts、story.ts、make-decorator.ts、decorators.ts

### API参考/类型定义参考/框架类型定义.md

- reference 标题：框架类型定义
- 生成页：概念指南/框架类型定义.md（框架类型定义）
- 匹配分数：238
- 页面类型：other / other
- 行数：337 / 9
- 段落行数：139 / 2
- Evidence：17 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework.ts、story.ts、frameworks.ts、renderers.ts、abstractrenderer.ts、canvasrenderer.ts、docsrenderer.ts、rendererfactory.ts

### API参考/类型定义参考/类型定义参考.md

- reference 标题：类型定义参考
- 生成页：概念指南/类型定义参考.md（类型定义参考）
- 匹配分数：240
- 页面类型：other / other
- 行数：480 / 9
- 段落行数：362 / 2
- Evidence：22 / 0
- Mermaid：12 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：types.ts、typings.d.ts、main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：types.ts、typings.d.ts、main.ts、builders.ts、core-common.ts、frameworks.ts、renderers.ts

### API参考/配置API参考/main.js配置.md

- reference 标题：main.js配置
- 生成页：概念指南/main-js配置.md（main.js配置）
- 匹配分数：270
- 页面类型：other / other
- 行数：407 / 9
- 段落行数：130 / 2
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：main.js
- reference 关键文件未覆盖：main.ts、main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、index.md、frameworks.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、main-config-typical.md、addon-types.md、main-config-build.md、main-config-core.md、main-config-features.md、index.md、frameworks.md

### API参考/配置API参考/manager.js配置.md

- reference 标题：manager.js配置
- 生成页：概念指南/manager-js配置.md（manager.js配置）
- 匹配分数：268
- 页面类型：other / other
- 行数：254 / 9
- 段落行数：100 / 2
- Evidence：16 / 0
- Mermaid：7 / 0
- 文件提及重合：manager.js
- reference 关键文件未覆盖：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、layout.ts、layout.test.ts、layout.stories.ts、index.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md

### API参考/配置API参考/preview.js配置.md

- reference 标题：preview.js配置
- 生成页：概念指南/preview-js配置.md（preview.js配置）
- 匹配分数：268
- 页面类型：other / other
- 行数：351 / 9
- 段落行数：127 / 2
- Evidence：19 / 0
- Mermaid：9 / 0
- 文件提及重合：preview.js
- reference 关键文件未覆盖：preview.ts、basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、basic.stories.ts、process-preview-annotation.ts、preview-preset.ts、composeconfigs.test.ts、getvaluesfromglobaltypes.ts、storybook-builder-api-preview-exports.md、next.js

### API参考/配置API参考/构建器配置.md

- reference 标题：构建器配置
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/配置API参考.md

- reference 标题：配置API参考
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/预设配置.md

- reference 标题：预设配置
- 生成页：无
- 问题：缺少对应生成页面

### 主题和外观/主题和外观.md

- reference 标题：主题和外观
- 生成页：主题系统/themes.md（themes）
- 匹配分数：114
- 页面类型：topic / topic
- 行数：359 / 32
- 段落行数：132 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、utils.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts

### 主题和外观/主题系统概览.md

- reference 标题：主题系统概览
- 生成页：主题系统/themes.md（themes）
- 匹配分数：116
- 页面类型：topic / topic
- 行数：302 / 32
- 段落行数：113 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts、theme-switcher.ts
- reference 关键文件未覆盖：api.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts、light.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts、light.ts

### 主题和外观/布局和间距设计.md

- reference 标题：布局和间距设计
- 生成页：主题系统/themes.md（themes）
- 匹配分数：110
- 页面类型：topic / topic
- 行数：185 / 32
- 段落行数：64 / 5
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js

### 主题和外观/自定义主题开发.md

- reference 标题：自定义主题开发
- 生成页：主题系统/themes.md（themes）
- 匹配分数：162
- 页面类型：topic / topic
- 行数：372 / 32
- 段落行数：123 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts

### 主题和外观/颜色和字体系统.md

- reference 标题：颜色和字体系统
- 生成页：插件生态/a11y.md（a11y）
- 匹配分数：146
- 页面类型：topic / other
- 行数：244 / 32
- 段落行数：90 / 5
- Evidence：13 / 0
- Mermaid：5 / 0
- 文件提及重合：manager.js、preset.js、preview.js、a11yrunner.ts、index.ts
- reference 关键文件未覆盖：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunnerutils.ts、axerulemappinghelper.ts、constants.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunnerutils.ts、axerulemappinghelper.ts、constants.ts

### 多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：无
- 问题：缺少对应生成页面

### 多框架支持/HTML框架支持.md

- reference 标题：HTML框架支持
- 生成页：无
- 问题：缺少对应生成页面

### 多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：多框架支持/react-webpack5.md（react-webpack5）
- 匹配分数：64
- 页面类型：other / other
- 行数：349 / 32
- 段落行数：225 / 5
- Evidence：16 / 0
- Mermaid：7 / 0
- 文件提及重合：index.ts、preset.ts
- reference 关键文件未覆盖：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js

### 多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- 匹配分数：92
- 页面类型：other / other
- 行数：355 / 32
- 段落行数：233 / 5
- Evidence：15 / 0
- Mermaid：8 / 0
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts

### 多框架支持/Vue 3框架支持.md

- reference 标题：Vue 3框架支持
- 生成页：多框架支持/vue3-vite.md（vue3-vite）
- 匹配分数：94
- 页面类型：other / other
- 行数：279 / 32
- 段落行数：115 / 5
- Evidence：15 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts、preset.ts、vite-plugin.ts
- reference 关键文件未覆盖：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts

### 多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：平台绑定-Web/web-components.md（web-components）
- 匹配分数：140
- 页面类型：module / module
- 行数：300 / 32
- 段落行数：223 / 5
- Evidence：13 / 0
- Mermaid：7 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts

### 多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：488
- 页面类型：other / other
- 行数：259 / 32
- 段落行数：87 / 2
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、package.js、index.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/Addons概览.md

- reference 标题：Addons概览
- 生成页：API-参考/code.md（API：code）
- 匹配分数：66
- 页面类型：other / other
- 行数：276 / 24
- 段落行数：97 / 4
- Evidence：16 / 0
- Mermaid：5 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、preview.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts

### 插件系统/插件系统.md

- reference 标题：插件系统
- 生成页：API-参考/code.md（API：code）
- 匹配分数：94
- 页面类型：other / other
- 行数：356 / 24
- 段落行数：107 / 4
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：main.ts、manager.ts、index.ts
- reference 关键文件未覆盖：preview.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js

### 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md

- reference 标题：A11y Addon（可访问性测试）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Actions Addon（动作记录）.md

- reference 标题：Actions Addon（动作记录）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md

- reference 标题：Backgrounds Addon（背景设置）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Controls Addon（参数控制）.md

- reference 标题：Controls Addon（参数控制）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Docs Addon（文档生成）.md

- reference 标题：Docs Addon（文档生成）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Links Addon（故事导航）.md

- reference 标题：Links Addon（故事导航）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md

- reference 标题：Outline/Measure/Toolbar Addons（辅助工具）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Themes Addon（主题切换）.md

- reference 标题：Themes Addon（主题切换）
- 生成页：主题系统/themes.md（themes）
- 匹配分数：134
- 页面类型：topic / topic
- 行数：279 / 32
- 段落行数：120 / 5
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：class-name.decorator.ts、index.ts、provider.decorator.ts、preview.ts
- reference 关键文件未覆盖：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md

### 插件系统/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/第三方Addons集成.md

- reference 标题：第三方Addons集成
- 生成页：API-参考/code.md（API：code）
- 匹配分数：64
- 页面类型：other / other
- 行数：246 / 24
- 段落行数：98 / 4
- Evidence：20 / 0
- Mermaid：6 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、addon-types.md、index.md、writing-addons.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md

### 插件系统/自定义Addons开发.md

- reference 标题：自定义Addons开发
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/兼容性问题.md

- reference 标题：兼容性问题
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/安装问题.md

- reference 标题：安装问题
- 生成页：核心运行时/lib.md（lib）
- 匹配分数：70
- 页面类型：other / other
- 行数：350 / 32
- 段落行数：106 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：finalizationcommand.ts、preflightcheckcommand.ts、index.js
- reference 关键文件未覆盖：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/构建和性能问题.md

- reference 标题：构建和性能问题
- 生成页：平台绑定-Web/builder-webpack5.md（builder-webpack5）
- 匹配分数：64
- 页面类型：other / other
- 行数：312 / 32
- 段落行数：98 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts、base-webpack.config.ts
- reference 关键文件未覆盖：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts

### 故障排除/调试工具和技巧.md

- reference 标题：调试工具和技巧
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/运行时错误.md

- reference 标题：运行时错误
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/配置错误.md

- reference 标题：配置错误
- 生成页：无
- 问题：缺少对应生成页面

### 构建系统/Vite构建器详解.md

- reference 标题：Vite构建器详解
- 生成页：构建系统/builder-vite.md（builder-vite）
- 匹配分数：86
- 页面类型：other / other
- 行数：300 / 32
- 段落行数：120 / 5
- Evidence：0 / 0
- Mermaid：9 / 0
- 文件提及重合：index.ts、code-generator-plugin.ts、csf-plugin.ts
- reference 关键文件未覆盖：package.js、build.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts

### 构建系统/Webpack构建器详解.md

- reference 标题：Webpack构建器详解
- 生成页：无
- 问题：缺少对应生成页面

### 构建系统/构建器概览.md

- reference 标题：构建器概览
- 生成页：平台绑定-Web/builder-webpack5.md（builder-webpack5）
- 匹配分数：62
- 页面类型：other / other
- 行数：321 / 32
- 段落行数：113 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf

### 构建系统/构建系统.md

- reference 标题：构建系统
- 生成页：构建系统/构建系统.md（构建系统）
- 匹配分数：480
- 页面类型：other / other
- 行数：291 / 30
- 段落行数：103 / 2
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、package.js、build.ts、index.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、index.ts、types.ts、vite-config.ts、vite-server.ts、custom-webpack-preset.ts

### 构建系统/构建配置优化.md

- reference 标题：构建配置优化
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/Addons系统架构/Addons开发指南.md

- reference 标题：Addons开发指南
- 生成页：系统架构.md（系统架构）
- 匹配分数：86
- 页面类型：architecture / architecture
- 行数：584 / 6
- 段落行数：390 / 2
- Evidence：23 / 0
- Mermaid：13 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、.d.ts、main.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、main.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts

### 核心概念/Addons系统架构/Addons架构设计.md

- reference 标题：Addons架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：86
- 页面类型：architecture / architecture
- 行数：321 / 6
- 段落行数：146 / 2
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、channel.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts

### 核心概念/Addons系统架构/Addons系统架构.md

- reference 标题：Addons系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：356 / 6
- 段落行数：156 / 2
- Evidence：22 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md

### 核心概念/Addons系统架构/Addons通信机制.md

- reference 标题：Addons通信机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：290 / 6
- 段落行数：131 / 2
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：index.test.ts、index.ts、main.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、main.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts

### 核心概念/Addons系统架构/Addons配置管理.md

- reference 标题：Addons配置管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：86
- 页面类型：architecture / architecture
- 行数：273 / 6
- 段落行数：85 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.js、main.ts、package.js、preset.ts、presets.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、main.ts、package.js、preset.ts、presets.ts

### 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md

- reference 标题：A11y Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：362 / 6
- 段落行数：125 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts

### 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md

- reference 标题：Actions Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：302 / 6
- 段落行数：103 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts

### 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md

- reference 标题：Backgrounds Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：374 / 6
- 段落行数：115 / 2
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts

### 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md

- reference 标题：Controls Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：282 / 6
- 段落行数：95 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts

### 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md

- reference 标题：Docs Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：348 / 6
- 段落行数：127 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts

### 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md

- reference 标题：Themes Addon
- 生成页：主题系统/themes.md（themes）
- 匹配分数：170
- 页面类型：architecture / topic
- 行数：293 / 32
- 段落行数：127 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts、provider.decorator.ts、manager.ts、preview.ts、theme-switcher.ts
- reference 关键文件未覆盖：constants.ts、helpers.ts、types.ts、themes.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、types.ts、themes.md

### 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md

- reference 标题：工具栏插件（Toolbars Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：86
- 页面类型：architecture / architecture
- 行数：290 / 6
- 段落行数：128 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts

### 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md

- reference 标题：视口插件（Viewport Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：257 / 6
- 段落行数：98 / 2
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts

### 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：系统架构.md（系统架构）
- 匹配分数：84
- 页面类型：architecture / architecture
- 行数：309 / 6
- 段落行数：113 / 2
- Evidence：7 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts

### 核心概念/CSF格式规范.md

- reference 标题：CSF格式规范
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/主题和参数系统.md

- reference 标题：主题和参数系统
- 生成页：主题系统/themes.md（themes）
- 匹配分数：88
- 页面类型：topic / topic
- 行数：356 / 32
- 段落行数：126 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、constants.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/组件故事（Stories）.md

- reference 标题：组件故事（Stories）
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/装饰器和全局状态.md

- reference 标题：装饰器和全局状态
- 生成页：主题系统/themes.md（themes）
- 匹配分数：64
- 页面类型：other / topic
- 行数：307 / 32
- 段落行数：119 / 5
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：preview.ts、manager.ts、provider.decorator.ts、index.ts
- reference 关键文件未覆盖：main.ts、api.md、helpers.ts、hooks.ts、decorator.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、api.md、helpers.ts、hooks.ts、decorator.ts、next.js

### 核心概念/预览和管理界面.md

- reference 标题：预览和管理界面
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/测试框架.md

- reference 标题：测试框架
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/组件测试.md

- reference 标题：组件测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试框架/视觉回归测试.md

- reference 标题：视觉回归测试
- 生成页：无
- 问题：缺少对应生成页面

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/Chromatic集成.md

- reference 标题：Chromatic集成
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/环境配置.md

- reference 标题：环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/部署和CI_CD.md

- reference 标题：部署和CI/CD
- 生成页：无
- 问题：缺少对应生成页面

### 部署和CI_CD/静态部署.md

- reference 标题：静态部署
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述/使用场景和案例.md

- reference 标题：使用场景和案例
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：271 / 28
- 段落行数：100 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、readme.md、package.js、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、readme.md、package.js、index.md

### 项目概述/基本概念.md

- reference 标题：基本概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：280 / 28
- 段落行数：80 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md

### 项目概述/技术架构/Monorepo设计.md

- reference 标题：Monorepo设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：249 / 28
- 段落行数：78 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、project.js、nx.js、build-package.ts、check-package.ts、prepare-sandbox.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、nx.js、build-package.ts、check-package.ts、prepare-sandbox.ts

### 项目概述/技术架构/技术架构.md

- reference 标题：技术架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：289 / 28
- 段落行数：141 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts

### 项目概述/技术架构/插件系统架构/插件架构设计.md

- reference 标题：插件架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：346 / 28
- 段落行数：158 / 3
- Evidence：17 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md

### 项目概述/技术架构/插件系统架构/插件注册机制.md

- reference 标题：插件注册机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：263 / 28
- 段落行数：100 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md

### 项目概述/技术架构/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：266 / 28
- 段落行数：94 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts

### 项目概述/技术架构/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：315 / 28
- 段落行数：127 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md

### 项目概述/技术架构/插件系统架构/插件通信协议.md

- reference 标题：插件通信协议
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：312 / 28
- 段落行数：136 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：index.test.ts、index.ts、main.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、main.ts、types.ts

### 项目概述/技术架构/构建系统架构/Vite构建器.md

- reference 标题：Vite构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：283 / 28
- 段落行数：127 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts

### 项目概述/技术架构/构建系统架构/Webpack构建器.md

- reference 标题：Webpack构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：242 / 28
- 段落行数：84 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js

### 项目概述/技术架构/构建系统架构/构建器架构设计.md

- reference 标题：构建器架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：363 / 28
- 段落行数：146 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts

### 项目概述/技术架构/构建系统架构/构建系统架构.md

- reference 标题：构建系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：244 / 28
- 段落行数：105 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts

### 项目概述/技术架构/核心引擎架构/CLI分发器.md

- reference 标题：CLI分发器
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：308 / 28
- 段落行数：152 / 3
- Evidence：0 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts

### 项目概述/技术架构/核心引擎架构/全局设置.md

- reference 标题：全局设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：360 / 28
- 段落行数：143 / 3
- Evidence：18 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：settings.js、main.ts、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、main.ts、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts

### 项目概述/技术架构/核心引擎架构/核心引擎架构.md

- reference 标题：核心引擎架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：272 / 28
- 段落行数：122 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js

### 项目概述/技术架构/核心引擎架构/核心服务器.md

- reference 标题：核心服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：334 / 28
- 段落行数：131 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts

### 项目概述/技术架构/核心引擎架构/版本管理.md

- reference 标题：版本管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：359 / 28
- 段落行数：137 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts

### 项目概述/技术架构/框架适配器架构/Angular框架适配器.md

- reference 标题：Angular框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：355 / 28
- 段落行数：245 / 3
- Evidence：17 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts

### 项目概述/技术架构/框架适配器架构/HTML框架适配器.md

- reference 标题：HTML框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：245 / 28
- 段落行数：99 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js

### 项目概述/技术架构/框架适配器架构/React框架适配器.md

- reference 标题：React框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：326 / 28
- 段落行数：239 / 3
- Evidence：17 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js

### 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md

- reference 标题：Svelte框架适配器
- 生成页：多框架支持/svelte-vite.md（svelte-vite）
- 匹配分数：122
- 页面类型：overview / other
- 行数：287 / 32
- 段落行数：131 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts、utils.ts
- reference 关键文件未覆盖：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts

### 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md

- reference 标题：Vue 3框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：305 / 28
- 段落行数：212 / 3
- Evidence：15 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts

### 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md

- reference 标题：Web Components框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：478 / 28
- 段落行数：315 / 3
- Evidence：17 / 0
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：...conf、main.ts、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、main.ts、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts

### 项目概述/技术架构/框架适配器架构/框架适配器架构.md

- reference 标题：框架适配器架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：297 / 28
- 段落行数：136 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js、main.ts

### 项目概述/技术架构/渲染系统架构/HTML渲染器.md

- reference 标题：HTML渲染器
- 生成页：核心模块/html.md（html）
- 匹配分数：118
- 页面类型：overview / module
- 行数：250 / 32
- 段落行数：108 / 5
- Evidence：14 / 0
- Mermaid：6 / 0
- 文件提及重合：entry-preview.ts、index.ts、portable-stories.ts、render.ts
- reference 关键文件未覆盖：package.js、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts

### 项目概述/技术架构/渲染系统架构/React渲染器.md

- reference 标题：React渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：238 / 28
- 段落行数：91 / 3
- Evidence：12 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts

### 项目概述/技术架构/渲染系统架构/Svelte渲染器.md

- reference 标题：Svelte渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：319 / 28
- 段落行数：149 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts

### 项目概述/技术架构/渲染系统架构/Vue3渲染器.md

- reference 标题：Vue3渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：217 / 28
- 段落行数：83 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、entry-preview.ts、index.ts、render.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts

### 项目概述/技术架构/渲染系统架构/渲染系统架构.md

- reference 标题：渲染系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：267 / 28
- 段落行数：130 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts

### 项目概述/核心特性/主题定制系统/主题切换机制.md

- reference 标题：主题切换机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：303 / 28
- 段落行数：101 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、index.ts、create.ts、utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、index.ts、create.ts、utils.ts

### 项目概述/核心特性/主题定制系统/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：275 / 28
- 段落行数：143 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts

### 项目概述/核心特性/主题定制系统/主题扩展开发.md

- reference 标题：主题扩展开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：343 / 28
- 段落行数：119 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts

### 项目概述/核心特性/主题定制系统/字体系统.md

- reference 标题：字体系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：82
- 页面类型：overview / overview
- 行数：221 / 28
- 段落行数：69 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：constants.ts、base.ts、global.ts、dark.ts、light.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts

### 项目概述/核心特性/主题定制系统/布局系统.md

- reference 标题：布局系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：314 / 28
- 段落行数：121 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts

### 项目概述/核心特性/主题定制系统/颜色系统.md

- reference 标题：颜色系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：82
- 页面类型：overview / overview
- 行数：192 / 28
- 段落行数：52 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、colors.ts、dark.ts、light.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts

### 项目概述/核心特性/多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：302 / 28
- 段落行数：137 / 3
- Evidence：22 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts

### 项目概述/核心特性/多框架支持/Next.js框架支持.md

- reference 标题：Next.js框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：237 / 28
- 段落行数：109 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js

### 项目概述/核心特性/多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：多框架支持/react-vite.md（react-vite）
- 匹配分数：92
- 页面类型：overview / other
- 行数：296 / 32
- 段落行数：107 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts、react-docgen.ts、preset.ts
- reference 关键文件未覆盖：main.ts、package.js、preview.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、preview.ts、public-types.ts、types.ts

### 项目概述/核心特性/多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：多框架支持/sveltekit.md（sveltekit）
- 匹配分数：92
- 页面类型：overview / other
- 行数：299 / 32
- 段落行数：106 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts、preset.ts、config-overrides.ts
- reference 关键文件未覆盖：package.js、svelte-docgen.ts、types.ts、globals.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、svelte-docgen.ts、types.ts、globals.ts、public-types.ts

### 项目概述/核心特性/多框架支持/Vue框架支持.md

- reference 标题：Vue框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：234 / 28
- 段落行数：129 / 3
- Evidence：12 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts

### 项目概述/核心特性/多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：268 / 28
- 段落行数：110 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：button.js、button.stories.js、build-config.ts、main.ts、index.ts、package.js、preset.js、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、main.ts、index.ts、package.js、preset.js、preset.ts

### 项目概述/核心特性/多框架支持/其他框架支持.md

- reference 标题：其他框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：309 / 28
- 段落行数：119 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、frameworks.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js

### 项目概述/核心特性/多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：多框架支持/多框架支持.md（多框架支持）
- 匹配分数：252
- 页面类型：overview / other
- 行数：494 / 32
- 段落行数：123 / 2
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts

### 项目概述/核心特性/插件系统架构/Addon API设计.md

- reference 标题：Addon API设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：262 / 28
- 段落行数：98 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts

### 项目概述/核心特性/插件系统架构/Manager API.md

- reference 标题：Manager API
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：335 / 28
- 段落行数：124 / 3
- Evidence：17 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.js、manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts

### 项目概述/核心特性/插件系统架构/Preview API.md

- reference 标题：Preview API
- 生成页：概念指南/Preview-API.md（Preview API）
- 匹配分数：246
- 页面类型：overview / other
- 行数：267 / 9
- 段落行数：101 / 2
- Evidence：15 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js

### 项目概述/核心特性/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：322 / 28
- 段落行数：119 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.js、manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md

### 项目概述/核心特性/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：417 / 28
- 段落行数：167 / 3
- Evidence：0 / 0
- Mermaid：10 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、main.ts、types.ts、instrumenter.test.ts、addons.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、main.ts、types.ts、instrumenter.test.ts、addons.ts

### 项目概述/核心特性/插件系统架构/通信机制.md

- reference 标题：通信机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：281 / 28
- 段落行数：128 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts

### 项目概述/核心特性/文档生成功能/Doc Blocks API.md

- reference 标题：Doc Blocks API
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：389 / 28
- 段落行数：121 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts

### 项目概述/核心特性/文档生成功能/MDX文档编写.md

- reference 标题：MDX文档编写
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：269 / 28
- 段落行数：121 / 3
- Evidence：19 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md

### 项目概述/核心特性/文档生成功能/文档生成功能.md

- reference 标题：文档生成功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：341 / 28
- 段落行数：129 / 3
- Evidence：17 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts

### 项目概述/核心特性/文档生成功能/自动文档系统.md

- reference 标题：自动文档系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：267 / 28
- 段落行数：118 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts

### 项目概述/核心特性/文档生成功能/自定义文档页面.md

- reference 标题：自定义文档页面
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：260 / 28
- 段落行数：110 / 3
- Evidence：18 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md

### 项目概述/核心特性/核心特性.md

- reference 标题：核心特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：349 / 28
- 段落行数：138 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts

### 项目概述/核心特性/测试支持功能/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：373 / 28
- 段落行数：157 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js

### 项目概述/核心特性/测试支持功能/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：291 / 28
- 段落行数：114 / 3
- Evidence：23 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、main.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、main.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts

### 项目概述/核心特性/测试支持功能/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：311 / 28
- 段落行数：136 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts

### 项目概述/核心特性/测试支持功能/测试支持功能.md

- reference 标题：测试支持功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：338 / 28
- 段落行数：133 / 3
- Evidence：21 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、main.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、main.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts

### 项目概述/核心特性/测试支持功能/组件测试.md

- reference 标题：组件测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：266 / 28
- 段落行数：106 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts

### 项目概述/核心特性/组件开发环境/交互式调试.md

- reference 标题：交互式调试
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：264 / 28
- 段落行数：110 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts

### 项目概述/核心特性/组件开发环境/实时预览.md

- reference 标题：实时预览
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：311 / 28
- 段落行数：108 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts

### 项目概述/核心特性/组件开发环境/开发服务器.md

- reference 标题：开发服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：84
- 页面类型：overview / overview
- 行数：303 / 28
- 段落行数：124 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts

### 项目概述/核心特性/组件开发环境/组件开发环境.md

- reference 标题：组件开发环境
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 页面类型：overview / overview
- 行数：312 / 28
- 段落行数：139 / 3
- Evidence：21 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts

### 项目概述/核心特性/组件开发环境/组件生命周期.md

- reference 标题：组件生命周期
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：319 / 28
- 段落行数：122 / 3
- Evidence：16 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、index.ts、store.ts、hooks.ts、main.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、main.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：322
- 页面类型：overview / overview
- 行数：312 / 28
- 段落行数：131 / 3
- Evidence：21 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、main.ts、package.js、index.md、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、main.ts、package.js、index.md、tsconfig.js

### 高级功能/工具集成/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/Codemod工具.md

- reference 标题：Codemod工具
- 生成页：核心模块/codemod.md（codemod）
- 匹配分数：68
- 页面类型：other / module
- 行数：286 / 32
- 段落行数：114 / 5
- Evidence：18 / 0
- Mermaid：5 / 0
- 文件提及重合：index.test.ts、index.ts
- reference 关键文件未覆盖：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts

### 高级功能/工具集成/ESLint集成.md

- reference 标题：ESLint集成
- 生成页：插件生态/eslint-plugin.md（eslint-plugin）
- 匹配分数：116
- 页面类型：other / other
- 行数：269 / 32
- 段落行数：69 / 5
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：csf.ts、recommended.ts、index.ts、csf-component.ts
- reference 关键文件未覆盖：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts

### 高级功能/工具集成/IDE配置优化.md

- reference 标题：IDE配置优化
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/工具集成.md

- reference 标题：工具集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/构建工具优化.md

- reference 标题：构建工具优化
- 生成页：平台绑定-Web/builder-webpack5.md（builder-webpack5）
- 匹配分数：60
- 页面类型：other / other
- 行数：287 / 32
- 段落行数：93 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts

### 高级功能/性能监控.md

- reference 标题：性能监控
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/Addon开发.md

- reference 标题：Addon开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/主题开发.md

- reference 标题：主题开发
- 生成页：主题系统/themes.md（themes）
- 匹配分数：162
- 页面类型：topic / topic
- 行数：320 / 32
- 段落行数：149 / 5
- Evidence：20 / 0
- Mermaid：9 / 0
- 文件提及重合：manager.ts、preview.ts、class-name.decorator.ts、index.ts
- reference 关键文件未覆盖：main.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、colorpalette.stories.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js、create.test.js

### 高级功能/扩展开发/扩展开发.md

- reference 标题：扩展开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/构建器扩展.md

- reference 标题：构建器扩展
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/渲染器扩展.md

- reference 标题：渲染器扩展
- 生成页：核心模块/html.md（html）
- 匹配分数：126
- 页面类型：other / module
- 行数：333 / 32
- 段落行数：127 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：render.ts、index.ts、portable-stories.ts、preset.ts、entry-preview.ts
- reference 关键文件未覆盖：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts

### 高级功能/扩展开发/预设开发.md

- reference 标题：预设开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/自定义渲染器.md

- reference 标题：自定义渲染器
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/预设配置.md

- reference 标题：预设配置
- 生成页：多框架支持/create-react-app.md（create-react-app）
- 匹配分数：64
- 页面类型：other / other
- 行数：288 / 29
- 段落行数：110 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：preset.js、index.ts
- reference 关键文件未覆盖：presets.ts、package.js、types.ts、index.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：无
- 问题：缺少对应生成页面

