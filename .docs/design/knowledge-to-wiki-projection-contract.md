---
title: Knowledge 到 Wiki 投影规范
description: 定义 .wiki/.knowledge 如何映射到 .wiki Markdown 页面、metadata 绑定、更新与回写边界
updated: 2026-06-15
owner: architecture
status: draft
---

# Knowledge 到 Wiki 投影规范

## 文档定位

本文是阶段性设计稿，用于把 `knowledge -> .wiki` 的映射规则从口头模型固化为实现合同。

它补足三类问题：

- `.wiki/.knowledge/**` 中的结构化知识如何投影为 `.wiki/pages/**/*.md`。
- `.wiki/pages/**/*.md` 的人工编辑如何被允许或拒绝回写到 declared knowledge。
- `wiki.metadata.json` 如何记录 knowledge、page、section、source 和 runtime 状态之间的绑定。

本文不改变目录归属：`.wiki/.knowledge/**` 是 knowledge truth，`.wiki/pages/**` 是 runtime page projection，`.wiki/wiki.metadata.json` 是绑定索引，`.wiki/.cache/**` 是可重建本地 cache。`.wiki/` 下的长期手写文档默认不是 runtime 自动投影目标，只有显式声明受管 section 时才参与同步或投影。

## 核心原则

### 1. Knowledge 是主输入，Markdown 是投影结果

正式链路固定为：

```text
index graph
  -> knowledge
  -> projection plan
  -> page draft
  -> .wiki/pages/**/*.md
  -> wiki.metadata.json
```

`.wiki/pages/**/*.md` 不得反向成为 derived knowledge 的主真相。

### 2. 映射必须显式

任何被自动写入或自动更新的页面内容，都必须能追溯到：

- knowledge ref
- source ref
- projection policy
- section identity
- renderer version

不能依靠“文件名像某个模块”或“标题文本相似”来长期维持映射。

### 3. 页面只允许局部受管更新

runtime 不得盲写整页。页面内 section 必须有 ownership：

```text
declared_managed
derived_managed
projection_static
manual_unmanaged
external_ref
```

不同 ownership 的更新和回写规则不同。

### 4. 人工编辑只通过合法 authoring surface 回写

人可以编辑 `.wiki`，但只有合法的 `declared_managed` section 或等价结构化 authoring block 能回写到 `.wiki/.knowledge/declared/**`。

其它页面改动只能产生 metadata-only、stale、conflict 或 illegal drift，不得污染 declared / derived truth。

### 5. Update 先算 knowledge scope，再刷新页面

`update` 必须遵循：

```text
ChangeSet
  -> AffectedKnowledgeScope
  -> AffectedProjectionScope
  -> refresh knowledge
  -> reproject impacted sections/pages
```

不得先比较最终 Markdown 差异，再反推 knowledge scope。

## 分层对象

## Object Ownership Matrix

| 对象 | 层级 | 是否 truth | 是否可上库 | 主要职责 | 不得承担 |
| --- | --- | --- | --- | --- | --- |
| `KnowledgeUnit` | knowledge | 是 | 是 | 稳定知识身份、scope、source 和 projection refs | 页面布局、Markdown 正文 |
| `DeclaredRecord` | knowledge | 是 | 是 | 人声明的规则、决策、约定、避坑 | 派生事实、页面摘要缓存 |
| `DerivedRecord` | knowledge | 是，本次 snapshot 输入 | 是 | 从 facts/research/compose 派生的知识 | 被页面人工编辑直接改写 |
| `ProjectionDigest` | runtime knowledge | 是，projection anchor | 是 | projection snapshot、readiness/status/reason | 正文垃圾桶、完整页面内容 |
| `PagePlan` | projection planning | 否 | 可选 | 规划页面目标、scope、section 组成 | knowledge truth |
| `SectionPlan` | projection planning | 否 | 可选 | 规划 section owner、refs、hash、位置 | 自然语言 ownership 推断 |
| `PageDraft` | transient work state | 否 | 否 | 渲染前中间态、diff/review 输入 | formal truth、恢复锚点 |
| `PageProjection` | projection output | 是，落盘绑定 | 是 | 已落盘页面、section hash、metadata patch | knowledge truth |
| `wiki.metadata.json` | binding index | 是，绑定索引 | 是 | page/section/knowledge/source/reverse refs | 全量正文、知识正文 |

