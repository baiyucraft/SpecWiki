## MODIFIED Requirements

### Requirement: 系统必须在 SQLite 中维护页面全文检索索引
系统 MUST 在 `.wiki/.cache/wiki-cache.db` 中为 Wiki 页面建立 `wiki_pages_fts` FTS5 虚拟表，并把页面标题、路径以及用于检索的文本字段同步到索引中。系统同时 MUST 为定义类符号维护 `symbols` 与 `symbols_fts`：`symbols_fts` MUST 同步索引至少 `name`、`file_path` 和可用于检索的符号文本字段；符号被新增、修改或删除时，系统 MUST 在同一轮 workflow 中刷新对应的 symbol 索引项。

#### Scenario: init 初始化全文检索 schema
- **WHEN** 用户首次执行 `init`
- **THEN** 系统 MUST 创建 `wiki_pages_fts`
- **THEN** 系统 MUST 同时创建 `symbols` 与 `symbols_fts`
- **THEN** 对于成功解析出的定义类符号，系统 MUST 在本轮 `init` 中写入对应 `symbols_fts` 记录

#### Scenario: update 或 sync 刷新页面索引
- **WHEN** `update` 或 `sync` 修改了页面标题、路径或用于检索的文本字段
- **THEN** 系统 MUST 在同一轮 workflow 中刷新对应的 `wiki_pages_fts` 记录
- **THEN** 过期索引项不得在后续 query 中继续命中

#### Scenario: update 或 rebuild 刷新符号索引
- **WHEN** `update` 或 `rebuild` 导致某个源码文件的 symbol snapshot 变化
- **THEN** 系统 MUST 在同一轮 workflow 中刷新该文件对应的 `symbols` 和 `symbols_fts`
- **THEN** 已删除源码文件对应的 symbol 索引项 MUST 被清理

### Requirement: query 必须支持 BM25 页面搜索并与结构化结果合并
系统 MUST 在 `query` workflow 中同时使用 `wiki_pages_fts` 和 `symbols_fts` 执行 BM25 检索，并把命中的页面与符号回填成结构化响应。结构化响应 MUST 显式包含 symbol 视图，而不是只把 symbol 命中折叠为页面或源码。对于 symbol 命中，系统 MUST 至少返回 `symbol_id`、`name`、`label`、`file_path`、`language`、`is_exported` 和 `reasons`；当同一页面或源码同时被 BM25 和结构化匹配命中时，系统 MUST 合并为单个结果，并在 provenance 中保留两类命中来源。

#### Scenario: 仅页面标题或路径命中时仍返回页面结果
- **WHEN** 用户查询词命中了某页面的标题或路径，但没有命中模块、源码或关系的结构化索引
- **THEN** `query` MUST 仍返回该页面
- **THEN** 该结果的 provenance MUST 标记为来自 BM25 页面检索

#### Scenario: 仅符号名或符号文件路径命中时返回符号结果
- **WHEN** 用户查询词命中了某个已索引符号的名称或文件路径，但没有命中页面标题或路径
- **THEN** `query` MUST 返回对应的 `matched_symbols`
- **THEN** 每个 symbol 结果 MUST 包含结构化字段和命中原因

#### Scenario: symbol 命中回填相关源码与页面上下文
- **WHEN** `symbols_fts` 命中了某个符号，且该符号所在源码文件已被 runtime 索引
- **THEN** `query` MUST 同步回填该符号相关的源码命中
- **THEN** 如果存在引用该源码的页面，系统 MUST 尽力回填相关页面或模块上下文

#### Scenario: BM25 与结构化命中合并
- **WHEN** 同一页面或源码既被 BM25 检索命中，又被结构化匹配命中
- **THEN** 系统 MUST 只返回一条对应结果
- **THEN** 该结果 MUST 同时保留 BM25 与结构化命中的 provenance 信息

#### Scenario: FTS 索引为空时回退到现有结构化匹配
- **WHEN** `wiki_pages_fts` 或 `symbols_fts` 尚未建立有效数据
- **THEN** `query` MUST 回退到现有的结构化匹配逻辑
- **THEN** 系统不得因为某一类 FTS 无结果而报错或返回空响应
