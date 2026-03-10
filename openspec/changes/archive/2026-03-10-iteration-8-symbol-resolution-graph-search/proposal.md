## Why

迭代 7 已经把 `parse_symbols` 接到主链，并把 definitions 写入 `symbols` / `symbols_fts`，但当前事实层仍停在“可检索符号名”而不是“可推理符号关系”。从本仓库真实代码看，[`crates/wiki-core/src/repo/symbols/models.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/symbols/models.rs) 里的 `imports / calls / heritage` 仍只是未解析的 raw capture，[`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 里的 `edges / communities / processes` 仍未承载真实业务数据，[`crates/wiki-core/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/query.rs) 也还不能回答调用链、影响范围和跨模块协作问题。继续停在这个层级，后续 planner、query 和 LLM 增强都缺少可依赖的图事实层。

上游真实实现已经把下一步路径证明得很清楚。GitNexus 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 不是把 tree-sitter 结果直接喂给搜索，而是显式拆成 `parsing -> imports -> calls -> heritage -> communities -> processes`；[`import-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/import-processor.ts) 和 [`call-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts) 分别围绕 `SuffixIndex`、resolve cache、`import-resolved / same-file / fuzzy-global` 置信度做独立解析；CodeWiki 的 [`call_graph_analyzer.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analysis/call_graph_analyzer.py) 也是先收集节点与原始关系、再统一 resolve / dedupe；deepwiki-rs 的 [`workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs) 与 [`preprocess/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/mod.rs) 则说明这些中间结果必须先收敛成稳定上下文，再交给后续生成与消费层。基于这些实际源码，迭代 8 最合理的目标就是把 spec-wiki 从“definitions-only”推进到“可解析关系、可分析图、可回答链路”的事实层。

## What Changes

- 在 `parse_symbols` 之后新增独立的 symbol resolution 阶段，把 raw `import / call / heritage` capture 解析为稳定的 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges，而不是继续停留在仅 definitions 可检索的状态。
- 为 symbol resolution 引入可复用的解析上下文：后缀索引、语言专用 import resolver、resolve cache、调用目标置信度分层和 built-in/noise 过滤，并把这些逻辑留在独立层，而不是塞回 scanner 或 query。
- 补齐 Vue / Svelte 单文件组件的 `<script>` / `<script setup>` 包装层解析，委托到底层 JS / TS parser，并把符号与关系行号映射回原始组件文件。
- 在 SQLite 中把 `edges`、`communities`、`community_members`、`processes`、`process_steps` 从预留空表升级为真实 runtime 数据，并让 `init / update / rebuild` 维护其事务一致性和按文件增量刷新。
- 新增图分析阶段：基于解析出的 symbol graph 做社区检测、入口点评分与执行流追踪、Tarjan SCC 环检测与断边/拓扑排序，形成后续 planner 和 query 可消费的图事实。
- 升级 `query`：在保留页面 / symbol BM25 的基础上，新增基于图的结构化查询与结果合并，返回调用链、影响范围、process 命中和来源 provenance，而不是只回填页面与源码。
- 升级 hierarchy / context / planner：消费 `IMPORTS / CALLS` 强化模块间关系推断，消费 communities / processes 强化 workflow 页面与页面上下文，而不是继续只依赖目录与 manifest 启发式。
- 扩展验证链路：新增关系解析精度、Vue/Svelte wrapper、graph query、community/process 生成、增量 edge refresh 和 19 项目集图事实回归验证。

## Capabilities

### New Capabilities
- `symbol-resolution`: 定义 raw symbol capture 到稳定 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges 的解析、置信度、增量刷新与持久化契约。
- `symbol-graph-analysis`: 定义基于 symbol graph 的 communities / processes / cycle detection、持久化以及面向 planner/query 的消费契约。

### Modified Capabilities
- `symbol-parsing`: 扩展 parser 输出契约，补齐 Vue / Svelte 包装层解析，并要求为 resolution 阶段提供足够的 raw capture 元信息。
- `repo-wiki-workflow`: 主链从 `scan -> parse_symbols -> module_tree -> planner -> render` 升级为包含 resolution 与 graph analysis 的完整事实链，且 `update / rebuild` 必须维护 edges/graph state 一致性。
- `repo-hierarchy-model`: 模块关系与 workflow 页面规划改为消费 `IMPORTS / CALLS`、communities 和 processes，而不是只依赖目录启发式与 CI 文件线索。
- `sqlite-cache-storage`: `edges / communities / processes` 从空 schema 升级为真实写盘数据，并参与事务刷新与增量清理。
- `wiki-bm25-query`: query 从“页面/符号 BM25 + 结构化回填”升级为“BM25 + 图查询 + merge/rank”，新增关系和执行流视图。
- `workflow-verification`: 验证必须覆盖关系解析、graph analysis、graph query、Vue/Svelte wrapper、增量 edge refresh 和项目集图事实分析。

## Impact

- 主要影响 `crates/wiki-core/src/repo/symbols/*`、新增的 graph/resolution 子层、`crates/wiki-core/src/workflows/{init,update,rebuild,query}.rs`、`crates/wiki-core/src/storage/{sqlite_store,state_store}.rs`、`crates/wiki-core/src/generation/{context,planner,renderer}.rs` 与对应测试。
- 需要为图遍历、SCC 和社区检测引入或复用图算法依赖，但不会引入 GitNexus 的 KuzuDB、deepwiki-open 的 embedding/RAG pipeline，SQLite 仍是唯一持久化后端。
- `query` 的返回结构会增加 relation/process 相关字段与 provenance 信息，但应保持向后兼容的增量扩展，而不是重写现有 page/module/source/symbol 返回面。
