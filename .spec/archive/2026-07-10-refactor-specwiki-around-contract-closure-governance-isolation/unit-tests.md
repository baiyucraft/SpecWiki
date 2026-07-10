# refactor-specwiki-around-contract-closure-governance-isolation 单元测试设计

## 测试总览

本文件定义 governance-isolation 的 TDD 蓝图。八个 `UT-*` 覆盖公开 DTO、evidence discovery、policy parity、fingerprint、SQLite cache、workflow composition、query refs 和 TS parser。实际测试文件只在 `unispec-apply` 的 Red task 中创建；本阶段不写生产代码或真实测试代码。

## 单元测试用例

### UT-001 Governance DTO 闭集与序列化合同

**目标行为**

`wiki-model` 能稳定序列化五态 readiness、change/artifact refs、issue、gate 和 validation result，并拒绝未知闭集值。

**关联**

- Design: 数据设计 / 公开 DTO；Transport 合同
- 系统测试用例: ST-001、ST-002、ST-010
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-model/tests/governance_contract.rs`
- Modify: `crates/wiki-model/src/domain/governance.rs`、`crates/wiki-model/src/domain/mod.rs`
- Reference: `crates/wiki-model/tests/query_contract.rs`、`crates/wiki-model/src/domain/query.rs`

**测试代码蓝图**

~~~rust
#[test]
fn governance_contract_serializes_closed_product_dto() {
    let summary = ready_governance_summary_with_change_and_artifact();
    let value = serde_json::to_value(summary).unwrap();
    assert_eq!(value["readiness"], "ready");
    assert_eq!(value["issues"][0]["severity"], "warning");
    assert_eq!(value["recommended_action"], "review_governance");
    assert!(value.get("governance_readiness").is_none());
}

#[test]
fn governance_contract_rejects_unknown_readiness_and_issue_severity() {
    assert!(serde_json::from_str::<GovernanceReadiness>(r#""unknown""#).is_err());
    assert!(serde_json::from_str::<GovernanceIssueSeverity>(r#""fatal""#).is_err());
}
~~~

**测试数据 / Fixture / Mock 边界**

- 只构造纯 DTO，不访问文件系统或 SQLite。
- fixture helper 必须显式填写 fingerprint、rule id、relative path 和 gate 状态。

**运行命令**

`cargo test -p wiki-model --test governance_contract`

**预期 Red 失败**

- 失败测试名: `governance_contract_serializes_closed_product_dto`
- 关键错误 / 断言差异: `unresolved import wiki_model::domain::governance` 或 DTO 字段不存在
- 失败原因: governance 公开对象语言尚未定义；不是 serde 环境或测试导入错误。

**Green 通过条件**

- 所有公开枚举使用 snake_case，合法 DTO 完整 roundtrip，未知值被 serde 拒绝。

**Refactor 守卫**

- 不把 governance readiness 并入 `LayerReadiness`；不恢复旧扁平 `governance_readiness` 字段。

### UT-002 EvidenceStore discovery、故障隔离与路径安全

**目标行为**

文件系统 store 能区分 not-enabled、enabled-empty、active/archive evidence，并隔离损坏 change、拒绝 repo root 外路径。

**关联**

- Design: Governance evidence discovery；错误分类
- 系统测试用例: ST-001、ST-002、ST-004、ST-009
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_evidence.rs`
- Modify: `crates/wiki-runtime/src/storage/governance_fs.rs`、`crates/wiki-runtime/src/storage/mod.rs`、`crates/wiki-runtime/tests/runtime.rs`
- Reference: `crates/wiki-runtime/tests/runtime/test_support.rs`、`C:/Program Files/nodejs/node_modules/unispec/src/core/change/paths.ts`

**测试代码蓝图**

~~~rust
#[test]
fn governance_evidence_store_discovers_valid_changes_and_isolates_failures() {
    let repo = governance_fixture_repo();
    write_active_child(&repo, "parent-child");
    write_archived_change(&repo, "done");
    write_malformed_metadata(&repo, "broken");

    let snapshot = FsGovernanceEvidenceStore::new(repo.path()).discover_repo().unwrap();
    assert_eq!(snapshot.active_changes.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), ["parent-child"]);
    assert_eq!(snapshot.archived_changes[0].id, "done");
    assert_eq!(snapshot.failures[0].change_id.as_deref(), Some("broken"));
    assert!(snapshot.artifacts.iter().all(|artifact| !artifact.relative_path.contains("..")));
}
~~~

