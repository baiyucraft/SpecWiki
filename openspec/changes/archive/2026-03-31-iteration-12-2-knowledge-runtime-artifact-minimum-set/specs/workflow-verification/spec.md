## ADDED Requirements

### Requirement: 本 change 的专项验证必须覆盖 `.knowledge` 最小正式产物与 cold restore
系统 MUST 在本 change 的专项验证中优先验证 `.wiki/.knowledge/**` 的最小正式产物集、cold restore / cache rebuild 与恢复后的 runtime 可消费性，而不是继续把页面 fidelity 作为主门禁。验证 MUST 直接读取 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与重建后的 `.wiki/.cache/wiki-cache.db`，确认：

- formal artifact taxonomy 是否完整
- restore 是否只执行 cache rebuild，而不是隐式重跑 research / compose / assemble
- 恢复后 `status` 是否能正确表达 `ready / stale / needs_update / blocker`
- 恢复后当前 `query` 入口是否能消费恢复态 runtime

恢复成功的结论 MUST 只表示“本地 cache 可重建、`status/query` 可工作、blocker/readiness 可诊断”，MUST NOT 被表述成“完整 wiki runtime 已 ready”。

#### Scenario: storybook 成为 `.knowledge` 与 cold restore 的主门禁
- **WHEN** 系统为本 change 执行专项验证
- **THEN** `storybook` MUST 作为 `.knowledge` 落盘、cache rebuild 与恢复态诊断的主门禁样本
- **THEN** 验证 MUST 输出 `.knowledge` 对象完整性、restore 结果与恢复后 `status/query` 表现

#### Scenario: restore 不得偷换成隐式 regenerate
- **WHEN** 验证脚本在删除或缺失 `.wiki/.cache/**` 的情况下执行恢复流程
- **THEN** 验证 MUST 证明系统执行的是 cache rebuild
- **THEN** 验证 MUST 证明系统没有隐式触发 planning、research、compose 或 assemble 主链

#### Scenario: dagger 作为观察项但既有基线不得降级
- **WHEN** 系统在本 change 中同时记录 `dagger` 的专项结果
- **THEN** `dagger` MAY 作为观察项而不是前置门禁
- **THEN** 系统 MUST NOT 因此删除、降级或绕过既有 lifecycle、index-readiness 与 blocker/readiness 基线验证
