## MODIFIED Requirements

### Requirement: `query` 必须以 `index -> knowledge -> page fallback` 结果为正式稳定合同
系统 MUST 将 `query` 的正式稳定合同收敛到外部 `term-only`、内部 `index/symbol -> graph -> declared -> derived -> page fallback` 的结果路由。宿主与用户当前版本可稳定依赖的字段 MUST 继续以 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 和 `provenance_summary` 为主；其中 `query_mode` 负责表达结果主要落在哪一层粗粒度模式，`query_trust` 与 `recommended_action` 负责表达当前结果是否可直接消费及下一步动作，`provenance_summary` MUST 至少能稳定区分 `index_hit`、`graph_hit`、`declared_hit`、`derived_hit` 与 `page_fallback` 五类 route tags。其他实现可见字段 MAY 出现，但 MUST NOT 脱离该 route 语义漂移成新的公开 payload。

#### Scenario: `query` 优先返回 facts 与 formal knowledge lane
- **WHEN** 用户执行 `spec-wiki wiki query`，且当前 query 同时存在 facts/index、graph 或 formal knowledge 命中
- **THEN** 响应 MUST 优先体现 facts 与 formal knowledge lane 的结果，而不是把 page fallback 提升成默认主路径
- **THEN** 调用方 MUST 能从 `provenance_summary` 看出当前命中属于 `index_hit`、`graph_hit`、`declared_hit` 或 `derived_hit`

#### Scenario: declared 与 derived 在 workflow contract 中必须可区分
- **WHEN** 当前 query 命中了 formal knowledge
- **THEN** `provenance_summary` 与等价字段 MUST 能稳定区分 `declared_hit` 与 `derived_hit`
- **THEN** 系统 MUST NOT 继续只用单一 `knowledge_hit` 掩盖 knowledge lane 的不同 truth source

#### Scenario: 只有页面兜底时显式保留 fallback 语义
- **WHEN** 当前 query 没有足够的 facts、graph 或 formal knowledge 命中，只能依赖页面内容兜底
- **THEN** `query` MUST 继续返回可消费结果
- **THEN** 调用方 MUST 能稳定读取到 `page_fallback` provenance 与独立的 `query_trust / recommended_action`

#### Scenario: readiness 与 provenance 不得混层
- **WHEN** 当前 query 命中了任一 route lane，且 runtime 同时存在 stale、blocker 或 health degradation
- **THEN** `query_trust` 与 `recommended_action` MUST 继续负责表达 readiness / next action
- **THEN** `provenance_summary` MUST 继续只负责表达 route 来源，而不是被用来承载 blocker 或 update 诊断
