# refactor-specwiki-around-contract-closure-truth-restore-snapshot 单元测试设计

## 测试总览

本文件服务 TDD 实现，覆盖分层 readiness、restore outcome、manifest.yaml、query degraded 四条主链。测试先写失败断言，再以最小实现满足合同。

## 单元测试用例

### UT-001 status exposes RuntimeReadiness instead of facts/query fields

**目标行为**

status JSON 公开 `readiness`，不再公开 `facts_ready` 与 `query_readiness`。

**关联**

- Design: Runtime Readiness
- 系统测试用例: ST-001
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/status_and_update.rs`
- Modify: `crates/wiki-runtime/src/domain/runtime_profile.rs`, `crates/wiki-runtime/src/workflows/status.rs`
- Reference: `design.md`

**测试代码蓝图**

```rust
#[test]
fn status_exposes_layered_readiness_contract() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    let status = run_status(repo_root).unwrap();
    let payload = serde_json::to_value(&status).unwrap();

    assert!(payload.get("readiness").is_some());
    assert!(payload.get("facts_ready").is_none());
    assert!(payload.get("query_readiness").is_none());
    assert_eq!(payload["readiness"]["index"], "ready");
    assert_eq!(payload["readiness"]["knowledge"], "ready");
    assert_eq!(payload["readiness"]["projection"], "ready");
}
```

**测试数据 / Fixture / Mock 边界**

- 使用真实 temp repo 和 `run_init`。

**运行命令**

`cargo test -p wiki-runtime status_exposes_layered_readiness_contract --test runtime`

**预期 Red 失败**

- 失败测试名: `status_exposes_layered_readiness_contract`
- 关键错误 / 断言差异: `payload.get("readiness").is_some()` 失败，或旧字段仍存在。
- 失败原因: status 尚未公开 RuntimeReadiness。

**Green 通过条件**

- status 序列化包含 `readiness`，且不包含旧公开字段。

**Refactor 守卫**

- 不新增旧字段兼容输出；现有 health/recommended action 行为保持。

### UT-002 Level 1 restore outcome keeps index not ready

**目标行为**

删除 `.cache` 后，restore outcome 为 level1，knowledge/projection ready，index 不 ready。

**关联**

- Design: Restore Outcome
- 系统测试用例: ST-001
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`
- Modify: `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`, `crates/wiki-runtime/src/storage/state_store.rs`
- Reference: `design.md`

**测试代码蓝图**

```rust
#[test]
fn restore_runtime_cache_returns_level1_outcome_without_index_ready() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let outcome = restore_runtime_cache_from_artifacts(repo_root).unwrap();

    assert!(outcome.restored_cache);
    assert_eq!(outcome.restored_level, RestoredLevel::Level1);
    assert_ne!(outcome.readiness.index, LayerReadiness::Ready);
    assert_eq!(outcome.readiness.knowledge, LayerReadiness::Ready);
    assert_eq!(outcome.readiness.projection, LayerReadiness::Ready);
    assert!(!index_graph_ready(repo_root).unwrap());
}
```

**测试数据 / Fixture / Mock 边界**

- 使用真实 `run_init` artifact 和删除 `.wiki/.cache`。

**运行命令**

`cargo test -p wiki-runtime restore_runtime_cache_returns_level1_outcome_without_index_ready --test runtime`

**预期 Red 失败**

- 失败测试名: `restore_runtime_cache_returns_level1_outcome_without_index_ready`
- 关键错误 / 断言差异: restore 返回 bool 或 index 仍 ready。
- 失败原因: restore outcome 与 graph origin marker 尚未实现。

**Green 通过条件**

- restore 返回结构化 outcome，Level 1 不满足 index ready。

**Refactor 守卫**

- 不通过删除 scan/module mirror 让测试通过；Level 1 mirror 仍可诊断。

### UT-003 committed snapshot manifest writes manifest.yaml

**目标行为**

