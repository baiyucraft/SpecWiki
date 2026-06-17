---
verification-result: pass
scope: full
---

# formalize-declared-authoring-contract 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：验证证据覆盖 proposal 成功标准、ST-001 至 ST-008 和关键 workspace regression gate；`node scripts/run-test-projects.mjs` 因 0 total 按 ST-008 仅记录为 skipped/not-applicable smoke，不作为 pass gate。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-16 |
| 执行环境 | Windows / PowerShell；Rust workspace；Node + pnpm；UniSpec CLI |
| 测试方式 | 自动化测试 + CLI 验证 + 非门禁 smoke |

## 验证范围

- `formalize-declared-authoring-contract` 的 proposal、design、system-tests、tasks、meta artifact。
- `DeclaredRecord` typed scope、relations、status、identity 与 declared model 行为。
- managed declared block 的 lifecycle relation parse、artifact writeback、restore / roundtrip。
- `sync` 分类优先级、artifact restore truth source、status / update / query 可观察诊断。
- workspace lint、workspace test、targeted Vitest、样本项目 smoke 降级规则。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 至少 422 个自动化用例有明确通过摘要 |
| 单元测试通过 | 至少 422 |
| 单元测试失败 | 0 |
| 系统测试总数 | 8 |
| 系统测试通过 | 7 |
| 系统测试失败 | 0 |
| 跳过 | 1 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust cargo test；Vitest；workspace package test runner |
| 执行命令 | `cargo test -p wiki-model declared -- --nocapture`; `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture`; `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture`; `cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture`; `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture`; `pnpm exec vitest run --config vitest.config.mjs scripts/tests/test-project-analysis.test.ts scripts/tests/test-project-analysis-2_0.test.ts scripts/tests/reference-report-snapshot.test.ts`; `pnpm run test` |
| 执行时间 | 未记录 |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 至少 422 |
| 通过 | 至少 422 |
| 失败 | 0 |
| 跳过 | 未统计 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `wiki-model declared`: 6 passed；`editable_runtime`: 17 passed；`knowledge_artifacts_roundtrip`: 8 passed；`status_and_update`: 30 passed；`query_sync_rebuild`: 16 passed；targeted Vitest: 3 files / 6 tests passed；`pnpm run test`: wiki-runtime lib 75 passed、acceptance 11 passed、hierarchy 41 passed、llm_runtime 18 passed、repo 47 passed、runtime integration 141 passed、symbols 21 passed、packages/spec-wiki Vitest 10 files / 46 tests passed、root scripts Vitest 12 files / 41 tests passed、package build pass | 统计 / 覆盖映射 |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 未统计覆盖率 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| Task 1.1 | `unispec validate formalize-declared-authoring-contract` | pass | pass | `system-tests.md` 已可被 required artifact 结构验证覆盖 |
| Task 1.2 / 1.3 | artifact 审查 + `unispec validate` + workspace tests | pass | pass | capability baseline 已与 declared formal truth layer 保持一致 |
| Task 1.4 | artifact 审查 | pass | pass | 历史 fail 报告未在 apply 阶段被提前改写；本轮由 `unispec-review` 正式签发 pass |
| Task 2.1 | `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture` | 17 passed | pass | 覆盖 managed declared block lifecycle parse -> artifact |
| Task 2.2 | `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture` | 8 passed | pass | 覆盖 lifecycle relation restore / roundtrip |
| Task 2.3 | `cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture`; `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture` | 30 passed / 16 passed | pass | 覆盖 status/query/update 诊断消费 |
| Task 2.4 | `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture` | 8 passed | pass | 覆盖 page drift 不反推 truth |
| Task 3.1 | `pnpm run test` | pass | pass | 原 workspace gate 失败已修复并纳入通过证据 |
| Task 3.2 | `node scripts/run-test-projects.mjs` | exit 0, `0 total` | skipped | 按 ST-008 记录为 not-applicable smoke，不作为 pass gate |
| Task 3.3 | declared 定向验证命令 | all pass | pass | 覆盖 model/runtime declared 定向路径 |
| Task 3.4 | `pnpm run lint`; `pnpm run test` | pass | pass | 覆盖 workspace gate |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI / Rust runtime integration / workspace automation / smoke |
| 测试环境 | 本地 workspace；无浏览器入口 |
| 工具 | UniSpec CLI；cargo test；pnpm；Vitest；Node script |
| verification mode | CLI/API + unit + project automation + skipped smoke |
| tool availability | available；Playwright not-applicable |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | 见“测试命令和结果” |
| 动作摘要 | 执行结构校验、Rust model/runtime 测试、workspace lint/test、targeted Vitest、样本 smoke |
| 断言点 | CLI exit/result、框架通过摘要、ST 覆盖映射、0 total smoke 降级规则 |
| 证据路径 | 控制台摘要；未提供独立日志路径 |
| fallback 原因 | 本 change 无 UI / browser 验证入口，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | 本 change 的断言由 CLI、Rust runtime tests、Vitest 与 workspace test runner 覆盖 |
| fallback reason | `system-tests.md` 明确本 change 不要求浏览器验证 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 8 |
| 通过 | 7 |
| 失败 | 0 |
| 跳过 | 1 |
| 关键路径通过率 | 100% gate 通过；ST-008 为非门禁 skipped/not-applicable |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | change artifact 结构可被 UniSpec 识别 | 正常 | pass | `unispec validate formalize-declared-authoring-contract`: pass |
| ST-002 | DeclaredRecord schema 支持 typed scope、relations 与 lifecycle | 正常 | pass | `cargo test -p wiki-model declared -- --nocapture`: 6 passed |
| ST-003 | 受管 declared block 可回写 lifecycle relations | 正常 | pass | `editable_runtime`: 17 passed；覆盖 lifecycle writeback + roundtrip |
| ST-004 | sync 分类优先级稳定 | 边界 | pass | `editable_runtime`: 17 passed；覆盖 `illegal_drift > declared_writeback > metadata_only` |
| ST-005 | artifact restore 不从页面正文反推 declared truth | 异常 / 边界 | pass | `knowledge_artifacts_roundtrip`: 8 passed |
| ST-006 | declared lifecycle 驱动 update/status/query 可观察诊断 | 正常 | pass | `status_and_update`: 30 passed；`query_sync_rebuild`: 16 passed |
| ST-007 | workspace gate 无 declared 回归 | 正常 | pass | `pnpm run lint`: pass；`pnpm run test`: pass |
| ST-008 | 样本验证是非门禁 smoke check | 边界 | skipped | `node scripts/run-test-projects.mjs`: exit 0, `Done. 0 passed, 0 failed, 0 total. jobs=1`；按 ST-008 记为 skipped/not-applicable |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `unispec validate formalize-declared-authoring-contract` | pass | change artifact 结构校验通过 |
| `cargo test -p wiki-model declared -- --nocapture` | pass | 6 passed |
| `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture` | pass | 17 passed |
| `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture` | pass | 8 passed |
| `cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture` | pass | 30 passed |
| `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture` | pass | 16 passed |
| `node scripts/run-test-projects.mjs` | skipped | exit 0，`Done. 0 passed, 0 failed, 0 total. jobs=1`；按 ST-008 为 skipped/not-applicable smoke |
| `pnpm run lint` | pass | `eslint .` |
| `pnpm exec vitest run --config vitest.config.mjs scripts/tests/test-project-analysis.test.ts scripts/tests/test-project-analysis-2_0.test.ts scripts/tests/reference-report-snapshot.test.ts` | pass | 3 files / 6 tests passed |
| `pnpm run test` | pass | wiki-runtime lib 75 passed；acceptance 11 passed；hierarchy 41 passed；llm_runtime 18 passed；repo 47 passed；runtime integration 141 passed；symbols 21 passed；packages/spec-wiki Vitest 10 files / 46 tests passed；root scripts Vitest 12 files / 41 tests passed；package build pass |
| `git diff --check` | pass | pass with CRLF warnings only |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | UniSpec CLI validate / status evidence | pass | Required artifacts 可被识别 |
| ST-002 | `wiki-model declared` tests | pass | typed scope、relations、status、identity 有模型级覆盖 |
| ST-003 | `editable_runtime` + roundtrip evidence | pass | lifecycle writeback 与 artifact 字段稳定性有 runtime 覆盖 |
| ST-004 | `editable_runtime` tests | pass | sync 分类优先级有集成测试覆盖 |
| ST-005 | `knowledge_artifacts_roundtrip` tests | pass | cache-less restore / page drift 负向场景有覆盖 |
| ST-006 | `status_and_update` + `query_sync_rebuild` tests | pass | status/update/query diagnostic consumption 有覆盖 |
| ST-007 | `pnpm run lint` + `pnpm run test` | pass | workspace gate 通过 |
| ST-008 | `node scripts/run-test-projects.mjs` | skipped | 0 total 时按设计不计入 pass gate |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| change 目录具备 proposal、design、system-tests、tasks、meta artifact | ST-001；`unispec validate formalize-declared-authoring-contract` | pass |
| `DeclaredRecord` 具备 typed scope、稳定 identity、relations、lifecycle status 与 canonical serialization | ST-002；`cargo test -p wiki-model declared -- --nocapture` | pass |
| managed declared block 能解析为正式 declared record 并支持 lifecycle 关系 | ST-003；`editable_runtime` 17 passed | pass |
| `sync` 分类优先级稳定 | ST-004；`editable_runtime` 17 passed | pass |
| `.wiki/.knowledge/declared/**` 是正式 restore/audit truth source | ST-005；`knowledge_artifacts_roundtrip` 8 passed | pass |
| declared lifecycle 能影响 readiness、health summary、recommended_action、derived/projection refresh 或等价诊断 | ST-006；`status_and_update` 30 passed；`query_sync_rebuild` 16 passed | pass |
| 本 change 不破坏 workspace regression gate | ST-007；`pnpm run lint`; `pnpm run test` | pass |
| 样本验证 0 total 不冒充 pass 证据 | ST-008；`node scripts/run-test-projects.mjs` skipped/not-applicable | pass |

## 失败项

- 无。

## 未验证项

- 无。

## 证据缺口

- 无。样本项目脚本输出 0 total 已按 ST-008 明确降级为 skipped/not-applicable，不构成本 change 的 evidence gap。

## 下一步

- 在 `review-report.md` 同为 pass/full 后进入 archive 流程。
