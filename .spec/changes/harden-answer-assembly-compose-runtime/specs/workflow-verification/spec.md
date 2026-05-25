## ADDED Requirements

### Requirement: Lifecycle verification MUST assert answer and compose hardening on the existing gate surface
生命周期验证 MUST 在既有 quality gate surface 上增加 answer / compose runtime 断言，至少覆盖 restore、rebuild、provider-backed compose 与长流程 query 的 substrate consistency。系统 MUST NOT 通过新增平行 gate 协议来完成这些验证。

#### Scenario: 运行 answer hardening 的生命周期验证
- **WHEN** 团队执行 lifecycle verification 来验证 answer / compose runtime
- **THEN** 验证结果 MUST 通过既有 gate surface 输出 capability-specific assertions
- **THEN** 系统 MUST NOT 生成独立于 `knowledge-quality-gates` 之外的新 gate output shape
