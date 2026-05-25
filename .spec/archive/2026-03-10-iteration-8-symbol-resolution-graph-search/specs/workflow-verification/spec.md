## ADDED Requirements

### Requirement: 端到端验证必须覆盖 symbol resolution 与 graph analysis 生命周期
系统 MUST 提供自动化测试，验证适配层调用 Rust core 后能够在目标仓库完成 symbol resolution、graph analysis、增量 edge refresh 和 graph query，而不仅是 symbol definitions 的写盘。验证 MUST 覆盖 `edges`、`communities`、`community_members`、`processes`、`process_steps` 的生成、一致性与增量清理，并继续确保页面 runtime、不变页面和 user section 保持稳定。

#### Scenario: init 后 graph tables 生成真实数据
- **WHEN** 测试在包含可解析 import/call/heritage 关系的临时仓库中执行 `init`
- **THEN** 测试 MUST 观察到 `edges` 表存在真实业务 rows
- **THEN** 当图分析输入充分时，测试 MUST 观察到 `communities` 和 `processes` 相关表存在真实 rows

#### Scenario: update 后 graph rows 增量刷新
- **WHEN** 测试在 `init` 后修改或删除某个已跟踪源码文件并执行 `update`
- **THEN** 测试 MUST 观察到该文件对应的 symbol rows 和 edge rows 被刷新或清理
- **THEN** 测试 MUST 观察到 graph-derived 结果与新的 edges 保持一致

#### Scenario: query 返回 graph context
- **WHEN** 测试执行 `query` 并使用某个已知 symbol 名作为检索词
- **THEN** 测试 MUST 观察到返回结果除了 `matched_symbols` 外，还包含 graph relation 或 process/community 上下文
- **THEN** 测试 MUST 观察到 provenance 区分 BM25 与 graph 命中来源

### Requirement: 包装语言与项目集验证必须覆盖 Vue / Svelte graph 解析
系统 MUST 在 fixture、integration test 或项目集分析中覆盖 Vue / Svelte 单文件组件的 script wrapper 解析，验证定义类符号、raw relation captures 与 graph resolution 都能映射回原始组件文件。每轮与迭代 8 相关的 tasks 设计、实现或测试时，还 MUST 对 `DESIGN.md § 测试项目集` 的完整项目集执行 `init` 分析，并在 `test-project-analysis.md` 中按项目输出 graph facts、graph query 命中表现与 reference 差异。

#### Scenario: Vue 或 Svelte wrapper 解析通过
- **WHEN** 测试在包含 Vue 或 Svelte 单文件组件的 fixture 或项目中执行 `init`
- **THEN** 测试 MUST 观察到这些组件的 definitions 或 graph relations 能映射回原始组件路径
- **THEN** 测试 MUST 观察到对应 diagnostics 不会把这些文件误记为 unsupported parser

#### Scenario: 测试项目集分析输出 graph 事实表现
- **WHEN** 迭代 8 的 tasks 设计或测试阶段执行完整项目集 `init` 分析
- **THEN** 报告 MUST 按项目逐个说明 edges / communities / processes 的生成表现
- **THEN** 报告 MUST 说明 graph query 的命中表现以及与 reference 的差异或“无 reference”状态

