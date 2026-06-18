# refactor-specwiki-around-contract-closure-projection-writeback-boundaries 设计方案

## 方案概述

本方案把 projection/writeback 从“Markdown 页面生成流程”收口为一组可验收的跨 crate 合同：`wiki-model` 提供共享 DTO、ID、ownership 和状态枚举；`wiki-knowledge` 负责 projection intent、section contract、declared authoring 的语义校验与归一化；`wiki-runtime` 负责页面协议、parse/merge/render/write、metadata binding、snapshot commit 与 restore binding。

本 change 不做 query、code graph、CLI、governance、archive 或 `.wiki/pages/**` 兼容工作。实现必须以合同闭合为验收标准，而不是以代码移动量为验收标准。

核心路径固定为：

```text
Knowledge truth
  -> Projection intent
  -> Runtime page protocol
  -> Declared writeback validation
  -> Artifact / metadata / snapshot commit
```

## 架构分析

当前代码已经具备 projection/writeback 的雏形，但职责混在一起：

| 位置 | 当前职责 | 问题 | 设计落点 |
| --- | --- | --- | --- |
| `crates/wiki-knowledge/src/projection.rs` | `PlannedPage` 从 `KnowledgeTree` 生成页面规划 | 只到 page 粒度，缺少 section ownership / writeback policy | 收敛为 `PagePlan` / `SectionPlan` / `ProjectionIntent` 语义层 |
| `crates/wiki-knowledge/src/domain/research.rs` | `PlannedSection`、`PageDigest`、`ProjectionDigestStatus` | 持久化 digest 与 compose 摘要语义混杂 | 持久化 projection digest 上提到 `wiki-model`，compose 摘要留在 knowledge |
| `crates/wiki-knowledge/src/domain/compose.rs` | `PageDraft` / `ComposeSectionDraft` | `managed: bool` 过粗，容易被当成 formal truth | 保留为 transient compose output，不写 `.wiki/.knowledge/**` |
| `crates/wiki-runtime/src/generation/managed_sections.rs` | marker parse / render / merge | marker 只有 managed/user，且存在旧 heading fallback | 升级为 runtime page protocol kernel，删除兼容 fallback |
| `crates/wiki-runtime/src/workflows/sync.rs` | parse、declared materialize、drift、metadata、artifact commit | workflow 变成第二套 knowledge layer | 只编排 extract -> validate -> commit |
| `crates/wiki-runtime/src/storage/knowledge_artifacts.rs` | artifact persistence、restore、conflict/health 派生、projection digest binding | storage 解释太多语义 | 收窄为 artifact/snapshot storage adapter |
| `crates/wiki-runtime/src/domain/state.rs` / `wiki-model/src/domain/state.rs` | `WikiSectionState { managed: bool }` | 无法机器判定 ownership 与 drift 行为 | 替换为共享 `SectionOwnership` 与 section binding |

依赖方向保持：

```text
wiki-model
  <- wiki-index
  <- wiki-knowledge
  <- wiki-runtime
```

`wiki-runtime` 可以依赖并调用 `wiki-knowledge` 的校验能力；`wiki-knowledge` 不得依赖 runtime，也不得认识 marker 字符串、页面落盘路径、metadata 文件格式或 snapshot commit 细节。

## 功能设计

### 功能模块划分

| 模块 | 功能描述 | 优先级 | 对应 proposal |
| --- | --- | --- | --- |
| Shared projection model | 在 `wiki-model` 固化 ownership、binding、projection digest、sync result kind | P0 | 自动生成 section 必须有 ownership、binding、hash |
| Knowledge projection intent | 在 `wiki-knowledge` 输出 page/section plan 和 declared writeback 语义校验 | P0 | knowledge 层只拥有 planning、contract validation / normalization |
| Runtime page protocol | 在 `wiki-runtime::generation` 解析/渲染 section marker，分类 marker 与 hash drift | P0 | runtime 拥有 visible page parse、marker / drift 分类 |
| Sync orchestration | 拆分 `sync` 为 extract、validate、commit 三段 | P0 | declared writeback 闭合为 runtime parse/extract -> knowledge validate -> runtime commit |
| Metadata / snapshot binding | section 级 binding 写入 metadata，并接入既有 snapshot manifest | P0 | init/update/sync/restore 后绑定一致 |
| Duplicate model cleanup | 删除 runtime 侧重复 compose/research DTO 或改为复用 model/knowledge DTO | P1 | 双 page generation 通道不再产生行为分叉 |

