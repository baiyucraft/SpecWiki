---
title: SpecWiki Contract Closure
description: 收口新版 SpecWiki 的页面树、truth 分层、恢复、投影归属、查询、治理隔离和产品体验合同
updated: 2026-06-15
owner: architecture
status: draft
---

# SpecWiki Contract Closure

## 文档定位

本文是新版 SpecWiki 设计草案的收口合同，用于统一当前多个 draft 中互相打架的核心边界。

它不表示当前代码已经实现，也不直接修改 `.wiki` 稳定知识。后续实现应先以本文为上位草案，再拆分 `.spec/changes/**` 逐步落地。

本文优先解决七类问题：

- 页面树到底写到哪里。
- index graph、knowledge、Markdown、metadata、cache、governance evidence 分别是什么 truth kind。
- `.cache` 缺失时到底能恢复什么，不能恢复什么。
- projection planning、render、merge、writeback 分别归哪个 crate。
- query route、readiness、ranking 和 fallback trust 如何统一。
- governance 如何融合进 SpecWiki，但不污染 core wiki runtime。
- 用户第一次使用、失败恢复和默认 CLI 输出应该如何解释。

## 设计总原则

SpecWiki 的第一核心承诺是：

```text
让人和 Agent 在同一个 repo 内共享可查询、可更新、可追踪来源的项目知识层。
```

因此新版主链固定为：

```text
Current Source Repo
  -> Local Rebuildable Code Graph
  -> Shared Knowledge Artifacts
  -> Markdown Projection / Authoring Surface
  -> Query / Status / Update / Governance
```

graph 侧固定约束：

- 唯一 SQLite graph DB 是 `.wiki/.cache/wiki-cache.db`。
- `init / update / rebuild / query` 的事实顺序固定为 `source repo -> code graph -> knowledge -> projection`。
- Agent 不直接写最终 Markdown，只输出绑定 input hash、source refs、unit id 和 provider/version 的 structured artifact；runtime 负责落盘 `.wiki/.knowledge/**` 并投影 Markdown。
- Code graph/index 细节见 [SpecWiki Code Graph Index Design](./specwiki-code-graph-index-design.md)。

用户侧第一心智固定为：

```text
init -> status -> query -> update
```

governance、sync、rebuild、repair、trace 和 archive 都是场景触发能力，不进入首次使用主路径。

## 1. Page Tree Contract

### 决策

新版 SpecWiki 的用户可见 Wiki 页面树采用唯一结构：

```text
.wiki/
  INDEX.md
  <栏目>/INDEX.md
  <栏目>/NN-主题.md
  <栏目>/<子栏目>/INDEX.md
  <栏目>/<子栏目>/NN-主题.md
```

`.wiki/pages/**` 不作为新版目标页面目录。

### 目录语义

```text
.wiki/INDEX.md
  项目知识总入口、阅读路径、当前状态入口

.wiki/<栏目>/INDEX.md
  栏目范围、页面索引、入口推荐、事实来源或不适用说明

.wiki/<栏目>/NN-主题.md
  稳定长期知识页
```

允许多级目录，但默认最多生成到二级；三级只有在代码结构或领域结构强烈支持时才生成。

### `.wiki/pages/**` 处理口径

```text
new runtime target: 禁止
legacy import source: 允许
compat write path: 禁止
```

如果旧产物中存在 `.wiki/pages/**`：

- `init / update / rebuild` 不再写入该目录。
- migration dry-run 可以读取它并生成迁移报告。
- 迁移后页面必须落到 `.wiki/INDEX.md`、栏目 `INDEX.md` 或 `NN-主题.md`。
- metadata 不得同时把 `.wiki/pages/**` 和 `.wiki/**/*.md` 都视为正式 page truth。

### Legacy Read Policy

当仓库同时存在旧 `.wiki/pages/**` 和新版用户可见页面树时：

