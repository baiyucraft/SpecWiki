## ADDED Requirements

### Requirement: Reference reports MUST include knowledge governance summary and evidence manifest linkage
`reference-fidelity-reporting` MUST 在既有样本报告中加入 knowledge governance summary，并能链接到 release evidence manifest。系统 MUST 不再只输出页面结构差异或 gate 结论。

#### Scenario: 为 storybook 或 dagger 生成样本报告
- **WHEN** 系统生成 `storybook` 或 `dagger` 的 reference report
- **THEN** 报告 MUST 包含该样本的 governance summary
- **THEN** 报告 MUST 能关联到对应的 release evidence manifest 或等价结构化证据
