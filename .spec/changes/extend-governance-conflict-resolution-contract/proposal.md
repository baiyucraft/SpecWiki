## Why

`iteration-12-9-4-formalize-declared-conflict-artifacts` 已经把 conflict artifact、persist/restore 与 health/status 消费正式化，但它停在“识别并暴露 open conflict”的层级，尚未形成真正可消费的 resolution / decision lifecycle。只要冲突仍然只能以 open artifact 存在，declared authoring、query answer 和后续 release 收口都会停留在“知道有问题”，却无法稳定表达“问题如何被审阅、如何被解决、何时解除下游 stale”。

因此这轮要做的不是再 formalize conflict artifact，而是沿着当前 formal object 主线继续扩展：把治理冲突从 `open conflict artifact` 推进到 `review-required / resolved / dismissed / superseded-by-decision` 的 resolution contract，让 runtime、artifacts 和 workflow 都有稳定的决策生命周期可消费。

## What Changes

- 新增 `governance-conflict-resolution` capability，正式定义 conflict resolution / decision lifecycle object、状态机与 artifact contract。
- 扩展 `.wiki/.knowledge/conflicts/**`，使其除了 open conflict 之外，还能稳定表达 `review-required`、`resolved`、`dismissed`、`superseded-by-decision` 等治理状态。
- 让 `status / update / sync / query` 能区分“存在未决 conflict”与“已有 resolution 但仍需下游 refresh”。
- 让 resolution decision 能稳定驱动 declared / derived / projection 的失效恢复，而不是只在 health summary 中留下静态提示。
- **BREAKING**：若当前 runtime 将 governance conflict 视为只读诊断或一次性 health signal，本轮会将其收紧为正式 resolution lifecycle truth，不再允许调用方只依赖 open conflict 文本判断状态。

## Capabilities

### New Capabilities
- `governance-conflict-resolution`: 定义治理冲突的 resolution / decision lifecycle、artifact 持久化与 workflow 消费边界。

### Modified Capabilities
- `knowledge-runtime-artifacts`: conflict artifact 需要新增 resolution state、decision refs 与恢复字段。
- `knowledge-runtime-health-signals`: health 输出需要区分 open conflict、resolved-pending-refresh 与 dismissed conflict。
- `repo-wiki-workflow`: `sync / status / update / query` 需要正式消费 governance resolution，而不是只消费 open conflict artifact。

## Impact

- `crates/wiki-model/**` 的 conflict object、decision state 与 artifact schema
- `crates/wiki-runtime/**` 的 sync / status / update / query / restore 逻辑
- `.wiki/.knowledge/conflicts/**` 的持久化、恢复与审计 contract
- 下游 answer、quality gates 与 release evidence 对 conflict 状态的消费方式
