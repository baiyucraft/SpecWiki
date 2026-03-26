## Context

`DESIGN-3.0.md`、`DESIGN-RUNTIME.md` 与 `DESIGN-ITER.md` 已经把迭代 11 的边界写得很清楚：这一轮要强化 `wiki-index`，让“方法定位、入口定位、基础影响分析”优先走 facts/index，而不是继续把查询语义堆在 runtime 或 page 上。当前真实代码与真实产物都说明这件事还没有完成。

当前仓库的源码状态：

- [`crates/wiki-index/src/store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/store.rs) 已经定义了 `IndexSnapshotStore` 与 `IndexQueryStore`，但 `IndexQueryStore` 目前只暴露 `search_symbols / list_edges / trace_call_edges_from_seeds / list_processes / list_communities` 这类底层读取能力，还没有正式的 query service 与结果语义。
- [`crates/wiki-runtime/src/storage/sqlite/index_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/storage/sqlite/index_store.rs) 说明 SQLite 适配层已经能实现这些 trait，包含 `write_module_tree / replace_symbol_graph / search_symbols / trace_call_edges_from_seeds` 等入口，说明“存储适配”并不是本轮最大缺口。
- [`crates/wiki-runtime/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs) 的 `run_query(repo_root, term)` 仍然直接加载 `WikiState`，自己执行 page FTS、symbol FTS、module/source/relation 收集、graph 扩展，再拼出 `QueryReport`。也就是说，正式查询语义 owner 还在 runtime，而不是 `wiki-index`。

真实样本产物暴露出的缺口更直接：

- [`tmp/test/storybook/.wiki/.cache/wiki-cache.db`](E:/project/!byAI/spec-wiki/tmp/test/storybook/.wiki/.cache/wiki-cache.db) 中 `knowledge_units=226 / research_cache=232`，但 `modules=0 / symbols=0 / edges=0`。
- [`tmp/test/dagger/.wiki/.cache/wiki-cache.db`](E:/project/!byAI/spec-wiki/tmp/test/dagger/.wiki/.cache/wiki-cache.db) 中 `knowledge_units=83 / research_cache=93 / page_drafts=13`，但 `modules=0 / symbols=0 / edges=0`。

这说明迭代 11 的核心问题不是“页面还不够好”，而是 facts/index 在复杂仓库上还没有形成稳定、可恢复、可查询的 substrate。

参考仓库的真实源码可以借鉴两类实现思路，但都只借鉴到本轮边界为止：

- `GitNexus`
  - [`tmp/upstream/GitNexus/gitnexus/src/core/ingestion/symbol-table.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/symbol-table.ts) 通过 `fileIndex / globalIndex / callableIndex / fieldByOwner` 维持厚索引底座。
  - [`tmp/upstream/GitNexus/gitnexus/src/core/ingestion/resolution-context.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/resolution-context.ts) 用 `TieredCandidates` 把候选与 tier 绑定。
  - [`tmp/upstream/GitNexus/gitnexus/src/core/wiki/graph-queries.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/wiki/graph-queries.ts) 面向消费层暴露紧凑结果，而不是把底层图存储直接泄露出去。
- `deepwiki-rs`
  - [`tmp/upstream/deepwiki-rs/src/generator/step_forward_agent.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/step_forward_agent.rs) 通过 `DataSource`、`AgentDataConfig` 与 `required_sources` 强调“输入 contract 必须显式、缺失时应明确失败”。

因此，本设计只解决三件事：`wiki-index` 的正式查询面、facts/index 的 readiness 契约，以及 runtime 的最小 index-first adapter；不在本轮定版外部 JSON IPC 或 Agent query 协议。

## Goals / Non-Goals

**Goals:**

- 让 `wiki-index` 成为正式的查询语义 owner，而不再只是底层 `IndexQueryStore` 的集合。
- 定义最小但完整的 index 查询意图，至少覆盖 `symbol / source / module / entrypoint / callers / callees / impact_slice`。
- 定义由 `wiki-index` 拥有的紧凑结果结构，包含命中对象、命中依据、可用时的 `score` 或 graph `confidence/reason`、锚点信息与有限图切片。
- 明确 facts/index 的 readiness 契约，使其在 downstream 未完成时仍可恢复、可查询。
- 让 `wiki-runtime` 保留当前 query 入口，但内部切到 `index first` 的最小 adapter。
- 用 `storybook + dagger` 验证复杂仓库上的 index 可用性，而不是在本轮重开页面 fidelity 专项。

**Non-Goals:**

- 不在本轮定版外部 JSON IPC、CLI 或 Agent 的结构化 query 协议。
- 不在本轮把新的 query DTO 提升为 `wiki-model` 的跨层公共契约。
- 不在本轮收口 `wiki-knowledge` 的 lifecycle、projection decision 或 update 主线。
- 不在本轮处理 `storybook + dagger` 的页面 fidelity、citation、skeleton、reuse 问题。
- 不引入新的图数据库、向量库或 retrieval 真相层。
- 不把 `page_source_map` 或其他 page-owned 对象拉进 index 语义与验收口径。

## Decisions

### 决策 1：`wiki-index::query` 只能消费 facts snapshot，不得依赖 `WikiState`

本轮新增 `wiki-index` 内部查询服务，但它的输入源必须限定为 index-owned snapshot，而不能继续像 [`crates/wiki-runtime/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs) 那样直接从 `WikiState.pages / sources / modules / relations` 推导结果。

