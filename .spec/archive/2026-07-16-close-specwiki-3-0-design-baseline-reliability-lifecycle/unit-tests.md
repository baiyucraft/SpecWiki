# close-specwiki-3-0-design-baseline-reliability-lifecycle 单元测试设计

## 测试总览

本文件定义 TDD 所需的 reducer、authority、projection、resume、commit 和文档合同测试蓝图。每个 `UT-*` 绑定 `design.md` 决策、至少一个 `ST-*` 和 `tasks.md` 的 Red/Green/Refactor 任务；真实测试代码由 apply 阶段写入。

## 单元测试用例

### UT-001 ReliabilityAssessment 前置优先级与双轴 layer 状态

**目标行为**

同一组 evidence 只能产生唯一 runtime state、layer freshness/consumability 和 core action，formal invalid、workflow interrupted、Level 1、source stale、pending sync、health-only、ready 按设计顺序短路。

**关联**

- Design: 统一可靠性决策
- 系统测试用例: ST-001、ST-002、ST-009
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-runtime/src/domain/reliability.rs`
- Modify: `crates/wiki-runtime/src/domain/reliability.rs`、`domain/runtime_profile.rs`
- Reference: `workflows/status.rs`、`workflows/query.rs`

**测试代码蓝图**

```rust
#[test]
fn assessment_uses_prerequisite_order_and_separates_freshness_from_consumability() {
    for case in reliability_decision_cases() {
        let assessment = assess_runtime_reliability(case.evidence);
        assert_eq!(assessment.runtime_state, case.state);
        assert_eq!(assessment.core_action, case.action);
        assert_eq!(assessment.layers, case.layers);
    }
}
```

**测试数据 / Fixture / Mock 边界**

- 纯 struct table，无文件系统和网络 mock；每个 case 只改变一组 evidence。

**运行命令**

`cargo test -p wiki-runtime domain::reliability::tests::assessment_uses_prerequisite_order`

**预期 Red 失败**

- 失败测试名: `assessment_uses_prerequisite_order_and_separates_freshness_from_consumability`
- 关键错误 / 断言差异: `assess_runtime_reliability`/layer enums 不存在，或 stale index 被投影为 direct ready。
- 失败原因: 统一 reducer 和双轴状态尚未实现。

**Green 通过条件**

- 全部表项返回唯一预期 state/action/layer，不依赖字符串 fallback。

**Refactor 守卫**

- status/query 不得重新实现相同优先级；所有 enum exhaustive match。

### UT-002 route-local query trust 与 action 投影

**目标行为**

query trust 由实际 supporting layers决定，governance conflict与无关 core trust正交，result/answer action不复制错误全局值。

**关联**

- Design: 统一可靠性决策
- 系统测试用例: ST-001、ST-009
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `crates/wiki-runtime/src/domain/reliability.rs`、`crates/wiki-runtime/tests/runtime/reliability_lifecycle.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`、`status.rs`
- Reference: `.wiki/06-设计文档/06-Runtime查询合同.md`

**测试代码蓝图**

```rust
#[test]
fn query_trust_uses_supporting_layers_and_keeps_governance_orthogonal() {
    assert_eq!(trust_for(&[ready_index()], governance_conflict()), QueryTrust::Ready);
    assert_eq!(trust_for(&[stale_index()], no_governance()), QueryTrust::StaleButQueryable);
    assert_eq!(trust_for(&[], blocked_core()), QueryTrust::Blocked);
}
```

**测试数据 / Fixture / Mock 边界**

- 纯 route support refs；integration只使用临时 SQLite fixture。

**运行命令**

`cargo test -p wiki-runtime query_trust_uses_supporting_layers`

**预期 Red 失败**

- 失败测试名: `query_trust_uses_supporting_layers_and_keeps_governance_orthogonal`
- 关键错误 / 断言差异: 当前 `effective_query_trust` 依据全局 state/action产生 `stale_but_queryable` 或错误 action。
- 失败原因: route-local assessment投影尚未接入。

**Green 通过条件**

- 三种 trust 和 result/answer action均符合 canonical query合同。

**Refactor 守卫**

- 公开 query payload 字段不变化，TS parser无需新增推导。

### UT-003 provider outcome exhaustive decision table

**目标行为**

production只有有效 output的 completed/no-further-tools可 accepted；所有无output stop/failure blocked，development fixture仅 diagnostic。

**关联**

- Design: A9 failure/degradation matrix
- 系统测试用例: ST-008、ST-009
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/src/domain/research.rs`
- Modify: `crates/wiki-runtime/src/domain/research.rs`、`workflows/research_provider.rs`
- Reference: `src/llm/mod.rs`

