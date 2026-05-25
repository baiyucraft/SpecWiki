## ADDED Requirements

### Requirement: runtime 必须只拥有 projection、lifecycle、transport 与 storage adapter
系统 MUST 让 `wiki-runtime` 的正式职责收敛为 workflow orchestration、projection/render、managed section merge、storage adapter、transport、query route 骨架与 lifecycle。`wiki-runtime` MUST NOT 再拥有 scanner、symbol graph、knowledge planning、research engine 或 compose engine 的主实现。

#### Scenario: runtime 只保留 projection 与 lifecycle 主实现
- **WHEN** 开发者检查 `wiki-runtime` crate 的模块边界
- **THEN** runtime MUST 包含 renderer、managed section、page merge、storage adapter、transport、workflow 与 lifecycle 逻辑
- **THEN** runtime MUST NOT 直接承载 facts/index 或 knowledge planning/research/compose 的主实现

### Requirement: runtime 的 SQLite 存储必须按 index、knowledge、runtime 三段分治
系统 MUST 在 `wiki-runtime` 中把 SQLite 具体实现拆成 `index_store`、`knowledge_store` 和 `runtime_store` 三段。`wiki-index` 与 `wiki-knowledge` 必须各自定义自己需要的 store trait，`wiki-runtime` 只实现这些 trait。系统 MUST 明确每张表的 schema contract owner、读写 API owner 与 truth kind，避免跨层直接读库。

#### Scenario: index facts 通过 index_store 持久化
- **WHEN** 系统持久化或读取 `modules`、`symbols`、`edges` 或 graph analysis 相关数据
- **THEN** 这些表 MUST 归 `wiki-index` 合同所有
- **THEN** 读写 API MUST 由 `wiki-index` 定义 trait、由 `wiki-runtime::storage::sqlite::index_store` 实现
- **THEN** 这些数据 MUST 被视为 facts/index formal snapshot

#### Scenario: knowledge cache 通过 knowledge_store 持久化
- **WHEN** 系统持久化或读取 `research_cache`、`page_digests` 或 `page_drafts`
- **THEN** 这些表 MUST 归 `wiki-knowledge` 合同所有
- **THEN** 读写 API MUST 由 `wiki-knowledge` 定义 trait、由 `wiki-runtime::storage::sqlite::knowledge_store` 实现
- **THEN** 这些数据 MUST 被视为 derived knowledge cache 或 compose artifact，而不是 runtime 真相

#### Scenario: runtime projection 与 lifecycle 通过 runtime_store 持久化
- **WHEN** 系统持久化或读取 `wiki_pages`、`wiki_page_sections`、`wiki_relations`、`runtime_meta`、`pipeline_checkpoint` 或 `unit_runtime_gates`
- **THEN** 这些表 MUST 归 `wiki-runtime` 合同所有
- **THEN** 系统 MUST 把 `wiki_pages` 视为 runtime projection truth，把 `pipeline_checkpoint` 与 `unit_runtime_gates` 视为 lifecycle truth
- **THEN** 系统 MUST NOT 让 `page_drafts` 或 `page_digests` 充当 runtime 正式状态

