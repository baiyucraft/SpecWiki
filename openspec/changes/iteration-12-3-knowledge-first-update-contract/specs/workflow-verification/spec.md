## ADDED Requirements

### Requirement: lifecycle 验证必须把磁盘诊断快照提升为可消费的 diagnostic state
系统 MUST 让 lifecycle verification 在遇到“磁盘侧只有恢复/缺失快照”的场景时，将底层 `missing` 或等价原始状态提升为可消费的 diagnostic runtime state，而不是把它继续当成普通 `missing`。提升后的结果 MUST 保留 `query_readiness`、`recommended_action`，并让诊断原因继续通过 harness/runtime snapshot 可见，使测试与宿主能够消费一致的状态语义。

#### Scenario: 缺失 metadata 时提升为 runtime_incomplete 诊断态
- **WHEN** lifecycle harness 观察到底层状态为 `missing`，且磁盘诊断快照表明当前 repo 只是缺少 `wiki.metadata.json` 或等价恢复入口
- **THEN** harness MUST 把该结果提升为可消费的 diagnostic runtime state
- **THEN** 提升后的结果 MUST 继续保留 `query_readiness` 与 `recommended_action`

#### Scenario: 诊断态提升不伪造 fresh/ready
- **WHEN** lifecycle harness 把底层磁盘快照提升为 diagnostic runtime state
- **THEN** 系统 MUST NOT 把它伪装成 `fresh`、`ready` 或正常可写的 full runtime
- **THEN** 诊断原因 MUST 继续可见，便于调用方判断是否需要 `init`、`update` 或其他恢复动作
