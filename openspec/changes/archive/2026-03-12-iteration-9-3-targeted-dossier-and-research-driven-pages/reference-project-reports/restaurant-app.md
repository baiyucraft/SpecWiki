# restaurant-app Reference 对比报告

生成页面：35 页
reference 页面：127 页
命中对比：76 页
缺失对比：51 页
运行模式：cold (cache_mode=clear)
LLM usage：requests=30, total_tokens=229262, page_research=12, page_enrichment=2

## 覆盖统计

- 专题页覆盖：generated 19 / reference 0（repo-archetype=4）
- evidence 落页：generated 30 / reference 127
- citation 密度：generated 0.97 / reference 75.67
- 图表达覆盖：generated 26 / reference 127
- page research 请求：12
- page enrichment 请求：2
- 已规划专题类型：专题页(13)、流程主题(6)
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

- 核心模块/src/backend/services.md 被 9 个 reference 页面共享映射
- 核心模块/src/backend/services/web.admin.md 被 3 个 reference 页面共享映射
- 核心模块/src/backend/services/web.admin/dashboard-app.md 被 5 个 reference 页面共享映射
- 系统架构.md 被 32 个 reference 页面共享映射
- 核心模块/src/backend/services/web.admin/dashboard.md 被 2 个 reference 页面共享映射
- 工作流与部署.md 被 15 个 reference 页面共享映射
- 专题/module-capability/module-b956eb7f2423-service-dashboard-app能力：服务协作.md 被 2 个 reference 页面共享映射
- 项目概述.md 被 5 个 reference 页面共享映射

## 额外生成页面

- 专题/module-capability/module-48423669fc7c-service-manifests能力：服务协作.md (manifests能力：服务协作, 24 行)
- 专题/module-capability/module-4bf92bb05517-service-web.admin能力：服务协作.md (web.admin能力：服务协作, 34 行)
- 专题/module-capability/module-55d3428a6f12-service-services能力：服务协作.md (services能力：服务协作, 32 行)
- 专题/module-capability/module-5f11b0ec8bcc-service-checkout-api能力：服务协作.md (checkout-api能力：服务协作, 53 行)
- 专题/module-capability/module-7664ec0cbf9c-service-catalog-api能力：服务协作.md (catalog-api能力：服务协作, 47 行)
- 专题/module-capability/module-826359ff36b2-service-dashboard能力：服务协作.md (dashboard能力：服务协作, 49 行)
- 专题/module-capability/module-b9d0ca451eea-service-web-app能力：服务协作.md (web-app能力：服务协作, 50 行)
- 专题/module-capability/module-dc9cc1e5ef17-service-src能力：服务协作.md (src能力：服务协作, 32 行)
- 专题/module-capability/module-e130f7775743-service-backend能力：服务协作.md (backend能力：服务协作, 39 行)
- 专题/process/process-create-flow-流程主题：Create-flow.md (流程主题：Create flow, 35 行)
- 专题/process/process-getcart-flow-流程主题：GetCart-flow.md (流程主题：GetCart flow, 25 行)
- 专题/process/process-handle-flow-流程主题：Handle-flow.md (流程主题：Handle flow, 35 行)
- 专题/process/process-main-flow-流程主题：main-flow.md (流程主题：main flow, 37 行)
- 专题/process/process-run_migrations-flow-流程主题：run_migrations-flow.md (流程主题：run_migrations flow, 45 行)
- 专题/process/process-startotel-flow-流程主题：StartOTEL-flow.md (流程主题：StartOTEL flow, 35 行)
- 专题/repo-archetype/config-runtime-配置与运行时.md (配置与运行时, 19 行)
- 专题/repo-archetype/core-api-models-核心-API-与数据模型.md (核心 API 与数据模型, 17 行)
- 专题/repo-archetype/ops-runtime-部署与环境.md (部署与环境, 17 行)
- 专题/repo-archetype/request-lifecycle-请求处理链.md (请求处理链, 19 行)
- 核心模块/src.md (模块：src, 71 行)
- 核心模块/src/backend.md (模块：backend, 96 行)
- 核心模块/src/backend/archive.md (模块：archive, 54 行)
- 核心模块/src/backend/archive/nginx.md (模块：nginx, 35 行)
- 核心模块/src/backend/services/web-app.md (模块：web-app, 89 行)

## 逐文件对比

