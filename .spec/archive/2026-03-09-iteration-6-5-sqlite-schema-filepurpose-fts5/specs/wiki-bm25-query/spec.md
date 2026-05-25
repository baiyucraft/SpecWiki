## ADDED Requirements

### Requirement: 系统必须在 SQLite 中维护页面全文检索索引
系统 MUST 在 `.wiki/.cache/wiki-cache.db` 中为 Wiki 页面建立 `wiki_pages_fts` FTS5 虚拟表，并把页面标题、路径以及用于检索的文本字段同步到索引中。系统同时 MUST 预建 `symbols` 与 `symbols_fts` 表结构，为迭代 7 的符号检索留出稳定 schema；在 6.5 中这些符号表可以为空，但 schema 必须存在。

#### Scenario: init 初始化全文检索 schema
- **WHEN** 用户首次执行 `init`
- **THEN** 系统 MUST 创建 `wiki_pages_fts`
- **THEN** 系统 MUST 同时创建 `symbols` 与 `symbols_fts` 的表结构，即使当前没有任何符号数据

#### Scenario: update 或 sync 刷新页面索引
- **WHEN** `update` 或 `sync` 修改了页面标题、路径或用于检索的文本字段
- **THEN** 系统 MUST 在同一轮 workflow 中刷新对应的 `wiki_pages_fts` 记录
- **THEN** 过期索引项不得在后续 query 中继续命中

### Requirement: query 必须支持 BM25 页面搜索并与结构化结果合并
系统 MUST 在 `query` workflow 中使用 `wiki_pages_fts` 执行 BM25 页面检索，并把命中的 `page_id` 回填成现有的结构化 `QueryReport`。当同一页面同时被 BM25 和结构化匹配命中时，系统 MUST 合并为单个结果，并在 provenance 中保留两类命中来源。

#### Scenario: 仅页面标题或路径命中时仍返回页面结果
- **WHEN** 用户查询词命中了某页面的标题或路径，但没有命中模块、源码或关系的结构化索引
- **THEN** `query` MUST 仍返回该页面
- **THEN** 该结果的 provenance MUST 标记为来自 BM25 页面检索

#### Scenario: BM25 与结构化命中合并
- **WHEN** 同一页面既被 BM25 检索命中，又被结构化匹配命中
- **THEN** 系统 MUST 只返回一条页面结果
- **THEN** 该结果 MUST 同时保留 BM25 与结构化命中的 provenance 信息

#### Scenario: FTS 索引为空时回退到现有结构化匹配
- **WHEN** `wiki_pages_fts` 尚未建立有效数据或当前仓库还没有页面索引记录
- **THEN** `query` MUST 回退到现有的结构化匹配逻辑
- **THEN** 系统不得因为 FTS 无结果而报错或返回空响应