### Section Ownership 行为

`SectionOwnership` 必须是机器可判定枚举，建议放入 `wiki-model`：

| owner kind | 更新行为 | 回写行为 | drift 行为 |
| --- | --- | --- | --- |
| `declared_managed` | runtime 可根据 declared record 重投影 | 合法 authoring 可写回 `.wiki/.knowledge/declared/**` | 字段、scope、lifecycle 冲突进入 `conflict` |
| `derived_managed` | runtime 可由 derived knowledge 刷新 | 不允许从 Markdown 回写 derived knowledge | 人工改动进入 `illegal_drift` 或 `stale` |
| `projection_static` | runtime 可重建静态投影片段 | 不允许回写 knowledge | 人工改动进入 `illegal_drift` |
| `manual_unmanaged` | runtime 不覆盖 | 不自动生成 declared record | 只允许 `metadata_only` |
| `external_ref` | runtime 维护引用、摘要和 hash | 不复制外部正文，不回写 knowledge | 引用失效进入 `stale` 或 `conflict` |

`managed: bool` 不再作为正式合同字段。短期实现可在内部临时桥接，但对外 state、metadata、marker parse 和测试断言必须转向 `SectionOwnership`。

### Runtime Page Protocol

`ManagedSectionBlock` 从 `section_id/title/version/body` 扩展为 section protocol block：

```text
section_id
owner_kind
marker_version
title
body
knowledge_refs
source_refs
input_hash
content_hash
generated_content_hash
projection_digest_ref
```

marker schema 必须包含 `id / owner / version`。缺失 `id`、缺失 `owner`、版本无法解析、start/end id 不匹配、owner 与 metadata 不一致，都不能 best-effort 修复，必须进入 `illegal_drift` 或 `conflict`。

`parse_wiki_page` 只解析正式页面树。旧 heading fallback、旧页面目录迁移、`.wiki/pages/**` 读取和诊断均不进入本 change。

### Declared Writeback

declared writeback 拆成三段：

```text
wiki-runtime
  parse official page tree
  extract declared authoring candidates
  compare marker / metadata / hash

wiki-knowledge
  validate declared authoring contract
  normalize scope / lifecycle / relation
  produce declared record patch or rejection

wiki-runtime
  persist declared artifacts
  update metadata binding
  mark impacted projection stale
  commit snapshot binding
```

knowledge 层只消费 runtime 提取后的结构化 candidate，不读取 Markdown，不解析 marker 字符串。

### Drift 与异常分类

`sync` 结果不能压平成 success/warning，必须固定分类：

| 分类 | 触发场景 | 后续行为 |
| --- | --- | --- |
| `declared_writeback` | 合法 `declared_managed` 修改通过校验 | 写 declared artifact，标记 impacted projection stale |
| `metadata_only` | `manual_unmanaged` 变化或只需刷新 page-local binding | 只更新 metadata，不改变 knowledge truth |
| `illegal_drift` | derived/static 被改、marker 缺失、marker malformed | 不写 knowledge，要求人工修复或显式 discard |
| `conflict` | declared scope/lifecycle/relation 与现有 truth 冲突，或 metadata binding mismatch | 阻断本 section/page 的 writeback，报告 evidence |
| `stale` | input/content hash 或 snapshot ref 已过期但可解释 | 等待 update/reproject |

page-level commit 建议保持原子：同一页面内任何 managed section 出现 `conflict` 或 `illegal_drift` 时，不提交该页面的 knowledge writeback；不受影响页面仍可继续处理。

## 数据设计

### 共享模型

`wiki-model` 新增或固化以下稳定对象：

