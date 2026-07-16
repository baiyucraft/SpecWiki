---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-reliability-lifecycle 任务计划

## 任务总览

任务按六个可独立验收能力块拆分：统一 reliability、provider failure、declared authority、projection governance、KnowledgeUnit resume，以及文档合同/全链验收。用户已明确授权无需阶段性审核并要求自动化 TDD、完整实现、review 和归档，因此规划产物互证后直接标记 implementation-ready。

## 实现模式

tdd

先为每个 `UT-*` 写入真实失败测试并确认失败来自待实现行为，再编写最小实现，最后在测试保持通过时重构并执行局部质量检查。

## 1. 统一 Runtime reliability authority

- [x] 1.1 Red: UT-001 编写 evidence优先级与freshness/consumability双轴失败测试并确认Red
- [x] 1.2 Green: UT-001 新增`ReliabilityAssessment`纯reducer并接入最小layer/action投影
- [x] 1.3 Refactor: UT-001 删除旧字符串preflight/action rank重复逻辑并保持测试通过
- [x] 1.4 Red: UT-002 编写route-local trust、governance正交与result/answer action失败测试并确认Red
- [x] 1.5 Green: UT-002 将status/query/workflow preflight统一接入assessment且保持canonical query payload
- [x] 1.6 Refactor: UT-002 删除query/status二次推导与局部特判并执行局部回归

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt/check/test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. A9 provider 与工具链失败矩阵

- [x] 2.1 Red: UT-003 编写stop reason × execution policy × output presence exhaustive失败测试并确认Red
- [x] 2.2 Green: UT-003 实现`ResearchOutcomeDecision` reducer和production有效output gate
- [x] 2.3 Refactor: UT-003 删除production structural-seed success双轨并保持development diagnostic显式
- [x] 2.4 Red: UT-004 编写`ProviderFailureKind`贯穿artifact/gate/status/action与原workflow retry失败测试并确认Red
- [x] 2.5 Green: UT-004 接入provider/LLM/page pipeline typed failure、checkpoint、terminal和exit语义
- [x] 2.6 Refactor: UT-004 收口context trim、tools fallback observation与exhaustive summary reason映射

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt/check/test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. A6 declared authority、删除与治理历史

- [x] 3.1 Red: UT-005 编写replacement graph、unique/none/conflict及非法关系失败测试并确认Red
- [x] 3.2 Green: UT-005 新增group-level authority evaluator和canonical edge/status校验
- [x] 3.3 Refactor: UT-005 将authority语义从storage派生迁入wiki-knowledge纯模块并接入query
- [x] 3.4 Red: UT-006 修改旧block removal断言为missing/detached保留record并确认Red
- [x] 3.5 Green: UT-006 实现authoring state、禁止prune、恢复binding和先deprecated再detach路径
- [x] 3.6 Refactor: UT-006 删除旧物理删除兼容行为并统一sync/update/restore传播
- [x] 3.7 Red: UT-007 编写conflict open/resolved/reopen/no-op事件历史失败测试并确认Red
- [x] 3.8 Green: UT-007 实现authority decisions、append-only governance events和snapshot验证
- [x] 3.9 Refactor: UT-007 保持current open view与history正交并完成same-facts传播回归

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt/check/test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. A8 projection governance 与原子回收

- [x] 4.1 Red: UT-008 编写required/budget/include/exclude/priority/hints deterministic policy失败测试并确认Red
- [x] 4.2 Green: UT-008 实现projection decision模型、配置schema和两步PagePlan入口
- [x] 4.3 Refactor: UT-008 删除默认one-unit-one-page并迁移change set/init/update/rebuild消费者
- [x] 4.4 Red: UT-009 编写manual/declared/manual-link保护和clean demotion失败测试并确认Red
- [x] 4.5 Green: UT-009 实现PageLinkRef、retiring reconciliation、protection preflight和managed link校验
- [x] 4.6 Refactor: UT-009 保证Runtime不越权修改manual内容且protected page不伪ready
- [x] 4.7 Red: UT-010 编写commit phase rollback/roll-forward和composite snapshot失败测试并确认Red
- [x] 4.8 Green: UT-010 实现repo-local lock/plan/staging/trash/pointer commit与幂等恢复
- [x] 4.9 Refactor: UT-010 接入sync/update/rebuild/restore，清理直接写/删半提交路径

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt/check/test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 5. KnowledgeUnit resume 与 provider session 边界

