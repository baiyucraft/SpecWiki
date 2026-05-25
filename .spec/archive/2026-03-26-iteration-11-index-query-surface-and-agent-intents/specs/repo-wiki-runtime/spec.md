## ADDED Requirements

### Requirement: runtime 的外部 query 入口本轮必须保持 `term` 合同稳定
系统 MUST 在本轮继续保留外部 `CoreCommand.term` 与 `run_query(repo_root, term)` 入口，不得提前引入新的外部结构化 query payload。runtime 内部 MAY 构造结构化请求调用 `wiki-index::query`，但该结构只属于 crate 内部边界，不属于本轮正式 transport 合同。

#### Scenario: 非空 `term` 映射为内部 `auto` 请求
- **WHEN** 调用方通过现有 query 入口传入非空 `term`
- **THEN** runtime MUST 将该输入映射为内部 `auto` 查询请求
- **THEN** runtime MUST 优先把该请求路由到 `wiki-index::query`

#### Scenario: 空 `term` 继续返回空结果
- **WHEN** 调用方通过现有 query 入口传入空 `term`
- **THEN** runtime MUST 继续返回空查询结果
- **THEN** 系统 MUST NOT 因此引入新的外部参数或 breaking contract

### Requirement: runtime 的 query route 必须以 index 投影为主、page fallback 为辅
系统 MUST 让 `run_query` 的正式结果优先来自 `wiki-index::query` 的投影，而不是继续由 runtime 自己重新实现 facts 查询语义。`matched_symbols`、`matched_sources`、`matched_modules`、`matched_symbol_edges` 等字段 MUST 以 index 投影为主，`matches` 中的页面结果只可作为 fallback 或补充 provenance。

#### Scenario: 存在 index 命中时优先返回 index 投影
- **WHEN** 某次 query 在 `wiki-index::query` 中命中了 symbol、source、module、entrypoint 或 graph 结果
- **THEN** runtime 返回中的 `matched_symbols`、`matched_sources`、`matched_modules`、`matched_symbol_edges` MUST 来自 index 投影
- **THEN** runtime MUST NOT 再通过 `WikiState` 自行重算一套等价 facts 结果

#### Scenario: page fallback 必须显式标注 provenance
- **WHEN** 某次 query 没有有效 index 命中，只能通过 page fallback 命中页面内容
- **THEN** runtime MUST 在结果中显式标注 page fallback provenance
- **THEN** 系统 MUST NOT 把页面命中伪装成 facts/index 命中

### Requirement: runtime 必须区分 `index not ready` 与“空命中”
系统 MUST 对外区分“facts snapshot 尚未提交”和“查询执行成功但没有命中”这两类状态。当前者发生时，runtime MUST 返回显式 `index not ready` 错误，而不能返回空命中成功。

#### Scenario: snapshot 未就绪返回显式错误
- **WHEN** 调用方执行 query，但当前仓库尚未完成首次 facts snapshot 提交
- **THEN** runtime MUST 返回显式 `index not ready` 错误
- **THEN** 返回结果 MUST NOT 伪装成正常空命中

#### Scenario: 查询成功但无命中返回空结果
- **WHEN** 调用方执行 query，且 facts snapshot 已就绪，但当前 term 或内部请求没有命中任何对象
- **THEN** runtime MUST 返回成功的空结果
- **THEN** 系统 MUST 不得把这种情况提升为 `index not ready`
