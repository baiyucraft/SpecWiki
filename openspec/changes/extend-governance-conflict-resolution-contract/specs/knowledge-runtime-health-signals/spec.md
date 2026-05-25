## ADDED Requirements

### Requirement: Health signals MUST distinguish open conflicts from resolved-pending-refresh governance states
系统 MUST 将 `open conflict`、`resolved but refresh pending` 与 `dismissed/non-blocking` 的 governance 状态区分为稳定的 health / recommended action 语义。系统 MUST NOT 将这些状态全部压平成同一个 conflict warning。

#### Scenario: conflict 已决议但 runtime 尚未恢复
- **WHEN** 某个 conflict 对应的 resolution state 为 `resolved` 且 refresh state 为 `pending`
- **THEN** health signals MUST 将其表达为与 open conflict 不同的稳定状态
- **THEN** recommended action MUST 指向 refresh / update，而不是继续要求 review
