---
verification-result: pass
scope: full
---

# build-specwiki-lite 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：自动化测试、发行合同、路径安全回归和独立 tarball 自举覆盖全部成功标准与 ST-001 至 ST-008 的能力路径，0 失败、0 证据缺口。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-28T02:05:13+08:00 |
| 执行环境 | Windows；Node.js 20.20.0；pnpm 10.6.3；无数据库、浏览器或外部服务 |
| 测试方式 | Vitest 自动化、静态门禁、发行包清单、独立 tarball CLI 自举 |

## 验证范围

- package/bin、六命令、Codex-only、asset ownership、Wiki 静态健康、`.spec` stage/artifact/archive、路径安全、旧 runtime 清零、公开文档与发行制品。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 46 |
| 单元测试通过 | 45 |
| 单元测试失败 | 0 |
| 系统测试总数 | 8 |
| 系统测试通过 | 8 |
| 系统测试失败 | 0 |
| 跳过 | 1（Windows file-symlink 条件测试） |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Vitest 3.2.4 |
| 执行命令 | `pnpm test`；聚焦 TDD 使用 package Vitest 命令 |
| 执行时间 | 最近完整 package run 4.66s；root contract run 1.81s |
| 框架报告 | 控制台摘要；未生成 JUnit/coverage 文件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 46 |
| 通过 | 45 |
| 失败 | 0 |
| 跳过 | 1 |
| 覆盖率 | 未统计；以行为与系统测试矩阵验收 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | package 6 files，45 passed / 1 skipped；root 2 files，4/4 passed | 统计与门禁证据 |
| JUnit XML | 无 | 不适用 |
| Coverage | 无 | 未要求阈值 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / Task 1 | `sync.test.ts` | 3 tests | pass | ownership、幂等、自定义页、Skills |
| UT-002 / Task 2 | `inspect.test.ts` | 3 tests | pass | INDEX/frontmatter/link/orphan/SSOT/path |
| UT-003/004 / Task 3 | `validate.test.ts` | 18 tests，1 条件跳过 | pass | stage、evidence、tasks、multi-change |
| UT-005/006 / Task 4 | `archive.test.ts` | 8 tests | pass | 原子归档、依赖、回滚、伪造 evidence |
| UT-007/008 / Task 5-6 | path/CLI/root contracts | 18 package/root tests | pass | path、命令、发行与 current surface |
| Task 7 | 完整门禁、status、tarball smoke | 全部通过 | pass | Wiki ready、Skills 8/8、发行与自举 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | project automation + CLI |
| 测试环境 | Windows 临时目录与仓库工作区 |
| 工具 | Vitest、TypeScript、ESLint、Vite、npm pack、installed CLI |
| verification mode | project automation / CLI |
| tool availability | available；浏览器工具 not-applicable |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | tests/lint/typecheck/build/pack；tarball install 后 init/status/validate/archive |
| 动作摘要 | 构建、打包、安装、初始化、strict validation、dated archive、active changes 清空 |
| 断言点 | 退出码/JSON、文件清单、Wiki issues、Skills 8/8、artifacts、archive target、surface scan |
| 证据路径 | `dist/spec-wiki-lite/spec-wiki-lite-0.1.0.tgz`；SHA-256 `5A8750BD694C942D100F184BE201C0CE96B0E47B5745AEFBEEC5F65FF4676C79` |
| fallback 原因 | 纯 Node CLI，无 UI/browser surface |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无；公开行为均由 CLI/文件系统断言覆盖 |
| evidence paths | 无浏览器附件 |
| assertion points | stdout/stderr、退出码、JSON、tarball inventory、文件系统状态 |
| fallback reason | 无网页、DOM 或浏览器交互 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 8 |
| 通过 | 8 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | 新包安装与公开命令 | 合同 | pass | 15 文件 tarball、独立安装、help/bin |
| ST-002 | Init/Update ownership 与 Codex-only | 正常/异常 | pass | asset/CLI integration tests |
| ST-003 | Wiki 静态健康 | 正常/异常 | pass | inspector tests、status ready |
| ST-004 | `.spec` stage/show/validate | 正常/异常 | pass | change core、JSON/exit tests |
| ST-005 | 普通/child/parent 归档 | 正常/异常 | pass | 8 archive tests |
| ST-006 | 路径安全失败关闭 | 安全/边界 | pass | path/init/Wiki/artifact regressions |
| ST-007 | 旧 runtime 当前合同清零 | 合同 | pass | current-surface 2/2 |
| ST-008 | 自举完整闭环 | 端到端 | pass | tarball `init -> status -> strict validate -> archive`；仓库自身目录移动在本报告后执行 |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `pnpm test` | pass | package 45/46、root 4/4；0 failed |
| `pnpm lint` | pass | exit 0 |
| root/package `tsc --noEmit` | pass | 两层 exit 0 |
| `pnpm run pack` | pass | 0.1.0、15 files、无 native binary |
| tarball install + `init/status/validate/archive` | pass | Wiki ready、Skills 8/8、strict valid、2026-07-28 archive |
| `git diff --check` | pass | 无 whitespace error |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | distribution + tarball | pass | package/bin/engine/inventory |
| ST-002 | integration | pass | ownership/Codex/idempotency |
| ST-003 | unit + status | pass | Wiki issues/health |
| ST-004 | core + CLI | pass | stage/artifact/evidence/show |
| ST-005 | archive integration | pass | single/multi/dependency/evidence |
| ST-006 | security regression | pass | lexical + realpath |
| ST-007 | contract scan | pass | old surface removed |
| ST-008 | installed CLI smoke | pass | equivalent full archive path |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| package/bin 身份且无 Rust binary | ST-001、tarball inventory | pass |
| 六命令且 Codex-only | ST-001/ST-002、CLI tests | pass |
| init/幂等/自定义页/Skills | ST-002、asset tests | pass |
| `.spec` stage/artifact/status/show/validate/archive | ST-004/ST-005/ST-008 | pass |
| Wiki 静态健康与路径安全 | ST-003/ST-006 | pass |
| current surface 无旧 runtime/index/knowledge | ST-007 | pass |
| tests/lint/build/pack/smoke/diff | 命令结果表 | pass |

## 失败项

- 无。

## 未验证项

- 真实 Linux/macOS 安装未在本 Windows 环境执行；proposal 已接受平台无关实现、无 `os/cpu` 限制与条件测试作为本 change 替代证据，正式发布前由跨平台 CI 复验。
- npm publish、tag、merge 明确不在本 change 范围。

## 证据缺口

- 无。

## 下一步

- 使用 built CLI strict validate `build-specwiki-lite`，通过后进入 `unispec-archive`。
