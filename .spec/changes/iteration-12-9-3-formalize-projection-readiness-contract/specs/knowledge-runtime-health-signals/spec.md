## MODIFIED Requirements

### Requirement: runtime 必须持久化最小 knowledge health signals
系统 MUST 让 health / status 正式消费 projection digest 的 `ready / stale / blocked` 状态，而不是只依赖页面文件、cache 缺失或 runtime gate。若某个 unit 的正式 projection digest 为 `stale` 或 `blocked`，系统 MUST 通过现有 `projection_stale` 或等价稳定 signal 暴露对应诊断。

#### Scenario: stale projection digest 进入 health layer
- **WHEN** 某个 `KnowledgeUnit` 的正式 projection digest 状态为 `stale`
- **THEN** 系统 MUST 生成或保留可聚合的 projection 诊断 signal
- **THEN** `status` MUST 能将其反映为 health degraded，而不是继续显示完全 healthy

#### Scenario: blocked projection digest 进入 status 聚合
- **WHEN** 某个 `KnowledgeUnit` 的正式 projection digest 状态为 `blocked`
- **THEN** `status` 或等价聚合输出 MUST 能稳定暴露该阻断事实
- **THEN** 调用方 MUST 不需要反查 runtime gate 细节才能知道 projection 当前不可消费
