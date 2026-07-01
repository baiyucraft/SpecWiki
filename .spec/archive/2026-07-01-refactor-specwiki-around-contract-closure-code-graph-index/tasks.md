---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-code-graph-index 任务计划

## 任务总览

任务按 design 中的可验收能力块拆分：先闭合 `wiki-index` graph DTO/store/query 合同，再落 SQLite snapshot 和 `.spec` persistence guard，然后接入 workflow/readiness，最后完成 query route projection 与系统级验证。实现阶段采用 TDD，每个 Red/Green/Refactor 小任务引用 `unit-tests.md` 中的 `UT-*`。

Contract-first gate：task 1 必须先完成 DTO/trait/compile tests，再进入 SQLite 实现；任何 runtime 私表都不能反向成为公共 graph 合同。

## 实现模式

tdd

先写失败单元测试并确认失败，再写最小实现，通过后重构。`implementation-ready` 已根据用户确认切换为 `true`，本 change 可以进入实现。

## 1. 闭合 wiki-index graph DTO 与 store 合同

- [x] 1.1 Red: UT-001 编写 `SymbolNode / SourceRange / SymbolProvenance` 合同失败单元测试，并确认失败原因是 DTO 字段缺失
- [x] 1.2 Green: UT-001 在 `wiki-index` 中补齐扩展 `SymbolNode`、range、provenance 与稳定 identity 所需最小实现
- [x] 1.3 Refactor: UT-001 在测试保持通过的前提下清理 `label` 到 `symbol_kind` 的过渡命名和 search text
- [x] 1.4 Red: UT-002 编写 raw capture 与 unresolved ref DTO 失败单元测试，并确认失败原因是结构化 capture/ref 合同缺失
- [x] 1.5 Green: UT-002 在 `wiki-index` 中补齐 `RawCaptureBase`、三类 raw capture、`UnresolvedRef`、phase/ref 枚举或等价类型
- [x] 1.6 Refactor: UT-002 在测试保持通过的前提下整理 diagnostics 类型复用，避免 runtime 私表语义泄漏进 DTO

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：`cargo test -p wiki-index symbol_node_contract_preserves_identity_range_and_provenance raw_capture_dtos_preserve_source_identity`、必要时 `cargo fmt --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. 实现 SQLite graph snapshot、raw persistence 与 `.spec` guard

- [x] 2.1 Red: UT-003 与 UT-009 编写 graph snapshot/schema read API 失败测试，覆盖 files/folders/symbols/raw/unresolved/edges/FTS/phase/current pointer 和 source authority
- [x] 2.2 Green: UT-003 与 UT-009 在 `wiki-runtime` SQLite adapter 中新增或调整 graph tables、write/read API 和事务写入路径
- [x] 2.3 Refactor: UT-003 与 UT-009 在测试保持通过的前提下整理 `replace_symbol_graph` 到 snapshot 写入边界，确保正式 source read API 不依赖 `scan_cache`
- [x] 2.4 Red: UT-004 编写 `.spec` persistence guard 失败测试，确认 `.spec` 可绕过 scanner 时仍会污染 graph 表或 FTS
- [x] 2.5 Green: UT-004 在 scanner/hierarchy 输入、SQLite adapter 写入前、runtime context 和 query 读取前加入 `.spec` path guard，覆盖 raw/unresolved/FTS
- [x] 2.6 Refactor: UT-004 在测试保持通过的前提下统一 repo-root-relative path 规范化和 Windows 分隔符处理
- [x] 2.7 Red: UT-010 编写 scoped update 清理旧 graph facts 失败测试，确认旧 symbol/edge/raw/unresolved/FTS 会残留
- [x] 2.8 Green: UT-010 实现 scoped graph snapshot refresh 对受影响文件的 symbols/raw/unresolved/edges/FTS 一致替换
- [x] 2.9 Refactor: UT-010 在测试保持通过的前提下整理 scoped update 事务和 snapshot pointer 切换

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：`cargo test -p wiki-runtime graph_snapshot_roundtrip_persists_raw_unresolved_phase_and_fts_atomically graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts graph_snapshot_rejects_spec_paths_before_tables_and_fts scoped_graph_snapshot_refresh_replaces_symbols_raw_unresolved_edges_and_fts --test sqlite_storage`、必要时 `cargo fmt --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. 接入 workflow phase diagnostics 与 GraphReadiness

