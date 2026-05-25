# zustand Reference 对比报告

生成页面：4 页
reference 页面：97 页
命中对比：80 页
缺失对比：17 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/src.md 被 76 个 reference 页面共享映射
- 项目概述.md 被 4 个 reference 页面共享映射

## 额外生成页面

- 工作流与部署.md (工作流与部署, 34 行)
- 系统架构.md (系统架构, 33 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/src.md | 292/48 | 100/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md |
| API 参考/React Hooks/React Hooks.md | 核心模块/src.md | 325/48 | 128/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts |
| API 参考/React Hooks/useShallow Hook.md | 核心模块/src.md | 277/48 | 102/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts |
| API 参考/React Hooks/useStore Hook.md | 核心模块/src.md | 232/48 | 90/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：use-store-with-equality-fn.md、use-store.md、basic.test.ts |
| API 参考/React Hooks/绑定 Store 模式.md | 核心模块/src.md | 299/48 | 125/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts |
| API 参考/核心 API/StateCreator 类型.md | 核心模块/src.md | 286/48 | 66/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts |
| API 参考/核心 API/create 函数.md | 核心模块/src.md | 314/48 | 95/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts |
| API 参考/核心 API/createStore 函数.md | 核心模块/src.md | 273/48 | 107/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、combine.ts、basic.test.ts、subscribe.test.ts、node.js |
| API 参考/核心 API/核心 API.md | 核心模块/src.md | 309/48 | 116/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、basic.test.ts |
| API 参考/类型定义.md | 核心模块/src.md | 248/48 | 78/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts |
| React 集成/React 集成.md | 核心模块/src.md | 272/48 | 106/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts |
| React 集成/SSR 和水合支持.md | 核心模块/src.md | 237/48 | 104/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、create-store.md、persisting-store-data.md、package.js、ssrsafe.ts、ssr.test.ts、next.js |
| React 集成/useStore Hook 使用.md | 核心模块/src.md | 254/48 | 103/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts |
| React 集成/性能优化策略.md | 核心模块/src.md | 288/48 | 104/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts |
| React 集成/框架集成实践.md | 核心模块/src.md | 352/48 | 139/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js |
| React 集成/绑定 Store 使用.md | 核心模块/src.md | 249/48 | 103/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts |
| TypeScript 支持/TypeScript 支持.md | 核心模块/src.md | 347/48 | 150/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts |
| TypeScript 支持/基础类型使用.md | 核心模块/src.md | 397/48 | 138/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts |
| TypeScript 支持/第三方库集成.md | 核心模块/src.md | 321/48 | 122/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、persist.ts、redux.ts |
| TypeScript 支持/自动类型推断.md | 核心模块/src.md | 334/48 | 124/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts |
| TypeScript 支持/高级类型模式.md | 核心模块/src.md | 366/48 | 175/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts |
| 中间件系统/Immer 中间件.md | 核心模块/src.md | 222/48 | 93/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、advanced-typescript.md、immutable-state-and-merging.md、immer-middleware.md、immer.md、package.js |
| 中间件系统/Redux 兼容性中间件.md | 核心模块/src.md | 274/48 | 180/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、redux.md、redux.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts |
| 中间件系统/SSR 安全中间件.md | 核心模块/src.md | 250/48 | 111/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js |
| 中间件系统/中间件概述.md | 核心模块/src.md | 328/48 | 144/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、persist.md、package.js、combine.ts、devtools.ts、persist.ts、redux.ts |
| 中间件系统/中间件系统.md | 核心模块/src.md | 331/48 | 129/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、immer.md、persist.md、redux.md、combine.ts、devtools.ts、persist.ts |
| 中间件系统/开发工具中间件.md | 核心模块/src.md | 253/48 | 82/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：devtools.md、package.js、devtools.ts、devtools.test.ts |
| 中间件系统/持久化中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/组合中间件.md | 核心模块/src.md | 229/48 | 79/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、combine.ts |
| 中间件系统/选择器订阅中间件.md | 核心模块/src.md | 294/48 | 119/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts |
| 快速开始.md | 核心模块/src.md | 209/48 | 80/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js |
| 性能优化/内存管理与泄漏防护.md | 核心模块/src.md | 288/48 | 92/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts |
| 性能优化/性能优化.md | 核心模块/src.md | 283/48 | 108/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts |
| 性能优化/性能监控与分析.md | 核心模块/src.md | 315/48 | 103/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-store.md、create.md、use-shallow.md、devtools.md、package.js、devtools.ts |
| 性能优化/浅比较优化策略.md | 核心模块/src.md | 309/48 | 121/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts |
| 性能优化/选择性订阅机制.md | 核心模块/src.md | 247/48 | 105/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts |
| 核心概念/React 集成设计.md | 核心模块/src.md | 239/48 | 85/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts |
| 核心概念/不可变性原则.md | 核心模块/src.md | 284/48 | 109/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、basic.test.ts |
| 核心概念/核心概念.md | 核心模块/src.md | 317/48 | 158/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts |
| 核心概念/状态管理基础.md | 核心模块/src.md | 272/48 | 112/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create.md、use-store.md、immer.md、app.js、basic.test.ts |
| 核心概念/订阅机制.md | 核心模块/src.md | 320/48 | 124/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts |
| 框架集成/Next.js 集成.md | 核心模块/src.md | 383/48 | 285/11 | 14/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts |
| 框架集成/SSR 与水合.md | 核心模块/src.md | 300/48 | 106/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ssr-and-hydration.md、persisting-store-data.md、package.js、persist.ts、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js |
| 框架集成/URL 哈希同步.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 框架集成/其他框架集成.md | 核心模块/src.md | 296/48 | 68/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts |
| 框架集成/框架集成.md | 核心模块/src.md | 323/48 | 126/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、persist.md、migrating-to-v5.md |
| 测试策略/中间件测试/React 组件测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/中间件测试/中间件测试.md | 核心模块/src.md | 346/48 | 213/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、redux.ts |
| 测试策略/中间件测试/中间件集成测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/中间件测试/基础 Store 测试.md | 核心模块/src.md | 355/48 | 247/11 | 14/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf |
| 测试策略/中间件测试/异步状态测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/中间件测试/性能测试.md | 核心模块/src.md | 249/48 | 106/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf |
| 测试策略/单元测试 Zustand Store/Store Mock 和重置机制.md | 核心模块/src.md | 385/48 | 266/11 | 11/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、basic.test.ts、setup.ts、test-utils.ts、vitest.conf、zustand.ts |
| 测试策略/单元测试 Zustand Store/Store 直接测试.md | 核心模块/src.md | 335/48 | 164/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts |
| 测试策略/单元测试 Zustand Store/单元测试 Zustand Store.md | 核心模块/src.md | 362/48 | 146/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts |
| 测试策略/单元测试 Zustand Store/测试最佳实践.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/单元测试 Zustand Store/测试环境配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/单元测试 Zustand Store/组件测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/同步测试.md | 核心模块/src.md | 370/48 | 150/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts |
| 测试策略/异步测试/异步中间件测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/异步持久化测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/异步测试/异步测试.md | 核心模块/src.md | 313/48 | 229/11 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts |
| 测试策略/测试最佳实践.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/测试环境配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/测试策略.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/Context Provider 测试.md | 核心模块/src.md | 289/48 | 100/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts |
| 测试策略/组件测试/Store 测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/异步状态测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/测试环境配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/组件测试.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 测试策略/组件测试/组件集成测试.md | 核心模块/src.md | 272/48 | 98/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts |
| 示例和教程/基础示例.md | 核心模块/src.md | 317/48 | 129/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js |
| 示例和教程/复杂应用示例.md | 核心模块/src.md | 333/48 | 142/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js |
| 示例和教程/完整教程.md | 核心模块/src.md | 280/48 | 119/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts |
| 示例和教程/故障排除指南.md | 核心模块/src.md | 436/48 | 140/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、devtools.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts |
| 示例和教程/示例和教程.md | 核心模块/src.md | 335/48 | 117/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js |
| 贡献和开发.md | 核心模块/src.md | 337/48 | 107/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts |
| 迁移和升级/从 v3 迁移到 v4.md | 核心模块/src.md | 239/48 | 90/11 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts |
| 迁移和升级/从 v4 迁移到 v5.md | 核心模块/src.md | 244/48 | 92/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts |
| 迁移和升级/兼容性指南.md | 核心模块/src.md | 341/48 | 144/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：test-old-typescript.yml、readme.md、testing.md、third-party-libraries.md、migrating-to-v4.md、migrating-to-v5.md、zustand-v3-create-context.md、package.js |
| 迁移和升级/迁移和升级.md | 核心模块/src.md | 301/48 | 98/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts |
| 项目概述/与竞品对比.md | 核心模块/src.md | 310/48 | 120/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、persist.ts |
| 项目概述/安装与设置.md | 项目概述.md | 270/25 | 93/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js |
| 项目概述/快速开始.md | 核心模块/src.md | 277/48 | 116/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js |
| 项目概述/架构概览.md | 核心模块/src.md | 314/48 | 131/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、package.js、combine.ts、devtools.ts、persist.ts、redux.ts、types.d.ts |
| 项目概述/核心概念概览/Store 设计架构.md | 核心模块/src.md | 338/48 | 154/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts |
| 项目概述/核心概念概览/不可变状态更新.md | 项目概述.md | 282/25 | 110/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts |
| 项目概述/核心概念概览/中间件系统概念.md | 核心模块/src.md | 327/48 | 148/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、redux.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、devtools.test.ts |
| 项目概述/核心概念概览/核心概念概览.md | 核心模块/src.md | 303/48 | 153/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts |
| 项目概述/核心概念概览/状态管理基础.md | 核心模块/src.md | 267/48 | 108/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts |
| 项目概述/核心概念概览/选择性订阅机制.md | 项目概述.md | 296/25 | 121/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、react.ts、shallow.test.ts |
| 项目概述/项目概述.md | 项目概述.md | 279/25 | 98/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts |
| 高级用法/Slices 模式.md | 核心模块/src.md | 324/48 | 130/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、prevent-rerenders-with-use-shallow.md、slices-pattern.md、testing.md、use-store.md、combine.md、persist.md、combine.ts |
| 高级用法/传统模式.md | 核心模块/src.md | 353/48 | 131/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts |
| 高级用法/复杂状态管理.md | 核心模块/src.md | 244/48 | 76/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、package.js |
| 高级用法/自定义中间件开发.md | 核心模块/src.md | 293/48 | 138/11 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、persist.md、combine.ts、devtools.ts、persist.ts、redux.ts、subscribewithselector.ts |
| 高级用法/高级用法.md | 核心模块/src.md | 371/48 | 147/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、persist.ts |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：212
- 行数：292 / 48
- 段落行数：100 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、combine.md、persist.md、migrating-to-v4.md

### API 参考/React Hooks/React Hooks.md

- reference 标题：React Hooks
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 行数：325 / 48
- 段落行数：128 / 11
- Mermaid：7 / 0
- 文件提及重合：react.ts、shallow.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、subscribe-with-selector.md、basic.test.ts、shallow.test.ts、ssr.test.ts

### API 参考/React Hooks/useShallow Hook.md

- reference 标题：useShallow Hook
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 行数：277 / 48
- 段落行数：102 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、subscribewithselector.ts、shallow.test.ts

### API 参考/React Hooks/useStore Hook.md

- reference 标题：useStore Hook
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：150
- 行数：232 / 48
- 段落行数：90 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：use-store-with-equality-fn.md、use-store.md、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：use-store-with-equality-fn.md、use-store.md、basic.test.ts

### API 参考/React Hooks/绑定 Store 模式.md

- reference 标题：绑定 Store 模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：180
- 行数：299 / 48
- 段落行数：125 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：beginner-typescript.md、slices-pattern.md、create.md、use-store.md、basic.test.ts、middlewaretypes.test.ts

### API 参考/核心 API/StateCreator 类型.md

- reference 标题：StateCreator 类型
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 行数：286 / 48
- 段落行数：66 / 11
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create-store.md、create.md、combine.md、persist.md、combine.ts、persist.ts、types.test.ts

### API 参考/核心 API/create 函数.md

- reference 标题：create 函数
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：132
- 行数：314 / 48
- 段落行数：95 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、persist.md、basic.test.ts、types.test.ts

### API 参考/核心 API/createStore 函数.md

- reference 标题：createStore 函数
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：64
- 行数：273 / 48
- 段落行数：107 / 11
- Mermaid：4 / 0
- 文件提及重合：react.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create.md、combine.ts、basic.test.ts、subscribe.test.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、combine.ts、basic.test.ts、subscribe.test.ts、node.js

### API 参考/核心 API/核心 API.md

- reference 标题：核心 API
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：208
- 行数：309 / 48
- 段落行数：116 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：create-store.md、create.md、use-store.md、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-store.md、create.md、use-store.md、basic.test.ts

### API 参考/类型定义.md

- reference 标题：类型定义
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：232
- 行数：248 / 48
- 段落行数：78 / 11
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、middlewaretypes.test.ts、types.test.ts

### React 集成/React 集成.md

- reference 标题：React 集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：104
- 行数：272 / 48
- 段落行数：106 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts
- reference 关键文件未覆盖：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store-with-equality-fn.md、use-store.md、ssrsafe.ts、subscribewithselector.ts

### React 集成/SSR 和水合支持.md

- reference 标题：SSR 和水合支持
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 行数：237 / 48
- 段落行数：104 / 11
- Mermaid：6 / 0
- 文件提及重合：middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：nextjs.md、ssr-and-hydration.md、create-store.md、persisting-store-data.md、package.js、ssrsafe.ts、ssr.test.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、create-store.md、persisting-store-data.md、package.js、ssrsafe.ts、ssr.test.ts、next.js

### React 集成/useStore Hook 使用.md

- reference 标题：useStore Hook 使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：102
- 行数：254 / 48
- 段落行数：103 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、slices-pattern.md、use-store-with-equality-fn.md、use-store.md、basic.test.ts、shallow.test.ts

### React 集成/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：60
- 行数：288 / 48
- 段落行数：104 / 11
- Mermaid：5 / 0
- 文件提及重合：shallow.ts
- reference 关键文件未覆盖：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、use-store-with-equality-fn.md、shallow.test.ts

### React 集成/框架集成实践.md

- reference 标题：框架集成实践
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：130
- 行数：352 / 48
- 段落行数：139 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、connect-to-state-with-url-hash.md、nextjs.md、ssr-and-hydration.md、third-party-libraries.md、devtools.md、persist.md、app.js

### React 集成/绑定 Store 使用.md

- reference 标题：绑定 Store 使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：126
- 行数：249 / 48
- 段落行数：103 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create.md、use-store.md、app.js、types.d.ts、middlewaretypes.test.ts、persistasync.test.ts

### TypeScript 支持/TypeScript 支持.md

- reference 标题：TypeScript 支持
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：186
- 行数：347 / 48
- 段落行数：150 / 11
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、tsconfig.js、package.js、combine.ts、persist.ts、types.d.ts、types.test.ts

### TypeScript 支持/基础类型使用.md

- reference 标题：基础类型使用
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：276
- 行数：397 / 48
- 段落行数：138 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、beginner-typescript.md、create.md、typescript-code.js、types.d.ts、types.test.ts

### TypeScript 支持/第三方库集成.md

- reference 标题：第三方库集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：156
- 行数：321 / 48
- 段落行数：122 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、persist.ts、redux.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、third-party-libraries.md、devtools.md、immer.md、package.js、devtools.ts、persist.ts、redux.ts

### TypeScript 支持/自动类型推断.md

- reference 标题：自动类型推断
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：234
- 行数：334 / 48
- 段落行数：124 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、auto-generating-selectors.md、beginner-typescript.md、app.js、combine.ts、devtools.ts、subscribewithselector.ts、types.d.ts

### TypeScript 支持/高级类型模式.md

- reference 标题：高级类型模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：188
- 行数：366 / 48
- 段落行数：175 / 11
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、create.md、migrating-to-v4.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts、types.d.ts

### 中间件系统/Immer 中间件.md

- reference 标题：Immer 中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：62
- 行数：222 / 48
- 段落行数：93 / 11
- Mermaid：5 / 0
- 文件提及重合：immer.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、advanced-typescript.md、immutable-state-and-merging.md、immer-middleware.md、immer.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、advanced-typescript.md、immutable-state-and-merging.md、immer-middleware.md、immer.md、package.js

### 中间件系统/Redux 兼容性中间件.md

- reference 标题：Redux 兼容性中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 行数：274 / 48
- 段落行数：180 / 11
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、vanilla.ts、immer.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、redux.md、redux.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、redux.md、redux.ts、devtools.test.ts、middlewaretypes.test.ts、devtools.ts、persist.ts

### 中间件系统/SSR 安全中间件.md

- reference 标题：SSR 安全中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：112
- 行数：250 / 48
- 段落行数：111 / 11
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、persisting-store-data.md、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js

### 中间件系统/中间件概述.md

- reference 标题：中间件概述
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：114
- 行数：328 / 48
- 段落行数：144 / 11
- Mermaid：8 / 0
- 文件提及重合：middleware.ts、immer.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、devtools.md、persist.md、package.js、combine.ts、devtools.ts、persist.ts、redux.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、persist.md、package.js、combine.ts、devtools.ts、persist.ts、redux.ts

### 中间件系统/中间件系统.md

- reference 标题：中间件系统
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：134
- 行数：331 / 48
- 段落行数：129 / 11
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、immer.ts、vanilla.ts
- reference 关键文件未覆盖：combine.md、devtools.md、immer.md、persist.md、redux.md、combine.ts、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、immer.md、persist.md、redux.md、combine.ts、devtools.ts、persist.ts

### 中间件系统/开发工具中间件.md

- reference 标题：开发工具中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：86
- 行数：253 / 48
- 段落行数：82 / 11
- Mermaid：5 / 0
- 文件提及重合：middleware.ts、vanilla.ts
- reference 关键文件未覆盖：devtools.md、package.js、devtools.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：devtools.md、package.js、devtools.ts、devtools.test.ts

### 中间件系统/持久化中间件.md

- reference 标题：持久化中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/组合中间件.md

- reference 标题：组合中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：100
- 行数：229 / 48
- 段落行数：79 / 11
- Mermaid：5 / 0
- 文件提及重合：middleware.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、combine.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、slices-pattern.md、combine.md、devtools.md、combine.ts

### 中间件系统/选择器订阅中间件.md

- reference 标题：选择器订阅中间件
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：94
- 行数：294 / 48
- 段落行数：119 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、middleware.ts、shallow.ts
- reference 关键文件未覆盖：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：auto-generating-selectors.md、prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、shallow.test.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：94
- 行数：209 / 48
- 段落行数：80 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、advanced-typescript.md、beginner-typescript.md、create.md、use-store.md、app.js、package.js

### 性能优化/内存管理与泄漏防护.md

- reference 标题：内存管理与泄漏防护
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：160
- 行数：288 / 48
- 段落行数：92 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：maps-and-sets-usage.md、create.md、persist.md、app.js、persist.ts、subscribewithselector.ts、subscribe.test.ts

### 性能优化/性能优化.md

- reference 标题：性能优化
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：134
- 行数：283 / 48
- 段落行数：108 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、shallow.md、use-shallow.md、subscribe-with-selector.md、package.js、subscribewithselector.ts、shallow.test.ts

### 性能优化/性能监控与分析.md

- reference 标题：性能监控与分析
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：80
- 行数：315 / 48
- 段落行数：103 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、prevent-rerenders-with-use-shallow.md、create-store.md、create.md、use-shallow.md、devtools.md、package.js、devtools.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、create-store.md、create.md、use-shallow.md、devtools.md、package.js、devtools.ts

### 性能优化/浅比较优化策略.md

- reference 标题：浅比较优化策略
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：108
- 行数：309 / 48
- 段落行数：121 / 11
- Mermaid：6 / 0
- 文件提及重合：shallow.ts、traditional.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、shallow.test.ts

### 性能优化/选择性订阅机制.md

- reference 标题：选择性订阅机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：130
- 行数：247 / 48
- 段落行数：105 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、use-shallow.md、use-store.md、subscribe-with-selector.md、app.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts

### 核心概念/React 集成设计.md

- reference 标题：React 集成设计
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：124
- 行数：239 / 48
- 段落行数：85 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、prevent-rerenders-with-use-shallow.md、ssr-and-hydration.md、use-shallow.md、use-store.md、app.js、package.js、basic.test.ts

### 核心概念/不可变性原则.md

- reference 标题：不可变性原则
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：168
- 行数：284 / 48
- 段落行数：109 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、immer.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、create-store.md、shallow.md、immer.md、app.js、basic.test.ts

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：230
- 行数：317 / 48
- 段落行数：158 / 11
- Mermaid：9 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts

### 核心概念/状态管理基础.md

- reference 标题：状态管理基础
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：142
- 行数：272 / 48
- 段落行数：112 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、immer.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、create.md、use-store.md、immer.md、app.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create.md、use-store.md、immer.md、app.js、basic.test.ts

### 核心概念/订阅机制.md

- reference 标题：订阅机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：200
- 行数：320 / 48
- 段落行数：124 / 11
- Mermaid：7 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、use-shallow.md、use-store.md、subscribe-with-selector.md、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts

### 框架集成/Next.js 集成.md

- reference 标题：Next.js 集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：142
- 行数：383 / 48
- 段落行数：285 / 11
- Mermaid：14 / 0
- 文件提及重合：react.ts、shallow.ts、vanilla.ts、index.ts
- reference 关键文件未覆盖：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nextjs.md、ssr-and-hydration.md、use-shallow.md、use-store.md、ssr.test.ts、next.js、_app.ts、layout.ts

### 框架集成/SSR 与水合.md

- reference 标题：SSR 与水合
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：70
- 行数：300 / 48
- 段落行数：106 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、vanilla.ts
- reference 关键文件未覆盖：ssr-and-hydration.md、persisting-store-data.md、package.js、persist.ts、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：ssr-and-hydration.md、persisting-store-data.md、package.js、persist.ts、ssrsafe.ts、types.d.ts、ssr.test.ts、next.js

### 框架集成/URL 哈希同步.md

- reference 标题：URL 哈希同步
- 生成页：无
- 问题：缺少对应生成页面

### 框架集成/其他框架集成.md

- reference 标题：其他框架集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：124
- 行数：296 / 48
- 段落行数：68 / 11
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、persisting-store-data.md、third-party-libraries.md、package.js、rollup.conf、devtools.ts、persist.ts

### 框架集成/框架集成.md

- reference 标题：框架集成
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 行数：323 / 48
- 段落行数：126 / 11
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
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：68
- 行数：346 / 48
- 段落行数：213 / 11
- Mermaid：9 / 0
- 文件提及重合：middleware.ts、immer.ts
- reference 关键文件未覆盖：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、redux.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、redux.md、devtools.ts、persist.ts、redux.ts

### 测试策略/中间件测试/中间件集成测试.md

- reference 标题：中间件集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/中间件测试/基础 Store 测试.md

- reference 标题：基础 Store 测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：70
- 行数：355 / 48
- 段落行数：247 / 11
- Mermaid：14 / 0
- 文件提及重合：vanilla.ts、index.ts
- reference 关键文件未覆盖：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、subscribe.test.ts、test-utils.ts、vitest.conf

### 测试策略/中间件测试/异步状态测试.md

- reference 标题：异步状态测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/中间件测试/性能测试.md

- reference 标题：性能测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：116
- 行数：249 / 48
- 段落行数：106 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts
- reference 关键文件未覆盖：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、shallow.md、use-shallow.md、package.js、subscribewithselector.ts、shallow.test.ts、subscribe.test.ts、vitest.conf

### 测试策略/单元测试 Zustand Store/Store Mock 和重置机制.md

- reference 标题：Store Mock 和重置机制
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 行数：385 / 48
- 段落行数：266 / 11
- Mermaid：11 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、package.js、basic.test.ts、setup.ts、test-utils.ts、vitest.conf、zustand.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、basic.test.ts、setup.ts、test-utils.ts、vitest.conf、zustand.ts

### 测试策略/单元测试 Zustand Store/Store 直接测试.md

- reference 标题：Store 直接测试
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 行数：335 / 48
- 段落行数：164 / 11
- Mermaid：9 / 0
- 文件提及重合：index.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、subscribewithselector.ts、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts

### 测试策略/单元测试 Zustand Store/单元测试 Zustand Store.md

- reference 标题：单元测试 Zustand Store
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：138
- 行数：362 / 48
- 段落行数：146 / 11
- Mermaid：9 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：testing.md、package.js、basic.test.ts、devtools.test.ts、persistasync.test.ts、persistsync.test.ts、setup.ts、shallow.test.ts

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
- 匹配分数：124
- 行数：370 / 48
- 段落行数：150 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts、test-utils.ts

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
- 匹配分数：170
- 行数：313 / 48
- 段落行数：229 / 11
- Mermaid：10 / 0
- 文件提及重合：middleware.ts、index.ts、immer.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、persist.ts、basic.test.ts、persistasync.test.ts、persistsync.test.ts、subscribe.test.ts、test-utils.ts

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
- 匹配分数：144
- 行数：289 / 48
- 段落行数：100 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、react.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、initialize-state-with-props.md、testing.md、use-store-with-equality-fn.md、package.js、basic.test.ts、setup.ts、shallow.test.ts

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
- 匹配分数：70
- 行数：272 / 48
- 段落行数：98 / 11
- Mermaid：3 / 0
- 文件提及重合：react.ts、shallow.ts
- reference 关键文件未覆盖：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、package.js、subscribewithselector.ts、basic.test.ts、setup.ts、shallow.test.ts、subscribe.test.ts

### 示例和教程/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：98
- 行数：317 / 48
- 段落行数：129 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、slices-pattern.md、combine.md、immer.md、app.js、codepreview.js、javascript-code.js、typescript-code.js

### 示例和教程/复杂应用示例.md

- reference 标题：复杂应用示例
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：182
- 行数：333 / 48
- 段落行数：142 / 11
- Mermaid：9 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、codepreview.js、fireflies.js、scene.js、main.js、layermaterial.js、package.js

### 示例和教程/完整教程.md

- reference 标题：完整教程
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：182
- 行数：280 / 48
- 段落行数：119 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、testing.md、tutorial-tic-tac-toe.md、app.js、package.js、combine.ts、basic.test.ts

### 示例和教程/故障排除指南.md

- reference 标题：故障排除指南
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：288
- 行数：436 / 48
- 段落行数：140 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、package.js、devtools.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、devtools.ts、persist.ts、types.d.ts、basic.test.ts、devtools.test.ts

### 示例和教程/示例和教程.md

- reference 标题：示例和教程
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：100
- 行数：335 / 48
- 段落行数：117 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts
- reference 关键文件未覆盖：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、tutorial-tic-tac-toe.md、immer-middleware.md、persisting-store-data.md、app.js、codepreview.js、main.js、package.js

### 贡献和开发.md

- reference 标题：贡献和开发
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：102
- 行数：337 / 48
- 段落行数：107 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：pull_request_template.md、contributing.md、eslint.conf、package.js、pnpm-workspace.yaml、rollup.conf、types.d.ts、setup.ts

### 迁移和升级/从 v3 迁移到 v4.md

- reference 标题：从 v3 迁移到 v4
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：132
- 行数：239 / 48
- 段落行数：90 / 11
- Mermaid：4 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、migrating-to-v4.md、migrating-to-v5.md、app.js、package.js、combine.ts、devtools.ts、persist.ts

### 迁移和升级/从 v4 迁移到 v5.md

- reference 标题：从 v4 迁移到 v5
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：210
- 行数：244 / 48
- 段落行数：92 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、create-with-equality-fn.md、use-shallow.md、persisting-store-data.md、migrating-to-v5.md、package.js、persist.ts

### 迁移和升级/兼容性指南.md

- reference 标题：兼容性指南
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：98
- 行数：341 / 48
- 段落行数：144 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：test-old-typescript.yml、readme.md、testing.md、third-party-libraries.md、migrating-to-v4.md、migrating-to-v5.md、zustand-v3-create-context.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：test-old-typescript.yml、readme.md、testing.md、third-party-libraries.md、migrating-to-v4.md、migrating-to-v5.md、zustand-v3-create-context.md、package.js

### 迁移和升级/迁移和升级.md

- reference 标题：迁移和升级
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：178
- 行数：301 / 48
- 段落行数：98 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、index.md、migrating-to-v4.md、migrating-to-v5.md、package.js、devtools.ts、persist.ts

### 项目概述/与竞品对比.md

- reference 标题：与竞品对比
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：208
- 行数：310 / 48
- 段落行数：120 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、create.md、use-store.md、app.js、package.js、devtools.ts、persist.ts

### 项目概述/安装与设置.md

- reference 标题：安装与设置
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：270 / 25
- 段落行数：93 / 7
- Mermaid：5 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、introduction.md、nextjs.md、ssr-and-hydration.md、create.md、use-store.md、package.js

### 项目概述/快速开始.md

- reference 标题：快速开始
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：164
- 行数：277 / 48
- 段落行数：116 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、tutorial-tic-tac-toe.md、create-store.md、create.md、use-store.md、combine.md、package.js

### 项目概述/架构概览.md

- reference 标题：架构概览
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：208
- 行数：314 / 48
- 段落行数：131 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、app.js、package.js、combine.ts、devtools.ts、persist.ts、redux.ts、types.d.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、app.js、package.js、combine.ts、devtools.ts、persist.ts、redux.ts、types.d.ts

### 项目概述/核心概念概览/Store 设计架构.md

- reference 标题：Store 设计架构
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：238
- 行数：338 / 48
- 段落行数：154 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、subscribewithselector.ts、basic.test.ts、subscribe.test.ts

### 项目概述/核心概念概览/不可变状态更新.md

- reference 标题：不可变状态更新
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：282 / 25
- 段落行数：110 / 7
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：immutable-state-and-merging.md、updating-state.md、create-store.md、create-with-equality-fn.md、create.md、immer.md、immer.ts、react.ts

### 项目概述/核心概念概览/中间件系统概念.md

- reference 标题：中间件系统概念
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：160
- 行数：327 / 48
- 段落行数：148 / 11
- Mermaid：7 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、vanilla.ts
- reference 关键文件未覆盖：combine.ts、devtools.ts、persist.ts、redux.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、devtools.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.ts、devtools.ts、persist.ts、redux.ts、ssrsafe.ts、subscribewithselector.ts、types.d.ts、devtools.test.ts

### 项目概述/核心概念概览/核心概念概览.md

- reference 标题：核心概念概览
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：228
- 行数：303 / 48
- 段落行数：153 / 11
- Mermaid：7 / 0
- 文件提及重合：middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、flux-inspired-practice.md、create.md、combine.ts、devtools.ts、persist.ts、subscribewithselector.ts

### 项目概述/核心概念概览/状态管理基础.md

- reference 标题：状态管理基础
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：134
- 行数：267 / 48
- 段落行数：108 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、react.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、introduction.md、slices-pattern.md、create.md、combine.md、package.js、combine.ts

### 项目概述/核心概念概览/选择性订阅机制.md

- reference 标题：选择性订阅机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 行数：296 / 25
- 段落行数：121 / 7
- Mermaid：7 / 0
- 文件提及重合：shallow.ts
- reference 关键文件未覆盖：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、react.ts、shallow.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：prevent-rerenders-with-use-shallow.md、create-with-equality-fn.md、shallow.md、use-shallow.md、subscribewithselector.ts、react.ts、shallow.test.ts

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：366
- 行数：279 / 25
- 段落行数：98 / 7
- Mermaid：6 / 0
- 文件提及重合：index.ts
- reference 关键文件未覆盖：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、comparison.md、introduction.md、create.md、use-store.md、persist.md、package.js、middleware.ts

### 高级用法/Slices 模式.md

- reference 标题：Slices 模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：88
- 行数：324 / 48
- 段落行数：130 / 11
- Mermaid：8 / 0
- 文件提及重合：index.ts、react.ts、shallow.ts、vanilla.ts
- reference 关键文件未覆盖：advanced-typescript.md、prevent-rerenders-with-use-shallow.md、slices-pattern.md、testing.md、use-store.md、combine.md、persist.md、combine.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：advanced-typescript.md、prevent-rerenders-with-use-shallow.md、slices-pattern.md、testing.md、use-store.md、combine.md、persist.md、combine.ts

### 高级用法/传统模式.md

- reference 标题：传统模式
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：146
- 行数：353 / 48
- 段落行数：131 / 11
- Mermaid：6 / 0
- 文件提及重合：react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：create-with-equality-fn.md、create.md、use-store-with-equality-fn.md、use-store.md、migrating-to-v4.md、migrating-to-v5.md、package.js、basic.test.ts

### 高级用法/复杂状态管理.md

- reference 标题：复杂状态管理
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：256
- 行数：244 / 48
- 段落行数：76 / 11
- Mermaid：3 / 0
- 文件提及重合：index.ts、middleware.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、devtools.md、immer.md、persist.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、devtools.md、immer.md、persist.md、package.js

### 高级用法/自定义中间件开发.md

- reference 标题：自定义中间件开发
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：96
- 行数：293 / 48
- 段落行数：138 / 11
- Mermaid：10 / 0
- 文件提及重合：middleware.ts、immer.ts、vanilla.ts
- reference 关键文件未覆盖：combine.md、devtools.md、persist.md、combine.ts、devtools.ts、persist.ts、redux.ts、subscribewithselector.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：combine.md、devtools.md、persist.md、combine.ts、devtools.ts、persist.ts、redux.ts、subscribewithselector.ts

### 高级用法/高级用法.md

- reference 标题：高级用法
- 生成页：核心模块/src.md（模块：src）
- 匹配分数：286
- 行数：371 / 48
- 段落行数：147 / 11
- Mermaid：9 / 0
- 文件提及重合：index.ts、middleware.ts、immer.ts、react.ts、shallow.ts、traditional.ts、vanilla.ts
- reference 关键文件未覆盖：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、persist.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、flux-inspired-practice.md、slices-pattern.md、devtools.md、immer.md、persist.md、devtools.ts、persist.ts

