## ADDED Requirements

### Requirement: reference fidelity 报告必须作为 primary gate 的正式输入
系统 MUST 将 reference fidelity 报告明确收口为当前 primary gate 的正式输入之一，而不是继续把它视为独立、无限扩张的质量分析工具。对于 `storybook + dagger`，报告 MUST 为 gate 结论提供可复现的结构化输入，包括 runtime 完整性、页面 fidelity、reuse、skeleton fidelity 与 key source coverage 等正式指标。

#### Scenario: storybook 或 dagger 进入 primary gate 验收
- **WHEN** 系统对 `storybook` 或 `dagger` 执行当前 primary gate 验收
- **THEN** 系统 MUST 读取对应的 reference fidelity 报告作为正式 gate 输入
- **THEN** 报告 MUST 提供足够的结构化指标支撑 gate 结论，而不是只给出自由文本摘要

### Requirement: reference fidelity 报告不得取代其他正式 gates
系统 MUST 明确 reference fidelity 报告只是当前 primary gate 的一个输入面，而不是所有验收的总替代物。即使 fidelity 指标看起来良好，系统仍 MUST 继续验证 artifact validity、restore validity、query route contract 与 `status/recommended_action` stability；系统 MUST NOT 仅凭 reference fidelity 通过就判定整个 change 收口完成。

#### Scenario: fidelity 良好但 formal gate 失败
- **WHEN** `storybook` 或 `dagger` 的 fidelity 指标满足门槛，但 formal artifacts、restore 或 query route contract 仍失败
- **THEN** 系统 MUST 继续判定当前 change 未通过正式 gate
- **THEN** 系统 MUST 不得仅凭 fidelity 结果给出通过结论
