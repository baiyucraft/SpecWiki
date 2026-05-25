## ADDED Requirements

### Requirement: 9.7 专项验证必须同时覆盖父页 contract 与 runtime readiness
系统 MUST 在 `storybook + dagger` 专项验证中同时验证高层父页 contract 与 runtime readiness，而不是继续只看最终 Markdown 匹配率。验证 MUST 直接读取最终 `.wiki/*.md`、runtime SQLite 状态与 parent contract 摘要，确认高层父页是否消费 child-backed rollup，及 runtime incomplete 是否能定位到具体 gate。

#### Scenario: storybook 专项验证高层父页 contract
- **WHEN** 系统对 `storybook` 运行 9.7 专项验证
- **THEN** 报告 MUST 指出高层 `Overview`、`Architecture`、`DomainIndex` 或 `config_surface` parent unit 的 reuse 收敛情况
- **THEN** 验证 MUST 证明这些父页存在 child-backed compose contract 或对应的 runtime 摘要
- **THEN** 系统 MUST NOT 仅凭 `overall_match_rate` 或页面数量判断通过

#### Scenario: dagger 专项验证 runtime readiness
- **WHEN** 系统对 `dagger` 运行 9.7 专项验证
- **THEN** 验证 MUST 指出 workflow 当前停在 `research`、`compose` 还是 `assemble` 阶段
- **THEN** 验证 MUST 输出对应 unit 的 gate/readiness 原因
- **THEN** 当 workflow 成功完成时，验证 MUST 观察到 `page_drafts`、最终 wiki 页面和 `wiki.metadata.json` 一并落盘
