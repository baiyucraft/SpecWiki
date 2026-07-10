# refactor-specwiki-around-contract-closure-governance-isolation

## 问题

当前 SpecWiki 已完成 query route/readiness 与 code graph/index 合同收口，但 governance 仍停留在分散规则和占位状态：

- `.spec/changes/**` 与 `.spec/archive/**` 是治理 evidence truth，runtime 尚无独立 `GovernanceEvidenceStore` 或等价读取边界。
- query 公开 DTO 已预留 governance route tag，但 runtime 只返回 `governance_readiness: not_enabled`，无法表达 enabled、stale、blocked 或 conflict。
- 当前治理 stage、required artifact、review/verification gate 和 archive readiness 主要由现有治理 CLI/Skill 解释；如果 SpecWiki runtime 另写一套规则，容易形成双 validator 漂移。
- `.spec` 已被 code scanner 和 graph persistence 排除，因此源码 `ChangeSet` 无法感知治理 artifact 变化；现有 update 在源码无变化时会直接 no-op。
- CLI、archive 和后续 host assets 都需要治理共享合同，但当前 parent 把 CLI 与 governance 视为可并行实现，导致生产端、transport 和消费端所有权不清。
- `.wiki` 与 CLI 文档已经描述部分治理命令，但实际 runtime action、transport DTO 和 TS parser 尚未提供对应能力，形成文档领先于实现。

本 change 要解决的真实问题是：建立与 code facts 隔离、以 `.spec` 为 evidence truth、可被 status/query/validate 消费的只读治理内核，为后续 CLI 产品面和 archive 写事务提供唯一共享合同。

## 目标

- 在 `wiki-model` 定义稳定的 governance product-level DTO，至少覆盖 readiness、change summary、artifact reference、blocking issue、review/verification gate 和 validate result。
- 明确 governance readiness 属于产品级组合状态，不塞入 core `RuntimeReadiness` 的 index/knowledge/projection/fusion 内部状态。
- 在 `wiki-runtime` 建立 `GovernanceEvidenceStore` 或等价端口，独立读取 `.spec/changes/**`、`.spec/archive/**`、`meta.yaml` 和 required artifacts。
- 以 `.spec` 文件系统内容为正式 evidence truth；SQLite 或其它缓存只能保存可重建的 artifact refs、fingerprint 和 derived status。
- 实现只读 governance discovery/status/validate，覆盖 standalone、parent、child、active、archived、missing artifact、report frontmatter 和 parent/child relation。
- 定义 validator parity：同一 fixture 的 runtime validate 与当前治理规则必须得到一致的 blocking/pass 结论。
- 为 `.spec` 建立独立 fingerprint/change detection；治理变化只刷新 governance index/summary，不进入 source `ChangeSet`，不触发 code graph 重建。
- 将 governance readiness 和 refs 接入 status/query/update 的产品级组合输出；governance blocked 只阻断治理动作，不阻断普通 Wiki query/update 的可用部分。
- 为 query 产出 `governance_evidence_ref / governance_summary_hit` 所需的结构化 refs，但不把 `.spec` 原文复制进 `.wiki` 或 code graph FTS。
- 同步 Rust transport DTO 与 TS parser 的 governance readiness/ref 闭集，TS 层只校验和透传，不重写治理规则。

## 非目标

- 不执行 archive 目录移动、parent 状态写回、dry-run manifest、confirm、重试或恢复；这些属于 `refactor-specwiki-around-contract-closure-archive-dry-run-manifest`。
- 不重排默认 CLI help，不实现顶层 `changes / change / validate / archive` command router，不统一 init；这些属于 `refactor-specwiki-around-contract-closure-cli-product-surface`。
- 不把 `.spec` 放入 `files / symbols / edges / raw_* / files_fts / symbols_fts`，也不复用业务 `scan_repo`。
- 不把 SQLite、query cache 或 derived summary 变成 governance evidence truth。
- 不把 proposal、design、review-report 或 test-report 原文复制到 Wiki 页面或 declared knowledge。
- 不保留旧 validator 兼容层；在 parity 通过后，后续 change 可以删除重复状态机。
- 不实现 release gate、quality gate 或 capability baseline 的完整产品化语义；本 change 只提供可扩展 DTO/refs 和最小 derived summary 边界。

## 成功标准

- 无 `.spec` 的仓库返回 `governance_readiness: not_enabled`，且普通 status/query/update 继续可用。
- 有合法 `.spec` 的仓库能列出 active/archive changes、stage、parent/child 关系、artifact refs 和治理 readiness。
- missing required artifact、meta id 不一致、非法 parent/child relation、review/test report frontmatter 缺失或非 full/pass 能生成稳定 blocking issue。
- runtime validate 与当前治理规则在 fixture 上保持 parity；禁止在 Skill、TS CLI 和 runtime 中维护三套 required artifact matrix。
- `.spec` 修改会改变 governance fingerprint 并刷新 governance derived state，但不会改变 code graph snapshot、source fingerprint 或 source dirty set。
- governance blocked/conflict 只影响治理结果和相关 next action，不把 core Wiki query/update 强制降为全局 blocked。
- ready 时 query 可返回 `governance_evidence_ref / governance_summary_hit`，每条结果携带 change/artifact refs、provenance 和 recommended action。
- `.spec` 原始证据仍保留在原目录，不进入 `.wiki` 正文、code facts 或业务 FTS。
- Rust DTO、runtime transport、TS parser 和自动化测试对 governance readiness/ref 闭集保持一致。
- 本 change 不产生任何 archive move、副作用 manifest 或 parent 写操作。

