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
