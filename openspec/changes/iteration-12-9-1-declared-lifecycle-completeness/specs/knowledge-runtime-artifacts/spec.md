## ADDED Requirements

### Requirement: declared 删除写回必须从正式 snapshot 中移除 removed records
系统 MUST 让 declared 删除写回真实反映到 `.wiki/.knowledge/declared/**` 的正式 snapshot 中。若某页在前一 snapshot 与当前合法写回结果之间存在 removed records，则 artifact merge MUST 从新 snapshot 中移除这些旧 records，而不是静默保留；若该页当前合法 snapshot 仍包含其它 declared records，则系统 MUST 保留这些 surviving records。

#### Scenario: page-scoped declared diff pruning 成功提交
- **WHEN** 某页发生合法 declared 删除写回，且 page-scoped snapshot diff 中存在 removed records
- **THEN** 新的 `.wiki/.knowledge/declared/**` snapshot MUST 不再包含这些 removed records
- **THEN** restore 后的 runtime MUST 也不得重新恢复这些已删除 records
