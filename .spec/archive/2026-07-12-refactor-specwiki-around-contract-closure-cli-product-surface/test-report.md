---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-cli-product-surface 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：自动化测试覆盖 proposal 成功标准、ST-001 至 ST-006 和关键失败路径，全部通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-13T02:28:00+08:00 |
| 执行环境 | Windows x64, Node 20.20.0, pnpm 10.6.3, Rust/Cargo, Vitest 3.2.4 |
| 测试方式 | 自动化测试、CLI E2E、distribution、静态扫描 |

## 验证范围

- CLI router/help/参数、机器协议、统一 init、治理 transport、宿主资产、文档迁移、发布 staging 和旧调用扫描。

## 部分范围

无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | Rust 全套 + Vitest 63 |
| 单元测试通过 | 全部 |
| 单元测试失败 | 0 |
| 系统测试总数 | 42 |
| 系统测试通过 | 42 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Cargo test / Vitest |
| 执行命令 | `pnpm run test` |
| 执行时间 | 约 129 秒完整链 |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | Rust 全套 + 63 Vitest |
| 通过 | 全部 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | Rust 77 library、14 acceptance、8 governance workflows、157 runtime、21 symbols 等全部通过；Vitest 63/63 | 统计和失败定位 |
| JUnit XML | 无 | 不适用 |
| Coverage | 无 | 未统计 |

### 单元测试失败项

无。

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001/002 | CLI tests | 13 tests | pass | router/help/参数 |
| UT-003/004/009 | parser/exit/stream/renderer tests | 26+ tests | pass | 协议与呈现 |
| UT-005 | bootstrap tests | 7 tests | pass | ready/partial/failed |
| UT-006/007/008 | Rust acceptance/governance + TS parser | Cargo/Vitest | pass | cli_init 与治理 |
| UT-010 | asset tests + scan gate | Vitest | pass | 迁移和 identity |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI E2E / distribution / project automation |
| 测试环境 | Windows x64 临时仓库，真实 built wiki-runtime |
| 工具 | Vitest / Cargo |
| verification mode | project automation / CLI/API |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `pnpm run test` |
| 动作摘要 | 构建 runtime/package，执行统一 init、status、update、query，验证 staging/help/scan gate |
| 断言点 | exit code、JSON/NDJSON terminal、runtime state、文件产物、help actions、旧调用扫描 |
| 证据路径 | 控制台摘要 |
| fallback 原因 | 无 UI/browser surface，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | CLI/文件/协议自动化断言 |
| fallback reason | 本 change 不包含 UI/browser 交互 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 42 |
| 通过 | 42 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | 一级命令与分层帮助 | 正常/异常 | pass | CLI/distribution |
| ST-002 | 参数、输出模式与退出码 | 边界 | pass | parser/stream/CLI |
| ST-003 | 统一 init 与 partial recovery | 正常/异常 | pass | bootstrap/Rust/E2E |
| ST-004 | 只读治理命令 | 正常/异常 | pass | governance/acceptance |
| ST-005 | 宿主 identity 与调用文本 | 正常 | pass | asset/API tests |
| ST-006 | 旧调用扫描 | 边界 | pass | cli-surface gate |

### 系统测试失败项

无。

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `pnpm run lint` | pass | ESLint 无错误 |
| `pnpm --filter spec-wiki exec tsc --noEmit` | pass | TypeScript 类型检查通过 |
| `git diff --check` | pass | 无 whitespace error |
| `pnpm run test` | pass | Rust、包测试、构建、E2E、distribution 全部通过 |
| `unispec validate refactor-specwiki-around-contract-closure-cli-product-surface` | pass | change valid |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | CLI Vitest + distribution help | pass | 默认/完整 help 与 legacy rejection |
| ST-002 | parser/stream/exit policy | pass | 0/2/64/1 与唯一终态 |
| ST-003 | bootstrap + Rust + E2E | pass | unified init 与 partial |
| ST-004 | governance workflows/acceptance | pass | typed envelope 和 errorKind |
| ST-005 | bootstrap/asset/API tests | pass | identity 保留 |
| ST-006 | static scan gate | pass | 当前产品面无旧调用 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 四命令默认主路径与一级 router | ST-001 | pass |
| query/host 参数与输出协议 | ST-002 | pass |
| 统一 init、partial、landing | ST-003 | pass |
| 治理只读命令和 DTO | ST-004 | pass |
| 宿主 identity 保留、调用迁移 | ST-005 | pass |
| 当前文档和代码清除旧调用 | ST-006 | pass |

## 失败项

无。

## 未验证项

无。

## 证据缺口

无。

## 下一步

- 使用 unispec-archive 归档当前 child change。