| Reference | 生成页 | 行数(ref/gen) | 段落(ref/gen) | Evidence(ref/gen) | Mermaid(ref/gen) | 主要结论 |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| API接口文档/API安全与认证.md | 核心模块/src/backend/services.md | 299/83 | 133/24 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、prometheus.yaml、docker-compose.yml、checkout.js、configuration.go、package-lock.js、auth.ts、next.js |
| API接口文档/API接口文档.md | 核心模块/src/backend/services.md | 390/83 | 139/24 | 18/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、payment-methods.go |
| API接口文档/API测试指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/REST API规范/REST API规范.md | 核心模块/src/backend/services.md | 383/83 | 150/24 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、cart_handler.go、cargo.toml、package.js、node.js |
| API接口文档/REST API规范/目录API端点.md | 核心模块/src/backend/services.md | 326/83 | 92/24 | 0/1 | 3/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：catalog-api.postman_collection.js、schema.rs |
| API接口文档/REST API规范/结账API端点.md | 核心模块/src/backend/services/checkout-api.md | 242/87 | 84/21 | 11/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：checkout-api.postman_collection.js、package.js、model.ts、express.js |
| API接口文档/REST API规范/认证与授权.md | 核心模块/src/backend/services/web.admin.md | 281/69 | 113/14 | 17/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：local-docker.postman_environment.js、realm.js、auth.ts、next-auth-d.ts、auth.guard.ts、auth.service.ts、route.ts、nextauth.js |
| API接口文档/REST API规范/购物车API端点.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/REST API规范/错误处理与响应格式.md | 核心模块/src/backend/services/web.admin/dashboard-app.md | 334/51 | 110/16 | 0/2 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、http_error.go、logger.ts、payment-methods.go、middleware.ts |
| API接口文档/gRPC接口设计/Protocol Buffers定义.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/gRPC接口设计/gRPC接口设计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/gRPC接口设计/客户端集成指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/gRPC接口设计/支付服务API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| API接口文档/gRPC接口设计/购物车服务API.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 前端应用架构/Web应用（Next.js）/API集成与数据获取.md | 系统架构.md | 246/74 | 120/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basket-api.postman_collection.js、cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、config.spec.ts、instrumentation.ts |
| 前端应用架构/Web应用（Next.js）/Web应用（Next.js）.md | 系统架构.md | 277/74 | 104/16 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、page.ts、layout.ts、providers.ts、instrumentation.node.ts、instrumentation.ts、utils.ts |
| 前端应用架构/Web应用（Next.js）/应用架构设计.md | 核心模块/src/backend/services.md | 282/83 | 133/24 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、providers.ts、instrumentation.node.ts、middleware.ts、tailwind.config.ts、next.js |
| 前端应用架构/Web应用（Next.js）/性能优化.md | 系统架构.md | 239/74 | 81/16 | 14/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、instrumentation.node.ts、instrumentation.ts、middleware.ts、tailwind.config.ts、tsconfig.js |
| 前端应用架构/Web应用（Next.js）/样式系统与主题.md | 系统架构.md | 280/74 | 113/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、tailwind.config.ts、next.js |
| 前端应用架构/Web应用（Next.js）/状态管理.md | 系统架构.md | 276/74 | 138/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、next-auth-d.ts、next.js、nextauth.js |
| 前端应用架构/Web应用（Next.js）/组件系统.md | 核心模块/src/backend/services/web.admin/dashboard-app.md | 299/51 | 134/16 | 0/2 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、actions.ts、cart-detail.ts、cart-item.ts、open-cart.ts、checkout-form.ts、payment-details.ts |
| 前端应用架构/Web应用（Next.js）/页面路由管理.md | 系统架构.md | 285/74 | 100/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、page.ts、layout.ts、providers.ts、index.ts、instrumentation.ts、middleware.ts |
| 前端应用架构/Web管理后台（Angular）/Web管理后台（Angular）.md | 核心模块/src/backend/services/web.admin/dashboard.md | 194/56 | 92/21 | 0/2 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、angular.js、package.js、shared.module.ts |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/UI组件库.md | 系统架构.md | 414/74 | 189/16 | 0/1 | 10/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layout.ts、providers.ts、add-to-cart.ts、cart-item.ts、categories-sidebar.ts、checkout-form.ts、cart.ts、next.js |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/传统Angular仪表板.md | 系统架构.md | 261/74 | 131/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、tsconfig.js、readme.md、angular.js、app.component.ts、app.module.ts、next.js、test.ts |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/用户认证模块.md | 系统架构.md | 340/74 | 123/16 | 0/1 | 4/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：account-routing.module.ts、app.router.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts、base.service.ts、constants.ts、environment.prod.ts |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/菜品分类管理.md | 系统架构.md | 277/74 | 130/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：category.rs、schema.rs、add.component.ts、categories-routing.module.ts、categories.module.ts、list.component.ts、confirmation-dialog.component.ts、category.ts |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/菜品管理模块.md | 系统架构.md | 401/74 | 297/16 | 17/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：category.rs、upload.rs、categories-sidebar.ts、fetch.ts、food-item.ts、add.component.ts、food.ts、foodpicture.ts |
| 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/订单监控模块.md | 核心模块/src/backend/services/web.admin/dashboard-app.md | 320/51 | 224/16 | 17/2 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：orderscontroller.java、checkout-completed.ts、order.ts、order-table-row.ts |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/UI组件库与设计系统.md | 系统架构.md | 230/74 | 95/16 | 16/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、tailwind.config.ts、next.js、dashboard-next.config.js、next.conf、web-app-next.config.js |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/产品管理系统.md | 系统架构.md | 325/74 | 151/16 | 0/1 | 9/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：up.sql、catalog.rs、upload.rs、category.rs、schema.rs、seed.rs、catalog_handler.rs、upload_handler.rs |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/仪表板概览与架构.md | 系统架构.md | 284/74 | 126/16 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、layout.ts、providers.ts、instrumentation.ts、utils.ts、middleware.ts |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/客户管理系统.md | 系统架构.md | 350/74 | 158/16 | 21/1 | 9/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：identity-pgsql.yaml、orderscontroller.java、page.ts、checkout-completed.ts、user-profile.ts、fetch.ts、order.ts、auth.ts |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/现代化Next.js仪表板应用.md | 系统架构.md | 338/74 | 213/16 | 16/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、index.ts、layout.ts、providers.ts、instrumentation.ts、utils.ts |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/订单管理系统.md | 系统架构.md | 268/74 | 110/16 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：order-api.yml、orderscontroller.java、ordersservice.java、ordersservicesiml.java、orderscontrollertests.java、fetch.ts |
| 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/认证与路由系统.md | 系统架构.md | 387/74 | 273/16 | 16/1 | 9/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：realm.js、next.config.js、auth.ts、middleware.ts、login.component.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts |
| 前端应用架构/前端应用架构.md | 核心模块/src/backend/services/web.admin/dashboard.md | 332/56 | 152/21 | 0/2 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、index.ts、layout.ts、providers.ts、tailwind.config.ts、angular.js、storage.service.ts |
| 后端服务详解/Cart API (购物车服务).md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端服务详解/Catalog API (目录管理服务).md | 核心模块/src/backend/services/catalog-api.md | 401/77 | 152/15 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：catalog-api.postman_collection.js、cargo.toml、db.rs、schema.rs、openapi.js |
| 后端服务详解/Checkout API (结账服务).md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端服务详解/Identity API (用户认证服务).md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端服务详解/Order API (订单处理服务).md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端服务详解/Payment API (支付服务).md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 后端服务详解/后端服务详解.md | 核心模块/manifests.md | 548/54 | 177/7 | 0/1 | 9/0 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、telemetry.yaml、kafka-stateful-set.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml |
| 基础设施与部署/CI_CD流水线.md | 工作流与部署.md | 388/106 | 157/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、readme.md、next.js |
| 基础设施与部署/Kubernetes部署配置.md | 工作流与部署.md | 314/106 | 155/11 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：kustomization.yaml、namespace.yaml、namespaces.yaml |
| 基础设施与部署/基础设施与部署.md | 工作流与部署.md | 365/106 | 168/11 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、istio-operator-crds.yaml、kustomization.yaml、gateway.yaml、telemetry.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、service.yaml |
| 基础设施与部署/消息队列与缓存.md | 工作流与部署.md | 326/106 | 116/11 | 16/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、namespace.yaml、pvc.yaml、service.yaml、redis.yaml、main.go |
| 基础设施与部署/监控与可观测性.md | 工作流与部署.md | 369/106 | 132/11 | 0/1 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：grafana-datasources.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml、zipkin.yaml、deployment.yaml |
| 安全与认证/API安全策略.md | 核心模块/src/backend/services/web.admin.md | 311/69 | 107/14 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package-lock.js、payments.pb.go、middleware.ts |
| 安全与认证/会话管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全与认证/安全与认证.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全与认证/数据保护与安全.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全与认证/权限控制系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 安全与认证/身份认证系统.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/CI_CD流水线/Azure Pipelines配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/CI_CD流水线/CI_CD流水线.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/CI_CD流水线/GitHub Actions工作流.md | 工作流与部署.md | 307/106 | 112/11 | 0/1 | 5/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：action.yml、build-release.yml、checkout-api.yml、codeql-analysis.yml、deploy.yml、order-api.yml、payment-api.yml、web-app.yml |
| 开发者指南/CI_CD流水线/代码质量检查.md | 核心模块/src/backend/services/web.admin/dashboard-app.md | 414/51 | 148/16 | 25/2 | 10/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、cargo.toml、package.js、prettier.config.js、tailwind.config.ts、tsconfig.js |
| 开发者指南/CI_CD流水线/容器化CI_CD.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/CI_CD流水线/移动应用CI_CD.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/代码贡献流程.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发最佳实践/代码质量保证.md | 专题/module-capability/module-b956eb7f2423-service-dashboard-app能力：服务协作.md | 384/45 | 122/20 | 20/2 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.gitpod.yml、.rustfmt.toml、azure-pipelines.yml、cargo.toml、package.js、tslint.js、prettier.config.js |
| 开发者指南/开发最佳实践/可观测性与监控.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发最佳实践/多语言微服务开发.md | 核心模块/src/backend/services.md | 527/83 | 160/24 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、minio.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml |
| 开发者指南/开发最佳实践/容器化与DevOps.md | 工作流与部署.md | 365/106 | 158/11 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、azure-pipelines.yml、kafka-stateful-set.yaml、grafana.yaml、prometheus.yaml、deployment.yaml、http-route.yaml、service.yaml |
| 开发者指南/开发最佳实践/开发工具与调试.md | 核心模块/src/backend/services.md | 401/83 | 177/24 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、docker-compose.override.yml、docker-compose.yml、instrumentation.go、cargo.toml |
| 开发者指南/开发最佳实践/开发最佳实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发最佳实践/微服务设计原则.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发最佳实践/测试策略与实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发环境配置/Docker容器化配置/Docker Compose配置详解.md | 工作流与部署.md | 469/106 | 255/11 | 22/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：prometheus.yaml、tempo.yaml、next.js |
| 开发者指南/开发环境配置/Docker容器化配置/Docker容器化配置.md | 工作流与部署.md | 540/106 | 415/11 | 24/1 | 14/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docker-compose.load-tests.yml、next.js、node.js、docker-compose.grafana.yml、realm.js |
| 开发者指南/开发环境配置/Docker容器化配置/容器网络配置.md | 工作流与部署.md | 372/106 | 186/11 | 0/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、istio-system.yaml、kafka-stateful-set.yaml、service.yaml、deployment.yaml、http-route.yaml |
| 开发者指南/开发环境配置/Docker容器化配置/微服务Dockerfile配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发环境配置/Docker容器化配置/数据持久化与卷管理.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发环境配置/IDE开发工具配置/IDE开发工具配置.md | 核心模块/src/backend/services/web.admin/dashboard-app.md | 426/51 | 182/16 | 21/2 | 9/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、tslint.js、prettier.config.js、tailwind.config.ts、node.js |
| 开发者指南/开发环境配置/IDE开发工具配置/IntelliJ IDEA配置.md | 专题/module-capability/module-b956eb7f2423-service-dashboard-app能力：服务协作.md | 318/45 | 110/20 | 16/2 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、prettier.config.js、node.js、package-lock.js |
| 开发者指南/开发环境配置/IDE开发工具配置/VS Code工作区配置.md | 核心模块/src/backend/services/web.admin.md | 414/69 | 254/14 | 16/1 | 8/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、package.js、tsconfig.js |
| 开发者指南/开发环境配置/IDE开发工具配置/Visual Studio配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发环境配置/IDE开发工具配置/语言特定开发环境.md | 核心模块/src/backend/services.md | 357/83 | 113/24 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、package.js、tsconfig.js、next.js、node.js、index.js、pnpm-lock.yaml |
| 开发者指南/开发环境配置/Vagrant虚拟机配置.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发环境配置/开发环境配置.md | 工作流与部署.md | 440/106 | 146/11 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、local-docker.postman_environment.js、node.js、next.js、index.js、server.js |
| 开发者指南/开发环境配置/本地环境搭建.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/开发者指南.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除/开发环境调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除/性能问题诊断.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除/服务间通信调试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除/生产环境故障排除.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 开发者指南/调试与故障排除/调试与故障排除.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 微服务架构设计/微服务架构设计.md | 系统架构.md | 276/74 | 101/16 | 0/1 | 3/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、deployment.yaml、cargo.toml、package.js、next.js、node.js |
| 微服务架构设计/数据流设计.md | 系统架构.md | 451/74 | 180/16 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、kustomization.yaml、namespace.yaml、pvc.yaml、service.yaml、deployment.yaml |
| 微服务架构设计/整体架构概览.md | 系统架构.md | 214/74 | 76/16 | 0/1 | 3/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、node.js、next.js |
| 微服务架构设计/服务发现与治理.md | 系统架构.md | 389/74 | 150/16 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、istio-system.yaml、kustomization.yaml、telemetry.yaml、grafana.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml |
| 微服务架构设计/服务间通信模式.md | 系统架构.md | 381/74 | 168/16 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、deployment.yaml、service.yaml、cart_handler.go、message_reciever.go、index.ts、publisher.ts、usercheckouteventhandler.java |
| 微服务架构设计/错误处理策略.md | 系统架构.md | 315/74 | 163/16 | 0/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：otel-collector.yaml、cart-api.postman_collection.js、otel-connector-config.yaml、tempo.yaml、cart_handler.go、http_error.go、instrumentation.ts、middleware.ts |
| 快速开始.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/前端性能优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/微服务性能优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/性能优化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/监控与指标收集.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 性能优化/负载测试与压力测试.md | 工作流与部署.md | 300/106 | 126/11 | 0/1 | 7/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：kustomization.yaml、deployment.yaml、performance-test-dasboard.js、datasource.yaml、prometheus.yaml、checkout.js、otel.yml |
| 故障排除与维护/常见问题解决.md | 工作流与部署.md | 462/106 | 96/11 | 0/1 | 3/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、local-docker.postman_environment.js、nginx.conf、dashboard.yaml、influxdb-dashboard.js、performance-test-dasboard.js、datasource.yaml |
| 故障排除与维护/故障排除与维护.md | 工作流与部署.md | 373/106 | 119/11 | 0/1 | 4/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、kustomization.yaml、prometheus.yaml、deployment.yaml、datasource.yaml、node.js |
| 故障排除与维护/日志分析与监控.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除与维护/系统维护.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 故障排除与维护/运维工具与实践.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据库架构.md | 系统架构.md | 328/74 | 122/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：menu-pgsql.yaml、identity-pgsql.yaml、order-pgsql.yaml、docker-compose.yml、diesel.toml、schema.rs |
| 数据库设计/数据库设计.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据库迁移.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据模型.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 数据库设计/数据种子管理.md | 核心模块/src/backend/services.md | 256/83 | 100/24 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：down.sql、up.sql、schema.rs、mod.rs |
| 测试策略/单元测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/测试策略.md | 工作流与部署.md | 310/106 | 121/11 | 0/1 | 6/1 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、local-docker.postman_environment.js、checkout.js、cart_handler_test.go、cargo.toml、package.js、payment-methods_test.go |
| 测试策略/测试自动化.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 测试策略/集成测试.md | 缺失 | - | - | - | - | 缺少对应生成页面 |
| 移动应用架构/UI组件与页面.md | 系统架构.md | 326/74 | 132/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 移动应用架构/平台特定实现.md | 系统架构.md | 334/74 | 123/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 移动应用架构/抽象层设计.md | 系统架构.md | 422/74 | 202/16 | 0/1 | 8/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 移动应用架构/核心逻辑层.md | 系统架构.md | 453/74 | 235/16 | 22/1 | 10/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块 |
| 移动应用架构/移动应用架构.md | 系统架构.md | 287/74 | 158/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 移动应用架构/移动应用架构概览.md | 系统架构.md | 268/74 | 112/16 | 0/1 | 6/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块 |
| 移动应用架构/移动应用测试.md | 系统架构.md | 319/74 | 125/16 | 22/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块 |
| 项目概述/快速开始指南.md | 项目概述.md | 262/93 | 102/21 | 0/1 | 5/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、deployment.yaml、local-docker.postman_environment.js、docker-compose.yml、next.js、node.js |
| 项目概述/技术栈概览.md | 项目概述.md | 312/93 | 108/21 | 0/1 | 3/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、kustomization.yaml、deployment.yaml、cargo.toml、package.js、apinode.js、next.js、node.js |
| 项目概述/架构设计理念.md | 项目概述.md | 395/93 | 183/21 | 0/1 | 9/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、service.yaml、cargo.toml、package.js、node.js、next.js |
| 项目概述/项目介绍与目标.md | 项目概述.md | 315/93 | 185/21 | 19/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.yml、contributing.md、readme.md、azure-pipelines.yml、deployment.yaml、docker-compose.yml、cargo.toml、package.js |
| 项目概述/项目概述.md | 项目概述.md | 334/93 | 179/21 | 21/1 | 7/2 | 内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、kafka-stateful-set.yaml、grafana.yaml、deployment.yaml、docker-compose.yml、cargo.toml、package.js |

