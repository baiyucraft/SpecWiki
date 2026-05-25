## ADDED Requirements

### Requirement: runtime 必须把 `.knowledge / pages / metadata / cache` 作为正式分层
系统 MUST 将 `.wiki/.knowledge/**`、`.wiki/pages/**`、`wiki.metadata.json` 与 `.wiki/.cache/**` 视为不同 truth kind 的正式分层，而不是继续让 SQLite 或 `.cache` 充当隐性主真相。`.wiki/.knowledge/**` MUST 承载可上库的 formal knowledge artifacts，`.wiki/pages/**` MUST 承载 page projection，`wiki.metadata.json` MUST 承载正式索引与恢复入口，`.wiki/.cache/**` MUST 只承载本地 working state 与可重建缓存。

#### Scenario: runtime 写盘时保持四层职责分离
- **WHEN** 系统执行正式 `init`、`update` 或 `rebuild`
- **THEN** `.wiki/.knowledge/**` MUST 只写入 formal knowledge artifacts 与 recovery anchors
- **THEN** `.wiki/pages/**` MUST 只写入 page projection
- **THEN** `.wiki/.cache/**` MUST 只写入本地 working state
- **THEN** 系统 MUST NOT 把 `.cache` 或 SQLite 继续当成唯一正式 knowledge 真相

### Requirement: runtime 必须支持从正式产物恢复本地 cache 与可消费状态
系统 MUST 支持在 `.wiki/.cache/**` 缺失或需要重建时，从 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复本地 cache 与 runtime 可消费状态。恢复完成后，`status` MUST 能表达当前仓库是 `ready`、`stale`、`needs_update` 还是 `blocker`；当前 `query` 入口 MUST 能消费恢复后的 runtime，而不是强制要求先执行 full `init`。该恢复成功语义 MUST 只表示“本地 cache 已恢复且 runtime 可被消费与诊断”，MUST NOT 被表述成“完整 wiki runtime 已 ready”。

#### Scenario: cold restore 后 `status` 可直接消费恢复态 runtime
- **WHEN** 系统基于正式产物完成本地 `.wiki/.cache/**` 重建
- **THEN** `status` MUST 能直接读取恢复出的 runtime 状态
- **THEN** 若当前代码与正式产物不一致，`status` MUST 返回 `stale`、`needs_update` 或 blocker，而不是伪装成全新 fresh runtime

#### Scenario: cold restore 后当前 `query` 入口无需 full init
- **WHEN** 系统已基于正式产物恢复出本地 runtime
- **THEN** 当前 `query` 入口 MUST 能消费该恢复态 runtime
- **THEN** 系统 MUST NOT 把“缺失 `.cache`”本身当成必须 full `init` 的理由

## MODIFIED Requirements

### Requirement: runtime 的 SQLite 存储必须按 index、knowledge、runtime 三段分治
系统 MUST 在 `wiki-runtime` 中把 SQLite 具体实现拆成 `index_store`、`knowledge_store` 和 `runtime_store` 三段。`wiki-index` 与 `wiki-knowledge` 必须各自定义自己需要的 store trait，`wiki-runtime` 只实现这些 trait。系统 MUST 明确每张表的 schema contract owner、读写 API owner 与 truth kind，避免跨层直接读库。与此前不同的是，`knowledge_store` 中的 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 等对象在本轮后 MUST 被视为本地 working cache、加速索引或 `.wiki/.knowledge/**` 的 rebuild target，而不是唯一正式 knowledge truth。正式可共享 truth MUST 由 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 承载。

#### Scenario: index facts 通过 index_store 持久化
- **WHEN** 系统持久化或读取 `modules`、`symbols`、`edges` 或 graph analysis 相关数据
- **THEN** 这些表 MUST 归 `wiki-index` 合同所有
- **THEN** 读写 API MUST 由 `wiki-index` 定义 trait、由 `wiki-runtime::storage::sqlite::index_store` 实现
- **THEN** 这些数据 MUST 被视为 facts/index formal snapshot

#### Scenario: knowledge cache 通过 knowledge_store 持久化
- **WHEN** 系统持久化或读取 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 或 `page_drafts`
- **THEN** 这些表 MUST 归 `wiki-knowledge` 合同所有
- **THEN** 读写 API MUST 由 `wiki-knowledge` 定义 trait、由 `wiki-runtime::storage::sqlite::knowledge_store` 实现
- **THEN** 这些数据 MUST 被视为本地 working cache、derived knowledge cache 或 compose artifact，而不是唯一正式 knowledge truth

#### Scenario: formal knowledge artifact 可重建 knowledge_store
- **WHEN** 本地 `knowledge_store` 缺失或被清理，但 `.wiki/.knowledge/**` 仍然可读
- **THEN** 系统 MUST 能基于正式 knowledge artifact 重建对应的本地 `knowledge_store`
- **THEN** 系统 MUST NOT 依赖重新执行 planning、research 或 compose 才能恢复这些本地表

#### Scenario: runtime projection 与 lifecycle 通过 runtime_store 持久化
- **WHEN** 系统持久化或读取 `wiki_pages`、`wiki_page_sections`、`wiki_relations`、`runtime_meta`、`pipeline_checkpoint` 或 `unit_runtime_gates`
- **THEN** 这些表 MUST 归 `wiki-runtime` 合同所有
- **THEN** 系统 MUST 把 `wiki_pages` 视为 runtime projection truth，把 `pipeline_checkpoint` 与 `unit_runtime_gates` 视为 lifecycle truth
- **THEN** 系统 MUST NOT 让 `page_drafts` 或 `page_digests` 充当 runtime 正式状态