```text
status
  -> report legacy_pages_detected / migration_pending

query
  -> 默认不读取 `.wiki/pages/**`
  -> 只有显式 debug / migration 模式才允许返回 legacy_page_debug_ref

update
  -> 不刷新 `.wiki/pages/**`
  -> 不把 `.wiki/pages/**` 纳入 AffectedProjectionScope

rebuild
  -> 不从 `.wiki/pages/**` 重建正式 metadata
  -> 可生成 migration report
```

legacy pages 不参与正式 page truth，不参与默认 query，不参与 projection readiness。它们只允许作为迁移输入或显式 legacy debug route。

### Markdown 页面术语

Markdown 页面统一称为：

```text
projection output / authoring surface
```

不要称为 `knowledge truth` 或 `query answer truth`。

本文后续使用 `.wiki/**/*.md` 时，只指用户可见 Wiki 页面树，必须排除：

```text
.wiki/.knowledge/**
.wiki/.cache/**
.wiki/pages/**
```

## 2. Truth Kind Contract

### Truth Kind Matrix

| 区域 | truth kind | 是否上库 | 主要职责 | 禁止承担 |
| --- | --- | --- | --- | --- |
| 当前源码 | source truth | 是 | 当前代码、配置、文档事实来源 | 长期知识状态 |
| `.wiki/.cache/wiki-cache.db` 的 graph tables | local rebuildable authority | 否 | 当前工作区 code graph、symbols、edges、FTS、process/community | shared truth、审计证据 |
| `.wiki/.knowledge/declared/**` | shared declared knowledge truth | 是 | 人确认的规则、决策、约定、避坑 | 派生事实缓存 |
| `.wiki/.knowledge/derived/**` | snapshot-scoped derived knowledge truth | 是 | 某次 graph/research/compose 的派生知识输入 | 被页面人工编辑直接改写 |
| `.wiki/.knowledge/runtime/**` | shared runtime recovery truth | 是 | projection digest、recovery manifest、runtime gates | 完整 page 正文 |
| `.wiki/**/*.md` 用户可见页面树 | projection output / authoring surface | 是 | 人读、Agent 导航、合法 declared authoring | derived truth、query answer truth |
| `.wiki/wiki.metadata.json` | binding truth | 是 | page/section/knowledge/source/evidence refs、hash、reverse refs | 知识正文、projection digest 主状态 |
| `.wiki/.cache/**` 其它表 | local cache / working state | 否 | query 加速、workflow checkpoint、本地 mirror | shared truth |
| `.spec/changes/**` | active governance evidence truth | 是 | active change artifact 和审计证据 | Wiki 正文 |
| `.spec/archive/**` | archived governance evidence truth | 是 | archived change artifact 和审计证据 | Wiki 正文 |

### Derived Snapshot 规则

`DerivedRecord` 可重建，但一旦写入 `.wiki/.knowledge/derived/**`，它就是本次 projection 的正式输入。它必须绑定：

```text
derived_record_id
input_hash
generator_id
generator_version
graph_snapshot_id
knowledge_snapshot_id
source_refs
created_at
status
```

缺少这些字段时，`update / rebuild` 不能可靠判断 derived record 是 fresh、stale、obsolete 还是 conflict。

### Metadata 权限边界

`wiki.metadata.json` 只负责绑定和索引：

```text
page_id / section_id
path
knowledge_refs
source_refs
evidence_refs
content_hash
input_hash
reverse_refs
snapshot refs
```

`ProjectionDigest`、recovery manifest 和 runtime gates 放在 `.wiki/.knowledge/runtime/**`。

本地 lifecycle checkpoint 放在 `.wiki/.cache/**`。

metadata 不得和 ProjectionDigest 双写同一份 readiness 主状态。

### Committed Snapshot Manifest

shared committed snapshot 的唯一主记录位于：

```text
.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml
```

manifest 负责记录：

```text
snapshot_id
graph_snapshot_id
knowledge_snapshot_id
projection_snapshot_id
metadata_hash
page_hashes
projection_digest_refs
created_at
status
```

`wiki.metadata.json` 只能保存当前 `snapshot_id` 指针和 binding index，不保存 runtime gates 的主状态。

`.wiki/.cache/**` 可以镜像当前 snapshot 以加速读取，但不得成为 committed snapshot truth。

## 3. Restore Contract

### 两级恢复

`.cache` 缺失后的恢复必须分成两级。

```text
Level 1: Knowledge / Projection Runtime Restore
  input:
    .wiki/.knowledge/**
    .wiki/**/*.md 用户可见页面树
    .wiki/wiki.metadata.json

  output:
    status 可诊断
    artifacts 校验通过时，knowledge query 可用
    artifacts 校验通过时，page refs 可用
    artifacts 校验通过时，projection binding 可检查
    artifacts 校验失败时，仅 diagnostic / status 可用
    index_readiness = missing / stale