## 逐文件详情

### API接口文档/API安全与认证.md

- reference 标题：API安全与认证
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：88
- 页面类型：other / module
- 行数：299 / 83
- 段落行数：133 / 24
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：nginx.conf、health.rs、config.spec.ts
- reference 关键文件未覆盖：gateway.yaml、prometheus.yaml、docker-compose.yml、checkout.js、configuration.go、package-lock.js、auth.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、prometheus.yaml、docker-compose.yml、checkout.js、configuration.go、package-lock.js、auth.ts、next.js

### API接口文档/API接口文档.md

- reference 标题：API接口文档
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：74
- 页面类型：other / module
- 行数：390 / 83
- 段落行数：139 / 24
- Evidence：18 / 1
- Mermaid：7 / 2
- 文件提及重合：main.go、catalog.rs、main.rs、index.ts、routes.ts
- reference 关键文件未覆盖：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、payment-methods.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、payment-methods.go

### API接口文档/API测试指南.md

- reference 标题：API测试指南
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/REST API规范/REST API规范.md

- reference 标题：REST API规范
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：66
- 页面类型：other / module
- 行数：383 / 83
- 段落行数：150 / 24
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：main.go、main.rs、index.ts、routes.ts
- reference 关键文件未覆盖：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、cart_handler.go、cargo.toml、package.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、cart_handler.go、cargo.toml、package.js、node.js

### API接口文档/REST API规范/目录API端点.md

- reference 标题：目录API端点
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：60
- 页面类型：other / module
- 行数：326 / 83
- 段落行数：92 / 24
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：catalog.rs、category.rs、upload.rs、main.rs
- reference 关键文件未覆盖：catalog-api.postman_collection.js、schema.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：catalog-api.postman_collection.js、schema.rs

### API接口文档/REST API规范/结账API端点.md

- reference 标题：结账API端点
- 生成页：核心模块/src/backend/services/checkout-api.md（模块：checkout-api）
- 匹配分数：72
- 页面类型：other / module
- 行数：242 / 87
- 段落行数：84 / 21
- Evidence：11 / 1
- Mermaid：4 / 1
- 文件提及重合：index.ts、routes.ts
- reference 关键文件未覆盖：checkout-api.postman_collection.js、package.js、model.ts、express.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：checkout-api.postman_collection.js、package.js、model.ts、express.js

### API接口文档/REST API规范/认证与授权.md

