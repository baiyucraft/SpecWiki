## ADDED Requirements

### Requirement: 长流程事件流必须让宿主稳定消费 usage 与 runtime 摘要
系统 MUST 让 `init`、`update` 与 `rebuild` 的事件流对宿主稳定暴露 `usage` 与最终 `runtime_summary`。`usage` MAY 出现在中间 `progress` 事件中；`runtime_summary` MUST 在最终 `result` 或等价终态载荷中可消费。

#### Scenario: progress 事件可观测 usage
- **WHEN** 长流程 workflow 在执行中发生真实 LLM 请求
- **THEN** 宿主 MUST 能从事件流中的 `progress` 或等价事件读取累计 `usage`
- **THEN** CodeBuddy 或其他宿主 MUST 不必依赖 debug trace 才能获取 usage

#### Scenario: 终态事件回传 runtime 摘要
- **WHEN** 长流程 workflow 成功结束
- **THEN** 最终 `result` 事件中的载荷 MUST 允许包含当前阶段可消费的 `runtime_summary`
- **THEN** 宿主 MUST 能从该摘要恢复 workflow 的 readiness 结果

### Requirement: 真实执行路径信号只在实际 workflow 终态中出现
系统 MUST 只在真正执行过的长流程 workflow 终态中输出真实 `llm_execution_mode`。`progress` 事件与非长流程 action MUST NOT 伪造某次 workflow 已真实采用的 `provider_direct / agent_bridge / deterministic_only` 路径。

#### Scenario: provider direct workflow 终态输出真实执行路径
- **WHEN** workflow 由 core 直接走 provider 路径完成
- **THEN** 最终终态载荷 MUST 在适用时输出 `llm_execution_mode = provider_direct`
- **THEN** 宿主 MUST 不必再通过“是否收到 llm_request”做反向推断

#### Scenario: 非长流程 action 不输出伪执行路径
- **WHEN** 调用方执行 `status`、`query` 或其他非长流程 action
- **THEN** 系统 MUST NOT 在这些 action 的结果中冒充已有某次 workflow 的真实 `llm_execution_mode`
- **THEN** 调用方 MUST 不会把这些结果误解成实际 workflow 执行记录
