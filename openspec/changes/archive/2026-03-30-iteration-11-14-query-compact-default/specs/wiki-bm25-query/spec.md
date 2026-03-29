## ADDED Requirements

### Requirement: 默认 query 命中结果必须对 Agent 以统一 hits 形式暴露
系统 MUST 将默认 query 的页面、符号、源码、模块和调用边线索压平成统一 `hits`，避免要求宿主在多套并行数组之间自行拼接。每个 hit MUST 至少包含类型、标题、位置和摘要；当命中来源需要说明时，系统 SHOULD 同时返回 reasons 或 provenance。

#### Scenario: symbol 查询返回 compact symbol hit
- **WHEN** 用户查询词命中了某个 symbol
- **THEN** 默认 query 结果 MUST 至少返回一个 `hit_type = symbol` 的命中
- **THEN** 该 hit MUST 包含 symbol 名称和源码位置

#### Scenario: graph 查询返回 compact call edge hit
- **WHEN** 用户查询命中了可扩展的调用边上下文
- **THEN** 默认 query 结果 MUST 至少返回一个 `hit_type = call_edge` 的命中
- **THEN** 该 hit MUST 包含边摘要和最小 provenance 信息
