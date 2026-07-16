# declared-knowledge-lifecycle Specification

## Purpose

定义 declared knowledge 的正式 authoring、lifecycle、artifact 与 workflow 消费合同。Declared knowledge 是 `.wiki/.knowledge/declared/**` 中的长期 truth layer，不是 page projection、cache、临时正文或未结构化注释。

## Requirements

### Requirement: `declared knowledge` 必须成为正式可操作对象

系统 MUST 将 declared knowledge 收敛为正式可操作对象。每条 declared record MUST 至少包含稳定 `record_id`、`record_kind`、typed `scope`、`status`、`relations`、`source_ref`、`updated_at`、`unit_refs` 与 `projection_refs`。`declared knowledge` MUST 写入 `.wiki/.knowledge/declared/**`，并被视为可审计长期知识真相源，而不是 cache、page draft 或 projection 摘要。

#### Scenario: 正式 runtime 写出可审计 declared record
- **WHEN** 用户显式新增或更新一条项目规范、约定、避坑或决策，且该对象进入正式 knowledge runtime
- **THEN** 系统 MUST 在 `.wiki/.knowledge/declared/**` 写出带稳定 `record_id` 的 declared record
- **THEN** 该 record MUST 同时记录 typed `scope`、来源、状态、关系与关联的 `KnowledgeUnit`

#### Scenario: declared record 不得退化为页面正文真相
- **WHEN** 某条知识仅存在于页面正文、临时 cache 或未结构化段落中
- **THEN** 系统 MUST NOT 将其视为正式 declared knowledge 成功落库
- **THEN** 只有形成结构化 declared record 后，系统才可把它视为可审计长期知识

### Requirement: typed scope 必须具备稳定 canonical identity

系统 MUST 使用 typed `scope` object 描述 declared record 的作用范围，而不是开放字符串。Typed scope MUST 至少携带 `kind` 与稳定引用字段，并允许按 scope kind 携带受控 selector。系统 MUST 提供 canonical serialization，用于 `record_id`、snapshot diff、roundtrip、restore 和 conflict detection。

#### Scenario: scope 字段顺序不影响 identity
- **WHEN** 两条 declared records 使用语义相同但字段顺序不同的 typed scope
- **THEN** 系统 MUST 生成相同 canonical scope key
- **THEN** snapshot diff 不得因为字段顺序漂移而误报 lifecycle 变化

#### Scenario: 不合法 scope 被拒绝
- **WHEN** declared block 无法映射为 typed scope，或 scope 缺少必要稳定引用
- **THEN** 系统 MUST 将该输入标记为 `illegal_drift` 或等价 parse failure
- **THEN** 系统 MUST NOT 写出半结构化 declared record

### Requirement: lifecycle relations 必须显式且单义

系统 MUST 使用显式 `relations` 与 `status` 表达 declared lifecycle。最小 lifecycle MUST 支持 `deprecated`、`replaced_by`、`supersedes`，并能判断同一 kind + canonical scope 下是否存在唯一 authoritative head。关系 target MUST 指向存在的 declared record，且必须保持同 kind、同 canonical scope、无环。

#### Scenario: deprecated record 保持可审计状态
- **WHEN** managed declared block 声明某 record 为 `deprecated`
- **THEN** 系统 MUST 在 declared artifact 中保留该 record 的 lifecycle status
- **THEN** status/query 或等价诊断面 MUST 能说明该 record 不再是 active truth

#### Scenario: replaced_by 与 supersedes 形成单义替代链
- **WHEN** record A 声明 `replaced_by` record B，或 record B 声明 `supersedes` record A
- **THEN** 系统 MUST 在 declared artifact 中持久化该 relation
- **THEN** restore 后该 relation MUST 保持稳定
- **THEN** 若关系形成环、跨 kind、跨 scope 或指向缺失 target，系统 MUST 拒绝 snapshot

### Requirement: semantic lifecycle、authoring state 与 authority 必须分离

系统 MUST 分别表达 record 的 semantic status、`bound/missing/detached` authoring state，以及 group 的 unique/none/conflict authority decision。Authoring block 消失 MUST 保留 formal record；authority record missing MUST 触发治理复核，非 authority record 只有先进入非 active semantic status 后才可 detached。系统 MUST NOT 把 block 消失解释为物理 prune。

