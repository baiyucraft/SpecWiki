## ADDED Requirements

### Requirement: declared 删除/撤销必须进入正式 lifecycle writeback
系统 MUST 将“合法删除此前存在的 declared records”视为正式 declared lifecycle writeback，而不是普通 metadata-only 编辑。删除判定 MUST 建立在 page-scoped declared snapshot diff 之上：若某页前一正式 snapshot 与当前合法受管编辑面的 declared snapshot 之间存在 removed records，则系统 MUST 将这次变化视为 declared truth 的正式删除写回。该删除语义仅表示当前正式 snapshot 不再包含这些 records，MUST NOT 被解释为 tombstone、deprecated 或 superseded。

#### Scenario: 合法删除部分 declared records
- **WHEN** 某个页面在前一正式 snapshot 中已有多条 declared records，且用户通过合法受管编辑面删除了其中部分 records
- **THEN** `sync` MUST 将该页归类为 `declared_writeback`
- **THEN** 系统 MUST 将 page-scoped snapshot diff 中的 removed records 视为本次 lifecycle writeback 的删除结果

#### Scenario: 合法删除全部 declared records
- **WHEN** 某个页面在前一正式 snapshot 中已有 declared records，且用户通过合法受管编辑面删除了全部 records
- **THEN** `sync` MUST 继续将该页归类为 `declared_writeback`
- **THEN** 系统 MUST 将该页当前 declared snapshot 视为合法的空集合，而不是退化为 metadata-only

#### Scenario: 非法 drift 不得伪装成 declared 删除
- **WHEN** 用户删除 declared blocks 的同时破坏了 marker、产生非 declared 正文漂移，或使页面进入 parse failure
- **THEN** `sync` MUST 继续将该页归类为 `illegal_drift`
- **THEN** 系统 MUST NOT 提交这次 declared 删除写回
