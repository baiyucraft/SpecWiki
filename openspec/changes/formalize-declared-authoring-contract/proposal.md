## Why

当前 `v0.2.0` 已经建立 `minimal formal knowledge runtime`，但 `declared knowledge` 仍主要停留在最小 writeback block 形态，尚不足以正式承接替代、废弃、稳定 scope 与 lifecycle 传播。这使 `.wiki/.knowledge/declared/**` 虽已进入 runtime 分层，却还没有形成可恢复、可诊断、可测试的 authoring contract，也不足以支撑后续更完整的 knowledge system 演进。

现在推进这项变更，是为了先完成 `Phase 1: Formal Completeness` 中最关键的 declared contract 收口，把真相边界、对象模型与 workflow 消费关系收稳，避免后续继续依赖 page-first 暗语义或临时 runtime 约定。

## What Changes

- 正式扩展 `DeclaredRecord` 对象模型，至少支持 `supersedes`、`replaced_by`、`deprecated` 等 declared lifecycle 关系。
- 将当前松散的字符串 scope 收敛为稳定的 typed scope object，使 declared scope 可以被 runtime、artifact 与 workflow 一致消费。
- 正式定义 declared lifecycle 传播语义，让 declared 变更能够稳定驱动 `research_stale`、`projection_stale` 或等价下游失效状态，而不是只作为单次 writeback 结果存在。
- 让 `sync`、`update`、`status` 与相关 runtime 读取 declared lifecycle contract，而不是只把 declared 当作页面回写块。
- 收紧 `.wiki/.knowledge/declared/**` 的 artifact contract，使其具备 restore、audit、diagnosis 与 roundtrip test 所需的最小正式字段与关系。
- **BREAKING**：如果当前实现仍把 declared 视为字符串 scope + 单次 block writeback 占位，本轮会将其收紧为正式 declared authoring contract，并删除不再成立的弱约定。

## Capabilities

### New Capabilities

无。

### Modified Capabilities

- `declared-knowledge-lifecycle`：扩展 declared record 的关系模型、typed scope 与废弃/替代生命周期合同。
- `knowledge-runtime-artifacts`：将 declared artifact 从最小落盘对象提升为可恢复、可审计、可 roundtrip 的正式 snapshot 组成部分。
- `repo-wiki-workflow`：让 `sync`、`update`、`status` 与相关 workflow 正式消费 declared lifecycle 与诊断语义。

## Impact

- `crates/wiki-model/**` 的 declared record / scope / relation schema
- `crates/wiki-runtime/**` 的 sync / update / status / restore / workflow 诊断逻辑
- `.wiki/.knowledge/declared/**` 的持久化、恢复与审计 contract
- `query / status / sync / update` 对 declared lifecycle 的可观察输出
