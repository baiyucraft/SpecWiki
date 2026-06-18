## Context

当前仓库已经具备进入符号层的几个前提，但真正的 `parse_symbols` 还不存在：

- [`crates/wiki-core/src/workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 仍然是 `scan_repo_with_boundary -> build_module_tree -> build_repo_context -> plan_pages -> render_page_bundle`，没有任何符号解析阶段。
- [`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 已经建好了 `symbols`、`edges`、`symbols_fts` 等表，但当前只有 schema，没有读写路径。
- [`crates/wiki-core/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/query.rs) 只会收集页面、模块、源码和关系命中，`QueryReport` 没有 symbol 视图，`collect_fts_page_matches` 也只查 `wiki_pages_fts`。
- [`crates/wiki-core/src/repo/language_processors/javascript.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/language_processors/javascript.rs) 与 [`crates/wiki-core/src/repo/language_processors/python.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/language_processors/python.rs) 已经在“文件级依赖提取”里使用 tree-sitter；说明仓库已经接受 tree-sitter 作为 parser 依赖，但当前抽象层只覆盖“依赖谁”，没有覆盖“定义了什么符号”。

这次设计不只参考文档，而是直接对照借鉴仓库的真实实现：

- `CodeWiki`
  - [`analysis/analysis_service.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analysis/analysis_service.py) 先做 `RepoAnalyzer`，再调 `CallGraphAnalyzer`；解析层和上层服务边界很清楚。
  - [`analysis/call_graph_analyzer.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analysis/call_graph_analyzer.py) 先遍历所有文件收集节点和原始关系，再统一 `_resolve_call_relationships()` 和 `_deduplicate_relationships()`；这证明“先解析节点、后做全局关系解析”是更稳的两阶段结构。
  - [`analyzers/typescript.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analyzers/typescript.py) 先 `_extract_all_entities()`、再 `_filter_top_level_declarations()`、最后 `_extract_all_relationships()`；说明 parser 输出里应同时保留 definitions 和未决 capture，而不是只返回最终边。
  - [`analyzers/python.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analyzers/python.py) 用 Python AST 而不是 tree-sitter，提醒我们“不同语言最佳解析器不同”；但当前仓库的设计边界已明确为 tree-sitter，因此这里作为备选思路而不是本轮主方案。
  - [`topo_sort.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/topo_sort.py) 的 Tarjan / DFS 是图分析层能力，不应挤进本轮的 parser change。
- `deepwiki-rs`
  - [`src/generator/workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs) 把 preprocess / research / compose / outlet 切成显式阶段。
  - [`src/generator/preprocess/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/mod.rs) 和 [`src/generator/context.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/context.rs) 说明“阶段结果先收敛到统一上下文，再给后续阶段消费”比把所有逻辑塞进一个 workflow 更可维护。
  - [`src/generator/preprocess/extractors/language_processors/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/extractors/language_processors/mod.rs) 用 manager 分发语言实现，这和当前 spec-wiki 的 `LanguageProcessorManager` 结构兼容。
- `GitNexus`
  - [`src/core/ingestion/pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 明确分成 `structure -> parsing -> imports -> calls -> heritage -> communities -> processes` 多阶段，并用 `CHUNK_BYTE_BUDGET = 20 * 1024 * 1024` 做分批解析；这直接证明 parser 不应该和 query / graph enrichment 混成一坨。
  - [`src/core/ingestion/parsing-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/parsing-processor.ts) 通过 `DEFINITION_CAPTURE_KEYS` 把 definition capture 统一收口，同时保留 imports/calls/heritage/routes 的原始提取；这非常接近 spec-wiki 本轮“先 definitions，关系留到迭代 8”的边界。
  - [`src/core/ingestion/symbol-table.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/symbol-table.ts) 用 `fileIndex + globalIndex` 双索引支持 same-file 和 fuzzy-global 两类解析；其配套测试 [`test/unit/symbol-table.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/symbol-table.test.ts) 还明确暴露了“同文件同名 last wins，全局索引 append”的语义，这既值得借鉴，也提醒我们避免直接照搬。
  - [`src/core/ingestion/import-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/import-processor.ts) 的 `ImportResolutionContext`、`SuffixIndex` 和 resolve cache 说明跨文件解析需要单独的解析上下文；这也是为什么本轮不应该提前承诺边解析边持久化 `edges`。
  - [`src/core/ingestion/call-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts) 把 `same-file / import-resolved / fuzzy-global` 解析优先级和置信度固定成显式策略，对应单测 [`test/unit/call-processor.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/call-processor.test.ts)；这进一步说明 symbol definitions 和关系解析应拆成两轮。
  - [`src/core/ingestion/tree-sitter-queries.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/tree-sitter-queries.ts) 与 [`test/unit/tree-sitter-queries.test.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/test/unit/tree-sitter-queries.test.ts) 把 12 种语言 query 常量和 coverage 测试绑定在一起，这比散落在实现文件里的匿名 query 更便于演进。
  - [`src/core/search/hybrid-search.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/search/hybrid-search.ts) 和 [`src/core/kuzu/schema.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/kuzu/schema.ts) 表明 GitNexus 的后续路径是“symbol graph + hybrid search”；这对迭代 8/9 有参考价值，但不是本轮最小实现边界。
- `deepwiki-open`
  - [`api/data_pipeline.py`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-open/api/data_pipeline.py) 把所有文件当纯文本读取并做 embedding。
  - [`api/websocket_wiki.py`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-open/api/websocket_wiki.py) 每次请求构造 RAG 实例。它适合作为消费层参考，但反过来也证明 symbol parsing 不该混进 query/RAG 层。

设计约束：

- 必须继续遵守 `Repository Scan -> ModuleTree -> Page Planner -> Wiki Assembler` 主路径；新增 `parse_symbols` 只能插在 scanner 与 module tree 之间，不能绕过现有 pipeline。
- Agents 继续保持 thin boundary；LLM、RAG、embedding、graph analysis 都不进入本迭代，也不直接引入 GitNexus 的 Kuzu graph schema、community/process enrichment 或 hybrid search。
- 不能无视当前代码现状强行引入第二套解析后端；但当前仓库如果要接入 12 种核心语言，必须先把已有的 `tree-sitter = 0.20` 栈整体升级到与新增 grammar 兼容的统一版本族，避免仓库内长期并存两套 parser API。

## Goals / Non-Goals

**Goals:**

- 在 scanner 之后新增独立的 `parse_symbols` 阶段，建立从 `ScannedFile` 到 `SymbolNode` / `SymbolTable` 的稳定转换。
- 为 12 种核心语言提供 tree-sitter parser registry 和统一 capture 抽取能力，并让 React(JSX/TSX) 直接委托到底层 JavaScript / TypeScript parser；Vue/Svelte 包装语言 symbol parsing 延后到迭代 8。
- 把定义类符号真正写入 `symbols` 和 `symbols_fts`，让 `query` 具备符号名 / 文件路径检索能力。
- 让 `init / update / rebuild` 维护 symbol snapshot 的一致性，并在源码增删改后按文件增量刷新。
- 为后续迭代 8 预留原始 import / call / heritage capture 的内存模型和接口，不在本轮提前实现全局关系解析。
- 让解析阶段具备字节预算、失败隔离和 deterministic 顺序，避免大型仓库或坏文件拖垮整条 workflow。

**Non-Goals:**

- 本迭代不实现 `resolve_imports`、`resolve_calls`、`resolve_heritage` 的跨文件解析，不向 `edges`、`communities`、`processes` 写真实业务数据。
- 本迭代不让 module tree、planner、page renderer 直接消费 symbol graph 做页面内容增强；页面信息密度提升留给迭代 8/9。
- 本迭代不引入 Python 原生 AST、Go 原生 parser 或其他双后端混合方案。
- 本迭代不引入向量检索、RRF、TOON 或 LLM explain layer。
- 本迭代不改变 `.wiki/*.md`、`wiki.metadata.json` 与 `.wiki/.cache/` 的产物边界。

## Decisions

### 决策 1：新增 `repo::symbols` 子层，而不是继续扩张 `repo::language_processors`

**Decision**

- 在 `crates/wiki-core/src/repo/` 下新增独立的 `symbols/` 子层，例如：
  - `models.rs`：`SymbolNode`、`RawImportCapture`、`RawCallCapture`、`RawHeritageCapture`、`ParsedFileSymbols`
  - `registry.rs`：`SymbolParser` trait 和 registry
  - `queries/*.scm`：各语言 S-expression query
  - `frontend.rs`：后续迭代预留的包装语言委托逻辑（本轮仅 React JSX/TSX 直连底层 parser）
  - `pipeline.rs`：批量解析、聚合和 `SymbolTable` 构建
- `repo/language_processors/*` 继续只服务 scanner 的“文件级依赖目标 / alias”提取；不承担 symbol parser 责任。

**Rationale**

- 当前 `LanguageProcessor` trait 只暴露 `extract_dependency_targets` 和 `extract_declared_aliases`，把 symbol parsing 硬塞进去会把文件级启发式和 AST 级事实层混在一起。
- `CodeWiki` 把 `CallGraphAnalyzer` 和 `analyzers/*` 单独成层，`deepwiki-rs` 也把 preprocess extractor 和后续阶段切开，`GitNexus` 更是直接在 `pipeline.ts` 里把 parsing 与 imports/calls/heritage 拆成相邻但独立的 processor；三者都指向同一个工程结论：parser 层应该独立存在。

**Alternatives considered**

- 方案 A：直接扩展 `LanguageProcessor` trait
  - 否决原因：职责污染，且 scanner 每次读取源码时都会被迫携带 symbol 解析上下文。
- 方案 B：把 parser 写进 workflow
  - 否决原因：workflow 只应编排阶段，不应承载语言分发和 AST 细节。

### 决策 2：迭代 7 只把“定义类符号”持久化到 SQLite，原始关系 capture 先保留为内存模型

**Decision**

- `symbols` / `symbols_fts` 在迭代 7 起承载真实的 definition 节点。
- `edges` 继续只保留空 schema，不在本轮写入 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS`。
- parser 仍会返回 `RawImportCapture`、`RawCallCapture`、`RawHeritageCapture`，但这些 capture 只在当前 workflow 内存中存在，为迭代 8 的 `resolve_*` 接口预留形态。

**Rationale**

- `CodeWiki` 的实际实现就是“先抓实体，再全局解析关系”。`GitNexus` 也是同一路径：`parsing-processor.ts` 先产出 definitions/imports/calls/heritage 原始结果，再由 `import-processor.ts`、`call-processor.ts`、`heritage-processor.ts` 分阶段消费。
- 当前 spec-wiki 还没有 `SuffixIndex`、全局 resolve cache、内置函数过滤和置信度评分，直接写 `edges` 只会制造大量半成品数据。
- 当前迭代的直接可交付目标是“symbol definitions + symbol search”；这只需要 `symbols` 和 `symbols_fts`。

**Alternatives considered**

- 方案 A：现在就持久化所有原始 import/call/heritage capture
  - 否决原因：会强行扩当前 SQLite schema，且这些 raw capture 在迭代 7 还没有消费方。
- 方案 B：现在就直接填 `edges`
  - 否决原因：会提前侵入迭代 8 的关系解析边界。

### 决策 3：先整套升级 tree-sitter 栈，再统一单后端解析

**Decision**

- 先把 `tree-sitter` 核心 crate 以及现有 JavaScript / TypeScript / Python grammar 升到同一版本族，再补齐 Go / Java / C / C++ / C# / Rust / PHP / Kotlin / Swift 的 grammar crate。
- Python 仍使用 `tree-sitter-python`，不在本轮切换到原生 Python AST。
- React 不新增专用 grammar，而是把 `.tsx/.jsx` 直接委托到底层 TypeScript / JavaScript parser；Vue/Svelte 的 script block 包装层不进入本轮最小边界，当前先保持 fail-soft，并在迭代 8 补齐。

**Rationale**

- 当前仓库已经在 scanner 的 JavaScript / Python / TypeScript 依赖提取中使用 tree-sitter，但新增语言的官方 grammar crate 已经普遍迁移到更新 API。继续固守 `0.20` 会迫使仓库内部长期并存两套绑定方式，比一次性升级核心栈的维护成本更高。
- `GitNexus` 的 `tree-sitter-queries.ts` 把 12 种语言 query 统一收口，并通过 `tree-sitter-queries.test.ts` 保证所有 supported languages 都有 query；这和本轮“先统一 query registry，再补语言覆盖”的推进顺序完全一致。
- 虽然 `CodeWiki` 的 `analyzers/python.py` 证明 Python AST 更精确，但当前设计文档和仓库现状都收敛到 tree-sitter 方案；本轮引入第二后端只会扩大测试面和维护成本。
- React 的 JSX/TSX 路径已经能由现有 grammar 直接覆盖；Vue/Svelte 仍需要额外的 script block 包装层，这部分超出本轮最小边界，先放到迭代 8 与符号关系解析一起推进，比仓促接入更稳。

**Alternatives considered**

- 方案 A：继续停在 `0.20`，只为新增语言单独找旧版 grammar
  - 否决原因：短期看改动较小，但会把“升级 parser 基础设施”的债务继续滚到后续迭代，也会让现有 JS/TS/Python 与新增语言在 API 和测试基线上脱节。
- 方案 B：Python 使用原生 AST，其他语言用 tree-sitter
  - 否决原因：实现更复杂，也不符合本轮“统一 parser registry”的目标。

### 决策 4：符号 ID 继续遵守当前仓库的稳定 ID 习惯，但以可重建 seed 为事实源

**Decision**

- 定义类符号的 seed 使用 `"{file_path}:{label}:{name}:{start_line}"`。
- 实际 `symbol_id` 继续通过当前仓库已有的 `stable_id("symbol", seed)` 生成，避免在 runtime 中混入第二套 ID 风格。
- `SymbolTable` 采用双索引：
  - `file_index: BTreeMap<String, BTreeMap<String, Vec<String>>>`
  - `global_index: BTreeMap<String, Vec<String>>`
- 同名符号使用 `Vec<symbol_id>` 保存，不沿用 `CodeWiki` 那种“名称只映射一个节点”的简化策略。

**Rationale**

- 当前 spec-wiki 的 source/page/module/relation 都使用稳定哈希 ID；symbol 层突然改成人类可读长 ID，会让 persistence 风格断裂。
- `GitNexus` 的 `symbol-table.ts` 证明双索引是必要的，但其 file-local `Map<name, nodeId>` 在同文件同名时是覆盖语义，配套单测也把这个行为固化了。spec-wiki 需要借鉴“双索引”，但不能照搬“同名只保留最后一个”的局限。

**Alternatives considered**

- 方案 A：直接使用 `file_path::name::label::start_line` 作为最终 ID
  - 否决原因：可读但和当前仓库其余 ID 风格不一致。
- 方案 B：全局名称只映射一个 symbol
  - 否决原因：无法处理重名导出、重载和多文件同名工具函数。

### 决策 5：`query` 显式返回 symbol 命中，而不是只把 symbol search 折叠回 source/page

**Decision**

- `QueryReport` 新增 `matched_symbols: Vec<QuerySymbolMatch>`。
- `QueryContextPack` 新增 `symbols` 视图，用于携带与页面或源码相关的关键符号摘要。
- `run_query()` 同时执行 `wiki_pages_fts` 与 `symbols_fts` 搜索：
  - 页面命中继续进入现有 `matched_pages` / `matches`
  - 符号命中进入 `matched_symbols`
  - 同时按 `file_path -> source -> page/module` 关系回填相关源码、页面和模块命中

**Rationale**

- 如果 symbol BM25 只回填成 source/page，用户无法知道究竟命中了哪个符号，这会把新能力降回旧能力的另一种实现细节。
- 这也是对 `.wiki/06-设计文档/00-总体设计.md` 中 `ContextPack.symbols` 和后续 TOON/query surface 的自然前置铺垫。
- `GitNexus` 的 `hybrid-search.ts` 最终结果里显式保留 `name`、`label`、`startLine`、`endLine` 等 symbol 元数据，而不是只返回 file；这说明符号检索结果在消费层应该是 first-class 数据，而不只是回填附件。
- Agent 集成 spec 只约束工具集合和 thin boundary，没有把 query JSON 字段锁死；新增字段属于可兼容扩展。

**Alternatives considered**

- 方案 A：只把 symbol 命中折叠为 `matched_sources`
  - 否决原因：新能力不可见，且后续迭代 8 还得再改一次 query contract。
- 方案 B：等迭代 8 再一次性引入 symbol query
  - 否决原因：`symbols_fts` 会继续空转，迭代 7 的 parser 收益无法直接验证。

### 决策 6：`update` 只对受影响文件增量重解析符号；解析失败时清空该文件旧符号而不是保留陈旧数据

**Decision**

- `init / rebuild`：对全部受支持源码执行 symbol parsing。
- `update`：只对新增、修改、删除的源码路径更新 symbol snapshot。
  - 新增/修改：重新解析后 replace 该文件的 symbol rows 和 `symbols_fts`
  - 删除：按 `file_path` 删除 symbol rows 和 `symbols_fts`
  - 解析失败：删除该文件旧 symbol rows，记录 warning，让 query 不再命中陈旧符号

**Rationale**

- 当前 `ChangeSet` 已经能把源码变化收敛到文件路径；definition-only 的 symbol snapshot 是天然文件级增量数据。
- 对 parse failure 保留旧 symbol rows 会把 query 引到已经不存在的定义上，风险比“暂时无符号”更大。

**Alternatives considered**

- 方案 A：任何源码变化都全量重解析所有符号
  - 否决原因：和现有 incremental runtime 的方向冲突，大仓库成本过高。
- 方案 B：解析失败时保留旧 symbol rows
  - 否决原因：会让 query 返回过期事实。

### 决策 7：解析按字节预算分批，并对单文件失败做 fail-soft 处理

**Decision**

- 新增 `CHUNK_BYTE_BUDGET = 20 * 1024 * 1024` 的解析批次预算。
- 待解析文件按仓库相对路径稳定排序，再按预算切批；批内逐文件解析，批结束后释放临时 AST/文本对象。
- 对 unsupported language、超大文件、tree-sitter parse error 或 query mismatch：
  - 只记录诊断
  - 当前文件返回空 symbol 结果
  - 整个 workflow 继续执行

**Rationale**

- 当前 scanner 和依赖提取经常直接 `fs::read_to_string()`；如果 symbol parsing 继续无节制读取，大仓库会把内存峰值抬得很高。
- `GitNexus` 的 `pipeline.ts` 已经把 `CHUNK_BYTE_BUDGET` 写成 20MB，并在 chunk 结束后清 AST cache；`parsing-processor.ts` 对 parser unavailable、超大文件、query error 和 parse error 也都是 warn + continue。这说明“按预算分批 + 单文件 fail-soft”不是抽象原则，而是被真实仓库验证过的实现策略。
- `deepwiki-open` 的文本/embedding pipeline 也从反面说明“把所有文件同时吃进内存”代价很高；GitNexus 风格的 byte budget 更适合 core pipeline。

**Alternatives considered**

- 方案 A：一次性把所有源码读入内存再解析
  - 否决原因：对 `storybook`、`dagger` 这类仓库风险过高。
- 方案 B：遇到任何 parser 错误就让 init / update 失败
  - 否决原因：用户仓库里的单文件语法错误不应阻断整个 Wiki runtime。

### 决策 8：symbol parsing 与 SQLite 写入通过统一存储接口收口，而不是让 workflow 直接拼 SQL

**Decision**

- 在 `storage/sqlite_store.rs` 中新增 symbol 相关接口，例如：
  - `replace_all_symbols(...)`
  - `replace_symbols_for_files(...)`
  - `remove_symbols_for_files(...)`
  - `search_symbols_fts(...)`
- workflow 只调用存储接口，不直接写 SQL。
- symbol rows 与 `symbols_fts` 的刷新放进同一事务。

**Rationale**

- 当前仓库的状态和页面缓存已经逐步往 SQLite 接口层集中；如果 symbol 写入从 workflow 直接拼 SQL，会再次把存储细节扩散回 orchestrator。
- 这也和 `deepwiki-rs` 的 context/cache 分层思路一致：workflow 编排，具体存取由专门层处理。

**Alternatives considered**

- 方案 A：直接在 `init.rs` / `update.rs` 里写 symbol SQL
  - 否决原因：后续 query、status、rebuild 都会复制同类逻辑。

## Risks / Trade-offs

- `[12 种语言一次接入会拉高 parser 维护成本]` → 统一 parser registry 和 `.scm` query 目录；项目集主覆盖当前 19 仓库使用到的语言，C/C++ 用 fixture 级验证补齐。
- `[tree-sitter query 对某些语言语法覆盖不完整]` → 先把 definition capture 做稳，raw capture 为迭代 8 预留；不要在本轮追求关系解析闭环。
- `[QueryReport 新增 matched_symbols 需要改测试和消费代码]` → 采用 additive 字段，现有 callers 可忽略；同步补 Rust 测试与 Agent 透传测试。
- `[解析失败删除旧 symbol rows 可能导致短期 query 结果变少]` → 这是有意选择的保守策略，优先保证“不返回陈旧事实”。
- `[symbol parsing 暂时不反哺 planner，用户可能感知不到页面内容立刻变深]` → 用 symbol search 和后续迭代 8 的 graph analysis 作为直接收益链路，本轮不夸大页面内容效果。

## Verification Feedback

- 19 个测试项目的完整 `init` 分析已经验证当前最小边界可行：`symbols` / `symbols_fts` 真写盘、`query` 返回 `matched_symbols`、页面拓扑保持稳定。项目集总计产出 `215` 个页面、`51,300` 个 symbols；除 `docker-mailserver` 这类以 shell/config 为主的仓库外，其余有 symbol 的项目都能用代表性符号词拿到精确命中。
- 项目集分析直接暴露了两个需要回修的真实问题，并已纳入实现：一是 `storybook` 级别的 monorepo 会超过原来测试脚本固定 `60s` 的 IPC 超时，因此验证层现在对 `init / update / rebuild` 使用更长默认超时；二是 `tree-sitter-kotlin-ng` 的真实 AST 字段名与旧 Kotlin query 假设不一致，导致 `leakcanary` 初次分析时近乎空跑，现已按 grammar 实际字段修正并补单测。
- 包装语言边界与设计收口保持一致：`React(.jsx/.tsx)` 继续委托到底层 JS/TS grammar，`Vue/Svelte` 仍然 fail-soft。`wot-starter` 的 symbol 结果主要来自 TS/JS/Python 辅助代码，这与“包装层延期到迭代 8”的设计一致，不视为本轮回归。
- 拆分后的生命周期脚本现已完成全量分阶段验证：`bootstrap` 为 `147/147`、`steady` 为 `249/249`、`mutation` 为 `228/228`、`rebuild` 为 `257/257`。其中 `steady / mutation / rebuild` 均验证了 symbol table 可读、symbol count 稳定，以及对可查询项目的 exact symbol hit 仍然存在；`docker-mailserver` 因无 symbols 走显式 skip，`aLocal/spec-wiki` 因 real-repo `.wiki` 不位于 `repoRoot` 走 query skip，但 workflow/state 仍保持 fresh。
- 生命周期验证还回修了两个实现细节：一是 mutation 阶段必须优先 touch `source_states` 中已跟踪的源码文件，才能稳定触发 `stale -> fresh` 与增量 symbol refresh；二是 Windows 下 `rebuild` 删除 runtime/SQLite 文件需要对 `os error 5/32` 做重试，否则会因短暂文件锁导致误报失败。

## Migration Plan

1. 先补 `tree-sitter` grammar 依赖和 `repo::symbols` 模块骨架，建立 parser registry、基础模型和 fixture。
2. 实现 definition capture、导出判断、双索引和解析批处理，先在内存中跑通 `parse_symbols`。
3. 在 SQLite 中补 symbol 写入/删除/查询接口，让 `init` 和 `rebuild` 写入真实 `symbols` / `symbols_fts`。
4. 扩展 `QueryReport` 和 `run_query()`，接入 symbol BM25 与结构化回填。
5. 把 `update` 接到按文件增量重解析路径，处理新增/修改/删除/解析失败四类情况。
6. 补测试、生命周期验证和测试项目集分析，最后视结果调整 query 排序或语言 query 细节。

回滚策略：

- 删除 `.wiki/.cache/wiki-cache.db` 后执行 `rebuild`，即可回到干净 runtime。
- 如果 symbol query 合同需要临时回退，可保留 page BM25 路径并关闭 `matched_symbols` 输出，但不回退 parser/SQLite schema。

## Open Questions

- `matched_symbols` 的排序是否需要与页面结果做统一 rank，还是先保持“页面/符号分开列表、各自排序”。当前倾向于先分列表，避免本轮提前进入全局排序策略设计。
- C/C++ 的真实项目集样本目前不在 `.wiki/06-设计文档/00-总体设计.md` 的 19 个仓库里，是否需要额外加入 1-2 个轻量 fixture 仓库做集成级验证。当前倾向于本轮先用 fixture 级验证，避免扩大项目集基线。
