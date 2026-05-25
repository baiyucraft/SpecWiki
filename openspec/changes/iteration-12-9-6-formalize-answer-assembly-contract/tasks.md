## 1. Contract 收口

- [x] 1.1 定义 answer assembly 的 formal inputs，固定只允许消费 `index / graph hits`、`declared records`、`derived summaries`、`projection/page refs` 与 `health / readiness`。
- [x] 1.2 定义最小 `AnswerEnvelope`，固定 `answer_mode / answer_trust / recommended_action / provenance / supporting_refs` 的正式语义。
- [x] 1.3 收紧本轮非目标：不重写 query route、不做 host/UI 扩张、不做 multi-turn memory、不做复杂 synthesis 或 provider/prompt 优化。

## 2. Policy 设计

- [x] 2.1 定义 answer assembly 的 `direct / degraded / refuse` 装配策略。
- [x] 2.2 定义 degraded answer policy，覆盖 fallback、stale、health degraded 与 governance conflict。
- [x] 2.3 定义宿主与 Agent 共用的最小 answer surface，避免平行 contract。

## 3. Spec 与实现准备

- [x] 3.1 让 `research-driven-page-composition` 与 answer contract 对齐，明确 projection digest 与 supporting refs 的关系。
- [x] 3.2 为后续 `wiki-runtime` / `wiki-knowledge` 实现列出 DTO、workflow consumer 与 artifact consumer 变更点。
- [x] 3.3 单独检查 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 对 answer contract 相关类型、字段和流程注释的要求。

## 4. 验收规划

- [x] 4.1 为 direct / degraded / refuse 三类 answer 场景补自动化测试设计。
- [x] 4.2 为 supporting refs、provenance 与 recommended_action 的宿主消费补 transport 验收设计。
- [x] 4.3 规划 `storybook + dagger` 与批量项目集在 answer contract 上的后续验收入口。
