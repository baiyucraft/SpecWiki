## ADDED Requirements

### Requirement: Governance resolution MUST be modeled as a companion artifact linked to open conflict records
系统 MUST 保持 `KnowledgeConflictRecord` 为 open-only deterministic conflict artifact，并新增独立的 governance resolution companion artifact 通过 `conflict_id` 关联。系统 MUST NOT 将既有 open conflict object 原地扩展成完整治理状态机。

#### Scenario: runtime 为已检测的 conflict 写入治理决议
- **WHEN** 某个 `open` conflict 进入治理决议流程
- **THEN** 系统 MUST 保留原有 open conflict record 的 truth boundary 不变
- **THEN** 系统 MUST 新增或更新与该 `conflict_id` 关联的 resolution artifact

### Requirement: Governance resolution MUST separate decision state from refresh state
resolution artifact MUST 将 `decision state` 与 `refresh state` 分层表达。最小 `decision state` MUST 至少覆盖 `review-required`、`resolved`、`dismissed`、`superseded-by-decision`；最小 `refresh state` MUST 至少覆盖 `none`、`pending`、`completed`。系统 MUST NOT 用单个状态同时表达治理决议和下游物化恢复。

#### Scenario: 决议已通过但下游 projection 尚未刷新
- **WHEN** 冲突已经被标记为 `resolved`
- **THEN** 系统 MUST 仍能将 refresh state 表达为 `pending`
- **THEN** `status` 与后续 workflow MUST 能区分“已决议”和“已恢复完成”

### Requirement: Governance resolution consumers MUST stay at blocker and degraded semantics
`status / update / sync / query` MUST 只消费治理冲突的 blocker、degraded 与 refresh-needed 结论。系统 MUST NOT 在 query surface 中直接暴露治理决策细节、审批语义或平台化工作流字段。

#### Scenario: query 命中带有 governance resolution 的 knowledge target
- **WHEN** 某个命中目标关联到 governance resolution artifact
- **THEN** query MUST 只暴露 blocker/degraded 结论与必要的 recommended action
- **THEN** query MUST NOT 直接返回治理决策细节或审批工作流信息
