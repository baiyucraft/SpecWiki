## ADDED Requirements

### Requirement: Knowledge governance metrics MUST aggregate only from formal runtime signals
系统 MUST 仅从正式 runtime signals 聚合 knowledge governance metrics，包括 health signals、governance resolution state、answer degradation state 与 readiness / recovery 结果。系统 MUST NOT 从 markdown 报告正文或人工摘要反推这些指标。

#### Scenario: 生成 release-facing governance metrics
- **WHEN** 系统为某次 release 收集 governance metrics
- **THEN** 所有指标 MUST 来源于正式 runtime signals
- **THEN** 系统 MUST NOT 通过扫描 markdown 报告正文来反推 stale、blocked、conflict 或 degraded 指标

### Requirement: Release judgment MUST separate metrics, blockers, and waivers
release-facing governance layer MUST 将 `metrics`、`blockers` 与 `waivers` 作为不同对象表达。系统 MUST NOT 只输出一个不可解释的 pass/fail 结论。

#### Scenario: 某次 release 存在 conflict blocker 但有临时 waiver
- **WHEN** 某次 release 同时存在 knowledge metric 异常与批准的 waiver
- **THEN** 系统 MUST 能分别表达指标值、blocker 判断与 waiver 记录
- **THEN** release judgment MUST 不被压平成单一无解释分数

### Requirement: Release evidence MUST combine primary samples and batch baseline without flattening their roles
release evidence MUST 同时引用 `storybook + dagger` primary sample 与 `19` 项目 batch baseline，但 MUST 保持二者职责分离。系统 MUST NOT 将二者压平为单一“总分”或单一通过率。

#### Scenario: 团队生成 release evidence
- **WHEN** 系统为某轮 release 生成 evidence manifest
- **THEN** evidence MUST 同时列出 primary sample 结论与 batch baseline 结论
- **THEN** 系统 MUST 保留二者各自的角色说明，而不是只给一个总通过率
