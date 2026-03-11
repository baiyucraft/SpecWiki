# chi Reference 对比报告

生成页面：5 页
reference 页面：65 页
命中对比：57 页
缺失对比：8 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 项目概述.md 被 49 个 reference 页面共享映射
- 核心模块/middleware.md 被 3 个 reference 页面共享映射
- 系统架构.md 被 3 个 reference 页面共享映射

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 项目概述.md | 345/40 | 157/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、context.go、middleware.go |
| API 参考/上下文 API.md | 项目概述.md | 406/40 | 143/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、request_id.go、value.go |
| API 参考/中间件 API.md | 项目概述.md | 296/40 | 116/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go |
| API 参考/路由器接口.md | 项目概述.md | 396/40 | 190/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go |
| API 参考/路由遍历接口.md | 项目概述.md | 274/40 | 100/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go |
| API 参考/辅助函数.md | 项目概述.md | 375/40 | 158/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go |
| 上下文管理.md | 项目概述.md | 411/40 | 176/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go |
| 中间件系统/中间件基础概念.md | 项目概述.md | 339/40 | 234/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go |
| 中间件系统/中间件系统.md | 项目概述.md | 380/40 | 105/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go |
| 中间件系统/中间件链机制.md | 项目概述.md | 323/40 | 140/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go、request_id.go |
| 中间件系统/内置中间件详解/HTTP 协议增强中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/内置中间件详解.md | 核心模块/middleware.md | 453/33 | 100/9 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：basic_auth.go、compress.go、content_type.go、get_head.go、heartbeat.go、middleware.go、nocache.go、path_rewrite.go |
| 中间件系统/内置中间件详解/响应控制中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/安全防护中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/性能优化中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/日志记录中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/请求追踪中间件.md | 核心模块/middleware.md | 415/33 | 284/9 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go、request_id.go、request_id_test.go、wrap_writer.go、chi.go、mux.go |
| 中间件系统/内置中间件详解/路径处理中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件详解/辅助工具中间件.md | 核心模块/middleware.md | 541/33 | 362/9 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go |
| 中间件系统/内置中间件详解/错误处理中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/自定义中间件开发.md | 项目概述.md | 351/40 | 121/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go |
| 快速开始.md | 项目概述.md | 289/40 | 108/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go |
| 核心架构/上下文管理系统.md | 系统架构.md | 313/28 | 123/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go |
| 核心架构/中间件链执行机制.md | 系统架构.md | 313/28 | 150/8 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go |
| 核心架构/基数树路由算法.md | 系统架构.md | 304/28 | 153/8 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、mux.go、pattern_test.go、tree.go、tree_test.go |
| 核心架构/核心架构.md | 项目概述.md | 329/40 | 145/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go |
| 核心架构/组件交互关系.md | 项目概述.md | 365/40 | 171/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go |
| 核心架构/路由器接口设计.md | 项目概述.md | 444/40 | 318/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go |
| 测试与部署.md | 工作流与部署.md | 238/32 | 58/8 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go |
| 示例教程/中间件示例.md | 项目概述.md | 325/40 | 143/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go |
| 示例教程/基础示例.md | 项目概述.md | 269/40 | 100/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、logger.go、recoverer.go |
| 示例教程/示例教程.md | 项目概述.md | 349/40 | 122/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md |
| 示例教程/进阶示例.md | 核心模块/_examples.md | 373/49 | 162/9 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routes.md、article.go、chi.go、context.go、middleware.go、mux.go |
| 贡献指南.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 路由系统/上下文管理/Context结构设计.md | 项目概述.md | 317/40 | 208/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go |
| 路由系统/上下文管理/URL参数提取.md | 项目概述.md | 317/40 | 161/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 路由系统/上下文管理/上下文生命周期管理.md | 项目概述.md | 309/40 | 161/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、get_head.go、mux_test.go |
| 路由系统/上下文管理/上下文管理.md | 项目概述.md | 320/40 | 130/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go |
| 路由系统/上下文管理/上下文键值系统.md | 项目概述.md | 328/40 | 140/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go |
| 路由系统/基数树算法实现/基数树算法实现.md | 项目概述.md | 313/40 | 149/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、tree_test.go |
| 路由系统/基数树算法实现/插入算法实现.md | 项目概述.md | 375/40 | 283/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 路由系统/基数树算法实现/查找算法实现.md | 项目概述.md | 376/40 | 277/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 路由系统/基数树算法实现/树遍历和遍历算法.md | 项目概述.md | 321/40 | 143/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 路由系统/基数树算法实现/节点结构设计.md | 项目概述.md | 322/40 | 197/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 路由系统/基数树算法实现/路由模式解析.md | 项目概述.md | 357/40 | 229/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/子路由器系统/上下文传递机制.md | 项目概述.md | 313/40 | 209/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、mux_test.go |
| 路由系统/子路由器系统/子路由器创建与配置.md | 项目概述.md | 260/40 | 103/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 路由系统/子路由器系统/子路由器系统.md | 项目概述.md | 315/40 | 158/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、middleware.go、mux_test.go |
| 路由系统/子路由器系统/嵌套设计模式.md | 项目概述.md | 313/40 | 121/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、mux_test.go |
| 路由系统/子路由器系统/继承行为与覆盖规则.md | 项目概述.md | 328/40 | 112/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、middleware.go、mux_test.go |
| 路由系统/路由匹配机制/动态参数提取.md | 项目概述.md | 291/40 | 108/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由优先级规则.md | 项目概述.md | 250/40 | 114/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由匹配机制.md | 项目概述.md | 277/40 | 107/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go |
| 路由系统/路由匹配机制/路由匹配流程.md | 项目概述.md | 320/40 | 107/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go |
| 路由系统/路由模式语法.md | 项目概述.md | 253/40 | 88/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go |
| 路由系统/路由系统.md | 项目概述.md | 374/40 | 153/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go |
| 项目概述.md | 项目概述.md | 275/40 | 143/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go |
| 高级特性/上下文高级用法.md | 项目概述.md | 306/40 | 115/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go |
| 高级特性/中间件组合技巧.md | 项目概述.md | 433/40 | 208/7 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go |
| 高级特性/复杂路由模式.md | 项目概述.md | 309/40 | 125/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go |
| 高级特性/子路由器高级配置.md | 项目概述.md | 328/40 | 157/7 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go |
| 高级特性/性能优化策略.md | 项目概述.md | 294/40 | 115/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go |
| 高级特性/自定义 HTTP 方法.md | 项目概述.md | 266/40 | 104/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go |
| 高级特性/路由遍历与文档生成.md | 项目概述.md | 254/40 | 101/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go |
| 高级特性/高级特性.md | 项目概述.md | 435/40 | 187/7 | 11/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：345 / 40
- 段落行数：157 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：changelog.md、readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：changelog.md、readme.md、context.go、middleware.go

