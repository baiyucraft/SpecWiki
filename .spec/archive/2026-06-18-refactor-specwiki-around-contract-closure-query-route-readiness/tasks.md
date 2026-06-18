---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-query-route-readiness 任务计划

## 任务总览

任务按 `design.md` 的能力块拆分：先固定 `wiki-model` query 公共合同，再在 `wiki-runtime` 接入 route fusion、readiness/trust gate、fallback/governance placeholder，最后收口 transport 与 TS parser guardrail。当前采用 TDD 模式：先写失败测试并确认 Red，再写最小实现，测试通过后重构。

## 实现模式

tdd

tdd：先写失败单元测试并确认失败，再写最小实现，通过后重构。每个 Red/Green/Refactor 小任务引用 `unit-tests.md` 的 `UT-*` 蓝图；本计划未获得用户确认前保持 `implementation-ready: false`。

## 1. Query 公共合同

- [x] 1.1 Red: UT-001 编写 `wiki-model` query DTO serde 与闭集枚举失败单元测试，并确认失败原因符合预期
- [x] 1.2 Green: UT-001 在 `wiki-model::domain::query` 编写最小 DTO / enum / serde 实现，使测试通过
- [x] 1.3 Refactor: UT-001 在测试保持通过的前提下清理 DTO 命名、导出边界和 helper

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 2. Runtime route fusion

- [x] 2.1 Red: UT-002 编写 runtime route fusion 失败单元测试，确认 `route_groups/results` 缺失或旧摘要主导
- [x] 2.2 Green: UT-002 在 `wiki-runtime` query workflow 中接入最小 route fusion，使 index/knowledge/projection 映射为公开 DTO
- [x] 2.3 Refactor: UT-002 在测试保持通过的前提下清理 fusion adapter，确保 `wiki-index` 不知道公开 route tag

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 3. Readiness/trust gate

- [x] 3.1 Red: UT-003 编写 index missing/stale/blocked 禁止 index route 的失败单元测试，并确认仍可观察 knowledge/projection 降级结果
- [x] 3.2 Green: UT-003 在 runtime query gate 中接入 readiness/trust/action 判定，使非 ready index 不输出 `index_*`
- [x] 3.3 Refactor: UT-003 在测试保持通过的前提下清理 readiness 判断和推荐动作命名

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 4. Fallback 与 governance placeholder

- [x] 4.1 Red: UT-004 编写 rendered page fallback 显式 route 和 trust 降级失败单元测试
- [x] 4.2 Green: UT-004 在 runtime query fallback 路径中输出 `rendered_page_debug_fallback` 并降级 trust
- [x] 4.3 Refactor: UT-004 在测试保持通过的前提下清理 fallback explanation，避免旧 `page_fallback` 成为主合同
- [x] 4.4 Red: UT-005 编写 governance `not_enabled` placeholder 不阻断普通 query 的失败单元测试
- [x] 4.5 Green: UT-005 在 runtime query response 中接入 governance readiness placeholder，不扫描 `.spec` evidence
- [x] 4.6 Refactor: UT-005 在测试保持通过的前提下清理 governance placeholder 边界，避免提前实现 governance scanner

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 5. Transport 与 TS parser guardrail

- [x] 5.1 Red: UT-006 编写 runtime transport 保留 query 主合同且不重建语义的失败单元测试
- [x] 5.2 Green: UT-006 调整 `wiki-runtime` transport payload，使 route groups/results/readiness/trust/action/source refs 从 rich report 透传
- [x] 5.3 Refactor: UT-006 在测试保持通过的前提下清理 transport-only 字段，保留 `provenance_summary` 为只读派生摘要
- [x] 5.4 Red: UT-007 编写 TS parser 闭集校验、未知字段保留和未知 route/ref kind 失败测试
- [x] 5.5 Green: UT-007 调整 `packages/spec-wiki` parser，只做结构校验和原样保留，不计算 trust/action
- [x] 5.6 Refactor: UT-007 在测试保持通过的前提下清理 Agent/Skill 消费侧类型引用，避免私造 route tag

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 6. 验证与文档收口

- [x] 6.1 执行 Rust 局部验证：`cargo test -p wiki-model --test query_contract` 与 `cargo test -p wiki-runtime query_sync_rebuild --test runtime`
- [x] 6.2 执行 runtime acceptance/transport 验证：`cargo test -p wiki-runtime query_transport_returns_slim_payload_but_internal_query_stays_rich --test acceptance`
- [x] 6.3 执行 TS parser 验证：`pnpm --filter spec-wiki test -- index.test.ts`
- [x] 6.4 执行全量可用验证：`pnpm run lint`、`pnpm run test`、`cargo test`
- [x] 6.5 更新必要的 `.wiki` 对外合同索引或模块指南，只沉淀稳定事实，不复制 proposal/design/review 原文
- [x] 6.6 运行 `unispec validate refactor-specwiki-around-contract-closure-query-route-readiness` 并准备后续 review/archive 输入

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（按项目可用命令执行：lint / typecheck / static analysis / formatter check / compiler check 等）
- [x] 注释规范检查完成（参考 .wiki/02-开发指南/00-代码注释规范.md）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 1. Query 公共合同, 2. Runtime route fusion | 1.1 / 1.2 / 1.3 / UT-001, 2.1 / 2.2 / 2.3 / UT-002 |
| ST-002 | 3. Readiness/trust gate | 3.1 / 3.2 / 3.3 / UT-003 |
| ST-003 | 4. Fallback 与 governance placeholder, 6. 验证与文档收口 | 4.1 / 4.2 / 4.3 / UT-004, 6.1 / 6.4 |
| ST-004 | 4. Fallback 与 governance placeholder | 4.4 / 4.5 / 4.6 / UT-005 |
| ST-005 | 5. Transport 与 TS parser guardrail, 6. 验证与文档收口 | 5.1 / 5.2 / 5.3 / UT-006, 5.4 / 5.5 / 5.6 / UT-007, 6.2 / 6.3 |

## 执行顺序

- 先执行 Task 1，固定 `wiki-model` DTO，否则 runtime/transport/TS 无稳定类型可依赖。
- 再执行 Task 2 与 Task 3，先接 route fusion，再收紧 readiness/trust gate。
- Task 4 在 runtime 主路径稳定后执行，明确 fallback 与 governance placeholder 的降级边界。
- Task 5 最后收口 transport 与 TS parser，确保接入层只消费 Rust 主合同。
- Task 6 作为实现完成后的验证和 Wiki 沉淀收口。

## 暂缓事项

- graph schema、GraphStore、IndexQueryStore、imports/calls/heritage 和 graph scoring 暂缓到 `code-graph-index`。
- `.spec` evidence scanner、governance summary、validate/archive 和治理状态机暂缓到 `governance-isolation`。
- CLI 默认 help、Quick Start、一级命令产品面暂缓到 `cli-product-surface`。
