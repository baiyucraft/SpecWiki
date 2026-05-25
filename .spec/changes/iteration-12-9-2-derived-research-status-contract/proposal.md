## Why

当前仓库已经把 `KnowledgeResearchSummary` 落成正式 artifact，但它仍然只是“最小能用”的对象，还没有稳定的 status contract：

- `summary_status` 仍是松散字符串
- `provider_stop_reason` 只是旁路字段，不能稳定表达 formal `blocked / degraded`
- artifact snapshot 与 restore 还没有 research summary 合法性校验
- `unit / status / health` 也还不能正式区分 `ready / degraded / blocked` research

这会导致 derived 层虽然“存在”，但其状态语义仍然松散，调用方只能靠若干隐式规则和 runtime gate 侧推，无法把 `ResearchSummary` 当成真正的 formal object。

因此，这个子 change 不追求“补完 derived research 全合同”，而是只收最小硬缺口：把 `derived research status contract` 正式化。

## What Changes

- 为 `KnowledgeResearchSummary` 引入 typed `summary_status`，不再使用松散字符串。
- 为 `degraded / blocked` 引入最小 machine-readable reason contract，只覆盖 `reason_kind / reason_message / upstream_ref?`。
- 在 artifact 持久化与 restore 前增加 research summary snapshot 合法性校验。
- 让 `KnowledgeUnit`、`status` 和 health signals 正式消费 `ready / degraded / blocked` research summary，而不是只依赖 runtime gate 或自由字符串。
- 保持边界：本轮不重做 evidence layer，不修改 query/answer contract，不引入 governance / provider policy / 新 recommended action 家族。

## Capabilities

### Modified Capabilities

- `research-driven-page-composition`: research summary 从“最小可落盘对象”推进到“带稳定状态合同的 formal derived object”。
- `knowledge-runtime-artifacts`: derived snapshot 与 restore 必须校验 research summary 合法性。
- `knowledge-runtime-health-signals`: health/status 必须消费 `degraded / blocked` research summary 的正式状态。

### New Capabilities

- `derived-research-status-contract`: 定义 `KnowledgeResearchSummary` 的正式 status / reason / validation contract。

## Impact

- `crates/wiki-model/**` 中 research summary schema 与校验函数
- `crates/wiki-knowledge/**` 中 `UnitResearch -> KnowledgeResearchSummary` 的收敛规则
- `crates/wiki-runtime/**` 中 artifact 持久化、restore、unit/status/health 投影与相关测试
