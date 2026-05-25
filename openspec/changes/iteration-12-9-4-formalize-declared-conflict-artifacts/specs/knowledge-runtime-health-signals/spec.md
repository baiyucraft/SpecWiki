## ADDED Requirements

### Requirement: governance conflict 必须进入 health / status 正式诊断面
系统 MUST 将 open governance conflict 投影为正式 health signal，并允许 `status` 聚合为稳定 `review` 建议。第一批 conflict health signal MUST 能表达冲突对象、严重度、目标引用与 reason，而不是只留下模糊字符串。

#### Scenario: open conflict 被投影为 review signal
- **WHEN** 当前 runtime snapshot 中存在 open declared conflict
- **THEN** 系统 MUST 生成对应的 health signal
- **THEN** 该 signal 的 `recommended_action` MUST 为 `review`

#### Scenario: status 聚合 governance conflict
- **WHEN** runtime readiness 仍可消费，但存在 open declared conflicts
- **THEN** `status` MUST 继续返回可消费 readiness
- **THEN** `status` MUST 同时返回 conflict 相关 health 摘要与 `review` 建议
