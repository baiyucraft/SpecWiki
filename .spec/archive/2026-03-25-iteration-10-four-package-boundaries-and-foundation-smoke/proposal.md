## Why

`.wiki/06-设计文档/00-总体设计.md`、`.wiki/06-设计文档/01-Runtime设计.md` 和 `.docs/roadmap/implementation-roadmap.md` 已经把 3.0 的真相改成 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime` 四层，但当前工作区仍只有 [`Cargo.toml`](E:/project/!byAI/spec-wiki/Cargo.toml) 中的单一 `crates/wiki-core` 成员，且 [`lib.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/lib.rs) 继续把 `domain / generation / llm / repo / storage / transport / workflows` 一并导出。再继续在这个单包上追加能力，只会让 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble` 的 3.0 边界长期停留在文档里，后续 `wiki-index`、`wiki-knowledge`、`wiki-runtime` 的职责都会继续互相污染。

本轮之所以必须现在做，是因为当前代码里的跨层耦合已经进入“会反向定义设计”的阶段：[`workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs) 直接串起 facts、knowledge、LLM、storage 与 runtime 写盘；[`workflows/page_render.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 同时负责 knowledge planning、research、compose 与 SQLite checkpoint；[`domain/change_set.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/change_set.rs) 在“domain”层直接依赖 generation、repo 和 storage；[`llm/mod.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs) 又横穿 scanner/hierarchy、research、SQLite cache 与 section/title 推导。与此同时，reference/rdb 与现产物分析已经说明：`storybook` 当前仍存在 runtime missing，`dagger` 仍存在 grounding/skeleton/reuse 缺口；如果不先把 crate 边界拆正，后续任何 fidelity、query 或 knowledge 生命周期优化都会继续建立在错误的工程分层上。

## What Changes

- 新增正式的四包边界：`wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime`，并让 workspace 依赖方向与 `.wiki/06-设计文档/00-总体设计.md` 一致。
- 把当前 `wiki-core` 收缩并重命名为 `wiki-runtime`；旧 `wiki-core` crate/binary/package 不保留兼容层或并行实现。
- 新增一份明确的“对象归属矩阵”，至少覆盖 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree / UnitScope / SourceCitation / PageDraft / PageDigest / SystemResearch / DomainResearch / UnitResearch / PageContext / PlannedPage / WikiState / WikiMetadata / PipelineRuntimeSummary / UnitRuntimeGate / ChangePlan / ExportContext / ManagedSectionBlock`，禁止再以“按目录搬家”代替按边界拆层。
- 把 `knowledge_planner / research_engine / compose_engine` 收回 `wiki-knowledge`，但把 `renderer / managed_sections / page merge / managed marker / Markdown projection` 留在 `wiki-runtime`，不允许把整个 `generation/**` 整包塞进 `wiki-knowledge`。
- 为 LLM 横切层切出两组显式合同：
  - `wiki-index` 的可选 facts/index assist 合同，用于 scanner / hierarchy 的不确定性辅助；
  - `wiki-knowledge` 的 research contract，用于 provider-backed research；
  - `wiki-runtime` 只实现这些合同并拥有 LLM provider、prompt cache 与宿主桥接，不再重新拥有 facts 或 knowledge 主实现。
- 拆分 `sqlite_store` 的 schema 归属与 API 责任：`symbols / edges / graph analysis` 归 `wiki-index` 合同，`research_cache / page_digests / page_drafts` 归 `wiki-knowledge` 合同，`wiki_pages / wiki_page_sections / runtime summary / gates / metadata/cache` 归 `wiki-runtime` 生命周期；`rusqlite` 具体实现仍留在 `wiki-runtime`，但 `wiki-index` 与 `wiki-knowledge` 只能通过 trait/adapter 访问，不能反向依赖 runtime 存储实现。
- 同步完成 rename 迁移面：workspace members、Rust package/bin 名称、`agents/*` 调用入口、根脚本/构建脚本/发布脚本、测试脚本、fixture 路径与 UniSpec 验证命令全部切到新的 `wiki-runtime` 命名。
- 把本轮验收收紧为“拆层不回退”：要求多 crate build/test、基础 `init / query / status / update / rebuild` smoke、生命周期脚本、以及 `storybook + dagger` 作为 runtime/workflow 不退化样本；本轮不重开页面质量专项，也不提前做迭代 13/15 的 runtime finalization 或消费层扩张。
- **BREAKING**：`wiki-core` crate、binary、npm/Rust 包名与相关脚本入口改为 `wiki-runtime`，旧名称不再保留兼容 alias。
- **BREAKING**：runtime 不再允许直接内嵌 facts/index 或 knowledge 主实现；任何跨层访问都必须通过四包依赖方向和显式合同完成。

## Capabilities

### New Capabilities
- `workspace-crate-boundaries`: 定义四包边界、对象归属、依赖方向、禁止反向依赖与 rename 迁移面的正式合同。

### Modified Capabilities
- `repo-wiki-workflow`: workflow 从“单 crate 直接串全链路”收紧为 `wiki-runtime` 编排 `wiki-index + wiki-knowledge` 的正式主链，且本轮只要求拆层后基础 workflow 不回退。
- `repo-wiki-runtime`: runtime 的正式职责收紧为 lifecycle、storage、transport、managed projection、query route 骨架与 adapter，不再拥有 scanner、knowledge planning、research engine、compose engine 的主实现。
- `codebuddy-agent-integration`: CodeBuddy Agent 继续保持 thin boundary，但必须解析并调用本地 `wiki-runtime` binary，而不是旧 `wiki-core` 命名。
- `adapter-distribution`: 平台包 staging、二进制复制目标和主包入口必须跟随 `wiki-runtime` 命名与新的 workspace 布局更新。
- `workflow-verification`: 验证面新增多 crate workspace build/test、crate 边界 smoke、生命周期脚本、`storybook + dagger` 不回退样本，以及 `.wiki/02-开发指南/00-代码注释规范.md` 单独检查要求。

## Impact

- 重点影响 Rust 工作区与 crate 布局：[`Cargo.toml`](E:/project/!byAI/spec-wiki/Cargo.toml)、[`crates/wiki-core/Cargo.toml`](E:/project/!byAI/spec-wiki/crates/wiki-core/Cargo.toml) 及后续新增的 `wiki-model / wiki-index / wiki-knowledge / wiki-runtime`。
- 重点影响当前横切热点：[`crates/wiki-core/src/workflows/init.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/init.rs)、[`crates/wiki-core/src/workflows/page_render.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs)、[`crates/wiki-core/src/domain/change_set.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/change_set.rs)、[`crates/wiki-core/src/llm/mod.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs)、[`crates/wiki-core/src/storage/sqlite_store.rs`](E:/project/!byAI/spec-wiki/crates/wiki-core/src/storage/sqlite_store.rs)。
- 重点影响 Agent 与分发入口：[`agents/codebuddy/src/runtime/invokeCore.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/invokeCore.ts)、[`agents/codebuddy/src/runtime/resolveBinary.ts`](E:/project/!byAI/spec-wiki/agents/codebuddy/src/runtime/resolveBinary.ts)、[`scripts/build-dist.mjs`](E:/project/!byAI/spec-wiki/scripts/build-dist.mjs)、[`scripts/testing/helpers.mjs`](E:/project/!byAI/spec-wiki/scripts/testing/helpers.mjs)、[`scripts/run-test-projects.mjs`](E:/project/!byAI/spec-wiki/scripts/run-test-projects.mjs)、[`scripts/test-wiki-lifecycle.mjs`](E:/project/!byAI/spec-wiki/scripts/test-wiki-lifecycle.mjs)。
- 依赖本轮不新增第五个 `wiki-llm`/`wiki-storage` crate；`rusqlite`、LLM provider 与宿主 bridge 仍由 `wiki-runtime` 持有，但必须通过四包合同被约束。
- `storybook + dagger` 本轮只作为拆层后的 runtime/workflow 守门样本，不允许把 reference 标题、目录结构或现有 rdb 结论反向固化成 crate 边界或样本特判。
