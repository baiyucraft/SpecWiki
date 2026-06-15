# 迭代 3 测试项目集分析

日期：2026-03-09（按更新后的 `.wiki/06-设计文档/00-总体设计.md` 重新执行）

## 范围

- 当前仓库：`E:\project\!byAI\spec-wiki`
- 真实参考仓库：`E:\project\aLocal`
- 其余测试项目集：当前 `.wiki/06-设计文档/00-总体设计.md` 中除 `test-aLocal`、`test-spec-wiki` 外的 17 个样本仓库
- reference 对照：
  - `tmp/reference-aLocal`
  - `tmp/reference-pinia`
  - `tmp/reference-restaurant-app`

备注：

- 当前 `.wiki/06-设计文档/00-总体设计.md` 的测试项目集实际列出 19 个项目；旧分析中的 `test-turbo-basic` 已不在当前名单里。
- 本轮新增关注项是 `test-wot-starter`、`test-storybook`，以及 `reference-restaurant-app` 对照。

## 自动化测试入口

执行：

- `pnpm test`

结果：

- 通过。
- Rust `wiki-core` 测试、`agents/codebuddy` 测试、根级 distribution / e2e 测试均通过。

## 当前仓库 `spec-wiki`

执行：

- `init` 成功，结果 `fresh`
- 后续 `status` 返回 `fresh`
- 后续 `update` 返回空更新，`updated_pages = []`

运行时统计：

- 页面：36
- 模块：35
- 源码：212
- 关系：44

重点观察：

- `crates/wiki-core/tests/fixtures/**` 仍被整体提升为正式模块页，存在 fixture 误提升。
- `eslint.config.mjs`、`vitest.config.mjs` 这类单文件配置仍被单独提升为模块页。
- 增量 runtime 边界保持稳定：`init -> fresh`，紧接着 `update` 不会误触发写盘。

结论：

- 当前仓库没有出现因测试项目集更新带来的 runtime 回归。
- 现有差异仍集中在 planner / ranking 质量，不是迭代 3 的状态内核或增量 workflow 错误。

## `aLocal` 真仓库对照

执行：

- `E:\project\aLocal` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：6
- 模块：5
- 源码：164
- 关系：7

生成页面：

- `系统架构.md`
- `项目概述.md`
- `核心模块/nginx.md`
- `核心模块/nginx/conf.md`
- `核心模块/spider.md`
- `核心模块/web.md`

与 `reference-aLocal` 对照：

- 当前 core 产物仍是 repo/module 结构导向的 6 页运行时，重点表达模块边界、入口文件、关键源码和依赖。
- `reference-aLocal` 仍是解释型产品文档形态，目录按“核心功能模块 / 开发指南 / API / 运维 / 数据库设计”等主题域组织，页面数量远多于当前 core。
- metadata 结构仍不属于同一产品协议：当前输出使用 `schema_version / wiki_items / source_files / dirty_state`；reference 是另一套更重的历史结构。
- 内容信号层面，当前 `web` 页仍能抓到 `web/src/main.ts`、`App.vue`、`api/*.ts`、`views/*.vue` 等高信号源码，但深度明显弱于 reference 的专题化拆页。

结论：

- 没有出现 init 失败、metadata 缺失或 runtime 不 fresh 的问题。
- 差异仍是页面规划与内容编排深度差距，不是本 change 的新增回归。

## `pinia` reference 对照

执行：

- `tmp/test-pinia` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：18
- 模块：17
- 源码：390
- 关系：29

与 `reference-pinia` 对照：

- 当前输出仍以 repo/module 结构为主，仍会出现 `.prettierrc.js`、`vitest.config.ts` 这类单文件配置模块页。
- `packages.md` 会聚合 workspace 子模块，并向下拆出 `packages/pinia`、`packages/nuxt`、`packages/online-playground` 等页面。
- `reference-pinia` 仍按“核心概念 / API 参考 / 最佳实践 / 高级功能 / 示例教程”等知识域组织，明显更接近消费层文档产品形态。

结论：

- 说明当前 planner 在 monorepo 上仍能稳定递归拆页，但尚未接近 reference 的主题化知识组织。

