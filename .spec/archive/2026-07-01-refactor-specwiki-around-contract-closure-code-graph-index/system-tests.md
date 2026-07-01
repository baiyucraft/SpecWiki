# refactor-specwiki-around-contract-closure-code-graph-index 系统测试用例

## 用例总览

本文件覆盖 code graph/index substrate 合同闭合的系统级验收：schema/read API、raw captures 持久化、unresolved refs、`SymbolNode` 稳定身份合同、`.spec` 全落点隔离、graph readiness 分层、runtime workflow 通过 `wiki-index` traits/DTO 消费 graph、query route 映射，以及 phase diagnostics 可追踪性。

这些用例只定义验收结果和验证方式，不写实现步骤。后续实现采用 TDD，单元测试蓝图见 `unit-tests.md`，执行拆分见 `tasks.md`。

## 系统测试用例

### ST-001 Raw captures 可持久化、读取和诊断

- 关联成功标准: raw imports、raw calls、raw heritage 有稳定持久化与读取验收，不再只存在于 `ParsedFileSymbols` 内存结构；phase diagnostics 能追踪到 raw capture 或 resolver。
- 覆盖设计点: Raw capture envelope、`raw_imports / raw_calls / raw_heritage` 表、`unresolved_refs` 表、`IndexQueryStore` raw/unresolved 读取能力、resolve phase 消费 raw captures。
- 前置条件: 临时仓库包含 TypeScript import、函数调用和 class heritage 源码；执行 `run_init` 或等价 graph snapshot 写入路径。
- 操作 / 触发: 读取 SQLite graph facts 和 `wiki-index` query/store DTO，检查 raw import/call/heritage capture。
- 期望结果: 三类 raw capture 均带 `capture_id / file_id / language / range / parser_id / parser_version / diagnostics` 等基础字段，并可按文件或 capture id 读取；resolver 产生的 unresolved ref 能按 `capture_id / file_id / resolver_phase / range / reason / diagnostics` 读回。
- 验证方式: `cargo test -p wiki-runtime raw_graph_captures_roundtrip --test sqlite_storage` 与 `cargo test -p wiki-index raw_capture_dtos_preserve_source_identity`。

### ST-002 SymbolNode 稳定身份合同可存储和查询

- 关联成功标准: `SymbolNode` 具备稳定身份合同，至少覆盖 `symbol_id / file_id / language / symbol_kind / name`，并能表达 qualified name、signature、docstring、visibility、owner symbol、range 和 provenance；同一源码 snapshot 内 symbol identity 必须可重复。
- 覆盖设计点: `SourceRange`、`SymbolProvenance`、扩展 `SymbolNode`、SQLite `symbols` columns、FTS search text、query source refs。
- 前置条件: 临时仓库包含导出函数、类、方法和嵌套 owner 关系；同一源码 snapshot 重建两次。
- 操作 / 触发: 执行 init/rebuild 后读取 symbols、FTS 命中和 query result source refs。
- 期望结果: symbol 身份字段和扩展字段完整 roundtrip；同一源码 snapshot 重建两次 symbol id 稳定；scoped update 后受影响文件旧 symbol、edge、raw capture 和 unresolved ref 不残留；query 命中携带 range/provenance/source refs。
- 验证方式: `cargo test -p wiki-index symbol_node_contract_preserves_identity_range_and_provenance`、`cargo test -p wiki-runtime extended_symbols_roundtrip_and_query_refs`。

### ST-003 `.spec` 不进入 code graph facts

- 关联成功标准: `.spec` 不进入 code graph `files / symbols / edges / raw_* / files_fts / symbols_fts`，相关 scanner、hierarchy、runtime context 和 query tests 能验证这一点。
- 覆盖设计点: scanner guard、persistence guard、query guard、FTS guard、runtime context 过滤。
- 前置条件: 临时仓库同时包含 `src/app.ts` 和 `.spec/changes/demo/tasks.md`，其中 `.spec` 文件故意包含可被 parser 或 FTS 命中的源码样式文本。
- 操作 / 触发: 执行 init/query/rebuild，并直接检查 graph 表、FTS 和 runtime context 输出。
- 期望结果: `.spec` 路径不会出现在 scanner/hierarchy source 输入、runtime context、files/folders/symbols/edges/raw/unresolved/FTS/query index route；业务源码仍正常入库。
- 验证方式: `cargo test -p wiki-runtime scanner_noise_filter`、`cargo test -p wiki-runtime hierarchy_noise_filter`、新增 `.spec` graph persistence/query guard 测试。