**测试代码蓝图**

```rust
#[test]
fn production_never_accepts_research_without_valid_provider_output() {
    for reason in all_research_stop_reasons() {
        let decision = reduce_research_outcome(production_evidence(reason, None));
        assert_ne!(decision.decision, ResearchDecision::Accepted);
    }
}
```

**测试数据 / Fixture / Mock 边界**

- 枚举所有 stop reasons、production/development、valid/invalid output；不调用真实 provider。

**运行命令**

`cargo test -p wiki-runtime production_never_accepts_research_without_valid_provider_output`

**预期 Red 失败**

- 失败测试名: `production_never_accepts_research_without_valid_provider_output`
- 关键错误 / 断言差异: reducer不存在，或 `not_run/turn_budget_exhausted` 仍经 structural seed被视为 ready。
- 失败原因: stop reason到workflow decision尚未收口。

**Green 通过条件**

- exhaustive table无遗漏，valid output gate为accepted必要条件。

**Refactor 守卫**

- 禁止 `_ => accepted`；新增 stop reason必须触发编译期覆盖。

### UT-004 ProviderFailureKind 到 artifact/gate/action 映射

**目标行为**

unavailable、transport、timeout、tool_error、context_limit具有稳定failure kind，并映射到一致summary reason、unit gate和原workflow retry action。

**关联**

- Design: A9 failure/degradation matrix
- 系统测试用例: ST-008、ST-009
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `crates/wiki-runtime/src/domain/research.rs`、`tests/runtime/reliability_lifecycle.rs`
- Modify: `src/llm/mod.rs`、`workflows/research_provider.rs`、`workflows/page_render.rs`
- Reference: `wiki-model/src/domain/knowledge_artifact.rs`

**测试代码蓝图**

```rust
#[test]
fn provider_failure_kind_is_preserved_across_gate_summary_and_retry_action() {
    let result = project_failure(ProviderFailureKind::Timeout, WorkflowAction::Update);
    assert_eq!(result.gate.reason_kind, "provider_timeout");
    assert_eq!(result.summary.runtime_state, "runtime_incomplete");
    assert_eq!(result.action, RecommendedAction::Update);
}
```

**测试数据 / Fixture / Mock 边界**

- fake provider errors，不解析真实HTTP文本；context trim仅允许一次。

**运行命令**

`cargo test -p wiki-runtime provider_failure_kind_is_preserved`

**预期 Red 失败**

- 失败测试名: `provider_failure_kind_is_preserved_across_gate_summary_and_retry_action`
- 关键错误 / 断言差异: 所有错误被折为`provider_error`，或action一律rebuild。
- 失败原因: typed failure和原action映射缺失。

**Green 通过条件**

- 所有failure kind稳定序列化并贯穿artifact/gate/status。

**Refactor 守卫**

- development diagnostic与production blocker共用evidence但不能共用success decision。

### UT-005 declared canonical replacement graph 与 authority head

**目标行为**

single active、A->B->C、all deprecated、parallel head、missing/cross-scope/cycle产生唯一正确authority结果。

**关联**

- Design: Declared 生命周期与 authority
- 系统测试用例: ST-003
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-knowledge/tests/declared_authority.rs`
- Modify: `crates/wiki-knowledge/src/declared_authority.rs`、`wiki-model/src/domain/knowledge_artifact.rs`
- Reference: `crates/wiki-knowledge/tests/declared_writeback.rs`

**测试代码蓝图**

```rust
#[test]
fn authority_evaluator_normalizes_replacement_chains_and_zero_head() {
    assert_unique_head(single_active(), "A");
    assert_unique_head(chain("A", "B", "C"), "C");
    assert_authority(all_deprecated(), AuthorityState::None);
    assert_conflict(parallel_heads());
    assert!(evaluate(cycle()).is_err());
}
```

**测试数据 / Fixture / Mock 边界**

- 纯 declared records，canonical scope固定；不读Markdown。

**运行命令**

`cargo test -p wiki-knowledge --test declared_authority`

**预期 Red 失败**

- 失败测试名: `authority_evaluator_normalizes_replacement_chains_and_zero_head`
- 关键错误 / 断言差异: 模块/authority types不存在，或replaced head未被识别。
- 失败原因: group-level graph evaluator尚未实现。

**Green 通过条件**

- graph与状态校验确定且稳定排序。

**Refactor 守卫**

- wiki-knowledge纯函数不读写storage/Markdown。

### UT-006 authoring missing/detached 禁止物理 prune

**目标行为**

authority block消失保留record并变missing/conflict；非authority block消失变detached；相同authoring_id恢复bound。

**关联**

- Design: Declared 删除语义
- 系统测试用例: ST-004、ST-005
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/editable_runtime.rs`、`declared_projection_lifecycle.rs`
- Modify: `crates/wiki-runtime/src/workflows/sync.rs`、`storage/knowledge_artifacts.rs`
- Reference: 现有 block removal tests

