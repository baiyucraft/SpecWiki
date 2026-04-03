## ADDED Requirements

### Requirement: 公开 workflow surface 验证必须覆盖 `sync` 与 `rebuild`
系统 MUST 提供自动化测试，验证当前版本的 CLI/help、宿主 action skills 与公开 workflow 常量已经同步开放 `sync` 与 `rebuild`，而不是仍停留在 `init / status / update / query` 四入口。

#### Scenario: CLI 与帮助文本公开 `sync` / `rebuild`
- **WHEN** 测试读取主包 CLI 帮助文本或等价公开入口
- **THEN** 结果 MUST 包含 `sync` 与 `rebuild`
- **THEN** 测试 MUST 证明系统不再把两者表述为 unsupported action

#### Scenario: 宿主 action skills 与公开 workflow 常量同步开放
- **WHEN** 测试执行宿主 bootstrap 或读取共享 workflow/action 常量
- **THEN** 结果 MUST 同步包含 `wiki-sync` 与 `wiki-rebuild`
- **THEN** 测试 MUST 证明 CLI、CodeBuddy、Codex/Claude 等公开入口没有出现半公开面分裂

### Requirement: 公开后的 `sync` 与 `rebuild` 必须维持现有 transport 合同
系统 MUST 提供自动化测试或脚本验证，证明 `sync` 在公开后继续作为短流程 JSON action，`rebuild` 在公开后继续作为 long-running `progress / result / error` action。验证 MUST 证明这是公开入口对齐，而不是新发明一套 transport 语义。

#### Scenario: `sync` 保持短流程 JSON
- **WHEN** 测试执行公开后的 `sync`
- **THEN** 测试 MUST 观察到短流程 JSON 结果
- **THEN** 测试 MUST 证明系统没有为 `sync` 新增流式协议

#### Scenario: `rebuild` 保持长流程事件流
- **WHEN** 测试执行公开后的 `rebuild`
- **THEN** 测试 MUST 观察到 `progress / result / error` 事件流
- **THEN** 测试 MUST 证明该协议与现有长流程合同保持一致

### Requirement: 代表性闭环验证必须覆盖公开后的 `sync` 与 `rebuild`
系统 MUST 在本轮专项验证中使用 `storybook` 作为代表性样本，验证公开后的 `sync` 与 `rebuild` 能被 CLI 或脚本入口实际调用，并与现有 `status/query/update` 闭环连通。该验证是 public-surface smoke，不要求借机重做 19 项目全量回归。

#### Scenario: `storybook` 覆盖公开后的代表性闭环
- **WHEN** 本轮脚本验证对 `storybook` 执行 `init -> status -> sync -> query/update -> rebuild`
- **THEN** `sync` 与 `rebuild` MUST 能通过正式公开入口被调用
- **THEN** 报告 MUST 明确记录这是 public-surface smoke，而不是 19 项目全量 gate

