## ADDED Requirements

### Requirement: `sync` / `update` 必须把 conflict artifact 重算纳入正式提交链
系统 MUST 在 declared snapshot 合法提交后重算 conflict artifacts，并把它们与 declared / health / projection artifacts 一起提交到同一正式 snapshot。`illegal_drift` MUST 继续高于 conflict artifact 提交；系统 MUST NOT 在半合法 declared 输入之上刷新 conflict objects。

#### Scenario: declared writeback 后同步刷新 conflicts
- **WHEN** 某次 `sync` 成功提交 declared writeback
- **THEN** 系统 MUST 基于最新 declared snapshot 重算 conflict artifacts
- **THEN** 新 snapshot MUST 同步反映 conflict 的新增、延续或消失

#### Scenario: illegal drift 不得触发 conflict 提交
- **WHEN** 某次 `sync` 判定页面变更为 `illegal_drift`
- **THEN** 系统 MUST NOT 提交新的 declared snapshot 或 conflict artifact
- **THEN** runtime MUST 保持上一份已提交的 formal governance 结果
