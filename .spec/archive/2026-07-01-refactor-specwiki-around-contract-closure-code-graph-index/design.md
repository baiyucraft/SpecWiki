# refactor-specwiki-around-contract-closure-code-graph-index 设计方案

## 方案概述

本方案把当前已有的 symbol graph 雏形收口为 SpecWiki 本地可重建的 code graph authority。设计重点不是提高解析器准确率到 upstream 水平，而是闭合 graph substrate 的合同、持久化、读取、诊断、readiness 和 query adapter 边界。

本 change 只交付本地 SQLite graph/index 的第一阶段合同闭合：

- `wiki-index` 作为 graph 合同层，定义 DTO、store trait、query DTO 和 readiness DTO。
- `wiki-runtime` 继续作为 SQLite adapter 和 workflow 编排层，负责实现私有表结构、事务和 lifecycle 接入。
- `wiki-model` 已有 query route DTO 不重命名、不扩 route tag；本 change 只让 `index_symbol_hit / index_path_hit / index_graph_hit` 具有真实 graph substrate 支撑。
- `.spec` governance artifacts 不进入 code graph facts，governance artifact index 延后到后续 child。

核心边界：

```text
source repo
  -> scan / structure
  -> parse symbols + raw captures
  -> resolve imports / calls / heritage
  -> analyze graph views
  -> build FTS
  -> GraphStore / IndexQueryStore DTO
  -> runtime query projection
```

## 架构分析

当前代码已经具备以下基础：

- `crates/wiki-index/src/symbols/models.rs` 定义 `SymbolNode`、raw import/call/heritage capture 和 `ParsedSymbolsSnapshot`。
- `crates/wiki-index/src/symbol_graph/pipeline.rs` 已串起 `resolve_imports -> resolve_calls -> resolve_heritage -> analyze_symbol_graph`。
- `crates/wiki-index/src/store.rs` 已有 `IndexSnapshotStore` / `IndexQueryStore` 雏形。
- `crates/wiki-runtime/src/storage/sqlite_store.rs` 已有 `symbols`、`edges`、communities、processes 和 `symbols_fts`。
- `crates/wiki-runtime/src/workflows/query.rs` 已把 index 结果投影到 `QueryResultDto` 的 `index_symbol_hit / index_path_hit / index_graph_hit`。

本方案不重写上述主链，而是在现有边界上补齐缺口：

```text
wiki-index
  owns:
    graph DTO
    graph store traits
    facts-only query DTO
    graph readiness DTO
    phase / diagnostic DTO

wiki-runtime
  owns:
    SQLite private schema
    SqliteIndexStore adapter
    workflow transaction boundary
    RuntimeReadiness projection

wiki-model
  owns:
    public query route DTO
    public query route tags
```

`wiki-runtime` 可以实现 SQLite 表、事务和 helper，但不得成为公共 graph 语义来源。`wiki-knowledge`、host、skill、CLI 和 query workflow 都只能通过 `wiki-index` trait/DTO 或既有 `wiki-model` query DTO 消费 graph。

Upstream 采用边界：

| 来源 | 目标落点 | 采用方式 | 明确排除 |
| --- | --- | --- | --- |
| `.upstream/codegraph/src/db/schema.sql` | symbols/files/edges/unresolved refs/FTS/schema metadata 方向 | 改写借鉴 | 不照搬 `nodes` 命名，不把 `file_path` 当唯一长期主键 |
| `.upstream/codegraph/src/extraction/index.ts` | scan/parse budget、git-aware 增量、diagnostics 思路 | 改写借鉴 | 不迁移 TS worker 实现、`.codegraph` 目录规则或 embedded repo 产品语义 |
| `.upstream/codegraph/src/resolution/index.ts` | unresolved-first、confidence、provenance、bounded resolve 思路 | 改写借鉴 | 不引入 callback synthesis 或完整多语言 built-in 列表 |
| `.upstream/codegraph/src/context/index.ts` | bounded context pack/query shaping | 仅借鉴 | 不引入 semantic/vector search 或 Claude-specific formatter |
| `.upstream/GitNexus/ARCHITECTURE.md` | phase DAG、phase output typing、process/community 边界 | 改写借鉴 | 不迁移 CLI/MCP/HTTP/Web、LadybugDB、embedding、global registry |
| `.upstream/GitNexus/type-resolution-system.md` | 保守解析原则、后续 receiver-bound 方向 | 后续阶段借鉴 | 不在本 change 实现 type/scope/MRO/fixpoint 全量能力 |

