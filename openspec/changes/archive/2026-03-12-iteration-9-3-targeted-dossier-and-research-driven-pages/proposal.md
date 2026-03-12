## Why

9.2 已经把 dossier、bounded research session、provider tools 和 budget/usage 控制接入主链，但 reference 对比和 trace 复盘都表明页面质量瓶颈仍不在“是否已接上大模型”，而在“给模型的源码材料太浅、research 结果太薄、planner 页粒度偏粗，以及 renderer 仍主要由固定模板主导”。如果不把这些结构性问题收掉，继续调 prompt 或单纯增加 tool turns 只会增加 token 和耗时，无法明显缩小与 reference 的差距。

## What Changes

- 引入精准 dossier 取证路径：围绕 symbol、call trace、process、entry point 和 child rollup 选择定点源码片段，替代当前以文件前 24 行为主的源码预览。
- 引入 research-driven page composition：让 `PageResearchResult` 从“摘要补丁”升级为“section plan + section evidence/diagram refs + summary”，由 research 结果驱动页面结构，而不是继续只回填 `summary/key_points/open_questions`。
- 把 bounded research session 从 `module/topic` 扩展到 `overview/architecture`，并让 session state 真正成为可复用的显式对象，而不是只在单页 loop 内部短暂存在。
- 升级 planner 与 topic seed 规则，按仓库 archetype 增补根级机制页、路由/入口链路页、配置与运行时页、协议/数据模型页等高频缺失专题，并继续抑制重复页。
- 升级 evidence/source layer：让页面和 research 输入都能携带真实 `path + line span + section-level evidence refs`，提升出处层密度与可追溯性。
- 补充 reference 验证与 trace 复盘指标，明确观测 section-plan 覆盖率、citation 密度、overview/architecture research 命中和 archetype-based planner 效果。

## Capabilities

### New Capabilities
- `research-driven-page-composition`: 定义 research 输出如何驱动页面 section 结构、section 级证据引用和图计划，而不是只补摘要字段。

### Modified Capabilities
- `page-research-dossier`: dossier 输入改为精准源码片段、真实 line span 和高价值 child rollup 优先。
- `page-evidence-layer`: evidence layer 增加真实来源行号、section 内证据引用和更细的分组约束。
- `topic-page-planner`: planner 按仓库 archetype 扩展专题页发现、去重和父子关系规则。
- `repo-wiki-runtime`: runtime 持久化 section-plan 驱动的页面状态、evidence/source provenance 和 research session 摘要。
- `repo-wiki-workflow`: `overview/architecture` 进入 research 主链，并在 render 前消费 section plan。
- `wiki-llm-enhancement`: research session、tool 使用和结构化结果 contract 从“摘要补丁”升级为“research-driven composition”。
- `workflow-verification`: 项目集和 reference 报告新增 section-plan、精准 evidence 和 archetype planner 验证。

## Impact

- 受影响代码主要集中在 [llm/mod.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs)、[generation/context.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs)、[generation/sections.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs)、[generation/planner.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/planner.rs)、[workflows/page_render.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 和对应 tests / scripts。
- 主要影响 provider-first 的页面质量、reference 对齐程度和 cold/warm token 使用结构；不引入新的宿主依赖，不把 CodeBuddy Agent 范围提前到 9.3。
