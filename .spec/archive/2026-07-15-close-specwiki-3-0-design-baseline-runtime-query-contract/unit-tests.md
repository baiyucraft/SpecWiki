# close-specwiki-3-0-design-baseline-runtime-query-contract 单元测试设计

## 测试总览

TDD 蓝图覆盖共享 DTO、真实 BM25/ranking、Runtime state/fallback、transport errors、TS strict parser、Agents 字段和 Wiki authority。每个 UT 对应 system test 与 tasks 的 Red-Green-Refactor 组。

## 单元测试用例

### UT-001 Query DTO 闭集与 route group ranking 元数据

**目标行为**

共享 DTO 支持 module route/ref、闭集 provenance、rank/optional score 和必备 group ranking/count/truncation 字段。

**关联**

- Design: Route 与结果闭集、数据设计
- 系统测试用例: ST-001、ST-003、ST-004
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-model/tests/query_contract.rs`
- Modify: `crates/wiki-model/src/domain/query.rs`
- Reference: `./design.md`

**测试代码蓝图**

```rust
#[test]
fn query_group_serializes_ranking_and_module_contract() {
    let group = module_group_fixture();
    let value = serde_json::to_value(group).unwrap();
    assert_eq!(value["route_tag"], "index_module_hit");
    assert_eq!(value["ranking_basis"], "structural_match");
    assert_eq!(value["score_direction"], "none");
    assert_eq!(value["results"][0]["rank"], 1);
}
```

**测试数据 / Fixture / Mock 边界**

- 只构造 DTO，不 mock Runtime。

**运行命令**

`cargo test -p wiki-model --test query_contract`

**预期 Red 失败**

- 失败测试名: `query_group_serializes_ranking_and_module_contract`
- 关键错误 / 断言差异: 缺少 enum variants 和 ranking/rank 字段，编译或序列化断言失败。
- 失败原因: 新主合同尚未实现。

**Green 通过条件**

- 新闭集和字段稳定 serde，旧自由字符串/可选 score_basis 不再存在。

**Refactor 守卫**

- 不把 workflow 私有字段放入 wiki-model，不加入 deferred routes。

### UT-002 真实 BM25 次序和 source 排序保持

**目标行为**

真实 SQLite BM25 lower-is-better 次序贯穿 index query，source 命中不再被路径排序覆盖。

**关联**

- Design: Score 与 confidence
- 系统测试用例: ST-004
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`、`crates/wiki-index/tests/query.rs`
- Modify: `crates/wiki-index/src/query.rs`
- Reference: `crates/wiki-runtime/src/storage/sqlite_store.rs`

**测试代码蓝图**

```rust
#[test]
fn source_query_preserves_lower_is_better_bm25_order() {
    let hits = query_sources_from_real_sqlite_fixture("payment");
    assert!(hits.windows(2).all(|pair| pair[0].score <= pair[1].score));
    assert_ne!(hits, sorted_by_path(hits.clone()));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用真实临时 SQLite 和 FTS5，不伪造正分数。

**运行命令**

`cargo test -p wiki-runtime --test runtime sqlite_storage && cargo test -p wiki-index --test query`

**预期 Red 失败**

- 失败测试名: `source_query_preserves_lower_is_better_bm25_order`
- 关键错误 / 断言差异: 返回顺序等于 path order 或不满足 raw BM25 ascending。
- 失败原因: `lookup_sources` 无条件 path sort。

**Green 通过条件**

- FTS 命中保持数据库相关性顺序，结构化 fallback 仍确定性排序。

**Refactor 守卫**

- 不改变 storage raw BM25，不把绝对 rank 写入业务阈值。

### UT-003 Runtime route group 排序、去重、截断与 module projection

**目标行为**

Runtime 为 module 等 routes 生成唯一、稳定、有 ranking metadata 的 groups，不暴露跨 route flat results。

**关联**

- Design: Route 与结果闭集、Score 与 confidence
- 系统测试用例: ST-003、ST-004
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`
- Reference: `crates/wiki-index/src/query.rs`

**测试代码蓝图**

```rust
#[test]
fn query_groups_module_hits_and_assigns_stable_route_local_ranks() {
    let query = run_query(module_fixture(), "payments").unwrap();
    let group = route(&query, IndexModuleHit);
    assert_eq!(group.results[0].rank, 1);
    assert_eq!(group.returned_count, group.results.len());
    assert_unique_route_refs(group);
}
```

**测试数据 / Fixture / Mock 边界**

