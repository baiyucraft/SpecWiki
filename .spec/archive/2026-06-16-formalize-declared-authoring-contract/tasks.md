---
implementation-ready: true
---

# formalize-declared-authoring-contract 任务计划

## 任务总览

本任务计划用于收口 declared authoring contract 的剩余阻塞项。既有实现已经覆盖大部分模型、artifact 和 runtime 主路径；本轮重点修正验收 artifact、长期 capability baseline、缺失的 lifecycle relation runtime 测试，以及 workspace gate 失败。

## 实现模式

normal

normal：参考结构化实现模式，先按具体实现清单推进，完成大 task 前补齐单元测试或替代局部验证。

## 1. 验收 artifact 与 capability baseline 收口

- [x] 1.1 将 `system-tests.md` 从迁移占位改为 declared authoring 归档级系统测试用例，覆盖 ST-001 至 ST-008。
- [x] 1.2 更新 `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md`，移除 declared 仅占位 / 不进入正式 contract 的旧口径。
- [x] 1.3 更新 `.wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md`，补齐 typed `scope`、`relations`、`status`、`record_id` 和正式 truth layer 语义。
- [x] 1.4 复核 `review-report.md` / `test-report.md` 的 fail 结论仍作为历史审查记录，不把它们改写成 pass。

### CheckList

- [x] 单元测试或替代局部验证已覆盖
- [x] 相关验证通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. Declared lifecycle relation runtime 测试

- [x] 2.1 在 runtime integration tests 中覆盖 managed declared block 写入 `deprecated / replaced_by / supersedes` 的 parse -> artifact 行为。
- [x] 2.2 覆盖 lifecycle relation artifact restore / roundtrip 后字段保持稳定。
- [x] 2.3 覆盖 declared relation 被 status/query 或等价诊断面可观察消费。
- [x] 2.4 覆盖 page 正文与 declared artifact 冲突时不得从页面反推 truth 的负向场景。

### CheckList

- [x] 单元测试或替代局部验证已覆盖
- [x] 相关验证通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. Verification gate 修复

- [x] 3.1 修复 `pnpm run test` 中 `build_minimal_page_context_persists_child_contract_and_unit_identity` 的 runtime workflow 失败，或补正式隔离依据并从归档门槛移出。
- [x] 3.2 将 `node scripts/run-test-projects.mjs` 的样本验证重新定义为非门禁 smoke check；当输出 0 total 时只能记录 skipped / not-applicable。
- [x] 3.3 运行 declared 定向验证：`unispec validate`、`cargo test -p wiki-model declared`、`editable_runtime`、`knowledge_artifacts_roundtrip`、`status_and_update`、`query_sync_rebuild`。
- [x] 3.4 运行 workspace 验证：`pnpm run lint`、`pnpm run test`。

### CheckList

- [x] 单元测试或替代局部验证已覆盖
- [x] 相关验证通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. 验收 artifact 与 capability baseline 收口 | 1.1 |
| ST-002 | 2. Declared lifecycle relation runtime 测试 | 2.1 |
| ST-003 | 2. Declared lifecycle relation runtime 测试 | 2.1 / 2.2 |
| ST-004 | 2. Declared lifecycle relation runtime 测试 | 2.1 |
| ST-005 | 2. Declared lifecycle relation runtime 测试 | 2.2 / 2.4 |
| ST-006 | 2. Declared lifecycle relation runtime 测试 | 2.3 |
| ST-007 | 3. Verification gate 修复 | 3.1 / 3.4 |
| ST-008 | 3. Verification gate 修复 | 3.2 |

## 执行顺序

- 先完成 task 1，避免在旧 contract 上补测试。
- 再完成 task 2，测试暴露实现缺口时只做局部实现修复。
- 最后完成 task 3，并重新进入 `unispec-review`。

## 暂缓事项

- 不扩展 answer assembly、provider hardening 或治理平台。
- 不把样本验证 0 total 当作 pass 证据。
