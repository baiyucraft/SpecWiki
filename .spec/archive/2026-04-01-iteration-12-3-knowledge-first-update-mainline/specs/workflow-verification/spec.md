## ADDED Requirements

### Requirement: 本 change 的专项验证必须覆盖 knowledge-first update 作用域与 parent propagation
系统 MUST 在本 change 的专项验证中优先验证 `storybook + dagger` 的 knowledge-first update 行为，而不是继续把页面 fidelity 作为主门禁。验证 MUST 直接读取 formal knowledge artifacts、runtime state 与最终页面投影，确认：

- `ChangeSet` 是否先落到 `AffectedKnowledgeScope`
- parent refresh 是否只在 `child contract changed` 时传播
- `update` 是否只刷新受影响的 knowledge scope 与 projection targets
- escalation reason 是否与实际结构变化一致

#### Scenario: storybook 局部变更只刷新局部 knowledge scope
- **WHEN** 系统对 `storybook` 执行一次局部源码变更后的 `update` 验证
- **THEN** 报告 MUST 指出本次命中的 `AffectedKnowledgeScope`、派生出的 projection targets 与未受影响的稳定 records
- **THEN** 验证 MUST 证明系统没有把无关 parent 或页面默认一起重刷

#### Scenario: dagger 结构变化显式提升 scope escalation
- **WHEN** 系统对 `dagger` 执行触发结构变化的 `update` 验证
- **THEN** 报告 MUST 指出本次 escalation 是 `subtree_replan`、`repo_replan` 或 `rebuild_recommended`
- **THEN** 验证 MUST 证明该升级来自 knowledge scope 判定，而不是页面 dirty 数量或 Markdown 差异

### Requirement: 本 change 的 tasks 设计与测试必须继续执行完整项目集 init 分析与注释合规检查
系统 MUST 在本 change 的 tasks 设计或测试阶段继续执行 `node scripts/run-test-projects.mjs` 的完整项目集 `init` 分析，并单独执行一次 `.wiki/02-开发指南/00-代码注释规范.md` 合规检查。完整项目集分析 MUST 作为 baseline guard 保留，但本轮 primary gate 仍然是 `storybook + dagger` 的 knowledge-first update 专项结果。

#### Scenario: tasks 设计或测试阶段执行完整项目集 init 分析
- **WHEN** 开发者进入本 change 的 tasks 设计或测试阶段
- **THEN** 系统 MUST 执行 `node scripts/run-test-projects.mjs` 的完整项目集 `init` 分析
- **THEN** 系统 MUST 输出 `test-project-analysis.md` 或等价逐项目报告，并明确这轮 primary gate 仍是 `storybook + dagger`

#### Scenario: 本 change 单独执行注释合规检查
- **WHEN** 本 change 进入实现前校验或测试收口阶段
- **THEN** 系统 MUST 单独检查涉及代码与脚本的注释是否符合 `.wiki/02-开发指南/00-代码注释规范.md`
- **THEN** 该检查结果 MUST 进入 tasks 或验收记录，而不是口头略过
