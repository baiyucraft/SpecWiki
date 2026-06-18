## Context

当前 `wiki-core` 的真实调用链是：

- `run_init()` -> `select_runtime_research_provider()` -> `run_compose_pipeline()` -> `render_page_draft()` -> `write_page()` / `write_metadata()`
- 对应关键代码位于：
  - `crates/wiki-core/src/workflows/init.rs`
  - `crates/wiki-core/src/workflows/page_render.rs`
  - `crates/wiki-core/src/workflows/research_provider.rs`
  - `crates/wiki-core/src/generation/compose_engine.rs`
  - `crates/wiki-core/src/generation/renderer.rs`

基于这些真实实现，`storybook` 与 `dagger` 暴露出来的问题不是抽象“页面质量差”，而是两类明确的 contract 缺口。

第一类缺口是高层父页 contract 失真。

- `run_compose_pipeline()` 在 unit research 阶段显式跳过了 `Overview / Architecture / DomainIndex`，只对其它 unit 执行 `research_provider.research_unit()`。
- `compose_unit_page()` 对这三类高层页走的是专门分支：`compose_system_page()` 只消费 `SystemResearch + domain digests`，`compose_index_page()` 只消费 `DomainResearch + child digests`。
- `compose_system_page()` 与 `compose_index_page()` 在 `compose_engine.rs` 中使用固定骨架：`简介 / 项目结构 / 核心组件 / 依赖关系分析 / 结论` 一类 section 组合仍然是主导路径。
- `collect_compose_input_digests()` 对 `Overview / Architecture` 只收集 `DomainIndex` 一层 digest，没有把更深层 leaf evidence/section contract 继续上卷。
- `ProviderBackedResearchProvider` 只增强 `research_unit()`；`research_system()` 与 `research_domain()` 仍直接走 structural 结果，因此高层页没有进入 provider-backed parent research 主线。
- 某个 domain 下、`decomposition_profile = config_surface` 的 parent `KnowledgeUnit` 虽然不走 `compose_system_page()` / `compose_index_page()`，但其 research/compose 仍主要依赖 `ConfigSurface` 固定 section plan，再在 child slot 中拼接 `render_child_digest_summary()`；本质上仍是“摘要拼接”，不是显式消费 child section/evidence/diagram contract。这里讨论的是 parent `KnowledgeUnit`，不是新增页面语义。
- `build_minimal_page_context()` 只把 `unit.scope.source_ids` 写入 `page_context_cache`，没有把 child rollup、section plan、citation digest、diagram digest 等 parent contract 输入写入 runtime。`storybook` 当前高复用页 `page_context_cache.context` 接近空壳，正与这条实现吻合。

这解释了为什么 `storybook` 的 many-to-one reuse 主要集中在高层父页，而不是均匀分布在所有 leaf 页：系统当前对高层页仍采取“特殊模板页”策略，而不是 “parent KnowledgeUnit 消费 child outputs” 策略。

第二类缺口是 runtime gate / compose-readiness 不透明。

- `save_checkpoint_and_return()` 只在 research / compose 抛出 `io::Error` 时写 `pipeline_checkpoint`。
- `load_or_compute_research()` 与 `compose_unit_page()` 没有“soft gate”概念：`provider_stop_reason`、child digest 缺失、section/diagram contract 不满足都不会形成显式 runtime gate 记录。
- `prepare_resume_state()` 只围绕单条 `pipeline_checkpoint` 与 `facts_input_hash` 决定是否恢复，不表达“哪些 unit 已 research、哪些 unit 不能 compose、哪些页面没有 assemble”。
- 更糟的是，`prepare_resume_state()` 在“没有 checkpoint”或 `facts_input_hash` 不一致时会直接清空已有 `page_drafts / page_digests`，因此只要系统没留下 checkpoint，半完成态就没有可恢复的正式状态载体。
- `init.rs` 最终才统一 `write_page()`、`write_state_with_symbol_graph()`、`write_metadata()`；如果 workflow 停在这个阶段之前，runtime 只有 cache DB 而无 Markdown/metadata，但现有状态层并不能准确解释卡点。

`dagger` 当前 `.wiki` 只有 `.cache/wiki-cache.db`，并且已知 `knowledge_units=63`、`research_cache=50`、`page_drafts=0`、`wiki_pages=0`、`pipeline_checkpoint=0`。这说明现在不是“checkpoint 太弱”这么简单，而是 runtime 根本没有一个正式的 compose-readiness / assemble-readiness contract 去解释这种半完成态。

