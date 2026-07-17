---
verification-result: pass
scope: full
---

# close-specwiki-3-0-design-baseline-host-trigger-contract 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：最终代码后的项目级自动化验证 exit 0，UT-001..UT-008、ST-001..ST-008 和全部成功标准均通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-17 |
| 执行环境 | Windows x64；Node.js；pnpm 10.6.3；Vitest；Rust/Cargo；构建后的 package dist |
| 测试方式 | 项目自动化、CLI/API、单元测试与静态合同验证 |

## 验证范围

- 共享 trigger taxonomy/policy/reducer、版本化双语 corpus、Codex-first host matrix、三宿主 guidance、CodeBuddy adapter/generated hook、bootstrap、distribution 与跨层 drift gate。

## 部分范围

- 无

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | package Vitest 108（21 files） |
| 单元测试通过 | 108 |
| 单元测试失败 | 0 |
| 系统测试总数 | 8 |
| 系统测试通过 | 8 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Vitest；Rust `cargo test` |
| 执行命令 | `pnpm test` |
| 执行时间 | 169s |
| 框架报告 | 控制台摘要；无独立 JUnit/Coverage 文件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | package 108；workspace 68；全部 Rust suites |
| 通过 | package 108；workspace 68；Rust 全部通过 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计；使用表驱动 corpus、invariant、可执行 hook 和跨层合同断言 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `pnpm test` exit 0 / 169s；package 108/108；workspace 68/68；Rust 全通过 | 最终全量证据 |
| JUnit XML | 无 | 无独立 CI XML |
| Coverage | 无 | 本 change 未配置覆盖率产物 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1-1.3 | trigger invariant/policy tests | package 108/108 | pass | 三态闭集和非法组合拒绝 |
| UT-002 / 1.4-1.6 | reducer/corpus table tests | package 108/108 | pass | NFKC、precedence、近碰撞、词/标点边界 |
| UT-003 / 2.1-2.3 | corpus schema/conformance | package 108/108 | pass | v1 版本和双语 cases |
| UT-004 / 2.4-2.6 | host/asset validator tests | package 108/108 | pass | 唯一 reference、能力与精确 settings command |
| UT-005 / 3.1-3.3 | shared/Codex/Claude/CodeBuddy assets | package 108/108 | pass | shared policy 和 canonical query fields |
| UT-006 / 3.4-3.6 | CodeBuddy adapter tests | package 108/108 | pass | 单字段 parser 与 fail closed |
| UT-007 / 4.1-4.3 | generated hook Node 子进程 | package 108/108 | pass | 唯一 JSON、无 CLI、source corpus parity |
| UT-008 / 4.4-4.6 | bootstrap/workspace/distribution | workspace 68/68；contract 4/4 | pass | cross-layer 与 dist drift gate |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | project automation / CLI/API / unit |
| 测试环境 | Windows x64 workspace；临时 bootstrap 目录；构建后的 `dist/index.js` 与生成 `.mjs` |
| 工具 | Vitest、Node 子进程、Cargo、Vite、ESLint、TypeScript、UniSpec CLI |
| verification mode | project automation / CLI/API / unit |
| tool availability | not-applicable（无 UI/browser） |
| 入口 URL | 无 |
| 浏览器 / viewport | 无 |
| 执行命令或动作 | `pnpm test`、build、定向 lint/tsc、diff-check、`unispec validate` |
| 动作摘要 | 构建三宿主资产，在临时目录 bootstrap CodeBuddy，并以 Node stdin 执行完整 corpus |
| 断言点 | role/capabilities、decision/reason/evidence、query fields、无 CLI、session 禁入、source/dist parity |
| 证据路径 | 无独立附件；采用控制台摘要、fixture 和自动化测试源码 |
| fallback 原因 | change 不含 UI/browser，无需 Playwright |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | project automation / CLI/API / unit |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | Vitest 状态/对象/文本/子进程输出断言覆盖全部 ST-* |
| fallback reason | change 不含 UI、页面、URL 或浏览器交互面 |

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
| ST-001 | Codex 唯一 reference host | 正常/边界 | pass | registry/asset/bootstrap/contract tests |
| ST-002 | 确定性三态 decision | 正常/异常/边界 | pass | contract invariant 与 reducer tests |
| ST-003 | 双语 corpus 跨宿主一致 | 正常/边界 | pass | 29 semantic + 6 adapter source/dist parity |
| ST-004 | 三宿主 guidance 服从 Runtime authority | 正常/边界 | pass | renderer/bootstrap tests |
| ST-005 | CodeBuddy 真实结构化包络 | 正常/异常 | pass | adapter/generated hook tests |
| ST-006 | Trigger 不执行 CLI | 异常/边界 | pass | 无 I/O 与唯一 context JSON 断言 |
| ST-007 | Trigger/bridge/provider session 正交 | 边界 | pass | session forbidden/drift contract |
| ST-008 | Bootstrap/发布/全量回归无漂移 | 正常/回归 | pass | full test、build、distribution、validate |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `pnpm test` | pass | 最终代码后 exit 0 / 169s；Rust、package、workspace、build、distribution 全通过 |
| `pnpm --dir packages/spec-wiki test` | pass | 21 files / 108 tests |
| `pnpm exec vitest run scripts/tests/host-trigger-contract.test.ts` | pass | 4/4；dist hook 完整 corpus |
| `pnpm --dir packages/spec-wiki build` | pass | Vite build 成功 |
| Agents/root contract 定向 ESLint | pass | 无错误 |
| Agents 17 files 与 root contract strict 定向 tsc | pass | change-owned TypeScript 零错误 |
| package/root 全量 tsc | baseline-only diagnostics | 仅既有无关 `parseResult.ts:632/680` 与 root 既有类型债务 |
| `git diff --check` | pass | 无 whitespace error |
| `unispec validate close-specwiki-3-0-design-baseline-host-trigger-contract` | pass | artifact 校验通过 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | registry/asset/bootstrap + contract | pass | 唯一 Codex reference 与 capability matrix |
| ST-002 | contract table/invariant tests | pass | 三态、precedence、fail closed |
| ST-003 | corpus + source/dist executable parity | pass | 29 semantic + 6 adapter |
| ST-004 | renderer/bootstrap tests | pass | shared policy 与 canonical DTO |
| ST-005 | adapter + generated `.mjs` | pass | official envelope、SessionStart、唯一 JSON |
| ST-006 | 无 I/O/CLI 静态与子进程断言 | pass | negative/ambiguous/invalid 不执行 |
| ST-007 | workspace cross-layer contract | pass | session/bridge identity 不进入 trigger |
| ST-008 | full tests/build/distribution/lint/tsc | pass | bootstrap 与发布无漂移 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 单一 trigger taxonomy/decision authority | UT-001/002；ST-002 | pass |
| 机器可消费 host role/capability matrix | UT-004；ST-001/008 | pass |
| 三宿主 decision/target/reason 可比较 | UT-003/005；ST-003/004 | pass |
| 中英文三态、近碰撞、malformed 表驱动覆盖 | 29 semantic + 6 adapter；ST-002/003/005 | pass |
| negative/ambiguous 无 CLI，mutation explicit-only | UT-001/002/006/007；ST-006 | pass |
| query 只消费 canonical fields | UT-005/008；ST-004/007 | pass |
| Runtime 结论不被宿主改写 | query assets + full regression；ST-004/007 | pass |
| CodeBuddy 无私有关键词 authority并 fail closed | UT-006/007；ST-005 | pass |
| trigger/bridge/request-local provider session 正交 | UT-008；ST-007 | pass |
| 新增/变更宿主通过 capability/corpus/drift gate | UT-003/004/008；ST-001/008 | pass |

## 失败项

- 无。全量 tsc 的既有诊断不属于本 change；change-owned strict 定向检查全部通过。

## 未验证项

- 无。外部宿主模型开放域选 skill 准确率、production research bridge 与多轮 agent-session bridge 属于 proposal 明确非目标。

## 证据缺口

- 无

## 下一步

- 在正式 review 同为 full/pass 后使用 `unispec-archive`。
