## MODIFIED Requirements

### Requirement: 本 change 的专项验证必须先覆盖 storybook 的父页 contract
系统 MUST 在本 change 的专项验证中先验证 `storybook` 的高层父页 contract，而不是继续只看最终 Markdown 匹配率。验证 MUST 直接读取最终 `.wiki/*.md`、runtime SQLite 状态与 parent contract 摘要，确认高层父页是否真实产出自己的 `UnitResearch`、是否消费 child-backed rollup。`dagger` 的 runtime readiness 专项 MAY 保留到后续 change，但本 change 不得把其作为前置通过条件。

#### Scenario: storybook 专项验证高层父页 contract
- **WHEN** 系统对 `storybook` 运行 9.7 专项验证
- **THEN** 报告 MUST 指出高层 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 的 reuse 收敛情况
- **THEN** 验证 MUST 证明这些父页存在自己的 `UnitResearch`，并存在 child-backed compose contract 或对应的 runtime 摘要
- **THEN** 系统 MUST NOT 仅凭 `overall_match_rate` 或页面数量判断通过

#### Scenario: 本 change 不把 dagger 作为前置通过条件
- **WHEN** 系统执行 `parent-unit-research-contract` 的专项验收
- **THEN** `storybook` MUST 作为当前唯一的专项门禁样本
- **THEN** 系统 MAY 记录 `dagger` 的观察结果，但 MUST NOT 将其作为本 change 的前置通过条件
