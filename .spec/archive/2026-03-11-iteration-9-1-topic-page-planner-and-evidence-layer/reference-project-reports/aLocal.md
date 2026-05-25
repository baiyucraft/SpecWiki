# aLocal Reference 对比报告

生成页面：18 页
reference 页面：86 页
命中对比：72 页
缺失对比：14 页

## 覆盖统计

- 专题页覆盖：generated 11 / reference 5
- evidence 落页：generated 15 / reference 86
- 图表达覆盖：generated 11 / reference 86
- 已规划专题类型：流程主题(8)、专题页(3)
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

- 核心模块/spider.md 被 24 个 reference 页面共享映射
- 核心模块/web.md 被 16 个 reference 页面共享映射
- 系统架构.md 被 8 个 reference 页面共享映射
- 专题/process/process-api_links-flow-流程主题：api_links-flow.md 被 4 个 reference 页面共享映射
- 专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md 被 3 个 reference 页面共享映射
- 工作流与部署.md 被 10 个 reference 页面共享映射
- 项目概述.md 被 5 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-240f4d7393a9-plugin-nginx能力：扩展机制.md (nginx能力：扩展机制, 28 行)
- 专题/module-capability/module-5380ddae5fb7-service-web能力：服务协作.md (web能力：服务协作, 38 行)
- 专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md (流程主题：api_get_log flow, 43 行)
- 专题/process/process-api_news_find-flow-流程主题：api_news_find-flow.md (流程主题：api_news_find flow, 43 行)
- 专题/process/process-get_last_run_time-flow-流程主题：get_last_run_time-flow.md (流程主题：get_last_run_time flow, 35 行)
- 专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md (流程主题：run_feedback_cycle flow, 58 行)
- 专题/process/process-update_last_run_time-flow-流程主题：update_last_run_time-flow.md (流程主题：update_last_run_time flow, 27 行)
- 核心模块/nginx.md (模块：nginx, 57 行)
- 核心模块/nginx/conf.md (模块：conf, 42 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API接口文档/API接口文档.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/新闻API.md | 核心模块/spider.md | 377/71 | 101/13 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、index.ts、news.ts |
| API接口文档/爬虫API.md | 核心模块/spider.md | 323/71 | 111/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts |
| API接口文档/系统管理API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/转换API.md | 核心模块/spider.md | 363/71 | 146/13 | 19/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts |
| 前端开发指南/API集成.md | 核心模块/web.md | 380/74 | 163/15 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts |
| 前端开发指南/UI设计系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 前端开发指南/Vue应用架构.md | 系统架构.md | 434/59 | 166/19 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts |
| 前端开发指南/前端开发指南.md | 核心模块/web.md | 285/74 | 104/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 前端开发指南/状态管理.md | 核心模块/web.md | 302/74 | 120/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 前端开发指南/组件开发/UI组件集成.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 前端开发指南/组件开发/基础组件开发.md | 核心模块/web.md | 475/74 | 336/15 | 20/1 | 12/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 前端开发指南/组件开发/组件开发.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 374/33 | 127/8 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts |
| 前端开发指南/组件开发/高级组件模式.md | 核心模块/web.md | 328/74 | 130/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globalticker.ts、vite.config.ts |
| 前端开发指南/路由系统.md | 核心模块/web.md | 254/74 | 78/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 后端开发指南/API开发最佳实践.md | 核心模块/spider.md | 336/71 | 116/13 | 18/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、func.py、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts |
| 后端开发指南/Flask应用架构.md | 系统架构.md | 378/59 | 248/19 | 19/1 | 11/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py |
| 后端开发指南/后端开发指南.md | 核心模块/spider.md | 351/71 | 177/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、utils.py、links.js、async_fetch.py |
| 后端开发指南/数据库操作系统.md | 核心模块/spider.md | 346/71 | 145/13 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 后端开发指南/爬虫引擎系统/同步爬取机制.md | 专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md | 345/42 | 158/16 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py |
| 后端开发指南/爬虫引擎系统/异步爬取机制.md | 专题/module-capability/module-4c517846f747-repository-spider能力：数据访问.md | 432/36 | 248/12 | 18/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md |
| 后端开发指南/爬虫引擎系统/数据处理管道.md | 核心模块/spider.md | 375/71 | 142/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、func.py |
| 后端开发指南/爬虫引擎系统/爬虫引擎系统.md | 核心模块/spider.md | 375/71 | 158/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 后端开发指南/爬虫引擎系统/调度与定时机制.md | 专题/process/process-update_data-flow-流程主题：update_data-flow.md | 437/34 | 310/9 | 21/1 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、spec.md、design.md、add_sys_data_update_interval.sql |
| 后端开发指南/视频转换系统/MKV文件合并.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/任务管理系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/子进程集成机制.md | 专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md | 284/42 | 127/16 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 后端开发指南/视频转换系统/文件比较功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频压缩处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频转换系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发规范/Git工作流程.md | 工作流与部署.md | 235/38 | 101/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml |
| 开发规范/代码规范.md | 核心模块/web.md | 335/74 | 105/15 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、func.py、gc_spider.py、eslint.conf、package.js、tsconfig.js |
| 开发规范/开发工具配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发规范/开发规范.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 390/33 | 129/8 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md |
| 开发规范/文档规范.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 324/33 | 142/8 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js |
| 开发规范/测试规范.md | 核心模块/spider.md | 361/71 | 133/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js |
| 快速开始.md | 专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md | 300/42 | 86/16 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js |
| 扩展与定制/UniSpec规范管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 扩展与定制/功能扩展开发.md | 核心模块/spider.md | 386/71 | 142/13 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql |
| 扩展与定制/性能优化定制.md | 项目概述.md | 318/71 | 119/13 | 0/0 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql |
| 扩展与定制/扩展与定制.md | 核心模块/web.md | 339/74 | 140/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 扩展与定制/插件开发机制.md | 核心模块/spider.md | 373/71 | 151/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py |
| 扩展与定制/界面定制方案.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 384/33 | 163/8 | 0/1 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 扩展与定制/第三方集成开发.md | 项目概述.md | 404/71 | 154/13 | 0/0 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、spec.md |
| 数据库设计/全文搜索功能.md | 项目概述.md | 250/71 | 80/13 | 0/0 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 数据库设计/数据库性能调优.md | 核心模块/spider.md | 304/71 | 112/13 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 数据库设计/数据库表结构设计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据库设计.md | 核心模块/spider.md | 343/71 | 117/13 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts |
| 数据库设计/查询优化技术.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/索引优化策略.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心功能模块/内容展示系统/UI组件库.md | 核心模块/web.md | 370/74 | 149/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts |
| 核心功能模块/内容展示系统/内容展示系统.md | 核心模块/web.md | 338/74 | 153/15 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 核心功能模块/内容展示系统/搜索功能页面.md | 核心模块/web.md | 390/74 | 139/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql |
| 核心功能模块/内容展示系统/新片速递页面.md | 核心模块/web.md | 551/74 | 417/15 | 21/1 | 13/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块 |
| 核心功能模块/内容展示系统/视频转换界面.md | 核心模块/web.md | 346/74 | 185/15 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts |
| 核心功能模块/内容展示系统/首页新闻展示.md | 核心模块/web.md | 317/74 | 150/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 核心功能模块/新闻爬虫系统/内容处理与存储.md | 核心模块/spider.md | 351/71 | 136/13 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/去重与筛选机制.md | 核心模块/spider.md | 363/71 | 161/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/同步爬取实现.md | 核心模块/spider.md | 349/71 | 153/13 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py |
| 核心功能模块/新闻爬虫系统/异步爬取实现.md | 核心模块/spider.md | 326/71 | 131/13 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/新闻爬虫系统/数据解析与提取.md | 核心模块/spider.md | 378/71 | 151/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php |
| 核心功能模块/新闻爬虫系统/新闻爬虫系统.md | 核心模块/spider.md | 409/71 | 163/13 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/核心功能模块.md | 核心模块/spider.md | 343/71 | 133/13 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts |
| 核心功能模块/视频转换系统/视频压缩功能.md | 核心模块/spider.md | 559/71 | 284/13 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 核心功能模块/视频转换系统/视频合并功能.md | 核心模块/spider.md | 348/71 | 221/13 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 核心功能模块/视频转换系统/视频处理工具集.md | 核心模块/spider.md | 363/71 | 159/13 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 核心功能模块/视频转换系统/视频质量比较.md | 核心模块/web.md | 510/74 | 349/15 | 22/1 | 12/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、compare_video.py、func.py、output.py、utils.py、spec.md、vue.js |
| 核心功能模块/视频转换系统/视频转换系统.md | 核心模块/web.md | 372/74 | 148/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md |
| 核心功能模块/视频转换系统/转换核心引擎.md | 核心模块/spider.md | 355/71 | 139/13 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 系统架构/异步处理模式.md | 系统架构.md | 367/59 | 142/19 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py |
| 系统架构/技术栈选择.md | 系统架构.md | 313/59 | 116/19 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts |
| 系统架构/数据流设计.md | 系统架构.md | 373/59 | 147/19 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql |
| 系统架构/整体架构设计.md | 系统架构.md | 315/59 | 120/19 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts |
| 系统架构/系统架构.md | 系统架构.md | 386/59 | 155/19 | 15/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |
| 系统架构/组件交互机制.md | 系统架构.md | 326/59 | 133/19 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts |
| 部署与运维/备份与恢复.md | 工作流与部署.md | 291/38 | 73/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js |
| 部署与运维/安全加固.md | 工作流与部署.md | 247/38 | 88/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Docker容器化部署.md | 工作流与部署.md | 270/38 | 57/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、package.js、index.ts、vite.config.ts、node.js、docker-compose.yml |
| 部署与运维/服务部署/Flask后端服务部署.md | 工作流与部署.md | 446/38 | 159/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Nginx反向代理配置.md | 工作流与部署.md | 312/38 | 165/3 | 13/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：fastcgi.conf、nginx.conf、readme.md、vue.js |
| 部署与运维/服务部署/Vue前端应用部署.md | 工作流与部署.md | 292/38 | 84/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、package.js、main.ts、index.ts、tsconfig.app.js、tsconfig.node.js、uno.config.ts |
| 部署与运维/服务部署/服务部署.md | 工作流与部署.md | 291/38 | 92/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、app.py、config.yaml、package.js、main.ts、vite.config.ts、node.js |
| 部署与运维/环境配置.md | 工作流与部署.md | 329/38 | 129/3 | 0/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、app.py、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts |
| 部署与运维/监控与日志.md | 项目概述.md | 324/71 | 147/13 | 0/0 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts、news.ts |
| 部署与运维/部署与运维.md | 工作流与部署.md | 306/38 | 76/3 | 0/0 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、security.md、app.py、config.yaml、links.js、package.js、vite.config.ts |
| 项目概述.md | 项目概述.md | 351/71 | 192/13 | 18/0 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |

## 逐文件详情

### API接口文档/API接口文档.md

- reference 标题：API接口文档
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/新闻API.md

- reference 标题：新闻API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：76
- 页面类型：other / module
- 行数：377 / 71
- 段落行数：101 / 13
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、index.ts、news.ts

### API接口文档/爬虫API.md

- reference 标题：爬虫API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：98
- 页面类型：other / module
- 行数：323 / 71
- 段落行数：111 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts

### API接口文档/系统管理API.md

- reference 标题：系统管理API
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/转换API.md

- reference 标题：转换API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：68
- 页面类型：other / module
- 行数：363 / 71
- 段落行数：146 / 13
- Evidence：19 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、xunlei.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts

### 前端开发指南/API集成.md

- reference 标题：API集成
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：170
- 页面类型：other / module
- 行数：380 / 74
- 段落行数：163 / 15
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、news.ts、main.ts
- reference 关键文件未覆盖：nginx.conf、package.js、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts

### 前端开发指南/UI设计系统.md

- reference 标题：UI设计系统
- 生成页：无
- 问题：缺少对应生成页面

### 前端开发指南/Vue应用架构.md

- reference 标题：Vue应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：118
- 页面类型：architecture / architecture
- 行数：434 / 59
- 段落行数：166 / 19
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts

### 前端开发指南/前端开发指南.md

- reference 标题：前端开发指南
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：62
- 页面类型：other / module
- 行数：285 / 74
- 段落行数：104 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、links.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 前端开发指南/状态管理.md

- reference 标题：状态管理
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：82
- 页面类型：other / module
- 行数：302 / 74
- 段落行数：120 / 15
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 前端开发指南/组件开发/UI组件集成.md

- reference 标题：UI组件集成
- 生成页：无
- 问题：缺少对应生成页面

### 前端开发指南/组件开发/基础组件开发.md

- reference 标题：基础组件开发
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：66
- 页面类型：other / module
- 行数：475 / 74
- 段落行数：336 / 15
- Evidence：20 / 1
- Mermaid：12 / 1
- 文件提及重合：news.ts、main.ts、index.ts、crawler.ts、links.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 前端开发指南/组件开发/组件开发.md

- reference 标题：组件开发
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：88
- 页面类型：other / topic
- 行数：374 / 33
- 段落行数：127 / 8
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、news.ts、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts

### 前端开发指南/组件开发/高级组件模式.md

- reference 标题：高级组件模式
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：60
- 页面类型：other / module
- 行数：328 / 74
- 段落行数：130 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：crawler.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globalticker.ts、vite.config.ts

### 前端开发指南/路由系统.md

- reference 标题：路由系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：70
- 页面类型：other / module
- 行数：254 / 74
- 段落行数：78 / 15
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：convert.ts、main.ts、index.ts、news.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 后端开发指南/API开发最佳实践.md

- reference 标题：API开发最佳实践
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：100
- 页面类型：other / module
- 行数：336 / 71
- 段落行数：116 / 13
- Evidence：18 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、config.yaml、func.py、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、func.py、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts

### 后端开发指南/Flask应用架构.md

- reference 标题：Flask应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：378 / 59
- 段落行数：248 / 19
- Evidence：19 / 1
- Mermaid：11 / 2
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py

### 后端开发指南/后端开发指南.md

- reference 标题：后端开发指南
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：80
- 页面类型：other / module
- 行数：351 / 71
- 段落行数：177 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py、xunlei.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、utils.py、links.js、async_fetch.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、utils.py、links.js、async_fetch.py

### 后端开发指南/数据库操作系统.md

- reference 标题：数据库操作系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：70
- 页面类型：other / module
- 行数：346 / 71
- 段落行数：145 / 13
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 后端开发指南/爬虫引擎系统/同步爬取机制.md

- reference 标题：同步爬取机制
- 生成页：专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md（流程主题：run_ffmpeg_with_progress flow）
- 匹配分数：120
- 页面类型：topic / topic
- 行数：345 / 42
- 段落行数：158 / 16
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py

### 后端开发指南/爬虫引擎系统/异步爬取机制.md

- reference 标题：异步爬取机制
- 生成页：专题/module-capability/module-4c517846f747-repository-spider能力：数据访问.md（spider能力：数据访问）
- 匹配分数：102
- 页面类型：topic / topic
- 行数：432 / 36
- 段落行数：248 / 12
- Evidence：18 / 1
- Mermaid：9 / 1
- 文件提及重合：gc_sql.py
- reference 关键文件未覆盖：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md

### 后端开发指南/爬虫引擎系统/数据处理管道.md

- reference 标题：数据处理管道
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：104
- 页面类型：other / module
- 行数：375 / 71
- 段落行数：142 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、func.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、func.py

### 后端开发指南/爬虫引擎系统/爬虫引擎系统.md

- reference 标题：爬虫引擎系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 页面类型：other / module
- 行数：375 / 71
- 段落行数：158 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 后端开发指南/爬虫引擎系统/调度与定时机制.md

- reference 标题：调度与定时机制
- 生成页：专题/process/process-update_data-flow-流程主题：update_data-flow.md（流程主题：update_data flow）
- 匹配分数：106
- 页面类型：topic / topic
- 行数：437 / 34
- 段落行数：310 / 9
- Evidence：21 / 1
- Mermaid：12 / 0
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、spec.md、design.md、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、spec.md、design.md、add_sys_data_update_interval.sql

### 后端开发指南/视频转换系统/MKV文件合并.md

- reference 标题：MKV文件合并
- 生成页：无
- 问题：缺少对应生成页面

### 后端开发指南/视频转换系统/任务管理系统.md

- reference 标题：任务管理系统
- 生成页：无
- 问题：缺少对应生成页面

### 后端开发指南/视频转换系统/子进程集成机制.md

- reference 标题：子进程集成机制
- 生成页：专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md（流程主题：run_ffmpeg_with_progress flow）
- 匹配分数：132
- 页面类型：topic / topic
- 行数：284 / 42
- 段落行数：127 / 16
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 后端开发指南/视频转换系统/文件比较功能.md

- reference 标题：文件比较功能
- 生成页：无
- 问题：缺少对应生成页面

### 后端开发指南/视频转换系统/视频压缩处理.md

- reference 标题：视频压缩处理
- 生成页：无
- 问题：缺少对应生成页面

### 后端开发指南/视频转换系统/视频转换系统.md

- reference 标题：视频转换系统
- 生成页：无
- 问题：缺少对应生成页面

### 开发规范/Git工作流程.md

- reference 标题：Git工作流程
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 页面类型：workflow / workflow
- 行数：235 / 38
- 段落行数：101 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml

### 开发规范/代码规范.md

- reference 标题：代码规范
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：94
- 页面类型：other / module
- 行数：335 / 74
- 段落行数：105 / 15
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：links.ts、main.ts、news.ts、index.ts
- reference 关键文件未覆盖：app.py、config.yaml、func.py、gc_spider.py、eslint.conf、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、func.py、gc_spider.py、eslint.conf、package.js、tsconfig.js

### 开发规范/开发工具配置.md

- reference 标题：开发工具配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发规范/开发规范.md

- reference 标题：开发规范
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：128
- 页面类型：other / topic
- 行数：390 / 33
- 段落行数：129 / 8
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md

### 开发规范/文档规范.md

- reference 标题：文档规范
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：86
- 页面类型：other / topic
- 行数：324 / 33
- 段落行数：142 / 8
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：app.py、main.ts
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js

### 开发规范/测试规范.md

- reference 标题：测试规范
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：102
- 页面类型：other / module
- 行数：361 / 71
- 段落行数：133 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：utils.py、spec.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md（流程主题：run_ffmpeg_with_progress flow）
- 匹配分数：112
- 页面类型：other / topic
- 行数：300 / 42
- 段落行数：86 / 16
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js

### 扩展与定制/UniSpec规范管理.md

- reference 标题：UniSpec规范管理
- 生成页：无
- 问题：缺少对应生成页面

### 扩展与定制/功能扩展开发.md

- reference 标题：功能扩展开发
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：110
- 页面类型：other / module
- 行数：386 / 71
- 段落行数：142 / 13
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql

### 扩展与定制/性能优化定制.md

- reference 标题：性能优化定制
- 生成页：项目概述.md（项目概述）
- 匹配分数：132
- 页面类型：other / overview
- 行数：318 / 71
- 段落行数：119 / 13
- Evidence：0 / 0
- Mermaid：6 / 1
- 文件提及重合：nginx.conf、app.py、main.ts、index.ts
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql

### 扩展与定制/扩展与定制.md

- reference 标题：扩展与定制
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：174
- 页面类型：other / module
- 行数：339 / 74
- 段落行数：140 / 15
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、main.ts、news.ts
- reference 关键文件未覆盖：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 扩展与定制/插件开发机制.md

- reference 标题：插件开发机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：142
- 页面类型：topic / module
- 行数：373 / 71
- 段落行数：151 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py、xunlei.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py

### 扩展与定制/界面定制方案.md

- reference 标题：界面定制方案
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：86
- 页面类型：other / topic
- 行数：384 / 33
- 段落行数：163 / 8
- Evidence：0 / 1
- Mermaid：12 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 扩展与定制/第三方集成开发.md

- reference 标题：第三方集成开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 页面类型：other / overview
- 行数：404 / 71
- 段落行数：154 / 13
- Evidence：0 / 0
- Mermaid：6 / 1
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：readme.md、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、spec.md

### 数据库设计/全文搜索功能.md

- reference 标题：全文搜索功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：78
- 页面类型：other / overview
- 行数：250 / 71
- 段落行数：80 / 13
- Evidence：0 / 0
- Mermaid：4 / 1
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 数据库设计/数据库性能调优.md

- reference 标题：数据库性能调优
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：102
- 页面类型：other / module
- 行数：304 / 71
- 段落行数：112 / 13
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql

### 数据库设计/数据库表结构设计.md

- reference 标题：数据库表结构设计
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/数据库设计.md

- reference 标题：数据库设计
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：82
- 页面类型：other / module
- 行数：343 / 71
- 段落行数：117 / 13
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts

### 数据库设计/查询优化技术.md

- reference 标题：查询优化技术
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/索引优化策略.md

- reference 标题：索引优化策略
- 生成页：无
- 问题：缺少对应生成页面

### 核心功能模块/内容展示系统/UI组件库.md

- reference 标题：UI组件库
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：208
- 页面类型：module / module
- 行数：370 / 74
- 段落行数：149 / 15
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：crawler.ts、news.ts、main.ts、index.ts
- reference 关键文件未覆盖：globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts

### 核心功能模块/内容展示系统/内容展示系统.md

- reference 标题：内容展示系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：144
- 页面类型：module / module
- 行数：338 / 74
- 段落行数：153 / 15
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：convert.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 核心功能模块/内容展示系统/搜索功能页面.md

- reference 标题：搜索功能页面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：142
- 页面类型：module / module
- 行数：390 / 74
- 段落行数：139 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、news.ts
- reference 关键文件未覆盖：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql

### 核心功能模块/内容展示系统/新片速递页面.md

- reference 标题：新片速递页面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：124
- 页面类型：module / module
- 行数：551 / 74
- 段落行数：417 / 15
- Evidence：21 / 1
- Mermaid：13 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块

### 核心功能模块/内容展示系统/视频转换界面.md

- reference 标题：视频转换界面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：124
- 页面类型：module / module
- 行数：346 / 74
- 段落行数：185 / 15
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：convert.ts、main.ts、index.ts
- reference 关键文件未覆盖：func.py、output.py、utils.py、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts

### 核心功能模块/内容展示系统/首页新闻展示.md

- reference 标题：首页新闻展示
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：134
- 页面类型：module / module
- 行数：317 / 74
- 段落行数：150 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 核心功能模块/新闻爬虫系统/内容处理与存储.md

- reference 标题：内容处理与存储
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：188
- 页面类型：module / module
- 行数：351 / 71
- 段落行数：136 / 13
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、func.py、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql

### 核心功能模块/新闻爬虫系统/去重与筛选机制.md

- reference 标题：去重与筛选机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：184
- 页面类型：module / module
- 行数：363 / 71
- 段落行数：161 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql

### 核心功能模块/新闻爬虫系统/同步爬取实现.md

- reference 标题：同步爬取实现
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：186
- 页面类型：module / module
- 行数：349 / 71
- 段落行数：153 / 13
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py

### 核心功能模块/新闻爬虫系统/异步爬取实现.md

- reference 标题：异步爬取实现
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：188
- 页面类型：module / module
- 行数：326 / 71
- 段落行数：131 / 13
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 核心功能模块/新闻爬虫系统/数据解析与提取.md

- reference 标题：数据解析与提取
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：168
- 页面类型：module / module
- 行数：378 / 71
- 段落行数：151 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php

### 核心功能模块/新闻爬虫系统/新闻爬虫系统.md

- reference 标题：新闻爬虫系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：192
- 页面类型：module / module
- 行数：409 / 71
- 段落行数：163 / 13
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 核心功能模块/核心功能模块.md

- reference 标题：核心功能模块
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：182
- 页面类型：module / module
- 行数：343 / 71
- 段落行数：133 / 13
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts

### 核心功能模块/视频转换系统/视频压缩功能.md

- reference 标题：视频压缩功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：104
- 页面类型：module / module
- 行数：559 / 71
- 段落行数：284 / 13
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts

### 核心功能模块/视频转换系统/视频合并功能.md

- reference 标题：视频合并功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：102
- 页面类型：module / module
- 行数：348 / 71
- 段落行数：221 / 13
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 核心功能模块/视频转换系统/视频处理工具集.md

- reference 标题：视频处理工具集
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：152
- 页面类型：module / module
- 行数：363 / 71
- 段落行数：159 / 13
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：app.py、xunlei.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、convert.ts

### 核心功能模块/视频转换系统/视频质量比较.md

- reference 标题：视频质量比较
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：104
- 页面类型：module / module
- 行数：510 / 74
- 段落行数：349 / 15
- Evidence：22 / 1
- Mermaid：12 / 1
- 文件提及重合：convert.ts
- reference 关键文件未覆盖：app.py、compare_video.py、func.py、output.py、utils.py、spec.md、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、compare_video.py、func.py、output.py、utils.py、spec.md、vue.js

### 核心功能模块/视频转换系统/视频转换系统.md

- reference 标题：视频转换系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：106
- 页面类型：module / module
- 行数：372 / 74
- 段落行数：148 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：convert.ts
- reference 关键文件未覆盖：app.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、spec.md

### 核心功能模块/视频转换系统/转换核心引擎.md

- reference 标题：转换核心引擎
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：150
- 页面类型：module / module
- 行数：355 / 71
- 段落行数：139 / 13
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、xunlei.py
- reference 关键文件未覆盖：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 系统架构/异步处理模式.md

- reference 标题：异步处理模式
- 生成页：系统架构.md（系统架构）
- 匹配分数：116
- 页面类型：architecture / architecture
- 行数：367 / 59
- 段落行数：142 / 19
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py

### 系统架构/技术栈选择.md

- reference 标题：技术栈选择
- 生成页：系统架构.md（系统架构）
- 匹配分数：212
- 页面类型：architecture / architecture
- 行数：313 / 59
- 段落行数：116 / 19
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts

### 系统架构/数据流设计.md

- reference 标题：数据流设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：134
- 页面类型：architecture / architecture
- 行数：373 / 59
- 段落行数：147 / 19
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：app.py、index.ts
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql

### 系统架构/整体架构设计.md

- reference 标题：整体架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：218
- 页面类型：architecture / architecture
- 行数：315 / 59
- 段落行数：120 / 19
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts

### 系统架构/系统架构.md

- reference 标题：系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：452
- 页面类型：architecture / architecture
- 行数：386 / 59
- 段落行数：155 / 19
- Evidence：15 / 1
- Mermaid：6 / 2
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts

### 系统架构/组件交互机制.md

- reference 标题：组件交互机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：180
- 页面类型：architecture / architecture
- 行数：326 / 59
- 段落行数：133 / 19
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：app.py、index.ts、main.ts
- reference 关键文件未覆盖：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts

### 部署与运维/备份与恢复.md

- reference 标题：备份与恢复
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：82
- 页面类型：workflow / workflow
- 行数：291 / 38
- 段落行数：73 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js

### 部署与运维/安全加固.md

- reference 标题：安全加固
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：90
- 页面类型：workflow / workflow
- 行数：247 / 38
- 段落行数：88 / 3
- Evidence：0 / 0
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py

### 部署与运维/服务部署/Docker容器化部署.md

- reference 标题：Docker容器化部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：82
- 页面类型：workflow / workflow
- 行数：270 / 38
- 段落行数：57 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、app.py、config.yaml、package.js、index.ts、vite.config.ts、node.js、docker-compose.yml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、package.js、index.ts、vite.config.ts、node.js、docker-compose.yml

### 部署与运维/服务部署/Flask后端服务部署.md

- reference 标题：Flask后端服务部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：92
- 页面类型：workflow / workflow
- 行数：446 / 38
- 段落行数：159 / 3
- Evidence：0 / 0
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py

### 部署与运维/服务部署/Nginx反向代理配置.md

- reference 标题：Nginx反向代理配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：82
- 页面类型：workflow / workflow
- 行数：312 / 38
- 段落行数：165 / 3
- Evidence：13 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：fastcgi.conf、nginx.conf、readme.md、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：fastcgi.conf、nginx.conf、readme.md、vue.js

### 部署与运维/服务部署/Vue前端应用部署.md

- reference 标题：Vue前端应用部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 页面类型：workflow / workflow
- 行数：292 / 38
- 段落行数：84 / 3
- Evidence：0 / 0
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、readme.md、package.js、main.ts、index.ts、tsconfig.app.js、tsconfig.node.js、uno.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、package.js、main.ts、index.ts、tsconfig.app.js、tsconfig.node.js、uno.config.ts

### 部署与运维/服务部署/服务部署.md

- reference 标题：服务部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：291 / 38
- 段落行数：92 / 3
- Evidence：0 / 0
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、readme.md、app.py、config.yaml、package.js、main.ts、vite.config.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、app.py、config.yaml、package.js、main.ts、vite.config.ts、node.js

### 部署与运维/环境配置.md

- reference 标题：环境配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：workflow / workflow
- 行数：329 / 38
- 段落行数：129 / 3
- Evidence：0 / 0
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、readme.md、app.py、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、app.py、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts

### 部署与运维/监控与日志.md

- reference 标题：监控与日志
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：workflow / overview
- 行数：324 / 71
- 段落行数：147 / 13
- Evidence：0 / 0
- Mermaid：7 / 1
- 文件提及重合：nginx.conf、app.py、index.ts
- reference 关键文件未覆盖：config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts、news.ts

### 部署与运维/部署与运维.md

- reference 标题：部署与运维
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 页面类型：workflow / workflow
- 行数：306 / 38
- 段落行数：76 / 3
- Evidence：0 / 0
- Mermaid：4 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、readme.md、security.md、app.py、config.yaml、links.js、package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、security.md、app.py、config.yaml、links.js、package.js、vite.config.ts

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：696
- 页面类型：overview / overview
- 行数：351 / 71
- 段落行数：192 / 13
- Evidence：18 / 0
- Mermaid：8 / 1
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts

