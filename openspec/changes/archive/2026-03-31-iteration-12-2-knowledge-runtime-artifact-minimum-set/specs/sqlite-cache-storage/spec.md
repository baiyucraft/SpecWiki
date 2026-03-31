## ADDED Requirements

### Requirement: SQLite knowledge 表必须是正式知识产物的本地镜像与恢复目标
系统 MUST 将 `.wiki/.cache/wiki-cache.db` 中的 knowledge 相关表视为本地 working cache、查询加速层或恢复目标，而不是正式可共享 truth。对于 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 和等价 knowledge 表，正式可共享对象 MUST 位于 `.wiki/.knowledge/**`；SQLite 中的对应 rows MUST 能由这些正式产物重建。

#### Scenario: `.knowledge` 正式产物可重建 SQLite knowledge 表
- **WHEN** 本地 `.wiki/.cache/wiki-cache.db` 缺失或其中 knowledge 相关表被清理，但 `.wiki/.knowledge/**` 仍然可读
- **THEN** 系统 MUST 能基于 `.wiki/.knowledge/**` 重建对应 SQLite knowledge rows
- **THEN** 系统 MUST NOT 要求重新执行 planning、research、compose 或 assemble 才能恢复这些本地表

## MODIFIED Requirements

### Requirement: 系统必须使用 SQLite 数据库统一承载缓存和状态数据
系统 MUST 使用单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db` 统一承载运行时状态、增量缓存、symbol graph 和全文索引数据。数据库初始化时 MUST 启用 WAL journal mode（`PRAGMA journal_mode=WAL`）。除了现有的每页缓存表外，数据库 MUST 至少包含以下状态与索引表：`wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、`module_source_map`、`page_source_map`、`page_module_map`、`wiki_relations`、`scan_cache`、`llm_cache`、`symbols`、`edges`、`communities`、`community_members`、`processes`、`process_steps`、`wiki_pages_fts`、`symbols_fts`，以及 knowledge runtime 需要的本地镜像表。系统 MAY 保留 `kv_store` 表壳用于历史遗留测试，但不得继续把它作为运行时事实源或自动升级入口。与此前不同的是，SQLite 数据库在本轮后 MUST 被明确定位为本地 cache / state substrate，而不是 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 的替代品。与迭代 7 不同的是，`symbols`、`edges`、`communities`、`community_members`、`processes` 和 `process_steps` 在本迭代起 MUST 承载真实的 symbol graph 与 graph-derived rows，而不再只是空 schema。

#### Scenario: 初始化时创建关系型 schema
- **WHEN** 系统首次执行 `init` 且 `.wiki/.cache/wiki-cache.db` 不存在
- **THEN** 系统 MUST 创建数据库文件并初始化全部关系型状态表、缓存表、graph 表和 FTS5 虚拟表
- **THEN** 数据库 MUST 至少包含 `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、映射表、`symbols`、`edges`、`communities`、`processes`、`wiki_pages_fts` 和 `symbols_fts`
- **THEN** 如果仓库中存在可解析的 symbol graph，系统 MUST 在首次 `init` 后向这些 graph 表写入真实 rows

#### Scenario: 数据库已存在但 schema 不完整
- **WHEN** 系统执行 `init`、`update`、`sync` 或 `rebuild` 且数据库缺少关键状态表、graph 表或索引表
- **THEN** 系统 MUST 在打开现有数据库后补齐当前 schema
- **THEN** 若关键状态记录仍无法恢复，系统 MUST 走 metadata / rebuild 回退，而不是尝试旧版 DB 自动迁移

#### Scenario: cache 缺失时可由正式产物重建
- **WHEN** 本地 SQLite 数据库缺失，但 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 可读
- **THEN** 系统 MUST 允许先执行 restore 重建本地数据库
- **THEN** 系统 MUST NOT 把 SQLite 缺失本身等同于必须重新 full `init`