Level 2: Code Graph Rebuild
  input:
    current source repo

  output:
    graph tables ready
    symbol / edge / impact query 可用
    index_readiness = ready
```

### 禁止伪恢复

- `.knowledge` 不能伪造 symbol graph。
- `wiki.metadata.json` 不能伪造 index graph。
- 页面全文不能提升为 current source facts。
- cache restore 成功不能被表述为完整 graph ready。
- `.knowledge`、metadata、projection digest 或 committed snapshot manifest 不一致时，Level 1 restore 只能进入 diagnostic mode。

### Status 组合

status 至少要拆成：

```text
index_readiness: ready / stale / missing / rebuilding / blocked
knowledge_readiness: ready / stale / conflict / missing / blocked
projection_readiness: ready / stale / conflict / missing / blocked
governance_readiness: ready / not_enabled / blocked
fusion_readiness: ready / degraded / blocked
```

典型组合：

| 状态组合 | query 能力 | 默认 next action |
| --- | --- | --- |
| index ready + knowledge ready + projection ready | 全能力可用 | none |
| index missing + knowledge ready + projection ready | knowledge / page refs 可用，graph direct answer 不可用 | `rebuild` 或 `update` |
| index ready + knowledge stale | graph query 可用，knowledge answer 降级 | `update` |
| projection conflict | query 可返回非冲突层，页面 refs 降级 | `sync` 或 `repair --plan` |
| governance blocked + core ready | 普通 query / update 可用，archive / release gate 阻断 | `validate <change-id>` |

## 4. Projection Ownership Contract

### Crate Ownership

```text
wiki-model
  stable DTO / IDs / enums / refs

wiki-index
  source scan / graph build / symbol-edge-process facts / index query

wiki-knowledge
  knowledge planning
  research / compose contracts
  DeclaredRecord / DerivedRecord
  PagePlan / SectionPlan
  projection intent

wiki-runtime
  workflow orchestration
  render
  managed section merge
  page write
  metadata update
  storage adapters
  lifecycle / transport
```

### Projection Chain

```text
KnowledgeUnit / DeclaredRecord / DerivedRecord
  -> ProjectionDigest
  -> PagePlan
  -> SectionPlan
  -> PageDraft
  -> PageProjection
  -> .wiki/**/*.md 用户可见页面树 + wiki.metadata.json
```

### 禁止规则

- `wiki-knowledge` 不直接写 Markdown。
- `wiki-runtime` 不重新决定 knowledge scope。
- render 不从最终 Markdown 反推 knowledge。
- managed section merge 不能覆盖 `manual_unmanaged`。
- derived section drift 不能写回 derived knowledge。

### Declared Authoring Writeback

合法 declared section 的回写流程固定为：

```text
wiki-runtime
  -> parse visible wiki pages
  -> detect managed markers / declared authoring blocks
  -> extract candidate declared edits

wiki-knowledge
  -> validate declared contract
  -> normalize scope / lifecycle / relation
  -> produce DeclaredRecord patch

wiki-runtime
  -> write `.wiki/.knowledge/declared/**`
  -> update metadata binding
  -> mark affected projection stale
