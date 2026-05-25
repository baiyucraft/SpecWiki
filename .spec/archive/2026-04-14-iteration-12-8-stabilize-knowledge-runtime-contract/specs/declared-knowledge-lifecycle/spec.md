## ADDED Requirements

### Requirement: `declared knowledge` 必须成为最小正式可操作对象
系统 MUST 将 `declared knowledge` 收敛为正式可操作对象，而不是继续只保留目录占位或未来能力预留。每条 declared record MUST 至少包含稳定 `record_id`、`record_kind`、`scope_ref`、`status`、`source_ref`、`updated_at`、`unit_refs` 与 `projection_refs`。`declared knowledge` MUST 写入 `.wiki/.knowledge/declared/**`，并被视为可审计长期知识真相源，而不是 cache、page draft 或 projection 摘要。

#### Scenario: 正式 runtime 写出可审计 declared record
- **WHEN** 用户显式新增或更新一条项目规范、约定、避坑或决策，且该对象进入正式 knowledge runtime
- **THEN** 系统 MUST 在 `.wiki/.knowledge/declared/**` 写出带稳定 `record_id` 的 declared record
- **THEN** 该 record MUST 同时记录其作用范围、来源与关联的 `KnowledgeUnit`

#### Scenario: declared record 不得退化为页面正文真相
- **WHEN** 某条知识仅存在于页面正文、临时 cache 或未结构化段落中
- **THEN** 系统 MUST NOT 将其视为正式 declared knowledge 成功落库
- **THEN** 只有形成结构化 declared record 后，系统才可把它视为可审计长期知识

### Requirement: `sync` 回写 declared 时必须受受管编辑面约束
系统 MUST 将 `sync` 对 declared knowledge 的回写限制在受管编辑面内。只有显式标记为可回写 declared 的 managed section 或等价结构化编辑面，才允许被解析为 declared record 更新。系统 MUST 区分 `declared_writeback`、`metadata_only` 与 `illegal_drift` 三类结果，并保留对应 reason。

#### Scenario: 受管 declared section 成功回写 record
- **WHEN** 用户编辑某个允许回写 declared 的 managed section，并执行 `spec-wiki wiki sync`
- **THEN** 系统 MUST 将该编辑解析为结构化 declared record 变更
- **THEN** `sync` 结果 MUST 显式标记此次变更属于 `declared_writeback`

#### Scenario: 非法页面改动不得反向污染 declared truth
- **WHEN** 用户修改了 derived-only 段落、破坏 managed marker，或进行了无法映射为 declared record 的 projection-first 编辑
- **THEN** 系统 MUST 将该编辑标记为 `illegal_drift`
- **THEN** 系统 MUST NOT 直接把该改动回写为 declared knowledge