- 复用真实 init graph fixture，不 mock index result。

**运行命令**

`cargo test -p wiki-runtime --test runtime query_groups_module_hits_and_assigns_stable_route_local_ranks`

**预期 Red 失败**

- 失败测试名: 上述测试
- 关键错误 / 断言差异: module route/rank/count fields 不存在。
- 失败原因: 当前 modules 没投影到 public DTO，group metadata 不完整。

**Green 通过条件**

- module route 存在，group 内 rank/count/truncated/dedup 正确。

**Refactor 守卫**

- 固定 route order 不被当成 relevance；flat internal results 只供装配。

### UT-004 Readiness 与逐 route provenance/answer 矩阵

**目标行为**

ready/stale/fallback/governance blocker 场景中的 result layer/state、trust、answer 和 action 一致且正交。

**关联**

- Design: Readiness、trust 与 provenance
- 系统测试用例: ST-003、ST-005
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`、`crates/wiki-runtime/tests/governance_workflows.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`
- Reference: `crates/wiki-runtime/src/domain/runtime_profile.rs`

**测试代码蓝图**

```rust
#[test]
fn stale_routes_project_actual_layer_state_and_fallback_is_degraded() {
    let stale = query_stale_fixture();
    assert_all_index_states(&stale, Stale);
    let fallback = query_fallback_fixture();
    assert_eq!(only_result(&fallback).provenance.state, Fallback);
    assert_eq!(fallback.query_trust, StaleButQueryable);
}
```

**测试数据 / Fixture / Mock 边界**

- 复用现有 graph phase、fallback 和 governance fixtures。

**运行命令**

`cargo test -p wiki-runtime --test runtime query_ && cargo test -p wiki-runtime --test governance_workflows`

**预期 Red 失败**

- 失败测试名: `stale_routes_project_actual_layer_state_and_fallback_is_degraded`
- 关键错误 / 断言差异: source/graph state 实际为 `ready`，fallback state 为 `derived`。
- 失败原因: provenance state 当前硬编码且非闭集 readiness 投影。

**Green 通过条件**

- 每个 route 使用实际 layer/state；fallback degraded；governance blocker 不降 core trust。

**Refactor 守卫**

- 不在 TS/Agents 复制 state matrix。

### UT-005 Transport 主合同与 typed query errors

**目标行为**

Rust transport 只返回 canonical query 字段，route_groups 空时仍存在，term/error 分类稳定。

**关联**

- Design: 主合同与内部 rich report、错误和空结果
- 系统测试用例: ST-001、ST-002
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/command_contract.rs`
- Modify: `crates/wiki-runtime/src/transport/query_payload.rs`、`cli.rs`、`dto.rs`
- Reference: `./design.md`

**测试代码蓝图**

```rust
#[test]
fn query_transport_is_canonical_and_types_invalid_or_not_ready_inputs() {
    assert_eq!(query_without_term().error_kind, InvalidArgument);
    assert_eq!(query_uninitialized().error_kind, IndexNotReady);
    let payload = query_without_hits().data.unwrap();
    assert!(payload["route_groups"].is_array());
    assert_no_legacy_query_fields(&payload);
}
```

**测试数据 / Fixture / Mock 边界**

- 调用真实 transport handler，临时仓库隔离。

**运行命令**

`cargo test -p wiki-runtime --test acceptance command_contract::query_`

**预期 Red 失败**

- 失败测试名: 上述测试
- 关键错误 / 断言差异: 空 term 成功；NotFound 变 workflow_failed；旧字段仍存在；空 groups 被省略。
- 失败原因: transport 尚无 query 专用 contract/encoder。

**Green 通过条件**

- canonical payload 与 typed errors 全部匹配设计。

**Refactor 守卫**

- 内部 rich report 可保留但不能泄漏到 transport。

### UT-006 TypeScript parser fail-closed 与 AnswerEnvelope

**目标行为**

TS parser 接受完整 canonical payload，拒绝缺 route_groups/answer、未知闭集和旧 runtime payload。

**关联**

- Design: Parser contract
- 系统测试用例: ST-001、ST-002、ST-005
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `packages/spec-wiki/src/runtime/parseResult.test.ts`
- Modify: `packages/spec-wiki/src/runtime/parseResult.ts`
- Reference: Rust query DTO contract

**测试代码蓝图**

```typescript
test("query parser requires canonical groups and answer", () => {
  expect(parseResult(canonicalQuery())).toMatchObject({ ok: true });
  expect(() => parseResult(without(canonicalQuery(), "route_groups"))).toThrow();
  expect(() => parseResult(without(canonicalQuery(), "answer"))).toThrow();
});
```