正式的数据归属收口为：

- `scan_report` 与 entry point 候选：来自 `IndexSnapshotStore::read_scan_report()`
- `module_tree` 与模块级 `entry_points / source_ids / child_ids`：来自 `IndexSnapshotStore::read_module_tree()`
- `modules / module_source_map` 真相表：由 `wiki-index` 定义读取合同，由 [`crates/wiki-runtime/src/storage/sqlite/index_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/storage/sqlite/index_store.rs) 实现
- `symbols / edges / communities / processes`：继续来自 `IndexQueryStore`

这意味着本轮必须补齐 `wiki-index` 自己拥有的读取接口，至少覆盖：

- 按 source path 读取所属 module
- 按 module 读取 source paths
- 读取 entrypoint source paths
- 读取 source 命中需要的最小 source 视图

`wiki-runtime` 仍可在 index 命中之后追加 page fallback，但 `wiki-index::query` 自身不得回读 `WikiState` 或 page 关系。

这样做的理由：

- 当前 `run_query` 的主要耦合点正是 `WikiState`，不切断这层依赖，`wiki-index` 名义上下沉，实质上仍会偷偷依赖 runtime state。
- [`crates/wiki-runtime/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/storage/sqlite_store.rs) 已经把 `modules / module_source_map` 写成 SQLite 真相表，本轮只是把其 contract owner 从“runtime 内部实现细节”提升为“wiki-index 正式读取面”。

备选方案：

- 继续让 `wiki-index::query` 通过 runtime helper 间接读 `WikiState`。
  - 否决原因：边界没有真正收回来，后续 specs 只会把耦合合法化。
- 只做 symbol/edge 查询，不补 module/source/entrypoint。
  - 否决原因：无法满足迭代 11 的“找入口、找文件、看影响”核心场景。

### 决策 2：统一 intent taxonomy，并把“可解释性”收敛到当前真实 substrate

本轮只保留一套正式 intent 枚举：

- `auto`
- `symbol_lookup`
- `source_lookup`
- `module_lookup`
- `entrypoint_lookup`
- `callers`
- `callees`
- `impact_slice`

对应的最小结果类型也只保留一套：

- `SymbolHit`
- `SourceHit`
- `ModuleHit`
- `EntrypointHit`
- `CallEdgeHit`
- `ImpactSlice`

为了避免写出“伪可解释性”，字段语义按当前代码能力收敛：

- `match_basis`：必须是当前底层可稳定判断的命中依据，例如 `symbol_fts`、`source_path`、`module_root_membership`、`entry_point_membership`、`call_trace`
- `score`：仅用于 FTS 驱动的 symbol/source 命中，直接承接当前已有的 FTS score
- `confidence` 与 `reason`：仅用于 graph-derived 命中，直接承接 `trace_call_edges_from_seeds` 与 resolved edge 上已经存在的语义
- 本轮不承诺 GitNexus 那种跨场景统一的 `tier`；如果需要排序，只允许在当前 deterministic basis 上做本地排序，不把它命名成跨仓库稳定语义

这样做的理由：