**测试数据 / Fixture / Mock 边界**

- 使用 `tempfile` 和真实 YAML/frontmatter 文件，不 mock YAML parser。
- Windows 权限失败使用实现 `GovernanceEvidenceStore` 的 error fixture；路径逃逸使用 symlink 或规范化 path fixture。
- 单文件大小限制使用稀疏或短阈值测试配置，不创建大文件。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_evidence`

**预期 Red 失败**

- 失败测试名: `governance_evidence_store_discovers_valid_changes_and_isolates_failures`
- 关键错误 / 断言差异: `could not find governance_fs` 或 store 无 discovery API
- 失败原因: evidence port 与 filesystem adapter 尚未实现。

**Green 通过条件**

- store 返回稳定排序的 active/archive evidence、relative refs 和独立 failures；不读取或返回 artifact 正文。

**Refactor 守卫**

- store 只负责读取与规范化，不计算 ready/blocked/conflict，不写 `.spec` 或 cache。

### UT-003 GovernancePolicy stage/artifact/metadata parity

**目标行为**

Rust policy 对 stage required artifacts、delivery shape、parent/child metadata 的判断与版本化 UniSpec 0.1.0 fixtures 一致。

**关联**

- Design: Governance policy 与 parity；唯一规则所有权
- 系统测试用例: ST-003、ST-004、ST-005
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_policy.rs`
- Modify: `crates/wiki-runtime/src/domain/governance.rs`、`crates/wiki-runtime/src/domain/mod.rs`
- Reference: `C:/Program Files/nodejs/node_modules/unispec/src/core/change/artifacts.ts`、`metadata.ts`、`state.ts`

**测试代码蓝图**

~~~rust
#[test]
fn governance_policy_matches_versioned_stage_and_metadata_fixtures() {
    for fixture in load_governance_parity_fixtures("stage-artifact-metadata.json") {
        let result = GovernancePolicy::v1().evaluate(&fixture.snapshot);
        assert_eq!(normalize(result), fixture.expected, "fixture={}", fixture.id);
    }
}

#[test]
fn normal_in_progress_stage_is_not_blocked_by_archive_readiness() {
    let result = GovernancePolicy::v1().evaluate(&design_stage_snapshot());
    assert_eq!(result.readiness, GovernanceReadiness::Ready);
    assert!(!result.gates.archive_ready);
    assert!(result.blocking_issues.is_empty());
}
~~~

**测试数据 / Fixture / Mock 边界**

- expected JSON 固定来源版本 `@uni-sw/unispec` 0.1.0，并记录 oracle version。
- 常规测试禁止执行全局 `unispec`；fixture 只包含规范化机器字段，不断言英文文案。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_policy`

**预期 Red 失败**

- 失败测试名: `governance_policy_matches_versioned_stage_and_metadata_fixtures`
- 关键错误 / 断言差异: `GovernancePolicy` 未定义或 normalized issues 与 expected 不同
- 失败原因: required artifact matrix 和 metadata consistency rule 尚未迁移。

**Green 通过条件**

- 所有 fixture 的 rule id、required status、role/order/dependency 和 blocking 分类一致。

**Refactor 守卫**

- required matrix 只能存在于 Rust policy；TS、Skill 和 storage adapter 不复制规则。

### UT-004 Review、verification 与 archive marker gate

**目标行为**

policy 正确解析 report frontmatter 和 parent archive markers，并区分 blocked 与 conflict。

**关联**

- Design: Governance policy 与 parity；blocked/conflict 分类
- 系统测试用例: ST-003、ST-004、ST-005、ST-010
- Tasks: 1.7 Red / 1.8 Green / 1.9 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_policy.rs`
- Modify: `crates/wiki-runtime/src/domain/governance.rs`、`crates/wiki-runtime/src/storage/governance_fs.rs`
- Reference: `C:/Program Files/nodejs/node_modules/unispec/src/core/review/review.ts`、`readiness.ts`

