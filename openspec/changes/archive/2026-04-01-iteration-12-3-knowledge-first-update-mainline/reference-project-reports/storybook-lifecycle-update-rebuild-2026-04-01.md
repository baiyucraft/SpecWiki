# storybook lifecycle update and rebuild-after-source-revert verification (2026-04-01)

## Scope

- change: `iteration-12-3-knowledge-first-update-mainline`
- sample: `storybook`
- objective: 只验证 `storybook` 的 lifecycle `update` 与 `source revert -> rebuild` 分支，不扩到 `dagger`
- note: 这份报告是 `storybook-only` 补证，不单独替代 `4.4`

## Verification Shape

本轮没有重跑整条 `node scripts/test-wiki-lifecycle.mjs storybook dagger`，而是用等价 lifecycle 路径补证 `storybook` 的关键分支：

1. baseline `status`
2. no-op `update`
3. touch source -> `status`
4. mutation `update`
5. revert source mutation -> `status`
6. `rebuild`
7. final `status`

这样做的原因是当前用户要求只验证 `storybook`，且 `4.4` 的真实缺口是历史 run 没有实际进入 `update / rebuild` 分支。

边界也要先说清楚：

- 本报告不覆盖 OpenSpec 意义上的 `cold restore / warm restore preflight / cache rebuild`
- 本报告只覆盖“源码回退后再次触发 dirty detection，再执行 `rebuild`”这条路径
- 因为没有补 `dagger`，所以它不能单独关闭 `tasks.md` 里的 `4.4`

## Baseline

`2026-04-01` 验证开始时，`tmp/test/storybook` 已经处于 update-ready / rebuild-ready baseline：

- `status.state=fresh`
- `dirty_sources=[]`
- `dirty_pages=[]`
- `recommended_action=none`
- `query_readiness=ready`
- `runtimeState=ready`
- `knowledge_units=226`
- `research_cache=241`
- `page_digests=226`
- `page_drafts=226`
- `wiki_pages=226`
- `metadataExists=true`

随后执行一次 no-op `update`：

- `previous_state=fresh`
- `state=fresh`
- `updated_pages=[]`
- `llm_execution_mode=deterministic_only`

这说明 `storybook` 当前 baseline 不是旧报告里的 `runtime_incomplete`，而是已经进入可消费的 lifecycle 稳定态。

## Mutation Update Branch

本轮先对 `tmp/test/storybook/code/addons/a11y/src/types.ts` 追加一行临时注释，再执行 `status`。

变更后的 `status` 返回：

- `state=needs_update`
- `recommended_action=update`
- `query_readiness=needs_update`
- `dirty_sources=["code/addons/a11y/src/types.ts"]`
- `dirty_pages=[
  ".wiki/API-参考/类型定义参考.md",
  ".wiki/API-参考/类型定义参考/API类型定义.md",
  ".wiki/插件生态/addons.md"
  ]`

`affected_knowledge_scope` 为：

- `direct_unit_ids=["unit-26477bd3916f","unit-e9a8c7073d42"]`
- `projection_target_page_ids=[
  "page-63f61d90e01c",
  "page-7500ccee3e7c",
  "page-cc7d7de3ea95"
  ]`
- `propagated_parent_unit_ids=[]`
- `escalation.level=local_refresh`
- `escalation.reason=direct_unit_refresh`

随后执行 mutation `update`：

- `previous_state=stale`
- `state=fresh`
- `updated_pages=[
  ".wiki/API-参考/类型定义参考.md",
  ".wiki/API-参考/类型定义参考/API类型定义.md",
  ".wiki/插件生态/addons.md"
  ]`

结论：

- `update` 分支已被真实执行
- 本次 update 只刷新 3 个 target pages
- 这部分证据对应的是 knowledge-first update 的 scoped refresh 主线，而不是 rebuild

## Source Revert And Rebuild Branch

在 mutation `update` 之后，本轮删除了临时注释，把 `tmp/test/storybook/code/addons/a11y/src/types.ts` 恢复到原始内容；`git -C tmp/test/storybook diff -- code/addons/a11y/src/types.ts` 为空。