**测试代码蓝图**

```rust
#[test]
fn removing_authority_block_preserves_formal_record_and_requires_governance_review() {
    let before = sync_active_record();
    remove_declared_block();
    let report = run_sync();
    assert_record_preserved(before.record_id, AuthoringState::Missing);
    assert_eq!(report.recommended_action, ReviewGovernance);
}
```

**测试数据 / Fixture / Mock 边界**

- 临时repo和真实managed marker parser；不mock sync merge。

**运行命令**

`cargo test -p wiki-runtime --test runtime removing_authority_block_preserves_formal_record`

**预期 Red 失败**

- 失败测试名: `removing_authority_block_preserves_formal_record_and_requires_governance_review`
- 关键错误 / 断言差异: record count从1变0。
- 失败原因: 当前sync仍将消失block解释为prune。

**Green 通过条件**

- missing/detached/restore三条路径均保留formal lifecycle。

**Refactor 守卫**

- 不新增hard purge或旧prune兼容分支。

### UT-007 governance events open/resolved/reopen 幂等

**目标行为**

authority/conflict变化生成append-only events，no-op不重复，resolved后reopen产生新occurrence且current open view正确。

**关联**

- Design: Declared conflict history
- 系统测试用例: ST-004、ST-005
- Tasks: 3.7 Red / 3.8 Green / 3.9 Refactor

**Files**

- Test: `crates/wiki-knowledge/tests/declared_authority.rs`、`wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`
- Modify: `wiki-model/src/domain/knowledge_artifact.rs`、`wiki-knowledge/src/declared_authority.rs`、`wiki-runtime/src/storage/knowledge_artifacts.rs`
- Reference: current `conflict-records.jsonl`

**测试代码蓝图**

```rust
#[test]
fn governance_event_history_is_append_only_and_noop_sync_is_idempotent() {
    let events = reconcile(open_then_resolve_then_reopen());
    assert_eq!(event_kinds(&events), [ConflictOpened, ConflictResolved, ConflictOpened]);
    assert_unique_event_ids(&events);
}
```

**测试数据 / Fixture / Mock 边界**

- 固定snapshot ids和sequence，时间字段由test clock注入。

**运行命令**

`cargo test -p wiki-knowledge --test declared_authority governance_event_history`

**预期 Red 失败**

- 失败测试名: `governance_event_history_is_append_only_and_noop_sync_is_idempotent`
- 关键错误 / 断言差异: resolved conflict从artifact消失且无event。
- 失败原因: current实现只有open派生视图。

**Green 通过条件**

- event sequence稳定，restore可验证，current conflict只含open。

**Refactor 守卫**

- history不得反向降低已解决scope的query trust。

### UT-008 projection policy required/budget/override deterministic

**目标行为**

structural unit required，leaf按domain预算和稳定priority选中，include/exclude冲突fail closed，hints不改变eligibility。

**关联**

