## Context

当前仓库已经具备 symbol definitions 的基础设施，但还没有进入真正的“图事实层”：

- [`crates/wiki-core/src/repo/symbols/pipeline.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/symbols/pipeline.rs) 只把 definition capture 提升为 `SymbolNode`，`imports / calls / heritage` 没有被解析和持久化。
- [`crates/wiki-core/src/repo/symbols/models.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/symbols/models.rs) 里的 raw capture 模型只有文件路径、名称和少量文本，缺少 resolution 阶段真正需要的 owner / source symbol / line / import path 等元信息。
- [`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs) 已经建好了 `edges / communities / processes` 表，但当前 workflow 只写 `symbols` 与 `symbols_fts`，这些图表仍是空壳。
- [`crates/wiki-core/src/workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 和 [`crates/wiki-core/src/workflows/rebuild.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/rebuild.rs) 目前主链是 `scan -> parse_symbols -> module_tree -> context -> planner -> render`，缺少 resolution 与 graph analysis。
- [`crates/wiki-core/src/domain/change_set.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/change_set.rs) 的增量规划只围绕源码 fingerprint、模块树和页面计划，没有任何 graph-aware 反向依赖扩散。
- [`crates/wiki-core/src/repo/hierarchy.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs) 与 [`crates/wiki-core/src/generation/context.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 仍主要消费 `ScanReport.dependency_hints` 和 `ModuleTree.cross_module_edges`；这些关系来自文件级启发式，不是 symbol-level facts。
- [`crates/wiki-core/src/generation/planner.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/planner.rs) 的 workflow 页面只依赖 CI/CD、Makefile、Dockerfile 线索，而不是端到端执行流。
- [`crates/wiki-core/src/workflows/query.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/query.rs) 已经能做页面与 symbol 的 FTS/BM25，但它没有 graph query，也没有 relation/process 的 first-class 视图。

上游真实实现对这一步的拆层方式很一致：

- GitNexus 的 [`pipeline.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/pipeline.ts) 明确是 `structure -> parsing -> imports -> calls -> heritage -> communities -> processes`。其中 [`parsing-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/parsing-processor.ts) 只抽 definitions 与 raw capture，不在 parsing 阶段直接做关系闭环；[`import-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/import-processor.ts) 把 `SuffixIndex`、manifest config 和 resolve cache 单独收口；[`call-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts) 通过 `SymbolTable + ImportMap` 解析调用关系并过滤大量 built-ins；[`community-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/community-processor.ts) 与 [`process-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/process-processor.ts) 则建立在 CALLS / EXTENDS / IMPLEMENTS 图之上。
- CodeWiki 的 [`analysis_service.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analysis/analysis_service.py) 先让 `RepoAnalyzer` 找文件，再让 [`call_graph_analyzer.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analysis/call_graph_analyzer.py) 做统一关系解析；语言分析器如 [`analyzers/typescript.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/analyzers/typescript.py) 也是先 `_extract_all_entities()`，再 `_extract_all_relationships()`；[`topo_sort.py`](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/dependency_analyzer/topo_sort.py) 则把 Tarjan SCC 和 topo order 作为后续图分析能力，而不是混在 parser 里。
- deepwiki-rs 的 [`workflow.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/workflow.rs) 与 [`preprocess/mod.rs`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/mod.rs) 虽然更偏 LLM-heavy，但它们同样证明“先把事实层收敛成上下文，再交给后续阶段消费”才是可维护的工程组织。
- deepwiki-open 的 [`data_pipeline.py`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-open/api/data_pipeline.py) 与 [`websocket_wiki.py`](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-open/api/websocket_wiki.py) 主要是文本/RAG 消费层，这反过来说明这些能力不应混进当前 core 的事实层迭代。

这轮设计必须满足两个硬约束：

- 继续遵守当前仓库的主边界：Facts 在 core 内 deterministic 产生，Agents 只做薄接入；`.wiki/*.md`、`wiki.metadata.json`、`.wiki/.cache/` 三层职责不能混用。
- 参考 upstream 的真实源码拆层，但不能照搬 GitNexus 的 KuzuDB / worker-pool / Node 生态，也不能把 deepwiki-open 的消费层形态反灌进 core。

## Goals / Non-Goals

**Goals:**

- 在 `parse_symbols` 之后新增独立的 symbol resolution 与 graph analysis 层，形成 `scan -> parse_symbols -> resolve_symbol_graph -> analyze_symbol_graph -> module_tree -> context -> planner -> render` 的事实主链。
- 把 raw import/call/heritage capture 解析为稳定的 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges，并写入 SQLite 的 `edges` 表。
- 补齐 Vue / Svelte 包装语言的 script block 解析与行号映射，使这些文件不再长期停留在 fail-soft。
- 基于 symbol graph 生成 communities、processes 和 cycle/topology 派生结果，并供 module_tree/context/planner/query 消费。
- 升级 query，让现有 `term` 检索返回 BM25 结果的同时，自动回填 graph context、调用链和 process 命中，而不要求 Agents 先升级成更复杂的查询协议。
- 在 update 中保持 symbol/edge 的按文件增量刷新，同时确保 graph-derived 结果不会因为局部更新留下陈旧数据。
- 对 19 个测试项目集和 lifecycle 脚本补齐关系解析、graph analysis 和 graph query 的回归验证。

**Non-Goals:**

- 本迭代不引入向量检索、RRF、TOON、embedding pipeline 或 LLM explain layer；这些仍属于后续迭代。
- 本迭代不引入新的持久化后端，SQLite 仍是唯一事实存储。
- 本迭代不把 symbol graph 全量塞进 `WikiState` 或 `wiki.metadata.json`；页面/runtime 的主状态模型继续聚焦 page/module/source/section。
- 本迭代不重写现有 Agent JSON IPC 为多参数查询 DSL；query 入口继续维持薄协议。
- 本迭代不追求完全精确的全语言静态分析闭环，尤其是动态语言和框架魔法场景仍允许降级为低置信度或 unresolved。

## Decisions

### 决策 1：在 `repo` 层新增独立的 `symbol_graph` 子层，而不是继续膨胀 `repo::symbols` 或 workflow

**Decision**

- 在 `crates/wiki-core/src/repo/` 下新增与 `symbols/` 并列的 `symbol_graph/`（或等价命名）子层，至少包含：
  - `models.rs`：`ResolvedSymbolEdge`、`ImportRecord`、`CallRecord`、`HeritageRecord`、`CommunityNode`、`ProcessNode`、`CycleSummary`
  - `resolve/`：`imports.rs`、`calls.rs`、`heritage.rs`
  - `analyze/`：`communities.rs`、`processes.rs`、`cycles.rs`
  - `pipeline.rs`：把 `ParsedSymbolsSnapshot` 变成 `ResolvedGraphSnapshot`
- workflow 只负责编排这些阶段，不直接承载语言解析、CTE 细节或图算法。

**Rationale**

- 当前 `repo::symbols` 已经承担 parser registry、definition capture 和 `SymbolTable`；继续把 resolution、graph analysis、SQLite query helper 塞进去，会把“抽取事实”和“解释事实”混成一个模块。
- GitNexus 的真实源码就是把 parsing、imports、calls、heritage、communities、processes 拆成独立 processor；CodeWiki 也把 analyzer 与后续 resolve / topo_sort 分开。这不是抽象建议，而是经过实际工程验证的分层。

**Alternatives considered**

- 方案 A：把 resolution 和 graph analysis 继续放进 `repo::symbols`
  - 否决原因：职责太宽，后续任何 query/planner 改动都会重新侵入 parser 模块。
- 方案 B：在 workflow 中直接串若干 helper 函数
  - 否决原因：会把 orchestrator 重新变回“大型业务函数”，不利于增量测试和局部重用。

### 决策 2：扩展 raw capture 契约，但继续保持“先解析、后解析关系”的两阶段结构

**Decision**

- `ParsedFileSymbols` 继续作为 parser 阶段的输出，但 raw capture 模型升级为能被 resolution 真正消费：
  - `RawImportCapture`: `file_path`、`raw_path`、`line`、`language`、可选 `source_symbol_id`
  - `RawCallCapture`: `file_path`、`called_name`、`line`、`source_symbol_id`、可选 `receiver_text`
  - `RawHeritageCapture`: `file_path`、`owner_symbol_id`、`target_name`、`relation_kind`
- parser 阶段不直接填 `edges` 表；`edges` 只接受 resolve 后的稳定关系。
- Vue / Svelte 在 parser 阶段先抽出可解析的 script slice，再把解析结果映射回原文件路径和原始行号。

**Rationale**

- 当前 raw capture 只够“知道文件里出现过什么字符串”，不够支撑“谁调用了谁/谁继承了谁”的关系解析。
- GitNexus 的 [`parsing-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/parsing-processor.ts) 明确把 worker 输出定义为 raw imports/calls/heritage/routes，再由后续 processor 消费；CodeWiki 的 TS analyzer 也先保留 `all_entities` 再做 `_extract_all_relationships()`。
- Vue/Svelte 包装层如果继续留到 query 或 resolution 再处理，会让 line mapping 和 diagnostics 分散到多层；正确位置仍是 parser 输出边界。

**Alternatives considered**

- 方案 A：保持当前极简 raw capture，依赖 resolver 再次读取 AST
  - 否决原因：会把 update 成本翻倍，也会让 parser 与 resolver 的诊断口径不一致。
- 方案 B：parser 直接输出最终 edges
  - 否决原因：会让语言专用解析逻辑和全局索引/置信度策略耦合，失去分阶段优势。

### 决策 3：symbol/graph facts 继续留在 SQLite 专表，不扩进 `WikiState`

**Decision**

- `WikiState` 继续只承载页面、section、源码、模块、wiki-level relations、dirty/build state。
- `symbols`、`edges`、`communities`、`processes` 通过 `sqlite_store` 独立读写，不并入 `WikiState` 的序列化和恢复。
- 为 graph facts 新增专门接口，例如：
  - `replace_symbol_graph_for_files(...)`
  - `replace_graph_analysis(...)`
  - `load_symbol_edges_for_symbols(...)`
  - `load_processes_for_symbols(...)`
  - `trace_call_chain(...)`

**Rationale**

- 当前 `WikiState` 已经是 page/module/source runtime 的统一主模型，但 `status`、`sync`、`change_set` 并不需要加载全量 edges/communities/processes。若把几十万条 edges 一并装进 `WikiState`，只会拖慢所有 workflow。
- 当前实现已经把 `symbols` 独立于 `WikiState` 持久化，这一思路应继续延续到 graph 层。

**Alternatives considered**

- 方案 A：把 graph facts 纳入 `WikiState`
  - 否决原因：状态体积会急剧膨胀，且大部分 workflow 并不消费这些数据。
- 方案 B：完全绕过 SQLite，只在 query 时临时重算 graph
  - 否决原因：会把 init/update 的收益丢回 query 时延，也无法做生命周期验证和增量一致性检查。

### 决策 4：import resolution 采用“预构建上下文 + 语言专用 resolver”模式

**Decision**

- 在 init/rebuild/update 的单轮 workflow 中，先基于 `ScanReport.files` 与 manifest/配置构建 `ImportResolutionContext`：
  - `SuffixIndex`
  - 归一化文件路径列表
  - resolve cache
  - TS path alias、Go module path、PHP PSR-4、Swift target map、Rust crate roots 等语言配置
- import resolution 输出的是 symbol-level `IMPORTS` edges；当 import 只能解析到文件但不能唯一定位具体 symbol 时，先落到“文件拥有者 symbol”的保守规则，并在 `reason` 中标记。

**Rationale**

- GitNexus 的 import-processor 并不是边遍历边临时猜路径，而是显式先构建 `ImportResolutionContext`，再复用 `SuffixIndex` 和 resolve cache 处理整轮解析。
- 当前 spec-wiki 的 scanner 已经能识别 manifest、workspace roots 和语言标签，这些都是构建 import context 的现成输入。

**Alternatives considered**

- 方案 A：每个 import capture 现场扫描整个文件列表
  - 否决原因：大仓库性能不可接受，也无法与 update 的局部刷新复用。
- 方案 B：只解析文件级 import，不尝试落到 symbol
  - 否决原因：会让 CALLS / EXTENDS 的后续推理缺少必要的 file-to-symbol bridge。

### 决策 5：call resolution 采用“词法优先 + 置信度分层”的策略，保留 unresolved 而不是强行命中

**Decision**

- `resolve_calls` 使用三层策略：
  - `same-file`: 当前文件内存在同名定义时优先命中
  - `import-resolved`: 当前调用名在已导入文件的 symbol 集中命中
  - `fuzzy-global`: 全局名称索引的保守兜底
- 维护语言级 built-in/noise 名单，默认过滤标准库、常见集合方法、日志函数、框架 hook 和无语义工具函数。
- 当同名候选过多或 receiver 语义不明确时，允许保留 unresolved，不为 `edges` 表强行制造低质量 CALLS 记录。

**Rationale**

- GitNexus 的 [`call-processor.ts`](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/call-processor.ts) 已经证明：真正可用的调用图不在于“尽量多连边”，而在于“可解释的置信度 + 足够 agressive 的噪声过滤”。
- 当前 spec-wiki 的 `SymbolTable` 是 `file_index + global_index` 双索引，天然适合做 same-file 与 fuzzy-global 的分层查找。
- 从语言语义看，同文件局部定义通常比模糊全局候选更可信；这与 GitNexus 实际代码的 lookup 顺序也一致。

**Alternatives considered**

- 方案 A：任何命中的名称都写 CALLS 边
  - 否决原因：在 JS/TS/Python 中会迅速把图噪声拉满，后续 process/community 都会失真。
- 方案 B：没有 import 证据就完全不落边
  - 否决原因：会丢失 Rust/Go/Java 中大量同文件和显式局部调用关系。

### 决策 6：graph-derived 结果采用“edges 增量、analysis 全量”策略

**Decision**

- `symbols` 与 `edges` 在 update 中按受影响文件增量刷新。
- 受影响文件集合不只包括变更源码本身，还包括“上一轮或当前轮通过 import 边直接依赖这些文件”的一跳反向依赖文件。
- `communities`、`processes` 与 cycle/topology 结果在任何 edge 变化后全量重算，然后整体替换。

**Rationale**

- per-file 的 symbol/edge refresh 与当前 `ChangeSet` 模型天然兼容；而 communities/processes 是全局导出视图，强行做局部 patch 复杂度很高、收益有限。
- GitNexus 也是在完整关系图上统一跑 community/process 检测，而不是边解析边局部更新这些结果。
- 当前 `change_set.rs` 已经有稳定的 dirty source 路径集合，我们只需要再补一层 graph-aware fan-out，而不必完全重写增量内核。

**Alternatives considered**

- 方案 A：任意源码变化都全量重算 symbols + edges + analysis
  - 否决原因：违背本仓库已有 incremental runtime 方向。
- 方案 B：对 communities/processes 做局部修补
  - 否决原因：实现复杂、结果不稳定，第一版很难验证正确性。

### 决策 7：module tree 和 planner 只消费 graph summary，不直接消费全量 symbol graph

**Decision**

- `build_module_tree` 的根路径发现、模块提升/抑制和 steering 边界仍由 `ScanReport` 主导。
- 新增 `GraphSummary` 作为 hierarchy/context/planner 的输入，只提供聚合后的高层信号：
  - `module_dependency_hints`
  - `module_call_hotspots`
  - `communities_by_module`
  - `detected_processes`
  - `cycle_warnings`
- workflow 页面从“仅凭 CI/CD 线索生成”升级为“CI/CD 线索或 detected processes 二者任一成立就生成”，且优先展示 process traces。

**Rationale**

- 当前 `build_module_tree(report)` 的强项是稳定的目录/manifest 结构理解，不应因为引入 symbol graph 就把模块边界完全交给调用图驱动。
- GitNexus 的 structure 阶段也是先于 parsing；说明 structure 和 graph 应该是“前后相邻、互相增强”，而不是互相取代。
- 当前 `generation/context.rs` / `planner.rs` 只消费模块级摘要，向它们直接暴露 symbol graph 只会扩大耦合面。

**Alternatives considered**

- 方案 A：让 module tree 直接基于 communities 重建
  - 否决原因：页面拓扑稳定性会变差，且不符合当前 root_path 锚定设计。
- 方案 B：完全不让 planner/context 消费 graph 信号
  - 否决原因：会使迭代 8 的图事实难以反馈到用户可见产物。

### 决策 8：query 入口继续保持 `term` 语义，graph context 由 top symbol hits 自动展开

**Decision**

- `transport/dto.rs` 保持 `action / repoRoot / term` 的薄协议，不新增 query DSL。
- `run_query()` 仍以 `term` 为入口，但在 high-confidence symbol 命中后自动追加：
  - 直接相关 `IMPORTS / CALLS / EXTENDS / IMPLEMENTS` edges
  - 上下游 1-3 跳调用链摘要
  - 命中 symbol 所在的 processes / communities
- 返回结构新增 symbol-graph 视图，例如 `matched_symbol_edges`、`matched_processes`、`matched_communities`，并在 provenance 中区分 `bm25`、`graph-cte`、`process-trace`。

**Rationale**

- 当前 Agent IPC 真实代码只接受 `term`；如果在 iter 8 就改协议，变更会扩散到 transport 与 Agent specs，超出本轮必要范围。
- GitNexus 的 hybrid-search 也是以 query string 为入口，再在结果层附带 symbol metadata；对 spec-wiki 来说，graph context 自动扩展是最稳的中间形态。

**Alternatives considered**

- 方案 A：新增 `queryMode` / `symbolId` / `depth` 等协议字段
  - 否决原因：会提早侵入 Agent 接口层，不符合当前“Agents 薄接入”边界。
- 方案 B：query 仍只返回页面/源码，不返回 graph 视图
  - 否决原因：用户无法直接感知 iter 8 的主要收益。

### 决策 9：社区检测与流程检测通过 trait 抽象收口，并允许 deterministic fallback

**Decision**

- 定义 `CommunityDetector` 与 `ProcessDetector` 抽象，默认实现落在 core 内。
- 对 communities：
  - 首选基于 CALLS / EXTENDS / IMPLEMENTS 的加权聚类
  - 大图时过滤低置信度边与度为 1 的节点
  - 当目标算法不可用或结果超时，回退到 deterministic 的连通分量/标签传播近似
- 对 processes：
  - 采用入口点评分 + BFS forward trace + subset/endpoint dedupe
  - 只使用中高置信度 CALLS 边
- 对 cycles：
  - 使用 Tarjan SCC
  - 断边与 topo order 主要作为 planner/query 的派生辅助，不新增独立持久化表

**Rationale**

- GitNexus 实际使用 graphology vendored Leiden；但当前仓库边界是 Rust + SQLite，直接移植其依赖栈并不现实。
- 真正需要继承的是“把社区/流程视为 graph-derived views”这一设计，而不是绑定某个 JS 生态实现。
- CodeWiki 的 `topo_sort.py` 已经证明 Tarjan SCC 与 cycle breaking 非常适合作为这类图派生阶段的固定能力。

**Alternatives considered**

- 方案 A：现在就硬绑定一个特定的 Rust Leiden 实现
  - 否决原因：依赖成熟度和跨平台可用性不确定，风险过高。
- 方案 B：完全不做 fallback，算法不可用就整轮失败
  - 否决原因：不符合当前 workflow 一贯的 fail-soft 原则。

## Risks / Trade-offs

- `[raw capture 模型扩张会增加 parser 复杂度]` → 只补 resolution 必需字段，不在 iter 8 提前引入 full AST serialization 或 source snippet cache。
- `[一跳反向依赖扩散仍可能漏掉动态调用影响]` → update 先保证 import-based dependents 与直接 symbol owners 一致；遇到结构性变化继续允许触发 replan/full graph analysis。
- `[communities/processes 全量重算会拉高 update 成本]` → 保持 symbols/edges 增量，graph-derived 结果单独全量；并通过大图过滤、trace 深度/分支上限控制成本。
- `[Vue/Svelte script mapping 可能带来行号偏移错误]` → 统一通过 `VirtualSourceSlice` 记录 `line_offset`，并对 fixture/SFC 样例补回归测试。
- `[query 返回面继续膨胀]` → 保持 additive 字段，不改现有 page/module/source/symbol 字段语义；大型 graph context 只返回摘要而不是全量边。
- `[community 算法与 GitNexus 不完全等价]` → 通过 detector trait 隔离实现，并把规范约束放在“聚类结果可消费且稳定”而不是锁死具体第三方库。

## Migration Plan

1. 扩展 `ParsedFileSymbols` raw capture 契约，并补齐 Vue/Svelte script wrapper。
2. 新增 `symbol_graph` 子层，实现 import/call/heritage resolution 与 SQLite edge 写盘接口。
3. 在 `init / rebuild / update` 接入 `resolve_symbol_graph`，并让 update 先支持 dirty file + reverse-import fan-out。
4. 实现 community/process/cycle 分析器，接到 `edges` 写盘之后、`module_tree` 之前。
5. 扩展 hierarchy/context/planner，消费 `GraphSummary` 强化模块关系和 workflow 页面。
6. 扩展 query 的 graph 读取与 CTE/trace merge。
7. 补齐 Rust tests、lifecycle scripts、19 项目集分析和 `test-project-analysis.md`。

回滚策略：

- 若 graph analysis 某一步不稳定，可保留 `symbols + edges` 路径并临时关闭 communities/processes 输出，不需要回滚 parser 与 resolution。
- 若 `edges` 数据需要清空，可删除 `.wiki/.cache/wiki-cache.db` 后执行 `rebuild` 恢复。

## Open Questions

- community 检测的默认实现是否直接采用第三方 Leiden crate，还是先以 deterministic fallback 作为主实现、把 Leiden 留成可插拔后端。当前倾向后者。
- graph query 结果是否需要单独暴露 `matched_cycles` 视图。当前倾向不暴露，先把 cycle 结果作为 planner/query 的内部辅助。
- workflow 页面是否要从单页扩展到“每个 detected process 一页”。当前倾向 iter 8 先保留单个 workflow 聚合页，避免破坏 page topology 稳定性。
