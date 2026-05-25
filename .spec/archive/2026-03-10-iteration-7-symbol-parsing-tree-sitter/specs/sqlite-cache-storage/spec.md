## MODIFIED Requirements

### Requirement: 系统必须使用 SQLite 数据库统一承载缓存和状态数据
系统 MUST 使用单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db` 统一承载运行时状态、增量缓存和全文索引数据。数据库初始化时 MUST 启用 WAL journal mode（`PRAGMA journal_mode=WAL`）。除了现有的每页缓存表外，数据库 MUST 至少包含以下状态与索引表：`wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、`module_source_map`、`page_source_map`、`page_module_map`、`wiki_relations`、`scan_cache`、`llm_cache`、`symbols`、`edges`、`communities`、`community_members`、`processes`、`process_steps`、`wiki_pages_fts`、`symbols_fts`。系统 MAY 保留 `kv_store` 表壳用于历史遗留测试，但不得继续把它作为运行时事实源或自动升级入口。与 6.5 不同的是，`symbols` 和 `symbols_fts` 在迭代 7 起 MUST 承载真实的定义类符号节点和全文索引，而不再只是空 schema；`edges`、`communities` 和 `processes` 在本迭代可以继续为空表。

#### Scenario: 初始化时创建关系型 schema
- **WHEN** 系统首次执行 `init` 且 `.wiki/.cache/wiki-cache.db` 不存在
- **THEN** 系统 MUST 创建数据库文件并初始化全部关系型状态表、缓存表和 FTS5 虚拟表
- **THEN** 数据库 MUST 至少包含 `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、映射表以及 `wiki_pages_fts`
- **THEN** 如果仓库中存在受支持源码文件，系统 MUST 在首次 `init` 后向 `symbols` 和 `symbols_fts` 写入对应 symbol rows

#### Scenario: 数据库已存在但 schema 不完整
- **WHEN** 系统执行 `init`、`update`、`sync` 或 `rebuild` 且数据库缺少关键状态表、映射表或索引表
- **THEN** 系统 MUST 在打开现有数据库后补齐当前 schema
- **THEN** 若关键状态记录仍无法恢复，系统 MUST 走 metadata / rebuild 回退，而不是尝试旧版 DB 自动迁移

### Requirement: 所有写入操作必须使用事务保证原子性
系统 MUST 在单次 workflow 执行中使用 SQLite 事务包裹所有状态表、symbol 表、缓存表和 FTS 表的写入，确保 `WikiState`、扫描缓存、模块映射、页面缓存、`symbols`、`wiki_pages_fts` 和 `symbols_fts` 保持一致。

#### Scenario: init 的写入原子性
- **WHEN** `init` 完成所有页面生成、symbol parsing 和状态装配
- **THEN** 页面状态表、section 表、源码表、模块表、映射表、扫描缓存、symbol 表、每页缓存、`wiki_pages_fts` 和 `symbols_fts` 的写入 MUST 在同一事务中完成
- **THEN** 如果任何写入失败，所有写入 MUST 回滚

#### Scenario: update 的写入原子性
- **WHEN** `update` 完成增量页面重建和受影响文件的 symbol 重解析
- **THEN** 受影响页面的状态行、section 行、页面缓存、symbol rows、`wiki_pages_fts` 和 `symbols_fts` 更新 MUST 在同一事务中完成
- **THEN** 系统不得出现页面正文已更新但 symbol 索引仍是旧值的部分成功状态
