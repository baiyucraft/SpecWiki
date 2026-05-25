## ADDED Requirements

### Requirement: knowledge-first query 必须遵循固定 lane precedence 与 participation matrix
系统 MUST 将 knowledge-first query 的内部 route 正式收敛为 `index/symbol -> graph -> declared -> derived -> page fallback`。其中 `index/symbol` 与 `graph` 属于 facts substrate；`declared` 与 `derived` 属于 formal knowledge lane；`page fallback` 只允许在前述 lane 不能提供足够可消费命中时参与。系统 MUST 为每条 lane 固定参与条件与角色：`graph` 只能作为 facts 命中的直接扩展；`declared` 优先回答规则、约定、决策、避坑；`derived` 优先回答结构性理解、摘要与研究结论；`page` 不得重新上升为默认主路径。

#### Scenario: facts 命中时 graph 只作为直接扩展
- **WHEN** query 已命中稳定 symbol 或其它 facts substrate，且对应 graph context 可用
- **THEN** 系统 MUST 允许 `graph` 作为同一条 route 的直接扩展参与结果
- **THEN** 系统 MUST NOT 把该类 graph 命中伪装成 declared、derived 或 page fallback

#### Scenario: declared 优先于 derived 与 page
- **WHEN** 同一 query 同时命中 declared knowledge、derived knowledge 与页面投影
- **THEN** 系统 MUST 将 declared knowledge 视为更高优先级的知识 lane
- **THEN** 系统 MUST NOT 让 page fallback 压过 declared 或 derived 形成主命中

#### Scenario: page 只在前置 lane 不足时作为 fallback
- **WHEN** 当前 query 缺失足够的 facts、graph、declared 或 derived 命中
- **THEN** 系统 MAY 返回 page fallback 结果
- **THEN** page fallback MUST 被明确标记为最后一层兜底，而不是 knowledge 主命中

### Requirement: query provenance 必须稳定区分 lane route tags
系统 MUST 让 query route 的 provenance 成为正式稳定合同，而不是临时字符串。`provenance_summary` 与等价字段 MUST 至少能稳定区分 `index_hit`、`graph_hit`、`declared_hit`、`derived_hit` 与 `page_fallback`。当 query 返回页面级 `matches` 时，每条 `matches[*].provenance` MUST 能反映该页面是由 facts、declared、derived 还是 page fallback 命中带入。

#### Scenario: declared 命中保留 declared route tag
- **WHEN** query 的主命中来自 formal declared knowledge
- **THEN** `provenance_summary` MUST 包含稳定 `declared_hit` route tag
- **THEN** 对应页面或结果项的 provenance MUST NOT 只被压平成宽泛 `knowledge_hit`

#### Scenario: derived 命中保留 derived route tag
- **WHEN** query 的主命中来自 formal derived knowledge
- **THEN** `provenance_summary` MUST 包含稳定 `derived_hit` route tag
- **THEN** 对应结果项 MUST 能与 declared lane 稳定区分

#### Scenario: page fallback 显式保留兜底 provenance
- **WHEN** query 最终只能依赖页面内容或页面 BM25 兜底
- **THEN** `provenance_summary` MUST 包含稳定 `page_fallback` route tag
- **THEN** 对应结果项 MUST 显式暴露 fallback provenance，而不是伪装成 facts 或 knowledge

### Requirement: query 的 degraded policy 必须与 route provenance 分层表达
系统 MUST 将 query 的 degraded policy 与 route provenance 分层表达。`query_trust` 与 `recommended_action` MUST 负责回答“当前结果是否可直接消费、下一步该做什么”；`provenance_summary` 与 `matches[*].provenance` MUST 负责回答“结果来自哪条 route lane”。系统 MUST NOT 用 fallback provenance 代替 blocker / stale 诊断，也 MUST NOT 用 `query_trust` 取代 route tags。

#### Scenario: stale 但仍可消费时保留 route 与 action 分层
- **WHEN** 当前 runtime 处于 stale 或 health degraded，但 query 仍返回了 facts 或 knowledge 命中
- **THEN** 系统 MUST 继续返回对应 lane 的 provenance route tags
- **THEN** 系统 MUST 通过 `query_trust` 与 `recommended_action` 明确表达 `stale_but_queryable`、`update`、`review` 或等价动作

#### Scenario: fallback 与 blocker 不得混成单一状态
- **WHEN** query 命中了 page fallback，且 runtime 同时存在 blocker、needs_update 或其它 degraded 诊断
- **THEN** 系统 MUST 同时保留 `page_fallback` provenance 与独立的 `query_trust / recommended_action`
- **THEN** 系统 MUST NOT 让调用方只能从单一字段猜测“这是 fallback 还是 blocker”
