# knowledge-runtime-artifacts Specification

## Purpose
定义可上库 formal knowledge artifacts、committed snapshot identity 与 cold restore 边界。
## Requirements
### Requirement: `.wiki/.knowledge/**` 必须落最小正式知识产物集
系统 MUST 将 `v0.2.0` knowledge runtime 的最小正式产物写入 `.wiki/.knowledge/**`，并按 truth kind 分层，而不是继续只把这些对象留在 SQLite 或 `.wiki/.cache/**`。最小正式产物集 MUST 至少覆盖以下对象：

- `.wiki/.knowledge/derived/knowledge-domains.json`
- `.wiki/.knowledge/derived/knowledge-units.jsonl`
- `.wiki/.knowledge/derived/knowledge-tree.json`
- `.wiki/.knowledge/derived/research-summaries.jsonl`
- `.wiki/.knowledge/declared/records.jsonl`
- `.wiki/.knowledge/runtime/page-digests.jsonl`
- `.wiki/.knowledge/runtime/projection-digests.jsonl`
- `.wiki/.knowledge/runtime/conflict-records.jsonl`
- `.wiki/.knowledge/runtime/runtime-gates.jsonl`
- `.wiki/.knowledge/runtime/health-signals.jsonl`
- `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml`

其中：

- `knowledge-domains / knowledge-units / knowledge-tree` MUST 被视为 formal identity objects
- `research-summaries` MUST 被视为 formal derived summaries
- `declared records` MUST 被视为 formal declared truth
- `page-digests / projection-digests / conflict-records / runtime-gates / health-signals / committed snapshot manifest` MUST 被视为 projection、governance health 或 recovery anchors

#### Scenario: 正式 workflow 写出 identity 与 summary 对象
- **WHEN** 系统完成正式 `init`、`update` 或 `rebuild` 的 knowledge planning、research 与 compose 主链
- **THEN** `.wiki/.knowledge/derived/**` MUST 写出 `knowledge-domains`、`knowledge-units`、`knowledge-tree` 与 `research-summaries`
- **THEN** 系统 MUST NOT 继续只把这些对象留在 SQLite `knowledge_store`

#### Scenario: 正式 workflow 写出 projection 与 recovery anchor
- **WHEN** 系统完成页面投影和 runtime gate 汇总
- **THEN** `.wiki/.knowledge/declared/**` MUST 写出 `records.jsonl`
- **THEN** `.wiki/.knowledge/runtime/**` MUST 写出 `page-digests`、`projection-digests`、`conflict-records`、`runtime-gates`、`health-signals`，并在 `snapshots/<snapshot-id>/manifest.yaml` 写出 committed snapshot manifest
- **THEN** 系统 MUST 明确这些对象是 projection / recovery anchor，而不是 knowledge formal identity

### Requirement: 正式知识产物必须与 metadata 共享稳定 snapshot 身份
系统 MUST 让 `.wiki/.knowledge/**` 与 `wiki.metadata.json` 共享稳定的 snapshot 身份线索，用于审计、恢复和一致性校验。loader MUST 只读取 `wiki.metadata.json.current_snapshot_id` 精确指向的 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml`，不得按目录名、修改时间或任意“最新”启发式选择。`CommittedSnapshotManifest` MUST 至少包含 `schema_version`、`snapshot_id`、`repo_root`、`facts_input_hash`、graph/knowledge/declared/projection snapshot identity、`metadata_hash`、page hashes 和 projection/runtime refs；manifest 内 `snapshot_id` MUST 与 metadata pointer 一致，否则 fail closed。

#### Scenario: restore 前校验正式产物快照一致性
- **WHEN** 系统尝试基于 `.wiki/.knowledge/** + official page tree + metadata` 恢复本地 runtime
- **THEN** 系统 MUST 先校验 committed snapshot manifest 与 `wiki.metadata.json` 的 snapshot 锚点是否一致
- **THEN** 若锚点不一致，系统 MUST 返回显式 `stale`、`needs_update` 或 blocker，而不是静默继续恢复

### Requirement: restore 必须从正式产物恢复本地 runtime，而不是重新生成知识
系统 MUST 支持在 `.wiki/.cache/**` 缺失、被删除或需要重建时，仅凭 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 恢复本地 runtime。official page tree 只包括 `.wiki/INDEX.md`、`.wiki/<栏目路径>/INDEX.md` 和 `.wiki/<栏目路径>/NN-主题.md`。该 restore 语义 MUST 是“重建本地 `.cache` 与可消费状态”，而不是重新执行 planning、research、compose 或 assemble。系统 MUST NOT 因 `.cache` 缺失就自动退回 full `init` 并把它伪装成 restore 成功。

#### Scenario: B 用户从已上库正式产物恢复本地 `.cache`
- **WHEN** 仓库中已存在可读取的 `.wiki/.knowledge/**`、official page tree 与 `wiki.metadata.json`，但本地 `.wiki/.cache/**` 缺失
- **THEN** 系统 MUST 仅基于这些正式产物重建本地 `.wiki/.cache/**`
- **THEN** restore MUST NOT 重新调用 planning、research 或 compose 主链

#### Scenario: 正式产物不可恢复时返回显式状态
- **WHEN** `.wiki/.knowledge/**` 缺失关键文件、committed snapshot manifest 不可读，或正式产物之间的锚点不一致
- **THEN** 系统 MUST 返回显式 blocker 或 `needs_update`
- **THEN** 系统 MUST NOT 静默退回 full `init` 并把该结果标记为 restore 成功

### Requirement: 最小正式知识产物集不得混入工作态缓存
系统 MUST 让 `.wiki/.knowledge/**` 保持正式知识产物定位。`declared/**`、`derived/**` 与 `runtime/**` 都属于正式 snapshot 的组成部分，但 `page_drafts`、`llm_cache`、临时 session state、工作中的 compose artifact 和其它本地工作态缓存 MUST NOT 进入 `.wiki/.knowledge/**`。

#### Scenario: 工作态缓存继续留在 `.cache`
- **WHEN** 系统持久化 `page_drafts`、`llm_cache`、临时 session state 或其它工作态缓存
- **THEN** 这些对象 MUST 继续留在 `.wiki/.cache/**` 或等价本地 working state
- **THEN** 系统 MUST NOT 把它们升级为 `.wiki/.knowledge/**` 的正式对象

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
系统 MUST 在 `.wiki/.knowledge/**` 的正式产物集中补充 `declared/**` 与 health artifacts，而不再只稳定承诺 `derived/**` 与 `runtime/**`。最小补充对象 MUST 至少包括：结构化 declared records、declared snapshot identity、declared conflict / lifecycle diagnostics，以及可供 `status` / `query` / `sync` 消费的 health signals 摘要。

#### Scenario: declared artifact 成为正式 snapshot 组成部分
- **WHEN** 仓库中存在至少一条正式 declared record
- **THEN** `.wiki/.knowledge/**` 的正式 snapshot MUST 包含对应 declared artifact
- **THEN** restore 或 audit MUST 能直接消费该对象，而不依赖页面正文反推

#### Scenario: health artifact 进入正式 runtime snapshot
- **WHEN** 当前 runtime 检测到 orphan unit、stale projection 或 declared/derived 不一致
- **THEN** `.wiki/.knowledge/runtime/**` 或等价正式层 MUST 写出可聚合的 health artifact
- **THEN** 系统 MUST NOT 只把这些问题留在临时日志或内存态

