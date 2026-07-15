---
verification-result: pass
scope: full
---

# close-specwiki-3-0-design-baseline-runtime-query-contract 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：UT-001 至 UT-008、ST-001 至 ST-006 和 proposal 成功标准均由自动化 Rust/Vitest/CLI E2E/contract tests 覆盖，未发现失败或证据缺口。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-16T01:57:00+08:00 |
| 执行环境 | Windows x64；Rust stable 1.94；Node/pnpm workspace；真实临时 SQLite FTS；无网络或外部服务 |
| 测试方式 | 自动化单元、集成、CLI E2E、distribution 和固定路径合同测试 |

## 验证范围

- Rust query DTO/index/runtime/transport，TypeScript parser/CLI/human renderer/Agents，Wiki authority、workspace lifecycle、打包分发和 UniSpec consistency。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 聚焦命令汇总 138 |
| 单元测试通过 | 138 |
| 单元测试失败 | 0 |
| 系统测试总数 | 6 |
| 系统测试通过 | 6 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust cargo test + Vitest |
| 执行命令 | model/index/runtime/governance/acceptance 聚焦 cargo tests；`pnpm --filter spec-wiki test` |
| 执行时间 | 聚焦命令单项约 1 秒至 74 秒；全量 workspace `pnpm test` 约 235 秒 |
| 框架报告 | 控制台摘要；无 JUnit/Coverage 文件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 138 |
| 通过 | 138 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计；以 UT/ST 行为覆盖矩阵验收 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | model 3/3；index 1/1；runtime query 25/25；governance 8/8；acceptance query 4/4；TS package 97/97 | 统计与失败定位 |
| JUnit XML | 无 | 不适用 |
| Coverage | 无 | 未统计 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1-1.3 | wiki-model contract | 3/3 | pass | module route/ref、provenance/rank/group metadata |
| UT-002 / 1.4-1.6 | wiki-index + runtime SQLite | index 1/1 + workspace storage tests | pass | FTS 次序不被 path sort 覆盖 |
| UT-003 / 2.1-2.3 | runtime query integration | 25/25 | pass | module、rank/count/dedup/group metadata |
| UT-004 / 2.4-2.6 | runtime/governance integration | 25/25 + 8/8 | pass | 状态矩阵；负 raw BM25 不决定 confidence |
| UT-005 / 3.1-3.3 | acceptance transport | 4/4 | pass | canonical payload、空 groups、typed errors |
| UT-006 / 3.4-3.6 | TS parser | package 97/97 | pass | AnswerEnvelope、闭集、legacy fail-closed |
| UT-007 / 4.1-4.3 | TS renderer/CLI/Agents | package 97/97 | pass | canonical fields 与薄消费 |
| UT-008 / 5.1-5.3 | Wiki contract | 3/3 | pass | authority、导航和延期边界 |
| Task 5.4-5.5 | E2E/full gates | E2E 1/1、distribution 5/5、workspace gates pass | pass | canonical payload 与发布产物 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI E2E + Rust integration + fixed-path contract + distribution automation |
| 测试环境 | 临时仓库、当前 `wiki-runtime` debug/release 二进制、SQLite FTS；无外部依赖 |
| 工具 | Vitest、cargo test、项目 build/distribution scripts |
| verification mode | project automation / CLI/API |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `pnpm test`、`pnpm build`、`pnpm lint`、E2E/contract/distribution 聚焦命令 |
| 动作摘要 | init 临时 repo，status/update/query，经 TS CLI 调用当前 Rust binary |
| 断言点 | JSON schema、typed errors、route/ref/provenance/rank/count、CLI exit、source refs、authority markers、package layout |
| 证据路径 | 自动化测试源码与控制台摘要；无附件 |
| fallback 原因 | 本 change 无 UI/browser 交互，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | CLI/API / project automation |
| host Playwright availability | not-applicable |
| manual verification | 无；自动化 CLI/E2E 足以覆盖全部 ST |
| evidence paths | 无图片/trace/video；证据为测试源码与命令摘要 |
| assertion points | JSON/exit code/filesystem/DTO/contract 非图片断言 |
| fallback reason | 无浏览器界面或交互需求 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 6 |
| 通过 | 6 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | 唯一 authority 与跨层字段一致 | 合同 | pass | UT-008、transport、TS、E2E |
| ST-002 | term-only、空结果与 typed errors | 异常/边界 | pass | acceptance、CLI/parser |
| ST-003 | route fusion、module 与 fallback | 正常/降级 | pass | runtime query、model |
| ST-004 | ranking、截断、去重、confidence | 边界 | pass | index order、runtime query、负 BM25 confidence |
| ST-005 | readiness/trust/provenance/answer/governance | 状态 | pass | runtime/governance/parser |
| ST-006 | richer query 延期门禁 | 合同 | pass | Wiki/Agents contract |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | pass | 格式通过 |
| `cargo test -p wiki-model --test query_contract` | pass | 3/3 |
| `cargo test -p wiki-index source_query_preserves_store_relevance_order` | pass | 1/1 |
| `cargo test -p wiki-runtime --test runtime query_` | pass | 25/25 |
| `cargo test -p wiki-runtime --test governance_workflows` | pass | 8/8 |
| `cargo test -p wiki-runtime --test acceptance command_contract::query_` | pass | 4/4 |
| `pnpm --filter spec-wiki test` | pass | 97/97 |
| Runtime query contract | pass | 3/3 |
| CLI E2E | pass | 1/1 |
| Distribution | pass | 5/5 |
| `pnpm test` | pass | workspace Rust、TS、51 项 scripts 全绿 |
| `pnpm build` | pass | release runtime + staged package |
| `pnpm lint` | pass | ESLint 全仓 |
| UniSpec validate | pass | change valid |
| `cargo clippy ... -D warnings` | skipped | 未修改文件的既有告警；其它证据充分，记录为残余风险 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | contract + transport + TS/Agents + E2E | pass | canonical 字段一致 |
| ST-002 | acceptance + CLI/parser | pass | term、空 groups、typed errors |
| ST-003 | runtime + model | pass | fusion/module/fallback |
| ST-004 | index + runtime | pass | route-local ordering/count/dedup/confidence |
| ST-005 | runtime/governance/parser | pass | 状态矩阵和正交性 |
| ST-006 | Wiki/Agents contract | pass | richer 延期边界 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 唯一 authority 与跨层字段一致 | ST-001、UT-005..008、E2E | pass |
| term-only/richer 边界 | ST-002、ST-006 | pass |
| route/ranking/截断/去重 | ST-003、ST-004 | pass |
| provenance/confidence/source refs | ST-004、ST-005 | pass |
| readiness/trust/answer/action | ST-002、ST-005 | pass |
| fallback 显式降级 | ST-003、ST-005 | pass |
| typed errors、空结果、恢复建议 | ST-002 | pass |
| deferred richer 能力分类 | ST-006、UT-008 | pass |
| 后续 child 可直接引用 | canonical authority + contract | pass |

## 失败项

- 无。

## 未验证项

- 无。

## 证据缺口

- 无。

## 下一步

- 在 review report 同为 full/pass 后使用 `unispec-archive`。
