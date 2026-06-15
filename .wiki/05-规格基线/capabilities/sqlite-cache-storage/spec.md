# sqlite-cache-storage Specification

## Purpose
定义 Repo Wiki SQLite 缓存与状态库的结构、原子性，以及增量 workflow 需要的局部读取能力。
## Requirements
### Requirement: 系统必须使用 SQLite 数据库统一承载缓存和状态数据
系统 MUST 使用单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db` 统一承载运行时状态、增量缓存、symbol graph 和全文索引数据。数据库初始化时 MUST 启用 WAL journal mode（`PRAGMA journal_mode=WAL`）。除了现有的每页缓存表外，数据库 MUST 至少包含以下状态与索引表：`wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、`module_source_map`、`page_source_map`、`page_module_map`、`wiki_relations`、`scan_cache`、`llm_cache`、`symbols`、`edges`、`communities`、`community_members`、`processes`、`process_steps`、`wiki_pages_fts`、`symbols_fts`，以及 knowledge runtime 需要的本地镜像表。系统 MAY 保留 `kv_store` 表壳用于历史遗留测试，但不得继续把它作为运行时事实源或自动升级入口。与此前不同的是，SQLite 数据库在本轮后 MUST 被明确定位为本地 cache / state substrate，而不是 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 的替代品。与迭代 7 不同的是，`symbols`、`edges`、`communities`、`community_members`、`processes` 和 `process_steps` 在本迭代起 MUST 承载真实的 symbol graph 与 graph-derived rows，而不再只是空 schema。

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
- **WHEN** 本地 SQLite 数据库缺失，但 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 可读
- **THEN** 系统 MUST 允许先执行 restore 重建本地数据库
- **THEN** 系统 MUST NOT 把 SQLite 缺失本身等同于必须重新 full `init`

### Requirement: 所有写入操作必须使用事务保证原子性
系统 MUST 在单次 workflow 执行中使用 SQLite 事务包裹所有状态表、symbol 表、graph 表、缓存表和 FTS 表的写入，确保 `WikiState`、扫描缓存、模块映射、页面缓存、`symbols`、`edges`、`communities`、`processes`、`wiki_pages_fts` 和 `symbols_fts` 保持一致。

#### Scenario: init 或 rebuild 的写入原子性
- **WHEN** `init` 或 `rebuild` 完成页面生成、symbol parsing、symbol resolution 和 graph analysis
- **THEN** 页面状态表、section 表、源码表、模块表、映射表、扫描缓存、symbol 表、graph 表、每页缓存、`wiki_pages_fts` 和 `symbols_fts` 的写入 MUST 在同一事务中完成
- **THEN** 如果任何写入失败，所有写入 MUST 回滚

#### Scenario: update 的 graph 刷新原子性
- **WHEN** `update` 完成增量页面重建、symbol/edge 刷新和 graph-derived 结果重算
- **THEN** 受影响页面的状态行、section 行、页面缓存、symbol rows、edge rows、community/process rows、`wiki_pages_fts` 和 `symbols_fts` 更新 MUST 在同一事务中完成
- **THEN** 系统不得出现页面正文已更新但 graph tables 仍是旧值的部分成功状态

### Requirement: SQLite 必须支持增量 workflow 所需的文件级 symbol 与 edge 读取
系统 MUST 为增量 workflow 提供按文件路径集合读取 `symbols` 与 `edges` 的能力，而不是只暴露全量枚举接口。文件级读取能力 MUST 能支撑 `update` 组装局部 symbol/edge 工作集，并允许继续向外扩展必要的一跳 dependents 或 graph frontier。相关查询不得破坏现有事务一致性要求。

#### Scenario: 按文件集合读取 symbol rows
- **WHEN** `update` 需要刷新一组受影响源码文件的 symbol snapshot
- **THEN** 系统 MUST 能只读取这些文件对应的 `symbols` rows
- **THEN** 结果中不得混入与该文件集合无关的 symbol rows

#### Scenario: 按文件集合读取 edge rows 并补充必要 frontier
- **WHEN** `update` 需要为一组受影响源码文件重建局部 graph 工作集
- **THEN** 系统 MUST 能读取这些文件相关的 `edges` rows，并补充后续合并所需的必要 frontier
- **THEN** 系统不得要求任何小范围更新都先执行一次无条件全表 edge 枚举

### Requirement: SQLite knowledge 表必须是正式知识产物的本地镜像与恢复目标
系统 MUST 将 `.wiki/.cache/wiki-cache.db` 中的 knowledge 相关表视为本地 working cache、查询加速层或恢复目标，而不是正式可共享 truth。对于 `knowledge_domains`、`knowledge_units`、`research_cache`、`page_digests` 和等价 knowledge 表，正式可共享对象 MUST 位于 `.wiki/.knowledge/**`；SQLite 中的对应 rows MUST 能由这些正式产物重建。

#### Scenario: `.knowledge` 正式产物可重建 SQLite knowledge 表
- **WHEN** 本地 `.wiki/.cache/wiki-cache.db` 缺失或其中 knowledge 相关表被清理，但 `.wiki/.knowledge/**` 仍然可读
- **THEN** 系统 MUST 能基于 `.wiki/.knowledge/**` 重建对应 SQLite knowledge rows
- **THEN** 系统 MUST NOT 要求重新执行 planning、research、compose 或 assemble 才能恢复这些本地表

