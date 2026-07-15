# close-specwiki-3-0-design-baseline-runtime-query-contract

## 问题

Repo Wiki 3.0 已经拥有 route groups、逐结果 provenance、分层 readiness、query trust 和 recommended action 等实现基础，但 Runtime、CLI、Agents 与长期设计对“哪些字段是稳定主合同、哪些只是兼容或派生视图”仍没有唯一结论。当前 Rust transport 仍并列暴露 `matched_pages`、`provenance_summary`、`route_groups` 和 `results`，宿主资产继续把前两个派生字段列为稳定消费入口，而产品基线要求宿主只消费 Runtime authority，不能自行重建 query 语义。

同时，外部输入仍是 term-only，代码内部已经具有 symbol、path、graph、process、community 等查询 substrate，但 intent-aware input 以及稳定的 owner / entrypoint / impact 输出尚未形成可审计合同。若不先收口 Runtime / Query authority，后续核心场景验收和宿主触发会继续固化不同字段、排序、可信度与降级解释。

本 change 要解决的真实问题是：为 Repo Wiki 3.0 建立一份跨 Runtime、CLI 和 Agents 一致的 query 消费合同，明确当前稳定输入输出、route 与排序边界、来源与质量解释、错误和降级语义，并对 richer query 能力作出当前纳入或明确延期的判定。

## 目标

- 建立唯一可引用的 Runtime / Query 合同，逐项定义稳定 input、output、route group、result、readiness、trust、recommended action、错误和 degraded 语义。
- 统一 Runtime、CLI 与 Agents 对主合同字段、派生字段和兼容字段的分类，停止宿主依赖 matched-pages-only 或 provenance-summary-only 解释。
- 明确 term-only 与 richer query 的关系，判断 intent-aware input 是否进入当前 architecture baseline；未进入的能力必须有清晰延期边界。
- 明确不同 route 内与跨 route 的 ranking、截断和 score 可比性，并要求每类结果可解释来源、置信度、质量状态和验证引用。
- 明确 index、knowledge、governance、projection 与 rendered page fallback 的可用条件；不可用或过时时不得伪装为可信命中。
- 对 owner / entrypoint / impact、process / community 和更完整 graph projection 作出当前合同或延期判定，为后续核心场景与宿主触发提供规范输入。

## 非目标

- 不在 proposal 阶段决定具体 DTO 类型归属、字段结构、算法或 transport 实现；这些留到 `design.md`。
- 不把 semantic search、外部 embedding service 或 LLM reranking 设为当前主链前提。
- 不实现新的 code graph ingestion、process/community extraction、owner 推断或 impact analysis 算法。
- 不设计 Codex、Claude、CodeBuddy 的 trigger taxonomy、capability matrix 或测试语料；这些属于 `close-specwiki-3-0-design-baseline-host-trigger-contract`。
- 不完成 9 个核心场景的 command/API、fixture 与 gate 映射；这些属于 `close-specwiki-3-0-design-baseline-core-scenario-acceptance`。
- 不执行全库旧字段、状态标签或 authority 指针迁移；最终清理由 `close-specwiki-3-0-design-baseline-documentation-closure` 承担。
- 不改写历史 archive，不把历史报告提升为当前 authority。

## 成功标准

- 存在唯一 Runtime / Query authority，Runtime、CLI 与 Agents 对稳定输入和输出字段逐项一致，并明确主合同、派生视图和延期字段。
- term-only 与 richer query 的关系有明确结论；若 intent、focus、scope 或 traversal 未进入当前合同，则宿主不得通过 description 或私有字段模拟这些输入。
- route tag、result 最小语义、route grouping、ranking、score basis、截断和去重规则可被系统测试验证；不可比较的 route score 不被强行解释为全局排序。
- 每类稳定结果都能表达 provenance、confidence、recommended action 和 source refs，并能说明命中来自 facts、knowledge、governance、projection 还是 debug fallback。
- readiness、query trust、answer mode 与 recommended action 的关系可验证；index missing/stale/blocked、knowledge/projection 降级和 governance blocker 不会产生跨层误判。
- rendered page fallback 被明确标记并降低 trust，不能伪装为 index 或 knowledge 正向命中。
- 错误、空结果、部分可用、stale、blocked、conflict 和恢复建议具有跨 Rust transport、TS parser、CLI 与 Agents 一致的消费语义。
- owner / entrypoint / impact、process / community 和更完整 graph projection 均被明确归类为当前稳定合同或延期能力，并记录进入稳定合同所需证据。
- 后续 `core-scenario-acceptance` 与 `host-trigger-contract` 能引用本合同，不需要重新定义 query DTO、ranking、readiness 或 trust 语义。

## 影响范围

### 包含内容

