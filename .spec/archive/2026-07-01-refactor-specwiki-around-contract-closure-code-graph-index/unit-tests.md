# refactor-specwiki-around-contract-closure-code-graph-index 单元测试设计

## 测试总览

本文件服务 TDD 实现阶段，只定义计划中的单元测试蓝图，不把测试代码写入真实测试文件。测试覆盖 `wiki-index` 合同、SQLite adapter、readiness、runtime query projection 和 workflow diagnostics，并与 `system-tests.md` 的 ST-001 至 ST-007 互证。

## 单元测试用例

### UT-001 SymbolNode 合同保留 identity、range 和 provenance

**目标行为**

扩展后的 `SymbolNode` 能表达 file identity、symbol kind、qualified name、signature、visibility、owner symbol、`SourceRange` 和 `SymbolProvenance`，并保持同一源码 snapshot 内稳定 symbol id。

**关联**

- Design: `### 1. wiki-index 合同层`、`SourceRange`、`SymbolNode 最小稳定合同`、`SymbolProvenance`
- 系统测试用例: ST-002
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-index/src/symbols/models.rs`
- Modify: `crates/wiki-index/src/symbols/models.rs`
- Reference: `crates/wiki-index/src/query.rs`、`crates/wiki-runtime/tests/runtime/sqlite_storage.rs`、`design.md`

**测试代码蓝图**

```rust
#[test]
fn symbol_node_contract_preserves_identity_range_and_provenance() {
    let range = SourceRange {
        file_id: "file:src/service.ts".to_string(),
        path: "src/service.ts".to_string(),
        start_line: 3,
        end_line: 8,
        start_column: 1,
        end_column: 2,
    };
    let provenance = SymbolProvenance {
        parser_id: "tree-sitter-typescript".to_string(),
        parser_version: "test-version".to_string(),
        source_kind: SymbolSourceKind::Parser,
        confidence: 1.0,
        diagnostics: Vec::new(),
    };

    let symbol = SymbolNode {
        symbol_id: "symbol:src/service.ts:function:PaymentService.run".to_string(),
        file_id: "file:src/service.ts".to_string(),
        language: "typescript".to_string(),
        symbol_kind: "method".to_string(),
        name: "run".to_string(),
        qualified_name: Some("PaymentService.run".to_string()),
        signature: Some("run(input: PaymentInput): PaymentResult".to_string()),
        docstring: Some("Execute payment.".to_string()),
        visibility: Some("public".to_string()),
        owner_symbol_id: Some("symbol:src/service.ts:class:PaymentService".to_string()),
        range: range.clone(),
        is_exported: true,
        provenance: provenance.clone(),
    };

    assert_eq!(symbol.file_id, "file:src/service.ts");
    assert_eq!(symbol.symbol_kind, "method");
    assert_eq!(symbol.range, range);
    assert_eq!(symbol.provenance, provenance);
    assert!(symbol.search_text().contains("PaymentService.run"));

    let rebuilt = SymbolNode::from_definition(
        "file:src/service.ts",
        "typescript",
        "method",
        "run",
        Some("PaymentService.run"),
        range,
        provenance,
    );
    assert_eq!(rebuilt.symbol_id, symbol.symbol_id);
}
```

**测试数据 / Fixture / Mock 边界**

- 只构造内存 DTO，不 mock store。
- 可在同文件追加稳定 ID helper 测试，但不得依赖当前薄 `label/file_path/start_line` 兼容字段。

**运行命令**

`cargo test -p wiki-index symbol_node_contract_preserves_identity_range_and_provenance`

**预期 Red 失败**

- 失败测试名: `symbol_node_contract_preserves_identity_range_and_provenance`
- 关键错误 / 断言差异: `cannot find type SourceRange`、`struct SymbolNode has no field named file_id` 或 `struct SymbolNode has no field named provenance`
- 失败原因: 当前 `SymbolNode` 合同偏薄，缺少设计要求的字段和 DTO。

**Green 通过条件**

- `wiki-index` 新 DTO 和扩展 `SymbolNode` 编译通过。
- `search_text` 或等价查询文本包含 symbol kind、name/qualified name、path/language 等检索必要信息。
- 同一源码 snapshot 的同一 symbol 定义重复构造得到相同 `symbol_id`。

**Refactor 守卫**

- 不把 `label` 继续作为唯一公开语义；如保留过渡字段，必须能派生或映射到 `symbol_kind`。
- 不让 runtime SQLite 私表类型泄漏进 `wiki-index` DTO。

### UT-002 Raw capture 和 unresolved ref DTO 保留结构化来源

**目标行为**

raw import/call/heritage capture 与 unresolved ref 使用结构化字段表达 capture、range、parser、resolver、reason 和 diagnostics，不能退化为纯文本或 JSON blob。

**关联**

- Design: `Raw capture 采用统一 envelope + 类型特有字段`、`UnresolvedRef 不是普通 diagnostics 垃圾桶`
- 系统测试用例: ST-001, ST-007
- Tasks: 1.4 Red / 1.5 Green / 1.6 Refactor

**Files**

- Test: `crates/wiki-index/src/symbols/models.rs`
- Modify: `crates/wiki-index/src/symbols/models.rs`、可能新增 `crates/wiki-index/src/diagnostics.rs`
- Reference: `crates/wiki-index/src/symbol_graph/models.rs`、`design.md`

**测试代码蓝图**

```rust
#[test]
fn raw_capture_dtos_preserve_source_identity() {
    let base = RawCaptureBase {
        capture_id: "capture:import:src/service.ts:1".to_string(),
        file_id: "file:src/service.ts".to_string(),
        language: "typescript".to_string(),
        capture_kind: RawCaptureKind::Import,
        source_symbol_id: None,
        raw_text: "import { run } from './runner'".to_string(),
        target_hint: Some("./runner".to_string()),
        range: SourceRange::new("file:src/service.ts", "src/service.ts", 1, 1, 0, 31),
        parser_id: "tree-sitter-typescript".to_string(),
        parser_version: "test-version".to_string(),
        diagnostics: Vec::new(),
    };
    let import = RawImportCapture {
        base: base.clone(),
        raw_path: "./runner".to_string(),
        imported_name: Some("run".to_string()),
        alias: None,
    };
    let unresolved = UnresolvedRef {
        unresolved_ref_id: "unresolved:capture:import:src/service.ts:1".to_string(),
        capture_id: base.capture_id.clone(),
        file_id: base.file_id.clone(),
        resolver_phase: GraphPhase::ResolveImports,
        reference_kind: ReferenceKind::Import,
        reference_name: "run".to_string(),
        target_hint: Some("./runner".to_string()),
        range: base.range.clone(),
        candidates: Vec::new(),
        reason: "target_not_found".to_string(),
        diagnostics: Vec::new(),
    };

    assert_eq!(import.base.capture_id, unresolved.capture_id);
    assert_eq!(unresolved.resolver_phase, GraphPhase::ResolveImports);
    assert_eq!(unresolved.reference_kind, ReferenceKind::Import);
}
```

**测试数据 / Fixture / Mock 边界**

- 只构造 DTO；不用 parser fixture。
- 允许新增枚举或字符串新类型，但测试必须确保 capture/ref 能通过 Rust 类型访问关键字段。

**运行命令**

`cargo test -p wiki-index raw_capture_dtos_preserve_source_identity`

**预期 Red 失败**

- 失败测试名: `raw_capture_dtos_preserve_source_identity`
- 关键错误 / 断言差异: `cannot find type RawCaptureBase`、`cannot find type UnresolvedRef` 或 raw capture 仍只有 `file_path/line/source_text`
- 失败原因: 当前 raw capture 只在解析结果中保留薄字段，缺少持久化和诊断所需结构化合同。

**Green 通过条件**

- import/call/heritage capture 与 unresolved ref 均有结构化 DTO。
- DTO 字段能追踪到 file、range、parser/capture/resolver。

**Refactor 守卫**

- 不把 capture 类型特有字段压成不可查询 JSON blob。
- diagnostics 可以复用通用类型，但不得丢失 capture/ref 绑定。

### UT-003 SQLite graph snapshot 原子写入 symbols/raw/unresolved/FTS/phase

**目标行为**

SQLite adapter 能在一个 graph snapshot 事务中写入 files、symbols、raw captures、unresolved refs、edges、phase runs 和 FTS，并只在事务成功后切换 current snapshot。

**关联**

- Design: `### 2. SQLite adapter 与 schema`、`事务边界`
- 系统测试用例: ST-001, ST-002, ST-005, ST-007
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`
- Modify: `crates/wiki-runtime/src/storage/sqlite_store.rs`、`crates/wiki-runtime/src/storage/sqlite/index_store.rs`、`crates/wiki-index/src/store.rs`
- Reference: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs` existing graph tests、`design.md`

