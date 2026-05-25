## ADDED Requirements

### Requirement: CodeBuddy 必须薄消费当前阶段 runtime preflight 结果
CodeBuddy Agent MUST 能薄消费当前阶段 `wiki-runtime` 的 preflight 结果，包括 `state`、`facts_ready`、`query_readiness`、`recommended_action` 以及可用时的 `runtime_summary / gate_summary` 投影。Agent MUST 只做协议解析、类型校验和结果透传，而 MUST NOT 在 TS 层实现新的 Wiki 业务状态机。

#### Scenario: CodeBuddy 解析 preflight 结果但不重写决策
- **WHEN** CodeBuddy 调用 `wikiStatus`
- **THEN** Agent MUST 能解析并返回当前阶段 preflight 结果
- **THEN** Agent MUST NOT 在 TS 层自行推导另一套 Wiki 业务状态机

#### Scenario: 可选摘要缺失不导致 Agent 失败
- **WHEN** `wikiStatus` 返回的 `runtime_summary` 或 `gate_summary` 缺失
- **THEN** CodeBuddy MUST 仍能正确处理该响应
- **THEN** Agent MUST 不把这类缺失视为协议错误

### Requirement: CodeBuddy 必须消费当前阶段 query trust/profile，而不是只看最薄外壳
CodeBuddy Agent MUST 能消费当前阶段 query 返回中的 `query_mode`、`query_trust`、`recommended_action` 与 provenance 相关字段。Agent MUST 将这些字段透传给宿主，而不是在 TS 层重写 facts/page 判断。

#### Scenario: CodeBuddy 保留 index-first 与 page fallback 区分
- **WHEN** CodeBuddy 调用 `wikiQuery`
- **THEN** Agent MUST 能透传 query 返回中的 `query_mode` 与 `query_trust`
- **THEN** Agent MUST 不得把 page fallback 命中伪装成 facts/index 结果

### Requirement: CodeBuddy 必须消费长流程中的 usage、runtime 摘要与真实执行路径
CodeBuddy Agent 在调用 `wiki-runtime` 的长流程 workflow 时，MUST 能消费 `usage`、最终 `runtime_summary` 与适用时的真实 `llm_execution_mode`。这些字段 MUST 被视为 runtime profile 的一部分，但 Agent 仍 MUST 保持 thin boundary，不在 TS 层解释 Wiki 业务语义。

#### Scenario: CodeBuddy 消费 usage 与 runtime 摘要
- **WHEN** CodeBuddy 调用 `wikiInit`、`wikiUpdate` 或 `wikiRebuild`
- **THEN** Agent MUST 能消费事件流中的 `usage`
- **THEN** Agent MUST 能从最终终态结果读取并返回 `runtime_summary`

#### Scenario: CodeBuddy 只在终态里消费真实执行路径
- **WHEN** 某次长流程 workflow 实际走了 provider direct、agent bridge 或 deterministic 路径
- **THEN** CodeBuddy MUST 只把终态里返回的真实 `llm_execution_mode` 作为执行路径信号
- **THEN** Agent MUST NOT 仅凭是否收到 `llm_request` 就自行推导并固化另一套执行路径语义
