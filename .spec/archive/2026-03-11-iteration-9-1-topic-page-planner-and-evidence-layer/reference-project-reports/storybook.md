# storybook Reference 对比报告

生成页面：122 页
reference 页面：176 页
命中对比：129 页
缺失对比：47 页

## 覆盖统计

- 专题页覆盖：generated 55 / reference 8
- evidence 落页：generated 120 / reference 176
- 图表达覆盖：generated 109 / reference 176
- 已规划专题类型：专题页(47)、流程主题(5)、路由主题(2)、核心机制主题(1)
- 高频缺失专题：无

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/code.md 被 12 个 reference 页面共享映射
- 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md 被 7 个 reference 页面共享映射
- 核心模块/code/frameworks/svelte-vite.md 被 3 个 reference 页面共享映射
- 核心模块/code/builders/builder-webpack5.md 被 5 个 reference 页面共享映射
- 系统架构.md 被 14 个 reference 页面共享映射
- 核心模块/code/addons/themes.md 被 2 个 reference 页面共享映射
- 项目概述.md 被 68 个 reference 页面共享映射
- 工作流与部署.md 被 4 个 reference 页面共享映射
- 核心模块/code/renderers/html.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-024ce3ea962b-page-react-native-web-vite能力：界面组成.md (react-native-web-vite能力：界面组成, 44 行)
- 专题/module-capability/module-0f2ff1753d5a-page-svelte-vite能力：界面组成.md (svelte-vite能力：界面组成, 84 行)
- 专题/module-capability/module-0f96ce00c9a2-page-react-webpack5能力：界面组成.md (react-webpack5能力：界面组成, 50 行)
- 专题/module-capability/module-221ebe304f45-router-nextjs能力：路由.md (nextjs能力：路由, 57 行)
- 专题/module-capability/module-2373d5792d4d-component-preact能力：界面组成.md (preact能力：界面组成, 45 行)
- 专题/module-capability/module-3006222a09ec-entry-angular能力：入口编排.md (angular能力：入口编排, 65 行)
- 专题/module-capability/module-3c338a256b00-component-onboarding能力：界面组成.md (onboarding能力：界面组成, 50 行)
- 专题/module-capability/module-40617804ce25-repository-scripts能力：数据访问.md (scripts能力：数据访问, 192 行)
- 专题/module-capability/module-41906d72e7a0-component-svelte能力：界面组成.md (svelte能力：界面组成, 56 行)
- 专题/module-capability/module-52b3dc54672a-component-renderers能力：界面组成.md (renderers能力：界面组成, 28 行)
- 专题/module-capability/module-5b6454edd8d9-component-web-components能力：界面组成.md (web-components能力：界面组成, 50 行)
- 专题/module-capability/module-5ed2352a9657-plugin-docs能力：扩展机制.md (docs能力：扩展机制, 24 行)
- 专题/module-capability/module-5f60b9426ab9-page-server-kitchen-sink能力：界面组成.md (server-kitchen-sink能力：界面组成, 44 行)
- 专题/module-capability/module-61698a049849-component-html能力：界面组成.md (html能力：界面组成, 45 行)
- 专题/module-capability/module-63cd1c3f569d-plugin-vue3-vite能力：扩展机制.md (vue3-vite能力：扩展机制, 55 行)
- 专题/module-capability/module-726bb07a2a71-entry-yarn-pnp能力：入口编排.md (yarn-pnp能力：入口编排, 47 行)
- 专题/module-capability/module-75fea8f5ed89-component-ember-cli能力：界面组成.md (ember-cli能力：界面组成, 44 行)
- 专题/module-capability/module-7744c084cfdc-library-create-storybook能力：扩展机制.md (create-storybook能力：扩展机制, 60 行)
- 专题/module-capability/module-794a59a98110-component-links能力：界面组成.md (links能力：界面组成, 49 行)
- 专题/module-capability/module-7ebbec0086be-page-html-vite能力：界面组成.md (html-vite能力：界面组成, 42 行)
- 专题/module-capability/module-802bfc6aff6d-page-frameworks能力：界面组成.md (frameworks能力：界面组成, 28 行)
- 专题/module-capability/module-8182c18ba204-component-a11y能力：界面组成.md (a11y能力：界面组成, 73 行)
- 专题/module-capability/module-831cabed9c6e-plugin-react-vite能力：扩展机制.md (react-vite能力：扩展机制, 65 行)
- 专题/module-capability/module-8992db69b585-component-docs能力：界面组成.md (docs能力：界面组成, 78 行)
- 专题/module-capability/module-8d6dc973c6b0-component-addons能力：界面组成.md (addons能力：界面组成, 28 行)
- 专题/module-capability/module-8da0ecf9a6af-page-nextjs能力：界面组成.md (nextjs能力：界面组成, 48 行)
- 专题/module-capability/module-99d8c90f46b4-plugin-builders能力：扩展机制.md (builders能力：扩展机制, 28 行)
- 专题/module-capability/module-9afde067d120-library-react-dom-shim能力：扩展机制.md (react-dom-shim能力：扩展机制, 48 行)
- 专题/module-capability/module-9f842b811b09-page-web-components-vite能力：界面组成.md (web-components-vite能力：界面组成, 43 行)
- 专题/module-capability/module-b340351f93b1-library-eslint-plugin能力：扩展机制.md (eslint-plugin能力：扩展机制, 59 行)
- 专题/module-capability/module-b35d061ec501-component-core能力：界面组成.md (core能力：界面组成, 211 行)
- 专题/module-capability/module-b67616f5f23b-component-code能力：界面组成.md (code能力：界面组成, 173 行)
- 专题/module-capability/module-b6d1a1d8c5a8-component-vue3能力：界面组成.md (vue3能力：界面组成, 57 行)
- 专题/module-capability/module-c28fd0809d91-page-test-storybooks能力：界面组成.md (test-storybooks能力：界面组成, 28 行)
- 专题/module-capability/module-c673c600225a-library-react能力：扩展机制.md (react能力：扩展机制, 86 行)
- 专题/module-capability/module-ca2e0a6d81e4-library-cli-sb能力：扩展机制.md (cli-sb能力：扩展机制, 44 行)
- 专题/module-capability/module-d00597c4403b-component-vitest能力：界面组成.md (vitest能力：界面组成, 68 行)
- 专题/module-capability/module-d04cc4beb05d-library-presets能力：扩展机制.md (presets能力：扩展机制, 28 行)
- 专题/module-capability/module-da1388cbf763-plugin-builder-webpack5能力：扩展机制.md (builder-webpack5能力：扩展机制, 104 行)
- 专题/module-capability/module-e1a57e3163b0-library-lib能力：扩展机制.md (lib能力：扩展机制, 28 行)
- 专题/module-capability/module-e24dd2bf80a2-plugin-sveltekit能力：扩展机制.md (sveltekit能力：扩展机制, 50 行)
- 专题/module-capability/module-e87d89923754-component-external-docs能力：界面组成.md (external-docs能力：界面组成, 45 行)
- 专题/module-capability/module-eae31257ec3a-entry-portable-stories-kitchen-sink能力：入口编排.md (portable-stories-kitchen-sink能力：入口编排, 36 行)
- 专题/module-capability/module-eb98f4d9d066-library-core-webpack能力：扩展机制.md (core-webpack能力：扩展机制, 57 行)
- 专题/module-capability/module-f0dd7c8445c0-router-nextjs-vite能力：路由.md (nextjs-vite能力：路由, 55 行)
- 专题/module-capability/module-f16579f4c153-library-cli-storybook能力：扩展机制.md (cli-storybook能力：扩展机制, 75 行)
- 专题/process/process-handleaddvalueadd-flow-流程主题：handleAddValueAdd-flow.md (流程主题：handleAddValueAdd flow, 41 行)
- 专题/process/process-handlechange-flow-流程主题：handleChange-flow.md (流程主题：handleChange flow, 31 行)
- 专题/process/process-handlefilechange-flow-流程主题：handleFileChange-flow.md (流程主题：handleFileChange flow, 31 行)
- 专题/process/process-handlesnippetrendered-flow-流程主题：handleSnippetRendered-flow.md (流程主题：handleSnippetRendered flow, 31 行)
- 专题/process/process-handletriggerrunevent-flow-流程主题：handleTriggerRunEvent-flow.md (流程主题：handleTriggerRunEvent flow, 79 行)
- 核心模块/code/addons/a11y.md (模块：a11y, 115 行)
- 核心模块/code/addons/docs.md (模块：docs, 115 行)
- 核心模块/code/addons/onboarding.md (模块：onboarding, 90 行)
- 核心模块/code/addons/pseudo-states.md (模块：pseudo-states, 92 行)
- 核心模块/code/addons/vitest.md (模块：vitest, 117 行)
- 核心模块/code/builders.md (模块：builders, 99 行)
- 核心模块/code/core.md (模块：core, 309 行)
- 核心模块/code/frameworks.md (模块：frameworks, 125 行)
- 核心模块/code/frameworks/angular.md (模块：angular, 98 行)
- 核心模块/code/frameworks/ember.md (模块：ember, 92 行)
- 核心模块/code/frameworks/html-vite.md (模块：html-vite, 83 行)
- 核心模块/code/frameworks/nextjs-vite.md (模块：nextjs-vite, 99 行)
- 核心模块/code/frameworks/nextjs.md (模块：nextjs, 101 行)
- 核心模块/code/frameworks/preact-vite.md (模块：preact-vite, 77 行)
- 核心模块/code/frameworks/react-native-web-vite.md (模块：react-native-web-vite, 80 行)
- 核心模块/code/frameworks/react-webpack5.md (模块：react-webpack5, 88 行)
- 核心模块/code/frameworks/server-webpack5.md (模块：server-webpack5, 82 行)
- 核心模块/code/frameworks/sveltekit.md (模块：sveltekit, 94 行)
- 核心模块/code/frameworks/web-components-vite.md (模块：web-components-vite, 80 行)
- 核心模块/code/lib/cli-sb.md (模块：cli-sb, 50 行)
- 核心模块/code/lib/cli-storybook.md (模块：cli-storybook, 112 行)
- 核心模块/code/lib/codemod.md (模块：codemod, 96 行)
- 核心模块/code/lib/core-webpack.md (模块：core-webpack, 102 行)
- 核心模块/code/lib/create-storybook.md (模块：create-storybook, 106 行)
- 核心模块/code/lib/csf-plugin.md (模块：csf-plugin, 83 行)
- 核心模块/code/lib/react-dom-shim.md (模块：react-dom-shim, 81 行)
- 核心模块/code/presets.md (模块：presets, 72 行)
- 核心模块/code/presets/react-webpack.md (模块：react-webpack, 88 行)
- 核心模块/code/presets/server-webpack.md (模块：server-webpack, 92 行)
- 核心模块/code/renderers.md (模块：renderers, 104 行)
- 核心模块/code/renderers/preact.md (模块：preact, 80 行)
- 核心模块/code/renderers/react.md (模块：react, 143 行)
- 核心模块/code/renderers/server.md (模块：server, 83 行)
- 核心模块/code/renderers/svelte.md (模块：svelte, 102 行)
- 核心模块/code/renderers/vue3.md (模块：vue3, 103 行)
- 核心模块/docs.md (模块：docs, 49 行)
- 核心模块/scripts.md (模块：scripts, 323 行)
- 核心模块/scripts/eslint-plugin-local-rules.md (模块：eslint-plugin-local-rules, 56 行)
- 核心模块/test-storybooks.md (模块：test-storybooks, 87 行)
- 核心模块/test-storybooks/ember-cli.md (模块：ember-cli, 69 行)
- 核心模块/test-storybooks/external-docs.md (模块：external-docs, 76 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink.md (模块：portable-stories-kitchen-sink, 86 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/nextjs.md (模块：nextjs, 81 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/react-vitest-3.md (模块：react-vitest-3, 99 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/react.md (模块：react, 99 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/svelte.md (模块：svelte, 72 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/vue3.md (模块：vue3, 73 行)
- 核心模块/test-storybooks/server-kitchen-sink.md (模块：server-kitchen-sink, 69 行)
- 核心模块/test-storybooks/standalone-preview.md (模块：standalone-preview, 64 行)
- 核心模块/test-storybooks/yarn-pnp.md (模块：yarn-pnp, 81 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API参考/API参考.md | 核心模块/code.md | 368/240 | 101/168 | 0/1 | 4/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts |
| API参考/CLI命令参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Addon API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/CSF API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Decorators API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Hooks API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Preview API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Store API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Types API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/开发API参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/API类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/工具类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/插件类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/构建器类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/核心类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/框架类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/类型定义参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/main.js配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/manager.js配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/preview.js配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/构建器配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/配置API参考.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/预设配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 主题和外观/主题和外观.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 359/59 | 132/33 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts |
| 主题和外观/主题系统概览.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 302/59 | 113/33 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js |
| 主题和外观/布局和间距设计.md | 专题/module-capability/module-7526a957f0fc-library-server-webpack能力：扩展机制.md | 185/52 | 64/28 | 0/1 | 3/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js |
| 主题和外观/自定义主题开发.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 372/59 | 123/33 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts |
| 主题和外观/颜色和字体系统.md | 核心模块/code/addons.md | 244/95 | 90/23 | 13/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts |
| 多框架支持/Angular框架支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 多框架支持/HTML框架支持.md | 核心模块/code.md | 215/240 | 89/168 | 0/1 | 4/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build-config.ts、package.js、sandbox-templates.ts、render.ts |
| 多框架支持/React框架支持.md | 核心模块/code/frameworks/react-vite.md | 349/112 | 225/44 | 16/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js |
| 多框架支持/Svelte框架支持.md | 核心模块/code/frameworks/svelte-vite.md | 355/148 | 233/65 | 15/1 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts |
| 多框架支持/Vue 3框架支持.md | 核心模块/code/frameworks/vue3-vite.md | 279/99 | 115/32 | 15/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts |
| 多框架支持/Web Components框架支持.md | 核心模块/code/renderers/web-components.md | 300/92 | 223/27 | 13/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts |
| 多框架支持/多框架支持.md | 核心模块/code.md | 259/240 | 87/168 | 0/1 | 3/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js |
| 快速开始.md | 核心模块/code.md | 264/240 | 120/168 | 0/1 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js |
| 插件系统/Addons概览.md | 核心模块/code.md | 276/240 | 97/168 | 16/1 | 5/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts |
| 插件系统/插件系统.md | 核心模块/code.md | 356/240 | 107/168 | 0/1 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js |
| 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Actions Addon（动作记录）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Controls Addon（参数控制）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Docs Addon（文档生成）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Links Addon（故事导航）.md | 核心模块/code/addons/links.md | 362/96 | 125/30 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts |
| 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Themes Addon（主题切换）.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 279/59 | 120/33 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md |
| 插件系统/核心Addons详解/核心Addons详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 插件系统/第三方Addons集成.md | 核心模块/code.md | 246/240 | 98/168 | 20/1 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md |
| 插件系统/自定义Addons开发.md | 核心模块/code.md | 304/240 | 108/168 | 0/1 | 5/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts |
| 故障排除/兼容性问题.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/安装问题.md | 核心模块/code/lib.md | 350/97 | 106/25 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts |
| 故障排除/故障排除.md | 核心模块/code.md | 348/240 | 106/168 | 0/1 | 6/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts、options.ts |
| 故障排除/构建和性能问题.md | 核心模块/code/builders/builder-webpack5.md | 312/174 | 98/85 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts |
| 故障排除/调试工具和技巧.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/运行时错误.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除/配置错误.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 构建系统/Vite构建器详解.md | 核心模块/code/builders/builder-vite.md | 300/125 | 120/49 | 0/1 | 9/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts |
| 构建系统/Webpack构建器详解.md | 核心模块/code/builders/builder-webpack5.md | 339/174 | 138/85 | 17/1 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts、merge-webpack-config.ts、my-component-story-import-static-asset.md、next.js |
| 构建系统/构建器概览.md | 核心模块/code/builders/builder-webpack5.md | 321/174 | 113/85 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf |
| 构建系统/构建系统.md | 核心模块/code/builders/builder-webpack5.md | 291/174 | 103/85 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts |
| 构建系统/构建配置优化.md | 核心模块/code.md | 264/240 | 70/168 | 0/1 | 4/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js |
| 核心概念/Addons系统架构/Addons开发指南.md | 系统架构.md | 584/869 | 390/298 | 23/1 | 13/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts、types.ts |
| 核心概念/Addons系统架构/Addons架构设计.md | 系统架构.md | 321/869 | 146/298 | 0/1 | 8/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts |
| 核心概念/Addons系统架构/Addons系统架构.md | 系统架构.md | 356/869 | 156/298 | 22/1 | 7/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md |
| 核心概念/Addons系统架构/Addons通信机制.md | 系统架构.md | 290/869 | 131/298 | 0/1 | 7/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts |
| 核心概念/Addons系统架构/Addons配置管理.md | 系统架构.md | 273/869 | 85/298 | 0/1 | 4/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.ts、presets.ts |
| 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md | 系统架构.md | 362/869 | 125/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts |
| 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md | 系统架构.md | 302/869 | 103/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts |
| 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md | 系统架构.md | 374/869 | 115/298 | 0/1 | 5/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts |
| 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md | 系统架构.md | 282/869 | 95/298 | 0/1 | 4/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts |
| 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md | 系统架构.md | 348/869 | 127/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts |
| 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md | 核心模块/code/addons/themes.md | 293/84 | 127/22 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、types.ts、themes.md |
| 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md | 系统架构.md | 290/869 | 128/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts |
| 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md | 系统架构.md | 257/869 | 98/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts |
| 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md | 系统架构.md | 309/869 | 113/298 | 7/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts |
| 核心概念/CSF格式规范.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/主题和参数系统.md | 专题/module-capability/module-7351b0cc5a9b-library-csf-plugin能力：扩展机制.md | 356/50 | 126/26 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts、convert.ts |
| 核心概念/核心概念.md | 核心模块/code.md | 388/240 | 139/168 | 16/1 | 7/2 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、api.md、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md |
| 核心概念/组件故事（Stories）.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/装饰器和全局状态.md | 核心模块/code/addons/themes.md | 307/84 | 119/22 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、api.md、helpers.ts、hooks.ts、decorator.ts、next.js |
| 核心概念/预览和管理界面.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/Playwright测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/Vitest集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/可访问性测试.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 300/59 | 109/33 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、manager.ts、params.ts、preview.ts、types.ts |
| 测试框架/测试框架.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/组件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试框架/视觉回归测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 项目概述.md | 228/120 | 77/44 | 0/1 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、contributing.md、security.md、package.js、playwright.config.ts |
| 部署和CI_CD/CI_CD集成.md | 工作流与部署.md | 317/324 | 96/3 | 0/0 | 5/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts |
| 部署和CI_CD/Chromatic集成.md | 工作流与部署.md | 242/324 | 123/3 | 18/0 | 7/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js |
| 部署和CI_CD/环境配置.md | 项目概述.md | 299/120 | 97/44 | 17/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、package.js |
| 部署和CI_CD/部署和CI_CD.md | 工作流与部署.md | 264/324 | 105/3 | 0/0 | 6/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js |
| 部署和CI_CD/静态部署.md | 工作流与部署.md | 201/324 | 63/3 | 0/0 | 4/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts |
| 项目概述/使用场景和案例.md | 项目概述.md | 271/120 | 100/44 | 0/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、package.js、index.md |
| 项目概述/基本概念.md | 项目概述.md | 280/120 | 80/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md |
| 项目概述/技术架构/Monorepo设计.md | 项目概述.md | 249/120 | 78/44 | 0/1 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts |
| 项目概述/技术架构/技术架构.md | 项目概述.md | 289/120 | 141/44 | 0/1 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts |
| 项目概述/技术架构/插件系统架构/插件架构设计.md | 项目概述.md | 346/120 | 158/44 | 17/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md |
| 项目概述/技术架构/插件系统架构/插件注册机制.md | 项目概述.md | 263/120 | 100/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md |
| 项目概述/技术架构/插件系统架构/插件生命周期管理.md | 项目概述.md | 266/120 | 94/44 | 0/1 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts |
| 项目概述/技术架构/插件系统架构/插件系统架构.md | 项目概述.md | 315/120 | 127/44 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md |
| 项目概述/技术架构/插件系统架构/插件通信协议.md | 项目概述.md | 312/120 | 136/44 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、types.ts |
| 项目概述/技术架构/构建系统架构/Vite构建器.md | 项目概述.md | 283/120 | 127/44 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts |
| 项目概述/技术架构/构建系统架构/Webpack构建器.md | 项目概述.md | 242/120 | 84/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js |
| 项目概述/技术架构/构建系统架构/构建器架构设计.md | 项目概述.md | 363/120 | 146/44 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts |
| 项目概述/技术架构/构建系统架构/构建系统架构.md | 专题/module-capability/module-f183267d666d-plugin-builder-vite能力：扩展机制.md | 244/81 | 105/54 | 0/1 | 5/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、types.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts |
| 项目概述/技术架构/核心引擎架构/CLI分发器.md | 项目概述.md | 308/120 | 152/44 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts |
| 项目概述/技术架构/核心引擎架构/全局设置.md | 项目概述.md | 360/120 | 143/44 | 18/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts |
| 项目概述/技术架构/核心引擎架构/核心引擎架构.md | 项目概述.md | 272/120 | 122/44 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js |
| 项目概述/技术架构/核心引擎架构/核心服务器.md | 项目概述.md | 334/120 | 131/44 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts |
| 项目概述/技术架构/核心引擎架构/版本管理.md | 项目概述.md | 359/120 | 137/44 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts |
| 项目概述/技术架构/框架适配器架构/Angular框架适配器.md | 项目概述.md | 355/120 | 245/44 | 17/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts |
| 项目概述/技术架构/框架适配器架构/HTML框架适配器.md | 项目概述.md | 245/120 | 99/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js |
| 项目概述/技术架构/框架适配器架构/React框架适配器.md | 项目概述.md | 326/120 | 239/44 | 17/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js |
| 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md | 核心模块/code/frameworks/svelte-vite.md | 287/148 | 131/65 | 0/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts |
| 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md | 项目概述.md | 305/120 | 212/44 | 15/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts |
| 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md | 项目概述.md | 478/120 | 315/44 | 17/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts |
| 项目概述/技术架构/框架适配器架构/框架适配器架构.md | 项目概述.md | 297/120 | 136/44 | 0/1 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js |
| 项目概述/技术架构/渲染系统架构/HTML渲染器.md | 核心模块/code/renderers/html.md | 250/85 | 108/22 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts |
| 项目概述/技术架构/渲染系统架构/React渲染器.md | 项目概述.md | 238/120 | 91/44 | 12/1 | 4/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts |
| 项目概述/技术架构/渲染系统架构/Svelte渲染器.md | 项目概述.md | 319/120 | 149/44 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts |
| 项目概述/技术架构/渲染系统架构/Vue3渲染器.md | 项目概述.md | 217/120 | 83/44 | 0/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts |
| 项目概述/技术架构/渲染系统架构/渲染系统架构.md | 项目概述.md | 267/120 | 130/44 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts |
| 项目概述/核心特性/主题定制系统/主题切换机制.md | 项目概述.md | 303/120 | 101/44 | 0/1 | 6/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、create.ts、utils.ts |
| 项目概述/核心特性/主题定制系统/主题定制系统.md | 项目概述.md | 275/120 | 143/44 | 0/1 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts |
| 项目概述/核心特性/主题定制系统/主题扩展开发.md | 项目概述.md | 343/120 | 119/44 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts |
| 项目概述/核心特性/主题定制系统/字体系统.md | 项目概述.md | 221/120 | 69/44 | 0/1 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts |
| 项目概述/核心特性/主题定制系统/布局系统.md | 项目概述.md | 314/120 | 121/44 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts |
| 项目概述/核心特性/主题定制系统/颜色系统.md | 项目概述.md | 192/120 | 52/44 | 0/1 | 3/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts |
| 项目概述/核心特性/多框架支持/Angular框架支持.md | 项目概述.md | 302/120 | 137/44 | 22/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts |
| 项目概述/核心特性/多框架支持/Next.js框架支持.md | 项目概述.md | 237/120 | 109/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js |
| 项目概述/核心特性/多框架支持/React框架支持.md | 项目概述.md | 296/120 | 107/44 | 0/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、preview.ts、public-types.ts、types.ts |
| 项目概述/核心特性/多框架支持/Svelte框架支持.md | 核心模块/code/frameworks/svelte-vite.md | 299/148 | 106/65 | 0/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts |
| 项目概述/核心特性/多框架支持/Vue框架支持.md | 项目概述.md | 234/120 | 129/44 | 12/1 | 6/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts |
| 项目概述/核心特性/多框架支持/Web Components框架支持.md | 项目概述.md | 268/120 | 110/44 | 0/1 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts |
| 项目概述/核心特性/多框架支持/其他框架支持.md | 项目概述.md | 309/120 | 119/44 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js |
| 项目概述/核心特性/多框架支持/多框架支持.md | 项目概述.md | 494/120 | 123/44 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts |
| 项目概述/核心特性/插件系统架构/Addon API设计.md | 项目概述.md | 262/120 | 98/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts |
| 项目概述/核心特性/插件系统架构/Manager API.md | 项目概述.md | 335/120 | 124/44 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md |
| 项目概述/核心特性/插件系统架构/Preview API.md | 项目概述.md | 267/120 | 101/44 | 15/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js |
| 项目概述/核心特性/插件系统架构/插件生命周期管理.md | 项目概述.md | 322/120 | 119/44 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md |
| 项目概述/核心特性/插件系统架构/插件系统架构.md | 项目概述.md | 417/120 | 167/44 | 0/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts |
| 项目概述/核心特性/插件系统架构/通信机制.md | 项目概述.md | 281/120 | 128/44 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts |
| 项目概述/核心特性/文档生成功能/Doc Blocks API.md | 项目概述.md | 389/120 | 121/44 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts |
| 项目概述/核心特性/文档生成功能/MDX文档编写.md | 项目概述.md | 269/120 | 121/44 | 19/1 | 8/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md |
| 项目概述/核心特性/文档生成功能/文档生成功能.md | 项目概述.md | 341/120 | 129/44 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md |
| 项目概述/核心特性/文档生成功能/自动文档系统.md | 项目概述.md | 267/120 | 118/44 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts |
| 项目概述/核心特性/文档生成功能/自定义文档页面.md | 项目概述.md | 260/120 | 110/44 | 18/1 | 6/1 | 章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md |
| 项目概述/核心特性/核心特性.md | 项目概述.md | 349/120 | 138/44 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts |
| 项目概述/核心特性/测试支持功能/Playwright测试.md | 项目概述.md | 373/120 | 157/44 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js |
| 项目概述/核心特性/测试支持功能/Vitest集成.md | 项目概述.md | 291/120 | 114/44 | 23/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts |
| 项目概述/核心特性/测试支持功能/可访问性测试.md | 项目概述.md | 311/120 | 136/44 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts |
| 项目概述/核心特性/测试支持功能/测试支持功能.md | 项目概述.md | 338/120 | 133/44 | 21/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts |
| 项目概述/核心特性/测试支持功能/组件测试.md | 项目概述.md | 266/120 | 106/44 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts |
| 项目概述/核心特性/组件开发环境/交互式调试.md | 项目概述.md | 264/120 | 110/44 | 0/1 | 7/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts |
| 项目概述/核心特性/组件开发环境/实时预览.md | 项目概述.md | 311/120 | 108/44 | 0/1 | 5/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts |
| 项目概述/核心特性/组件开发环境/开发服务器.md | 项目概述.md | 303/120 | 124/44 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts |
| 项目概述/核心特性/组件开发环境/组件开发环境.md | 项目概述.md | 312/120 | 139/44 | 21/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts |
| 项目概述/核心特性/组件开发环境/组件生命周期.md | 项目概述.md | 319/120 | 122/44 | 16/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts |
| 项目概述/项目概述.md | 项目概述.md | 312/120 | 131/44 | 21/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、index.md、tsconfig.js |
| 高级功能/工具集成/CI_CD集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/工具集成/Codemod工具.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 286/59 | 114/33 | 18/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts |
| 高级功能/工具集成/ESLint集成.md | 核心模块/code/lib/eslint-plugin.md | 269/105 | 69/36 | 0/1 | 4/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts |
| 高级功能/工具集成/IDE配置优化.md | 项目概述.md | 339/120 | 105/44 | 15/1 | 4/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js |
| 高级功能/工具集成/工具集成.md | 项目概述.md | 313/120 | 105/44 | 0/1 | 6/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts |
| 高级功能/工具集成/构建工具优化.md | 核心模块/code/builders/builder-webpack5.md | 287/174 | 93/85 | 0/1 | 5/1 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts |
| 高级功能/性能监控.md | 核心模块/code.md | 340/240 | 165/168 | 0/1 | 8/2 | 章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts |
| 高级功能/扩展开发/Addon开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/主题开发.md | 专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md | 320/59 | 149/33 | 20/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts |
| 高级功能/扩展开发/扩展开发.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/构建器扩展.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/渲染器扩展.md | 核心模块/code/renderers/html.md | 333/85 | 127/22 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts |
| 高级功能/扩展开发/预设开发.md | 系统架构.md | 292/869 | 123/298 | 0/1 | 6/2 | 内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js |
| 高级功能/自定义渲染器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/预设配置.md | 核心模块/code/presets/create-react-app.md | 288/74 | 110/22 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js |
| 高级功能/高级功能.md | 项目概述.md | 375/120 | 139/44 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、package.js、preset.ts、build.ts、index.ts、postinstalladdon.ts |

## 逐文件详情

### API参考/API参考.md

- reference 标题：API参考
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：96
- 页面类型：other / module
- 行数：368 / 240
- 段落行数：101 / 168
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：main.ts、manager.ts、index.ts、main.js、manager.js
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts

### API参考/CLI命令参考.md

- reference 标题：CLI命令参考
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Addon API.md

- reference 标题：插件API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/CSF API.md

- reference 标题：CSF API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Decorators API.md

- reference 标题：装饰器API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Hooks API.md

- reference 标题：Hooks API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Preview API.md

- reference 标题：Preview API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Store API.md

- reference 标题：Store API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/Types API.md

- reference 标题：Types API
- 生成页：无
- 问题：缺少对应生成页面

### API参考/开发API参考/开发API参考.md

- reference 标题：开发API参考
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/API类型定义.md

- reference 标题：API类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/工具类型定义.md

- reference 标题：工具类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/插件类型定义.md

- reference 标题：插件类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/构建器类型定义.md

- reference 标题：构建器类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/核心类型定义.md

- reference 标题：核心类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/框架类型定义.md

- reference 标题：框架类型定义
- 生成页：无
- 问题：缺少对应生成页面

### API参考/类型定义参考/类型定义参考.md

- reference 标题：类型定义参考
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/main.js配置.md

- reference 标题：main.js配置
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/manager.js配置.md

- reference 标题：manager.js配置
- 生成页：无
- 问题：缺少对应生成页面

### API参考/配置API参考/preview.js配置.md

- reference 标题：preview.js配置
- 生成页：无
- 问题：缺少对应生成页面

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
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：124
- 页面类型：topic / topic
- 行数：359 / 59
- 段落行数：132 / 33
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：utils.ts、index.ts
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts

### 主题和外观/主题系统概览.md

- reference 标题：主题系统概览
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：120
- 页面类型：topic / topic
- 行数：302 / 59
- 段落行数：113 / 33
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：utils.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js

### 主题和外观/布局和间距设计.md

- reference 标题：布局和间距设计
- 生成页：专题/module-capability/module-7526a957f0fc-library-server-webpack能力：扩展机制.md（server-webpack能力：扩展机制）
- 匹配分数：102
- 页面类型：topic / topic
- 行数：185 / 52
- 段落行数：64 / 28
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：index.ts、types.ts
- reference 关键文件未覆盖：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、fs-extra.ts、fs.ts、promises.ts、htmlmock.js、lodash-es.js、stylemock.js、uuid.js

### 主题和外观/自定义主题开发.md

- reference 标题：自定义主题开发
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：148
- 页面类型：topic / topic
- 行数：372 / 59
- 段落行数：123 / 33
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、utils.ts
- reference 关键文件未覆盖：package.js、constants.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、preview.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts

### 主题和外观/颜色和字体系统.md

- reference 标题：颜色和字体系统
- 生成页：核心模块/code/addons.md（模块：addons）
- 匹配分数：122
- 页面类型：topic / module
- 行数：244 / 95
- 段落行数：90 / 23
- Evidence：13 / 1
- Mermaid：5 / 1
- 文件提及重合：manager.js、preset.js、preview.js、index.ts
- reference 关键文件未覆盖：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts

### 多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：无
- 问题：缺少对应生成页面

### 多框架支持/HTML框架支持.md

- reference 标题：HTML框架支持
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：60
- 页面类型：other / module
- 行数：215 / 240
- 段落行数：89 / 168
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：build-config.ts、package.js、sandbox-templates.ts、render.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build-config.ts、package.js、sandbox-templates.ts、render.ts

### 多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：核心模块/code/frameworks/react-vite.md（模块：react-vite）
- 匹配分数：70
- 页面类型：other / module
- 行数：349 / 112
- 段落行数：225 / 44
- Evidence：16 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、preset.ts
- reference 关键文件未覆盖：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js

### 多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：102
- 页面类型：other / module
- 行数：355 / 148
- 段落行数：233 / 65
- Evidence：15 / 1
- Mermaid：8 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts

### 多框架支持/Vue 3框架支持.md

- reference 标题：Vue 3框架支持
- 生成页：核心模块/code/frameworks/vue3-vite.md（模块：vue3-vite）
- 匹配分数：100
- 页面类型：other / module
- 行数：279 / 99
- 段落行数：115 / 32
- Evidence：15 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、preset.ts、vite-plugin.ts
- reference 关键文件未覆盖：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts

### 多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：核心模块/code/renderers/web-components.md（模块：web-components）
- 匹配分数：144
- 页面类型：module / module
- 行数：300 / 92
- 段落行数：223 / 27
- Evidence：13 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts

### 多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：70
- 页面类型：other / module
- 行数：259 / 240
- 段落行数：87 / 168
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：main.ts、manager.ts、index.ts
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：66
- 页面类型：other / module
- 行数：264 / 240
- 段落行数：120 / 168
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js

### 插件系统/Addons概览.md

- reference 标题：Addons概览
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：86
- 页面类型：other / module
- 行数：276 / 240
- 段落行数：97 / 168
- Evidence：16 / 1
- Mermaid：5 / 2
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、preview.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts

### 插件系统/插件系统.md

- reference 标题：插件系统
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：94
- 页面类型：other / module
- 行数：356 / 240
- 段落行数：107 / 168
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts、manager.ts、index.ts
- reference 关键文件未覆盖：preview.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js

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
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：94
- 页面类型：other / module
- 行数：362 / 96
- 段落行数：125 / 30
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：index.ts、manager.ts、preview.ts、link.test.ts、link.ts、utils.test.ts、utils.ts
- reference 关键文件未覆盖：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts

### 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md

- reference 标题：Outline/Measure/Toolbar Addons（辅助工具）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Themes Addon（主题切换）.md

- reference 标题：Themes Addon（主题切换）
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：176
- 页面类型：topic / topic
- 行数：279 / 59
- 段落行数：120 / 33
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：build-config.ts、project.js、index.ts
- reference 关键文件未覆盖：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md、tailwind.md

### 插件系统/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/第三方Addons集成.md

- reference 标题：第三方Addons集成
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：76
- 页面类型：other / module
- 行数：246 / 240
- 段落行数：98 / 168
- Evidence：20 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、addon-types.md、index.md、writing-addons.md
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md

### 插件系统/自定义Addons开发.md

- reference 标题：自定义Addons开发
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：80
- 页面类型：other / module
- 行数：304 / 240
- 段落行数：108 / 168
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：main.ts、manager.ts、index.ts
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts

### 故障排除/兼容性问题.md

- reference 标题：兼容性问题
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/安装问题.md

- reference 标题：安装问题
- 生成页：核心模块/code/lib.md（模块：lib）
- 匹配分数：82
- 页面类型：other / module
- 行数：350 / 97
- 段落行数：106 / 25
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：finalizationcommand.ts、preflightcheckcommand.ts、index.js
- reference 关键文件未覆盖：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：60
- 页面类型：other / module
- 行数：348 / 240
- 段落行数：106 / 168
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts、options.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、environments-support.ts、package.js、playwright.config.ts、tsconfig.js、vitest.config.ts、task.ts、options.ts

### 故障排除/构建和性能问题.md

- reference 标题：构建和性能问题
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：78
- 页面类型：other / module
- 行数：312 / 174
- 段落行数：98 / 85
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、base-webpack.config.ts
- reference 关键文件未覆盖：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts

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
- 生成页：核心模块/code/builders/builder-vite.md（模块：builder-vite）
- 匹配分数：106
- 页面类型：other / module
- 行数：300 / 125
- 段落行数：120 / 49
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：index.ts、code-generator-plugin.ts、csf-plugin.ts
- reference 关键文件未覆盖：package.js、build.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts

### 构建系统/Webpack构建器详解.md

- reference 标题：Webpack构建器详解
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：62
- 页面类型：other / module
- 行数：339 / 174
- 段落行数：138 / 85
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、custom-webpack-preset.ts、preview-preset.ts、iframe-webpack.config.ts
- reference 关键文件未覆盖：package.js、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts、merge-webpack-config.ts、my-component-story-import-static-asset.md、next.js
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、webpack.ts、webpack-loader.ts、load-custom-webpack-config.ts、merge-webpack-config.ts、my-component-story-import-static-asset.md、next.js

### 构建系统/构建器概览.md

- reference 标题：构建器概览
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：68
- 页面类型：other / module
- 行数：321 / 174
- 段落行数：113 / 85
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf

### 构建系统/构建系统.md

- reference 标题：构建系统
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：126
- 页面类型：other / module
- 行数：291 / 174
- 段落行数：103 / 85
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、custom-webpack-preset.ts、preview-preset.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts

### 构建系统/构建配置优化.md

- reference 标题：构建配置优化
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：68
- 页面类型：other / module
- 行数：264 / 240
- 段落行数：70 / 168
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：main.ts、manager.ts
- reference 关键文件未覆盖：preview.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js

### 核心概念/Addons系统架构/Addons开发指南.md

- reference 标题：Addons开发指南
- 生成页：系统架构.md（系统架构）
- 匹配分数：128
- 页面类型：architecture / architecture
- 行数：584 / 869
- 段落行数：390 / 298
- Evidence：23 / 1
- Mermaid：13 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、.d.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts、types.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、manager.ts、preview.ts、storybook.setup.ts、constants.ts、index.ts、types.ts

### 核心概念/Addons系统架构/Addons架构设计.md

- reference 标题：Addons架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：110
- 页面类型：architecture / architecture
- 行数：321 / 869
- 段落行数：146 / 298
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、channel.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts

### 核心概念/Addons系统架构/Addons系统架构.md

- reference 标题：Addons系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：140
- 页面类型：architecture / architecture
- 行数：356 / 869
- 段落行数：156 / 298
- Evidence：22 / 1
- Mermaid：7 / 2
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md

### 核心概念/Addons系统架构/Addons通信机制.md

- reference 标题：Addons通信机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：128
- 页面类型：architecture / architecture
- 行数：290 / 869
- 段落行数：131 / 298
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts

### 核心概念/Addons系统架构/Addons配置管理.md

- reference 标题：Addons配置管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：126
- 页面类型：architecture / architecture
- 行数：273 / 869
- 段落行数：85 / 298
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：main.js、main.ts
- reference 关键文件未覆盖：package.js、preset.ts、presets.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.ts、presets.ts

### 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md

- reference 标题：A11y Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：362 / 869
- 段落行数：125 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts

### 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md

- reference 标题：Actions Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：96
- 页面类型：architecture / architecture
- 行数：302 / 869
- 段落行数：103 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts

### 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md

- reference 标题：Backgrounds Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 页面类型：architecture / architecture
- 行数：374 / 869
- 段落行数：115 / 298
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、manager.ts、preview.ts、types.ts、utils.ts

### 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md

- reference 标题：Controls Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：104
- 页面类型：architecture / architecture
- 行数：282 / 869
- 段落行数：95 / 298
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts

### 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md

- reference 标题：Docs Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：96
- 页面类型：architecture / architecture
- 行数：348 / 869
- 段落行数：127 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts

### 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md

- reference 标题：Themes Addon
- 生成页：核心模块/code/addons/themes.md（模块：themes）
- 匹配分数：176
- 页面类型：architecture / module
- 行数：293 / 84
- 段落行数：127 / 22
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、provider.decorator.ts、manager.ts、preview.ts、theme-switcher.ts
- reference 关键文件未覆盖：constants.ts、helpers.ts、types.ts、themes.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、types.ts、themes.md

### 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md

- reference 标题：工具栏插件（Toolbars Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：290 / 869
- 段落行数：128 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、index.ts、manager.ts、preview.ts、types.ts、get-selected.ts

### 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md

- reference 标题：视口插件（Viewport Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：106
- 页面类型：architecture / architecture
- 行数：257 / 869
- 段落行数：98 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts

### 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：系统架构.md（系统架构）
- 匹配分数：104
- 页面类型：architecture / architecture
- 行数：309 / 869
- 段落行数：113 / 298
- Evidence：7 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts

### 核心概念/CSF格式规范.md

- reference 标题：CSF格式规范
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/主题和参数系统.md

- reference 标题：主题和参数系统
- 生成页：专题/module-capability/module-7351b0cc5a9b-library-csf-plugin能力：扩展机制.md（csf-plugin能力：扩展机制）
- 匹配分数：98
- 页面类型：topic / topic
- 行数：356 / 50
- 段落行数：126 / 26
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：constants.ts
- reference 关键文件未覆盖：parameters.stories.ts、basic.stories.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：parameters.stories.ts、basic.stories.ts、helpers.ts、types.ts、globalsstore.ts、getvaluesfromglobaltypes.ts、parameters.ts、convert.ts

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：74
- 页面类型：other / module
- 行数：388 / 240
- 段落行数：139 / 168
- Evidence：16 / 1
- Mermaid：7 / 2
- 文件提及重合：main.ts、manager.ts、index.ts、types.ts、index.js
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、api.md、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、api.md、unsupported-csf-variances.stories.ts、addons.ts、csf4.md、decorators.md

### 核心概念/组件故事（Stories）.md

- reference 标题：组件故事（Stories）
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/装饰器和全局状态.md

- reference 标题：装饰器和全局状态
- 生成页：核心模块/code/addons/themes.md（模块：themes）
- 匹配分数：70
- 页面类型：other / module
- 行数：307 / 84
- 段落行数：119 / 22
- Evidence：0 / 1
- Mermaid：7 / 1
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
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：70
- 页面类型：other / topic
- 行数：300 / 59
- 段落行数：109 / 33
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、utils.ts
- reference 关键文件未覆盖：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、manager.ts、params.ts、preview.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、manager.ts、params.ts、preview.ts、types.ts

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
- 生成页：项目概述.md（项目概述）
- 匹配分数：64
- 页面类型：other / overview
- 行数：228 / 120
- 段落行数：77 / 44
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：nx.js
- reference 关键文件未覆盖：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、contributing.md、security.md、package.js、playwright.config.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、.yarnrc.yml、pull_pr_template.md、code_of_conduct.md、contributing.md、security.md、package.js、playwright.config.ts

### 部署和CI_CD/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：317 / 324
- 段落行数：96 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts

### 部署和CI_CD/Chromatic集成.md

- reference 标题：Chromatic集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：242 / 324
- 段落行数：123 / 3
- Evidence：18 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js

### 部署和CI_CD/环境配置.md

- reference 标题：环境配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：workflow / overview
- 行数：299 / 120
- 段落行数：97 / 44
- Evidence：17 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts、codecov.yml、dependabot.yml、nx.js
- reference 关键文件未覆盖：config.yml、.yarnrc.yml、vitest.config.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、package.js

### 部署和CI_CD/部署和CI_CD.md

- reference 标题：部署和CI/CD
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：264 / 324
- 段落行数：105 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js

### 部署和CI_CD/静态部署.md

- reference 标题：静态部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 页面类型：workflow / workflow
- 行数：201 / 324
- 段落行数：63 / 3
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts

### 项目概述/使用场景和案例.md

- reference 标题：使用场景和案例
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 页面类型：overview / overview
- 行数：271 / 120
- 段落行数：100 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、package.js、index.md
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、package.js、index.md

### 项目概述/基本概念.md

- reference 标题：基本概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：280 / 120
- 段落行数：80 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md

### 项目概述/技术架构/Monorepo设计.md

- reference 标题：Monorepo设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：134
- 页面类型：overview / overview
- 行数：249 / 120
- 段落行数：78 / 44
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：nx.js
- reference 关键文件未覆盖：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts

### 项目概述/技术架构/技术架构.md

- reference 标题：技术架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：overview / overview
- 行数：289 / 120
- 段落行数：141 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts

### 项目概述/技术架构/插件系统架构/插件架构设计.md

- reference 标题：插件架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：overview / overview
- 行数：346 / 120
- 段落行数：158 / 44
- Evidence：17 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md

### 项目概述/技术架构/插件系统架构/插件注册机制.md

- reference 标题：插件注册机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：108
- 页面类型：overview / overview
- 行数：263 / 120
- 段落行数：100 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.js
- reference 关键文件未覆盖：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md

### 项目概述/技术架构/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：266 / 120
- 段落行数：94 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts

### 项目概述/技术架构/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：315 / 120
- 段落行数：127 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md

### 项目概述/技术架构/插件系统架构/插件通信协议.md

- reference 标题：插件通信协议
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 页面类型：overview / overview
- 行数：312 / 120
- 段落行数：136 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：index.test.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、types.ts

### 项目概述/技术架构/构建系统架构/Vite构建器.md

- reference 标题：Vite构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：overview / overview
- 行数：283 / 120
- 段落行数：127 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts

### 项目概述/技术架构/构建系统架构/Webpack构建器.md

- reference 标题：Webpack构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：242 / 120
- 段落行数：84 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js

### 项目概述/技术架构/构建系统架构/构建器架构设计.md

- reference 标题：构建器架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 页面类型：overview / overview
- 行数：363 / 120
- 段落行数：146 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts

### 项目概述/技术架构/构建系统架构/构建系统架构.md

- reference 标题：构建系统架构
- 生成页：专题/module-capability/module-f183267d666d-plugin-builder-vite能力：扩展机制.md（builder-vite能力：扩展机制）
- 匹配分数：94
- 页面类型：overview / topic
- 行数：244 / 81
- 段落行数：105 / 54
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、has-vite-plugins.ts
- reference 关键文件未覆盖：package.js、build.ts、types.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、types.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts、virtual-module-mapping.ts

### 项目概述/技术架构/核心引擎架构/CLI分发器.md

- reference 标题：CLI分发器
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：308 / 120
- 段落行数：152 / 44
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts

### 项目概述/技术架构/核心引擎架构/全局设置.md

- reference 标题：全局设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 页面类型：overview / overview
- 行数：360 / 120
- 段落行数：143 / 44
- Evidence：18 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts

### 项目概述/技术架构/核心引擎架构/核心引擎架构.md

- reference 标题：核心引擎架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：272 / 120
- 段落行数：122 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js

### 项目概述/技术架构/核心引擎架构/核心服务器.md

- reference 标题：核心服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：334 / 120
- 段落行数：131 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts

### 项目概述/技术架构/核心引擎架构/版本管理.md

- reference 标题：版本管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：359 / 120
- 段落行数：137 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts

### 项目概述/技术架构/框架适配器架构/Angular框架适配器.md

- reference 标题：Angular框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：overview / overview
- 行数：355 / 120
- 段落行数：245 / 44
- Evidence：17 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts

### 项目概述/技术架构/框架适配器架构/HTML框架适配器.md

- reference 标题：HTML框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：overview / overview
- 行数：245 / 120
- 段落行数：99 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js

### 项目概述/技术架构/框架适配器架构/React框架适配器.md

- reference 标题：React框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：108
- 页面类型：overview / overview
- 行数：326 / 120
- 段落行数：239 / 44
- Evidence：17 / 1
- Mermaid：9 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js

### 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md

- reference 标题：Svelte框架适配器
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：132
- 页面类型：overview / module
- 行数：287 / 148
- 段落行数：131 / 65
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts、utils.ts
- reference 关键文件未覆盖：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts

### 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md

- reference 标题：Vue 3框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 页面类型：overview / overview
- 行数：305 / 120
- 段落行数：212 / 44
- Evidence：15 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts

### 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md

- reference 标题：Web Components框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：overview / overview
- 行数：478 / 120
- 段落行数：315 / 44
- Evidence：17 / 1
- Mermaid：9 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts

### 项目概述/技术架构/框架适配器架构/框架适配器架构.md

- reference 标题：框架适配器架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：297 / 120
- 段落行数：136 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js

### 项目概述/技术架构/渲染系统架构/HTML渲染器.md

- reference 标题：HTML渲染器
- 生成页：核心模块/code/renderers/html.md（模块：html）
- 匹配分数：120
- 页面类型：overview / module
- 行数：250 / 85
- 段落行数：108 / 22
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：entry-preview.ts、index.ts、portable-stories.ts、render.ts
- reference 关键文件未覆盖：package.js、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts

### 项目概述/技术架构/渲染系统架构/React渲染器.md

- reference 标题：React渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：238 / 120
- 段落行数：91 / 44
- Evidence：12 / 1
- Mermaid：4 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts

### 项目概述/技术架构/渲染系统架构/Svelte渲染器.md

- reference 标题：Svelte渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：319 / 120
- 段落行数：149 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts

### 项目概述/技术架构/渲染系统架构/Vue3渲染器.md

- reference 标题：Vue3渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：217 / 120
- 段落行数：83 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、entry-preview.ts、index.ts、render.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts

### 项目概述/技术架构/渲染系统架构/渲染系统架构.md

- reference 标题：渲染系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：110
- 页面类型：overview / overview
- 行数：267 / 120
- 段落行数：130 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts

### 项目概述/核心特性/主题定制系统/主题切换机制.md

- reference 标题：主题切换机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：303 / 120
- 段落行数：101 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、index.ts、create.ts、utils.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、create.ts、utils.ts

### 项目概述/核心特性/主题定制系统/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：overview / overview
- 行数：275 / 120
- 段落行数：143 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts

### 项目概述/核心特性/主题定制系统/主题扩展开发.md

- reference 标题：主题扩展开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 页面类型：overview / overview
- 行数：343 / 120
- 段落行数：119 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、class-name.decorator.ts、data-attribute.decorator.ts、helpers.ts、provider.decorator.ts

### 项目概述/核心特性/主题定制系统/字体系统.md

- reference 标题：字体系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：221 / 120
- 段落行数：69 / 44
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：constants.ts、base.ts、global.ts、dark.ts、light.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts

### 项目概述/核心特性/主题定制系统/布局系统.md

- reference 标题：布局系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：314 / 120
- 段落行数：121 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts

### 项目概述/核心特性/主题定制系统/颜色系统.md

- reference 标题：颜色系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 页面类型：overview / overview
- 行数：192 / 120
- 段落行数：52 / 44
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、colors.ts、dark.ts、light.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts

### 项目概述/核心特性/多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：302 / 120
- 段落行数：137 / 44
- Evidence：22 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts

### 项目概述/核心特性/多框架支持/Next.js框架支持.md

- reference 标题：Next.js框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 页面类型：overview / overview
- 行数：237 / 120
- 段落行数：109 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js

### 项目概述/核心特性/多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：296 / 120
- 段落行数：107 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、index.ts、react-docgen.ts、preset.ts、preview.ts、public-types.ts、types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、preview.ts、public-types.ts、types.ts

### 项目概述/核心特性/多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：98
- 页面类型：overview / module
- 行数：299 / 148
- 段落行数：106 / 65
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts

### 项目概述/核心特性/多框架支持/Vue框架支持.md

- reference 标题：Vue框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：234 / 120
- 段落行数：129 / 44
- Evidence：12 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts

### 项目概述/核心特性/多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：overview / overview
- 行数：268 / 120
- 段落行数：110 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts

### 项目概述/核心特性/多框架支持/其他框架支持.md

- reference 标题：其他框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：309 / 120
- 段落行数：119 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、frameworks.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js

### 项目概述/核心特性/多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：overview / overview
- 行数：494 / 120
- 段落行数：123 / 44
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts

### 项目概述/核心特性/插件系统架构/Addon API设计.md

- reference 标题：Addon API设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 页面类型：overview / overview
- 行数：262 / 120
- 段落行数：98 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js、manager.ts

### 项目概述/核心特性/插件系统架构/Manager API.md

- reference 标题：Manager API
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：335 / 120
- 段落行数：124 / 44
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md

### 项目概述/核心特性/插件系统架构/Preview API.md

- reference 标题：Preview API
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 页面类型：overview / overview
- 行数：267 / 120
- 段落行数：101 / 44
- Evidence：15 / 1
- Mermaid：5 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js

### 项目概述/核心特性/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：overview / overview
- 行数：322 / 120
- 段落行数：119 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md

### 项目概述/核心特性/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：overview / overview
- 行数：417 / 120
- 段落行数：167 / 44
- Evidence：0 / 1
- Mermaid：10 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts

### 项目概述/核心特性/插件系统架构/通信机制.md

- reference 标题：通信机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 页面类型：overview / overview
- 行数：281 / 120
- 段落行数：128 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts

### 项目概述/核心特性/文档生成功能/Doc Blocks API.md

- reference 标题：Doc Blocks API
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 页面类型：overview / overview
- 行数：389 / 120
- 段落行数：121 / 44
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts

### 项目概述/核心特性/文档生成功能/MDX文档编写.md

- reference 标题：MDX文档编写
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：overview / overview
- 行数：269 / 120
- 段落行数：121 / 44
- Evidence：19 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md

### 项目概述/核心特性/文档生成功能/文档生成功能.md

- reference 标题：文档生成功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：overview / overview
- 行数：341 / 120
- 段落行数：129 / 44
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md

### 项目概述/核心特性/文档生成功能/自动文档系统.md

- reference 标题：自动文档系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：overview / overview
- 行数：267 / 120
- 段落行数：118 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts

### 项目概述/核心特性/文档生成功能/自定义文档页面.md

- reference 标题：自定义文档页面
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：overview / overview
- 行数：260 / 120
- 段落行数：110 / 44
- Evidence：18 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md
- 结论：章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md

### 项目概述/核心特性/核心特性.md

- reference 标题：核心特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：110
- 页面类型：overview / overview
- 行数：349 / 120
- 段落行数：138 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts

### 项目概述/核心特性/测试支持功能/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：373 / 120
- 段落行数：157 / 44
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js

### 项目概述/核心特性/测试支持功能/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 页面类型：overview / overview
- 行数：291 / 120
- 段落行数：114 / 44
- Evidence：23 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts

### 项目概述/核心特性/测试支持功能/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：311 / 120
- 段落行数：136 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、index.ts、manager.ts、params.ts、preview.ts、types.ts、addon-a11y.spec.ts

### 项目概述/核心特性/测试支持功能/测试支持功能.md

- reference 标题：测试支持功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：overview / overview
- 行数：338 / 120
- 段落行数：133 / 44
- Evidence：21 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts

### 项目概述/核心特性/测试支持功能/组件测试.md

- reference 标题：组件测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 页面类型：overview / overview
- 行数：266 / 120
- 段落行数：106 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts

### 项目概述/核心特性/组件开发环境/交互式调试.md

- reference 标题：交互式调试
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 页面类型：overview / overview
- 行数：264 / 120
- 段落行数：110 / 44
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts

### 项目概述/核心特性/组件开发环境/实时预览.md

- reference 标题：实时预览
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：overview / overview
- 行数：311 / 120
- 段落行数：108 / 44
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts

### 项目概述/核心特性/组件开发环境/开发服务器.md

- reference 标题：开发服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：303 / 120
- 段落行数：124 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts

### 项目概述/核心特性/组件开发环境/组件开发环境.md

- reference 标题：组件开发环境
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 页面类型：overview / overview
- 行数：312 / 120
- 段落行数：139 / 44
- Evidence：21 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts

### 项目概述/核心特性/组件开发环境/组件生命周期.md

- reference 标题：组件生命周期
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 页面类型：overview / overview
- 行数：319 / 120
- 段落行数：122 / 44
- Evidence：16 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：378
- 页面类型：overview / overview
- 行数：312 / 120
- 段落行数：131 / 44
- Evidence：21 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts
- reference 关键文件未覆盖：contributing.md、readme.md、package.js、index.md、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、index.md、tsconfig.js

### 高级功能/工具集成/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/Codemod工具.md

- reference 标题：Codemod工具
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：140
- 页面类型：other / topic
- 行数：286 / 59
- 段落行数：114 / 33
- Evidence：18 / 1
- Mermaid：5 / 1
- 文件提及重合：build-config.ts、index.ts、utils.ts
- reference 关键文件未覆盖：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts

### 高级功能/工具集成/ESLint集成.md

- reference 标题：ESLint集成
- 生成页：核心模块/code/lib/eslint-plugin.md（模块：eslint-plugin）
- 匹配分数：130
- 页面类型：other / module
- 行数：269 / 105
- 段落行数：69 / 36
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：csf.ts、recommended.ts、index.ts、csf-component.ts
- reference 关键文件未覆盖：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts

### 高级功能/工具集成/IDE配置优化.md

- reference 标题：IDE配置优化
- 生成页：项目概述.md（项目概述）
- 匹配分数：78
- 页面类型：other / overview
- 行数：339 / 120
- 段落行数：105 / 44
- Evidence：15 / 1
- Mermaid：4 / 1
- 文件提及重合：main.ts、prettier.conf
- reference 关键文件未覆盖：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js

### 高级功能/工具集成/工具集成.md

- reference 标题：工具集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：other / overview
- 行数：313 / 120
- 段落行数：105 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts、prettier.conf
- reference 关键文件未覆盖：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts

### 高级功能/工具集成/构建工具优化.md

- reference 标题：构建工具优化
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：70
- 页面类型：other / module
- 行数：287 / 174
- 段落行数：93 / 85
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts

### 高级功能/性能监控.md

- reference 标题：性能监控
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：64
- 页面类型：other / module
- 行数：340 / 240
- 段落行数：165 / 168
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：main.ts、index.ts、types.ts、utils.ts
- reference 关键文件未覆盖：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts
- 结论：章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts

### 高级功能/扩展开发/Addon开发.md

- reference 标题：Addon开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/主题开发.md

- reference 标题：主题开发
- 生成页：专题/module-capability/module-b51ea519f0f1-library-codemod能力：扩展机制.md（codemod能力：扩展机制）
- 匹配分数：130
- 页面类型：topic / topic
- 行数：320 / 59
- 段落行数：149 / 33
- Evidence：20 / 1
- Mermaid：9 / 1
- 文件提及重合：index.ts、utils.ts
- reference 关键文件未覆盖：main.ts、manager.ts、preview.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、manager.ts、preview.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts

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
- 生成页：核心模块/code/renderers/html.md（模块：html）
- 匹配分数：132
- 页面类型：other / module
- 行数：333 / 85
- 段落行数：127 / 22
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：render.ts、index.ts、portable-stories.ts、preset.ts、entry-preview.ts
- reference 关键文件未覆盖：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts

### 高级功能/扩展开发/预设开发.md

- reference 标题：预设开发
- 生成页：系统架构.md（系统架构）
- 匹配分数：64
- 页面类型：other / architecture
- 行数：292 / 869
- 段落行数：123 / 298
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js
- 结论：内容比 reference 更展开；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js

### 高级功能/自定义渲染器.md

- reference 标题：自定义渲染器
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/预设配置.md

- reference 标题：预设配置
- 生成页：核心模块/code/presets/create-react-app.md（模块：create-react-app）
- 匹配分数：76
- 页面类型：other / module
- 行数：288 / 74
- 段落行数：110 / 22
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：preset.js、index.ts
- reference 关键文件未覆盖：presets.ts、package.js、types.ts、index.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：other / overview
- 行数：375 / 120
- 段落行数：139 / 44
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.ts、prettier.conf
- reference 关键文件未覆盖：.eslintrc.js、manager.ts、preview.ts、package.js、preset.ts、build.ts、index.ts、postinstalladdon.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、package.js、preset.ts、build.ts、index.ts、postinstalladdon.ts

