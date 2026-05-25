# 迭代 4 测试项目集分析

日期：2026-03-09

## 范围

- 当前仓库：`E:\project\!byAI\spec-wiki`（通过 `tmp/test-spec-wiki` 验证）
- 真实参考仓库：`E:\project\aLocal`（通过 `tmp/test-aLocal` 验证）
- 其余测试项目集：17 个样本仓库
- reference 对照：
  - `tmp/reference-aLocal`
  - `tmp/reference-pinia`
  - `tmp/reference-restaurant-app`

## 自动化测试入口

执行：

- `pnpm test`
- `cargo test --package wiki-core`

结果：

- 全部通过。52 个 Rust 测试（含本轮新增 14 个）、3 个 e2e/distribution 测试均通过。
- 本轮新增测试文件：`scanner_noise_filter.rs`（5 个）、`hierarchy_noise_filter.rs`（3 个）、`module_kind_classification.rs`（4 个）、`key_source_selection.rs`（2 个）。

## 当前仓库 `spec-wiki`

执行：

- `init` 成功，结果 `fresh`

运行时统计：

- 页面：7
- 模块：6
- 源码：119
- 关系：7

生成页面：

- `项目概述.md`
- `系统架构.md`
- `核心模块/agents.md`
- `核心模块/agents/codebuddy.md`
- `核心模块/crates.md`
- `核心模块/crates/wiki-core.md`
- `核心模块/scripts.md`

模块分类：

- `spec-wiki`: repository
- `agents`: module-group
- `codebuddy`: cli-tool
- `crates`: module-group
- `wiki-core`: cli-tool
- `scripts`: frontend-app

与迭代 3 对比（迭代 3：36 页 / 35 模块 / 212 源码 / 44 关系）：

- 页面从 36 降至 7，模块从 35 降至 6。大幅收敛的原因：
  - `crates/wiki-core/tests/fixtures/**` 不再被提升为模块（scanner 排除 fixture 目录）。
  - `eslint.config.mjs`、`vitest.config.mjs` 等单文件配置不再生成独立模块页（单文件模块抑制）。
  - `.spec` 不再被提升为模块（非代码产物目录排除）。
  - `.github` 不再被提升为模块。
- `wiki-core` kind 从 `infrastructure` 修正为 `cli-tool`（Cargo.toml 含 `[[bin]]`）。
- `codebuddy` kind 从 `frontend-app` 修正为 `cli-tool`（agents 路径覆盖）。
- 关键源码列表全部来自 `src/` 下核心实现文件，无 fixture 或 test 文件污染。

结论：

- 迭代 4 的 scanner 噪声过滤、单文件模块抑制、module kind 分类升级和关键源码信号修正全部生效。
- 页面结构显著收敛，噪声问题基本消除。

## `aLocal` 真仓库对照

执行：

- `E:\project\aLocal` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：6
- 模块：5
- 源码：112
- 关系：7

生成页面：

- `项目概述.md`
- `系统架构.md`
- `核心模块/nginx.md`
- `核心模块/nginx/conf.md`
- `核心模块/spider.md`
- `核心模块/web.md`

模块分类：

- `aLocal`: repository
- `nginx`: module-group
- `conf`: infrastructure
- `spider`: backend-service
- `web`: frontend-app

与迭代 3 对比（迭代 3：6 页 / 5 模块 / 164 源码 / 7 关系）：

- 页面和模块数量不变，结构稳定。
- 源码从 164 降至 112，原因是 `.github` 等非代码产物目录被排除。
- `conf` 正确分类为 `infrastructure`（只含 nginx.conf）。

与 `reference-aLocal` 对照：

- 当前 core 产物仍是 repo/module 结构导向的 6 页运行时。
- reference 仍是解释型产品文档形态，差异属于 planner / composition 深度，不是本迭代范围。

结论：

- 无退化，结构稳定。

## `pinia` reference 对照

执行：

- `tmp/test-pinia` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：15
- 模块：14
- 源码：376
- 关系：26

与迭代 3 对比（迭代 3：18 页 / 17 模块 / 390 源码 / 29 关系）：

- 页面从 18 降至 15，模块从 17 降至 14。减少的 3 个模块是被单文件抑制或噪声过滤排除的配置文件模块。
- workspace 子模块（`packages/pinia`、`packages/nuxt`、`packages/testing` 等）仍正确展开。

结论：

- monorepo 递归拆页稳定，噪声收敛。

## `restaurant-app` reference 对照

执行：

- `tmp/test-restaurant-app` 上 `init` 成功，结果 `fresh`

运行时统计：

- 页面：13
- 模块：12
- 源码：1095
- 关系：17

与迭代 3 对比（迭代 3：13 页 / 12 模块 / 1106 源码 / 17 关系）：

