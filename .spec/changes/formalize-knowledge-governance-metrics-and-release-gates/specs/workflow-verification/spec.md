## ADDED Requirements

### Requirement: Workflow verification MUST emit governance metrics and blocker judgment through the existing gate summary contract
`workflow-verification` MUST 在既有 `knowledge-quality-gates` summary contract 上补充 knowledge governance metrics 与 blocker judgment。系统 MUST NOT 为 release governance 再定义第二套 summary transport shape。

#### Scenario: 生命周期与批量验证完成后生成 release judgment
- **WHEN** workflow verification 汇总 primary sample 与 batch baseline 结果
- **THEN** 输出 MUST 复用既有 gate summary contract
- **THEN** 输出 MUST 额外包含 governance metrics、blocker judgment 与 waiver 信息
