## Why

`.docs/roadmap/implementation-roadmap.md` 已经把 3.0 的 `迭代 11` 定义为“强化 `wiki-index`，先服务 AGENT/provider 初步消费”，但当前真实实现还停在半成品状态：[`crates/wiki-index/src/store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/store.rs) 只有底层 store/query trait，正式查询语义仍主要堆在 [`crates/wiki-runtime/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs) 的混合 keyword query 里。这样继续演进，只会让 runtime 再次长成 facts 查询、图扩展、页面回填混杂的大包。

更关键的是，复杂样本的真实产物已经暴露出 index readiness 缺口：[`tmp/test/storybook/.wiki/.cache/wiki-cache.db`](E:/project/!byAI/spec-wiki/tmp/test/storybook/.wiki/.cache/wiki-cache.db) 与 [`tmp/test/dagger/.wiki/.cache/wiki-cache.db`](E:/project/!byAI/spec-wiki/tmp/test/dagger/.wiki/.cache/wiki-cache.db) 中已经有 `knowledge_units / research_cache`，但 `modules / symbols / edges` 仍为 `0`。结合历史 reference 报告里“页面 fidelity 主要阻断在 planner/research/compose/citation”的结论，本轮更合理的目标不是重开页面专项，而是先把 `wiki-index` 做成真实可查、可解释、可在 downstream 未完成时仍保持可用的 substrate。

## What Changes

- 新增 `wiki-index` 的正式查询面，覆盖 `symbol lookup / source lookup / module lookup / entrypoint lookup / callers / callees / impact slice`，不再只停留在 `search_symbols / list_edges / trace_call_edges` 这类底层读取。
- 为 `wiki-index` 收口紧凑结果结构，至少包含命中对象、命中依据、可用时的 `score` 或 graph `confidence/reason`、锚点文件或符号，以及裁剪后的图切片；这些结构只允许消费 facts snapshot，不得回读 `WikiState`、pages 或 page fallback 关系。
- 为 workflow 增加 index readiness 契约：facts/index 一旦完成可恢复的 snapshot，就必须在 downstream 的 `knowledge planning / research / compose` 未完成时仍保持可查询，不能再把“下游未完成”表现成“上游 facts 为空”。
- 让 `wiki-runtime` 的 query 路由收缩为最小 adapter，保留当前 `CoreCommand.term` 与 `run_query(repo_root, term)` 入口，但把 free-text 查询优先映射到 `wiki-index` 的 `auto` intent，再按需要做 knowledge/page fallback。
- 补充 `storybook + dagger` 的专项验证，本轮只验收 index 落盘、方法定位、入口定位、影响分析与 downstream incomplete 下的可查询性，不把页面 fidelity、citation、skeleton、reuse 混入同一轮。

## Capabilities

### New Capabilities
- `wiki-index-query-surface`: 定义 `wiki-index` 的正式查询 contract、紧凑结果结构、facts-owned 命中依据语义，以及 `impact slice` 等 index-first 返回。

### Modified Capabilities
- `repo-wiki-workflow`: 强化 workflow 对 facts/index readiness 的要求，确保 `init / update / rebuild` 在下游 knowledge 或 compose 未完成时，仍然保留可查询的 `modules / symbols / edges` 与模块快照。
- `repo-wiki-runtime`: 将 runtime 的 query route 收紧为 `index-first` 的薄 adapter，在保留当前 query 入口的前提下优先消费 `wiki-index` 查询 contract，而不是继续在 runtime 内拥有主要 facts 查询语义。
- `workflow-verification`: 新增对复杂样本 index 落盘、方法/入口/影响查询，以及“downstream 未完成但 index 仍可用”的自动化验证。

## Impact

- 重点影响 Rust facts/query 边界：[`crates/wiki-index/src/store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/store.rs)、[`crates/wiki-index/src/symbols/pipeline.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/symbols/pipeline.rs)、[`crates/wiki-index/src/symbol_graph/pipeline.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/symbol_graph/pipeline.rs)、[`crates/wiki-index/src/hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-index/src/hierarchy.rs) 与新增的 index 查询服务层；尤其包括 `module_source_map`、entrypoint 与 source 命中的读取合同归属。
- 重点影响 runtime 适配层与存储适配：[`crates/wiki-runtime/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/workflows/query.rs)、[`crates/wiki-runtime/src/storage/sqlite/index_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/src/storage/sqlite/index_store.rs) 以及 facts/index 相关 workflow。
- 重点影响验证资产与样本口径：[`crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs)、[`crates/wiki-runtime/tests/symbols/symbol_resolution.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/tests/symbols/symbol_resolution.rs)、[`crates/wiki-runtime/tests/symbols/symbol_graph_analysis.rs`](E:/project/!byAI/spec-wiki/crates/wiki-runtime/tests/symbols/symbol_graph_analysis.rs)、[`scripts/run-test-projects.mjs`](E:/project/!byAI/spec-wiki/scripts/run-test-projects.mjs)。
- 本轮不变更外部 JSON IPC query 协议；transport/Agent 正式结构化 query payload 留到后续 runtime/agent consumption 迭代统一收口。
