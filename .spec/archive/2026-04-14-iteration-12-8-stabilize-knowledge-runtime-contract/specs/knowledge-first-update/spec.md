## ADDED Requirements

### Requirement: `update` 必须显式传播 `declared -> derived -> projection` 失效链
系统 MUST 在 knowledge-first update 中显式处理 `declared changed` 与 `facts/index changed` 两类起点，并把它们分别传播到 `derived`、`projection` 与 `cache`。`AffectedKnowledgeScope` MUST 至少能够标记 `declared_record_ids`、`stale_unit_ids`、`stale_projection_ids` 与 `health_signal_targets`，而不是只覆盖 derived unit 与 projection page。

#### Scenario: declared writeback 触发 derived 与 projection 失效
- **WHEN** 某次 `sync` 或显式 authoring 更新了正式 declared record
- **THEN** 后续 `update` MUST 将其关联的 `KnowledgeUnit` 标记为 `stale_unit_ids`
- **THEN** 系统 MUST 继续传播到相关 derived summary 与 projection target，而不是只更新 metadata

#### Scenario: facts 变更不直接跳过 derived 层
- **WHEN** 某个源码文件或 symbol graph 发生变化，并影响既有 `KnowledgeUnit`
- **THEN** `update` MUST 先使对应 research/derived 进入 stale
- **THEN** 系统 MUST NOT 直接重写页面投影并把该动作伪装成 knowledge-first refresh

### Requirement: `update` 必须在提交阶段同步刷新 health signals
系统 MUST 让 `update` 在提交 formal artifacts 与 projection anchors 时同步刷新 knowledge health signals。任何因本次更新被修复、延续或新引入的 health signal，MUST 在同一 snapshot 中更新，而不是留到后续被动诊断。

#### Scenario: 局部 update 修复 stale signal
- **WHEN** 某次 `update` 成功刷新了原先 stale 的 derived summary 与 projection
- **THEN** 对应的 `derived_stale` 或 `projection_stale` signal MUST 在本次提交中被清除或降级
- **THEN** 新 snapshot MUST 不再继续引用已修复的旧 signal

#### Scenario: update 后仍存在不一致时显式保留 signal
- **WHEN** 某次 `update` 完成后 formal artifacts 与 projection 仍存在可诊断不一致
- **THEN** 系统 MUST 在结果中保留相应 health signal
- **THEN** 系统 MUST 不得把这次执行简单投影为“完全 healthy 的 ready snapshot”