```

失败状态：

| failure | owner | resulting state |
| --- | --- | --- |
| marker missing / malformed | runtime | `projection_readiness: conflict` |
| scope invalid | knowledge | `declared_writeback: rejected` |
| lifecycle relation invalid | knowledge | `declared_writeback: conflict` |
| metadata binding mismatch | runtime | `projection_readiness: blocked` |
| derived section edited | runtime | `illegal_drift` |

runtime 负责检测、提取、落盘和绑定；knowledge 负责 declared 语义校验与 record 归一。

### Store Trait 边界

```text
wiki-index
  -> GraphStore / IndexQueryStore

wiki-knowledge
  -> KnowledgeArtifactStore / KnowledgeCacheStore

wiki-runtime
  -> RuntimeStore / WorkflowCheckpointStore
  -> 实现 storage adapters，但不解释 graph / knowledge 私有 schema

query fusion
  -> 只消费 adapter DTO
  -> 不跨库 join 私有表
```

## 5. Update / Delete / Commit Contract

### Affected Scope 输入

`update` 的主线固定：

```text
ChangeSet
  -> AffectedKnowledgeScope
  -> AffectedProjectionScope
  -> refresh knowledge
  -> render impacted pages / sections
  -> commit snapshot
```

ChangeSet 至少支持：

| change kind | scope target |
| --- | --- |
| source path changed | path / module / symbol |
| symbol signature changed | symbol / callers / impacted module |
| config changed | config / repo / affected workflow |
| docs changed | declared authoring / page projection / repo docs |
| `.spec` changed | governance evidence / governance summary |
| package manifest changed | dependency / build / runtime surface |
| deletion observed | stale suspect，不能立即物理删除 |

`.spec changed` 不进入 `scan_repo` 业务源码 facts，也不触发 code graph files/symbols/edges 更新。

它只触发：

```text
governance artifact reference index refresh
governance summary refresh
governance readiness recompute
optional governance query cache refresh
```

governance blocked 不得阻断 core `update` 对 source / knowledge / projection 的提交。

### 删除语义

```text
absent_once
  -> stale / suspect

confirmed_removed
  -> obsolete / tombstone

user_confirmed_or_policy_allowed
  -> physical delete page / section
```

Declared knowledge 不因单次扫描缺失物理删除。

Derived knowledge 可 obsolete，但必须保留本次判断的 source evidence。

页面或 section 物理删除前必须检查：

```text
inbound refs
metadata reverse refs
projection digest
manual sections
governance evidence refs
```

### Snapshot Commit Protocol

正式写入必须采用可测试的 snapshot commit：

```text
prepare snapshot
  -> write temp artifacts
  -> validate refs / hashes
  -> atomically swap metadata pointer or manifest
  -> cleanup old temp
  -> report committed snapshot id
```

失败后必须能进入 degraded state：

```text
partial_write_detected
  -> status: blocked
  -> trusted snapshot: previous committed snapshot
  -> next action: repair --plan