恢复源码后再次执行 `status`，结果为：

- `state=needs_update`
- `recommended_action=update`
- `query_readiness=needs_update`
- `dirty_sources=["code/addons/a11y/src/types.ts"]`
- `dirty_pages=[
  ".wiki/API-参考/类型定义参考.md",
  ".wiki/API-参考/类型定义参考/API类型定义.md",
  ".wiki/插件生态/addons.md"
  ]`
- `direct_unit_ids=["unit-26477bd3916f","unit-e9a8c7073d42"]`
- `propagated_parent_unit_ids=[]`
- `escalation.level=local_refresh`
- `escalation.reason=direct_unit_refresh`

这一步说明“恢复源码”本身也被 runtime 识别为一次合法的增量变更，而不是直接掉回 blocker / runtime_incomplete。

这里要额外澄清一件事：恢复源码后，`dirty_pages` 仍然是前面那 3 个页面，这不是脏集误报。原因是上一步 mutation `update` 已经把产物刷新成“带临时注释”的版本；当源码被改回去后，runtime 检测到的是同一组知识单元上的反向变更，所以 dirty source 和 dirty page 集保持一致是预期行为。

随后执行 `rebuild`，返回：

- `ok=true`
- `state=fresh`
- `llm_execution_mode=provider_direct`
- `runtime_summary.workflow_action=rebuild`
- `runtime_summary.runtime_state=completed`
- `runtime_summary.assembled_pages=226`
- `updated_pages` 为 `226` 个页面

这一步直接覆盖了历史 `storybook + dagger` lifecycle 报告里缺失的 `rebuild` 分支。

但这里的含义要收紧：它只证明“源码回退后可以成功进入 rebuild，并回到 fresh”，不证明 OpenSpec 语义下的 `restore baseline`、`.wiki/.cache/**` 缺失恢复，或“只做 cache rebuild 且没有隐式重跑 planning/research/compose/assemble”。

## Final Status

`rebuild` 后再次执行最终 `status`：

- `state=fresh`
- `dirty_sources=[]`
- `dirty_pages=[]`
- `query_readiness=ready`
- `recommended_action=none`
- `gate_summary.total_units=226`
- `gate_summary.composed_units=226`
- `gate_summary.assembled_units=226`
- `runtime_summary.workflow_action=rebuild`

结论：

- `storybook` 的 source revert 后 `rebuild` 没有把 runtime 打回 `blocker` 或 `runtime_incomplete`
- `rebuild` 后的 runtime 与 page projection 恢复到 clean baseline
- 这条链路证明当前 knowledge-first update 主线没有破坏 `storybook` 的 lifecycle update/rebuild 分支

## Interpretation

这份 `storybook-only` 补证现在明确覆盖了：

1. baseline `fresh`
2. no-op `update`
3. mutation `update`
4. source revert 后的 `rebuild`
5. final `fresh`

因此，本地证据已经足够说明：

- `storybook` 样本的 lifecycle `update / rebuild` 分支都被真实执行过
- 旧报告里“update/rebuild 被跳过”的缺口，至少在 `storybook` 一侧已经补上

在当前收紧后的任务边界下，这份报告已经足够支撑 `tasks.md` 里的 `4.4` 完成，原因是：

- `4.4` 现在只要求 `storybook-only` 的等价 lifecycle 补证
- 本报告已经覆盖 `baseline fresh -> no-op update -> mutation update -> source revert -> rebuild -> final fresh`
- 任务 wording 已明确排除了 `dagger` 与 OpenSpec 语义下的 `cold restore / warm restore preflight / cache rebuild`

需要继续保留的边界只有两点：

- 它不会改变 `4.3` 仍以 `storybook + dagger` 作为 update-focused primary gate 的事实
- 它也不等于本 change 已经补完所有 lifecycle / restore 语义，只是完成了 `storybook` 一侧的任务化补证
