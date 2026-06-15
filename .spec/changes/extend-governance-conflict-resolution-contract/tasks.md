## 1. Resolution Model And Artifacts

- [ ] 1.1 定义 `KnowledgeConflictResolution` 或等价 companion artifact schema，并保持 `KnowledgeConflictRecord` 继续 open-only
- [ ] 1.2 在既有 runtime snapshot layer 上新增 resolution companion artifact 的持久化、恢复与校验路径，不暗改 `12-9-4` 的 conflict truth boundary
- [ ] 1.3 补 decision state 与 refresh state 的最小状态机、canonical serialization 与 identity/association 规则

## 2. Workflow Consumption

- [ ] 2.1 更新 `status / sync / update`，让 workflow 正式消费 resolution artifact 的 blocker、refresh-needed 与 recommended action 语义
- [ ] 2.2 更新 `query` 侧消费边界，只暴露 blocker/degraded 结论，不暴露治理决策细节或平台化字段
- [ ] 2.3 保持 deterministic conflict detection 仍由 `12-9-4` 的 open conflict object 承担，本轮不重写 detection 逻辑

## 3. Tests And Acceptance

- [ ] 3.1 补 model / storage tests，覆盖 open conflict record 与 companion resolution artifact 的分层、roundtrip 与恢复路径
- [ ] 3.2 补 workflow integration tests，覆盖 `review-required`、`resolved + refresh pending`、`dismissed` 等关键状态
- [ ] 3.3 运行相关层级测试，并按需要补跑 `node scripts/test-wiki-lifecycle.mjs` 与 `storybook + dagger` 样本验证，证明治理状态不会破坏现有 runtime gate

## 4. Docs And Hygiene

- [ ] 4.1 更新相关文档，明确 conflict record 与 resolution artifact 的职责分层
- [ ] 4.2 检查实现注释是否符合 [.wiki/02-开发指南/00-代码注释规范.md](/E:/project/!byAI/spec-wiki/.wiki/02-开发指南/00-代码注释规范.md)，删除把 governance resolution 说成平台审批系统的误导性注释
- [ ] 4.3 复核 UniSpec 与实现边界一致，确保本轮不扩成 dashboard、审批 UI 或跨 repo 治理
