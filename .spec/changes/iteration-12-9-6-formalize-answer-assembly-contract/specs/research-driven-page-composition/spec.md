## MODIFIED Requirements

### Requirement: compose 必须从 unit summary 生成 projection digest，而不是直接把页面当真相
系统 MUST 让 compose 以 `KnowledgeUnit` 的 formal research summary、child rollup 与 projection policy 为输入，生成可复用的 `projection digest` 或等价 formal projection object。页面 Markdown 只是 projection 落盘结果，不得反向成为 compose 的主真相，也不得成为 answer assembly 的唯一依据。answer assembly 若主要依赖 knowledge 层结果，MUST 优先消费 unit research summary、projection digest 与正式 supporting refs，而不是直接回读页面正文或临时 cache。

#### Scenario: compose 先生成 projection digest 再写页面
- **WHEN** 系统为某个 `KnowledgeUnit` 生成正式页面投影
- **THEN** compose MUST 先形成稳定的 `projection digest` 或等价 formal projection object
- **THEN** 页面写盘 MUST 视为该 projection 的结果，而不是直接替代 projection contract

#### Scenario: query answer assembly 不得绕过 unit summary
- **WHEN** 某次 query 或 answer assembly 主要依赖 knowledge 层结果回答问题
- **THEN** answer assembly MUST 优先消费 unit research summary、projection digest 与正式 supporting refs
- **THEN** 系统 MUST NOT 继续把页面正文或临时 cache 当作唯一 knowledge answer substrate