## `restaurant-app` reference 对照

执行：

- `tmp/test-restaurant-app` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：13
- 模块：12
- 源码：1106
- 关系：17

当前生成页面：

- `系统架构.md`
- `项目概述.md`
- `核心模块/src.md`
- `核心模块/src/backend.md`
- `核心模块/src/backend/archive.md`
- `核心模块/src/backend/services.md`
- `核心模块/src/backend/archive/nginx.md`
- `核心模块/src/backend/services/catalog-api.md`
- `核心模块/src/backend/services/checkout-api.md`
- `核心模块/src/backend/services/web-app.md`
- `核心模块/src/backend/services/web.admin.md`
- `核心模块/src/backend/services/web.admin/dashboard-app.md`
- `核心模块/src/backend/services/web.admin/dashboard.md`

与 `reference-restaurant-app` 对照：

- 当前 core 仍以目录树和模块树为主，主要输出 backend / service / web admin 等实现模块页。
- `reference-restaurant-app` 按“API接口文档 / 前端应用架构 / 后端服务详解 / 基础设施与部署 / 安全与认证 / 数据库设计 / 测试策略 / 移动应用架构”等主题域大规模拆页。
- 当前结构已能识别部分多服务层级，但在多语言微服务、前端/后端/移动端/基础设施跨域编排上，与 reference 的消费层知识组织差距很大。

结论：

- 多语言混合大型仓库能稳定 init，说明增量 runtime 布局在复杂仓库上可落盘。
- 差异仍在 planner / composition 深度，不构成迭代 3 的运行时回归。

## 其余 17 个样本仓库

所有仓库均成功执行 `init` 并返回 `fresh`，未出现 crash、metadata 写盘失败或 cache 布局缺失。

| 仓库 | 页面 | 模块 | 源码 | 关系 |
|------|------|------|------|------|
| test-axum | 63 | 62 | 488 | 132 |
| test-bat | 17 | 16 | 888 | 19 |
| test-chi | 3 | 1 | 95 | 2 |
| test-cobra | 3 | 1 | 66 | 2 |
| test-dagger | 29 | 28 | 3431 | 126 |
| test-django-ninja | 5 | 4 | 306 | 4 |
| test-docker-mailserver | 5 | 4 | 285 | 4 |
| test-fastapi | 8 | 7 | 2915 | 7 |
| test-gin | 3 | 2 | 131 | 2 |
| test-httpx | 6 | 5 | 125 | 5 |
| test-leakcanary | 10 | 9 | 758 | 16 |
| test-pinia | 18 | 17 | 390 | 29 |
| test-restaurant-app | 13 | 12 | 1106 | 17 |
| test-spring-petclinic | 3 | 2 | 127 | 2 |
| test-storybook | 78 | 77 | 5412 | 426 |
| test-wot-starter | 14 | 13 | 291 | 13 |
| test-zustand | 10 | 9 | 143 | 9 |

总体观察：

- 新增的 `test-wot-starter` 可以稳定落成 14 页，说明当前 scanner / hierarchy 对 Vue3 + TS + UniApp 模板类仓库能正常建立模块页。
- 新增的 `test-storybook` 可以稳定落成 78 页、426 条关系，没有出现大仓库 init 崩溃或 cache 布局异常，说明当前 runtime 在超大 monorepo 上仍然可运行。
- 扁平 Go / Java 小仓库仍容易收敛到 3-5 页，说明当前模块识别依然偏粗。
- workspace / monorepo / 多子系统仓库会稳定展开更多模块页，`axum`、`dagger`、`pinia`、`storybook`、`restaurant-app` 表现正常。

## 是否需要回调本 change

当前不需要回调本 change 的 spec / design / tasks。

原因：

- 本轮新增项目集样本没有暴露出增量 runtime 的崩溃、cache 缺失误判、`status/update/rebuild` 语义漂移等问题。
- 新增差异仍主要集中在内容选择、主题化组织、fixture 抑制、单文件配置页抑制和 kind 分类质量。
- 这些问题仍属于 planner / composition 质量议题，不是迭代 3 引入的 ChangePlan、WikiState、page cache 或 section runtime 错误。
