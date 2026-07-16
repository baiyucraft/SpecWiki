# close-specwiki-3-0-design-baseline-core-scenario-acceptance 单元测试设计

## 测试总览

本文件为 TDD 实现定义 11 个稳定 UT 蓝图，覆盖 matrix/plan、Gate Kernel v2、脚本 adapters/orchestrator、Runtime 场景、A/B restore 和 Wiki authority。每个 UT 都能追溯到 design、ST 和 tasks；实际测试写入由 `unispec-apply` 的 Red 任务完成。

## 单元测试用例

### UT-001 9/9 canonical acceptance matrix 闭集合同

**目标行为**

版本化 matrix 固定 9 个场景、支持等级、必填字段和 deferred/query 边界，并对非法输入 fail closed。

**关联**

- Design: `9 场景支持矩阵`、`CoreScenarioRecord`
- 系统测试用例: ST-001、ST-002
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `scripts/tests/core-scenario-acceptance-contract.test.ts`
- Modify: `scripts/testing/core-scenario-acceptance.mjs`
- Reference: `.spec/changes/close-specwiki-3-0-design-baseline-core-scenario-acceptance/design.md`

**测试代码蓝图**

```typescript
test("固定 9 个核心场景及其支持边界", async () => {
  expect(existsSync(matrixModulePath)).toBe(true);
  const { CORE_SCENARIO_ACCEPTANCE_MATRIX, validateCoreScenarioMatrix } = await import(matrixModulePath);
  expect(CORE_SCENARIO_ACCEPTANCE_MATRIX.map(item => item.scenario_id)).toEqual(
    Array.from({ length: 9 }, (_, index) => `CS-${String(index + 1).padStart(2, "0")}`),
  );
  expect(() => validateCoreScenarioMatrix(invalidMatrix)).toThrow();
});
```

**测试数据 / Fixture / Mock 边界**

- 复制单条记录构造重复 ID、未知 enum、缺字段和缺 evidence；不 mock 文件系统存在性。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/core-scenario-acceptance-contract.test.ts -t "固定 9 个核心场景及其支持边界"`

**预期 Red 失败**

- 失败测试名: `固定 9 个核心场景及其支持边界`
- 关键错误 / 断言差异: `expected false to be true`，目标 matrix module 尚不存在。
- 失败原因: 待实现的 matrix authority 缺失，不是导入、语法或环境错误。

**Green 通过条件**

- 9 条记录、支持等级、必填字段、evidence 和 deferred/query 边界全部通过。

**Refactor 守卫**

- 稳定排序；不加入 cache、旧 `provenance_summary` 或 richer query 公开入口。

### UT-002 Acceptance Plan 不硬编码历史样本

**目标行为**

Acceptance Plan 只消费调用方显式声明的 fixtures、required gates/guards 和 report-only。

**关联**

- Design: `Acceptance Plan`、`createAcceptancePlan`
- 系统测试用例: ST-003、ST-007、ST-014
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `scripts/tests/core-scenario-acceptance-contract.test.ts`
- Modify: `scripts/testing/core-scenario-acceptance.mjs`
- Reference: `scripts/testing/quality-gates.mjs`

**测试代码蓝图**

```typescript
test("acceptance plan 不硬编码历史样本", async () => {
  const plan = createAcceptancePlan({
    plan_id: "fixture-plan", primary_fixtures: ["fixture-a"],
    required_gates: ["artifact_validity"], required_guards: [], report_only: false,
  });
  expect(plan.primary_fixtures).toEqual(["fixture-a"]);
  expect(Object.isFrozen(plan)).toBe(true);
});
```

**测试数据 / Fixture / Mock 边界**

- 纯对象输入；未知 gate、空 plan id、缺 primary fixture 为负例。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/core-scenario-acceptance-contract.test.ts -t "acceptance plan 不硬编码历史样本"`

**预期 Red 失败**

- 失败测试名: `acceptance plan 不硬编码历史样本`
- 关键错误 / 断言差异: `expected "undefined" to be "function"`，factory 尚不存在。
- 失败原因: 显式 plan contract 尚未实现。

**Green 通过条件**

- 调用方输入原样规范化并深冻结；不自动注入 storybook/dagger。

