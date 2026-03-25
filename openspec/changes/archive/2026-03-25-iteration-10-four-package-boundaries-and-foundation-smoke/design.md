## Context

`DESIGN-ITER.md` 已把新的 `迭代 10` 定义为“四包拆分与基础测试收口”，但当前工作区仍只有单一 [`crates/wiki-core`](E:/project/!byAI/spec-wiki/crates/wiki-core) 成员，且 [`lib.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/lib.rs) 继续把 `domain / generation / llm / repo / storage / transport / workflows` 一并导出。当前主问题已经不是“有没有 2.0 主链”，而是这条主链是否被错误的工程边界持续反向定义。

当前最典型的污染点有五处：

- [`workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 直接串起 facts、knowledge、LLM、storage 与 runtime 写盘。
- [`workflows/page_render.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 同时持有 knowledge planning、research、compose、checkpoint 与 SQLite 持久化。
- [`domain/change_set.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/change_set.rs) 在“domain”层直接依赖 generation、repo、storage。
- [`llm/mod.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs) 横穿 scanner/hierarchy 辅助、research prompt、SQLite cache 和 section/title 推导。
- [`storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 在同一模块内同时持有 `symbols / edges / graph analysis / research_cache / page_drafts / runtime_summary / wiki_state / llm_cache`。

参考仓库的真实源码给了边界证据，但不提供产品模板：

- `deepwiki-rs` 的 [`workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs#L45) 与 [`step_forward_agent.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/step_forward_agent.rs#L32) 证明“阶段硬边界 + 显式输入合同”是必要的，但它仍是 page-first。
- `CodeWiki` 的 [`documentation_generator.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py#L249) 证明 `leaf-first -> parent-consume-child` 的装配顺序可落地，但它以 module/doc tree 为主本体，不适合照搬。
- `GitNexus` 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 与 [`symbol-table.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/symbol-table.ts) 证明厚 facts/index substrate 应先独立，不该继续埋在 runtime 壳里。

与此同时，当前产物和 reference/rdb 说明：`storybook` 仍可能停在 runtime missing，`dagger` 仍有 source grounding / skeleton / reuse 缺口。它们并不要求本轮重开 fidelity 专项，但足以说明“先拆正 crate 边界，再谈后续收敛”是必要前提。

## Goals / Non-Goals

**Goals:**

- 把工作区正式拆成 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` 四个 crate，并让依赖方向与 `DESIGN-3.0.md` 一致。
- 用“对象归属矩阵”而不是“目录搬家”定义边界，明确哪些类型属于共享模型、facts/index、knowledge contract、runtime orchestration 或 projection/write contract。
- 把当前 `sqlite_store` 切成按职责分段的 schema/adapter，并确保 `wiki-index`、`wiki-knowledge` 通过 trait/contract 与 runtime 存储解耦。
- 把当前 `llm` 横切逻辑切成 `wiki-index` 可选 assist 合同与 `wiki-knowledge` research 合同，由 `wiki-runtime` 实现，不再让 runtime 重新变成“万物中心大包”。
- 直接完成 `wiki-core -> wiki-runtime` rename，并同步 agents、scripts、dist、tests、fixtures 与工作区入口。
- 让拆层后的基础 workflow 不回退，并满足本轮收紧后的测试约束：crate/workspace 测试、生命周期脚本、`storybook + dagger` 守门样本、`COMMENTING.md` 单独检查。

**Non-Goals:**

- 不在本轮收口迭代 13 的 runtime final query route、lifecycle 完整语义或恢复链细节。
- 不在本轮重开 `storybook + dagger` 的页面质量/fidelity 专项，也不为了样本仓库新增特化 planner / renderer 分支。
- 不在本轮新增第五个 `wiki-llm`、`wiki-storage` 或 `wiki-query` crate。
- 不把 `deepwiki-rs`、`CodeWiki`、`GitNexus` 的页面语义、目录结构或产物形态直接搬进当前仓库。
- 不允许新旧 `wiki-core` 与 `wiki-runtime` 长期并存，更不允许长期依赖 re-export 假拆分。

## Decisions

### 决策 1：先定义“对象归属矩阵”，再移动代码

本轮的真问题不是文件放在哪个目录，而是哪些对象构成哪一层的正式合同。为避免“名义四包、实际单包”，先固定对象归属：

- `wiki-model`
  - `KnowledgeDomain / KnowledgeUnit / KnowledgeTree / UnitScope`
  - `SourceCitation`
  - `ModuleNode / ModuleTree / WikiRelation / WikiItem`
  - `WikiState / WikiMetadata / DirtyState`
  - 这些对象要么是跨 crate 共享对象语言，要么是 `.wiki` 正式状态/元数据格式。
- `wiki-knowledge`
  - `SystemResearch / DomainResearch / UnitResearch`
  - `PageDraft / PageDigest`
  - `PlannedPage`
  - `ComposePageContract`、projection decision、decomposition profile、section plan、skeleton profile
  - 这些对象是 KnowledgeUnit 主线上的 planning/research/compose 合同，runtime 只能消费，不能重新解释。
- `wiki-runtime`
  - `PageContext`
  - `PipelineRuntimeSummary / UnitRuntimeGate`
  - `ChangePlan`
  - `ExportContext`
  - `ManagedSectionBlock / UserSectionBlock / PageMergePlan`
  - 这些对象表达 runtime orchestration、storage、projection merge 或 lifecycle 语义，不进入共享模型白名单。

字段级切割要求同步固定：

- `WikiState`
  - 进入 `wiki-model` 的只有 `WikiPageState / SourceState / BuildState / WikiState` 这些正式 DTO。
  - 当前 [`state.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/state.rs) 中依赖 `PlannedPage / SectionDraft / ScanReport` 的 `PageBuildResult`、`assemble_state*`、`build_*` helper 一律留在 `wiki-runtime`。
  - 也就是说，`WikiState` 会被瘦身成“共享正式状态对象”，不会把 builder/helper 一起搬去 model。
- `PlannedPage`
  - 作为 knowledge 侧 projection decision 保留在 `wiki-knowledge`，但会收缩为“稳定投影计划 DTO”。
  - runtime/query 继续使用自己的 `WikiPageState` 或等价 projection view，不允许再把 `PlannedPage` 当 runtime 真相对象。
  - `PlannedPage` 不得被持久化为 runtime 真相对象，也不得作为 query 结果的长期状态载体。
  - 是否最终保留 `PlannedPage` 这个类型名是实现细节；边界真相是“knowledge-side projection plan 与 runtime-side page state 分离”。
- `WikiMetadata`
  - 只把 `.wiki/wiki.metadata.json` 的正式文件格式留在 `wiki-model`。
  - [`metadata_mapper.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/metadata_mapper.rs) 的导出逻辑、`ExportContext` 和任何持久化 helper 都留在 `wiki-runtime`。
- `DirtyState`
  - 若保留在 `wiki-model`，只表示正式状态对象；
  - 状态迁移、回退原因推导、`fresh/stale/needs_rebuild` 计算逻辑全部留在 `wiki-runtime`。

这样划分的理由是：

- `wiki-model` 只持有“被多个 crate 共享、且脱离具体执行器仍成立”的正式对象；
- `wiki-knowledge` 持有“KnowledgeUnit 主线上的中间合同和投影决策”；
- `wiki-runtime` 持有“执行、恢复、写盘、marker、merge、gate、transport”。

备选方案：

- 继续按现有 `domain / generation / repo / workflows` 目录整体搬家。
  - 否决原因：这会把 `PageContext`、`ManagedSectionBlock`、`PipelineRuntimeSummary` 这类横切对象错误带过去，形成假拆分。
- 把几乎所有 DTO 都塞进 `wiki-model`。
  - 否决原因：会让 `wiki-model` 重新长成“大一统 DTO 包”，掩盖真正的层边界。

### 决策 2：`generation/**` 不能整包进入 `wiki-knowledge`，必须拆成 knowledge contract 与 runtime projection 两半

当前 [`renderer.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/renderer.rs)、[`managed_sections.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/managed_sections.rs) 和 [`sections.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 已经包含 managed marker、Markdown 装配、页面 merge 和 section 草稿 contract，它们本质上属于 runtime projection/write contract，不是纯 knowledge。

因此本轮按职责拆分：

- 进入 `wiki-knowledge`
  - `knowledge_planner.rs`
  - `research_engine.rs`
  - `compose_engine.rs`
  - 与 `KnowledgeUnit -> section_plan -> PageDraft/PageDigest` 有关的纯 knowledge 合同
- 留在 `wiki-runtime`
  - `renderer.rs`
  - `managed_sections.rs`
  - `sections.rs` 中与 `SectionDraft`、managed marker、`build_page_compose_plan()`、page merge、Markdown section 组装有关的部分

这也意味着 `sections.rs` 不能整文件移动，而是按职责切开：Knowledge 侧保留 `PlannedSection`、`SkeletonProfile` 一类 section 规划对象；runtime 侧保留 `SectionDraft`、section slot、managed marker 与最终 Markdown 组装。

备选方案：

- 把整个 `generation/**` 视为 knowledge。
  - 否决原因：runtime projection/write contract 会被误吸进 knowledge 层，违背 `page 是结果、runtime 负责投影和持久化` 的边界。
- 把 renderer 也塞回 runtime，但保留 `sections.rs` 整体在 knowledge。
  - 否决原因：`sections.rs` 当前同时含有 runtime section draft 和 compose-plan 组装逻辑，不拆会继续制造循环依赖。

### 决策 3：LLM 横切层按“facts assist 合同”和“knowledge research 合同”切开，`wiki-runtime` 只提供实现

当前 [`llm/mod.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs) 同时参与：

- file-purpose assist
- hierarchy/module tree 辅助
- research prompt 与 provider 调用
- SQLite prompt cache
- section/title 辅助逻辑

如果简单把整个 `llm` 文件移动到 `wiki-runtime`，runtime 会重新吞掉 facts 与 knowledge 细节。因此本轮把合同拆成两类：

- `wiki-index::assist::FactsAssist`
  - 负责 scanner / hierarchy 的可选不确定性辅助，例如 file purpose、module grouping、structure hint。
  - `wiki-index` 必须始终可在无 assist 情况下走 deterministic 路径。
- `wiki-knowledge::research::ResearchProvider`
  - 负责 `system / domain / unit` research 合同。
  - `wiki-knowledge` 只依赖 trait，不依赖 provider transport、prompt cache 或宿主协议。

`wiki-runtime` 拥有：

- provider 配置
- prompt/cache 实现
- Agent bridge / JSON IPC
- 这些 trait 的 concrete adapter

本轮不新增 provider-backed compose 能力，compose 继续消费已稳定的 research contract。这是故意的边界收缩，用来避免 iter 10 顺带变成 LLM/消费层扩张。

备选方案：

- 让 `wiki-index`、`wiki-knowledge` 继续直接调用 runtime 内的 LLM 代码。
  - 否决原因：会形成反向依赖，四层边界立刻失效。
- 在 iter 10 顺手引入单独 `wiki-llm` crate。
  - 否决原因：违反当前阶段“明确不拆 `wiki-llm`”的设计边界。

### 决策 4：SQLite 具体实现仍留在 `wiki-runtime`，但 schema 和 API 按 `index / knowledge / runtime` 三段分治

由于本轮不拆 `wiki-storage`，`rusqlite` 具体实现仍放在 `wiki-runtime`。但必须显式拆出三段 schema/adapter，避免 `wiki-index` 和 `wiki-knowledge` 回调 runtime 细节。这里的关键约束不是“runtime 里有几个文件”，而是：

- `wiki-index` 自己定义它需要的 store trait 与 query trait；
- `wiki-knowledge` 自己定义它需要的 store trait；
- `wiki-runtime` 只实现这些 trait，不得拥有这些 trait 的定义权；
- 任何跨层读库都必须经过对应 crate 暴露的 trait/contract。

表级真相源矩阵固定如下：

- `scan_cache / modules / module_source_map / symbols / symbols_fts / edges / communities / community_members / processes / process_steps / graph diagnostics`
  - schema contract：`wiki-index`
  - read/write API：`wiki-index` 定义 trait，`wiki-runtime::storage::sqlite::index_store` 实现
  - truth kind：facts/index formal snapshot
- `research_cache / page_digests / page_drafts`
  - schema contract：`wiki-knowledge`
  - read/write API：`wiki-knowledge` 定义 trait，`wiki-runtime::storage::sqlite::knowledge_store` 实现
  - truth kind：derived knowledge cache / compose artifact
- `wiki_pages / wiki_page_sections / source_states / page_source_map / page_module_map / wiki_relations`
  - schema contract：`wiki-runtime`
  - read/write API：`wiki-runtime`
  - truth kind：runtime projection / query-ready state
- `runtime_meta / pipeline_checkpoint / unit_runtime_gates`
  - schema contract：`wiki-runtime`
  - read/write API：`wiki-runtime`
  - truth kind：runtime lifecycle / readiness truth
- `llm_cache`
  - schema contract：`wiki-runtime`
  - read/write API：`wiki-runtime`
  - truth kind：runtime-owned derived cache

其中两条需要特别钉死：

- `modules` 不再算 runtime_store；它表达 module tree / facts substrate，因此归 `wiki-index` 合同。
- `wiki_relations` 只表达最终 page/query 投影关系，不再承载 facts 或 knowledge 的正式关系真相。

`wiki_pages / page_drafts / page_digests` 的关系也固定如下：

- `page_drafts`：knowledge compose artifact，属于可丢弃、可重算的 derived cache；
- `page_digests`：knowledge-side child/parent rollup artifact，属于 derived cache；
- `wiki_pages`：runtime 投影后的正式页面状态，供 query、sync、update、rebuild 读取。

据此拆成三段：

- `wiki-runtime::storage::sqlite::index_store`
  - tables: `scan_cache`、`modules`、`module_source_map`、`symbols`、`symbols_fts`、`edges`、graph analysis 相关表
  - contract owner: `wiki-index`
  - runtime 只实现 `IndexSnapshotWriter / IndexQueryReader` 一类 trait
- `wiki-runtime::storage::sqlite::knowledge_store`
  - tables: `research_cache`、`page_digests`、`page_drafts`
  - contract owner: `wiki-knowledge`
  - runtime 只实现 `ResearchCacheStore / ComposeArtifactStore`
- `wiki-runtime::storage::sqlite::runtime_store`
  - tables: `wiki_pages`、`wiki_page_sections`、`source_states`、`modules`、`wiki_relations`、`runtime_meta`、`pipeline_checkpoint`、`unit_runtime_gates`、`llm_cache`
  - contract owner: `wiki-runtime`
  - 负责生命周期、恢复、query route 骨架、managed section merge 与 prompt cache

约束：

- `wiki-index` 与 `wiki-knowledge` 只能依赖模型类型和 trait，不能 `use wiki_runtime::storage::*`
- `wiki-runtime` 可以依赖 `wiki-index` 与 `wiki-knowledge` 并实现其持久化 trait
- schema 仍在 runtime 落盘，但“哪些表表达哪层合同”必须明确可读
- 读路径也必须收紧：
  - facts/index-first query 只能优先读 `index_store`
  - research/compose 恢复只能优先读 `knowledge_store`
  - lifecycle / projection / query-ready state 只能优先读 `runtime_store`

备选方案：

- 让 `wiki-index` 自己持有 SQLite 实现。
  - 否决原因：当前阶段不拆 `wiki-storage`，会过早扩大 crate 数量与存储重复实现成本。
- 继续保留单一 `sqlite_store.rs`。
  - 否决原因：横切表继续混在一起，之后谁拥有哪张表会永远说不清。

### 决策 5：`wiki-core -> wiki-runtime` 直接 rename，本轮一次切完调用面

既然当前阶段不要求兼容层，本轮直接把 `wiki-core` 收缩并 rename 为 `wiki-runtime`。迁移面必须一次写清：

- workspace members 与 Rust package/bin 名称
- [`agents/codebuddy/src/runtime/invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts) 与 [`resolveBinary.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/resolveBinary.ts) 的 binary 名
- 根脚本与 dist/publish/build 入口
- 测试脚本、fixture 路径、distribution tests、streaming/e2e tests
- 文档与 OpenSpec 验证命令

这样做的原因是：

- 继续保留 `wiki-core` 兼容 alias 只会让单包心智延续到后续迭代；
- Agent 边界本来就要求 thin，rename 只应影响二进制定位和脚本入口，不应引入业务逻辑分叉。
- `wiki-runtime` 只允许对外暴露 workflow/storage/transport/query/lifecycle API surface，不得重新导出 `wiki-index` 或 `wiki-knowledge` 的内部实现模块给外部直接使用。
- `wiki-model` 禁止依赖 IO、SQL、transport、prompt、render helper；若迁移后出现这类依赖，视为边界失效。

备选方案：

- 同时保留 `wiki-core` 与 `wiki-runtime` 两套入口一段时间。
  - 否决原因：会让测试、发布、Agent 分发长期维持双命名，成本高且容易遮蔽真正的拆层问题。

### 决策 6：迁移按“切断 back-edge 后立即删旧入口”的顺序推进，不允许长期 re-export

迁移顺序固定为：

1. 建立四个 crate 和依赖方向，先让 workspace 能表达目标边界。
2. 先抽 `wiki-model` 最小共享对象白名单。
3. 再抽 `wiki-index` 的 facts/index 实现与 `FactsAssist` trait，切断 runtime 对 facts 主实现的直接拥有。
4. 再抽 `wiki-knowledge` 的 planning/research/compose 合同与 `ResearchProvider` trait，切断 runtime 对 knowledge 主实现的直接拥有。
5. 最后把 `wiki-runtime` 收成 orchestration/storage/transport/projection 壳，并完成 rename 与脚本切换。
6. 每一阶段移动完成后立即删除旧入口或旧 `pub use`，不允许新旧并存。

这里允许极短期编译垫片，但不允许“新 crate 暴露 API，旧 crate 继续作为真正实现”跨越一个阶段以上。每个阶段的退出条件都是“旧入口被删、workspace 仍可编译与测试”。

备选方案：

- 先复制四个 crate 壳，再慢慢把旧实现一点点转发过去。
  - 否决原因：这正是 reviewer 指出的“假拆分”，会让旧 wiki-core 心智继续存活。

## Risks / Trade-offs

- [风险：`sqlite_store` 横切太重，拆分中容易形成 trait 过多或序列化重复] → 先按 `index / knowledge / runtime` 三段表族切分，保持同一 SQLite backend，但只暴露分段 adapter；避免引入第二套存储实现。
- [风险：`llm/mod.rs` 同时服务 facts assist 与 knowledge research，抽 trait 时容易漏掉 scanner/hierarchy 的辅助入口] → 先列出现有 assist 调用点，再用 `FactsAssist` 白名单收口；任何未被列入白名单的方法不得继续直接跨 crate 调用。
- [风险：`WikiState / WikiMetadata` 放到 `wiki-model` 后，runtime 仍可能把执行态对象偷偷一起塞进去] → 用对象归属矩阵做白名单；`PageContext / ChangePlan / PipelineRuntimeSummary / ManagedSectionBlock` 明确排除在 `wiki-model` 外。
- [风险：rename 会同时影响 Rust、TS、dist、tests、fixtures 和用户脚本] → 在 tasks 中把 rename 单独列成一组迁移项，并通过 distribution/e2e/agent tests 兜底，不留“后面顺手改”的灰区。
- [风险：为了满足拆层，可能诱发提前实现 iter 13 的 query route/runtime finalization] → 明确本轮只收边界、依赖方向和基础 smoke；query route/lifecycle completeness 仍留到 iter 13。
- [风险：`storybook + dagger` 当前已经脆弱，结构重构可能进一步暴露 runtime/workflow 退化] → 把它们定义为守门样本，只要求“不更坏”，不把 fidelity 收敛当成本轮通过条件。

## Migration Plan

1. 扩展 workspace，新增 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` crate，并建立目标依赖方向。
2. 先移动共享模型白名单，修正跨 crate `use`，并删除旧 `wiki-core::domain` 中对应定义。
3. 抽出 `wiki-index` 的 scanner/symbol/graph/hierarchy 主实现和 `FactsAssist` trait；runtime 只保留 adapter 调用点。
4. 抽出 `wiki-knowledge` 的 planner/research/compose 主实现和 `ResearchProvider` trait；把 projection/write contract 留在 runtime。
5. 把 `sqlite_store` 拆成分段 store/adapter，并按 `index / knowledge / runtime` 表族重新整理读写 API。
6. 完成 `wiki-core -> wiki-runtime` rename，统一 agents、scripts、dist、tests、fixtures、docs 与命令入口。
7. 删除旧 `wiki-core` 入口和兼容转发，确保工作区不再存在平行实现。
8. 执行 crate/workspace 测试、生命周期脚本、`storybook + dagger` 守门样本与注释规范检查，确认拆层后基础 workflow 不回退。

回滚策略：

- 本轮是结构重命名与边界收口，默认通过 Git 回滚整个 change，而不是在代码里保留兼容分叉。
- 若中途发现某一阶段切分导致主链失稳，应回滚该阶段提交并重做对象归属或 trait 边界，而不是引入长期 alias。

## Open Questions

- `wiki-model` 是否需要在 iter 10 一次性承接全部 symbol/graph DTO，还是先只承接 `ModuleNode / ModuleTree / WikiRelation / WikiState / WikiMetadata` 等已经被 runtime 正式消费的对象；当前倾向先收白名单最小集，避免模型包过早膨胀。
- `sections.rs` 的拆分粒度是否需要独立成两个文件（knowledge section planning vs runtime section drafting），还是先在同名模块下拆子模块；当前倾向直接拆子模块，减少后续再搬一次的成本。
