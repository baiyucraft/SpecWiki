# refactor-specwiki-around-contract-closure-code-graph-index

## 问题

当前 `wiki-index` 已有 symbol parsing、symbol graph、SQLite index store 和 query adapter 的雏形，但它还没有完全满足新版 SpecWiki contract closure 对 code graph/index 的要求：

- raw imports、raw calls、raw heritage 已能在解析结果中出现，但缺少稳定持久化与可诊断读取路径。
- `SymbolNode` 仍偏薄，缺少足够表达长期 symbol identity、ownership、range、signature、docstring、visibility 和 provenance 的公开合同。
- SQLite graph schema 已有 `symbols`、`edges`、analysis tables 和 `symbols_fts`，但还没有覆盖 raw captures、unresolved refs、files/folders/path FTS 等 graph substrate 所需能力。
- init、update、rebuild、query 的 core runtime/index 读取路径还没有用统一 readiness、phase diagnostics 和 wiki-index DTO 闭合。
- `.spec` governance artifacts 已被 scanner 排除在普通源码扫描外，但本 child 需要把排除口径固定到 code graph facts 的所有落点，避免后续 governance isolation 被污染。

本 change 要解决的真实问题是：把现有 graph/index 雏形收口为本地可重建 code graph authority，让后续 knowledge、projection、query 和 governance child 能依赖稳定、可诊断、可查询的 index substrate。

## 目标

- 固定 `wiki-index` graph substrate 的最小合同，覆盖 files/folders、symbols、edges、raw captures、unresolved refs、routes/tools、communities/processes 和 FTS 的职责边界。
- 让 raw imports、raw calls、raw heritage 不再只停留在内存解析结果中，而是进入可重建、可查询、可诊断的 graph authority。
- 扩展 `SymbolNode` 合同，保留 `symbol_id / file_id / language / symbol_kind / name` 等最小身份字段，并补齐 qualified name、signature、docstring、visibility、owner symbol、range 和 provenance 等能力。
- 收口 phase DAG 的公开阶段边界，使 scan、structure、parse、resolve imports、resolve calls、resolve heritage、analyze communities、analyze processes、build FTS 的输入输出可诊断。
- 让 GraphStore / IndexQueryStore 和 index query adapter 只通过 `wiki-index` traits/DTO 暴露 graph 能力，避免 runtime、knowledge、host 或 skill 跨层解释 graph 私有表。
- 接上已归档 query-route-readiness 的 route contract，使 index 查询能稳定映射到 `index_symbol_hit`、`index_path_hit` 和 `index_graph_hit`。
- 固定 `.spec` 排除规则：`.spec` 不进入 code graph `files / symbols / edges / raw_* / files_fts / symbols_fts`，治理引用留给后续 governance isolation。

## 非目标

- 不调整 CLI 产品面、默认 help、landing state 或一级命令；这些属于 `refactor-specwiki-around-contract-closure-cli-product-surface`。
- 不实现 governance artifact reference index、governance summary、validate/archive 或治理状态机；这些属于 `refactor-specwiki-around-contract-closure-governance-isolation` 和后续 archive child。
- 不实现完整 compiler-grade type checker、完整 scope-resolution system、完整 receiver-bound call resolution 或跨语言 MRO 质量追平；本 change 只保留后续可演进的 phase 和 provenance 边界。
- 不迁移旧兼容层，不保留旧 fallback；当前阶段按新版主线重构。
- 不直接迁移 upstream CLI、Web UI、Embedding、多仓 registry、服务层或 LadybugDB/KuzuDB 存储模型。
- 不把 process/community 分析质量做成完整产品承诺；本 change 只固定它们在 graph phase、storage 和 query consumption 中的边界。

## 成功标准

- raw imports、raw calls、raw heritage 有稳定持久化与读取验收，不再只存在于 `ParsedFileSymbols` 内存结构。
- `SymbolNode` 具备稳定身份合同，至少覆盖 `symbol_id / file_id / language / symbol_kind / name`，并能表达 qualified name、signature、docstring、visibility、owner symbol、range 和 provenance；同一源码 snapshot 内 symbol identity 必须可重复。
- `.spec` 不进入 code graph `files / symbols / edges / raw_* / files_fts / symbols_fts`，相关 scanner、hierarchy、runtime context 和 query tests 能验证这一点。
- Graph readiness 能区分 `missing / stale / blocked / rebuilding / ready` 的最小判定来源：cache 缺失、关键表缺失、migration 不一致、source snapshot 不一致、重建中和可查询状态不得混淆。
- init、update、rebuild、query 的 core runtime/index 读取路径通过 `wiki-index` traits/DTO 消费 graph，不依赖 CLI 产品面或 markdown/page fallback 来伪造 graph 命中。
- index query adapter 能稳定产出或映射 `index_symbol_hit`、`index_path_hit` 和 `index_graph_hit`；index 不 ready 时不得返回 graph 伪命中。
- phase diagnostics 能追踪到 file、parser、raw capture 或 resolver；parse / resolve fail-soft 不得静默吞掉问题。
- 后续测试计划至少覆盖 schema/read API、raw persistence、`.spec` exclusion、readiness degrade、query route mapping 和 core workflow 接入。

