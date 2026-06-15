---
title: SpecWiki Code Graph Index Design
description: 定义 SpecWiki code graph/index 的职责、SQLite graph schema 方向、phase DAG、query adapter 和参考实现边界
updated: 2026-06-15
owner: architecture
status: draft
---

# SpecWiki Code Graph Index Design

## 文档定位

本文是 code graph / index 专项设计草案，从属于 [SpecWiki Contract Closure](./specwiki-contract-closure.md)。

本文只定义 `wiki-index` 的 graph substrate、phase DAG、schema 方向和 query adapter 边界，不重新定义页面树、truth kind、restore、projection ownership、query route 或 governance isolation。若本文与 `specwiki-contract-closure.md` 冲突，以上位合同为准。

## 上位合同摘录

Code graph 层的硬约束：

- 唯一 SQLite graph DB 是 `.wiki/.cache/wiki-cache.db`。
- graph tables 是当前工作区 source facts 的本地可重建 authority，不是 shared truth。
- `.wiki/.knowledge/**`、`wiki.metadata.json` 和 Markdown 页面不能伪恢复 symbol / edge graph。
- `init / update / rebuild / query` 的事实顺序固定为 `source repo -> code graph -> knowledge -> projection`。
- Agent 不直接写最终 Markdown，只消费 graph / knowledge context 并输出 structured artifact。

## 参考来源与采用方式

| 来源 | 目标落点 | 采用方式 |
| --- | --- | --- |
| `.upstream/codegraph/src/db/schema.sql` | `wiki-index` 的 SQLite graph schema、FTS、unresolved refs | 改写借鉴 |
| `.upstream/codegraph/src/extraction/index.ts` | git-aware scanning、增量 sync、parse budget、worker/error 策略 | 改写借鉴 |
| `.upstream/codegraph/src/resolution/index.ts` | import/name/framework resolution、resolved edge provenance | 改写借鉴 |
| `.upstream/codegraph/src/context/index.ts` | context pack、代码片段预算、query result shaping | 仅借鉴 |
| `.upstream/GitNexus/ARCHITECTURE.md` | `wiki-index` phase DAG、process/community 分析、graph consumption | 改写借鉴 |
| `.upstream/GitNexus/type-resolution-system.md` | SemanticModel、receiver-bound call resolution、MRO、cross-file binding | 后续阶段借鉴 |

禁止直接迁移 upstream CLI、Web UI、多仓 registry、Embedding、LadybugDB 或与 SpecWiki local repo runtime 无关的服务层。

## wiki-index 职责

`wiki-index` 负责当前 source repo 的事实索引：

```text
scan
structure
parse
resolve
analyze
query
```

它应产出：

- files / folders
- symbols
- edges
- raw imports / calls / heritage captures
- unresolved refs
- routes / tools
- communities
- processes / process steps
- FTS indexes

它不负责：

- declared knowledge
- derived knowledge
- Markdown rendering
- metadata binding
- governance evidence truth
- archive workflow

`.spec` governance artifact 不进入 code graph files / symbols / edges。治理引用由独立 artifact reference index 处理。

## SQLite Graph Schema 方向

目标 graph schema 至少覆盖：

```text
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
symbols_fts
files_fts
```

schema owner 归属：

```text
wiki-index owns:
  graph schema
  graph migrations
  graph query DTO

wiki-runtime implements:
  storage adapter
  transaction boundary
  lifecycle checkpoint mirror

wiki-knowledge consumes:
  GraphQuery DTO
  affected scope input
  evidence-backed source refs
```

其它 crate 不得直接跨层解释 graph 私有表。

## SymbolNode Contract

当前 `SymbolNode` 过薄。目标字段至少包括：

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
provenance
```

稳定性要求：

- `symbol_id` 必须在同一源码 snapshot 内稳定。
- `qualified_name` 应优先来自语言语义，无法完整解析时允许 fail-soft。
- `owner_symbol_id` 用于表达 class / module / impl / object ownership。
- `provenance` 必须能说明 symbol 来自 parser、fallback heuristic 还是 recovered cache。

## Raw Capture Contract

raw captures 不能只留在内存里。

必须持久化：

```text
raw_imports
raw_calls
raw_heritage
```

raw capture 最小字段：

```text
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
```

语义：

- raw capture 可以 unresolved。
- unresolved 不等于 parse failure。
- resolve phase 只能消费 raw captures 和 graph context，不重新解析源码全文。
- parse diagnostics 必须可追踪到 capture / file。

## Phase DAG

目标 phase DAG：

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

phase 规则：

- phase 输入输出必须可诊断。
- phase 可增量执行，但必须保持 deterministic ordering。
- parse 可以 fail-soft，但不得吞掉 diagnostics。
- resolve phase 必须保留 unresolved refs。
- analyze phase 不得反向修改 symbol / raw capture truth。

第一阶段不要求完整 scope resolution，但必须预留 phase 边界。

## GraphStore / IndexQueryStore

`wiki-index` 应定义 store traits：

```text
GraphStore
  write_files
  write_symbols
  write_edges
  write_raw_captures
  write_unresolved_refs
  read_graph_snapshot