- 当前仓库真实 substrate 里，symbol 查询只有 FTS score，而 graph trace 才有 edge-level `confidence / reason`；把 GitNexus 的 `TieredCandidates` 原样套过来会直接把 specs 写虚。
- intent 命名必须和测试名、DTO 字段名完全一致，否则 `callers/callees` 与 `graph_neighbors` 混写会把后续 artifact 全部污染。

备选方案：

- 保留 `graph_neighbors` 作为泛化 intent。
  - 否决原因：对迭代 11 来说太宽，会稀释 `callers/callees/impact_slice` 这些真正需要验收的场景。
- 继续承诺统一 `tier + confidence`。
  - 否决原因：当前代码基础不支持，设计不能提前透支后续实现能力。

### 决策 3：外部 transport 本轮不升级，`term` 只映射到内部 `auto` 请求

本轮不新增 `CoreCommand.query`，也不修改 [`crates/wiki-runtime/src/transport/dto.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/transport/dto.rs) 的外部 JSON IPC 合同。外部入口继续保持：

- `CoreCommand.term`
- `run_query(repo_root, term)`

新增的结构化 query request 只在 runtime 内部存在，用于 `wiki-runtime -> wiki-index::query` 的调用边界：

- 非空 `term` 映射为 `IndexQueryRequest { intent: auto, text: term, ... }`
- 空 `term` 继续返回空结果
- 本轮不存在 `term` 与 `query` 并存、优先级冲突或兼容层双栈问题，因为根本不新增外部 `query` 字段

`QueryReport` 也暂时保留为 runtime 对外返回壳，但其 `matched_symbols / matched_sources / matched_modules / matched_symbol_edges` 必须来自 `wiki-index::query` 的投影，`matches` 里的页面结果只作为 fallback/补充 provenance，而不是主语义 owner。

这样做的理由：

- `DESIGN-ITER.md` 已明确迭代 11 的重点是 `wiki-index` 初步可用，而不是 transport/Agent 协议定版。
- 保留现有外部入口，才能把这轮变化压缩在 crate 内部边界与 runtime adapter 内，不提前把宿主升级面一并放大。

备选方案：

- 本轮直接新增外部结构化 `query` payload。
  - 否决原因：scope 过大，且会把 Agent/CLI/JSON IPC 的 breaking contract 提前拉进迭代 11。
- 保持 `term` 入口，同时继续由 runtime 自己拼全部查询结果。
  - 否决原因：这会让 `wiki-index` 仍然没有正式查询 owner。

### 决策 4：把 facts snapshot 写入从 `write_state_with_symbol_graph` 中解耦出来，并前移到 knowledge 之前

review 发现的核心事实是：当前 `init / rebuild / update` 虽然在 compose 前就执行了 `write_scan_cache` 与 `write_module_tree_cache`，但 symbol graph 的持久化仍耦在 [`crates/wiki-runtime/src/storage/state_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/storage/state_store.rs) 的 `write_state_with_symbol_graph*()` 里，而这些函数是在 compose/render/state assemble 之后才执行。这正是“已有 knowledge_units，但 modules/symbols/edges 仍为空”需要优先修复的路径。

本轮的函数级约束是：

- `scan` 完成后写入 `scan_cache`
- `build_module_tree` 完成后写入 `module-tree` cache，并刷新 `modules / module_source_map`
- `resolve/analyze graph` 完成后写入 `symbols / edges / communities / processes`
- 上述三类写入必须在 `knowledge_planning / research / compose` 之前完成
- `write_state`、page render、metadata 导出继续属于 downstream runtime/page state；它们失败时不得反向跳过或清空已经写好的 facts snapshot

实现上，本轮应把“写 facts snapshot”从 `write_state_with_symbol_graph` 中拆成独立 helper，由 workflow 在 facts 阶段直接调用；`write_state_with_symbol_graph` 这类名字要么删除，要么退化成“先写 state，再可选刷新 facts”的兼容薄壳，但不能继续作为 facts 落盘的唯一入口。

查询期的状态契约也随之明确：

- 如果 facts snapshot 尚未首次提交，query 必须返回显式的 `index not ready` 错误，而不是空命中成功
- 如果 facts snapshot 已提交，而 downstream 处于 `researching / compose_pending`，query 仍必须能回答 index-first 问题

这样做的理由：

