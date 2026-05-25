## REMOVED Requirements

### Requirement: runtime 的 query route 必须以 index 投影为主、page fallback 为辅
**Reason**: `v0.2.0` 的 runtime query route 不再是简单的“两层模型”，而是必须显式经过 `index -> knowledge -> page fallback` 三层。
**Migration**: 改为遵循新增的 `index -> knowledge -> page fallback` requirement；runtime 继续保留 `term` 外部入口，但结果整形与 `provenance_summary` 必须体现 knowledge 层。

## ADDED Requirements

### Requirement: runtime 不得继续保留 `v0.1.0 index-only` 公开成功短路
系统 MUST 移除 `SPEC_WIKI_V0_1_INDEX_ONLY`、`index_only` 或等价“facts/index 可用即可成功”的公开短路语义。facts/index 仍可作为 query substrate、恢复基础或诊断依据存在，但 runtime MUST NOT 再把该层单独完成投影为 `init / update` 的正式成功态。

#### Scenario: `init` 与 `update` 不得在 facts snapshot 后提前成功返回
- **WHEN** runtime 已完成 facts/index snapshot，但 knowledge/runtime formal artifacts 尚未形成
- **THEN** `init` 或 `update` MUST 继续返回显式非完成诊断，或继续执行后续主链
- **THEN** 系统 MUST NOT 仅因 facts/index 已可查询就提前返回公开成功

#### Scenario: 对外状态投影不得再生成 `index_only`
- **WHEN** runtime 内部状态处于 `missing`、`runtime_incomplete` 或其它非完成态，且 facts/index 已可用
- **THEN** 对外状态投影 MUST 保持真实诊断态
- **THEN** 系统 MUST NOT 再把该状态改写为 `index_only`

### Requirement: runtime 的 query route 必须遵循 `index -> knowledge -> page fallback`
系统 MUST 让 `run_query` 的正式结果按 `index -> knowledge -> page fallback` 路由，而不是继续把 `wiki-index::query` 与页面兜底直接并列。`index` 命中 MUST 继续来自 `wiki-index::query` 的 facts/graph 投影；`knowledge` 命中 MUST 来自正式 `.wiki/.knowledge/**` 与恢复后的 runtime mirror；`matches` 中的页面结果只可作为最后一层 fallback 或补充 provenance。

#### Scenario: 存在 index 命中时优先返回 facts/graph 投影
- **WHEN** 某次 query 在 `wiki-index::query` 中命中了 symbol、source、module、entrypoint 或 graph 结果
- **THEN** runtime 返回中的 `matched_symbols`、`matched_sources`、`matched_modules`、`matched_symbol_edges` MUST 优先来自 index 投影
- **THEN** runtime MUST NOT 再通过 `WikiState` 或页面文本重算一套等价 facts 结果

#### Scenario: index 命中不足时回落到 formal knowledge
- **WHEN** 某次 query 没有足够的 index 命中，但正式 `.wiki/.knowledge/**` 中存在可用的 knowledge 记录、摘要或 projection anchor
- **THEN** runtime MUST 尝试返回对应的 knowledge 命中
- **THEN** runtime MUST NOT 直接跳过 knowledge 层而把页面结果当成唯一 fallback

#### Scenario: page fallback 只能作为最后一层兜底
- **WHEN** 某次 query 没有足够的 index 或 knowledge 命中，只能通过页面内容补足结果
- **THEN** runtime MUST 在结果中显式标注 page fallback provenance
- **THEN** 系统 MUST NOT 把页面命中伪装成 facts 或 formal knowledge 命中

### Requirement: runtime 的 query 结果必须通过稳定 route tags 分离 readiness 与 provenance
系统 MUST 让 `query` 结果中的 readiness 与 provenance 分层表达。`query_trust`、`recommended_action` 与等价字段 MUST 负责回答“当前结果是否可直接消费、是否需要 update/rebuild”；`provenance_summary` 与等价字段 MUST 负责回答“结果来自 index、knowledge 还是 page fallback”。本轮 `provenance_summary` MUST 至少能稳定区分 `index_hit`、`knowledge_hit` 与 `page_fallback` 三类 route tags。系统 MUST NOT 用单一字段同时承载这两类语义。

#### Scenario: 恢复态 runtime 可查询但不伪装成 ready
- **WHEN** 当前 runtime 是基于 `.wiki/.knowledge/** + pages + metadata` 恢复出的可查询状态，但当前代码与正式 snapshot 不一致
- **THEN** `query` MAY 返回可消费结果
- **THEN** `query_trust` 与 `recommended_action` MUST 提醒调用方该结果处于恢复态或待更新态
- **THEN** `provenance_summary` MUST 继续只描述命中来源，而不是把恢复态直接编码成 provenance

#### Scenario: knowledge 命中可通过稳定 route tags 观测
- **WHEN** 某次 query 主要依赖 formal knowledge artifacts 命中，而不是直接 index 命中或页面兜底
- **THEN** `provenance_summary` MUST 显式标注 `knowledge_hit`
- **THEN** 验证与宿主消费 MUST 能仅凭稳定 route tags 区分该结果并进行断言

#### Scenario: page fallback 与 blocker 语义不混层
- **WHEN** 当前 query 命中了 page fallback，且 runtime 同时存在 `needs_update` 或 blocker 诊断
- **THEN** 结果 MUST 同时保留 `page_fallback` provenance 与对应推荐动作
- **THEN** 系统 MUST NOT 因为存在页面兜底就把 blocker/readiness 问题隐藏掉