### API 参考/上下文 API.md

- reference 标题：上下文 API
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：406 / 40
- 段落行数：143 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、context_test.go、logger.go、request_id.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、request_id.go、value.go

### API 参考/中间件 API.md

- reference 标题：中间件 API
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：296 / 40
- 段落行数：116 / 7
- Mermaid：5 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、timeout.go、value.go

### API 参考/路由器接口.md

- reference 标题：路由器接口
- 生成页：项目概述.md（项目概述）
- 匹配分数：150
- 行数：396 / 40
- 段落行数：190 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go

### API 参考/路由遍历接口.md

- reference 标题：路由遍历接口
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 行数：274 / 40
- 段落行数：100 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go

### API 参考/辅助函数.md

- reference 标题：辅助函数
- 生成页：项目概述.md（项目概述）
- 匹配分数：186
- 行数：375 / 40
- 段落行数：158 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go

### 上下文管理.md

- reference 标题：上下文管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：411 / 40
- 段落行数：176 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go、request_id.go

### 中间件系统/中间件基础概念.md

- reference 标题：中间件基础概念
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：339 / 40
- 段落行数：234 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、logger.go、middleware.go、recoverer.go、request_id.go

### 中间件系统/中间件系统.md

- reference 标题：中间件系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：380 / 40
- 段落行数：105 / 7
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、compress.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go

### 中间件系统/中间件链机制.md

- reference 标题：中间件链机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：144
- 行数：323 / 40
- 段落行数：140 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：basic_auth.go、logger.go、logger_test.go、middleware.go、middleware_test.go、recoverer.go、recoverer_test.go、request_id.go

