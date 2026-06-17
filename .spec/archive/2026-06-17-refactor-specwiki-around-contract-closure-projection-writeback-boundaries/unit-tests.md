# refactor-specwiki-around-contract-closure-projection-writeback-boundaries 单元测试设计

## 测试总览

本文件定义 TDD 模式下的单元测试蓝图。UT 覆盖共享 projection 模型、runtime marker protocol、drift 分类、knowledge declared validation、metadata binding、projection digest 归属和旧 fallback 删除。系统级闭环由 `system-tests.md` 的 ST-* 覆盖；本文件只描述计划中的单元测试，不写真实测试代码。

## 单元测试用例

### UT-001 Projection Model Rejects Incomplete Section Binding

**目标行为**

验证 `SectionOwnership`、`SectionBinding`、`ProjectionDigest`、`SyncResultKind` 是 `wiki-model` 的正式合同，并拒绝缺失 id、owner、refs 或 hash 的无效 binding。

**关联**

- Design: 数据设计 / 共享模型；设计决策 / `ProjectionDigest` 放入 `wiki-model/src/domain/projection.rs`
- 系统测试用例: ST-001, ST-005, ST-006
- Tasks: 1.1 Red / 1.2 Green / 1.3 Refactor

**Files**

- Test: `crates/wiki-model/src/domain/projection.rs`
- Modify: `crates/wiki-model/src/domain/projection.rs`, `crates/wiki-model/src/domain/mod.rs`, `crates/wiki-model/src/lib.rs`
- Reference: `crates/wiki-model/src/domain/knowledge_artifact.rs`

**测试代码蓝图**

```rust
#[test]
fn section_binding_requires_owner_refs_and_hashes() {
    let valid = sample_section_binding();
    assert!(valid.validate().is_ok());

    let missing_owner = SectionBinding {
        owner_kind: None,
        ..sample_section_binding()
    };
    assert!(matches!(
        missing_owner.validate(),
        Err(ProjectionContractError::MissingOwner { .. })
    ));

    let missing_hash = SectionBinding {
        content_hash: String::new(),
        ..sample_section_binding()
    };
    assert!(matches!(
        missing_hash.validate(),
        Err(ProjectionContractError::MissingContentHash { .. })
    ));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用纯模型 fixture，不读写文件。
- `sample_section_binding()` 必须包含 `section_id / owner_kind / knowledge_refs / source_refs / input_hash / content_hash / projection_digest_ref`。

**运行命令**

`cargo test -p wiki-model section_binding_requires_owner_refs_and_hashes`

**预期 Red 失败**

- 失败测试名: `section_binding_requires_owner_refs_and_hashes`
- 关键错误 / 断言差异: `cannot find type SectionBinding` 或 `no method named validate`
- 失败原因: `wiki-model/src/domain/projection.rs` 尚不存在或模型合同未实现。

**Green 通过条件**

- `SectionOwnership` 支持 `declared_managed / derived_managed / projection_static / manual_unmanaged / external_ref`。
- `SectionBinding::validate` 能拒绝缺失 owner、refs、hash 的对象。
- `ProjectionDigest` 和 `SyncResultKind` 可 serde roundtrip。

**Refactor 守卫**

- 不把 `ProjectionDigest` 放入 `knowledge_artifact.rs`。
- 不重新引入 `managed: bool` 作为正式 projection 合同字段。

### UT-002 Runtime Marker Parser Emits Typed Diagnostics

**目标行为**

验证 runtime marker parser 对缺 `id`、缺 `owner`、非法 version、start/end mismatch 和 metadata owner mismatch 返回 typed diagnostics，而不是 warning string 或 best-effort fallback。

**关联**

- Design: Runtime Page Protocol；接口设计 / Runtime-facing API
- 系统测试用例: ST-001, ST-003, ST-007
- Tasks: 2.1 Red / 2.2 Green / 2.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`
- Modify: `crates/wiki-runtime/src/generation/managed_sections.rs`
- Reference: `crates/wiki-runtime/src/generation/managed_sections.rs`

**测试代码蓝图**

