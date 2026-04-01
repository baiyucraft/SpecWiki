## 1. Contract Narrowing

- [x] 1.1 收紧 `proposal.md` 边界，明确本 change 只覆盖 parent unit contract 与 lifecycle diagnostic verification
- [x] 1.2 补齐 `design.md`，说明为什么本 change 不能继续混入 `wiki-index/query`、`steering/config` 与 `Agents/CLI`

## 2. Delta Specs

- [x] 2.1 为 `knowledge-unit-decomposition` 增加 delta spec，明确高层 parent unit 必须保持 `UnitResearch` 与 child rollup 的稳定输入身份
- [x] 2.2 为 `workflow-verification` 增加 delta spec，明确 lifecycle harness 必须把磁盘诊断快照提升为可消费的 diagnostic state

## 3. Verification

- [x] 3.1 保留 `scripts/tests/streaming-protocol.test.ts` 中对 diagnostic state coercion 的聚焦测试

## 4. Review And Hygiene

- [x] 4.1 让高标准 reviewer 审核“剩余改动不能一起并入本 change”的边界判断
- [x] 4.2 检查本轮 OpenSpec 产物与现有剩余改动的边界一致性
