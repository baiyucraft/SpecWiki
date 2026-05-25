## Why

迭代 6.5 已经把 `symbols`、`edges` 和 `symbols_fts` 的 SQLite schema 预建出来，但当前 `wiki-core` 的真实主链仍停留在文件级事实：[`init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 还是 `scan -> module_tree -> context -> planner -> render`，[`query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/query.rs) 也只消费页面、模块、源码和关系，`symbols` 表没有任何写入路径。继续停在这个状态，后续迭代 8 的关系解析 / 图分析和迭代 9 的 LLM 增强都没有稳定的符号事实层可以依赖。

这一步现在做最合适，因为存储底座和页面 BM25 已经就位，且上游源码已经把“解析层必须单独成层”证明得很清楚：`CodeWiki` 的 `analysis_service.py -> call_graph_analyzer.py -> analyzers/*` 先统一提取节点，再做关系解析；`deepwiki-rs` 的 `workflow.rs` 和 `preprocess/mod.rs` 把结构提取、后续研究和输出明确拆开；`GitNexus` 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 则更直接，把 `structure -> parsing -> imports -> calls -> heritage -> communities -> processes` 切成独立阶段，并在 [`parsing-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/parsing-processor.ts) 中先统一抽取 definitions/imports/calls/heritage，再交给后续 processor 消费。spec-wiki 现在缺的正是这层介于 scanner 与 graph analysis 之间的 `parse_symbols`。

## What Changes

- 在 `scan_repo` 之后、`build_module_tree` 之前新增 `parse_symbols` 阶段，引入独立的符号解析层，而不是继续把更深的 AST 逻辑塞进现有 `repo/language_processors/*` 的文件级依赖提取器。
- 先把当前仓库整套 `tree-sitter` 依赖与调用从 `0.20` 系列升级到可覆盖 12 种核心语言的统一版本族，再在此基础上落地 `parse_symbols`；避免新增语言 parser 用最新 API，而旧 JavaScript/Python/TypeScript 仍停在旧 API，导致仓库内并存两套 parser 接口。
- 新增稳定的符号事实模型和解析结果模型，包括 `SymbolNode`、导出可见性判断、按文件/按名称的 `SymbolTable` 双索引，以及供迭代 8 继续消费的原始 import / call / heritage capture 结果。
- 基于 `tree-sitter` 为 12 种核心语言建立 parser registry 与 S-expression query 集合；query 组织方式参考 [`tree-sitter-queries.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/tree-sitter-queries.ts) 那种“语言常量集中管理 + coverage 单测”的工程形态。前端包装语言继续复用当前仓库已有的 React/Vue/Svelte script 提取与语言映射逻辑，而不是单独再造一套页面框架解析器。
- 把定义类符号真正写入 SQLite 的 `symbols` 表和 `symbols_fts`，不再只保留空 schema；`query` 升级为既能搜索页面，也能搜索符号名和符号文件路径。
- 升级 `init / update / rebuild` 的写盘路径，让符号解析结果和 `symbols_fts` 与页面状态一同刷新；源码变更时按受影响文件增量重解析符号，而不是每次都全量重扫所有符号。
- 为解析阶段补上字节预算、失败隔离和 fixture/项目集验证，避免单个语法错误文件或超大仓库直接拖垮整条 workflow；这里直接借鉴 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 的 `CHUNK_BYTE_BUDGET = 20 * 1024 * 1024` 分批思路，但不引入其图数据库和 worker pool 形态。

## Capabilities

### New Capabilities
- `symbol-parsing`: 定义 tree-sitter 符号解析阶段、符号模型、双索引和语言覆盖边界。

### Modified Capabilities
- `repo-wiki-workflow`: `init / update / rebuild` 需要接入 `parse_symbols` 阶段，并维护符号索引的一致性。
- `sqlite-cache-storage`: `symbols` 与 `symbols_fts` 从“预建空表”升级为真实 runtime 事实存储，并参与事务写入。
- `wiki-bm25-query`: `query` 从仅搜索页面升级为同时搜索页面与符号，并返回结构化符号命中。
- `workflow-verification`: 端到端验证需要覆盖多语言符号提取、符号检索、增量重解析和全项目集回归。

## Impact

- 主要影响 `crates/wiki-core/src/repo/*`、`crates/wiki-core/src/workflows/{init,update,rebuild,query}.rs`、`crates/wiki-core/src/storage/sqlite_store.rs` 与相关 domain/query 结构。
- 需要先升级 `crates/wiki-core/Cargo.toml` 里的 `tree-sitter` 核心 crate 与现有 JavaScript/Python/TypeScript grammar，再补齐缺失的 grammar 依赖，并围绕 parser registry、新的 query 返回结构和 SQLite upsert 路径补测试；测试粒度会直接参考 `GitNexus` 的 [`symbol-table.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/symbol-table.test.ts)、[`import-processor.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/import-processor.test.ts)、[`call-processor.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/call-processor.test.ts)、[`tree-sitter-queries.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/tree-sitter-queries.test.ts) 的覆盖方式。
- 会新增一层 parser/registry/query 代码，但不改变 `.wiki/*.md`、`wiki.metadata.json` 与 `.wiki/.cache/` 的三层 runtime 边界，也不把 `deepwiki-open` 式的消费层 RAG/cache 带进 core 事实层。
