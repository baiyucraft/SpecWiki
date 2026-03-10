## ADDED Requirements

### Requirement: query 必须把 symbol 命中扩展为 graph context
系统 MUST 在保留页面与 symbol BM25 的基础上，把高置信度 symbol 命中扩展为 graph context。对于命中的 symbol，query MUST 能回填其直接相关的 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges，以及该 symbol 所属或经过的 processes / communities。query 的结构化输出 MUST 把这些 graph 结果作为 first-class 数据返回，而不是只折叠为页面或源码。

#### Scenario: symbol 命中回填直接关系边
- **WHEN** 用户查询词命中了某个已索引 symbol
- **THEN** query MUST 返回与该 symbol 直接相关的 graph relation 结果
- **THEN** 这些结果 MUST 至少包含 edge 类型、目标 symbol 和 provenance

#### Scenario: symbol 命中回填 process 与 community 上下文
- **WHEN** 命中的 symbol 参与某个 detected process 或归属于某个 community
- **THEN** query MUST 返回对应的 process 或 community 摘要
- **THEN** 返回结果 MUST 说明这些 graph 命中是如何与原始 symbol 命中关联的

### Requirement: query 必须支持基于 edges 的调用链与影响范围扩展
系统 MUST 基于 `edges` 表提供结构化 graph query 能力，用于从命中 symbol 自动扩展调用链和影响范围。系统 MAY 通过 SQLite CTE 或等价手段实现多跳遍历，但 MUST 对遍历深度和返回规模进行限制，避免 query 因大图膨胀失控。graph query 命中 MUST 与 BM25 / 结构化页面结果合并，并在 provenance 中区分来源。

#### Scenario: 命中 symbol 后自动扩展调用链
- **WHEN** 用户查询词高置信命中某个 symbol，且该 symbol 在 CALLS 图中存在上下游
- **THEN** query MUST 能返回围绕该 symbol 的有限深度调用链摘要
- **THEN** 这些 graph 命中 MUST 与已有页面/源码结果合并返回

#### Scenario: graph tables 缺失时回退到现有 BM25 与结构化匹配
- **WHEN** `edges`、`processes` 或 `communities` 尚未建立有效数据
- **THEN** query MUST 回退到现有页面 / symbol BM25 与结构化匹配逻辑
- **THEN** 系统不得因为 graph tables 暂时为空而报错或返回空响应

