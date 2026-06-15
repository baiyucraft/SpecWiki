## 1. Model Contract

- [x] 1.1 在 `wiki-model` 中扩展 `DeclaredRecord` schema，补齐 typed `scope`、`relations`、lifecycle status 与稳定 identity 字段
- [x] 1.2 为 `supersedes / replaced_by / deprecated` 定义最小 relation / status 枚举、单义状态判定规则与 canonical serialization
- [x] 1.3 收紧 declared block 到正式 record 的解析合同，明确 parse failure、delete/tombstone 与 upsert 规则

## 2. Artifact Storage

- [x] 2.1 更新 `.wiki/.knowledge/declared/**` 的持久化格式，使 declared artifact 能稳定写出 scope、relations、status 与 snapshot identity
- [x] 2.2 更新 recovery / metadata 锚点，使 declared snapshot 成为正式 restore 校验的一部分
- [x] 2.3 实现 declared artifact 的 load / roundtrip / cache-less restore 路径，禁止从页面正文反推 declared truth

## 3. Workflow Runtime

- [x] 3.1 更新 `sync`，让受管 declared 编辑面能够回写 typed scope、relations 与 lifecycle delta，并固定 `illegal_drift > declared_writeback > metadata_only` 分类优先级
- [x] 3.2 更新 `update`，让 declared lifecycle stale scope 在无源码 dirty set 时也能驱动 derived / projection refresh
- [x] 3.3 更新 `status` 与相关 runtime 诊断输出，使 declared lifecycle 能稳定影响 readiness、health summary 与 `recommended_action`
- [x] 3.4 检查 `query` 与 workflow DTO，确保只增强 declared lifecycle 可解释性，不引入 answer assembly 或治理平台 contract

## 4. Tests

- [x] 4.1 补 unit tests，覆盖 declared artifact 的 relation consistency、identity 稳定性与 typed scope canonical serialization
- [x] 4.2 补 `sync` 分类矩阵测试，覆盖 `declared_writeback`、`metadata_only`、`illegal_drift` 与混合变更优先级
- [x] 4.3 补 storage / workflow integration tests，覆盖局部 parse failure、stale propagation、recommended action 与 cache-less restore
- [x] 4.4 执行相关层级测试，并按 AGENTS 约束补跑 `node scripts/run-test-projects.mjs` 的样本验证

## 5. Docs And Hygiene

- [x] 5.1 更新 runtime / knowledge 相关文档，明确 declared artifact 是 truth layer，page 不是 source of truth
- [x] 5.2 在实现改动中检查注释是否符合 `.wiki/02-开发指南/00-代码注释规范.md`，删除不再成立的弱约定与兼容描述
- [x] 5.3 复核 UniSpec 产物与实现边界一致，不把本轮扩成 answer assembly、provider hardening 或治理平台
