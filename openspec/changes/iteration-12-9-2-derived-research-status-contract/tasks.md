## 1. Spec 收口

- [x] 1.1 更新 `research-driven-page-composition`、`knowledge-runtime-artifacts`、`knowledge-runtime-health-signals`，明确 derived research status contract。
- [x] 1.2 新增 `derived-research-status-contract` spec，固定 `summary_status`、reason contract 与 validation 边界。
- [x] 1.3 复核本轮非目标：不碰 evidence layer 全合同、不碰 query/answer、不碰 governance/provider policy。

## 2. Model 与 Artifact 实现

- [x] 2.1 为 `KnowledgeResearchSummary` 引入 typed `summary_status` 与最小 reason object。
- [x] 2.2 在 `UnitResearch -> KnowledgeResearchSummary` 收敛路径中补齐 `ready / degraded / blocked` 的正式判定。
- [x] 2.3 为 research summary snapshot 增加 canonicalize / validate，并在 artifact persist / restore 前执行。

## 3. Runtime 投影

- [x] 3.1 让 `KnowledgeUnit` 正式消费 derived research `ready / degraded / blocked` 状态。
- [x] 3.2 让 `status` / health signals 能聚合 degraded / blocked research summary，而不是只看 runtime gate。
- [x] 3.3 保持本轮只做只读投影，不引入新的 update/rebuild 状态机或 recommended action 家族。

## 4. 测试与验证

- [x] 4.1 补 model / artifact roundtrip / restore validation 测试。
- [x] 4.2 补 runtime status / health / unit projection 测试，覆盖 `ready / degraded / blocked`。
- [x] 4.3 跑 `cargo test -p wiki-runtime`。
- [x] 4.4 跑 `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 90 storybook dagger`。
- [x] 4.5 单独检查 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 合规性，并同步更新 tasks 状态。
