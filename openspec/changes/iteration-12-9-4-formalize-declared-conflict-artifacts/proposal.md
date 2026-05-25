## Why

当前仓库已经把 `declared records` 正式化，也补了 lifecycle 校验，但“治理冲突”仍然停留在隐含状态：

- `sync` 只能把非法 authoring / lifecycle 输入直接打回 `illegal_drift`
- 对于“每条 record 单独都合法，但整体无法推出唯一 authoritative head”的 declared 冲突，runtime 还没有 formal object
- `status` / health 也无法把这类冲突稳定暴露为可消费 artifact
- restore 更无法校验或恢复这层治理诊断结果

这意味着 declared lifecycle 已经存在，但 governance 仍然没有正式 conflict artifact。

因此，这个子 change 只收第一批最小缺口：formalize `declared conflict artifacts`，而不是试图一次补完整个治理系统。

## What Changes

- 为 deterministic declared conflicts 引入最小正式对象 `KnowledgeConflictRecord`。
- 只覆盖无需语义猜测的 declared 冲突：`parallel_active_declared` 与 `lifecycle_head_ambiguity`。
- 在 `.wiki/.knowledge/runtime/**` 中正式持久化 conflict artifacts，并拉进 persist / restore 校验主链。
- 让 `status` / health signals 正式消费 open conflicts，并给出稳定 `review` 建议。
- 让 `sync` 在合法 declared writeback 后重算 conflict artifacts，但保持 `illegal_drift` 优先，不提交半合法结果。
- 保持边界：本轮不做 override / accept / reject、不做 query / answer 消费、不做 declared vs code reality 的语义冲突。

## Capabilities

### New Capabilities
- `declared-conflict-artifacts`: 定义 deterministic declared conflict 的 formal object、validation、persist / restore 与 runtime 消费边界。

### Modified Capabilities
- `knowledge-runtime-artifacts`: runtime snapshot 需要正式包含 conflict artifacts，并在 restore 前校验其合法性。
- `knowledge-runtime-health-signals`: health/status 必须正式暴露 governance conflict，而不只是 stale / drift。
- `knowledge-first-update`: `sync` / `update` 需要把 declared conflict 重算纳入正式提交链。

## Impact

- `crates/wiki-model/**` 中 conflict formal object 与 snapshot validation
- `crates/wiki-runtime/**` 中 conflict artifact persist / load / restore、sync 重算、status / health 聚合
- `crates/wiki-runtime/tests/**` 中 declared conflict、restore、status / sync 覆盖
