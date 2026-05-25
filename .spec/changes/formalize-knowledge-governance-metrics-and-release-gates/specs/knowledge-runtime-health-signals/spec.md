## ADDED Requirements

### Requirement: Health signals MUST support stable aggregation into governance metrics
`knowledge-runtime-health-signals` MUST 提供可稳定聚合到 governance metrics 的最小字段和分类语义，使 `stale`、`blocked`、`conflict`、`missing provenance` 与 `degraded answer` 能被一致统计。系统 MUST NOT 依赖 ad-hoc 文本解析来计算这些 release-facing metrics。

#### Scenario: release governance 聚合 health 状态
- **WHEN** 系统从 runtime health signals 计算 governance metrics
- **THEN** 聚合逻辑 MUST 能稳定识别各类 release-facing health bucket
- **THEN** 系统 MUST 不需要解析自由文本 message 才能区分这些 bucket
