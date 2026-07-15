---
implementation-ready: true
---

# close-specwiki-3-0-design-baseline-runtime-query-contract 任务计划

## 任务总览

任务按共享查询对象与排名、Runtime fusion/state、transport/TS 协议、宿主消费、Wiki authority 五个能力块推进。用户已明确要求完整实现且无需审核，因此规划完成后直接标记可实现。

## 实现模式

tdd

先写失败测试并确认失败，再写最小实现，通过后重构；每组引用 `unit-tests.md` 的稳定编号。

## 1. 收口共享 Query DTO 与 route-local ranking

- [x] 1.1 Red: UT-001 编写 module route/ref、provenance/rank/group metadata 的失败合同测试
- [x] 1.2 Green: UT-001 在 `wiki-model` 实现最小闭集 DTO
- [x] 1.3 Refactor: UT-001 清理 serde/注释并保持 model tests 通过
- [x] 1.4 Red: UT-002 添加真实 SQLite BM25/source order 失败测试
- [x] 1.5 Green: UT-002 修正 `wiki-index` FTS 与结构化 fallback 排序
- [x] 1.6 Refactor: UT-002 保持 FTS store 次序并仅对结构化 fallback 使用确定性路径排序

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. 统一 Runtime fusion、ranking 与状态语义

- [x] 2.1 Red: UT-003 编写 module projection、group rank/count/truncation/dedup 失败测试
- [x] 2.2 Green: UT-003 实现 route group builder 与 module result projection
- [x] 2.3 Refactor: UT-003 收口 route policy，明确 internal flat results 非公开
- [x] 2.4 Red: UT-004 编写 stale/source/graph/fallback/governance provenance 矩阵失败测试
- [x] 2.5 Green: UT-004 从 layer readiness 投影 provenance/confidence/answer/action
- [x] 2.6 Refactor: UT-004 提取 route state/confidence helper 并移除硬编码状态

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. 统一 Rust transport 与 TS parser 主协议

- [x] 3.1 Red: UT-005 编写 canonical payload、空 groups、invalid term、index_not_ready 失败测试
- [x] 3.2 Green: UT-005 实现 query 专用 transport payload/encoder/error kind
- [x] 3.3 Refactor: UT-005 删除公开旧派生字段并保持 internal report 边界清楚
- [x] 3.4 Red: UT-006 编写 canonical payload/answer 严格解析与破坏性 fixture 失败测试
- [x] 3.5 Green: UT-006 实现 TS canonical types 和 fail-closed parser
- [x] 3.6 Refactor: UT-006 收口共享 parser helpers，不复制 Runtime 决策

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. 迁移 CLI、人类输出与 Agents 薄消费

- [x] 4.1 Red: UT-007 更新 CLI/renderer/三宿主资产测试，使旧字段当前实现失败
- [x] 4.2 Green: UT-007 迁移 human renderer、command assets 和 error parity 到 canonical contract
- [x] 4.3 Refactor: UT-007 去除宿主私有推导并统一共享字段清单

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. 发布长期 authority 与回归门禁

- [x] 5.1 Red: UT-008 新增 canonical Wiki authority/导航/延期能力失败合同测试
- [x] 5.2 Green: UT-008 新增 `06-Runtime查询合同.md` 并同步 Runtime/Agents/INDEX 摘要
- [x] 5.3 Refactor: UT-008 删除重复稳定定义，保留链接和责任边界
- [x] 5.4 更新 workspace E2E 只断言 canonical query payload
- [x] 5.5 运行 Rust/TS 聚焦测试、lint、全量 `pnpm test`、build/E2E/distribution 与 UniSpec validate

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 3、4、5 | UT-005、UT-006、UT-007、UT-008 |
| ST-002 | 3、4 | UT-005、UT-006、UT-007 |
| ST-003 | 1、2 | UT-001、UT-003、UT-004 |
| ST-004 | 1、2 | UT-001、UT-002、UT-003 |
| ST-005 | 2、3 | UT-004、UT-005、UT-006 |
| ST-006 | 4、5 | UT-007、UT-008 |

## 执行顺序

- 先完成共享 DTO 与真实 ranking，再改 Runtime 装配。
- Runtime 合同稳定后同步 transport/TS；随后迁移 CLI/Agents。
- Wiki authority 与 contract test 最后收口，并执行全量回归。

## 暂缓事项

- explicit intent/focus/scope/traversal、owner、entrypoint、impact、process/community 与 semantic/vector/LLM rerank，按 design 延期。
- capability/roadmap 全库历史迁移归 `documentation-closure`。