**测试代码蓝图**

```rust
#[test]
fn graph_snapshot_roundtrip_persists_raw_unresolved_phase_and_fts_atomically() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_raw_and_unresolved();

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();

    assert_eq!(sqlite_store::list_symbols(repo.path()).unwrap().len(), 2);
    assert_eq!(sqlite_store::list_raw_imports(repo.path()).unwrap().len(), 1);
    assert_eq!(sqlite_store::list_raw_calls(repo.path()).unwrap().len(), 1);
    assert_eq!(sqlite_store::list_raw_heritage(repo.path()).unwrap().len(), 1);
    assert_eq!(sqlite_store::list_unresolved_refs(repo.path()).unwrap().len(), 1);
    assert!(sqlite_store::search_files_fts(repo.path(), "service", 8).unwrap().iter().any(|hit| hit.path == "src/service.ts"));
    assert!(sqlite_store::list_graph_phase_runs(repo.path()).unwrap().iter().any(|phase| phase.phase == GraphPhase::BuildFts));
    assert_eq!(sqlite_store::read_current_graph_snapshot(repo.path()).unwrap().unwrap().snapshot_id, snapshot.snapshot_id);
}
```

**测试数据 / Fixture / Mock 边界**

- 使用临时 SQLite DB 和 helper fixture。
- 可新增 `sample_graph_snapshot_with_raw_and_unresolved()` 测试 helper。
- 不通过 CLI 命令模拟；直接测 adapter 原子 roundtrip。

