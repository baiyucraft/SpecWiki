# refactor-specwiki-around-contract-closure-truth-restore-snapshot

## 问题

新版 SpecWiki 已经收口正式页面树，但 runtime 的 truth / restore / readiness 语义仍然没有完全闭合。当前代码已经存在 formal artifacts、cache restore、manifest hash 校验和 `query_readiness` 等能力，但这些能力仍容易把不同层级混成一个状态：

- `.wiki/.cache/**` 缺失后，Level 1 restore 重建出的本地 runtime 可能被误读为完整 graph ready。
- `facts_ready`、`query_readiness` 和 `state` 仍不足以稳定区分 index graph、knowledge、projection 与 diagnostic mode。
- `wiki.metadata.json` 与 `.wiki/.knowledge/runtime/**` 的职责边界需要继续收紧，避免 metadata 继续承载 projection digest、runtime gates 或 committed snapshot 主状态。
- `recovery-manifest.json` 已能表达部分恢复锚点，但还没有形成 committed snapshot manifest 的唯一主记录语义。

本 change 要解决的真实问题是：把 “restore 不是 rebuild、knowledge ready 不是 graph ready、metadata 不是 runtime state 主存储” 写成可验收合同，避免 status/query 宿主继续把恢复态误判为完整 ready。

## 目标

- 固定 truth kind：source、index graph、knowledge artifacts、projection pages、metadata 和 cache 各自承担明确职责。
- 固定两级 restore：
  - Level 1：从 `.wiki/.knowledge/** + official page tree + wiki.metadata.json` 恢复 knowledge / projection / runtime diagnostic，可诊断、可给 next action，但不能声称 graph ready。
  - Level 2：从当前源码重建 SQLite graph，完成后才允许 index graph ready。
- 固定 committed snapshot manifest：正式 snapshot 主记录位于 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml` 或等价 runtime artifact；`wiki.metadata.json` 只保存 current snapshot pointer 与 binding index。
- 拆分 readiness：至少稳定表达 `index_readiness`、`knowledge_readiness` 和 `projection_readiness`，不再用单一 `facts_ready + query_readiness` 掩盖层级状态。
- 更新现有测试与状态断言，使 `.cache` 缺失、manifest stale、metadata binding mismatch 和 graph missing 的结果可被机器稳定区分。

## 非目标

- 不做 query route tags / result DTO 全量重构；该范围留给 `refactor-specwiki-around-contract-closure-query-route-readiness`。
- 不做 PagePlan / SectionPlan / declared writeback；该范围留给 `refactor-specwiki-around-contract-closure-projection-writeback-boundaries`。
- 不做 wiki-index graph schema、phase DAG、raw imports / calls / heritage；该范围留给 `refactor-specwiki-around-contract-closure-code-graph-index`。
- 不做 CLI help、landing state、安全模式或首次使用文案；该范围留给 `refactor-specwiki-around-contract-closure-cli-product-surface`。
- 不做 `.spec` governance isolation 或 archive operation manifest。
- 不处理旧 `.wiki/pages/**`；旧目录不迁移、不清理、不诊断、不查询。
- 不保留旧 manifest 兼容双轨；当前阶段不考虑历史兼容性。

## 成功标准

- 删除 `.wiki/.cache/**` 后，Level 1 restore 不得把 `index_readiness` 标记为 `ready`，除非 Level 2 graph rebuild 已实际完成。
- Level 1 restore 成功时，`status` 能表达 knowledge / projection 可诊断或可消费，同时 index graph 可以保持 `missing`、`stale` 或等价非 ready 状态。
- committed snapshot manifest、metadata binding、page hashes 或 formal artifacts 锚点不一致时，restore 必须进入 diagnostic / blocked / needs_update 等显式状态，不得静默 full init 或把结果标记为 restore 成功。
- `wiki.metadata.json` 不继续保存 projection digest、runtime gates 或 committed snapshot 主状态；这些主状态必须归属 `.wiki/.knowledge/runtime/**`。
- `query` 在 index graph 不 ready 时不能伪装 graph 命中；若允许消费 Level 1 恢复态结果，必须通过 readiness / recommended action 告知调用方当前处于恢复态或待 rebuild / update。
- 现有测试中把 cache restore 直接等同于 `facts_ready == true` 或完整 `ready` 的断言必须被调整为分层 readiness 断言。
- `unispec validate refactor-specwiki-around-contract-closure-truth-restore-snapshot` 通过。

## 影响范围

- `crates/wiki-runtime/src/workflows/status.rs`：状态投影、restore 失败显示、readiness 字段。
- `crates/wiki-runtime/src/workflows/query.rs`：cache missing / index not ready / restored runtime 的 query readiness 投影。
- `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`：formal artifacts、recovery manifest、snapshot manifest 与 restore guard。
- `crates/wiki-runtime/src/storage/state_store.rs` 与 SQLite 相关存储：区分 Level 1 mirror / diagnostic state 与 Level 2 graph readiness。
- `crates/wiki-runtime/src/domain/*`：可能需要新增或收紧 runtime readiness DTO。
- `crates/wiki-runtime/tests/runtime/status_and_update.rs`、`knowledge_artifacts_roundtrip.rs`、query / rebuild 相关测试：调整 cache missing、manifest stale、metadata mismatch、graph missing 的断言。
- `.wiki/05-规格基线/capabilities/**` 与 `.wiki/06-设计文档/01-Runtime设计.md`：归档时如发现实现取舍改变长期合同，需要同步沉淀。

## 交付形态

single-change

这是 parent `refactor-specwiki-around-contract-closure` 下的第 2 个 child change，依赖已归档的 `refactor-specwiki-around-contract-closure-page-tree-contract`。本 change 只闭合 truth / restore / snapshot / readiness 合同，不扩大到 query DTO、projection ownership、code graph index、CLI product surface 或 governance isolation。

## 风险

- `facts_ready` 命名和现有调用点可能已经混合 “cache restored” 与 “graph ready” 两种语义，设计阶段需要决定是拆字段、改含义，还是新增更明确的 readiness DTO。
- `restore_runtime_cache_from_artifacts` 当前会重建部分 SQLite 状态；设计阶段必须明确哪些重建结果属于 Level 1 mirror / diagnostic，哪些必须由 Level 2 graph rebuild 才能标记 ready。
- `recovery-manifest.json` 到 committed snapshot manifest 的替换可能触发较多测试与 fixture 调整；本阶段不保留历史兼容路径。
- status/query 的 machine-readable 输出调整可能影响宿主消费，需要在 system-tests 中覆盖关键字段。

## 未知项

- committed snapshot manifest 的文件名、格式和 schema version 是否直接采用 `manifest.yaml`，或继续使用 JSON 等价 artifact，需要在 design 阶段确定。
- Level 1 restore 后 query 是否允许返回 knowledge / projection 命中，还是只允许 diagnostic，需要在 design 阶段结合现有 query 能力和后续 `query-route-readiness` 边界确定。
- `index_readiness` 与现有 `facts_ready` 是否同时保留一个过渡期不考虑兼容，但实现上仍需选择最小改动路径。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.wiki/06-设计文档/01-Runtime设计.md`
- `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md`
- `.wiki/05-规格基线/capabilities/repo-wiki-runtime/spec.md`
