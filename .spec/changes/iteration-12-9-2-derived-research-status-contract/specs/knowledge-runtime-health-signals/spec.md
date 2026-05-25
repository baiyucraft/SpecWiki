## MODIFIED Requirements

### Requirement: runtime 必须持久化最小 knowledge health signals
系统 MUST 让 health / status 正式消费 derived research summary 的 `ready / degraded / blocked` 状态，而不是只依赖 runtime gate。若某个 unit 的正式 research summary 为 `degraded` 或 `blocked`，系统 MUST 通过现有 health signal 框架暴露对应诊断，并保持推荐动作仍落在既有稳定集合中。

#### Scenario: degraded research summary 进入 health layer
- **WHEN** 某个 `KnowledgeUnit` 的正式 research summary 状态为 `degraded`
- **THEN** 系统 MUST 生成或保留可聚合的 health signal
- **THEN** `status` MUST 能将其反映为 health degraded，而不是继续显示完全 healthy

#### Scenario: blocked research summary 进入 status 聚合
- **WHEN** 某个 `KnowledgeUnit` 的正式 research summary 状态为 `blocked`
- **THEN** `status` 或等价聚合输出 MUST 能稳定暴露该阻塞事实
- **THEN** 调用方 MUST 不需要反查 runtime gate 细节才能知道 derived research 当前不可用