**运行命令**

`cargo test -p wiki-runtime graph_snapshot_roundtrip_persists_raw_unresolved_phase_and_fts_atomically --test sqlite_storage`

**预期 Red 失败**

- 失败测试名: `graph_snapshot_roundtrip_persists_raw_unresolved_phase_and_fts_atomically`
- 关键错误 / 断言差异: `no such table: raw_imports`、`cannot find function replace_graph_snapshot` 或 current snapshot 未记录
- 失败原因: 当前 SQLite schema 还没有 raw/unresolved/files_fts/phase snapshot 的完整原子写入能力。

**Green 通过条件**

- 所有 graph facts 在事务完成后可读。
- FTS 与 phase runs 可读。
- current snapshot pointer 只指向完整 snapshot。

**Refactor 守卫**

- 不让 `scan_cache` 成为正式 source record 的唯一读取来源。
- scoped update 可先全局重写 analysis，但不能留下半新半旧 edges/raw/FTS。

### UT-009 Schema/read API 暴露 files/folders source authority

**目标行为**

graph schema 的 `files/folders` 是正式 source authority；写后读取 API 能暴露 files、folders、raw、unresolved、phase、FTS 和 current snapshot，且不依赖 `scan_cache` JSON 兜底。

**关联**

- Design: `files / folders 是 code graph 的结构化 source authority`、`scan_cache 可以继续保存原始 ScanReport，但正式 source record 读取应从 files 表出来`
- 系统测试用例: ST-008, ST-005
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`
- Modify: `crates/wiki-runtime/src/storage/sqlite_store.rs`、`crates/wiki-runtime/src/storage/sqlite/index_store.rs`、`crates/wiki-index/src/store.rs`
- Reference: `crates/wiki-index/src/store.rs`、`design.md`

**测试代码蓝图**

```rust
#[test]
fn graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_raw_and_unresolved();

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();
    sqlite_store::clear_scan_cache_for_test(&conn).unwrap();

    assert!(sqlite_store::list_files(repo.path()).unwrap().iter().any(|file| file.path == "src/service.ts"));
    assert!(sqlite_store::list_folders(repo.path()).unwrap().iter().any(|folder| folder.path == "src"));
    assert!(!sqlite_store::list_raw_imports(repo.path()).unwrap().is_empty());
    assert!(!sqlite_store::list_unresolved_refs(repo.path()).unwrap().is_empty());
    assert!(!sqlite_store::list_graph_phase_runs(repo.path()).unwrap().is_empty());
    assert!(!sqlite_store::search_files_fts(repo.path(), "service", 8).unwrap().is_empty());
    assert!(sqlite_store::read_current_graph_snapshot(repo.path()).unwrap().is_some());
}
```

**测试数据 / Fixture / Mock 边界**

- 使用临时 SQLite DB 和 graph snapshot fixture。
- `clear_scan_cache_for_test` 可换成等价测试 helper；测试意图是证明 graph source read API 不依赖 scan JSON。

**运行命令**

`cargo test -p wiki-runtime graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts --test sqlite_storage`

**预期 Red 失败**

- 失败测试名: `graph_schema_read_api_exposes_files_folders_raw_unresolved_phase_and_fts`
- 关键错误 / 断言差异: `list_files/list_folders/search_files_fts/read_current_graph_snapshot` 不存在，或清空 `scan_cache` 后 source 读取为空
- 失败原因: 当前正式 source 读取还没有完全从 graph files/folders 表闭合。

**Green 通过条件**

- schema/read API 覆盖 files/folders/raw/unresolved/phase/FTS/current snapshot。
- `scan_cache` 缺失不影响正式 graph source authority 读取。

**Refactor 守卫**

- `scan_cache` 可以作为缓存或内部 helper，但不能成为正式 source read API 的唯一真相。
- read API 必须通过 `wiki-index` trait/DTO 暴露给上层。

### UT-004 `.spec` persistence guard 拒绝 graph 表和 FTS 污染

**目标行为**

即使上游扫描误传 `.spec` 路径，SQLite graph 写入和 query 读取层也会拒绝 `.spec` 进入 files/symbols/edges/raw/unresolved/FTS。

**关联**

- Design: `.spec persistence guard`
- 系统测试用例: ST-003
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`
- Modify: `crates/wiki-runtime/src/storage/sqlite_store.rs`、`crates/wiki-runtime/src/storage/sqlite/index_store.rs`
- Reference: `crates/wiki-runtime/tests/repo/scanner_noise_filter.rs`、`crates/wiki-runtime/tests/hierarchy/hierarchy_noise_filter.rs`