上游源码可借鉴的，是阶段纪律与数据依赖边界，而不是产品形态：

- `CodeWiki` 的 `documentation_generator.py` 用 `get_processing_order()` 和 `generate_parent_module_docs()` 严格执行 `leaf -> parent -> overview`，证明 parent-consume-child 必须是执行纪律。
- `deepwiki-rs` 的 `OverviewEditor::data_config()` 与 `StepForwardAgent::execute()` 证明高层页 compose 依赖应是显式 required sources，缺依赖就 fail-fast。
- `GitNexus` 的 `runPipelineFromRepo()` 和 `saveMeta()` 证明 runtime 进度摘要必须是正式产物，而不只是日志。

但这些实现都不能直接搬进 core：

- 不能回退到 `module tree -> 文档树`
- 不能回读最终 Markdown 作为父页输入
- 不能用固定 doc taxonomy 替代 `KnowledgeUnit`
- 不能让 facts/graph 直接跳过 research -> compose 主链

## Goals / Non-Goals

**Goals:**

- 让 `Overview / Architecture / DomainIndex` 以及 `decomposition_profile = config_surface` 的 parent `KnowledgeUnit` 回到统一的 parent `KnowledgeUnit` contract，不再绕过 unit-scoped parent research。
- 为父页建立显式的 child-backed compose input，至少覆盖 child digests、section-scoped citation digest、diagram digest、key sources 与 child readiness。
- 在 runtime 中持久化“research 已完成但 compose/assemble 未完成”的正式 gate/readiness 状态，使 `runtime_incomplete` 可以被恢复、定位和报告。
- 让 `storybook` 的高层页 reuse 问题与 `dagger` 的 runtime incomplete 问题都能映射回可测试、可诊断的实现 contract。
- 保持主链边界仍是 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，不新增旁路生成链。

**Non-Goals:**

- 不回退到 `module/topic/family` 旧语义重新主导 planner。
- 不让父页通过回读 `.wiki/*.md` 或 child 最终 Markdown 生成正文。
- 不把 `deepwiki-open` 的 query/session/cache 修补逻辑带进 core 生成主链。
- 不在 9.7 里一次性解决所有单页文风、prompt 或 citation 密度问题。
- 不为了 `storybook` 或 `dagger` 新增样本仓库专有 planner / renderer 分支。

## Decisions

### 决策 1：高层父页必须具备 unit-scoped parent research contract，但不预先锁死函数入口

9.7 将取消当前对高层父页“只有 system/domain research、没有 unit-scoped parent research contract”的弱路径。高层父页仍然可以复用 `SystemResearch / DomainResearch` 作为 facts 级输入，但最终进入 compose 前，必须拥有与其它 parent unit 等价的 unit-scoped parent research contract。

这意味着：

- 高层父页不能继续只靠 `research_system()` / `research_domain()` 的 structural 结果直接成页。
- `research_provider` 需要支持高层父页的 provider-backed parent research，而不是只增强 leaf/普通 parent unit。
- `SystemResearch / DomainResearch` 退回为 parent research 的补充输入，而不是直接驱动最终页面 section 的唯一依据。

这里锁定的是 contract，而不是现成函数入口。实现可以选择：

- 让高层父页进入现有 `research_unit()` 路径
- 或扩展出与其等价的 system/domain compose-ready research contract

但最终都必须满足“高层父页拥有自己的 section/evidence/diagram/child-backed research 结果”这一点。

这样做的理由很直接：如果高层父页继续停留在 `SystemResearch / DomainResearch + fixed compose skeleton`，即便 child digests 更丰富，也只是把模板页喂得更满，不会真正形成 parent-consume-child contract。

备选方案：

- 保留 `compose_system_page()` / `compose_index_page()` 特殊路径，只扩充 child digests。
  - 否决原因：根因不是 child digest 不存在，而是高层页根本没有 unit-scoped parent research 与 section contract。
- 让高层页直接消费 child 最终 Markdown。
  - 否决原因：这会把 core 拉回 CodeWiki / GitNexus 的“读子页文本拼父页”模式，破坏现有 runtime contract。

### 决策 2：父页 compose 输入升级为结构化 child rollup，并坚持逐层上卷

