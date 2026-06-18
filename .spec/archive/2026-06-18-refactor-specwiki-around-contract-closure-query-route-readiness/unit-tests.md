# refactor-specwiki-around-contract-closure-query-route-readiness 单元测试设计

## 测试总览

本文件服务于 TDD 实现模式，只描述计划中的单元测试蓝图，不在 plan 阶段写入真实测试文件。测试覆盖 `wiki-model` 的公开 query DTO、`wiki-runtime` 的 route fusion/readiness/fallback/governance placeholder、transport 边界，以及 `packages/spec-wiki` 的 TS parser guardrail。

## 单元测试用例

### UT-001 Query DTO serde 与闭集枚举

**目标行为**

验证 `wiki-model` 暴露稳定 query DTO，serde 字段名与公开 JSON 合同一致，未知 route tag/ref kind 会失败。

**关联**

- Design: Query Public Contract、数据模型设计、设计决策
- 系统测试用例: ST-001, ST-005
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-model/tests/query_contract.rs`
- Modify: `crates/wiki-model/src/domain/query.rs`, `crates/wiki-model/src/domain/mod.rs`
- Reference: `.spec/changes/refactor-specwiki-around-contract-closure-query-route-readiness/design.md`

**测试代码蓝图**

```rust
#[test]
fn query_result_dto_serializes_public_contract_fields() {
    let result = QueryResultDto {
        route_tag: QueryRouteTag::IndexPathHit,
        ref_kind: QueryRefKind::SourcePath,
        ref_id: "src/lib.rs".into(),
        label: "src/lib.rs".into(),
        score: 0.91,
        provenance: QueryProvenance::new("index", "ready"),
        confidence: QueryConfidence::High,
        recommended_action: RecommendedAction::OpenSourceRef,
        source_refs: vec![QuerySourceRef::source_path("src/lib.rs")],
    };

    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value["route_tag"], "index_path_hit");
    assert_eq!(value["ref_kind"], "source_path");
    assert!(value.get("source_refs").unwrap().is_array());
}