**Refactor 守卫**

- unknown gate fail closed；输出不可被调用者后改写。

### UT-003 同一 failure identity 只有一个 owning blocker

**目标行为**

同一 failure 只由唯一 formal/primary gate 计 blocker，其他 gate/guard 仅引用。

**关联**

- Design: `Gate 聚合流程`、`FailureRecord`
- 系统测试用例: ST-004
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `scripts/tests/quality-gates-contract.test.ts`
- Modify: `scripts/testing/quality-gates.mjs`
- Reference: 现有 `buildAcceptanceHarnessSummary`

**测试代码蓝图**

```typescript
test("同一 failure identity 只产生一个 owning blocker", () => {
  const summary = aggregateGateResults({ plan, failures: [ownedFailure], gate_evidence: evidence });
  expect(summary.failures).toHaveLength(1);
  expect(Object.values(summary.gate_results).filter(gate => gate.decision === "blocker")).toHaveLength(1);
  expect(() => aggregateGateResults({ plan, failures: [leftOwner, rightOwner] })).toThrow();
});
```

**测试数据 / Fixture / Mock 边界**

- 纯对象 failure/evidence；覆盖无 owner、双 owner、unknown owner。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/quality-gates-contract.test.ts -t "同一 failure identity 只产生一个 owning blocker"`

**预期 Red 失败**

- 失败测试名: `同一 failure identity 只产生一个 owning blocker`
- 关键错误 / 断言差异: 预期 blocker 数 `1`，当前 lifecycle/v1 语义为 `3` 或 `4`，且没有 failure refs。
- 失败原因: v1 没有 failure identity 和 owner。

**Green 通过条件**

- failure 唯一、owner 唯一、无关 gate 不污染，非法 ownership fail closed。

**Refactor 守卫**

- 稳定 failure 排序；禁止从总失败数反推全部 gates。

### UT-004 Required coverage、diagnostic 与退出策略

**目标行为**

Gate/overall decision 分层，required 未覆盖与 diagnostic 不得伪 pass，默认退出码为 0/1/2。

**关联**

- Design: `Gate 聚合流程`、`默认退出策略`
- 系统测试用例: ST-005、ST-006
- Tasks: 2.7 Red / 2.8 Green / 2.9 Refactor

**Files**

- Test: `scripts/tests/quality-gates-contract.test.ts`
- Modify: `scripts/testing/quality-gates.mjs`
- Reference: `packages/spec-wiki/src/runtime/exitPolicy.ts`

**测试代码蓝图**

```typescript
test.each([
  ["pass", 0], ["blocker", 1], ["incomplete", 2], ["diagnostic", 2],
])("required coverage 与 exit policy 保持一致: %s", (decision, exitCode) => {
  expect(exitCodeForAcceptance({ decision, report_only: false })).toBe(exitCode);
});
```

**测试数据 / Fixture / Mock 边界**

- 纯对象表驱动；另测 report-only 只改 process exit，不改 summary decision。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/quality-gates-contract.test.ts -t "required coverage 与 exit policy 保持一致"`

**预期 Red 失败**

- 失败测试名: `required coverage 与 exit policy 保持一致`
- 关键错误 / 断言差异: 预期 `incomplete/2`，实际 `diagnostic/undefined` 或无导出函数。
- 失败原因: v1 无 required coverage/exit contract。

**Green 通过条件**

- required gate 未覆盖优先 incomplete；diagnostic-only plan 才为 diagnostic；退出表一致。

**Refactor 守卫**

- 删除可矛盾的 `covered` fallback；blocking 只由 plan/level/decision 派生。

### UT-005 Lifecycle adapter 不污染无关 gates

**目标行为**

Lifecycle assertion 以 owner 聚合，短路未执行 gate 保留 not_covered。

**关联**

- Design: `脚本迁移策略`
- 系统测试用例: ST-005、ST-007
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `scripts/tests/quality-gates-contract.test.ts`
- Modify: `scripts/test-wiki-lifecycle.mjs`
- Reference: `scripts/testing/quality-gates.mjs`

**测试代码蓝图**

