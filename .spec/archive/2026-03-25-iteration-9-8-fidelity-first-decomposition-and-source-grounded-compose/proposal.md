## Why

`9.7` 已经把 `storybook + dagger` 的 runtime gate、parent contract 和 readiness 诊断收敛到可观测状态，但最终产物仍未通过 fidelity 验收：`storybook` 仍停在 `overall=92.05% / reuse_overage=123 / median_skeleton=0.13 / median_key_source=0.03`，`dagger` 仍停在 `overall=95.38% / reuse_overage=25 / median_skeleton=0.08 / median_key_source=0.07`。现在主矛盾已经从“看不见卡在哪”转成“KnowledgeUnit 拆分不准、research 不够深、compose 不够贴近关键源码与 reference 骨架”，必须把 2.0 主链真正收敛到可读、可对齐、可复用的页面质量。

## What Changes

- 收紧 `KnowledgeUnit` 拆分 contract，把 `storybook / dagger` 暴露出的高频缺页、many-to-one reuse 和粗粒度折叠，沉淀为通用的 `repo_archetype_signals`、`leaf decomposition policy`、`collapse guard` 和 `domain/unit` 规则，而不是继续依赖关键词表或样本特征散落在 planner 中。
- 提升 `Research` 主链的 fidelity contract，在当前 runtime 已选择 provider-backed 路径时，让 docs-backed、api/config/example/troubleshooting、runtime/compiler/testing 这些知识面稳定产出可被 compose 直接消费的 `skeleton profile`、`key source clusters` 和 `section plan`，而不是继续依赖后置补丁式增强；本轮不单独改写现有 agent bridge / fallback 可用性策略。
- 重构 `Compose` 页面 contract，要求最终 Markdown 围绕统一的 `ComposePageContract(section_plan + skeleton_profile + key source clusters + citation/evidence + child digest/citation digest/diagram digest/readiness rollup)` 成页，并把 `Overview / Architecture / DomainIndex / config_surface parent unit` 一并纳入，而不是继续由固定骨架、轻量 child summary 或泛化段落拼装主导。
- 把 `key source coverage`、`skeleton fidelity`、`missing pages` 和 `reuse_overage` 从“报告指标”前移为正式实现目标，要求 planner / research / compose 三段都能对这些指标负责。
- 强化 reference/runtime 分析口径，让 `storybook + dagger` 的 gap ledger 直接映射回 `KnowledgeUnit decomposition`、`source-grounded compose` 和 `reference-style skeleton` 的链路断点，并把这些映射沉淀成通用诊断规则。
- **BREAKING**：正式 core 不再接受基于仓库名、reference 标题、样本关键词或固定目录白名单驱动的 planner/compose 特判作为合法收敛路径。
- **BREAKING**：正式页面不再接受“有 citation 但正文仍由固定模板或泛化摘要主导”的输出作为达标结果。

## Capabilities

### New Capabilities

- 无

### Modified Capabilities

- `knowledge-unit-decomposition`: 新增通用 `repo_archetype_signals`、`leaf decomposition policy` 和 `collapse guard`，要求 planner 直接对 missing/collapsed page 负责，而不是只声明中粒度拆分。
- `research-driven-page-composition`: 新增 `skeleton profile`、`key source clusters` 和统一 `ComposePageContract`，要求 compose 显式消费 `child digest / citation digest / diagram digest / readiness`，并把 `config_surface` parent unit 一并纳入统一父页 contract。
- `page-evidence-layer`: 扩展 evidence contract 到 `key source cluster`、`section_grounding_refs` 与 section 解释层，要求 citation 不只是落页，还要支撑关键文件覆盖与 section-grounded 正文。
- `reference-fidelity-reporting`: 增加 `gap ledger -> planner/research/compose` 的链路级诊断，直接回答 fidelity regression 发生在哪一层 contract。
- `workflow-verification`: 增加 9.8 的专项验证要求，确认 `storybook + dagger` 的 missing/reuse/skeleton/key-source 回归都能映射回对应主链 contract，且 fresh init 与 skip-init/warm 报告口径不再混淆。

## Impact

- 重点影响 `crates/wiki-core/src/generation/knowledge_planner.rs`、`crates/wiki-core/src/generation/research_engine.rs`、`crates/wiki-core/src/workflows/research_provider.rs`、`crates/wiki-core/src/generation/compose_engine.rs` 以及相关 runtime/cache bridge。
- 重点影响 `scripts/collect-reference-project-reports.mjs`、`scripts/testing/wiki-runtime-inspection.mjs`、`scripts/tests/*.test.ts` 的专项分析与验收口径。
- 影响 `storybook + dagger` 的 reference/runtime 对照方式，但结论必须沉淀成可复用的通用规则，不新增样本仓库专有分支；`9.8` 本轮验收不要求回到 19 项目集全量测试。