#[test]
fn query_route_tag_rejects_unknown_values() {
    let payload = r#""index_hit""#;
    let err = serde_json::from_str::<QueryRouteTag>(payload).unwrap_err();
    assert!(err.to_string().contains("unknown variant"));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用纯 DTO 构造，不 mock runtime。
- 可新增最小 helper 构造 `QueryProvenance` / `QuerySourceRef`，但 helper 必须在生产 DTO 之上。

**运行命令**

`cargo test -p wiki-model query_contract --test query_contract`

**预期 Red 失败**

- 失败测试名: `query_result_dto_serializes_public_contract_fields`
- 关键错误 / 断言差异: `cannot find type QueryResultDto` 或 serialized field `route_tag` 缺失。
- 失败原因: 公开 query DTO 和闭集枚举尚未实现。

**Green 通过条件**

- DTO serde 输出字段与公开合同一致。
- 未知 route tag/ref kind 反序列化失败。

**Refactor 守卫**

- DTO 继续归属 `wiki-model`，不能挪回 `wiki-runtime` 私有 transport。
- 字段名不能被后续 adapter 反向重命名。

### UT-002 Runtime route fusion 映射 index/knowledge/projection 为公开 DTO

**目标行为**

验证 runtime query workflow 输出 `route_groups` 和 `results`，并把 index、knowledge、projection 命中映射为公开 route tag。

**关联**

- Design: Query Fusion Adapter、处理流程、业务规则实现
- 系统测试用例: ST-001
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`
- Reference: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`

**测试代码蓝图**

```rust
#[test]
fn query_fusion_outputs_route_groups_and_results() {
    let runtime = runtime_fixture_with_ready_index_and_knowledge();
    let report = runtime.query("runtime readiness").unwrap();

    assert!(report.route_groups.iter().any(|g| g.route_tag == QueryRouteTag::IndexPathHit));
    assert!(report.results.iter().any(|r| r.route_tag == QueryRouteTag::KnowledgeDeclaredHit));
    assert!(report.results.iter().all(|r| !r.source_refs.is_empty()));
    assert_ne!(report.provenance_summary, "");
}
```

**测试数据 / Fixture / Mock 边界**

- 复用现有 runtime temp repo fixture。
- 允许构造最小 knowledge/projection fixture；不 mock query fusion 本身。

**运行命令**

`cargo test -p wiki-runtime query_fusion_outputs_route_groups_and_results --test runtime`

**预期 Red 失败**

- 失败测试名: `query_fusion_outputs_route_groups_and_results`
- 关键错误 / 断言差异: `no field route_groups on type QueryReport` 或 `results` 为空。
- 失败原因: runtime 仍只输出粗粒度摘要，未装配公开 route DTO。

**Green 通过条件**

- runtime rich report 同时包含 `route_groups` 与 `results`。
- `provenance_summary` 保留但可由主合同派生。

**Refactor 守卫**

- `wiki-index` 不知道公开 route tag；映射必须留在 runtime fusion 边界。

### UT-003 Index readiness gate 禁止 missing/stale/blocked 输出 index route

**目标行为**

验证 index 非 ready 时 runtime 不输出 `index_symbol_hit`、`index_path_hit`、`index_graph_hit`，且 trust/action 显式降级。

**关联**

- Design: Query Fusion Adapter、异常处理设计、业务规则实现
- 系统测试用例: ST-002
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`, `crates/wiki-runtime/src/domain/runtime_profile.rs`
- Reference: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`

**测试代码蓝图**

```rust
#[test]
fn query_does_not_emit_index_routes_when_index_is_not_ready() {
    for state in [IndexReadiness::Missing, IndexReadiness::Stale, IndexReadiness::Blocked] {
        let runtime = runtime_fixture_with_index_state_and_knowledge(state);
        let report = runtime.query("known topic").unwrap();

        assert!(report.results.iter().all(|r| !r.route_tag.is_index_route()));
        assert!(report.query_trust.is_degraded());
        assert_eq!(report.recommended_action, RecommendedAction::RebuildIndex);
    }
}
```

**测试数据 / Fixture / Mock 边界**

- 可用 enum/helper 注入 index readiness 状态。
- knowledge/projection fixture 保持真实查询路径，避免把无结果误判为 gate 生效。

**运行命令**

`cargo test -p wiki-runtime query_does_not_emit_index_routes_when_index_is_not_ready --test runtime`

**预期 Red 失败**

- 失败测试名: `query_does_not_emit_index_routes_when_index_is_not_ready`
- 关键错误 / 断言差异: result 中仍包含 `index_path_hit` 或 `query_trust` 未降级。
- 失败原因: runtime route gate 尚未根据 readiness 阻止 index route。

**Green 通过条件**

- missing/stale/blocked 三种状态都无 index route。
- 仍可返回 knowledge/projection 时必须标记降级 trust/action。

**Refactor 守卫**

- 不通过删除全部结果来通过测试；knowledge/projection 可用结果仍应保留。

### UT-004 Fallback route 显式为 rendered_page_debug_fallback 并降级

**目标行为**

验证 Markdown 正文 fallback 不能伪装为 index/knowledge route，只能输出 `rendered_page_debug_fallback` 并降低 trust。

**关联**

- Design: Query Fusion Adapter、Human Explanation Contract、异常处理设计
- 系统测试用例: ST-003
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`
- Reference: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`

**测试代码蓝图**

```rust
#[test]
fn rendered_page_fallback_is_explicit_and_degraded() {
    let runtime = runtime_fixture_with_only_rendered_page_fallback();
    let report = runtime.query("orphan topic").unwrap();

    assert!(report.results.iter().all(|r| r.route_tag == QueryRouteTag::RenderedPageDebugFallback));
    assert!(report.query_trust.is_degraded());
    assert!(report.answer.contains("fallback"));
}
```

**测试数据 / Fixture / Mock 边界**

- fixture 中禁用 index/knowledge/projection 正向命中，只保留 rendered page 文本。
- 不 mock answer formatter，确保人类解释路径可观察。

**运行命令**

`cargo test -p wiki-runtime query_keeps_textual_page_fallback_degraded_even_with_graph_hits --test runtime`

**预期 Red 失败**

- 失败测试名: `rendered_page_fallback_is_explicit_and_degraded`
- 关键错误 / 断言差异: `page_fallback` 或 `knowledge_hit` 出现在结果中，或 trust 未降级。
- 失败原因: fallback 尚未显式 route 化。

**Green 通过条件**

- fallback-only 场景只输出 `rendered_page_debug_fallback`。
- answer/trust 表达降级。

**Refactor 守卫**

- fallback 不能被后续 formatter 或 transport 改名为旧泛化标签。

### UT-005 Governance not_enabled placeholder 不阻断 query

**目标行为**

验证 governance 未启用时，query 返回 `not_enabled` 或空 governance group，不扫描 `.spec` evidence，也不阻断普通结果。

**关联**

- Design: Query Fusion Adapter、业务规则实现、非目标边界
- 系统测试用例: ST-004
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`
- Reference: `.docs/design/governance-runtime-integration.md`

**测试代码蓝图**

```rust
#[test]
fn governance_not_enabled_does_not_block_query_results() {
    let runtime = runtime_fixture_without_governance_index();
    let report = runtime.query("known topic").unwrap();

    assert_eq!(report.readiness.governance_readiness, GovernanceReadiness::NotEnabled);
    assert!(report.results.iter().all(|r| !r.route_tag.is_governance_route()));
    assert!(report.results.iter().any(|r| r.route_tag == QueryRouteTag::KnowledgeDeclaredHit));
}
```

**测试数据 / Fixture / Mock 边界**

- 不读取 `.spec/changes` 作为 evidence。
- fixture 中保留一个普通 knowledge 命中以证明未阻断。

**运行命令**

`cargo test -p wiki-runtime query_reports_governance_not_enabled_without_blocking_query --test runtime`

**预期 Red 失败**

- 失败测试名: `governance_not_enabled_does_not_block_query_results`
- 关键错误 / 断言差异: governance readiness 缺失，或普通 result 被阻断。
- 失败原因: governance placeholder 尚未进入 query 主合同。

**Green 通过条件**

- governance readiness 可观察为 `not_enabled`。
- 普通 query 结果不受阻断。

**Refactor 守卫**

- 本测试不得引入 `.spec` evidence scanner 或 governance state machine。

### UT-006 Runtime transport 保留主合同且不重建语义

**目标行为**

验证 transport payload 从 runtime rich report 映射主合同，不重新计算 route/trust/action，不丢失 `source_refs`。

**关联**

- Design: Runtime Transport Slimmer、接口设计
- 系统测试用例: ST-005
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/acceptance/command_contract.rs`
- Modify: `crates/wiki-runtime/src/transport/query_payload.rs`
- Reference: `crates/wiki-runtime/src/workflows/query.rs`

**测试代码蓝图**

```rust
#[test]
fn query_transport_returns_slim_payload_but_internal_query_stays_rich() {
    let rich = rich_query_report_fixture_with_all_route_fields();
    let payload = QueryPayload::from_report(&rich);

    assert_eq!(payload.results[0].route_tag, rich.results[0].route_tag);
    assert_eq!(payload.results[0].source_refs, rich.results[0].source_refs);
    assert_eq!(payload.query_trust, rich.query_trust);
    assert_eq!(payload.provenance_summary, rich.provenance_summary);
}
```

**测试数据 / Fixture / Mock 边界**

- 允许用 rich report fixture 测 transport 映射。
- 不允许在 transport fixture 中手写与 runtime 不一致的 route/trust 逻辑。

**运行命令**

`cargo test -p wiki-runtime query_transport_returns_slim_payload_but_internal_query_stays_rich --test acceptance`

**预期 Red 失败**

- 失败测试名: `query_transport_returns_slim_payload_but_internal_query_stays_rich`
- 关键错误 / 断言差异: payload 没有 `results` / `route_groups`，或 `source_refs` 丢失。
- 失败原因: transport 仍使用旧 slim 字段，未承载主合同。

**Green 通过条件**

- transport payload 保留 route groups/results/readiness/trust/action/source refs。
- `provenance_summary` 仍为只读派生摘要。

**Refactor 守卫**

- transport 不新增独立 score/trust 推导。

### UT-007 TS parser 校验闭集合同并保留附加字段

**目标行为**

验证 `packages/spec-wiki` 只解析/校验 Rust 主合同，拒绝未知 route/ref kind，保留未知附加字段，不重新计算 trust。

**关联**

- Design: TS Parser Guardrail、接口详细定义
- 系统测试用例: ST-005
- Tasks: 5.4 Red / 5.5 Green / 5.6 Refactor

**Files**

- Test: `packages/spec-wiki/src/index.test.ts`
- Modify: `packages/spec-wiki/src/runtime/parseResult.ts`
- Reference: `packages/spec-wiki/src/agents/shared/commandAssets.ts`

**测试代码蓝图**

```typescript
test('parseResult preserves query route contract and extras', () => {
  const payload = parseResult(JSON.stringify({
    ok: true,
    route_groups: [{ route_tag: 'knowledge_declared_hit', results: [] }],
    results: [{
      route_tag: 'knowledge_declared_hit',
      ref_kind: 'knowledge_page',
      ref_id: '.wiki/INDEX.md',
      label: 'INDEX',
      score: 0.8,
      provenance: { layer: 'knowledge' },
      confidence: 'high',
      recommended_action: 'open_reference',
      source_refs: [{ ref_kind: 'knowledge_page', ref_id: '.wiki/INDEX.md' }],
      extra_field: 'preserve-me'
    }],
    query_trust: 'ready',
    recommended_action: 'open_reference'
  }));

  expect(payload.results[0].route_tag).toBe('knowledge_declared_hit');
  expect(payload.results[0].extra_field).toBe('preserve-me');
});

test('parseResult rejects unknown query route tag', () => {
  expect(() => parseResult(JSON.stringify({
    ok: true,
    results: [{ route_tag: 'index_hit', ref_kind: 'source_path' }]
  }))).toThrow(/unknown query route tag/i);
});
```

**测试数据 / Fixture / Mock 边界**

- 使用 JSON 字符串 fixture，不调用 Rust binary。
- parser 负向 fixture 只验证协议错误，不覆盖 runtime 业务。

**运行命令**

`pnpm --filter spec-wiki test -- index.test.ts`

**预期 Red 失败**

- 失败测试名: `parseResult preserves query route contract and extras`
- 关键错误 / 断言差异: `results` 字段丢失、未知字段被丢弃，或未知 route tag 未抛错。
- 失败原因: TS parser 尚未理解 query 主合同，仍只消费旧摘要。

**Green 通过条件**

- parser 保留 Rust 主合同与未知附加字段。
- 未知主 route/ref kind 早失败。

**Refactor 守卫**

- TS 不得新增 trust/action 计算逻辑。

## 测试辅助边界

- Rust runtime 测试复用现有 `query_sync_rebuild.rs` fixture helper，必须复用真实 query workflow。
- DTO serde 测试应尽量无 runtime 依赖，保持 `wiki-model` 边界清晰。
- TS parser 测试使用 JSON fixture，不引入端到端 Rust binary 调用。

## 不纳入单元测试的内容

- 真实 graph schema、imports/calls/heritage 与 graph scoring 算法不纳入本 change。
- governance evidence scanner、summary 生成、validate/archive 不纳入本 change。
- CLI 默认 help、Quick Start、一级命令产品体验不纳入本 change。
