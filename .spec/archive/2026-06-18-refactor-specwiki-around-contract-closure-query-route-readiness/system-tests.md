# refactor-specwiki-around-contract-closure-query-route-readiness 系统测试用例

## 用例总览

本文件定义 query route/readiness 合同的系统级验收用例，覆盖 `proposal.md` 的成功标准和 `design.md` 的关键边界。验收重点是：公开 JSON 主合同必须以 route groups/results 表达 query 结果；readiness/trust 必须阻止伪造 index 命中；fallback、governance placeholder、transport 和 TS parser 都只能消费稳定合同，不能自造语义。

## 系统测试用例

### ST-001 Query JSON 暴露闭集 route tag 与统一 result DTO

- 关联成功标准: Query JSON 能稳定观察正式 route tag；每条 query result 都携带统一 DTO 字段；不同 route 的 score 以 group/route 表达。
- 覆盖设计点: Query Public Contract、Query Fusion Adapter、`QueryResultDto` 字段固定。
- 前置条件: 测试仓库已存在可查询的 index / knowledge / projection fixture，runtime query 以 JSON 输出。
- 操作 / 触发: 运行 query 流程查询一个同时命中 source path、knowledge 和 projection 的 term。
- 期望结果: JSON 顶层包含 `route_groups` 与 `results`；每个 result 都包含 `route_tag`、`ref_kind`、`ref_id`、`label`、`score`、`provenance`、`confidence`、`recommended_action`、`source_refs`；route tag 使用闭集值，不出现 `index_hit`、`knowledge_hit`、`page_fallback` 作为主合同。
- 验证方式: Rust runtime/acceptance 测试断言 JSON payload；TS parser 测试断言字段被保留。

### ST-002 Readiness/trust gate 阻止 missing/stale/blocked index 伪命中

- 关联成功标准: index missing、stale 或 blocked 时 query 不返回 graph 伪命中；仍可消费的 knowledge/projection 结果通过 readiness 和 query trust 显式降级。
- 覆盖设计点: Runtime readiness、Query Fusion Adapter、Readiness/trust gate。
- 前置条件: 构造 index 状态分别为 `missing`、`stale`、`blocked` 的 runtime fixture，并保留可用 knowledge/projection fixture。
- 操作 / 触发: 对相同 term 执行 query。
- 期望结果: payload 不包含 `index_symbol_hit`、`index_path_hit`、`index_graph_hit`；可返回 `knowledge_declared_hit`、`knowledge_derived_hit` 或 `projection_ref`，但 `query_trust` / `recommended_action` 必须体现降级或需重建动作。
- 验证方式: Rust runtime 测试断言 route 缺失、readiness 状态和 trust/action 组合。

### ST-003 Markdown fallback 只能作为显式 debug fallback

- 关联成功标准: Markdown 正文 fallback 被标记为 `rendered_page_debug_fallback`，并使 query trust 或 answer trust 降级。
- 覆盖设计点: fallback 显式化、Human Explanation Contract。
- 前置条件: index、knowledge、projection 均没有正向命中，但存在可渲染页面文本 fallback。
- 操作 / 触发: 运行 query 并请求 JSON 与人类可读输出。
- 期望结果: JSON 只以 `rendered_page_debug_fallback` 表示该命中，不能伪装为 knowledge/index route；trust 低于 ready 正向命中；人类输出说明这是 fallback、来源页面和验证位置。
- 验证方式: Rust runtime 测试覆盖 JSON；acceptance 或 snapshot 测试覆盖人类输出关键词。

### ST-004 Governance 未启用不阻断普通 query

- 关联成功标准: 未启用 governance 时返回 `governance_readiness: not_enabled` 或等价状态，不阻断普通 query；后续 governance child 不需要重定 route tag、readiness 或 trust 语义。
- 覆盖设计点: governance placeholder、非目标边界。
- 前置条件: runtime fixture 不启用 governance evidence index，同时存在可查询 knowledge/projection 或 fallback。
- 操作 / 触发: 运行普通 query。
- 期望结果: payload 包含 `governance_readiness: not_enabled` 或 governance route group 为空；普通 query 仍返回允许的非 governance route；不扫描 `.spec` evidence。
- 验证方式: Rust runtime 测试断言 not_enabled/空组与非阻断行为。

