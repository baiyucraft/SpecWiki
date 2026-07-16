---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-core-scenario-acceptance 任务计划

## 任务总览

任务按 6 个可验证能力块拆分：场景 authority、Acceptance Plan/Gate Kernel v2、脚本 adapters/orchestrator、Runtime 场景证据、A/B restore 和长期 Wiki/capability authority。每个能力块按 Red -> Green -> Refactor 执行，并在完成时运行对应 ST、局部质量检查和注释规范检查。

## 实现模式

tdd

用户已明确要求自动化 TDD 并授权无需逐阶段审核，因此规划产物完成后可直接进入实现。

## 1. 建立 9 场景 canonical authority

- [x] 1.1 Red: UT-001 写入 matrix 闭集、9/9、支持等级、evidence 和 deferred/query 边界失败测试，并确认目标 module 缺失的预期失败
- [x] 1.2 Green: UT-001 创建最小 `core-scenario-acceptance.mjs` matrix/validator，使 ST-001、ST-002 通过
- [x] 1.3 Refactor: UT-001 收口闭集、稳定排序和 evidence path helper，保持 matrix tests 通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（Vitest + ESLint）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. 建立 Acceptance Plan 与 Gate Kernel v2

- [x] 2.1 Red: UT-002 写入显式 acceptance plan、深冻结和禁止历史样本硬编码的失败测试
- [x] 2.2 Green: UT-002 实现最小 plan factory/validator，使 ST-003 的 plan 合同通过
- [x] 2.3 Refactor: UT-002 集中 plan/gate 闭集并删除隐式 sample 默认值，保持测试通过
- [x] 2.4 Red: UT-003 写入单 failure 唯一 owner、无关 gate 不污染和非法 ownership 失败测试
- [x] 2.5 Green: UT-003 升级 `quality-gates.mjs` 为 failure-aware gate 聚合，使 ST-004 通过
- [x] 2.6 Refactor: UT-003 提取稳定 failure normalization/dedup，移除总失败数批量 gate 决策
- [x] 2.7 Red: UT-004 写入 not_covered/incomplete/diagnostic 优先级和 0/1/2 退出表失败测试
- [x] 2.8 Green: UT-004 实现 overall aggregation、exit policy 和 report-only，使 ST-005、ST-006 通过
- [x] 2.9 Refactor: UT-004 删除 v1 `covered`/blocking fallback，统一 v2 summary invariant

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（Vitest + ESLint）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. 迁移脚本 adapters 并建立统一 orchestrator

- [x] 3.1 Red: UT-005 写入 lifecycle 单 failure owner、短路 not_covered 和无关 gate 不污染失败测试
- [x] 3.2 Green: UT-005 迁移 lifecycle/project-set/reference adapters 到共享 Gate Kernel，使 ST-007 通过
- [x] 3.3 Refactor: UT-005 清理三类脚本重复 decision/exit 和硬编码 primary samples，保持现有脚本 tests 通过
- [x] 3.4 Red: UT-006 写入 9/9 orchestrator summary、默认/report-only 子进程退出失败测试
- [x] 3.5 Green: UT-006 创建离线可测 `run-core-scenario-acceptance.mjs`，使 ST-006、ST-014 通过
- [x] 3.6 Refactor: UT-006 统一 fixture/真实 adapter 调用链、稳定排序和 JSON 输出，保持 script regression 通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（Vitest + ESLint）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. 补齐 Runtime 核心场景行为证据

- [x] 4.1 Red: UT-007 注册 Rust core scenarios 模块，写入 CS-01..05 公开 transport/init/update/query/graph 聚合失败测试
- [x] 4.2 Green: UT-007 复用现有 workflows/fixtures 并补最小主链修正，使 ST-008、ST-009 通过
- [x] 4.3 Refactor: UT-007 提取动态仓库 helper，拒绝旧 query/internal DTO 并保持 Rust tests 通过
- [x] 4.4 Red: UT-008 写入 pitfall/policy/convention、非法/重复 marker 和结构化 conflict create/query/clear 失败测试
- [x] 4.5 Green: UT-008 补最小 declared/governance 主链修正，使 ST-010、ST-011 通过
- [x] 4.6 Refactor: UT-008 复用 record/conflict helper，保持 authoring/语义检测非目标和 Rust tests 通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt + 聚焦 cargo test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 5. 建立 A/B 正式产物 restore 验收

- [x] 5.1 Red: UT-009 写入双工作副本、排除 cache、level1/readiness/rebuild 和无 index route 失败测试
- [x] 5.2 Green: UT-009 补最小 restore/status/query 修正，使 ST-012 通过
- [x] 5.3 Refactor: UT-009 抽取 formal artifact copy/helper，原样保留 manifest 且严格排除派生层
- [x] 5.4 Red: UT-010 写入 B 页面漂移 rebuild 与源码漂移 needs_update/update 失败测试
- [x] 5.5 Green: UT-010 修正必要的 drift 分类/恢复顺序，使 ST-013 通过
- [x] 5.6 Refactor: UT-010 收口 drift assertion/helper，不降低 hash 或状态断言并保持 restore regression 通过

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt + 聚焦 cargo test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 6. 收口 Wiki 与 capability authority

- [x] 6.1 Red: UT-011 写入核心场景等级、gate v2、脚本职责、canonical query 和旧冲突文案失败合同测试
- [x] 6.2 Green: UT-011 更新核心场景、测试/脚本指南和 workflow-verification 当前 authority，使 ST-015 通过
- [x] 6.3 Refactor: UT-011 删除重复定义、保持链接/SSOT 边界并确认 archive 未修改

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（Vitest + ESLint + Markdown fixed-path contract）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001、ST-002 | 1. 场景 authority | 1.1-1.3 / UT-001 |
| ST-003 | 2. Plan/Gate Kernel | 2.1-2.3 / UT-002 |
| ST-004 | 2. Plan/Gate Kernel | 2.4-2.6 / UT-003 |
| ST-005、ST-006 | 2. Plan/Gate Kernel | 2.7-2.9 / UT-004 |
| ST-007 | 3. 脚本迁移 | 3.1-3.3 / UT-005 |
| ST-014 | 3. Orchestrator | 3.4-3.6 / UT-006 |
| ST-008、ST-009 | 4. Runtime 场景 | 4.1-4.3 / UT-007 |
| ST-010、ST-011 | 4. Declared/conflict | 4.4-4.6 / UT-008 |
| ST-012 | 5. A/B restore | 5.1-5.3 / UT-009 |
| ST-013 | 5. Drift | 5.4-5.6 / UT-010 |
| ST-015 | 6. Wiki/capability | 6.1-6.3 / UT-011 |

## 执行顺序

- 先完成 Task 1，再完成 Task 2；matrix/plan/gate identity 是所有 adapters 和报告的输入。
- Task 3 依赖 Task 2；Task 4 可在 Task 1 后推进，但最终 orchestrator 证据依赖 Task 3/4。
- Task 5 复用 Task 4 的 Rust fixture helper。
- Task 6 在源码和行为合同稳定后收口长期 authority。
- 所有局部门禁完成后执行 full workspace test、lint、fmt、build、UniSpec validate，再进入 full review/verification。

## 暂缓事项

- 宿主 trigger parity、专用 declared authoring UX、自然语言规则-vs-code 语义检测、richer query 和完整 reliability lifecycle 按 proposal 留给后续 child/change。
- 不运行需要网络或外部 reference 仓库的额外样本作为当前 TDD 单元门禁；最终 verification 执行仓库现有全量自动化。