```typescript
test("lifecycle 按 assertion owner 聚合而非批量阻断", () => {
  const summary = buildLifecycleSummary([singleRestoreFailure], { acceptancePlan: plan });
  expect(summary.gate_results.restore_validity.decision).toBe("blocker");
  expect(summary.gate_results.query_route_contract.decision).not.toBe("blocker");
});
```

**测试数据 / Fixture / Mock 边界**

- 使用离线 summary fixture，不运行真实项目。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/quality-gates-contract.test.ts -t "lifecycle 按 assertion owner 聚合而非批量阻断"`

**预期 Red 失败**

- 失败测试名: `lifecycle 按 assertion owner 聚合而非批量阻断`
- 关键错误 / 断言差异: 预期只有 restore blocker，实际 query/status/artifact 同时 blocker。
- 失败原因: 当前实现按总 failedAssertions 批量赋值。

**Green 通过条件**

- adapter 只产生 evidence/failure，decision 由共享内核计算。

**Refactor 守卫**

- 不得重新读取全局失败总数设置 gate decision。

### UT-006 Orchestrator decision 与子进程退出一致

**目标行为**

核心场景 orchestrator 生成稳定 9/9 summary，并保证默认/report-only 退出语义一致。

**关联**

- Design: `Core Scenario Orchestrator`、`默认退出策略`
- 系统测试用例: ST-006、ST-007、ST-014
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `scripts/tests/core-scenario-acceptance-cli.test.ts`
- Modify: `scripts/run-core-scenario-acceptance.mjs`、三类脚本 adapters
- Reference: `scripts/tests/distribution.test.ts`

**测试代码蓝图**

```typescript
test("orchestrator 的进程退出码与 summary decision 一致", () => {
  const run = spawnSync(process.execPath, [scriptPath, "--fixture-plan", planPath, "--json"], { encoding: "utf8" });
  const summary = JSON.parse(run.stdout);
  expect(run.status).toBe(summary.exit_code);
  expect(summary.scenario_results).toHaveLength(9);
});
```

**测试数据 / Fixture / Mock 边界**

- 临时离线 JSON plan/result fixture；不联网、不运行真实 reference 项目。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/core-scenario-acceptance-cli.test.ts -t "orchestrator 的进程退出码与 summary decision 一致"`

**预期 Red 失败**

- 失败测试名: `orchestrator 的进程退出码与 summary decision 一致`
- 关键错误 / 断言差异: `expected false to be true`，orchestrator 尚不存在；接入 v1 后非 pass 仍可能 exit 0。
- 失败原因: 最终编排/共享退出尚未实现。

**Green 通过条件**

- pass/blocker/incomplete/diagnostic 默认 0/1/2/2；report-only 不篡改 decision；summary 9/9。

**Refactor 守卫**

- 相同输入稳定排序和输出；fixture runner 与真实 adapters 共用聚合器。

### UT-007 CS-01/02/03/04/05 只消费公开场景合同

**目标行为**

Init/update/query/graph 场景通过公开 Runtime transport 提供有界证据，不泄漏旧字段或内部 DTO。

**关联**

- Design: `CS-01` 至 `CS-05`、`Rust Scenario Fixtures`
- 系统测试用例: ST-008、ST-009
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/core_scenario_acceptance.rs`
- Modify: 必要时 `crates/wiki-runtime/src/transport/cli.rs`、query/update 主链
- Reference: `crates/wiki-runtime/tests/acceptance/command_contract.rs`

**测试代码蓝图**

```rust
#[test]
fn canonical_core_scenarios_stay_within_public_contract() {
    // 动态仓库 init -> status/query -> source delta/update；仅断言公开 payload。
    assert!(payload["route_groups"].is_array());
    assert!(payload.get("provenance_summary").is_none());
}
```

**测试数据 / Fixture / Mock 边界**

- 真实 tempdir/SQLite；不 mock Runtime workflow；复用 acceptance helper。

**运行命令**

`cargo test -p wiki-runtime --test acceptance core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract -- --exact`

**预期 Red 失败**

- 失败测试名: `core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract`
- 关键错误 / 断言差异: 新 module/test 未注册或某场景缺 route/evidence/状态串联。
- 失败原因: 核心场景聚合 fixture 尚未建立；不是环境错误。

**Green 通过条件**

- CS-01..05 的公开入口、formal evidence、状态和 deferred 边界均通过。

**Refactor 守卫**

- 不新增产品 action/richer DTO；不以内部 `matched_*` 作为通过证据。

### UT-008 Pitfall、规范与结构化 conflict 串联

**目标行为**

CS-06/07/08 的 pitfall/policy/convention 和结构化 conflict 可从 managed edit 追踪到 formal/query/governance evidence。

**关联**

- Design: `CS-06/07/08`
- 系统测试用例: ST-010、ST-011
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/core_scenario_acceptance.rs`
- Modify: 必要时 `crates/wiki-runtime/src/workflows/sync.rs`、query/governance 主链
- Reference: `crates/wiki-runtime/tests/runtime/editable_runtime.rs`