| 对象 | 关键字段 | 说明 |
| --- | --- | --- |
| `SectionOwnership` | `declared_managed / derived_managed / projection_static / manual_unmanaged / external_ref` | 跨 metadata、state、parser、knowledge validation 共用 |
| `SectionBinding` | `section_id / owner_kind / knowledge_refs / source_refs / input_hash / content_hash / projection_status` | metadata section 级绑定最小合同 |
| `ProjectionBinding` | `projection_id / page_id / section_ids / knowledge_refs / snapshot_id / digest_ref` | page/section 与 projection digest 的恢复锚点 |
| `ProjectionDigest` | `projection_id / input_hash / renderer_version / section_hashes / staleness / status_reason` | 持久化 projection/runtime recovery anchor，放入 `wiki-model/src/domain/projection.rs` |
| `SyncResultKind` | `declared_writeback / metadata_only / illegal_drift / conflict / stale` | workflow 与测试共享状态分类 |
| `DeclaredAuthoringCandidate` | 见下节 | runtime extract 与 knowledge validate 的边界 DTO |
| `DeclaredRecordPatch` | `record_id / operation / normalized_record / affected_projection_refs` | knowledge 输出给 runtime 的提交输入 |

`PageDraft` 保留为 transient output：可以用于 diff、review 和 renderer 输入，但不得写入 `.wiki/.knowledge/**`，不得作为 restore anchor。若继续留在 `wiki-knowledge`，建议改名为 `ComposePageDraft` 并在类型注释中明确 transient。

`PageDigest` 拆分为两个语义：

```text
PageComposeDigest / ChildPageDigest
  留在 wiki-knowledge，表达 compose/research 摘要

ProjectionDigest
  放入 wiki-model/src/domain/projection.rs，作为持久化 projection/runtime recovery anchor
```

### Declared Authoring Candidate

runtime 提取的 candidate 最小字段：

```text
authoring_id
record_kind
scope_ref
lifecycle
source_ref
page_id
section_id
body
baseline_hash
current_hash
marker_version
metadata_binding_ref
relations
```

`baseline_hash` 可由 `generated_content_hash` 或 metadata 中的上次 committed section hash 承担。knowledge 校验只基于 DTO，不依赖 Markdown 原文位置。

### Metadata Binding

`wiki.metadata.json` 必须支持 section 级正向与反向索引：

```text
page -> sections -> knowledge_refs / source_refs
knowledge -> pages / sections
source -> pages / sections / knowledge
projection_digest -> pages / sections
```

section binding 最小字段固定为：

```text
section_id
owner_kind
knowledge_refs
source_refs
input_hash
content_hash
projection_status
projection_digest_ref
```

metadata 不承载 knowledge 正文，也不承载 projection readiness 的唯一主状态。projection digest 和 snapshot manifest 是 runtime recovery anchor，metadata 保存绑定索引和当前 snapshot 指针。

## 接口设计

### Knowledge-facing API

建议在 `wiki-knowledge` 提供明确的 runtime-facing API：

```text
plan_projection(tree, policy) -> Vec<PagePlan>
plan_sections(page_plan, knowledge_snapshot) -> Vec<SectionPlan>
validate_declared_writeback(candidate, snapshot) -> DeclaredWritebackDecision
```

`DeclaredWritebackDecision`：

```text
Accepted(DeclaredRecordPatch)
Rejected { reason, evidence_refs }
Conflict { reason, conflicting_record_refs, evidence_refs }
```

knowledge API 不接受 marker string、文件路径写入句柄、metadata patch 或 snapshot manifest writer。

### Runtime-facing API

建议在 `wiki-runtime::generation` 固化：

```text
parse_wiki_page(content, metadata_binding) -> ParsedWikiPage
classify_section_drift(parsed_section, metadata_binding) -> SyncResultKind
merge_sections(new_sections, old_page, metadata_binding) -> PageMergePlan
render_page_with_markers(page_projection) -> String
```

`ParsedWikiPage` 必须暴露 parse diagnostics，而不是只返回 warnings。diagnostic 至少区分：

```text
marker_missing
marker_malformed
marker_version_unsupported
marker_end_mismatch
metadata_binding_mismatch
hash_mismatch
```

### Storage / Commit API

`knowledge_artifacts` 收窄为 storage adapter：

```text
commit_knowledge_snapshot(input: KnowledgeSnapshotCommitInput) -> CommittedSnapshotManifest
load_committed_snapshot(snapshot_id) -> KnowledgeArtifacts
restore_runtime_binding(snapshot_id) -> RuntimeRestoreBinding
```

storage adapter 不再临时派生 declared lifecycle、projection health 或 conflict 语义；这些必须在 `wiki-knowledge` 或 runtime workflow 的上游阶段完成。

本 child 只要求 projection/writeback binding 接入现有 snapshot manifest 校验，不重做完整 snapshot commit protocol。

