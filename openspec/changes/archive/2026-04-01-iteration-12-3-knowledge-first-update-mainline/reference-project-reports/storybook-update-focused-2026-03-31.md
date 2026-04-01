# storybook update-focused verification (2026-03-31)

## Scope

- change: `iteration-12-3-knowledge-first-update-mainline`
- sample: `storybook`
- objective: 在 `storybook` 上验证 knowledge-first update 的 `affected scope / projection refresh / escalation reason`
- note: 本报告只覆盖 `storybook`，不替代 `dagger` 结论

## Baseline Recovery

本轮先把 `tmp/test/storybook` 从 `runtime_incomplete` 补到了可验证 baseline：

- resume command wall time: `2694454ms`，约 `44.9` 分钟
- init 完成后状态:
  - `state=fresh`
  - `runtimeState=ready`
  - `metadataExists=true`
  - `markdownPageCount=226`
  - `knowledge_units=226`
  - `knowledge_domains=14`
  - `research_cache=241`
  - `page_digests=226`
  - `page_drafts=226`
  - `wiki_pages=226`

这一步解决了此前 [storybook-update-focused-blocker-2026-03-31.md](/E:/project/!byAI/spec-wiki/openspec/changes/iteration-12-3-knowledge-first-update-mainline/reference-project-reports/storybook-update-focused-blocker-2026-03-31.md) 里“baseline 还没进入 update-ready”的阻塞。

## Probe Change

选择的最小变更点:

- source: `tmp/test/storybook/code/addons/a11y/src/types.ts`
- change type: 追加一行临时注释 `// update-focused probe marker`

这条链对应的已知单元 / 页面：

- direct unit `unit-26477bd3916f`: `API类型定义` -> `API-参考/类型定义参考/API类型定义.md`
- direct unit `unit-e9a8c7073d42`: `addons` -> `插件生态/addons.md`
- target page `page-63f61d90e01c` -> `.wiki/插件生态/addons.md`
- target page `page-7500ccee3e7c` -> `.wiki/API-参考/类型定义参考.md`
- target page `page-cc7d7de3ea95` -> `.wiki/API-参考/类型定义参考/API类型定义.md`

## Status Result

变更后执行 `status`，结果如下：

- `state=needs_update`
- `recommended_action=update`
- `query_readiness=needs_update`
- `dirty_sources=["code/addons/a11y/src/types.ts"]`
- `dirty_pages=[
  ".wiki/API-参考/类型定义参考.md",
  ".wiki/API-参考/类型定义参考/API类型定义.md",
  ".wiki/插件生态/addons.md"
  ]`

`affected_knowledge_scope` 明确为：

- `affected_domain_ids=["domain-006c7002a0fc","domain-94ece2bc9556"]`
- `direct_unit_ids=["unit-26477bd3916f","unit-e9a8c7073d42"]`
- `propagated_parent_unit_ids=[]`
- `projection_target_page_ids=[
  "page-63f61d90e01c",
  "page-7500ccee3e7c",
  "page-cc7d7de3ea95"
  ]`
- `removed_unit_ids=[]`
- `escalation.level=local_refresh`
- `escalation.reason=direct_unit_refresh`

结论：

- 这次局部源码变化没有升级到 `subtree_replan` / `repo_replan` / `rebuild_recommended`
- 没有额外的 parent propagation
- dirty page 集与 projection target 集一一对应

## Update Result

执行 `update` 后返回：

- `state=fresh`
- `previous_state=stale`
- `updated_pages=[
  ".wiki/API-参考/类型定义参考.md",
  ".wiki/API-参考/类型定义参考/API类型定义.md",
  ".wiki/插件生态/addons.md"
  ]`
- `affected_knowledge_scope` 与 `status` 保持一致

这一步的关键 runtime 结果：

- `workflow_action=update`
- `runtime_state=completed`
- `assembled_pages=226`
- `page_digests=3`
- `page_drafts=3`
- `wiki_pages=226`

这里的 `page_digests=3 / page_drafts=3` 说明本次 update 提交的是 scoped formal refresh，而不是把全部 `226` 个 page digest / draft 重新写一遍。

## Projection Refresh Evidence

update 前后页面时间戳如下：

- changed:
  - `.wiki/API-参考/类型定义参考.md`: `16:01:12 -> 16:04:52 UTC`
  - `.wiki/API-参考/类型定义参考/API类型定义.md`: `16:01:10 -> 16:04:51 UTC`
  - `.wiki/插件生态/addons.md`: `16:01:01 -> 16:04:51 UTC`
- unchanged sample:
  - `.wiki/构建系统/vitest.md`: 维持 `16:00:18 UTC`

formal artifact 时间戳也只在 update 时前移：

- `.wiki/.knowledge/runtime/page-digests.jsonl` -> `16:05:01 UTC`
- `.wiki/.knowledge/derived/research-summaries.jsonl` -> `16:05:01 UTC`
- `.wiki/wiki.metadata.json` -> `16:04:53 UTC`

结论：

- projection refresh 命中了 3 个目标页面
- 未抽样命中的无关页面没有被一起重写
- scoped formal artifacts 已随 update 提交

## Cleanup

临时注释随后已删除，并再次执行一次 `update` 把样本恢复到 clean baseline。

恢复后：

- `git -C tmp/test/storybook diff -- code/addons/a11y/src/types.ts` 为空
- `status.state=fresh`
- `dirty_sources=[]`
- `dirty_pages=[]`

## Interpretation

`storybook` 现在已经提供了 `4.3` 所需的一半专项证据：

1. `status` 能把局部变更稳定映射到 `AffectedKnowledgeScope`
2. escalation 保持在 `local_refresh/direct_unit_refresh`
3. `propagated_parent_unit_ids=[]`，说明这次变更没有被错误升级成 parent propagation
4. `update` 只刷新了 3 个 projection targets
5. scoped formal artifacts 也只提交了局部 `page_digests / page_drafts`

## Task Impact

- `storybook` 侧的 update-focused 证据已补齐
- 整体 `4.3` 仍不能直接勾选，因为 task wording 仍要求 `storybook + dagger`
