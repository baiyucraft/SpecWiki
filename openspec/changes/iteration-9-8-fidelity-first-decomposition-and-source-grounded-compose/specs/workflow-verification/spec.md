## ADDED Requirements

### Requirement: 9.8 专项验证必须把 fidelity 缺口收敛到 decomposition 与 source-grounded compose contract
系统 MUST 将 `9.8` 的专项验收固定为 `storybook + dagger` 的 fidelity-first 收敛验证。验证 MUST 同时检查 `missing pages`、`reuse overage`、`skeleton fidelity` 和 `key source coverage`，并把每类缺口映射到 `KnowledgeUnit decomposition`、`research` 或 `ComposePageContract` 的明确 contract，而不是继续只验证 runtime gate 可见性。

#### Scenario: fresh init 专项验证直接读取 9.8 contract 指标
- **WHEN** 系统对 `storybook` 或 `dagger` 执行 fresh `init` 并生成 9.8 专项报告
- **THEN** 验证 MUST 同时输出 `missing pages`、`reuse overage`、`skeleton fidelity` 与 `key source coverage`
- **THEN** 验证 MUST 把这些指标映射到对应的 decomposition、research 或 compose contract
- **THEN** 系统 MUST 不得继续只以 runtime gate/readiness 作为本轮通过条件

#### Scenario: warm / skip-init 报告不得替代 fresh init 的 9.8 结论
- **WHEN** 系统对同一项目执行 warm report 或 `--skip-init` 报告
- **THEN** 报告 MUST 明确标记其为 warm/runtime 复用口径
- **THEN** 该结果 MUST 只能用于稳定性对照或 gap ledger 复核
- **THEN** 系统 MUST NOT 让 warm / skip-init 报告单独替代 fresh init 的 9.8 验收结论
