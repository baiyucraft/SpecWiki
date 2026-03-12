# django-ninja Reference 对比报告

生成页面：18 页
reference 页面：47 页
命中对比：35 页
缺失对比：12 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=30, total_tokens=194430, page_research=14, page_enrichment=2

## 覆盖统计

- 专题页覆盖：generated 12 / reference 9（repo-archetype=4）
- evidence 落页：generated 13 / reference 47
- citation 密度：generated 0.89 / reference 77.85
- 图表达覆盖：generated 9 / reference 47
- page research 请求：14
- page enrichment 请求：2
- 已规划专题类型：流程主题(8)、专题页(4)
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

- 核心模块/ninja.md 被 21 个 reference 页面共享映射
- 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md 被 10 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-515a3133ef5f-library-ninja能力：扩展机制.md (ninja能力：扩展机制, 24 行)
- 专题/process/process-handle-flow-流程主题：handle-flow.md (流程主题：handle flow, 23 行)
- 专题/process/process-sync_view_wrapper-flow-流程主题：sync_view_wrapper-flow.md (流程主题：sync_view_wrapper flow, 79 行)
- 专题/process/process-test_async_view_handles_async_auth_cls-flow-流程主题：test_async_view_handles_async_auth_cls-flow.md (流程主题：test_async_view_handles_async_auth_cls flow, 23 行)
- 专题/process/process-test_asyncio_operations-flow-流程主题：test_asyncio_operations-flow.md (流程主题：test_asyncio_operations flow, 37 行)
- 专题/process/process-test_sync_authenticate_method-flow-流程主题：test_sync_authenticate_method-flow.md (流程主题：test_sync_authenticate_method flow, 50 行)
- 专题/repo-archetype/config-runtime-配置与运行时.md (配置与运行时, 20 行)
- 专题/repo-archetype/core-api-models-核心-API-与数据模型.md (核心 API 与数据模型, 17 行)
- 专题/repo-archetype/ops-runtime-部署与环境.md (部署与环境, 17 行)
- 专题/repo-archetype/request-lifecycle-请求处理链.md (请求处理链, 20 行)
- 核心模块/docs.md (模块：docs, 60 行)
- 系统架构.md (系统架构, 46 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API 参考/API 参考.md | 核心模块/ninja.md | 424/54 | 131/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.py、decorators.py、errors.py、pagination.py、responses.py、base.py、types.py |
| API 参考/常量与枚举.md | 核心模块/ninja.md | 350/54 | 117/7 | 16/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：constants.py、filter_schema.py、responses.py、types.py、test_enum.py、test_query_schema.py、test_response.py、test_status.py |
| API 参考/核心 API 类.md | 核心模块/ninja.md | 521/54 | 246/7 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：conf.py、constants.py、decorators.py、errors.py、utils.py、types.py、test_api_instance.py、test_router_add_router.py |
| API 参考/管理命令.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API 参考/装饰器系统.md | 核心模块/ninja.md | 380/54 | 163/7 | 18/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、throttling.md、decorators.py、throttling.py、test_add_decorator.py、test_decorators.py、test_throttling.py |
| API 参考/配置选项.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 扩展与集成.md | 核心模块/ninja.md | 273/54 | 126/7 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、errors.py、utils.py、pyproject.toml、test_add_decorator.py、test_add_decorator_async.py |
| 故障排除与常见问题.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 文档与规范.md | 核心模块/ninja.md | 331/54 | 129/7 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api-docs.md、mkdocs.yml、conf.py、swagger-ui-init.js、test_openapi_docs.py、openapi.js |
| 核心概念/NinjaAPI 实例管理.md | 核心模块/ninja.md | 323/54 | 117/7 | 20/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：conf.py、throttling.py、test_api_instance.py、openapi.js、ninja.conf |
| 核心概念/Operation 执行机制.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 408/33 | 162/14 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、operation.py、renderers.py、responses.py、details.py、utils.py、test_async.py、test_body.py |
| 核心概念/Router 路由系统/Router 路由系统.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 348/33 | 126/14 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、urls.md、__init__.py、decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py |
| 核心概念/Router 路由系统/URL 生成与命名.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 287/33 | 111/14 | 17/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：urls.md、operation.py、utils.py、test_api_instance.py、test_reverse.py、test_router_reuse.py |
| 核心概念/Router 路由系统/端点注册与装饰器.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 313/33 | 124/14 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_decorators.py |
| 核心概念/Router 路由系统/装饰器继承与传递.md | 专题/process/process-async_view_wrapper-flow-流程主题：async_view_wrapper-flow.md | 244/73 | 85/50 | 0/2 | 4/1 | 内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_auth_inheritance_routers.py、test_inheritance_routers.py |
| 核心概念/Router 路由系统/路由创建与配置.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 305/33 | 99/14 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、throttling.md、test_router_add_router.py、test_router_defaults.py、test_router_reuse.py、test_throttling.py |
| 核心概念/Router 路由系统/路由挂载与嵌套.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 327/33 | 115/14 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、operation.py、test_inheritance_routers.py、test_router_add_router.py、test_router_defaults.py、test_router_path_params.py、test_router_reuse.py |
| 核心概念/Router 路由系统/路由构建与绑定.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 302/33 | 141/14 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py、test_router_add_router.py |
| 核心概念/核心概念.md | 核心模块/ninja.md | 432/54 | 175/7 | 21/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、utils.py、types.py、test_api_instance.py、test_decorators.py、test_router_add_router.py |
| 核心概念/装饰器系统.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 293/33 | 119/14 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_decorators.py |
| 测试与调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证与授权/内置认证方式.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证与授权/权限控制.md | 核心模块/ninja.md | 299/54 | 103/7 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：authentication.md、apikey01.py、bearer01.py、global01.py、multiple01.py、decorators.py、apikey.py、base.py |
| 认证与授权/自定义认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 认证与授权/认证与授权.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 367/33 | 163/14 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：authentication.md、apikey01.py、apikey02.py、apikey03.py、basic01.py、bearer01.py、__init__.py、apikey.py |
| 认证与授权/认证基础.md | 核心模块/ninja.md | 363/54 | 181/7 | 0/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：apikey.py、base.py、http.py、session.py、test_auth.py、test_auth_async.py、apikey01.py、basic01.py |
| 输入处理/文件参数 (File).md | 核心模块/ninja.md | 298/54 | 125/7 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：file-params.md、files.py、conf.py、test_files.py、test_forms_and_files.py |
| 输入处理/查询参数 (Query).md | 核心模块/ninja.md | 233/54 | 60/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：functions.py、test_lists.py、test_parser.py、test_query.py、filtering.md、query-params.md、code01.py、code010.py |
| 输入处理/表单参数 (Form).md | 核心模块/ninja.md | 283/54 | 117/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：form-params.md、code01.py、code02.py、code03.py、functions.py、test_forms.py、test_forms_and_files.py、test_multi_param_parsing.py |
| 输入处理/请求体参数 (Body).md | 核心模块/ninja.md | 256/54 | 93/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：body.md、request-parsers.md、code01.py、code02.py、code03.py、functions.py、details.py、test_body.py |
| 输入处理/请求头与Cookie参数.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 输入处理/路径参数 (Path).md | 核心模块/ninja.md | 272/54 | 119/7 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：functions.py、details.py、test_path.py、test_router_path_params.py、code01.py、code010.py、code02.py、routers.md |
| 输入处理/输入处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 输出处理/响应对象.md | 核心模块/ninja.md | 290/54 | 102/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、response-renderers.md、temporal_response.md、responses.py、test_response.py、test_response_cookies.py、test_response_multiple.py、test_response_params.py |
| 输出处理/响应渲染器.md | 核心模块/ninja.md | 240/54 | 90/7 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：response-renderers.md、responses.py、test_renderer.py |
| 输出处理/数据序列化.md | 核心模块/ninja.md | 239/54 | 87/7 | 17/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.js、errors.py、responses.py、streaming.py、test_errors.py、test_response.py、test_serialization_context.py、config-pydantic.md |
| 输出处理/状态码处理.md | 核心模块/ninja.md | 260/54 | 93/7 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：errors.md、temporal_response.md、errors.py、responses.py、test_status.py |
| 输出处理/输出处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 部署与生产.md | 工作流与部署.md | 308/92 | 139/12 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：test_full.yml、readme.md、csrf.md、settings.md、conf.py、throttling.py、pyproject.toml、asgi.py |
| 项目概述.md | 项目概述.md | 309/70 | 152/12 | 0/0 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、motivation.md、__init__.py、conf.py、schema.py、pyproject.toml、urls.py |
| 高级功能/分页功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/异步支持.md | 核心模块/ninja.md | 250/54 | 121/7 | 15/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：async-support.md、decorators.py、test_async.py、test_auth_async.py、test_pagination_async.py、manage.py |
| 高级功能/流式响应.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 高级功能/版本控制.md | 专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md | 260/33 | 123/14 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：urls.md、versioning.md、export_openapi_schema.py、schema.py、urls.py、api.py、test_api_instance.py、test_openapi_extra.py |
| 高级功能/限流机制.md | 专题/process/process-test_async_auth-flow-流程主题：test_async_auth-flow.md | 277/43 | 100/22 | 0/1 | 4/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：throttling.md、conf.py、errors.py、operation.py、throttling.py、test_throttling.py |
| 高级功能/高级功能.md | 核心模块/ninja.md | 359/54 | 153/7 | 17/1 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：async-support.md、throttling.md、versioning.md、conf.py、pagination.py、streaming.py、throttling.py、test_async.py |

## 逐文件详情

### API 参考/API 参考.md

- reference 标题：API 参考
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：146
- 页面类型：other / module
- 行数：424 / 54
- 段落行数：131 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：__init__.py、main.py、schema.py、operation.py、router.py
- reference 关键文件未覆盖：constants.py、decorators.py、errors.py、pagination.py、responses.py、base.py、types.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：constants.py、decorators.py、errors.py、pagination.py、responses.py、base.py、types.py

### API 参考/常量与枚举.md

- reference 标题：常量与枚举
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：66
- 页面类型：other / module
- 行数：350 / 54
- 段落行数：117 / 7
- Evidence：16 / 1
- Mermaid：7 / 0
- 文件提及重合：__init__.py、main.py
- reference 关键文件未覆盖：constants.py、filter_schema.py、responses.py、types.py、test_enum.py、test_query_schema.py、test_response.py、test_status.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：constants.py、filter_schema.py、responses.py、types.py、test_enum.py、test_query_schema.py、test_response.py、test_status.py

### API 参考/核心 API 类.md

- reference 标题：核心 API 类
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：126
- 页面类型：other / module
- 行数：521 / 54
- 段落行数：246 / 7
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：__init__.py、main.py、operation.py、router.py
- reference 关键文件未覆盖：conf.py、constants.py、decorators.py、errors.py、utils.py、types.py、test_api_instance.py、test_router_add_router.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：conf.py、constants.py、decorators.py、errors.py、utils.py、types.py、test_api_instance.py、test_router_add_router.py

### API 参考/管理命令.md

- reference 标题：管理命令
- 生成页：无
- 问题：缺少对应生成页面

### API 参考/装饰器系统.md

- reference 标题：装饰器系统
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：94
- 页面类型：other / module
- 行数：380 / 54
- 段落行数：163 / 7
- Evidence：18 / 1
- Mermaid：7 / 0
- 文件提及重合：main.py、operation.py、router.py
- reference 关键文件未覆盖：decorators.md、throttling.md、decorators.py、throttling.py、test_add_decorator.py、test_decorators.py、test_throttling.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、throttling.md、decorators.py、throttling.py、test_add_decorator.py、test_decorators.py、test_throttling.py

### API 参考/配置选项.md

- reference 标题：配置选项
- 生成页：无
- 问题：缺少对应生成页面

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 扩展与集成.md

- reference 标题：扩展与集成
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：120
- 页面类型：other / module
- 行数：273 / 54
- 段落行数：126 / 7
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：__init__.py、main.py、operation.py、router.py
- reference 关键文件未覆盖：decorators.md、decorators.py、errors.py、utils.py、pyproject.toml、test_add_decorator.py、test_add_decorator_async.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、errors.py、utils.py、pyproject.toml、test_add_decorator.py、test_add_decorator_async.py

### 故障排除与常见问题.md

- reference 标题：故障排除与常见问题
- 生成页：无
- 问题：缺少对应生成页面

### 文档与规范.md

- reference 标题：文档与规范
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：158
- 页面类型：other / module
- 行数：331 / 54
- 段落行数：129 / 7
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.py、export_openapi_schema.py、docs.py、schema.py、urls.py、views.py
- reference 关键文件未覆盖：api-docs.md、mkdocs.yml、conf.py、swagger-ui-init.js、test_openapi_docs.py、openapi.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：api-docs.md、mkdocs.yml、conf.py、swagger-ui-init.js、test_openapi_docs.py、openapi.js

### 核心概念/NinjaAPI 实例管理.md

- reference 标题：NinjaAPI 实例管理
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：184
- 页面类型：other / module
- 行数：323 / 54
- 段落行数：117 / 7
- Evidence：20 / 1
- Mermaid：6 / 0
- 文件提及重合：__init__.py、main.py、schema.py、urls.py、parser.py、renderers.py、router.py
- reference 关键文件未覆盖：conf.py、throttling.py、test_api_instance.py、openapi.js、ninja.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：conf.py、throttling.py、test_api_instance.py、openapi.js、ninja.conf

### 核心概念/Operation 执行机制.md

- reference 标题：Operation 执行机制
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：158
- 页面类型：topic / topic
- 行数：408 / 33
- 段落行数：162 / 14
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：decorators.py、operation.py、renderers.py、responses.py、details.py、utils.py、test_async.py、test_body.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、operation.py、renderers.py、responses.py、details.py、utils.py、test_async.py、test_body.py

### 核心概念/Router 路由系统/Router 路由系统.md

- reference 标题：Router 路由系统
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：158
- 页面类型：topic / topic
- 行数：348 / 33
- 段落行数：126 / 14
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：routers.md、urls.md、__init__.py、decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、urls.md、__init__.py、decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py

### 核心概念/Router 路由系统/URL 生成与命名.md

- reference 标题：URL 生成与命名
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：194
- 页面类型：topic / topic
- 行数：287 / 33
- 段落行数：111 / 14
- Evidence：17 / 1
- Mermaid：5 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：urls.md、operation.py、utils.py、test_api_instance.py、test_reverse.py、test_router_reuse.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：urls.md、operation.py、utils.py、test_api_instance.py、test_reverse.py、test_router_reuse.py

### 核心概念/Router 路由系统/端点注册与装饰器.md

- reference 标题：端点注册与装饰器
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：152
- 页面类型：topic / topic
- 行数：313 / 33
- 段落行数：124 / 14
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_decorators.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_decorators.py

### 核心概念/Router 路由系统/装饰器继承与传递.md

- reference 标题：装饰器继承与传递
- 生成页：专题/process/process-async_view_wrapper-flow-流程主题：async_view_wrapper-flow.md（流程主题：async_view_wrapper flow）
- 匹配分数：156
- 页面类型：topic / topic
- 行数：244 / 73
- 段落行数：85 / 50
- Evidence：0 / 2
- Mermaid：4 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_auth_inheritance_routers.py、test_inheritance_routers.py
- 结论：内容明显短于 reference；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_auth_inheritance_routers.py、test_inheritance_routers.py

### 核心概念/Router 路由系统/路由创建与配置.md

- reference 标题：路由创建与配置
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：152
- 页面类型：topic / topic
- 行数：305 / 33
- 段落行数：99 / 14
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：routers.md、throttling.md、test_router_add_router.py、test_router_defaults.py、test_router_reuse.py、test_throttling.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、throttling.md、test_router_add_router.py、test_router_defaults.py、test_router_reuse.py、test_throttling.py

### 核心概念/Router 路由系统/路由挂载与嵌套.md

- reference 标题：路由挂载与嵌套
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：192
- 页面类型：topic / topic
- 行数：327 / 33
- 段落行数：115 / 14
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：routers.md、operation.py、test_inheritance_routers.py、test_router_add_router.py、test_router_defaults.py、test_router_path_params.py、test_router_reuse.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：routers.md、operation.py、test_inheritance_routers.py、test_router_add_router.py、test_router_defaults.py、test_router_path_params.py、test_router_reuse.py

### 核心概念/Router 路由系统/路由构建与绑定.md

- reference 标题：路由构建与绑定
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：188
- 页面类型：topic / topic
- 行数：302 / 33
- 段落行数：141 / 14
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py、test_router_add_router.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、operation.py、test_api_instance.py、test_auth_inheritance_routers.py、test_inheritance_routers.py、test_router_add_router.py

### 核心概念/核心概念.md

- reference 标题：核心概念
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：178
- 页面类型：other / module
- 行数：432 / 54
- 段落行数：175 / 7
- Evidence：21 / 1
- Mermaid：9 / 0
- 文件提及重合：__init__.py、main.py、schema.py、operation.py、models.py、router.py
- reference 关键文件未覆盖：decorators.py、utils.py、types.py、test_api_instance.py、test_decorators.py、test_router_add_router.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.py、utils.py、types.py、test_api_instance.py、test_decorators.py、test_router_add_router.py

### 核心概念/装饰器系统.md

- reference 标题：装饰器系统
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：112
- 页面类型：other / topic
- 行数：293 / 33
- 段落行数：119 / 14
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_decorators.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：decorators.md、decorators.py、operation.py、utils.py、test_add_decorator.py、test_add_decorator_async.py、test_decorators.py

### 测试与调试.md

- reference 标题：测试与调试
- 生成页：无
- 问题：缺少对应生成页面

### 认证与授权/内置认证方式.md

- reference 标题：内置认证方式
- 生成页：无
- 问题：缺少对应生成页面

### 认证与授权/权限控制.md

- reference 标题：权限控制
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：100
- 页面类型：other / module
- 行数：299 / 54
- 段落行数：103 / 7
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：main.py、operation.py、router.py、__init__.py
- reference 关键文件未覆盖：authentication.md、apikey01.py、bearer01.py、global01.py、multiple01.py、decorators.py、apikey.py、base.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：authentication.md、apikey01.py、bearer01.py、global01.py、multiple01.py、decorators.py、apikey.py、base.py

### 认证与授权/自定义认证.md

- reference 标题：自定义认证
- 生成页：无
- 问题：缺少对应生成页面

### 认证与授权/认证与授权.md

- reference 标题：认证与授权
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：68
- 页面类型：other / topic
- 行数：367 / 33
- 段落行数：163 / 14
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：main.py
- reference 关键文件未覆盖：authentication.md、apikey01.py、apikey02.py、apikey03.py、basic01.py、bearer01.py、__init__.py、apikey.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：authentication.md、apikey01.py、apikey02.py、apikey03.py、basic01.py、bearer01.py、__init__.py、apikey.py

### 认证与授权/认证基础.md

- reference 标题：认证基础
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：74
- 页面类型：other / module
- 行数：363 / 54
- 段落行数：181 / 7
- Evidence：0 / 1
- Mermaid：8 / 0
- 文件提及重合：main.py、operation.py、__init__.py
- reference 关键文件未覆盖：apikey.py、base.py、http.py、session.py、test_auth.py、test_auth_async.py、apikey01.py、basic01.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：apikey.py、base.py、http.py、session.py、test_auth.py、test_auth_async.py、apikey01.py、basic01.py

### 输入处理/文件参数 (File).md

- reference 标题：文件参数（File）
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：92
- 页面类型：other / module
- 行数：298 / 54
- 段落行数：125 / 7
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：__init__.py、models.py、parser.py
- reference 关键文件未覆盖：file-params.md、files.py、conf.py、test_files.py、test_forms_and_files.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：file-params.md、files.py、conf.py、test_files.py、test_forms_and_files.py

### 输入处理/查询参数 (Query).md

- reference 标题：查询参数（Query）
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：70
- 页面类型：other / module
- 行数：233 / 54
- 段落行数：60 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：__init__.py、models.py、parser.py
- reference 关键文件未覆盖：functions.py、test_lists.py、test_parser.py、test_query.py、filtering.md、query-params.md、code01.py、code010.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：functions.py、test_lists.py、test_parser.py、test_query.py、filtering.md、query-params.md、code01.py、code010.py

### 输入处理/表单参数 (Form).md

- reference 标题：表单参数（Form）
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：72
- 页面类型：other / module
- 行数：283 / 54
- 段落行数：117 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：__init__.py、models.py、parser.py
- reference 关键文件未覆盖：form-params.md、code01.py、code02.py、code03.py、functions.py、test_forms.py、test_forms_and_files.py、test_multi_param_parsing.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：form-params.md、code01.py、code02.py、code03.py、functions.py、test_forms.py、test_forms_and_files.py、test_multi_param_parsing.py

### 输入处理/请求体参数 (Body).md

- reference 标题：请求体参数（Body）
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：96
- 页面类型：other / module
- 行数：256 / 54
- 段落行数：93 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.py、schema.py、models.py、parser.py
- reference 关键文件未覆盖：body.md、request-parsers.md、code01.py、code02.py、code03.py、functions.py、details.py、test_body.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：body.md、request-parsers.md、code01.py、code02.py、code03.py、functions.py、details.py、test_body.py

### 输入处理/请求头与Cookie参数.md

- reference 标题：请求头与Cookie参数
- 生成页：无
- 问题：缺少对应生成页面

### 输入处理/路径参数 (Path).md

- reference 标题：路径参数（Path）
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：74
- 页面类型：other / module
- 行数：272 / 54
- 段落行数：119 / 7
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：main.py、__init__.py、models.py
- reference 关键文件未覆盖：functions.py、details.py、test_path.py、test_router_path_params.py、code01.py、code010.py、code02.py、routers.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：functions.py、details.py、test_path.py、test_router_path_params.py、code01.py、code010.py、code02.py、routers.md

### 输入处理/输入处理.md

- reference 标题：输入处理
- 生成页：无
- 问题：缺少对应生成页面

### 输出处理/响应对象.md

- reference 标题：响应对象
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：90
- 页面类型：other / module
- 行数：290 / 54
- 段落行数：102 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.py、operation.py、renderers.py
- reference 关键文件未覆盖：index.md、response-renderers.md、temporal_response.md、responses.py、test_response.py、test_response_cookies.py、test_response_multiple.py、test_response_params.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：index.md、response-renderers.md、temporal_response.md、responses.py、test_response.py、test_response_cookies.py、test_response_multiple.py、test_response_params.py

### 输出处理/响应渲染器.md

- reference 标题：响应渲染器
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：62
- 页面类型：other / module
- 行数：240 / 54
- 段落行数：90 / 7
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：main.py、renderers.py
- reference 关键文件未覆盖：response-renderers.md、responses.py、test_renderer.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：response-renderers.md、responses.py、test_renderer.py

### 输出处理/数据序列化.md

- reference 标题：数据序列化
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：88
- 页面类型：other / module
- 行数：239 / 54
- 段落行数：87 / 7
- Evidence：17 / 1
- Mermaid：4 / 0
- 文件提及重合：main.py、renderers.py、schema.py
- reference 关键文件未覆盖：.js、errors.py、responses.py、streaming.py、test_errors.py、test_response.py、test_serialization_context.py、config-pydantic.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.js、errors.py、responses.py、streaming.py、test_errors.py、test_response.py、test_serialization_context.py、config-pydantic.md

### 输出处理/状态码处理.md

- reference 标题：状态码处理
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：64
- 页面类型：other / module
- 行数：260 / 54
- 段落行数：93 / 7
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：main.py、operation.py
- reference 关键文件未覆盖：errors.md、temporal_response.md、errors.py、responses.py、test_status.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：errors.md、temporal_response.md、errors.py、responses.py、test_status.py

### 输出处理/输出处理.md

- reference 标题：输出处理
- 生成页：无
- 问题：缺少对应生成页面

### 部署与生产.md

- reference 标题：部署与生产
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：136
- 页面类型：workflow / workflow
- 行数：308 / 92
- 段落行数：139 / 12
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：docker-compose.yml
- reference 关键文件未覆盖：test_full.yml、readme.md、csrf.md、settings.md、conf.py、throttling.py、pyproject.toml、asgi.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：test_full.yml、readme.md、csrf.md、settings.md、conf.py、throttling.py、pyproject.toml、asgi.py

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：638
- 页面类型：overview / overview
- 行数：309 / 70
- 段落行数：152 / 12
- Evidence：0 / 0
- Mermaid：7 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：contributing.md、readme.md、motivation.md、__init__.py、conf.py、schema.py、pyproject.toml、urls.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、motivation.md、__init__.py、conf.py、schema.py、pyproject.toml、urls.py

### 高级功能/分页功能.md

- reference 标题：分页功能
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/异步支持.md

- reference 标题：异步支持
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：122
- 页面类型：other / module
- 行数：250 / 54
- 段落行数：121 / 7
- Evidence：15 / 1
- Mermaid：5 / 0
- 文件提及重合：__init__.py、main.py、operation.py、router.py
- reference 关键文件未覆盖：async-support.md、decorators.py、test_async.py、test_auth_async.py、test_pagination_async.py、manage.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：async-support.md、decorators.py、test_async.py、test_auth_async.py、test_pagination_async.py、manage.py

### 高级功能/流式响应.md

- reference 标题：流式响应
- 生成页：无
- 问题：缺少对应生成页面

### 高级功能/版本控制.md

- reference 标题：版本控制
- 生成页：专题/process/process-test_async_authenticate_method_in_sync_context-flow-流程主题：test_async_authenticate_method_in_sync_context-flow.md（流程主题：test_async_authenticate_method_in_sync_context flow）
- 匹配分数：116
- 页面类型：other / topic
- 行数：260 / 33
- 段落行数：123 / 14
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：urls.md、versioning.md、export_openapi_schema.py、schema.py、urls.py、api.py、test_api_instance.py、test_openapi_extra.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：urls.md、versioning.md、export_openapi_schema.py、schema.py、urls.py、api.py、test_api_instance.py、test_openapi_extra.py

### 高级功能/限流机制.md

- reference 标题：限流机制
- 生成页：专题/process/process-test_async_auth-flow-流程主题：test_async_auth-flow.md（流程主题：test_async_auth flow）
- 匹配分数：156
- 页面类型：topic / topic
- 行数：277 / 43
- 段落行数：100 / 22
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：main.py、router.py
- reference 关键文件未覆盖：throttling.md、conf.py、errors.py、operation.py、throttling.py、test_throttling.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：throttling.md、conf.py、errors.py、operation.py、throttling.py、test_throttling.py

### 高级功能/高级功能.md

- reference 标题：高级功能
- 生成页：核心模块/ninja.md（模块：ninja）
- 匹配分数：64
- 页面类型：other / module
- 行数：359 / 54
- 段落行数：153 / 7
- Evidence：17 / 1
- Mermaid：8 / 0
- 文件提及重合：__init__.py、main.py
- reference 关键文件未覆盖：async-support.md、throttling.md、versioning.md、conf.py、pagination.py、streaming.py、throttling.py、test_async.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：async-support.md、throttling.md、versioning.md、conf.py、pagination.py、streaming.py、throttling.py、test_async.py