- Design: Page projection governance
- 系统测试用例: ST-006
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-knowledge/tests/projection_governance.rs`
- Modify: `crates/wiki-knowledge/src/projection.rs`、`wiki-model/src/domain/projection.rs`
- Reference: current `plan_pages_from_knowledge_tree`

**测试代码蓝图**

```rust
#[test]
fn projection_policy_is_bounded_and_deterministic() {
    let first = plan_projection_intents(&tree_with_many_leaf_units(), &policy(2));
    let second = plan_projection_intents(&same_tree_different_input_order(), &policy(2));
    assert_eq!(first, second);
    assert_eq!(selected_leaf_count(&first, "domain-a"), 2);
    assert_all_structural_required(&first);
}
```

**测试数据 / Fixture / Mock 边界**

- 手工KnowledgeTree，无runtime/storage依赖。

**运行命令**

`cargo test -p wiki-knowledge --test projection_governance`

**预期 Red 失败**

- 失败测试名: `projection_policy_is_bounded_and_deterministic`
- 关键错误 / 断言差异: page count等于所有unit数，decision类型不存在。
- 失败原因: 当前无eligibility policy。

**Green 通过条件**

- decisions覆盖每unit，PagePlan只来自required/selected，预算/override稳定。

**Refactor 守卫**

- planner不读取Markdown、cache或在线usage。

### UT-009 projection protection preflight 与 retiring

**目标行为**

manual content、authority block、manual inbound link阻止removal；clean page可remove，managed link必须无悬空。

**关联**

- Design: Page projection governance
- 系统测试用例: ST-007
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/declared_projection_lifecycle.rs`
- Modify: `crates/wiki-runtime/src/workflows/projection_governance.rs`、`generation/managed_sections.rs`
- Reference: `wiki-model/src/domain/projection.rs`

**测试代码蓝图**

```rust
#[test]
fn projection_removal_blocks_on_protected_sections_and_manual_links() {
    for protected in [manual_content(), authority_block(), manual_inbound_link()] {
        let plan = reconcile_retiring_page(protected);
        assert_eq!(plan.action, ProjectionDecisionAction::Block);
        assert!(plan.removals.is_empty());
    }
}
```

**测试数据 / Fixture / Mock 边界**

- 真实section parser与结构化PageLinkRef；不使用regex-only mock。

**运行命令**

`cargo test -p wiki-runtime --test runtime projection_removal_blocks_on_protected_sections`

**预期 Red 失败**

- 失败测试名: `projection_removal_blocks_on_protected_sections_and_manual_links`
- 关键错误 / 断言差异: current update直接删除page。
- 失败原因: retiring/protection preflight尚未实现。

**Green 通过条件**

- protected page完整保留并给review action，clean plan无dangling managed link。

**Refactor 守卫**

- Runtime不改写manual text，也不创建redirect兼容层。

### UT-010 RuntimeCommitPlan rollback/roll-forward 与 composite snapshot

**目标行为**

每个commit phase中断都能在pointer前rollback、pointer后roll-forward，same-facts declared/projection变化产生新snapshot identity。

**关联**

