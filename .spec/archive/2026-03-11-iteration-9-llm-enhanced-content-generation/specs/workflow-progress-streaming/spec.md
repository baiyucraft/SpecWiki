## ADDED Requirements

### Requirement: 长流程 NDJSON 协议必须支持可协商的 LLM 请求事件
系统 MUST 在长流程 JSON IPC 中支持可协商的 LLM 请求事件。当调用方显式声明支持 LLM 桥接时，`wiki-core --json` 除了现有 `progress / result / error` 事件外，还 MAY 输出 `llm_request` 事件；对应请求 MUST 带有稳定 `request_id`、`prompt_type`、作用域信息、消息内容、期望返回格式和是否允许回退。未协商桥接时，系统 MUST 保持当前只输出 `progress / result / error` 的行为，不得对旧调用方输出未识别事件。

#### Scenario: 协商开启时输出 `llm_request`
- **WHEN** 调用方以支持 LLM 桥接的模式执行 `init`、`update` 或 `rebuild`
- **THEN** 系统 MAY 在 workflow 中输出一个或多个 `llm_request` 事件
- **THEN** 每个请求 MUST 携带稳定 `request_id` 与可解析的 prompt 元数据

#### Scenario: 未协商时保持现有事件集兼容
- **WHEN** 调用方未声明支持 LLM 桥接
- **THEN** 系统 MUST 只输出现有 `progress / result / error` 事件
- **THEN** 旧版调用方不得因为迭代 9 的协议扩展而收到未知事件

#### Scenario: 已配置 provider 直连时不得无意义地产生 `llm_request`
- **WHEN** 当前 workflow 已经在 core 侧拿到可用的 provider 直连配置
- **THEN** 系统 MUST 直接由 core 发起 provider 调用
- **THEN** 长流程不得再为了同一个请求额外输出 `llm_request` 事件

### Requirement: LLM 会话扩展不得破坏终态唯一性和 progress 顺序
无论是否协商 LLM 桥接，长流程事件流都 MUST 保持现有的终态唯一性约束。`llm_request` 事件只能出现在终态之前；每次请求 MUST 对应一个成功响应、不可用响应或超时回退；workflow 结束时系统 MUST 仍然输出且只输出一个最终 `result` 或 `error` 事件。progress 事件的相对顺序 MUST 与真实 workflow 阶段一致，不得因为 LLM 会话而伪造额外业务阶段。

#### Scenario: `llm_request` 不改变最终终态语义
- **WHEN** 工作流过程中发生一个或多个 `llm_request`
- **THEN** workflow 结束时系统 MUST 仍然输出且只输出一个最终 `result` 或 `error`
- **THEN** 调用方 MUST 能像当前一样恢复最终 `CoreResponse`

#### Scenario: LLM 回退后 progress 仍可观测
- **WHEN** 某个 `llm_request` 因不可用、超时或回退而未得到有效响应
- **THEN** 系统 MUST 继续输出后续 progress 事件和最终终态事件
- **THEN** progress 阶段顺序 MUST 反映真实 workflow，而不是停留在未完成状态
