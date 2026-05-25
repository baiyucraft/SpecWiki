## ADDED Requirements

### Requirement: reference fidelity 报告必须诊断高层父页 contract offender 与 runtime gate blocker
系统 MUST 在 reference fidelity 报告中把 severe reuse 页面映射回具体的高层 `KnowledgeUnit` 类型与 parent contract 状态，并把 runtime incomplete 样本细化为明确的 gate blocker。报告 MUST 能区分“高层父页吞页导致的 coarse reuse”和“runtime 未达到 compose/assemble-ready 导致的无页可比”，而不是把两者混成统一的 fidelity 低分。

#### Scenario: storybook 报告定位高层 parent contract offender
- **WHEN** 报告脚本对 `storybook` 生成 reference fidelity 报告
- **THEN** 报告 MUST 指出 severe reuse 页面对应的 `KnowledgeUnit` 类型或 `decomposition_profile`
- **THEN** 报告 MUST 说明这些页面是否缺少 child-backed contract 摘要或只消费了轻量 child digest

#### Scenario: dagger 报告定位 runtime gate blocker
- **WHEN** 报告脚本对 `dagger` 生成 reference fidelity 报告，且 runtime 尚未完整
- **THEN** 报告 MUST 指出该样本卡在 `research-ready`、`compose-ready` 还是 `assemble-ready` 之前
- **THEN** 报告 MUST 输出对应 blocker、缺失依赖或 interrupted stage
- **THEN** 报告 MUST NOT 把该样本继续并入最终页面 fidelity 汇总分母
