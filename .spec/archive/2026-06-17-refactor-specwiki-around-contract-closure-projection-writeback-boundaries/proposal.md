# refactor-specwiki-around-contract-closure-projection-writeback-boundaries

## 问题

新版 SpecWiki 已经明确 `.wiki/.knowledge/**` 是 shared knowledge truth，正式可见 `.wiki/**/*.md` 页面树是 projection output / authoring surface，`wiki.metadata.json` 是绑定索引。但当前实现中 projection planning、page compose、render/merge/write、declared authoring writeback、metadata binding 和 artifact persistence 的职责边界仍然交叉。

现状中 `wiki-knowledge` 已经承担 `PlannedPage`、section contract、`PageDraft` 和 `PageDigest` 等 projection intent / compose 产物；`wiki-runtime` 同时承担页面 parse、render、merge、write、metadata 更新、snapshot commit，并在 `sync` 中直接解析 declared marker、构造 declared record、判定 drift 和提交 artifact。这个边界如果继续漂移，会让 Markdown 页面重新变成隐式 truth source，也会让 declared writeback、derived drift、projection readiness 和 restore binding 难以独立验收。

本 change 要解决的真实问题是：把 `wiki-knowledge` 与 `wiki-runtime` 在 projection/writeback 主链上的职责切清，让 knowledge 层表达计划、合同和语义校验，让 runtime 层执行页面操作、落盘和绑定提交。

## 目标

- 收口 projection/writeback 边界，使 `wiki-knowledge` 只拥有 knowledge planning、page/section projection intent、declared authoring contract validation / normalization 等语义职责。
- 明确 `wiki-runtime` 拥有 visible wiki page parse、managed marker / drift 分类、render、merge、page write、metadata binding、snapshot commit 与 restore binding。
- 将 declared authoring writeback 闭合为：runtime parse/extract，knowledge/model validate/normalize，runtime persist/metadata/stale commit。
- 让 derived/projection/static 区段漂移只产生 `illegal_drift`、`metadata_only`、`conflict` 或 `stale` 等状态，不得写回 `.wiki/.knowledge/derived/**`。
- 消除或隔离双 page generation 通道、重复 domain 模型、ownership marker 语义不一致造成的职责漂移。

## 非目标

- 不处理 query route tags、query result DTO、trust/readiness、人类可解释 query 输出；这些属于 `refactor-specwiki-around-contract-closure-query-route-readiness`。
- 不处理 code graph/index schema、GraphStore、IndexQueryStore、symbol/import/call/heritage graph；这些属于 `refactor-specwiki-around-contract-closure-code-graph-index`。
- 不处理 CLI 默认命令面、help、landing state、产品文案或命令重命名；这些属于 `refactor-specwiki-around-contract-closure-cli-product-surface`。
- 不处理 governance evidence index、governance summary、archive readiness 或 archive 事务；这些属于后续 governance / archive child。
- 不迁移、不兼容、不诊断 `.wiki/pages/**` 或旧页面树。
- 不提升 LLM research 质量，不扩展 provider 能力，不重做整体 knowledge planning 算法。

## 成功标准

- 自动生成的 managed section 均有稳定 section id、ownership、knowledge/source/projection binding 和 content hash。
- `wiki.metadata.json` 能反查 page -> section -> knowledge refs，并且不会指向不存在的 page、section 或 knowledge artifact。
- 合法 declared authoring 修改后，`sync` 产生 `declared_writeback`，更新 `.wiki/.knowledge/declared/**`，并标记 impacted unit/projection stale。
- derived/projection/static managed section 被人工修改时，结果必须是 `illegal_drift` 或等价 conflict，不得改写 `.wiki/.knowledge/derived/**`。
- manual/unmanaged section 变化只能进入 metadata-only 或等价非 truth 写回状态，不能自动生成 declared record。
- `init / update / sync / restore` 后 `.wiki/.knowledge/**`、正式页面树、`wiki.metadata.json`、snapshot manifest 的绑定保持一致。
- runtime 不从 Markdown 正文反推 derived knowledge；knowledge 不承担页面落盘、metadata、cache 或 transport。
- 双 page generation 通道和重复模型不再造成对外行为分叉。
- 当前阶段不保留 `.wiki/pages/**` 或 legacy compatibility 逻辑作为验收前提。

## 影响范围

- `crates/wiki-knowledge/src/projection.rs`
- `crates/wiki-knowledge/src/compose.rs`
- `crates/wiki-knowledge/src/domain/compose.rs`
- `crates/wiki-knowledge/src/domain/research.rs`
- `crates/wiki-knowledge/src/section_contract.rs`
- `crates/wiki-knowledge/src/store.rs`
- `crates/wiki-model/src/domain/knowledge_artifact.rs`
- `crates/wiki-runtime/src/generation/**`
- `crates/wiki-runtime/src/workflows/init.rs`
- `crates/wiki-runtime/src/workflows/update.rs`
- `crates/wiki-runtime/src/workflows/sync.rs`
- `crates/wiki-runtime/src/workflows/page_render.rs`
- `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- `crates/wiki-runtime/src/storage/sqlite/knowledge_store.rs`
- `crates/wiki-runtime/src/domain/metadata_mapper.rs`
- `crates/wiki-runtime/src/domain/state.rs`
- `crates/wiki-runtime/src/domain/change_set.rs`
- Runtime integration tests, especially `runtime/knowledge_artifacts_roundtrip.rs`, `runtime/query_sync_rebuild.rs`, `runtime/section_template_integration.rs`, `runtime/editable_runtime.rs`, and `runtime/status_and_update.rs`.

## 交付形态

single-change

这是 `refactor-specwiki-around-contract-closure` parent split 下的 child change。它只交付 projection/writeback ownership boundary，不扩大到 query、graph、CLI、governance 或 archive。

## 风险

- 当前 `wiki-runtime/src/workflows/sync.rs` 同时承担 parse、declared materialize、snapshot diff、metadata 和 cache refresh，拆边界时容易只移动代码而没有形成可复用合同。
- `wiki-runtime/src/generation/managed_sections.rs` 当前 marker 只有 managed/user，缺少明确 owner kind，可能阻塞 `declared_managed / derived_managed / projection_static / manual_unmanaged` 的机器判定。
- `wiki-knowledge/src/compose.rs` 已经生成接近 Markdown 的 section body、citation 和 diagram draft，设计阶段需要判断哪些属于 projection intent，哪些属于 runtime render responsibility。
- `wiki-runtime/src/storage/knowledge_artifacts.rs` 同时承担 artifact persistence、restore、projection digest binding 和 health/conflict 派生，需要避免本 change 顺手吞并 governance 或 query readiness。
- 不考虑历史兼容后，删除旧 fallback 会影响依赖旧测试 fixture 的断言，需要同步收紧测试。

## 未知项

- `PageDraft / PageDigest / ProjectionDigestStatus` 最终归属是保留在 `wiki-knowledge`，还是移动到 `wiki-model` / runtime-facing DTO，需要在 design 阶段确定。
- 是否引入显式 `SectionOwnership` / `SectionPlan` 类型，还是先扩展现有 `PlannedPage`、`PlannedSection` 和 managed marker 协议，需要在 design 阶段确定。
- `sync` 中 declared marker parse 与 lifecycle validation 的具体拆分粒度需要在 design 阶段确定。
- Snapshot commit 的失败恢复是否需要新增更细的 projection write manifest，留到 design 阶段评估。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
