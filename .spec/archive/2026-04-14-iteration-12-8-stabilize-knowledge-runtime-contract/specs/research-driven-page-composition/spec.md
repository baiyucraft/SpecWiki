## ADDED Requirements

### Requirement: research 必须为每个 `KnowledgeUnit` 形成最小正式 summary
系统 MUST 让 research 为每个进入正式 knowledge runtime 的 `KnowledgeUnit` 形成最小正式 summary，而不是继续只把 research 结果留在临时 cache、自由文本摘要或页面拼装上下文。该 summary MUST 至少绑定 `unit_id`、`source_refs`、`citation_refs`、`summary_status` 与 compose 可消费的最小 identity。

#### Scenario: unit research summary 成为 derived formal object
- **WHEN** 某个 `KnowledgeUnit` 完成 research
- **THEN** 系统 MUST 生成与该 `unit_id` 绑定的正式 research summary
- **THEN** compose 与 query 后续 MUST 能引用该 summary，而不是重新读取页面正文反推

#### Scenario: research 缺失来源时不得伪装成正式 summary
- **WHEN** 当前 research 输出缺少必要 `source_refs` 或 citation 绑定
- **THEN** 系统 MUST NOT 将其视为 healthy 的正式 summary
- **THEN** 系统 MUST 产生相应 health signal 或 gate 结果

### Requirement: compose 必须从 unit summary 生成 projection digest，而不是直接把页面当真相
系统 MUST 让 compose 以 `KnowledgeUnit` 的 formal research summary、child rollup 与 projection policy 为输入，生成可复用的 `projection digest` 或等价 formal projection object。页面 Markdown 只是 projection 落盘结果，不得反向成为 compose 的主真相或 answer assembly 的唯一依据。

#### Scenario: compose 先生成 projection digest 再写页面
- **WHEN** 系统为某个 `KnowledgeUnit` 生成正式页面投影
- **THEN** compose MUST 先形成稳定的 `projection digest` 或等价 formal projection object
- **THEN** 页面写盘 MUST 视为该 projection 的结果，而不是直接替代 projection contract

#### Scenario: query answer assembly 不得绕过 unit summary
- **WHEN** 某次 query 主要依赖 knowledge 层结果回答问题
- **THEN** answer assembly MUST 优先消费 unit research summary 与 projection digest
- **THEN** 系统 MUST NOT 继续把页面正文或临时 cache 当作唯一 knowledge answer substrate
