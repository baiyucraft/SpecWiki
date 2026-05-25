## Why

9.3 已经把 targeted dossier、section plan 和 `overview/architecture` research 接进主链，但 reference 对比仍表明，当前质量瓶颈不在“模型是否可用”，而在“页面本体、研究时机和父子页组织方式”仍然偏向 `module/topic` 模板树。尤其在 `storybook` 这类 docs-heavy 平台仓库上，reference 实际是产品知识树、API 树、配置树和插件树，而当前实现仍把大量内容折叠进少数总览页、模块页和专题页；而 `dagger` 这类 runtime-heavy / compiler-heavy 工程平台仓库又进一步暴露出 family planner 之外的模块树、主题拆分和 API 文档粒度问题。9.4 的核心目标不是泛化到所有仓库，而是先用 `storybook + dagger` 这两个验收样本把通用抽象收稳。

进一步对照 CodeWiki 与 deepwiki-rs 的真实源码后，方向已经很清楚：当前主链需要从“planner + renderer 主导，LLM 补摘要”转向“family/domain planning + leaf-first 一手材料 + research-first composition”。如果不在这一层做大调整，继续追加 tool turn、prompt 或局部专题页，只会继续增加 token 和耗时，而不会把页面结构真正拉向 reference。

## What Changes

- 引入内容家族规划层（content family planner），在现有模块树之上稳定发现 `concept / addon / framework / builder / api / config / ops / troubleshooting / theme` 等页面家族，并为每个家族规划索引页和子页。
- **BREAKING**：页面本体从“overview / architecture / workflow / module / topic”为主，扩展为“family index + family child + module/topic”混合树；对 `storybook` 一类 docs-heavy 仓库，部分现有专题页和模块页将被新的 family 页族替代或收编。
- 引入 leaf-first/source-fed 生成路径：叶子 family 页、叶子模块页和高置信主题页优先消费一手源码、类型/API surface、配置入口和 docs anchors，再由父页基于子页 rollup 组织内容。
- 把 research-first compose 提升为正式页面主路径：research 不再只生成 section 补丁，而是先形成 `family/module/overview/architecture` 的研究材料，再由 compose 层按页面类型成页。
- 扩展 dossier 输入来源：除 targeted snippets 外，新增 docs anchors、public API surface、config surface、child page results 和 family-scoped evidence 作为一等输入。
- 强化 parent consume child：父页优先消费子页 section plan、evidence rollup、diagram rollup 和 key sources，而不是重复重扫 facts。
- 参考 Qoder 的中间索引分层，把 `tree / graph / memory / vector / completion` 视为不同职责层：9.4 的 family planner 只消费树、图和稳定事实信号，research/session 结果继续以 digest/rollup 形式单独持久化，不把样本仓库目录规则直接混进 renderer。
- 升级 reference 验证：针对 `storybook` 与 `dagger` 两个验收样本，分别观察 docs-heavy family 贴近度、runtime-heavy 模块/API 拆分贴近度、API/config/docs 锚点命中率和页面折叠度指标。
- 本轮测试与验证范围收敛到 `storybook + dagger`，并以“生成产物尽可能贴近 reference”作为验收标准，不在 9.4 内要求回跑全量 19 个测试项目。

- **移除 Deterministic Fallback**：Research 层和 Compose 层不再支持退化到模板填充。LLM 接入是 Research/Compose 层的硬前置条件，不允许生成质量退化。
- **引入中断-保存-恢复（Checkpoint & Resume）**：LLM 请求异常时 pipeline 立即中断，保存当前进度（已完成的 research/compose 结果 + 中断位置）到 SQLite `pipeline_checkpoint` 表；下次重启时检测检查点，若 Facts 层输入未变化则从中断处恢复。
- **清理 Steering 旧 LLM 子开关**：移除 `content_enrichment_enabled` / `session_enabled` / `uncertainty_gate_enabled` / `page_enrichment_*` 等废弃字段，简化为 `llm.enabled` + `max_research_calls` + `max_compose_calls`。

## Capabilities

### New Capabilities
- `content-family-planner`: 定义如何在模块树之上发现稳定内容家族、索引页与子页，并将 docs/API/config/plugin/framework 等产品化知识域纳入正式页面规划。

### Modified Capabilities
- `page-research-dossier`: dossier 输入从代码/图片段扩展到 docs anchors、public API surface、config surface、child page results 和 family-scoped evidence。
- `research-driven-page-composition`: 页面生成从 section-plan 驱动升级为 research-first compose，要求父页显式消费子页结果，且 family 页与 overview/architecture 同样受该 contract 约束。
- `topic-page-planner`: 现有专题页规划需要与 family planner 协同，明确 family 页、专题页和模块页的去重、收编与父子关系规则。
- `page-evidence-layer`: evidence layer 需要支持 family-scoped provenance、docs/API/config 锚点和按页面家族重排的出处命中。
- `repo-wiki-runtime`: runtime 需要持久化 family page、family rollup、child compose inputs 和新的页面树关系。
- `repo-wiki-workflow`: workflow 主链要在 planner 与 render 之间引入 family/domain research 与 compose，不再只围绕模块页和专题页执行 research。
- `wiki-llm-enhancement`: LLM 使用边界从“页级补充”升级为“leaf-first/source-fed/research-first compose” 的正式 contract。
- `workflow-verification`: reference 与项目集验证需要增加 family coverage、page collapse、docs/API/config 命中和父页消费子页结果的检查。

## Impact

- 受影响代码主要集中在 [planner.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/planner.rs)、[context.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs)、[sections.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs)、[page_render.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs)、[mod.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs)、[research_engine.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/research_engine.rs)、[steering.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/steering.rs) 以及相应 tests / reports / scripts。
- 这轮会显著调整 storybook 等文档型、平台型仓库的页面树和相对路径；当前处于测试开发阶段，不保留旧页面拓扑兼容。
- 移除 deterministic fallback 后，Research/Compose 层必须有 LLM 接入才能运行；无 LLM 时 pipeline 返回错误而非生成低质量模板页面。
- 新增 `pipeline_checkpoint` 表和中断恢复逻辑，影响所有 workflow 入口（init / rebuild / update）。
- 不引入新的宿主依赖，也不提前实现迭代 11 的 CodeBuddy Agent bridge；这轮仍然只收 core 生成链。
