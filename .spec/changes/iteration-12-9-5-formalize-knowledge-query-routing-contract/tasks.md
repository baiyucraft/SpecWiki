## 1. Route Contract 收口

- [ ] 1.1 为 query route 引入稳定的 lane / provenance contract，明确 `index_hit / graph_hit / declared_hit / derived_hit / page_fallback` 的正式来源与使用边界。
- [ ] 1.2 收紧 `query_mode / query_trust / recommended_action / provenance_summary` 的职责分层，避免 readiness 与 provenance 混层。
- [ ] 1.3 复核本轮非目标：不引入 answer assembly、不改 host/UI、不开放 intent-aware 外部 payload、不做复杂 rerank。

## 2. Workflow 实现

- [ ] 2.1 调整 `wiki-runtime` 的 query route，让 `symbol/index -> graph -> declared -> derived -> page fallback` 的参与条件与优先级可被稳定判断。
- [ ] 2.2 让 `declared` 与 `derived` 在 query 命中中可稳定区分，不再只压平成单一 `knowledge_hit`。
- [ ] 2.3 收紧 page/BM25 fallback 语义，确保 page 只作为最后一层兜底或补充 provenance。

## 3. 测试与验收

- [ ] 3.1 补 query route 单测 / runtime 测试，覆盖 graph lane、declared lane、derived lane 与 page fallback 的 precedence 与 provenance。
- [ ] 3.2 补 degraded policy 测试，覆盖 `query_trust / recommended_action / provenance_summary` 的分层表达。
- [ ] 3.3 跑 `cargo test -p wiki-runtime`。
- [ ] 3.4 跑 `node scripts/run-test-projects.mjs --jobs 1 --timeout-minutes 90 storybook dagger`，确认样本仓库 query route 不退化成 page-first。
- [ ] 3.5 跑 `node scripts/run-test-projects.mjs`，按当前项目集完成批量 `init` 验收并检查 query 相关回归。

## 4. 注释与收尾

- [ ] 4.1 按 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md) 复核本轮新增或修改的 query route 注释，确保注释解释语义与边界，而不是复述代码。
- [ ] 4.2 同步更新 UniSpec tasks 状态，并记录本轮 route contract 的验证结果与剩余缺口。
