# pinia Reference 对比报告

生成页面：16 页
reference 页面：61 页
命中对比：42 页
缺失对比：19 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/packages/pinia.md 被 26 个 reference 页面共享映射
- 核心模块/packages/nuxt.md 被 4 个 reference 页面共享映射
- 核心模块/packages/online-playground.md 被 2 个 reference 页面共享映射
- 核心模块/packages/nuxt/playground.md 被 3 个 reference 页面共享映射
- 核心模块/packages/testing.md 被 2 个 reference 页面共享映射
- 项目概述.md 被 4 个 reference 页面共享映射

## 额外生成页面

- 工作流与部署.md (工作流与部署, 49 行)
- 核心模块/packages.md (模块：packages, 39 行)
- 核心模块/packages/docs.md (模块：docs, 33 行)
- 核心模块/packages/online-playground/src.md (模块：src, 36 行)
- 核心模块/packages/online-playground/src/download.md (模块：download, 37 行)
- 核心模块/packages/online-playground/src/download/template.md (模块：template, 32 行)
- 核心模块/packages/size-check.md (模块：size-check, 37 行)
- 核心模块/scripts.md (模块：scripts, 33 行)
- 系统架构.md (系统架构, 56 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/packages/pinia.md | 376/40 | 86/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts |
| API 参考/Store API.md | 核心模块/packages/nuxt.md | 322/45 | 109/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、pinia.md、composables.md、index.ts、store.ts |
| API 参考/扩展 API.md | 核心模块/packages/pinia.md | 264/40 | 84/9 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、auto-hmr-plugin.ts、module.ts、storeplugins.spec.ts、globalextensions.ts、hmr.ts |
| API 参考/映射 API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/类型定义.md | 核心模块/packages/pinia.md | 278/40 | 118/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test-d.ts、plugins.md、storeplugins.spec.ts、types.ts、index.d.ts、store.test-d.ts、typehelpers.test-d.ts |
| 开发工具/Playground 演示平台.md | 核心模块/packages/online-playground.md | 240/45 | 84/9 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、vite.config.ts |
| 开发工具/代码质量工具.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 开发工具/开发工具.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 开发工具/文档生成系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 开发工具/调试工具.md | 核心模块/packages/pinia.md | 304/40 | 127/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、nuxt.config.ts、main.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts |
| 快速开始.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除/性能问题诊断.md | 核心模块/packages/pinia.md | 257/40 | 120/9 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts |
| 故障排除/故障排除.md | 核心模块/packages/pinia.md | 349/40 | 140/9 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md |
| 故障排除/类型错误诊断.md | 核心模块/packages/pinia.md | 347/40 | 143/9 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts |
| 故障排除/调试工具使用.md | 核心模块/packages/pinia.md | 276/40 | 118/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、types.ts、testing.ts |
| 故障排除/运行时错误排查.md | 核心模块/packages/pinia.md | 375/40 | 114/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts |
| 最佳实践/团队协作规范.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 最佳实践/常见陷阱与反模式.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 最佳实践/性能优化策略.md | 核心模块/packages/pinia.md | 325/40 | 127/9 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、module.ts、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts |
| 最佳实践/最佳实践.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 最佳实践/状态设计原则.md | 核心模块/packages/nuxt/playground.md | 288/43 | 88/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md |
| 最佳实践/项目结构组织.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/Pinia 实例管理.md | 核心模块/packages/pinia.md | 319/40 | 132/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：composables.ts、plugin.vue3.ts、lifespan.spec.ts、store.spec.ts、app.conf |
| 核心概念/Store 定义和使用/Options Store.md | 核心模块/packages/pinia.md | 217/40 | 65/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、counter.ts、hmr.spec.ts、cart.ts、user.ts、store.spec.ts、types.ts、hmr.ts |
| 核心概念/Store 定义和使用/Setup Store.md | 核心模块/packages/nuxt/playground.md | 257/43 | 88/9 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts |
| 核心概念/Store 定义和使用/Store 定义和使用.md | 核心模块/packages/pinia.md | 282/40 | 106/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts |
| 核心概念/Store 定义和使用/defineStore 和 useStore.md | 核心模块/packages/pinia.md | 275/40 | 109/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.spec.ts、hmr.spec.ts、types.ts、main.ts |
| 核心概念/Store 定义和使用/mapHelpers 辅助函数.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/核心概念.md | 核心模块/packages/pinia.md | 357/40 | 145/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、maphelpers.ts、storetorefs.ts、types.ts |
| 核心概念/状态管理机制/动作跟踪与拦截.md | 核心模块/packages/pinia.md | 220/40 | 85/9 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、onaction.spec.ts、subscriptions.ts、types.ts |
| 核心概念/状态管理机制/响应式状态系统.md | 核心模块/packages/pinia.md | 384/40 | 181/9 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、maphelpers.ts、storetorefs.ts、subscriptions.ts |
| 核心概念/状态管理机制/水合与 SSR 支持.md | 核心模块/packages/nuxt.md | 302/45 | 121/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts、store.ts |
| 核心概念/状态管理机制/状态管理机制.md | 核心模块/packages/pinia.md | 378/40 | 164/9 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：env.ts、subscriptions.ts、types.ts、wholestore.ts |
| 核心概念/状态管理机制/状态补丁与重置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/状态管理机制/状态订阅系统.md | 核心模块/packages/pinia.md | 327/40 | 124/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、subscriptions.ts、types.ts、mutationtype.md |
| 核心概念/类型安全系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 框架集成/Nuxt 集成.md | 核心模块/packages/nuxt.md | 203/45 | 92/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js |
| 框架集成/Vue 3 集成.md | 核心模块/packages/pinia.md | 284/40 | 113/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts |
| 框架集成/构建工具集成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 框架集成/框架集成.md | 核心模块/packages/nuxt.md | 278/45 | 102/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、createpinia.ts、index.ts、rootstore.ts、store.ts、node.js |
| 测试策略/单元测试.md | 核心模块/packages/testing.md | 257/46 | 93/5 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md |
| 测试策略/测试策略.md | 核心模块/packages/testing.md | 235/46 | 82/5 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts |
| 测试策略/集成测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例教程/在线演示.md | 核心模块/packages/online-playground.md | 242/45 | 87/9 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、netlify.toml、package.js、defaults.ts、formatting.ts、plugin.ts |
| 示例教程/基础示例.md | 核心模块/packages/playground.md | 297/52 | 106/5 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md |
| 示例教程/实战场景.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例教程/示例教程.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例教程/进阶示例.md | 核心模块/packages/nuxt/playground.md | 268/43 | 138/9 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts |
| 贡献指南.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 项目概述/Monorepo 架构解析.md | 项目概述.md | 409/65 | 229/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、module.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js、node.js |
| 项目概述/Pinia 核心概念.md | 项目概述.md | 248/65 | 122/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、store.ts |
| 项目概述/快速开始指南.md | 核心模块/packages/pinia.md | 198/40 | 67/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts |
| 项目概述/技术栈和工具链.md | 项目概述.md | 297/65 | 122/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml |
| 项目概述/项目概述.md | 项目概述.md | 307/65 | 113/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、module.ts、pnpm-workspace.yaml、tsconfig.js |
| 高级功能/Devtools 集成.md | 核心模块/packages/pinia.md | 232/40 | 75/9 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.ts、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts |
| 高级功能/SSR 支持.md | 核心模块/packages/pinia.md | 314/40 | 134/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、nuxt.md、plugin.vue3.ts、ssr.spec.ts、state.spec.ts |
| 高级功能/异步状态管理.md | 核心模块/packages/pinia.md | 301/40 | 118/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、payload-plugin.ts、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts |
| 高级功能/插件系统.md | 核心模块/packages/pinia.md | 311/40 | 126/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md |
| 高级功能/状态持久化.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 高级功能/高级功能.md | 核心模块/packages/pinia.md | 331/40 | 136/9 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、with-skip-hydrate.ts、ssr.spec.ts、storeplugins.spec.ts、actions.ts |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：204
- 行数：376 / 40
- 段落行数：86 / 9
- Mermaid：3 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、package.js、maphelpers.ts

### API 参考/Store API.md

- reference 标题：Store API
- 生成页：核心模块/packages/nuxt.md（模块：nuxt）
- 匹配分数：76
- 行数：322 / 45
- 段落行数：109 / 11
- Mermaid：7 / 0
- 文件提及重合：with-skip-hydrate.ts、payload-plugin.ts
- reference 关键文件未覆盖：readme.md、pinia.md、composables.md、index.ts、store.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、pinia.md、composables.md、index.ts、store.ts

### API 参考/扩展 API.md

- reference 标题：扩展 API
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：142
- 行数：264 / 40
- 段落行数：84 / 9
- Mermaid：5 / 0
- 文件提及重合：plugin.ts、index.ts、store.ts
- reference 关键文件未覆盖：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、auto-hmr-plugin.ts、module.ts、storeplugins.spec.ts、globalextensions.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：plugins.md、pinia.piniaplugin.md、hot-module-replacement.md、auto-hmr-plugin.ts、module.ts、storeplugins.spec.ts、globalextensions.ts、hmr.ts

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
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：60
- 行数：278 / 40
- 段落行数：118 / 9
- Mermaid：3 / 0
- 文件提及重合：store.ts
- reference 关键文件未覆盖：.test-d.ts、plugins.md、storeplugins.spec.ts、types.ts、index.d.ts、store.test-d.ts、typehelpers.test-d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.test-d.ts、plugins.md、storeplugins.spec.ts、types.ts、index.d.ts、store.test-d.ts、typehelpers.test-d.ts

### 开发工具/Playground 演示平台.md

- reference 标题：Playground 演示平台
- 生成页：核心模块/packages/online-playground.md（模块：online-playground）
- 匹配分数：62
- 行数：240 / 45
- 段落行数：84 / 9
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、vite.config.ts

### 开发工具/代码质量工具.md

- reference 标题：代码质量工具
- 生成页：无
- 问题：缺少对应生成页面

### 开发工具/开发工具.md

- reference 标题：开发工具
- 生成页：无
- 问题：缺少对应生成页面

### 开发工具/文档生成系统.md

- reference 标题：文档生成系统
- 生成页：无
- 问题：缺少对应生成页面

### 开发工具/调试工具.md

- reference 标题：调试工具
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：144
- 行数：304 / 40
- 段落行数：127 / 9
- Mermaid：8 / 0
- 文件提及重合：createpinia.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、testing.md、plugins.md、nuxt.config.ts、main.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、nuxt.config.ts、main.ts、devtools.spec.ts、onaction.spec.ts、store.spec.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除/性能问题诊断.md

- reference 标题：性能问题诊断
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：156
- 行数：257 / 40
- 段落行数：120 / 9
- Mermaid：7 / 0
- 文件提及重合：createpinia.ts、index.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、lifespan.spec.ts、subscriptions.spec.ts、subscriptions.ts

### 故障排除/故障排除.md

- reference 标题：故障排除
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：78
- 行数：349 / 40
- 段落行数：140 / 9
- Mermaid：9 / 0
- 文件提及重合：plugin.ts、index.ts
- reference 关键文件未覆盖：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、migration-v2-v3.md、migration-vuex.md、actions.md、index.md、plugins.md、getting-started.md

### 故障排除/类型错误诊断.md

- reference 标题：类型错误诊断
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：156
- 行数：347 / 40
- 段落行数：143 / 9
- Mermaid：7 / 0
- 文件提及重合：index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：state.spec.ts、store.spec.ts、globalextensions.ts、types.ts、actions.test-d.ts、index.d.ts、typehelpers.test-d.ts

### 故障排除/调试工具使用.md

- reference 标题：调试工具使用
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：92
- 行数：276 / 40
- 段落行数：118 / 9
- Mermaid：6 / 0
- 文件提及重合：plugin.ts、index.ts
- reference 关键文件未覆盖：readme.md、testing.md、plugins.md、types.ts、testing.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、plugins.md、types.ts、testing.ts

### 故障排除/运行时错误排查.md

- reference 标题：运行时错误排查
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：204
- 行数：375 / 40
- 段落行数：114 / 9
- Mermaid：6 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.spec.ts、readme.md、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.spec.ts、storeplugins.spec.ts、hmr.ts

### 最佳实践/团队协作规范.md

- reference 标题：团队协作规范
- 生成页：无
- 问题：缺少对应生成页面

### 最佳实践/常见陷阱与反模式.md

- reference 标题：常见陷阱与反模式
- 生成页：无
- 问题：缺少对应生成页面

### 最佳实践/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：106
- 行数：325 / 40
- 段落行数：127 / 9
- Mermaid：7 / 0
- 文件提及重合：plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、module.ts、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、module.ts、ssr.spec.ts、subscriptions.spec.ts、formatting.ts、hmr.ts、subscriptions.ts、tsdown.config.ts

### 最佳实践/最佳实践.md

- reference 标题：最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 最佳实践/状态设计原则.md

- reference 标题：状态设计原则
- 生成页：核心模块/packages/nuxt/playground.md（模块：playground）
- 匹配分数：94
- 行数：288 / 43
- 段落行数：88 / 9
- Mermaid：3 / 0
- 文件提及重合：counter.ts、some-store.ts、with-skip-hydrate.ts
- reference 关键文件未覆盖：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、migration-vuex.md、actions.md、getters.md、plugins.md、state.md、index.md

### 最佳实践/项目结构组织.md

- reference 标题：项目结构组织
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/Pinia 实例管理.md

- reference 标题：Pinia 实例管理
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：158
- 行数：319 / 40
- 段落行数：132 / 9
- Mermaid：8 / 0
- 文件提及重合：createpinia.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：composables.ts、plugin.vue3.ts、lifespan.spec.ts、store.spec.ts、app.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：composables.ts、plugin.vue3.ts、lifespan.spec.ts、store.spec.ts、app.conf

### 核心概念/Store 定义和使用/Options Store.md

- reference 标题：Options Store
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：182
- 行数：217 / 40
- 段落行数：65 / 9
- Mermaid：3 / 0
- 文件提及重合：createpinia.ts、index.ts、store.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、counter.ts、hmr.spec.ts、cart.ts、user.ts、store.spec.ts、types.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、counter.ts、hmr.spec.ts、cart.ts、user.ts、store.spec.ts、types.ts、hmr.ts

### 核心概念/Store 定义和使用/Setup Store.md

- reference 标题：Setup Store
- 生成页：核心模块/packages/nuxt/playground.md（模块：playground）
- 匹配分数：126
- 行数：257 / 43
- 段落行数：88 / 9
- Mermaid：4 / 0
- 文件提及重合：teststore.ts、basic.ts、counter.ts、some-store.ts
- reference 关键文件未覆盖：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：introduction.md、with-skip-hdrate.ts、defaults.ts、actions.spec.ts、combinedstores.spec.ts、getters.spec.ts、maphelpers.ts、store.ts

### 核心概念/Store 定义和使用/Store 定义和使用.md

- reference 标题：Store 定义和使用
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：106
- 行数：282 / 40
- 段落行数：106 / 9
- Mermaid：6 / 0
- 文件提及重合：plugin.ts、index.ts、store.ts
- reference 关键文件未覆盖：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、hmr.spec.ts、storetorefs.spec.ts、maphelpers.ts、storetorefs.ts、types.ts、maphelpers.test-d.ts、typehelpers.test-d.ts

### 核心概念/Store 定义和使用/defineStore 和 useStore.md

- reference 标题：defineStore 和 useStore
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：106
- 行数：275 / 40
- 段落行数：109 / 9
- Mermaid：6 / 0
- 文件提及重合：index.ts、store.ts
- reference 关键文件未覆盖：readme.md、actions.spec.ts、hmr.spec.ts、types.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.spec.ts、hmr.spec.ts、types.ts、main.ts

### 核心概念/Store 定义和使用/mapHelpers 辅助函数.md

- reference 标题：mapHelpers 辅助函数
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：206
- 行数：357 / 40
- 段落行数：145 / 9
- Mermaid：8 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、maphelpers.ts、storetorefs.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、maphelpers.ts、storetorefs.ts、types.ts

### 核心概念/状态管理机制/动作跟踪与拦截.md

- reference 标题：动作跟踪与拦截
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：102
- 行数：220 / 40
- 段落行数：85 / 9
- Mermaid：4 / 0
- 文件提及重合：rootstore.ts、store.ts
- reference 关键文件未覆盖：actions.md、onaction.spec.ts、subscriptions.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：actions.md、onaction.spec.ts、subscriptions.ts、types.ts

### 核心概念/状态管理机制/响应式状态系统.md

- reference 标题：响应式状态系统
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：204
- 行数：384 / 40
- 段落行数：181 / 9
- Mermaid：10 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、maphelpers.ts、storetorefs.ts、subscriptions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、payload-plugin.ts、env.ts、globalextensions.ts、hmr.ts、maphelpers.ts、storetorefs.ts、subscriptions.ts

### 核心概念/状态管理机制/水合与 SSR 支持.md

- reference 标题：水合与 SSR 支持
- 生成页：核心模块/packages/nuxt.md（模块：nuxt）
- 匹配分数：128
- 行数：302 / 45
- 段落行数：121 / 11
- Mermaid：7 / 0
- 文件提及重合：with-skip-hydrate.ts、auto-hmr-plugin.ts、module.ts、payload-plugin.ts
- reference 关键文件未覆盖：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts、store.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、readme.md、hot-module-replacement.md、ssr.spec.ts、state.spec.ts、env.ts、hmr.ts、store.ts

### 核心概念/状态管理机制/状态管理机制.md

- reference 标题：状态管理机制
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：206
- 行数：378 / 40
- 段落行数：164 / 9
- Mermaid：9 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：env.ts、subscriptions.ts、types.ts、wholestore.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：env.ts、subscriptions.ts、types.ts、wholestore.ts

### 核心概念/状态管理机制/状态补丁与重置.md

- reference 标题：状态补丁与重置
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/状态管理机制/状态订阅系统.md

- reference 标题：状态订阅系统
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：68
- 行数：327 / 40
- 段落行数：124 / 9
- Mermaid：6 / 0
- 文件提及重合：plugin.ts、store.ts
- reference 关键文件未覆盖：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、subscriptions.ts、types.ts、mutationtype.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pinia.mutationtype.md、state.md、subscriptions.spec.ts、formatting.ts、subscriptions.ts、types.ts、mutationtype.md

### 核心概念/类型安全系统.md

- reference 标题：类型安全系统
- 生成页：无
- 问题：缺少对应生成页面

### 框架集成/Nuxt 集成.md

- reference 标题：Nuxt 集成
- 生成页：核心模块/packages/nuxt.md（模块：nuxt）
- 匹配分数：76
- 行数：203 / 45
- 段落行数：92 / 11
- Mermaid：4 / 0
- 文件提及重合：module.ts、payload-plugin.ts
- reference 关键文件未覆盖：readme.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js

### 框架集成/Vue 3 集成.md

- reference 标题：Vue 3 集成
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：206
- 行数：284 / 40
- 段落行数：113 / 9
- Mermaid：6 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、plugins.md、state.md、maphelpers.ts、types.ts

### 框架集成/构建工具集成.md

- reference 标题：构建工具集成
- 生成页：无
- 问题：缺少对应生成页面

### 框架集成/框架集成.md

- reference 标题：框架集成
- 生成页：核心模块/packages/nuxt.md（模块：nuxt）
- 匹配分数：170
- 行数：278 / 45
- 段落行数：102 / 11
- Mermaid：6 / 0
- 文件提及重合：module.ts、composables.ts、plugin.vue3.ts
- reference 关键文件未覆盖：readme.md、package.js、createpinia.ts、index.ts、rootstore.ts、store.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、createpinia.ts、index.ts、rootstore.ts、store.ts、node.js

### 测试策略/单元测试.md

- reference 标题：单元测试
- 生成页：核心模块/packages/testing.md（模块：testing）
- 匹配分数：80
- 行数：257 / 46
- 段落行数：93 / 5
- Mermaid：5 / 0
- 文件提及重合：index.ts、testing.ts
- reference 关键文件未覆盖：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、pinia_testing.testingoptions.md、package.js、initialstate.spec.ts、testing.spec.ts、testingoptions.md

### 测试策略/测试策略.md

- reference 标题：测试策略
- 生成页：核心模块/packages/testing.md（模块：testing）
- 匹配分数：76
- 行数：235 / 46
- 段落行数：82 / 5
- Mermaid：4 / 0
- 文件提及重合：index.ts、testing.ts
- reference 关键文件未覆盖：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、vitest-setup.ts、package.js、mocked-store.spec.ts、testing.spec.ts、vitest.config.ts

### 测试策略/集成测试.md

- reference 标题：集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 示例教程/在线演示.md

- reference 标题：在线演示
- 生成页：核心模块/packages/online-playground.md（模块：online-playground）
- 匹配分数：72
- 行数：242 / 45
- 段落行数：87 / 9
- Mermaid：5 / 0
- 文件提及重合：main.ts
- reference 关键文件未覆盖：readme.md、netlify.toml、package.js、defaults.ts、formatting.ts、plugin.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、netlify.toml、package.js、defaults.ts、formatting.ts、plugin.ts

### 示例教程/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/packages/playground.md（模块：playground）
- 匹配分数：86
- 行数：297 / 52
- 段落行数：106 / 5
- Mermaid：6 / 0
- 文件提及重合：counter.ts、user.ts、jokes.ts、nasa.ts
- reference 关键文件未覆盖：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、actions.md、getters.md、index.md、state.md、getting-started.md

### 示例教程/实战场景.md

- reference 标题：实战场景
- 生成页：无
- 问题：缺少对应生成页面

### 示例教程/示例教程.md

- reference 标题：示例教程
- 生成页：无
- 问题：缺少对应生成页面

### 示例教程/进阶示例.md

- reference 标题：进阶示例
- 生成页：核心模块/packages/nuxt/playground.md（模块：playground）
- 匹配分数：130
- 行数：268 / 43
- 段落行数：138 / 9
- Mermaid：9 / 0
- 文件提及重合：teststore.ts、counter.ts、some-store.ts、with-skip-hydrate.ts
- reference 关键文件未覆盖：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migration-vuex.md、plugins.md、hmr.spec.ts、cart.ts、user.ts

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述/Monorepo 架构解析.md

- reference 标题：Monorepo 架构解析
- 生成页：项目概述.md（项目概述）
- 匹配分数：152
- 行数：409 / 65
- 段落行数：229 / 6
- Mermaid：7 / 0
- 文件提及重合：index.ts、vitest.config.ts
- reference 关键文件未覆盖：readme.md、package.js、module.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、module.ts、pnpm-lock.yaml、pnpm-workspace.yaml、renovate.js、tsconfig.js、node.js

### 项目概述/Pinia 核心概念.md

- reference 标题：Pinia 核心概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 行数：248 / 65
- 段落行数：122 / 6
- Mermaid：8 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、store.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、migration-vuex.md、index.md、hmr.spec.ts、storeplugins.spec.ts、package.js、store.ts

### 项目概述/快速开始指南.md

- reference 标题：快速开始指南
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：206
- 行数：198 / 40
- 段落行数：67 / 9
- Mermaid：3 / 0
- 文件提及重合：createpinia.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.md、getters.md、state.md、getting-started.md、types.ts

### 项目概述/技术栈和工具链.md

- reference 标题：技术栈和工具链
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 行数：297 / 65
- 段落行数：122 / 6
- Mermaid：9 / 0
- 文件提及重合：vitest.config.ts
- reference 关键文件未覆盖：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.d.ts、ci.yml、.iife.js、.prettierrc.js、codecov.yml、netlify.toml、package.js、pnpm-workspace.yaml

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：386
- 行数：307 / 65
- 段落行数：113 / 6
- Mermaid：6 / 0
- 文件提及重合：index.ts、vitest.config.ts
- reference 关键文件未覆盖：contributing.md、readme.md、package.js、module.ts、pnpm-workspace.yaml、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、package.js、module.ts、pnpm-workspace.yaml、tsconfig.js

### 高级功能/Devtools 集成.md

- reference 标题：Devtools 集成
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：208
- 行数：232 / 40
- 段落行数：75 / 9
- Mermaid：4 / 0
- 文件提及重合：createpinia.ts、index.ts、plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、actions.ts、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、actions.ts、formatting.ts、utils.ts、types.ts、tsdown.config.ts、vite-env.d.ts

### 高级功能/SSR 支持.md

- reference 标题：SSR 支持
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：110
- 行数：314 / 40
- 段落行数：134 / 9
- Mermaid：8 / 0
- 文件提及重合：createpinia.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、index.md、nuxt.md、plugin.vue3.ts、ssr.spec.ts、state.spec.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、nuxt.md、plugin.vue3.ts、ssr.spec.ts、state.spec.ts

### 高级功能/异步状态管理.md

- reference 标题：异步状态管理
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：112
- 行数：301 / 40
- 段落行数：118 / 9
- Mermaid：6 / 0
- 文件提及重合：plugin.ts、store.ts
- reference 关键文件未覆盖：readme.md、composing-stores.md、payload-plugin.ts、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、composing-stores.md、payload-plugin.ts、actions.spec.ts、onaction.spec.ts、state.spec.ts、store.patch.spec.ts、hmr.ts

### 高级功能/插件系统.md

- reference 标题：插件系统
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：132
- 行数：311 / 40
- 段落行数：126 / 9
- Mermaid：6 / 0
- 文件提及重合：createpinia.ts、plugin.ts、index.ts、rootstore.ts
- reference 关键文件未覆盖：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、plugins.md、pinia.piniaplugincontext.md、globalextensions.ts、types.ts、piniaplugincontext.md

### 高级功能/状态持久化.md

- reference 标题：状态持久化
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：核心模块/packages/pinia.md（模块：pinia）
- 匹配分数：236
- 行数：331 / 40
- 段落行数：136 / 9
- Mermaid：7 / 0
- 文件提及重合：createpinia.ts、plugin.ts、index.ts、rootstore.ts、store.ts
- reference 关键文件未覆盖：readme.md、package.js、plugins.md、index.md、with-skip-hydrate.ts、ssr.spec.ts、storeplugins.spec.ts、actions.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、plugins.md、index.md、with-skip-hydrate.ts、ssr.spec.ts、storeplugins.spec.ts、actions.ts

