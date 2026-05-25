## 1. Formal Substrate Hardening

- [ ] 1.1 收紧 answer / compose runtime 的 substrate contract，明确 declared / derived / projection 是 primary answer substrate，禁止 silent page fallback
- [ ] 1.2 强化 supporting refs、trust、provenance 与 degraded reason 的一致性校验，覆盖 restore、rebuild 与 provider-backed compose
- [ ] 1.3 更新 compose 输出面，使 answer 可直接复用 provenance、citation 与 projection substrate，而不必回读正文

## 2. Workflow And Runtime

- [ ] 2.1 更新 `query / restore / rebuild` 的 answer 装配逻辑，在 substrate 漂移时稳定输出 `degraded / refuse`
- [ ] 2.2 复用 `12-9-7` 已 formalize 的 gate surface，只新增 answer / compose capability-specific assertions，不新增平行 gate transport
- [ ] 2.3 将 governance resolution richer state 接入 degraded policy，使 answer 能区分 blocker、resolved-pending-refresh 与 non-blocking 状态

## 3. Validation And Evidence

- [ ] 3.1 补 runtime / transport tests，覆盖 supporting refs 缺失、trust 漂移、restore 后 answer substrate 漂移等场景
- [ ] 3.2 扩展生命周期验证，覆盖 restore、rebuild、provider-backed compose 与长流程 query 的 substrate consistency
- [ ] 3.3 以 `storybook + dagger` 为 primary evidence 补充 answer / compose hardening 验证，但保持其挂接在既有 quality gate surface 上

## 4. Docs And Hygiene

- [ ] 4.1 更新 query / answer / compose 相关文档，明确这轮是 runtime hardening，不是重新 formalize answer contract
- [ ] 4.2 检查实现注释是否符合 [COMMENTING.md](/E:/project/!byAI/spec-wiki/COMMENTING.md)，删除暗示 page fallback 可作为正常 answer substrate 的旧注释
- [ ] 4.3 复核 OpenSpec、测试和实现边界一致，确保本轮不重写 query route、不扩成 host/UI 功能