**测试代码蓝图**

```rust
#[test]
fn declared_knowledge_and_structured_conflict_are_traceable() {
    // 写 pitfall/policy/convention markers，sync/update/query；创建并清除 parallel conflict。
    assert!(declared_kinds.contains(&"pitfall"));
    assert_eq!(query["recommended_action"], "review_governance");
}
```

**测试数据 / Fixture / Mock 边界**

- 动态 managed markers；非法/重复 ID、derived-owned edit 为负例；不加入自然语言语义 mock。

**运行命令**

`cargo test -p wiki-runtime --test acceptance core_scenario_acceptance::declared_knowledge_and_structured_conflict_are_traceable -- --exact`

**预期 Red 失败**

- 失败测试名: `core_scenario_acceptance::declared_knowledge_and_structured_conflict_are_traceable`
- 关键错误 / 断言差异: 缺 `pitfall` query evidence、conflict 双方 refs 或 clear 后残留。
- 失败原因: 聚合 fixture/必要主链边界尚未完成。

**Green 通过条件**

- 三类 declared 记录与结构化 conflict 全链可追踪，非法输入原子拒绝。

**Refactor 守卫**

- 不新增专用 authoring CLI 或任意语义冲突检测。

### UT-009 A/B 正式产物恢复到 level1

**目标行为**

B 不复制 cache，仅凭 A 的完整正式产物恢复 level1，并如实报告 index/fusion/action。

**关联**

- Design: `A/B restore fixture`
- 系统测试用例: ST-012
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/core_scenario_acceptance.rs`
- Modify: 必要时 `crates/wiki-runtime/src/workflows/status.rs`、restore/query helper
- Reference: `crates/wiki-runtime/tests/runtime/status_and_update.rs`

**测试代码蓝图**

```rust
#[test]
fn formal_artifacts_restore_second_worktree_to_level1() {
    // A init；B 复制源码+formal artifacts，明确排除 cache；B status/query。
    assert_eq!(status["readiness"]["restored_level"], "level1");
    assert_eq!(status["recommended_action"], "rebuild");
}
```

**测试数据 / Fixture / Mock 边界**

- 两个 tempdir、真实 formal artifacts/SQLite；copy helper 必须排除 cache/trace/temp reports。

**运行命令**

`cargo test -p wiki-runtime --test acceptance core_scenario_acceptance::formal_artifacts_restore_second_worktree_to_level1 -- --exact`

**预期 Red 失败**

- 失败测试名: `core_scenario_acceptance::formal_artifacts_restore_second_worktree_to_level1`
- 关键错误 / 断言差异: 预期 level1/rebuild，实际可能 missing/fresh 或跨路径恢复失败。
- 失败原因: 尚无真实双工作副本验收/必要实现修正。

**Green 通过条件**

- B cache 重建；knowledge/projection ready、index missing、fusion degraded、level1/rebuild；无 index route。

**Refactor 守卫**

- 不修改 A manifest 路径绕过校验，不复制任何派生 cache。

### UT-010 A/B 页面与源码漂移 fail closed

**目标行为**

正式页面漂移拒绝可信恢复，源码漂移产生 needs_update/update。

**关联**

- Design: `A/B restore 最小用例 2/3`
- 系统测试用例: ST-013
- Tasks: 5.4 Red / 5.5 Green / 5.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/core_scenario_acceptance.rs`
- Modify: 必要时 `crates/wiki-runtime/src/workflows/status.rs`、artifact validation/restore helper
- Reference: UT-009 helper