- reference 标题：认证与授权
- 生成页：核心模块/src/backend/services/web.admin.md（模块：web.admin）
- 匹配分数：62
- 页面类型：other / module
- 行数：281 / 69
- 段落行数：113 / 14
- Evidence：17 / 1
- Mermaid：6 / 1
- 文件提及重合：next.js
- reference 关键文件未覆盖：local-docker.postman_environment.js、realm.js、auth.ts、next-auth-d.ts、auth.guard.ts、auth.service.ts、route.ts、nextauth.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：local-docker.postman_environment.js、realm.js、auth.ts、next-auth-d.ts、auth.guard.ts、auth.service.ts、route.ts、nextauth.js

### API接口文档/REST API规范/购物车API端点.md

- reference 标题：购物车API端点
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/REST API规范/错误处理与响应格式.md

- reference 标题：错误处理与响应格式
- 生成页：核心模块/src/backend/services/web.admin/dashboard-app.md（模块：dashboard-app）
- 匹配分数：74
- 页面类型：other / module
- 行数：334 / 51
- 段落行数：110 / 16
- Evidence：0 / 2
- Mermaid：6 / 1
- 文件提及重合：fetch.ts、next.js
- reference 关键文件未覆盖：cart-api.postman_collection.js、http_error.go、logger.ts、payment-methods.go、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、http_error.go、logger.ts、payment-methods.go、middleware.ts

### API接口文档/gRPC接口设计/Protocol Buffers定义.md

- reference 标题：Protocol Buffers定义
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/gRPC接口设计/gRPC接口设计.md

- reference 标题：gRPC接口设计
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/gRPC接口设计/客户端集成指南.md

- reference 标题：客户端集成指南
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/gRPC接口设计/支付服务API.md

- reference 标题：支付服务API
- 生成页：无
- 问题：缺少对应生成页面

### API接口文档/gRPC接口设计/购物车服务API.md

- reference 标题：购物车服务API
- 生成页：无
- 问题：缺少对应生成页面

### 前端应用架构/Web应用（Next.js）/API集成与数据获取.md

- reference 标题：API集成与数据获取
- 生成页：系统架构.md（系统架构）
- 匹配分数：102
- 页面类型：architecture / architecture
- 行数：246 / 74
- 段落行数：120 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、basket-api.postman_collection.js、cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、config.spec.ts、instrumentation.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、basket-api.postman_collection.js、cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、config.spec.ts、instrumentation.ts

### 前端应用架构/Web应用（Next.js）/Web应用（Next.js）.md

- reference 标题：Web应用（Next.js）
- 生成页：系统架构.md（系统架构）
- 匹配分数：98
- 页面类型：architecture / architecture
- 行数：277 / 74
- 段落行数：104 / 16
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、page.ts、layout.ts、providers.ts、instrumentation.node.ts、instrumentation.ts、utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、page.ts、layout.ts、providers.ts、instrumentation.node.ts、instrumentation.ts、utils.ts

### 前端应用架构/Web应用（Next.js）/应用架构设计.md

- reference 标题：应用架构设计
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：102
- 页面类型：architecture / module
- 行数：282 / 83
- 段落行数：133 / 24
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：layout.ts、instrumentation.ts
- reference 关键文件未覆盖：next.config.js、package.js、postcss.config.js、providers.ts、instrumentation.node.ts、middleware.ts、tailwind.config.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、providers.ts、instrumentation.node.ts、middleware.ts、tailwind.config.ts、next.js

### 前端应用架构/Web应用（Next.js）/性能优化.md

- reference 标题：性能优化
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 页面类型：architecture / architecture
- 行数：239 / 74
- 段落行数：81 / 16
- Evidence：14 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、postcss.config.js、instrumentation.node.ts、instrumentation.ts、middleware.ts、tailwind.config.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、instrumentation.node.ts、instrumentation.ts、middleware.ts、tailwind.config.ts、tsconfig.js

### 前端应用架构/Web应用（Next.js）/样式系统与主题.md

- reference 标题：样式系统与主题
- 生成页：系统架构.md（系统架构）
- 匹配分数：98
- 页面类型：architecture / architecture
- 行数：280 / 74
- 段落行数：113 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、postcss.config.js、tailwind.config.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、tailwind.config.ts、next.js

### 前端应用架构/Web应用（Next.js）/状态管理.md

- reference 标题：状态管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：98
- 页面类型：architecture / architecture
- 行数：276 / 74
- 段落行数：138 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、next-auth-d.ts、next.js、nextauth.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、next-auth-d.ts、next.js、nextauth.js

### 前端应用架构/Web应用（Next.js）/组件系统.md

- reference 标题：组件系统
- 生成页：核心模块/src/backend/services/web.admin/dashboard-app.md（模块：dashboard-app）
- 匹配分数：96
- 页面类型：architecture / module
- 行数：299 / 51
- 段落行数：134 / 16
- Evidence：0 / 2
- Mermaid：5 / 1
- 文件提及重合：page.ts、layout.ts、next.js
- reference 关键文件未覆盖：next.config.js、package.js、actions.ts、cart-detail.ts、cart-item.ts、open-cart.ts、checkout-form.ts、payment-details.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、actions.ts、cart-detail.ts、cart-item.ts、open-cart.ts、checkout-form.ts、payment-details.ts

### 前端应用架构/Web应用（Next.js）/页面路由管理.md

- reference 标题：页面路由管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 页面类型：architecture / architecture
- 行数：285 / 74
- 段落行数：100 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、page.ts、layout.ts、providers.ts、index.ts、instrumentation.ts、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、page.ts、layout.ts、providers.ts、index.ts、instrumentation.ts、middleware.ts

### 前端应用架构/Web管理后台（Angular）/Web管理后台（Angular）.md

- reference 标题：Web管理后台（Angular）
- 生成页：核心模块/src/backend/services/web.admin/dashboard.md（模块：dashboard）
- 匹配分数：136
- 页面类型：architecture / module
- 行数：194 / 56
- 段落行数：92 / 21
- Evidence：0 / 2
- Mermaid：5 / 1
- 文件提及重合：app.module.ts、app.router.ts、main.ts
- reference 关键文件未覆盖：readme.md、angular.js、package.js、shared.module.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、angular.js、package.js、shared.module.ts

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/UI组件库.md

- reference 标题：UI组件库
- 生成页：系统架构.md（系统架构）
- 匹配分数：90
- 页面类型：architecture / architecture
- 行数：414 / 74
- 段落行数：189 / 16
- Evidence：0 / 1
- Mermaid：10 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：layout.ts、providers.ts、add-to-cart.ts、cart-item.ts、categories-sidebar.ts、checkout-form.ts、cart.ts、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：layout.ts、providers.ts、add-to-cart.ts、cart-item.ts、categories-sidebar.ts、checkout-form.ts、cart.ts、next.js

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/传统Angular仪表板.md

- reference 标题：传统Angular仪表板
- 生成页：系统架构.md（系统架构）
- 匹配分数：114
- 页面类型：architecture / architecture
- 行数：261 / 74
- 段落行数：131 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：main.ts
- reference 关键文件未覆盖：package.js、tsconfig.js、readme.md、angular.js、app.component.ts、app.module.ts、next.js、test.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package.js、tsconfig.js、readme.md、angular.js、app.component.ts、app.module.ts、next.js、test.ts

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/用户认证模块.md

- reference 标题：用户认证模块
- 生成页：系统架构.md（系统架构）
- 匹配分数：108
- 页面类型：architecture / architecture
- 行数：340 / 74
- 段落行数：123 / 16
- Evidence：0 / 1
- Mermaid：4 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：account-routing.module.ts、app.router.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts、base.service.ts、constants.ts、environment.prod.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：account-routing.module.ts、app.router.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts、base.service.ts、constants.ts、environment.prod.ts

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/菜品分类管理.md

- reference 标题：菜品分类管理
- 生成页：系统架构.md（系统架构）
- 匹配分数：104
- 页面类型：architecture / architecture
- 行数：277 / 74
- 段落行数：130 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：category.rs、schema.rs、add.component.ts、categories-routing.module.ts、categories.module.ts、list.component.ts、confirmation-dialog.component.ts、category.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：category.rs、schema.rs、add.component.ts、categories-routing.module.ts、categories.module.ts、list.component.ts、confirmation-dialog.component.ts、category.ts

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/菜品管理模块.md

