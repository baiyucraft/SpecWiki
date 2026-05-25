## ADDED Requirements

### Requirement: deterministic declared conflicts 必须成为正式治理 artifact
系统 MUST 将 deterministic declared conflicts 收敛为正式 `KnowledgeConflictRecord`，而不是继续只把它们留在日志、warning 或临时 status 推断里。`KnowledgeConflictRecord` MUST 至少包含稳定 `conflict_id`、`conflict_kind`、`status`、`severity`、`scope_ref`、`record_ids`、`authoring_ids`、`unit_refs`、`projection_refs`、`reason` 与 `detected_at`。第一批 `conflict_kind` MUST 至少覆盖 `parallel_active_declared` 与 `lifecycle_head_ambiguity`。第一批 `status` MUST 固定为 `open`。

#### Scenario: 同 scope 多 active record 生成正式 conflict
- **WHEN** 同一 `record_kind + canonical_scope` 下存在多条 `active` declared records，且 formal lifecycle 关系无法推出唯一 authoritative head
- **THEN** 系统 MUST 生成 `parallel_active_declared` conflict artifact
- **THEN** 该 artifact MUST 显式引用冲突的 `record_ids` 与 `authoring_ids`

#### Scenario: lifecycle 头节点歧义生成正式 conflict
- **WHEN** 一组 declared records 单条都合法，但整体图上存在多个当前有效头节点
- **THEN** 系统 MUST 生成 `lifecycle_head_ambiguity` conflict artifact
- **THEN** 系统 MUST NOT 仅把该情况降级为普通 warning

### Requirement: conflict artifact 必须只从 formal declared snapshot 推导
系统 MUST 只基于 formal declared snapshot 推导 deterministic conflicts，而不是从页面正文、临时 cache、自由文本或 host 侧猜测反推。conflict artifact MUST 被视为治理诊断对象，而不是新的 declared truth source。

#### Scenario: 页面正文差异不得直接生成 conflict artifact
- **WHEN** 页面正文发生变化，但未形成合法提交的 declared snapshot
- **THEN** 系统 MUST NOT 直接生成或刷新 conflict artifact
- **THEN** 系统 MUST 先回到 declared snapshot 的合法性判断
