# wiki-bm25-query Specification

## Purpose

定义 `wiki-index` 与 Runtime page debug route 使用的 FTS/BM25 检索 substrate、索引刷新和 route-local ranking 边界。该 capability 不拥有 Runtime transport，不得定义公开 request/response，也不把内部 symbol/graph intent 自动提升为公开 query 行为。

## Requirements

### Requirement: 系统必须维护可恢复的 FTS 索引

系统 MUST 在 `.wiki/.cache/wiki-cache.db` 中维护页面、源码和符号检索所需的 FTS5 索引。页面标题、路径、可检索文本与 symbol snapshot 变化时，`init / update / sync / rebuild` MUST 按各自职责刷新或删除对应索引项；cache 丢失后 MUST 能从正式 facts/runtime inputs 重建，不得把 FTS 表当作 durable authority。

#### Scenario: init 或 rebuild 建立 FTS schema

- **WHEN** runtime 首次初始化或显式重建 cache
- **THEN** 系统 MUST 创建当前实现需要的页面、源码和符号 FTS schema
- **THEN** 已解析的 facts snapshot MUST 能生成相应索引记录

#### Scenario: update 或 sync 清理过期索引项

- **WHEN** source snapshot 或页面投影发生新增、修改或删除
- **THEN** 对应 FTS 索引 MUST 在本轮 workflow 中刷新
- **THEN** 已删除对象不得在后续 query 中继续命中

### Requirement: BM25 分数只能在 route 内参与排序

系统 MUST 把 BM25 score、match basis 和 rank 保留在各自 route 的 ranking contract 内。页面、源码和符号命中 MAY 为 Runtime route assembler 提供候选与 supporting refs，但不同 route 的 score MUST NOT 直接比较、相加或折叠成全局分数。

#### Scenario: 同一 route 内按 BM25 排序

- **WHEN** 同一 route 存在多个 FTS 候选
- **THEN** route-local BM25 ranking MUST 使用确定的 score direction 和 tie-breaker
- **THEN** 返回候选 MUST 能解释 match basis、rank 和截断信息

#### Scenario: 多 route 候选交给 Runtime 组装

- **WHEN** index、knowledge 或 page debug route 同时存在候选
- **THEN** 本 capability MUST 只提供各 route 内已排序的候选
- **THEN** Runtime MUST 通过 canonical `route_groups` 组织结果，不得在本 capability 内创造顶层合并 transport

### Requirement: page FTS 只属于受控 debug fallback

系统 MAY 在 Runtime 明确选择 `rendered_page_debug_fallback` 时使用页面 FTS 补充诊断结果。Page FTS MUST NOT 冒充 facts 或 formal knowledge，不得绕过 readiness、query trust、recommended action 或 governance 诊断。

#### Scenario: formal routes 无命中时使用 page debug fallback

- **WHEN** formal index/knowledge routes 无可用结果且 Runtime 允许 debug fallback
- **THEN** 页面 FTS MAY 返回 route-local 候选与 supporting refs
- **THEN** route identity MUST 明确为 debug fallback，页面命中不得改写为 formal knowledge

### Requirement: richer index intent 不自动成为公开 query surface

内部 `wiki-index` MAY 使用 symbol/source/module/entrypoint/callers/callees/impact 等受控 intent 构建候选，但公开 CLI 继续只接受非空 `term`。Richer intent 只有在独立 change 定义 Runtime schema、ranking、errors 和 host consumption 后才能进入公开合同。

#### Scenario: CLI 提交 term-only query

- **WHEN** 用户执行公开 `spec-wiki query <term>`
- **THEN** Runtime MUST 自行选择适用 routes 并返回 canonical response
- **THEN** 调用方不得通过本 capability 传入内部 intent 或依赖内部 hit DTO
