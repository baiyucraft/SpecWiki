## Why

当前仓库已经逐步把 query route、declared/derived knowledge、projection digest 与 health/readiness 收进 formal contract，但“answer 怎么从这些正式对象组装出来”仍然没有稳定合同。若继续把 answer 留给宿主或 Agent 临时拼接，后续很容易绕过 formal knowledge substrate，重新退回“看页面正文 + 临时 prompt”。

因此这轮需要单独收敛 `answer assembly contract`：先定义 answer 允许消费哪些 formal inputs、最小输出壳是什么、哪些场景只能降级或拒答，而不是提前去做更聪明的回答系统。

## What Changes

- 新增 `answer-assembly-contract` capability，正式定义 answer assembly 可消费的 formal inputs、最小 `AnswerEnvelope`、assembly policy 与 supporting refs。
- 新增 `degraded-answer-policy` capability，正式定义当 route 落到 fallback、runtime stale、health degraded 或存在 governance conflict 时，answer 如何降级、提示动作或拒答。
- 新增 `host-knowledge-answer-surface` capability，固定宿主/Agent 可以稳定依赖的最小 answer surface，而不是继续把 answer 视为 query 之后的宿主私活。
- 修改 `research-driven-page-composition`，明确 answer assembly 必须优先消费 unit research summary、projection digest 与 formal knowledge refs，而不是回退到页面正文或临时 cache。
- 固定本轮非目标：不重写 query route、不做 host/UI 扩展、不做 multi-turn session memory、不做复杂 synthesis 或 provider/prompt 优化平台。

## Capabilities

### New Capabilities
- `answer-assembly-contract`: 定义 answer assembly 的正式输入、正式输出与最小装配策略。
- `degraded-answer-policy`: 定义 answer 在 fallback、stale、health degraded、governance conflict 等场景下的降级与拒答合同。
- `host-knowledge-answer-surface`: 定义宿主与 Agent 可稳定消费的最小 answer transport surface。

### Modified Capabilities
- `research-driven-page-composition`: 收紧 answer assembly 与 research summary / projection digest 的消费边界，防止页面正文重新成为唯一 answer substrate。

## Impact

- 主要影响 `wiki-runtime` 后续的 query/answer 装配层、transport/DTO，以及 `wiki-knowledge` 中可供 answer 复用的 summary / projection substrate。
- 影响宿主与 Agent 后续如何消费 answer，但本轮不直接扩展 UI 或多轮会话协议。
- 为后续 answer 实现、degraded diagnostics 和 quality gates 建立正式前置合同。
