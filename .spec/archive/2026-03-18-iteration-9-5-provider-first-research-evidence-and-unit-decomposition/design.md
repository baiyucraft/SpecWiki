## Context

`9.4` 之后，`wiki-core` 的主链已经具备 2.0 外形，但当前实现仍保留了明显的过渡期结构：

- `crates/wiki-core/src/workflows/init.rs`、`rebuild.rs`、`update.rs` 当前都把 `run_compose_pipeline()` 绑定到 `StructuralResearchProvider`，说明 runtime 仍以结构型 research 兜底，而不是 provider-first 的 Research 主链。
- `crates/wiki-core/src/workflows/page_render.rs` 已经具备 `system -> domain -> unit` 的 research cache、checkpoint 和 leaf-first compose 顺序，但 `build_research_digest()` 与 `compose_unit_page()` 仍主要消费扁平 `summary / key_topics / key_sources`。
- `crates/wiki-core/src/generation/compose_engine.rs` 已经在 `PageDraft` 中累计 `citation_count`，storyboard 当前缓存库中 `106/143` 个 page draft、dagger 中 `38/72` 个 page draft 的 `citation_count > 0`，但 `compose_section()` 只是把 citation 写成 `` `path` (Lx-Ly) `` 的普通 bullet。
- `crates/wiki-core/src/generation/renderer.rs` 渲染 `PageDraft` 时没有把 citation 和 diagram 提升为正式 Markdown contract；`render_page_draft()` 只把 citation 映射成 `source_ids`，最终写盘仍只是 section body。
- `scripts/collect-reference-project-reports.mjs` 当前只把 `(file://...)` 链接和 evidence heading 统计为 citation/evidence，并把 ` ```mermaid ` 统计为图表达；因此虽然 cache 中已有 citation 计数，专项报告仍显示 `storybook 0/77.72`、`dagger 0/79.69`。
- 当前 `knowledge_units` 实际分布表明 planner 仍停留在大桶拆分：
  - storybook: `143` 个 unit 中 `91` 个 `ModuleDoc`、`20` 个 `ConceptGuide`、`13` 个 `ApiDoc`
  - dagger: `72` 个 unit 中 `32` 个 `ModuleDoc`、`20` 个 `ConceptGuide`、`2` 个 `ApiDoc`
  - 这与 reference 的缺口一致：storybook 还有 `50` 个 reference 页面缺失，dagger 还有 `31` 个缺失，而且大量 reference 主题被折叠到 `项目概述.md`、`dagger-runtime.md`、`dagger.md` 这类大页上。

这说明 `9.5` 不应再围绕“补一个 page type”或“继续做 family 特判”推进，而应同时修正三个断层：

1. runtime research 仍不是 LLM/provider 主驱动；
2. citation / diagram 仍停留在中间结构，未成为正式 markdown contract；
3. KnowledgeUnit 的中粒度拆分规则仍不够贴近真实代码结构与 reference 的主题边界。

除此之外，当前产物还暴露了两个直接影响“看起来不像 reference”的问题：

- 仓库内同时存在英文 raw docs 与本地化/派生 docs 语料时，planner 仍会并存生成两套 docs-backed 页面，导致 `.wiki/` 中残留大量英文文件名。
- docs-backed 页面即使已经消费了 reference 式 markdown，也可能因为 planner / compose contract 没有把“主章节骨架保真”定义为正式约束，而退回英文原始 docs heading 序列或泛化模板节标题。

这三个方向都有明确的上游源码依据：

- CodeWiki 的 `DocumentationGenerator.get_processing_order()` 与 `build_overview_structure()` 体现的是真正的 `leaf-first + parent-consume-child-docs`，不是父页直接再扫一遍 facts。
- deepwiki-rs 的 `ResearchOrchestrator.execute_research_pipeline()` 先跑 C1/C2/C3 研究对象；`compose/agents/overview_editor.rs` 的 `data_config()` 只消费 `ResearchResult(...)`，这正是 research-first compose 的硬边界。
- GitNexus 的 `pipeline.ts`、`symbol-table.ts`、`call-processor.ts`、`community-processor.ts` 说明复杂度应该收在厚 facts / 索引 / 图分析层，而不是继续把页面猜测逻辑塞回 renderer。
- deepwiki-open 的 `api/api.py` 和 websocket / rag 路由把 wiki cache 与 query/session 消费层分开，说明生成主链与消费层必须继续解耦。

## Goals / Non-Goals

**Goals:**

- 让 `init / rebuild / update` 的正式 runtime 走 provider-first research / compose 主链，并保留 checkpoint / cache / leaf-first 顺序。
- 把 docs/API/config/runtime/testing/example/tutorial/troubleshooting 这些真实代码与文档信号统一纳入 KnowledgeUnit decomposition policy，减少 storybook / dagger 的大页折叠。
- 把 citation / evidence / diagram 变成最终 `.wiki/*.md` 可统计、可追溯、可被 reference 脚本识别的正式 contract。
- 让 reference 报告与 lifecycle 验证直接验证最终 markdown，而不是只验证 cache 中间态。
- 把 `9.5` 的测试与验收范围明确收敛到 `storybook + dagger`，不在本轮实现中追加 19 项目集全量回归。
- 让 `storybook` 与 `dagger` 各自对 reference 的总体对齐率收敛到 `>=95%`，并把这一目标拆解为可迭代的 planner / research / renderer / report 收敛步骤。
- 让 docs-backed 页面在命名与路径上优先跟随本地化/派生 docs 语料，而不是在最终 `.wiki/` 中继续大批保留英文 raw docs 文件名。
- 让 docs-backed 页面正文稳定收敛到 reference 的主章节骨架，而不是只做到“内容大致相关”。
- 让 LLM hard-stop 预算成为显式 steering contract，而不是隐藏在 runtime 内部的固定常量；默认预算必须对中大仓库足够宽松，同时仍保留可配置 hard-stop。

**Non-Goals:**

- 不在 `9.5` 引入 query / RAG / session 新能力；deepwiki-open 仅作为生成与消费分层参考。
- 不新增针对 storybook 或 dagger 的硬编码 planner / renderer 分支。
- 不保留旧的 family-centric 页面拓扑兼容层；当前阶段允许直接替换过渡期 contract。
- 不把 Agents 层变成业务规则承载处。
- 不在 `9.5` 内把专项验证重新扩展到全量 19 项目。

## Decisions

### 决策 1：runtime 正式切到 provider-first research provider，`StructuralResearchProvider` 仅保留给测试

当前 `run_compose_pipeline()` 的 research contract 已经可缓存、可恢复、可按 `system -> domain -> unit` 顺序执行，真正的问题不是流程不存在，而是 workflow 入口始终注入 `StructuralResearchProvider`。`9.5` 将新增 runtime 级 provider 选择层：

- 正式 workflow：
  - 优先使用 provider-backed `ResearchProvider + ComposeProvider`
  - 仅在测试 fixture、离线单测或显式开发开关下允许 `StructuralResearchProvider`
- `StructuralResearchProvider`：
  - 保留用于 tests、golden fixtures、无网络结构调试
  - 不再是正式 `init / rebuild / update` 的默认实现

原因：

- 这与 [.wiki/06-设计文档/00-总体设计.md](E:/project/!byAI/spec-wiki/.wiki/06-设计文档/00-总体设计.md) 的 `LLM-required Research / Compose` 一致。
- 现有 `page_render.rs` 已有 cache/checkpoint，不需要重建 pipeline，只需要把 provider 选择从“固定 structural”升级为“runtime capability”。
- deepwiki-rs 的 research orchestrator 和 compose editor 都把 `ResearchResult` 作为正式输入，而非测试补丁。

备选方案：

- 继续让 `StructuralResearchProvider` 做默认实现，在有 provider 时“加一层增强”。
  - 否决原因：这会继续把 research 结果降级为补丁，无法解决 `page_research=0` 与 reference 正文不足的问题。

### 决策 2：引入 `ResearchProfile` 与 `DecompositionProfile`，按知识单元类型生成不同 research object

当前 `UnitResearch` 过于统一，storyboard 和 dagger 都把大量 reference 页面挤进 `ConceptGuide` 或 `ModuleDoc`。`9.5` 将把“研究对象类型”和“拆分策略类型”显式建模：

- `DecompositionProfile`
  - `runtime`
  - `api-surface`
  - `config-surface`
  - `docs-guide`
  - `testing`
  - `example-tutorial`
  - `troubleshooting`
  - `integration-platform`
  - `compiler-pipeline`
- `ResearchProfile`
  - 与 `DecompositionProfile` 对齐，但更偏 research 输出契约
  - 控制 section plan、evidence clustering、diagram suggestion、child digest slot

planner 规则：

- `content-family-planner` 中已有的 family signal 不再直接定义页面体系，只作为 `knowledge-unit-decomposition` 的一个信号面。
- domain 内拆分不再只看目录名，而是联合以下真实输入：
  - docs anchors
  - public API surface
  - config surface
  - module kind / module graph
  - process / community / SCC 信号
  - tests/examples/tutorial 目录与 manifest 元数据

对应结论：

- storybook 的 `Addon API / Preview API / Store API / main.js配置 / manager.js配置 / preview.js配置` 不应继续统统落在 `ConceptGuide`，而应按 `api-surface` 和 `config-surface` 进入独立 profile。
- dagger 的 `快速开始 / 示例与教程 / 编译时处理机制 / 测试策略与最佳实践` 不应继续被 `dagger-runtime`、`dagger.md` 或 `javatests.md` 吸收，而应走 `example-tutorial`、`compiler-pipeline`、`testing` profile。

备选方案：

- 只给现有 `UnitType::ConceptGuide` 增加更多 heuristics。
  - 否决原因：这会继续放大 `ConceptGuide` 大桶，无法解释 reference 的 API/config/testing/example 页树。

### 决策 2.1：当仓库同时存在多套 docs 语料时，planner 优先选择本地化/派生 docs corpus

当前 storybook 同时存在英文 raw docs 与结构化的本地化/派生 docs 语料；如果 planner 不显式做 corpus 级优先级选择，就会在最终 `.wiki/` 中并存生成两套页面，一套是参考系更强的本地化页面，另一套是英文 raw docs 页面。这会直接拉高 `extra generated pages`，也会让用户第一眼看到“文件名都还是英文”。

`9.5` 在 docs-backed unit planning 中引入 corpus 级优先级：

- 若仓库中只存在单一 docs corpus，保持现有行为；
- 若仓库中同时存在多套 docs corpus，则：
  - 优先选择已经带有本地化/派生特征、且更接近最终消费形态的那一套；
  - 未被选中的 raw docs corpus 继续作为 evidence / citation 来源，但不再直接生成平行 wiki 页面。

实现约束：

- 规则必须建立在通用 docs corpus 信号之上，而不是硬编码样本仓库名、reference 标题或固定目录；
- corpus 选择影响的是 docs-backed page generation，不影响 facts 层对原始 docs 文件的扫描与引用。

备选方案：

- 继续同时生成多套 docs 页面，再依赖后处理或 report 去忽略英文 raw docs。
  - 否决原因：这会把核心错误留在 planner 层，最终用户仍然会看到一堆英文文件名和重复页面。

### 决策 3：citation / evidence / diagram 采用统一的最终 Markdown contract

当前 cache 中已有 citation 数据，但 final markdown 与验证脚本使用的 contract 不一致。`9.5` 统一 contract：

- citation：
  - 在 section 内或紧随 section 输出正式 evidence block
  - 引用项必须包含可被脚本识别的 `file://` 路径或等价稳定格式
- evidence block：
  - 使用稳定 heading / marker，允许 `scripts/collect-reference-project-reports.mjs` 和 `scripts/collect-test-project-analysis.mjs` 直接统计
- diagram：
  - `DiagramSuggestion -> DiagramDraft -> final markdown mermaid fence`
  - 不再只停留在 `diagram_suggestions` 或 `diagrams: Vec::new()`

实现约束：

- renderer 才是最终 contract 的唯一出口，compose 只产出结构化 `citations / diagrams / section_plan`
- 报告脚本不再假定 citation 只来自旧 `(file://...)` 文本，但仍要有一个唯一、稳定、正式的 markdown 识别方式

原因：

- 现状已经证明“draft 有 citation_count，但报告仍是 0”不是 research 缺失，而是 markdown contract 缺失。
- 这也更符合 2.0 的 `Citation-driven Content`，因为 citation 必须进入正式页面，而不是留在缓存表里。

备选方案：

- 只修改验证脚本，让它直接读 `page_drafts`。
  - 否决原因：这会掩盖最终产物缺失，无法满足 `.wiki/*.md` 是正式 runtime 输出的边界。

### 决策 3.1：docs-backed 页面必须把“主章节骨架”作为正式 markdown contract 的一部分

对于 docs-heavy 样本，用户看到“像不像 reference”不仅取决于有没有 citation / diagram，还取决于最终页面是否稳定呈现出 reference 的主章节骨架。`9.5` 对 docs-backed 页面新增这一约束：

- research / compose 必须显式区分：
  - 文档前言 / cite / preamble
  - 主章节骨架
  - 补充 evidence / child digest / diagram
- 当 reference 页面已经表现出稳定章节序列时，最终 Markdown 必须优先保留或收敛到该序列，例如：
  - `cite -> 目录 -> 简介 -> 项目结构 -> 核心组件 -> 架构总览 -> 详细组件分析 -> 依赖关系分析 -> 性能考量 -> 故障排查指南 -> 结论 -> 附录`
- renderer 不得再把这类页面重新包成英文原始 docs heading，或退化为“概述 / 详细说明 / 子页摘要”之类的泛化模板章节。

备选方案：

- 只要求“内容相近”，不要求章节骨架相近。
  - 否决原因：这会继续产生“内容可能相关，但第一眼完全不像 reference”的产物。

### 决策 4：父页只消费结构化 leaf result，不回退到扁平 facts

CodeWiki 的关键不是“先 DFS”，而是父页真实消费子文档结果。`9.5` 继续收紧这一边界：

- leaf unit 先产出：
  - `UnitResearch`
  - `PageDraft`
  - `PageDigest`
  - `EvidenceDigest`
  - `DiagramDigest`
- parent unit 输入：
  - 优先消费 `child digests + child evidence/diagram digests`
  - facts 只作为 scope 校验与 citation 回溯来源

这样可以避免：

- storybook 的 overview 再次吸收几十个 reference 主题
- dagger 的 `dagger-runtime` 再次承担教程、测试、API 三种不同职责

备选方案：

- 父页允许“缺 digest 时回退到 facts 全扫”。
  - 否决原因：这会重新引入 page collapse，且与 deepwiki-rs / CodeWiki 的分层思路相反。

### 决策 5：reference 与 lifecycle 验证改为“最终页面 contract + decomposition 命中”双重验收

`9.5` 的验证不再只关注页面数、topic 数，而改为同时检查：

- 最终 markdown 是否包含可识别 citation / evidence / mermaid
- `pageCollapseRatio`
- `knowledge-unit decomposition hit rate`
- `api/config/docs/runtime/testing/example/tutorial` 的命中分布
- storybook / dagger 专项缺口

这会直接修复当前“cache 有 citation，报告却是 0”的验收偏差，并让 `knowledge-unit-decomposition` 成为可观测 contract，而不是只存在于 planner 内部。

为了把 `storybook + dagger` 真正收敛到 `>=95%`，`9.5` 采用固定的专项收敛方法，而不是一次性大改后再看结果：

1. 先固定总指标：
   - `overall_match_rate = matched / reference`
   - 以最终 `.wiki/*.md` 为准
   - `storybook >= 95%`
   - `dagger >= 95%`
2. 再把未达标差距拆成三类：
   - `missing pages`
   - `collapsed pages`
   - `matched but low-fidelity pages`
3. 每轮只做一类收敛：
   - 缺页优先修 planner / decomposition
   - 折叠优先修 leaf-first parent-consume-child
   - 命中了但质量不足优先修 research / citation / diagram / renderer contract
4. 每次改动后必须重跑 `storybook + dagger` 专项 report：
   - 如果总匹配率提升，但新的 extra pages 明显上升，则回收拆分阈值
   - 如果 citation / diagram 仍是主要短板，则禁止继续追加 planner 复杂度，先补 renderer/report contract
5. 只有当两个项目同时满足以下条件时，本轮才算完成：
   - `overall_match_rate >= 95%`
   - 主要高频 collapse 页面已消除
   - citation / diagram 不再是报告中的主短板
   - 英文 raw docs 文件名不再大批残留
   - docs-backed 页面主章节骨架不再明显偏离 reference
   - lifecycle 脚本通过

### 决策 5.1：LLM hard-stop 预算必须显式配置化，默认值按中大仓库场景抬高

当前 `wiki-core` 的 LLM runtime 存在两个问题：

- 一部分硬限制已经可配置，但默认值偏小，仍按小仓库调优；
- 另一部分硬限制仍写死在 runtime 内部，例如 page research turn 上限；
- 更关键的是，某些预算分配公式即使把顶层配置抬高，也会继续把真实 page research 配额卡在固定小值。

`9.5` 在 runtime contract 上新增以下要求：

- 全局调用预算、page research turn 上限与相关 hard-stop 必须通过 steering 暴露；
- 默认值必须针对 `storybook / dagger` 这类中大仓库提高，而不是继续以 20 多次调用为基线；
- 提高默认预算时，内部 page research 配额公式也必须同步放宽，保证“默认值更高”会真实转化为更多 page research 机会；
- 预算仍然是 hard-stop，不允许无限制循环；只是默认值不再过早截断 provider-first 主链。

原因：

- 当前 reference 收敛阶段最大的误判之一就是“模型已经结束思考”，实际却是 page research 提前被预算打断；
- 如果默认预算仍过低，后续对 planner / research / renderer 的质量判断会持续被截断噪音污染；
- 这与 Codex 式 agent loop 借鉴点一致：应区分“任务真的完成”和“预算先耗尽”。

备选方案：

- 保持当前默认值，只靠 `wiki.dev.yaml` 为个别测试仓库单独加预算。
  - 否决原因：这会把核心 runtime contract 留在“不合理默认值 + 局部人工覆盖”的状态，无法作为 2.0 主链基线。

备选方案：

- 保持现有 reference 报告逻辑，只扩充 prompt 和 pages。
  - 否决原因：验收口径错误时，任何内容改进都会被低估或误判。

### 决策 6：把 95% 对齐目标拆成显式收敛门槛，而不是抽象“更像 reference”

`95%` 不是模糊目标，必须转成可停止的工程门槛。`9.5` 采用以下门槛：

- 一级门槛：总体匹配率
  - `storybook >= 95%`
  - `dagger >= 95%`
- 二级门槛：结构错误不能掩盖在总体分数里
  - 高频 collapse 页面数必须下降到“非主要问题”
  - 缺失页必须集中到边缘主题，而不能还集中在核心 domain
- 三级门槛：内容表达必须达标
  - citation / diagram 已在最终 markdown 中稳定出现
  - matched 页面不再大面积出现“命中了但正文仍明显短于 reference”的系统性短板

原因：

- 只看总体命中率，可能通过“多生成一批相似页”抬高分数，但核心 collapse 问题仍然存在。
- 只看 citation / diagram，也无法证明页树已经对齐。
- 三层门槛同时存在，才能逼近“真正对标 reference 95%”。

备选方案：

- 只要求 `matched / reference >= 95%`。
  - 否决原因：这会允许结构仍错、内容仍空但表面分数达标。

## Risks / Trade-offs

- [风险：provider-first runtime 让本地无 provider 环境更容易失败] → 通过显式开发模式保留 `StructuralResearchProvider`，但正式 workflow 默认不再 silently fallback。
- [风险：decomposition profile 过细导致页面暴涨] → 引入 profile-specific 阈值、child budget 和 collapse guard，不满足多信号命中的候选直接合并回父单元。
- [风险：为了追求 95% 盲目增页，导致 extra generated pages 暴涨] → 把 `missing/collapsed/low-fidelity` 分开治理，每轮专项报告同时观察 `extra generated` 与 collapse 变化，禁止只靠增页堆分。
- [风险：优先本地化/派生 docs corpus 后丢掉 raw docs 中独有的信息] → 原始 docs 仍保留在 facts / evidence / citation 输入中，只是不再平行生成页面。
- [风险：主章节骨架过度收紧，导致非 reference 风格仓库被错误模板化] → 章节骨架规则只在 docs-backed 页面存在稳定结构信号时启用，不对所有仓库强推固定章节表。
- [风险：citation contract 改动会影响现有 markdown 与报告脚本] → 统一由 renderer 和脚本同时升级，并保留一次性 migration 逻辑重写旧格式统计。
- [风险：diagram 正式落页后更容易出现无效 Mermaid] → diagram 继续走结构化 `DiagramSuggestion -> DiagramDraft` 守卫，不接受模型自由图。
- [风险：`content-family-planner` 被弱化后历史测试用例需要重写] → 直接把 family spec 收编为 decomposition signal，并同步替换相关 fixture / assertion。

## Migration Plan

1. 先升级 provider 选择与 runtime contract，让 workflow 不再固定使用 `StructuralResearchProvider`。
2. 在 planner 中引入 `DecompositionProfile`，把 family/docs/api/config/testing/example 等信号统一映射到 KnowledgeUnit 拆分规则。
3. 扩展 `UnitResearch / PageDigest / DiagramDraft / EvidenceDigest`，让 research-first / leaf-first 的结构化结果可被父页正式消费。
4. 调整 renderer 与 markdown contract，正式落 citation / evidence / mermaid。
5. 升级 reference report 和 lifecycle 脚本，改为对最终 `.wiki/*.md`、docs 命名/路径、主章节骨架与 decomposition 命中做校验。
6. 只对 `storybook + dagger` 执行专项 init / lifecycle / reference 收敛，并按 `missing -> collapse -> low-fidelity` 顺序循环修正，直到两者都达到 `>=95%`。

当前阶段不保留旧页面拓扑兼容；若旧的 family/page contract 与新 decomposition contract 冲突，允许直接替换。

## Open Questions

- provider-first runtime 第一版是否允许“显式 `--allow-structural-research`”这种开发开关，还是只允许测试代码路径调用 structural provider。
- citation 最终 markdown contract 采用 `file://` 链接、专用 evidence shortcode，还是两者并存一个版本周期。
- `knowledge-unit-decomposition` 是否需要单独持久化 profile 命中证据，还是复用 `knowledge_units.scope + research_cache` 即可。
- `example-tutorial` 与 `docs-guide` 的边界，第一版是否通过 reference 项目的 docs/example 命中率再细化。
