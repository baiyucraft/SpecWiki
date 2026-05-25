# dagger update-focused verification (2026-04-01)

## Scope

- change: `iteration-12-3-knowledge-first-update-mainline`
- sample: `dagger`
- objective: 验证结构变化下的 knowledge-first update 是否会升级到 `repo_replan`，且升级理由来自 knowledge scope，而不是页面 diff 数量

## Baseline Recovery

`tmp/test/dagger` 初始停在 `runtime_incomplete`，需要先补成 update-ready。

恢复完成后：

- `runtimeState=ready`
- `state=fresh`
- `knowledge_units=83`
- `knowledge_domains=12`
- `research_cache=96`
- `page_digests=83`
- `page_drafts=83`
- `wiki_pages=83`

随后又发现一个现成的 structural dirty source：

- `dirty_sources=["wiki.dev.yaml"]`
- `escalation.level=repo_replan`
- `escalation.reason=structural_change:wiki.dev.yaml`

我先执行了一次 no-op `update` 吸收这条历史脏变更。吸收后 clean baseline 为：

- `state=fresh`
- `knowledge_units=81`
- `page_digests=81`
- `wiki_pages=81`

后续结构探针都基于这个 `81` 页 clean baseline 执行。

## Probe Change

为了制造明确的 module membership 变化，我做了一个可逆结构探针：

1. 在 `settings.gradle.kts` 追加：
   - `includeProject(":codex-dagger-probe", "codex-dagger-probe")`
2. 新建：
   - `codex-dagger-probe/build.gradle.kts`
   - `codex-dagger-probe/README.md`

这类变化会同时触发：

- workspace/module tree 变化
- config surface 变化
- 新模块知识单元候选出现

## Status Result

变更后执行 `status`，结果如下：

- `state=needs_update`
- `recommended_action=update`
- `query_readiness=needs_update`
- `dirty_sources=[
  "codex-dagger-probe/README.md",
  "codex-dagger-probe/build.gradle.kts",
  "settings.gradle.kts"
  ]`

`affected_knowledge_scope` 为：

- `affected_domain_ids=[
  "domain-8984fdaabf39",
  "domain-94ece2bc9556",
  "system"
  ]`
- `direct_unit_ids=[
  "unit-277d633328c5",
  "unit-aecccca044d6"
  ]`
- `propagated_parent_unit_ids=[
  "unit-73b5821a1486",
  "unit-e50a8961f419"
  ]`
- `projection_target_page_ids=[
  "page-2eeeb97c252f",
  "page-5ad8cb4a5683",
  "page-7e4553385b7f",
  "page-8ef00f7742b8"
  ]`
- `removed_unit_ids=[]`
- `escalation.level=repo_replan`
- `escalation.reason=structural_change:codex-dagger-probe/build.gradle.kts,settings.gradle.kts`

对应 dirty pages：

- `.wiki/API-参考/dagger.md`
- `.wiki/核心模块/codex-dagger-probe.md`
- `.wiki/核心模块/核心模块.md`
- `.wiki/系统架构.md`
- `.wiki/项目概述.md`

这里最关键的结论有两点：

1. 升级确实发生在 knowledge scope 层，理由直接指向 `settings.gradle.kts` 和新增 module build file。
2. parent propagation 不是空集，命中了：
   - `unit-73b5821a1486` -> `核心模块/核心模块.md`
   - `unit-e50a8961f419` -> `项目概述.md`

## Update Result

执行 `update` 后返回：

- `previous_state=stale`
- `state=fresh`
- `workflow_action=update`
- `runtime_state=completed`
- `assembled_pages=82`

update 返回的 `affected_knowledge_scope` 与 `status` 保持一致：

- escalation 仍为 `repo_replan`
- reason 仍为 `structural_change:codex-dagger-probe/build.gradle.kts,settings.gradle.kts`
- parent propagation 仍命中 `核心模块` 与 `项目概述`

并且生成了新的 projection target：

- `.wiki/核心模块/codex-dagger-probe.md`

这说明本次结构变化虽然升级到了 `repo_replan`，但仍然走的是 `update` 主线，不是 `rebuild`。

## Projection Refresh Evidence

update 前后关键页面时间戳如下：

- changed:
  - `.wiki/API-参考/dagger.md`: `17:25:40 -> 17:40:54 UTC`
  - `.wiki/核心模块/codex-dagger-probe.md`: 新增，`17:40:27 UTC`
  - `.wiki/核心模块/核心模块.md`: `17:25:19 -> 17:40:28 UTC`
  - `.wiki/系统架构.md`: `17:25:19 -> 17:40:28 UTC`
  - `.wiki/项目概述.md`: `17:25:19 -> 17:40:29 UTC`
  - `.wiki/构建系统/gradle.md`: `17:25:22 -> 17:40:31 UTC`

formal artifacts 也随 update 前移：

- `.wiki/.knowledge/runtime/page-digests.jsonl` -> `17:41:01 UTC`
- `.wiki/.knowledge/derived/research-summaries.jsonl` -> `17:41:01 UTC`
- `.wiki/wiki.metadata.json` -> `17:40:58 UTC`

这说明：

- structural change 的确带来了更大范围 projection refresh
- 但刷新是通过 `update` 提交的正式 artifacts 与页面投影完成的

## Rollback

验证完成后，我删除了：

- `codex-dagger-probe/build.gradle.kts`
- `codex-dagger-probe/README.md`
- `settings.gradle.kts` 中的临时 `includeProject(...)`

并再次执行 `update` 把 `.wiki` 恢复回无 probe 的 clean baseline。

恢复后：

- `status.state=fresh`
- `dirty_sources=[]`
- `dirty_pages=[]`
- `Test-Path tmp/test/dagger/.wiki/核心模块/codex-dagger-probe.md` -> `False`

## Interpretation

`dagger` 现在提供了 `4.3` 需要的结构变化证据：

1. 结构变化会把 escalation 从 `local_refresh` 提升到 `repo_replan`
2. 升级理由直接来自 knowledge scope 判定，不是 Markdown diff 数量
3. parent propagation 会显式命中上层汇总页
4. 即使是 `repo_replan`，执行路径仍属于 `update` 主线

## Task Impact

- `storybook + dagger` 两侧的 update-focused 专项证据现已齐备
- `4.3` 可以勾选完成