- [x] 5.1 Red: UT-011 编写action/facts/tree/contract resume identity变更失败测试并确认Red
- [x] 5.2 Green: UT-011 实现`PipelineResumeIdentity`并迁移checkpoint/summary/cache schema
- [x] 5.3 Refactor: UT-011 先planning后identity，删除旧schema/cache fallback
- [x] 5.4 Red: UT-012 编写完整research与draft/digest pair复用/半写拒绝失败测试并确认Red
- [x] 5.5 Green: UT-012 实现unit commit-point validators并统一full/scoped update resume
- [x] 5.6 Refactor: UT-012 从commit points重建gate/summary并统一公开workflow cleanup
- [x] 5.7 Red: UT-013 编写request-local session、当前unit重启与session不落盘失败测试并确认Red
- [x] 5.8 Green: UT-013 固定`session=None`恢复边界并补storage/注释守卫
- [x] 5.9 Refactor: UT-013 保留内部bounded session但删除durable/resume误导路径和表述

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（cargo fmt/check/test）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 6. 场景 authority、规模 fixture 与全链验收

- [x] 6.1 Red: UT-014 编写A1-A10分类、A4 authority和session/decomposition边界文档合同测试并确认Red
- [x] 6.2 Green: UT-014 更新Runtime/扩展场景/直接相关capability Wiki并加入机器可验收边界
- [x] 6.3 Refactor: UT-014 删除当前authority过度承诺，保留历史archive和documentation-closure边界
- [x] 6.4 Red: UT-015 编写32-64 unit第K中断、identity变化拒绝和公开workflow终态失败测试并确认Red
- [x] 6.5 Green: UT-015 完成resume/commit integration使计数、checkpoint、summary/gates和formal snapshot通过
- [x] 6.6 Refactor: UT-015 复用test support，删除仓库名特判、sleep和非确定性timeout
- [x] 6.7 执行ST-001至ST-013 focused验证、workspace full test、lint、fmt、check和UniSpec validate
- [x] 6.8 使用`unispec-review`生成review/test报告，修复全部blocker后重新验证
- [x] 6.9 使用`unispec-archive`同步`.wiki`与parent状态、归档change并提交一次

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（pnpm lint/test、cargo fmt/check/test、unispec validate）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1 | 1.1-1.6 / UT-001、UT-002 |
| ST-002 | 1、4 | UT-001、UT-010 / 6.7 |
| ST-003 | 3 | 3.1-3.3 / UT-005 |
| ST-004 | 3 | 3.4-3.9 / UT-006、UT-007 |
| ST-005 | 3、4 | UT-006、UT-007、UT-010 |
| ST-006 | 4 | 4.1-4.3 / UT-008 |
| ST-007 | 4 | 4.4-4.9 / UT-009、UT-010 |
| ST-008 | 2 | 2.1-2.6 / UT-003、UT-004 |
| ST-009 | 1、2 | UT-001-UT-004 / 6.7 |
| ST-010 | 5、6 | UT-011、UT-012、UT-015 |
| ST-011 | 5 | 5.7-5.9 / UT-013 |
| ST-012 | 6 | 6.1-6.3 / UT-014 |
| ST-013 | 4、5、6 | UT-010-UT-012、UT-015 / 6.7-6.9 |

## 执行顺序

- 先完成 task 1，建立所有 workflow 共用的状态/action authority。
- task 2 与 task 3 可在模型边界稳定后并行，但接入 status/query 时以 task 1 reducer为唯一入口。
- task 4 依赖 declared protection语义和reliability incomplete状态。
- task 5 依赖provider decision与projection page set稳定。
- task 6 最后执行文档合同、规模fixture、全量review、归档与commit。

## 暂缓事项

- PR/review ingestion、onboarding、richer monorepo impact、ACL/visibility、incident模型、hard purge、redirect、在线usage ranking、turn级checkpoint、跨机器session/cache和任意大仓SLA均不在本change。