## 功能设计

### 1. wiki-index 合同层

在 `wiki-index` 中补齐 graph DTO。设计上保留现有类型名可以逐步调整，但公开合同要表达以下语义。

`SourceRange`：

```text
file_id
path
start_line
end_line
start_column
end_column
```

列信息允许 parser 暂时填 `0`，但字段必须存在，避免后续 query/source refs 再拆合同。

`SymbolNode` 最小稳定合同：

```text
symbol_id
file_id
language
symbol_kind
name
qualified_name
signature
docstring
visibility
owner_symbol_id
range
is_exported
provenance
```

`label` 可在实现阶段作为 `symbol_kind` 的过渡来源，但 design 合同以 `symbol_kind` 为主。`symbol_id` 在同一 source snapshot 内必须可重复；如果后续更换 ID 算法，应绑定 `file_id + symbol_kind + qualified_name/name + range`，并记录 provenance。

`SymbolProvenance`：

```text
parser_id
parser_version
source_kind
confidence
diagnostics
```

`source_kind` 至少区分 parser、heuristic、recovered 或 unknown。第一阶段不要求 parser_version 的具体字符串格式，只要求存在并能进入 diagnostics。

Raw capture 采用统一 envelope + 类型特有字段，不压扁成 JSON blob：

```text
RawCaptureBase
  capture_id
  file_id
  language
  capture_kind
  source_symbol_id
  raw_text
  target_hint
  range
  parser_id
  parser_version
  diagnostics

RawImportCapture
  base
  raw_path
  imported_name
  alias

RawCallCapture
  base
  called_name
  receiver_text
  argument_shape

RawHeritageCapture
  base
  owner_symbol_id
  owner_name
  target_name
  relation_kind
```

`UnresolvedRef` 不是普通 diagnostics 垃圾桶，必须绑定来源：

```text
unresolved_ref_id
capture_id
file_id
resolver_phase
reference_kind
reference_name
target_hint
range
candidates
reason
diagnostics
```

resolve phase 消费 raw captures 与 graph context，不重新解析源码全文。unresolved 不等于 parse failure，它是可查询、可诊断的 graph fact。

`GraphPhaseStatus`：

```text
phase
status
input_fingerprint
output_fingerprint
started_at
completed_at
diagnostics
```

第一阶段只要求记录 phase 状态和 diagnostics，不要求复杂 checkpoint/resume。

`GraphReadiness`：

```text
status: missing | stale | blocked | rebuilding | ready
reason
snapshot_id
source_fingerprint
required_tables
diagnostics
```

`GraphReadiness` 是 index/facts 层状态，不代表 knowledge/projection ready。

### 2. SQLite adapter 与 schema

SQLite graph 表仍位于 `.wiki/.cache/wiki-cache.db`，由 `wiki-runtime` 私有实现。schema 采用 SpecWiki 命名，不直接迁移 upstream `nodes` 表。

目标表族：

```text
graph_snapshots
files
folders
symbols
edges
raw_imports
raw_calls
raw_heritage
unresolved_refs
routes
tools
communities
community_members
processes
process_steps
graph_phase_runs
symbols_fts
files_fts
```

`files` / `folders` 是 code graph 的结构化 source authority。实现后 `SqliteIndexStore::list_sources()` 不应继续只从 `scan_cache` JSON 派生；`scan_cache` 可以保留为缓存或兼容内部 helper，但不能成为第二套 source truth。

`file_id` 与路径规范：

- 路径统一为 repo-root-relative、`/` 分隔、无前导 `./`。
- `file_id` 基于规范化 path 与 source fingerprint 规则生成。
- `source_fingerprint` 来自 scan 阶段的内容 hash 或等价 fingerprint。
- `graph_snapshots.snapshot_id` 绑定本轮 source fingerprint 集合，供 stale 判断。