### 中间件系统/内置中间件详解/HTTP 协议增强中间件.md

- reference 标题：HTTP 协议增强中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/内置中间件详解.md

- reference 标题：内置中间件详解
- 生成页：核心模块/middleware.md（模块：middleware）
- 匹配分数：150
- 行数：453 / 33
- 段落行数：100 / 9
- Mermaid：3 / 0
- 文件提及重合：logger.go、maybe.go、realip.go、strip.go、value.go
- reference 关键文件未覆盖：basic_auth.go、compress.go、content_type.go、get_head.go、heartbeat.go、middleware.go、nocache.go、path_rewrite.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：basic_auth.go、compress.go、content_type.go、get_head.go、heartbeat.go、middleware.go、nocache.go、path_rewrite.go

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
- 生成页：核心模块/middleware.md（模块：middleware）
- 匹配分数：70
- 行数：415 / 33
- 段落行数：284 / 9
- Mermaid：9 / 0
- 文件提及重合：logger.go、value.go
- reference 关键文件未覆盖：readme.md、context.go、middleware.go、request_id.go、request_id_test.go、wrap_writer.go、chi.go、mux.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go、request_id.go、request_id_test.go、wrap_writer.go、chi.go、mux.go

### 中间件系统/内置中间件详解/路径处理中间件.md

- reference 标题：路径处理中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件详解/辅助工具中间件.md

- reference 标题：辅助工具中间件
- 生成页：核心模块/middleware.md（模块：middleware）
- 匹配分数：104
- 行数：541 / 33
- 段落行数：362 / 9
- Mermaid：8 / 0
- 文件提及重合：logger.go、maybe.go、value.go
- reference 关键文件未覆盖：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：main.go、heartbeat.go、middleware_test.go、recoverer.go、request_id.go、terminal.go、wrap_writer.go、wrap_writer_test.go

### 中间件系统/内置中间件详解/错误处理中间件.md

- reference 标题：错误处理中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/自定义中间件开发.md

- reference 标题：自定义中间件开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 行数：351 / 40
- 段落行数：121 / 7
- Mermaid：5 / 0
- 文件提及重合：chain.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、basic_auth.go、get_head.go、logger.go、maybe.go、middleware.go、nocache.go、recoverer.go

### 快速开始.md

- reference 标题：快速开始
- 生成页：项目概述.md（项目概述）
- 匹配分数：164
- 行数：289 / 40
- 段落行数：108 / 7
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go

### 核心架构/上下文管理系统.md

- reference 标题：上下文管理系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：128
- 行数：313 / 28
- 段落行数：123 / 8
- Mermaid：7 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux.go

### 核心架构/中间件链执行机制.md

- reference 标题：中间件链执行机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：134
- 行数：313 / 28
- 段落行数：150 / 8
- Mermaid：9 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chain.go、chi.go、context.go、logger.go、middleware.go、middleware_test.go、recoverer.go、request_id.go

### 核心架构/基数树路由算法.md

- reference 标题：基数树路由算法
- 生成页：系统架构.md（系统架构）
- 匹配分数：98
- 行数：304 / 28
- 段落行数：153 / 8
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：chi.go、context.go、mux.go、pattern_test.go、tree.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：chi.go、context.go、mux.go、pattern_test.go、tree.go、tree_test.go

### 核心架构/核心架构.md

- reference 标题：核心架构
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 行数：329 / 40
- 段落行数：145 / 7
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、middleware.go

### 核心架构/组件交互关系.md

- reference 标题：组件交互关系
- 生成页：项目概述.md（项目概述）
- 匹配分数：166
- 行数：365 / 40
- 段落行数：171 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、middleware.go

### 核心架构/路由器接口设计.md

- reference 标题：路由器接口设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：134
- 行数：444 / 40
- 段落行数：318 / 7
- Mermaid：9 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go

### 测试与部署.md

- reference 标题：测试与部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 行数：238 / 32
- 段落行数：58 / 8
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、security.md、context_test.go、compress_test.go、content_charset_test.go、logger_test.go、middleware_test.go

### 示例教程/中间件示例.md