**测试数据 / Fixture / Mock 边界**

- 纯 JSON fixtures，不 mock core invocation。

**运行命令**

`pnpm --filter spec-wiki exec vitest run src/runtime/parseResult.test.ts`

**预期 Red 失败**

- 失败测试名: `query parser requires canonical groups and answer`
- 关键错误 / 断言差异: 缺字段被静默补空或 answer 未校验。
- 失败原因: 当前 parser 兼容性默认掩盖协议漂移。

**Green 通过条件**

- canonical payload 严格通过，所有破坏性 fixture 明确失败。

**Refactor 守卫**

- parser 不重算 ranking/readiness/answer。

### UT-007 CLI、human renderer 与 Agents 薄消费 canonical fields

**目标行为**

CLI/renderer/生成资产只消费 canonical groups/answer，不出现旧派生字段或 deferred richer 字段。

**关联**

- Design: Parser contract、Richer query 延期边界
- 系统测试用例: ST-001、ST-002、ST-006
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/agents/shared/commandAssets.test.ts`、`assets.test.ts`、`humanRenderer.test.ts`、`cli.test.ts`
- Modify: `commandAssets.ts`、`humanRenderer.ts`、CLI/parser types
- Reference: `.wiki/06-设计文档/02-Agents设计.md`

**测试代码蓝图**

```typescript
test("query skill consumes canonical groups and defers richer inputs", () => {
  const skill = renderHostActionSkill("query");
  expect(skill).toContain("`route_groups`");
  expect(skill).toContain("`answer`");
  expect(skill).not.toMatch(/matched_pages|provenance_summary|owner|entrypoint|impact/);
});
```

**测试数据 / Fixture / Mock 边界**

- 复用资产渲染器；CLI 测试只 mock core boundary，不重写 payload parser。

**运行命令**

`pnpm --filter spec-wiki test`

**预期 Red 失败**

- 失败测试名: `query skill consumes canonical groups and defers richer inputs`
- 关键错误 / 断言差异: skill 含旧字段且缺 canonical groups/answer。
- 失败原因: 生成资产仍固化旧合同。

**Green 通过条件**

- 三宿主生成资产、human output 和 CLI error parity 通过。

**Refactor 守卫**

- 宿主不复制 route 枚举或状态机，仍建议精确问题回到源码核验。

### UT-008 Canonical Wiki authority 与延期边界合同

**目标行为**

长期 Wiki 提供唯一 Runtime查询合同入口，Runtime/Agents 页面链接并遵守主字段、fusion/ranking/error/延期规则。

**关联**

- Design: 方案概述、Richer query 延期边界
- 系统测试用例: ST-001、ST-006
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `scripts/tests/runtime-query-contract.test.ts`
- Modify: `.wiki/06-设计文档/06-Runtime查询合同.md`、`01-Runtime设计.md`、`02-Agents设计.md`、`INDEX.md`
- Reference: `05-产品基线与设计治理.md`

**测试代码蓝图**

```typescript
test("Runtime query authority defines canonical and deferred contracts", () => {
  const page = readAuthority();
  expect(page).toContain("route_groups");
  expect(page).toContain("index_module_hit");
  expect(page).toMatch(/owner.*延期/s);
  expect(runtimeDesign()).toContain("./06-Runtime查询合同.md");
});
```

**测试数据 / Fixture / Mock 边界**

- 只读取固定 Wiki 和源码路径，不扫描 archive/upstream。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/runtime-query-contract.test.ts`

**预期 Red 失败**

- 失败测试名: 上述测试
- 关键错误 / 断言差异: canonical 页面不存在，Agents 仍列旧字段。
- 失败原因: 长期 authority 尚未建立。

**Green 通过条件**

- 页面、导航和源码合同一致；延期表明确。

**Refactor 守卫**

- 不复制 proposal/design/review 原文，不迁移全库 capability 历史。

## 测试辅助边界

- 允许扩展现有临时 repo、SQLite、graph phase 与 governance fixtures。
- 不允许用 mock 正分数替代真实 BM25 排名验证。
- contract test 只扫描当前 authority、代码和生成资产，不递归历史 archive/upstream。

## 不纳入单元测试的内容

- 完整 storybook/dagger/19 项目质量分析：由现有综合测试和后续场景/质量 change 负责。
- richer query 实现：明确延期，本轮只做 contract guard。
