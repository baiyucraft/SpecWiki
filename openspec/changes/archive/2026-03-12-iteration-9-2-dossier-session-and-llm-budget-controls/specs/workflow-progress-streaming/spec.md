## ADDED Requirements

### Requirement: progress/event stream 必须支持 bounded research session 事件
系统 MUST 在开启 provider research session 时，通过现有 workflow event stream 暴露 bounded session 事件。事件流 MUST 至少支持 `agent_session_start`、`agent_message`、`agent_tool_call`、`agent_tool_result`、`agent_final`、`agent_abort`，并继续保持最终终态唯一。后续 agent-bridge 路径 MAY 复用同一事件集。

#### Scenario: research session 期间输出结构化 session 事件
- **WHEN** 当前 workflow 对某个页面执行 research session
- **THEN** 系统 MUST 按到达顺序输出该 session 的结构化事件
- **THEN** workflow 结束时仍 MUST 只输出一个最终 `result` 或 `error`

#### Scenario: 未协商 research session 时保持普通 progress 流
- **WHEN** 调用方未声明支持 session 事件
- **THEN** 系统 MAY 回退到不启用 session 的路径
- **THEN** event stream 不得无意义输出未协商的 session 事件

### Requirement: progress/event stream 必须实时暴露 LLM usage snapshot
系统 MUST 在普通 workflow 模式下实时暴露 LLM usage snapshot，而不是只在 debug trace 中可见。每次真实 LLM 请求完成后，对应 progress 或等价可观测事件 MUST 刷新累计 usage。

#### Scenario: 请求完成后实时刷新 usage snapshot
- **WHEN** workflow 中某次真实 provider 或 agent-bridge 请求完成
- **THEN** 对应 event stream MUST 刷新累计 `request_count`、`input_tokens`、`output_tokens`、`total_tokens`
- **THEN** 调用方 MUST 能在 workflow 尚未结束时观察到 usage 增长

#### Scenario: usage snapshot 附带 prompt_type 或 model 维度
- **WHEN** 系统上报 usage snapshot
- **THEN** snapshot MUST 能区分至少一个 `prompt_type` 或 provider/model 维度
- **THEN** 调用方不得只能拿到没有分组信息的一团总数