## 影响范围

- `crates/wiki-index` 的 symbol models、symbol graph pipeline、resolver、query adapter、store traits 和相关单元测试。
- `crates/wiki-runtime` 的 SQLite index store、schema/migration 初始化、state store、init/update/rebuild/query workflow 与 runtime tests。
- `crates/wiki-model` 中可能需要承载的稳定 DTO、ID、range、source ref 或 query ref 类型。
- scanner noise filter、hierarchy noise filter、runtime context 过滤、symbol parsing / graph analysis / sqlite storage / command contract tests。
- `.docs/design/specwiki-code-graph-index-design.md` 和 `.docs/design/specwiki-contract-closure.md` 对本 child 的上位约束。
- 后续 `governance-isolation`、`cli-product-surface` 和 `archive-dry-run-manifest` 的消费边界。

## 交付形态

single-change

这是 parent change `refactor-specwiki-around-contract-closure` 下的第 5 个 child change。它依赖已归档的 `truth-restore-snapshot` 和 `query-route-readiness`，交付一个可独立验收的 code graph/index substrate 收口，不重新拆分 parent，也不进入 CLI 或 governance 实现。

## 风险

- 该 child 同时触及 schema、raw persistence、SymbolNode、phase DAG、store traits、query adapter 和 readiness，scope 较硬；如果 design 阶段不继续收窄，会变成长期半成品。
- 如果只做表和字段，不接 core runtime/index 读取路径，graph substrate 会成为孤岛，query-route-readiness 不能真实消费 index 结果。
- 如果 `.spec` 只在 scanner 层过滤，而 raw captures、FTS 或 runtime context 仍能读入 `.spec`，后续 governance isolation 会被污染。
- 如果 readiness 只根据“是否有 DB 文件”判断，cache restore、migration mismatch 或 stale source snapshot 会被误报为 graph ready。
- 如果过早追求 upstream 级别的 type/scope resolution、process/community 质量或 context pack 排序，会挤占本 change 的 contract closure 目标。

## 未知项

- 具体 SQLite columns、migration 策略、FTS tokenizer、files/folders 表形态和索引设计留到 design 阶段决定。
- GraphStore / IndexQueryStore 的最终方法签名、事务边界、runtime adapter 结构和 private schema ownership 留到 design 阶段决定。
- raw capture 的精确字段、diagnostics 模型、parser_id / parser_version 编码和 unresolved refs 归一方式留到 design 阶段决定。
- phase DAG 的增量执行、checkpoint 粒度、错误恢复和 rebuild/update 差异留到 design 阶段决定。
- context pack、impact slice、query result 裁剪与 ranking 是否在本 child 做最小版本，留到 design 阶段按 scope 再收紧。

## 参考资料

- `.spec/changes/refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/specwiki-code-graph-index-design.md`
- `.spec/archive/2026-06-16-refactor-specwiki-around-contract-closure-truth-restore-snapshot/proposal.md`
- `.spec/archive/2026-06-18-refactor-specwiki-around-contract-closure-query-route-readiness/proposal.md`
- `.upstream/codegraph/src/db/schema.sql`：来源为 CodeGraph SQLite schema；目标落点是 `wiki-index` graph schema、FTS 和 unresolved refs；采用方式为改写借鉴。
- `.upstream/codegraph/src/extraction/index.ts`：来源为 CodeGraph extraction orchestrator；目标落点是 git-aware scan、incremental sync、parse budget 和 error strategy；采用方式为改写借鉴。
- `.upstream/codegraph/src/resolution/index.ts`：来源为 CodeGraph reference resolver；目标落点是 import/name/framework resolution 与 resolved edge provenance 的边界；采用方式为改写借鉴。
- `.upstream/codegraph/src/context/index.ts`：来源为 CodeGraph context builder；目标落点是 context pack 与 query result shaping 的方向；采用方式为仅借鉴。
- `.upstream/GitNexus/ARCHITECTURE.md`：来源为 GitNexus pipeline architecture；目标落点是 phase DAG、process/community phase 边界和 graph consumption；采用方式为改写借鉴。
- `.upstream/GitNexus/type-resolution-system.md`、`.upstream/GitNexus/type-resolution-roadmap.md`：来源为 GitNexus type resolution 文档；目标落点是后续 receiver-bound call resolution、MRO 和 cross-file binding 的演进参考；采用方式为后续阶段借鉴。