硬约束：

- `PageDraft` 不得写入 `.wiki/.knowledge/**`。
- Markdown page 不得反向成为 `DerivedRecord` 的来源。
- 页面只能通过 declared owner section 回写 `DeclaredRecord`。
- `ProjectionDigest` 承接 projection readiness/status/reason，避免 `PageProjection`、metadata 和 digest 各自维护一套状态。

### Knowledge Objects

#### `KnowledgeUnit`

从 index graph、research 和 planning 得到的正式知识单元。

必需字段：

```text
unit_id
unit_kind
title
scope_ref
source_refs
status
updated_at
summary_ref
projection_refs
```

语义：

- `unit_id` 稳定，不随页面路径变化。
- `scope_ref` 指向 typed scope，例如 repo、module、path、symbol、config、workflow。
- `projection_refs` 指向该 unit 当前投影到的页面和 section。

#### `DeclaredRecord`

人明确沉淀的规则、决策、约定、避坑和流程知识。

必需字段：

```text
record_id
record_kind
authoring_id
scope_ref
status
source_ref
updated_at
unit_refs
projection_refs
relations
```

语义：

- 正式 truth 位于 `.wiki/.knowledge/declared/**`。
- 页面 declared block 只是 authoring surface。
- lifecycle 关系至少支持 `deprecated / replaced_by / supersedes`。

#### `DerivedRecord`

系统从 facts、graph、research 或 compose 中提炼的派生知识。

必需字段：

```text
record_id
record_kind
unit_id
source_refs
input_hash
generator
status
summary
projection_refs
```

语义：

- 可重建，但一旦写入 `.wiki/.knowledge/derived/**`，就是本次 projection 的正式输入。
- 不能由页面人工编辑直接生成。

#### `ProjectionDigest`

knowledge 到页面投影的稳定摘要。

必需字段：

```text
projection_id
unit_refs
record_refs
target_page_id
target_section_ids
input_hash
renderer_version
content_digest
staleness
```

语义：

- 它是 `.wiki/.knowledge/runtime/**` 中的 projection / recovery anchor。
- Markdown 文件是 projection 落盘结果，不替代 projection digest。

## 页面对象

### `PagePlan`

决定哪些 knowledge 应该进入哪一页。

必需字段：

```text
page_id
path
title
page_kind
scope_ref
knowledge_refs
section_plans
projection_policy
```

页面路由规则：

| Knowledge 类型 | 默认目标 |
| --- | --- |
| repo overview / architecture | `.wiki/pages/overview/**` |
| module unit | `.wiki/pages/modules/**` |
| public CLI / config / protocol | `.wiki/pages/public-contracts/**` |
| convention / policy / workflow rule | `.wiki/pages/conventions/**` |
| testing / scripts / acceptance | `.wiki/pages/development/**` |
| governance summary / wiki-sync issue | 只投影摘要和引用，不复制 `.spec` 原文 |

`.wiki/INDEX.md`、`.wiki/00-文档约定/**`、`.wiki/02-开发指南/**`、`.wiki/03-模块指南/**`、`.wiki/04-对外方法/**`、`.wiki/06-设计文档/**` 等长期手写 Wiki 页面是项目知识入口，不是默认 runtime 自动投影目标。后续如果需要让其中某个 section 受 runtime 管理，必须显式声明 section owner 与 metadata binding。

路由不得只靠标题。必须至少消费：

- `unit_kind / record_kind`
- `scope_ref`
- `source_refs`
- 当前 page tree policy
- 已存在 projection_refs

### `SectionPlan`

决定页面内 section 如何生成、更新和回写。

必需字段：

```text
section_id
heading
ownership
knowledge_refs
source_refs
position_policy
update_policy
writeback_policy
```

`section_id` 必须稳定。标题变化不得导致 section identity 丢失。

### `PageDraft`

渲染前的页面中间对象。

必需字段：

```text
page_id
path
frontmatter
sections
source_refs
knowledge_refs
projection_digest_ref
```

`PageDraft` 可以被测试、diff 和 review，但不是最终 truth。

### `PageProjection`

最终落盘对象。

必需字段：

```text
page_id
path
content_hash
section_hashes
projection_digest_ref
metadata_patch
```

`PageProjection` 写入 `.wiki/pages/**/*.md`，并同步更新 `wiki.metadata.json`。

