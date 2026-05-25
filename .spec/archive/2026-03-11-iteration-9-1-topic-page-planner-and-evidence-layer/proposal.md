## Why

迭代 9 已经把 LLM 可选桥接、内容增强和批量化不确定性判断接进了真实主链，但 10 个 reference 项目的逐项目报告表明，当前结果和 reference 的主要差距已经不再是“正文写得不够像文档”，而是页面主题拆分、来源落页和图表达仍然偏粗。现在继续只调 prompt 的收益已经明显下降，下一步必须把 page planner、page context 和 evidence/render contract 一起收紧。

## What Changes

- 新增“专题页 planner”能力：在现有 overview / architecture / module / workflow 之外，允许 planner 产出核心机制页、能力簇页和流程主题页，而不是把大量 reference 主题折叠进少数目录页。
- 新增“页面 evidence layer”能力：为 page / section 提供稳定的关键来源文件、证据簇和落页 contract，让页面不仅有结论，还能稳定说明“这些结论来自哪些源码”。
- 把 Mermaid 收敛为 facts-driven 表达：模块依赖、父子结构和流程图优先由 deterministic facts 提供图输入，LLM 只负责解释层组织，不再自由发明图结构。
- 调整 root-level 高信号源码的晋升逻辑：允许 `chi` 的 `context.go / tree.go / chain.go`、`axum` 的 `extract/* / response/* / routing/*` 这类根级核心文件簇进入专题页规划，而不是只能依赖目录模块。
- 升级 reference 报告和验证口径：后续项目集分析必须显式统计专题页覆盖率、evidence 落页情况和图表达覆盖，而不再只看页面数量和段落密度。

## Capabilities

### New Capabilities
- `topic-page-planner`: 定义专题页的发现、分组、父子关系和稳定页面身份。
- `page-evidence-layer`: 定义页面与 section 的来源证据、关键源码块和 evidence identity contract。

### Modified Capabilities
- `repo-hierarchy-model`: 调整模块/主题发现输入，允许根级高信号文件簇和能力簇参与页面规划。
- `repo-wiki-runtime`: 页面 runtime 需要持久化专题页、evidence block 和对应的稳定身份。
- `repo-wiki-workflow`: `build_contexts -> plan_pages -> render` 主链需要支持专题页规划、evidence 输入和 facts-driven 图表达。
- `wiki-llm-enhancement`: LLM 增强必须消费专题页与 evidence 输入，并把图表达约束为 facts-driven block，而不是自由生成结构。
- `workflow-verification`: 项目集验证和 reference 报告需要覆盖专题页命中率、evidence 落页和图表达覆盖。

## Impact

- 受影响代码主要集中在 `crates/wiki-core/src/repo/{hierarchy,scanner}.rs`、`crates/wiki-core/src/generation/{context,planner,renderer,sections}.rs`、`crates/wiki-core/src/workflows/{init,update,rebuild,page_render}.rs` 和 `crates/wiki-core/src/llm/mod.rs`。
- reference 报告脚本、项目集分析文档和对应自动化测试需要一起升级。
- 页面结构和 metadata 会新增专题页/evidence 相关字段，但不引入新的 runtime 存储层。
