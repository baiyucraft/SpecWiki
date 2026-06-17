---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-truth-restore-snapshot 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：所有关键 Rust、TypeScript、脚本和 UniSpec 验证均已通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-17T01:52:00+08:00 |
| 执行环境 | Windows / PowerShell / Cargo / pnpm / Vitest / UniSpec CLI |
| 测试方式 | 自动化测试 |

## 验证范围

- RuntimeReadiness status/query 公开合同。
- Level 1 restore outcome、graph origin marker、Level 2 graph ready。
- committed snapshot manifest YAML 与 metadata pointer。
- restore guard reason。
- query degraded fallback 与 graph hit 禁用。
- TS parser、lifecycle scripts、CLI/e2e contract。

## 部分范围

- 无

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | Cargo 全 workspace + package Vitest |
| 单元测试通过 | 全部通过 |
| 单元测试失败 | 0 |
| 系统测试总数 | ST-001 到 ST-005 |
| 系统测试通过 | 5 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Cargo test / Vitest |
| 执行命令 | `cargo test`; `pnpm --filter spec-wiki test`; `pnpm run test` |
| 执行时间 | 未记录 |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | Cargo 全 workspace、package Vitest 47、root Vitest 41 |
| 通过 | 全部通过 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `cargo test` 全部通过；`pnpm --filter spec-wiki test` 10 files / 47 tests 通过；`pnpm run test` root scripts 12 files / 41 tests 通过 | 验证 runtime、parser、脚本与 CLI 合同 |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 无 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1-1.3 | `cargo test -p wiki-runtime --test runtime` + TS parser tests | runtime 141 passed, package 47 passed | pass | status/query 公开 `readiness` |
| UT-002 / 2.1-2.3 | `status_restores_runtime_from_formal_artifacts_when_cache_is_missing`、`query_restores_runtime_cache_from_formal_artifacts` | runtime 141 passed | pass | Level 1 restore 不标 index ready |
| UT-003 / 3.1-3.3 | `init_persists_minimal_knowledge_artifacts`、artifact roundtrip tests | runtime 141 passed | pass | `manifest.yaml` 为 snapshot 主记录 |
| UT-004 / 4.1-4.3 | restore guard drift/mismatch tests | runtime 141 passed | pass | machine-readable reason 覆盖 |
| UT-005 / 5.1-5.3 | degraded query 与 graph/context tests | runtime 141 passed | pass | index missing 时禁用 graph 伪命中 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI/API/单元/集成自动化 |
| 测试环境 | 本地临时 repo fixture、SQLite cache、Cargo、pnpm |
| 工具 | Cargo test / Vitest / UniSpec CLI |
| verification mode | project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `cargo test -p wiki-runtime --test acceptance -- --nocapture`; `cargo test`; `pnpm run test`; `unispec validate refactor-specwiki-around-contract-closure-truth-restore-snapshot` |
| 动作摘要 | 自动化 fixture 初始化、restore、status、query、sync、update、rebuild、parser 与脚本断言 |
| 断言点 | readiness layer、manifest path、restore reason、query hits、recommended_action、TS parseResult、lifecycle preflight |
| 证据路径 | 控制台摘要 |
| fallback 原因 | 无 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | 本 change 不涉及 UI/browser；CLI/API/单元/集成自动化覆盖成功标准 |
| fallback reason | 不适用 |

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
| ST-001 | status/query 统一 RuntimeReadiness 公开合同 | 正常 / 边界 | pass | runtime tests、acceptance、TS parser tests |
| ST-002 | Level 1 restore 不标 index ready | 异常 / 边界 | pass | cache deletion restore tests |
| ST-003 | committed snapshot manifest.yaml 主记录 | 正常 | pass | artifact persistence tests |
| ST-004 | restore guard 机器可读阻断原因 | 异常 | pass | metadata/page/declared/knowledge mismatch tests |
| ST-005 | index missing query degraded 且无 graph 伪命中 | 边界 | pass | degraded query tests |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test -p wiki-runtime --test runtime -- --nocapture` | pass | 141 passed |
| `cargo test -p wiki-runtime --test acceptance -- --nocapture` | pass | 11 passed |
| `cargo test` | pass | 全 workspace 通过 |
| `pnpm --filter spec-wiki test` | pass | 10 files / 47 tests passed |
| `pnpm run test` | pass | workspace test、build、root Vitest、Cargo 子集全部通过 |
| `unispec validate refactor-specwiki-around-contract-closure-truth-restore-snapshot` | pass | Change is valid |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | Cargo runtime / acceptance / TS parser tests | pass | 公开 payload 与 parser contract 已覆盖 |
| ST-002 | Cargo runtime restore tests | pass | Level 1 / Level 2 readiness 已覆盖 |
| ST-003 | Cargo artifact tests | pass | YAML manifest 写读与旧 JSON 主记录移除已覆盖 |
| ST-004 | Cargo restore guard tests | pass | 多类 mismatch reason 已覆盖 |
| ST-005 | Cargo query tests | pass | degraded fallback 无 graph hits 已覆盖 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| status/query 公开 readiness 主合同 | runtime + acceptance + TS parser tests | pass |
| Level 1 restore 不再伪装 index ready | cache deletion restore tests | pass |
| Level 2 rebuild/update 才恢复 graph ready | runtime graph/query/update tests | pass |
| committed snapshot manifest YAML 为主记录 | artifact persistence tests | pass |
| restore guard reason 可机器读取 | mismatch tests | pass |
| index missing 时 query 不返回 graph/source/module/symbol/edge 伪命中 | degraded query tests | pass |

## 失败项

- 无

## 未验证项

- 无

## 证据缺口

- 无

## 下一步

- 在 review-report.md 通过后使用 `unispec archive refactor-specwiki-around-contract-closure-truth-restore-snapshot`。