**测试代码蓝图**

~~~rust
#[test]
fn governance_policy_classifies_report_and_archive_evidence() {
    let blocked = evaluate_fixture("verification-report-not-pass");
    assert_eq!(blocked.readiness, GovernanceReadiness::Blocked);
    assert!(blocked.issues.iter().any(|issue| issue.rule_id == "report.review.pass"));

    let conflict = evaluate_fixture("parent-marker-target-missing");
    assert_eq!(conflict.readiness, GovernanceReadiness::Conflict);
    assert!(conflict.issues.iter().any(|issue| issue.rule_id == "parent.archive_marker.consistent"));
}
~~~

**测试数据 / Fixture / Mock 边界**

- 使用真实 YAML frontmatter；report body 可最小化。
- archive target 只在 temp fixture 中创建或删除，不调用 archive command。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_policy governance_policy_classifies_report_and_archive_evidence`

**预期 Red 失败**

- 失败测试名: `governance_policy_classifies_report_and_archive_evidence`
- 关键错误 / 断言差异: blocked/conflict 相同或缺少稳定 rule id
- 失败原因: report evidence 与 archive marker policy 尚未实现。

**Green 通过条件**

- 非 full/pass 在 verification 阶段 blocked；不可解析或互相矛盾 evidence conflict；中间 stage 不被 archive gate 阻断。

**Refactor 守卫**

- 只读 policy 不移动目录、不写 parent metadata，不调用 UniSpec archive。

### UT-005 Fingerprint 与 SQLite governance cache 原子刷新

**目标行为**

content-based fingerprint 稳定识别 `.spec` 变化，SQLite cache 以单事务替换 snapshot/changes/refs/issues，并拒绝消费不匹配 fingerprint。

**关联**

- Design: 独立 fingerprint 与 update；Evidence、derived cache 与 truth
- 系统测试用例: ST-006、ST-008、ST-009
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_cache.rs`
- Modify: `crates/wiki-runtime/src/storage/sqlite/governance_store.rs`、`crates/wiki-runtime/src/storage/sqlite/mod.rs`、`crates/wiki-runtime/src/storage/sqlite_store.rs`
- Reference: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`、`crates/wiki-index/src/fingerprint.rs`

**测试代码蓝图**

~~~rust
#[test]
fn governance_cache_refresh_is_atomic_and_fingerprint_bound() {
    let repo = initialized_repo();
    let first = evidence_snapshot("proposal-v1");
    let second = evidence_snapshot("proposal-v2");
    assert_ne!(first.fingerprint, second.fingerprint);
    assert_eq!(first.fingerprint, evidence_snapshot_with_changed_mtime("proposal-v1").fingerprint);

    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    SqliteGovernanceCache::new(&mut conn).replace_snapshot(&first.derived()).unwrap();
    assert!(load_refs(&conn, &first.fingerprint).is_some());
    assert!(load_refs(&conn, &second.fingerprint).is_none());

    inject_governance_cache_write_failure(&conn);
    assert!(SqliteGovernanceCache::new(&mut conn).replace_snapshot(&second.derived()).is_err());
    assert!(load_refs(&conn, &first.fingerprint).is_some());
}
~~~

**测试数据 / Fixture / Mock 边界**

- 使用真实 bundled SQLite 和 transaction；故障注入通过 test-only trigger/constraint，不 mock rusqlite commit。
- fingerprint 输入固定排序，mtime 变化但 bytes 不变时 fingerprint 保持相同。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_cache`

**预期 Red 失败**

- 失败测试名: `governance_cache_refresh_is_atomic_and_fingerprint_bound`
- 关键错误 / 断言差异: governance tables/store 不存在或旧 snapshot 在失败后丢失
- 失败原因: governance cache schema 与 transaction wrapper 尚未实现。

**Green 通过条件**

- cache 只返回 fingerprint 一致的 refs；失败 transaction 保留前一 snapshot；其它 SQLite 表不受影响。