### ST-004 Graph readiness 能区分 missing/stale/blocked/rebuilding/ready

- 关联成功标准: Graph readiness 能区分 `missing / stale / blocked / rebuilding / ready` 的最小判定来源：cache 缺失、关键表缺失、migration 不一致、source snapshot 不一致、重建中和可查询状态不得混淆。
- 覆盖设计点: `GraphReadiness`、`graph_snapshots`、`graph_phase_runs`、schema/migration 检查、`RuntimeReadiness.index` 投影。
- 前置条件: 分别构造无 DB、缺关键表、schema 版本不一致、源码 fingerprint 变化、phase rebuilding 和完整 snapshot 六类 fixture。
- 操作 / 触发: 调用 readiness evaluator、`run_status` 和 `run_query`。
- 期望结果: evaluator 返回正确 graph status、reason、required_tables、diagnostics；DB/cache 缺失为 `missing`，关键表缺失为 `blocked`，schema/migration mismatch 为 `blocked`，source snapshot mismatch 为 `stale`，phase rebuilding 为 `rebuilding`，blocking diagnostic 为 `blocked`，完整 snapshot 为 `ready`；runtime readiness index 映射一致；非 ready 时 query 不返回任何 index route。
- 验证方式: `cargo test -p wiki-runtime graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready`。

### ST-005 Core workflow 只通过 `wiki-index` traits/DTO 消费 graph

- 关联成功标准: init、update、rebuild、query 的 core runtime/index 读取路径通过 `wiki-index` traits/DTO 消费 graph，不依赖 CLI 产品面或 markdown/page fallback 来伪造 graph 命中。
- 覆盖设计点: `IndexSnapshotStore`、`IndexQueryStore`、SQLite adapter 私有 schema、runtime workflow orchestration、facts-only query。
- 前置条件: 临时仓库包含可形成 symbol 和 call edge 的源码；`.wiki` markdown 页面不包含查询词。
- 操作 / 触发: 执行 init、update、rebuild、query；在 query 前清理 page fallback 可能命中的 markdown 文本。
- 期望结果: index route 命中来自 graph facts；workflow 不直接暴露 SQLite 私表语义给 runtime 以外层；缺 graph facts 时不会由 page fallback 伪装成 index 命中。
- 验证方式: `cargo test -p wiki-runtime query_returns_graph_context_for_symbol_hits`、`cargo test -p wiki-index query_contract`、新增 workflow trait/DTO boundary 测试。

### ST-006 Query route 稳定映射 index_symbol_hit/index_path_hit/index_graph_hit

- 关联成功标准: index query adapter 能稳定产出或映射 `index_symbol_hit`、`index_path_hit` 和 `index_graph_hit`；index 不 ready 时不得返回 graph 伪命中。
- 覆盖设计点: `wiki-index::query` facts-only DTO、`wiki-runtime::workflows::query` projection、`QueryResultDto` route tags、source refs。
- 前置条件: 临时仓库包含符号、源码路径、call edge/process/community graph facts；另有 index missing fixture。
- 操作 / 触发: 查询 symbol 名、路径片段和 graph 相关 term；再删除 `.wiki/.cache` 或标记 rebuilding 后重复查询。
- 期望结果: ready 时分别生成 `IndexSymbolHit`、`IndexPathHit`、`IndexGraphHit` route group/result；source refs、confidence、recommended action 合理；不新增 route tag；markdown/page fallback 不会伪装成 index 命中；missing/stale/blocked/rebuilding 时 route groups/results 不包含 index route。
- 验证方式: `cargo test -p wiki-runtime query_fusion_outputs_route_groups_and_results`、`cargo test -p wiki-runtime query_does_not_emit_index_routes_when_index_is_not_ready`、新增 path hit 映射测试。

### ST-007 Phase diagnostics 可追踪到 file、parser、capture 或 resolver

- 关联成功标准: phase diagnostics 能追踪到 file、parser、raw capture 或 resolver；parse / resolve fail-soft 不得静默吞掉问题。
- 覆盖设计点: `GraphPhaseStatus`、`graph_phase_runs`、`UnresolvedRef`、parse/resolve diagnostics、fail-soft phase boundary。
- 前置条件: 临时仓库包含一个可解析文件、一个 parser fail-soft 文件、一个无法解析目标的 import/call/heritage。
- 操作 / 触发: 执行 init/rebuild 后读取 graph phase runs、unresolved refs 和 runtime status/query diagnostics。
- 期望结果: parse failure 记录到 file/parser；unresolved import/call/heritage 记录到 capture/resolver；workflow 继续完成可用部分；diagnostic 不被静默丢弃。
- 验证方式: `cargo test -p wiki-runtime graph_phase_diagnostics_trace_file_parser_capture_resolver`。

