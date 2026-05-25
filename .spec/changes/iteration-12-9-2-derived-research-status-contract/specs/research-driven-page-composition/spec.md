## MODIFIED Requirements

### Requirement: research 必须为每个 `KnowledgeUnit` 形成最小正式 summary
系统 MUST 让 research 为每个进入正式 knowledge runtime 的 `KnowledgeUnit` 形成最小正式 summary，而不是继续只把 research 结果留在临时 cache、自由文本摘要或页面拼装上下文。该 summary MUST 至少绑定 `unit_id`、`source_refs`、`citation_refs`、typed `summary_status` 与 compose 可消费的最小 identity。若 summary 被判为 `degraded` 或 `blocked`，系统 MUST 同时写出最小 formal reason contract。

#### Scenario: degraded research summary 不得伪装成 ready
- **WHEN** 当前 research 输出缺少必要 contract，或被正式判为 `degraded`
- **THEN** 系统 MUST 将其写为 `degraded` research summary，而不是继续写成 `ready`
- **THEN** 后续 compose / status / health consumer MUST 能稳定识别这一状态