## 非功能性设计

### 可维护性

- 以 DTO 和状态分类作为边界，避免靠 trait 包一层 runtime storage 来伪装解耦。
- 删除 `wiki-runtime/src/domain/research.rs` 与 `wiki-runtime/src/domain/compose.rs` 中和 `wiki-knowledge` 重复的模型，runtime 直接消费 `wiki-model` / `wiki-knowledge` 的正式类型。
- `sync.rs` 不再直接 materialize `DeclaredKnowledgeRecord`；它只提取 candidate、调用 knowledge validate、提交 patch。
- `managed_sections` 不解释 declared lifecycle，只负责页面协议和 hash/drift 信号。

### 可靠性

- `.wiki/.knowledge/**`、正式页面树、`wiki.metadata.json`、snapshot manifest 的提交必须共享同一个 commit boundary。
- 任一 artifact 或 metadata 写入失败时，不得留下 metadata 指向不存在 page、section 或 knowledge artifact。
- restore 后必须校验 manifest -> metadata -> page hash -> section binding -> knowledge refs 的闭环；失败时进入 blocked/conflict，而不是伪装 ready。

### 兼容性

当前阶段不考虑旧版本兼容：

- 不迁移 `.wiki/pages/**`。
- 不读取 `.wiki/pages/**`。
- 不保留 legacy heading parse fallback。
- 不新增旧页面诊断字段。
- 不为旧 `managed: bool` 暴露兼容 DTO。

## 资源评估

无新增外部服务、网络资源或额外运行时依赖。资源增量主要来自 metadata section binding、projection digest 和 sync diagnostics 的 JSON/YAML 体积增长，预计与页面数和 section 数线性相关。

本 change 不引入新的 SQLite graph schema，也不改变 code graph 构建成本。

## 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| 实现退化为搬代码，边界仍不清 | 高 | 先实现共享 DTO、状态分类和测试 gate，再拆 workflow |
| `PageDraft / PageDigest` 语义继续混杂 | 高 | 拆分 transient compose output 与 persisted projection digest |
| `sync.rs` 拆分后提交语义不原子 | 高 | 以 snapshot manifest 和 metadata binding roundtrip 做验收 |
| marker 升级影响大量旧测试 | 中 | 当前不保兼容，测试同步改为 ownership/hash/binding 断言 |
| metadata 反向索引遗漏导致 query/restore 误判 | 中 | 增加 page -> section -> knowledge 和 knowledge -> section 双向校验 |
| derived drift 被误写回 `.wiki/.knowledge/derived/**` | 高 | 将 derived/static drift 测试列为 P0 gate |

## 设计决策

- `SectionOwnership` 放入 `wiki-model`，不继续使用 `managed: bool` 表达正式 section 状态。
- `PageDraft` 只作为 transient compose/render 输入，不作为 truth、metadata 或 restore anchor。
- 持久化 `ProjectionDigest` 上提到 `wiki-model`；compose 子页摘要留在 `wiki-knowledge` 并改名区分。
- `ProjectionDigest` 放入 `wiki-model/src/domain/projection.rs`，不继续塞进 `knowledge_artifact.rs`。
- `PagePlan / SectionPlan` 作为新正式命名，一次性替换现有 `PlannedPage / PlannedSection`，不保留兼容别名。
- declared writeback 的唯一合法路径是 runtime extract -> knowledge validate/normalize -> runtime persist/metadata/stale commit。
- `wiki-knowledge` 不解析 marker、不写 Markdown、不生成 metadata patch。
- `wiki-runtime` 不重新决定 knowledge scope，不从 Markdown 正文反推 derived knowledge。
- 删除旧 heading fallback 和 `.wiki/pages/**` 相关读取/迁移/诊断逻辑。
- `knowledge_artifacts` 只承担 storage/snapshot adapter，不派生 declared lifecycle、conflict 或 projection health。

## 待确认问题

无。以下设计点已确认：`ProjectionDigest` 放入 `wiki-model/src/domain/projection.rs`；本 child 只接入现有 snapshot manifest 校验，不重做完整 commit protocol；`PagePlan / SectionPlan` 一次性替换 `PlannedPage / PlannedSection`，不保留历史兼容名。

## 参考资料

- `proposal.md`
- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.wiki/06-设计文档/01-Runtime设计.md`
