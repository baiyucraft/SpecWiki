## Why

当前仓库已经把 `PageDigest` 落成正式 artifact，但 projection/readiness/recovery 这层仍然偏松散：

- `PageDigest.readiness_stage` 仍是自由字符串，formal snapshot 不能自校验
- projection 是否 `ready / stale / blocked` 仍然依赖 runtime gate、页面文件或 restore 侧推
- artifact persist / restore 还没有把 projection digest 的合法性拉进主链
- `KnowledgeUnit / status / health` 也还不能正式消费 projection stale / blocked

这意味着 projection 已经存在，但 projection 还不是“有正式状态合同的 formal object”。

因此，这个子 change 不追求“补完 projection 全合同”，而是只收第一批硬缺口：formalize `projection digest readiness contract`。

## What Changes

- 为 `PageDigest` 引入 typed `projection_status`，至少覆盖 `ready / stale / blocked`。
- 为 `stale / blocked` 引入最小 machine-readable `status_reasons`，只覆盖当前 runtime 能稳定消费的最小集合。
- 在 artifact 持久化与 restore 前增加 projection digest snapshot 合法性校验。
- 让 `KnowledgeUnit`、`status` 和 health signals 正式消费 projection `ready / stale / blocked`，而不是只看 `readiness_stage` 或页面文件。
- 保持边界：本轮不修改 query/answer、不碰 governance、不重做 renderer/evidence layer、不做 engineering hardening 指标。

## Capabilities

### Modified Capabilities

- `research-driven-page-composition`: projection digest 从“最小可落盘结果”推进到“带稳定状态合同的 formal projection object”。
- `knowledge-runtime-artifacts`: persist / restore 必须校验 projection digest snapshot 的合法性与最小对齐关系。
- `knowledge-runtime-health-signals`: health/status 必须消费 projection stale / blocked 的正式状态。
- `knowledge-first-update`: projection stale 不再只是隐式结果，而是 formal projection object 的状态。

### New Capabilities

- `projection-readiness-contract`: 定义 `PageDigest` 的正式 `projection_status / status_reasons / validation / restore` 边界。

## Impact

- `crates/wiki-knowledge/**` 中 `PageDigest` schema 与 projection status 校验
- `crates/wiki-runtime/**` 中 artifact 持久化、restore、unit/status/health 投影与相关测试
