## MODIFIED Requirements

### Requirement: 第一阶段 Agent 集成必须限定为 Windows 下的 CodeBuddy Agent
系统 MUST 将第一阶段宿主支持范围限定为 Windows 下的 CodeBuddy Agent，不要求在本阶段支持其他平台或宿主。

#### Scenario: 在 Windows 下使用 Agent
- **WHEN** 用户在 Windows 环境中加载 CodeBuddy Agent
- **THEN** Agent 必须能够解析并调用本地 `wiki-runtime` binary

#### Scenario: 非当前阶段宿主
- **WHEN** 用户尝试在非 Windows 或非 CodeBuddy 宿主中复用当前第一阶段集成
- **THEN** 系统不得声称该集成已受支持

### Requirement: CodeBuddy Agent 必须保持 thin Agent 边界
CodeBuddy Agent MUST 通过本地进程调用 `wiki-runtime`，并只负责参数收集、binary 定位、结果解析和错误透传，不在 Agent 层实现 Wiki 业务逻辑。

#### Scenario: 调用 Wiki runtime
- **WHEN** 任一 Wiki 工具被调用
- **THEN** Agent 必须将请求转换为 runtime 可识别的命令
- **THEN** Agent 必须调用本地 binary
- **THEN** Agent 必须将结果转换为宿主可消费的返回值

#### Scenario: runtime 返回错误
- **WHEN** `wiki-runtime` 返回错误
- **THEN** Agent 必须向宿主返回明确错误信息
- **THEN** Agent 不得在 TS 层静默改写 Wiki 业务状态