- reference 标题：菜品管理模块
- 生成页：系统架构.md（系统架构）
- 匹配分数：124
- 页面类型：architecture / architecture
- 行数：401 / 74
- 段落行数：297 / 16
- Evidence：17 / 1
- Mermaid：8 / 2
- 文件提及重合：main.rs
- reference 关键文件未覆盖：category.rs、upload.rs、categories-sidebar.ts、fetch.ts、food-item.ts、add.component.ts、food.ts、foodpicture.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：category.rs、upload.rs、categories-sidebar.ts、fetch.ts、food-item.ts、add.component.ts、food.ts、foodpicture.ts

### 前端应用架构/Web管理后台（Angular）/传统Angular仪表板/订单监控模块.md

- reference 标题：订单监控模块
- 生成页：核心模块/src/backend/services/web.admin/dashboard-app.md（模块：dashboard-app）
- 匹配分数：162
- 页面类型：architecture / module
- 行数：320 / 51
- 段落行数：224 / 16
- Evidence：17 / 2
- Mermaid：8 / 1
- 文件提及重合：page.ts、fetch.ts、orders-table.ts、next.js
- reference 关键文件未覆盖：orderscontroller.java、checkout-completed.ts、order.ts、order-table-row.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：orderscontroller.java、checkout-completed.ts、order.ts、order-table-row.ts

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/UI组件库与设计系统.md

- reference 标题：UI组件库与设计系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：94
- 页面类型：architecture / architecture
- 行数：230 / 74
- 段落行数：95 / 16
- Evidence：16 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、tailwind.config.ts、next.js、dashboard-next.config.js、next.conf、web-app-next.config.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、tailwind.config.ts、next.js、dashboard-next.config.js、next.conf、web-app-next.config.js

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/产品管理系统.md

- reference 标题：产品管理系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：130
- 页面类型：architecture / architecture
- 行数：325 / 74
- 段落行数：151 / 16
- Evidence：0 / 1
- Mermaid：9 / 2
- 文件提及重合：main.rs
- reference 关键文件未覆盖：up.sql、catalog.rs、upload.rs、category.rs、schema.rs、seed.rs、catalog_handler.rs、upload_handler.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：up.sql、catalog.rs、upload.rs、category.rs、schema.rs、seed.rs、catalog_handler.rs、upload_handler.rs

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/仪表板概览与架构.md

- reference 标题：仪表板概览与架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：94
- 页面类型：architecture / architecture
- 行数：284 / 74
- 段落行数：126 / 16
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、postcss.config.js、layout.ts、providers.ts、instrumentation.ts、utils.ts、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、layout.ts、providers.ts、instrumentation.ts、utils.ts、middleware.ts

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/客户管理系统.md

- reference 标题：客户管理系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：112
- 页面类型：architecture / architecture
- 行数：350 / 74
- 段落行数：158 / 16
- Evidence：21 / 1
- Mermaid：9 / 2
- 文件提及重合：main.go
- reference 关键文件未覆盖：identity-pgsql.yaml、orderscontroller.java、page.ts、checkout-completed.ts、user-profile.ts、fetch.ts、order.ts、auth.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：identity-pgsql.yaml、orderscontroller.java、page.ts、checkout-completed.ts、user-profile.ts、fetch.ts、order.ts、auth.ts

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/现代化Next.js仪表板应用.md

- reference 标题：现代化Next.js仪表板应用
- 生成页：系统架构.md（系统架构）
- 匹配分数：104
- 页面类型：architecture / architecture
- 行数：338 / 74
- 段落行数：213 / 16
- Evidence：16 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：next.config.js、package.js、postcss.config.js、index.ts、layout.ts、providers.ts、instrumentation.ts、utils.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、postcss.config.js、index.ts、layout.ts、providers.ts、instrumentation.ts、utils.ts

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/订单管理系统.md

- reference 标题：订单管理系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：268 / 74
- 段落行数：110 / 16
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：order-api.yml、orderscontroller.java、ordersservice.java、ordersservicesiml.java、orderscontrollertests.java、fetch.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：order-api.yml、orderscontroller.java、ordersservice.java、ordersservicesiml.java、orderscontrollertests.java、fetch.ts

### 前端应用架构/Web管理后台（Angular）/现代化Next.js仪表板应用/认证与路由系统.md

- reference 标题：认证与路由系统
- 生成页：系统架构.md（系统架构）
- 匹配分数：112
- 页面类型：architecture / architecture
- 行数：387 / 74
- 段落行数：273 / 16
- Evidence：16 / 1
- Mermaid：9 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：realm.js、next.config.js、auth.ts、middleware.ts、login.component.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：realm.js、next.config.js、auth.ts、middleware.ts、login.component.ts、auth-callback.component.ts、auth.guard.ts、auth.service.ts

### 前端应用架构/前端应用架构.md

- reference 标题：前端应用架构
- 生成页：核心模块/src/backend/services/web.admin/dashboard.md（模块：dashboard）
- 匹配分数：180
- 页面类型：architecture / module
- 行数：332 / 56
- 段落行数：152 / 21
- Evidence：0 / 2
- Mermaid：6 / 1
- 文件提及重合：app.module.ts、app.router.ts、auth.guard.ts、main.ts
- reference 关键文件未覆盖：next.config.js、package.js、index.ts、layout.ts、providers.ts、tailwind.config.ts、angular.js、storage.service.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：next.config.js、package.js、index.ts、layout.ts、providers.ts、tailwind.config.ts、angular.js、storage.service.ts

### 后端服务详解/Cart API (购物车服务).md

- reference 标题：购物车服务（Cart API）
- 生成页：无
- 问题：缺少对应生成页面

### 后端服务详解/Catalog API (目录管理服务).md

- reference 标题：目录管理服务（Catalog API）
- 生成页：核心模块/src/backend/services/catalog-api.md（模块：catalog-api）
- 匹配分数：78
- 页面类型：other / module
- 行数：401 / 77
- 段落行数：152 / 15
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：catalog.rs、category.rs、upload.rs、main.rs、seed.rs
- reference 关键文件未覆盖：catalog-api.postman_collection.js、cargo.toml、db.rs、schema.rs、openapi.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：catalog-api.postman_collection.js、cargo.toml、db.rs、schema.rs、openapi.js

### 后端服务详解/Checkout API (结账服务).md

- reference 标题：结账API（结账服务）
- 生成页：无
- 问题：缺少对应生成页面

### 后端服务详解/Identity API (用户认证服务).md

- reference 标题：Identity API（用户认证服务）
- 生成页：无
- 问题：缺少对应生成页面

### 后端服务详解/Order API (订单处理服务).md

- reference 标题：订单API（订单处理服务）
- 生成页：无
- 问题：缺少对应生成页面

### 后端服务详解/Payment API (支付服务).md

- reference 标题：Payment API（支付服务）
- 生成页：无
- 问题：缺少对应生成页面

### 后端服务详解/后端服务详解.md

- reference 标题：后端服务详解
- 生成页：核心模块/manifests.md（模块：manifests）
- 匹配分数：168
- 页面类型：other / module
- 行数：548 / 54
- 段落行数：177 / 7
- Evidence：0 / 1
- Mermaid：9 / 0
- 文件提及重合：service.yaml、http-route.yaml、kustomization.yaml、main.go
- reference 关键文件未覆盖：readme.md、gateway.yaml、telemetry.yaml、kafka-stateful-set.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、telemetry.yaml、kafka-stateful-set.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml

### 基础设施与部署/CI_CD流水线.md

- reference 标题：CI/CD流水线
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：302
- 页面类型：workflow / workflow
- 行数：388 / 106
- 段落行数：157 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：docker-compose.elk.yml、docker-compose.kafka.yml、docker-compose.load-tests.yaml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.traefik.yml、docker-compose.yml
- reference 关键文件未覆盖：azure-pipelines.yml、readme.md、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、readme.md、next.js

### 基础设施与部署/Kubernetes部署配置.md

- reference 标题：Kubernetes部署配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：96
- 页面类型：workflow / workflow
- 行数：314 / 106
- 段落行数：155 / 11
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：kustomization.yaml、namespace.yaml、namespaces.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：kustomization.yaml、namespace.yaml、namespaces.yaml

### 基础设施与部署/基础设施与部署.md