事务边界：

```text
prepare graph snapshot
  -> write files / folders
  -> write symbols / raw_* / unresolved_refs / edges
  -> write analysis tables
  -> rebuild symbols_fts / files_fts
  -> write graph_phase_runs
  -> update graph_snapshots current pointer
```

`replace_symbol_graph` 和 `replace_symbol_graph_for_files` 必须写成同一 graph snapshot 的一致集合。`symbols`、`edges`、`raw_*`、`unresolved_refs`、`graph_phase_runs` 和 FTS 不能出现半新半旧。scoped update 可以先采用“受影响文件替换 + 全局 analysis 重写”的现有策略，但必须保证当前 snapshot pointer 只在事务完成后切换。

`.spec` persistence guard：

- scanner 输入层继续排除 `.spec`。
- 写入 `files / folders / symbols / edges / raw_* / unresolved_refs / *_fts` 前再次拒绝 `.spec` 路径。
- query 层不得从 graph 表返回 `.spec` 命中。

### 3. Workflow 与 readiness

init、update、rebuild 继续沿用现有主链，但把 raw captures、unresolved refs、phase runs 和 graph snapshot 一并写入。

```text
scan
  -> structure
  -> parse
  -> resolve_imports
  -> resolve_calls
  -> resolve_heritage
  -> analyze_communities
  -> analyze_processes
  -> build_fts
```

当前 `resolve_symbol_graph` 可以继续是实现入口，但 design 语义上要把 imports/calls/heritage 分成可诊断 phase。communities/processes 第一阶段只固定存储、phase 和 query consumption 边界，不承诺高质量分析。

readiness evaluator 替代“只返回 bool”的判断口径。原 `runtime_mirror_ready` / `index_graph_ready` 可保留为内部 helper 或兼容 wrapper，但新判断源应能生成 `GraphReadiness`，再投影到 `RuntimeReadiness.index`。

Readiness 决策表：

| 场景 | GraphReadiness | RuntimeReadiness.index | 说明 |
| --- | --- | --- | --- |
| DB 不存在 | missing | Missing | `.cache` 缺失，不能伪造 graph |
| 关键表缺失 | blocked | Blocked | schema 初始化或 migration 不完整 |
| schema/migration 不一致 | blocked | Blocked | 需要 rebuild/repair，而不是继续 query |
| source snapshot 与当前 scan 不一致 | stale | Stale | graph 可诊断但不能作为 fresh source facts |
| phase run 标记 rebuilding | rebuilding | Rebuilding | query 不返回 index graph 命中 |
| phase diagnostics 标记阻断 | blocked | Blocked | 保留 diagnostics 和 recommended action |
| snapshot 完整且来源为 source rebuild/update | ready | Ready | index query 可用 |

index 不 ready 时，query workflow 只能返回 knowledge/projection degraded 结果或 diagnostic，不得产出 `index_symbol_hit / index_path_hit / index_graph_hit` 伪命中。

### 4. Query adapter

`wiki-index::query` 保持 facts-only：

- 不读取 runtime state。
- 不读取 Markdown page fallback。
- 不读取 governance artifacts。
- 不跨层 join runtime 私有状态表。

`IndexQueryStore` 需要补齐 path、raw/unresolved 和 graph 相关读取能力，但 runtime query workflow 只能通过 trait/DTO 消费，不能直接 JOIN SQLite 私有表。

query DTO 映射保持既有 `wiki-model::domain::query`：

```text
SymbolHit -> QueryResultDto(route_tag = index_symbol_hit)
Source/Path hit -> QueryResultDto(route_tag = index_path_hit)
Edge/impact/process/community hit -> QueryResultDto(route_tag = index_graph_hit)
```

第一阶段 query 增强范围：

- symbol 命中带 `file_id`、range、provenance 和 source refs。
- path 命中优先来自 `files` / `files_fts`，不再只遍历 scan JSON。
- graph 命中带 edge provenance、confidence、hop distance 和 diagnostics。
- bounded context pack 可作为 DTO 预留或最小输出，但不做 semantic/vector ranking。

