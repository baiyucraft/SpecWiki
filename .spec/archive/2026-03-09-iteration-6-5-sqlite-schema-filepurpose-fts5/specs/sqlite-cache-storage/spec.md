## MODIFIED Requirements

### Requirement: 系统必须使用 SQLite 数据库统一承载缓存和状态数据
系统 MUST 使用单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db` 统一承载运行时状态、增量缓存和全文索引数据。数据库初始化时 MUST 启用 WAL journal mode（`PRAGMA journal_mode=WAL`）。除了现有的每页缓存表外，数据库 MUST 至少包含以下状态与索引表：`wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、`module_source_map`、`page_source_map`、`page_module_map`、`wiki_relations`、`scan_cache`、`llm_cache`、`symbols`、`edges`、`communities`、`community_members`、`processes`、`process_steps`、`wiki_pages_fts`、`symbols_fts`。系统 MAY 保留 `kv_store` 表壳用于历史遗留测试，但不得继续把它作为运行时事实源或自动升级入口。

#### Scenario: 初始化时创建关系型 schema
- **WHEN** 系统首次执行 `init` 且 `.wiki/.cache/wiki-cache.db` 不存在
- **THEN** 系统 MUST 创建数据库文件并初始化全部关系型状态表、缓存表和 FTS5 虚拟表
- **THEN** 数据库 MUST 至少包含 `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、映射表以及 `wiki_pages_fts`

#### Scenario: 数据库已存在但 schema 不完整
- **WHEN** 系统执行 `init`、`update`、`sync` 或 `rebuild` 且数据库缺少关键状态表、映射表或索引表
- **THEN** 系统 MUST 在打开现有数据库后补齐当前 schema
- **THEN** 若关键状态记录仍无法恢复，系统 MUST 走 metadata / rebuild 回退，而不是尝试旧版 DB 自动迁移

### Requirement: WikiState 必须通过 SQLite 持久化
系统 MUST 通过关系型表持久化 `WikiState`，而不是再把整个 `WikiState` 序列化为单条 `kv_store` JSON。页面状态 MUST 写入 `wiki_pages` 与 `wiki_page_sections`，源码状态 MUST 写入 `source_states`，模块状态 MUST 写入 `modules` 与 `module_source_map`，页面-源码与页面-模块关系 MUST 分别写入 `page_source_map` 与 `page_module_map`，结构关系 MUST 写入 `wiki_relations`。读取时系统 MUST 从这些表重新装配 `WikiState`；当关系型状态缺失或数据库无法读取时，MUST 回退到从 `wiki.metadata.json` 重建。

#### Scenario: 写入 WikiState
- **WHEN** 任何 workflow 完成 WikiState 装配后
- **THEN** 系统 MUST 把页面、section、源码、模块、映射和关系拆分写入对应表
- **THEN** 同一 `page_id`、`section_id`、`source_id`、`module_id` 的旧行 MUST 被覆盖或清理为最新状态

#### Scenario: 读取 WikiState
- **WHEN** 系统需要加载 WikiState
- **THEN** 系统 MUST 优先从 `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、映射表和 `wiki_relations` 组装 `WikiState`
- **THEN** 不得再以 `kv_store.wiki-state` 作为主要读取来源

#### Scenario: 遇到仅包含 legacy JSON 键的旧库
- **WHEN** 数据库中只剩旧的 `wiki-state`、`repo-scan` 或 `module-tree` JSON 键，而缺失关系型状态表或关键状态行
- **THEN** 系统 MUST 不把这些 JSON 键视为 6.5 的主要恢复来源
- **THEN** 系统 MUST 回退到 `wiki.metadata.json` 或 `rebuild`

#### Scenario: 数据库损坏时回退
- **WHEN** 数据库文件存在但无法打开或关键状态表查询失败
- **THEN** 系统 MUST 回退到从 `wiki.metadata.json` 重建 WikiState
- **THEN** 系统 MUST 输出 warning 提示数据库损坏或 schema 不完整

### Requirement: 扫描缓存和模块树缓存必须通过 SQLite 持久化
系统 MUST 通过 SQLite 持久化最近一次扫描结果和模块树事实，但不得继续把它们作为 `kv_store` 的主键值对保存。`ScanReport` MUST 写入 `scan_cache` 等专用缓存表；模块树的模块节点与映射 MUST 写入 `modules`、`module_source_map`、`page_module_map` 和 `wiki_relations` 等关系型表，以便后续 `status`、`update` 和 `query` 直接消费。

#### Scenario: 写入扫描缓存
- **WHEN** 系统完成仓库扫描
- **THEN** 系统 MUST 把最近一次 `ScanReport` 写入专用扫描缓存表
- **THEN** 后续 `status` 和 `change_set` MUST 能从该缓存恢复上一轮扫描快照

#### Scenario: 读取扫描缓存
- **WHEN** 系统需要加载上一次扫描结果
- **THEN** 系统 MUST 从专用扫描缓存表读取并反序列化最近一次 `ScanReport`
- **THEN** 不得要求 `kv_store.repo-scan` 仍然存在

#### Scenario: 持久化模块树节点与映射
- **WHEN** 系统完成模块树构建和页面规划
- **THEN** 系统 MUST 把模块节点和模块-源码、页面-模块映射写入关系型表
- **THEN** 模块树的层级与页面映射必须可由数据库记录重建

### Requirement: 所有写入操作必须使用事务保证原子性
系统 MUST 在单次 workflow 执行中使用 SQLite 事务包裹所有状态表、缓存表和 FTS 表的写入，确保 `WikiState`、扫描缓存、模块映射、页面缓存和全文索引保持一致。

#### Scenario: init 的写入原子性
- **WHEN** `init` 完成所有页面生成和状态装配
- **THEN** 页面状态表、section 表、源码表、模块表、映射表、扫描缓存、每页缓存和 `wiki_pages_fts` 的写入 MUST 在同一事务中完成
- **THEN** 如果任何写入失败，所有写入 MUST 回滚

#### Scenario: update 的写入原子性
- **WHEN** `update` 完成增量页面重建
- **THEN** 受影响页面的状态行、section 行、页面缓存和 FTS 索引更新 MUST 在同一事务中完成
- **THEN** 系统不得出现页面正文已更新但状态表或索引仍是旧值的部分成功状态

### Requirement: 缓存布局检查必须适配 SQLite
系统 MUST 把 `has_cache_layout()` 和 `missing_incremental_cache_components()` 的检查逻辑升级为针对数据库 schema 和关键记录的检查。检查内容 MUST 覆盖状态表、section 表、映射表、扫描缓存表和每页缓存表，而不是只检查 `kv_store` 中的键。

#### Scenario: 检查缓存布局完整性
- **WHEN** 系统检查 runtime 缓存是否完整
- **THEN** 系统 MUST 检查 `wiki-cache.db` 是否存在
- **THEN** 系统 MUST 检查 `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、映射表和扫描缓存表是否存在
- **THEN** 系统 MUST 检查每个已知页面在 `page_context_cache`、`page_generation_cache` 和 `wiki_page_sections` 中是否有对应记录

#### Scenario: 关键状态表缺失时触发 rebuild
- **WHEN** 任一关键状态表或页面对应的关键记录缺失，导致当前 runtime 无法恢复页面与 section 映射
- **THEN** 系统 MUST 把 runtime 视为损坏
- **THEN** 后续 `status` / `update` MUST 进入 `needs_rebuild` 路径
