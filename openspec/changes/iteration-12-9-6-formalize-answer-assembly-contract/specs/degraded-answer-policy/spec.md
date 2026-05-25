## ADDED Requirements

### Requirement: degraded answer policy 必须显式消费 runtime 风险信号
系统 MUST 让 degraded answer policy 显式消费 route provenance、`query_trust`、`recommended_action`、health degradation 与 governance conflict，而不是继续把这些风险藏在实现内部。只要当前 answer 依赖 fallback、runtime stale、health degraded 或存在 open governance conflict，系统 MUST 判断是否降级或拒答。

#### Scenario: page fallback 导致 degraded answer
- **WHEN** 当前 answer 主要依赖 `page_fallback` route tag
- **THEN** 系统 MUST 将该 answer 标记为 `degraded`
- **THEN** answer envelope MUST 同时暴露 fallback provenance 与后续 `recommended_action`

#### Scenario: governance conflict 导致 review 型 degraded answer
- **WHEN** 当前 answer 相关的 formal knowledge 同时存在 open governance conflict
- **THEN** 系统 MUST 将该 answer 标记为 `degraded` 或 `refuse`
- **THEN** `recommended_action` MUST 至少能表达 `review` 或等价人工处理动作

### Requirement: degraded answer 必须稳定表达 trust 与 supporting refs
系统 MUST 让 degraded answer 继续返回稳定 `answer_trust`、`recommended_action`、`provenance` 与 `supporting_refs`。系统 MUST NOT 仅因为答案被降级，就退化成无 provenance、无 supporting refs 的自由文本输出。

#### Scenario: stale runtime 下仍可消费的 degraded answer
- **WHEN** 当前 runtime stale，但 formal substrate 仍允许返回受限 answer
- **THEN** 系统 MUST 返回 `degraded` answer，并显式给出 `answer_trust`
- **THEN** 系统 MUST 继续输出 supporting refs，而不是让宿主自行补引用

#### Scenario: health degraded 时 answer 不得伪装成 direct
- **WHEN** 当前 answer 依赖的 formal knowledge 带有 health degradation
- **THEN** 系统 MUST 不得把该 answer 标记成 `direct`
- **THEN** 系统 MUST 继续保留相关 provenance 与 supporting refs
