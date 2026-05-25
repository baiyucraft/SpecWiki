# zustand Reference 对比报告

生成页面：12 页
reference 页面：97 页
命中对比：73 页
缺失对比：24 页

## 覆盖统计

- 专题页覆盖：generated 8 / reference 3
- evidence 落页：generated 10 / reference 97
- 图表达覆盖：generated 4 / reference 97
- 已规划专题类型：流程主题(7)、中间件主题(1)
- 高频缺失专题：中间件主题(2)

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference
- 高频缺失专题集中在：中间件主题

## 多页折叠现象

- 核心模块/src.md 被 59 个 reference 页面共享映射
- 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md 被 9 个 reference 页面共享映射
- 项目概述.md 被 4 个 reference 页面共享映射

## 额外生成页面

- 专题/process/process-counter-flow-流程主题：Counter-flow.md (流程主题：Counter flow, 25 行)
- 专题/process/process-external-flow-流程主题：external-flow.md (流程主题：external flow, 40 行)
- 专题/process/process-getitem-flow-流程主题：getItem-flow.md (流程主题：getItem flow, 24 行)
- 专题/process/process-persistimpl-flow-流程主题：persistImpl-flow.md (流程主题：persistImpl flow, 50 行)
- 专题/process/process-reviver-flow-流程主题：reviver-flow.md (流程主题：reviver flow, 20 行)
- 专题/process/process-setitem-flow-流程主题：setItem-flow.md (流程主题：setItem flow, 24 行)
- 工作流与部署.md (工作流与部署, 45 行)
- 系统架构.md (系统架构, 33 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/src.md | 292/49 | 100/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md |
| API 参考/React Hooks/React Hooks.md | 核心模块/src.md | 325/49 | 128/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts |
| API 参考/React Hooks/useShallow Hook.md | 核心模块/src.md | 277/49 | 102/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts |
| API 参考/React Hooks/useStore Hook.md | 核心模块/src.md | 232/49 | 90/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：use-store-with-equality-fn.md、use-store.md、basic.test.ts |
| API 参考/React Hooks/绑定 Store 模式.md | 核心模块/src.md | 299/49 | 125/6 | 19/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts |
| API 参考/核心 API/StateCreator 类型.md | 核心模块/src.md | 286/49 | 66/6 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts |
| API 参考/核心 API/create 函数.md | 核心模块/src.md | 314/49 | 95/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts |
| API 参考/核心 API/createStore 函数.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/核心 API.md | 核心模块/src.md | 309/49 | 116/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、basic.test.ts |
| API 参考/类型定义.md | 核心模块/src.md | 248/49 | 78/6 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts |
| React 集成/React 集成.md | 核心模块/src.md | 272/49 | 106/6 | 19/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts |
| React 集成/SSR 和水合支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| React 集成/useStore Hook 使用.md | 核心模块/src.md | 254/49 | 103/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts |
| React 集成/性能优化策略.md | 核心模块/src.md | 288/49 | 104/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts |
| React 集成/框架集成实践.md | 核心模块/src.md | 352/49 | 139/6 | 21/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js |
| React 集成/绑定 Store 使用.md | 核心模块/src.md | 249/49 | 103/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts |
| TypeScript 支持/TypeScript 支持.md | 核心模块/src.md | 347/49 | 150/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts |
| TypeScript 支持/基础类型使用.md | 核心模块/src.md | 397/49 | 138/6 | 19/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts |
| TypeScript 支持/第三方库集成.md | 核心模块/src.md | 321/49 | 122/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、immer.ts、persist.ts |
| TypeScript 支持/自动类型推断.md | 核心模块/src.md | 334/49 | 124/6 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts |
| TypeScript 支持/高级类型模式.md | 核心模块/src.md | 366/49 | 175/6 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts |
| 中间件系统/Immer 中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/Redux 兼容性中间件.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 274/28 | 180/7 | 14/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、redux.md、vanilla.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts |
| 中间件系统/SSR 安全中间件.md | 核心模块/src.md | 250/49 | 111/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js |
| 中间件系统/中间件概述.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 328/28 | 144/7 | 19/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、persist.md、package.js、devtools.ts、persist.ts、subscribewithselector.ts、vanilla.ts |
| 中间件系统/中间件系统.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 331/28 | 129/7 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、subscribewithselector.ts |
| 中间件系统/开发工具中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/持久化中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/组合中间件.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 229/28 | 79/7 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、vanilla.ts |
| 中间件系统/选择器订阅中间件.md | 核心模块/src.md | 294/49 | 119/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts |
| 快速开始.md | 核心模块/src.md | 209/49 | 80/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js |
| 性能优化/内存管理与泄漏防护.md | 核心模块/src.md | 288/49 | 92/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts |
| 性能优化/性能优化.md | 核心模块/src.md | 283/49 | 108/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts |
| 性能优化/性能监控与分析.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/浅比较优化策略.md | 核心模块/src.md | 309/49 | 121/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts |
| 性能优化/选择性订阅机制.md | 核心模块/src.md | 247/49 | 105/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts |
| 核心概念/React 集成设计.md | 核心模块/src.md | 239/49 | 85/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts |
| 核心概念/不可变性原则.md | 核心模块/src.md | 284/49 | 109/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、immer.ts、basic.test.ts |
| 核心概念/核心概念.md | 核心模块/src.md | 317/49 | 158/6 | 20/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts |
| 核心概念/状态管理基础.md | 核心模块/src.md | 272/49 | 112/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create.md、use-store.md、immer.md、app.js、immer.ts、basic.test.ts |
| 核心概念/订阅机制.md | 核心模块/src.md | 320/49 | 124/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts |
| 框架集成/Next.js 集成.md | 核心模块/src.md | 383/49 | 285/6 | 21/1 | 14/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts |
| 框架集成/SSR 与水合.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 框架集成/URL 哈希同步.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 框架集成/其他框架集成.md | 核心模块/src.md | 296/49 | 68/6 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts |
| 框架集成/框架集成.md | 核心模块/src.md | 323/49 | 126/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、persist.md、migrating-to-v5.md |
| 测试策略/中间件测试/React 组件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/中间件测试/中间件测试.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 346/28 | 213/7 | 18/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、devtools.test.ts |
| 测试策略/中间件测试/中间件集成测试.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 360/28 | 248/7 | 17/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、devtools.ts、persist.ts、devtools.test.ts、middlewaretypes.test.ts、persistasync.test.ts、persistsync.test.ts、test-utils.ts |
| 测试策略/中间件测试/基础 Store 测试.md | 核心模块/src.md | 355/49 | 247/6 | 21/1 | 14/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf |
| 测试策略/中间件测试/异步状态测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/中间件测试/性能测试.md | 核心模块/src.md | 249/49 | 106/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf |
| 测试策略/单元测试 Zustand Store/Store Mock 和重置机制.md | 专题/process/process-devtoolsimpl-flow-流程主题：devtoolsImpl-flow.md | 385/49 | 266/28 | 19/1 | 11/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、react.ts、vanilla.ts、basic.test.ts、setup.ts、test-utils.ts |
| 测试策略/单元测试 Zustand Store/Store 直接测试.md | 核心模块/src.md | 335/49 | 164/6 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts |
| 测试策略/单元测试 Zustand Store/单元测试 Zustand Store.md | 核心模块/src.md | 362/49 | 146/6 | 23/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts |
| 测试策略/单元测试 Zustand Store/测试最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/单元测试 Zustand Store/测试环境配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/单元测试 Zustand Store/组件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/同步测试.md | 核心模块/src.md | 370/49 | 150/6 | 19/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts |
| 测试策略/异步测试/异步中间件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/异步持久化测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/异步测试.md | 核心模块/src.md | 313/49 | 229/6 | 18/1 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts |
| 测试策略/测试最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/测试环境配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/测试策略.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/Context Provider 测试.md | 核心模块/src.md | 289/49 | 100/6 | 16/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts |
| 测试策略/组件测试/Store 测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/异步状态测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/测试环境配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/组件测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/组件集成测试.md | 核心模块/src.md | 272/49 | 98/6 | 15/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts |
| 示例和教程/基础示例.md | 核心模块/src.md | 317/49 | 129/6 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js |
| 示例和教程/复杂应用示例.md | 核心模块/src.md | 333/49 | 142/6 | 17/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js |
| 示例和教程/完整教程.md | 核心模块/src.md | 280/49 | 119/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts |
| 示例和教程/故障排除指南.md | 核心模块/src.md | 436/49 | 140/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、devtools.ts、immer.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts |
| 示例和教程/示例和教程.md | 核心模块/src.md | 335/49 | 117/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js |
| 贡献和开发.md | 核心模块/src.md | 337/49 | 107/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts |
| 迁移和升级/从 v3 迁移到 v4.md | 核心模块/src.md | 239/49 | 90/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts |
| 迁移和升级/从 v4 迁移到 v5.md | 核心模块/src.md | 244/49 | 92/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts |
| 迁移和升级/兼容性指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 迁移和升级/迁移和升级.md | 核心模块/src.md | 301/49 | 98/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts |
| 项目概述/与竞品对比.md | 核心模块/src.md | 310/49 | 120/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、immer.ts |
| 项目概述/安装与设置.md | 项目概述.md | 270/55 | 93/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js |
| 项目概述/快速开始.md | 核心模块/src.md | 277/49 | 116/6 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js |
| 项目概述/架构概览.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 314/28 | 131/7 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、index.ts、package.js、devtools.ts、persist.ts、react.ts、types.d.ts |
| 项目概述/核心概念概览/Store 设计架构.md | 核心模块/src.md | 338/49 | 154/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts |
| 项目概述/核心概念概览/不可变状态更新.md | 项目概述.md | 282/55 | 110/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts |
| 项目概述/核心概念概览/中间件系统概念.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 327/28 | 148/7 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、devtools.ts、persist.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、vanilla.ts、devtools.test.ts |
| 项目概述/核心概念概览/核心概念概览.md | 核心模块/src.md | 303/49 | 153/6 | 18/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts |
| 项目概述/核心概念概览/状态管理基础.md | 项目概述.md | 267/55 | 108/5 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts、react.ts |
| 项目概述/核心概念概览/选择性订阅机制.md | 核心模块/src.md | 296/49 | 121/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、shallow.test.ts |
| 项目概述/项目概述.md | 项目概述.md | 279/55 | 98/5 | 17/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts |
| 高级用法/Slices 模式.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级用法/传统模式.md | 核心模块/src.md | 353/49 | 131/6 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts |
| 高级用法/复杂状态管理.md | 核心模块/src.md | 244/49 | 76/6 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、package.js |
| 高级用法/自定义中间件开发.md | 专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md | 293/28 | 138/7 | 24/1 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、persist.md、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts、vanilla.ts |
| 高级用法/高级用法.md | 核心模块/src.md | 371/49 | 147/6 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、immer.ts |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：148
- 页面类型：other / module
- 行数：292 / 49
- 段落行数：100 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md

### API 参考/React Hooks/React Hooks.md

- reference 标题：React Hooks
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：100
- 页面类型：other / module
- 行数：325 / 49
- 段落行数：128 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：react.ts、shallow.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts

### API 参考/React Hooks/useShallow Hook.md

- reference 标题：useShallow Hook
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：128
- 页面类型：other / module
- 行数：277 / 49
- 段落行数：102 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts

### API 参考/React Hooks/useStore Hook.md

- reference 标题：useStore Hook
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：162
- 页面类型：other / module
- 行数：232 / 49
- 段落行数：90 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：use-store-with-equality-fn.md、use-store.md、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：use-store-with-equality-fn.md、use-store.md、basic.test.ts

### API 参考/React Hooks/绑定 Store 模式.md

- reference 标题：绑定 Store 模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：299 / 49
- 段落行数：125 / 6
- Evidence：19 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts

### API 参考/核心 API/StateCreator 类型.md

- reference 标题：StateCreator 类型
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：92
- 页面类型：other / module
- 行数：286 / 49
- 段落行数：66 / 6
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts

### API 参考/核心 API/create 函数.md

- reference 标题：create 函数
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：314 / 49
- 段落行数：95 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts

### API 参考/核心 API/createStore 函数.md

- reference 标题：createStore 函数
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/核心 API.md

- reference 标题：核心 API
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：144
- 页面类型：other / module
- 行数：309 / 49
- 段落行数：116 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create.md、use-store.md、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、basic.test.ts

### API 参考/类型定义.md

- reference 标题：类型定义
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：168
- 页面类型：other / module
- 行数：248 / 49
- 段落行数：78 / 6
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts

### React 集成/React 集成.md

- reference 标题：React 集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：other / module
- 行数：272 / 49
- 段落行数：106 / 6
- Evidence：19 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts
- reference 关键文件未覆盖：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts

### React 集成/SSR 和水合支持.md

- reference 标题：SSR 和水合支持
- 生成页：无
- 问题：缺少对应生成页面

### React 集成/useStore Hook 使用.md

- reference 标题：useStore Hook 使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：114
- 页面类型：other / module
- 行数：254 / 49
- 段落行数：103 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts

### React 集成/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：72
- 页面类型：other / module
- 行数：288 / 49
- 段落行数：104 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：shallow.ts
- reference 关键文件未覆盖：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts

### React 集成/框架集成实践.md

- reference 标题：框架集成实践
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 页面类型：other / module
- 行数：352 / 49
- 段落行数：139 / 6
- Evidence：21 / 1
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js

### React 集成/绑定 Store 使用.md

- reference 标题：绑定 Store 使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：other / module
- 行数：249 / 49
- 段落行数：103 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts

### TypeScript 支持/TypeScript 支持.md

- reference 标题：TypeScript 支持
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：120
- 页面类型：other / module
- 行数：347 / 49
- 段落行数：150 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts

### TypeScript 支持/基础类型使用.md

- reference 标题：基础类型使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：192
- 页面类型：other / module
- 行数：397 / 49
- 段落行数：138 / 6
- Evidence：19 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts

### TypeScript 支持/第三方库集成.md

- reference 标题：第三方库集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：321 / 49
- 段落行数：122 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、immer.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、immer.ts、persist.ts

### TypeScript 支持/自动类型推断.md

- reference 标题：自动类型推断
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：152
- 页面类型：other / module
- 行数：334 / 49
- 段落行数：124 / 6
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts

### TypeScript 支持/高级类型模式.md

- reference 标题：高级类型模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：160
- 页面类型：other / module
- 行数：366 / 49
- 段落行数：175 / 6
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts

### 中间件系统/Immer 中间件.md

- reference 标题：Immer 中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/Redux 兼容性中间件.md

- reference 标题：Redux 兼容性中间件
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：78
- 页面类型：other / topic
- 行数：274 / 28
- 段落行数：180 / 7
- Evidence：14 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、redux.ts、immer.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、redux.md、vanilla.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、redux.md、vanilla.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts

### 中间件系统/SSR 安全中间件.md

- reference 标题：SSR 安全中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 页面类型：other / module
- 行数：250 / 49
- 段落行数：111 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js

### 中间件系统/中间件概述.md

- reference 标题：中间件概述
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：126
- 页面类型：other / topic
- 行数：328 / 28
- 段落行数：144 / 7
- Evidence：19 / 1
- Mermaid：8 / 0
- 文件提及重合：middleware.ts、combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：readme.md、devtools.md、persist.md、package.js、devtools.ts、persist.ts、subscribewithselector.ts、vanilla.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、persist.md、package.js、devtools.ts、persist.ts、subscribewithselector.ts、vanilla.ts

### 中间件系统/中间件系统.md

- reference 标题：中间件系统
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：148
- 页面类型：other / topic
- 行数：331 / 28
- 段落行数：129 / 7
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：combine.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、subscribewithselector.ts

### 中间件系统/开发工具中间件.md

- reference 标题：开发工具中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/持久化中间件.md

- reference 标题：持久化中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/组合中间件.md

- reference 标题：组合中间件
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：84
- 页面类型：other / topic
- 行数：229 / 28
- 段落行数：79 / 7
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：middleware.ts、combine.ts
- reference 关键文件未覆盖：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、vanilla.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、vanilla.ts

### 中间件系统/选择器订阅中间件.md

- reference 标题：选择器订阅中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：108
- 页面类型：other / module
- 行数：294 / 49
- 段落行数：119 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、shallow.ts
- reference 关键文件未覆盖：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 页面类型：other / module
- 行数：209 / 49
- 段落行数：80 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js

### 性能优化/内存管理与泄漏防护.md

- reference 标题：内存管理与泄漏防护
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：136
- 页面类型：other / module
- 行数：288 / 49
- 段落行数：92 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts

### 性能优化/性能优化.md

- reference 标题：性能优化
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：128
- 页面类型：other / module
- 行数：283 / 49
- 段落行数：108 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts

### 性能优化/性能监控与分析.md

- reference 标题：性能监控与分析
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/浅比较优化策略.md

- reference 标题：浅比较优化策略
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：100
- 页面类型：other / module
- 行数：309 / 49
- 段落行数：121 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：shallow.ts、traditional.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts

### 性能优化/选择性订阅机制.md

- reference 标题：选择性订阅机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：108
- 页面类型：topic / module
- 行数：247 / 49
- 段落行数：105 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts

### 核心概念/React 集成设计.md

- reference 标题：React 集成设计
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：136
- 页面类型：other / module
- 行数：239 / 49
- 段落行数：85 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts

### 核心概念/不可变性原则.md

- reference 标题：不可变性原则
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：other / module
- 行数：284 / 49
- 段落行数：109 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、immer.ts、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、immer.ts、basic.test.ts

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：184
- 页面类型：other / module
- 行数：317 / 49
- 段落行数：158 / 6
- Evidence：20 / 1
- Mermaid：9 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts

### 核心概念/状态管理基础.md

- reference 标题：状态管理基础
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：68
- 页面类型：other / module
- 行数：272 / 49
- 段落行数：112 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、create.md、use-store.md、immer.md、app.js、immer.ts、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create.md、use-store.md、immer.md、app.js、immer.ts、basic.test.ts

### 核心概念/订阅机制.md

- reference 标题：订阅机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：156
- 页面类型：topic / module
- 行数：320 / 49
- 段落行数：124 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts

### 框架集成/Next.js 集成.md

- reference 标题：Next.js 集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：98
- 页面类型：other / module
- 行数：383 / 49
- 段落行数：285 / 6
- Evidence：21 / 1
- Mermaid：14 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts、index.ts
- reference 关键文件未覆盖：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts

### 框架集成/SSR 与水合.md

- reference 标题：SSR 与水合
- 生成页：无
- 问题：缺少对应生成页面

### 框架集成/URL 哈希同步.md

- reference 标题：URL 哈希同步
- 生成页：无
- 问题：缺少对应生成页面

### 框架集成/其他框架集成.md

- reference 标题：其他框架集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：114
- 页面类型：other / module
- 行数：296 / 49
- 段落行数：68 / 6
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts

### 框架集成/框架集成.md

- reference 标题：框架集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：323 / 49
- 段落行数：126 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、comparison.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、persist.md、migrating-to-v5.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、persist.md、migrating-to-v5.md

### 测试策略/中间件测试/React 组件测试.md

- reference 标题：React 组件测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/中间件测试/中间件测试.md

- reference 标题：中间件测试
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：100
- 页面类型：other / topic
- 行数：346 / 28
- 段落行数：213 / 7
- Evidence：18 / 1
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、devtools.test.ts

### 测试策略/中间件测试/中间件集成测试.md

- reference 标题：中间件集成测试
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：104
- 页面类型：other / topic
- 行数：360 / 28
- 段落行数：248 / 7
- Evidence：17 / 1
- Mermaid：8 / 0
- 文件提及重合：combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：package.js、devtools.ts、persist.ts、devtools.test.ts、middlewaretypes.test.ts、persistasync.test.ts、persistsync.test.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、devtools.ts、persist.ts、devtools.test.ts、middlewaretypes.test.ts、persistasync.test.ts、persistsync.test.ts、test-utils.ts

### 测试策略/中间件测试/基础 Store 测试.md

- reference 标题：基础 Store 测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：62
- 页面类型：other / module
- 行数：355 / 49
- 段落行数：247 / 6
- Evidence：21 / 1
- Mermaid：14 / 0
- 文件提及重合：vanilla.ts、index.ts
- reference 关键文件未覆盖：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf

### 测试策略/中间件测试/异步状态测试.md

- reference 标题：异步状态测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/中间件测试/性能测试.md

- reference 标题：性能测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：128
- 页面类型：other / module
- 行数：249 / 49
- 段落行数：106 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts
- reference 关键文件未覆盖：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf

### 测试策略/单元测试 Zustand Store/Store Mock 和重置机制.md

- reference 标题：Store Mock 和重置机制
- 生成页：专题/process/process-devtoolsimpl-flow-流程主题：devtoolsImpl-flow.md（流程主题：devtoolsImpl flow）
- 匹配分数：118
- 页面类型：topic / topic
- 行数：385 / 49
- 段落行数：266 / 28
- Evidence：19 / 1
- Mermaid：11 / 1
- 文件提及重合：index.ts
- reference 关键文件未覆盖：readme.md、testing.md、package.js、react.ts、vanilla.ts、basic.test.ts、setup.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、react.ts、vanilla.ts、basic.test.ts、setup.ts、test-utils.ts

### 测试策略/单元测试 Zustand Store/Store 直接测试.md

- reference 标题：Store 直接测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：64
- 页面类型：other / module
- 行数：335 / 49
- 段落行数：164 / 6
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：index.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts

### 测试策略/单元测试 Zustand Store/单元测试 Zustand Store.md

- reference 标题：单元测试 Zustand Store
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：92
- 页面类型：other / module
- 行数：362 / 49
- 段落行数：146 / 6
- Evidence：23 / 1
- Mermaid：9 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts

### 测试策略/单元测试 Zustand Store/测试最佳实践.md

- reference 标题：测试最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/单元测试 Zustand Store/测试环境配置.md

- reference 标题：测试环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/单元测试 Zustand Store/组件测试.md

- reference 标题：组件测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/异步测试/同步测试.md

- reference 标题：同步测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：other / module
- 行数：370 / 49
- 段落行数：150 / 6
- Evidence：19 / 1
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts

### 测试策略/异步测试/异步中间件测试.md

- reference 标题：异步中间件测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/异步测试/异步持久化测试.md

- reference 标题：异步持久化测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/异步测试/异步测试.md

- reference 标题：异步测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：313 / 49
- 段落行数：229 / 6
- Evidence：18 / 1
- Mermaid：10 / 0
- 文件提及重合：middleware.ts、index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts

### 测试策略/测试最佳实践.md

- reference 标题：测试最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/测试环境配置.md

- reference 标题：测试环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/测试策略.md

- reference 标题：测试策略
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/组件测试/Context Provider 测试.md

- reference 标题：Context Provider 测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：other / module
- 行数：289 / 49
- 段落行数：100 / 6
- Evidence：16 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、react.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts

### 测试策略/组件测试/Store 测试.md

- reference 标题：Store 测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/组件测试/异步状态测试.md

- reference 标题：异步状态测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/组件测试/测试环境配置.md

- reference 标题：测试环境配置
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/组件测试/组件测试.md

- reference 标题：组件测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/组件测试/组件集成测试.md

- reference 标题：组件集成测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：80
- 页面类型：other / module
- 行数：272 / 49
- 段落行数：98 / 6
- Evidence：15 / 1
- Mermaid：3 / 0
- 文件提及重合：react.ts、shallow.ts
- reference 关键文件未覆盖：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts

### 示例和教程/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：317 / 49
- 段落行数：129 / 6
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js

### 示例和教程/复杂应用示例.md

- reference 标题：复杂应用示例
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：333 / 49
- 段落行数：142 / 6
- Evidence：17 / 1
- Mermaid：9 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js

### 示例和教程/完整教程.md

- reference 标题：完整教程
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：118
- 页面类型：other / module
- 行数：280 / 49
- 段落行数：119 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts

### 示例和教程/故障排除指南.md

- reference 标题：故障排除指南
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：172
- 页面类型：other / module
- 行数：436 / 49
- 段落行数：140 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、devtools.ts、immer.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、devtools.ts、immer.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts

### 示例和教程/示例和教程.md

- reference 标题：示例和教程
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：335 / 49
- 段落行数：117 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts
- reference 关键文件未覆盖：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js

### 贡献和开发.md

- reference 标题：贡献和开发
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：90
- 页面类型：other / module
- 行数：337 / 49
- 段落行数：107 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts

### 迁移和升级/从 v3 迁移到 v4.md

- reference 标题：从 v3 迁移到 v4
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：64
- 页面类型：other / module
- 行数：239 / 49
- 段落行数：90 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts

### 迁移和升级/从 v4 迁移到 v5.md

- reference 标题：从 v4 迁移到 v5
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：166
- 页面类型：other / module
- 行数：244 / 49
- 段落行数：92 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts

### 迁移和升级/兼容性指南.md

- reference 标题：兼容性指南
- 生成页：无
- 问题：缺少对应生成页面

### 迁移和升级/迁移和升级.md

- reference 标题：迁移和升级
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：188
- 页面类型：other / module
- 行数：301 / 49
- 段落行数：98 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts

### 项目概述/与竞品对比.md

- reference 标题：与竞品对比
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 页面类型：overview / module
- 行数：310 / 49
- 段落行数：120 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、immer.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、immer.ts

### 项目概述/安装与设置.md

- reference 标题：安装与设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：116
- 页面类型：overview / overview
- 行数：270 / 55
- 段落行数：93 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js

### 项目概述/快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：156
- 页面类型：overview / module
- 行数：277 / 49
- 段落行数：116 / 6
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js

### 项目概述/架构概览.md

- reference 标题：架构概览
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：146
- 页面类型：overview / topic
- 行数：314 / 28
- 段落行数：131 / 7
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：readme.md、app.js、index.ts、package.js、devtools.ts、persist.ts、react.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、index.ts、package.js、devtools.ts、persist.ts、react.ts、types.d.ts

### 项目概述/核心概念概览/Store 设计架构.md

- reference 标题：Store 设计架构
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：192
- 页面类型：overview / module
- 行数：338 / 49
- 段落行数：154 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts

### 项目概述/核心概念概览/不可变状态更新.md

- reference 标题：不可变状态更新
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：overview / overview
- 行数：282 / 55
- 段落行数：110 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts

### 项目概述/核心概念概览/中间件系统概念.md

- reference 标题：中间件系统概念
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：146
- 页面类型：overview / topic
- 行数：327 / 28
- 段落行数：148 / 7
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：index.ts、devtools.ts、persist.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、vanilla.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.ts、devtools.ts、persist.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、vanilla.ts、devtools.test.ts

### 项目概述/核心概念概览/核心概念概览.md

- reference 标题：核心概念概览
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：144
- 页面类型：overview / module
- 行数：303 / 49
- 段落行数：153 / 6
- Evidence：18 / 1
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts

### 项目概述/核心概念概览/状态管理基础.md

- reference 标题：状态管理基础
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 页面类型：overview / overview
- 行数：267 / 55
- 段落行数：108 / 5
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts、react.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts、react.ts

### 项目概述/核心概念概览/选择性订阅机制.md

- reference 标题：选择性订阅机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：100
- 页面类型：overview / module
- 行数：296 / 49
- 段落行数：121 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：react.ts、shallow.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、shallow.test.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：354
- 页面类型：overview / overview
- 行数：279 / 55
- 段落行数：98 / 5
- Evidence：17 / 0
- Mermaid：6 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts

### 高级用法/Slices 模式.md

- reference 标题：Slices 模式
- 生成页：无
- 问题：缺少对应生成页面

### 高级用法/传统模式.md

- reference 标题：传统模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：156
- 页面类型：other / module
- 行数：353 / 49
- 段落行数：131 / 6
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts

### 高级用法/复杂状态管理.md

- reference 标题：复杂状态管理
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：170
- 页面类型：other / module
- 行数：244 / 49
- 段落行数：76 / 6
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、devtools.md、immer.md、persist.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、package.js

### 高级用法/自定义中间件开发.md

- reference 标题：自定义中间件开发
- 生成页：专题/module-capability/module-dc9cc1e5ef17-middleware-src能力：中间件.md（src能力：中间件）
- 匹配分数：128
- 页面类型：other / topic
- 行数：293 / 28
- 段落行数：138 / 7
- Evidence：24 / 1
- Mermaid：10 / 0
- 文件提及重合：middleware.ts、combine.ts、immer.ts、redux.ts
- reference 关键文件未覆盖：combine.md、devtools.md、persist.md、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts、vanilla.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、persist.md、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts、vanilla.ts

### 高级用法/高级用法.md

- reference 标题：高级用法
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：172
- 页面类型：other / module
- 行数：371 / 49
- 段落行数：147 / 6
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、immer.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、immer.ts

