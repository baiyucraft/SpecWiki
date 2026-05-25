## MODIFIED Requirements

### Requirement: compose 必须从 unit summary 生成 projection digest，而不是直接把页面当真相
系统 MUST 让 compose 以 `KnowledgeUnit` 的 formal research summary、child rollup 与 projection policy 为输入，生成可复用的 `projection digest` 或等价 formal projection object。该 projection digest 除现有内容摘要外，MUST 同时携带 typed `projection_status` 与必要时的最小 formal reason contract。页面 Markdown 只是 projection 落盘结果，不得反向成为 compose 的主真相或 answer assembly 的唯一依据。

#### Scenario: compose-ready projection digest 被标记为 ready
- **WHEN** 系统已完成某个页面的 compose 与 projection 绑定
- **THEN** 对应 `PageDigest` MUST 标记为 `ready`
- **THEN** 它不得继续只靠 `readiness_stage=compose_ready` 暗示 projection 已可正式消费
