# storybook Reference 对比报告

生成页面：67 页
reference 页面：176 页
命中对比：138 页
缺失对比：38 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/code/addons/pseudo-states.md 被 16 个 reference 页面共享映射
- 核心模块/code/addons/links.md 被 21 个 reference 页面共享映射
- 核心模块/code.md 被 4 个 reference 页面共享映射
- 核心模块/code/frameworks/react-vite.md 被 2 个 reference 页面共享映射
- 核心模块/code/frameworks/svelte-vite.md 被 3 个 reference 页面共享映射
- 系统架构.md 被 11 个 reference 页面共享映射
- 核心模块/code/addons/themes.md 被 2 个 reference 页面共享映射
- 核心模块/code/builders/builder-webpack5.md 被 4 个 reference 页面共享映射
- 工作流与部署.md 被 4 个 reference 页面共享映射
- 项目概述.md 被 62 个 reference 页面共享映射
- 核心模块/code/renderers/html.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 核心模块/code/addons.md (模块：addons, 65 行)
- 核心模块/code/addons/a11y.md (模块：a11y, 56 行)
- 核心模块/code/addons/docs.md (模块：docs, 37 行)
- 核心模块/code/addons/onboarding.md (模块：onboarding, 37 行)
- 核心模块/code/addons/vitest.md (模块：vitest, 47 行)
- 核心模块/code/builders.md (模块：builders, 79 行)
- 核心模块/code/core.md (模块：core, 116 行)
- 核心模块/code/frameworks.md (模块：frameworks, 81 行)
- 核心模块/code/frameworks/angular.md (模块：angular, 61 行)
- 核心模块/code/frameworks/ember.md (模块：ember, 58 行)
- 核心模块/code/frameworks/html-vite.md (模块：html-vite, 55 行)
- 核心模块/code/frameworks/nextjs-vite.md (模块：nextjs-vite, 62 行)
- 核心模块/code/frameworks/nextjs.md (模块：nextjs, 62 行)
- 核心模块/code/frameworks/preact-vite.md (模块：preact-vite, 48 行)
- 核心模块/code/frameworks/react-native-web-vite.md (模块：react-native-web-vite, 50 行)
- 核心模块/code/frameworks/react-webpack5.md (模块：react-webpack5, 52 行)
- 核心模块/code/frameworks/server-webpack5.md (模块：server-webpack5, 49 行)
- 核心模块/code/frameworks/sveltekit.md (模块：sveltekit, 60 行)
- 核心模块/code/frameworks/web-components-vite.md (模块：web-components-vite, 51 行)
- 核心模块/code/lib/cli-sb.md (模块：cli-sb, 35 行)
- 核心模块/code/lib/cli-storybook.md (模块：cli-storybook, 65 行)
- 核心模块/code/lib/core-webpack.md (模块：core-webpack, 63 行)
- 核心模块/code/lib/create-storybook.md (模块：create-storybook, 64 行)
- 核心模块/code/lib/csf-plugin.md (模块：csf-plugin, 52 行)
- 核心模块/code/lib/react-dom-shim.md (模块：react-dom-shim, 54 行)
- 核心模块/code/presets.md (模块：presets, 50 行)
- 核心模块/code/presets/react-webpack.md (模块：react-webpack, 57 行)
- 核心模块/code/presets/server-webpack.md (模块：server-webpack, 58 行)
- 核心模块/code/renderers.md (模块：renderers, 74 行)
- 核心模块/code/renderers/preact.md (模块：preact, 53 行)
- 核心模块/code/renderers/react.md (模块：react, 75 行)
- 核心模块/code/renderers/server.md (模块：server, 52 行)
- 核心模块/code/renderers/svelte.md (模块：svelte, 64 行)
- 核心模块/code/renderers/vue3.md (模块：vue3, 64 行)
- 核心模块/docs.md (模块：docs, 39 行)
- 核心模块/scripts.md (模块：scripts, 137 行)
- 核心模块/scripts/eslint-plugin-local-rules.md (模块：eslint-plugin-local-rules, 40 行)
- 核心模块/test-storybooks.md (模块：test-storybooks, 59 行)
- 核心模块/test-storybooks/ember-cli.md (模块：ember-cli, 43 行)
- 核心模块/test-storybooks/external-docs.md (模块：external-docs, 49 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink.md (模块：portable-stories-kitchen-sink, 52 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/nextjs.md (模块：nextjs, 51 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/react-vitest-3.md (模块：react-vitest-3, 61 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/react.md (模块：react, 61 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/svelte.md (模块：svelte, 47 行)
- 核心模块/test-storybooks/portable-stories-kitchen-sink/vue3.md (模块：vue3, 48 行)
- 核心模块/test-storybooks/server-kitchen-sink.md (模块：server-kitchen-sink, 43 行)
- 核心模块/test-storybooks/standalone-preview.md (模块：standalone-preview, 41 行)
- 核心模块/test-storybooks/yarn-pnp.md (模块：yarn-pnp, 48 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API参考/API参考.md | 核心模块/code/addons/pseudo-states.md | 368/42 | 101/8 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts |
| API参考/CLI命令参考.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Addon API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/CSF API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Decorators API.md | 核心模块/code/addons/pseudo-states.md | 248/42 | 92/8 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts |
| API参考/开发API参考/Hooks API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Preview API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Store API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/Types API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/开发API参考/开发API参考.md | 核心模块/code/addons/pseudo-states.md | 318/42 | 107/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts |
| API参考/类型定义参考/API类型定义.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/工具类型定义.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/插件类型定义.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/构建器类型定义.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/核心类型定义.md | 核心模块/code/addons/pseudo-states.md | 358/42 | 255/8 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts |
| API参考/类型定义参考/框架类型定义.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/类型定义参考/类型定义参考.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/main.js配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/manager.js配置.md | 核心模块/code/addons/pseudo-states.md | 254/42 | 100/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layout.ts、layout.test.ts、layout.stories.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md、features-and-behavior.md、main.js |
| API参考/配置API参考/preview.js配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/构建器配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API参考/配置API参考/配置API参考.md | 核心模块/code/addons/links.md | 260/55 | 71/10 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts |
| API参考/配置API参考/预设配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 主题和外观/主题和外观.md | 核心模块/code/addons/links.md | 359/55 | 132/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts、categorize-render-errors.test.ts |
| 主题和外观/主题系统概览.md | 核心模块/code/addons/links.md | 302/55 | 113/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts |
| 主题和外观/布局和间距设计.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 主题和外观/自定义主题开发.md | 核心模块/code/addons/links.md | 372/55 | 123/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts |
| 主题和外观/颜色和字体系统.md | 核心模块/code.md | 244/65 | 90/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts |
| 多框架支持/Angular框架支持.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 多框架支持/HTML框架支持.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 多框架支持/React框架支持.md | 核心模块/code/frameworks/react-vite.md | 349/63 | 225/5 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js |
| 多框架支持/Svelte框架支持.md | 核心模块/code/frameworks/svelte-vite.md | 355/78 | 233/5 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts |
| 多框架支持/Vue 3框架支持.md | 核心模块/code/frameworks/vue3-vite.md | 279/62 | 115/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts |
| 多框架支持/Web Components框架支持.md | 核心模块/code/renderers/web-components.md | 300/60 | 223/5 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts |
| 多框架支持/多框架支持.md | 核心模块/code/addons/pseudo-states.md | 259/42 | 87/8 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、package.js |
| 快速开始.md | 系统架构.md | 264/563 | 120/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js |
| 插件系统/Addons概览.md | 核心模块/code.md | 276/65 | 97/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts |
| 插件系统/插件系统.md | 核心模块/code/addons/links.md | 356/55 | 107/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js |
| 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md | 核心模块/code/addons/links.md | 335/55 | 174/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、a11ypanel.ts、a11ycontext.ts、visionsimulator.ts、constants.ts、params.ts |
| 插件系统/核心Addons详解/Actions Addon（动作记录）.md | 核心模块/code/addons/pseudo-states.md | 354/42 | 162/8 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、actiondisplay.ts、actionoptions.ts、actionsfunction.ts |
| 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md | 核心模块/code/addons/links.md | 355/55 | 126/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.js |
| 插件系统/核心Addons详解/Controls Addon（参数控制）.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Docs Addon（文档生成）.md | 核心模块/code/addons/pseudo-states.md | 239/42 | 86/8 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md |
| 插件系统/核心Addons详解/Links Addon（故事导航）.md | 核心模块/code/addons/links.md | 362/55 | 125/10 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts |
| 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 插件系统/核心Addons详解/Themes Addon（主题切换）.md | 核心模块/code/addons/themes.md | 279/47 | 120/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md |
| 插件系统/核心Addons详解/核心Addons详解.md | 核心模块/code/addons/links.md | 342/55 | 88/10 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、main.ts、package.js、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts |
| 插件系统/第三方Addons集成.md | 核心模块/code.md | 246/65 | 98/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md |
| 插件系统/自定义Addons开发.md | 核心模块/code/addons/links.md | 304/55 | 108/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts |
| 故障排除/兼容性问题.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除/安装问题.md | 核心模块/code/lib.md | 350/65 | 106/3 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、contributing.md、readme.md、package.js、npmproxy.test.ts、npmproxy.ts、yarn2proxy.test.ts、hasmultipleversions.ts |
| 故障排除/故障排除.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除/构建和性能问题.md | 核心模块/code/builders/builder-webpack5.md | 312/84 | 98/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts |
| 故障排除/调试工具和技巧.md | 核心模块/code/addons/pseudo-states.md | 295/42 | 121/8 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、util.ts、context-in-play-function.md、context-in-play-function.ts |
| 故障排除/运行时错误.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除/配置错误.md | 核心模块/code/addons/pseudo-states.md | 299/42 | 92/8 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts、package.js |
| 构建系统/Vite构建器详解.md | 核心模块/code/builders/builder-vite.md | 300/40 | 120/8 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts |
| 构建系统/Webpack构建器详解.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 构建系统/构建器概览.md | 核心模块/code/builders/builder-webpack5.md | 321/84 | 113/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf |
| 构建系统/构建系统.md | 核心模块/code/builders/builder-webpack5.md | 291/84 | 103/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts |
| 构建系统/构建配置优化.md | 核心模块/code/addons/links.md | 264/55 | 70/10 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js |
| 核心概念/Addons系统架构/Addons开发指南.md | 核心模块/code/addons/links.md | 584/55 | 390/10 | 13/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts |
| 核心概念/Addons系统架构/Addons架构设计.md | 系统架构.md | 321/563 | 146/1 | 8/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts |
| 核心概念/Addons系统架构/Addons系统架构.md | 系统架构.md | 356/563 | 156/1 | 7/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md |
| 核心概念/Addons系统架构/Addons通信机制.md | 系统架构.md | 290/563 | 131/1 | 7/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts |
| 核心概念/Addons系统架构/Addons配置管理.md | 系统架构.md | 273/563 | 85/1 | 4/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.ts、presets.ts |
| 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md | 核心模块/code/addons/links.md | 362/55 | 125/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts |
| 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md | 系统架构.md | 302/563 | 103/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts |
| 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md | 核心模块/code/addons/links.md | 374/55 | 115/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.md |
| 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md | 系统架构.md | 282/563 | 95/1 | 4/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts |
| 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md | 系统架构.md | 348/563 | 127/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts |
| 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md | 核心模块/code/addons/pseudo-states.md | 293/42 | 127/8 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、provider.decorator.ts、theme-switcher.ts、types.ts、themes.md |
| 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md | 核心模块/code/addons/pseudo-states.md | 290/42 | 128/8 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts、register-shortcuts.ts、globals.stories.ts |
| 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md | 系统架构.md | 257/563 | 98/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts |
| 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md | 系统架构.md | 309/563 | 113/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts |
| 核心概念/CSF格式规范.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/主题和参数系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/核心概念.md | 核心模块/code/addons/pseudo-states.md | 388/42 | 139/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、index.js、csf4.md |
| 核心概念/组件故事（Stories）.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/装饰器和全局状态.md | 核心模块/code/addons/links.md | 307/55 | 119/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js |
| 核心概念/预览和管理界面.md | 核心模块/code/addons/links.md | 417/55 | 154/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts、panel.ts |
| 测试框架/Playwright测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试框架/Vitest集成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试框架/可访问性测试.md | 核心模块/code/addons/links.md | 300/55 | 109/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、addonconfigurationcommand.ts |
| 测试框架/测试框架.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试框架/组件测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试框架/视觉回归测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 部署和CI_CD/CI_CD集成.md | 工作流与部署.md | 317/324 | 96/3 | 5/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts |
| 部署和CI_CD/Chromatic集成.md | 工作流与部署.md | 242/324 | 123/3 | 7/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js |
| 部署和CI_CD/环境配置.md | 项目概述.md | 299/72 | 97/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、package.js |
| 部署和CI_CD/部署和CI_CD.md | 工作流与部署.md | 264/324 | 105/3 | 6/0 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js |
| 部署和CI_CD/静态部署.md | 工作流与部署.md | 201/324 | 63/3 | 4/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts |
| 项目概述/使用场景和案例.md | 项目概述.md | 271/72 | 100/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、package.js、index.md |
| 项目概述/基本概念.md | 项目概述.md | 280/72 | 80/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md |
| 项目概述/技术架构/Monorepo设计.md | 项目概述.md | 249/72 | 78/6 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts |
| 项目概述/技术架构/技术架构.md | 项目概述.md | 289/72 | 141/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts |
| 项目概述/技术架构/插件系统架构/插件架构设计.md | 项目概述.md | 346/72 | 158/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md |
| 项目概述/技术架构/插件系统架构/插件注册机制.md | 项目概述.md | 263/72 | 100/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md |
| 项目概述/技术架构/插件系统架构/插件生命周期管理.md | 项目概述.md | 266/72 | 94/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts |
| 项目概述/技术架构/插件系统架构/插件系统架构.md | 项目概述.md | 315/72 | 127/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md |
| 项目概述/技术架构/插件系统架构/插件通信协议.md | 项目概述.md | 312/72 | 136/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、types.ts |
| 项目概述/技术架构/构建系统架构/Vite构建器.md | 项目概述.md | 283/72 | 127/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts |
| 项目概述/技术架构/构建系统架构/Webpack构建器.md | 项目概述.md | 242/72 | 84/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js |
| 项目概述/技术架构/构建系统架构/构建器架构设计.md | 项目概述.md | 363/72 | 146/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts |
| 项目概述/技术架构/构建系统架构/构建系统架构.md | 项目概述.md | 244/72 | 105/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts |
| 项目概述/技术架构/核心引擎架构/CLI分发器.md | 项目概述.md | 308/72 | 152/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts |
| 项目概述/技术架构/核心引擎架构/全局设置.md | 项目概述.md | 360/72 | 143/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts |
| 项目概述/技术架构/核心引擎架构/核心引擎架构.md | 项目概述.md | 272/72 | 122/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js |
| 项目概述/技术架构/核心引擎架构/核心服务器.md | 项目概述.md | 334/72 | 131/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts |
| 项目概述/技术架构/核心引擎架构/版本管理.md | 项目概述.md | 359/72 | 137/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts |
| 项目概述/技术架构/框架适配器架构/Angular框架适配器.md | 项目概述.md | 355/72 | 245/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts |
| 项目概述/技术架构/框架适配器架构/HTML框架适配器.md | 项目概述.md | 245/72 | 99/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js |
| 项目概述/技术架构/框架适配器架构/React框架适配器.md | 项目概述.md | 326/72 | 239/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js |
| 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md | 核心模块/code/frameworks/svelte-vite.md | 287/78 | 131/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts |
| 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md | 项目概述.md | 305/72 | 212/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts |
| 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md | 项目概述.md | 478/72 | 315/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts |
| 项目概述/技术架构/框架适配器架构/框架适配器架构.md | 项目概述.md | 297/72 | 136/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js |
| 项目概述/技术架构/渲染系统架构/HTML渲染器.md | 核心模块/code/renderers/html.md | 250/58 | 108/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts |
| 项目概述/技术架构/渲染系统架构/React渲染器.md | 项目概述.md | 238/72 | 91/6 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts |
| 项目概述/技术架构/渲染系统架构/Svelte渲染器.md | 项目概述.md | 319/72 | 149/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts |
| 项目概述/技术架构/渲染系统架构/Vue3渲染器.md | 项目概述.md | 217/72 | 83/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts |
| 项目概述/技术架构/渲染系统架构/渲染系统架构.md | 项目概述.md | 267/72 | 130/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts |
| 项目概述/核心特性/主题定制系统/主题切换机制.md | 核心模块/code/addons/links.md | 303/55 | 101/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、create.ts |
| 项目概述/核心特性/主题定制系统/主题定制系统.md | 项目概述.md | 275/72 | 143/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts |
| 项目概述/核心特性/主题定制系统/主题扩展开发.md | 核心模块/code/addons/themes.md | 343/47 | 119/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、data-attribute.decorator.ts、helpers.ts |
| 项目概述/核心特性/主题定制系统/字体系统.md | 项目概述.md | 221/72 | 69/6 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts |
| 项目概述/核心特性/主题定制系统/布局系统.md | 项目概述.md | 314/72 | 121/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts |
| 项目概述/核心特性/主题定制系统/颜色系统.md | 项目概述.md | 192/72 | 52/6 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts |
| 项目概述/核心特性/多框架支持/Angular框架支持.md | 项目概述.md | 302/72 | 137/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts |
| 项目概述/核心特性/多框架支持/Next.js框架支持.md | 项目概述.md | 237/72 | 109/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js |
| 项目概述/核心特性/多框架支持/React框架支持.md | 核心模块/code/frameworks/react-vite.md | 296/63 | 107/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、preview.ts、public-types.ts、types.ts |
| 项目概述/核心特性/多框架支持/Svelte框架支持.md | 核心模块/code/frameworks/svelte-vite.md | 299/78 | 106/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts |
| 项目概述/核心特性/多框架支持/Vue框架支持.md | 项目概述.md | 234/72 | 129/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts |
| 项目概述/核心特性/多框架支持/Web Components框架支持.md | 项目概述.md | 268/72 | 110/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts |
| 项目概述/核心特性/多框架支持/其他框架支持.md | 项目概述.md | 309/72 | 119/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js |
| 项目概述/核心特性/多框架支持/多框架支持.md | 项目概述.md | 494/72 | 123/6 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts |
| 项目概述/核心特性/插件系统架构/Addon API设计.md | 核心模块/code/addons/pseudo-states.md | 262/42 | 98/8 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js |
| 项目概述/核心特性/插件系统架构/Manager API.md | 项目概述.md | 335/72 | 124/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md |
| 项目概述/核心特性/插件系统架构/Preview API.md | 项目概述.md | 267/72 | 101/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js |
| 项目概述/核心特性/插件系统架构/插件生命周期管理.md | 项目概述.md | 322/72 | 119/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md |
| 项目概述/核心特性/插件系统架构/插件系统架构.md | 项目概述.md | 417/72 | 167/6 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts |
| 项目概述/核心特性/插件系统架构/通信机制.md | 项目概述.md | 281/72 | 128/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts |
| 项目概述/核心特性/文档生成功能/Doc Blocks API.md | 项目概述.md | 389/72 | 121/6 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts |
| 项目概述/核心特性/文档生成功能/MDX文档编写.md | 项目概述.md | 269/72 | 121/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md |
| 项目概述/核心特性/文档生成功能/文档生成功能.md | 项目概述.md | 341/72 | 129/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md |
| 项目概述/核心特性/文档生成功能/自动文档系统.md | 项目概述.md | 267/72 | 118/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts |
| 项目概述/核心特性/文档生成功能/自定义文档页面.md | 项目概述.md | 260/72 | 110/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md |
| 项目概述/核心特性/核心特性.md | 项目概述.md | 349/72 | 138/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts |
| 项目概述/核心特性/测试支持功能/Playwright测试.md | 项目概述.md | 373/72 | 157/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js |
| 项目概述/核心特性/测试支持功能/Vitest集成.md | 项目概述.md | 291/72 | 114/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts |
| 项目概述/核心特性/测试支持功能/可访问性测试.md | 核心模块/code/addons/pseudo-states.md | 311/42 | 136/8 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf |
| 项目概述/核心特性/测试支持功能/测试支持功能.md | 项目概述.md | 338/72 | 133/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts |
| 项目概述/核心特性/测试支持功能/组件测试.md | 项目概述.md | 266/72 | 106/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts |
| 项目概述/核心特性/组件开发环境/交互式调试.md | 项目概述.md | 264/72 | 110/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts |
| 项目概述/核心特性/组件开发环境/实时预览.md | 项目概述.md | 311/72 | 108/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts |
| 项目概述/核心特性/组件开发环境/开发服务器.md | 项目概述.md | 303/72 | 124/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts |
| 项目概述/核心特性/组件开发环境/组件开发环境.md | 项目概述.md | 312/72 | 139/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts |
| 项目概述/核心特性/组件开发环境/组件生命周期.md | 项目概述.md | 319/72 | 122/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts |
| 项目概述/项目概述.md | 项目概述.md | 312/72 | 131/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、index.md、tsconfig.js |
| 高级功能/工具集成/CI_CD集成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级功能/工具集成/Codemod工具.md | 核心模块/code/lib/codemod.md | 286/61 | 114/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts |
| 高级功能/工具集成/ESLint集成.md | 核心模块/code/lib/eslint-plugin.md | 269/64 | 69/5 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts |
| 高级功能/工具集成/IDE配置优化.md | 项目概述.md | 339/72 | 105/6 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js |
| 高级功能/工具集成/工具集成.md | 项目概述.md | 313/72 | 105/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts |
| 高级功能/工具集成/构建工具优化.md | 核心模块/code/builders/builder-webpack5.md | 287/84 | 93/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts |
| 高级功能/性能监控.md | 核心模块/code.md | 340/65 | 165/5 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts |
| 高级功能/扩展开发/Addon开发.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/主题开发.md | 核心模块/code/addons/links.md | 320/55 | 149/10 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js |
| 高级功能/扩展开发/扩展开发.md | 核心模块/code/addons/links.md | 243/55 | 106/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js |
| 高级功能/扩展开发/构建器扩展.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级功能/扩展开发/渲染器扩展.md | 核心模块/code/renderers/html.md | 333/58 | 127/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts |
| 高级功能/扩展开发/预设开发.md | 系统架构.md | 292/563 | 123/1 | 6/0 | 内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js |
| 高级功能/自定义渲染器.md | 核心模块/code/addons/links.md | 324/55 | 98/10 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、render.ts、types.ts |
| 高级功能/预设配置.md | 核心模块/code/presets/create-react-app.md | 288/50 | 110/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js |
| 高级功能/高级功能.md | 核心模块/code/addons/pseudo-states.md | 375/42 | 139/8 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、main.ts、package.js、preset.ts、build.ts、postinstalladdon.ts、prettier.conf |

## 逐文件详情

### API参考/API参考.md

- reference 标题：API参考
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：80
- 行数：368 / 42
- 段落行数：101 / 8
- Mermaid：4 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、typings.d.ts、get-renderer-name.ts、root.ts、hooks.ts、package.js、mainconfigfile.ts

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
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：70
- 行数：248 / 42
- 段落行数：92 / 8
- Mermaid：5 / 0
- 文件提及重合：preview.ts、index.ts
- reference 关键文件未覆盖：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：withpseudostate.ts、decoratorfunction.ts、make-decorator.test.ts、make-decorator.ts、previewweb.ts

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
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：98
- 行数：318 / 42
- 段落行数：107 / 8
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、addons.ts、shortcuts.ts、hooks.ts、preview-web.ts

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
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：70
- 行数：358 / 42
- 段落行数：255 / 8
- Mermaid：8 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：unsupported-csf-variances.stories.ts、sbtype.ts、core-annotations.ts、csf-factories.ts、story.ts、make-decorator.ts、decorators.ts

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
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：66
- 行数：254 / 42
- 段落行数：100 / 8
- Mermaid：7 / 0
- 文件提及重合：manager.ts、index.ts
- reference 关键文件未覆盖：layout.ts、layout.test.ts、layout.stories.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md、features-and-behavior.md、main.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layout.ts、layout.test.ts、layout.stories.ts、main-config-manager-head.md、storybook-main-register-addon.md、main-config.md、features-and-behavior.md、main.js

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
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：72
- 行数：260 / 55
- 段落行数：71 / 10
- Mermaid：3 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、build-config.ts、sync-main-preview-addons.ts、csf-factories.ts、core-common.ts

### API参考/配置API参考/预设配置.md

- reference 标题：预设配置
- 生成页：无
- 问题：缺少对应生成页面

### 主题和外观/主题和外观.md

- reference 标题：主题和外观
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：100
- 行数：359 / 55
- 段落行数：132 / 10
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts、utils.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts、categorize-render-errors.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、styles.ts、story.ts、layout.test.ts、components.ts、exports.ts、categorize-render-errors.test.ts

### 主题和外观/主题系统概览.md

- reference 标题：主题系统概览
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：96
- 行数：302 / 55
- 段落行数：113 / 10
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、theme-switcher.ts、base.ts、convert.ts、create.ts、convert.test.js、create.test.js、dark.ts

### 主题和外观/布局和间距设计.md

- reference 标题：布局和间距设计
- 生成页：无
- 问题：缺少对应生成页面

### 主题和外观/自定义主题开发.md

- reference 标题：自定义主题开发
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：90
- 行数：372 / 55
- 段落行数：123 / 10
- Mermaid：5 / 0
- 文件提及重合：index.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、types.ts、base.ts、convert.ts、create.ts、ensure.ts、dark.ts

### 主题和外观/颜色和字体系统.md

- reference 标题：颜色和字体系统
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：134
- 行数：244 / 65
- 段落行数：90 / 5
- Mermaid：5 / 0
- 文件提及重合：manager.js、preset.js、preview.js、index.ts、types.ts、utils.ts
- reference 关键文件未覆盖：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：filemock.js、stylemock.js、build-config.ts、accessibilityrulemaps.ts、a11yrunner.test.ts、a11yrunner.ts、a11yrunnerutils.ts、axerulemappinghelper.ts

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
- 生成页：核心模块/code/frameworks/react-vite.md（模块：react-vite）
- 匹配分数：66
- 行数：349 / 63
- 段落行数：225 / 5
- Mermaid：7 / 0
- 文件提及重合：index.ts、preset.ts
- reference 关键文件未覆盖：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、preview.ts、storybook.setup.ts、package.js、tsconfig.js

### 多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：98
- 行数：355 / 78
- 段落行数：233 / 5
- Mermaid：8 / 0
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-svelte.spec.ts、package.js、types.ts、globals.ts、public-types.ts

### 多框架支持/Vue 3框架支持.md

- reference 标题：Vue 3框架支持
- 生成页：核心模块/code/frameworks/vue3-vite.md（模块：vue3-vite）
- 匹配分数：96
- 行数：279 / 62
- 段落行数：115 / 5
- Mermaid：6 / 0
- 文件提及重合：index.ts、preset.ts、vite-plugin.ts
- reference 关键文件未覆盖：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、types.ts、globals.ts、public-types.ts、render.ts

### 多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：核心模块/code/renderers/web-components.md（模块：web-components）
- 匹配分数：140
- 行数：300 / 60
- 段落行数：223 / 5
- Mermaid：7 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、framework-api.ts、globals.ts、public-types.ts、types.ts

### 多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：74
- 行数：259 / 42
- 段落行数：87 / 8
- Mermaid：3 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：系统架构.md（系统架构）
- 匹配分数：64
- 行数：264 / 563
- 段落行数：120 / 1
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、canvas.stories.ts、controls.stories.ts、story.stories.ts、package.js、node.js

### 插件系统/Addons概览.md

- reference 标题：Addons概览
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：66
- 行数：276 / 65
- 段落行数：97 / 5
- Mermaid：5 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、preview.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preview.ts

### 插件系统/插件系统.md

- reference 标题：插件系统
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：124
- 行数：356 / 55
- 段落行数：107 / 10
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js

### 插件系统/核心Addons详解/A11y Addon（可访问性测试）.md

- reference 标题：A11y Addon（可访问性测试）
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：106
- 行数：335 / 55
- 段落行数：174 / 10
- Mermaid：7 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：package.js、a11yrunner.ts、a11yrunnerutils.ts、a11ypanel.ts、a11ycontext.ts、visionsimulator.ts、constants.ts、params.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、a11ypanel.ts、a11ycontext.ts、visionsimulator.ts、constants.ts、params.ts

### 插件系统/核心Addons详解/Actions Addon（动作记录）.md

- reference 标题：Actions Addon（动作记录）
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：96
- 行数：354 / 42
- 段落行数：162 / 8
- Mermaid：9 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts
- reference 关键文件未覆盖：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、actiondisplay.ts、actionoptions.ts、actionsfunction.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：loaders.ts、addargs.ts、addargshelpers.ts、constants.ts、decorator.ts、actiondisplay.ts、actionoptions.ts、actionsfunction.ts

### 插件系统/核心Addons详解/Backgrounds Addon（背景设置）.md

- reference 标题：Backgrounds Addon（背景设置）
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：92
- 行数：355 / 55
- 段落行数：126 / 10
- Mermaid：5 / 0
- 文件提及重合：manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.js

### 插件系统/核心Addons详解/Controls Addon（参数控制）.md

- reference 标题：Controls Addon（参数控制）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Docs Addon（文档生成）.md

- reference 标题：Docs Addon（文档生成）
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：70
- 行数：239 / 42
- 段落行数：86 / 8
- Mermaid：5 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、preset.ts、addon-docs-options.md、storybook-builder-api-mdx.md、mdx-plugin.ts、story.md

### 插件系统/核心Addons详解/Links Addon（故事导航）.md

- reference 标题：Links Addon（故事导航）
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：276
- 行数：362 / 55
- 段落行数：125 / 10
- Mermaid：8 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts、link.test.ts、link.ts、utils.test.ts、utils.ts
- reference 关键文件未覆盖：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test.ts、readme.md、package.js、constants.ts、decorator.stories.ts、hrefto.stories.ts

### 插件系统/核心Addons详解/Outline_Measure_Toolbar Addons（辅助工具）.md

- reference 标题：Outline/Measure/Toolbar Addons（辅助工具）
- 生成页：无
- 问题：缺少对应生成页面

### 插件系统/核心Addons详解/Themes Addon（主题切换）.md

- reference 标题：Themes Addon（主题切换）
- 生成页：核心模块/code/addons/themes.md（模块：themes）
- 匹配分数：148
- 行数：279 / 47
- 段落行数：120 / 9
- Mermaid：8 / 0
- 文件提及重合：class-name.decorator.ts、index.ts、provider.decorator.ts、preview.ts
- reference 关键文件未覆盖：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、api.md、bootstrap.md、emotion.md、material-ui.md、postcss.md、styled-components.md

### 插件系统/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：66
- 行数：342 / 55
- 段落行数：88 / 10
- Mermaid：3 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：.yarnrc.yml、main.ts、package.js、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yarnrc.yml、main.ts、package.js、addon-a11y.spec.ts、addon-backgrounds.spec.ts、addon-controls.spec.ts、addon-docs.spec.ts、addon-links.spec.ts

### 插件系统/第三方Addons集成.md

- reference 标题：第三方Addons集成
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：64
- 行数：246 / 65
- 段落行数：98 / 5
- Mermaid：6 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、addon-types.md、index.md、writing-addons.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-types.md、index.md、writing-addons.md

### 插件系统/自定义Addons开发.md

- reference 标题：自定义Addons开发
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：140
- 行数：304 / 55
- 段落行数：108 / 10
- Mermaid：5 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、package.js、addon-a11y.spec.ts、manager.spec.ts、preview-api.spec.ts

### 故障排除/兼容性问题.md

- reference 标题：兼容性问题
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/安装问题.md

- reference 标题：安装问题
- 生成页：核心模块/code/lib.md（模块：lib）
- 匹配分数：76
- 行数：350 / 65
- 段落行数：106 / 3
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
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：74
- 行数：312 / 84
- 段落行数：98 / 5
- Mermaid：5 / 0
- 文件提及重合：index.ts、base-webpack.config.ts
- reference 关键文件未覆盖：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite-server.ts、file-cache.ts、utils.ts、bench-packages.ts、safe-args.ts、generate-bundle.ts

### 故障排除/调试工具和技巧.md

- reference 标题：调试工具和技巧
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：68
- 行数：295 / 42
- 段落行数：121 / 8
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、util.ts、context-in-play-function.md、context-in-play-function.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、canvas.stories.ts、instrumenter.ts、console.ts、util.ts、context-in-play-function.md、context-in-play-function.ts

### 故障排除/运行时错误.md

- reference 标题：运行时错误
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/配置错误.md

- reference 标题：配置错误
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：70
- 行数：299 / 42
- 段落行数：92 / 8
- Mermaid：4 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、normalize-stories.ts、paths.ts、server-statics.ts、configfile.ts、mainconfigfile.ts、package.js

### 构建系统/Vite构建器详解.md

- reference 标题：Vite构建器详解
- 生成页：核心模块/code/builders/builder-vite.md（模块：builder-vite）
- 匹配分数：104
- 行数：300 / 40
- 段落行数：120 / 8
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
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：64
- 行数：321 / 84
- 段落行数：113 / 5
- Mermaid：5 / 0
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts、builders.ts、vite.conf

### 构建系统/构建系统.md

- reference 标题：构建系统
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：122
- 行数：291 / 84
- 段落行数：103 / 5
- Mermaid：5 / 0
- 文件提及重合：index.ts、custom-webpack-preset.ts、preview-preset.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、build.ts、types.ts、vite-config.ts、vite-server.ts

### 构建系统/构建配置优化.md

- reference 标题：构建配置优化
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：72
- 行数：264 / 55
- 段落行数：70 / 10
- Mermaid：4 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：main.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、build.ts、chromatic.config.js、build-config.ts、core.ts、standalone.ts、tsconfig.js、package.js

### 核心概念/Addons系统架构/Addons开发指南.md

- reference 标题：Addons开发指南
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：180
- 行数：584 / 55
- 段落行数：390 / 10
- Mermaid：13 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts、utils.ts
- reference 关键文件未覆盖：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、.d.ts、main.ts、storybook.setup.ts、constants.ts、types.ts、manager.spec.ts、preview-api.spec.ts

### 核心概念/Addons系统架构/Addons架构设计.md

- reference 标题：Addons架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：106
- 行数：321 / 563
- 段落行数：146 / 1
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、channel.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、channel.ts

### 核心概念/Addons系统架构/Addons系统架构.md

- reference 标题：Addons系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：136
- 行数：356 / 563
- 段落行数：156 / 1
- Mermaid：7 / 0
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.js、remove.ts、addons.ts、postinstalladdon.ts、storybook-addon-disable-addon.md、storybook-addons-api-selectstory.md、storybook-addons-api-togglepanel.md、storybook-main-register-addon.md

### 核心概念/Addons系统架构/Addons通信机制.md

- reference 标题：Addons通信机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：124
- 行数：290 / 563
- 段落行数：131 / 1
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、geteventsourceurl.ts、types.ts、server-channel.test.ts、get-server-channel.ts、getaccesscontrolmiddleware.ts

### 核心概念/Addons系统架构/Addons配置管理.md

- reference 标题：Addons配置管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：122
- 行数：273 / 563
- 段落行数：85 / 1
- Mermaid：4 / 0
- 文件提及重合：main.js、main.ts
- reference 关键文件未覆盖：package.js、preset.ts、presets.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.ts、presets.ts

### 核心概念/Addons系统架构/核心Addons详解/A11y Addon.md

- reference 标题：A11y Addon
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：104
- 行数：362 / 55
- 段落行数：125 / 10
- Mermaid：6 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、accessibilityrulemaps.ts、a11yrunner.ts、a11ypanel.stories.ts、a11ypanel.test.ts、a11ypanel.ts、a11ycontext.test.ts、a11ycontext.ts

### 核心概念/Addons系统架构/核心Addons详解/Actions Addon.md

- reference 标题：Actions Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 行数：302 / 563
- 段落行数：103 / 1
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、migration.md、constants.ts、index.ts、loaders.ts、manager.ts、instrumenter.test.ts、instrumenter.ts

### 核心概念/Addons系统架构/核心Addons详解/Backgrounds Addon.md

- reference 标题：Backgrounds Addon
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：92
- 行数：374 / 55
- 段落行数：115 / 10
- Mermaid：5 / 0
- 文件提及重合：manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：tool.ts、constants.ts、decorator.ts、defaults.ts、types.ts、globals.stories.ts、addon-backgrounds.spec.ts、backgrounds.md

### 核心概念/Addons系统架构/核心Addons详解/Controls Addon.md

- reference 标题：Controls Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 行数：282 / 563
- 段落行数：95 / 1
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argcontrol.ts、argstable.ts、boolean.ts、color.ts、date.ts、files.ts、number.ts、object.ts

### 核心概念/Addons系统架构/核心Addons详解/Docs Addon.md

- reference 标题：Docs Addon
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 行数：348 / 563
- 段落行数：127 / 1
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、docsrenderer.ts、blocks.ts、docsstory.ts、index.ts、manager.ts、manifest.ts、mdx-react-shim.ts

### 核心概念/Addons系统架构/核心Addons详解/Themes Addon.md

- reference 标题：Themes Addon
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：92
- 行数：293 / 42
- 段落行数：127 / 8
- Mermaid：6 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts
- reference 关键文件未覆盖：constants.ts、helpers.ts、provider.decorator.ts、theme-switcher.ts、types.ts、themes.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、helpers.ts、provider.decorator.ts、theme-switcher.ts、types.ts、themes.md

### 核心概念/Addons系统架构/核心Addons详解/Toolbars Addon.md

- reference 标题：工具栏插件（Toolbars Addon）
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：96
- 行数：290 / 42
- 段落行数：128 / 8
- Mermaid：6 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts
- reference 关键文件未覆盖：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts、register-shortcuts.ts、globals.stories.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：toolbarmanager.ts、toolbarmenuselect.ts、constants.ts、types.ts、get-selected.ts、normalize-toolbar-arg-type.ts、register-shortcuts.ts、globals.stories.ts

### 核心概念/Addons系统架构/核心Addons详解/Viewport Addon.md

- reference 标题：视口插件（Viewport Addon）
- 生成页：系统架构.md（系统架构）
- 匹配分数：102
- 行数：257 / 563
- 段落行数：98 / 1
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、viewports.test.ts、viewports.ts、defaults.ts、index.ts、addon-viewport.spec.ts

### 核心概念/Addons系统架构/核心Addons详解/核心Addons详解.md

- reference 标题：核心Addons详解
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 行数：309 / 563
- 段落行数：113 / 1
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addon-a11y.spec.ts、addon-docs.spec.ts、addon-themes.spec.ts

### 核心概念/CSF格式规范.md

- reference 标题：CSF格式规范
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/主题和参数系统.md

- reference 标题：主题和参数系统
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：98
- 行数：388 / 42
- 段落行数：139 / 8
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、index.js、csf4.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、api.md、types.ts、unsupported-csf-variances.stories.ts、addons.ts、index.js、csf4.md

### 核心概念/组件故事（Stories）.md

- reference 标题：组件故事（Stories）
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/装饰器和全局状态.md

- reference 标题：装饰器和全局状态
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：76
- 行数：307 / 55
- 段落行数：119 / 10
- Mermaid：7 / 0
- 文件提及重合：preview.ts、manager.ts、index.ts
- reference 关键文件未覆盖：main.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、api.md、helpers.ts、provider.decorator.ts、hooks.ts、decorator.ts、next.js

### 核心概念/预览和管理界面.md

- reference 标题：预览和管理界面
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：68
- 行数：417 / 55
- 段落行数：154 / 10
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：main.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts、panel.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、storybook.setup.ts、types.ts、addons.ts、app.ts、layout.ts、mainareacontainer.ts、panel.ts

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
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：186
- 行数：300 / 55
- 段落行数：109 / 10
- Mermaid：5 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts、utils.ts
- reference 关键文件未覆盖：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、addonconfigurationcommand.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、a11yrunner.ts、a11yrunnerutils.ts、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、addonconfigurationcommand.ts

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
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 行数：317 / 324
- 段落行数：96 / 3
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、common-jobs.ts、main.ts、executors.ts、helpers.ts、parameters.ts、types.ts

### 部署和CI_CD/Chromatic集成.md

- reference 标题：Chromatic集成
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 行数：242 / 324
- 段落行数：123 / 3
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ischromatic.ts、main.ts、chromatic.config.js、get-chromatic-version.ts、package.js

### 部署和CI_CD/环境配置.md

- reference 标题：环境配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 行数：299 / 72
- 段落行数：97 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts、codecov.yml、dependabot.yml、nx.js
- reference 关键文件未覆盖：config.yml、.yarnrc.yml、vitest.config.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.yarnrc.yml、vitest.config.ts、package.js

### 部署和CI_CD/部署和CI_CD.md

- reference 标题：部署和CI/CD
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 行数：264 / 324
- 段落行数：105 / 3
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.generated.yml、config.yml、distribution-config.yaml、main.ts、readme.md、chromatic.config.js、package.js、project.js

### 部署和CI_CD/静态部署.md

- reference 标题：静态部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 行数：201 / 324
- 段落行数：63 / 3
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、main.ts、manager.ts、preview.ts、storybook.setup.ts、chromatic.config.js、package.js、build-package.ts

### 项目概述/使用场景和案例.md

- reference 标题：使用场景和案例
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：271 / 72
- 段落行数：100 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、package.js、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、package.js、index.md

### 项目概述/基本概念.md

- reference 标题：基本概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：280 / 72
- 段落行数：80 / 6
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、readme.md、button.stories.ts、index.md、storybook-addons.md

### 项目概述/技术架构/Monorepo设计.md

- reference 标题：Monorepo设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 行数：249 / 72
- 段落行数：78 / 6
- Mermaid：3 / 0
- 文件提及重合：nx.js
- reference 关键文件未覆盖：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、project.js、build-package.ts、check-package.ts、prepare-sandbox.ts

### 项目概述/技术架构/技术架构.md

- reference 标题：技术架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：289 / 72
- 段落行数：141 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、package.js、dispatcher.ts、index.ts、manager-stores.ts、next.js、index.js、preset.ts

### 项目概述/技术架构/插件系统架构/插件架构设计.md

- reference 标题：插件架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：346 / 72
- 段落行数：158 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、get-addon-annotations.ts、sync-main-preview-addons.ts、csf-factories.ts、addons.ts、storybook-addon-load-external-addons-preset.md、preset.md

### 项目概述/技术架构/插件系统架构/插件注册机制.md

- reference 标题：插件注册机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：263 / 72
- 段落行数：100 / 6
- Mermaid：5 / 0
- 文件提及重合：main.js
- reference 关键文件未覆盖：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、addons.ts、postinstalladdon.ts、no-uninstalled-addons.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-root-preset-manager-entries.md

### 项目概述/技术架构/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：266 / 72
- 段落行数：94 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、index.ts、remove.ts、csf-factories.ts、instrumenter.test.ts、instrumenter.ts、storyrender.test.ts

### 项目概述/技术架构/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：315 / 72
- 段落行数：127 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、addons.ts、root.ts、store.ts、storybook-addon-load-external-addons-preset.md、storybook-addons-api-getqueryparam.md、storybook-addons-api-on.md

### 项目概述/技术架构/插件系统架构/插件通信协议.md

- reference 标题：插件通信协议
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 行数：312 / 72
- 段落行数：136 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：index.test.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.test.ts、index.ts、types.ts

### 项目概述/技术架构/构建系统架构/Vite构建器.md

- reference 标题：Vite构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 行数：283 / 72
- 段落行数：127 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、preset.js、build.ts、codegen-modern-iframe-script.ts、index.ts、csf-plugin.ts、preset.ts、vite-config.ts

### 项目概述/技术架构/构建系统架构/Webpack构建器.md

- reference 标题：Webpack构建器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：242 / 72
- 段落行数：84 / 6
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、virtual-module-mapping.ts、types.ts、virtualmodulemodernentry.js

### 项目概述/技术架构/构建系统架构/构建器架构设计.md

- reference 标题：构建器架构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：363 / 72
- 段落行数：146 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、framework.ts、build-static.ts、load.ts、core-common.ts

### 项目概述/技术架构/构建系统架构/构建系统架构.md

- reference 标题：构建系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：244 / 72
- 段落行数：105 / 6
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、build.ts、index.ts、types.ts、has-vite-plugins.ts、without-vite-plugins.ts、vite-config.ts、vite-server.ts

### 项目概述/技术架构/核心引擎架构/CLI分发器.md

- reference 标题：CLI分发器
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：308 / 72
- 段落行数：152 / 6
- Mermaid：9 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：build.ts、buildindex.ts、detect.ts、dev.ts、globalsettings.ts、helpers.ts、index.ts、upgrade.ts

### 项目概述/技术架构/核心引擎架构/全局设置.md

- reference 标题：全局设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 行数：360 / 72
- 段落行数：143 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：settings.js、npmoptions.ts、globalsettings.test.ts、globalsettings.ts、index.ts、get-storybook-info.ts、load-manager-or-addons-file.ts、mainconfigfile.ts

### 项目概述/技术架构/核心引擎架构/核心引擎架构.md

- reference 标题：核心引擎架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：272 / 72
- 段落行数：122 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：dispatcher.js、readme.md、build-config.ts、package.js、tsconfig.js

### 项目概述/技术架构/核心引擎架构/核心服务器.md

- reference 标题：核心服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：334 / 72
- 段落行数：131 / 6
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、build-config.ts、index.ts、build.ts、dev.ts、server-address.ts、server-errors.ts、server-statics.ts

### 项目概述/技术架构/核心引擎架构/版本管理.md

- reference 标题：版本管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 行数：359 / 72
- 段落行数：137 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：releasing.md、migration.md、resolutions.md、jspackagemanager.ts、jspackagemanagerfactory.test.ts、yarn1proxy.ts、yarn2proxy.ts、hasmultipleversions.ts

### 项目概述/技术架构/框架适配器架构/Angular框架适配器.md

- reference 标题：Angular框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：355 / 72
- 段落行数：245 / 6
- Mermaid：8 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、build-config.ts、package.js、project.js、index.ts、error-handler.ts、run-compodoc.ts、standalone-options.ts

### 项目概述/技术架构/框架适配器架构/HTML框架适配器.md

- reference 标题：HTML框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：245 / 72
- 段落行数：99 / 6
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.ts、project.js、index.ts、framework.ts、interpolate.ts、versions.ts、frameworks.ts、package.js

### 项目概述/技术架构/框架适配器架构/React框架适配器.md

- reference 标题：React框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：326 / 72
- 段落行数：239 / 6
- Mermaid：9 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、preset.ts、types.ts、preview.ts、public-types.ts、tsconfig.js

### 项目概述/技术架构/框架适配器架构/Svelte框架适配器.md

- reference 标题：Svelte框架适配器
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：128
- 行数：287 / 78
- 段落行数：131 / 5
- Mermaid：6 / 0
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts、utils.ts
- reference 关键文件未覆盖：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.svelte.ts、framework-svelte.spec.ts、package.js、generatedocgen.ts、types.ts、portable-stories.ts、public-types.ts、main.ts

### 项目概述/技术架构/框架适配器架构/Vue 3框架适配器.md

- reference 标题：Vue 3框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：305 / 72
- 段落行数：212 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、vue-component-meta.ts、vue-docgen.ts、vue-template.ts、preset.ts、types.ts、vite-plugin.ts

### 项目概述/技术架构/框架适配器架构/Web Components框架适配器.md

- reference 标题：Web Components框架适配器
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：478 / 72
- 段落行数：315 / 6
- Mermaid：9 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：...conf、package.js、index.ts、types.ts、framework-api.ts、globals.ts、preview.ts、public-types.ts

### 项目概述/技术架构/框架适配器架构/框架适配器架构.md

- reference 标题：框架适配器架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：297 / 72
- 段落行数：136 / 6
- Mermaid：8 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、react-docgen.ts、preset.ts、types.ts、next.js

### 项目概述/技术架构/渲染系统架构/HTML渲染器.md

- reference 标题：HTML渲染器
- 生成页：核心模块/code/renderers/html.md（模块：html）
- 匹配分数：116
- 行数：250 / 58
- 段落行数：108 / 5
- Mermaid：6 / 0
- 文件提及重合：entry-preview.ts、index.ts、portable-stories.ts、render.ts
- reference 关键文件未覆盖：package.js、globals.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globals.ts、public-types.ts、types.ts

### 项目概述/技术架构/渲染系统架构/React渲染器.md

- reference 标题：React渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：238 / 72
- 段落行数：91 / 6
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：docsrenderer.ts、preset.ts、versions.ts、package.js、react-18.ts、rendertocanvas.ts

### 项目概述/技术架构/渲染系统架构/Svelte渲染器.md

- reference 标题：Svelte渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：319 / 72
- 段落行数：149 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.ts、framework-svelte.spec.ts、package.js、globals.ts、index.ts、mount.ts、portable-stories.ts、preset.ts

### 项目概述/技术架构/渲染系统架构/Vue3渲染器.md

- reference 标题：Vue3渲染器
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：217 / 72
- 段落行数：83 / 6
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、entry-preview.ts、index.ts、render.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、entry-preview.ts、index.ts、render.ts

### 项目概述/技术架构/渲染系统架构/渲染系统架构.md

- reference 标题：渲染系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：267 / 72
- 段落行数：130 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、package.js、manager-stores.ts、index.ts、globals.ts

### 项目概述/核心特性/主题定制系统/主题切换机制.md

- reference 标题：主题切换机制
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：98
- 行数：303 / 55
- 段落行数：101 / 10
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts、utils.ts
- reference 关键文件未覆盖：main.ts、create.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、create.ts

### 项目概述/核心特性/主题定制系统/主题定制系统.md

- reference 标题：主题定制系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：275 / 72
- 段落行数：143 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、create.ts、index.ts、types.ts

### 项目概述/核心特性/主题定制系统/主题扩展开发.md

- reference 标题：主题扩展开发
- 生成页：核心模块/code/addons/themes.md（模块：themes）
- 匹配分数：130
- 行数：343 / 47
- 段落行数：119 / 9
- Mermaid：6 / 0
- 文件提及重合：class-name.decorator.ts、provider.decorator.ts、index.ts、preview.ts
- reference 关键文件未覆盖：api.md、bootstrap.md、package.js、constants.ts、data-attribute.decorator.ts、helpers.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api.md、bootstrap.md、package.js、constants.ts、data-attribute.decorator.ts、helpers.ts

### 项目概述/核心特性/主题定制系统/字体系统.md

- reference 标题：字体系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：82
- 行数：221 / 72
- 段落行数：69 / 6
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：constants.ts、base.ts、global.ts、dark.ts、light.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.ts、base.ts、global.ts、dark.ts、light.ts

### 项目概述/核心特性/主题定制系统/布局系统.md

- reference 标题：布局系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：314 / 72
- 段落行数：121 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、theming.md、theme-switcher.ts、spaced.stories.ts、spaced.ts、layout.ts、layout.test.ts、visualizer.ts

### 项目概述/核心特性/主题定制系统/颜色系统.md

- reference 标题：颜色系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：192 / 72
- 段落行数：52 / 6
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、colors.ts、dark.ts、light.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、colors.ts、dark.ts、light.ts

### 项目概述/核心特性/多框架支持/Angular框架支持.md

- reference 标题：Angular框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：302 / 72
- 段落行数：137 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、config.ts、preview.ts、public-types.ts、types.ts、index.ts、framework-preset-angular-cli.ts、framework-preset-angular-ivy.ts

### 项目概述/核心特性/多框架支持/Next.js框架支持.md

- reference 标题：Next.js框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：237 / 72
- 段落行数：109 / 6
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、index.ts、preview.ts、types.ts、package.js、decorator.ts、next.js、next.config.js

### 项目概述/核心特性/多框架支持/React框架支持.md

- reference 标题：React框架支持
- 生成页：核心模块/code/frameworks/react-vite.md（模块：react-vite）
- 匹配分数：98
- 行数：296 / 63
- 段落行数：107 / 5
- Mermaid：6 / 0
- 文件提及重合：index.ts、react-docgen.ts、preset.ts
- reference 关键文件未覆盖：main.ts、package.js、preview.ts、public-types.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、preview.ts、public-types.ts、types.ts

### 项目概述/核心特性/多框架支持/Svelte框架支持.md

- reference 标题：Svelte框架支持
- 生成页：核心模块/code/frameworks/svelte-vite.md（模块：svelte-vite）
- 匹配分数：94
- 行数：299 / 78
- 段落行数：106 / 5
- Mermaid：6 / 0
- 文件提及重合：index.ts、svelte-docgen.ts、preset.ts
- reference 关键文件未覆盖：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、types.ts、config-overrides.ts、globals.ts、public-types.ts

### 项目概述/核心特性/多框架支持/Vue框架支持.md

- reference 标题：Vue框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：234 / 72
- 段落行数：129 / 6
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts

### 项目概述/核心特性/多框架支持/Web Components框架支持.md

- reference 标题：Web Components框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：268 / 72
- 段落行数：110 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：button.js、button.stories.js、build-config.ts、index.ts、package.js、preset.js、preset.ts

### 项目概述/核心特性/多框架支持/其他框架支持.md

- reference 标题：其他框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：309 / 72
- 段落行数：119 / 6
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：package.js、index.ts、frameworks.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、index.ts、frameworks.js

### 项目概述/核心特性/多框架支持/多框架支持.md

- reference 标题：多框架支持
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：494 / 72
- 段落行数：123 / 6
- Mermaid：4 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：framework-nextjs.spec.ts、framework-svelte.spec.ts、package.js、config.ts、index.ts、preview-prod.ts、preview.ts、preset.ts

### 项目概述/核心特性/插件系统架构/Addon API设计.md

- reference 标题：Addon API设计
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：90
- 行数：262 / 42
- 段落行数：98 / 8
- Mermaid：5 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts
- reference 关键文件未覆盖：params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：params.ts、preset.ts、types.ts、docsrenderer.ts、manager.js、preview.js

### 项目概述/核心特性/插件系统架构/Manager API.md

- reference 标题：Manager API
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：335 / 72
- 段落行数：124 / 6
- Mermaid：7 / 0
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、index.ts、types.ts、addons.ts、root.ts、store.ts、managererrorboundary.ts、storybook-addons-root-preset-manager-entries.md

### 项目概述/核心特性/插件系统架构/Preview API.md

- reference 标题：Preview API
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 行数：267 / 72
- 段落行数：101 / 6
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、wstoken.ts、stories.ts、previewweb.test.ts、preview-api.spec.ts、stories.js

### 项目概述/核心特性/插件系统架构/插件生命周期管理.md

- reference 标题：插件生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：322 / 72
- 段落行数：119 / 6
- Mermaid：8 / 0
- 文件提及重合：main.js
- reference 关键文件未覆盖：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、addons.ts、root.ts、status.ts、postinstalladdon.ts、storybook-addons-api-getqueryparam.md、storybook-addons-api-selectincurrentkind.md、storybook-addons-api-togglepanel.md

### 项目概述/核心特性/插件系统架构/插件系统架构.md

- reference 标题：插件系统架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：417 / 72
- 段落行数：167 / 6
- Mermaid：10 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、manager.ts、preview.ts、codegen-set-addon-channel.ts、types.ts、instrumenter.test.ts、addons.ts、root.ts

### 项目概述/核心特性/插件系统架构/通信机制.md

- reference 标题：通信机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 行数：281 / 72
- 段落行数：128 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：geteventsourceurl.ts、index.ts、types.ts、addons.ts、app.ts

### 项目概述/核心特性/文档生成功能/Doc Blocks API.md

- reference 标题：Doc Blocks API
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：389 / 72
- 段落行数：121 / 6
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：argtypes.ts、canvas.ts、controls.ts、description.ts、docspage.test.ts、meta.ts、primary.ts、source.ts

### 项目概述/核心特性/文档生成功能/MDX文档编写.md

- reference 标题：MDX文档编写
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 行数：269 / 72
- 段落行数：121 / 6
- Mermaid：8 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、docsrenderer.ts、blocks.ts、argstable.ts、index.ts、metaof.md、title.md

### 项目概述/核心特性/文档生成功能/文档生成功能.md

- reference 标题：文档生成功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 行数：341 / 72
- 段落行数：129 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、package.js、blocks.ts、index.ts、metaof.md、title.md、button.stories.ts、autodocs.md

### 项目概述/核心特性/文档生成功能/自动文档系统.md

- reference 标题：自动文档系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 行数：267 / 72
- 段落行数：118 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、docspage.md、mdx.md、csf-plugin.ts、package.js、index.ts、shared.ts

### 项目概述/核心特性/文档生成功能/自定义文档页面.md

- reference 标题：自定义文档页面
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：260 / 72
- 段落行数：110 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、metaof.md、title.md、create.ts、index.ts、index.md

### 项目概述/核心特性/核心特性.md

- reference 标题：核心特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：349 / 72
- 段落行数：138 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、preset.ts、manifest.test.ts、project.js、package.js、tabs.hooks.ts、constants.ts

### 项目概述/核心特性/测试支持功能/Playwright测试.md

- reference 标题：Playwright测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：373 / 72
- 段落行数：157 / 6
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、chromatic.config.js、addon-a11y.spec.ts、component-tests.spec.ts、manager.spec.ts、navigation.spec.ts、util.ts、package.js

### 项目概述/核心特性/测试支持功能/Vitest集成.md

- reference 标题：Vitest集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 行数：291 / 72
- 段落行数：114 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、.test.ts、vitest.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.helpers.ts

### 项目概述/核心特性/测试支持功能/可访问性测试.md

- reference 标题：可访问性测试
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：94
- 行数：311 / 42
- 段落行数：136 / 8
- Mermaid：6 / 0
- 文件提及重合：index.ts、manager.ts、preview.ts
- reference 关键文件未覆盖：package.js、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、constants.ts、params.ts、types.ts、addon-a11y.spec.ts、a11y.conf

### 项目概述/核心特性/测试支持功能/测试支持功能.md

- reference 标题：测试支持功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：338 / 72
- 段落行数：133 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、preview.ts、storybook.setup.ts、addon-a11y.spec.ts、component-tests.spec.ts、util.ts、playwright.config.ts、vitest-setup.ts

### 项目概述/核心特性/测试支持功能/组件测试.md

- reference 标题：组件测试
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 行数：266 / 72
- 段落行数：106 / 6
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preview.ts、storybook.setup.ts、vitest.config.ts、component-tests.spec.ts、playwright.config.ts、vitest-setup.ts、vitest.config.storybook.ts、vitest.shared.ts

### 项目概述/核心特性/组件开发环境/交互式调试.md

- reference 标题：交互式调试
- 生成页：项目概述.md（项目概述）
- 匹配分数：114
- 行数：264 / 72
- 段落行数：110 / 6
- Mermaid：7 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.manager.ts、manager.ts、preview.ts、index.ts、constants.ts、panel.ts

### 项目概述/核心特性/组件开发环境/实时预览.md

- reference 标题：实时预览
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：311 / 72
- 段落行数：108 / 6
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、story.ts、geteventsourceurl.ts、index.ts、zoomiframe.ts、provider.ts、csfdocsrender.ts

### 项目概述/核心特性/组件开发环境/开发服务器.md

- reference 标题：开发服务器
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 行数：303 / 72
- 段落行数：124 / 6
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、logger.ts、vite-config.ts、vite-server.ts、iframe-webpack.config.ts、file-cache.ts、dev-server.ts、get-caching-middleware.ts

### 项目概述/核心特性/组件开发环境/组件开发环境.md

- reference 标题：组件开发环境
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 行数：312 / 72
- 段落行数：139 / 6
- Mermaid：8 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：manager.ts、preview.ts、storybook.setup.ts、tests.stories.ts、button.stories.ts、environments-support.ts、index.ts、types.ts

### 项目概述/核心特性/组件开发环境/组件生命周期.md

- reference 标题：组件生命周期
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：319 / 72
- 段落行数：122 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.ts、store.ts、hooks.ts、storyrender.ts、types.ts、storybookwrappercomponent.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：360
- 行数：312 / 72
- 段落行数：131 / 6
- Mermaid：8 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：contributing.md、readme.md、package.js、index.md、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、index.md、tsconfig.js

### 高级功能/工具集成/CI_CD集成.md

- reference 标题：CI/CD集成
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/工具集成/Codemod工具.md

- reference 标题：Codemod工具
- 生成页：核心模块/code/lib/codemod.md（模块：codemod）
- 匹配分数：74
- 行数：286 / 61
- 段落行数：114 / 5
- Mermaid：5 / 0
- 文件提及重合：index.test.ts、index.ts
- reference 关键文件未覆盖：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.stories.js、run.ts、csf-factories.ts、config-to-csf-factory.ts、csf-factories-utils.ts、remove-unused-types.ts、story-to-csf-factory.ts、migrate.ts

### 高级功能/工具集成/ESLint集成.md

- reference 标题：ESLint集成
- 生成页：核心模块/code/lib/eslint-plugin.md（模块：eslint-plugin）
- 匹配分数：126
- 行数：269 / 64
- 段落行数：69 / 5
- Mermaid：4 / 0
- 文件提及重合：csf.ts、recommended.ts、index.ts、csf-component.ts
- reference 关键文件未覆盖：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、package.js、await-interactions.ts、story-exports.ts、use-storybook-expect.ts

### 高级功能/工具集成/IDE配置优化.md

- reference 标题：IDE配置优化
- 生成页：项目概述.md（项目概述）
- 匹配分数：74
- 行数：339 / 72
- 段落行数：105 / 6
- Mermaid：4 / 0
- 文件提及重合：main.ts、prettier.conf
- reference 关键文件未覆盖：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、manager.ts、preview.ts、storybook.setup.ts、tsconfig.js、vitest.config.storybook.ts、vitest.config.ts、package.js

### 高级功能/工具集成/工具集成.md

- reference 标题：工具集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：80
- 行数：313 / 72
- 段落行数：105 / 6
- Mermaid：6 / 0
- 文件提及重合：main.ts、prettier.conf
- reference 关键文件未覆盖：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、.eslintrc.js、manager.ts、preview.ts、package.js、tsconfig.js、prettier.config.js、codemod.ts

### 高级功能/工具集成/构建工具优化.md

- reference 标题：构建工具优化
- 生成页：核心模块/code/builders/builder-webpack5.md（模块：builder-webpack5）
- 匹配分数：66
- 行数：287 / 84
- 段落行数：93 / 5
- Mermaid：5 / 0
- 文件提及重合：index.ts、virtual-module-mapping.ts
- reference 关键文件未覆盖：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js、vite-config.ts、vite-server.ts、environments-support.ts、generate-bundle.ts

### 高级功能/性能监控.md

- reference 标题：性能监控
- 生成页：核心模块/code.md（模块：code）
- 匹配分数：60
- 行数：340 / 65
- 段落行数：165 / 5
- Mermaid：8 / 0
- 文件提及重合：main.ts、index.ts、types.ts、utils.ts
- reference 关键文件未覆盖：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yml、environments-support.ts、storybook-builder-api-shutdown-server.md、bench-packages.ts、bench.ts、upload-bench.ts

### 高级功能/扩展开发/Addon开发.md

- reference 标题：Addon开发
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/主题开发.md

- reference 标题：主题开发
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：104
- 行数：320 / 55
- 段落行数：149 / 10
- Mermaid：9 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts、utils.ts
- reference 关键文件未覆盖：main.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、colorpalette.stories.ts、class-name.decorator.ts、colorpalette.md、base.ts、convert.ts、create.ts、convert.test.js

### 高级功能/扩展开发/扩展开发.md

- reference 标题：扩展开发
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：68
- 行数：243 / 55
- 段落行数：106 / 10
- Mermaid：7 / 0
- 文件提及重合：manager.ts、preview.ts
- reference 关键文件未覆盖：main.ts、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.ts、package.js

### 高级功能/扩展开发/构建器扩展.md

- reference 标题：构建器扩展
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/扩展开发/渲染器扩展.md

- reference 标题：渲染器扩展
- 生成页：核心模块/code/renderers/html.md（模块：html）
- 匹配分数：128
- 行数：333 / 58
- 段落行数：127 / 5
- Mermaid：6 / 0
- 文件提及重合：render.ts、index.ts、portable-stories.ts、preset.ts、entry-preview.ts
- reference 关键文件未覆盖：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：get-renderer-name.ts、renderers.ts、canvasrenderer.ts、rendererfactory.ts、renderer.ts、renderer-to-framework.ts、no-renderer-packages.ts、sourcedecorator.ts

### 高级功能/扩展开发/预设开发.md

- reference 标题：预设开发
- 生成页：系统架构.md（系统架构）
- 匹配分数：60
- 行数：292 / 563
- 段落行数：123 / 1
- Mermaid：6 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js
- 结论：内容比 reference 更展开；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：preset.ts、build-static.ts、load.ts、configfile.ts、merge-webpack-config.ts、package.js、common-preset.js

### 高级功能/自定义渲染器.md

- reference 标题：自定义渲染器
- 生成页：核心模块/code/addons/links.md（模块：links）
- 匹配分数：86
- 行数：324 / 55
- 段落行数：98 / 10
- Mermaid：4 / 0
- 文件提及重合：index.ts、preview.ts
- reference 关键文件未覆盖：package.js、render.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、render.ts、types.ts

### 高级功能/预设配置.md

- reference 标题：预设配置
- 生成页：核心模块/code/presets/create-react-app.md（模块：create-react-app）
- 匹配分数：72
- 行数：288 / 50
- 段落行数：110 / 5
- Mermaid：6 / 0
- 文件提及重合：preset.js、index.ts
- reference 关键文件未覆盖：presets.ts、package.js、types.ts、index.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：presets.ts、package.js、types.ts、index.js

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：核心模块/code/addons/pseudo-states.md（模块：pseudo-states）
- 匹配分数：80
- 行数：375 / 42
- 段落行数：139 / 8
- Mermaid：6 / 0
- 文件提及重合：manager.ts、preview.ts、index.ts
- reference 关键文件未覆盖：.eslintrc.js、main.ts、package.js、preset.ts、build.ts、postinstalladdon.ts、prettier.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.eslintrc.js、main.ts、package.js、preset.ts、build.ts、postinstalladdon.ts、prettier.conf

