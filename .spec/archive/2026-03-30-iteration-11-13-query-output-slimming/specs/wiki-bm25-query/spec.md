## MODIFIED Requirements

### Requirement: query 必须支持 BM25 页面搜索并与结构化结果合并
系统 MUST 在 `query` workflow 中同时使用 `wiki_pages_fts` 和 `symbols_fts` 执行 BM25 检索，并把命中的页面与符号回填成结构化响应。结构化响应 MUST 显式包含 symbol 视图，而不是只把 symbol 命中折叠为页面或源码。对于 symbol 命中，默认外部 query 响应 MUST 至少返回 `name`、`label`、`file_path`、`language` 和 `reasons`；系统 MUST NOT 把 `score`、`page_ids`、`module_ids` 这类更偏内部排序或拼接用途的字段当成默认外部合同。当同一页面或源码同时被 BM25 和结构化匹配命中时，系统 MUST 合并为单个结果，并在 provenance 中保留两类命中来源。

#### Scenario: 仅页面标题或路径命中时仍返回页面结果
- **WHEN** 用户查询词命中了某页面的标题或路径，但没有命中模块、源码或关系的结构化索引
- **THEN** `query` MUST 仍返回该页面
- **THEN** 该结果的 provenance MUST 标记为来自 BM25 页面检索

#### Scenario: 仅符号名或符号文件路径命中时返回符号结果
- **WHEN** 用户查询词命中了某个已索引符号的名称或文件路径，但没有命中页面标题或路径
- **THEN** `query` MUST 返回对应的 `matched_symbols`
- **THEN** 每个 symbol 结果 MUST 包含默认外部可读字段和命中原因

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

### Requirement: query 必须把 symbol 命中扩展为 graph context
系统 MUST 在保留页面与 symbol BM25 的基础上，把高置信度 symbol 命中扩展为 graph context。对于命中的 symbol，query MUST 能回填其直接相关的 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges，以及该 symbol 所属或经过的 processes / communities。默认外部 query 响应中的 graph 结果 MUST 以可读摘要为主，而不是继续把内部 graph row 主键、遍历距离和置信度直接作为默认合同返回。

#### Scenario: symbol 命中回填直接关系边
- **WHEN** 用户查询词命中了某个已索引 symbol
- **THEN** query MUST 返回与该 symbol 直接相关的 graph relation 结果
- **THEN** 这些结果 MUST 至少包含 edge 类型、相关 symbol 名称和命中原因

#### Scenario: symbol 命中回填 process 与 community 上下文
- **WHEN** 命中的 symbol 参与某个 detected process 或归属于某个 community
- **THEN** query MUST 返回对应的 process 或 community 摘要
- **THEN** 返回结果 MUST 说明这些 graph 命中是如何与原始 symbol 命中关联的