- Design: Runtime commit 与恢复
- 系统测试用例: ST-007、ST-013
- Tasks: 4.7 Red / 4.8 Green / 4.9 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/runtime_commit.rs`
- Modify: `crates/wiki-runtime/src/storage/runtime_commit.rs`、`storage/knowledge_artifacts.rs`、workflow callers
- Reference: `storage/archive_fs.rs`

**测试代码蓝图**

```rust
#[test]
fn runtime_commit_recovers_idempotently_on_both_sides_of_commit_pointer() {
    for phase in RuntimeCommitPhase::ALL {
        let repo = fail_once_at(phase);
        recover_runtime_commit(&repo).unwrap();
        assert_consistent_before_or_after_snapshot(&repo, phase);
        recover_runtime_commit(&repo).unwrap();
        assert_no_incomplete_operation(&repo);
    }
}
```

**测试数据 / Fixture / Mock 边界**

- 临时文件树和deterministic failpoint；不mock filesystem write ordering。

**运行命令**

`cargo test -p wiki-runtime --test runtime runtime_commit_recovers_idempotently`

**预期 Red 失败**

- 失败测试名: `runtime_commit_recovers_idempotently_on_both_sides_of_commit_pointer`
- 关键错误 / 断言差异: module不存在或page/metadata半提交。
- 失败原因: 当前workflows直接写文件且snapshot id只含facts。

**Green 通过条件**

- 所有phase恢复幂等，snapshot identity包含formal/page decisions。

**Refactor 守卫**

- journal仅在`.wiki/.cache`，不成为formal truth；single-writer lock生效。

### UT-011 PipelineResumeIdentity 对 action/facts/tree/contract 敏感

**目标行为**

resume key只在action、facts、stable tree和research contract全部相同时相等，任一变化拒绝旧working state。

**关联**

- Design: KnowledgeUnit compose resume
- 系统测试用例: ST-010
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `crates/wiki-runtime/src/domain/checkpoint.rs`
- Modify: `crates/wiki-runtime/src/domain/checkpoint.rs`、`workflows/page_render.rs`
- Reference: current `compute_*_input_hash`

**测试代码蓝图**

```rust
#[test]
fn resume_identity_changes_for_each_contract_dimension() {
    let base = resume_identity_fixture();
    for changed in mutations_of_action_facts_tree_and_contract(&base) {
        assert_ne!(base.resume_key, changed.resume_key);
    }
}
```

**测试数据 / Fixture / Mock 边界**

- stable tree serialization不含时间戳或输入顺序。

**运行命令**

`cargo test -p wiki-runtime domain::checkpoint::tests::resume_identity_changes`

**预期 Red 失败**

- 失败测试名: `resume_identity_changes_for_each_contract_dimension`
- 关键错误 / 断言差异: action变化key仍相同。
- 失败原因: current research input hash不含完整resume contract。

**Green 通过条件**

- repeat input key稳定，四类mutation均变化。

**Refactor 守卫**

- 旧schema直接discard，不增加兼容fallback。

### UT-012 research 与 draft/digest commit point 校验

**目标行为**

只有完整identity匹配research和合法draft+digest pair可复用，单边/错id/path/readiness必须整unit重算。

**关联**

- Design: KnowledgeUnit compose resume
- 系统测试用例: ST-010
- Tasks: 5.4 Red / 5.5 Green / 5.6 Refactor

**Files**

- Test: `crates/wiki-runtime/src/workflows/page_render.rs`
- Modify: `crates/wiki-runtime/src/workflows/page_render.rs`、`storage/sqlite/runtime_store.rs`
- Reference: current `prepare_resume_state/load_or_compute_research`

**测试代码蓝图**

```rust
#[test]
fn resume_reuses_only_complete_matching_unit_commit_points() {
    assert!(validate_cached_compose_pair(valid_pair()).is_ok());
    for invalid in [draft_only(), digest_only(), wrong_unit(), wrong_path(), stale_digest()] {
        assert!(validate_cached_compose_pair(invalid).is_err());
    }
}
```

**测试数据 / Fixture / Mock 边界**

- in-memory DTO与临时SQLite；不调用provider。

**运行命令**

`cargo test -p wiki-runtime resume_reuses_only_complete_matching_unit_commit_points`

**预期 Red 失败**

- 失败测试名: `resume_reuses_only_complete_matching_unit_commit_points`
- 关键错误 / 断言差异: current resume接受单边/旧action cache或无校验读取。
- 失败原因: unit commit-point validator缺失。

**Green 通过条件**

- valid pair复用，所有invalid reason确定性recompute。

**Refactor 守卫**

- gate/summary可从commit points重建，不成为第二truth。

### UT-013 request-local provider session 不落盘

**目标行为**

当前unit恢复重新以`session=None`请求，session summary/turn/tool refs不进入checkpoint/cache/formal artifacts。

**关联**

- Design: provider session 边界
- 系统测试用例: ST-011
- Tasks: 5.7 Red / 5.8 Green / 5.9 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/llm_runtime.rs`、`tests/runtime/compose_resume_large_fixture.rs`
- Modify: `crates/wiki-runtime/src/llm/mod.rs`、`workflows/research_provider.rs`
- Reference: `domain/context.rs`

**测试代码蓝图**

```rust
#[test]
fn interrupted_unit_starts_a_new_request_local_session() {
    let requests = run_interrupt_then_resume_same_unit();
    assert!(requests[0].session.is_none());
    assert!(requests[1].session.is_none());
    assert_no_session_fields_in_persisted_runtime();
}
```

**测试数据 / Fixture / Mock 边界**

- recording fake LlmService；只mockprovider response，不mockworkflow persistence。

**运行命令**

`cargo test -p wiki-runtime --test llm_runtime interrupted_unit_starts_a_new_request_local_session`

**预期 Red 失败**

- 失败测试名: `interrupted_unit_starts_a_new_request_local_session`
- 关键错误 / 断言差异: 缺少明确storage contract或workflow错误地复用session。
- 失败原因: request-local边界尚无自动化守卫。

**Green 通过条件**

- 每次新provider调用session=None，已完成unit无第二次调用，落盘扫描无session state。

**Refactor 守卫**

- 不删除内部session DTO，但注释不能暗示durable resume。