## Section Ownership

section ownership 必须机器可判定。标准 owner kind 为：

```text
declared_managed
derived_managed
projection_static
manual_unmanaged
external_ref
```

禁止：

- 依赖标题、顺序或自然语言猜测 ownership。
- 一个 section 同时归 declared 与 derived。
- derived section 被 sync-pages 回写。
- marker 缺失时继续猜测 section 边界。

### `declared_managed`

人可编辑，并可通过 `sync-pages` 回写 declared knowledge。

规则：

- 必须有 `authoring_id`。
- 必须绑定目标 `record_kind` 和 `scope_ref`。
- sync-pages 成功后写入 `.wiki/.knowledge/declared/**`。
- 非法字段、缺失 id、scope 冲突或 lifecycle 关系不合法时，不得回写。

### `derived_managed`

runtime 可自动更新。

规则：

- 内容只能来自 knowledge refs 和 source refs。
- 人工编辑后，下次 `sync-pages` 应标记为 `illegal_drift` 或 `metadata_only`，不得直接回写 knowledge。
- update 可以覆盖该 section，但必须保留稳定 section id。
- 不允许 sync-pages 将其写回 derived knowledge。

### `manual_unmanaged`

人维护的自由说明。

规则：

- runtime 不覆盖。
- 可以被 metadata 记录为 page local context。
- 不进入 declared / derived truth，除非用户显式转换为 `declared_managed` block。

### `projection_static`

由 runtime 生成但不允许人工编辑。

规则：

- 常用于 citation、evidence summary、自动生成目录。
- 人工改动视为 drift。

### `external_ref`

指向外部文件或系统的引用。

规则：

- runtime 只维护链接和摘要。
- 不复制外部正文。

## Markdown 标记要求

每个受管 section 必须有稳定边界标记或等价结构化 metadata。

建议形式：

```markdown
<!-- spec-wiki:section id="cli-public-surface" owner="derived_managed" -->
## 一级命令

...
<!-- spec-wiki:section:end -->
```

declared managed 示例：

```markdown
<!-- spec-wiki:section id="comment-policy" owner="declared_managed" record-kind="policy" scope="repo" -->
## 注释规范

...
<!-- spec-wiki:section:end -->
```

约束：

- marker 缺失时，runtime 不得猜测覆盖范围。
- marker 被破坏时，sync-pages 必须输出 `illegal_drift`。
- marker 内的 `id`、`owner`、`record-kind`、`scope` 必须进入 metadata。

## Metadata Contract

`wiki.metadata.json` 是 page / knowledge / section / source 的绑定索引。

最小结构方向：

```json
{
  "version": 1,
  "pages": [
    {
      "page_id": "page:cli",
      "path": ".wiki/04-对外方法/00-CLI.md",
      "title": "CLI",
      "page_kind": "public_contract",
      "scope_ref": "repo",
      "knowledge_refs": [
        "declared:cli-public-surface",
        "derived:package-spec-wiki-cli"
      ],
      "source_refs": [
        "packages/spec-wiki/src/cli.ts",
        "packages/spec-wiki/src/cli.test.ts"
      ],
      "sections": [
        {
          "section_id": "cli-public-surface",
          "heading": "一级命令",
          "owner_kind": "derived_managed",
          "knowledge_refs": ["declared:cli-public-surface"],
          "source_refs": ["packages/spec-wiki/src/cli.ts"],
          "input_hash": "sha256:...",
          "content_hash": "sha256:..."
        }
      ],
      "projection_digest_ref": "projection:cli",
      "content_hash": "sha256:...",
      "status": "ready",
      "staleness": "fresh"
    }
  ]
}
```

metadata 还必须支持反向索引：

```json
{
  "reverse_refs": {
    "knowledge_to_pages": {
      "declared:cli-public-surface": ["page:cli"]
    },
    "knowledge_to_sections": {
      "declared:cli-public-surface": ["section:cli-public-surface"]
    },
    "source_to_knowledge": {
      "packages/spec-wiki/src/cli.ts": ["derived:package-spec-wiki-cli"]
    }
  }
}
```

规则：

