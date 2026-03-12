# chi Reference 对比报告

生成页面：16 页
reference 页面：65 页
命中对比：56 页
缺失对比：9 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=18, total_tokens=113264, page_research=12, page_enrichment=2

## 覆盖统计

- 专题页覆盖：generated 11 / reference 6（repo-archetype=2）
- evidence 落页：generated 14 / reference 65
- citation 密度：generated 1 / reference 73.57
- 图表达覆盖：generated 2 / reference 65
- page research 请求：12
- page enrichment 请求：2
- 已规划专题类型：流程主题(6)、专题页(3)、中间件主题(1)、核心机制主题(1)
- 高频缺失专题：无

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- evidence block 已进入页面，但覆盖率和密度仍低于 reference
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 项目概述.md 被 43 个 reference 页面共享映射
- 专题/root-mechanism/root-core-根级核心机制.md 被 9 个 reference 页面共享映射
- 系统架构.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-90018151b0b9-entry-_examples能力：入口编排.md (_examples能力：入口编排, 24 行)
- 专题/module-capability/module-fb4109464cc5-middleware-middleware能力：中间件.md (middleware能力：中间件, 24 行)
- 专题/process/process-handlefunc-flow-流程主题：HandleFunc-flow.md (流程主题：HandleFunc flow, 39 行)
- 专题/process/process-handler-flow-流程主题：Handler-flow.md (流程主题：Handler flow, 41 行)
- 专题/process/process-handlerfunc-flow-流程主题：HandlerFunc-flow.md (流程主题：HandlerFunc flow, 25 行)
- 专题/process/process-listarticles-flow-流程主题：ListArticles-flow.md (流程主题：ListArticles flow, 73 行)
- 专题/process/process-testmuxhandlepatternvalidation-flow-流程主题：TestMuxHandlePatternValidation-flow.md (流程主题：TestMuxHandlePatternValidation flow, 25 行)
- 专题/process/process-testsinglehandler-flow-流程主题：TestSingleHandler-flow.md (流程主题：TestSingleHandler flow, 44 行)
- 专题/repo-archetype/config-runtime-配置与运行时.md (配置与运行时, 19 行)
- 专题/repo-archetype/request-lifecycle-请求处理链.md (请求处理链, 26 行)
- 核心模块/_examples.md (模块：_examples, 54 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 项目概述.md | 345/78 | 157/8 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、context.go、middleware.go |
| API 参考/上下文 API.md | 项目概述.md | 406/78 | 143/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、request_id.go、value.go |
| API 参考/中间件 API.md | 项目概述.md | 296/78 | 116/8 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go |
| API 参考/路由器接口.md | 项目概述.md | 396/78 | 190/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go |
| API 参考/路由遍历接口.md | 项目概述.md | 274/78 | 100/8 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go |
| API 参考/辅助函数.md | 项目概述.md | 375/78 | 158/8 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go |
| 上下文管理.md | 项目概述.md | 411/78 | 176/8 | 20/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go |
| 中间件系统/中间件基础概念.md | 项目概述.md | 339/78 | 234/8 | 16/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go |
| 中间件系统/中间件系统.md | 项目概述.md | 380/78 | 105/8 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go |
| 中间件系统/中间件链机制.md | 专题/root-mechanism/root-core-根级核心机制.md | 323/23 | 140/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go |
| 中间件系统/内置中间件详解/HTTP 协议增强中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/内置中间件详解.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/响应控制中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/安全防护中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/性能优化中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/日志记录中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/请求追踪中间件.md | 项目概述.md | 415/78 | 284/8 | 17/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、request_id.go、request_id_test.go、value.go、wrap_writer.go |
| 中间件系统/内置中间件详解/路径处理中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/辅助工具中间件.md | 核心模块/middleware.md | 541/54 | 362/7 | 21/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go |
| 中间件系统/内置中间件详解/错误处理中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/自定义中间件开发.md | 项目概述.md | 351/78 | 121/8 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go |
| 快速开始.md | 项目概述.md | 289/78 | 108/8 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go |
| 核心架构/上下文管理系统.md | 系统架构.md | 313/35 | 123/4 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go |
| 核心架构/中间件链执行机制.md | 系统架构.md | 313/35 | 150/4 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go |
| 核心架构/基数树路由算法.md | 项目概述.md | 304/78 | 153/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 核心架构/核心架构.md | 项目概述.md | 329/78 | 145/8 | 14/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go |
| 核心架构/组件交互关系.md | 项目概述.md | 365/78 | 171/8 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go |
| 核心架构/路由器接口设计.md | 项目概述.md | 444/78 | 318/8 | 17/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go |
| 测试与部署.md | 工作流与部署.md | 238/52 | 58/5 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go |
| 示例教程/中间件示例.md | 项目概述.md | 325/78 | 143/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go |
| 示例教程/基础示例.md | 项目概述.md | 269/78 | 100/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、logger.go、recoverer.go |
| 示例教程/示例教程.md | 项目概述.md | 349/78 | 122/8 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md |
| 示例教程/进阶示例.md | 项目概述.md | 373/78 | 162/8 | 16/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：routes.md、todos.go、users.go、article.go、context.go、middleware.go |
| 贡献指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 路由系统/上下文管理/Context结构设计.md | 项目概述.md | 317/78 | 208/8 | 13/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go |
| 路由系统/上下文管理/URL参数提取.md | 项目概述.md | 317/78 | 161/8 | 15/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 路由系统/上下文管理/上下文生命周期管理.md | 项目概述.md | 309/78 | 161/8 | 15/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、get_head.go、mux_test.go |
| 路由系统/上下文管理/上下文管理.md | 项目概述.md | 320/78 | 130/8 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go |
| 路由系统/上下文管理/上下文键值系统.md | 项目概述.md | 328/78 | 140/8 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go |
| 路由系统/基数树算法实现/基数树算法实现.md | 专题/root-mechanism/root-core-根级核心机制.md | 313/23 | 149/6 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、tree_test.go |
| 路由系统/基数树算法实现/插入算法实现.md | 项目概述.md | 375/78 | 283/8 | 16/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 路由系统/基数树算法实现/查找算法实现.md | 项目概述.md | 376/78 | 277/8 | 15/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 路由系统/基数树算法实现/树遍历和遍历算法.md | 项目概述.md | 321/78 | 143/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 路由系统/基数树算法实现/节点结构设计.md | 专题/root-mechanism/root-core-根级核心机制.md | 322/23 | 197/6 | 15/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 路由系统/基数树算法实现/路由模式解析.md | 专题/root-mechanism/root-core-根级核心机制.md | 357/23 | 229/6 | 16/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/子路由器系统/上下文传递机制.md | 专题/root-mechanism/root-core-根级核心机制.md | 313/23 | 209/6 | 19/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、context_test.go、logger.go、mux_test.go |
| 路由系统/子路由器系统/子路由器创建与配置.md | 项目概述.md | 260/78 | 103/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 路由系统/子路由器系统/子路由器系统.md | 项目概述.md | 315/78 | 158/8 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、middleware.go、mux_test.go |
| 路由系统/子路由器系统/嵌套设计模式.md | 项目概述.md | 313/78 | 121/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、mux_test.go |
| 路由系统/子路由器系统/继承行为与覆盖规则.md | 项目概述.md | 328/78 | 112/8 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、middleware.go、mux_test.go |
| 路由系统/路由匹配机制/动态参数提取.md | 专题/root-mechanism/root-core-根级核心机制.md | 291/23 | 108/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由优先级规则.md | 专题/root-mechanism/root-core-根级核心机制.md | 250/23 | 114/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由匹配机制.md | 专题/root-mechanism/root-core-根级核心机制.md | 277/23 | 107/6 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由匹配流程.md | 专题/root-mechanism/root-core-根级核心机制.md | 320/23 | 107/6 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、mux_test.go、pattern_test.go |
| 路由系统/路由模式语法.md | 项目概述.md | 253/78 | 88/8 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由系统.md | 项目概述.md | 374/78 | 153/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go |
| 项目概述.md | 项目概述.md | 275/78 | 143/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go |
| 高级特性/上下文高级用法.md | 项目概述.md | 306/78 | 115/8 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go |
| 高级特性/中间件组合技巧.md | 项目概述.md | 433/78 | 208/8 | 0/1 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go |
| 高级特性/复杂路由模式.md | 项目概述.md | 309/78 | 125/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go |
| 高级特性/子路由器高级配置.md | 项目概述.md | 328/78 | 157/8 | 0/1 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go |
| 高级特性/性能优化策略.md | 项目概述.md | 294/78 | 115/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go |
| 高级特性/自定义 HTTP 方法.md | 项目概述.md | 266/78 | 104/8 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 高级特性/路由遍历与文档生成.md | 项目概述.md | 254/78 | 101/8 | 20/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 高级特性/高级特性.md | 项目概述.md | 435/78 | 187/8 | 26/1 | 11/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：项目概述.md（项目概述）
- 匹配分数：170
- 页面类型：other / overview
- 行数：345 / 78
- 段落行数：157 / 8
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：changelog.md、readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、context.go、middleware.go

### API 参考/上下文 API.md

- reference 标题：上下文 API
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：other / overview
- 行数：406 / 78
- 段落行数：143 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、context_test.go、logger.go、request_id.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、request_id.go、value.go

### API 参考/中间件 API.md

- reference 标题：中间件 API
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 页面类型：other / overview
- 行数：296 / 78
- 段落行数：116 / 8
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go

### API 参考/路由器接口.md

- reference 标题：路由器接口
- 生成页：项目概述.md（项目概述）
- 匹配分数：170
- 页面类型：other / overview
- 行数：396 / 78
- 段落行数：190 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go

### API 参考/路由遍历接口.md

- reference 标题：路由遍历接口
- 生成页：项目概述.md（项目概述）
- 匹配分数：154
- 页面类型：other / overview
- 行数：274 / 78
- 段落行数：100 / 8
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go

### API 参考/辅助函数.md

- reference 标题：辅助函数
- 生成页：项目概述.md（项目概述）
- 匹配分数：206
- 页面类型：other / overview
- 行数：375 / 78
- 段落行数：158 / 8
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go

### 上下文管理.md

- reference 标题：上下文管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：162
- 页面类型：other / overview
- 行数：411 / 78
- 段落行数：176 / 8
- Evidence：20 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go

### 中间件系统/中间件基础概念.md

- reference 标题：中间件基础概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：other / overview
- 行数：339 / 78
- 段落行数：234 / 8
- Evidence：16 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go

### 中间件系统/中间件系统.md

- reference 标题：中间件系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：124
- 页面类型：other / overview
- 行数：380 / 78
- 段落行数：105 / 8
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go

### 中间件系统/中间件链机制.md

- reference 标题：中间件链机制
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：180
- 页面类型：topic / topic
- 行数：323 / 23
- 段落行数：140 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：chain.go、chi.go、mux.go
- reference 关键文件未覆盖：main.go、basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go

### 中间件系统/内置中间件详解/HTTP 协议增强中间件.md

- reference 标题：HTTP 协议增强中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/内置中间件详解.md

- reference 标题：内置中间件详解
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/响应控制中间件.md

- reference 标题：响应控制中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/安全防护中间件.md

- reference 标题：安全防护中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/性能优化中间件.md

- reference 标题：性能优化中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/日志记录中间件.md

- reference 标题：日志记录中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/请求追踪中间件.md

- reference 标题：请求追踪中间件
- 生成页：项目概述.md（项目概述）
- 匹配分数：74
- 页面类型：other / overview
- 行数：415 / 78
- 段落行数：284 / 8
- Evidence：17 / 1
- Mermaid：9 / 0
- 文件提及重合：chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、request_id.go、request_id_test.go、value.go、wrap_writer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、request_id.go、request_id_test.go、value.go、wrap_writer.go

### 中间件系统/内置中间件详解/路径处理中间件.md

- reference 标题：路径处理中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/辅助工具中间件.md

- reference 标题：辅助工具中间件
- 生成页：核心模块/middleware.md（模块：middleware）
- 匹配分数：96
- 页面类型：other / module
- 行数：541 / 54
- 段落行数：362 / 7
- Evidence：21 / 1
- Mermaid：8 / 0
- 文件提及重合：logger.go、maybe.go、value.go
- reference 关键文件未覆盖：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go

### 中间件系统/内置中间件详解/错误处理中间件.md

- reference 标题：错误处理中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/自定义中间件开发.md

- reference 标题：自定义中间件开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 页面类型：other / overview
- 行数：351 / 78
- 段落行数：121 / 8
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go

### 快速开始.md

- reference 标题：快速开始
- 生成页：项目概述.md（项目概述）
- 匹配分数：180
- 页面类型：other / overview
- 行数：289 / 78
- 段落行数：108 / 8
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go

### 核心架构/上下文管理系统.md

- reference 标题：上下文管理系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：148
- 页面类型：architecture / architecture
- 行数：313 / 35
- 段落行数：123 / 4
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go

### 核心架构/中间件链执行机制.md

- reference 标题：中间件链执行机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：138
- 页面类型：architecture / architecture
- 行数：313 / 35
- 段落行数：150 / 4
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go

### 核心架构/基数树路由算法.md

- reference 标题：基数树路由算法
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 页面类型：architecture / overview
- 行数：304 / 78
- 段落行数：153 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 核心架构/核心架构.md

- reference 标题：核心架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：166
- 页面类型：architecture / overview
- 行数：329 / 78
- 段落行数：145 / 8
- Evidence：14 / 1
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go

### 核心架构/组件交互关系.md

- reference 标题：组件交互关系
- 生成页：项目概述.md（项目概述）
- 匹配分数：188
- 页面类型：architecture / overview
- 行数：365 / 78
- 段落行数：171 / 8
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go

### 核心架构/路由器接口设计.md

- reference 标题：路由器接口设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：152
- 页面类型：architecture / overview
- 行数：444 / 78
- 段落行数：318 / 8
- Evidence：17 / 1
- Mermaid：9 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go

### 测试与部署.md

- reference 标题：测试与部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：238 / 52
- 段落行数：58 / 5
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go

### 示例教程/中间件示例.md

- reference 标题：中间件示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：86
- 页面类型：other / overview
- 行数：325 / 78
- 段落行数：143 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go

### 示例教程/基础示例.md

- reference 标题：基础示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：162
- 页面类型：other / overview
- 行数：269 / 78
- 段落行数：100 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、logger.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、logger.go、recoverer.go

### 示例教程/示例教程.md

- reference 标题：示例教程
- 生成页：项目概述.md（项目概述）
- 匹配分数：252
- 页面类型：other / overview
- 行数：349 / 78
- 段落行数：122 / 8
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md

### 示例教程/进阶示例.md

- reference 标题：进阶示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：other / overview
- 行数：373 / 78
- 段落行数：162 / 8
- Evidence：16 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：routes.md、todos.go、users.go、article.go、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：routes.md、todos.go、users.go、article.go、context.go、middleware.go

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 路由系统/上下文管理/Context结构设计.md

- reference 标题：Context结构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 页面类型：other / overview
- 行数：317 / 78
- 段落行数：208 / 8
- Evidence：13 / 1
- Mermaid：8 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go

### 路由系统/上下文管理/URL参数提取.md

- reference 标题：URL参数提取
- 生成页：项目概述.md（项目概述）
- 匹配分数：108
- 页面类型：other / overview
- 行数：317 / 78
- 段落行数：161 / 8
- Evidence：15 / 1
- Mermaid：5 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 路由系统/上下文管理/上下文生命周期管理.md

- reference 标题：上下文生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：136
- 页面类型：other / overview
- 行数：309 / 78
- 段落行数：161 / 8
- Evidence：15 / 1
- Mermaid：5 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、get_head.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、get_head.go、mux_test.go

### 路由系统/上下文管理/上下文管理.md

- reference 标题：上下文管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 页面类型：other / overview
- 行数：320 / 78
- 段落行数：130 / 8
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go

### 路由系统/上下文管理/上下文键值系统.md

- reference 标题：上下文键值系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：other / overview
- 行数：328 / 78
- 段落行数：140 / 8
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go

### 路由系统/基数树算法实现/基数树算法实现.md

- reference 标题：基数树算法实现
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：130
- 页面类型：other / topic
- 行数：313 / 23
- 段落行数：149 / 6
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、tree_test.go

### 路由系统/基数树算法实现/插入算法实现.md

- reference 标题：插入算法实现
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 页面类型：other / overview
- 行数：375 / 78
- 段落行数：283 / 8
- Evidence：16 / 1
- Mermaid：9 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 路由系统/基数树算法实现/查找算法实现.md

- reference 标题：查找算法实现
- 生成页：项目概述.md（项目概述）
- 匹配分数：106
- 页面类型：other / overview
- 行数：376 / 78
- 段落行数：277 / 8
- Evidence：15 / 1
- Mermaid：7 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 路由系统/基数树算法实现/树遍历和遍历算法.md

- reference 标题：树遍历和遍历算法
- 生成页：项目概述.md（项目概述）
- 匹配分数：150
- 页面类型：other / overview
- 行数：321 / 78
- 段落行数：143 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 路由系统/基数树算法实现/节点结构设计.md

- reference 标题：节点结构设计
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：94
- 页面类型：other / topic
- 行数：322 / 23
- 段落行数：197 / 6
- Evidence：15 / 1
- Mermaid：6 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 路由系统/基数树算法实现/路由模式解析.md

- reference 标题：路由模式解析
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：66
- 页面类型：other / topic
- 行数：357 / 23
- 段落行数：229 / 6
- Evidence：16 / 1
- Mermaid：9 / 0
- 文件提及重合：mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/子路由器系统/上下文传递机制.md

- reference 标题：上下文传递机制
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：206
- 页面类型：topic / topic
- 行数：313 / 23
- 段落行数：209 / 6
- Evidence：19 / 1
- Mermaid：9 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：main.go、context.go、context_test.go、logger.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、context_test.go、logger.go、mux_test.go

### 路由系统/子路由器系统/子路由器创建与配置.md

- reference 标题：子路由器创建与配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：152
- 页面类型：other / overview
- 行数：260 / 78
- 段落行数：103 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 路由系统/子路由器系统/子路由器系统.md

- reference 标题：子路由器系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：166
- 页面类型：other / overview
- 行数：315 / 78
- 段落行数：158 / 8
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、middleware.go、mux_test.go

### 路由系统/子路由器系统/嵌套设计模式.md

- reference 标题：嵌套设计模式
- 生成页：项目概述.md（项目概述）
- 匹配分数：182
- 页面类型：other / overview
- 行数：313 / 78
- 段落行数：121 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、mux_test.go

### 路由系统/子路由器系统/继承行为与覆盖规则.md

- reference 标题：继承行为与覆盖规则
- 生成页：项目概述.md（项目概述）
- 匹配分数：128
- 页面类型：other / overview
- 行数：328 / 78
- 段落行数：112 / 8
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、middleware.go、mux_test.go

### 路由系统/路由匹配机制/动态参数提取.md

- reference 标题：动态参数提取
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：178
- 页面类型：topic / topic
- 行数：291 / 23
- 段落行数：108 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：main.go、context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由优先级规则.md

- reference 标题：路由优先级规则
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：178
- 页面类型：topic / topic
- 行数：250 / 23
- 段落行数：114 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由匹配机制.md

- reference 标题：路由匹配机制
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：202
- 页面类型：topic / topic
- 行数：277 / 23
- 段落行数：107 / 6
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：main.go、context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由匹配流程.md

- reference 标题：路由匹配流程
- 生成页：专题/root-mechanism/root-core-根级核心机制.md（根级核心机制）
- 匹配分数：206
- 页面类型：topic / topic
- 行数：320 / 23
- 段落行数：107 / 6
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：main.go、context.go、mux_test.go、pattern_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、context.go、mux_test.go、pattern_test.go

### 路由系统/路由模式语法.md

- reference 标题：路由模式语法
- 生成页：项目概述.md（项目概述）
- 匹配分数：142
- 页面类型：other / overview
- 行数：253 / 78
- 段落行数：88 / 8
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/路由系统.md

- reference 标题：路由系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：158
- 页面类型：other / overview
- 行数：374 / 78
- 段落行数：153 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：762
- 页面类型：overview / overview
- 行数：275 / 78
- 段落行数：143 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go

### 高级特性/上下文高级用法.md

- reference 标题：上下文高级用法
- 生成页：项目概述.md（项目概述）
- 匹配分数：118
- 页面类型：other / overview
- 行数：306 / 78
- 段落行数：115 / 8
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go

### 高级特性/中间件组合技巧.md

- reference 标题：中间件组合技巧
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 页面类型：other / overview
- 行数：433 / 78
- 段落行数：208 / 8
- Evidence：0 / 1
- Mermaid：12 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go

### 高级特性/复杂路由模式.md

- reference 标题：复杂路由模式
- 生成页：项目概述.md（项目概述）
- 匹配分数：204
- 页面类型：other / overview
- 行数：309 / 78
- 段落行数：125 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go

### 高级特性/子路由器高级配置.md

- reference 标题：子路由器高级配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：162
- 页面类型：other / overview
- 行数：328 / 78
- 段落行数：157 / 8
- Evidence：0 / 1
- Mermaid：10 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go

### 高级特性/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：项目概述.md（项目概述）
- 匹配分数：176
- 页面类型：other / overview
- 行数：294 / 78
- 段落行数：115 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go

### 高级特性/自定义 HTTP 方法.md

- reference 标题：自定义 HTTP 方法
- 生成页：项目概述.md（项目概述）
- 匹配分数：164
- 页面类型：other / overview
- 行数：266 / 78
- 段落行数：104 / 8
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 高级特性/路由遍历与文档生成.md

- reference 标题：路由遍历与文档生成
- 生成页：项目概述.md（项目概述）
- 匹配分数：178
- 页面类型：other / overview
- 行数：254 / 78
- 段落行数：101 / 8
- Evidence：20 / 1
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 高级特性/高级特性.md

- reference 标题：高级特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：152
- 页面类型：other / overview
- 行数：435 / 78
- 段落行数：187 / 8
- Evidence：26 / 1
- Mermaid：11 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go