- reference 标题：中间件示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：82
- 行数：325 / 40
- 段落行数：143 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go
- reference 关键文件未覆盖：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basic_auth.go、clean_path.go、compress.go、logger.go、middleware.go、realip.go、recoverer.go

### 示例教程/基础示例.md

- reference 标题：基础示例
- 生成页：项目概述.md（项目概述）
- 匹配分数：154
- 行数：269 / 40
- 段落行数：100 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、logger.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、logger.go、recoverer.go

### 示例教程/示例教程.md

- reference 标题：示例教程
- 生成页：项目概述.md（项目概述）
- 匹配分数：264
- 行数：349 / 40
- 段落行数：122 / 7
- Mermaid：3 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go、request_id.go、throttle.go、timeout.go、routes.md

### 示例教程/进阶示例.md

- reference 标题：进阶示例
- 生成页：核心模块/_examples.md（模块：_examples）
- 匹配分数：164
- 行数：373 / 49
- 段落行数：162 / 9
- Mermaid：7 / 0
- 文件提及重合：main.go、todos.go、users.go
- reference 关键文件未覆盖：routes.md、article.go、chi.go、context.go、middleware.go、mux.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routes.md、article.go、chi.go、context.go、middleware.go、mux.go

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：无
- 问题：缺少对应生成页面

### 路由系统/上下文管理/Context结构设计.md

- reference 标题：Context结构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：90
- 行数：317 / 40
- 段落行数：208 / 7
- Mermaid：8 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go

### 路由系统/上下文管理/URL参数提取.md

- reference 标题：URL参数提取
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 行数：317 / 40
- 段落行数：161 / 7
- Mermaid：5 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 路由系统/上下文管理/上下文生命周期管理.md

- reference 标题：上下文生命周期管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：122
- 行数：309 / 40
- 段落行数：161 / 7
- Mermaid：5 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、get_head.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、get_head.go、mux_test.go

### 路由系统/上下文管理/上下文管理.md

- reference 标题：上下文管理
- 生成页：项目概述.md（项目概述）
- 匹配分数：96
- 行数：320 / 40
- 段落行数：130 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、middleware.go、value.go、mux_test.go

### 路由系统/上下文管理/上下文键值系统.md

- reference 标题：上下文键值系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：328 / 40
- 段落行数：140 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chi.go、mux.go
- reference 关键文件未覆盖：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、basic_auth.go、logger.go、middleware.go、request_id.go、value.go

### 路由系统/基数树算法实现/基数树算法实现.md

- reference 标题：基数树算法实现
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：313 / 40
- 段落行数：149 / 7
- Mermaid：7 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、tree_test.go

### 路由系统/基数树算法实现/插入算法实现.md

- reference 标题：插入算法实现
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：375 / 40
- 段落行数：283 / 7
- Mermaid：9 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 路由系统/基数树算法实现/查找算法实现.md

- reference 标题：查找算法实现
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 行数：376 / 40
- 段落行数：277 / 7
- Mermaid：7 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 路由系统/基数树算法实现/树遍历和遍历算法.md

- reference 标题：树遍历和遍历算法
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：321 / 40
- 段落行数：143 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 路由系统/基数树算法实现/节点结构设计.md

- reference 标题：节点结构设计
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 行数：322 / 40
- 段落行数：197 / 7
- Mermaid：6 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 路由系统/基数树算法实现/路由模式解析.md

- reference 标题：路由模式解析
- 生成页：项目概述.md（项目概述）
- 匹配分数：64
- 行数：357 / 40
- 段落行数：229 / 7
- Mermaid：9 / 0
- 文件提及重合：mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/子路由器系统/上下文传递机制.md

- reference 标题：上下文传递机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：150
- 行数：313 / 40
- 段落行数：209 / 7
- Mermaid：9 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、logger.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、mux_test.go

### 路由系统/子路由器系统/子路由器创建与配置.md

- reference 标题：子路由器创建与配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：260 / 40
- 段落行数：103 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 路由系统/子路由器系统/子路由器系统.md

- reference 标题：子路由器系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：150
- 行数：315 / 40
- 段落行数：158 / 7
- Mermaid：8 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、middleware.go、mux_test.go

### 路由系统/子路由器系统/嵌套设计模式.md

