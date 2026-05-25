## ADDED Requirements

### Requirement: index-only runtime 必须暴露显式 `index_only` 外部状态
当 `v0.1.0` release scope 让 workflow 在 facts snapshot 后短路时，系统 MUST 对外返回显式 `index_only` 状态，而不是继续复用 `missing`。该状态 MUST 表示“facts/index 已就绪，knowledge/page runtime 未进入正式承诺范围”，并且 `status`、`query`、`init`、`update` 看到的外部状态 MUST 保持一致。

#### Scenario: init 的 index-only 终态不再冒充 missing
- **WHEN** 用户在开启 `v0.1.0 index-only` release scope 的情况下执行 `init`
- **THEN** `init` 返回中的 `state` MUST 为 `index_only`
- **THEN** 系统 MUST 继续写入可供 query 使用的 facts snapshot
- **THEN** 系统 MUST NOT 把该终态写成 `missing`

#### Scenario: status 能区分未初始化与 index-only 已就绪
- **WHEN** 仓库 facts snapshot 已提交，但当前 release scope 只完成 index-only runtime
- **THEN** `status.state` MUST 为 `index_only`
- **THEN** `query_readiness` MUST 为 `ready`
- **THEN** `recommended_action` MUST 为 `none`

### Requirement: query trust 必须反映本次返回结果是否可消费
系统 MUST 让 `query_trust` 至少区分“本次结果可直接消费”“结果来自陈旧或降级路径但仍可消费”“结果当前不可消费”三种状态，而不能只把 runtime preflight 原样透传为 trust。若本次 query 已返回 page fallback 或 mixed 结果，即使 runtime 需要 rebuild，`query_trust` 也 MUST NOT 继续报告为 `blocked`。

#### Scenario: page fallback 命中时 trust 至少可查询
- **WHEN** 当前 runtime preflight 为 `blocked`，但本次 query 已返回 page fallback 或 mixed 结果
- **THEN** `query_trust` MUST 为可查询状态
- **THEN** `recommended_action` MAY 继续提示 `rebuild`
- **THEN** 返回结果 MUST 不得同时表现为“有结果但完全 blocked”
