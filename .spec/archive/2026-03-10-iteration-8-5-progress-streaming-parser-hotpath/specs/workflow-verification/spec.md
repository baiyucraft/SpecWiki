## ADDED Requirements

### Requirement: 端到端验证必须覆盖 progress 事件流
系统 MUST 提供自动化测试，验证 `wiki-core --json` 在执行 `init`、`update` 或 `rebuild` 时会输出 progress 事件流。验证 MUST 同时覆盖 core 直接调用与 CodeBuddy Agent 流式消费路径。

#### Scenario: core 直接调用时输出 progress 与最终 result
- **WHEN** 测试执行 `init`、`update` 或 `rebuild`
- **THEN** 测试 MUST 观察到至少一个 `progress` 事件和一个最终 `result` 或 `error` 事件
- **THEN** 测试 MUST 观察到最终终态事件可恢复出现有 `CoreResponse` 语义

#### Scenario: Agent 流式消费 progress 后仍返回最终结果
- **WHEN** 测试通过 CodeBuddy Agent 调用长流程 workflow
- **THEN** 测试 MUST 观察到 Agent 能消费 progress 事件而不报协议错误
- **THEN** 测试 MUST 观察到 Agent 最终仍返回与终态事件等价的最终结果

### Requirement: 验证必须覆盖 parser 热路径优化的一致性与局部读取行为
系统 MUST 提供自动化测试，验证 parse/query 复用与有限并行不会改变 symbol parsing 结果，并验证 `update` 在小范围 graph 变化下优先走局部 symbol/edge 读取，而不是固定回退为全量读取。

#### Scenario: parse 工件复用不改变 symbol 语义
- **WHEN** 测试对包含 definitions 与 raw relation captures 的代表性源码执行 symbol parsing
- **THEN** 测试 MUST 观察到优化后的 symbols、imports、calls 和 heritage 结果与基线语义一致
- **THEN** 测试 MUST 观察到诊断隔离行为保持不变

#### Scenario: 不同并行度下 symbol parsing 结果一致
- **WHEN** 测试分别以单工作单元和默认并行度执行同一批源码的 symbol parsing
- **THEN** 测试 MUST 观察到相同的 symbol IDs、raw captures 和诊断集合
- **THEN** 测试 MUST 观察到结果排序保持一致

#### Scenario: 小范围 update 优先走局部读取
- **WHEN** 测试在 `init` 后只修改少量源码文件并执行 `update`
- **THEN** 测试 MUST 观察到系统优先使用局部 symbol/edge 读取路径
- **THEN** 测试 MUST 观察到系统不会对该类小变更固定执行全量 `symbols / edges` 枚举
