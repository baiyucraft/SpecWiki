## ADDED Requirements

### Requirement: `sync` 必须正确区分 declared 删除写回与 metadata-only
系统 MUST 在 `sync` 中显式区分“当前页没有 declared records”与“当前页删除了此前已有的 declared records”。这一判定 MUST 建立在 page-scoped declared snapshot diff 上：只要 diff 中存在 removed records，`sync` 就 MUST 将其分类为 `declared_writeback`，并返回与 declared 变更一致的推荐动作与受影响对象引用。

#### Scenario: 当前页一直没有 declared，仍然属于 metadata-only
- **WHEN** 某页前一正式 snapshot 中没有 declared records，且本次同步后仍然没有 declared records
- **THEN** `sync` MAY 将该页分类为 `metadata_only`
- **THEN** 系统 MUST NOT 将其误判为 declared 删除写回

#### Scenario: 删除原有 declared 后推荐继续 update
- **WHEN** 某页前一正式 snapshot 中已有 declared records，且本次合法同步后的 page-scoped snapshot diff 中存在 removed records
- **THEN** `sync` MUST 返回 `declared_writeback`
- **THEN** 推荐动作 MUST 继续保持与 declared lifecycle 变更一致，例如 `update`
