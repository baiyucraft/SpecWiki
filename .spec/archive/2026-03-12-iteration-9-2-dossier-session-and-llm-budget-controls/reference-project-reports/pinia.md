# pinia Reference 对比报告

生成页面：31 页
reference 页面：61 页
命中对比：57 页
缺失对比：4 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=30, total_tokens=183972, page_research=14, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 15 / reference 6
- evidence 落页：generated 29 / reference 61
- 图表达覆盖：generated 26 / reference 61
- page research 请求：14
- page enrichment 请求：0
- 已规划专题类型：专题页(8)、流程主题(7)
- 高频缺失专题：无

## 项目结论

- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/packages.md 被 31 个 reference 页面共享映射
- 核心模块/packages/online-playground/src.md 被 12 个 reference 页面共享映射
- 项目概述.md 被 6 个 reference 页面共享映射
- 核心模块/packages/nuxt/playground.md 被 2 个 reference 页面共享映射
- 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md 被 5 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-08246e30f71f-component-docs能力：界面组成.md (docs能力：界面组成, 32 行)
- 专题/module-capability/module-1babb1b52955-repository-nuxt能力：数据访问.md (nuxt能力：数据访问, 35 行)
- 专题/module-capability/module-2799d89a93ec-repository-playground能力：数据访问.md (playground能力：数据访问, 35 行)
- 专题/module-capability/module-a7c65021981c-component-packages能力：界面组成.md (packages能力：界面组成, 24 行)
- 专题/module-capability/module-d8f69edf7906-component-src能力：界面组成.md (src能力：界面组成, 34 行)
- 专题/module-capability/module-e94f45e11e08-component-online-playground能力：界面组成.md (online-playground能力：界面组成, 32 行)
- 专题/module-capability/module-f6f925556b6c-page-playground能力：界面组成.md (playground能力：界面组成, 33 行)
- 专题/process/process-asyncincrement-flow-流程主题：asyncIncrement-flow.md (流程主题：asyncIncrement flow, 25 行)
- 专题/process/process-build-flow-流程主题：build-flow.md (流程主题：build flow, 35 行)
- 专题/process/process-createtypedocapp-flow-流程主题：createTypeDocApp-flow.md (流程主题：createTypeDocApp flow, 37 行)
- 专题/process/process-generatebundle-flow-流程主题：generateBundle-flow.md (流程主题：generateBundle flow, 25 行)
- 专题/process/process-logout-flow-流程主题：logout-flow.md (流程主题：logout flow, 35 行)
- 专题/process/process-setup-flow-流程主题：setup-flow.md (流程主题：setup flow, 25 行)
- 专题/process/process-transform-flow-流程主题：transform-flow.md (流程主题：transform flow, 35 行)
- 工作流与部署.md (工作流与部署, 59 行)
- 核心模块/packages/docs.md (模块：docs, 52 行)
- 核心模块/packages/online-playground.md (模块：online-playground, 81 行)
- 核心模块/packages/online-playground/src/download.md (模块：download, 72 行)
- 核心模块/packages/online-playground/src/download/template.md (模块：template, 61 行)
- 核心模块/packages/pinia.md (模块：pinia, 108 行)
- 核心模块/packages/playground.md (模块：playground, 76 行)
- 核心模块/packages/size-check.md (模块：size-check, 53 行)
- 核心模块/packages/testing.md (模块：testing, 66 行)
- 核心模块/scripts.md (模块：scripts, 43 行)
- 系统架构.md (系统架构, 81 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/packages.md | 376/81 | 86/24 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts |
| API 参考/Store API.md | 核心模块/packages.md | 322/81 | 109/24 | 18/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、pinia.md、composables.md |
| API 参考/扩展 API.md | 核心模块/packages.md | 264/81 | 84/24 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、storeplugins.spec.ts、globalextensions.ts、hmr.ts、types.ts、demo-counter.ts |
| API 参考/映射 API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/类型定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发工具/Playground 演示平台.md | 核心模块/packages/online-playground/src.md | 240/82 | 84/22 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：vite.config.ts |
| 开发工具/代码质量工具.md | 项目概述.md | 307/86 | 116/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.oxfmtrc.js、.prettierrc.js、package.js、vitest-setup.ts、tsconfig.js、changelog.md |
| 开发工具/开发工具.md | 核心模块/packages/online-playground/src.md | 313/82 | 114/22 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、vite.config.ts、tsdown.config.ts、pnpm-workspace.yaml、vite.conf、index.ts |
| 开发工具/文档生成系统.md | 核心模块/packages/online-playground/src.md | 301/82 | 104/22 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、en.ts、shared.ts、zh.ts、typedoc.tsconfig.js、vite-typedoc-plugin.ts、pnpm-workspace.yaml |
| 开发工具/调试工具.md | 核心模块/packages.md | 304/81 | 127/24 | 19/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、nuxt.config.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts、subscriptions.spec.ts |
| 快速开始.md | 核心模块/packages/online-playground/src.md | 205/82 | 59/22 | 0/1 | 3/2 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts |
| 故障排除/性能问题诊断.md | 核心模块/packages.md | 257/81 | 120/24 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts |
| 故障排除/故障排除.md | 核心模块/packages.md | 349/81 | 140/24 | 24/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md |
| 故障排除/类型错误诊断.md | 核心模块/packages.md | 347/81 | 143/24 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts |
| 故障排除/调试工具使用.md | 核心模块/packages.md | 276/81 | 118/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、types.ts |
| 故障排除/运行时错误排查.md | 核心模块/packages.md | 375/81 | 114/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts |
| 最佳实践/团队协作规范.md | 核心模块/packages/online-playground/src.md | 301/82 | 99/22 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.spec.ts、.test.ts、pnpm-workspace.yaml |
| 最佳实践/常见陷阱与反模式.md | 核心模块/packages/online-playground/src.md | 270/82 | 103/22 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts |
| 最佳实践/性能优化策略.md | 核心模块/packages.md | 325/81 | 127/24 | 19/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts、rollup.conf |
| 最佳实践/最佳实践.md | 核心模块/packages/online-playground/src.md | 268/82 | 85/22 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md |
| 最佳实践/状态设计原则.md | 核心模块/packages.md | 288/81 | 88/24 | 15/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md |
| 最佳实践/项目结构组织.md | 核心模块/packages/online-playground/src.md | 241/82 | 114/22 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、commit-convention.md、pnpm-workspace.yaml、tsconfig.js、counterstore.ts、usexxxstore.ts、userstore.ts |
| 核心概念/Pinia 实例管理.md | 核心模块/packages.md | 319/81 | 132/24 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：lifespan.spec.ts、store.spec.ts、app.conf |
| 核心概念/Store 定义和使用/Options Store.md | 核心模块/packages.md | 217/81 | 65/24 | 12/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、store.spec.ts、types.ts、hmr.ts、subscriptions.ts |
| 核心概念/Store 定义和使用/Setup Store.md | 核心模块/packages/nuxt/playground.md | 257/80 | 88/19 | 14/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts |
| 核心概念/Store 定义和使用/Store 定义和使用.md | 核心模块/packages.md | 282/81 | 106/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts |
| 核心概念/Store 定义和使用/defineStore 和 useStore.md | 核心模块/packages.md | 275/81 | 109/24 | 18/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.spec.ts、hmr.spec.ts、types.ts |
| 核心概念/Store 定义和使用/mapHelpers 辅助函数.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心概念/核心概念.md | 核心模块/packages.md | 357/81 | 145/24 | 18/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、maphelpers.ts、storetorefs.ts、types.ts |
| 核心概念/状态管理机制/动作跟踪与拦截.md | 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md | 220/55 | 85/34 | 0/1 | 4/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、onaction.spec.ts、subscriptions.ts、types.ts |
| 核心概念/状态管理机制/响应式状态系统.md | 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md | 384/55 | 181/34 | 22/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、index.ts、maphelpers.ts、storetorefs.ts |
| 核心概念/状态管理机制/水合与 SSR 支持.md | 核心模块/packages.md | 302/81 | 121/24 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts |
| 核心概念/状态管理机制/状态管理机制.md | 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md | 378/55 | 164/34 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：env.ts、index.ts、subscriptions.ts、types.ts、wholestore.ts |
| 核心概念/状态管理机制/状态补丁与重置.md | 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md | 267/55 | 106/34 | 18/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.patch.spec.ts、types.ts、testing.ts |
| 核心概念/状态管理机制/状态订阅系统.md | 专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md | 327/55 | 124/34 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、plugin.ts、subscriptions.ts、types.ts、mutationtype.md |
| 核心概念/类型安全系统.md | 核心模块/packages.md | 301/81 | 132/24 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、pinia.piniacustomproperties.md、pinia.piniacustomstateproperties.md、pinia.piniaplugin.md、globalextensions.ts、types.ts、tsconfig.js |
| 框架集成/Nuxt 集成.md | 核心模块/packages/nuxt.md | 203/81 | 92/23 | 0/1 | 4/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js |
| 框架集成/Vue 3 集成.md | 核心模块/packages.md | 284/81 | 113/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts |
| 框架集成/构建工具集成.md | 项目概述.md | 311/86 | 119/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts、tsdown.config.ts、rollup.conf、tsup.config.ts、pnpm-workspace.yaml、tsconfig.js、changelog.md |
| 框架集成/框架集成.md | 核心模块/packages.md | 278/81 | 102/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、node.js |
| 测试策略/单元测试.md | 核心模块/packages.md | 257/81 | 93/24 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md |
| 测试策略/测试策略.md | 核心模块/packages.md | 235/81 | 82/24 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts |
| 测试策略/集成测试.md | 核心模块/packages/online-playground/src.md | 293/82 | 99/22 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、combinedstores.spec.ts、onaction.spec.ts、state.spec.ts、storeplugins.spec.ts、subscriptions.spec.ts、initialstate.spec.ts、mocked-store.spec.ts |
| 示例教程/在线演示.md | 核心模块/packages/online-playground/src.md | 242/82 | 87/22 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、defaults.ts、formatting.ts、plugin.ts |
| 示例教程/基础示例.md | 核心模块/packages.md | 297/81 | 106/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md |
| 示例教程/实战场景.md | 核心模块/packages/online-playground/src.md | 259/82 | 110/22 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、getters.md、index.md、state.md、getting-started.md |
| 示例教程/示例教程.md | 核心模块/packages.md | 221/81 | 61/24 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-vuex.md、store.patch.spec.ts、types.ts |
| 示例教程/进阶示例.md | 核心模块/packages/nuxt/playground.md | 268/80 | 138/19 | 19/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts |
| 贡献指南.md | 核心模块/packages/online-playground/src.md | 223/82 | 81/22 | 0/1 | 4/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.prettierrc.js、codecov.yml、pnpm-workspace.yaml |
| 项目概述/Monorepo 架构解析.md | 项目概述.md | 409/86 | 229/27 | 18/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、module.ts、index.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js |
| 项目概述/Pinia 核心概念.md | 项目概述.md | 248/86 | 122/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、index.ts |
| 项目概述/快速开始指南.md | 核心模块/packages.md | 198/81 | 67/24 | 0/1 | 3/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts |
| 项目概述/技术栈和工具链.md | 项目概述.md | 297/86 | 122/27 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml |
| 项目概述/项目概述.md | 项目概述.md | 307/86 | 113/27 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、module.ts、index.ts、pnpm-workspace.yaml、tsconfig.js |
| 高级功能/Devtools 集成.md | 核心模块/packages.md | 232/81 | 75/24 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts |
| 高级功能/SSR 支持.md | 核心模块/packages.md | 314/81 | 134/24 | 18/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、nuxt.md、ssr.spec.ts、state.spec.ts |
| 高级功能/异步状态管理.md | 核心模块/packages.md | 301/81 | 118/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts、usecachedrequest.ts |
| 高级功能/插件系统.md | 核心模块/packages.md | 311/81 | 126/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md |
| 高级功能/状态持久化.md | 核心模块/packages.md | 267/81 | 94/24 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、env.ts |
| 高级功能/高级功能.md | 核心模块/packages.md | 331/81 | 136/24 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、ssr.spec.ts、storeplugins.spec.ts、formatting.ts、global.d.ts |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：142
- 页面类型：other / module
- 行数：376 / 81
- 段落行数：86 / 24
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts

### API 参考/Store API.md

- reference 标题：Store API
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：144
- 页面类型：other / module
- 行数：322 / 81
- 段落行数：109 / 24
- Evidence：18 / 1
- Mermaid：7 / 1
- 文件提及重合：with-skip-hydrate.ts、payload-plugin.ts、index.ts、store.ts
- reference 关键文件未覆盖：readme.md、pinia.md、composables.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、pinia.md、composables.md

### API 参考/扩展 API.md

- reference 标题：扩展 API
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：196
- 页面类型：other / module
- 行数：264 / 81
- 段落行数：84 / 24
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：auto-hmr-plugin.ts、module.ts、plugin.ts、index.ts、store.ts、counter.ts
- reference 关键文件未覆盖：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、storeplugins.spec.ts、globalextensions.ts、hmr.ts、types.ts、demo-counter.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、storeplugins.spec.ts、globalextensions.ts、hmr.ts、types.ts、demo-counter.ts

### API 参考/映射 API.md

- reference 标题：映射 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API.md

- reference 标题：核心 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/类型定义.md

- reference 标题：类型定义
- 生成页：无
- 问题：缺少对应生成页面

### 开发工具/Playground 演示平台.md

- reference 标题：Playground 演示平台
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：240 / 82
- 段落行数：84 / 22
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：readme.md、package.js、main.ts
- reference 关键文件未覆盖：vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：vite.config.ts

### 开发工具/代码质量工具.md

- reference 标题：代码质量工具
- 生成页：项目概述.md（项目概述）
- 匹配分数：60
- 页面类型：other / overview
- 行数：307 / 86
- 段落行数：116 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：.oxfmtrc.js、.prettierrc.js、package.js、vitest-setup.ts、tsconfig.js、changelog.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.oxfmtrc.js、.prettierrc.js、package.js、vitest-setup.ts、tsconfig.js、changelog.md

### 开发工具/开发工具.md

- reference 标题：开发工具
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：86
- 页面类型：other / module
- 行数：313 / 82
- 段落行数：114 / 22
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：netlify.toml、vite.config.ts、tsdown.config.ts、pnpm-workspace.yaml、vite.conf、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、vite.config.ts、tsdown.config.ts、pnpm-workspace.yaml、vite.conf、index.ts

### 开发工具/文档生成系统.md

- reference 标题：文档生成系统
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：72
- 页面类型：other / module
- 行数：301 / 82
- 段落行数：104 / 22
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：netlify.toml、en.ts、shared.ts、zh.ts、typedoc.tsconfig.js、vite-typedoc-plugin.ts、pnpm-workspace.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、en.ts、shared.ts、zh.ts、typedoc.tsconfig.js、vite-typedoc-plugin.ts、pnpm-workspace.yaml

### 开发工具/调试工具.md

- reference 标题：调试工具
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：138
- 页面类型：other / module
- 行数：304 / 81
- 段落行数：127 / 24
- Evidence：19 / 1
- Mermaid：8 / 1
- 文件提及重合：main.ts、createpinia.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、testing.md、plugins.md、nuxt.config.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts、subscriptions.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、nuxt.config.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts、subscriptions.spec.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：205 / 82
- 段落行数：59 / 22
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：readme.md、package.js、main.ts
- reference 关键文件未覆盖：actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts

### 故障排除/性能问题诊断.md

- reference 标题：性能问题诊断
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：114
- 页面类型：other / module
- 行数：257 / 81
- 段落行数：120 / 24
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：createpinia.ts、index.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：92
- 页面类型：other / module
- 行数：349 / 81
- 段落行数：140 / 24
- Evidence：24 / 1
- Mermaid：9 / 1
- 文件提及重合：plugin.ts、index.ts
- reference 关键文件未覆盖：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md

### 故障排除/类型错误诊断.md

- reference 标题：类型错误诊断
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：112
- 页面类型：other / module
- 行数：347 / 81
- 段落行数：143 / 24
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts

### 故障排除/调试工具使用.md

- reference 标题：调试工具使用
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：132
- 页面类型：other / module
- 行数：276 / 81
- 段落行数：118 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：plugin.ts、index.ts、testing.ts
- reference 关键文件未覆盖：readme.md、testing.md、plugins.md、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、types.ts

### 故障排除/运行时错误排查.md

- reference 标题：运行时错误排查
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：162
- 页面类型：other / module
- 行数：375 / 81
- 段落行数：114 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts

### 最佳实践/团队协作规范.md

- reference 标题：团队协作规范
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：78
- 页面类型：other / module
- 行数：301 / 82
- 段落行数：99 / 22
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.spec.ts、.test.ts、pnpm-workspace.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.spec.ts、.test.ts、pnpm-workspace.yaml

### 最佳实践/常见陷阱与反模式.md

- reference 标题：常见陷阱与反模式
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：78
- 页面类型：other / module
- 行数：270 / 82
- 段落行数：103 / 22
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md、getting-started.md、counter.ts

### 最佳实践/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：118
- 页面类型：other / module
- 行数：325 / 81
- 段落行数：127 / 24
- Evidence：19 / 1
- Mermaid：7 / 1
- 文件提及重合：module.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts、rollup.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts、rollup.conf

### 最佳实践/最佳实践.md

- reference 标题：最佳实践
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：82
- 页面类型：other / module
- 行数：268 / 82
- 段落行数：85 / 22
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、composing-stores.md、testing.md、actions.md、getters.md、index.md、state.md

### 最佳实践/状态设计原则.md

- reference 标题：状态设计原则
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：112
- 页面类型：other / module
- 行数：288 / 81
- 段落行数：88 / 24
- Evidence：15 / 1
- Mermaid：3 / 1
- 文件提及重合：counter.ts、with-skip-hydrate.ts、actions.ts、plugin.ts
- reference 关键文件未覆盖：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md

### 最佳实践/项目结构组织.md

- reference 标题：项目结构组织
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：84
- 页面类型：other / module
- 行数：241 / 82
- 段落行数：114 / 22
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、commit-convention.md、pnpm-workspace.yaml、tsconfig.js、counterstore.ts、usexxxstore.ts、userstore.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、commit-convention.md、pnpm-workspace.yaml、tsconfig.js、counterstore.ts、usexxxstore.ts、userstore.ts

### 核心概念/Pinia 实例管理.md

- reference 标题：Pinia 实例管理
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：150
- 页面类型：other / module
- 行数：319 / 81
- 段落行数：132 / 24
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：composables.ts、plugin.vue3.ts、createpinia.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：lifespan.spec.ts、store.spec.ts、app.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：lifespan.spec.ts、store.spec.ts、app.conf

### 核心概念/Store 定义和使用/Options Store.md

- reference 标题：Options Store
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：160
- 页面类型：other / module
- 行数：217 / 81
- 段落行数：65 / 24
- Evidence：12 / 1
- Mermaid：3 / 1
- 文件提及重合：counter.ts、cart.ts、user.ts、createpinia.ts、index.ts、store.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、hmr.spec.ts、store.spec.ts、types.ts、hmr.ts、subscriptions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、store.spec.ts、types.ts、hmr.ts、subscriptions.ts

### 核心概念/Store 定义和使用/Setup Store.md

- reference 标题：Setup Store
- 生成页：核心模块/packages/nuxt/playground.md（模块：playground）
- 匹配分数：144
- 页面类型：other / module
- 行数：257 / 80
- 段落行数：88 / 19
- Evidence：14 / 1
- Mermaid：4 / 1
- 文件提及重合：teststore.ts、basic.ts、counter.ts、some-store.ts
- reference 关键文件未覆盖：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts

### 核心概念/Store 定义和使用/Store 定义和使用.md

- reference 标题：Store 定义和使用
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：62
- 页面类型：other / module
- 行数：282 / 81
- 段落行数：106 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：plugin.ts、index.ts、store.ts
- reference 关键文件未覆盖：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts

### 核心概念/Store 定义和使用/defineStore 和 useStore.md

- reference 标题：defineStore 和 useStore
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：116
- 页面类型：other / module
- 行数：275 / 81
- 段落行数：109 / 24
- Evidence：18 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、store.ts、main.ts
- reference 关键文件未覆盖：readme.md、actions.spec.ts、hmr.spec.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.spec.ts、hmr.spec.ts、types.ts

### 核心概念/Store 定义和使用/mapHelpers 辅助函数.md

- reference 标题：mapHelpers 辅助函数
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：144
- 页面类型：other / module
- 行数：357 / 81
- 段落行数：145 / 24
- Evidence：18 / 1
- Mermaid：8 / 1
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、maphelpers.ts、storetorefs.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、maphelpers.ts、storetorefs.ts、types.ts

### 核心概念/状态管理机制/动作跟踪与拦截.md

- reference 标题：动作跟踪与拦截
- 生成页：专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md（pinia能力：扩展机制）
- 匹配分数：148
- 页面类型：topic / topic
- 行数：220 / 55
- 段落行数：85 / 34
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：rootstore.ts、store.ts
- reference 关键文件未覆盖：actions.md、onaction.spec.ts、subscriptions.ts、types.ts
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、onaction.spec.ts、subscriptions.ts、types.ts

### 核心概念/状态管理机制/响应式状态系统.md

- reference 标题：响应式状态系统
- 生成页：专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md（pinia能力：扩展机制）
- 匹配分数：182
- 页面类型：topic / topic
- 行数：384 / 55
- 段落行数：181 / 34
- Evidence：22 / 1
- Mermaid：10 / 1
- 文件提及重合：createpinia.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、index.ts、maphelpers.ts、storetorefs.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、index.ts、maphelpers.ts、storetorefs.ts

### 核心概念/状态管理机制/水合与 SSR 支持.md

- reference 标题：水合与 SSR 支持
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：166
- 页面类型：topic / module
- 行数：302 / 81
- 段落行数：121 / 24
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：with-skip-hydrate.ts、auto-hmr-plugin.ts、module.ts、payload-plugin.ts、store.ts
- reference 关键文件未覆盖：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts

### 核心概念/状态管理机制/状态管理机制.md

- reference 标题：状态管理机制
- 生成页：专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md（pinia能力：扩展机制）
- 匹配分数：182
- 页面类型：topic / topic
- 行数：378 / 55
- 段落行数：164 / 34
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：createpinia.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：env.ts、index.ts、subscriptions.ts、types.ts、wholestore.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：env.ts、index.ts、subscriptions.ts、types.ts、wholestore.ts

### 核心概念/状态管理机制/状态补丁与重置.md

- reference 标题：状态补丁与重置
- 生成页：专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md（pinia能力：扩展机制）
- 匹配分数：124
- 页面类型：topic / topic
- 行数：267 / 55
- 段落行数：106 / 34
- Evidence：18 / 1
- Mermaid：6 / 1
- 文件提及重合：store.ts
- reference 关键文件未覆盖：state.spec.ts、store.patch.spec.ts、types.ts、testing.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.patch.spec.ts、types.ts、testing.ts

### 核心概念/状态管理机制/状态订阅系统.md

- reference 标题：状态订阅系统
- 生成页：专题/module-capability/module-93bb514e371a-library-pinia能力：扩展机制.md（pinia能力：扩展机制）
- 匹配分数：104
- 页面类型：topic / topic
- 行数：327 / 55
- 段落行数：124 / 34
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：store.ts
- reference 关键文件未覆盖：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、plugin.ts、subscriptions.ts、types.ts、mutationtype.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、plugin.ts、subscriptions.ts、types.ts、mutationtype.md

### 核心概念/类型安全系统.md

- reference 标题：类型安全系统
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：60
- 页面类型：other / module
- 行数：301 / 81
- 段落行数：132 / 24
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：package.js、pinia.piniacustomproperties.md、pinia.piniacustomstateproperties.md、pinia.piniaplugin.md、globalextensions.ts、types.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、pinia.piniacustomproperties.md、pinia.piniacustomstateproperties.md、pinia.piniaplugin.md、globalextensions.ts、types.ts、tsconfig.js

### 框架集成/Nuxt 集成.md

- reference 标题：Nuxt 集成
- 生成页：核心模块/packages/nuxt.md（模块：nuxt）
- 匹配分数：92
- 页面类型：other / module
- 行数：203 / 81
- 段落行数：92 / 23
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：module.ts、payload-plugin.ts
- reference 关键文件未覆盖：readme.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js

### 框架集成/Vue 3 集成.md

- reference 标题：Vue 3 集成
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：140
- 页面类型：other / module
- 行数：284 / 81
- 段落行数：113 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts

### 框架集成/构建工具集成.md

- reference 标题：构建工具集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：62
- 页面类型：other / overview
- 行数：311 / 86
- 段落行数：119 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：package.js、vite.config.ts、tsdown.config.ts、rollup.conf、tsup.config.ts、pnpm-workspace.yaml、tsconfig.js、changelog.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts、tsdown.config.ts、rollup.conf、tsup.config.ts、pnpm-workspace.yaml、tsconfig.js、changelog.md

### 框架集成/框架集成.md

- reference 标题：框架集成
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：222
- 页面类型：other / module
- 行数：278 / 81
- 段落行数：102 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：module.ts、composables.ts、plugin.vue3.ts、createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、node.js

### 测试策略/单元测试.md

- reference 标题：单元测试
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：90
- 页面类型：other / module
- 行数：257 / 81
- 段落行数：93 / 24
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、testing.ts
- reference 关键文件未覆盖：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md

### 测试策略/测试策略.md

- reference 标题：测试策略
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：90
- 页面类型：other / module
- 行数：235 / 81
- 段落行数：82 / 24
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：index.ts、testing.ts
- reference 关键文件未覆盖：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts

### 测试策略/集成测试.md

- reference 标题：集成测试
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：78
- 页面类型：other / module
- 行数：293 / 82
- 段落行数：99 / 22
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：.spec.ts、combinedstores.spec.ts、onaction.spec.ts、state.spec.ts、storeplugins.spec.ts、subscriptions.spec.ts、initialstate.spec.ts、mocked-store.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、combinedstores.spec.ts、onaction.spec.ts、state.spec.ts、storeplugins.spec.ts、subscriptions.spec.ts、initialstate.spec.ts、mocked-store.spec.ts

### 示例教程/在线演示.md

- reference 标题：在线演示
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：136
- 页面类型：other / module
- 行数：242 / 82
- 段落行数：87 / 22
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：readme.md、package.js、main.ts
- reference 关键文件未覆盖：netlify.toml、defaults.ts、formatting.ts、plugin.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：netlify.toml、defaults.ts、formatting.ts、plugin.ts

### 示例教程/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：96
- 页面类型：other / module
- 行数：297 / 81
- 段落行数：106 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：counter.ts、user.ts、jokes.ts、nasa.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md

### 示例教程/实战场景.md

- reference 标题：实战场景
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：76
- 页面类型：other / module
- 行数：259 / 82
- 段落行数：110 / 22
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：actions.md、getters.md、index.md、state.md、getting-started.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、getters.md、index.md、state.md、getting-started.md

### 示例教程/示例教程.md

- reference 标题：示例教程
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：100
- 页面类型：other / module
- 行数：221 / 81
- 段落行数：61 / 24
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：counter.ts、cart.ts、store.ts、user.ts
- reference 关键文件未覆盖：readme.md、package.js、migration-vuex.md、store.patch.spec.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-vuex.md、store.patch.spec.ts、types.ts

### 示例教程/进阶示例.md

- reference 标题：进阶示例
- 生成页：核心模块/packages/nuxt/playground.md（模块：playground）
- 匹配分数：144
- 页面类型：other / module
- 行数：268 / 80
- 段落行数：138 / 19
- Evidence：19 / 1
- Mermaid：9 / 1
- 文件提及重合：teststore.ts、counter.ts、some-store.ts、with-skip-hydrate.ts
- reference 关键文件未覆盖：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：核心模块/packages/online-playground/src.md（模块：src）
- 匹配分数：80
- 页面类型：other / module
- 行数：223 / 82
- 段落行数：81 / 22
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：readme.md、package.js
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.prettierrc.js、codecov.yml、pnpm-workspace.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、issue_template.md、pull_request_template.md、commit-convention.md、.prettierrc.js、codecov.yml、pnpm-workspace.yaml

### 项目概述/Monorepo 架构解析.md

- reference 标题：Monorepo 架构解析
- 生成页：项目概述.md（项目概述）
- 匹配分数：142
- 页面类型：overview / overview
- 行数：409 / 86
- 段落行数：229 / 27
- Evidence：18 / 1
- Mermaid：7 / 1
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：readme.md、package.js、module.ts、index.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、module.ts、index.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js

### 项目概述/Pinia 核心概念.md

- reference 标题：Pinia 核心概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：overview / overview
- 行数：248 / 86
- 段落行数：122 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、index.ts

### 项目概述/快速开始指南.md

- reference 标题：快速开始指南
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：142
- 页面类型：overview / module
- 行数：198 / 81
- 段落行数：67 / 24
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts

### 项目概述/技术栈和工具链.md

- reference 标题：技术栈和工具链
- 生成页：项目概述.md（项目概述）
- 匹配分数：140
- 页面类型：overview / overview
- 行数：297 / 86
- 段落行数：122 / 27
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：372
- 页面类型：overview / overview
- 行数：307 / 86
- 段落行数：113 / 27
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：contributing.md、readme.md、package.js、module.ts、index.ts、pnpm-workspace.yaml、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、module.ts、index.ts、pnpm-workspace.yaml、tsconfig.js

### 高级功能/Devtools 集成.md

- reference 标题：Devtools 集成
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：164
- 页面类型：other / module
- 行数：232 / 81
- 段落行数：75 / 24
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：createpinia.ts、actions.ts、index.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts

### 高级功能/SSR 支持.md

- reference 标题：SSR 支持
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：110
- 页面类型：other / module
- 行数：314 / 81
- 段落行数：134 / 24
- Evidence：18 / 1
- Mermaid：8 / 1
- 文件提及重合：plugin.vue3.ts、createpinia.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、index.md、nuxt.md、ssr.spec.ts、state.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、nuxt.md、ssr.spec.ts、state.spec.ts

### 高级功能/异步状态管理.md

- reference 标题：异步状态管理
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：104
- 页面类型：other / module
- 行数：301 / 81
- 段落行数：118 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：payload-plugin.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、composing-stores.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts、usecachedrequest.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts、usecachedrequest.ts

### 高级功能/插件系统.md

- reference 标题：插件系统
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：68
- 页面类型：other / module
- 行数：311 / 81
- 段落行数：126 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：createpinia.ts、plugin.ts、index.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md

### 高级功能/状态持久化.md

- reference 标题：状态持久化
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：102
- 页面类型：other / module
- 行数：267 / 81
- 段落行数：94 / 24
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：with-skip-hydrate.ts、actions.ts、main.ts
- reference 关键文件未覆盖：readme.md、package.js、plugins.md、index.md、env.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、env.ts

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：核心模块/packages.md（模块：packages）
- 匹配分数：222
- 页面类型：other / module
- 行数：331 / 81
- 段落行数：136 / 24
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：with-skip-hydrate.ts、createpinia.ts、actions.ts、plugin.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、plugins.md、index.md、ssr.spec.ts、storeplugins.spec.ts、formatting.ts、global.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、ssr.spec.ts、storeplugins.spec.ts、formatting.ts、global.d.ts