IndexQueryStore
  find_symbol
  find_path
  find_edges
  find_callers
  find_callees
  impact_slice
  context_pack
```

`wiki-runtime` 可以实现这些 traits，但不得把 graph table schema 变成 runtime-owned schema。

query fusion 只能消费 adapter DTO：

```text
IndexQueryResult
GraphContextPack
ImpactSlice
SymbolHit
PathHit
GraphHit
```

不得让 host / skill / wiki-knowledge 直接跨库 join graph 私有表。

## Query Adapter Contract

index query adapter 输出必须能映射到上位 query route：

```text
index_symbol_hit
index_path_hit
index_graph_hit
```

最小字段：

```text
ref_kind
ref_id
label
score
provenance
confidence
recommended_action
source_refs
diagnostics
```

`rendered_page_debug_fallback` 不属于 `wiki-index`。

## Graph Readiness

graph readiness 至少包含：

```text
ready
stale
missing
rebuilding
blocked
```

判定原则：

- `.cache` 缺失时，graph readiness 是 `missing`，不能由 `.knowledge` 或 metadata 伪造。
- 源码 snapshot 与 graph snapshot 不一致时，graph readiness 是 `stale`。
- graph tables 缺失关键表或 migration 不一致时，graph readiness 是 `blocked`。
- rebuild 从当前 source repo 重建 graph，成功后才能进入 `ready`。

## Workflow 对接

### init

```text
validate config
  -> build graph
  -> analyze graph
  -> expose GraphQuery DTO
  -> pass graph context to knowledge planning
```

`init` 后必须先存在可查询 graph，后续 knowledge 和 projection 才能消费 graph facts。

### update

```text
detect source changes
  -> refresh affected graph phases
  -> produce index delta
  -> map to affected knowledge scope
```

`.spec` changes 不进入 graph update，只进入 governance artifact reference index。

### rebuild

```text
drop / ignore stale graph cache
  -> rebuild graph from current source
  -> rebuild FTS
  -> refresh readiness
```

`.knowledge` 可以帮助恢复 knowledge cache，但不能恢复 graph facts。

## 风险与约束

### Graph 重构过大

风险：一次性照搬 CodeGraph / GitNexus 造成长期半成品。

约束：先做核心语言和核心 graph 表，再扩 process/community 质量。

### Query 信任失真

风险：page fallback 或 stale graph 被当成 direct answer。

约束：graph readiness 必须进入 query response，fallback 只能降级。

### 双写漂移

风险：SQLite、`.knowledge`、metadata、Markdown 互相补字段。

约束：每条读取路径必须声明读取的是 source truth、local graph authority、shared knowledge truth、projection output 还是 cache。

### 非确定性 Agent

风险：同一 graph snapshot 生成不同 knowledge。

约束：Agent 输出必须绑定 input hash、source refs、unit id、provider/version。

## 验收标准

- `init` 后存在可查询 graph，再进入 knowledge planning。
- `raw_imports / raw_calls / raw_heritage` 持久化，不只存在内存。
- `SymbolNode` 至少具备 qualified name、signature、kind、language、range 和 provenance。
- `.cache` 缺失时不允许从 metadata、Markdown 或 `.knowledge` 伪造 graph。
- query 输出能区分 `index_symbol_hit`、`index_path_hit` 和 `index_graph_hit`。
- `.spec` 不进入 code graph files / symbols / edges。
- phase diagnostics 可追踪到 file、parser、capture 或 resolver。

## 结论

Code graph/index 是 SpecWiki 的 facts substrate。它必须足够结构化、可诊断、可增量和可查询，但它仍然只是本地可重建 authority，不是 shared knowledge truth。
