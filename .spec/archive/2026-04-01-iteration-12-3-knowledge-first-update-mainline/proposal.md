## Why

`12.2` 已经把 `.wiki/.knowledge/**` 的最小正式产物和 cold restore 闭环扶正，但当前 `update` 仍然主要停留在 page/section dirty repair 心智上，缺少 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections` 这条正式主线。这样会让 `v0.2.0` 的 knowledge runtime 虽然“可落盘、可恢复”，却仍然不能把增量更新解释为 KnowledgeUnit 主线上的定向刷新。

## What Changes

- 为 `v0.2.0` 定义正式的 knowledge-first update contract，把 `update` 的主路径收口为 `ChangeSet -> affected knowledge scope -> refresh derived knowledge -> refresh impacted projections`
- 新增受影响知识范围（affected knowledge scope）合同，要求系统先定位受影响的 `KnowledgeDomain / KnowledgeUnit / parent aggregate`，再由该范围派生需要重算的 research、summary、digest 与 projection anchors / page projection
- 收口 `update` 的刷新顺序与回退边界：局部源码改动优先刷新局部 knowledge scope；只有当 scope 无法可信定位或运行时一致性破坏时，才提升为更大范围 refresh 或显式 rebuild 建议
- 要求 `update` 的删除、新增与结构变化都通过知识范围表达，而不是继续以“默认重写一批页面”作为一等语义
- 明确内部 planning escalation 与外部 runtime state 的边界：`subtree_replan / repo_replan / rebuild_recommended` 只属于内部规划与推荐动作，不额外引入新的公开状态枚举
- 补充 `storybook + dagger` 的增量专项验证，证明小改动不会默认重写整批页面，且 refresh reason 能回溯到受影响 knowledge scope
- 本轮明确不做 `v0.2.0` query route 最终发布语义，不引入 declared knowledge lifecycle，也不把公开 workflow surface 从当前 `v0.1.0` 合同提前改写

## Capabilities

### New Capabilities
- `knowledge-first-update`: 定义 affected knowledge scope、derived knowledge 定向刷新顺序、projection refresh 边界与 update 的升级/回退条件

### Modified Capabilities
- `wiki-change-set-kernel`: 变更 `ChangeSet` 的正式下游语义，使其先映射到 knowledge scope，再映射到页面与 section 投影
- `repo-wiki-runtime`: 变更 `update` 的编排、状态诊断与推荐动作表达，使 runtime 围绕 knowledge-first refresh 工作，而不是 page-first dirty rewrite
- `workflow-verification`: 增加 knowledge-first update 的自动化验证与 `storybook + dagger` 专项分析要求

## Impact

- 重点影响 `crates/wiki-runtime/src/workflows/update.rs`、`status.rs` 以及 change detection / rebuild recommendation 相关流程
- 重点影响 `crates/wiki-knowledge/src/**` 中的 affected scope 计算、research/summary/digest 刷新与 parent aggregate 传播逻辑
- 重点影响 `.wiki/.knowledge/**` 与 `.wiki/pages/**` 在增量更新时的刷新策略、删除策略和 snapshot 一致性
- 重点影响 Rust 集成测试、生命周期脚本和 `storybook + dagger` 专项报告输出
