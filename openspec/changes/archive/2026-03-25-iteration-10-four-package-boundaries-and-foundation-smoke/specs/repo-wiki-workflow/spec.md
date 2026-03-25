## ADDED Requirements

### Requirement: workflow 编排必须遵守四 crate 公共合同
系统 MUST 让 `init`、`update` 与 `rebuild` 由 `wiki-runtime` 编排 `wiki-index` 与 `wiki-knowledge` 的公开合同完成，而不是继续在 runtime 内直接持有 scanner、knowledge planner、research engine 或 compose engine 的主实现。workflow 的执行边界 MUST 继续体现 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，但本轮只要求拆层后基础 workflow 不回退，不要求提前完成 runtime final query route 或 lifecycle 收口。

#### Scenario: init 通过 index 与 knowledge 合同执行主链
- **WHEN** 系统执行正式 `init`
- **THEN** `wiki-runtime` MUST 通过 `wiki-index` 合同完成 facts/index 相关阶段
- **THEN** `wiki-runtime` MUST 通过 `wiki-knowledge` 合同完成 knowledge planning、research 与 compose
- **THEN** `wiki-runtime` MUST 只负责 assemble、写盘、状态导出与进度/transport 协调

#### Scenario: update 与 rebuild 不得重新把主实现粘回 runtime
- **WHEN** 系统执行 `update` 或 `rebuild`
- **THEN** runtime MUST 继续通过 `wiki-index` 与 `wiki-knowledge` 的公开合同完成对应阶段
- **THEN** 系统 MUST NOT 为了兼容旧入口在 runtime 内保留 facts 或 knowledge 的平行主实现