**测试代码蓝图**

```rust
#[test]
fn formal_restore_rejects_page_drift_and_reports_source_drift() {
    assert_eq!(page_drift["recommended_action"], "rebuild");
    assert_eq!(source_drift["state"], "needs_update");
    assert_eq!(source_drift["recommended_action"], "update");
}
```

**测试数据 / Fixture / Mock 边界**

- 基于 UT-009 builder 创建两条独立分支；不得修改 manifest 或放宽 hash 断言。

**运行命令**

`cargo test -p wiki-runtime --test acceptance core_scenario_acceptance::formal_restore_rejects_page_drift_and_reports_source_drift -- --exact`

**预期 Red 失败**

- 失败测试名: `core_scenario_acceptance::formal_restore_rejects_page_drift_and_reports_source_drift`
- 关键错误 / 断言差异: `expected needs_update/update, got <state/action>` 或页面漂移错误恢复。
- 失败原因: 跨副本 drift 分类尚未由 fixture/实现证明。

**Green 通过条件**

- 页面 drift fail closed；source drift 准确给出 update；无模糊多状态放行。

**Refactor 守卫**

- 不降低断言、不改 fixture manifest、不接受多个互斥状态。

### UT-011 Wiki/capability authority 收口

**目标行为**

长期 Wiki 投影与源码 matrix/gate v2 保持一致，旧 gate/query authority 被当前文档移除。

**关联**

- Design: `Wiki/Capability Contract`、`兼容性设计`
- 系统测试用例: ST-015
- Tasks: 6.1 Red / 6.2 Green / 6.3 Refactor

**Files**

- Test: `scripts/tests/core-scenario-wiki-contract.test.ts`
- Modify: `.wiki/06-设计文档/03-核心场景.md`、`.wiki/02-开发指南/01-测试与验收.md`、`.wiki/02-开发指南/02-脚本与工作流.md`、`.wiki/05-规格基线/capabilities/workflow-verification/spec.md`
- Reference: `scripts/testing/core-scenario-acceptance.mjs`、`scripts/testing/quality-gates.mjs`

**测试代码蓝图**

```typescript
test("Wiki 与 capability 只发布当前场景矩阵和 gate v2 authority", () => {
  expect(coreScenarioPage).toContain("CS-01");
  expect(coreScenarioPage).toContain("CS-09");
  expect(workflowSpec).toContain("knowledge-quality-gates.v2");
  expect(currentDocs).not.toContain("`provenance_summary`");
});
```

**测试数据 / Fixture / Mock 边界**

- 固定路径读取当前 Wiki/源码；不扫描或修改历史 archive。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/core-scenario-wiki-contract.test.ts`

**预期 Red 失败**

- 失败测试名: `Wiki 与 capability 只发布当前场景矩阵和 gate v2 authority`
- 关键错误 / 断言差异: 当前核心场景仍标草案，workflow spec 缺 v2 且仍含旧字段/冲突门禁。
- 失败原因: 长期 authority 尚未同步。

**Green 通过条件**

- 9 场景等级、gate v2、脚本职责和 canonical query 边界与源码一致。

**Refactor 守卫**

- 不复制测试报告/设计全文，不修改 archive，不把 reliability/host/documentation-closure 内容提前纳入。

## 测试辅助边界

- 允许新增纯对象 fixture、离线 JSON acceptance plan、动态 tempdir 仓库和 repo-relative evidence helper。
- 允许复用现有 acceptance/runtime helper；共享 helper 只在消除真实重复时抽取。
- 不 mock Runtime workflow、SQLite restore、hash 校验或 query transport。
- 不运行网络、真实 reference 仓库或浏览器。

## 不纳入单元测试的内容

- full workspace tests、lint、fmt、release build、distribution 和 UniSpec validate 在 verification 阶段执行。
- CS-01 的主观“用户理解”只以自动化正式产物/readiness 代理证据覆盖，不进行用户研究。
- 宿主 trigger parity、自然语言语义冲突、专用 authoring UX、richer query 和完整 reliability lifecycle 是明确非目标。