9.7 为所有 parent unit 引入统一的 `ParentComposeInput` 概念。它不是新页面类型，而是 compose 前的结构化输入对象，至少包含：

- `child_page_digests`
- `child_section_citation_digest`
- `child_diagram_digest`
- `child_key_sources`
- `child_readiness`
- `source_lineage`

其中高层父页与普通 parent unit 的差别，只体现在 child 集合来源不同：

- `Overview / Architecture` 消费逐层上卷后的 domain-level child rollup
- `DomainIndex / config_surface parent unit / family-child` 消费本 unit 的 child units

这里必须明确一条硬约束：高层父页只能消费逐层上卷的 child rollup，不允许越过中间父节点直接抓更深层 leaf 输入。也就是说，`Overview / Architecture` 不应直接跨过 `DomainIndex` 去读 deeper leaf digests，而应通过各层 parent unit 持久化出的 child rollup 逐层汇聚证据。

`compose_engine` 不再让高层页以固定 section 骨架为默认主路径，而是改为：

- parent research 先产出 section plan
- compose 依据 section plan 消费 `ParentComposeInput`
- renderer 只负责落盘，不再补出主导内容结构的 fallback 骨架

同时，`Overview / Architecture` 的 child 汇聚不再停在 `DomainIndex` 的轻量摘要层，而是要允许 `DomainIndex` 在自身 rollup 中继续上卷 section/citation/diagram digest，避免深层 leaf 证据在进入高层页前被截断。

这条决策直接参考了：

- `CodeWiki` 的 `build_overview_structure()`：父页必须建立在 child outputs 上
- `deepwiki-rs` 的 `OverviewEditor::data_config()`：父页 compose 依赖应显式声明
- `GitNexus` 的 `PARENT_SYSTEM_PROMPT`：父页应综合 child 成果，而不是重新扫描源码

备选方案：

- 继续沿用 `PageDigest.summary` 和少量 citation 作为父页输入。
  - 否决原因：`storybook` 的 reuse 现象已经证明“轻量 summary + 固定模板”不足以支撑高层页。
- 让 renderer 自动补更多目录/附录/结构图，提升表面密度。
  - 否决原因：这只会进一步掩盖 parent contract 缺失，不会减少 coarse reuse。

### 决策 3：runtime 从“错误检查点”升级为“readiness + checkpoint”双层持久化 contract

现有 `pipeline_checkpoint` 只能表达 hard failure。9.7 将把 runtime 状态拆成两层：

- `pipeline_checkpoint`
  - 继续表示 hard interruption，例如 provider 调用失败、compose 序列化失败、写盘失败
- `runtime readiness/gate state`
  - 表示正常执行中的 readiness 状态、soft blockers、阶段进度与可恢复性

最小持久化 contract 至少需要表达：

- `unit_id`
- `unit_type`
- `research_status`
- `compose_status`
- `assemble_status`
- `last_ready_stage`
- `blocked_reason`
- `missing_dependencies`
- `updated_at`

以及 workflow 级别至少需要表达：

- `facts_input_hash`
- `workflow_action`
- `runtime_state`
- `researched_units`
- `compose_ready_units`
- `composed_units`
- `assembled_pages`
- `blocked_units`
- `last_interrupted_stage`
- `summary_reason`

这样设计的核心原因是：`dagger` 当前不是因为抛错而停下，而是系统没有正式表达“为什么还不能 compose / assemble”。如果继续把所有中断都压成单条 checkpoint，就无法解释 research partial completion 这种真实状态。

这些字段可以通过一个或多个 SQLite 表承载，也可以扩展现有 state DB；9.7 在 spec 层锁定的是持久化语义与可读字段，不预先锁死表名与拆分方式。

备选方案：

- 继续只用 `pipeline_checkpoint`，往里面加更多字段。
  - 否决原因：checkpoint 是“异常事件”，不是“持续 runtime 状态”；混在一起会让 resume 与诊断都变脏。
- 完全依赖 `research_cache/page_drafts/wiki_pages` 的存在性推导 runtime 状态。
  - 否决原因：这正是当前 `dagger` 难以定位的根因，推导口径过弱且不稳定。

### 决策 4：`page_context_cache` 不再只存最小 `source_ids`，而要持久化最小 parent contract 摘要

