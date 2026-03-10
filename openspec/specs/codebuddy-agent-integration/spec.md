# codebuddy-agent-integration Specification

## Purpose
TBD - created by archiving change bootstrap-windows-codebuddy-core. Update Purpose after archive.
## Requirements
### Requirement: 第一阶段 Agent 集成必须限定为 Windows 下的 CodeBuddy Agent
系统 MUST 将第一阶段宿主支持范围限定为 Windows 下的 CodeBuddy Agent，不要求在本阶段支持其他平台或宿主。

#### Scenario: 在 Windows 下使用 Agent
- **WHEN** 用户在 Windows 环境中加载 CodeBuddy Agent
- **THEN** Agent 必须能够解析并调用本地 `wiki-core` binary

#### Scenario: 非当前阶段宿主
- **WHEN** 用户尝试在非 Windows 或非 CodeBuddy 宿主中复用当前第一阶段集成
- **THEN** 系统不得声称该集成已受支持

### Requirement: CodeBuddy Agent 必须暴露与 core 对应的一组 Wiki 工具
CodeBuddy Agent MUST 暴露 `wikiInit`、`wikiStatus`、`wikiUpdate`、`wikiQuery`、`wikiSync`、`wikiRebuild` 六个工具，并与 core action 一一对应。

#### Scenario: 创建工具集合
- **WHEN** 宿主加载 CodeBuddy Agent 工具
- **THEN** Agent 必须返回上述六个 Wiki 工具
- **THEN** 每个工具必须映射到对应的 core action

### Requirement: CodeBuddy Agent 必须保持 thin Agent 边界
CodeBuddy Agent MUST 通过本地进程调用 `wiki-core`，并只负责参数收集、binary 定位、结果解析和错误透传，不在 Agent 层实现 Wiki 业务逻辑。

#### Scenario: 调用 Wiki core
- **WHEN** 任一 Wiki 工具被调用
- **THEN** Agent 必须将请求转换为 core 可识别的命令
- **THEN** Agent 必须调用本地 binary
- **THEN** Agent 必须将结果转换为宿主可消费的返回值

#### Scenario: core 返回错误
- **WHEN** `wiki-core` 返回错误
- **THEN** Agent 必须向宿主返回明确错误信息
- **THEN** Agent 不得在 TS 层静默改写 Wiki 业务状态

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