- [x] 3.1 Red: UT-005 编写 readiness evaluator 失败测试，覆盖 missing/stale/blocked/rebuilding/ready
- [x] 3.2 Green: UT-005 实现 `GraphReadiness` DTO、evaluator 和 `RuntimeReadiness.index` 投影
- [x] 3.3 Refactor: UT-005 在测试保持通过的前提下删除或替换旧 bool readiness 主路径
- [x] 3.4 Red: UT-008 编写 init/update/rebuild phase diagnostics 失败测试，确认 phase runs/unresolved refs 当前缺失或不可追踪
- [x] 3.5 Green: UT-008 将 init/update/rebuild 接入 graph snapshot 写入、phase runs、fail-soft diagnostics 和 unresolved refs
- [x] 3.6 Refactor: UT-008 在测试保持通过的前提下整理 pipeline phase 边界，保持 parse/resolve fail-soft 行为

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：`cargo test -p wiki-runtime graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready workflows_write_graph_phase_diagnostics_and_unresolved_refs`、必要时 `cargo fmt --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. 完成 facts-only query adapter 与 runtime route projection

- [x] 4.1 Red: UT-006 编写 `wiki-index::query` 增强 symbol/path/graph 命中失败测试，确认 range/provenance/path/graph 字段缺失
- [x] 4.2 Green: UT-006 扩展 `IndexQueryStore` 和 query DTO，使 symbol/path/graph hits 通过 facts-only trait 返回
- [x] 4.3 Refactor: UT-006 在测试保持通过的前提下清理 MemoryStore fixture 和 query mapping，确保不读取 runtime/page fallback
- [x] 4.4 Red: UT-007 编写 runtime query 非 ready 状态抑制 index routes 与 page fallback 不伪装 index 的失败测试
- [x] 4.5 Green: UT-007 将 runtime query projection 绑定 GraphReadiness，只在 ready 时生成 `index_symbol_hit / index_path_hit / index_graph_hit`
- [x] 4.6 Refactor: UT-007 在测试保持通过的前提下清理 `QueryResultDto` source refs/confidence/recommended action 映射，不新增 route tag

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：`cargo test -p wiki-index index_query_returns_enriched_symbol_path_and_graph_hits`、`cargo test -p wiki-runtime query_suppresses_index_routes_for_all_non_ready_graph_states`、必要时 `cargo fmt --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. 系统验收、回归清理与文档沉淀

- [x] 5.1 Red: 按 ST-001 至 ST-008 补齐缺口测试列表，确认尚未覆盖的系统验收路径全部有失败或待实现证据
- [x] 5.2 Green: 跑通 raw persistence、SymbolNode contract、`.spec` exclusion、readiness degrade、query route mapping、workflow diagnostics 的局部和集成验证
- [x] 5.3 Refactor: 在测试保持通过的前提下删除不需要的旧 fallback/兼容主路径，整理注释和错误信息
- [x] 5.4 更新 `.wiki` 或 `.docs` 中与 code graph/index 合同相关的稳定设计说明；只沉淀已实现且稳定的事实
- [x] 5.5 运行最终验证命令并整理 `test-report.md` 输入：`unispec validate refactor-specwiki-around-contract-closure-code-graph-index`、`cargo test -p wiki-index`、`cargo test -p wiki-runtime`

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：`unispec validate refactor-specwiki-around-contract-closure-code-graph-index`、`cargo test -p wiki-index`、`cargo test -p wiki-runtime`、必要时 `cargo fmt --check`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 Raw captures 可持久化、读取和诊断 | 1. 闭合 wiki-index graph DTO 与 store 合同；2. 实现 SQLite graph snapshot | 1.4 / 1.5 / 1.6 / 2.1 / 2.2 / 2.3 / UT-002 / UT-003 / UT-009 |
| ST-002 SymbolNode 稳定身份合同可存储和查询 | 1. 闭合 wiki-index graph DTO 与 store 合同；2. 实现 SQLite graph snapshot；4. query projection | 1.1 / 1.2 / 1.3 / 2.1 / 2.2 / 2.7 / 2.8 / 4.1 / 4.2 / UT-001 / UT-003 / UT-006 / UT-010 |
| ST-003 `.spec` 不进入 code graph facts | 2. 实现 SQLite graph snapshot、raw persistence 与 `.spec` guard；5. 系统验收 | 2.4 / 2.5 / 2.6 / 5.1 / 5.2 / UT-004 |
| ST-004 Graph readiness 能区分五类状态 | 3. 接入 workflow phase diagnostics 与 GraphReadiness；4. query projection | 3.1 / 3.2 / 3.3 / 4.4 / 4.5 / UT-005 / UT-007 |
| ST-005 Core workflow 只通过 `wiki-index` traits/DTO 消费 graph | 3. workflow/readiness；4. facts-only query adapter；5. 系统验收 | 3.4 / 3.5 / 3.6 / 4.1 / 4.2 / 4.3 / 5.2 / UT-006 / UT-008 |
| ST-006 Query route 稳定映射三类 index route | 4. facts-only query adapter 与 runtime route projection | 4.1 / 4.2 / 4.3 / 4.4 / 4.5 / 4.6 / UT-006 / UT-007 |
| ST-007 Phase diagnostics 可追踪 | 1. DTO 合同；2. SQLite snapshot；3. workflow diagnostics | 1.4 / 1.5 / 2.1 / 2.2 / 3.4 / 3.5 / 3.6 / UT-002 / UT-003 / UT-008 |
| ST-008 Schema/read API 闭合 graph source authority | 2. SQLite graph snapshot；4. facts-only query adapter | 2.1 / 2.2 / 2.3 / 4.1 / 4.2 / UT-003 / UT-009 |

## 执行顺序

- 先执行 task 1，保证所有后续实现有 `wiki-index` DTO 和 trait 合同。
- 再执行 task 2，建立 SQLite graph snapshot、schema/read API、raw persistence、scoped update 清理和 `.spec` 三层 guard。
- 然后执行 task 3，把 workflow 和 readiness 接到 graph snapshot。
- 接着执行 task 4，完成 facts-only query 和 runtime route projection。
- 最后执行 task 5，跑系统验收、清理旧 fallback、整理稳定文档和测试报告输入。

## 暂缓事项

- 完整 type/scope/MRO/callback resolver、semantic/vector search、CLI 产品面、governance artifact index、archive 行为全部暂缓到后续 child。
- process/community 只做 storage/phase/query consumption 边界验收，不做算法质量优化。