```rust
#[test]
fn parse_marker_without_owner_reports_typed_diagnostic() {
    let content = r#"# Page

<!-- wiki:managed:start id=section:intro title="Intro" version=2 -->
## Intro

body
<!-- wiki:managed:end id=section:intro -->
"#;

    let parsed = parse_wiki_page(content, &empty_binding_index());

    assert!(parsed.diagnostics.iter().any(|diagnostic| {
        matches!(
            diagnostic.kind,
            PageParseDiagnosticKind::MarkerMissingOwner
        )
    }));
    assert!(parsed.blocks.is_empty());
}
```

**测试数据 / Fixture / Mock 边界**

- 使用内联 Markdown 字符串。
- `empty_binding_index()` 是测试 helper，只提供 metadata binding 空视图，不模拟 knowledge 行为。

**运行命令**

`cargo test -p wiki-runtime --test runtime parse_marker_without_owner_reports_typed_diagnostic`

**预期 Red 失败**

- 失败测试名: `parse_marker_without_owner_reports_typed_diagnostic`
- 关键错误 / 断言差异: `no field diagnostics on type ParsedWikiPage` 或 parser 默认接受缺 owner marker
- 失败原因: 当前 parser 只有 string warnings，且 marker schema 不要求 owner。

**Green 通过条件**

- `ParsedWikiPage` 暴露 typed diagnostics。
- 缺 owner / 缺 id / version 非法 / id mismatch / owner mismatch 都能映射到明确 diagnostic kind。
- 无效 managed marker 不被解析成合法 managed section。

**Refactor 守卫**

- 不把 marker 字符串解析逻辑移到 `wiki-knowledge`。
- 不使用 warning 文本作为测试唯一依据。

### UT-003 Marker Render Roundtrip Preserves Ownership And Binding

**目标行为**

验证 `render_page_with_markers -> parse_wiki_page` roundtrip 保留 `owner_kind`、content hash、generated content hash、knowledge refs、source refs 和 projection digest ref。

**关联**

- Design: Runtime Page Protocol；Metadata Binding
- 系统测试用例: ST-001, ST-005
- Tasks: 2.4 Red / 2.5 Green / 2.6 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`
- Modify: `crates/wiki-runtime/src/generation/managed_sections.rs`, `crates/wiki-runtime/src/generation/renderer.rs`
- Reference: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`

**测试代码蓝图**

```rust
#[test]
fn render_parse_roundtrip_preserves_section_binding() {
    let block = ManagedSectionBlock {
        section_id: "section:intro".to_string(),
        owner_kind: SectionOwnership::DerivedManaged,
        title: "简介".to_string(),
        version: MARKER_VERSION,
        body: "generated body".to_string(),
        knowledge_refs: vec!["unit:repo".to_string()],
        source_refs: vec!["src/lib.rs".to_string()],
        input_hash: "input-hash".to_string(),
        content_hash: "content-hash".to_string(),
        generated_content_hash: Some("content-hash".to_string()),
        projection_digest_ref: Some("projection:repo".to_string()),
    };

    let rendered = render_page_with_markers("Repo", &[PageBlock::Managed(block.clone())]);
    let parsed = parse_wiki_page(&rendered, &binding_index_for(&block));

    assert_eq!(parsed.managed_blocks()[0].owner_kind, block.owner_kind);
    assert_eq!(parsed.managed_blocks()[0].projection_digest_ref, block.projection_digest_ref);
    assert_eq!(parsed.managed_blocks()[0].knowledge_refs, block.knowledge_refs);
}
```

**测试数据 / Fixture / Mock 边界**

- 使用单页单 section fixture。
- metadata binding helper 只验证 owner/hash/ref 对齐。

**运行命令**

`cargo test -p wiki-runtime --test runtime render_parse_roundtrip_preserves_section_binding`

**预期 Red 失败**

- 失败测试名: `render_parse_roundtrip_preserves_section_binding`
- 关键错误 / 断言差异: `ManagedSectionBlock` 缺少 `owner_kind` / `knowledge_refs` / `projection_digest_ref`
- 失败原因: 当前 marker block 只有 `section_id/title/version/body`。

**Green 通过条件**

- marker schema 可表达 owner 与 binding refs。
- render/parse roundtrip 不丢失 binding 字段。

**Refactor 守卫**

- 不通过标题或顺序猜测 ownership。
- 不把 metadata refs 塞进 Markdown 正文非结构化区域。

### UT-004 Drift Classifier Covers Five Result Kinds

**目标行为**