- metadata 不承载全部正文。
- metadata 不替代 `.wiki/.knowledge/**`。
- metadata 必须能回答：哪个 knowledge 影响哪些 page / section。
- metadata 必须能回答：某个 page / section 当前是否 stale、conflict 或 illegal drift。
- metadata 必须记录 `knowledge_snapshot_id` 或等价 snapshot identity。
- section 级绑定至少包含 `section_id / owner_kind / source_refs / input_hash / content_hash / projection_status`。
- metadata hash 与实际文件不一致时，status 必须降级。

## Update 流程

`update` 输入是 repo change，而不是 Markdown diff。

```text
detect source/docs/spec changes
  -> build ChangeSet
  -> map to AffectedKnowledgeScope
  -> refresh index graph if needed
  -> refresh declared / derived / runtime knowledge
  -> compute AffectedProjectionScope
  -> re-render impacted sections/pages
  -> update wiki.metadata.json
  -> report stale/conflict/wiki-sync issues
```

要求：

- 未受影响的 knowledge、section 和 page identity 必须保持稳定。
- update 不得覆盖 `user` section。
- update 只能覆盖 `derived_managed` 或 `projection_static` section。
- 对 `declared_managed` section，update 只能重投影由 declared record 产生的内容，不得覆盖未同步的人为改动。
- `.spec` 变化只进入 governance summary / wiki-sync issue / projection refs，不复制 proposal、design、review 原文。

## Sync Pages 流程

`sync-pages` 是高级维护动作，用于把合法页面编辑回写到 knowledge。

```text
read .wiki/pages/**/*.md
  -> compare metadata hashes
  -> locate changed sections
  -> classify section ownership
  -> declared_managed -> validate authoring block
  -> write .wiki/.knowledge/declared/**
  -> derived_managed/projection_static drift -> illegal_drift or metadata_only
  -> manual_unmanaged section change -> metadata_only
  -> mark affected knowledge/page stale
```

结果分类：

| 结果 | 含义 |
| --- | --- |
| `declared_writeback` | 合法 declared managed 变更已写回 declared knowledge |
| `metadata_only` | 只更新 metadata，不改变 knowledge truth |
| `illegal_drift` | 页面改动破坏受管边界或试图改写 derived/managed truth |
| `conflict` | 页面声明与现有 declared record、scope 或 lifecycle 关系冲突 |
| `stale` | 页面或 section 需要后续 update 重投影 |

硬约束：

- sync-pages 不得把 Markdown 正文直接写为 derived knowledge。
- sync-pages 不得从普通 `manual_unmanaged` section 自动生成 declared record。
- sync-pages 不得在 marker 缺失或无法验证 scope 时猜测回写。

## 删除与回收语义

删除不是一种统一事件。必须按对象类型区分：

| 事件 | 语义 | 处理 |
| --- | --- | --- |
| `KnowledgeUnit removed` | scope 对应的知识单元不再成立 | 标记 obsolete，回收 projection refs，必要时删除或降级页面 |
| `DeclaredRecord deprecated` | 人声明知识仍保留历史，但不再生效 | 更新 lifecycle relation，重投影摘要 |
| `DeclaredRecord replaced` | 旧规则被新规则替代 | 设置 `replaced_by / supersedes`，不得物理丢失审计链 |
| `DerivedRecord obsolete` | 派生输入失效 | 失效 derived record 和 projection digest，可由 update 重建 |
| `Page removed` | 页面不再有长期阅读价值或对应 knowledge 全部移除 | 必须检查 inbound refs 和 metadata reverse refs |
| `Section removed` | 页面内某 section 不再有投影来源 | 删除 section 前必须更新 metadata 和 projection digest |

硬约束：

- 不得因为本轮扫描暂时缺失就立即删除长期 declared record。
- 不得保留已无 projection source 的页面来伪装 update 成功。
- 删除 page / section 必须同步更新 metadata reverse refs。

## Governance 投影边界

`.spec` 是 change evidence truth，不能被 `.wiki` 吞并。

允许进入 `.wiki` 的内容：

- change 状态摘要。
- wiki-sync issue。
- release / quality gate 摘要。
- 指向 `.spec/changes/**` 或 `.spec/archive/**` 的 evidence refs。
- 已确认长期稳定的结论，经 declared knowledge 或 derived governance summary 投影。

禁止进入 `.wiki` 的内容：

- proposal 原文复制。
- design 原文复制。
- review-report 原文复制。
- test-report 原文复制。
- archive operation manifest 原文复制。

archive 后如需沉淀长期知识，流程是：

