## ADDED Requirements

### Requirement: 当前阶段 `wiki-runtime` 必须提供稳定的宿主消费轮廓
系统 MUST 为当前阶段的 `wiki-runtime` 定义稳定的宿主消费轮廓，使 Provider / CodeBuddy 能按 `preflight -> action -> query` 的方式消费 runtime，而不必读取内部实现细节。该消费轮廓 MUST 建立在现有 `status / init / update / rebuild / sync / query` 与长流程事件流之上，而不是新增高层 Agent 业务工具。

#### Scenario: 宿主按 preflight -> action -> query 消费当前阶段 runtime
- **WHEN** Provider 或 CodeBuddy 需要在当前阶段使用 `wiki-runtime`
- **THEN** 宿主 MUST 能先通过 `status` 获取 preflight 结果
- **THEN** 宿主 MUST 能通过现有 `init / update / rebuild / sync` 推进 runtime
- **THEN** 宿主 MUST 能在合适时机通过 `query` 消费 index-first 查询结果

#### Scenario: 宿主轮廓不要求 Agent 自行实现业务状态机
- **WHEN** CodeBuddy 或其他宿主消费这套 runtime profile
- **THEN** 该 profile MUST 被视为推荐消费轮廓
- **THEN** 系统 MUST NOT 要求 Agent 在宿主层自行实现新的 Wiki 业务状态机或 query route

### Requirement: preflight 结果必须区分必有字段与可选 runtime 摘要
系统 MUST 让 `status` 提供当前阶段可消费的 preflight 结果。结果 MUST 至少包含当前 `state`、`facts_ready`、`query_readiness` 与 `recommended_action`。当 runtime 已持久化 `PipelineRuntimeSummary` 或 `UnitRuntimeGate` 时，`status` MAY 额外返回 `runtime_summary` 与 `gate_summary`；这些摘要 MUST 是可选投影，而不是无条件存在的必有字段。

#### Scenario: 无 checkpoint 摘要时 `status` 仍可独立工作
- **WHEN** 当前仓库尚未持久化 `PipelineRuntimeSummary` 或 `UnitRuntimeGate`
- **THEN** `status` MUST 仍能返回有效 preflight 结果
- **THEN** 系统 MUST NOT 因缺失这些摘要而把 `status` 视为协议失败

#### Scenario: preflight 只提供执行模式预判
- **WHEN** `status` 返回与 LLM/runtime 路径有关的模式字段
- **THEN** 该字段 MUST 仅表示配置或可用性层面的预判
- **THEN** 系统 MUST NOT 把它表述为某次 workflow 已真实采用的执行路径

### Requirement: 当前阶段 query 结果必须显式表达 trust 与来源边界
系统 MUST 让当前阶段 `query` 结果显式表达 `runtime_state`、`query_mode`、`query_trust` 与 `recommended_action`。系统 MUST 区分 `index_first`、`page_fallback` 与 `mixed` 这三类来源，并保留 provenance，使宿主能判断结果是否适合直接用于“找入口 / 看影响”类场景。

#### Scenario: facts/index 命中与 page fallback 被明确区分
- **WHEN** 某次 query 主要命中 facts/index 结果
- **THEN** 结果 MUST 标记 `query_mode = index_first` 或等价语义
- **THEN** 宿主 MUST 能把该结果与 page fallback 命中区分开

#### Scenario: runtime 未 fully ready 时结果带推荐动作
- **WHEN** 某次 query 发生在 `stale`、`blocked` 或等价非 fully ready 状态下
- **THEN** 结果 MUST 明确标记 `query_trust`
- **THEN** 结果 MUST 给出 `recommended_action`，而不是只返回一组无上下文的命中列表

### Requirement: 真实执行路径只属于实际跑过的长流程 workflow
系统 MUST 将 `provider_direct / agent_bridge / deterministic_only` 视为真实 workflow 的执行路径信号，而不是通用的静态模式字段。只有在 `init / update / rebuild` 这类实际跑过的长流程终态中，系统才 MUST 输出真实 `llm_execution_mode`；`status / query / sync` 等未实际触发 LLM/runtime 路径的 action MUST NOT 冒充已有真实执行路径。

#### Scenario: 长流程终态输出真实执行路径
- **WHEN** `init`、`update` 或 `rebuild` 实际执行并结束
- **THEN** 最终终态结果 MUST 在适用时包含真实 `llm_execution_mode`
- **THEN** 宿主 MUST 能用该信号区分 `provider_direct`、`agent_bridge` 与 `deterministic_only`

#### Scenario: 非长流程 action 不伪造执行路径
- **WHEN** 宿主调用 `status`、`query` 或 `sync`
- **THEN** 系统 MUST NOT 把这些 action 的返回值标记成某次 workflow 已真实采用的 `llm_execution_mode`
- **THEN** 宿主 MUST 不必把这些 action 当作“已执行过 LLM/runtime 路径”的证据