- `wiki-model` 的公开 query 对象语言与 route/result DTO 边界。
- `wiki-runtime` query workflow、runtime profile、transport payload、readiness/trust、fallback、错误与恢复语义。
- `packages/spec-wiki` 的 runtime parser、query 类型、人类/机器输出消费边界。
- Agents 共享 query 资产对稳定字段、命中解释和进一步代码核验的约束。
- `.wiki/06-设计文档/01-Runtime设计.md` 与 `02-Agents设计.md` 中当前互相漂移的 Runtime / Query 叙事。
- 与稳定合同直接相关的 Rust、TypeScript、CLI、transport 和宿主资产测试。

### 不包含内容

- 宿主触发词、should-trigger / should-not-trigger / ambiguous 语料与跨宿主决策比较。
- 核心场景、扩展场景、reliability lifecycle 和全库 documentation closure 的完整收口。
- 与合同收口无关的 query 性能优化或新检索后端。

### 业务规则

- `05-产品基线与设计治理.md` 提供产品级 authority、scope 和证据规则；本 change 只在 Runtime / Query scope 内建立专题 authority。
- Runtime 是 query 语义和状态判断的唯一规则所有者；CLI 与 Agents 只解析和呈现稳定 DTO，不复制 ranking、readiness、trust 或恢复决策。
- 历史 archive 仅作为只读证据引用，不自动成为当前合同。
- 当前阶段不要求保留旧合同兼容层；确认主合同后可以删除旧字段、旧解释和旧 fallback。

## 交付形态

single-change

这是 parent `close-specwiki-3-0-design-baseline` 下顺序第 2 个 child，依赖已归档的 `close-specwiki-3-0-design-baseline-product-contract`。它为 `core-scenario-acceptance`、`reliability-lifecycle` 和 `host-trigger-contract` 提供稳定 Runtime / Query 规范输入。

## 风险

- 当前 Wiki 叙事、Rust DTO、TS parser 和宿主生成资产并不完全一致，设计阶段若只选择其中一层会把局部实现误当成全局 authority。
- `matched_pages`、`provenance_summary`、`summary/hits` 与 `route_groups/results` 并列存在；若不明确主从关系，会长期形成双轨合同。
- richer query 范围若一次性包含 intent、owner、entrypoint、impact、process 和 community，可能把合同收口扩大成新的检索实现项目。
- readiness、query trust、answer mode、governance readiness 和 recommended action 属于不同语义层；错误合并会让治理 blocker 降低普通代码查询可信度，或让部分可用结果被误报为 ready。
- ranking 和 score 在不同 route 间可能没有统一量纲；强制全局排序会制造无法解释的可信度。

## 未知项

- 当前主合同是否应只保留 `route_groups/results`，还是保留 `matched_pages/provenance_summary/summary/hits` 作为明确标注的派生视图。
- term-only 是否继续作为 architecture baseline 的唯一外部输入；若引入 richer input，最小稳定 intent / focus / scope / traversal 集合是什么。
- owner / entrypoint / impact 中哪些已有足够事实与验证证据，可在本次设计中提升为稳定输出。
- process / community 命中应进入稳定 route tag，还是继续作为 index 内部 substrate。
- 跨 route 的排序、截断、去重与 score basis 应如何表达，才能避免宿主进行未授权的全局比较。
- 空结果、fallback、stale、blocked、conflict 和 governance-only 命中的最终协议矩阵与人类输出责任。

## 参考资料

- `../close-specwiki-3-0-design-baseline/split.md`：来源为当前 parent；目标落点是本 child 的目标、验收边界、顺序和依赖；采用方式为直接约束。
- `../close-specwiki-3-0-design-baseline/research/design-debt-and-delivery-boundary.md`：来源为 parent explore 调研；目标落点是 Runtime 与 Agents 漂移证据及 richer query 未知项；采用方式为改写提炼。
- `../../../.wiki/06-设计文档/05-产品基线与设计治理.md`：来源为已归档 product-contract 的 canonical 页面；目标落点是 authority、scope、正交证据和后续 child 依赖；采用方式为直接约束。
- `../../../.wiki/06-设计文档/01-Runtime设计.md`、`../../../.wiki/06-设计文档/02-Agents设计.md`：来源为当前专题设计；目标落点是 term-only、route、readiness、fallback、宿主稳定字段和延期能力的现状核对；采用方式为冲突识别与改写输入，不直接复制。
- `../../../crates/wiki-model/src/domain/query.rs`、`../../../crates/wiki-runtime/src/transport/query_payload.rs`、`../../../packages/spec-wiki/src/runtime/parseResult.ts`、`../../../packages/spec-wiki/src/agents/shared/commandAssets.ts`：来源为当前代码事实；目标落点是现有 DTO、transport、parser 与宿主消费差异；采用方式为事实核对。
- `../../archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/`：来源为已归档 query route/readiness change；目标落点是既有 route/result/readiness 证据与尚未解决的跨层 authority 问题；采用方式为只读证据引用，不作为当前 authority。
- 本 change 未使用 upstream；不存在直接迁移、改写或仅借鉴的外部实现。
