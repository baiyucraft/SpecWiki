## ADDED Requirements

### Requirement: Compose outputs MUST expose answer-reusable provenance and projection substrate
`research-driven-page-composition` MUST 为 answer runtime 稳定暴露可复用的 provenance、citation、projection digest 与 supporting refs substrate。系统 MUST NOT 只产出可读页面文本而缺失 formal answer inputs。

#### Scenario: compose 完成后 answer runtime 复用其结果
- **WHEN** 某个 `KnowledgeUnit` 完成 research / compose
- **THEN** compose 输出 MUST 同时包含页面投影与可供 answer 复用的 formal substrate
- **THEN** answer runtime MUST 不需要回读正文才能重新获得 provenance 与 supporting refs