**测试代码蓝图**

```rust
#[test]
fn graph_snapshot_rejects_spec_paths_before_tables_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let snapshot = sample_graph_snapshot_with_spec_path(".spec/changes/demo/tasks.md");

    sqlite_store::replace_graph_snapshot(&mut conn, &snapshot).unwrap();

    assert!(sqlite_store::list_sources(repo.path()).unwrap().iter().all(|source| !source.path.starts_with(".spec/")));
    assert!(sqlite_store::list_symbols(repo.path()).unwrap().iter().all(|symbol| !symbol.range.path.starts_with(".spec/")));
    assert!(sqlite_store::list_raw_imports(repo.path()).unwrap().is_empty());
    assert!(sqlite_store::search_files_fts(repo.path(), "governance", 8).unwrap().is_empty());
    assert!(sqlite_store::search_symbols_fts(repo.path(), "FakeSpecSymbol", 8).unwrap().is_empty());
}
```

**测试数据 / Fixture / Mock 边界**

- fixture 故意绕过 scanner，直接传入 `.spec` path。
- 测试目标是 persistence/query 双 guard，不是 scanner 本身。

**运行命令**

`cargo test -p wiki-runtime graph_snapshot_rejects_spec_paths_before_tables_and_fts --test sqlite_storage`

**预期 Red 失败**

- 失败测试名: `graph_snapshot_rejects_spec_paths_before_tables_and_fts`
- 关键错误 / 断言差异: `.spec/changes/demo/tasks.md` 出现在 `list_sources/list_symbols/search_*_fts`
- 失败原因: 当前过滤主要依赖 scanner，graph persistence 层尚未固定防污染 guard。

**Green 通过条件**

- `.spec` 路径不会进入任何 graph 表或 FTS。
- 业务源码路径不受影响。

**Refactor 守卫**

