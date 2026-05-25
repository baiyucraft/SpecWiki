## ADDED Requirements

### Requirement: runtime snapshot 必须正式包含 declared conflict artifacts
系统 MUST 在 `.wiki/.knowledge/runtime/**` 的正式 snapshot 中持久化 declared conflict artifacts。最小 artifact 集 MUST 包含 `conflict-records.jsonl` 或等价正式对象文件。persist / restore MUST 将 conflict snapshot 视为正式 runtime artifact 的一部分，而不是可有可无的辅助输出。

#### Scenario: declared conflict artifact 进入正式 runtime snapshot
- **WHEN** 当前 declared snapshot 中存在至少一条 open conflict
- **THEN** `.wiki/.knowledge/runtime/**` MUST 写出对应 `conflict-records` artifact
- **THEN** `recovery-manifest` 或等价 snapshot 身份机制 MUST 把该 artifact 纳入一致性校验

#### Scenario: restore 拒绝非法 conflict snapshot
- **WHEN** `conflict-records` artifact 自身 schema 非法、与 declared snapshot 不对齐，或引用了不存在的 declared record
- **THEN** restore MUST 显式拒绝当前 runtime snapshot
- **THEN** 系统 MUST NOT 静默忽略 conflict artifact 继续恢复
