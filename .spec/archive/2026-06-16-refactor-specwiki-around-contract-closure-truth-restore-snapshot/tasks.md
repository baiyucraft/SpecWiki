---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-truth-restore-snapshot 任务计划

## 任务总览

任务按 design.md 的能力块拆分：RuntimeReadiness 公开合同、RestoreOutcome 与 graph origin、committed snapshot manifest、restore guard、query degraded。采用 contract-first TDD。

## 实现模式

tdd

先写失败单元测试并确认失败，再写最小实现，通过后重构。

## 1. RuntimeReadiness 公开合同

- [x] 1.1 Red: UT-001 编写 status 公开 `readiness` 且不公开 `facts_ready/query_readiness` 的失败测试，并确认失败原因符合预期
- [x] 1.2 Green: UT-001 在 `runtime_profile.rs` 与 `status.rs` 中实现 `RuntimeReadiness` 输出，使测试通过
- [x] 1.3 Refactor: UT-001 在测试保持通过的前提下清理 status readiness 投影

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. RestoreOutcome 与 Level 1 graph origin

- [x] 2.1 Red: UT-002 编写 Level 1 restore outcome 不使 index ready 的失败测试，并确认失败原因符合预期
- [x] 2.2 Green: UT-002 将 restore 返回值升级为结构化 outcome，并在 SQLite runtime meta 标记 `level1_restored_mirror`
- [x] 2.3 Refactor: UT-002 在测试保持通过的前提下清理 `index_graph_ready/runtime_mirror_ready` 边界

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. committed snapshot manifest.yaml

- [x] 3.1 Red: UT-003 编写 `manifest.yaml` 主记录测试，并确认旧 JSON 主记录仍存在导致失败
- [x] 3.2 Green: UT-003 新增 committed snapshot manifest 模型与 YAML 写读路径，不再写 `recovery-manifest.json`
- [x] 3.3 Refactor: UT-003 在测试保持通过的前提下统一 artifact loader 命名与路径 helper

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. restore guard 机器可读阻断原因

- [x] 4.1 Red: UT-004 编写锚点不一致时 restore outcome 带 reason 且不恢复 cache 的失败测试
- [x] 4.2 Green: UT-004 将 manifest / metadata / page / declared / knowledge guard 失败映射到 readiness reason
- [x] 4.3 Refactor: UT-004 在测试保持通过的前提下统一 reason 常量和 status 投影

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 5. query degraded 且禁用 graph 伪命中

- [x] 5.1 Red: UT-005 编写 index missing 时 query 不返回 graph/index hit 的失败测试
- [x] 5.2 Green: UT-005 在 query 中消费 `RuntimeReadiness`，index 不 ready 时跳过 graph/index query 并返回 degraded fallback
- [x] 5.3 Refactor: UT-005 在测试保持通过的前提下清理 query trust / answer mode 投影

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. RuntimeReadiness 公开合同, 2. RestoreOutcome 与 Level 1 graph origin | 1.1-1.3 / 2.1-2.3 / UT-001 / UT-002 |
| ST-002 | 2. RestoreOutcome 与 Level 1 graph origin | 2.1-2.3 / UT-002 |
| ST-003 | 3. committed snapshot manifest.yaml | 3.1-3.3 / UT-003 |
| ST-004 | 4. restore guard 机器可读阻断原因 | 4.1-4.3 / UT-004 |
| ST-005 | 5. query degraded 且禁用 graph 伪命中 | 5.1-5.3 / UT-005 |

## 执行顺序

- 先实现 `RuntimeReadiness`，让 status/query 共享主 DTO。
- 再实现 restore outcome 与 graph origin marker，避免 Level 1 被 facts ready 误判。
- 再迁移 manifest.yaml，因为 restore outcome 需要读取 snapshot 主记录。
- 然后补 restore guard reasons。
- 最后切 query degraded 路径，并跑完整验证。

## 暂缓事项

- query route DTO 全量重构暂缓到后续 child change。
- `.wiki/pages/**` 旧目录不处理。
