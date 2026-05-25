## Why

`9.4` 已经把主链切到 `Facts -> Knowledge Planning -> Research -> Compose -> Assemble`，但当前真实实现和 reference 结果之间还存在三个结构性断层：一是 `init / rebuild / update` 仍直接绑定 `StructuralResearchProvider`，Research 在 runtime 中还不是真正的 provider-first 主驱动；二是 `page_drafts` 里已有 citation 计数，但最终 `.wiki/*.md` 与 reference 报告脚本都还无法把它识别成正式的 citation / evidence / diagram 落页；三是 `storybook / dagger` 的 KnowledgeUnit 仍大量收敛到 `ModuleDoc + ConceptGuide`，中粒度的 API / config / runtime / testing / tutorial / example 页树还不够稳定。现在继续堆 prompt 或继续加 family/topic 语义，只会放大复杂度，不能收敛到 [DESIGN-CORE2.0.md](E:/project/!byAI/spec-wiki/DESIGN-CORE2.0.md) 要求的 knowledge-unit driven、citation-driven、LLM-required 主线。

## What Changes

- 把 runtime 主链从结构兜底 research 升级为 provider-first research：`init / rebuild / update` 的正式流程必须消费 provider-backed 的 `system / domain / unit` 研究对象，`StructuralResearchProvider` 仅保留给测试和受控开发场景。
- 引入 KnowledgeUnit 中粒度拆分规则，把 docs anchors、public API surface、config surface、runtime graph、testing/example/tutorial signal 统一收敛到通用 decomposition policy，不再让 `storybook / dagger` 的命中率依赖 `family / topic / module` 的历史残留语义。
- 把 section-scoped citation、evidence block 和 diagram draft 正式落到最终 Markdown；reference 对比与 lifecycle 验证必须从 `.wiki/*.md` 读取可统计的 citation / mermaid 结果，而不是只看中间缓存。
- 收编旧的 family-centric planner 心智：family 只作为 KnowledgeUnit decomposition 的一种信号来源，不再作为专门的 planner 主抽象或样本仓库专用分支。
- 当前迭代的实现验证与专项报告只面向 `storybook + dagger` 两个验收样本，不扩展到 19 项目集全量回归。
- 当前迭代的最终验收目标是让 `storybook` 与 `dagger` 对各自 reference 的总体对齐率都达到 `>=95%`，并且这一结果必须来自最终 Markdown 与 reference 报告脚本，而不是中间缓存估算。
- 当前迭代必须按固定收敛路径逼近 `95%`：先建立 `storybook / dagger` 各自的差距台账，再把缺口拆成 `missing pages`、`collapsed pages`、`low-fidelity matched pages` 三类，按“先补缺页、再拆折叠、最后补内容质量”的顺序迭代收敛。
- 当前迭代还必须把 docs-backed 页面命名与章节骨架纳入正式验收：若仓库同时存在英文 raw docs 与本地化/派生 docs 语料，planner 必须优先后者，避免最终 `.wiki/` 大批残留英文 raw docs 文件名；对于 docs-backed 页面，最终 Markdown 必须优先保留或收敛到 reference 式的主章节骨架，而不是退回英文原始 docs 标题或泛化模板。
- 当前迭代还必须把 LLM hard-stop 预算纳入正式 runtime contract：全局调用上限、page research turn 上限等硬限制必须可通过 steering 配置，默认值也必须针对中大仓库提高，避免 `storybook / dagger` 这类样本在主链正确前就被默认预算截断。
- 当前迭代不得通过盲目增页刷高表面对齐率；每轮专项报告都必须同时输出 `extra generated pages`，并要求最终达到 `>=95%` 时，高频 collapse 与 citation / diagram 缺口都不再是主要短板。
- **BREAKING**：删除或降级当前仍服务于结构兜底 / family 过渡期的旧 contract、helper 和验证口径，不保留旧页面拓扑兼容层。

## Capabilities

### New Capabilities
- `knowledge-unit-decomposition`: 定义基于真实代码结构、docs/API/config/runtime/testing/example 信号的通用 KnowledgeUnit 中粒度拆分规则与验收指标。

### Modified Capabilities
- `content-family-planner`: 将 family 从专门页面体系降级为 KnowledgeUnit 规划信号，避免继续围绕 family-index / family-child 心智扩张 core。
- `research-driven-page-composition`: 把 runtime 正式切到 provider-first research object 主链，并要求 compose 只消费研究结果、子页 digest 与显式 citation / diagram plan。
- `page-evidence-layer`: 要求 citation / evidence / diagram 以最终 Markdown 可统计形式落页，而不是只停留在 `page_drafts` 或 context cache 中。
- `wiki-llm-enhancement`: 明确 LLM 是 Research / Compose 的正式前置条件，结构型 provider 仅用于测试与受控降级场景。
- `workflow-verification`: reference 报告与 lifecycle 验证新增对 citation、diagram、page collapse、unit decomposition 命中的正式校验。

## Impact

- 影响 `crates/wiki-core/src/workflows/*`、`crates/wiki-core/src/generation/knowledge_planner.rs`、`crates/wiki-core/src/generation/research_engine.rs`、`crates/wiki-core/src/generation/compose_engine.rs`、`crates/wiki-core/src/generation/renderer.rs`、`crates/wiki-core/src/domain/research.rs` 等 core 主链模块。
- 影响 `scripts/collect-reference-project-reports.mjs`、`scripts/collect-test-project-analysis.mjs` 与 `storybook / dagger` 专项验收口径。
- 影响 SQLite 中 research / draft / digest 与验证消费层之间的契约，但不新增 Agents 侧业务规则。

## Success Criteria

- `storybook` 与 `dagger` 的最终专项报告都基于 `.wiki/*.md` 给出 `overall_match_rate`，且两者都 `>=95%`。
- 报告必须同时给出 `missing pages`、`collapsed pages`、`low-fidelity matched pages` 与 `extra generated pages`，保证 95% 不是靠增页堆出来的表面结果。
- 最终验收时，高频 collapse 页面已不再是主要问题，citation / evidence / Mermaid 已进入正式 Markdown contract，且不再构成主要失分项。
- 最终验收时，英文 raw docs 文件名不再大批残留；如果仓库中存在本地化/派生 docs 语料，最终 `.wiki/` 页面路径与标题必须优先跟随该语料。
- 对 docs-backed 页面，最终 Markdown 主章节骨架必须稳定贴近 reference，尤其是 `cite -> 目录 -> 简介 -> 项目结构 -> 核心组件 -> 架构总览 -> 详细组件分析 -> 依赖关系分析 -> 性能考量 -> 故障排查指南 -> 结论 -> 附录` 这类稳定结构不得继续退化为英文原始 docs 标题或泛化模板章节。
