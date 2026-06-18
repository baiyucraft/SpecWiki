# refactor-specwiki-around-contract-closure-query-route-readiness

## 问题

当前 query 输出已经有 `query_mode`、`query_trust`、`readiness` 和 `provenance_summary` 的雏形，但公开语义仍偏粗，主要停留在 `index_hit`、`knowledge_hit`、`page_fallback` 等泛化标签。后续 `code-graph-index`、`cli-product-surface` 和 `governance-isolation` 都会消费 query 输出；如果本阶段不先固定 route tag、result DTO、readiness、trust 和人类解释合同，后续 child 很容易各自补一套私有字段或私有标签。

本 change 要解决的真实问题是：为 SpecWiki query 建立稳定、机器可读、可被人解释的统一消费合同，让 index、knowledge、governance、projection 和 fallback 的来源、可信度、下一步动作在同一份 query payload 中表达清楚。

## 目标

- 固定 query route tag 合同，至少包含：
  - `index_symbol_hit`
  - `index_path_hit`
  - `index_graph_hit`
  - `knowledge_declared_hit`
  - `knowledge_derived_hit`
  - `governance_evidence_ref`
  - `governance_summary_hit`
  - `projection_ref`
  - `rendered_page_debug_fallback`
- 固定 query result DTO 的最小公开字段，包括 `route_tag`、`ref_kind`、`ref_id`、`label`、`score`、`provenance`、`confidence`、`recommended_action` 和 `source_refs`。
- 让 query 顶层输出消费已归档 restore/readiness 主合同，并明确包含 index、knowledge、projection、governance、fusion、query trust 和 recommended action。
- 明确 degraded 与 fallback 语义：index 未 ready 时不得返回 `index_*` 或 `index_graph_hit` 伪命中；Markdown 正文 fallback 必须降低 trust，并标记为 `rendered_page_debug_fallback`。
- 让人类可读输出能解释每条高置信结果为什么命中、来自哪一层、是否过期、打开哪里验证、下一步动作是什么。
- 收口 Rust runtime transport、TS parser 和相关测试的公开 query 合同，避免 Host、Skill 或 CLI 自造私有 route tag。

## 非目标

- 不重构 code graph schema、raw imports、raw calls、heritage、phase DAG、GraphStore 或 IndexQueryStore；这些属于后续 `refactor-specwiki-around-contract-closure-code-graph-index`。
- 不实现 governance artifact reference index、governance summary 生成、validate、archive 或治理状态机；本 change 只定义 governance route 与 readiness 的消费合同。
- 不调整 CLI 默认 help、Quick Start、landing state 或一级命令产品面；这些属于后续 `refactor-specwiki-around-contract-closure-cli-product-surface`。
- 不回头修改 projection writeback、managed section、declared authoring 回写或 sync-pages 行为；这些已由 `refactor-specwiki-around-contract-closure-projection-writeback-boundaries` 收口。
- 不直接迁移 upstream 代码；upstream 只作为设计和边界参考。

## 成功标准

- Query JSON 能稳定观察到正式 route tag，且不再需要 Host、Skill 或 CLI 补造私有 tag 来区分 symbol、path、graph、knowledge、governance、projection 和 fallback。
- 每条 query result 都携带统一 DTO 字段，并能表达来源、置信度、推荐动作和可验证 source refs。
- 不同 route 的 score 不可比较时，默认以 group/route 方式表达，不强行混排为单一可信排序。
- index missing、stale 或 blocked 时，query 不返回 graph 伪命中；仍可消费的 knowledge/projection 结果必须通过 readiness 和 query trust 显式降级。
- Markdown 正文 fallback 被标记为 `rendered_page_debug_fallback`，并使 query trust 或 answer trust 降级。
- knowledge declared、knowledge derived、projection ref 和 governance route 能在 DTO 中分层表达；未启用 governance 时返回 `governance_readiness: not_enabled` 或等价状态，不阻断普通 query。
- 人类输出能解释命中原因、来源层、是否过期、验证位置和下一步动作。
- 后续 `code-graph-index` 只需填充 index adapter，不需要重定义 query DTO。
- 后续 `governance-isolation` 只需接入 governance refs，不需要重定 route tag、readiness 或 trust 语义。

## 影响范围

- `wiki-runtime` query workflow、runtime profile、transport query payload 和 acceptance/runtime tests。
- `packages/spec-wiki` 的 runtime response parser、query payload 类型、CLI/e2e/parser tests。
- Agent/Skill 可消费的 query 字段说明，尤其是 route provenance、readiness、trust 和 recommended action。
- `.docs/design/specwiki-contract-closure.md` 中 query route/readiness 合同的实现落点。

本 change 可以调整现有粗粒度 `provenance_summary` 的公开口径。当前阶段不要求历史兼容；如果保留旧字段，也只能作为派生摘要，不能成为新的主合同。

## 交付形态

single-change

这是 parent change `refactor-specwiki-around-contract-closure` 下的第 4 个 child change。它依赖 `refactor-specwiki-around-contract-closure-truth-restore-snapshot`，为后续 `code-graph-index`、`cli-product-surface` 和 `governance-isolation` 提供稳定 query 消费合同。

## 风险

- 如果继续保留 `index_hit`、`knowledge_hit`、`page_fallback` 作为主合同，后续实现会重新出现 route tag 双轨漂移。
- 如果 `query_trust` 只根据是否有结果判断，stale、fallback 或 graph missing 场景会被误报为可信答案。
- 如果 result DTO 只在 Rust workflow 或 TS parser 一侧临时拼装，Host/Skill 仍可能依赖不稳定字段。
- 如果本 change 过早实现 governance scanner 或 graph schema，会和后续 child 边界冲突。

## 未知项

- 稳定 query DTO 最终应放在 `wiki-model` 还是继续位于 `wiki-runtime` transport 层，留到 design 阶段决定。
- Governance route 在本 child 中应只表达 `not_enabled` 和空组语义，还是允许基于现有 `.spec` evidence 做极小引用；推荐在 design 阶段优先选择前者，避免提前实现 governance scanner。
- 旧 `provenance_summary` 是否保留为 summary 字符串，还是由新 route groups 派生后逐步弱化，留到 design 阶段结合测试面决定。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.docs/design/governance-runtime-integration.md`
- `.docs/design/specwiki-cli-unification.md`
- `.spec/archive/2026-06-16-refactor-specwiki-around-contract-closure-truth-restore-snapshot/proposal.md`
- `.spec/archive/2026-06-17-refactor-specwiki-around-contract-closure-projection-writeback-boundaries/proposal.md`
- `.upstream/codegraph/src/context/index.ts`：仅借鉴 query result shaping / context pack，不直接迁移。
- `.upstream/codegraph/src/db/schema.sql`、`.upstream/codegraph/src/resolution/index.ts`：为后续 code graph adapter 改写借鉴，本 change 不落地。
- `.upstream/GitNexus/ARCHITECTURE.md`：为后续 phase DAG 和 graph consumption 改写借鉴，本 change 不落地。