当前 `build_minimal_page_context()` 只保留 `source_ids`，这让 runtime 无法解释父页到底消费了哪些 child inputs。9.7 不要求把所有 research 原文塞回 `page_context_cache`，但要求它至少升级为“可诊断的 parent contract 摘要”。

对于 parent unit，缓存中至少应包含：

- `child_unit_ids`
- `child_page_ids`
- `child_digest_ids`
- `compose_contract_version`
- `citation_digest_refs`
- `diagram_digest_refs`
- `readiness_status`

对于 leaf unit，则继续保持较轻量的 source/evidence 摘要即可。

这样做的理由是：

- `page_context_cache` 是 update/rebuild/report 最容易复用的读取面；
- 如果这里仍是空壳 JSON，任何 reference/rdb 报告都只能猜“父页为什么变成模板页”；
- 但也不能把整个 final Markdown 或整段 session transcript 塞进去，避免 runtime 再长出新的 sidecar 层。

备选方案：

- 保持 `page_context_cache` 最小化，只从 `page_drafts` 反推 contract。
  - 否决原因：`page_drafts` 只描述已 compose 成功的页面，对 `dagger` 这种没 draft 的半完成态没有帮助。

### 决策 5：验收改成“父页 contract + runtime readiness”双专项，而不是继续用总分掩盖结构问题

9.7 的测试和报告按两条专项收口：

- `storybook`
  - 高层父页 severe reuse 必须下降
  - 高层父页必须能在 runtime 中追踪到 child-backed compose contract
  - 父页不能再主要由固定骨架 section 主导
- `dagger`
  - runtime incomplete 必须能定位到 research/compose/assemble 哪一层
  - 当 research partial completion 存在时，runtime 必须有 persisted readiness/gate 信息
  - 成功路径下必须落到 `page_drafts / wiki_pages / wiki.metadata.json`

这不是新增样本特判，而是把样本暴露出的真实问题沉淀成通用验收规则：

- `repo_archetype_signals`
- `high_level_parent_contract`
- `compose_readiness`
- `runtime_incomplete_diagnostics`

备选方案：

- 继续只在 reference report 里看 `overall_match_rate / reuse / skeleton`。
  - 否决原因：`dagger` 的主要问题不是 fidelity；仅靠页面对比指标无法约束 runtime gate。

## Risks / Trade-offs

- [风险：高层父页进入 unit research 后，research/compose 次数会增加] → 通过 child rollup 复用、cache contract versioning 和 leaf-first 执行顺序控制增量成本，避免父页重复重扫源码。
- [风险：runtime 持久化对象增多后，update/rebuild 的缓存失效逻辑会更复杂] → 采用最小字段集与单向写入语义，并让 input hash 继续由 facts hash + child digest hash 驱动，减少歧义。
- [风险：过度依赖 parent contract 可能让普通父页也变重] → 只要求 parent 摘要与 readiness 进入 runtime，不要求把 child 完整 research/session 全量复制到父页缓存。
- [风险：高层页取消固定骨架后，短期内章节稳定性可能下降] → section identity 仍由 section plan 的稳定 key 管理，renderer 继续负责 managed section marker，不把自由文本直接暴露给落盘层。
- [风险：如果 provider 路径本身返回部分结果，gate 可能导致更多 fail-fast] → 这是有意的；正式 runtime 应先可诊断，再谈“尽量生成一点东西”。

## Migration Plan

1. 先在 `page_render` 与 runtime storage 中引入最小 readiness/gate 持久化 contract，不动最终页面结构，先把可观测性补齐。
2. 再调整高层父页 research 路径，取消 `Overview / Architecture / DomainIndex` 跳过 unit research 的分支。
3. 在 `research_provider` 中补齐高层 parent research 输入，形成统一的 `ParentComposeInput`。
4. 收缩 `compose_engine` 的高层固定骨架，把高层页改为 section-plan 驱动。
5. 升级 `page_context_cache` 的 parent contract 摘要与对应测试。
6. 用 `storybook + dagger` 跑专项验证，确认父页 contract 与 runtime gate 两条专项都能稳定给出结论。

## Open Questions

- 是否存在少数特殊 domain 需要在 parent rollup 中追加 domain-only evidence 类型；当前倾向先复用统一 child rollup 结构，不为单一 archetype 开特例。
- readiness/gate 字段最终落在哪些 SQLite 表中，交给实现阶段基于现有 state DB 复用情况决定；spec 层不提前锁死表拆分。
