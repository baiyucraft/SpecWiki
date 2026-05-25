# aLocal Reference 对比报告

生成页面：7 页
reference 页面：86 页
命中对比：78 页
缺失对比：8 页

## 项目结论

- reference 文档树明显更细，当前 planner 仍以总览页和少量模块页为主
- 存在大量 reference 页面没有对应生成页，说明页面粒度和主题拆分不足
- 生成页普遍缺少 reference 那种源码引用/出处层
- Mermaid/结构图表达仍然不足
- 单页章节拆分比 reference 粗，主题混杂在同一页里
- 解释层正文密度仍低于 reference

## 多页折叠现象

- 核心模块/spider.md 被 37 个 reference 页面共享映射
- 核心模块/web.md 被 12 个 reference 页面共享映射
- 项目概述.md 被 14 个 reference 页面共享映射
- 系统架构.md 被 9 个 reference 页面共享映射
- 工作流与部署.md 被 6 个 reference 页面共享映射

## 额外生成页面

- 核心模块/nginx.md (模块：nginx, 32 行)
- 核心模块/nginx/conf.md (模块：conf, 26 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | --- |
| API接口文档/API接口文档.md | 核心模块/spider.md | 341/34 | 146/6 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、package.js、convert.ts、crawler.ts、index.ts、news.ts、vite.config.ts |
| API接口文档/新闻API.md | 核心模块/spider.md | 377/34 | 101/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、index.ts、news.ts |
| API接口文档/爬虫API.md | 核心模块/spider.md | 323/34 | 111/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts |
| API接口文档/系统管理API.md | 核心模块/spider.md | 330/34 | 153/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、mergetomkv.py、links.js、crawler.ts、index.ts、links.ts、vue.js、convert.func.py |
| API接口文档/转换API.md | 核心模块/spider.md | 363/34 | 146/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py |
| 前端开发指南/API集成.md | 核心模块/web.md | 380/41 | 163/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts |
| 前端开发指南/UI设计系统.md | 项目概述.md | 308/44 | 108/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 前端开发指南/Vue应用架构.md | 系统架构.md | 434/37 | 166/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts |
| 前端开发指南/前端开发指南.md | 核心模块/web.md | 285/41 | 104/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 前端开发指南/状态管理.md | 核心模块/web.md | 302/41 | 120/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 前端开发指南/组件开发/UI组件集成.md | 项目概述.md | 395/44 | 229/7 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、uno.config.ts、vite.config.ts |
| 前端开发指南/组件开发/基础组件开发.md | 核心模块/web.md | 475/41 | 336/11 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 前端开发指南/组件开发/组件开发.md | 项目概述.md | 374/44 | 127/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts |
| 前端开发指南/组件开发/高级组件模式.md | 核心模块/web.md | 328/41 | 130/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globalticker.ts、vite.config.ts |
| 前端开发指南/路由系统.md | 核心模块/web.md | 254/41 | 78/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts |
| 后端开发指南/API开发最佳实践.md | 核心模块/spider.md | 336/34 | 116/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts |
| 后端开发指南/Flask应用架构.md | 系统架构.md | 378/37 | 248/7 | 11/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py |
| 后端开发指南/后端开发指南.md | 核心模块/spider.md | 351/34 | 177/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py |
| 后端开发指南/数据库操作系统.md | 核心模块/spider.md | 346/34 | 145/6 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 后端开发指南/爬虫引擎系统/同步爬取机制.md | 核心模块/spider.md | 345/34 | 158/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py |
| 后端开发指南/爬虫引擎系统/异步爬取机制.md | 核心模块/spider.md | 432/34 | 248/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、design.md、spec.md |
| 后端开发指南/爬虫引擎系统/数据处理管道.md | 核心模块/spider.md | 375/34 | 142/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql |
| 后端开发指南/爬虫引擎系统/爬虫引擎系统.md | 核心模块/spider.md | 375/34 | 158/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 后端开发指南/爬虫引擎系统/调度与定时机制.md | 核心模块/spider.md | 437/34 | 310/6 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、spec.md、design.md、add_sys_data_update_interval.sql、crawler.ts、vue.js |
| 后端开发指南/视频转换系统/MKV文件合并.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/任务管理系统.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/子进程集成机制.md | 核心模块/spider.md | 284/34 | 127/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 后端开发指南/视频转换系统/文件比较功能.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频压缩处理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 后端开发指南/视频转换系统/视频转换系统.md | 核心模块/spider.md | 357/34 | 132/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py |
| 开发规范/Git工作流程.md | 工作流与部署.md | 235/50 | 101/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml |
| 开发规范/代码规范.md | 核心模块/spider.md | 335/34 | 105/6 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、eslint.conf、package.js、links.ts、main.ts、news.ts、index.ts、tsconfig.js |
| 开发规范/开发工具配置.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 开发规范/开发规范.md | 项目概述.md | 390/44 | 129/7 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md |
| 开发规范/文档规范.md | 项目概述.md | 324/44 | 142/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js |
| 开发规范/测试规范.md | 核心模块/spider.md | 361/34 | 133/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js |
| 快速开始.md | 项目概述.md | 300/44 | 86/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js |
| 扩展与定制/UniSpec规范管理.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 扩展与定制/功能扩展开发.md | 核心模块/spider.md | 386/34 | 142/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql |
| 扩展与定制/性能优化定制.md | 系统架构.md | 318/37 | 119/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql |
| 扩展与定制/扩展与定制.md | 核心模块/web.md | 339/41 | 140/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py |
| 扩展与定制/插件开发机制.md | 核心模块/spider.md | 373/34 | 151/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py |
| 扩展与定制/界面定制方案.md | 项目概述.md | 384/44 | 163/7 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 扩展与定制/第三方集成开发.md | 核心模块/spider.md | 404/34 | 154/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts |
| 数据库设计/全文搜索功能.md | 项目概述.md | 250/44 | 80/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql |
| 数据库设计/数据库性能调优.md | 核心模块/spider.md | 304/34 | 112/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 数据库设计/数据库表结构设计.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据库设计.md | 核心模块/spider.md | 343/34 | 117/6 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts |
| 数据库设计/查询优化技术.md | 核心模块/spider.md | 561/34 | 382/6 | 13/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、index.ts |
| 数据库设计/索引优化策略.md | 缺失 | - | - | - | 缺少对应生成页面 |
| 核心功能模块/内容展示系统/UI组件库.md | 核心模块/web.md | 370/41 | 149/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts |
| 核心功能模块/内容展示系统/内容展示系统.md | 核心模块/web.md | 338/41 | 153/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts |
| 核心功能模块/内容展示系统/搜索功能页面.md | 核心模块/spider.md | 390/34 | 139/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、add_fulltext_for_search.sql、index.ts、news.ts |
| 核心功能模块/内容展示系统/新片速递页面.md | 核心模块/web.md | 551/41 | 417/11 | 13/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 核心功能模块/内容展示系统/视频转换界面.md | 核心模块/web.md | 346/41 | 185/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts |
| 核心功能模块/内容展示系统/首页新闻展示.md | 核心模块/web.md | 317/41 | 150/11 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js |
| 核心功能模块/新闻爬虫系统/内容处理与存储.md | 核心模块/spider.md | 351/34 | 136/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/去重与筛选机制.md | 核心模块/spider.md | 363/34 | 161/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql |
| 核心功能模块/新闻爬虫系统/同步爬取实现.md | 核心模块/spider.md | 349/34 | 153/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py |
| 核心功能模块/新闻爬虫系统/异步爬取实现.md | 核心模块/spider.md | 326/34 | 131/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/新闻爬虫系统/数据解析与提取.md | 核心模块/spider.md | 378/34 | 151/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php |
| 核心功能模块/新闻爬虫系统/新闻爬虫系统.md | 核心模块/spider.md | 409/34 | 163/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md |
| 核心功能模块/核心功能模块.md | 核心模块/spider.md | 343/34 | 133/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts |
| 核心功能模块/视频转换系统/视频压缩功能.md | 核心模块/spider.md | 559/34 | 284/6 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 核心功能模块/视频转换系统/视频合并功能.md | 核心模块/spider.md | 348/34 | 221/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py |
| 核心功能模块/视频转换系统/视频处理工具集.md | 核心模块/spider.md | 363/34 | 159/6 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py |
| 核心功能模块/视频转换系统/视频质量比较.md | 核心模块/spider.md | 510/34 | 349/6 | 12/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js |
| 核心功能模块/视频转换系统/视频转换系统.md | 核心模块/spider.md | 372/34 | 148/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py |
| 核心功能模块/视频转换系统/转换核心引擎.md | 核心模块/spider.md | 355/34 | 139/6 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts |
| 系统架构/异步处理模式.md | 系统架构.md | 367/37 | 142/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py |
| 系统架构/技术栈选择.md | 系统架构.md | 313/37 | 116/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts |
| 系统架构/数据流设计.md | 系统架构.md | 373/37 | 147/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql |
| 系统架构/整体架构设计.md | 系统架构.md | 315/37 | 120/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts |
| 系统架构/系统架构.md | 系统架构.md | 386/37 | 155/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |
| 系统架构/组件交互机制.md | 系统架构.md | 326/37 | 133/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts |
| 部署与运维/备份与恢复.md | 工作流与部署.md | 291/50 | 73/11 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js |
| 部署与运维/安全加固.md | 工作流与部署.md | 247/50 | 88/11 | 5/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Docker容器化部署.md | 项目概述.md | 270/44 | 57/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf |
| 部署与运维/服务部署/Flask后端服务部署.md | 工作流与部署.md | 446/50 | 159/11 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py |
| 部署与运维/服务部署/Nginx反向代理配置.md | 工作流与部署.md | 312/50 | 165/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fastcgi.conf、nginx.conf、readme.md、vue.js |
| 部署与运维/服务部署/Vue前端应用部署.md | 项目概述.md | 292/44 | 84/7 | 3/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts |
| 部署与运维/服务部署/服务部署.md | 项目概述.md | 291/44 | 92/7 | 6/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、package.js、vite.config.ts、node.js |
| 部署与运维/环境配置.md | 项目概述.md | 329/44 | 129/7 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js |
| 部署与运维/监控与日志.md | 工作流与部署.md | 324/50 | 147/11 | 7/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts |
| 部署与运维/部署与运维.md | 项目概述.md | 306/44 | 76/7 | 4/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts |
| 项目概述.md | 项目概述.md | 351/44 | 192/7 | 8/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts |

## 逐文件详情

### API接口文档/API接口文档.md

- reference 标题：API接口文档
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：62
- 行数：341 / 34
- 段落行数：146 / 6
- Mermaid：4 / 0
- 文件提及重合：app.py、func.py、gc_spider.py
- reference 关键文件未覆盖：config.yaml、spec.md、package.js、convert.ts、crawler.ts、index.ts、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、package.js、convert.ts、crawler.ts、index.ts、news.ts、vite.config.ts

### API接口文档/新闻API.md

- reference 标题：新闻API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：94
- 行数：377 / 34
- 段落行数：101 / 6
- Mermaid：5 / 0
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、index.ts、news.ts

### API接口文档/爬虫API.md

- reference 标题：爬虫API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 行数：323 / 34
- 段落行数：111 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md、add_sys_data_update_interval.sql、crawler.ts

### API接口文档/系统管理API.md

- reference 标题：系统管理API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：64
- 行数：330 / 34
- 段落行数：153 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py、gc_spider.py
- reference 关键文件未覆盖：config.yaml、mergetomkv.py、links.js、crawler.ts、index.ts、links.ts、vue.js、convert.func.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、mergetomkv.py、links.js、crawler.ts、index.ts、links.ts、vue.js、convert.func.py

### API接口文档/转换API.md

- reference 标题：转换API
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：84
- 行数：363 / 34
- 段落行数：146 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、xunlei.py

### 前端开发指南/API集成.md

- reference 标题：API集成
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：188
- 行数：380 / 41
- 段落行数：163 / 11
- Mermaid：8 / 0
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、news.ts、main.ts
- reference 关键文件未覆盖：nginx.conf、package.js、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、package.js、globalticker.ts、vite.config.ts

### 前端开发指南/UI设计系统.md

- reference 标题：UI设计系统
- 生成页：项目概述.md（项目概述）
- 匹配分数：78
- 行数：308 / 44
- 段落行数：108 / 7
- Mermaid：7 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 前端开发指南/Vue应用架构.md

- reference 标题：Vue应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：118
- 行数：434 / 37
- 段落行数：166 / 7
- Mermaid：8 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、links.ts、news.ts、globalticker.ts、tsconfig.app.js、tsconfig.js、tsconfig.node.js、uno.config.ts

### 前端开发指南/前端开发指南.md

- reference 标题：前端开发指南
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：78
- 行数：285 / 41
- 段落行数：104 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、links.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 前端开发指南/状态管理.md

- reference 标题：状态管理
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：78
- 行数：302 / 41
- 段落行数：120 / 11
- Mermaid：5 / 0
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 前端开发指南/组件开发/UI组件集成.md

- reference 标题：UI组件集成
- 生成页：项目概述.md（项目概述）
- 匹配分数：74
- 行数：395 / 44
- 段落行数：229 / 7
- Mermaid：9 / 0
- 文件提及重合：main.ts、index.ts
- reference 关键文件未覆盖：package.js、news.ts、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、uno.config.ts、vite.config.ts

### 前端开发指南/组件开发/基础组件开发.md

- reference 标题：基础组件开发
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：72
- 行数：475 / 41
- 段落行数：336 / 11
- Mermaid：12 / 0
- 文件提及重合：news.ts、main.ts、index.ts、crawler.ts、links.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 前端开发指南/组件开发/组件开发.md

- reference 标题：组件开发
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 行数：374 / 44
- 段落行数：127 / 7
- Mermaid：5 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、news.ts、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、news.ts、globalticker.ts、vite.config.ts

### 前端开发指南/组件开发/高级组件模式.md

- reference 标题：高级组件模式
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：78
- 行数：328 / 41
- 段落行数：130 / 11
- Mermaid：6 / 0
- 文件提及重合：crawler.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、globalticker.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、globalticker.ts、vite.config.ts

### 前端开发指南/路由系统.md

- reference 标题：路由系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：88
- 行数：254 / 41
- 段落行数：78 / 11
- Mermaid：5 / 0
- 文件提及重合：convert.ts、main.ts、index.ts、news.ts
- reference 关键文件未覆盖：package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、vite.config.ts

### 后端开发指南/API开发最佳实践.md

- reference 标题：API开发最佳实践
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：144
- 行数：336 / 34
- 段落行数：116 / 6
- Mermaid：5 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、config.yaml、async_fetch.py、crawler_config.py、utils.py、convert.ts、index.ts、news.ts

### 后端开发指南/Flask应用架构.md

- reference 标题：Flask应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 行数：378 / 37
- 段落行数：248 / 7
- Mermaid：11 / 0
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、links.js、async_fetch.py、crawler_config.py、gc_spider.py、gc_sql.py、utils.py

### 后端开发指南/后端开发指南.md

- reference 标题：后端开发指南
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：78
- 行数：351 / 34
- 段落行数：177 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、utils.py、links.js、async_fetch.py、crawler_config.py

### 后端开发指南/数据库操作系统.md

- reference 标题：数据库操作系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：68
- 行数：346 / 34
- 段落行数：145 / 6
- Mermaid：8 / 0
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 后端开发指南/爬虫引擎系统/同步爬取机制.md

- reference 标题：同步爬取机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 行数：345 / 34
- 段落行数：158 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py

### 后端开发指南/爬虫引擎系统/异步爬取机制.md

- reference 标题：异步爬取机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：64
- 行数：432 / 34
- 段落行数：248 / 6
- Mermaid：9 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、design.md、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、design.md、spec.md

### 后端开发指南/爬虫引擎系统/数据处理管道.md

- reference 标题：数据处理管道
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：124
- 行数：375 / 34
- 段落行数：142 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py、func.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql

### 后端开发指南/爬虫引擎系统/爬虫引擎系统.md

- reference 标题：爬虫引擎系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：114
- 行数：375 / 34
- 段落行数：158 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 后端开发指南/爬虫引擎系统/调度与定时机制.md

- reference 标题：调度与定时机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：64
- 行数：437 / 34
- 段落行数：310 / 6
- Mermaid：12 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、spec.md、design.md、add_sys_data_update_interval.sql、crawler.ts、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、spec.md、design.md、add_sys_data_update_interval.sql、crawler.ts、vue.js

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
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：86
- 行数：284 / 34
- 段落行数：127 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts

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
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：66
- 行数：357 / 34
- 段落行数：132 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py、gc_spider.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py

### 开发规范/Git工作流程.md

- reference 标题：Git工作流程
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：86
- 行数：235 / 50
- 段落行数：101 / 11
- Mermaid：6 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、config.yaml、package.js、pnpm-lock.yaml

### 开发规范/代码规范.md

- reference 标题：代码规范
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：116
- 行数：335 / 34
- 段落行数：105 / 6
- Mermaid：3 / 0
- 文件提及重合：app.py、func.py、gc_spider.py
- reference 关键文件未覆盖：config.yaml、eslint.conf、package.js、links.ts、main.ts、news.ts、index.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、eslint.conf、package.js、links.ts、main.ts、news.ts、index.ts、tsconfig.js

### 开发规范/开发工具配置.md

- reference 标题：开发工具配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发规范/开发规范.md

- reference 标题：开发规范
- 生成页：项目概述.md（项目概述）
- 匹配分数：158
- 行数：390 / 44
- 段落行数：129 / 7
- Mermaid：5 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、spec.md、readme.md、eslint.conf、package.js、vite.config.ts、design.md、tasks.md

### 开发规范/文档规范.md

- reference 标题：文档规范
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：324 / 44
- 段落行数：142 / 7
- Mermaid：6 / 0
- 文件提及重合：app.py、main.ts
- reference 关键文件未覆盖：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：code_of_conduct.md、contributing.md、readme.md、security.md、config.yaml、func.py、gc_spider.py、package.js

### 开发规范/测试规范.md

- reference 标题：测试规范
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：118
- 行数：361 / 34
- 段落行数：133 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：utils.py、spec.md、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、spec.md、package.js

### 快速开始.md

- reference 标题：快速开始
- 生成页：项目概述.md（项目概述）
- 匹配分数：120
- 行数：300 / 44
- 段落行数：86 / 7
- Mermaid：7 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、gc_sql.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、package.js

### 扩展与定制/UniSpec规范管理.md

- reference 标题：UniSpec规范管理
- 生成页：无
- 问题：缺少对应生成页面

### 扩展与定制/功能扩展开发.md

- reference 标题：功能扩展开发
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：146
- 行数：386 / 34
- 段落行数：142 / 6
- Mermaid：5 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_sys_data_update_interval.sql

### 扩展与定制/性能优化定制.md

- reference 标题：性能优化定制
- 生成页：系统架构.md（系统架构）
- 匹配分数：136
- 行数：318 / 37
- 段落行数：119 / 7
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、main.ts、index.ts
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、async_fetch.py、crawler_config.py、gc_sql.py、utils.py、add_all_indexes.sql

### 扩展与定制/扩展与定制.md

- reference 标题：扩展与定制
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：190
- 行数：339 / 41
- 段落行数：140 / 11
- Mermaid：5 / 0
- 文件提及重合：convert.ts、crawler.ts、index.ts、links.ts、main.ts、news.ts
- reference 关键文件未覆盖：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：app.py、config.yaml、compare_video.py、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py

### 扩展与定制/插件开发机制.md

- reference 标题：插件开发机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：148
- 行数：373 / 34
- 段落行数：151 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、links.js、async_fetch.py、crawler_config.py、utils.py、xunlei.py

### 扩展与定制/界面定制方案.md

- reference 标题：界面定制方案
- 生成页：项目概述.md（项目概述）
- 匹配分数：112
- 行数：384 / 44
- 段落行数：163 / 7
- Mermaid：12 / 0
- 文件提及重合：index.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 扩展与定制/第三方集成开发.md

- reference 标题：第三方集成开发
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：126
- 行数：404 / 34
- 段落行数：154 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、readme.md、config.yaml、async_fetch.py、utils.py、spec.md、package.js、index.ts

### 数据库设计/全文搜索功能.md

- reference 标题：全文搜索功能
- 生成页：项目概述.md（项目概述）
- 匹配分数：98
- 行数：250 / 44
- 段落行数：80 / 7
- Mermaid：4 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_sql.py、utils.py、spec.md、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql

### 数据库设计/数据库性能调优.md

- reference 标题：数据库性能调优
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：116
- 行数：304 / 34
- 段落行数：112 / 6
- Mermaid：5 / 0
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
- 匹配分数：80
- 行数：343 / 34
- 段落行数：117 / 6
- Mermaid：5 / 0
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、add_sys_data_update_interval.sql、news.ts

### 数据库设计/查询优化技术.md

- reference 标题：查询优化技术
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：66
- 行数：561 / 34
- 段落行数：382 / 6
- Mermaid：13 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、index.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql、index.ts

### 数据库设计/索引优化策略.md

- reference 标题：索引优化策略
- 生成页：无
- 问题：缺少对应生成页面

### 核心功能模块/内容展示系统/UI组件库.md

- reference 标题：UI组件库
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：224
- 行数：370 / 41
- 段落行数：149 / 11
- Mermaid：5 / 0
- 文件提及重合：crawler.ts、news.ts、main.ts、index.ts
- reference 关键文件未覆盖：globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：globalticker.ts

### 核心功能模块/内容展示系统/内容展示系统.md

- reference 标题：内容展示系统
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：156
- 行数：338 / 41
- 段落行数：153 / 11
- Mermaid：8 / 0
- 文件提及重合：convert.ts、index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、uno.config.ts、vite.config.ts

### 核心功能模块/内容展示系统/搜索功能页面.md

- reference 标题：搜索功能页面
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：150
- 行数：390 / 34
- 段落行数：139 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_sql.py
- reference 关键文件未覆盖：nginx.conf、add_fulltext_for_search.sql、index.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、add_fulltext_for_search.sql、index.ts、news.ts

### 核心功能模块/内容展示系统/新片速递页面.md

- reference 标题：新片速递页面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：142
- 行数：551 / 41
- 段落行数：417 / 11
- Mermaid：13 / 0
- 文件提及重合：index.ts、news.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 核心功能模块/内容展示系统/视频转换界面.md

- reference 标题：视频转换界面
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：120
- 行数：346 / 41
- 段落行数：185 / 11
- Mermaid：6 / 0
- 文件提及重合：convert.ts、main.ts、index.ts
- reference 关键文件未覆盖：func.py、output.py、utils.py、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：func.py、output.py、utils.py、globalticker.ts

### 核心功能模块/内容展示系统/首页新闻展示.md

- reference 标题：首页新闻展示
- 生成页：核心模块/web.md（模块：web）
- 匹配分数：144
- 行数：317 / 41
- 段落行数：150 / 11
- Mermaid：6 / 0
- 文件提及重合：index.ts、news.ts、main.ts
- reference 关键文件未覆盖：package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js

### 核心功能模块/新闻爬虫系统/内容处理与存储.md

- reference 标题：内容处理与存储
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：226
- 行数：351 / 34
- 段落行数：136 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、utils.py、async_fetch.py、crawler_config.py、add_all_indexes.sql、add_fulltext_for_search.sql、add_index_for_list.sql

### 核心功能模块/新闻爬虫系统/去重与筛选机制.md

- reference 标题：去重与筛选机制
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：196
- 行数：363 / 34
- 段落行数：161 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、add_all_indexes.sql、add_index_for_list.sql

### 核心功能模块/新闻爬虫系统/同步爬取实现.md

- reference 标题：同步爬取实现
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：198
- 行数：349 / 34
- 段落行数：153 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py

### 核心功能模块/新闻爬虫系统/异步爬取实现.md

- reference 标题：异步爬取实现
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：192
- 行数：326 / 34
- 段落行数：131 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 核心功能模块/新闻爬虫系统/数据解析与提取.md

- reference 标题：数据解析与提取
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：178
- 行数：378 / 34
- 段落行数：151 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、crawler_config.py、utils.py、index.ts、thread.php

### 核心功能模块/新闻爬虫系统/新闻爬虫系统.md

- reference 标题：新闻爬虫系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：196
- 行数：409 / 34
- 段落行数：163 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、async_fetch.py、crawler_config.py、utils.py、spec.md

### 核心功能模块/核心功能模块.md

- reference 标题：核心功能模块
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：224
- 行数：343 / 34
- 段落行数：133 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py、gc_spider.py、gc_sql.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、mergetomkv.py、async_fetch.py、crawler_config.py、index.ts、news.ts、main.ts

### 核心功能模块/视频转换系统/视频压缩功能.md

- reference 标题：视频压缩功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：128
- 行数：559 / 34
- 段落行数：284 / 6
- Mermaid：6 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts

### 核心功能模块/视频转换系统/视频合并功能.md

- reference 标题：视频合并功能
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：130
- 行数：348 / 34
- 段落行数：221 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py

### 核心功能模块/视频转换系统/视频处理工具集.md

- reference 标题：视频处理工具集
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：166
- 行数：363 / 34
- 段落行数：159 / 6
- Mermaid：9 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts、xunlei.py

### 核心功能模块/视频转换系统/视频质量比较.md

- reference 标题：视频质量比较
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：132
- 行数：510 / 34
- 段落行数：349 / 6
- Mermaid：12 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、output.py、utils.py、spec.md、convert.ts、vue.js

### 核心功能模块/视频转换系统/视频转换系统.md

- reference 标题：视频转换系统
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：132
- 行数：372 / 34
- 段落行数：148 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、spec.md、convert.ts、script.py

### 核心功能模块/视频转换系统/转换核心引擎.md

- reference 标题：转换核心引擎
- 生成页：核心模块/spider.md（模块：spider）
- 匹配分数：146
- 行数：355 / 34
- 段落行数：139 / 6
- Mermaid：7 / 0
- 文件提及重合：app.py、func.py
- reference 关键文件未覆盖：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、__init__.py、compare_video.py、compressmkv.py、mergetomkv.py、output.py、utils.py、convert.ts

### 系统架构/异步处理模式.md

- reference 标题：异步处理模式
- 生成页：系统架构.md（系统架构）
- 匹配分数：120
- 行数：367 / 37
- 段落行数：142 / 7
- Mermaid：7 / 0
- 文件提及重合：app.py
- reference 关键文件未覆盖：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、compressmkv.py、func.py、mergetomkv.py、output.py、utils.py、async_fetch.py、crawler_config.py

### 系统架构/技术栈选择.md

- reference 标题：技术栈选择
- 生成页：系统架构.md（系统架构）
- 匹配分数：218
- 行数：313 / 37
- 段落行数：116 / 7
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、package.js、news.ts、tsconfig.js、vite.config.ts

### 系统架构/数据流设计.md

- reference 标题：数据流设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：136
- 行数：373 / 37
- 段落行数：147 / 7
- Mermaid：8 / 0
- 文件提及重合：app.py、index.ts
- reference 关键文件未覆盖：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、links.js、async_fetch.py、gc_spider.py、gc_sql.py、utils.py、add_all_indexes.sql、add_fulltext_for_search.sql

### 系统架构/整体架构设计.md

- reference 标题：整体架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：222
- 行数：315 / 37
- 段落行数：120 / 7
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、gc_spider.py、gc_sql.py、package.js、convert.ts、crawler.ts、links.ts、news.ts

### 系统架构/系统架构.md

- reference 标题：系统架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：456
- 行数：386 / 37
- 段落行数：155 / 7
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts

### 系统架构/组件交互机制.md

- reference 标题：组件交互机制
- 生成页：系统架构.md（系统架构）
- 匹配分数：182
- 行数：326 / 37
- 段落行数：133 / 7
- Mermaid：6 / 0
- 文件提及重合：app.py、index.ts、main.ts
- reference 关键文件未覆盖：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、convert.ts、crawler.ts、news.ts、globalticker.ts

### 部署与运维/备份与恢复.md

- reference 标题：备份与恢复
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 行数：291 / 50
- 段落行数：73 / 11
- Mermaid：3 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、security.md、app.py、config.yaml、links.js、gc_spider.py、gc_sql.py、package.js

### 部署与运维/安全加固.md

- reference 标题：安全加固
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：92
- 行数：247 / 50
- 段落行数：88 / 11
- Mermaid：5 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：.yaml、nginx.conf、security.md、app.py、config.yaml、gc_spider.py、gc_sql.py、utils.py

### 部署与运维/服务部署/Docker容器化部署.md

- reference 标题：Docker容器化部署
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 行数：270 / 44
- 段落行数：57 / 7
- Mermaid：3 / 0
- 文件提及重合：nginx.conf、app.py、index.ts
- reference 关键文件未覆盖：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、package.js、vite.config.ts、node.js、docker-compose.yml、fastcgi.conf

### 部署与运维/服务部署/Flask后端服务部署.md

- reference 标题：Flask后端服务部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：96
- 行数：446 / 50
- 段落行数：159 / 11
- Mermaid：8 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、func.py、async_fetch.py、gc_spider.py、gc_sql.py、utils.py

### 部署与运维/服务部署/Nginx反向代理配置.md

- reference 标题：Nginx反向代理配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 行数：312 / 50
- 段落行数：165 / 11
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：fastcgi.conf、nginx.conf、readme.md、vue.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：fastcgi.conf、nginx.conf、readme.md、vue.js

### 部署与运维/服务部署/Vue前端应用部署.md

- reference 标题：Vue前端应用部署
- 生成页：项目概述.md（项目概述）
- 匹配分数：126
- 行数：292 / 44
- 段落行数：84 / 7
- Mermaid：3 / 0
- 文件提及重合：nginx.conf、main.ts、index.ts
- reference 关键文件未覆盖：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、package.js、tsconfig.app.js、tsconfig.node.js、uno.config.ts、vite.config.ts

### 部署与运维/服务部署/服务部署.md

- reference 标题：服务部署
- 生成页：项目概述.md（项目概述）
- 匹配分数：100
- 行数：291 / 44
- 段落行数：92 / 7
- Mermaid：6 / 0
- 文件提及重合：nginx.conf、app.py、main.ts
- reference 关键文件未覆盖：readme.md、config.yaml、package.js、vite.config.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、package.js、vite.config.ts、node.js

### 部署与运维/环境配置.md

- reference 标题：环境配置
- 生成页：项目概述.md（项目概述）
- 匹配分数：102
- 行数：329 / 44
- 段落行数：129 / 7
- Mermaid：7 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、config.yaml、crawler_config.py、gc_sql.py、package.js、crawler.ts、vite.config.ts、node.js

### 部署与运维/监控与日志.md

- reference 标题：监控与日志
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：94
- 行数：324 / 50
- 段落行数：147 / 11
- Mermaid：7 / 0
- 文件提及重合：无
- reference 关键文件未覆盖：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：nginx.conf、app.py、config.yaml、gc_spider.py、utils.py、design.md、spec.md、crawler.ts

### 部署与运维/部署与运维.md

- reference 标题：部署与运维
- 生成页：项目概述.md（项目概述）
- 匹配分数：104
- 行数：306 / 44
- 段落行数：76 / 7
- Mermaid：4 / 0
- 文件提及重合：nginx.conf、app.py
- reference 关键文件未覆盖：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、security.md、config.yaml、links.js、package.js、vite.config.ts

### 项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：736
- 行数：351 / 44
- 段落行数：192 / 7
- Mermaid：8 / 0
- 文件提及重合：nginx.conf、app.py、index.ts、main.ts
- reference 关键文件未覆盖：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：config.yaml、func.py、gc_spider.py、gc_sql.py、package.js、news.ts、vite.config.ts