#### Scenario: authority authoring block 消失时保留审计记录
- **WHEN** 当前 authoritative record 的 managed block 从 authoring surface 消失
- **THEN** record MUST 保持在 formal artifact 中并进入 `missing`
- **THEN** recommended action MUST 指向 governance review，而不是静默删除

#### Scenario: deprecated record 才允许 detached
- **WHEN** 非 authority record 已显式 deprecated 且 authoring block 消失
- **THEN** 系统 MAY 将 authoring state 置为 `detached`
- **THEN** semantic record 与 lifecycle history MUST 继续保留

### Requirement: declared governance history 必须 append-only

系统 MUST 为 conflict opened/resolved/reopened、authoring missing/restored/detached 和 authority changed 生成稳定 append-only events。Current open conflict view MUST 与历史正交；no-op sync MUST NOT 重复事件，已解决历史 MUST NOT 降低无关 route trust。

### Requirement: `sync` 回写 declared 时必须受受管编辑面约束

系统 MUST 将 `sync` 对 declared knowledge 的回写限制在受管编辑面内。只有显式标记为可回写 declared 的 managed section 或等价结构化编辑面，才允许被解析为 declared record 更新。系统 MUST 区分 `declared_writeback`、`metadata_only` 与 `illegal_drift` 三类结果，并保留对应 reason。

#### Scenario: 受管 declared section 成功回写 record
- **WHEN** 用户编辑某个允许回写 declared 的 managed section，并执行 `spec-wiki sync`
- **THEN** 系统 MUST 将该编辑解析为结构化 declared record 变更
- **THEN** `sync` 结果 MUST 显式标记此次变更属于 `declared_writeback`

#### Scenario: 非法页面改动不得反向污染 declared truth
- **WHEN** 用户修改了 derived-only 段落、破坏 managed marker，或进行了无法映射为 declared record 的 projection-first 编辑
- **THEN** 系统 MUST 将该编辑标记为 `illegal_drift`
- **THEN** 系统 MUST NOT 直接把该改动回写为 declared knowledge

#### Scenario: 非知识改动只更新 metadata
- **WHEN** 用户编辑只影响标题、排序、section hash 或其它 projection metadata
- **THEN** 系统 MUST 返回 `metadata_only`
- **THEN** 系统 MUST NOT 伪装成 declared truth 已经更新

### Requirement: declared lifecycle 必须驱动 downstream diagnostics

系统 MUST 让 declared lifecycle 变化进入 update/status/query 可消费诊断。Declared record 变化 MUST 能标记 affected declared records、stale units、stale projections 和 health signals；即使源码 dirty set 为空，`update` 也必须能消费这些 declared-driven scopes。

#### Scenario: declared writeback 触发 derived 与 projection stale
- **WHEN** `sync` 写回或删除正式 declared record
- **THEN** 系统 MUST 输出 affected declared record ids
- **THEN** 后续 `status` / `update` MUST 能消费该 declared scope

#### Scenario: query 暴露 declared lifecycle provenance
- **WHEN** query 命中与 declared lifecycle 相关的知识
- **THEN** query MUST 保持 index/knowledge/page route 语义
- **THEN** query MUST 通过 provenance、trust 或 recommended action 表达 stale/degraded/declared lifecycle 诊断

### Requirement: declared artifact restore 不得从页面正文反推 truth

系统 MUST 在 restore/audit 时以 `.wiki/.knowledge/declared/**` 和 recovery manifest 为 declared truth。Page declared block 只能作为受管 authoring input；restore 不得在 artifact 缺失、stale 或冲突时从页面正文重新构造 declared truth。

#### Scenario: cache-less restore 使用 declared artifact
- **WHEN** `.wiki/.cache/**` 缺失，但 `.wiki/.knowledge/declared/**`、runtime artifacts、official page tree 和 metadata 一致
- **THEN** 系统 MUST 基于正式 artifacts 恢复本地 runtime
- **THEN** 系统 MUST NOT 重新从页面正文提炼 declared truth

#### Scenario: page 与 artifact 冲突时 artifact 不被页面覆盖
- **WHEN** 页面正文中的 declared block 与 `.wiki/.knowledge/declared/**` snapshot 或 metadata anchor 冲突
- **THEN** 系统 MUST 返回显式 stale/blocker 或拒绝 restore
- **THEN** 系统 MUST NOT 静默把页面正文覆盖为新的 declared truth
