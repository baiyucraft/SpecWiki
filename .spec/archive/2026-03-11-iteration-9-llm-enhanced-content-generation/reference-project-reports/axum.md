# axum Reference 对比报告

生成页面：7 页
reference 页面：111 页
命中对比：59 页
缺失对比：52 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/axum.md 被 21 个 reference 页面共享映射
- 核心模块/axum-core.md 被 17 个 reference 页面共享映射
- 核心模块/axum-macros.md 被 16 个 reference 页面共享映射
- 项目概述.md 被 2 个 reference 页面共享映射
- 系统架构.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 核心模块/axum-extra.md (模块：axum-extra, 45 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/axum.md | 335/47 | 74/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、extension.rs、form.rs、json.rs、docs.rs |
| API 参考/WebSocket API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/中间件 API.md | 核心模块/axum.md | 286/47 | 116/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs |
| API 参考/响应 API/JSON 和 HTML 响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/响应 API/SSE 流式响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/响应 API/响应 API.md | 核心模块/axum.md | 295/47 | 145/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、extension.rs、form.rs、json.rs、redirect.rs、sse.rs |
| API 参考/响应 API/基础响应类型.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/响应 API/自定义响应类型.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/响应 API/重定向响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Form 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Json 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Multipart 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Path 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/Query 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/State 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/WebSocketUpgrade 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/内置提取器/内置提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/提取器 API/提取器 API.md | 核心模块/axum-core.md | 291/43 | 105/10 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs |
| API 参考/提取器 API/提取器组合.md | 核心模块/axum-core.md | 299/43 | 132/10 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs |
| API 参考/提取器 API/提取器错误处理.md | 核心模块/axum-core.md | 366/43 | 166/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs |
| API 参考/提取器 API/自定义提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/服务器 API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/Extractor API/Extractor API.md | 核心模块/axum-core.md | 334/43 | 139/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、form.rs、json.rs、custom_extractor.rs |
| API 参考/核心 API/Extractor API/FromRequest Trait.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/Extractor API/FromRequestParts Trait.md | 核心模块/axum-core.md | 291/43 | 115/10 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs |
| API 参考/核心 API/Extractor API/拒绝系统.md | 核心模块/axum-core.md | 314/43 | 133/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs |
| API 参考/核心 API/Handler API.md | 核心模块/axum.md | 340/47 | 127/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debugging_handler_type_errors.md、handlers_intro.md、main.rs |
| API 参考/核心 API/Middleware API/Middleware API.md | 核心模块/axum.md | 290/47 | 111/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs |
| API 参考/核心 API/Middleware API/中间件未来类型.md | 核心模块/axum-core.md | 347/43 | 132/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs |
| API 参考/核心 API/Middleware API/中间件集成.md | 核心模块/axum.md | 319/47 | 129/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs |
| API 参考/核心 API/Response API/Response API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/Response API/响应类型.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/Response API/自定义响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/核心 API/Router API.md | 核心模块/axum.md | 399/47 | 99/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、strip_prefix.rs、merge.rs |
| API 参考/核心 API/核心 API.md | 核心模块/axum.md | 340/47 | 105/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：handlers_intro.md、response.md、main.rs |
| API 参考/核心 API/错误处理 API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/测试 API.md | 缺失 | - | - | - | 缺少对应生成页面 |
| API 参考/状态管理 API.md | 核心模块/axum-core.md | 329/43 | 121/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs |
| API 参考/错误处理 API.md | 核心模块/axum-core.md | 284/43 | 141/10 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、into_response.rs、debugging_handler_type_errors.md、error_handling.md、rejection.rs、handle_error.rs、main.rs |
| WebSocket 支持.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/中间件概念与原理.md | 核心模块/axum.md | 317/47 | 145/11 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs |
| 中间件系统/中间件系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/中间件链管理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/内置中间件.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 中间件系统/自定义中间件开发.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 响应系统/JSON 和 HTML 响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 响应系统/SSE 流式响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 响应系统/响应系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 响应系统/基础响应类型.md | 核心模块/axum-core.md | 288/43 | 117/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、form.rs、json.rs |
| 响应系统/自定义响应类型.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 响应系统/重定向响应.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 宏系统/FromRef 宏.md | 核心模块/axum-macros.md | 352/49 | 219/12 | 0/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs |
| 宏系统/FromRequest 相关宏/FromRequest 相关宏.md | 核心模块/axum-macros.md | 277/49 | 77/12 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs |
| 宏系统/FromRequest 相关宏/derive_from_request 宏.md | 核心模块/axum-macros.md | 345/49 | 94/12 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、attr.rs、double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs |
| 宏系统/FromRequest 相关宏/derive_from_request_parts 宏.md | 核心模块/axum-macros.md | 229/49 | 83/12 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：request_parts.rs、attr.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs |
| 宏系统/FromRequest 相关宏/字段级属性配置.md | 核心模块/axum-macros.md | 277/49 | 116/12 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、attr.rs、via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs |
| 宏系统/FromRequest 相关宏/容器级属性配置.md | 核心模块/axum-macros.md | 279/49 | 91/12 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs |
| 宏系统/FromRequest 相关宏/泛型支持与限制.md | 核心模块/axum-macros.md | 255/49 | 97/12 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs |
| 宏系统/FromRequest 相关宏/错误处理与调试.md | 核心模块/axum-macros.md | 320/49 | 114/12 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md |
| 宏系统/TypedPath 宏.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 宏系统/debug_handler 宏.md | 核心模块/axum-macros.md | 264/49 | 66/12 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs、multiple_extractors.rs |
| 宏系统/debug_middleware 宏.md | 核心模块/axum-macros.md | 192/49 | 85/12 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs |
| 宏系统/宏测试与调试.md | 核心模块/axum-macros.md | 324/49 | 98/12 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs |
| 宏系统/宏系统.md | 核心模块/axum-macros.md | 317/49 | 95/12 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs |
| 快速开始.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 性能优化.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 扩展开发.md | 项目概述.md | 287/35 | 101/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs、cached.rs、from_fn.rs、redirect.rs |
| 提取器系统/内置提取器详解/JSON 和表单提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/Path 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/Query 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/State 提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/内置提取器详解.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/内置提取器详解/原始查询和表单提取器.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/提取器系统.md | 系统架构.md | 347/44 | 180/10 | 11/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：mod.rs、connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs |
| 提取器系统/提取器组合与高级用法.md | 核心模块/axum-core.md | 337/43 | 153/10 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs |
| 提取器系统/提取器错误处理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 提取器系统/自定义提取器实现.md | 核心模块/axum-core.md | 329/43 | 183/10 | 10/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs |
| 故障排除与常见问题/常见错误与解决方案.md | 核心模块/axum-macros.md | 237/49 | 65/12 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：self_receiver.rs、container.rs、named_fields_struct.rs |
| 故障排除与常见问题/性能问题排查.md | 核心模块/axum.md | 338/47 | 154/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、benches.rs、util.rs、main.rs |
| 故障排除与常见问题/故障排除与常见问题.md | 核心模块/axum-core.md | 287/43 | 98/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md |
| 故障排除与常见问题/社区支持与问题报告.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 故障排除与常见问题/调试工具与技巧.md | 核心模块/axum-macros.md | 287/49 | 130/12 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs |
| 服务器集成.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心概念/Tower 服务抽象.md | 核心模块/axum.md | 289/47 | 149/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs |
| 核心概念/异步运行时模型.md | 核心模块/axum.md | 260/47 | 111/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs |
| 核心概念/核心概念.md | 核心模块/axum-core.md | 341/43 | 145/10 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、state.rs、method_routing.rs |
| 核心概念/模块化架构.md | 系统架构.md | 324/44 | 131/10 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs |
| 核心概念/类型安全设计.md | 核心模块/axum-core.md | 282/43 | 136/10 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debug_handler.rs、attr.rs、error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs |
| 核心概念/请求生命周期.md | 核心模块/axum.md | 308/47 | 150/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs |
| 测试策略.md | 核心模块/axum-macros.md | 327/49 | 120/12 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs |
| 状态管理.md | 核心模块/axum-core.md | 256/43 | 94/10 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、with_state.md、extension.rs、state.rs |
| 示例展示/Web 开发示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/基础示例.md | 核心模块/axum-core.md | 335/43 | 143/10 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：method_routing.rs、main.rs |
| 示例展示/数据库集成示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/测试与调试示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/示例展示.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/网络与 TLS 示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/认证授权示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 示例展示/高级特性示例.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 贡献指南.md | 核心模块/axum-macros.md | 380/49 | 151/12 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs |
| 路由系统/Router 基础概念.md | 核心模块/axum.md | 293/47 | 132/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：with_state.md、not_found.rs、fallback.rs、main.rs |
| 路由系统/回退处理.md | 核心模块/axum.md | 244/47 | 95/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、nest.md、not_found.rs、fallback.rs、main.rs |
| 路由系统/嵌套路由.md | 核心模块/axum.md | 255/47 | 95/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs |
| 路由系统/方法路由.md | 核心模块/axum.md | 287/47 | 102/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、route_layer.md、method_filter.rs |
| 路由系统/路径路由.md | 核心模块/axum.md | 287/47 | 107/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、route.md、de.rs、nest.rs、url_params.rs |
| 路由系统/路由合并.md | 核心模块/axum.md | 253/47 | 112/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、merge.md、merge.rs |
| 路由系统/路由定义与注册.md | 核心模块/axum.md | 295/47 | 102/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs |
| 路由系统/路由系统.md | 核心模块/axum.md | 228/47 | 94/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs |
| 部署运维.md | 工作流与部署.md | 396/45 | 183/11 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs |
| 错误处理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 项目概述.md | 项目概述.md | 234/35 | 101/9 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：112
- 行数：335 / 47
- 段落行数：74 / 11
- Mermaid：3 / 0
- 文件提及重合：mod.rs、lib.rs、service_ext.rs
- reference 关键文件未覆盖：cargo.toml、extension.rs、form.rs、json.rs、docs.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、extension.rs、form.rs、json.rs、docs.rs

### API 参考/WebSocket API.md

- reference 标题：WebSocket API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/中间件 API.md

- reference 标题：中间件 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：100
- 行数：286 / 47
- 段落行数：116 / 11
- Mermaid：8 / 0
- 文件提及重合：lib.rs、mod.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs

### API 参考/响应 API/JSON 和 HTML 响应.md

- reference 标题：JSON 和 HTML 响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/响应 API/SSE 流式响应.md

- reference 标题：SSE 流式响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/响应 API/响应 API.md

- reference 标题：响应 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：60
- 行数：295 / 47
- 段落行数：145 / 11
- Mermaid：7 / 0
- 文件提及重合：mod.rs、route.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、response.md、extension.rs、form.rs、json.rs、redirect.rs、sse.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、extension.rs、form.rs、json.rs、redirect.rs、sse.rs

### API 参考/响应 API/基础响应类型.md

- reference 标题：基础响应类型
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/响应 API/自定义响应类型.md

- reference 标题：自定义响应类型
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/响应 API/重定向响应.md

- reference 标题：重定向响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/Form 提取器.md

- reference 标题：Form 提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/内置提取器/Json 提取器.md

- reference 标题：Json 提取器
- 生成页：无
- 问题：缺少对应生成页面

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
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/提取器 API/提取器 API.md

- reference 标题：提取器 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：70
- 行数：291 / 43
- 段落行数：105 / 10
- Mermaid：4 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、connect_info.rs、matched_path.rs、multipart.rs、nested_path.rs、original_uri.rs、query.rs、raw_form.rs

### API 参考/提取器 API/提取器组合.md

- reference 标题：提取器组合
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：64
- 行数：299 / 43
- 段落行数：132 / 10
- Mermaid：8 / 0
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、matched_path.rs、nested_path.rs、query.rs

### API 参考/提取器 API/提取器错误处理.md

- reference 标题：提取器错误处理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：76
- 行数：366 / 43
- 段落行数：166 / 10
- Mermaid：7 / 0
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、error_handling.md、extract.md、query.rs、state.rs、custom_extractor.rs、derive_from_request.rs、main.rs

### API 参考/提取器 API/自定义提取器.md

- reference 标题：自定义提取器
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/服务器 API.md

- reference 标题：服务器 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/Extractor API/Extractor API.md

- reference 标题：提取器 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：72
- 行数：334 / 43
- 段落行数：139 / 10
- Mermaid：5 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、form.rs、json.rs、custom_extractor.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、multipart.rs、query.rs、raw_form.rs、state.rs、form.rs、json.rs、custom_extractor.rs

### API 参考/核心 API/Extractor API/FromRequest Trait.md

- reference 标题：FromRequest Trait
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/Extractor API/FromRequestParts Trait.md

- reference 标题：FromRequestParts Trait
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：120
- 行数：291 / 43
- 段落行数：115 / 10
- Mermaid：4 / 0
- 文件提及重合：request_parts.rs、mod.rs、tuple.rs
- reference 关键文件未覆盖：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、option.rs、rejection.rs、named_parts.rs、state_explicit_parts.rs、state_via_parts.rs、main.rs

### API 参考/核心 API/Extractor API/拒绝系统.md

- reference 标题：拒绝系统
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：76
- 行数：314 / 43
- 段落行数：133 / 10
- Mermaid：7 / 0
- 文件提及重合：macros.rs、mod.rs
- reference 关键文件未覆盖：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：error.rs、rejection.rs、into_response.rs、error_handling.md、main.rs

### API 参考/核心 API/Handler API.md

- reference 标题：Handler API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：134
- 行数：340 / 47
- 段落行数：127 / 11
- Mermaid：6 / 0
- 文件提及重合：lib.rs、future.rs、mod.rs、service.rs
- reference 关键文件未覆盖：debugging_handler_type_errors.md、handlers_intro.md、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debugging_handler_type_errors.md、handlers_intro.md、main.rs

### API 参考/核心 API/Middleware API/Middleware API.md

- reference 标题：中间件 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：128
- 行数：290 / 47
- 段落行数：111 / 11
- Mermaid：8 / 0
- 文件提及重合：mod.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、layer.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs

### API 参考/核心 API/Middleware API/中间件未来类型.md

- reference 标题：中间件未来类型
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：112
- 行数：347 / 43
- 段落行数：132 / 10
- Mermaid：7 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、main.rs

### API 参考/核心 API/Middleware API/中间件集成.md

- reference 标题：中间件集成
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：124
- 行数：319 / 47
- 段落行数：129 / 11
- Mermaid：7 / 0
- 文件提及重合：mod.rs、method_routing.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、map_request.rs、map_response.rs、response_axum_body.rs、main.rs

### API 参考/核心 API/Response API/Response API.md

- reference 标题：响应 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/Response API/响应类型.md

- reference 标题：响应类型
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/Response API/自定义响应.md

- reference 标题：自定义响应
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/核心 API/Router API.md

- reference 标题：Router API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：194
- 行数：399 / 47
- 段落行数：99 / 11
- Mermaid：3 / 0
- 文件提及重合：lib.rs、method_routing.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、strip_prefix.rs、merge.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layer.md、merge.md、nest.md、route.md、with_state.md、method_filter.rs、strip_prefix.rs、merge.rs

### API 参考/核心 API/核心 API.md

- reference 标题：核心 API
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：142
- 行数：340 / 47
- 段落行数：105 / 11
- Mermaid：3 / 0
- 文件提及重合：mod.rs、lib.rs、method_routing.rs、route.rs
- reference 关键文件未覆盖：handlers_intro.md、response.md、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：handlers_intro.md、response.md、main.rs

### API 参考/核心 API/错误处理 API.md

- reference 标题：错误处理 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/测试 API.md

- reference 标题：测试 API
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/状态管理 API.md

- reference 标题：状态管理 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：74
- 行数：329 / 43
- 段落行数：121 / 10
- Mermaid：6 / 0
- 文件提及重合：request.rs、mod.rs
- reference 关键文件未覆盖：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、benches.rs、extension.rs、state.rs、from_extractor.rs、from_fn.rs、service_ext.rs、main.rs

### API 参考/错误处理 API.md

- reference 标题：错误处理 API
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：60
- 行数：284 / 43
- 段落行数：141 / 10
- Mermaid：9 / 0
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
- 行数：317 / 47
- 段落行数：145 / 11
- Mermaid：9 / 0
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
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/JSON 和 HTML 响应.md

- reference 标题：JSON 和 HTML 响应
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/SSE 流式响应.md

- reference 标题：SSE 流式响应
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/响应系统.md

- reference 标题：响应系统
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/基础响应类型.md

- reference 标题：基础响应类型
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：68
- 行数：288 / 43
- 段落行数：117 / 10
- Mermaid：5 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：into_response.rs、into_response_parts.rs、response.md、form.rs、json.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：into_response.rs、into_response_parts.rs、response.md、form.rs、json.rs

### 响应系统/自定义响应类型.md

- reference 标题：自定义响应类型
- 生成页：无
- 问题：缺少对应生成页面

### 响应系统/重定向响应.md

- reference 标题：重定向响应
- 生成页：无
- 问题：缺少对应生成页面

### 宏系统/FromRef 宏.md

- reference 标题：FromRef 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：68
- 行数：352 / 49
- 段落行数：219 / 12
- Mermaid：0 / 0
- 文件提及重合：from_ref.rs、lib.rs
- reference 关键文件未覆盖：attr_parsing.rs、generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、generics.rs、basic.rs、reference-types.rs、skip.rs、state.rs

### 宏系统/FromRequest 相关宏/FromRequest 相关宏.md

- reference 标题：FromRequest 相关宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：60
- 行数：277 / 49
- 段落行数：77 / 12
- Mermaid：4 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：attr.rs、generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic.rs、via_on_container_and_field.rs、container.rs、enum_via.rs、named.rs、named_via.rs、override_rejection.rs

### 宏系统/FromRequest 相关宏/derive_from_request 宏.md

- reference 标题：derive_from_request 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：92
- 行数：345 / 49
- 段落行数：94 / 12
- Mermaid：4 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：attr_parsing.rs、attr.rs、double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、attr.rs、double_via_attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named.rs、override_rejection.rs、state_explicit.rs

### 宏系统/FromRequest 相关宏/derive_from_request_parts 宏.md

- reference 标题：derive_from_request_parts 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：94
- 行数：229 / 49
- 段落行数：83 / 12
- Mermaid：4 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：request_parts.rs、attr.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：request_parts.rs、attr.rs、named_parts.rs、named_via_parts.rs、override_rejection_parts.rs、state_via_parts.rs

### 宏系统/FromRequest 相关宏/字段级属性配置.md

- reference 标题：字段级属性配置
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：62
- 行数：277 / 49
- 段落行数：116 / 12
- Mermaid：6 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：attr_parsing.rs、attr.rs、via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr_parsing.rs、attr.rs、via_on_container_and_field.rs、named.rs、named_via.rs、override_rejection.rs、override_rejection_with_via_on_struct.rs、state_via.rs

### 宏系统/FromRequest 相关宏/容器级属性配置.md

- reference 标题：容器级属性配置
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：62
- 行数：279 / 49
- 段落行数：91 / 12
- Mermaid：6 / 0
- 文件提及重合：mod.rs
- reference 关键文件未覆盖：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、via_on_container_and_field.rs、named_via.rs、override_rejection.rs、state_enum_via.rs、state_field_infer.rs

### 宏系统/FromRequest 相关宏/泛型支持与限制.md

- reference 标题：泛型支持与限制
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：70
- 行数：255 / 49
- 段落行数：97 / 12
- Mermaid：5 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：attr.rs、generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、named_via.rs、state_enum_via.rs、state_infer.rs、state_via.rs

### 宏系统/FromRequest 相关宏/错误处理与调试.md

- reference 标题：错误处理与调试
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：102
- 行数：320 / 49
- 段落行数：114 / 12
- Mermaid：6 / 0
- 文件提及重合：debug_handler.rs、mod.rs、lib.rs
- reference 关键文件未覆盖：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：attr.rs、generic_without_via.rs、state_infer_multiple_different_types.rs、unknown_attr_container.rs、state_infer.rs、debugging_handler_type_errors.md、error_handling.md

### 宏系统/TypedPath 宏.md

- reference 标题：TypedPath 宏
- 生成页：无
- 问题：缺少对应生成页面

### 宏系统/debug_handler 宏.md

- reference 标题：debug_handler 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：72
- 行数：264 / 49
- 段落行数：66 / 12
- Mermaid：4 / 0
- 文件提及重合：debug_handler.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs、multiple_extractors.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、duplicate_args.rs、generics.rs、not_async.rs、too_many_extractors.rs、wrong_return_type.rs、infer_state.rs、multiple_extractors.rs

### 宏系统/debug_middleware 宏.md

- reference 标题：debug_middleware 宏
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：148
- 行数：192 / 49
- 段落行数：85 / 12
- Mermaid：5 / 0
- 文件提及重合：debug_handler.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、doesnt_take_next.rs、next_not_last.rs、takes_next_twice.rs、basic.rs

### 宏系统/宏测试与调试.md

- reference 标题：宏测试与调试
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：242
- 行数：324 / 49
- 段落行数：98 / 12
- Mermaid：5 / 0
- 文件提及重合：axum_test.rs、debug_handler.rs、mod.rs、lib.rs、typed_path.rs
- reference 关键文件未覆盖：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、not_a_function.rs、not_async.rs、wrong_return_type.rs、infer_state.rs、generic.rs、container.rs

### 宏系统/宏系统.md

- reference 标题：宏系统
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：286
- 行数：317 / 49
- 段落行数：95 / 12
- Mermaid：6 / 0
- 文件提及重合：axum_test.rs、debug_handler.rs、from_ref.rs、mod.rs、lib.rs、typed_path.rs
- reference 关键文件未覆盖：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、self_receiver.rs、basic.rs、state_infer.rs、named_fields_struct.rs

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化.md

- reference 标题：性能优化
- 生成页：无
- 问题：缺少对应生成页面

### 扩展开发.md

- reference 标题：扩展开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：287 / 35
- 段落行数：101 / 9
- Mermaid：6 / 0
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
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/内置提取器详解/原始查询和表单提取器.md

- reference 标题：原始查询和表单提取器
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/提取器系统.md

- reference 标题：提取器系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：90
- 行数：347 / 44
- 段落行数：180 / 10
- Mermaid：11 / 0
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：mod.rs、connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：mod.rs、connect_info.rs、multipart.rs、query.rs、raw_form.rs、raw_query.rs、rejection.rs、state.rs

### 提取器系统/提取器组合与高级用法.md

- reference 标题：提取器组合与高级用法
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：62
- 行数：337 / 43
- 段落行数：153 / 10
- Mermaid：8 / 0
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：option.rs、extract.md、de.rs、query.rs、rejection.rs、state.rs

### 提取器系统/提取器错误处理.md

- reference 标题：提取器错误处理
- 生成页：无
- 问题：缺少对应生成页面

### 提取器系统/自定义提取器实现.md

- reference 标题：自定义提取器实现
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：66
- 行数：329 / 43
- 段落行数：183 / 10
- Mermaid：10 / 0
- 文件提及重合：mod.rs、tuple.rs
- reference 关键文件未覆盖：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、attr.rs、extract.md、query.rs、state.rs、main.rs、custom_extractor.rs、derive_from_request.rs

### 故障排除与常见问题/常见错误与解决方案.md

- reference 标题：常见错误与解决方案
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：164
- 行数：237 / 49
- 段落行数：65 / 12
- Mermaid：3 / 0
- 文件提及重合：debug_handler.rs、mod.rs、typed_path.rs
- reference 关键文件未覆盖：self_receiver.rs、container.rs、named_fields_struct.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：self_receiver.rs、container.rs、named_fields_struct.rs

### 故障排除与常见问题/性能问题排查.md

- reference 标题：性能问题排查
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：106
- 行数：338 / 47
- 段落行数：154 / 11
- Mermaid：8 / 0
- 文件提及重合：mod.rs、lib.rs、method_routing.rs、path_router.rs
- reference 关键文件未覆盖：cargo.toml、benches.rs、util.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、benches.rs、util.rs、main.rs

### 故障排除与常见问题/故障排除与常见问题.md

- reference 标题：故障排除与常见问题
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：84
- 行数：287 / 43
- 段落行数：98 / 10
- Mermaid：6 / 0
- 文件提及重合：cargo.toml、lib.rs
- reference 关键文件未覆盖：contributing.md、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、debugging_handler_type_errors.md、error_handling.md、extract.md、middleware.md、response.md

### 故障排除与常见问题/社区支持与问题报告.md

- reference 标题：社区支持与问题报告
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除与常见问题/调试工具与技巧.md

- reference 标题：调试工具与技巧
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：84
- 行数：287 / 49
- 段落行数：130 / 12
- Mermaid：7 / 0
- 文件提及重合：axum_test.rs、debug_handler.rs、mod.rs
- reference 关键文件未覆盖：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、debugging_handler_type_errors.md、error_handling.md、from_fn.rs、test_client.rs、tracing_helpers.rs、main.rs

### 服务器集成.md

- reference 标题：服务器集成
- 生成页：无
- 问题：缺少对应生成页面

### 核心概念/Tower 服务抽象.md

- reference 标题：Tower 服务抽象
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：164
- 行数：289 / 47
- 段落行数：149 / 11
- Mermaid：8 / 0
- 文件提及重合：lib.rs、mod.rs、service.rs、route.rs、service_ext.rs
- reference 关键文件未覆盖：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：middleware.md、from_extractor.rs、from_fn.rs、response_axum_body.rs、main.rs

### 核心概念/异步运行时模型.md

- reference 标题：异步运行时模型
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：82
- 行数：260 / 47
- 段落行数：111 / 11
- Mermaid：5 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、body.rs、file_stream.rs、macros.rs、listener.rs、util.rs、main.rs、client.rs

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：112
- 行数：341 / 43
- 段落行数：145 / 10
- Mermaid：7 / 0
- 文件提及重合：cargo.toml、mod.rs、lib.rs
- reference 关键文件未覆盖：rejection.rs、state.rs、method_routing.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：rejection.rs、state.rs、method_routing.rs

### 核心概念/模块化架构.md

- reference 标题：模块化架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：198
- 行数：324 / 44
- 段落行数：131 / 10
- Mermaid：6 / 0
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、mod.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、mod.rs

### 核心概念/类型安全设计.md

- reference 标题：类型安全设计
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：158
- 行数：282 / 43
- 段落行数：136 / 10
- Mermaid：8 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：debug_handler.rs、attr.rs、error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：debug_handler.rs、attr.rs、error_handling.md、rejection.rs、custom_extractor.rs、derive_from_request.rs、main.rs、with_rejection.rs

### 核心概念/请求生命周期.md

- reference 标题：请求生命周期
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：134
- 行数：308 / 47
- 段落行数：150 / 11
- Mermaid：8 / 0
- 文件提及重合：lib.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：extract.md、handlers_intro.md、middleware.md、from_fn.rs、main.rs

### 测试策略.md

- reference 标题：测试策略
- 生成页：核心模块/axum-macros.md（模块：axum-macros）
- 匹配分数：72
- 行数：327 / 49
- 段落行数：120 / 12
- Mermaid：6 / 0
- 文件提及重合：axum_test.rs、lib.rs、mod.rs
- reference 关键文件未覆盖：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：benches.rs、connect_info.rs、fallback.rs、merge.rs、counting_cloneable_state.rs、test_client.rs、tracing_helpers.rs、main.rs

### 状态管理.md

- reference 标题：状态管理
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：90
- 行数：256 / 43
- 段落行数：94 / 10
- Mermaid：5 / 0
- 文件提及重合：mod.rs、lib.rs
- reference 关键文件未覆盖：from_ref.rs、with_state.md、extension.rs、state.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：from_ref.rs、with_state.md、extension.rs、state.rs

### 示例展示/Web 开发示例.md

- reference 标题：Web 开发示例
- 生成页：无
- 问题：缺少对应生成页面

### 示例展示/基础示例.md

- reference 标题：基础示例
- 生成页：核心模块/axum-core.md（模块：axum-core）
- 匹配分数：108
- 行数：335 / 43
- 段落行数：143 / 10
- Mermaid：8 / 0
- 文件提及重合：mod.rs、lib.rs、cargo.toml
- reference 关键文件未覆盖：method_routing.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：method_routing.rs、main.rs

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
- 匹配分数：170
- 行数：380 / 49
- 段落行数：151 / 12
- Mermaid：9 / 0
- 文件提及重合：mod.rs、lib.rs、axum_test.rs、debug_handler.rs、typed_path.rs
- reference 关键文件未覆盖：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.clippy.toml、pull_request_template.md、contributing.md、cargo.toml、ecosystem.md、body.rs、error.rs、middleware.rs

### 路由系统/Router 基础概念.md

- reference 标题：Router 基础概念
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：82
- 行数：293 / 47
- 段落行数：132 / 11
- Mermaid：5 / 0
- 文件提及重合：mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：with_state.md、not_found.rs、fallback.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：with_state.md、not_found.rs、fallback.rs、main.rs

### 路由系统/回退处理.md

- reference 标题：回退处理
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：82
- 行数：244 / 47
- 段落行数：95 / 11
- Mermaid：3 / 0
- 文件提及重合：mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：fallback.md、merge.md、nest.md、not_found.rs、fallback.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、nest.md、not_found.rs、fallback.rs、main.rs

### 路由系统/嵌套路由.md

- reference 标题：嵌套路由
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：74
- 行数：255 / 47
- 段落行数：95 / 11
- Mermaid：7 / 0
- 文件提及重合：mod.rs、path_router.rs
- reference 关键文件未覆盖：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、nested_path.rs、from_fn.rs、strip_prefix.rs、nest.rs、main.rs

### 路由系统/方法路由.md

- reference 标题：方法路由
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：82
- 行数：287 / 47
- 段落行数：102 / 11
- Mermaid：5 / 0
- 文件提及重合：method_routing.rs、mod.rs、route.rs
- reference 关键文件未覆盖：fallback.md、merge.md、route_layer.md、method_filter.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fallback.md、merge.md、route_layer.md、method_filter.rs

### 路由系统/路径路由.md

- reference 标题：路径路由
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：88
- 行数：287 / 47
- 段落行数：107 / 11
- Mermaid：5 / 0
- 文件提及重合：mod.rs、method_routing.rs、path_router.rs
- reference 关键文件未覆盖：nest.md、route.md、de.rs、nest.rs、url_params.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nest.md、route.md、de.rs、nest.rs、url_params.rs

### 路由系统/路由合并.md

- reference 标题：路由合并
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：84
- 行数：253 / 47
- 段落行数：112 / 11
- Mermaid：5 / 0
- 文件提及重合：method_routing.rs、mod.rs、path_router.rs
- reference 关键文件未覆盖：cargo.toml、merge.md、merge.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、merge.md、merge.rs

### 路由系统/路由定义与注册.md

- reference 标题：路由定义与注册
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：146
- 行数：295 / 47
- 段落行数：102 / 11
- Mermaid：6 / 0
- 文件提及重合：lib.rs、future.rs、method_routing.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：route.md、route_service.md、not_found.rs、merge.rs、nest.rs、main.rs

### 路由系统/路由系统.md

- reference 标题：路由系统
- 生成页：核心模块/axum.md（模块：axum）
- 匹配分数：128
- 行数：228 / 47
- 段落行数：94 / 11
- Mermaid：3 / 0
- 文件提及重合：lib.rs、future.rs、method_routing.rs、mod.rs、path_router.rs、route.rs
- reference 关键文件未覆盖：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：merge.md、nest.md、route.md、method_filter.rs、not_found.rs、strip_prefix.rs、url_params.rs

### 部署运维.md

- reference 标题：部署运维
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：90
- 行数：396 / 45
- 段落行数：183 / 11
- Mermaid：12 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：cargo.toml、main.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、main.rs

### 错误处理.md

- reference 标题：错误处理
- 生成页：无
- 问题：缺少对应生成页面

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：658
- 行数：234 / 35
- 段落行数：101 / 9
- Mermaid：6 / 0
- 文件提及重合：lib.rs
- reference 关键文件未覆盖：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、ecosystem.md、readme.md、extract.md、mod.rs、service_ext.rs

