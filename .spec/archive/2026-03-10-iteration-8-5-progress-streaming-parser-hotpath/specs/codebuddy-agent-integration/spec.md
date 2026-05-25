## ADDED Requirements

### Requirement: CodeBuddy Agent 必须消费并透传长流程 progress 事件
CodeBuddy Agent 在调用 `wiki-core` 的 `init`、`update` 和 `rebuild` 时 MUST 直接消费 progress 事件流。Agent MUST 逐行解析 core 输出的 `progress / result / error` 事件，并在保持 thin Agent 边界的前提下把 progress 转交给宿主侧可用的 observer、callback 或等价桥接；若宿主当前不消费 progress，Agent 也 MUST 正常 drain 整个事件流并返回最终结果。

#### Scenario: 宿主订阅 progress 时 Agent 透传阶段事件
- **WHEN** 宿主通过 Agent 调用长流程 workflow，且提供可消费 progress 的桥接
- **THEN** Agent MUST 把 core 发出的 `progress` 事件按到达顺序透传给宿主
- **THEN** Agent 不得在 TS 层重写 workflow 阶段语义或伪造新的业务状态

#### Scenario: 宿主未订阅 progress 时 Agent 仍返回最终结果
- **WHEN** 宿主通过 Agent 调用长流程 workflow，但未消费 progress 事件
- **THEN** Agent MUST 仍然完整读取 core 的事件流
- **THEN** Agent MUST 向宿主返回与非流式模式兼容的最终结果或错误
