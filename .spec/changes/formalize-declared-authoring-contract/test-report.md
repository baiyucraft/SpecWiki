---
verification-result: fail
scope: full
---

# formalize-declared-authoring-contract 测试报告

## 验证结论

- verification-result: fail
- scope: full
- 结论摘要：declared authoring contract 的定向 Rust 验证和 UniSpec artifact 结构校验通过，但 full verification 因 workspace test 失败、样本验证无有效覆盖和 review 阻塞问题未解决而失败。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-16T19:39:56+08:00 |
| 执行环境 | Windows / PowerShell；Node v20.20.0；pnpm 10.6.3；Rust cargo test |
| 测试方式 | 自动化测试 / CLI 验证 / 文件审查 / 替代验证 |

## 验证范围

- change artifacts：`proposal.md`、`design.md`、`system-tests.md`、`tasks.md`、`meta.yaml`
- proposal/design 成功标准：declared schema、typed scope、relations/lifecycle、sync/update/status/query 消费、artifact roundtrip/restore、diagnostics
- 系统测试：ST-001、ST-002
- tasks 已完成项：1.1-5.3
- 非破坏性命令：Rust 定向测试、UniSpec CLI 校验、样本验证脚本、workspace test

## 部分范围

无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 定向 Rust 75；workspace `pnpm run test` 已执行到 75 |
| 单元测试通过 | 定向 Rust 75；workspace `pnpm run test` 74 |
| 单元测试失败 | workspace `pnpm run test` 1 |
| 系统测试总数 | 2 |
| 系统测试通过 | 2 |
| 系统测试失败 | 0 |
| 跳过 | 样本验证未覆盖实际项目 |
| 证据缺口 | 3 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust `cargo test`；pnpm workspace test wrapper |
| 执行命令 | `cargo test -p wiki-model declared -- --nocapture`；`cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture`；`cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture`；`cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture`；`cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture`；`pnpm run test` |
| 执行时间 | 本轮执行；单命令耗时见控制台摘要 |
| 框架报告 | 控制台摘要；未生成 JUnit XML / coverage |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 定向 Rust 75；workspace `pnpm run test` 已执行到 75 |
| 通过 | 定向 Rust 75；workspace `pnpm run test` 74 |
| 失败 | workspace `pnpm run test` 1 |
| 跳过 | 不适用 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `cargo test -p wiki-model declared`：6 passed，0 failed | declared schema / scope / relation consistency |
| 控制台摘要 | `cargo test -p wiki-runtime --test runtime editable_runtime`：16 passed，0 failed | sync 分类、declared writeback、illegal drift、metadata-only |
| 控制台摘要 | `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip`：7 passed，0 failed | declared artifact roundtrip / restore guard |
| 控制台摘要 | `cargo test -p wiki-runtime --test runtime status_and_update`：30 passed，0 failed | status/update declared-aware diagnostics 与 refresh |
| 控制台摘要 | `cargo test -p wiki-runtime --test runtime query_sync_rebuild`：16 passed，0 failed | query/status/rebuild route 与 diagnostic 行为 |
| 控制台摘要 | `pnpm run test`：74 passed，1 failed；失败用例 `workflows::init::tests::build_minimal_page_context_persists_child_contract_and_unit_identity` | workspace regression gate |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 无 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| `crates/wiki-runtime/src/workflows/init.rs` lib test | `workflows::init::tests::build_minimal_page_context_persists_child_contract_and_unit_identity` | `parent page should be planned` | 全量 workspace test gate 失败，不能签发 verification pass | 回到 apply/debug 修复，或正式确认该失败与本 change 无关并补充风险处置 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| 1.1 / 1.2 | `cargo test -p wiki-model declared` | 6 passed | pass | 覆盖 declared typed scope、relations、status、identity 与 lifecycle consistency |
| 1.3 | `cargo test -p wiki-runtime --test runtime editable_runtime` | 16 passed | pass | 覆盖合法 declared block、parse failure / illegal drift、delete 行为 |
| 2.1 / 2.2 / 2.3 | `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip` | 7 passed | pass | 覆盖 declared artifact 持久化、roundtrip、restore guard |
| 3.1 | `cargo test -p wiki-runtime --test runtime editable_runtime` | 16 passed | pass | 覆盖 `declared_writeback`、`metadata_only`、`illegal_drift` 及优先级 |
| 3.2 / 3.3 | `cargo test -p wiki-runtime --test runtime status_and_update` | 30 passed | pass | 覆盖 declared stale scope 在无源码 dirty set 时驱动 update，以及 status recommended_action |
| 3.4 | `cargo test -p wiki-runtime --test runtime query_sync_rebuild` | 16 passed | pass | 覆盖 query/status/rebuild 诊断；未发现 answer assembly 扩张证据 |
| 4.1 / 4.2 / 4.3 | 上述定向 Rust 测试集合 | 75 passed | pass | 覆盖 declared model、sync matrix、storage/workflow integration |
| 4.4 | `node scripts/run-test-projects.mjs`；`pnpm run test` | 样本验证 0 total；workspace test failed | fail | 样本验证未形成有效覆盖；全量测试失败 |
| 5.1 / 5.2 / 5.3 | artifact / source grep 与 review draft | 文件证据 + review finding | fail | `.wiki` baseline 仍存在旧新 declared contract 冲突 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI / 文件审查 / 替代验证 |
| 测试环境 | 本地仓库 `E:\project\!byAI\spec-wiki` |
| 工具 | UniSpec CLI；PowerShell；Rust / Node test runner |
| configured playwright | true |
| verification mode | CLI/API / unit |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `unispec validate formalize-declared-authoring-contract`；`unispec status --json`；人工对照 proposal/design/system-tests/tasks |
| 动作摘要 | 校验 required artifacts；读取 status JSON；追踪 tasks 到 proposal/design 与测试命令 |
| 断言点 | CLI 输出、JSON artifact status、tasks checkbox、测试控制台摘要 |
| 证据路径 | 控制台摘要；本报告记录 |
| fallback 原因 | 本 change 无 UI/browser 交互目标；未执行 Playwright |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| configured playwright | true |
| verification mode | CLI/API / unit |
| host Playwright availability | not-applicable |
| manual verification | 人工对照 `proposal.md`、`design.md`、`system-tests.md`、`tasks.md`；执行者：Codex；时间：2026-06-16T19:39:56+08:00 |
| configured imageAnalysis | true |
| actual image analysis | not-used |
| evidence paths | 无截图、trace、video 或 browser report |
| assertion points | CLI 输出、JSON 状态、Rust/Node 测试输出、artifact 内容 |
| fallback reason | 验证目标为 CLI/runtime/artifact contract，无浏览器入口 URL 或 UI 断言点 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 2 |
| 通过 | 2 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100%，但整体 verification 仍因全量测试失败和 review blockers 失败 |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | 迁移后的 change artifact 结构可被 UniSpec 识别 | 正常 | pass | `unispec validate formalize-declared-authoring-contract` 输出 valid；`unispec status --json` 中 required artifacts present，blockingIssues 为空 |
| ST-002 | 原有任务闭环继续可追踪 | 正常 | pass | 人工审查 tasks 可追踪到 proposal/design；但这些 ST 只覆盖迁移结构，不覆盖核心业务验收 |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `unispec validate formalize-declared-authoring-contract` | pass | 输出 change valid |
| `cargo test -p wiki-model declared -- --nocapture` | pass | 6 passed，0 failed |
| `cargo test -p wiki-runtime --test runtime editable_runtime -- --nocapture` | pass | 16 passed，0 failed |
| `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip -- --nocapture` | pass | 7 passed，0 failed |
| `cargo test -p wiki-runtime --test runtime status_and_update -- --nocapture` | pass | 30 passed，0 failed |
| `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture` | pass | 16 passed，0 failed；有 metadata fallback warning，未导致失败 |
| `node scripts/run-test-projects.mjs` | skipped / risk | 0 passed，0 failed，0 total；只完成 release build，未形成样本项目 pass 证据 |
| `pnpm run lint` | pass | ESLint 通过 |
| `pnpm run test` | fail | workspace test 失败：74 passed，1 failed；失败用例 `build_minimal_page_context_persists_child_contract_and_unit_identity` |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | UniSpec CLI 结构校验 | pass | `validate` 与 `status --json` 覆盖 required artifact 完整性 |
| ST-002 | 人工追踪 + 定向测试映射 | pass | tasks 可追踪到 proposal/design，并有 declared 相关定向测试支撑；但 system-tests 仍需补核心业务 ST |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| DeclaredRecord 支持 typed scope、relations、lifecycle status 与稳定 identity | `cargo test -p wiki-model declared`；source grep 命中 `DeclaredKnowledgeRecord` / `DeclaredKnowledgeScope` / `DeclaredKnowledgeRelation` | pass |
| 支持 `supersedes / replaced_by / deprecated` 并保持单义 lifecycle 判定 | `declared_record_validates_single_meaning_lifecycle`、`declared_record_rejects_conflicting_lifecycle_inputs`、`declared_snapshot_rejects_relation_cycles` | pass |
| declared scope 从字符串收敛为 typed object，且 canonical serialization 稳定 | `declared_scope_canonical_key_is_stable` | pass |
| declared block parse failure / delete / upsert 进入显式诊断，不静默成功 | `cargo test -p wiki-runtime --test runtime editable_runtime` 中 illegal drift、delete、duplicate、conflict 相关用例 | pass |
| `.wiki/.knowledge/declared/**` artifact 可 roundtrip / restore / audit | `cargo test -p wiki-runtime --test runtime knowledge_artifacts_roundtrip` | pass |
| `sync` 分类固定为 `illegal_drift > declared_writeback > metadata_only` | `sync_prioritizes_illegal_drift_over_declared_writeback`、metadata-only 与 declared-writeback 用例 | pass |
| `update` 消费 declared lifecycle stale scope，即使无源码 dirty set | `update_consumes_declared_health_scope_without_source_dirty_set`、`update_consumes_removed_declared_health_scope_without_source_dirty_set` | pass |
| `status` 暴露 declared lifecycle readiness / health / recommended_action | `status_keeps_readiness_ready_but_exposes_health_degradation_after_declared_writeback` 等 status_and_update 用例 | pass |
| `query` 仅增强 declared lifecycle 可解释性，不新增 answer assembly contract | `query_sync_rebuild` 定向测试 | pass |
| ST-001 required artifact 完整 | `unispec validate formalize-declared-authoring-contract`；`unispec status --json` | pass |
| ST-002 迁移后任务语义不丢失 | 人工审查 proposal/design/tasks/system-tests | pass |
| 任务 4.4 样本验证 | `node scripts/run-test-projects.mjs` | skipped / risk |
| workspace 全量测试可作为 release-quality gate | `pnpm run test` | fail |

