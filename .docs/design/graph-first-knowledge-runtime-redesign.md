---
title: Graph-First Knowledge Runtime Redesign
description: 重新设计 spec-wiki 的 SQLite Graph、Knowledge truth 与 Wiki projection 主链
updated: 2026-05-26
owner: architecture
status: draft
---

# Graph-First Knowledge Runtime Redesign

## 目标

本设计把当前 Repo Wiki runtime 收敛为三层真相源：

```text
Source Repo
  -> .wiki/.cache/wiki-cache.db
  -> .wiki/.knowledge
  -> .wiki/*.md
```

含义：

- `.wiki/.cache/wiki-cache.db` 是本地 facts / index / graph 权威索引。
- `.wiki/.knowledge/**` 是可上库、可审计、可恢复的知识权威来源。
- `.wiki/*.md` 是面向人和 Agent 阅读的页面投影，不再作为正式 query truth。

这不是推翻当前主链，而是把已有的 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 收紧为：

```text
Build SQLite Graph
  -> Build Knowledge
  -> Project Pages
```

## 非目标

- 不新增第二套 `.cache/` 或第二个 graph DB。
- 不照搬 GitNexus 的 Web UI、多仓 registry、Embedding、LadybugDB。
- 不让 Agent 直接写最终 Markdown 页面。
- 不把 `.wiki/*.md` 的全文搜索作为正式 query truth。
- 不把 `wiki.metadata.json` 当成恢复 index graph 的事实来源。

## 参考来源与采用方式

| 来源 | 目标落点 | 采用方式 |
| --- | --- | --- |
| `.upstream/codegraph/src/db/schema.sql` | `wiki-index` 的 SQLite graph schema、FTS、unresolved refs | 改写借鉴 |
| `.upstream/codegraph/src/extraction/index.ts` | git-aware scanning、增量 sync、parse budget、worker/error 策略 | 改写借鉴 |
| `.upstream/codegraph/src/resolution/index.ts` | import/name/framework resolution、resolved edge provenance | 改写借鉴 |
| `.upstream/codegraph/src/context/index.ts` | context pack、代码片段预算、query result shaping | 仅借鉴 |
| `.upstream/GitNexus/ARCHITECTURE.md` | `wiki-index` phase DAG、process/community 分析、graph consumption | 改写借鉴 |
| `.upstream/GitNexus/type-resolution-system.md` | SemanticModel、receiver-bound call resolution、MRO、cross-file binding | 后续阶段借鉴 |

## 新分层

```text
wiki-index
  scan / structure / parse / resolve / analyze / query

wiki-knowledge
  planning / research / compose / declared / derived / projection

wiki-runtime
  workflow orchestration / storage adapters / query fusion / lifecycle

Agents
  bootstrap / host adapters / Wiki Agent execution
```

### Layer 1: SQLite Graph

职责：

- 扫描源码、配置、文档和入口。
- 建立 files、folders、symbols、edges、unresolved refs、raw captures。
- 支持 symbol lookup、call trace、impact slice、process、community 和 context pack。
- 为 knowledge planning 和 query 提供 facts-owned substrate。