```

## 6. Query Route Contract

### Route Tags

正式 route tags 统一为：

```text
index_symbol_hit
index_path_hit
index_graph_hit
knowledge_declared_hit
knowledge_derived_hit
governance_evidence_ref
governance_summary_hit
projection_ref
rendered_page_debug_fallback
```

禁止 Host / Skill 自造私有 route tag。

route tag 语义：

| route tag | 含义 |
| --- | --- |
| `index_symbol_hit` | 命中 symbol identity、definition、reference 或 signature |
| `index_path_hit` | 命中文件、目录、package、manifest 或 path pattern |
| `index_graph_hit` | 命中调用关系、依赖边、impact slice、process 或 community |
| `knowledge_declared_hit` | 命中 declared knowledge |
| `knowledge_derived_hit` | 命中 snapshot-scoped derived knowledge |
| `governance_evidence_ref` | 命中 `.spec` evidence artifact 或 change id |
| `governance_summary_hit` | 命中治理派生摘要 |
| `projection_ref` | 命中正式 Markdown projection 引用 |
| `rendered_page_debug_fallback` | 降级读取页面正文 |
| `legacy_page_debug_ref` | 显式 legacy / migration debug 模式命中旧 `.wiki/pages/**` |

### Readiness

query response 必须携带：

```text
index_readiness
knowledge_readiness
projection_readiness
governance_readiness
fusion_readiness
query_trust
recommended_action
```

每条 query result 必须携带：

```text
ref_kind
ref_id
label
score
provenance
confidence
recommended_action
source_refs
```

`query_trust` 不得只由是否有结果决定。任何 stale、conflict、fallback 或 missing graph 都必须降低 trust 或给出 action。

### Ranking / Grouping

默认 ranking：

```text
exact change-id / artifact ref
exact symbol
exact path
graph relation / impact
declared knowledge
derived knowledge
governance summary / evidence ref
projection ref
rendered page debug fallback
```

如果不同 route 的 score 不可比较，CLI 默认按 group 输出，不强行混排。

### 人类输出

默认人类输出必须翻译内部状态：

| 内部状态 | 用户文案 |
| --- | --- |
| `stale` | 需要刷新 |
| `conflict` | 事实或页面冲突，需要人工选择 |
| `illegal_drift` | 你改了自动生成区，系统不会采纳 |
| `projection_ref` | 已生成页面引用 |
| `rendered_page_debug_fallback` | 只命中过期或降级页面内容 |
| `governance_readiness: not_enabled` | 当前仓库未启用变更治理 |

每条高置信结果应回答：

```text
为什么命中
来自哪一层
是否过期
打开哪个文件验证
下一步动作
```

## 7. Governance Isolation Contract

### 隔离原则

`.spec` 是 governance evidence truth，不是 source facts。

```text
code graph files != governance evidence files
source_ref != evidence_ref
index_graph_hit != governance_evidence_ref
```

`.spec` 不进入 `scan_repo` 的业务源码 facts，不写入 code graph `files/symbols/edges`。

治理查询使用独立 artifact reference index。

### Governance Readiness

没有 `.spec` 或未启用治理时：

```text
governance_readiness = not_enabled
```

它不得阻断：

```text
spec-wiki init
spec-wiki status
spec-wiki query
spec-wiki update
```

它只能阻断：

```text
validate <change-id>
archive <change-id>
release gate
governance-specific status
```

### Evidence 到 Knowledge 的转换

```text
evidence summary
  -> .wiki/.knowledge/derived/governance/**
  -> snapshot scoped

stable governance rule
  -> .wiki/.knowledge/declared/governance/**
  -> requires explicit authoring / confirmation
```

review report、test report、archive report 的摘要不能静默升级为长期规则。

governance derived records 必须携带：

```text
record_kind: governance_summary | change_status | review_gate | release_gate | wiki_sync_issue
evidence_refs
change_refs
source_refs: empty unless explicitly linked to source files
truth_kind: governance_derived
```

普通 code knowledge query 不得把 governance derived summary 当成源码事实解释。

### Archive 事务边界

archive 事务只覆盖：

```text
.spec/changes/<change-id>
  -> .spec/archive/<date-change-id>
operation manifest
parent / child meta update if needed
```

Wiki 更新不是 archive 事务的一部分。archive 最多输出 wiki-sync issue / evidence refs。是否写入 `.wiki/.knowledge/**` 或重投影页面，必须由后续显式 `sync` / `update` 或用户确认触发。

## 8. CLI Product Contract

### 默认命令面

默认 help 和 Quick Start 只突出：

```text
spec-wiki init
spec-wiki status
spec-wiki query <term>
spec-wiki update
```

治理和维护命令通过 `status` 的 next action、场景页或 `--help --all` 暴露。

### 首次 init 输出

`init` 成功或部分成功后，必须输出固定 landing state：

```text
entry:
  .wiki/INDEX.md

health:
  index / knowledge / projection / governance

what is ready:
  可查询代码图 / 可查询知识 / 可阅读 Wiki / governance not_enabled

what is partial:
  哪些页面是骨架
  哪些能力降级

try next:
  spec-wiki status
  spec-wiki query "<suggested term>"
  spec-wiki update
```

### 安全模式

CLI 默认承诺：

```text
默认只读或安全写
破坏性动作必须 dry-run / manifest / confirm
repair 默认只输出 plan
archive 默认先 validate
失败必须给 recovery hint
```

破坏性动作清单：

```text
physical delete page / section
discard generated drift
archive apply
repair apply
legacy migration apply
metadata snapshot rollback
declared record lifecycle change to deprecated / replaced
```

上述动作默认不得静默执行。

### `validate` 与 `archive`

第一阶段不把 `archive` 放进默认 help。

`validate [change-id]` 可以保留，但输出第一行必须写明：

```text
mode: workspace
```

或：

```text
mode: change
change: <change-id>
```

如果用户测试仍混淆，后续拆成：

```text
spec-wiki validate
spec-wiki validate-change <change-id>
```

## 9. Failure Recovery Contract

### 用户级恢复剧本

| 问题 | 状态 | 默认动作 |
| --- | --- | --- |
| `.cache` 缺失 | `index_readiness: missing` | `spec-wiki rebuild` 或 `spec-wiki update` |
| metadata hash 不一致 | `projection_readiness: blocked` | `spec-wiki repair --plan` |
| managed marker 损坏 | `projection_readiness: conflict` | 修复 marker 或将内容移动到 manual section |
| derived section 被人工改动 | `illegal_drift` | discard / move to manual / convert to declared |
| partial write | `blocked` | 保留 previous committed snapshot，执行 `repair --plan` |
| archive 半失败 | `governance_readiness: blocked` | 根据 operation manifest resume / rollback |
| `.spec` 缺失 | `governance_readiness: not_enabled` | 不需要处理，core wiki 可继续用 |

### 冲突处理动作

冲突报告必须提供可选动作，而不是只报错：

```text
accept page edit as declared
discard page edit
move edit to manual section
re-render generated section
open evidence refs
repair marker
```

这些动作是否自动执行，由 `repair --apply` 或后续显式命令决定。

## 10. Implementation Order

新版实现顺序建议：

```text
1. Page Tree Contract
2. Truth Kind + Restore Contract
3. Projection Ownership + Store Trait Contract
4. Query Route + Readiness Contract
5. CLI Landing State + Human Output Contract
6. Governance Isolation + Read-only Status
7. Update/Delete/Snapshot Commit Contract
8. Archive Dry-run + Manifest
```

不要先实现 archive，也不要先扩完整 Agent 生成链。先把页面树、truth、restore 和 query readiness 钉住。

## 需要同步修正的草案

本文采纳后，应按以下方向修正现有草案：

| 文件 | 修正方向 |
| --- | --- |
| `specwiki-code-graph-index-design.md` | 已承接旧 graph-first 草案中的 code graph schema、phase DAG、raw captures、SymbolNode 和 upstream 借鉴 |
| `knowledge-to-wiki-projection-contract.md` | 投影目标从 `.wiki/pages/**` 改为 `.wiki/**/*.md` 的唯一页面树，并排除 `.knowledge/.cache` 等 runtime 目录 |
| `governance-runtime-integration.md` | 保留页面树决策，替换 `readable page truth` 为 `projection output / authoring surface` |
| `specwiki-cli-unification.md` | 默认 help 收窄到 4 个主命令，治理命令通过场景触发 |
| `.wiki/05-规格基线/**` | 后续通过 `.spec` change 同步正式 capability 口径 |

## 成功标准

- 设计草案不再同时把 `.wiki/pages/**` 和 `.wiki/**/*.md` 作为正式目标。
- `.cache` 缺失后的恢复不会伪装成 graph ready。
- query 输出能区分 index、knowledge、governance、projection 和 debug fallback。
- 没有 `.spec` 的仓库不会被治理状态阻断。
- `init` 后用户知道打开哪里、问什么、哪些结果可信、下一步做什么。
- archive 不和 Wiki 更新混成一个不可回滚事务。

## 结论

新版 SpecWiki 不应继续堆更多对象模型，而应先把合同闭合：

```text
一个页面树
一套 truth kind
两级恢复
明确投影归属
统一 query route
隔离 governance evidence
可解释的用户路径
```

这些合同成立后，再进入实现，才能避免 runtime 在代码里重新长出隐性兼容层。
