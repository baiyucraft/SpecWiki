---
verification-result: pass
scope: full
---

# close-specwiki-3-0-design-baseline-core-scenario-acceptance 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：ST-001 至 ST-015 全部通过，proposal 10 条成功标准均有自动化证据，最终全量测试和静态门禁全部通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-16T11:45:00+08:00 |
| 执行环境 | Windows 11 x64；Node 20.20.0；pnpm 10.6.3；Rust 1.94.1；本地临时仓库与真实 SQLite；无网络/外部服务 |
| 测试方式 | TDD Red-Green-Refactor + Vitest + Rust unit/integration/acceptance + CLI 子进程 + fixed-path contract + 静态门禁 |

## 验证范围

- 9 条 canonical scenario matrix、Acceptance Plan、Gate Kernel v2、三类 adapter、核心场景 orchestrator。
- Runtime `init/status/query/update/sync/rebuild`、declared/governance 当前边界、A/B formal artifact restore 与 drift。
- Wiki/capability authority、workspace 回归、构建、lint、formatter、diff 和 UniSpec consistency。

## 部分范围

无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 多 runner 分项：Rust unit 77、acceptance 22、runtime integration 159、symbols 21、package Vitest 97、root Vitest 63，另有 governance/archive/hierarchy suites |
| 单元测试通过 | 所有已执行用例通过 |
| 单元测试失败 | 0 |
| 系统测试总数 | 15 |
| 系统测试通过 | 15 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Vitest 3.2.4；Rust test harness；pnpm workspace build/lint |
| 执行命令 | `pnpm test`；聚焦 Vitest 5 文件；`cargo test -p wiki-runtime --test acceptance core_scenario -- --nocapture` |
| 执行时间 | 最终 `pnpm test` 129.2s；聚焦 Vitest约 2.7s；聚焦 Rust acceptance约 7.7s |
| 框架报告 | 控制台摘要；未生成 JUnit XML 或 coverage artifact |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 分项见测试结果汇总 |
| 通过 | 全部 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计；以 UT/ST 行为矩阵和全量回归验收 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 全量控制台摘要 | `pnpm test`: Rust 全 suites、package 97/97、root 18 files/63 tests、build 通过 | 全量回归与构建证据 |
| 聚焦 Vitest | matrix/gate/CLI/Wiki/reference 共 5 files/17 tests | ST-001..007、ST-014、ST-015 |
| 聚焦 Rust | core scenario 4/4，18 filtered out | ST-008..013 |
| JUnit XML | 无 | 未配置 |
| Coverage | 无 | proposal/design 未要求百分比门禁 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001..002 / Task 1、2.1-2.3 | matrix/plan contract | matrix contract 2/2 | pass | 9/9、闭集、深冻结、gate role/companions/thresholds |
| UT-003..004 / Task 2.4-2.9 | Gate 纯函数与退出策略 | quality-gates 8/8、CLI 4/4 | pass | single-owner、non-owner、required coverage、0/1/2/report-only |
| UT-005..006 / Task 3 | adapter/snapshot/CLI | reference snapshot 2/2 + 上述 tests | pass | 三类 adapter、plan 阈值、9/9 orchestrator |
| UT-007..008 / Task 4 | Rust public transport/declared/governance | core scenario acceptance | pass | CS-01..08 当前公开边界 |
| UT-009..010 / Task 5 | Rust A/B restore/drift | core scenario acceptance | pass | level1/rebuild、page/source drift |
| UT-011 / Task 6 | fixed-path Wiki contract | Wiki contract 1/1 | pass | 场景与 Gate v2 authority |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | project automation + CLI/API + Rust integration/acceptance + fixed-path contract |
| 测试环境 | 动态临时仓库、真实 SQLite、离线 fixture plan；无外部 reference 仓库 |
| 工具 | Vitest、Rust test harness、pnpm scripts |
| verification mode | project automation / CLI/API / unit |
| tool availability | available；browser not-applicable |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | 全量 `pnpm test`、聚焦 Vitest/Rust、lint/fmt/diff/validate |
| 动作摘要 | 构造正常/非法 matrix/plan/gate；运行 CLI 子进程；创建动态仓库和 A/B 工作副本；解析固定 Wiki 路径 |
| 断言点 | decision/exit、failure owner、canonical DTO、formal artifacts、runtime readiness/action、hash/drift、Markdown identity |
| 证据路径 | `scripts/tests/*.test.ts`、`crates/wiki-runtime/tests/acceptance/core_scenario_acceptance.rs`、控制台摘要 |
| fallback 原因 | 本 change 无 UI/browser/URL 交互，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | project automation / CLI/API / unit |
| host Playwright availability | not-applicable |
| manual verification | 无；全部验收项由非浏览器自动化覆盖 |
| evidence paths | 无浏览器附件；采用测试源码和控制台结果 |
| assertion points | process exit/JSON、runtime DTO/state、filesystem/hash、SQLite、Markdown 文本合同 |
| fallback reason | proposal/system-tests 明确无 UI/browser 范围 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 15 |
| 通过 | 15 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001..003 | matrix、支持边界、plan/gate registry | 合同/边界 | pass | matrix/plan contract |
| ST-004..007 | failure ownership、coverage/diagnostic、exit、adapters | 异常/集成 | pass | quality-gates、CLI、reference snapshot |
| ST-008..009 | init/update 与 canonical query/graph | 集成 | pass | Rust public transport acceptance |
| ST-010..011 | declared knowledge 与 governance conflict | 集成/异常 | pass | Rust declared/governance acceptance |
| ST-012..013 | A/B cold restore 与 drift | 集成/异常 | pass | Rust restore/drift acceptance |
| ST-014 | 9/9 orchestrator summary | 集成/异常 | pass | CLI 4/4 |
| ST-015 | Wiki/capability authority | 合同 | pass | fixed-path Wiki contract + validate |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `pnpm test` | pass | exit 0，129.2s；Rust/TS/build 全通过 |
| 聚焦 Vitest 5 文件 | pass | 5 files/17 tests |
| `cargo test -p wiki-runtime --test acceptance core_scenario -- --nocapture` | pass | 4/4 |
| `pnpm lint` | pass | exit 0 |
| `cargo fmt --all -- --check` | pass | exit 0 |
| `git diff --check` | pass | exit 0；仅 LF/CRLF working-tree warning，无 whitespace error |
| `unispec validate close-specwiki-3-0-design-baseline-core-scenario-acceptance` | pass | change valid |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001..007 | Vitest contract/adapter/CLI/snapshot | pass | matrix、plan、gate、exit 与 adapters |
| ST-008..013 | Rust acceptance | pass | Runtime 场景、declared/governance、restore/drift |
| ST-014..015 | CLI + fixed-path Wiki contract | pass | 9/9 summary 与 authority |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 1. 9/9 matrix 字段完整 | ST-001、ST-014 | pass |
| 2. supported/degraded/deferred 边界 | ST-002、ST-009..011 | pass |
| 3. gate level/scope/decision/blocking/companions 固定 | ST-003、ST-007 | pass |
| 4. failure identity 单 owner | ST-004 | pass |
| 5. diagnostic/skip 不伪 pass，exit 一致 | ST-005..007 | pass |
| 6. fixtures/guards/companions 显式声明 | ST-003、ST-007、ST-014 | pass |
| 7. canonical query only | ST-009、ST-015 | pass |
| 8. CS-06/07/08 结构化边界 | ST-010、ST-011 | pass |
| 9. CS-09 formal-only A/B restore/drift | ST-012、ST-013 | pass |
| 10. artifacts/实现/review/verification 共用 matrix/gate evidence | ST-014、ST-015 与两份正式报告 | pass |

## 失败项

无。

## 未验证项

- 未运行网络或真实外部 reference 项目；这是明确非门禁范围，reference adapter、plan thresholds 和 snapshot 已自动验证。
- UI/browser、宿主 trigger parity、专用 authoring UX、自然语言语义冲突、richer query 和完整 reliability lifecycle 为明确非目标或后续 child 范围。

## 证据缺口

无。未生成 JUnit/coverage/浏览器附件，但所有成功标准和 ST 均有可复跑的自动化断言。

## 下一步

- review report 同为 full/pass，使用 `unispec-archive`。
