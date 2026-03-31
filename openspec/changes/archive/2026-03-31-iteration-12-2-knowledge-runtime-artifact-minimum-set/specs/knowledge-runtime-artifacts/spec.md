## ADDED Requirements

### Requirement: `.wiki/.knowledge/**` 必须落最小正式知识产物集
系统 MUST 将 `v0.2.0` knowledge runtime 的最小正式产物写入 `.wiki/.knowledge/**`，并按 truth kind 分层，而不是继续只把这些对象留在 SQLite 或 `.wiki/.cache/**`。最小正式产物集 MUST 至少覆盖以下对象：

- `.wiki/.knowledge/derived/knowledge-domains.json`
- `.wiki/.knowledge/derived/knowledge-units.jsonl`
- `.wiki/.knowledge/derived/knowledge-tree.json`
- `.wiki/.knowledge/derived/research-summaries.jsonl`
- `.wiki/.knowledge/runtime/page-digests.jsonl`
- `.wiki/.knowledge/runtime/runtime-gates.jsonl`
- `.wiki/.knowledge/runtime/recovery-manifest.json`

其中：

- `knowledge-domains / knowledge-units / knowledge-tree` MUST 被视为 formal identity objects
- `research-summaries` MUST 被视为 formal derived summaries
- `page-digests / runtime-gates / recovery-manifest` MUST 被视为 projection / recovery anchors

#### Scenario: 正式 workflow 写出 identity 与 summary 对象
- **WHEN** 系统完成正式 `init`、`update` 或 `rebuild` 的 knowledge planning、research 与 compose 主链
- **THEN** `.wiki/.knowledge/derived/**` MUST 写出 `knowledge-domains`、`knowledge-units`、`knowledge-tree` 与 `research-summaries`
- **THEN** 系统 MUST NOT 继续只把这些对象留在 SQLite `knowledge_store`

#### Scenario: 正式 workflow 写出 projection 与 recovery anchor
- **WHEN** 系统完成页面投影和 runtime gate 汇总
- **THEN** `.wiki/.knowledge/runtime/**` MUST 写出 `page-digests`、`runtime-gates` 与 `recovery-manifest`
- **THEN** 系统 MUST 明确这些对象是 projection / recovery anchor，而不是 knowledge formal identity

### Requirement: 正式知识产物必须与 metadata 共享稳定 snapshot 身份
系统 MUST 让 `.wiki/.knowledge/**` 与 `wiki.metadata.json` 共享稳定的 snapshot 身份线索，用于审计、恢复和一致性校验。`recovery-manifest` MUST 至少包含 `schema_version`、`repo_root`、`facts_input_hash`、`knowledge_snapshot_id`、`metadata_hash` 或等价锚点字段。系统 MUST 能基于这些字段判断当前正式产物是否可用于 restore，而不是依赖目录存在与否盲猜。

#### Scenario: restore 前校验正式产物快照一致性
- **WHEN** 系统尝试基于 `.wiki/.knowledge/** + pages + metadata` 恢复本地 runtime
- **THEN** 系统 MUST 先校验 `recovery-manifest` 与 `wiki.metadata.json` 的 snapshot 锚点是否一致
- **THEN** 若锚点不一致，系统 MUST 返回显式 `stale`、`needs_update` 或 blocker，而不是静默继续恢复

### Requirement: restore 必须从正式产物恢复本地 runtime，而不是重新生成知识
系统 MUST 支持在 `.wiki/.cache/**` 缺失、被删除或需要重建时，仅凭 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复本地 runtime。该 restore 语义 MUST 是“重建本地 `.cache` 与可消费状态”，而不是重新执行 planning、research、compose 或 assemble。系统 MUST NOT 因 `.cache` 缺失就自动退回 full `init` 并把它伪装成 restore 成功。

#### Scenario: B 用户从已上库正式产物恢复本地 `.cache`
- **WHEN** 仓库中已存在可读取的 `.wiki/.knowledge/**`、`.wiki/pages/**` 与 `wiki.metadata.json`，但本地 `.wiki/.cache/**` 缺失
- **THEN** 系统 MUST 仅基于这些正式产物重建本地 `.wiki/.cache/**`
- **THEN** restore MUST NOT 重新调用 planning、research 或 compose 主链

#### Scenario: 正式产物不可恢复时返回显式状态
- **WHEN** `.wiki/.knowledge/**` 缺失关键文件、`recovery-manifest` 不可读，或正式产物之间的锚点不一致
- **THEN** 系统 MUST 返回显式 blocker 或 `needs_update`
- **THEN** 系统 MUST NOT 静默退回 full `init` 并把该结果标记为 restore 成功

### Requirement: 最小正式知识产物集不得混入工作态缓存与 declared lifecycle
系统 MUST 让 `.wiki/.knowledge/**` 保持最小正式知识产物定位。本轮正式 contract MUST 只覆盖 `derived/**` 与 `runtime/**` 的最小对象集；`declared/**` MAY 保留为空目录或占位，但 MUST NOT 在本轮被定型为正式 lifecycle contract。`page_drafts`、`llm_cache`、临时 session state、工作中的 compose artifact 和 declared knowledge authoring 状态 MUST NOT 在本轮进入 `.wiki/.knowledge/**` 的正式 contract。

#### Scenario: 工作态缓存继续留在 `.cache`
- **WHEN** 系统持久化 `page_drafts`、`llm_cache`、临时 session state 或其它工作态缓存
- **THEN** 这些对象 MUST 继续留在 `.wiki/.cache/**` 或等价本地 working state
- **THEN** 系统 MUST NOT 把它们升级为 `.wiki/.knowledge/**` 的正式对象