```text
archive
  -> produce wiki-sync issue
  -> user confirm stable conclusion
  -> write declared / derived governance knowledge
  -> update projection
```

## Conflict 与 Drift

### Stale

表示 knowledge 或 projection 已过期，但仍可解释。

例子：

- 源码变更影响某个 `KnowledgeUnit`。
- declared record 更新后页面未重投影。

### Conflict

表示两个 truth source 无法同时成立。

例子：

- declared policy 声明某 CLI 已废弃，但当前代码仍暴露该命令。
- 页面 `declared_managed` block 修改了 scope，但已有 record lifecycle 关系不允许。

### Illegal Drift

表示页面改动试图破坏 projection 合同。

例子：

- 人工修改 `derived_managed` section。
- 删除 section marker。
- 在 `projection_static` 区写入新事实。

处理原则：

- stale 可由 update 修复。
- conflict 必须报告 evidence 和建议动作，不能静默选边。
- illegal drift 不得回写 knowledge，必须要求人工修复或显式 discard。

## Query 消费规则

query 应优先消费 knowledge 与 index，而不是 Markdown 全文。

查询来源排序：

```text
index graph hit
  > declared knowledge hit
  > derived knowledge hit
  > projection digest / page ref
  > rendered page debug fallback
```

返回结果必须携带来源：

```text
ref_kind
ref_id
page_ref
section_ref
knowledge_ref
source_ref
confidence
staleness
```

如果只命中 Markdown fallback，结果必须显式降低 trust。

## Implementation Notes

建议新增或固化的内部对象：

```text
KnowledgeUnit
DeclaredRecord
DerivedRecord
ProjectionDigest
PagePlan
SectionPlan
PageDraft
PageProjection
ProjectionBinding
SectionOwnership
AffectedKnowledgeScope
AffectedProjectionScope
SyncPagesResult
DriftIssue
ConflictIssue
```

建议新增 gate：

```text
projection metadata roundtrip
derived_managed section drift detection
declared_managed writeback
manual_unmanaged section metadata-only update
affected knowledge scope to impacted page mapping
metadata stale/conflict reporting
```

## 成功标准

- 每个自动更新 section 都能追踪到 knowledge refs 和 source refs。
- 每个 page 都能通过 metadata 找到对应 knowledge 和 projection digest。
- `update` 可以只刷新受影响 knowledge scope 对应的页面和 section。
- `sync-pages` 只把合法 `declared_managed` block 写回 declared knowledge。
- `manual_unmanaged` section 永不被 runtime 覆盖。
- `derived_managed` 和 `projection_static` section 漂移不会污染 declared / derived knowledge。
- `.spec` 原文不会被复制进 `.wiki` 页面。
- `.wiki/.cache/**` 缺失时，可从 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复 runtime 可消费状态。

## Validation Gates

### Persist Gate

- 写入 `.wiki/.knowledge/**`、页面和 metadata 必须处于同一 snapshot。
- 任一写入失败时不得留下 metadata 指向不存在 page / section / knowledge。

### Restore Gate

- 删除 `.wiki/.cache/**` 后，系统必须能从 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复可消费 runtime。
- 恢复后若当前源码与 snapshot 不一致，状态必须是 `stale` 或 `needs_update`，不得伪装为完整 ready。

### Sync Gate

- 合法 declared owner section 必须能回写 declared record。
- derived owner section 被改必须产生 `illegal_drift` 或 conflict，不得写回 derived。
- marker 被破坏必须可定位到 page / section。

### Update Gate

- `AffectedKnowledgeScope` 必须先于 `AffectedProjectionScope`。
- 受影响范围之外的 page / section identity 必须保持稳定。
- stale、conflict、illegal drift 不得被压平为普通 success。

### Release Gate

- public docs、metadata、projection digest 和 `.wiki/.knowledge/**` 的绑定必须一致。
- README、help、Agent assets 中提到的 Wiki 入口必须能映射到当前 projection contract。

## 结论

`knowledge -> .wiki` 的正确模型不是“生成 Markdown 文件”，而是：

```text
Knowledge truth
  -> Projection contract
  -> Section-level rendering
  -> Metadata binding
  -> Controlled authoring feedback
```

这样 `.wiki` 才能同时服务人读、Agent 查询、长期维护和可审计恢复，而不会重新退化为 page-first 文档生成器。