**Refactor 守卫**

- 不新增 JSON 双写；cache 不保存 artifact 正文，不成为 evidence truth。

### UT-006 Status、validate 与 update 的独立治理组合

**目标行为**

status live-read 不写 cache，validate 绕过 cache，update 在 source no-op 时刷新 governance cache，且治理问题不改变 core fusion。

**关联**

- Design: Governance readiness；独立 fingerprint；产品级组合
- 系统测试用例: ST-001、ST-006、ST-007、ST-010
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_workflows.rs`
- Modify: `crates/wiki-runtime/src/workflows/governance.rs`、`status.rs`、`update.rs`、`domain/runtime_profile.rs`、`workflows/mod.rs`
- Reference: `crates/wiki-runtime/tests/runtime/status_and_update.rs`、`crates/wiki-runtime/src/domain/change_set.rs`

**测试代码蓝图**

~~~rust
#[test]
fn spec_only_delta_refreshes_governance_without_dirtying_core_runtime() {
    let repo = ready_repo_with_governance_cache();
    let core_before = core_snapshot_ids(repo.path());
    write_changed_proposal(repo.path());

    let status = run_status(repo.path()).unwrap();
    assert_eq!(status.governance.readiness, GovernanceReadiness::Stale);
    assert_eq!(status.readiness.fusion, FusionReadiness::Ready);
    assert_eq!(governance_cache_fingerprint(repo.path()), old_fingerprint());

    let update = run_update(repo.path()).unwrap();
    assert_eq!(update.governance.readiness, GovernanceReadiness::Ready);
    assert_eq!(core_snapshot_ids(repo.path()), core_before);
    assert!(update.updated_pages.is_empty());
}

#[test]
fn core_blocker_action_takes_priority_over_governance_review() {
    let status = composed_status(core_blocked(), governance_blocked());
    assert_eq!(status.recommended_action, RecommendedAction::Rebuild);
    assert_eq!(status.governance.readiness, GovernanceReadiness::Blocked);
}
~~~

**测试数据 / Fixture / Mock 边界**

- 使用真实 init/status/update workflow 和 temp repo。
- research provider 固定 deterministic/index-only，避免 LLM 与治理行为耦合。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_workflows spec_only_delta_refreshes_governance_without_dirtying_core_runtime`

**预期 Red 失败**

- 失败测试名: `spec_only_delta_refreshes_governance_without_dirtying_core_runtime`
- 关键错误 / 断言差异: status 无 governance 字段，或 update 在 source fresh 时提前返回且 cache 未刷新
- 失败原因: governance preflight/composition 尚未接入 workflow。

**Green 通过条件**

- status/query/update 都返回单一 governance summary；source/graph/knowledge snapshot 不变；workflow action 支持 `review_governance`。

**Refactor 守卫**

- 不把 governance 放入 source `ChangeSet`，不修改 `RuntimeReadiness.fusion` 的既有语义。

### UT-007 Governance query refs 与 freshness/trust

**目标行为**

query 仅从 fingerprint 一致的结构化 cache 返回治理 refs，并按 ready/blocked/stale/conflict 设置 route、provenance、confidence 和 action。

**关联**

- Design: Governance query refs；产品级组合
- 系统测试用例: ST-007、ST-008、ST-009
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/governance_workflows.rs`
- Modify: `crates/wiki-runtime/src/workflows/governance.rs`、`crates/wiki-runtime/src/workflows/query.rs`
- Reference: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`、`crates/wiki-model/src/domain/query.rs`

**测试代码蓝图**

~~~rust
#[test]
fn governance_query_uses_only_fresh_structured_refs() {
    let ready = query_fixture(GovernanceReadiness::Ready, "governance-isolation");
    assert!(ready.results.iter().any(|r| r.route_tag == QueryRouteTag::GovernanceSummaryHit));
    assert!(ready.results.iter().all(|r| !r.label.contains("proposal body token")));

    let stale = query_fixture(GovernanceReadiness::Stale, "governance-isolation");
    assert!(stale.results.iter().all(|r| !r.route_tag.is_governance_route()));
    assert_eq!(stale.governance.recommended_action, RecommendedAction::Update);
}
~~~