验证 `classify_section_drift` 能按 ownership 与 hash/binding 差异输出 `declared_writeback / metadata_only / illegal_drift / conflict / stale` 五类结果。

**关联**

- Design: Drift 与异常分类；Section Ownership 行为
- 系统测试用例: ST-002, ST-003, ST-004
- Tasks: 3.1 Red / 3.2 Green / 3.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`
- Modify: `crates/wiki-runtime/src/generation/managed_sections.rs`, `crates/wiki-runtime/src/workflows/sync.rs`
- Reference: `crates/wiki-runtime/src/workflows/sync.rs`

**测试代码蓝图**

```rust
#[test]
fn classify_section_drift_returns_all_contract_kinds() {
    assert_eq!(
        classify_section_drift(&declared_edit(), &declared_binding()).kind,
        SyncResultKind::DeclaredWriteback
    );
    assert_eq!(
        classify_section_drift(&manual_edit(), &manual_binding()).kind,
        SyncResultKind::MetadataOnly
    );
    assert_eq!(
        classify_section_drift(&derived_body_drift(), &derived_binding()).kind,
        SyncResultKind::IllegalDrift
    );
    assert_eq!(
        classify_section_drift(&owner_mismatch(), &declared_binding()).kind,
        SyncResultKind::Conflict
    );
    assert_eq!(
        classify_section_drift(&stale_input_hash(), &derived_binding()).kind,
        SyncResultKind::Stale
    );
}
```

**测试数据 / Fixture / Mock 边界**

- 使用纯 section block + section binding fixture。
- 不调用 `run_sync`，系统流程由 ST 覆盖。

**运行命令**

`cargo test -p wiki-runtime --test runtime classify_section_drift_returns_all_contract_kinds`

**预期 Red 失败**

- 失败测试名: `classify_section_drift_returns_all_contract_kinds`
- 关键错误 / 断言差异: `cannot find function classify_section_drift` 或 `SyncResultKind` 缺少 `Conflict/Stale`
- 失败原因: 当前 drift 分类散落在 sync workflow 中，且结果分类不完整。

**Green 通过条件**

- 五类结果均有稳定 enum。
- derived/static drift 不会被分类为 declared writeback。
- manual edit 不会生成 declared candidate。

**Refactor 守卫**

- 不把结果压平成 success/warning。
- 不用字符串比较作为正式分类。

### UT-005 Declared Writeback Validation Is Knowledge-Owned

**目标行为**

验证 `wiki-knowledge` 只消费结构化 `DeclaredAuthoringCandidate`，合法 candidate 输出 `DeclaredRecordPatch`，scope/lifecycle/relation 冲突输出 rejected/conflict。

**关联**

- Design: Declared Writeback；Knowledge-facing API
- 系统测试用例: ST-002, ST-004
- Tasks: 4.1 Red / 4.2 Green / 4.3 Refactor

**Files**

- Test: `crates/wiki-knowledge/src/declared_writeback.rs`
- Modify: `crates/wiki-knowledge/src/declared_writeback.rs`, `crates/wiki-knowledge/src/lib.rs`, `crates/wiki-model/src/domain/projection.rs`
- Reference: `crates/wiki-model/src/domain/knowledge_artifact.rs`

**测试代码蓝图**

```rust
#[test]
fn validate_declared_candidate_normalizes_scope_and_lifecycle() {
    let candidate = DeclaredAuthoringCandidate {
        authoring_id: "marker:runtime-policy".to_string(),
        record_kind: DeclaredKnowledgeRecordKind::Policy,
        scope_ref: "module:runtime".to_string(),
        lifecycle: DeclaredLifecycleInput::Active,
        source_ref: "manual".to_string(),
        page_id: "page:runtime".to_string(),
        section_id: "section:runtime-policy".to_string(),
        body: "runtime contract".to_string(),
        baseline_hash: "old".to_string(),
        current_hash: "new".to_string(),
        marker_version: 2,
        metadata_binding_ref: Some("binding:runtime-policy".to_string()),
        relations: Vec::new(),
    };

    let decision = validate_declared_writeback(&candidate, &empty_declared_snapshot());

    assert!(matches!(decision, DeclaredWritebackDecision::Accepted(_)));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用纯 DTO fixture。
- 测试不得包含 Markdown marker 字符串，不读写文件，不接收 path writer 或 metadata patch。

**运行命令**

`cargo test -p wiki-knowledge validate_declared_candidate_normalizes_scope_and_lifecycle`

**预期 Red 失败**

- 失败测试名: `validate_declared_candidate_normalizes_scope_and_lifecycle`
- 关键错误 / 断言差异: `cannot find function validate_declared_writeback`
- 失败原因: declared materialize / lifecycle validation 仍在 runtime sync 或 model/storage 层。

**Green 通过条件**

- 合法 candidate 输出 `DeclaredRecordPatch`。
- scope/lifecycle/relation 冲突输出 `Rejected` 或 `Conflict`。
- API 不接受 Markdown、marker string、文件路径写入句柄或 metadata patch。

**Refactor 守卫**

- `wiki-runtime` 不重新实现 declared scope/lifecycle 语义。
- `wiki-knowledge` 不依赖 `wiki-runtime`。

### UT-006 Metadata Mapper Emits Section Reverse Refs

**目标行为**

验证 metadata mapper 能生成 page -> section -> knowledge/source/projection_digest 的正向索引，以及 knowledge/source/projection_digest -> page/section 的反向索引，并拒绝悬挂引用。

**关联**

- Design: Metadata Binding；非功能性设计 / 可靠性
- 系统测试用例: ST-001, ST-005, ST-006
- Tasks: 5.1 Red / 5.2 Green / 5.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/metadata_roundtrip.rs`
- Modify: `crates/wiki-model/src/domain/metadata.rs`, `crates/wiki-runtime/src/domain/metadata_mapper.rs`
- Reference: `crates/wiki-runtime/src/domain/metadata_mapper.rs`

**测试代码蓝图**

```rust
#[test]
fn metadata_mapper_builds_section_reverse_refs() {
    let metadata = metadata_from_page_with_section_binding(sample_page_result());

    let section = metadata.section("section:intro").expect("section binding");
    assert_eq!(section.knowledge_refs, vec!["unit:repo"]);
    assert_eq!(
        metadata.reverse_refs.knowledge_to_sections["unit:repo"],
        vec!["section:intro"]
    );
    assert_eq!(
        metadata.reverse_refs.projection_to_sections["projection:repo"],
        vec!["section:intro"]
    );
}
```

**测试数据 / Fixture / Mock 边界**

- 使用 metadata mapper fixture，不运行 full init。
- 悬挂引用测试使用缺失 knowledge ref 的 sample binding。

**运行命令**

`cargo test -p wiki-runtime --test runtime metadata_mapper_builds_section_reverse_refs`

**预期 Red 失败**

- 失败测试名: `metadata_mapper_builds_section_reverse_refs`
- 关键错误 / 断言差异: metadata 无 section-level reverse refs 字段
- 失败原因: 当前 metadata 主要是 page-level binding，缺少 section 反向索引。

**Green 通过条件**

- metadata 能从 page 查到 section binding。
- metadata 能从 knowledge/source/projection digest 反查 page/section。
- 悬挂 section/page/artifact ref 被拒绝或标记为 conflict。

**Refactor 守卫**

- metadata 不承载 knowledge 正文。
- projection readiness 主状态不与 `ProjectionDigest` 双写。

### UT-007 Projection Digest Is The Only Persisted Projection Anchor

**目标行为**

验证 `PageDraft` 只作为 transient compose/render 输入，不能被 storage 当作 persisted restore anchor；`ProjectionDigest` 是 `.wiki/.knowledge/runtime/**` 的持久化 projection anchor。

**关联**

- Design: 数据设计 / `PageDigest` 拆分；设计决策 / `ProjectionDigest`
- 系统测试用例: ST-001, ST-005
- Tasks: 6.1 Red / 6.2 Green / 6.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/knowledge_artifacts_roundtrip.rs`
- Modify: `crates/wiki-model/src/domain/projection.rs`, `crates/wiki-knowledge/src/domain/compose.rs`, `crates/wiki-knowledge/src/domain/research.rs`, `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- Reference: `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`

**测试代码蓝图**

```rust
#[test]
fn persisted_artifacts_use_projection_digest_not_page_draft() {
    let artifacts = sample_persist_input_with_projection_digest();
    persist_knowledge_artifacts(artifacts).unwrap();

    let loaded = load_knowledge_artifacts(repo_root()).unwrap();

    assert!(!loaded.projection_digests.is_empty());
    assert!(loaded.projection_digests.iter().all(|digest| {
        digest.validate().is_ok()
    }));
    assert_no_page_draft_artifact_exists(repo_root());
}
```

**测试数据 / Fixture / Mock 边界**

- 使用现有 artifact roundtrip fixture。
- 不测试 query 输出，不新增 CLI snapshot。

**运行命令**

`cargo test -p wiki-runtime --test runtime persisted_artifacts_use_projection_digest_not_page_draft`

**预期 Red 失败**

- 失败测试名: `persisted_artifacts_use_projection_digest_not_page_draft`
- 关键错误 / 断言差异: storage 仍暴露 `page_digests` 依赖 knowledge 私有类型，缺少 `projection_digests`
- 失败原因: persisted digest 尚未上提到 `wiki-model/src/domain/projection.rs`。

**Green 通过条件**

- runtime storage 持久化 `ProjectionDigest`。
- `PageDraft` 不出现在 `.wiki/.knowledge/**` persisted artifact。
- restore 依赖 projection digest 而不是 transient draft。

**Refactor 守卫**

- 不把 compose 子页摘要和 projection readiness/status 继续混在同一个 `PageDigest`。
- 不在 storage adapter 内派生 projection health。

### UT-008 Legacy Heading Fallback Is Removed

**目标行为**

验证无 marker 的 heading 页面不会被 best-effort 解析成 managed section，`.wiki/pages/**` 不作为 runtime page surface。

**关联**

- Design: 兼容性设计；Runtime Page Protocol
- 系统测试用例: ST-007
- Tasks: 7.1 Red / 7.2 Green / 7.3 Refactor

**Files**

- Test: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`
- Modify: `crates/wiki-runtime/src/generation/managed_sections.rs`, `crates/wiki-runtime/src/storage/wiki_fs.rs`
- Reference: `crates/wiki-runtime/tests/runtime/managed_section_kernel.rs`

**测试代码蓝图**

```rust
#[test]
fn page_without_markers_is_not_legacy_managed() {
    let content = "# Repo\n\n## 简介\n\nold body";
    let parsed = parse_wiki_page(content, &empty_binding_index());

    assert!(parsed.managed_blocks().is_empty());
    assert!(parsed.diagnostics.iter().any(|diagnostic| {
        matches!(diagnostic.kind, PageParseDiagnosticKind::MarkerMissing)
    }));
}
```

**测试数据 / Fixture / Mock 边界**

- 使用无 marker Markdown fixture。
- 旧 `.wiki/pages/**` guard 使用临时目录，不运行 query/CLI。

**运行命令**

`cargo test -p wiki-runtime --test runtime page_without_markers_is_not_legacy_managed`

**预期 Red 失败**

- 失败测试名: `page_without_markers_is_not_legacy_managed`
- 关键错误 / 断言差异: `parse_wiki_page` 返回 `PageParseMode::LegacyHeadings` 并生成 managed block
- 失败原因: 当前 parser 仍保留 legacy heading fallback。

**Green 通过条件**

- `parse_with_legacy_headings` 不再作为正式路径。
- 无 marker 页面返回 typed diagnostic 或 unmanaged-only 结果，不自动生成 managed section。
- `.wiki/pages/**` 不进入 official page path。

**Refactor 守卫**

- 删除或改写保护 legacy 行为的旧测试。
- 不新增 migration/diagnostic 兼容分支。

## 测试辅助边界

- 允许新增 Rust test helper：sample `SectionBinding`、sample `ProjectionDigest`、sample metadata binding index、sample declared candidate。
- 允许调整现有 runtime integration helper 以创建 owned section marker。
- 不允许 mock `wiki-knowledge` 的 declared validation 结果来替代真实 validation UT。
- 不允许用 query/CLI 输出作为本 change 的单元测试断言。

## 不纳入单元测试的内容

- 完整 `init/update/sync/restore` 生命周期闭环由 ST 覆盖。
- Query route/readiness、graph/index、CLI help、governance/archive 均为本 change 非目标。
- 完整 snapshot commit protocol 重构不在本 child 中做 UT；只测试现有 manifest binding 接入。