- reference 标题：基础设施与部署
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：100
- 页面类型：workflow / workflow
- 行数：365 / 106
- 段落行数：168 / 11
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：azure-pipelines.yml、istio-operator-crds.yaml、kustomization.yaml、gateway.yaml、telemetry.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、service.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、istio-operator-crds.yaml、kustomization.yaml、gateway.yaml、telemetry.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、service.yaml

### 基础设施与部署/消息队列与缓存.md

- reference 标题：消息队列与缓存
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：94
- 页面类型：workflow / workflow
- 行数：326 / 106
- 段落行数：116 / 11
- Evidence：16 / 1
- Mermaid：7 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、namespace.yaml、pvc.yaml、service.yaml、redis.yaml、main.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、namespace.yaml、pvc.yaml、service.yaml、redis.yaml、main.go

### 基础设施与部署/监控与可观测性.md

- reference 标题：监控与可观测性
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：94
- 页面类型：workflow / workflow
- 行数：369 / 106
- 段落行数：132 / 11
- Evidence：0 / 1
- Mermaid：10 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：grafana-datasources.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml、zipkin.yaml、deployment.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：grafana-datasources.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml、zipkin.yaml、deployment.yaml

### 安全与认证/API安全策略.md

- reference 标题：API安全策略
- 生成页：核心模块/src/backend/services/web.admin.md（模块：web.admin）
- 匹配分数：62
- 页面类型：other / module
- 行数：311 / 69
- 段落行数：107 / 14
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：nginx.conf、next.js
- reference 关键文件未覆盖：package-lock.js、payments.pb.go、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：package-lock.js、payments.pb.go、middleware.ts

### 安全与认证/会话管理.md

- reference 标题：会话管理
- 生成页：无
- 问题：缺少对应生成页面

### 安全与认证/安全与认证.md

- reference 标题：安全与认证
- 生成页：无
- 问题：缺少对应生成页面

### 安全与认证/数据保护与安全.md

- reference 标题：数据保护与安全
- 生成页：无
- 问题：缺少对应生成页面

### 安全与认证/权限控制系统.md

- reference 标题：权限控制系统
- 生成页：无
- 问题：缺少对应生成页面

### 安全与认证/身份认证系统.md

- reference 标题：身份认证系统
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/CI_CD流水线/Azure Pipelines配置.md

- reference 标题：Azure Pipelines配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/CI_CD流水线/CI_CD流水线.md

- reference 标题：CI/CD流水线
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/CI_CD流水线/GitHub Actions工作流.md

- reference 标题：GitHub Actions工作流
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：92
- 页面类型：workflow / workflow
- 行数：307 / 106
- 段落行数：112 / 11
- Evidence：0 / 1
- Mermaid：5 / 1
- 文件提及重合：无
- reference 关键文件未覆盖：action.yml、build-release.yml、checkout-api.yml、codeql-analysis.yml、deploy.yml、order-api.yml、payment-api.yml、web-app.yml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：action.yml、build-release.yml、checkout-api.yml、codeql-analysis.yml、deploy.yml、order-api.yml、payment-api.yml、web-app.yml

### 开发者指南/CI_CD流水线/代码质量检查.md

- reference 标题：代码质量检查
- 生成页：核心模块/src/backend/services/web.admin/dashboard-app.md（模块：dashboard-app）
- 匹配分数：72
- 页面类型：other / module
- 行数：414 / 51
- 段落行数：148 / 16
- Evidence：25 / 2
- Mermaid：10 / 1
- 文件提及重合：.eslintrc.js、next.js
- reference 关键文件未覆盖：azure-pipelines.yml、cargo.toml、package.js、prettier.config.js、tailwind.config.ts、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、cargo.toml、package.js、prettier.config.js、tailwind.config.ts、tsconfig.js

### 开发者指南/CI_CD流水线/容器化CI_CD.md

- reference 标题：容器化CI/CD
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/CI_CD流水线/移动应用CI_CD.md

- reference 标题：移动应用CI/CD
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/代码贡献流程.md

- reference 标题：代码贡献流程
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发最佳实践/代码质量保证.md

- reference 标题：代码质量保证
- 生成页：专题/module-capability/module-b956eb7f2423-service-dashboard-app能力：服务协作.md（dashboard-app能力：服务协作）
- 匹配分数：82
- 页面类型：other / topic
- 行数：384 / 45
- 段落行数：122 / 20
- Evidence：20 / 2
- Mermaid：8 / 2
- 文件提及重合：.eslintrc.js、next.js
- reference 关键文件未覆盖：.gitpod.yml、.rustfmt.toml、azure-pipelines.yml、cargo.toml、package.js、tslint.js、prettier.config.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.gitpod.yml、.rustfmt.toml、azure-pipelines.yml、cargo.toml、package.js、tslint.js、prettier.config.js

### 开发者指南/开发最佳实践/可观测性与监控.md

- reference 标题：可观测性与监控
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发最佳实践/多语言微服务开发.md

- reference 标题：多语言微服务开发
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：256
- 页面类型：other / module
- 行数：527 / 83
- 段落行数：160 / 24
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：main.go、catalog.rs、category.rs、main.rs、seed.rs、index.ts、instrumentation.ts、routes.ts、page.ts
- reference 关键文件未覆盖：readme.md、gateway.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、minio.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、init-topics-job.yaml、kafka-stateful-set.yaml、minio.yaml、grafana.yaml、jaeger.yaml、otel-collector.yaml

### 开发者指南/开发最佳实践/容器化与DevOps.md

- reference 标题：容器化与DevOps
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：66
- 页面类型：other / workflow
- 行数：365 / 106
- 段落行数：158 / 11
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：docker-compose.yml
- reference 关键文件未覆盖：readme.md、azure-pipelines.yml、kafka-stateful-set.yaml、grafana.yaml、prometheus.yaml、deployment.yaml、http-route.yaml、service.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、azure-pipelines.yml、kafka-stateful-set.yaml、grafana.yaml、prometheus.yaml、deployment.yaml、http-route.yaml、service.yaml

### 开发者指南/开发最佳实践/开发工具与调试.md

- reference 标题：开发工具与调试
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：64
- 页面类型：other / module
- 行数：401 / 83
- 段落行数：177 / 24
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：main.go、main.rs、index.ts、instrumentation.ts
- reference 关键文件未覆盖：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、docker-compose.override.yml、docker-compose.yml、instrumentation.go、cargo.toml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.postman_collection.js、catalog-api.postman_collection.js、checkout-api.postman_collection.js、local-docker.postman_environment.js、docker-compose.override.yml、docker-compose.yml、instrumentation.go、cargo.toml

### 开发者指南/开发最佳实践/开发最佳实践.md

- reference 标题：开发最佳实践
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发最佳实践/微服务设计原则.md

- reference 标题：微服务设计原则
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发最佳实践/测试策略与实践.md

- reference 标题：测试策略与实践
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发环境配置/Docker容器化配置/Docker Compose配置详解.md

- reference 标题：Docker Compose配置详解
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：84
- 页面类型：other / workflow
- 行数：469 / 106
- 段落行数：255 / 11
- Evidence：22 / 1
- Mermaid：8 / 1
- 文件提及重合：docker-compose.grafana.yaml、docker-compose.kafka.yml、docker-compose.load-tests.yaml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.traefik.yml、docker-compose.yml
- reference 关键文件未覆盖：prometheus.yaml、tempo.yaml、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：prometheus.yaml、tempo.yaml、next.js

### 开发者指南/开发环境配置/Docker容器化配置/Docker容器化配置.md

- reference 标题：Docker容器化配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：88
- 页面类型：other / workflow
- 行数：540 / 106
- 段落行数：415 / 11
- Evidence：24 / 1
- Mermaid：14 / 1
- 文件提及重合：docker-compose.grafana.yaml、docker-compose.kafka.yml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.traefik.yml、docker-compose.yml
- reference 关键文件未覆盖：docker-compose.load-tests.yml、next.js、node.js、docker-compose.grafana.yml、realm.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：docker-compose.load-tests.yml、next.js、node.js、docker-compose.grafana.yml、realm.js

### 开发者指南/开发环境配置/Docker容器化配置/容器网络配置.md

