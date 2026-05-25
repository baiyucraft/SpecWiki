# 迭代 5 测试项目集分析报告

## 概述

19 个测试项目全部 `init` 成功，所有输出页面均包含 managed section marker（`<!-- wiki:managed:start ...-->`）。editable runtime 的 marker 协议与现有 pipeline 完全兼容，无回归。`.wiki` 目录已保留在 `tmp/test-*/.wiki/` 下供人工检查。

## 项目结果汇总

| 项目 | 页面数 | marker 覆盖 | 状态 | 备注 |
|------|--------|-------------|------|------|
| test-aLocal | 3 | 3/3 | OK | 混合语言参考项目 |
| test-spec-wiki | 3 | 3/3 | OK | 本项目自身 |
| test-axum | 6 | 6/6 | OK | Rust workspace，4 个 crate 模块页 |
| test-bat | 15 | 15/15 | OK | tests/syntax-tests 子树偏深 |
| test-cobra | 3 | 3/3 | OK | 扁平 Go 结构 |
| test-chi | 3 | 3/3 | OK | 轻量 Go web |
| test-gin | 3 | 3/3 | OK | Go 框架 |
| test-httpx | 4 | 4/4 | OK | Python HTTP 客户端 |
| test-django-ninja | 5 | 5/5 | OK | Django 生态 |
| test-fastapi | 7 | 7/7 | OK | docs/tests 各占一页 |
| test-spring-petclinic | 3 | 3/3 | OK | Maven + Spring Boot |
| test-leakcanary | 9 | 9/9 | OK | Gradle + Kotlin/Java |
| test-dagger | 27 | 27/27 | OK | 大型 DI 框架，模块页多 |
| test-zustand | 3 | 3/3 | OK | 纯 TS 小型库 |
| test-pinia | 15 | 15/15 | OK | pnpm workspace |
| test-wot-starter | 7 | 7/7 | OK | UniApp + Vue3 |
| test-storybook | 67 | 67/67 | OK | 大型 monorepo，页面最多 |
| test-docker-mailserver | 3 | 3/3 | OK | 运维项目 |
| test-restaurant-app | 13 | 13/13 | OK | 多语言微服务 |

## 重点项目分析

### test-aLocal（3 页）

- 页面结构：项目概述 + 系统架构 + 1 个模块页（test-aLocal）
- 与 reference-aLocal 对比：reference 有 85 个内容导向页面（API接口文档、前端开发指南、后端开发指南、数据库设计等），当前只有 3 页。差异来自 planner composition 层面（reference 是 LLM 深度生成的内容导向页面，当前是自动模块导向的骨架页面），不属于 editable runtime 范围
- managed marker：所有 3 页均包含 managed section marker

### test-spec-wiki（3 页）

- 页面结构：项目概述 + 系统架构 + 1 个模块页（test-spec-wiki）
- 迭代 4 修复的问题（fixture 误提升、单文件模块、.spec 误入）均未复现
- editable runtime 兼容性：managed marker 正常输出，legacy 迁移路径可用

## Managed Marker 验证

所有 19 个项目共 199 个页面，100% 通过以下检查：
- 每个 `.md` 页面包含 `<!-- wiki:managed:start` 开始标记
- 每个 managed section 有对应的 `<!-- wiki:managed:end` 结束标记
- section_id 稳定（基于 page_id + title 的 hash）
- 页面一级标题 `# xxx` 不包裹在 marker 中（符合设计决策 2）

## Reference 对照

10 个有 reference 的项目逐一对比。reference 来自第三方 wiki 工具（repowiki/deepwiki），采用 LLM 深度生成的内容导向页面；当前输出是自动模块导向的骨架页面。两者的页面组织逻辑不同，对比重点是模块覆盖度和 marker 兼容性。

### reference-aLocal（reference 85 页 vs 当前 3 页）
- reference 按功能域组织（API接口文档、前端开发指南、后端开发指南、数据库设计、部署与运维等 11 个顶级目录）
- 当前只有项目概述 + 系统架构 + 1 个模块页
- 差异原因：reference 是 LLM 深度内容生成，当前是模块骨架。planner composition 质量问题，不在本迭代范围

### reference-axum（reference 107 页 vs 当前 6 页）
- reference 按功能域组织（路由系统、提取器系统、中间件系统、响应系统、宏系统、API参考等）
- 当前 4 个 crate 模块页（axum、axum-core、axum-extra、axum-macros）覆盖了 reference 的核心模块边界
- 模块粒度对齐，内容深度差异来自 planner composition

### reference-bat（reference 80 页 vs 当前 15 页）
- reference 按功能域组织（核心功能详解、命令行接口参考、开发者指南、测试与质量保证等）
- 当前 15 页中 tests/syntax-tests 子树占 10 页（偏深），planner composition 问题
- 模块覆盖度：assets、build、src、tests 均有对应页面

### reference-chi（reference 62 页 vs 当前 3 页）
- reference 深度展开了路由系统、中间件系统、高级特性等
- 当前扁平结构（项目概述 + 系统架构 + 1 模块页），符合小型 Go 库的模块结构

### reference-cobra（reference 38 页 vs 当前 3 页）
- reference 展开了命令系统、标志系统、补全系统、文档生成系统等
- 当前扁平结构，符合小型 Go 库的模块结构

### reference-dagger（reference 65 页 vs 当前 27 页）
- reference 按概念域组织（核心概念、Hilt框架、Android集成、编译时处理等）
- 当前 25 个模块页覆盖了 dagger-android、dagger-compiler、hilt-android、hilt-compiler 等所有 Gradle 子模块
- 模块粒度对齐度高，是所有对比中最接近的

### reference-pinia（reference 62 页 vs 当前 15 页）
- reference 按概念域组织（核心概念、高级功能、框架集成、API参考等）
- 当前 packages 子树展开了 docs、nuxt、pinia、playground、testing 等 workspace 包
- 模块边界与 reference 的 monorepo 结构基本对齐

### reference-restaurant-app（reference 117 页 vs 当前 13 页）
- reference 深度展开了微服务架构、前端应用、后端服务、API接口、数据库设计等
- 当前 11 个模块页覆盖了 backend/services 下的 catalog-api、checkout-api、web-app、web.admin 等微服务
- 微服务边界对齐，内容深度差异来自 planner composition

### reference-storybook（reference 176 页 vs 当前 67 页）
- 两者都是大型 monorepo，页面数量最多
- 当前 67 页覆盖了 code/addons、code/builders、code/frameworks、code/renderers、code/lib 等所有 workspace 包
- 模块粒度对齐度较高

### reference-zustand（reference 96 页 vs 当前 3 页）
- reference 深度展开了核心概念、中间件系统、React集成、测试策略等
- 当前扁平结构（项目概述 + 系统架构 + src 模块页），符合小型 TS 库的模块结构

## 结论

1. editable runtime 的 managed marker 协议与所有 19 个测试项目完全兼容，199 个页面 100% 包含正确的 managed marker，无回归
2. 所有页面均正确输出 managed section marker，为后续 sync/update/rebuild 的 user section 保留提供了基础
3. 与 10 个 reference 的对比显示：模块边界覆盖度合理（尤其 dagger、storybook、pinia 等大型项目），页面数量差异主要来自 planner composition 层面（reference 是 LLM 深度内容生成，当前是模块骨架），这些不在迭代 5 范围内
4. 迭代 4 修复的 scanner/hierarchy 质量问题（fixture 误提升、单文件模块、.spec 误入、kind 分类）均未复现
5. `.wiki` 目录已保留在 `tmp/test-*/.wiki/` 下，可随时人工检查页面内容和 marker 格式