persist artifacts 写入 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml`，不写旧 JSON 主记录。

**关联**

- Design: Snapshot Manifest
- 系统测试用例: ST-003
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`
- Modify: `crates/wiki-model/src/domain/knowledge_artifact.rs`, `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- Reference: `design.md`

**测试代码蓝图**

```rust
#[test]
fn committed_snapshot_manifest_is_yaml_restore_truth() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();

    let artifacts = load_knowledge_artifacts(repo_root).unwrap();
    let manifest_path = repo_root
        .join(".wiki/.knowledge/runtime/snapshots")
        .join(&artifacts.snapshot_manifest.snapshot_id)
        .join("manifest.yaml");

    assert!(manifest_path.exists());
    assert!(!repo_root.join(".wiki/.knowledge/runtime/recovery-manifest.json").exists());
    assert_eq!(artifacts.snapshot_manifest.schema_version, "1");
}
```

**测试数据 / Fixture / Mock 边界**

- 使用真实 init 输出。

**运行命令**

`cargo test -p wiki-runtime committed_snapshot_manifest_is_yaml_restore_truth --test runtime`

**预期 Red 失败**

- 失败测试名: `committed_snapshot_manifest_is_yaml_restore_truth`
- 关键错误 / 断言差异: `manifest.yaml` 不存在或旧 JSON 仍存在。
- 失败原因: committed snapshot manifest 尚未迁移到 YAML 主记录。

**Green 通过条件**

- YAML manifest 存在并可被 loader 读取。

**Refactor 守卫**

- 不保留旧 JSON 双轨。

### UT-004 restore guard exposes blocked reason on anchor mismatch

**目标行为**

metadata/page/formal artifacts 锚点不一致时 restore outcome 带 reason 且不恢复 cache。

**关联**

- Design: Restore Guards
- 系统测试用例: ST-004
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`
- Modify: `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`, `crates/wiki-runtime/src/workflows/status.rs`
- Reference: `design.md`

**测试代码蓝图**

```rust
#[test]
fn restore_blocks_on_snapshot_anchor_mismatch() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    fs::write(repo_root.join("main.rs"), "fn main() {}").unwrap();
    run_init(repo_root).unwrap();
    fs::write(repo_root.join(".wiki/INDEX.md"), "# drift\n").unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let outcome = restore_runtime_cache_from_artifacts(repo_root).unwrap();

    assert!(!outcome.restored_cache);
    assert!(outcome.readiness.reasons.iter().any(|reason| reason.contains("page_hash_mismatch")));
    assert_eq!(outcome.readiness.projection, LayerReadiness::Conflict);
}
```

**测试数据 / Fixture / Mock 边界**

- 用真实 page drift 触发 guard。

**运行命令**

`cargo test -p wiki-runtime restore_blocks_on_snapshot_anchor_mismatch --test runtime`

**预期 Red 失败**

- 失败测试名: `restore_blocks_on_snapshot_anchor_mismatch`
- 关键错误 / 断言差异: outcome 无 reason 或仍返回 true。
- 失败原因: restore guard 仍用 bool 且丢失机器原因。

**Green 通过条件**

- restore 不写 ready mirror，并输出具体 reason。

**Refactor 守卫**

- 不用 panic/IO error 表示业务 guard。

### UT-005 query degrades without graph hits when index missing

**目标行为**

index 不 ready 时 query 不调用 graph/index 命中，但可以返回 degraded page/knowledge fallback。

**关联**

- Design: Query Degradation
- 系统测试用例: ST-005
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`, `crates/wiki-runtime/src/transport/query_payload.rs`
- Reference: `design.md`

**测试代码蓝图**

```rust
#[test]
fn query_degrades_without_graph_hits_when_index_missing() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_graph_query_repo(repo_root);
    run_init(repo_root).unwrap();
    fs::remove_dir_all(repo_root.join(".wiki/.cache")).unwrap();

    let query = run_query(repo_root, "UserService").unwrap();
    let payload = serde_json::to_value(&query).unwrap();

    assert_ne!(payload["readiness"]["index"], "ready");
    assert_eq!(payload["answer"]["answer_mode"], "degraded");
    assert!(query.matched_symbols.is_empty());
    assert!(query.matched_symbol_edges.is_empty());
    assert!(!query.provenance_summary.contains("index_hit"));
    assert!(!query.provenance_summary.contains("graph_hit"));
}
```

**测试数据 / Fixture / Mock 边界**

- 复用 query graph fixture；删除 cache 触发 Level 1。

**运行命令**

`cargo test -p wiki-runtime query_degrades_without_graph_hits_when_index_missing --test runtime`

**预期 Red 失败**

- 失败测试名: `query_degrades_without_graph_hits_when_index_missing`
- 关键错误 / 断言差异: query 返回 index/graph hit 或因 index missing 直接错误。
- 失败原因: query 尚未支持 Level 1 degraded 消费。

**Green 通过条件**

- index missing 时无 graph 命中，query 返回 degraded fallback 或明确 blocked 结果。

**Refactor 守卫**

- 不扩展 query route DTO 全量重构；只保证 readiness 与 graph 禁用合同。

## 测试辅助边界

- 可新增小型 helper 读取 readiness JSON。
- 不新增外部服务或网络依赖。

## 不纳入单元测试的内容

- CLI help / 产品文案不属于本 change。
- 完整 query route tags 全量重构留给后续 child change。
