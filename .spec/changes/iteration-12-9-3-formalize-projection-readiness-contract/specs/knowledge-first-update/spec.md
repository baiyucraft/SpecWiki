## MODIFIED Requirements

### Requirement: projection refresh 必须由受影响知识范围派生
系统 MUST 让 projection stale 不再只是隐式结果，而是 formal projection object 的状态。若某个 `PageDigest` 因 declared/derived 变化进入 `stale`，系统 MUST 让后续 `update` 继续消费该 projection stale，而不是只靠页面文件差异或 runtime gate 间接推断。

#### Scenario: declared 或 derived 变化使 projection digest 进入 stale
- **WHEN** 某次 declared / derived 变化使现有 projection snapshot 不再与上游 knowledge 对齐
- **THEN** 对应 `PageDigest` MUST 被正式标记为 `stale`
- **THEN** `update` MUST 能继续消费这类 projection stale，而不是只看页面落盘结果