不新增 route tag，不重命名 `QueryResultDto` 字段，不把 page fallback 伪装成 index route。

## 数据设计

新增或强化的数据结构分三层。

`wiki-index` DTO 层：

```text
SourceFileRecord
FolderRecord
SourceRange
SymbolNode
SymbolProvenance
RawImportCapture
RawCallCapture
RawHeritageCapture
UnresolvedRef
GraphPhaseStatus
GraphReadiness
GraphSnapshotRef
```

SQLite 私有表层：

```text
graph_snapshots(snapshot_id, source_fingerprint, schema_version, status, created_at)
files(file_id, path, language, kind, fingerprint, size, indexed_at, diagnostics)
folders(folder_id, path, parent_id)
symbols(...extended SymbolNode fields...)
edges(edge_id, source_id, target_id, edge_type, confidence, provenance, reason, range)
raw_imports(...)
raw_calls(...)
raw_heritage(...)
unresolved_refs(...)
graph_phase_runs(...)
files_fts(...)
symbols_fts(...)
```

`scan_cache` 可以继续保存原始 `ScanReport`，但正式 source record 读取应从 `files` 表出来，避免 JSON cache 与 graph tables 双真相。

删除/替换语义：

- full rebuild：生成新 graph snapshot，事务完成后切换 current pointer。
- scoped update：按 file_id 替换受影响文件的 symbols/raw/unresolved/edges，再重写必要 analysis 与 FTS；不得留下旧 symbol 的 edge。
- `.spec` 路径：落库前拒绝，不产生 tombstone，也不进入 unresolved。

## 接口设计

本 change 不新增 CLI 命令、不改变用户一级命令。接口变化集中在 crate 内部合同。

`IndexSnapshotStore` 推荐新增或调整的能力：

```text
write_source_snapshot
write_graph_snapshot
replace_graph_snapshot
replace_graph_snapshot_for_files
write_phase_statuses
read_graph_readiness
```

`IndexQueryStore` 推荐新增或调整的能力：

```text
list_files
search_files
list_raw_captures
list_unresolved_refs
read_graph_readiness
impact_slice
context_pack
```

具体 Rust 方法名、参数拆分和 helper 结构留到 implementation plan 决定；design 只固定职责和数据方向。

`RuntimeReadiness` 衔接：

- `GraphReadiness.ready` 映射到 `LayerReadiness::Ready`。
- `GraphReadiness.stale` 映射到 `LayerReadiness::Stale`。
- `GraphReadiness.missing` 映射到 `LayerReadiness::Missing`。
- `GraphReadiness.rebuilding` 映射到 `LayerReadiness::Rebuilding`。
- `GraphReadiness.blocked` 映射到 `LayerReadiness::Blocked`。

query 衔接：

- `wiki-index::query` 返回 facts-only index DTO。
- `wiki-runtime::workflows::query` 继续负责投影 `QueryResultDto`。
- transport payload 不新增 route tag。

## 非功能性设计

性能：

- FTS 构建随 graph snapshot 写入，不在 query 时临时扫描全部源码。
- scoped update 第一阶段优先保证一致性；复杂增量优化留给后续。
- raw captures 和 unresolved refs 需要可批量写入，避免逐条事务。

可靠性：

- graph snapshot pointer 只在事务成功后更新。
- readiness 必须能识别半写、关键表缺失和 schema mismatch。
- query 在 index 不 ready 时降级，不伪造 index 命中。

可维护性：

- graph DTO 与 SQLite schema 分层，避免 runtime 私表泄漏到 knowledge/host。
- raw/unresolved/phase diagnostics 都有稳定引用，可从测试报告定位到 file/capture/resolver。
- upstream 借鉴必须保留来源、目标落点和采用方式，不直接复制产品架构。

兼容性：

- 当前测试开发阶段不保留旧兼容层。
- 允许破坏旧的薄 `SymbolNode` 内部结构，但必须同步当前 crate 内所有构造、测试和 SQLite 读写。

