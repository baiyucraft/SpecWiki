## ADDED Requirements

### Requirement: CodeBuddy Agent 必须在保持 thin Agent 边界的前提下桥接可选 LLM 请求
CodeBuddy Agent 在调用 `wiki-core` 的长流程 workflow 时，MUST 能在协商开启的前提下桥接 `llm_request` 事件。Agent MUST 只负责协议解析、provider 调用、响应透传和错误上报，不得在 TS 层重写 prompt、重建页面上下文或实现 Wiki 业务规则。若宿主或 provider 不可用，Agent MUST 明确返回“不可用”响应，让 core 自行回退。

#### Scenario: 宿主支持 LLM 时 Agent 透传请求与响应
- **WHEN** Agent 调用长流程 workflow，且当前宿主或 provider 支持执行 LLM 请求
- **THEN** Agent MUST 按收到的 `llm_request` 元数据执行对应 LLM 调用
- **THEN** Agent MUST 把响应结果回写给 core，而不是在 TS 层解释 Wiki 语义

#### Scenario: 宿主不支持 LLM 时 Agent 显式返回不可用
- **WHEN** Agent 收到 `llm_request`，但当前宿主未配置可用 provider 或明确关闭增强
- **THEN** Agent MUST 显式回写不可用或跳过响应
- **THEN** Agent MUST 继续完成事件流消费，并向宿主返回最终 workflow 结果

#### Scenario: core 已配置 provider 直连时 Agent 不介入同一请求
- **WHEN** 当前 repo 的 LLM 配置已经让 core 侧拿到可用 provider 直连参数
- **THEN** Agent MUST 不会收到对应的 `llm_request`
- **THEN** Agent 仍只负责现有 progress/result/error 消费，不得重复发起同一轮 provider 调用
