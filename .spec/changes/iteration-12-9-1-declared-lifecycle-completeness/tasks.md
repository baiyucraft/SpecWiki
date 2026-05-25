## 1. Spec 收口

- [x] 1.1 更新 declared lifecycle、artifact、health 与 workflow 相关 specs，明确 declared 删除/撤销进入正式 lifecycle。
- [x] 1.2 复核本轮边界：不引入 tombstone object、不引入 governance decision、不修改 query ranking。

## 2. Sync 与 Artifact 实现

- [x] 2.1 在 `sync` 分析阶段引入 previous page declared snapshot 感知，并按 page-scoped declared snapshot diff 计算 removed records。
- [x] 2.2 调整 declared artifact merge，使合法删除支持 `N -> 0` 与 `N -> M`，都能从正式 snapshot 中 prune removed records。
- [x] 2.3 保持 `illegal_drift > declared_writeback > metadata_only` 优先级，并确保 illegal page 不会半提交 declared 删除。
- [x] 2.4 让 removed record refs 继续驱动 stale scope 与 health signals。

## 3. 测试与验证

- [x] 3.1 为 declared 删除写回补 integration tests，覆盖“原本有 declared → 合法删除部分/全部 → snapshot 正确 prune”。
- [x] 3.2 补 `illegal_drift` 压过 declared 删除写回的测试。
- [x] 3.3 补 declared 删除后 `update` 仍能消费 stale / health scope 的测试。
- [x] 3.4 单独检查 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 合规性，并同步更新 tasks 状态。