## 资源评估

本 change 会增加本地 SQLite 存储：

- `raw_*` 与 `unresolved_refs` 随源码引用数量增长。
- `files_fts` 与扩展后的 `symbols_fts` 增加索引空间。
- `graph_phase_runs` 与 diagnostics 为每次 snapshot 保留最小记录。

资源约束：

- 不引入外部 DB、服务进程或网络依赖。
- 不引入 embedding/vector index。
- 不改变 `.wiki/.cache/wiki-cache.db` 作为唯一本地图 DB 的定位。

第一阶段可以只保留当前 snapshot 的 graph facts；历史 snapshot 保留策略留给后续。

## 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| scope 扩大成完整解析器重写 | 高 | design 明确验收是合同闭合，不是解析准确率追平 upstream |
| SQLite 表扩展但 trait/DTO 不闭合 | 高 | 先改 `wiki-index` 合同，再由 `SqliteIndexStore` 实现 |
| graph snapshot 半新半旧 | 高 | symbols/raw/edges/unresolved/FTS/phase runs 同事务写入，同事务切 pointer |
| readiness 继续 bool 化 | 高 | 引入 `GraphReadiness` 决策表并投影到 `RuntimeReadiness.index` |
| `.spec` 污染 graph facts | 高 | scanner + persistence + query 三层 guard |
| process/community 被误解为质量承诺 | 中 | 只固定 phase/storage/query 边界，不承诺算法质量 |
| query ranking 吞掉 substrate 目标 | 中 | 只做 facts-only 最小 query adapter，不引入 semantic/vector search |

## 设计决策

- `wiki-index` 是 graph 合同层，`wiki-runtime` SQLite 是 adapter，不把 SQLite 私表作为跨层公共接口。
- `files` / `folders` 成为 code graph 的结构化 source authority，`scan_cache` 不再作为正式 source record 的唯一来源。
- raw capture 使用统一 envelope + 类型特有字段，避免 JSON blob，也避免 import/call/heritage 语义被抹平。
- unresolved refs 是一等 graph fact，必须绑定 capture、phase、range 和 reason。
- graph readiness 只评价 index/facts 层，不代表 knowledge/projection readiness。
- `wiki-index::query` 继续 facts-only，runtime 负责投影既有 query route DTO。
- 本 change 不新增 route tag、不引入 embedding/vector search、不实现完整 type/scope resolver。

## 待确认问题

- `file_id` 的具体生成算法采用 `stable_id("file", path)` 还是绑定 source fingerprint，留到 plan/tasks 在测试约束下确定。
- `parser_id` / `parser_version` 的具体字符串格式留到实现阶段确定，但字段必须存在。
- scoped update 是否第一阶段直接重写全量 analysis，或按受影响文件做最小重算，留到 tasks 按风险拆分。
- `context_pack` 第一阶段是只保留 DTO 预留，还是实现最小 path/symbol/edge pack，留到 plan 阶段结合测试成本决定。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure-code-graph-index/proposal.md`
- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/specwiki-code-graph-index-design.md`
- `.spec/archive/2026-06-16-refactor-specwiki-around-contract-closure-truth-restore-snapshot/proposal.md`
- `.spec/archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/proposal.md`
- `.upstream/codegraph/src/db/schema.sql`：改写借鉴 SQLite graph schema、FTS、unresolved refs 和 schema metadata。
- `.upstream/codegraph/src/extraction/index.ts`：改写借鉴 scan/parse budget、diagnostics 和增量 sync 思路。
- `.upstream/codegraph/src/resolution/index.ts`：改写借鉴 unresolved-first、confidence 和 provenance 边界。
- `.upstream/codegraph/src/context/index.ts`：仅借鉴 bounded context pack 与 query shaping 思路。
- `.upstream/GitNexus/ARCHITECTURE.md`：改写借鉴 phase DAG、phase output typing 和 graph consumption 边界。
- `.upstream/GitNexus/type-resolution-system.md`：后续阶段借鉴保守 receiver-bound resolution 原则。
