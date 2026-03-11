# workflow-progress-streaming Specification

## Purpose
定义 `wiki-core` 长流程 workflow 的 NDJSON progress 事件流协议及其终态兼容性约束。

## Requirements
### Requirement: 长流程 workflow 必须输出结构化进度事件流
系统 MUST 为 `init`、`update` 和 `rebuild` 提供结构化 progress 事件流。对这些长流程 action，`wiki-core --json` MUST 通过 stdout 逐行输出 NDJSON 事件，而不是只在结束时输出单个最终对象；调用方需要按 `progress / result / error` 事件流消费结果。

#### Scenario: 长流程 action 输出 NDJSON 事件流
- **WHEN** 调用方执行 `init`、`update` 或 `rebuild`
- **THEN** 系统 MUST 在 workflow 执行期间输出一个或多个 `progress` 事件
- **THEN** 系统 MUST 在结束时输出且只输出一个最终 `result` 或 `error` 事件

### Requirement: progress 事件必须暴露稳定阶段和统一字段
每个 `progress` 事件 MUST 至少包含 `action`、`phase`、`message`、`elapsed_ms`、`processed` 和 `total` 字段。对于存在明确工作项总数的阶段（例如源码解析、页面渲染），系统 MUST 提供非空的 `processed / total` 计数；对于无法可靠估算总数的阶段，系统 MUST 仍保留这两个字段并以 `null` 表示未知。phase 名称 MUST 使用稳定标识，而不是仅输出自由文本日志。

#### Scenario: 解析阶段提供稳定计数
- **WHEN** 系统在 `parse_symbols` 或等价的文件级解析阶段上报进度
- **THEN** 对应 `progress` 事件 MUST 包含稳定的 `phase` 名称
- **THEN** 对应 `progress` 事件 MUST 提供当前已处理文件数和总文件数

#### Scenario: 非计数型阶段仍保持统一字段
- **WHEN** 系统在 `write_state`、`write_metadata` 或其他难以预估总量的阶段上报进度
- **THEN** 对应 `progress` 事件 MUST 仍包含 `processed` 和 `total` 字段
- **THEN** 当总量未知时，这两个字段 MUST 使用 `null` 而不是缺失

### Requirement: 结果事件必须保留现有最终响应语义
启用流式进度时，最终 `result` 或 `error` 事件 MUST 保留现有 `CoreResponse` 的成功/失败语义，而不是定义另一套不兼容的终态格式。对同一请求，系统 MUST 保证最终只收口为一个终态事件；在终态事件发出后，不得继续输出新的 `progress` 事件。

#### Scenario: 成功 workflow 的终态事件兼容 CoreResponse
- **WHEN** `init`、`update` 或 `rebuild` 成功结束
- **THEN** 最终 `result` 事件 MUST 能表达与现有 `CoreResponse.success` 等价的成功载荷
- **THEN** 调用方 MUST 能从该终态事件恢复与非流式模式一致的最终结果

#### Scenario: 失败 workflow 的终态事件唯一且封闭
- **WHEN** 流式 workflow 在执行中失败
- **THEN** 系统 MUST 输出且只输出一个 `error` 终态事件
- **THEN** 在该 `error` 事件之后，系统不得继续输出新的 `progress` 或 `result` 事件

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
