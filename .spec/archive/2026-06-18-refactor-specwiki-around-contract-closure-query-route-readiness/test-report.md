---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-query-route-readiness 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：Rust、TS、workspace test、lint 与 UniSpec validate 均已通过；query route/readiness 合同的成功标准全部覆盖。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-18T09:53:25.3401032+08:00 起 |
| 执行环境 | Windows / PowerShell；本地工作区 `E:\project\!byAI\spec-wiki` |
| 测试方式 | 自动化测试 + CLI/transport 合同验证 + UniSpec validate |

## 验证范围

- `wiki-model` query DTO、route tag、ref kind、serde 闭集合同。
- `wiki-runtime` query fusion、declared/derived/projection 分层、readiness gate、fallback、governance placeholder、transport payload。
- `packages/spec-wiki` TS parser guardrail。
- `.wiki` 对外 query 合同沉淀与 `.spec` artifact 一致性。

## 部分范围

- 无

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | Rust workspace 多 crate 全量通过；`packages/spec-wiki` 48 个；workspace scripts 41 个 |
| 单元测试通过 | 全部通过 |
| 单元测试失败 | 0 |
| 系统测试总数 | 5 |
| 系统测试通过 | 5 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust cargo test；Vitest / pnpm test；ESLint；UniSpec validate |
| 执行命令 | 见“测试命令和结果” |
| 执行时间 | 未逐项记录；最终重跑均完成 |
| 框架报告 | 控制台摘要，无 JUnit / coverage artifact |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | `packages/spec-wiki` 48；workspace scripts 41；Rust workspace 全量通过 |
| 通过 | 全部通过 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `cargo test` 全量通过 | Rust workspace gate |
| 控制台摘要 | `pnpm run lint` 通过 | ESLint gate |
| 控制台摘要 | `pnpm run test` 通过 | TS/package/workspace gate |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 未统计覆盖率 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / Task 1 | `cargo test -p wiki-model --test query_contract` | `crates/wiki-model/tests/query_contract.rs` | pass | DTO serde 与闭集 route/ref kind。 |
| UT-002 / Task 2 | `cargo test -p wiki-runtime query_sync_rebuild --test runtime` | `query_fusion_outputs_route_groups_and_results` | pass | route groups/results fusion。 |
| UT-003 / Task 3 | `cargo test -p wiki-runtime query_sync_rebuild --test runtime` | `query_does_not_emit_index_routes_when_index_is_not_ready` | pass | 非 ready index 不输出 index route。 |
| UT-004 / Task 4 | `cargo test -p wiki-runtime query_sync_rebuild --test runtime` | fallback/trust runtime tests | pass | `rendered_page_debug_fallback` 和降级 trust。 |
| UT-005 / Task 4 | `cargo test -p wiki-runtime query_sync_rebuild --test runtime` | `query_reports_governance_not_enabled_without_blocking_query` | pass | governance not_enabled placeholder。 |
| UT-006 / Task 5 | `cargo test -p wiki-runtime query_transport_returns_slim_payload_but_internal_query_stays_rich --test acceptance` | `command_contract::query_transport_returns_slim_payload_but_internal_query_stays_rich` | pass | transport payload 保留主合同。 |
| UT-007 / Task 5 | `pnpm --filter spec-wiki test -- index.test.ts` | `parseResult parses query payload readiness contract`、`parseResult rejects unknown query route tags` | pass | TS parser 闭集校验与 extras 保留。 |
| Task 6 | `cargo test`、`pnpm run lint`、`pnpm run test`、`unispec validate ...` | 全量门禁 | pass | 实现、文档和 artifact 收口。 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI/API / unit / project automation |
| 测试环境 | 本地 Windows 工作区，无浏览器或外部服务依赖 |
| 工具 | cargo、pnpm、ESLint、Vitest、UniSpec CLI |
| verification mode | CLI/API + unit + project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | 见“测试命令和结果” |
| 动作摘要 | 执行 Rust/TS/Workspace/UniSpec 验证命令，并修正 stale test target 后重跑真实测试 |
| 断言点 | JSON 字段、route tag 闭集、declared route、readiness/trust/action、transport payload、parser 错误处理、UniSpec validate |
| 证据路径 | 代码与测试文件；无独立日志附件 |
| fallback 原因 | 本 change 不涉及 UI/browser 交互，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | 本 change 由 CLI/API/单元测试覆盖，无浏览器断言点 |
| fallback reason | 非 UI/browser change |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 5 |
| 通过 | 5 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | Query JSON 暴露闭集 route tag 与统一 result DTO | 正常 / 合同 | pass | DTO serde、runtime query、transport、TS parser tests。 |
| ST-002 | Readiness/trust gate 阻止 missing/stale/blocked index 伪命中 | 边界 / 异常 | pass | `query_does_not_emit_index_routes_when_index_is_not_ready`。 |
| ST-003 | Markdown fallback 只能作为显式 debug fallback | 边界 / fallback | pass | runtime fallback/trust tests。 |
| ST-004 | Governance 未启用不阻断普通 query | 边界 / placeholder | pass | `query_reports_governance_not_enabled_without_blocking_query`。 |
| ST-005 | Transport 与 TS parser 只消费主合同，不重建语义 | 合同 / parser | pass | acceptance transport test 与 TS parser tests。 |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test -p wiki-model --test query_contract` | pass | 2 passed |
| `cargo test -p wiki-runtime query_emits_declared_knowledge_route_for_declared_records --test runtime` | pass | 1 passed |
| `cargo test -p wiki-runtime query_sync_rebuild --test runtime` | pass | 20 passed |
| `cargo test -p wiki-runtime query_transport_returns_slim_payload_but_internal_query_stays_rich --test acceptance` | pass | 1 passed |
| `pnpm --filter spec-wiki test -- index.test.ts` | pass | 11 passed |
| `cargo test` | pass | Rust workspace 全量通过 |
| `pnpm run lint` | pass | ESLint 通过 |
| `pnpm run test` | pass | `packages/spec-wiki` 10 files / 48 tests；workspace scripts 12 files / 41 tests；Rust runtime gates 通过 |
| `unispec validate refactor-specwiki-around-contract-closure-query-route-readiness` | pass | Change is valid |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | Rust DTO serde + runtime query + transport + TS parser | pass | `route_groups`、`results`、DTO 字段和闭集 route tag 均覆盖。 |
| ST-002 | runtime query tests | pass | 非 ready index 不输出 index route，trust/action 显式降级。 |
| ST-003 | runtime fallback tests | pass | fallback route 显式为 `rendered_page_debug_fallback`。 |
| ST-004 | runtime governance placeholder tests | pass | governance 未启用时返回 `not_enabled`，普通 query 不阻断。 |
| ST-005 | transport acceptance + TS parser tests | pass | transport 透传主合同，parser 拒绝未知 route tag/ref kind。 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| Query JSON 能稳定观察正式 route tag | DTO、runtime、transport、TS parser tests | pass |
| 每条 query result 携带统一 DTO 字段 | `query_contract.rs`、runtime/TS tests | pass |
| 不同 route 的 score 默认以 group/route 表达 | runtime `route_groups` tests | pass |
| index missing/stale/blocked 时不返回 graph 伪命中 | `query_does_not_emit_index_routes_when_index_is_not_ready` | pass |
| Markdown fallback 标记为 `rendered_page_debug_fallback` 并降级 trust | runtime fallback tests | pass |
| knowledge declared、knowledge derived、projection ref 和 governance route 分层表达 | `query_emits_declared_knowledge_route_for_declared_records`、runtime/transport/parser tests | pass |
| governance 未启用返回 `not_enabled` 或等价状态且不阻断 query | `query_reports_governance_not_enabled_without_blocking_query` | pass |
| 人类输出能解释命中原因、来源、验证位置和下一步动作 | answer/trust/action runtime tests 与 command contract | pass |
| 后续 code graph 只需填充 index adapter，不重定义 DTO | `wiki-model` DTO + parser guardrail tests | pass |
| 后续 governance 只接 refs，不重定义 route/readiness/trust | governance placeholder tests + closed route contract | pass |

## 失败项

- 无。曾发现旧 artifact 中 `--test command_contract` / `query_route_contract` 命令不可复现，已改为当前真实 test target 并重跑通过。

## 未验证项

- 无

## 证据缺口

- 无

## 下一步

- `review-report.md` 已通过后，使用 `unispec-archive` 进入归档。