- reference 标题：嵌套设计模式
- 生成页：项目概述.md（项目概述）
- 匹配分数：168
- 行数：313 / 40
- 段落行数：121 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、mux_test.go

### 路由系统/子路由器系统/继承行为与覆盖规则.md

- reference 标题：继承行为与覆盖规则
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：328 / 40
- 段落行数：112 / 7
- Mermaid：8 / 0
- 文件提及重合：chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、middleware.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、middleware.go、mux_test.go

### 路由系统/路由匹配机制/动态参数提取.md

- reference 标题：动态参数提取
- 生成页：项目概述.md（项目概述）
- 匹配分数：140
- 行数：291 / 40
- 段落行数：108 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、path_value_test.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由优先级规则.md

- reference 标题：路由优先级规则
- 生成页：项目概述.md（项目概述）
- 匹配分数：92
- 行数：250 / 40
- 段落行数：114 / 7
- Mermaid：5 / 0
- 文件提及重合：chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由匹配机制.md

- reference 标题：路由匹配机制
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 行数：277 / 40
- 段落行数：107 / 7
- Mermaid：5 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、pattern_test.go、tree_test.go

### 路由系统/路由匹配机制/路由匹配流程.md

- reference 标题：路由匹配流程
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 行数：320 / 40
- 段落行数：107 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go

### 路由系统/路由模式语法.md

- reference 标题：路由模式语法
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 行数：253 / 40
- 段落行数：88 / 7
- Mermaid：4 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go、pattern_test.go、tree_test.go

### 路由系统/路由系统.md

- reference 标题：路由系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：146
- 行数：374 / 40
- 段落行数：153 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、context_test.go、mux_test.go、pattern_test.go、tree_test.go

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：746
- 行数：275 / 40
- 段落行数：143 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、recoverer.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、recoverer.go

### 高级特性/上下文高级用法.md

- reference 标题：上下文高级用法
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 行数：306 / 40
- 段落行数：115 / 7
- Mermaid：7 / 0
- 文件提及重合：main.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、context_test.go、logger.go、middleware.go、request_id.go、value.go、mux_test.go

### 高级特性/中间件组合技巧.md

- reference 标题：中间件组合技巧
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：433 / 40
- 段落行数：208 / 7
- Mermaid：12 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go
- reference 关键文件未覆盖：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：logger.go、maybe.go、middleware.go、middleware_test.go、recoverer.go、request_id.go、throttle.go、timeout.go

### 高级特性/复杂路由模式.md

- reference 标题：复杂路由模式
- 生成页：项目概述.md（项目概述）
- 匹配分数：190
- 行数：309 / 40
- 段落行数：125 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、recoverer.go、request_id.go、mux_test.go、pattern_test.go、tree_test.go

### 高级特性/子路由器高级配置.md

- reference 标题：子路由器高级配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：328 / 40
- 段落行数：157 / 7
- Mermaid：10 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go

### 高级特性/性能优化策略.md

- reference 标题：性能优化策略
- 生成页：项目概述.md（项目概述）
- 匹配分数：168
- 行数：294 / 40
- 段落行数：115 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、context.go、logger.go、middleware.go、recoverer.go、request_id.go、mux_test.go、tree_test.go

### 高级特性/自定义 HTTP 方法.md

- reference 标题：自定义 HTTP 方法
- 生成页：项目概述.md（项目概述）
- 匹配分数：148
- 行数：266 / 40
- 段落行数：104 / 7
- Mermaid：5 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、mux_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、mux_test.go

### 高级特性/路由遍历与文档生成.md

- reference 标题：路由遍历与文档生成
- 生成页：项目概述.md（项目概述）
- 匹配分数：170
- 行数：254 / 40
- 段落行数：101 / 7
- Mermaid：6 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、tree_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、tree_test.go

### 高级特性/高级特性.md

- reference 标题：高级特性
- 生成页：项目概述.md（项目概述）
- 匹配分数：138
- 行数：435 / 40
- 段落行数：187 / 7
- Mermaid：11 / 0
- 文件提及重合：main.go、chain.go、chi.go、mux.go、tree.go
- reference 关键文件未覆盖：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：context.go、logger.go、profiler.go、recoverer.go、request_id.go、throttle.go、timeout.go

