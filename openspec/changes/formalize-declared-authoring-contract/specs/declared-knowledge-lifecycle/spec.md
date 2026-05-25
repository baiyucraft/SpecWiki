## MODIFIED Requirements

### Requirement: `declared knowledge` 必须成为最小正式可操作对象
系统 MUST 将 `declared knowledge` 收敛为正式可操作对象，而不是继续只保留目录占位或未来能力预留。每条 declared record MUST 至少包含稳定 `record_id`、`record_kind`、typed `scope`、`status`、`relations`、`source_ref`、`updated_at`、`unit_refs` 与 `projection_refs`。其中 typed `scope` MUST 是可持久化、可 roundtrip 的稳定 object，而不是松散字符串；`relations` MUST 至少支持 `supersedes`、`replaced_by` 与 `deprecated` 三类 declared lifecycle 关系。`declared knowledge` MUST 写入 `.wiki/.knowledge/declared/**`，并被视为可审计长期知识真相源，而不是 cache、page draft 或 projection 摘要。

#### Scenario: 正式 runtime 写出带 typed scope 的 declared record
- **WHEN** 用户显式新增或更新一条项目规范、约定、避坑或决策，且该对象进入正式 knowledge runtime
- **THEN** 系统 MUST 在 `.wiki/.knowledge/declared/**` 写出带稳定 `record_id` 与 typed `scope` 的 declared record
- **THEN** 该 record MUST 同时记录其作用范围、来源与关联的 `KnowledgeUnit`

#### Scenario: declared relation 被正式持久化
- **WHEN** 某条 declared record 表达“替代旧规则”或“已废弃”的生命周期关系
- **THEN** 对应 artifact MUST 显式写出 `supersedes`、`replaced_by` 或 `deprecated` 关系
- **THEN** 调用方 MUST 不需要反查页面正文才能恢复这些关系

#### Scenario: declared record 不得退化为页面正文真相
- **WHEN** 某条知识仅存在于页面正文、临时 cache 或未结构化段落中
- **THEN** 系统 MUST NOT 将其视为正式 declared knowledge 成功落库
- **THEN** 只有形成结构化 declared record 后，系统才可把它视为可审计长期知识

### Requirement: `sync` 回写 declared 时必须受受管编辑面约束
系统 MUST 将 `sync` 对 declared knowledge 的回写限制在受管编辑面内。只有显式标记为可回写 declared 的 managed section 或等价结构化编辑面，才允许被解析为 declared record 更新。`sync` 在成功回写 declared 时 MUST 能同时更新 declared record 的 typed `scope`、`status` 与 `relations`，并区分 `declared_writeback`、`metadata_only` 与 `illegal_drift` 三类结果，保留对应 reason 与受影响对象引用。

#### Scenario: 受管 declared section 成功回写 record
- **WHEN** 用户编辑某个允许回写 declared 的 managed section，并执行 `spec-wiki wiki sync`
- **THEN** 系统 MUST 将该编辑解析为结构化 declared record 变更
- **THEN** `sync` 结果 MUST 显式标记此次变更属于 `declared_writeback`

#### Scenario: 关系或 scope 编辑进入正式 writeback
- **WHEN** 用户在受管 declared 编辑面中修改某条记录的 scope 或替代/废弃关系
- **THEN** `sync` MUST 将这些变更写回 declared artifact，而不是只更新页面 metadata
- **THEN** 结果 MUST 返回受影响 `declared_record_ids` 或等价稳定引用

#### Scenario: 非法页面改动不得反向污染 declared truth
- **WHEN** 用户修改了 derived-only 段落、破坏 managed marker，或进行了无法映射为 declared record 的 projection-first 编辑
- **THEN** 系统 MUST 将该编辑标记为 `illegal_drift`
- **THEN** 系统 MUST NOT 直接把该改动回写为 declared knowledge

## ADDED Requirements

### Requirement: declared lifecycle 变更必须驱动下游知识失效传播
系统 MUST 将 declared lifecycle 变更视为正式 runtime 输入，而不是一次性 writeback 副作用。凡是 declared record 的 `scope`、`status` 或 `relations` 发生变化，系统 MUST 能稳定推导受影响 `KnowledgeUnit`、derived research 与 projection 的失效范围，并将其写入可被 `update`、`status` 或等价 workflow 消费的正式状态。

#### Scenario: declared 变更触发下游 stale
- **WHEN** 某条 declared record 的 scope、状态或替代关系发生变化
- **THEN** 系统 MUST 将对应受影响单元标记为 `research_stale`、`projection_stale` 或等价下游失效状态
- **THEN** 后续 `update` MUST 能消费该失效范围，而不依赖源码 dirty set

#### Scenario: deprecated declared 影响查询与诊断
- **WHEN** 某条 declared record 被标记为 `deprecated` 或被新规则 `replaced_by`
- **THEN** runtime MUST 能将该生命周期信息暴露给 `status`、`query` 或等价诊断接口
- **THEN** 调用方 MUST 不需要自行重建 declared lifecycle 状态机
