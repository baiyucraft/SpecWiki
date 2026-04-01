## Why

`iteration-12-3-knowledge-first-update-mainline` 已经把 knowledge-first update 的主线实现和主规格落下来了，但仓库里还残留一小段没有被单独收口的 contract 余项：

- 高层 parent `KnowledgeUnit` 的 contract 还需要明确写清“child rollup 身份”之外的 `UnitResearch` 输入身份
- lifecycle harness 对磁盘恢复态 / 缺失态的诊断消费，需要一个可审计的 verification contract，而不是只留在脚本实现细节里

如果不把这两点单独收口，后续很容易再次把它们和 `wiki-index/query`、`steering/config` 或 `Agents/CLI` 这类独立主题混到一起，导致 change 边界继续漂移。

## What Changes

- 收紧高层 parent `KnowledgeUnit` 的正式 contract：`Overview / Architecture / DomainIndex / config-surface parent` 不仅要保持 child-backed 聚合身份，还必须保持后续 `UnitResearch` 与 child rollup 的稳定输入身份
- 为 lifecycle verification 增加“诊断态可消费”约束：当磁盘侧只剩恢复/缺失快照时，测试 harness 必须把它提升为可消费的 diagnostic runtime state，而不是把它误判成普通 `missing`
- 补一个聚焦的脚本测试，验证 lifecycle diagnostics 会保留 `query_readiness / recommended_action` 这类可消费字段
- 本 change 明确不扩张到 `wiki-index/query`、`steering/config`、`Agents/CLI`、全局 CLI forwarding 或 query route 对外合同

## Capabilities

### Modified Capabilities

- `knowledge-unit-decomposition`: 收紧高层 parent unit 的 `UnitResearch` / child rollup 输入身份 contract
- `workflow-verification`: 增加 lifecycle 诊断态消费的验证约束

## Impact

- 重点影响 OpenSpec 契约：
  - `openspec/specs/knowledge-unit-decomposition/spec.md`
  - `openspec/changes/iteration-12-3-knowledge-first-update-contract/specs/workflow-verification/spec.md`
- 重点影响验证：
  - `scripts/tests/streaming-protocol.test.ts`