## 影响范围

- `crates/wiki-model`：governance readiness、change/artifact refs、blocking issue、validate/review gate DTO。
- `crates/wiki-runtime`：EvidenceStore、policy/validator、governance fingerprint、status/query/update composition、transport DTO 和 fixtures。
- `crates/wiki-index`：可选的 governance artifact reference 查询端口/缓存适配器；不得修改 code facts 输入口径。
- `crates/wiki-knowledge`：最小 snapshot-scoped governance derived summary 边界；不接管 evidence truth。
- `packages/spec-wiki/src/runtime/parseResult.ts`：治理 DTO 闭集校验与透传，不实现规则。
- `.wiki` 与 `.docs`：只在实现稳定后同步实际合同，删除提前宣称或明确标为目标态。
- `.agents` / skills：本 change 只定义 parity 与后续 forwarding 边界，不在本 child 重写全部宿主命令面。

## 交付形态

single-change

这是 parent `refactor-specwiki-around-contract-closure` 下顺序第 6 个 child。它依赖已归档的 query-route-readiness 与 code-graph-index，先交付治理共享合同和只读 runtime；后续 CLI child 消费这些合同，archive child 再增加写事务。

## 风险

- 当前目标横跨 model/runtime/index/knowledge/TS，若 design 不按纵向验收收窄，可能退化为多个半成品。design 必须固定最小 DTO、只读 action 和 fixture 矩阵。
- 当前治理规则存在于 CLI、Skill 和文档中，parity 基线若没有唯一来源，runtime validator 会再次漂移。
- artifact reference index 与 EvidenceStore 容易职责重叠；design 必须明确 evidence read、derived cache、query lookup 的所有权。
- governance readiness 若并入 core `RuntimeReadiness`，会污染 index/knowledge/projection 的既有语义；必须保持产品级 composition。
- `.spec` 已从 source scanner 排除，若治理 detector 偷用 source `ChangeSet`，将出现治理变化无法刷新或反向污染 code facts 的错误。
- TS parser 当前只接受 `not_enabled`，新增状态若未同步闭集和 fixtures，会破坏 transport 兼容。
- Wiki/CLI 文档已有提前宣称，若不区分现状与目标，会让测试以错误文档为真相源。

## 未知项

- `GovernanceReadiness` 的精确状态闭集，以及 enabled-but-empty、stale、blocked、conflict 的区分方式。
- required artifact matrix 的唯一事实来源：复用当前治理规则数据、抽成共享配置，或在 Rust 中定义后由其它层生成。
- `GovernanceEvidenceStore` 与 artifact reference index 的 trait 边界、缓存位置和 fingerprint 粒度。
- governance derived summary 是否落入 `.wiki/.knowledge/derived/**`，还是第一步只存在于可重建 runtime cache。
- status/query/update transport 是在现有 DTO 增加 product-level governance 字段，还是新增统一 response envelope。
- validator parity fixture 如何覆盖 parent、child、standalone、exploration stub、verification、archived 和损坏 YAML。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`：来源为当前 parent program；目标落点是本 child 的顺序、依赖与验收边界；采用方式为直接约束。
- `.docs/design/governance-runtime-integration.md`：来源为治理融合阶段性设计；目标落点是 EvidenceStore、policy engine、read-only status/validate、query refs 与 archive 后置；采用方式为改写。
- `.docs/design/specwiki-contract-closure.md`：来源为新版 runtime 合同；目标落点是治理状态不阻断 core Wiki、共享 DTO 和 child 边界；采用方式为直接约束。
- `.spec/archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/`：来源为已归档 query contract；目标落点是 governance route tag、product-level readiness 和 result DTO；采用方式为扩展既有合同。
- `.spec/archive/2026-07-01-refactor-specwiki-around-contract-closure-code-graph-index/`：来源为已归档 code graph contract；目标落点是 `.spec` 与 code facts 隔离、独立治理索引边界；采用方式为扩展既有隔离规则。
- `.upstream/codegraph`：来源为本地 code graph/reference index 实现；目标落点仅限 artifact reference lookup、缓存可重建性与 query consumption；采用方式为仅借鉴，不直接迁移源码或 schema。
- `.upstream/GitNexus`：来源为 ingestion DAG 与同源 backend 设计；目标落点是 governance discovery/fingerprint/status 的阶段化编排；采用方式为仅借鉴，不采用其公开 CLI 规模。
- 当前治理 CLI/Skill 规则：来源为项目现有 stage/artifact/review/archive 行为；目标落点是 validator parity fixtures；采用方式为改写为共享 runtime contract，parity 通过前不删除原规则。