**测试数据 / Fixture / Mock 边界**

- 用真实 SQLite governance refs 和现有 query result projection，不 mock ranking。
- term 仅匹配 change id、stage、artifact kind/path 或 rule id。

**运行命令**

`cargo test -p wiki-runtime --test runtime governance_workflows governance_query_uses_only_fresh_structured_refs`

**预期 Red 失败**

- 失败测试名: `governance_query_uses_only_fresh_structured_refs`
- 关键错误 / 断言差异: query 只有固定 `not_enabled` 或 stale 仍返回旧 refs
- 失败原因: governance query adapter 与 freshness gate 尚未实现。

**Green 通过条件**

- ready/blocked/conflict 路由符合设计，stale 不消费旧 refs，非治理 route 的 trust 不被连带修改。

**Refactor 守卫**

- 不把 `.spec` 加入 code FTS、knowledge 或 Markdown fallback，不增加治理全文索引。

### UT-008 TS parser 消费唯一 governance summary

**目标行为**

TS parser 接受 Rust governance DTO 全闭集、保留允许扩展字段，并拒绝未知 readiness、severity、action 或残缺 summary。

**关联**

- Design: Transport 合同；可维护性
- 系统测试用例: ST-001、ST-010
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `packages/spec-wiki/src/index.test.ts`
- Modify: `packages/spec-wiki/src/runtime/parseResult.ts`
- Reference: `crates/wiki-model/tests/governance_contract.rs`

**测试代码蓝图**

~~~typescript
test("parseResult parses the governance product summary and removes the legacy field", () => {
  const parsed = parseResult(JSON.stringify({
    ok: true,
    data: statusPayload({
      governance: governanceSummary({ readiness: "blocked", recommended_action: "review_governance" }),
    }),
  }));
  expect(parsed.data?.governance).toMatchObject({ readiness: "blocked" });
  expect(parsed.data).not.toHaveProperty("governance_readiness");
});

test("parseResult rejects unknown governance readiness", () => {
  expect(() => parseResult(payloadWithGovernanceReadiness("unknown"))).toThrow(
    "invalid wiki-runtime governance.readiness",
  );
});
~~~

**测试数据 / Fixture / Mock 边界**

- 直接构造 JSON payload，不启动 Rust 子进程。
- status/query/update 三类 payload 共用测试 helper，helper 不实现业务默认值。

**运行命令**

`pnpm --filter spec-wiki exec vitest run src/index.test.ts -t "governance"`

**预期 Red 失败**

- 失败测试名: `parseResult parses the governance product summary and removes the legacy field`
- 关键错误 / 断言差异: `governance` 未解析或类型仍要求 `governance_readiness: not_enabled`
- 失败原因: TS transport 仍使用旧占位合同。

**Green 通过条件**

- 五态 readiness、blocking/warning、change/artifact refs 和 `review_governance` 全部可解析；未知值与残缺对象被拒绝。

**Refactor 守卫**

- TS 只做闭集校验与透传，不复制 required artifact matrix 或 blocked/conflict 规则。

## 测试辅助边界

- 允许新增 `crates/wiki-runtime/tests/fixtures/governance/**` 保存最小 `.spec` tree 和版本化 parity expected JSON。
- 允许新增 test-only fixture builder，统一生成 metadata、artifacts、reports 和 archive markers；builder 不参与生产代码。
- 允许为 SQLite transaction 增加 test-only 故障注入入口，但不能改变生产 transaction 路径。
- 不 mock serde、serde_yaml、rusqlite transaction 或 query route projection；只在跨平台权限错误处 mock store error。

## 不纳入单元测试的内容

- public CLI command router、help 和参数布局：由后续 CLI child 验证。
- archive move、operation manifest 和恢复：由后续 archive child 验证。
- 真实全局 `unispec` executable parity 执行：只由显式 oracle 刷新脚本执行，常规测试使用固定 expected fixtures。
- 全仓性能、真实大型 `.spec` 和 soak：在系统验证或后续性能基线中覆盖，不作为 Red/Green 单元门禁。