- reviewer 已经从 `init.rs / rebuild.rs / update.rs / state_store.rs` 的真实调用关系里验证了这个耦合点，设计必须把 owner 写清楚，而不是只写一句“index 要先落盘”。

备选方案：

- 继续把 facts snapshot 写入耦在 `write_state_with_symbol_graph` 里，只调整调用顺序。
  - 否决原因：职责仍然是混的，后续很容易再次被 runtime/page state 绑回去。
- 允许 query 在 snapshot 缺失时静默返回空结果。
  - 否决原因：这会把 readiness 缺口继续伪装成“没有命中”。

### 决策 5：验证只围绕 index 可用性，不回到页面专项

本轮验收与测试只检查 index substrate 是否真正站住，核心指标为：

- `storybook + dagger` 上 `modules / module_source_map / symbols / edges` 不再为 `0`
- `symbol_lookup / source_lookup / module_lookup / entrypoint_lookup / callers / callees / impact_slice` 有稳定测试
- 在 `researching` 或 `compose_pending` 之类 downstream incomplete 状态下，index 查询仍然可用
- page fallback 只验证 provenance 标记，不验证页面 fidelity

这样做的理由：

- 历史 reference 报告已经把页面 fidelity 的主问题定位在 planner/research/compose/citation，本轮再把它们和 index 缺口绑在一起，只会让 change 重新失焦。

备选方案：

- 同时把 `storybook + dagger` 页面质量也作为本轮门槛。
  - 否决原因：验收对象会重新变成“全链路一锅炖”，不利于确认 index substrate 是否独立成立。

## Risks / Trade-offs

- [风险：`wiki-index::query` 仍会被迫回读 runtime state] → 在 contract 级别补齐 `module_source_map / entrypoint / source` 的读取面，并在实现中禁止从 `WikiState` 取数据。
- [风险：结果字段被设计成“看起来很高级”，但底层并不支持] → 只保留 `match_basis / score / confidence / reason` 这些当前真实 substrate 能稳定产出的字段，不承诺统一 `tier`。
- [风险：facts snapshot 前移后，workflow 前半段开销上升] → 复用现有 SQLite 写入路径，并通过 scoped refresh 继续限制增量更新的写入面。
- [风险：保留 `QueryReport` 会形成一轮过渡期双语义] → 明确规定 `QueryReport` 只是 runtime 对外壳，facts 命中字段必须由 `wiki-index::query` 投影生成，页面结果只做 fallback。
- [风险：`index not ready` 与“无命中”被调用方混淆] → 用显式错误区分 snapshot 未提交与查询结果为空两种状态，并补对应测试。

## Migration Plan

1. 在 `wiki-index` 扩展 snapshot/query 读取合同，补齐 `module_source_map`、entrypoint 与 source 级读取面，并由 `SqliteIndexStore` 实现。
2. 在 `wiki-index` 新增内部 query service 与最小 request/result 类型，统一 intent taxonomy 与命中字段语义。
3. 拆分独立的 facts snapshot 写入 helper，把 `scan_cache / module_tree / modules / module_source_map / symbols / edges / communities / processes` 的写入前移到 knowledge 之前。
4. 重构 `wiki-runtime::run_query`，保留 `term` 入口，但只负责把 free-text 映射成内部 `auto` 请求，并把 index 结果投影回现有 `QueryReport`。
5. 增加 focused Rust 测试与 `storybook + dagger` 验证，确认 snapshot readiness、downstream incomplete 可查询性，以及各 intent 的最小正确性。

回滚策略：

- 本轮不涉及外部 transport 合同变更，因此回滚只需撤回 `wiki-index` query service、facts snapshot helper 与 runtime adapter；
- 若 facts snapshot 前移导致 workflow 不稳定，应先回滚前移写入，再保留外部 `term` 查询入口不变；
- 不允许通过把查询语义重新塞回 runtime state 来“临时修复”本轮问题。

## Open Questions

- `SourceHit` 第一轮是否直接复用现有 `source_id + path + module_ids` 组合，还是只暴露 `path` 与 module 关系；当前倾向保留 `source_id`，避免后续 impact/source join 再次失去稳定锚点。
- `index not ready` 在 runtime 层最终是以 `io::ErrorKind::NotFound` 还是专门错误类型暴露；当前倾向先用显式错误文本与测试断言，避免本轮把 transport 错误模型也一起扩大。