### UT-014 reliability lifecycle Wiki 与 scenario contract

**目标行为**

A1-A10分类、resume/session/decomposition边界和A4 query authority在当前非历史文档中唯一且机器可验证。

**关联**

- Design: 扩展场景与术语 authority
- 系统测试用例: ST-012
- Tasks: 6.1 Red / 6.2 Green / 6.3 Refactor

**Files**

- Test: `scripts/tests/reliability-lifecycle-contract.test.ts`
- Modify: `.wiki/06-设计文档/01-Runtime设计.md`、`04-扩展场景.md`及直接相关capability specs
- Reference: `.wiki/06-设计文档/06-Runtime查询合同.md`

**测试代码蓝图**

```typescript
test("reliability lifecycle authority classifies A1-A10 and states honest boundaries", () => {
  expect(classification()).toEqual({ baseline: ["A1", "A4", "A6", "A8", "A9"], next: ["A2", "A3", "A5", "A10"], nonGoal: ["A7"] });
  expect(authority).toContain("provider_session.scope = request_local");
  expect(authority).not.toMatch(/generic typed surface.*complete|durable provider session/);
});
```

**测试数据 / Fixture / Mock 边界**

- fixed-path repo files；不扫描archive，不访问网络。

**运行命令**

`pnpm exec vitest run --config vitest.config.mjs scripts/tests/reliability-lifecycle-contract.test.ts`

**预期 Red 失败**

- 失败测试名: `reliability lifecycle authority classifies A1-A10 and states honest boundaries`
- 关键错误 / 断言差异: 扩展场景仍是未分级草案，边界字段缺失。
- 失败原因: Wiki authority尚未同步本change。

**Green 通过条件**

- 分类与边界完整，A4只链接query authority，直接相关spec无冲突。

**Refactor 守卫**

- 不修改历史archive，不执行documentation-closure全库迁移。

### UT-015 32-64 unit deterministic interruption 与公开终态

**目标行为**

same identity恢复不重复已完成unit，变化identity全拒绝，init/update/rebuild成功后无checkpoint且计数准确。

**关联**

- Design: KnowledgeUnit compose resume、Runtime commit 与恢复
- 系统测试用例: ST-010、ST-013
- Tasks: 6.4 Red / 6.5 Green / 6.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/compose_resume_large_fixture.rs`
- Modify: `crates/wiki-runtime/src/workflows/page_render.rs`、`init.rs`、`update.rs`、`rebuild.rs`
- Reference: existing page_render resume tests

**测试代码蓝图**

```rust
#[test]
fn large_fixture_resumes_completed_units_once_and_clears_public_checkpoint() {
    let fixture = workspace_with_planned_units(48);
    let first = fixture.fail_provider_at_unit(17);
    assert!(first.is_err());
    fixture.resume_same_identity().unwrap();
    assert_completed_units_called_once(&fixture, 16);
    assert_current_unit_retried(&fixture, 17);
    assert_public_runtime_completed_without_checkpoint(&fixture);
}
```

**测试数据 / Fixture / Mock 边界**

- 动态48 unit workspace、计数fake provider、无sleep/网络/仓库名特判。

**运行命令**

`cargo test -p wiki-runtime --test runtime large_fixture_resumes_completed_units_once`

**预期 Red 失败**

- 失败测试名: `large_fixture_resumes_completed_units_once_and_clears_public_checkpoint`
- 关键错误 / 断言差异: scoped update重跑全部unit或action变化仍cache hit。
- 失败原因: 统一resume contract和公开workflow cleanup尚未完成。

**Green 通过条件**

- 调用计数、summary/gates、checkpoint和formal commit均满足ST-010/ST-013。

**Refactor 守卫**

- fixture按planned unit数断言，不依赖真实Storybook或timeout。

## 测试辅助边界

- 可新增 deterministic test clock、recording fake provider、runtime commit failpoint、48-unit workspace builder和fixed-path docs reader。
- failpoint与fake provider只在test/support边界可用，production默认无注入分支。
- 优先复用`tests/runtime/test_support.rs`、现有temp repo与SQLite helpers。

## 不纳入单元测试的内容

- 真实网络provider质量、外部大仓wall-clock、跨机器cache/session、hard purge、redirect和在线usage ranking；这些均为non-goal或diagnostic。
- 最终full review、归档与commit属于后续 UniSpec 阶段，不在单元测试蓝图中记录结果。
