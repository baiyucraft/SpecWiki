## ADDED Requirements

### Requirement: runtime 的 `status` 必须提供当前阶段 preflight 结果
系统 MUST 让 `status` 除现有脏判断外，还能为宿主提供当前阶段 preflight 结果。该结果 MUST 至少包含 `state`、`facts_ready`、`query_readiness` 与 `recommended_action`。当 runtime 已持久化相关 checkpoint/gate 状态时，`status` MAY 附带 `runtime_summary` 与 `gate_summary` 的可选投影。

#### Scenario: `status` 在 stale runtime 上返回推荐动作
- **WHEN** 宿主对一个 `stale` runtime 执行 `status`
- **THEN** 返回结果 MUST 指出当前 `state = stale`
- **THEN** 返回结果 MUST 给出面向宿主的 `recommended_action`

#### Scenario: `status` 摘要投影缺失时仍然成功
- **WHEN** 当前仓库不存在 `runtime_summary` 或 `gate_summary` 对应的持久化数据
- **THEN** `status` MUST 仍返回成功结果
- **THEN** 这些摘要字段 MUST 允许缺失，而不是导致协议失败

### Requirement: runtime 的 query 结果必须显式投影当前阶段 trust
系统 MUST 让 `query` 在当前阶段返回可供宿主消费的 trust/profile 字段。返回结果 MUST 显式区分 `index_first`、`page_fallback` 与 `mixed` 来源，并给出 `query_trust` 与 `recommended_action`，而不只返回 `matches` 或 `matched_*` 列表。

#### Scenario: index-first query 返回可消费 trust 字段
- **WHEN** 宿主执行 query，且结果主要来自当前 facts/index snapshot
- **THEN** 返回结果 MUST 标记 `query_mode = index_first` 或等价语义
- **THEN** 返回结果 MUST 明确当前 `query_trust`

#### Scenario: page fallback query 保留 provenance
- **WHEN** 宿主执行 query，且当前结果需要依赖 page fallback
- **THEN** 返回结果 MUST 保留 page fallback provenance
- **THEN** 系统 MUST NOT 把这类结果伪装成纯 facts/index 命中

### Requirement: 长流程终态必须回传 runtime 摘要与真实执行路径
系统 MUST 让 `init`、`update` 与 `rebuild` 的最终终态结果在适用时回传 `runtime_summary`，并在实际触发 LLM/runtime 路径时回传真实 `llm_execution_mode`。系统 MUST NOT 要求 `status` 或其他非长流程 action 伪造这些真实执行路径字段。

#### Scenario: `init` 终态返回 runtime 摘要
- **WHEN** 宿主执行 `init` 并成功结束
- **THEN** 最终终态结果 MUST 包含当前阶段可消费的 `runtime_summary`
- **THEN** 宿主 MUST 能用该摘要判断 workflow 已推进到哪一层

#### Scenario: 真实执行路径只在 workflow 终态里出现
- **WHEN** `update` 或 `rebuild` 实际执行并触发了 provider 直连、agent bridge 或 deterministic 路径
- **THEN** 最终终态结果 MUST 在适用时包含真实 `llm_execution_mode`
- **THEN** `status / query / sync` 这类 action MUST NOT 冒充已有该字段的真实语义
