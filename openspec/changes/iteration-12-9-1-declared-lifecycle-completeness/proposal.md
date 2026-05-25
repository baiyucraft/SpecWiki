## Why

当前仓库已经支持 declared block 的最小 writeback，但删除/撤销语义仍未进入 formal lifecycle。只要本次合法页面快照相对前一 declared snapshot 少了记录，不论是 `N -> 0` 还是 `N -> M`，系统现在都可能残留旧 declared artifacts，并漏掉这次删除带来的正式 stale propagation，结果是 artifact truth 与 page authoring surface 发生静默漂移。

因此，这个子 change 先只收 capability 1 里的一个硬缺口：让 declared 删除/撤销进入 formal lifecycle，使 `sync`、artifact snapshot、health signals 与后续 `update` 对这类变化保持一致。

## What Changes

- 扩展 declared lifecycle contract，明确“合法删除此前存在的 declared records”也是一种正式 writeback，而不是普通 metadata-only 编辑。
- 扩展 `sync` 的 page-level atomic 语义：以 page-scoped declared snapshot diff 计算本页 removed records，不论本次是 `N -> 0` 还是 `N -> M`，只要页面合法提交，就从 formal snapshot 中移除这些旧 records。
- 将 removed declared records 产生的受影响 scope 正式传播到 `stale_unit_ids`、`stale_projection_ids`、health signals 与后续 `update` 可消费状态。
- 保持现有边界：`illegal_drift` 仍然优先于 declared 删除写回；本轮不引入 governance decision、conflict artifact、query ranking 或新的 tombstone object。
- **BREAKING**：若当前实现把“删掉 declared block”视为 metadata-only 或静默忽略，本轮会收紧为正式 lifecycle writeback。

## Capabilities

### New Capabilities

无。

### Modified Capabilities

- `declared-knowledge-lifecycle`: 增加 declared 删除/撤销进入 formal lifecycle 的要求。
- `knowledge-runtime-artifacts`: 增加 declared 删除后的 snapshot pruning 与 restore 一致性要求。
- `knowledge-runtime-health-signals`: 增加 declared 删除导致的 divergence / stale 诊断要求。
- `knowledge-first-update`: 增加 removed declared records 驱动 stale scope 与 declared-only update 的要求。
- `repo-wiki-workflow`: 增加 `sync` 对 declared 删除分类、推荐动作与受影响 scope 的正式语义。

## Impact

- `crates/wiki-runtime/src/workflows/sync.rs` 的 page analysis、writeback 分类与 snapshot 提交流程
- `crates/wiki-runtime/src/storage/knowledge_artifacts.rs` 的 declared artifact 合并与 health snapshot 一致性
- `crates/wiki-runtime/tests/runtime/*.rs` 中与 declared writeback、health propagation、update scope 相关的测试
- OpenSpec 中 declared lifecycle / workflow / artifact / health 的基线合同
