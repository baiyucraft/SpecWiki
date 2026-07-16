---
verification-result: pass
scope: full
---

# close-specwiki-3-0-design-baseline-reliability-lifecycle 测试报告

## 验证结论

- verification recommendation: pass
- scope: full
- 结论摘要：UT-001 至 UT-015、ST-001 至 ST-013 和 proposal 全部成功标准均有自动化证据，修复上一轮 `operation_id` 路径逃逸 blocker 后全量门禁及定向负向测试通过，建议签发 full/pass。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-16T17:57:54+08:00 |
| 执行环境 | Windows / PowerShell；Rust workspace；Node.js + pnpm + Vitest；SQLite 本地依赖；离线 fake provider 与动态临时仓库；无浏览器、网络 provider 或外部大仓依赖 |
| 测试方式 | 项目自动化、CLI/API、Rust unit/integration/acceptance、Vitest 合同测试及静态质量门禁 |

## 验证范围

- 覆盖唯一 reliability reducer、freshness/consumability、route-local trust、provider failure、declared authority/history、projection governance/protection、runtime 原子提交、KnowledgeUnit resume identity、request-local provider session、A1-A10 Wiki authority 和公开 workflow 终态。
- 覆盖上一轮 P0 修复：非法 `operation_id` 在 capture、execute 和 recovery 共用边界被 fail closed，且不会在 journal 根或仓库外产生写入。
- 覆盖实现、Rust workspace、Node/Vitest、构建/分发/E2E、UniSpec artifact 和 diff whitespace 门禁。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 未统一统计；Rust workspace 含多个 test binary，另有 package Vitest 97、scripts Vitest 64 |
| 单元测试通过 | 全部；runtime integration 167/167，package Vitest 97/97，scripts Vitest 64/64 |
| 单元测试失败 | 0 |
| 系统测试总数 | 13 |
| 系统测试通过 | 13 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust built-in test harness、Vitest |
| 执行命令 | `cargo test --workspace`；`pnpm test`；修复后定向复验 `cargo test -p wiki-runtime --test runtime runtime_commit_rejects_operation_ids_that_escape_the_journal_root` |
| 执行时间 | 未记录 |
| 框架报告 | 控制台摘要；无持久化 JUnit/coverage 附件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 未统一统计；关键可核对统计为 runtime integration 167、package Vitest 97、scripts Vitest 64 |
| 通过 | 全部 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计；未生成 coverage 报告，不虚构行/分支/函数覆盖率 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `cargo test --workspace` 全绿；runtime integration `167 passed`；`pnpm test` 中 package Vitest `97 passed`、scripts Vitest `64 passed`，并通过 build/distribution/e2e | 用例统计与回归证据 |
| 定向控制台摘要 | `runtime_commit_rejects_operation_ids_that_escape_the_journal_root`: `1 passed; 0 failed; 166 filtered out` | P0 路径逃逸修复证据 |
| JUnit XML | 无 | 未生成 |
| Coverage | 无 | 覆盖率未单独统计 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / 1.1-1.3 | Rust reducer 与 runtime integration | `cargo test --workspace`；runtime profile/state/status tests | pass | evidence 优先级、freshness/consumability 双轴与 action 前置顺序受测 |
| UT-002 / 1.4-1.6 | Rust query/status/workflow integration | `cargo test --workspace`；query/status/state tests | pass | route-local trust、governance 正交和 canonical action 投影受测 |
| UT-003 / 2.1-2.3 | exhaustive reducer table | `cargo test --workspace`；research decision tests | pass | stop reason、execution policy、有效 output 的组合受测 |
| UT-004 / 2.4-2.6 | provider/transport/workflow integration | `cargo test --workspace`；research provider 与 LLM runtime tests | pass | typed failure 贯穿 artifact/gate/status/action，production fail closed |
| UT-005 / 3.1-3.3 | knowledge graph unit + runtime integration | `wiki-knowledge/tests/declared_authority.rs` 与 workspace 测试 | pass | replacement graph、head none/unique/conflict、非法关系和环受测 |
| UT-006 / 3.4-3.6 | sync/update/query/restore integration | runtime editable/declared lifecycle tests | pass | missing/detached 保留、恢复绑定、先 deprecated 再 detach 受测 |
| UT-007 / 3.7-3.9 | governance event integration | declared authority/lifecycle 与 artifact roundtrip tests | pass | open/resolved/reopen/no-op append-only history 受测 |
| UT-008 / 4.1-4.3 | projection policy unit + init/rebuild integration | `wiki-knowledge/tests/projection_governance.rs` 与 runtime tests | pass | required/budget/include/exclude/priority/hints 的确定性受测 |
| UT-009 / 4.4-4.6 | protection/demotion integration | declared projection lifecycle、managed section tests | pass | manual/declared/manual-link 保护及 retiring/clean removal 受测 |
| UT-010 / 4.7-4.9 | runtime commit failpoint/recovery integration | `runtime/runtime_commit.rs`、artifact roundtrip tests | pass | pointer 前 rollback、pointer 后 roll-forward、composite snapshot 与非法 operation identity 受测 |
| UT-011 / 5.1-5.3 | identity unit + resume integration | checkpoint/page render/large fixture tests | pass | action/facts/tree/contract 任一变化均拒绝旧 working state |
| UT-012 / 5.4-5.6 | commit-point validator integration | page render、SQLite gates、large fixture tests | pass | research cache 与 draft/digest pair 的完整性、commit ref 和整 unit 重算受测 |
| UT-013 / 5.7-5.9 | recording fake provider + storage scan | `llm_runtime.rs`、research provider、large fixture tests | pass | 新请求 `session=None`，session summary/turn/tool refs 不进入持久化状态 |
| UT-014 / 6.1-6.3 | fixed-path Vitest contract | `scripts/tests/reliability-lifecycle-contract.test.ts`，`pnpm test` scripts 64/64 | pass | A1-A10 分类、A4 authority 与 session/decomposition/规模边界受测 |
| UT-015 / 6.4-6.7 | 48-unit deterministic interruption fixture | `runtime/compose_resume_large_fixture.rs` 与 runtime integration 167/167 | pass | 第 17 unit 中断后前 16 不重算、当前重试、identity 变化全重算、公开终态清 checkpoint |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | 项目自动化、CLI/API、Rust unit/integration/acceptance 与 Vitest 合同测试 |
| 测试环境 | 本地离线动态仓库、SQLite、fake provider；无外部服务 |
| 工具 | Cargo/Rust test harness、pnpm/Vitest、UniSpec CLI |
| verification mode | project automation / CLI/API / unit |
| tool availability | not-applicable |
| 入口 URL | 不适用 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `cargo test --workspace`、`pnpm test`、`unispec validate close-specwiki-3-0-design-baseline-reliability-lifecycle` 及质量门禁 |
| 动作摘要 | 离线执行状态转换、失败注入、commit phase 中断恢复、48-unit resume、文档合同与 CLI/build/distribution/e2e 回归 |
| 断言点 | reducer decision、layer state/trust/action、artifact/gate/checkpoint、provider call count、page/hash/binding/snapshot、进程/CLI 输出、固定路径文档字段 |
| 证据路径 | 控制台摘要，无附件 |
| fallback 原因 | change 无 UI/browser 交互需求，Playwright 不适用于该验证范围 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | project automation / CLI/API / unit |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 控制台摘要，无附件 |
| assertion points | ST-001 至 ST-013 均由状态、artifact、CLI、计数和固定路径合同等非图片断言覆盖 |
| fallback reason | change 无 UI/browser 交互需求，无入口 URL、浏览器或 viewport；无需 Playwright fallback 执行 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 13 |
| 通过 | 13 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | A1 freshness 生命周期跨 workflow 一致 | 正常/边界 | pass | UT-001/002；runtime status/query/update/rebuild fixtures |
| ST-002 | formal artifact 和 page binding drift fail closed | 异常 | pass | integrity/restore/runtime commit tests；旧 cache 不恢复 formal truth |
| ST-003 | declared replacement graph 与 authority 状态闭合 | 正常/异常 | pass | UT-005；declared authority graph + runtime integration |
| ST-004 | declared block 删除保留 truth 和治理历史 | 正常/边界 | pass | UT-006/007；missing/detached、restore、event history tests |
| ST-005 | same-facts declared 变化确定性传播 | 边界 | pass | artifact roundtrip、status/update/restore integration |
| ST-006 | projection policy 默认有界且可重放 | 正常/边界 | pass | UT-008；projection policy + init/rebuild tests |
| ST-007 | projection demotion 安全回收与保护 | 正常/异常 | pass | UT-009/010；protection、retiring、commit recovery tests |
| ST-008 | production provider failure matrix 单值一致 | 异常 | pass | UT-003/004；exhaustive reducer、provider/transport workflow tests |
| ST-009 | parser/index/compose/assemble failure remaining capability 一致 | 异常 | pass | runtime/CLI/NDJSON acceptance regressions，workspace tests 全绿 |
| ST-010 | KnowledgeUnit resume 复用完整提交点并拒绝变更 identity | 边界/异常 | pass | UT-011/012/015；48-unit fixture 与 commit-pair validation |
| ST-011 | provider session 保持 request-local | 边界 | pass | UT-013；recording provider 和 durable storage forbidden-field scan |
| ST-012 | A1-A10 与术语 authority 一致 | 正常 | pass | UT-014；fixed-path Vitest contract，scripts 64/64 |
| ST-013 | 完整公开 workflow 与 committed snapshot 原子终态 | 正常/异常 | pass | UT-010-012/015；commit recovery、roundtrip、public workflow 与 full review evidence |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | pass | Rust formatter 门禁通过 |
| `cargo check --workspace` | pass | workspace compiler check 通过 |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | 全 target 静态检查零 warning |
| `cargo test --workspace` | pass | workspace 全绿；runtime integration 167 tests，含新增 `operation_id` 路径边界用例 |
| `cargo test -p wiki-runtime --test runtime runtime_commit_rejects_operation_ids_that_escape_the_journal_root` | pass | 独立复验 `1 passed; 0 failed; 166 filtered out` |
| `pnpm lint` | pass | Node/TypeScript lint 通过 |
| `pnpm test` | pass | package Vitest 97、scripts Vitest 64，build/distribution/e2e 全绿 |
| `unispec validate close-specwiki-3-0-design-baseline-reliability-lifecycle` | pass | 独立复验输出 `Change ... is valid.` |
| `git diff --check` | pass | 无 whitespace error；仅 Git 的 LF/CRLF 工作区提示 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | Rust reducer + workflow integration | pass | freshness、consumability、trust/action 与恢复闭环 |
| ST-002 | restore/integrity/runtime commit integration | pass | manifest/page/binding/hash drift fail closed |
| ST-003 | knowledge graph + runtime integration | pass | replacement authority 的合法与非法闭集 |
| ST-004 | sync/update/query/restore integration | pass | authoring state 与 append-only governance history |
| ST-005 | artifact roundtrip + same-facts update | pass | declared identity 传播且 source/index 不伪变更 |
| ST-006 | projection policy unit + init/rebuild | pass | required、budget、override 和 deterministic order |
| ST-007 | projection protection + commit failpoints | pass | clean removal、protected retiring、rollback/roll-forward |
| ST-008 | exhaustive reducer + provider workflow/transport | pass | production output gate 与 typed failures |
| ST-009 | workflow integration + CLI/acceptance | pass | pipeline failure 的 remaining capability/state/action/exit 一致 |
| ST-010 | 48-unit resume fixture | pass | same identity 复用，changed identity/half pair 拒绝，终态清理 |
| ST-011 | LLM/provider tests + storage scan | pass | request-local session 不落盘 |
| ST-012 | fixed-path Vitest contract | pass | A1-A10 与公开术语边界唯一 |
| ST-013 | runtime commit/roundtrip/public workflow + UniSpec review | pass | composite snapshot 原子终态和流程 identity 一致 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 唯一跨层 reliability lifecycle authority | ST-001、ST-002、ST-009；UT-001、UT-002 | pass |
| A1 fresh -> stale/needs_update -> update/rebuild -> fresh，旧 route/cache/checkpoint 不 direct-trust | ST-001、ST-002、ST-010 | pass |
| A6 lifecycle、关系、迁移、head none/unique/conflict 与非法 graph 行为唯一 | ST-003、ST-004；UT-005、UT-006、UT-007 | pass |
| same-facts declared 传播、非法 page drift 不覆盖 truth、冲突 evidence/action 单值 | ST-004、ST-005、ST-007 | pass |
| 每 unit 有 projection decision，结构页/knowledge-only/selected/retired 边界和默认预算有界 | ST-006；UT-008 | pass |
| projection 生命周期与 readiness 正交，安全回收且保护 manual/declared 内容 | ST-007、ST-013；UT-009、UT-010 | pass |
| A9 failure matrix 覆盖 index/formal/provider/budget/output/runtime/compose/assemble/page drift | ST-002、ST-008、ST-009 | pass |
| production 无有效 provider evidence 不得正式成功，status/gate/query/exit 一致 | ST-008、ST-009；UT-003、UT-004 | pass |
| 大规模 compose 相同 identity 复用、变化 identity 拒绝、公开完成无伪 checkpoint | ST-010、ST-013；UT-011、UT-012、UT-015 | pass |
| provider session request-local，decomposition 与任意大仓边界诚实 | ST-011、ST-012；UT-013、UT-014 | pass |
| A1-A10 唯一分类且 A4 只引用 canonical Runtime Query authority | ST-012；`reliability-lifecycle-contract.test.ts` | pass |
| proposal/design/system tests/实现/full review/verification/Wiki 使用同一 lifecycle identity | ST-012、ST-013；UniSpec validate 与通过的 review draft | pass |

## 失败项

- 无。上一轮 reviewer 发现的 `operation_id` 路径逃逸 P0 已修复，并由 capture/execute 负向测试、仓库外无写入断言、定向测试及 workspace 全量测试复验通过，不再是当前失败项。

## 未验证项

- 无 required 项未验证。真实网络 provider 质量、外部任意大仓 wall-clock、跨机器 cache/session、turn-level checkpoint、hard purge、redirect 和在线 usage ranking 属于 proposal 明确 non-goal/diagnostic，不计入本 change required gate。

## 证据缺口

- 无 blocking evidence gap。未生成独立 coverage 报告，因此覆盖率记为“未统计”；UT/ST/成功标准已有明确行为测试映射，这不影响本次 full verification recommendation。

## 下一步

- 由 `unispec-review` 校验本 draft 与 code review draft，签发正式 `review-report.md` / `test-report.md`；正式 full/pass 后推进 verification 并使用 `unispec-archive` 归档。
