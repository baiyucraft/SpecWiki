---
verification-result: pass
scope: full
---

# close-specwiki-3-0-design-baseline-product-contract 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：UT-001 至 UT-005、ST-001 至 ST-005、15/15 小 task 和 proposal 七条成功标准均有自动化证据覆盖，最终验证无失败、跳过或证据缺口。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | `2026-07-16T00:27:42.3488927+08:00` |
| 执行环境 | Windows / PowerShell；Node.js、pnpm 10.6.3、Vitest 3.2.4；仓库根目录；无浏览器或外部服务依赖 |
| 测试方式 | 自动化 TDD、工作区合同测试、完整项目测试链、lint 和 UniSpec 校验 |

## 验证范围

- Canonical baseline 的固定路径、设计 INDEX 与总体设计双入口和职责分离。
- 三类版本域的 authority、scope、显式 relation、manifest 对照和独立版本规则。
- 设计决策与 implementation / verification / release evidence 的正交状态合同。
- 单合同域和全项目两级设计完成规则。
- 有界材料分类、只读 archive 和 documentation-closure 延期边界。
- 仓库级 Rust runtime、主包、根测试、构建、E2E 和 distribution 回归。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 5 |
| 单元测试通过 | 5 |
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
| 测试框架 | Vitest 3.2.4 |
| 执行命令 | `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts` |
| 执行时间 | 最终聚焦执行约 2 秒；每次 Red / Green / Refactor 的单独耗时未作为归档证据保存 |
| 框架报告 | 控制台摘要；无独立 JUnit 或 coverage 文件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 5 |
| 通过 | 5 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计；以固定输入合同测试和 UT/ST/成功标准映射作为范围证据 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `scripts/tests/product-baseline-contract.test.ts`: 1 file / 5 tests passed；根 Vitest：14 files / 48 tests passed | 聚焦合同与工作区回归统计 |
| JUnit XML | 无 | 未生成独立 CI 报告 |
| Coverage | 无 | 本 change 验收固定 Markdown 合同和文件读取断言 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 最终执行无失败；五次 Red 均为预期合同缺口 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1-1.3 | 单用例 Red、聚焦 Green / Refactor | 缺页与双入口 Red；最终 canonical 用例通过 | pass | 固定路径、双入口、职责分离 |
| UT-002 / 2.1-2.3 | 单用例 Red、manifest 对照 Green / Refactor | authority / relation Red；最终版本域用例通过 | pass | 三域、显式关系、npm 与四 crate 事实，不做 lockstep |
| UT-003 / 3.1-3.3 | 单用例 Red、状态章节 Green / Refactor | 四维缺口 Red；最终正交状态用例通过 | pass | 四轴、evidenceRefs、合法组合、互不推导 |
| UT-004 / 3.4-3.6 | 单用例 Red、完成规则 Green / Refactor | 两级完成缺口 Red；最终完成规则用例通过 | pass | 单域条件、六 child、closure 门禁 |
| UT-005 / 4.1-4.3 | 单用例 Red、材料边界 Green / Refactor | 分类与 archive 规则 Red；最终材料用例通过 | pass | 固定材料、只读历史、延期全库迁移 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | 工作区级文档合同自动化和项目综合回归 |
| 测试环境 | 仓库根目录；固定 Markdown、JSON 和 Cargo manifest 输入 |
| 工具 | Vitest、ESLint、项目测试编排、UniSpec CLI |
| verification mode | project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 无 |
| 执行命令或动作 | 聚焦 Vitest、根 Vitest、`pnpm run lint`、`pnpm test`、`unispec validate` |
| 动作摘要 | 断言五组文档合同，再执行语言、构建、E2E、distribution 和 artifact 回归 |
| 断言点 | 文件、链接、authority、relation、manifest version、状态轴、完成规则、材料边界和 UniSpec artifact |
| 证据路径 | `scripts/tests/product-baseline-contract.test.ts` 和本报告记录的控制台摘要 |
| fallback 原因 | Playwright 不适用：本 change 无浏览器入口或交互 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | project automation |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | 固定文件、相对链接、文本 marker、manifest 值和 UniSpec 校验，均为非图片自动化断言 |
| fallback reason | 本 change 只修改 Markdown 合同和 Node 文件读取测试，无浏览器交互 |

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
| ST-001 | Canonical baseline 唯一且可达 | 正常 / 异常 | pass | UT-001 验证固定页面、双入口和职责边界 |
| ST-002 | 三类版本域具有独立 authority 和显式关系 | 正常 / 边界 | pass | UT-002 对照 npm 与四个 Cargo manifest |
| ST-003 | 设计决策与交付证据保持正交 | 正常 / 边界 | pass | UT-003 验证四维、互不推导和三个合法组合 |
| ST-004 | 单域与全项目完成规则不混淆 | 正常 / 边界 | pass | UT-004 验证两级证据集合和禁止替代规则 |
| ST-005 | 材料分类有界且历史 archive 只读 | 正常 / 边界 | pass | UT-005 验证固定类别、只读 archive 和延期责任 |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | ST-001 至 ST-005 全部通过 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| UT-001 至 UT-005 分别按 `-t` 执行预期 Red | pass | 均因对应合同缺失失败，不是语法、导入或环境错误 |
| `pnpm exec vitest run --config vitest.config.mjs scripts/tests/product-baseline-contract.test.ts` | pass | 1 file / 5 tests passed |
| `pnpm exec vitest run --config vitest.config.mjs` | pass | 14 files / 48 tests passed |
| `pnpm run lint` | pass | ESLint 退出码 0 |
| `pnpm test` | pass | Rust runtime 全套测试、主包 14 files / 85 tests、根 14 files / 48 tests、构建、E2E、distribution 全通过 |
| `unispec validate close-specwiki-3-0-design-baseline-product-contract` | pass | change valid，无 blocking issues |
| `tasks.md` checkbox 核验 | pass | 15/15 小 task、20/20 CheckList 完成 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | UT-001 + 聚焦 Vitest | pass | canonical 固定路径、双入口与职责分离 |
| ST-002 | UT-002 + manifest 对照 | pass | 三域 authority / scope、显式关系和独立版本规则 |
| ST-003 | UT-003 + 状态章节断言 | pass | decision 与三类 delivery evidence 正交 |
| ST-004 | UT-004 + 完成规则断言 | pass | 单域与全项目条件分离 |
| ST-005 | UT-005 + 固定材料断言 | pass | 有界分类、只读历史和延期迁移责任 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 唯一、可引用的 canonical product baseline，供后续五个 child 使用 | ST-001 / UT-001 | pass |
| 三类版本域及各自 authority、scope 和映射 | ST-002 / UT-002 | pass |
| 数值差异不自动构成漂移，只有 authority、scope 或显式 relation 冲突才是问题 | ST-002 / UT-002 | pass |
| 决策状态与交付证据正交，可表达三个代表性组合 | ST-003 / UT-003 | pass |
| 单域条件与六 child + 最终一致性门禁的全项目规则 | ST-004 / UT-004 | pass |
| 有界分类覆盖设计、capability、release、manifests、`.docs` 和只读 archive | ST-005 / UT-005 | pass |
| 验收不依赖全库迁移，收口由 documentation-closure 独立完成 | ST-005 / UT-005 | pass |

## 失败项

- 无。TDD Red 是实现前的预期失败证据，不是最终验证失败。

## 未验证项

- 无。Registry / tag / binary / staged publish evidence 与全库旧叙事迁移属于已确认非目标和后续 change，不是本次未验证的成功标准。

## 证据缺口

- 无。

## 下一步

- 使用 `unispec-archive` 归档当前 child。