## 失败项

- `pnpm run test` 失败：`workflows::init::tests::build_minimal_page_context_persists_child_contract_and_unit_identity` panic `parent page should be planned`。影响：全量 workspace test gate 未通过，本报告不能签发 verification pass。
- code review 存在 blocking issues：system-tests 仍是迁移占位、wiki baseline 有新旧 declared contract 冲突、relations/status runtime authoring path 缺闭环测试。

## 未验证项

- `node scripts/run-test-projects.mjs` 没有实际验证样本项目，命令结果为 0 total。
- 未执行 Playwright：虽然 `.spec/config.yaml` 配置 `playwright: true`，但本 change 没有浏览器入口或 UI 断言点。
- 未生成 coverage、JUnit XML、Playwright trace、截图或视频证据。

## 证据缺口

- 样本验证未形成有效 pass 证据。
- 全量 `pnpm run test` 失败，需要修复或正式隔离失败用例。
- 缺少 runtime integration test 覆盖 managed declared block 中 `deprecated / replaced_by / supersedes` 的 parse -> artifact -> restore/status/query roundtrip。

## 下一步

- 回到 plan/apply，补齐 system-tests、wiki baseline 和 runtime relation tests。
- 修复 `pnpm run test` 失败用例，或给出正式隔离依据。
- 重新执行 full review + full verification；通过后再推进 `stage: verification` 并使用 `unispec-archive`。
