## ADDED Requirements

### Requirement: `KnowledgeUnit` 正式产物必须携带最小合同字段
系统 MUST 让 `.wiki/.knowledge/derived/knowledge-units.jsonl` 或等价正式 artifact 中的每个 `KnowledgeUnit` 携带最小合同字段。该最小合同 MUST 至少覆盖 `unit_id`、`domain_id`、`unit_kind`、`declared_record_refs`、`derived_research_ref`、`projection_refs`、`source_refs`、`citation_refs`、`status`、`updated_at` 与 `invalidation_reason`。系统 MUST NOT 继续把 `KnowledgeUnit` 写成只够 planner 内部消费的弱对象。

#### Scenario: unit artifact 具备跨层引用能力
- **WHEN** 系统写出某个正式 `KnowledgeUnit`
- **THEN** 该对象 MUST 能同时引用其 declared、derived 与 projection 关联对象
- **THEN** 调用方 MUST 不需要反查最终 Markdown 才能恢复这些关系

#### Scenario: unit artifact 显式记录失效原因
- **WHEN** 某个 `KnowledgeUnit` 因 declared 变更、facts/index 变更或 projection 冲突进入 stale 状态
- **THEN** 对应正式 unit artifact MUST 记录 `status` 与 `invalidation_reason`
- **THEN** 系统 MUST NOT 仅靠 cache 缺失间接表达该状态

### Requirement: 正式知识产物必须补充 declared 与 health artifacts
系统 MUST 在 `.wiki/.knowledge/**` 的最小正式产物集中补充 `declared/**` 与 health artifacts，而不再只稳定承诺 `derived/**` 与 `runtime/**`。最小补充对象 MUST 至少包括：结构化 `declared records` 与可供 `status` / `query` / `sync` 诊断消费的 `health signals` 摘要。

#### Scenario: declared artifact 成为正式 snapshot 组成部分
- **WHEN** 仓库中存在至少一条正式 declared record
- **THEN** `.wiki/.knowledge/**` 的正式 snapshot MUST 包含对应 declared artifact
- **THEN** restore 或 audit MUST 能直接消费该对象，而不依赖页面正文反推

#### Scenario: health artifact 进入正式 runtime snapshot
- **WHEN** 当前 runtime 检测到 orphan unit、stale projection 或 declared/derived 不一致
- **THEN** `.wiki/.knowledge/runtime/**` 或等价正式层 MUST 写出可聚合的 health artifact
- **THEN** 系统 MUST NOT 只把这些问题留在临时日志或内存态