- 页面和模块数量不变。源码从 1106 微降至 1095（非代码产物排除）。
- `backend` 正确分类为 `infrastructure`（含 Dockerfile 等基础设施文件）。

结论：

- 多语言混合大型仓库结构稳定，无退化。

## 其余 17 个样本仓库

所有仓库均成功执行 `init` 并返回 `fresh`，未出现 crash、metadata 写盘失败或 cache 布局缺失。

| 仓库 | 页面 | 模块 | 源码 | 关系 | 迭代 3 页面 | 变化 |
|------|------|------|------|------|-------------|------|
| test-aLocal | 6 | 5 | 112 | 7 | 6 | 不变 |
| test-axum | 6 | 5 | 440 | 12 | 63 | -57 |
| test-bat | 15 | 14 | 904 | 17 | 17 | -2 |
| test-chi | 3 | 1 | 157 | 2 | 3 | 不变 |
| test-cobra | 3 | 1 | 101 | 2 | 3 | 不变 |
| test-dagger | 27 | 26 | 3449 | 119 | 29 | -2 |
| test-django-ninja | 5 | 4 | 296 | 4 | 5 | 不变 |
| test-docker-mailserver | 3 | 2 | 252 | 2 | 5 | -2 |
| test-fastapi | 7 | 6 | 2889 | 6 | 8 | -1 |
| test-gin | 3 | 1 | 114 | 2 | 3 | 不变 |
| test-httpx | 4 | 3 | 115 | 3 | 6 | -2 |
| test-leakcanary | 9 | 8 | 747 | 15 | 10 | -1 |
| test-pinia | 15 | 14 | 376 | 26 | 18 | -3 |
| test-restaurant-app | 13 | 12 | 1095 | 17 | 13 | 不变 |
| test-spec-wiki | 7 | 6 | 119 | 7 | 36 | -29 |
| test-spring-petclinic | 3 | 2 | 123 | 2 | 3 | 不变 |
| test-storybook | 67 | 66 | 5471 | 385 | 78 | -11 |
| test-wot-starter | 7 | 6 | 283 | 6 | 14 | -7 |
| test-zustand | 3 | 2 | 186 | 2 | 10 | -7 |

总体观察：

- 所有项目页面数量持平或下降，无膨胀。下降原因均为噪声过滤和单文件模块抑制生效。
- `test-axum` 从 63 页降至 6 页，降幅最大。原因是 axum 的 `examples/` 目录被排除（非代码产物目录），大量 example 子目录不再被提升为模块。axum 的 4 个核心 crate（`axum`、`axum-core`、`axum-extra`、`axum-macros`）仍正确保留。`axum-macros` 正确分类为 `library`（Cargo.toml 含 `[lib]`）。
- `test-spec-wiki` 从 36 页降至 7 页，fixture 误提升和单文件配置模块完全消除。
- `test-wot-starter` 从 14 页降至 7 页，`eslint.config.mjs`、`vite.config.ts` 等单文件配置不再生成独立模块页。
- `test-zustand` 从 10 页降至 3 页，单文件配置模块被抑制。
- `test-storybook` 从 78 页降至 67 页，`.github` 和部分噪声目录被排除。storybook 的 `code/core` 正确分类为 `cli-tool`（package.json 含 `"bin"`），`code/lib/cli-sb` 和 `code/lib/cli-storybook` 也正确分类为 `cli-tool`。
- Go 扁平仓库（cobra、chi、gin）保持 3 页不变，说明噪声过滤不影响简单仓库。
- `test-leakcanary` 的模块 kind 从混合的 `frontend-app` 修正为统一的 `module`（Android 项目不应被标为 frontend-app，build.gradle.kts 不再触发误分类）。

module kind 分类改善汇总：

- `wiki-core`: infrastructure → cli-tool（Cargo.toml `[[bin]]` 检测）
- `codebuddy`: frontend-app → cli-tool（agents 路径覆盖）
- `axum-macros`: module → library（Cargo.toml `[lib]` 检测）
- storybook `core`/`cli-sb`/`cli-storybook`/`create-storybook`: frontend-app → cli-tool（package.json `"bin"` 检测）
- `test-django-ninja/tests`: backend-service → infrastructure（纯 shell 脚本目录）
- `test-spring-petclinic/src`: frontend-app → module（前后端标签同时存在时不武断分类）

## 是否需要回调本 change

当前不需要回调本 change 的 spec / design / tasks。

原因：

- 所有测试项目均成功 init，无 crash 或 metadata 缺失。
- 页面数量全面收敛，噪声问题显著改善。
- module kind 分类质量明显提升，核心场景（library、cli-tool、infrastructure、agent/adapter 覆盖）均已验证。
- 关键源码选择信号修正生效，fixture 和 test 文件不再污染关键源码列表。
- 剩余差异仍集中在 planner / composition 深度和主题化知识组织，属于后续迭代范围。
