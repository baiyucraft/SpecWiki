# storybook update-focused blocker check (2026-03-31)

## Scope

- change: `iteration-12-3-knowledge-first-update-mainline`
- sample: `storybook`
- objective: 只针对 `storybook` 验证 `4.3` 所需的 update-focused 证据，不扩到 `dagger`
- local date: `2026-03-31`

## Commands

- `node --input-type=module -e "callCore({ action: 'status', repoRoot: 'tmp/test/storybook', developmentMode: true })"`
- `runInitWithResume({ projectRoot: 'tmp/test/storybook', repoRootArg: 'tmp/test/storybook', initialCacheMode: 'preserve', timeoutMs: 1800000 })`
- `git -C tmp/test/storybook diff -- code/addons/a11y/src/types.ts`
- `sqlite3 tmp/test/storybook/.wiki/.cache/wiki-cache.db ...`

## Baseline Before Resume

在 `2026-03-31` 开始本轮验证前，`tmp/test/storybook/.wiki` 只有 `.cache/wiki-cache.db`，缺少：

- `.wiki/wiki.metadata.json`
- `.wiki/.knowledge/**`
- `.wiki/pages/**`

当时 `status` 结果为：

- `state=runtime_incomplete`
- `query_readiness=ready`
- `recommended_action=none`
- `affected_knowledge_scope.direct_unit_ids=[]`
- `affected_knowledge_scope.propagated_parent_unit_ids=[]`
- `affected_knowledge_scope.projection_target_page_ids=[]`

SQLite runtime 摘要为：

- `knowledge_units=226`
- `knowledge_domains=14`
- `research_cache=1`
- `page_digests=0`
- `page_drafts=0`
- `wiki_pages=0`
- `pipeline_checkpoint=0`
- `runtime_summary.runtime_state=researching`
- `runtime_summary.last_ready_stage=knowledge_planning`

## Resume Attempt

我在 `2026-03-31 22:58:41` 启动了一次 `storybook` init resume。该进程在大约 `31` 分钟后因为外层命令超时被截断，但中间态确实向前推进了：

- `research_cache: 1 -> 155`
- `researched_units: 0 -> 140`
- `compose_ready_units: 0 -> 140`
- `compose_pending_units: 226 -> 86`

截断后的 runtime 仍然停在：

- `state=runtime_incomplete`
- `runtime_summary.runtime_state=researching`
- `runtime_summary.last_ready_stage=research_unit`
- `page_digests=0`
- `page_drafts=0`
- `wiki_pages=0`
- `pipeline_checkpoint=0`

这说明当前 `storybook` 在本地环境里不是“无法启动”，而是只能非常缓慢地推进 research，中途仍未进入可验证 `projection refresh` 的 compose / assemble 产物阶段。

## A11y Probe Chain

为了避免盲选变更点，我先从现有 SQLite 中定位了一条明确的 `addons -> a11y` 链：

- module `module-8d6dc973c6b0`: `addons`, `root_paths=["code/addons"]`
- module `module-8182c18ba204`: `a11y`, `root_paths=["code/addons/a11y"]`
- unit `unit-e9a8c7073d42`: `ModuleDoc`, title=`addons`, path=`插件生态/addons.md`
- unit `unit-8a8b04c07047`: `ModuleDoc`, title=`A11y Addon`, path=`插件系统/核心Addons详解/A11y-Addon.md`

这个链理论上适合观察：

- leaf/source 命中
- parent module doc 传播
- projection target 派生

## Dirty Source Probe

我对 `tmp/test/storybook/code/addons/a11y/src/types.ts` 追加了一行临时注释，并用 `git diff` 确认该文件确实已变脏。随后立刻重新执行 `status`。

`status` 返回没有任何变化：

- `dirty_sources=[]`
- `dirty_pages=[]`
- `affected_knowledge_scope.direct_unit_ids=[]`
- `affected_knowledge_scope.propagated_parent_unit_ids=[]`
- `affected_knowledge_scope.projection_target_page_ids=[]`
- `affected_knowledge_scope.escalation.level=local_refresh`
- `affected_knowledge_scope.escalation.reason=direct_unit_refresh`

临时注释随后已撤销，`tmp/test/storybook/code/addons/a11y/src/types.ts` 当前已恢复 clean。

## Interpretation

本轮 `storybook` 样本不能作为 `4.3` 的通过证据，原因不是 `ChangeSet`/`AffectedKnowledgeScope` 主线代码已经回退，而是样本 baseline 本身没有进入 update-ready 状态：

1. `storybook` 只能推进到 `runtime_incomplete(researching)`，没有形成 `.wiki/.knowledge/**`、`page_digests`、`wiki_pages` 这类可用于核对 projection refresh 的正式产物。
2. 即使对 `code/addons/a11y/src/types.ts` 做了真实源码变更，当前 `runtime_incomplete` 状态下的 `status` 仍然没有暴露 `dirty_sources` 和 `affected_knowledge_scope`，因此不能把这份样本输出当成可信的 scope-planning 证据。
3. `pipeline_checkpoint=0`，意味着这次长流程还没有沉淀出可直接复用的 checkpoint 行，现有 preserve-resume 机制对这个样本的帮助有限。

## Task Impact

- `4.3` 仍应保持 pending。
- 这份报告是 `storybook-only` 的 blocker 证据，不是 update-focused pass。
- 本轮没有新增 `dagger` 或全量项目的结论。

## Next Actions

- 继续完成 `storybook` 的 `init`，直到至少生成 `page_digests` / `wiki_pages` / `wiki.metadata.json`，再做真正的 update-focused 变更验证。
- 或者单独修复 `runtime_incomplete` 下 `status` 对 dirty source / affected scope 的诊断可见性，再重新执行 `storybook` 专项。
