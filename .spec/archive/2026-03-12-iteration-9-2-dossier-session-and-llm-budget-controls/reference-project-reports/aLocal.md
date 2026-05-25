# aLocal Reference 对比报告

生成页面：18 页
reference 页面：86 页
命中对比：74 页
缺失对比：12 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=28, total_tokens=134857, page_research=16, page_enrichment=0

## 覆盖统计

- 专题页覆盖：generated 11 / reference 5
- evidence 落页：generated 15 / reference 86
- 图表达覆盖：generated 11 / reference 86
- page research 请求：16
- page enrichment 请求：0
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

- 核心模块/web.md 被 14 个 reference 页面共享映射
- 核心模块/spider.md 被 26 个 reference 页面共享映射
- 专题/process/process-api_links-flow-流程主题：api_links-flow.md 被 13 个 reference 页面共享映射
- 系统架构.md 被 7 个 reference 页面共享映射
- 专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md 被 3 个 reference 页面共享映射
- 工作流与部署.md 被 6 个 reference 页面共享映射
- 专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md 被 2 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-240f4d7393a9-plugin-nginx能力：扩展机制.md (nginx能力：扩展机制, 24 行)
- 专题/module-capability/module-5380ddae5fb7-component-web能力：界面组成.md (web能力：界面组成, 35 行)
- 专题/process/process-api_news_find-flow-流程主题：api_news_find-flow.md (流程主题：api_news_find flow, 33 行)
- 专题/process/process-get_last_run_time-flow-流程主题：get_last_run_time-flow.md (流程主题：get_last_run_time flow, 23 行)
- 专题/process/process-update_data-flow-流程主题：update_data-flow.md (流程主题：update_data flow, 23 行)
- 专题/process/process-update_last_run_time-flow-流程主题：update_last_run_time-flow.md (流程主题：update_last_run_time flow, 23 行)
- 核心模块/nginx.md (模块：nginx, 63 行)
- 核心模块/nginx/conf.md (模块：conf, 50 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API接口文档/API接口文档.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/新闻API.md | 核心模块/web.md | 377/81 | 101/17 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、gc_sql.py、utils.py |
| API接口文档/爬虫API.md | 核心模块/spider.md | 323/78 | 111/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts |
| API接口文档/系统管理API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/转换API.md | 核心模块/spider.md | 363/78 | 146/15 | 19/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py |
| 前端开发指南/API集成.md | 核心模块/web.md | 380/81 | 163/17 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts |
| 前端开发指南/UI设计系统.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 308/23 | 108/5 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 前端开发指南/Vue应用架构.md | 系统架构.md | 434/61 | 166/20 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts |
| 前端开发指南/前端开发指南.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 285/23 | 104/5 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、vite.config.ts |
| 前端开发指南/状态管理.md | 核心模块/web.md | 302/81 | 120/17 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 前端开发指南/组件开发/UI组件集成.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 395/23 | 229/5 | 18/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、uno.config.ts、vite.config.ts |
| 前端开发指南/组件开发/基础组件开发.md | 核心模块/web.md | 475/81 | 336/17 | 20/1 | 12/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 前端开发指南/组件开发/组件开发.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 374/23 | 127/5 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts |
| 前端开发指南/组件开发/高级组件模式.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 328/23 | 130/5 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、crawler.ts、news.ts、globalticker.ts、vite.config.ts |
| 前端开发指南/路由系统.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 254/23 | 78/5 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、news.ts、vite.config.ts |
| 后端开发指南/API开发最佳实践.md | 核心模块/spider.md | 336/78 | 116/15 | 18/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts |
| 后端开发指南/Flask应用架构.md | 系统架构.md | 378/61 | 248/20 | 19/1 | 11/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py |
| 后端开发指南/后端开发指南.md | 核心模块/spider.md | 351/78 | 177/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py |
| 后端开发指南/数据库操作系统.md | 核心模块/spider.md | 346/78 | 145/15 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 后端开发指南/爬虫引擎系统/同步爬取机制.md | 专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md | 345/39 | 158/18 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py |
| 后端开发指南/爬虫引擎系统/异步爬取机制.md | 专题/module-capability/module-4c517846f747-repository-spider能力：数据访问.md | 432/33 | 248/12 | 18/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md |
| 后端开发指南/爬虫引擎系统/数据处理管道.md | 核心模块/spider.md | 375/78 | 142/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql |
| 后端开发指南/爬虫引擎系统/爬虫引擎系统.md | 核心模块/spider.md | 375/78 | 158/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 后端开发指南/爬虫引擎系统/调度与定时机制.md | 专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md | 437/39 | 310/18 | 21/1 | 12/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、spec.md、design.md、add_sys_data_update_interval.sql |
| 后端开发指南/视频转换系统/MKV文件合并.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/任务管理系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/子进程集成机制.md | 专题/process/process-run_ffmpeg_with_progress-flow-流程主题：run_ffmpeg_with_progress-flow.md | 284/33 | 127/12 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 后端开发指南/视频转换系统/文件比较功能.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频压缩处理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频转换系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发规范/Git工作流程.md | 工作流与部署.md | 235/38 | 101/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml |
| 开发规范/代码规范.md | 核心模块/web.md | 335/81 | 105/17 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、func.py、gc_spider.py、eslint.conf、package.js、tsconfig.js |
| 开发规范/开发工具配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发规范/开发规范.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 390/23 | 129/5 | 0/1 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md |
| 开发规范/文档规范.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 324/23 | 142/5 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js |
| 开发规范/测试规范.md | 核心模块/spider.md | 361/78 | 133/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js |
| 快速开始.md | 专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md | 300/39 | 86/18 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js |
| 扩展与定制/UniSpec规范管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 扩展与定制/功能扩展开发.md | 核心模块/spider.md | 386/78 | 142/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql |
| 扩展与定制/性能优化定制.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 318/23 | 119/5 | 0/1 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql |
| 扩展与定制/扩展与定制.md | 核心模块/web.md | 339/81 | 140/17 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 扩展与定制/插件开发机制.md | 核心模块/spider.md | 373/78 | 151/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py |
| 扩展与定制/界面定制方案.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 384/23 | 163/5 | 0/1 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 扩展与定制/第三方集成开发.md | 核心模块/spider.md | 404/78 | 154/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts |
| 数据库设计/全文搜索功能.md | 专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md | 250/23 | 80/5 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 数据库设计/数据库性能调优.md | 核心模块/spider.md | 304/78 | 112/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 数据库设计/数据库表结构设计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据库设计.md | 核心模块/spider.md | 343/78 | 117/15 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts |
| 数据库设计/查询优化技术.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/索引优化策略.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 核心功能模块/内容展示系统/UI组件库.md | 核心模块/web.md | 370/81 | 149/17 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts |
| 核心功能模块/内容展示系统/内容展示系统.md | 核心模块/web.md | 338/81 | 153/17 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 核心功能模块/内容展示系统/搜索功能页面.md | 核心模块/web.md | 390/81 | 139/17 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql |
| 核心功能模块/内容展示系统/新片速递页面.md | 核心模块/web.md | 551/81 | 417/17 | 21/1 | 13/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块 |
| 核心功能模块/内容展示系统/视频转换界面.md | 核心模块/web.md | 346/81 | 185/17 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts |
| 核心功能模块/内容展示系统/首页新闻展示.md | 核心模块/web.md | 317/81 | 150/17 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 核心功能模块/新闻爬虫系统/内容处理与存储.md | 核心模块/spider.md | 351/78 | 136/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/去重与筛选机制.md | 核心模块/spider.md | 363/78 | 161/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/同步爬取实现.md | 核心模块/spider.md | 349/78 | 153/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py |
| 核心功能模块/新闻爬虫系统/异步爬取实现.md | 核心模块/spider.md | 326/78 | 131/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/新闻爬虫系统/数据解析与提取.md | 核心模块/spider.md | 378/78 | 151/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php |
| 核心功能模块/新闻爬虫系统/新闻爬虫系统.md | 核心模块/spider.md | 409/78 | 163/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/核心功能模块.md | 核心模块/spider.md | 343/78 | 133/15 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts |
| 核心功能模块/视频转换系统/视频压缩功能.md | 核心模块/spider.md | 559/78 | 284/15 | 14/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 核心功能模块/视频转换系统/视频合并功能.md | 核心模块/spider.md | 348/78 | 221/15 | 17/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py |
| 核心功能模块/视频转换系统/视频处理工具集.md | 核心模块/spider.md | 363/78 | 159/15 | 0/1 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py |
| 核心功能模块/视频转换系统/视频质量比较.md | 核心模块/spider.md | 510/78 | 349/15 | 22/1 | 12/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js |
| 核心功能模块/视频转换系统/视频转换系统.md | 核心模块/spider.md | 372/78 | 148/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py |
| 核心功能模块/视频转换系统/转换核心引擎.md | 核心模块/spider.md | 355/78 | 139/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 系统架构/异步处理模式.md | 系统架构.md | 367/61 | 142/20 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py |
| 系统架构/技术栈选择.md | 系统架构.md | 313/61 | 116/20 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts |
| 系统架构/数据流设计.md | 系统架构.md | 373/61 | 147/20 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql |
| 系统架构/整体架构设计.md | 系统架构.md | 315/61 | 120/20 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts |
| 系统架构/系统架构.md | 系统架构.md | 386/61 | 155/20 | 15/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |
| 系统架构/组件交互机制.md | 核心模块/web.md | 326/81 | 133/17 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、package.js、globalticker.ts |
| 部署与运维/备份与恢复.md | 工作流与部署.md | 291/38 | 73/3 | 0/0 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js |
| 部署与运维/安全加固.md | 工作流与部署.md | 247/38 | 88/3 | 0/0 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Docker容器化部署.md | 专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md | 270/23 | 57/5 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf |
| 部署与运维/服务部署/Flask后端服务部署.md | 工作流与部署.md | 446/38 | 159/3 | 0/0 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Nginx反向代理配置.md | 工作流与部署.md | 312/38 | 165/3 | 13/0 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：fastcgi.conf、nginx.conf、readme.md、vue.js |
| 部署与运维/服务部署/Vue前端应用部署.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 292/23 | 84/5 | 0/1 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts |
| 部署与运维/服务部署/服务部署.md | 工作流与部署.md | 291/38 | 92/3 | 0/0 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、app.py、config.yaml、package.js、main.ts、vite.config.ts、node.js |
| 部署与运维/环境配置.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 329/23 | 129/5 | 0/1 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js |
| 部署与运维/监控与日志.md | 核心模块/web.md | 324/81 | 147/17 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md |
| 部署与运维/部署与运维.md | 专题/process/process-api_links-flow-流程主题：api_links-flow.md | 306/23 | 76/5 | 0/1 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts |
| 项目概述.md | 项目概述.md | 351/71 | 192/13 | 18/0 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |

## 逐文件详情

### API接口文档/API接口文档.md

- reference 标题：API接口文档
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/新闻API.md

- reference 标题：新闻API
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：78
- 页面类型：other / module
- 行数：377 / 81
- 段落行数：101 / 17
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、news.ts
- reference 关键文件未覆盖：app.py、config.yaml、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、gc_sql.py、utils.py

### API接口文档/爬虫API.md

- reference 标题：爬虫API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：98
- 页面类型：other / module
- 行数：323 / 78
- 段落行数：111 / 15
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
- 匹配分数：66
- 页面类型：other / module
- 行数：363 / 78
- 段落行数：146 / 15
- Evidence：19 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py

### 前端开发指南/API集成.md

- reference 标题：API集成
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：190
- 页面类型：other / module
- 行数：380 / 81
- 段落行数：163 / 17
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、news.ts、main.ts
- reference 关键文件未覆盖：nginx.conf、package.js、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts

### 前端开发指南/UI设计系统.md

- reference 标题：UI设计系统
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：76
- 页面类型：other / topic
- 行数：308 / 23
- 段落行数：108 / 5
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 前端开发指南/Vue应用架构.md

- reference 标题：Vue应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：118
- 页面类型：architecture / architecture
- 行数：434 / 61
- 段落行数：166 / 20
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts

### 前端开发指南/前端开发指南.md

- reference 标题：前端开发指南
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：82
- 页面类型：other / topic
- 行数：285 / 23
- 段落行数：104 / 5
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、links.ts、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、vite.config.ts

### 前端开发指南/状态管理.md

- reference 标题：状态管理
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：104
- 页面类型：other / module
- 行数：302 / 81
- 段落行数：120 / 17
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 前端开发指南/组件开发/UI组件集成.md

- reference 标题：UI组件集成
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：76
- 页面类型：other / topic
- 行数：395 / 23
- 段落行数：229 / 5
- Evidence：18 / 1
- Mermaid：9 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、news.ts、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、uno.config.ts、vite.config.ts

### 前端开发指南/组件开发/基础组件开发.md

- reference 标题：基础组件开发
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：72
- 页面类型：other / module
- 行数：475 / 81
- 段落行数：336 / 17
- Evidence：20 / 1
- Mermaid：12 / 1
- 文件提及重合：news.ts、main.ts、index.ts、crawler.ts、links.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 前端开发指南/组件开发/组件开发.md

- reference 标题：组件开发
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：112
- 页面类型：other / topic
- 行数：374 / 23
- 段落行数：127 / 5
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、news.ts、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts

### 前端开发指南/组件开发/高级组件模式.md

- reference 标题：高级组件模式
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：72
- 页面类型：other / topic
- 行数：328 / 23
- 段落行数：130 / 5
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、crawler.ts、news.ts、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、crawler.ts、news.ts、globalticker.ts、vite.config.ts

### 前端开发指南/路由系统.md

- reference 标题：路由系统
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：96
- 页面类型：other / topic
- 行数：254 / 23
- 段落行数：78 / 5
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、convert.ts、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、news.ts、vite.config.ts

### 后端开发指南/API开发最佳实践.md

- reference 标题：API开发最佳实践
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：128
- 页面类型：other / module
- 行数：336 / 78
- 段落行数：116 / 15
- Evidence：18 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts

### 后端开发指南/Flask应用架构.md

- reference 标题：Flask应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：378 / 61
- 段落行数：248 / 20
- Evidence：19 / 1
- Mermaid：11 / 2
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py

### 后端开发指南/后端开发指南.md

- reference 标题：后端开发指南
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：60
- 页面类型：other / module
- 行数：351 / 78
- 段落行数：177 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py

### 后端开发指南/数据库操作系统.md

- reference 标题：数据库操作系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：70
- 页面类型：other / module
- 行数：346 / 78
- 段落行数：145 / 15
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 后端开发指南/爬虫引擎系统/同步爬取机制.md

- reference 标题：同步爬取机制
- 生成页：专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md（流程主题：run_feedback_cycle flow）
- 匹配分数：122
- 页面类型：topic / topic
- 行数：345 / 39
- 段落行数：158 / 18
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
- 行数：432 / 33
- 段落行数：248 / 12
- Evidence：18 / 1
- Mermaid：9 / 1
- 文件提及重合：gc_sql.py
- reference 关键文件未覆盖：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、async_fetch.py、crawler_config.py、gc_spider.py、utils.py、design.md、spec.md

### 后端开发指南/爬虫引擎系统/数据处理管道.md

- reference 标题：数据处理管道
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：112
- 页面类型：other / module
- 行数：375 / 78
- 段落行数：142 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py、func.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql

### 后端开发指南/爬虫引擎系统/爬虫引擎系统.md

- reference 标题：爬虫引擎系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 页面类型：other / module
- 行数：375 / 78
- 段落行数：158 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 后端开发指南/爬虫引擎系统/调度与定时机制.md

- reference 标题：调度与定时机制
- 生成页：专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md（流程主题：run_feedback_cycle flow）
- 匹配分数：104
- 页面类型：topic / topic
- 行数：437 / 39
- 段落行数：310 / 18
- Evidence：21 / 1
- Mermaid：12 / 1
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
- 匹配分数：126
- 页面类型：topic / topic
- 行数：284 / 33
- 段落行数：127 / 12
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
- 匹配分数：114
- 页面类型：other / module
- 行数：335 / 81
- 段落行数：105 / 17
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
- 匹配分数：154
- 页面类型：other / topic
- 行数：390 / 23
- 段落行数：129 / 5
- Evidence：0 / 1
- Mermaid：5 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md

### 开发规范/文档规范.md

- reference 标题：文档规范
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：94
- 页面类型：other / topic
- 行数：324 / 23
- 段落行数：142 / 5
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
- 行数：361 / 78
- 段落行数：133 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：utils.py、spec.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：专题/process/process-run_feedback_cycle-flow-流程主题：run_feedback_cycle-flow.md（流程主题：run_feedback_cycle flow）
- 匹配分数：116
- 页面类型：other / topic
- 行数：300 / 39
- 段落行数：86 / 18
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
- 匹配分数：138
- 页面类型：other / module
- 行数：386 / 78
- 段落行数：142 / 15
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql

### 扩展与定制/性能优化定制.md

- reference 标题：性能优化定制
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：134
- 页面类型：other / topic
- 行数：318 / 23
- 段落行数：119 / 5
- Evidence：0 / 1
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、main.ts、index.ts
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql

### 扩展与定制/扩展与定制.md

- reference 标题：扩展与定制
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：194
- 页面类型：other / module
- 行数：339 / 81
- 段落行数：140 / 17
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、main.ts、news.ts
- reference 关键文件未覆盖：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 扩展与定制/插件开发机制.md

- reference 标题：插件开发机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：140
- 页面类型：topic / module
- 行数：373 / 78
- 段落行数：151 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；reference 专题被折叠进非专题页；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py

### 扩展与定制/界面定制方案.md

- reference 标题：界面定制方案
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：110
- 页面类型：other / topic
- 行数：384 / 23
- 段落行数：163 / 5
- Evidence：0 / 1
- Mermaid：12 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 扩展与定制/第三方集成开发.md

- reference 标题：第三方集成开发
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：134
- 页面类型：other / module
- 行数：404 / 78
- 段落行数：154 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts

### 数据库设计/全文搜索功能.md

- reference 标题：全文搜索功能
- 生成页：专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md（流程主题：api_get_log flow）
- 匹配分数：96
- 页面类型：other / topic
- 行数：250 / 23
- 段落行数：80 / 5
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 数据库设计/数据库性能调优.md

- reference 标题：数据库性能调优
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：102
- 页面类型：other / module
- 行数：304 / 78
- 段落行数：112 / 15
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
- 行数：343 / 78
- 段落行数：117 / 15
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
- 匹配分数：234
- 页面类型：module / module
- 行数：370 / 81
- 段落行数：149 / 17
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：crawler.ts、news.ts、main.ts、index.ts
- reference 关键文件未覆盖：globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts

### 核心功能模块/内容展示系统/内容展示系统.md

- reference 标题：内容展示系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：148
- 页面类型：module / module
- 行数：338 / 81
- 段落行数：153 / 17
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：convert.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 核心功能模块/内容展示系统/搜索功能页面.md

- reference 标题：搜索功能页面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：162
- 页面类型：module / module
- 行数：390 / 81
- 段落行数：139 / 17
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：index.ts、news.ts
- reference 关键文件未覆盖：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、gc_sql.py、add_fulltext_for_search.sql

### 核心功能模块/内容展示系统/新片速递页面.md

- reference 标题：新片速递页面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：128
- 页面类型：module / module
- 行数：551 / 81
- 段落行数：417 / 17
- Evidence：21 / 1
- Mermaid：13 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块

### 核心功能模块/内容展示系统/视频转换界面.md

- reference 标题：视频转换界面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：128
- 页面类型：module / module
- 行数：346 / 81
- 段落行数：185 / 17
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：convert.ts、main.ts、index.ts
- reference 关键文件未覆盖：func.py、output.py、utils.py、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts

### 核心功能模块/内容展示系统/首页新闻展示.md

- reference 标题：首页新闻展示
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：138
- 页面类型：module / module
- 行数：317 / 81
- 段落行数：150 / 17
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 核心功能模块/新闻爬虫系统/内容处理与存储.md

- reference 标题：内容处理与存储
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：216
- 页面类型：module / module
- 行数：351 / 78
- 段落行数：136 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql

### 核心功能模块/新闻爬虫系统/去重与筛选机制.md

- reference 标题：去重与筛选机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：184
- 页面类型：module / module
- 行数：363 / 78
- 段落行数：161 / 15
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
- 行数：349 / 78
- 段落行数：153 / 15
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
- 行数：326 / 78
- 段落行数：131 / 15
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
- 行数：378 / 78
- 段落行数：151 / 15
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
- 行数：409 / 78
- 段落行数：163 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 核心功能模块/核心功能模块.md

- reference 标题：核心功能模块
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：210
- 页面类型：module / module
- 行数：343 / 78
- 段落行数：133 / 15
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts

### 核心功能模块/视频转换系统/视频压缩功能.md

- reference 标题：视频压缩功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：112
- 页面类型：module / module
- 行数：559 / 78
- 段落行数：284 / 15
- Evidence：14 / 1
- Mermaid：6 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts

### 核心功能模块/视频转换系统/视频合并功能.md

- reference 标题：视频合并功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：110
- 页面类型：module / module
- 行数：348 / 78
- 段落行数：221 / 15
- Evidence：17 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py

### 核心功能模块/视频转换系统/视频处理工具集.md

- reference 标题：视频处理工具集
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：150
- 页面类型：module / module
- 行数：363 / 78
- 段落行数：159 / 15
- Evidence：0 / 1
- Mermaid：9 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py

### 核心功能模块/视频转换系统/视频质量比较.md

- reference 标题：视频质量比较
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：112
- 页面类型：module / module
- 行数：510 / 78
- 段落行数：349 / 15
- Evidence：22 / 1
- Mermaid：12 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js

### 核心功能模块/视频转换系统/视频转换系统.md

- reference 标题：视频转换系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 页面类型：module / module
- 行数：372 / 78
- 段落行数：148 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py

### 核心功能模块/视频转换系统/转换核心引擎.md

- reference 标题：转换核心引擎
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：130
- 页面类型：module / module
- 行数：355 / 78
- 段落行数：139 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts

### 系统架构/异步处理模式.md

- reference 标题：异步处理模式
- 生成页：系统架构.md（系统架构）
- 匹配分数：116
- 页面类型：architecture / architecture
- 行数：367 / 61
- 段落行数：142 / 20
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
- 行数：313 / 61
- 段落行数：116 / 20
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
- 行数：373 / 61
- 段落行数：147 / 20
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
- 行数：315 / 61
- 段落行数：120 / 20
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
- 行数：386 / 61
- 段落行数：155 / 20
- Evidence：15 / 1
- Mermaid：6 / 2
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts

### 系统架构/组件交互机制.md

- reference 标题：组件交互机制
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：182
- 页面类型：architecture / module
- 行数：326 / 81
- 段落行数：133 / 17
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：convert.ts、crawler.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：app.py、package.js、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、package.js、globalticker.ts

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
- 生成页：专题/process/process-api_get_log-flow-流程主题：api_get_log-flow.md（流程主题：api_get_log flow）
- 匹配分数：82
- 页面类型：workflow / topic
- 行数：270 / 23
- 段落行数：57 / 5
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：nginx.conf、app.py、index.ts
- reference 关键文件未覆盖：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf

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
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：122
- 页面类型：workflow / topic
- 行数：292 / 23
- 段落行数：84 / 5
- Evidence：0 / 1
- Mermaid：3 / 0
- 文件提及重合：nginx.conf、main.ts、index.ts
- reference 关键文件未覆盖：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts

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
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：96
- 页面类型：workflow / topic
- 行数：329 / 23
- 段落行数：129 / 5
- Evidence：0 / 1
- Mermaid：7 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js

### 部署与运维/监控与日志.md

- reference 标题：监控与日志
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：108
- 页面类型：workflow / module
- 行数：324 / 81
- 段落行数：147 / 17
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：crawler.ts、index.ts、news.ts
- reference 关键文件未覆盖：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md

### 部署与运维/部署与运维.md

- reference 标题：部署与运维
- 生成页：专题/process/process-api_links-flow-流程主题：api_links-flow.md（流程主题：api_links flow）
- 匹配分数：98
- 页面类型：workflow / topic
- 行数：306 / 23
- 段落行数：76 / 5
- Evidence：0 / 1
- Mermaid：4 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts

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