### ST-008 Schema/read API 闭合 graph source authority

- 关联成功标准: 后续测试计划至少覆盖 schema/read API、raw persistence、`.spec` exclusion、readiness degrade、query route mapping 和 core workflow 接入。
- 覆盖设计点: `graph_snapshots / files / folders / symbols / edges / raw_* / unresolved_refs / graph_phase_runs / files_fts / symbols_fts`，`files/folders` 作为 source authority，`scan_cache` 不再作为正式 source record 的唯一来源。
- 前置条件: 临时仓库包含多目录源码、嵌套文件夹、symbol、edge、raw capture、unresolved ref 和 phase diagnostics。
- 操作 / 触发: 执行 graph snapshot 写入后，通过 `wiki-index` traits/DTO 和 SQLite adapter read API 读取 source、folder、symbol、raw、unresolved、phase、FTS 和 current snapshot。
- 期望结果: 所有目标表族都有写后读验收；`list_sources/list_files/search_files` 等读取来源来自 graph files/folders 表；删除或清空 `scan_cache` 后正式 graph read API 仍能返回 source authority；current snapshot 指向完整写入的 graph facts。
- 验证方式: `cargo test -p wiki-runtime graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts --test sqlite_storage`。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| raw imports、raw calls、raw heritage 有稳定持久化与读取验收 | ST-001, ST-008 | Rust 单元/集成测试检查 DTO、schema/read API 与 SQLite roundtrip |
| `SymbolNode` 具备稳定身份合同和可重复 identity | ST-002 | Rust 单元/集成测试检查模型、落库、重建稳定性、scoped update 清理和 query refs |
| `.spec` 不进入 code graph facts 与 FTS | ST-003 | scanner/hierarchy/runtime/query guard 测试 |
| Graph readiness 区分 missing/stale/blocked/rebuilding/ready | ST-004 | readiness 状态矩阵、diagnostic 和 runtime query/status 测试 |
| init/update/rebuild/query 通过 `wiki-index` traits/DTO 消费 graph | ST-005 | workflow/query 合同测试与 fallback 反向测试 |
| query adapter 稳定映射 index route，not ready 不返回伪命中 | ST-006 | query route group/result 测试 |
| phase diagnostics 追踪 file/parser/capture/resolver | ST-007 | phase diagnostics 与 unresolved refs 测试 |
| 后续测试计划覆盖 schema/read API、raw persistence、`.spec` exclusion、readiness degrade、query route mapping 和 core workflow 接入 | ST-001, ST-003, ST-004, ST-005, ST-006, ST-007, ST-008 | 本文件 + `unit-tests.md` + `tasks.md` 互证 |

## 边界与异常

- `.spec` 文件即使包含可解析源码片段，也必须被 scanner、persistence 和 query 三层排除。
- 缺 DB、缺表、schema mismatch、snapshot stale 和 rebuilding 不能被统一压成空命中。
- raw capture 和 unresolved ref 不能只写入 diagnostics 文本或 JSON blob 后失去结构化读取能力。
- graph schema 扩展不能绕开 `wiki-index` traits/DTO；runtime 私有表不是跨层公共接口。
- process/community 只验收 graph phase、storage 和 query consumption 边界，不验收算法质量。
- 第一阶段不验收完整 type/scope/MRO/callback resolver，不验收 semantic/vector search。

## 验证数据与环境

- Rust 本地测试环境，使用 `cargo test`。
- 临时仓库 fixture 由测试创建，包含 TypeScript/Markdown/.spec 混合文件。
- SQLite DB 位于测试 fixture 的 `.wiki/.cache/wiki-cache.db`。
- 不需要网络、外部数据库、浏览器或 LLM provider。

## 未覆盖项

- 完整 compiler-grade type/scope resolution、receiver-bound call resolution、MRO、callback synthesis：本 change 非目标，后续能力 child 再定义。
- semantic/vector ranking、context pack 产品化输出：本 change 只保留 facts-only query 和最小 graph DTO 边界。
- governance artifact reference index：由后续 governance isolation child 覆盖。

## 参考资料

- `proposal.md`
- `design.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/specwiki-code-graph-index-design.md`
