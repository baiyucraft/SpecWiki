## ADDED Requirements

### Requirement: 系统必须使用 SQLite 数据库统一承载缓存和状态数据
系统 MUST 使用单个 SQLite 数据库文件 `.wiki/.cache/wiki-cache.db` 统一承载所有运行时缓存和状态数据，替代当前散落的 JSON 文件。数据库 MUST 包含 `kv_store`（全局键值存储）、`page_context_cache`（每页上下文缓存）和 `page_generation_cache`（每页生成缓存）三张表。数据库初始化时 MUST 启用 WAL journal mode（`PRAGMA journal_mode=WAL`）。

#### Scenario: 初始化时创建数据库
- **WHEN** 系统首次执行 `init` 且 `.wiki/.cache/wiki-cache.db` 不存在
- **THEN** 系统 MUST 创建数据库文件并初始化所有表结构
- **THEN** 数据库 MUST 包含 `kv_store`、`page_context_cache`、`page_generation_cache` 三张表

#### Scenario: 数据库已存在时直接使用
- **WHEN** 系统执行 `init`、`update`、`sync` 或 `rebuild` 且数据库已存在
- **THEN** 系统 MUST 直接打开并使用现有数据库
- **THEN** 系统不得重新创建或清空已有表

### Requirement: WikiState 必须通过 SQLite 持久化
系统 MUST 把 `WikiState` 序列化为 JSON 并存储在 `kv_store` 表中（key = `wiki-state`）。读取时 MUST 从 `kv_store` 反序列化恢复。当数据库不存在或 `wiki-state` 键不存在时，MUST 回退到从 `wiki.metadata.json` 重建。

#### Scenario: 写入 WikiState
- **WHEN** 任何 workflow 完成 WikiState 装配后
- **THEN** 系统 MUST 把 WikiState JSON 写入 `kv_store` 表的 `wiki-state` 键
- **THEN** 写入 MUST 在事务中完成

#### Scenario: 读取 WikiState
- **WHEN** 系统需要加载 WikiState
- **THEN** 系统 MUST 优先从 `kv_store` 表读取 `wiki-state` 键
- **THEN** 如果键不存在或数据库不存在，MUST 回退到从 `wiki.metadata.json` 重建

#### Scenario: 数据库损坏时回退
- **WHEN** 数据库文件存在但无法打开或查询失败
- **THEN** 系统 MUST 回退到从 `wiki.metadata.json` 重建 WikiState
- **THEN** 系统 MUST 输出 warning 提示数据库损坏

### Requirement: 扫描缓存和模块树缓存必须通过 SQLite 持久化
系统 MUST 把 `ScanReport` 和 `ModuleTree` 序列化为 JSON 并存储在 `kv_store` 表中（key 分别为 `repo-scan` 和 `module-tree`）。

#### Scenario: 写入扫描缓存
- **WHEN** 系统完成仓库扫描
- **THEN** 系统 MUST 把 ScanReport JSON 写入 `kv_store` 表的 `repo-scan` 键

#### Scenario: 读取扫描缓存
- **WHEN** 系统需要加载上一次扫描结果
- **THEN** 系统 MUST 从 `kv_store` 表读取 `repo-scan` 键并反序列化

#### Scenario: 写入模块树缓存
- **WHEN** 系统完成模块树构建
- **THEN** 系统 MUST 把 ModuleTree JSON 写入 `kv_store` 表的 `module-tree` 键

### Requirement: 每页缓存必须通过 SQLite 持久化
系统 MUST 把每页上下文缓存和每页生成缓存存储在专用表中，以 `page_id` 为主键。写入和删除 MUST 在事务中完成。

#### Scenario: 写入每页上下文缓存
- **WHEN** 系统为某页面生成上下文
- **THEN** 系统 MUST 把 page_id、input_hash 和 PageContext JSON 写入 `page_context_cache` 表
- **THEN** 如果该 page_id 已存在，MUST 覆盖更新

#### Scenario: 写入每页生成缓存
- **WHEN** 系统为某页面完成生成
- **THEN** 系统 MUST 把 page_id、input_hash、content_hash 和 sections JSON 写入 `page_generation_cache` 表
- **THEN** 如果该 page_id 已存在，MUST 覆盖更新

#### Scenario: 删除某页面的缓存
- **WHEN** 某页面被移除（如 update 中的 removed pages）
- **THEN** 系统 MUST 从 `page_context_cache` 和 `page_generation_cache` 表中删除对应 page_id 的记录

### Requirement: 所有写入操作必须使用事务保证原子性
系统 MUST 在单次 workflow 执行中使用 SQLite 事务包裹所有写入操作，确保 WikiState、缓存和状态数据的一致性。

#### Scenario: init 的写入原子性
- **WHEN** `init` 完成所有页面生成和状态装配
- **THEN** WikiState、扫描缓存、模块树缓存和所有每页缓存的写入 MUST 在同一事务中完成
- **THEN** 如果任何写入失败，所有写入 MUST 回滚

#### Scenario: update 的写入原子性
- **WHEN** `update` 完成增量页面重建
- **THEN** 受影响页面的缓存更新和 WikiState 更新 MUST 在同一事务中完成

### Requirement: 缓存布局检查必须适配 SQLite
系统 MUST 把 `has_cache_layout()` 和 `missing_incremental_cache_components()` 的检查逻辑从文件系统检查迁移到 SQLite 表和键的存在性检查。

#### Scenario: 检查缓存布局完整性
- **WHEN** 系统检查 runtime 缓存是否完整
- **THEN** 系统 MUST 检查 `wiki-cache.db` 是否存在
- **THEN** 系统 MUST 检查 `kv_store` 表中 `wiki-state`、`repo-scan`、`module-tree` 键是否存在
- **THEN** 系统 MUST 检查每个已知页面在 `page_context_cache` 和 `page_generation_cache` 表中是否有记录

### Requirement: rebuild 时必须清理并重建数据库
系统 MUST 在 `rebuild` 时删除旧的 `wiki-cache.db` 并重新创建，确保不残留过期数据。

#### Scenario: rebuild 清理数据库
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统 MUST 在读取旧页面内容（用于 user section 恢复）之后删除旧数据库
- **THEN** 系统 MUST 创建新数据库并写入全量重建结果

### Requirement: 系统必须使用 rusqlite bundled 模式
系统 MUST 使用 `rusqlite` crate 的 `bundled` feature，自带 SQLite 源码编译，不依赖系统级 SQLite 安装。

#### Scenario: Windows 编译
- **WHEN** 在 Windows 环境下编译 wiki-core
- **THEN** `rusqlite` MUST 使用 bundled SQLite 编译成功
- **THEN** 不得要求用户额外安装 SQLite