- reference 标题：容器网络配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：76
- 页面类型：other / workflow
- 行数：372 / 106
- 段落行数：186 / 11
- Evidence：0 / 1
- Mermaid：8 / 1
- 文件提及重合：docker-compose.grafana.yaml、docker-compose.kafka.yml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.traefik.yml、docker-compose.yml
- reference 关键文件未覆盖：gateway.yaml、istio-system.yaml、kafka-stateful-set.yaml、service.yaml、deployment.yaml、http-route.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、istio-system.yaml、kafka-stateful-set.yaml、service.yaml、deployment.yaml、http-route.yaml

### 开发者指南/开发环境配置/Docker容器化配置/微服务Dockerfile配置.md

- reference 标题：微服务Dockerfile配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发环境配置/Docker容器化配置/数据持久化与卷管理.md

- reference 标题：数据持久化与卷管理
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发环境配置/IDE开发工具配置/IDE开发工具配置.md

- reference 标题：IDE开发工具配置
- 生成页：核心模块/src/backend/services/web.admin/dashboard-app.md（模块：dashboard-app）
- 匹配分数：80
- 页面类型：other / module
- 行数：426 / 51
- 段落行数：182 / 16
- Evidence：21 / 2
- Mermaid：9 / 1
- 文件提及重合：.eslintrc.js、next.js
- reference 关键文件未覆盖：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、tslint.js、prettier.config.js、tailwind.config.ts、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、tslint.js、prettier.config.js、tailwind.config.ts、node.js

### 开发者指南/开发环境配置/IDE开发工具配置/IntelliJ IDEA配置.md

- reference 标题：IntelliJ IDEA配置
- 生成页：专题/module-capability/module-b956eb7f2423-service-dashboard-app能力：服务协作.md（dashboard-app能力：服务协作）
- 匹配分数：78
- 页面类型：other / topic
- 行数：318 / 45
- 段落行数：110 / 20
- Evidence：16 / 2
- Mermaid：7 / 2
- 文件提及重合：.eslintrc.js、next.js
- reference 关键文件未覆盖：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、prettier.config.js、node.js、package-lock.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：.rustfmt.toml、cargo.toml、package.js、tsconfig.js、prettier.config.js、node.js、package-lock.js

### 开发者指南/开发环境配置/IDE开发工具配置/VS Code工作区配置.md

- reference 标题：VS Code工作区配置
- 生成页：核心模块/src/backend/services/web.admin.md（模块：web.admin）
- 匹配分数：62
- 页面类型：other / module
- 行数：414 / 69
- 段落行数：254 / 14
- Evidence：16 / 1
- Mermaid：8 / 1
- 文件提及重合：next.js
- reference 关键文件未覆盖：cargo.toml、package.js、tsconfig.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、package.js、tsconfig.js

### 开发者指南/开发环境配置/IDE开发工具配置/Visual Studio配置.md

- reference 标题：Visual Studio配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发环境配置/IDE开发工具配置/语言特定开发环境.md

- reference 标题：语言特定开发环境
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：106
- 页面类型：other / module
- 行数：357 / 83
- 段落行数：113 / 24
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：main.go、main.rs、index.ts
- reference 关键文件未覆盖：cargo.toml、package.js、tsconfig.js、next.js、node.js、index.js、pnpm-lock.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：cargo.toml、package.js、tsconfig.js、next.js、node.js、index.js、pnpm-lock.yaml

### 开发者指南/开发环境配置/Vagrant虚拟机配置.md

- reference 标题：Vagrant虚拟机配置
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发环境配置/开发环境配置.md

- reference 标题：开发环境配置
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：140
- 页面类型：other / workflow
- 行数：440 / 106
- 段落行数：146 / 11
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：docker-compose.kafka.yml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.yml
- reference 关键文件未覆盖：contributing.md、readme.md、local-docker.postman_environment.js、node.js、next.js、index.js、server.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、local-docker.postman_environment.js、node.js、next.js、index.js、server.js

### 开发者指南/开发环境配置/本地环境搭建.md

- reference 标题：本地环境搭建
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/开发者指南.md

- reference 标题：开发者指南
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除/开发环境调试.md

- reference 标题：开发环境调试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除/性能问题诊断.md

- reference 标题：性能问题诊断
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除/服务间通信调试.md

- reference 标题：服务间通信调试
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除/生产环境故障排除.md

- reference 标题：生产环境故障排除
- 生成页：无
- 问题：缺少对应生成页面

### 开发者指南/调试与故障排除/调试与故障排除.md

- reference 标题：调试与故障排除
- 生成页：无
- 问题：缺少对应生成页面

### 微服务架构设计/微服务架构设计.md

- reference 标题：微服务架构设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 页面类型：architecture / architecture
- 行数：276 / 74
- 段落行数：101 / 16
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、gateway.yaml、deployment.yaml、cargo.toml、package.js、next.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、gateway.yaml、deployment.yaml、cargo.toml、package.js、next.js、node.js

### 微服务架构设计/数据流设计.md

- reference 标题：数据流设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：118
- 页面类型：architecture / architecture
- 行数：451 / 74
- 段落行数：180 / 16
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：main.go、main.rs
- reference 关键文件未覆盖：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、kustomization.yaml、namespace.yaml、pvc.yaml、service.yaml、deployment.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：init-topics-job.yaml、kafka-stateful-set.yaml、kafka-ui.yaml、kustomization.yaml、namespace.yaml、pvc.yaml、service.yaml、deployment.yaml

### 微服务架构设计/整体架构概览.md

- reference 标题：整体架构概览
- 生成页：系统架构.md（系统架构）
- 匹配分数：92
- 页面类型：architecture / architecture
- 行数：214 / 74
- 段落行数：76 / 16
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、deployment.yaml、node.js、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、node.js、next.js

### 微服务架构设计/服务发现与治理.md

- reference 标题：服务发现与治理
- 生成页：系统架构.md（系统架构）
- 匹配分数：116
- 页面类型：architecture / architecture
- 行数：389 / 74
- 段落行数：150 / 16
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：http-route.yaml
- reference 关键文件未覆盖：gateway.yaml、istio-system.yaml、kustomization.yaml、telemetry.yaml、grafana.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、istio-system.yaml、kustomization.yaml、telemetry.yaml、grafana.yaml、otel-collector.yaml、prometheus.yaml、tempo.yaml

### 微服务架构设计/服务间通信模式.md

- reference 标题：服务间通信模式
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：381 / 74
- 段落行数：168 / 16
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：gateway.yaml、deployment.yaml、service.yaml、cart_handler.go、message_reciever.go、index.ts、publisher.ts、usercheckouteventhandler.java
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：gateway.yaml、deployment.yaml、service.yaml、cart_handler.go、message_reciever.go、index.ts、publisher.ts、usercheckouteventhandler.java

### 微服务架构设计/错误处理策略.md

- reference 标题：错误处理策略
- 生成页：系统架构.md（系统架构）
- 匹配分数：128
- 页面类型：architecture / architecture
- 行数：315 / 74
- 段落行数：163 / 16
- Evidence：0 / 1
- Mermaid：7 / 2
- 文件提及重合：main.go
- reference 关键文件未覆盖：otel-collector.yaml、cart-api.postman_collection.js、otel-connector-config.yaml、tempo.yaml、cart_handler.go、http_error.go、instrumentation.ts、middleware.ts
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：otel-collector.yaml、cart-api.postman_collection.js、otel-connector-config.yaml、tempo.yaml、cart_handler.go、http_error.go、instrumentation.ts、middleware.ts

### 快速开始.md

- reference 标题：快速开始
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/前端性能优化.md

- reference 标题：前端性能优化
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/微服务性能优化.md

- reference 标题：微服务性能优化
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/性能优化.md

- reference 标题：性能优化
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/监控与指标收集.md

- reference 标题：监控与指标收集
- 生成页：无
- 问题：缺少对应生成页面

### 性能优化/负载测试与压力测试.md

- reference 标题：负载测试与压力测试
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：74
- 页面类型：other / workflow
- 行数：300 / 106
- 段落行数：126 / 11
- Evidence：0 / 1
- Mermaid：7 / 1
- 文件提及重合：docker-compose.load-tests.yaml、docker-compose.otel.yml
- reference 关键文件未覆盖：kustomization.yaml、deployment.yaml、performance-test-dasboard.js、datasource.yaml、prometheus.yaml、checkout.js、otel.yml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：kustomization.yaml、deployment.yaml、performance-test-dasboard.js、datasource.yaml、prometheus.yaml、checkout.js、otel.yml

