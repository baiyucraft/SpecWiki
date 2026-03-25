## 1. 工作区与 crate 骨架

- [x] 1.1 更新根级 `Cargo.toml`、各 crate manifest 与 `crates/*/package.json`，把工作区正式拆成 `wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime` 四个 crate，并直接完成 `wiki-core -> wiki-runtime` 的 crate/bin/package rename，不保留兼容 alias。
- [x] 1.2 收紧 crate 依赖方向与公开模块面，确保 `wiki-model <- wiki-index <- wiki-knowledge <- wiki-runtime` 成立，且 `wiki-index`、`wiki-knowledge` 不反向依赖 `wiki-runtime`。
- [x] 1.3 补工作区级边界 smoke 或等价检查，验证 `wiki-runtime` 不重新导出 `wiki-index` 或 `wiki-knowledge` 的内部实现模块。

## 2. 对象归属与模块迁移

- [x] 2.1 把 `KnowledgeDomain / KnowledgeUnit / KnowledgeTree / UnitScope / SourceCitation / ModuleNode / ModuleTree / WikiRelation / WikiItem / WikiState / WikiMetadata / DirtyState` 中属于稳定共享模型的正式 DTO 迁入 `wiki-model`，并把 builder、assembly、metadata export 等 helper 留在 `wiki-runtime`。
- [x] 2.2 抽出 `wiki-index` 的 facts/index 主实现，迁移 scanner、module tree、symbol graph、graph analysis 等能力，并定义 `FactsAssist` 与 index store/query trait，切断 runtime 对 facts 主实现的直接拥有。
- [x] 2.3 抽出 `wiki-knowledge` 的 planning/research/compose 主实现，迁移 `knowledge_planner / research_engine / compose_engine` 及其合同对象，明确 `PlannedPage` 只作为 knowledge-side projection decision，不得充当 runtime 真相对象。
- [x] 2.4 把 `PageContext / PipelineRuntimeSummary / UnitRuntimeGate / ChangePlan / ExportContext / ManagedSectionBlock / UserSectionBlock / PageMergePlan` 等执行态对象收回 `wiki-runtime`，删除旧 `wiki-core` 中平行入口、长期 `pub use` 或过渡转发。

## 3. Storage 与 LLM 边界

- [x] 3.1 将 `sqlite_store` 按 `index_store / knowledge_store / runtime_store` 三段拆分，明确 `modules / symbols / edges`、`research_cache / page_digests / page_drafts`、`wiki_pages / wiki_page_sections / wiki_relations / runtime_meta / pipeline_checkpoint / unit_runtime_gates / llm_cache` 的 schema owner、API owner 与 truth kind。
- [x] 3.2 让 `wiki-index` 与 `wiki-knowledge` 分别定义自己的 store trait，由 `wiki-runtime::storage::sqlite::*` 实现；同时收紧读路径，避免 facts/index、research/compose、runtime projection 跨层直接读库。
- [x] 3.3 把当前 `llm` 横切逻辑拆成 `wiki-index::assist::FactsAssist` 与 `wiki-knowledge::research::ResearchProvider` 两组合同，由 `wiki-runtime` 持有 provider 配置、prompt/cache、JSON IPC bridge 与 concrete adapter，本轮不新增 provider-backed compose 能力。

## 4. Runtime 编排与宿主/分发迁移

- [x] 4.1 重构 `init / update / rebuild` 主链，让 `wiki-runtime` 只编排 `wiki-index` 与 `wiki-knowledge` 的公开合同完成 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，不再在 runtime 内保留 scanner、research 或 compose 的平行主实现。
- [x] 4.2 保留 `renderer / managed_sections / page merge / managed marker / Markdown projection / lifecycle / transport` 在 `wiki-runtime`，并把 `sections` 中的 knowledge planning 对象与 runtime drafting/merge 逻辑按职责拆开。
- [x] 4.3 完整迁移 CodeBuddy Agent、distribution/staging、根脚本、fixture、测试脚本和发布入口到 `wiki-runtime` 命名，确保 Agent 仍是 thin boundary，平台包复制目标与 `optionalDependencies` 指向新的 runtime binary。
- [x] 4.4 全面清理仓库中的 `wiki-core` 残留引用、错误消息、路径常量、测试说明和生成产物入口，不保留“后续再改”的旧命名死角。

## 5. 测试、样本与分析报告

- [x] 5.1 运行 Rust 多 crate/workspace build/test 与相关脚本测试，覆盖 crate 边界、storage/LLM 合同、Agent IPC、distribution/staging 与 streaming 协议，确认每类行为变化都有对应测试。
- [x] 5.2 本轮按用户指令跳过 `storybook + dagger` 专项守门验证，不检查 `tmp/test/*/.wiki/` 与 reference 差异，也不把该专项作为本次 iteration 10 的完成条件。
- [x] 5.3 运行 `node scripts/test-wiki-lifecycle.mjs` 覆盖 `init -> status -> sync -> query -> update -> rebuild` 全链路，确认新的四 crate 边界下 JSON 响应、marker 覆盖率与状态流转不回退。

## 6. 注释与收尾检查

- [x] 6.1 单独执行一轮 [COMMENTING.md](E:/project/!byAI/spec-wiki/COMMENTING.md) 合规检查，确认本轮涉及的 Rust/TS 代码、测试与脚本注释都符合仓库规范。
- [x] 6.2 运行 `openspec status --change iteration-10-four-package-boundaries-and-foundation-smoke` 与等价校验命令，确认 proposal、design、specs、tasks 一致且 change 已 ready for implementation。



