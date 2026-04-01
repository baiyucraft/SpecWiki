## ADDED Requirements

### Requirement: runtime 的 `update` 必须以 knowledge-first refresh 为正式主线
系统 MUST 让正式 `update` 遵循 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections` 主线，而不是继续把 page-first dirty rewrite 作为一级语义。facts/index refresh MAY 继续先行，但 research / compose / assemble 的刷新范围 MUST 由 `AffectedKnowledgeScope` 决定。整树 knowledge replan 仍属于 `update` 的一种合法执行路径，不得自动等同于 lifecycle rebuild。

#### Scenario: 局部变更先刷新局部 knowledge scope
- **WHEN** 当前变更可以稳定映射到局部 `AffectedKnowledgeScope`
- **THEN** runtime MUST 只对该 scope 覆盖的 derived knowledge 执行 refresh
- **THEN** runtime MUST 只对由该 scope 派生出的页面投影执行 compose / assemble

#### Scenario: 整树 knowledge replan 仍属于 update
- **WHEN** 当前变更需要执行 `repo_replan` 级别的 knowledge tree 刷新，但 formal artifacts 与 runtime 状态仍保持一致可用
- **THEN** runtime MUST 将这次执行继续视为 `update`
- **THEN** 系统 MUST NOT 仅因整树 replan 就自动切换到 rebuild workflow

### Requirement: `update` 提交必须按 knowledge scope 定向刷新正式产物与 projection anchors
系统 MUST 让 `update` 在提交阶段按 `AffectedKnowledgeScope` 定向刷新 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与 `.wiki/.cache/**`。未受影响的 formal records、projection anchors 与页面投影 MUST 保持稳定；受影响或已移除的 unit/page 对应记录 MUST 被显式更新或回收。系统 MUST 以统一 snapshot 身份完成本次提交，而不是让 `.knowledge`、pages、metadata 与 cache 各自漂移。

#### Scenario: 未受影响 formal records 保持稳定
- **WHEN** 某次 `update` 只命中局部 `AffectedKnowledgeScope`
- **THEN** 不在该 scope 内的 formal knowledge records、projection anchors 与页面投影 MUST 保持稳定身份
- **THEN** 系统 MUST NOT 因为本次局部 refresh 而把未受影响记录一并标记为 touched

#### Scenario: 移除单元时显式回收正式产物
- **WHEN** 某次 `update` 生成了 `removed_unit_ids` 或 `removed_page_ids`
- **THEN** runtime MUST 显式回收对应的 formal records、projection anchors 与页面投影
- **THEN** runtime MUST 同步更新 metadata / recovery 锚点，使最终 snapshot 不再引用已移除对象