### 故障排除与维护/常见问题解决.md

- reference 标题：常见问题解决
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：222
- 页面类型：other / workflow
- 行数：462 / 106
- 段落行数：96 / 11
- Evidence：0 / 1
- Mermaid：3 / 1
- 文件提及重合：docker-compose.grafana.yaml、docker-compose.kafka.yml、docker-compose.load-tests.yaml、docker-compose.otel.yml、docker-compose.override.yml、docker-compose.traefik.yml、docker-compose.yml
- reference 关键文件未覆盖：readme.md、deployment.yaml、local-docker.postman_environment.js、nginx.conf、dashboard.yaml、influxdb-dashboard.js、performance-test-dasboard.js、datasource.yaml
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、local-docker.postman_environment.js、nginx.conf、dashboard.yaml、influxdb-dashboard.js、performance-test-dasboard.js、datasource.yaml

### 故障排除与维护/故障排除与维护.md

- reference 标题：故障排除与维护
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：74
- 页面类型：other / workflow
- 行数：373 / 106
- 段落行数：119 / 11
- Evidence：0 / 1
- Mermaid：4 / 1
- 文件提及重合：docker-compose.otel.yml、docker-compose.yml
- reference 关键文件未覆盖：readme.md、kustomization.yaml、prometheus.yaml、deployment.yaml、datasource.yaml、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、kustomization.yaml、prometheus.yaml、deployment.yaml、datasource.yaml、node.js

### 故障排除与维护/日志分析与监控.md

- reference 标题：日志分析与监控
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除与维护/系统维护.md

- reference 标题：系统维护
- 生成页：无
- 问题：缺少对应生成页面

### 故障排除与维护/运维工具与实践.md

- reference 标题：运维工具与实践
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/数据库架构.md

- reference 标题：数据库架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：90
- 页面类型：architecture / architecture
- 行数：328 / 74
- 段落行数：122 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：menu-pgsql.yaml、identity-pgsql.yaml、order-pgsql.yaml、docker-compose.yml、diesel.toml、schema.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：menu-pgsql.yaml、identity-pgsql.yaml、order-pgsql.yaml、docker-compose.yml、diesel.toml、schema.rs

### 数据库设计/数据库设计.md

- reference 标题：数据库设计
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/数据库迁移.md

- reference 标题：数据库迁移
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/数据模型.md

- reference 标题：数据模型
- 生成页：无
- 问题：缺少对应生成页面

### 数据库设计/数据种子管理.md

- reference 标题：数据种子管理
- 生成页：核心模块/src/backend/services.md（模块：services）
- 匹配分数：136
- 页面类型：other / module
- 行数：256 / 83
- 段落行数：100 / 24
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：main.rs、catalog.rs、category.rs、seed.rs
- reference 关键文件未覆盖：down.sql、up.sql、schema.rs、mod.rs
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：down.sql、up.sql、schema.rs、mod.rs

### 测试策略/单元测试.md

- reference 标题：单元测试
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/测试策略.md

- reference 标题：测试策略
- 生成页：工作流与部署.md（工作流与部署）
- 匹配分数：66
- 页面类型：other / workflow
- 行数：310 / 106
- 段落行数：121 / 11
- Evidence：0 / 1
- Mermaid：6 / 1
- 文件提及重合：docker-compose.load-tests.yaml
- reference 关键文件未覆盖：azure-pipelines.yml、local-docker.postman_environment.js、checkout.js、cart_handler_test.go、cargo.toml、package.js、payment-methods_test.go
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：azure-pipelines.yml、local-docker.postman_environment.js、checkout.js、cart_handler_test.go、cargo.toml、package.js、payment-methods_test.go

### 测试策略/测试自动化.md

- reference 标题：测试自动化
- 生成页：无
- 问题：缺少对应生成页面

### 测试策略/集成测试.md

- reference 标题：集成测试
- 生成页：无
- 问题：缺少对应生成页面

### 移动应用架构/UI组件与页面.md

- reference 标题：UI组件与页面
- 生成页：系统架构.md（系统架构）
- 匹配分数：88
- 页面类型：architecture / architecture
- 行数：326 / 74
- 段落行数：132 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 移动应用架构/平台特定实现.md

- reference 标题：平台特定实现
- 生成页：系统架构.md（系统架构）
- 匹配分数：88
- 页面类型：architecture / architecture
- 行数：334 / 74
- 段落行数：123 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 移动应用架构/抽象层设计.md

- reference 标题：抽象层设计
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：422 / 74
- 段落行数：202 / 16
- Evidence：0 / 1
- Mermaid：8 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 移动应用架构/核心逻辑层.md

- reference 标题：核心逻辑层
- 生成页：系统架构.md（系统架构）
- 匹配分数：114
- 页面类型：architecture / architecture
- 行数：453 / 74
- 段落行数：235 / 16
- Evidence：22 / 1
- Mermaid：10 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块

### 移动应用架构/移动应用架构.md

- reference 标题：移动应用架构
- 生成页：系统架构.md（系统架构）
- 匹配分数：108
- 页面类型：architecture / architecture
- 行数：287 / 74
- 段落行数：158 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 移动应用架构/移动应用架构概览.md

- reference 标题：移动应用架构概览
- 生成页：系统架构.md（系统架构）
- 匹配分数：90
- 页面类型：architecture / architecture
- 行数：268 / 74
- 段落行数：112 / 16
- Evidence：0 / 1
- Mermaid：6 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块

### 移动应用架构/移动应用测试.md

- reference 标题：移动应用测试
- 生成页：系统架构.md（系统架构）
- 匹配分数：100
- 页面类型：architecture / architecture
- 行数：319 / 74
- 段落行数：125 / 16
- Evidence：22 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块

### 项目概述/快速开始指南.md

- reference 标题：快速开始指南
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：262 / 93
- 段落行数：102 / 21
- Evidence：0 / 1
- Mermaid：5 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、deployment.yaml、local-docker.postman_environment.js、docker-compose.yml、next.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、deployment.yaml、local-docker.postman_environment.js、docker-compose.yml、next.js、node.js

### 项目概述/技术栈概览.md

- reference 标题：技术栈概览
- 生成页：项目概述.md（项目概述）
- 匹配分数：88
- 页面类型：overview / overview
- 行数：312 / 93
- 段落行数：108 / 21
- Evidence：0 / 1
- Mermaid：3 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、kustomization.yaml、deployment.yaml、cargo.toml、package.js、apinode.js、next.js、node.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、kustomization.yaml、deployment.yaml、cargo.toml、package.js、apinode.js、next.js、node.js

### 项目概述/架构设计理念.md

- reference 标题：架构设计理念
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：395 / 93
- 段落行数：183 / 21
- Evidence：0 / 1
- Mermaid：9 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：readme.md、deployment.yaml、service.yaml、cargo.toml、package.js、node.js、next.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；缺少引用/出处块；缺少关键文件提及：readme.md、deployment.yaml、service.yaml、cargo.toml、package.js、node.js、next.js

### 项目概述/项目介绍与目标.md

- reference 标题：项目介绍与目标
- 生成页：项目概述.md（项目概述）
- 匹配分数：94
- 页面类型：overview / overview
- 行数：315 / 93
- 段落行数：185 / 21
- Evidence：19 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：cart-api.yml、contributing.md、readme.md、azure-pipelines.yml、deployment.yaml、docker-compose.yml、cargo.toml、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：cart-api.yml、contributing.md、readme.md、azure-pipelines.yml、deployment.yaml、docker-compose.yml、cargo.toml、package.js

### 项目概述/项目概述.md

- reference 标题：项目概述
- 生成页：项目概述.md（项目概述）
- 匹配分数：330
- 页面类型：overview / overview
- 行数：334 / 93
- 段落行数：179 / 21
- Evidence：21 / 1
- Mermaid：7 / 2
- 文件提及重合：无
- reference 关键文件未覆盖：contributing.md、readme.md、kafka-stateful-set.yaml、grafana.yaml、deployment.yaml、docker-compose.yml、cargo.toml、package.js
- 结论：内容明显短于 reference；解释性段落明显不足；章节拆分比 reference 粗；图表少于 reference；evidence block 少于 reference；缺少引用/出处块；缺少关键文件提及：contributing.md、readme.md、kafka-stateful-set.yaml、grafana.yaml、deployment.yaml、docker-compose.yml、cargo.toml、package.js