### ST-005 Transport 与 TS parser 只消费主合同，不重建语义

- 关联成功标准: Rust runtime transport、TS parser 和相关测试收口公开 query 合同，避免 Host、Skill 或 CLI 自造私有 route tag；后续 code graph / governance child 只填 adapter 不重定义 DTO。
- 覆盖设计点: Runtime Transport Slimmer、TS Parser Guardrail。
- 前置条件: Rust runtime 输出包含完整 route groups/results、readiness、query trust 和派生 `provenance_summary`。
- 操作 / 触发: 通过 `packages/spec-wiki` parser 消费 runtime JSON，并注入未知主 route tag / ref kind 的负向 fixture。
- 期望结果: parser 保留主合同与未知附加字段；拒绝未知主 route tag/ref kind；不重新计算 trust、route 或 recommended action；`provenance_summary` 只作为只读派生摘要。
- 验证方式: TS 单元测试和现有 command/acceptance 测试断言 parser 行为。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| Query JSON 能稳定观察到正式 route tag | ST-001, ST-005 | Rust runtime/acceptance 测试、TS parser 测试 |
| 每条 query result 都携带统一 DTO 字段 | ST-001, ST-005 | Rust DTO serde 测试、TS parser 测试 |
| 不同 route 的 score 以 group/route 表达 | ST-001 | Rust runtime 测试 |
| index missing/stale/blocked 不返回 graph 伪命中 | ST-002 | Rust runtime 测试 |
| Markdown fallback 显式标记并降级 trust | ST-003 | Rust runtime 测试、输出 snapshot |
| knowledge/projection/governance route 分层表达 | ST-001, ST-004 | Rust runtime 测试 |
| 未启用 governance 不阻断普通 query | ST-004 | Rust runtime 测试 |
| 人类输出解释命中原因、来源和下一步动作 | ST-003, ST-005 | runtime/acceptance 输出断言 |
| 后续 code graph 只填 adapter，不重定义 DTO | ST-001, ST-005 | DTO serde 与 parser guardrail 测试 |
| 后续 governance 只接 refs，不重定义 route/readiness/trust | ST-004, ST-005 | governance placeholder 测试 |

## 边界与异常

- 未知主 route tag 或 ref kind 必须被 parser/serde 视为协议错误，不能吞掉。
- `provenance_summary` 可以存在，但只能从主合同派生，不能作为 route/trust 的来源。
- 本 change 不验证 `.spec` evidence 扫描、不新增 governance index、不定义 graph schema。
- score 只在同 route/group 内可比较；跨 route 的排序只能由 group 和解释文本表达。
- fallback 结果必须低 trust，并明确推荐用户重建或补充 knowledge/index。

## 验证数据与环境

- Rust 测试 fixture 需要覆盖 ready、missing、stale、blocked、not_enabled、fallback-only 状态。
- TS 测试 fixture 需要包含完整 payload、未知 route tag、未知 ref kind、未知附加字段。
- 计划命令包括 `cargo test -p wiki-model --test query_contract`、`cargo test -p wiki-runtime query_sync_rebuild --test runtime`、`cargo test -p wiki-runtime query_transport_returns_slim_payload_but_internal_query_stays_rich --test acceptance`、`pnpm --filter spec-wiki test -- index.test.ts`，最终以实现阶段实际测试名为准。

## 未覆盖项

- 不覆盖真实 code graph schema 和 graph scoring 算法，后续 `refactor-specwiki-around-contract-closure-code-graph-index` 负责。
- 不覆盖治理 evidence 生命周期、validate/archive 和 governance summary 生成，后续 `governance-isolation` 子项负责。
- 不覆盖 CLI 一级命令产品文案和默认 help，后续 `cli-product-surface` 子项负责。

## 参考资料

- `proposal.md`
- `design.md`
- `.docs/design/specwiki-contract-closure.md`
- `.wiki/03-模块指南/01-wiki-model.md`
- `.wiki/03-模块指南/04-wiki-runtime.md`
- `.wiki/03-模块指南/05-spec-wiki-cli.md`
