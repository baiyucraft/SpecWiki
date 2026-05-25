## Why

`9.6` 已经把 `storybook + dagger` 的问题从“总匹配率”里剥离出来：`storybook` 的主要失真集中在 `Overview / Architecture / DomainIndex` 以及 `decomposition_profile = config_surface` 的 parent `KnowledgeUnit` 这类高层父页，它们在真实产物中承担了大量 many-to-one reuse，却没有围绕子页研究结果形成高信息密度页面；`dagger` 则不是页面质量问题，而是 runtime 停留在 `knowledge_units + research_cache`，没有推进到 `page_drafts / wiki_pages / wiki.metadata.json`。现在必须先把这两个结构性缺口收敛，否则后续迭代会继续把“父页吞页”和“流水线未装配完成”误当成普通 fidelity 波动。

## What Changes

- 收紧高层父页的 parent-consume-child contract，明确 `Overview / Architecture / DomainIndex`，以及某个 domain 下 `decomposition_profile = config_surface` 的 parent `KnowledgeUnit`，必须围绕子 `KnowledgeUnit` 的 research、digest、citation 与 diagram 输入组织页面，而不是继续依赖固定骨架或扁平 surface 列表。
- 调整 knowledge planning 与 compose 输入契约，让高层父页成为显式的一等 `KnowledgeUnit` 聚合节点；其输入必须可追溯到 child unit 集合、child digest 聚合结果和对应的 key sources。
- 为 compose pipeline 增加明确的 compose-readiness/runtime gate，要求系统在“research 已完成但 compose/assemble 未完成”时落下可恢复、可诊断的阶段状态与原因，而不是只留下零散 cache。
- 强化 runtime 持久化，确保 `page_context_cache / page_drafts / wiki_pages / pipeline_checkpoint / runtime meta` 能准确表达当前卡在 research、compose 还是 assemble，以及哪些 unit 尚未就绪。
- 更新专项验证与 reference/rdb 报告口径，让 `storybook` 必须回答高层父页 reuse 是否下降、父页是否消费 child outputs；让 `dagger` 必须回答 compose-readiness 是否达标、runtime incomplete 是否能被准确定位。
- **BREAKING**：正式 runtime 不再接受“research 存在但高层父页仍以固定模板直接成页”作为合法输出。
- **BREAKING**：正式 runtime 不再接受“只写入 research cache 而没有 page_drafts/wiki_pages，且没有明确 gate/checkpoint/reason”作为可恢复状态。

## Capabilities

### New Capabilities

- 无

### Modified Capabilities

- `knowledge-unit-decomposition`: 高层父页 `KnowledgeUnit` 的父子边界、聚合输入和稳定身份需要升级，防止 Overview/DomainIndex 以及 `config_surface` parent unit 重新退回空壳聚合页。
- `research-driven-page-composition`: 父页 compose 必须显式消费 child research/digest/citation/diagram 输入；system/domain 级页面不得继续依赖固定模板骨架主导正文。
- `repo-wiki-runtime`: runtime 必须持久化 compose-readiness、pipeline gate、阶段性中断原因与可恢复检查点，避免只剩 research cache 的不透明半成品状态。
- `workflow-verification`: 专项验证需要把 `storybook` 的高层父页 reuse 收敛和 `dagger` 的 runtime gate/readiness 收敛纳入正式验收。
- `reference-fidelity-reporting`: 报告需要把高复用页映射回具体高层 `KnowledgeUnit` 类型，并把 runtime incomplete 细化为 research/compose/assemble gate 诊断。

## Impact

- 影响 `crates/wiki-core/src/generation/knowledge_planner.rs`、`crates/wiki-core/src/workflows/page_render.rs`、`crates/wiki-core/src/workflows/research_provider.rs`、`crates/wiki-core/src/generation/compose_engine.rs`、`crates/wiki-core/src/workflows/init.rs` 及对应的 runtime 持久化路径。
- 影响 `scripts/tests/*.test.ts`、`node scripts/run-test-projects.mjs`、`node scripts/test-wiki-lifecycle.mjs` 的专项验证口径，以及 `storybook + dagger` 的 reference/rdb 分析报告。
- 影响现有高层页生成策略与 runtime 中断恢复语义，但不要求保留旧 1.x/早期 2.0 兼容层。
