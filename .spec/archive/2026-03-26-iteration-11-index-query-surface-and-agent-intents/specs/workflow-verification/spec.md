## ADDED Requirements

### Requirement: 验证必须覆盖 `storybook` 与 `dagger` 的 index readiness
系统 MUST 在本轮专项验证中直接检查 `storybook` 与 `dagger` 的 facts snapshot 是否真正落盘，并验证 downstream incomplete 时 index-first 查询仍可工作。验证口径 MUST 以 `modules / module_source_map / symbols / edges` 和 query 结果为主，而不是页面 fidelity。

#### Scenario: 专项验证观察 facts snapshot 非空
- **WHEN** 验证脚本对 `storybook` 或 `dagger` 执行本轮 `init`、`update` 或 `rebuild` 分析
- **THEN** 验证 MUST 观察到 `modules / module_source_map / symbols / edges` 已被写入
- **THEN** 验证 MUST 不得接受这些表继续为 `0` 的结果

#### Scenario: downstream incomplete 时 index 查询仍可用
- **WHEN** `storybook` 或 `dagger` 的 runtime 仍处于 `researching`、`compose_pending` 或等价 downstream incomplete 状态
- **THEN** 验证 MUST 继续执行至少一组代表性的 index-first query
- **THEN** 验证 MUST 观察到 symbol、source、module、entrypoint 或 impact 类结果仍然可查询

### Requirement: 验证必须覆盖统一 query intents 与证据字段语义
系统 MUST 通过 Rust 测试、脚本验证或等价自动化方式覆盖统一 intent taxonomy 与结果字段语义。验证 MUST 至少覆盖 `symbol_lookup`、`source_lookup`、`module_lookup`、`entrypoint_lookup`、`callers`、`callees` 与 `impact_slice`，并验证 `match_basis / score / confidence / reason` 的边界不漂移。

#### Scenario: 统一 query intents 均有自动化覆盖
- **WHEN** 系统执行本轮自动化验证
- **THEN** 验证 MUST 分别覆盖 `symbol_lookup`、`source_lookup`、`module_lookup`、`entrypoint_lookup`、`callers`、`callees` 与 `impact_slice`
- **THEN** 验证 MUST 不得继续使用 `file_lookup` 或 `graph_neighbors` 这类已淘汰术语作为正式断言名

#### Scenario: 结果字段语义不伪造统一 `tier`
- **WHEN** 验证检查 symbol/source 与 graph-derived 查询结果
- **THEN** 验证 MUST 观察到 symbol/source 命中使用 `match_basis`，并在可用时暴露 `score`
- **THEN** 验证 MUST 观察到 graph-derived 命中在可用时暴露 `confidence / reason`
- **THEN** 验证 MUST 观察到系统没有为所有命中统一伪造 `tier`
