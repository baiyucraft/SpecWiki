# axum Reference 对比报告

生成页面：13 页
reference 页面：111 页
命中对比：81 页
缺失对比：30 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=27, total_tokens=171147, page_research=16, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 6 / reference 15
- evidence 落页：generated 11 / reference 111
- 图表达覆盖：generated 10 / reference 110
- page research 请求：16
- page enrichment 请求：0
- 已规划专题类型：专题页(3)、流程主题(2)、提取器主题(1)
- 高频缺失专题：中间件主题(1)

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

- 核心模块/axum.md 被 33 个 reference 页面共享映射
- 核心模块/axum-core.md 被 13 个 reference 页面共享映射
- 专题/module-capability/module-0d0ea2e3839c-library-axum能力：扩展机制.md 被 2 个 reference 页面共享映射
- 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md 被 7 个 reference 页面共享映射
- 专题/process/process-handle-flow-流程主题：handle-flow.md 被 5 个 reference 页面共享映射
- 核心模块/axum-macros.md 被 16 个 reference 页面共享映射
- 系统架构.md 被 3 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-760678685892-library-axum-extra能力：扩展机制.md (axum-extra能力：扩展机制, 40 行)
- 专题/module-capability/module-f31d8dabdc14-plugin-axum-macros能力：扩展机制.md (axum-macros能力：扩展机制, 39 行)
- 专题/process/process-handler-flow-流程主题：handler-flow.md (流程主题：handler flow, 25 行)
- 核心模块/axum-extra.md (模块：axum-extra, 88 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/axum.md | 335/104 | 74/27 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、extension.rs、docs.rs |
| API 参考/WebSocket API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/中间件 API.md | 核心模块/axum.md | 286/104 | 116/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs |
| API 参考/响应 API/JSON 和 HTML 响应.md | 核心模块/axum.md | 316/104 | 122/27 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、response.md、rejection.rs、readme.md、main.rs、tests.rs、derive_from_request.rs、with_rejection.rs |
| API 参考/响应 API/SSE 流式响应.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/响应 API/响应 API.md | 核心模块/axum.md | 295/104 | 145/27 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、extension.rs、redirect.rs、sse.rs、main.rs |
| API 参考/响应 API/基础响应类型.md | 核心模块/axum-core.md | 282/99 | 111/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：body.rs、append_headers.rs、into_response.rs、into_response_parts.rs、response.md |
| API 参考/响应 API/自定义响应类型.md | 核心模块/axum.md | 229/104 | 74/27 | 0/1 | 3/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、error_response.rs、multiple.rs、response.md、redirect.rs、sse.rs、main.rs |
| API 参考/响应 API/重定向响应.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Form 提取器.md | 核心模块/axum.md | 248/104 | 89/27 | 13/1 | 4/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、error_handling.md、extract.md、multipart.rs、query.rs、raw_form.rs、rejection.rs、main.rs |
| API 参考/提取器 API/内置提取器/Json 提取器.md | 核心模块/axum.md | 315/104 | 130/27 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：json_deserializer.rs、extract.md、rejection.rs、custom_extractor.rs |
| API 参考/提取器 API/内置提取器/Multipart 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Path 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Query 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/State 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/WebSocketUpgrade 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/内置提取器.md | 核心模块/axum.md | 364/104 | 127/27 | 17/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：multipart.rs、nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、ws.rs |
| API 参考/提取器 API/提取器 API.md | 核心模块/axum-core.md | 291/99 | 105/24 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs |
| API 参考/提取器 API/提取器组合.md | 核心模块/axum-core.md | 299/99 | 132/24 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs |
| API 参考/提取器 API/提取器错误处理.md | 核心模块/axum-core.md | 366/99 | 166/24 | 16/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs |
| API 参考/提取器 API/自定义提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/服务器 API.md | 核心模块/axum.md | 314/104 | 142/27 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、connect_info.rs、listener.rs、main.rs |
| API 参考/核心 API/Extractor API/Extractor API.md | 专题/module-capability/module-0d0ea2e3839c-library-axum能力：扩展机制.md | 334/43 | 139/21 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：mod.rs、rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、custom_extractor.rs、derive_from_request.rs |
| API 参考/核心 API/Extractor API/FromRequest Trait.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 302/40 | 141/18 | 20/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、parts_extracting_body.rs、named.rs、state_via.rs、extract.md、query.rs、state.rs、custom_extractor.rs |
| API 参考/核心 API/Extractor API/FromRequestParts Trait.md | 核心模块/axum-core.md | 291/99 | 115/24 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs |
| API 参考/核心 API/Extractor API/拒绝系统.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 314/40 | 133/18 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs |
| API 参考/核心 API/Handler API.md | 专题/process/process-handle-flow-流程主题：handle-flow.md | 340/35 | 127/12 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debugging_handler_type_errors.md、handlers_intro.md、future.rs、mod.rs、service.rs、main.rs |
| API 参考/核心 API/Middleware API/Middleware API.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 290/40 | 111/18 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、service_ext.rs |
| API 参考/核心 API/Middleware API/中间件未来类型.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 347/40 | 132/18 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs |
| API 参考/核心 API/Middleware API/中间件集成.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 319/40 | 129/18 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、method_routing.rs、service_ext.rs |
| API 参考/核心 API/Response API/Response API.md | 专题/module-capability/module-0d0ea2e3839c-library-axum能力：扩展机制.md | 301/43 | 155/21 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：append_headers.rs、into_response.rs、into_response_parts.rs、mod.rs、redirect.rs、sse.rs、main.rs |
| API 参考/核心 API/Response API/响应类型.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 289/40 | 98/18 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、redirect.rs、sse.rs、main.rs |
| API 参考/核心 API/Response API/自定义响应.md | 专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md | 244/40 | 106/18 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、async_read_body.rs、error_response.rs、response.md、redirect.rs、sse.rs、main.rs |
| API 参考/核心 API/Router API.md | 专题/process/process-handle-flow-流程主题：handle-flow.md | 399/35 | 99/12 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、method_routing.rs、mod.rs |
| API 参考/核心 API/核心 API.md | 核心模块/axum.md | 340/104 | 105/27 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：handlers_intro.md、response.md、main.rs |
| API 参考/核心 API/错误处理 API.md | 核心模块/axum-core.md | 302/99 | 119/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、into_response.rs、error_handling.md、rejection.rs、handle_error.rs、custom_extractor.rs、derive_from_request.rs、main.rs |
| API 参考/测试 API.md | 核心模块/axum.md | 281/104 | 137/27 | 21/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：axum_test.rs、benches.rs、connect_info.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs |
| API 参考/状态管理 API.md | 核心模块/axum-core.md | 329/99 | 121/24 | 16/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs |
| API 参考/错误处理 API.md | 核心模块/axum-core.md | 284/99 | 141/24 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、into_response.rs、debugging_handler_type_errors.md、error_handling.md、rejection.rs、handle_error.rs、main.rs |
| WebSocket 支持.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/中间件概念与原理.md | 核心模块/axum.md | 317/104 | 145/27 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs |
| 中间件系统/中间件系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/中间件链管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 中间件系统/自定义中间件开发.md | 核心模块/axum.md | 311/104 | 109/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.rs、middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、util.rs、main.rs |
| 响应系统/JSON 和 HTML 响应.md | 核心模块/axum.md | 316/104 | 108/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：response.md、redirect.rs、sse.rs、main.rs |
| 响应系统/SSE 流式响应.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 响应系统/响应系统.md | 核心模块/axum.md | 289/104 | 133/27 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、redirect.rs、sse.rs、main.rs |
| 响应系统/基础响应类型.md | 核心模块/axum.md | 288/104 | 117/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md |
| 响应系统/自定义响应类型.md | 核心模块/axum.md | 219/104 | 89/27 | 0/1 | 3/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、attachment.rs、file_stream.rs、response.md、extension.rs |
| 响应系统/重定向响应.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 宏系统/FromRef 宏.md | 核心模块/axum-macros.md | 352/87 | 219/21 | 0/1 | 0/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；缺少引用/出处块；缺少关键文件提及：generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs |
| 宏系统/FromRequest 相关宏/FromRequest 相关宏.md | 核心模块/axum-macros.md | 277/87 | 77/21 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs、state_enum_via.rs |
| 宏系统/FromRequest 相关宏/derive_from_request 宏.md | 核心模块/axum-macros.md | 345/87 | 94/21 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs、state_infer.rs、state_infer_multiple.rs |
| 宏系统/FromRequest 相关宏/derive_from_request_parts 宏.md | 核心模块/axum-macros.md | 229/87 | 83/21 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：request_parts.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs |
| 宏系统/FromRequest 相关宏/字段级属性配置.md | 核心模块/axum-macros.md | 277/87 | 116/21 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs、tuple_via.rs |
| 宏系统/FromRequest 相关宏/容器级属性配置.md | 核心模块/axum-macros.md | 279/87 | 91/21 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs、state_infer.rs |
| 宏系统/FromRequest 相关宏/泛型支持与限制.md | 核心模块/axum-macros.md | 255/87 | 97/21 | 20/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs、state_via_infer.rs |
| 宏系统/FromRequest 相关宏/错误处理与调试.md | 核心模块/axum-macros.md | 320/87 | 114/21 | 17/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md |
| 宏系统/TypedPath 宏.md | 核心模块/axum-macros.md | 261/87 | 78/21 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：typed.rs、missing_capture.rs、missing_field.rs、not_deserialize.rs、route_not_starting_with_slash.rs、unit_with_capture.rs、customize_rejection.rs、into_uri.rs |
| 宏系统/debug_handler 宏.md | 专题/process/process-handle-flow-流程主题：handle-flow.md | 264/35 | 66/12 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、debug_handler.rs、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs |
| 宏系统/debug_middleware 宏.md | 专题/process/process-handle-flow-流程主题：handle-flow.md | 192/35 | 85/12 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、debug_handler.rs、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs |
| 宏系统/宏测试与调试.md | 核心模块/axum-macros.md | 324/87 | 98/21 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs |
| 宏系统/宏系统.md | 核心模块/axum-macros.md | 317/87 | 95/21 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化.md | 核心模块/axum.md | 335/104 | 133/27 | 30/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、cargo.toml、benches.rs、listener.rs、main.rs |
| 扩展开发.md | 系统架构.md | 287/77 | 101/25 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs、cached.rs、from_fn.rs、redirect.rs |
| 提取器系统/内置提取器详解/JSON 和表单提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/Path 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/Query 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/State 提取器.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/内置提取器详解.md | 核心模块/axum.md | 371/104 | 154/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、main.rs |
| 提取器系统/内置提取器详解/原始查询和表单提取器.md | 核心模块/axum.md | 271/104 | 102/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：query.rs、raw_form.rs、raw_query.rs、rejection.rs、main.rs |
| 提取器系统/提取器系统.md | 核心模块/axum.md | 347/104 | 180/27 | 0/1 | 11/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs |
| 提取器系统/提取器组合与高级用法.md | 核心模块/axum-core.md | 337/99 | 153/24 | 16/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs |
| 提取器系统/提取器错误处理.md | 核心模块/axum-core.md | 340/99 | 139/24 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs |
| 提取器系统/自定义提取器实现.md | 核心模块/axum-core.md | 329/99 | 183/24 | 0/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs |
| 故障排除与常见问题/常见错误与解决方案.md | 核心模块/axum-macros.md | 237/87 | 65/21 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：self_receiver.rs、container.rs、named_fields_struct.rs |
| 故障排除与常见问题/性能问题排查.md | 核心模块/axum.md | 338/104 | 154/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、benches.rs、util.rs、main.rs |
| 故障排除与常见问题/故障排除与常见问题.md | 系统架构.md | 287/77 | 98/25 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md |
| 故障排除与常见问题/社区支持与问题报告.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除与常见问题/调试工具与技巧.md | 核心模块/axum-macros.md | 287/87 | 130/21 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs |
| 服务器集成.md | 核心模块/axum.md | 310/104 | 125/27 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、into_make_service_with_connect_info.md、connect_info.rs、listener.rs、main.rs |
| 核心概念/Tower 服务抽象.md | 核心模块/axum.md | 289/104 | 149/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs |
| 核心概念/异步运行时模型.md | 核心模块/axum.md | 260/104 | 111/27 | 0/1 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs |
| 核心概念/核心概念.md | 核心模块/axum.md | 341/104 | 145/27 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、rejection.rs、state.rs |
| 核心概念/模块化架构.md | 系统架构.md | 324/77 | 131/25 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs |
| 核心概念/类型安全设计.md | 核心模块/axum-macros.md | 282/87 | 136/21 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs |
| 核心概念/请求生命周期.md | 核心模块/axum.md | 308/104 | 150/27 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs |
| 测试策略.md | 核心模块/axum-macros.md | 327/87 | 120/21 | 23/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs |
| 状态管理.md | 核心模块/axum-core.md | 256/99 | 94/24 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、with_state.md、extension.rs、state.rs |
| 示例展示/Web 开发示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/基础示例.md | 核心模块/axum.md | 335/104 | 143/27 | 17/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs |
| 示例展示/数据库集成示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/测试与调试示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/示例展示.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/网络与 TLS 示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/认证授权示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 示例展示/高级特性示例.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 核心模块/axum-macros.md | 380/87 | 151/21 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs |
| 路由系统/Router 基础概念.md | 专题/process/process-handle-flow-流程主题：handle-flow.md | 293/35 | 132/12 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：with_state.md、mod.rs、not_found.rs、fallback.rs、main.rs |
| 路由系统/回退处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 路由系统/嵌套路由.md | 核心模块/axum.md | 255/104 | 95/27 | 0/1 | 7/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs |
| 路由系统/方法路由.md | 核心模块/axum.md | 287/104 | 102/27 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、route_layer.md、method_filter.rs |
| 路由系统/路径路由.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 路由系统/路由合并.md | 核心模块/axum.md | 253/104 | 112/27 | 0/1 | 5/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、merge.md、merge.rs |
| 路由系统/路由定义与注册.md | 核心模块/axum.md | 295/104 | 102/27 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs |
| 路由系统/路由系统.md | 核心模块/axum.md | 228/104 | 94/27 | 0/1 | 3/1 | 解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs |
| 部署运维.md | 工作流与部署.md | 396/127 | 183/3 | 0/0 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs |
| 错误处理.md | 核心模块/axum-core.md | 282/99 | 114/24 | 19/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、rejection.rs、from_fn.rs、not_found.rs |
| 项目概述.md | 项目概述.md | 234/58 | 101/5 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：170
- 页面类型：other / module
- 行数：335 / 104
- 段落行数：74 / 27
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：mod.rs、lib.rs、form.rs、json.rs、service_ext.rs
- reference 关键文件未覆盖：cargo.toml、extension.rs、docs.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、extension.rs、docs.rs

### API 参考/WebSocket API.md

- reference 标题：WebSocket API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/中间件 API.md

- reference 标题：中间件 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：96
- 页面类型：other / module
- 行数：286 / 104
- 段落行数：116 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：lib.rs、mod.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs

### API 参考/响应 API/JSON 和 HTML 响应.md

- reference 标题：JSON 和 HTML 响应
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：60
- 页面类型：other / module
- 行数：316 / 104
- 段落行数：122 / 27
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：mod.rs、json.rs
- reference 关键文件未覆盖：extract.md、response.md、rejection.rs、readme.md、main.rs、tests.rs、derive_from_request.rs、with_rejection.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、response.md、rejection.rs、readme.md、main.rs、tests.rs、derive_from_request.rs、with_rejection.rs

### API 参考/响应 API/SSE 流式响应.md

- reference 标题：SSE 流式响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/响应 API/响应 API.md

- reference 标题：响应 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：122
- 页面类型：other / module
- 行数：295 / 104
- 段落行数：145 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：mod.rs、form.rs、json.rs、route.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、response.md、extension.rs、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、extension.rs、redirect.rs、sse.rs、main.rs

### API 参考/响应 API/基础响应类型.md

- reference 标题：基础响应类型
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：66
- 页面类型：other / module
- 行数：282 / 99
- 段落行数：111 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：body.rs、append_headers.rs、into_response.rs、into_response_parts.rs、response.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：body.rs、append_headers.rs、into_response.rs、into_response_parts.rs、response.md

### API 参考/响应 API/自定义响应类型.md

- reference 标题：自定义响应类型
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：62
- 页面类型：other / module
- 行数：229 / 104
- 段落行数：74 / 27
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、error_response.rs、multiple.rs、response.md、redirect.rs、sse.rs、main.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、error_response.rs、multiple.rs、response.md、redirect.rs、sse.rs、main.rs

### API 参考/响应 API/重定向响应.md

- reference 标题：重定向响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/Form 提取器.md

- reference 标题：Form 提取器
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：60
- 页面类型：other / module
- 行数：248 / 104
- 段落行数：89 / 27
- Evidence：13 / 1
- Mermaid：4 / 1
- 文件提及重合：mod.rs、form.rs
- reference 关键文件未覆盖：cargo.toml、error_handling.md、extract.md、multipart.rs、query.rs、raw_form.rs、rejection.rs、main.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、error_handling.md、extract.md、multipart.rs、query.rs、raw_form.rs、rejection.rs、main.rs

### API 参考/提取器 API/内置提取器/Json 提取器.md

- reference 标题：Json 提取器
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：62
- 页面类型：other / module
- 行数：315 / 104
- 段落行数：130 / 27
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：mod.rs、json.rs
- reference 关键文件未覆盖：json_deserializer.rs、extract.md、rejection.rs、custom_extractor.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：json_deserializer.rs、extract.md、rejection.rs、custom_extractor.rs

### API 参考/提取器 API/内置提取器/Multipart 提取器.md

- reference 标题：Multipart 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/Path 提取器.md

- reference 标题：Path 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/Query 提取器.md

- reference 标题：Query 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/State 提取器.md

- reference 标题：State 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/WebSocketUpgrade 提取器.md

- reference 标题：WebSocketUpgrade 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/内置提取器.md

- reference 标题：内置提取器
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：92
- 页面类型：other / module
- 行数：364 / 104
- 段落行数：127 / 27
- Evidence：17 / 1
- Mermaid：4 / 1
- 文件提及重合：form.rs、mod.rs、json.rs
- reference 关键文件未覆盖：multipart.rs、nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、ws.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：multipart.rs、nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、ws.rs

### API 参考/提取器 API/提取器 API.md

- reference 标题：提取器 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：66
- 页面类型：other / module
- 行数：291 / 99
- 段落行数：105 / 24
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs

### API 参考/提取器 API/提取器组合.md

- reference 标题：提取器组合
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：74
- 页面类型：other / module
- 行数：299 / 99
- 段落行数：132 / 24
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs

### API 参考/提取器 API/提取器错误处理.md

- reference 标题：提取器错误处理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：74
- 页面类型：other / module
- 行数：366 / 99
- 段落行数：166 / 24
- Evidence：16 / 1
- Mermaid：7 / 1
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs

### API 参考/提取器 API/自定义提取器.md

- reference 标题：自定义提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/服务器 API.md

- reference 标题：服务器 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：70
- 页面类型：other / module
- 行数：314 / 104
- 段落行数：142 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：lib.rs、mod.rs
- reference 关键文件未覆盖：cargo.toml、connect_info.rs、listener.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、connect_info.rs、listener.rs、main.rs

### API 参考/核心 API/Extractor API/Extractor API.md

- reference 标题：提取器 API
- 生成页：专题/module-capability/module-0d0ea2e3839c-library-axum能力：扩展机制.md（axum能力：扩展机制）
- 匹配分数：152
- 页面类型：topic / topic
- 行数：334 / 43
- 段落行数：139 / 21
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：form.rs、json.rs
- reference 关键文件未覆盖：mod.rs、rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、custom_extractor.rs、derive_from_request.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：mod.rs、rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、custom_extractor.rs、derive_from_request.rs

### API 参考/核心 API/Extractor API/FromRequest Trait.md

- reference 标题：FromRequest Trait
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：130
- 页面类型：topic / topic
- 行数：302 / 40
- 段落行数：141 / 18
- Evidence：20 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：rejection.rs、parts_extracting_body.rs、named.rs、state_via.rs、extract.md、query.rs、state.rs、custom_extractor.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、parts_extracting_body.rs、named.rs、state_via.rs、extract.md、query.rs、state.rs、custom_extractor.rs

### API 参考/核心 API/Extractor API/FromRequestParts Trait.md

- reference 标题：FromRequestParts Trait
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：136
- 页面类型：topic / module
- 行数：291 / 99
- 段落行数：115 / 24
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：request_parts.rs、mod.rs、tuple.rs
- reference 关键文件未覆盖：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs

### API 参考/核心 API/Extractor API/拒绝系统.md

- reference 标题：拒绝系统
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：122
- 页面类型：topic / topic
- 行数：314 / 40
- 段落行数：133 / 18
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs

### API 参考/核心 API/Handler API.md

- reference 标题：Handler API
- 生成页：专题/process/process-handle-flow-流程主题：handle-flow.md（流程主题：handle flow）
- 匹配分数：150
- 页面类型：topic / topic
- 行数：340 / 35
- 段落行数：127 / 12
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：debugging_handler_type_errors.md、handlers_intro.md、future.rs、mod.rs、service.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debugging_handler_type_errors.md、handlers_intro.md、future.rs、mod.rs、service.rs、main.rs

### API 参考/核心 API/Middleware API/Middleware API.md

- reference 标题：中间件 API
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：102
- 页面类型：topic / topic
- 行数：290 / 40
- 段落行数：111 / 18
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、service_ext.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、service_ext.rs

### API 参考/核心 API/Middleware API/中间件未来类型.md

- reference 标题：中间件未来类型
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：108
- 页面类型：topic / topic
- 行数：347 / 40
- 段落行数：132 / 18
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs

### API 参考/核心 API/Middleware API/中间件集成.md

- reference 标题：中间件集成
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：102
- 页面类型：topic / topic
- 行数：319 / 40
- 段落行数：129 / 18
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、method_routing.rs、service_ext.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、method_routing.rs、service_ext.rs

### API 参考/核心 API/Response API/Response API.md

- reference 标题：响应 API
- 生成页：专题/module-capability/module-0d0ea2e3839c-library-axum能力：扩展机制.md（axum能力：扩展机制）
- 匹配分数：154
- 页面类型：topic / topic
- 行数：301 / 43
- 段落行数：155 / 21
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：form.rs、json.rs
- reference 关键文件未覆盖：append_headers.rs、into_response.rs、into_response_parts.rs、mod.rs、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：append_headers.rs、into_response.rs、into_response_parts.rs、mod.rs、redirect.rs、sse.rs、main.rs

### API 参考/核心 API/Response API/响应类型.md

- reference 标题：响应类型
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：124
- 页面类型：topic / topic
- 行数：289 / 40
- 段落行数：98 / 18
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、response.md、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、redirect.rs、sse.rs、main.rs

### API 参考/核心 API/Response API/自定义响应.md

- reference 标题：自定义响应
- 生成页：专题/module-capability/module-33ffb7a78287-library-axum-core能力：扩展机制.md（axum-core能力：扩展机制）
- 匹配分数：128
- 页面类型：topic / topic
- 行数：244 / 40
- 段落行数：106 / 18
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、async_read_body.rs、error_response.rs、response.md、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、async_read_body.rs、error_response.rs、response.md、redirect.rs、sse.rs、main.rs

### API 参考/核心 API/Router API.md

- reference 标题：Router API
- 生成页：专题/process/process-handle-flow-流程主题：handle-flow.md（流程主题：handle flow）
- 匹配分数：176
- 页面类型：topic / topic
- 行数：399 / 35
- 段落行数：99 / 12
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：lib.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、method_routing.rs、mod.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、method_routing.rs、mod.rs

### API 参考/核心 API/核心 API.md

- reference 标题：核心 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：144
- 页面类型：other / module
- 行数：340 / 104
- 段落行数：105 / 27
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：mod.rs、lib.rs、method_routing.rs、route.rs
- reference 关键文件未覆盖：handlers_intro.md、response.md、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：handlers_intro.md、response.md、main.rs

### API 参考/核心 API/错误处理 API.md

- reference 标题：错误处理 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：68
- 页面类型：other / module
- 行数：302 / 99
- 段落行数：119 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs、mod.rs
- reference 关键文件未覆盖：error.rs、into_response.rs、error_handling.md、rejection.rs、handle_error.rs、custom_extractor.rs、derive_from_request.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、into_response.rs、error_handling.md、rejection.rs、handle_error.rs、custom_extractor.rs、derive_from_request.rs、main.rs

### API 参考/测试 API.md

- reference 标题：测试 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：72
- 页面类型：other / module
- 行数：281 / 104
- 段落行数：137 / 27
- Evidence：21 / 1
- Mermaid：7 / 1
- 文件提及重合：lib.rs、mod.rs
- reference 关键文件未覆盖：axum_test.rs、benches.rs、connect_info.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：axum_test.rs、benches.rs、connect_info.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs

### API 参考/状态管理 API.md

- reference 标题：状态管理 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：84
- 页面类型：other / module
- 行数：329 / 99
- 段落行数：121 / 24
- Evidence：16 / 1
- Mermaid：6 / 1
- 文件提及重合：request.rs、mod.rs
- reference 关键文件未覆盖：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs

### API 参考/错误处理 API.md

- reference 标题：错误处理 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：72
- 页面类型：other / module
- 行数：284 / 99
- 段落行数：141 / 24
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：error.rs、into_response.rs、debugging_handler_type_errors.md、error_handling.md、rejection.rs、handle_error.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、into_response.rs、debugging_handler_type_errors.md、error_handling.md、rejection.rs、handle_error.rs、main.rs

### WebSocket 支持.md

- reference 标题：WebSocket 支持
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/中间件概念与原理.md

- reference 标题：中间件概念与原理
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：82
- 页面类型：other / module
- 行数：317 / 104
- 段落行数：145 / 27
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：lib.rs、mod.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs

### 中间件系统/中间件系统.md

- reference 标题：中间件系统
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/中间件链管理.md

- reference 标题：中间件链管理
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/内置中间件.md

- reference 标题：内置中间件
- 生成页：无
- 问题：缺少对应生成页面

### 中间件系统/自定义中间件开发.md

- reference 标题：自定义中间件开发
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：70
- 页面类型：other / module
- 行数：311 / 104
- 段落行数：109 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：lib.rs、mod.rs
- reference 关键文件未覆盖：middleware.rs、middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、util.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.rs、middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、util.rs、main.rs

### 响应系统/JSON 和 HTML 响应.md

- reference 标题：JSON 和 HTML 响应
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：90
- 页面类型：other / module
- 行数：316 / 104
- 段落行数：108 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、form.rs、json.rs
- reference 关键文件未覆盖：response.md、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：response.md、redirect.rs、sse.rs、main.rs

### 响应系统/SSE 流式响应.md

- reference 标题：SSE 流式响应
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/响应系统.md

- reference 标题：响应系统
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：70
- 页面类型：other / module
- 行数：289 / 104
- 段落行数：133 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：json.rs、mod.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、redirect.rs、sse.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、redirect.rs、sse.rs、main.rs

### 响应系统/基础响应类型.md

- reference 标题：基础响应类型
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：92
- 页面类型：other / module
- 行数：288 / 104
- 段落行数：117 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、form.rs、json.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、response.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md

### 响应系统/自定义响应类型.md

- reference 标题：自定义响应类型
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：92
- 页面类型：other / module
- 行数：219 / 104
- 段落行数：89 / 27
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：mod.rs、json.rs、lib.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、attachment.rs、file_stream.rs、response.md、extension.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、attachment.rs、file_stream.rs、response.md、extension.rs

### 响应系统/重定向响应.md

- reference 标题：重定向响应
- 生成页：无
- 问题：缺少对应生成页面

### 宏系统/FromRef 宏.md

- reference 标题：FromRef 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：94
- 页面类型：other / module
- 行数：352 / 87
- 段落行数：219 / 21
- Evidence：0 / 1
- Mermaid：0 / 1
- 文件提及重合：from_ref.rs、attr_parsing.rs、lib.rs
- reference 关键文件未覆盖：generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；缺少引用/出处块；缺少关键文件提及：generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs

### 宏系统/FromRequest 相关宏/FromRequest 相关宏.md

- reference 标题：FromRequest 相关宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：72
- 页面类型：other / module
- 行数：277 / 87
- 段落行数：77 / 21
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：attr.rs、mod.rs
- reference 关键文件未覆盖：generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs、state_enum_via.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs、state_enum_via.rs

### 宏系统/FromRequest 相关宏/derive_from_request 宏.md

- reference 标题：derive_from_request 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：122
- 页面类型：other / module
- 行数：345 / 87
- 段落行数：94 / 21
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：attr_parsing.rs、attr.rs、mod.rs、lib.rs
- reference 关键文件未覆盖：double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs、state_infer.rs、state_infer_multiple.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs、state_infer.rs、state_infer_multiple.rs

### 宏系统/FromRequest 相关宏/derive_from_request_parts 宏.md

- reference 标题：derive_from_request_parts 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：100
- 页面类型：other / module
- 行数：229 / 87
- 段落行数：83 / 21
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：attr.rs、mod.rs、lib.rs
- reference 关键文件未覆盖：request_parts.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：request_parts.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs

### 宏系统/FromRequest 相关宏/字段级属性配置.md

- reference 标题：字段级属性配置
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：98
- 页面类型：other / module
- 行数：277 / 87
- 段落行数：116 / 21
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：mod.rs、attr_parsing.rs、attr.rs
- reference 关键文件未覆盖：via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs、tuple_via.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs、tuple_via.rs

### 宏系统/FromRequest 相关宏/容器级属性配置.md

- reference 标题：容器级属性配置
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：68
- 页面类型：other / module
- 行数：279 / 87
- 段落行数：91 / 21
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：attr.rs、mod.rs
- reference 关键文件未覆盖：generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs、state_infer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs、state_infer.rs

### 宏系统/FromRequest 相关宏/泛型支持与限制.md

- reference 标题：泛型支持与限制
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：98
- 页面类型：other / module
- 行数：255 / 87
- 段落行数：97 / 21
- Evidence：20 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、attr.rs、lib.rs
- reference 关键文件未覆盖：generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs、state_via_infer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs、state_via_infer.rs

### 宏系统/FromRequest 相关宏/错误处理与调试.md

- reference 标题：错误处理与调试
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：126
- 页面类型：other / module
- 行数：320 / 87
- 段落行数：114 / 21
- Evidence：17 / 1
- Mermaid：6 / 1
- 文件提及重合：debug_handler.rs、attr.rs、mod.rs、lib.rs
- reference 关键文件未覆盖：generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md

### 宏系统/TypedPath 宏.md

- reference 标题：TypedPath 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：84
- 页面类型：other / module
- 行数：261 / 87
- 段落行数：78 / 21
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：mod.rs、attr_parsing.rs、typed_path.rs
- reference 关键文件未覆盖：typed.rs、missing_capture.rs、missing_field.rs、not_deserialize.rs、route_not_starting_with_slash.rs、unit_with_capture.rs、customize_rejection.rs、into_uri.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：typed.rs、missing_capture.rs、missing_field.rs、not_deserialize.rs、route_not_starting_with_slash.rs、unit_with_capture.rs、customize_rejection.rs、into_uri.rs

### 宏系统/debug_handler 宏.md

- reference 标题：debug_handler 宏
- 生成页：专题/process/process-handle-flow-流程主题：handle-flow.md（流程主题：handle flow）
- 匹配分数：116
- 页面类型：topic / topic
- 行数：264 / 35
- 段落行数：66 / 12
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、debug_handler.rs、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、debug_handler.rs、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs

### 宏系统/debug_middleware 宏.md

- reference 标题：debug_middleware 宏
- 生成页：专题/process/process-handle-flow-流程主题：handle-flow.md（流程主题：handle flow）
- 匹配分数：140
- 页面类型：topic / topic
- 行数：192 / 35
- 段落行数：85 / 12
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、debug_handler.rs、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、debug_handler.rs、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs

### 宏系统/宏测试与调试.md

- reference 标题：宏测试与调试
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：162
- 页面类型：other / module
- 行数：324 / 87
- 段落行数：98 / 21
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：axum_test.rs、debug_handler.rs、mod.rs、lib.rs、typed_path.rs
- reference 关键文件未覆盖：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs

### 宏系统/宏系统.md

- reference 标题：宏系统
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：194
- 页面类型：other / module
- 行数：317 / 87
- 段落行数：95 / 21
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：axum_test.rs、debug_handler.rs、from_ref.rs、mod.rs、lib.rs、typed_path.rs
- reference 关键文件未覆盖：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化.md

- reference 标题：性能优化
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：66
- 页面类型：other / module
- 行数：335 / 104
- 段落行数：133 / 27
- Evidence：30 / 1
- Mermaid：9 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：.clippy.toml、cargo.toml、benches.rs、listener.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、cargo.toml、benches.rs、listener.rs、main.rs

### 扩展开发.md

- reference 标题：扩展开发
- 生成页：系统架构.md（系统架构）
- 匹配分数：120
- 页面类型：other / architecture
- 行数：287 / 77
- 段落行数：101 / 25
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、mod.rs、cached.rs、from_fn.rs、redirect.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs、cached.rs、from_fn.rs、redirect.rs

### 提取器系统/内置提取器详解/JSON 和表单提取器.md

- reference 标题：JSON 和表单提取器
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/内置提取器详解/Path 提取器.md

- reference 标题：Path 提取器
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/内置提取器详解/Query 提取器.md

- reference 标题：Query 提取器
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/内置提取器详解/State 提取器.md

- reference 标题：State 提取器
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/内置提取器详解/内置提取器详解.md

- reference 标题：内置提取器详解
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：86
- 页面类型：other / module
- 行数：371 / 104
- 段落行数：154 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、form.rs、json.rs
- reference 关键文件未覆盖：nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nested_path.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs、main.rs

### 提取器系统/内置提取器详解/原始查询和表单提取器.md

- reference 标题：原始查询和表单提取器
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：84
- 页面类型：other / module
- 行数：271 / 104
- 段落行数：102 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、form.rs、lib.rs
- reference 关键文件未覆盖：query.rs、raw_form.rs、raw_query.rs、rejection.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：query.rs、raw_form.rs、raw_query.rs、rejection.rs、main.rs

### 提取器系统/提取器系统.md

- reference 标题：提取器系统
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：122
- 页面类型：other / module
- 行数：347 / 104
- 段落行数：180 / 27
- Evidence：0 / 1
- Mermaid：11 / 1
- 文件提及重合：mod.rs、lib.rs、form.rs、json.rs
- reference 关键文件未覆盖：connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs

### 提取器系统/提取器组合与高级用法.md

- reference 标题：提取器组合与高级用法
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：72
- 页面类型：other / module
- 行数：337 / 99
- 段落行数：153 / 24
- Evidence：16 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs

### 提取器系统/提取器错误处理.md

- reference 标题：提取器错误处理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：62
- 页面类型：other / module
- 行数：340 / 99
- 段落行数：139 / 24
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：rejection.rs、error_handling.md、extract.md、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs

### 提取器系统/自定义提取器实现.md

- reference 标题：自定义提取器实现
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：74
- 页面类型：other / module
- 行数：329 / 99
- 段落行数：183 / 24
- Evidence：0 / 1
- Mermaid：10 / 1
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs

### 故障排除与常见问题/常见错误与解决方案.md

- reference 标题：常见错误与解决方案
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：102
- 页面类型：other / module
- 行数：237 / 87
- 段落行数：65 / 21
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：debug_handler.rs、mod.rs、typed_path.rs
- reference 关键文件未覆盖：self_receiver.rs、container.rs、named_fields_struct.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：self_receiver.rs、container.rs、named_fields_struct.rs

### 故障排除与常见问题/性能问题排查.md

- reference 标题：性能问题排查
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：120
- 页面类型：other / module
- 行数：338 / 104
- 段落行数：154 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、lib.rs、method_routing.rs、path_router.rs
- reference 关键文件未覆盖：cargo.toml、benches.rs、util.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、benches.rs、util.rs、main.rs

### 故障排除与常见问题/故障排除与常见问题.md

- reference 标题：故障排除与常见问题
- 生成页：系统架构.md（系统架构）
- 匹配分数：78
- 页面类型：other / architecture
- 行数：287 / 77
- 段落行数：98 / 25
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：contributing.md、cargo.toml、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、cargo.toml、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md

### 故障排除与常见问题/社区支持与问题报告.md

- reference 标题：社区支持与问题报告
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除与常见问题/调试工具与技巧.md

- reference 标题：调试工具与技巧
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：80
- 页面类型：other / module
- 行数：287 / 87
- 段落行数：130 / 21
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：axum_test.rs、debug_handler.rs、mod.rs
- reference 关键文件未覆盖：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs

### 服务器集成.md

- reference 标题：服务器集成
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：68
- 页面类型：other / module
- 行数：310 / 104
- 段落行数：125 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：lib.rs、mod.rs
- reference 关键文件未覆盖：readme.md、into_make_service_with_connect_info.md、connect_info.rs、listener.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、into_make_service_with_connect_info.md、connect_info.rs、listener.rs、main.rs

### 核心概念/Tower 服务抽象.md

- reference 标题：Tower 服务抽象
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：170
- 页面类型：other / module
- 行数：289 / 104
- 段落行数：149 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：lib.rs、mod.rs、service.rs、route.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs

### 核心概念/异步运行时模型.md

- reference 标题：异步运行时模型
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：92
- 页面类型：other / module
- 行数：260 / 104
- 段落行数：111 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：118
- 页面类型：other / module
- 行数：341 / 104
- 段落行数：145 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：mod.rs、lib.rs、method_routing.rs
- reference 关键文件未覆盖：cargo.toml、rejection.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、rejection.rs、state.rs

### 核心概念/模块化架构.md

- reference 标题：模块化架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：200
- 页面类型：architecture / architecture
- 行数：324 / 77
- 段落行数：131 / 25
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、mod.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs

### 核心概念/类型安全设计.md

- reference 标题：类型安全设计
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：128
- 页面类型：other / module
- 行数：282 / 87
- 段落行数：136 / 21
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、lib.rs、debug_handler.rs、attr.rs
- reference 关键文件未覆盖：error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs

### 核心概念/请求生命周期.md

- reference 标题：请求生命周期
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：144
- 页面类型：other / module
- 行数：308 / 104
- 段落行数：150 / 27
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：lib.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs

### 测试策略.md

- reference 标题：测试策略
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：84
- 页面类型：other / module
- 行数：327 / 87
- 段落行数：120 / 21
- Evidence：23 / 1
- Mermaid：6 / 1
- 文件提及重合：axum_test.rs、lib.rs、mod.rs
- reference 关键文件未覆盖：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs

### 状态管理.md

- reference 标题：状态管理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：62
- 页面类型：other / module
- 行数：256 / 99
- 段落行数：94 / 24
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：from_ref.rs、with_state.md、extension.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、with_state.md、extension.rs、state.rs

### 示例展示/Web 开发示例.md

- reference 标题：Web 开发示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：100
- 页面类型：other / module
- 行数：335 / 104
- 段落行数：143 / 27
- Evidence：17 / 1
- Mermaid：8 / 1
- 文件提及重合：mod.rs、lib.rs、method_routing.rs
- reference 关键文件未覆盖：cargo.toml、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs

### 示例展示/数据库集成示例.md

- reference 标题：数据库集成示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/测试与调试示例.md

- reference 标题：测试与调试示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/示例展示.md

- reference 标题：示例展示
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/网络与 TLS 示例.md

- reference 标题：网络与 TLS 示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/认证授权示例.md

- reference 标题：认证授权示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/高级特性示例.md

- reference 标题：高级特性示例
- 生成页：无
- 问题：缺少对应生成页面

### 贡献指南.md

- reference 标题：贡献指南
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：158
- 页面类型：other / module
- 行数：380 / 87
- 段落行数：151 / 21
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：mod.rs、lib.rs、axum_test.rs、debug_handler.rs、typed_path.rs
- reference 关键文件未覆盖：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs

### 路由系统/Router 基础概念.md

- reference 标题：Router 基础概念
- 生成页：专题/process/process-handle-flow-流程主题：handle-flow.md（流程主题：handle flow）
- 匹配分数：116
- 页面类型：topic / topic
- 行数：293 / 35
- 段落行数：132 / 12
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：path_router.rs、route.rs
- reference 关键文件未覆盖：with_state.md、mod.rs、not_found.rs、fallback.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：with_state.md、mod.rs、not_found.rs、fallback.rs、main.rs

### 路由系统/回退处理.md

- reference 标题：回退处理
- 生成页：无
- 问题：缺少对应生成页面

### 路由系统/嵌套路由.md

- reference 标题：嵌套路由
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：64
- 页面类型：other / module
- 行数：255 / 104
- 段落行数：95 / 27
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：mod.rs、path_router.rs
- reference 关键文件未覆盖：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs

### 路由系统/方法路由.md

- reference 标题：方法路由
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：60
- 页面类型：other / module
- 行数：287 / 104
- 段落行数：102 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：method_routing.rs、mod.rs、route.rs
- reference 关键文件未覆盖：fallback.md、merge.md、route_layer.md、method_filter.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、route_layer.md、method_filter.rs

### 路由系统/路径路由.md

- reference 标题：路径路由
- 生成页：无
- 问题：缺少对应生成页面

### 路由系统/路由合并.md

- reference 标题：路由合并
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：60
- 页面类型：other / module
- 行数：253 / 104
- 段落行数：112 / 27
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：method_routing.rs、mod.rs、path_router.rs
- reference 关键文件未覆盖：cargo.toml、merge.md、merge.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、merge.md、merge.rs

### 路由系统/路由定义与注册.md

- reference 标题：路由定义与注册
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：160
- 页面类型：other / module
- 行数：295 / 104
- 段落行数：102 / 27
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：lib.rs、future.rs、method_routing.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs

### 路由系统/路由系统.md

- reference 标题：路由系统
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：88
- 页面类型：other / module
- 行数：228 / 104
- 段落行数：94 / 27
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：lib.rs、future.rs、method_routing.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs
- 结论：解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs

### 部署运维.md

- reference 标题：部署运维
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：90
- 页面类型：workflow / workflow
- 行数：396 / 127
- 段落行数：183 / 3
- Evidence：0 / 0
- Mermaid：12 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs

### 错误处理.md

- reference 标题：错误处理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：68
- 页面类型：other / module
- 行数：282 / 99
- 段落行数：114 / 24
- Evidence：19 / 1
- Mermaid：5 / 1
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：error.rs、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、rejection.rs、from_fn.rs、not_found.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、rejection.rs、from_fn.rs、not_found.rs

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：640
- 页面类型：overview / overview
- 行数：234 / 58
- 段落行数：101 / 5
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs

