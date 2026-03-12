## Context

9.2 之后，provider-first 的 bounded research session、tool schema、budget/usage 与 cold/warm cache 已经跑通，但 reference 对比和 trace 复盘表明页面质量仍然受限于当前主链的几个结构性问题：

- [generation/context.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 中的 `ModuleDossier` / `TopicDossier` 仍主要由 `key_sources + read_source_snippet()` 组装，`read_source_snippet()` 只读取文件前 24 行，且 evidence rollup 仍写 `start_line=0 / end_line=0`。
- [page_render.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 只让 `module/topic` 页进入 research session，`overview/architecture` 仍由 deterministic section 模板主导。
- [sections.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 虽然会消费 `research_result.summary`、`key_points`、`diagram_rollup`，但最终页面结构仍主要由固定模板决定，research 更像“为模板补材料”，不是“研究结果主导成页”。
- [llm/mod.rs](E:/project/!byAI/spec-wiki/crates/wiki-core/src/llm/mod.rs) 中的 tools 已经存在，但可用性不对称：`read_source_snippets` 是唯一真正深读代码的工具，而 `get_symbol_neighbors` 仍返回空邻居、`get_process_trace` 只有标题没有 step。

9.2 归档中的优化结论已经明确：下一轮不该继续把重点放在 prompt 或 provider bridge 上，而应收敛到“精准输入、research 驱动结构、planner 粒度和真实出处层”。

参考实现的源码也支持这个判断：

- CodeWiki 在 [prompt_template.py](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/prompt_template.py) 和 [read_code_components.py](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/agent_tools/read_code_components.py) 中，会把叶子模块的真实组件源码直接给到 LLM，再由 [documentation_generator.py](E:/project/!byAI/spec-wiki/tmp/upstream/codewiki/codewiki/src/be/documentation_generator.py) 按 `leaf -> parent -> repo overview` 顺序生成文档。
- deepwiki-rs 在 [code_analyze.rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/preprocess/agents/code_analyze.rs) 中，先做静态分析，再把依赖代码片段与 source summary 送入 research；[overview_editor.rs](E:/project/!byAI/spec-wiki/tmp/upstream/deepwiki-rs/src/generator/compose/agents/overview_editor.rs) 则显式消费前置 research 结果，而不是直接从 facts 写 overview。
- GitNexus 在 [process-processor.ts](E:/project/!byAI/spec-wiki/tmp/upstream/GitNexus/gitnexus/src/core/ingestion/process-processor.ts) 中先把 entry point、trace、step 序列做实，再把这些过程事实交给消费层。这说明 9.3 更该先补图/流程/片段事实，而不是先补更长的自由对话。

## Goals / Non-Goals

**Goals:**

- 让 dossier 输入从“文件预览”升级为“围绕关键实现点的精准片段集合”。
- 让 `PageResearchResult` 从摘要补丁升级为可驱动页面 section 结构的稳定 contract。
- 把 `overview/architecture` 纳入 bounded research 主链，让最关键页面也能消费 research 结果。
- 按仓库 archetype 扩展 planner，让 reference 高频缺失专题能进入正式页面集合。
- 让 evidence/source layer 携带真实 line span 和 section 级出处引用，提升可追溯性与 reference 对齐度。

**Non-Goals:**

- 不在 9.3 提前实现 CodeBuddy Agent 正式 bridge；该范围仍留在迭代 11。
- 不引入自由 Markdown 输出或让 LLM 直接落最终页面文件。
- 不把 query/RAG/embedding 体系并入 9.3；这仍属于迭代 10 范围。
- 不为了“更多页数”简单放宽 planner 阈值；9.3 要求的是按 archetype 的稳定拆页，而不是机械增页。

## Decisions

### 决策 1：引入 `TargetedSnippet` 管线，替代文件头预览

9.3 将在 dossier 组装前增加一层 `TargetedSnippet` 选择管线，按以下来源提取真实源码片段：

- `key_symbols` 的定义行段
- `CALLS / IMPORTS / process` 相关的关键节点行段
- 入口点、路由注册点、核心接口/实现对
- `child_rollup` 中命中的高价值来源
- 现有 evidence group 中的关键项

每个片段都必须带稳定 `source_id / path / start_line / end_line / snippet_kind / score`，由 score 和 phase budget 控制进入 dossier 与 tool 读取集合。

选择这一方案，而不是继续用全文件或文件头预览，原因是：

- 当前 `take(24)` 的文件头预览无法覆盖真正关键的 handler、route 注册点、状态机和流程终点。
- CodeWiki 的叶子页会直接读取组件源码；deepwiki-rs 的 `code_analyze` 也会把依赖代码片段送入 AI。9.3 需要跟这一层靠拢，但仍保持 deterministic 选择和预算控制。
- 全文件直接喂入虽然简单，但 token 成本和噪音都太高，不适合当前 provider-first 的预算模型。

备选方案：

- 继续使用文件头预览：实现成本低，但已经被 9.2 的 trace 明确证明不足。
- 直接给完整文件：上下文成本过高，且会放大参考项目中大型仓库的冷启动时延。

### 决策 2：把 `PageResearchResult` 升级为 `summary + section_plan + evidence_rollup + diagram_rollup + open_questions`

9.3 不再沿用 9.2 的 `summary / key_points / evidence_rollup / diagram_rollup / open_questions` 作为最终研究结构，而是改为：

- `summary`
- `section_plan[]`
- `evidence_rollup`
- `diagram_rollup`
- `open_questions`

其中 `section_plan[]` 为稳定数组，每项至少包含：

- `section_key`
- `section_title`
- `section_summary`
- `evidence_refs`
- `diagram_refs`
- `child_refs`

renderer 继续负责正式 Markdown 落盘，但 section 的顺序、侧重点和支撑出处将由 `section_plan` 主导，而不是完全由固定模板决定。

这样做而不是让 LLM 直接输出 Markdown，原因是：

- runtime 仍需要稳定 `section_id`、managed marker 和增量更新边界。
- 直接输出整页 Markdown 会破坏当前 deterministic renderer、page cache 和 user section 保留语义。
- 目前质量不足的关键是“页面结构不受 research 主导”，不是“LLM 没有足够写作自由”。

备选方案：

- 保持 summary-only：会继续让 renderer 成为主导，无法解决结构层差距。
- 直接输出整页 Markdown：会让 runtime 与 cache contract 变脆，且不符合当前主链边界。

### 决策 3：把 research 扩到 `overview/architecture`，但保持 `workflow` 继续 deterministic

9.3 将让 `overview / architecture / module / topic` 四类页面进入同一条 bounded research 主链；`workflow` 暂时保持 deterministic + facts-driven diagram，不在 9.3 提前扩展。

原因：

- 从 reference 对比看，最影响整体观感的是 `overview/architecture` 仍是模板页。
- `workflow` 的质量更多受 process graph 完整度影响；在 process trace 工具和 targeted snippets 尚未完全做实之前，先扩 `workflow` 的收益不如 `overview/architecture` 高。
- 这样可以控制 9.3 的 session 成本和 scope，不把所有页面类型一起拉进 research。

备选方案：

- 只保留 `module/topic`：无法解决最核心的总览页质量问题。
- 全页面全部 research：会明显扩大冷启动成本，也会把问题扩散到尚未准备好的 `workflow` 页。

### 决策 4：planner 继续 deterministic，但按 repo archetype 扩展专题页族

9.3 不会让 LLM 决定页面集合；planner 仍保持 deterministic，只是在现有 `root topic / module capability / process topic` 基础上，新增按仓库 archetype 的专题页族与抑制规则，例如：

- Web 框架/服务：路由链路页、请求处理链页、中间件页
- CLI：命令树页、配置解析页、执行入口页
- Library/SDK：协议/数据模型页、核心 API/实现页
- 全栈/应用：配置与运行时页、模块协作页、数据流页
- 运维/脚本仓库：部署流程页、环境与镜像页、CI/CD 页

这些规则将建立在当前 repo facts、entry point、manifest、process、route/call 线索之上，而不是放宽为自由主题生成。

原因：

- reference 的“缺页”主要是稳定主题没有被 planner 承载，而不是单页写得不够长。
- `topic-page-planner` 已经有 identity / dedupe contract，适合继续扩，而不是重起一条 LLM 决定页集合的链。

备选方案：

- 简单调大现有 topic 阈值：会增加噪音页和重复页。
- 让 LLM 直接规划页面集合：不符合现有 deterministic 主链边界。

### 决策 5：evidence/source layer 必须升级为真实 line span 与 section-scoped provenance

9.3 将让 evidence/source layer 不再只记录文件路径和空行号，而是记录：

- `path`
- `source_id`
- `start_line`
- `end_line`
- `evidence_type`
- `section_refs`
- `note`

其中 section-scoped provenance 会同时服务：

- dossier 取证
- `section_plan.evidence_refs`
- 正式页面 evidence block
- reference 报告与验证脚本

原因：

- 9.2 里 evidence block 已经有形，但由于 line span 仍为空，无法形成 reference 那种“出处密度”。
- 一旦 `section_plan` 存在，evidence 也必须变成 section 级可引用对象，否则 renderer 无法准确组装正文支撑材料。

备选方案：

- 继续只保留页面级 evidence group：不足以支持 section-plan 驱动渲染。
- 把证据直接展开成全文文件列表：会重新回到高噪音页面。

### 决策 6：session state 变成显式对象，但 9.3 只做“同页复用”，不做跨页长会话

9.3 会把 `session_id / session_summary / recent_turns / tool_artifact_refs` 从“loop 内部临时状态”升级为真正的显式 session state，并确保：

- 每轮 research 请求都携带稳定 `session_id`
- 同一页面 repeated research 可复用上轮压缩状态
- trace、cache 和 runtime 中都能看到同一对象

但 9.3 不做跨页长会话，也不把一个页面的原始 turn 直接灌给另一个页面。跨页复用继续通过 `child_rollup / dossier / evidence_rollup` 这些结构化对象完成。

原因：

- 9.2 trace 里虽然有多轮，但 `session_id` 仍没有成为真正的显式对象，导致 session 更像“messages loop”。
- 跨页长会话会立刻带来 token 失控和语义污染，不适合在质量问题还没收住时提前做。

## Risks / Trade-offs

- [风险：targeted snippets 选择错误，仍然抓不到关键实现] → 通过 `snippet_kind + score`、reference 项目回归和 trace 诊断输出持续校正选择器。
- [风险：section_plan 不稳定导致 section identity 抖动] → `section_key` 必须来自受控集合或稳定 slug，renderer 继续以 `page_id + section_key` 派生 `section_id`。
- [风险：overview/architecture research 扩展后 token 成本上升] → 沿用 phase budget、cold/warm 对照和同页 session reuse；`workflow` 暂不纳入 9.3。
- [风险：archetype planner 过度增页] → planner 继续保留 suppression/merge 规则，并以 reference 高频缺失专题为准，不允许仅凭阈值放宽机械增页。
- [风险：真实 line span 在 unsupported language/弱符号项目上不足] → 允许按 evidence source 降级为文件级，但必须显式标记为 coarse span，而不是伪造精确行号。

## Migration Plan

1. 升级 dossier / evidence / research result 的数据结构和 hash 语义。
2. 调整 render 主链，让 `overview/architecture` 也进入 research，并消费 `section_plan`。
3. 扩展 planner archetype 规则与 topic seed 发现。
4. 升级 runtime/cache 写盘结构，移除 9.2 的 summary-only research result 假设。
5. 更新 reference 报告与 lifecycle 验证脚本，增加 section-plan、精准 evidence 和 archetype coverage 指标。

本项目当前处于测试开发阶段，不保留旧版结构兼容。旧的 summary-only `PageResearchResult`、空行号 evidence rollup 和文件头预览路径可在 9.3 内直接替换。

## Open Questions

- `workflow` 页是否要在 9.3 尾声一并纳入 research，还是继续等 process trace 工具做实后放到后续迭代。
- `section_plan.section_key` 是完全自由 slug，还是按 page type 限定在受控 section 槽位集合内；这会影响 section identity 稳定性与页面多样性上限。
- archetype 检测第一版是否需要显式写入 steering/runtime，还是先作为 planner 内部派生结果存在。