- guard 统一使用 repo-root-relative path 判断，避免 Windows 路径分隔符绕过。
- 不为 `.spec` 生成 tombstone 或 unresolved ref。

### UT-005 GraphReadiness evaluator 区分状态并投影 RuntimeReadiness

**目标行为**

Graph readiness evaluator 能基于 DB、关键表、schema/migration、snapshot fingerprint、phase 状态返回 missing/stale/blocked/rebuilding/ready，并正确投影到 runtime index readiness。

**关联**

- Design: `GraphReadiness`、`Readiness 决策表`
- 系统测试用例: ST-004, ST-006
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs` 或新增 `crates/wiki-runtime/tests/runtime/graph_readiness.rs`
- Modify: `crates/wiki-index/src/store.rs`、`crates/wiki-runtime/src/storage/state_store.rs`、`crates/wiki-runtime/src/workflows/status.rs`
- Reference: `.spec/archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/proposal.md`、`design.md`

**测试代码蓝图**

```rust
#[test]
fn graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready() {
    let missing = readiness_fixture_missing_db();
    assert_eq!(read_graph_readiness(missing.path()).unwrap().status, GraphReadinessStatus::Missing);

    let blocked = readiness_fixture_missing_required_table();
    let blocked_state = read_graph_readiness(blocked.path()).unwrap();
    assert_eq!(blocked_state.status, GraphReadinessStatus::Blocked);
    assert!(blocked_state.required_tables.iter().any(|table| table == "graph_snapshots"));

    let stale = readiness_fixture_stale_source_fingerprint();
    assert_eq!(read_graph_readiness(stale.path()).unwrap().status, GraphReadinessStatus::Stale);

    let rebuilding = readiness_fixture_phase_rebuilding();
    assert_eq!(read_graph_readiness(rebuilding.path()).unwrap().status, GraphReadinessStatus::Rebuilding);

    let diagnostic_blocked = readiness_fixture_blocking_diagnostic();
    assert_eq!(read_graph_readiness(diagnostic_blocked.path()).unwrap().status, GraphReadinessStatus::Blocked);

    let ready = readiness_fixture_complete_snapshot();
    assert_eq!(read_graph_readiness(ready.path()).unwrap().status, GraphReadinessStatus::Ready);
    assert_eq!(run_status(ready.path()).unwrap().readiness.index.as_str(), "ready");
}
```

**测试数据 / Fixture / Mock 边界**

- 使用临时 DB fixture 操作表和 snapshot metadata。
- 不依赖 CLI shell 输出；直接调用 runtime functions。

**运行命令**

`cargo test -p wiki-runtime graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready`

**预期 Red 失败**

- 失败测试名: `graph_readiness_distinguishes_missing_stale_blocked_rebuilding_ready`
- 关键错误 / 断言差异: `cannot find type GraphReadinessStatus` 或 missing/stale/blocked/rebuilding 被压成 bool
- 失败原因: 当前 readiness 仍以 boolean/helper 为主，缺少图层状态和诊断来源。

**Green 通过条件**

- 五类状态可区分。
- blocking diagnostic 与 schema/migration 不一致都明确映射为 blocked。
- runtime readiness index 与 graph readiness 映射一致。

**Refactor 守卫**

- Graph readiness 只评价 index/facts 层，不代表 knowledge/projection。
- 不 ready 时 query route gating 必须依赖该状态。

### UT-006 `wiki-index::query` 返回增强 symbol/path/graph facts-only 命中

**目标行为**

`wiki-index::query` 通过 trait DTO 返回 symbol/path/graph 命中，并携带 range、provenance、confidence、hop distance、diagnostics，不读取 runtime state 或 page fallback。

**关联**

- Design: `### 4. Query adapter`、`wiki-index::query 保持 facts-only`
- 系统测试用例: ST-005, ST-006
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-index/src/query.rs`
- Modify: `crates/wiki-index/src/query.rs`、`crates/wiki-index/src/store.rs`
- Reference: `crates/wiki-index/src/query.rs` existing tests、`crates/wiki-model/src/domain/query.rs`

**测试代码蓝图**

```rust
#[test]
fn index_query_returns_enriched_symbol_path_and_graph_hits() {
    let store = sample_store_with_extended_graph_facts();
    let result = run_query(
        &store,
        &IndexQueryRequest {
            intent: IndexQueryIntent::Auto,
            text: "handleCheckout".to_string(),
            ..IndexQueryRequest::default()
        },
    )
    .unwrap();

    assert!(result.symbols.iter().any(|hit| {
        hit.symbol_id == "symbol:handleCheckout"
            && hit.range.path == "src/controller.ts"
            && hit.provenance.parser_id == "tree-sitter-typescript"
    }));
    assert!(result.sources.iter().any(|hit| hit.path == "src/controller.ts"));
    assert!(result.call_edges.iter().any(|edge| {
        edge.edge_id == "edge:handleCheckout->runPayment"
            && edge.confidence > 0.0
            && edge.hop_distance >= 1
    }));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用 `MemoryStore` 扩展后的内存 trait fixture。
- 不 mock runtime state/page fallback。

**运行命令**

`cargo test -p wiki-index index_query_returns_enriched_symbol_path_and_graph_hits`

**预期 Red 失败**

- 失败测试名: `index_query_returns_enriched_symbol_path_and_graph_hits`
- 关键错误 / 断言差异: `SymbolHit has no field range`、path hit 只能来自 scan cache 或 graph hit 缺少 provenance/diagnostics
- 失败原因: 当前 query DTO 未闭合设计要求的增强 source refs/provenance/path/graph 信息。

**Green 通过条件**

- query 结果通过 trait DTO 读取 facts。
- symbol/path/graph hits 均含后续 runtime projection 需要的字段。

**Refactor 守卫**

- `wiki-index::query` 不读取 runtime state、Markdown page fallback 或 governance artifacts。
- 不新增公开 route tag。

### UT-007 Runtime query 在 index 不 ready 时抑制 index routes

**目标行为**

runtime query 只在 graph readiness ready 时把 index DTO 投影成 `index_symbol_hit / index_path_hit / index_graph_hit`；missing/stale/blocked/rebuilding 时不得返回 index 伪命中，也不得把 markdown/page fallback 伪装成 index 命中。

**关联**

- Design: `index 不 ready 时，query workflow...不得产出 index_symbol_hit / index_path_hit / index_graph_hit 伪命中`
- 系统测试用例: ST-004, ST-006
- Tasks: 4.4 Red / 4.5 Green / 4.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
- Modify: `crates/wiki-runtime/src/workflows/query.rs`、`crates/wiki-runtime/src/storage/state_store.rs`
- Reference: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs` existing `query_does_not_emit_index_routes_when_index_is_not_ready`

**测试代码蓝图**

```rust
#[test]
fn query_suppresses_index_routes_for_all_non_ready_graph_states() {
    for fixture in [
        readiness_fixture_missing_db(),
        readiness_fixture_stale_source_fingerprint(),
        readiness_fixture_missing_required_table(),
        readiness_fixture_phase_rebuilding(),
    ] {
        let query = run_query(fixture.path(), "handleCheckout").unwrap_or_else(|err| {
            assert!(err.to_string().contains("index not ready") || err.kind() == std::io::ErrorKind::NotFound);
            return empty_query_result_for_assertion();
        });
        assert!(query.results.iter().all(|result| !result.route_tag.is_index_route()));
        assert!(query.route_groups.iter().all(|group| !group.route_tag.is_index_route()));
    }

    let fallback_query = run_query_with_markdown_only_hit("handleCheckout").unwrap();
    assert!(fallback_query.results.iter().all(|result| !result.route_tag.is_index_route()));
    assert!(QueryRouteTag::all_for_test().iter().all(|tag| {
        matches!(
            tag,
            QueryRouteTag::IndexSymbolHit
                | QueryRouteTag::IndexPathHit
                | QueryRouteTag::IndexGraphHit
                | QueryRouteTag::KnowledgeDeclaredHit
                | QueryRouteTag::KnowledgeDerivedHit
                | QueryRouteTag::GovernanceEvidenceRef
                | QueryRouteTag::GovernanceSummaryHit
                | QueryRouteTag::ProjectionRef
                | QueryRouteTag::RenderedPageDebugFallback
        )
    }));
}
```

**测试数据 / Fixture / Mock 边界**

- 复用 UT-005 readiness fixture。
- 允许 missing DB 返回显式 error；如果 runtime level1 可恢复，必须仍无 index route。

**运行命令**

`cargo test -p wiki-runtime query_suppresses_index_routes_for_all_non_ready_graph_states`

**预期 Red 失败**

- 失败测试名: `query_suppresses_index_routes_for_all_non_ready_graph_states`
- 关键错误 / 断言差异: stale/blocked/rebuilding 仍返回 `IndexSymbolHit` 或 `IndexGraphHit`
- 失败原因: 当前 query gating 只覆盖部分 missing cache 场景，尚未统一绑定 GraphReadiness。

**Green 通过条件**

- 所有非 ready graph 状态不产生 index route。
- query 可降级到 knowledge/projection 或显式 diagnostic，但不能伪造 graph hits。
- markdown/page fallback route 保持自身 route tag，不新增 route tag。

**Refactor 守卫**

- 不把 page fallback、knowledge 或 projection 命中标成 index route。
- `QueryRouteTag` 不新增、不重命名。

### UT-010 Scoped update 替换受影响文件并清理旧 graph facts

**目标行为**

scoped update 后，受影响文件旧 symbol、edge、raw capture、unresolved ref 和 FTS 命中被替换；未受影响文件 graph facts 保留。

**关联**

- Design: `scoped update 可以先采用“受影响文件替换 + 全局 analysis 重写”的现有策略`
- 系统测试用例: ST-002, ST-008
- Tasks: 2.7 Red / 2.8 Green / 2.9 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/sqlite_storage.rs`
- Modify: `crates/wiki-runtime/src/storage/sqlite_store.rs`、`crates/wiki-runtime/src/storage/sqlite/index_store.rs`
- Reference: existing `symbol_graph_for_files_refresh_replaces_stale_edges_and_analysis` test、`design.md`

**测试代码蓝图**

```rust
#[test]
fn scoped_graph_snapshot_refresh_replaces_symbols_raw_unresolved_edges_and_fts() {
    let repo = make_repo();
    let mut conn = sqlite_store::open_db(repo.path()).unwrap();
    let initial = sample_graph_snapshot_with_raw_and_unresolved();
    sqlite_store::replace_graph_snapshot(&mut conn, &initial).unwrap();

    let refreshed = sample_scoped_graph_snapshot_for_file("src/service.ts", "PaymentServiceV2");
    sqlite_store::replace_graph_snapshot_for_files(&mut conn, &["src/service.ts".to_string()], &refreshed).unwrap();

    assert!(sqlite_store::list_symbols(repo.path()).unwrap().iter().any(|symbol| symbol.name == "PaymentServiceV2"));
    assert!(sqlite_store::list_symbols(repo.path()).unwrap().iter().all(|symbol| symbol.name != "PaymentService"));
    assert!(sqlite_store::list_raw_calls(repo.path()).unwrap().iter().all(|capture| capture.base.file_id != "file:src/service.ts" || capture.base.raw_text.contains("PaymentServiceV2")));
    assert!(sqlite_store::list_unresolved_refs(repo.path()).unwrap().iter().all(|item| item.file_id != "file:src/service.ts" || item.reference_name.contains("V2")));
    assert!(sqlite_store::search_symbols_fts(repo.path(), "PaymentService", 8).unwrap().iter().all(|hit| hit.name != "PaymentService"));
    assert!(sqlite_store::list_files(repo.path()).unwrap().iter().any(|file| file.path == "src/helper.ts"));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用 SQLite fixture。
- 允许第一阶段全局重写 analysis，但受影响文件旧 facts 不得残留。

**运行命令**

`cargo test -p wiki-runtime scoped_graph_snapshot_refresh_replaces_symbols_raw_unresolved_edges_and_fts --test sqlite_storage`

**预期 Red 失败**

- 失败测试名: `scoped_graph_snapshot_refresh_replaces_symbols_raw_unresolved_edges_and_fts`
- 关键错误 / 断言差异: 旧 symbol/edge/raw/unresolved/FTS 命中仍存在
- 失败原因: 当前 scoped refresh 只覆盖 symbols/edges/analysis，未覆盖 raw/unresolved/files_fts 的一致替换。

**Green 通过条件**

- 受影响文件旧 graph facts 全部清理。
- 未受影响文件 facts 保留。

**Refactor 守卫**

- snapshot pointer 只在 scoped update 完成后切换。
- 不引入旧实现兼容路径。

### UT-008 init/update/rebuild 写入 phase diagnostics 与 graph snapshot metadata

**目标行为**

init、update、rebuild 主链会写入 graph snapshot metadata、phase runs、fail-soft diagnostics 和 unresolved refs，且 diagnostics 能定位 file/parser/capture/resolver。

**关联**

- Design: `### 3. Workflow 与 readiness`、`phase diagnostics`
- 系统测试用例: ST-005, ST-007
- Tasks: 3.4 Red / 3.5 Green / 3.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs` 或新增 `crates/wiki-runtime/tests/runtime/graph_phase_diagnostics.rs`
- Modify: `crates/wiki-runtime/src/workflows/init.rs`、`crates/wiki-runtime/src/workflows/update.rs`、`crates/wiki-runtime/src/workflows/rebuild.rs`、`crates/wiki-index/src/symbol_graph/pipeline.rs`
- Reference: `crates/wiki-runtime/tests/symbols/symbol_parsing.rs`、`crates/wiki-runtime/tests/symbols/symbol_graph_analysis.rs`

**测试代码蓝图**

```rust
#[test]
fn workflows_write_graph_phase_diagnostics_and_unresolved_refs() {
    let fixture = tempdir().unwrap();
    let repo_root = fixture.path();
    write_repo_file(repo_root, "src/service.ts", "import { missing } from './missing';\nexport function run() { return missing(); }\n");

    run_init(repo_root).unwrap();

    let phases = sqlite_store::list_graph_phase_runs(repo_root).unwrap();
    assert!(phases.iter().any(|phase| phase.phase == GraphPhase::ResolveImports));
    assert!(phases.iter().any(|phase| phase.phase == GraphPhase::ResolveCalls));

    let unresolved = sqlite_store::list_unresolved_refs(repo_root).unwrap();
    assert!(unresolved.iter().any(|item| {
        item.reference_name == "missing"
            && item.file_id.starts_with("file:")
            && matches!(item.resolver_phase, GraphPhase::ResolveImports | GraphPhase::ResolveCalls)
    }));

    let snapshot = sqlite_store::read_current_graph_snapshot(repo_root).unwrap().unwrap();
    assert_eq!(snapshot.status, GraphSnapshotStatus::Ready);
}
```

**测试数据 / Fixture / Mock 边界**

- 使用真实 temp repo + runtime workflow。
- 不要求 resolver 成功；重点验证 fail-soft diagnostics 和 unresolved facts。

**运行命令**

`cargo test -p wiki-runtime workflows_write_graph_phase_diagnostics_and_unresolved_refs`

**预期 Red 失败**

- 失败测试名: `workflows_write_graph_phase_diagnostics_and_unresolved_refs`
- 关键错误 / 断言差异: `list_graph_phase_runs` 不存在、unresolved refs 为空或 phase diagnostics 缺少 file/capture/resolver
- 失败原因: 当前 workflow 已有 symbol graph pipeline，但未把 phase diagnostics 和 unresolved refs 作为 graph facts 写入。

**Green 通过条件**

- init/update/rebuild 均通过同一 graph snapshot 写入路径。
- phase runs 和 unresolved refs 可查询、可诊断。

**Refactor 守卫**

- parse/resolve fail-soft 不得阻断整个 workflow 的可用部分。
- diagnostics 不得只写日志而不进入 graph facts。

## 测试辅助边界

- 可新增 Rust test helper：`sample_graph_snapshot_with_raw_and_unresolved`、`sample_graph_snapshot_with_spec_path`、readiness fixtures、temp repo writer。
- helper 必须留在测试模块或测试 support 范围，不能成为未设计的生产 API。
- 不引入网络、外部数据库、浏览器或 LLM mock。

## 不纳入单元测试的内容

- 完整解析准确率、type/scope/MRO/callback synthesis：非本 change 目标。
- CLI help/产品面命令：属于后续 CLI surface child。
- governance artifact index：属于后续 governance isolation child。