目标 schema 方向：

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
```

当前 `SymbolNode` 过薄，后续需要补：

- `qualified_name`
- `signature`
- `docstring`
- `visibility`
- `owner_symbol_id`
- `symbol_kind`
- `language`
- `provenance`

raw captures 不能只留在内存里。要持久化 raw import / call / heritage，才能让 graph 可审计、可诊断、可增量修复。

### Layer 2: Knowledge Truth

`.wiki/.knowledge/**` 是知识权威来源，分三类：

```text
.wiki/.knowledge/
  declared/
  derived/
  runtime/
```

`declared/`：

- 人明确沉淀的规范、约定、决策、避坑和流程规则。
- 可由页面 declared block 经 `sync` 回写。
- 必须有 typed scope、状态、生命周期关系和 provenance。

`derived/`：

- 从 SQLite Graph 和 Wiki Agent research 生成的 KnowledgeDomain、KnowledgeUnit、research summary、PageDigest。
- 可重建，但一旦进入 artifact，就成为本次页面投影的正式输入。

`runtime/`：

- recovery manifest、runtime gates、health signals、projection binding。
- 用于恢复 knowledge cache 和判断 query/update readiness。

### Layer 3: Markdown Projection

`.wiki/*.md` 是投影层：

- 供人阅读。
- 供 Agent 快速导航。
- 允许保留 user sections。
- 允许作为 declared authoring surface。
- 不作为 query 的正式 truth。

页面生成只能来自 `.knowledge`：

```text
KnowledgeUnit + UnitResearch + PageDigest + DeclaredRecord
  -> PageDraft
  -> .wiki/*.md
```

页面人工修改必须通过：

```text
page edit
  -> sync
  -> declared / projection artifact
  -> update
  -> regenerated page
```

## Workflow

### init

```text
validate config
  -> build SQLite Graph
  -> analyze graph
  -> plan KnowledgeTree
  -> run Wiki Agent research
  -> write .knowledge
  -> compose PageDrafts
  -> assemble .wiki/*.md
  -> write metadata
```

Agent 不直接写 Markdown。Agent 输出必须是结构化 artifact：

- `SystemResearch`
- `DomainResearch`
- `UnitResearch`
- `PageDigest`
- `PageDraft`

### update

```text
detect source changes
  -> refresh affected graph phase
  -> compute affected knowledge scope
  -> refresh declared / derived knowledge
  -> reproject affected pages
```

update 的权威顺序固定：

```text
SQLite Graph
  -> .knowledge
  -> .wiki/*.md
```

### sync

```text
parse edited pages
  -> accept only legal user sections / declared blocks
  -> write .knowledge declared records
  -> mark affected knowledge stale
  -> update projection
```

sync 不应把 Markdown 正文直接写成 derived knowledge。

### rebuild

```text
rebuild SQLite Graph from source
  -> reload .knowledge
  -> rebuild caches
  -> reproject pages when needed
```

`.knowledge` 可以恢复 knowledge cache，但不能伪造 index graph。index graph 缺失时必须重新扫描源码构建。

### query

正式 query 路由：

```text
term
  -> index graph query
  -> knowledge query
  -> fusion ranker
  -> page projection refs
```

降级 fallback：

```text
rendered page debug fallback
```

Markdown 全文 fallback 如果保留，必须降级为 debug / constrained，不参与 direct answer trust。

## Query Fusion

query 输出必须显式区分来源：

```text
index_graph_hit
knowledge_declared_hit
knowledge_derived_hit
projection_ref
rendered_page_debug_fallback
```

每条结果都要携带：

- `ref_kind`
- `ref_id`
- `label`
- `score`
- `provenance`
- `confidence`
- `recommended_action`

初始 ranking 规则：

```text
symbol / graph exact hit
  > declared knowledge hit
  > derived knowledge hit
  > projection ref
  > rendered page debug fallback
```

查询 readiness 拆成三类：

```text
index_readiness
knowledge_readiness
fusion_readiness
```

不再只用一个 `facts_ready` 判断 query 是否可用。

## 需要重构的关键代码点

| 文件 | 当前问题 | 目标 |
| --- | --- | --- |
| `crates/wiki-runtime/src/workflows/query.rs` | query 同时读 index、knowledge、WikiState 和 Markdown fallback | 拆成 `IndexQueryAdapter`、`KnowledgeQueryAdapter`、`FusionRanker` |
| `crates/wiki-runtime/src/storage/sqlite_store.rs` | index、knowledge、runtime page state 混在同一存储文件 | 拆清 graph authority、knowledge cache、runtime cache |
| `crates/wiki-runtime/src/storage/sqlite/index_store.rs` | 最接近 SQLite Graph 权威入口，但 source 仍来自 scan_cache JSON | 强化为结构化 graph store |
| `crates/wiki-runtime/src/storage/knowledge_artifacts.rs` | restore 会从 `.knowledge` / metadata 伪恢复部分 runtime state | 拆成 knowledge cache restore 与 index rebuild |
| `crates/wiki-runtime/src/workflows/init.rs` | 一个流程同时写 graph、knowledge、page、state、metadata | 拆成 build index / build knowledge / project pages |
| `crates/wiki-runtime/src/workflows/update.rs` | 增量 graph refresh、knowledge refresh、page render 混在一起 | 按 index delta -> knowledge delta -> projection delta 重排 |
| `crates/wiki-runtime/src/workflows/sync.rs` | 页面改动会同步 state/metadata/knowledge 多处 | 只接受合法 authoring，权威写入 `.knowledge` |
| `crates/wiki-index/src/store.rs` | store trait 还偏 snapshot，而不是 graph authority | 拆 `SourceStore`、`GraphStore`、`IndexQueryStore` |
| `crates/wiki-index/src/symbols/models.rs` | `SymbolNode` 过薄，raw captures 不持久化 | 扩展 symbol schema，持久化 raw captures |
| `crates/wiki-knowledge/src/research.rs` | `ResearchDataSource` 依赖大内存快照 | 改为 GraphQuery + KnowledgeStore 数据源 |

## 实施阶段

### Phase 1: Design Contract

- 固化三层真相源。
- 明确 `.wiki/.cache/wiki-cache.db` 是唯一 SQLite Graph。
- 明确 `.knowledge` 是页面内容权威来源。
- 明确 `.wiki/*.md` 是 projection。

### Phase 2: Graph Schema

- 参考 CodeGraph 改写 SQLite schema。
- 扩展 `SymbolNode`。
- 持久化 raw captures 与 unresolved refs。
- 建立 graph FTS 和 source/module 结构表。

### Phase 3: Phase Pipeline

- 参考 GitNexus 建立本项目版 phase DAG：

```text
scan
  -> structure
  -> parse
  -> resolve_imports
  -> resolve_calls
  -> resolve_heritage
  -> analyze_communities
  -> analyze_processes
```

第一阶段不做完整 scope-resolution，只预留 phase 边界。

### Phase 4: Knowledge Authority

- `.knowledge` artifact 成为 PageDraft 的唯一正式输入。
- SQLite knowledge 表若保留，只作为 cache。
- `sync` 只写 declared records 和 projection invalidation。

### Phase 5: Query Fusion

- 删除正式 Markdown fallback。
- 建立 index + knowledge 融合查询。
- 输出多层 provenance 和 readiness。

### Phase 6: Wiki Agent

- Agent 只消费 GraphQuery 和 KnowledgeContext。
- Agent 只输出 structured artifact。
- runtime 负责落盘 `.knowledge` 和投影 Markdown。

## 风险与约束

### 双写漂移

风险：SQLite、`.knowledge`、metadata、Markdown 继续互相补字段。

约束：每条读取路径必须声明读取的是 authority、cache 还是 projection。

### 非确定性 Agent

风险：同一 graph 两次 init 产物不同。

约束：Agent 输出必须有 input hash、source citations、unit id、provider/version 信息。

### Graph 重构过大

风险：一次性照搬 CodeGraph/GitNexus 造成长期半成品。

约束：先做 Rust/TypeScript/JavaScript/Python 的核心 graph，再扩其他语言。

### Query 信任失真

风险：page fallback 让过期页面看起来像正式答案。

约束：rendered page fallback 不能提升为 direct answer。

## 验收标准

- `init` 后必须先存在可查询 SQLite Graph，再写 `.knowledge`，最后写 Markdown。
- `.knowledge` 缺失时，page projection 不被视为正式 query truth。
- `.cache` 缺失时，不允许从 metadata 或 `.knowledge` 伪造 symbol/edge graph。
- query 结果必须区分 index、declared knowledge、derived knowledge、projection 和 fallback。
- 人工页面编辑必须通过 sync 转成 declared record 或 user section，不直接污染 derived knowledge。

## 一句话结论

```text
spec-wiki 的核心不应是“生成一批 Markdown”，而应是：
先建立可查询的代码图，
再沉淀可审计的知识，
最后把知识投影成页面。
```
