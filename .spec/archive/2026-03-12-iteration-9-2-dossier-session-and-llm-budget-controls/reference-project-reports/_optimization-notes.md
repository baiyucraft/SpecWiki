# Reference 对比后的优化收敛

## 当前收敛

这轮 9.2 已经把 dossier、bounded research session、provider tools、budget/usage 控制接入主链，但 reference 项目集仍然能看出剩余差距主要集中在页面规划粒度、源码输入精度、research 结果消费方式和出处层密度，而不是简单的正文措辞。

## 高频观察

- 已生成专题页的项目：17/18
- 已落 evidence block 的项目：18/18
- 已落 Mermaid 图的项目：17/18
- 高频缺失专题：中间件主题(2)、核心机制主题(1)

## 下一步建议

- 不再把“继续调 prompt”作为主方向。当前质量差距的主因不在模型接入，而在给模型的源码材料太浅、planner 页粒度偏粗、以及 renderer 对 research result 的消费仍然偏模板化。
- 优先改 dossier 输入。当前 page research 仍有大量输入只拿到文件头片段，下一步应按 symbol、call trace、process、entry point 和 child page rollup 选取定点源码片段，而不是继续扩大文件级摘要。
- 优先改 research contract。下一步不应只让 LLM 返回 `summary/key_points/open_questions`，而应让 research result 能表达 section plan、每节证据引用和图计划，让章节结构由 research 驱动，而不是由固定模板主导。
- 把 research 从 `module/topic` 扩到 `overview/architecture`。当前最重要的总览页和架构页仍偏 deterministic，这会直接拉低整套 Wiki 的观感上限。
- 优先继续调 planner 阈值和 topic seed 规则，让根级机制页、流程主题页、配置与运行时页、协议/数据模型页、模块能力页覆盖更多 reference 高频主题，而不是只增加已有页面长度。
- evidence layer 下一步应补“真实 line span”和“section 内更高证据密度”，尤其是模块页和专题页，不要回退到全文文件清单。
- Mermaid 已进入主链，下一步重点是让更多页面拥有 diagram inputs，尤其来自 process graph、module graph 和 entry-point trace 的图输入，而不是放宽 LLM 自由生成结构图。

## 质量问题的直接归因

- `overview` 和 `architecture` 目前没有进入 research 主链，导致最关键页面仍偏模板总结。
- `PageResearchResult` 仍然过薄，LLM 更多是在补摘要，而不是决定页面结构。
- 当前源码片段提取方式更接近“文件预览”，而不是“围绕关键实现点取证”。
- evidence block 已进入主链，但来源粒度、line span 和页面内分布密度还不够像 reference。
- reference 缺页问题本质上还是 planner 粒度问题，不是单页内容润色问题。

## Trace 复盘：research session 与设计预期的差距

- 从 `chi` 的 debug trace 看，当前已经不是纯单轮 prompt；`page_research` 确实存在页级 bounded session，并出现了 `tool_call -> tool_result -> final` 的多轮过程。
- 但当前实现更接近“单页内的最小工具闭环”，还不是先前设想的“持续研究型 session”：
  - 会话主要依赖单页内部的 `messages + recent_turns + tool_artifact_refs`，而不是一个在多页之间持续复用的显式 session 对象。
  - 当前 trace 中各页 `session_id` 仍未形成跨页持续上下文，说明 session 状态还没有成为真正的 repo 级研究记忆。
  - 目前大多数 session 只发生 1 次工具调用后就直接收口，深度更像 tool-augmented one-shot，而不是多步研究。
- 工具表面上已经有 7 个，但实际可用性不对称：
  - `read_source_snippets` 是主要被调用的工具。
  - `get_symbol_neighbors` 当前仍返回空邻居。
  - `get_process_trace` 仍只有流程标题，`steps` 为空。
  - 其余工具更多返回摘要级材料，尚不足以支撑持续追问。
- 因为工具深度不足，模型在 trace 中几乎总是优先选择 `read_source_snippets`，这也暴露了当前 session 的真实瓶颈不在“是否支持 tool calls”，而在“工具能否提供足够深的结构化证据”。
- `read_source_snippets` 目前仍然只读取文件前 24 行，这使得多轮 session 即使真的触发，也很难追到关键函数体、关键调用点和真实流程节点。
- `PageResearchResult` 仍只返回摘要、要点、证据、图和问题；最终页面结构仍主要由 deterministic renderer 决定。因此当前 session 更像“为模板补材料”，而不是“研究结果主导成页”。

## 对 session 方向的收敛

- 下一轮如果继续沿 research session 演进，重点不应再是“让模型多聊几轮”，而应先补齐：
  - 可复用的显式 session state
  - 真正可用的 symbol/process/page 工具
  - 定点源码片段读取，而不是文件头预览
  - 能驱动 section plan 的 research result
- 在这些基础补齐之前，单纯提高 `max_turns` 或增加 tool list，不会明显改善页面质量，只会增加 token 和耗时。

## 建议的下一轮质量范围

- `targeted dossier snippets`
  围绕符号、调用链、流程和入口点提取源码片段，替代文件头预览。
- `section-plan-driven research result`
  让 research 输出章节计划、证据引用和图计划，而不是只输出摘要字段。
- `overview/architecture research`
  把总览页和架构页接入 bounded research session。
- `planner granularity by repo archetype`
  针对 Web 框架、CLI、库、全栈应用、运维仓库分别补专题页规划规则。
- `real evidence line spans`
  让 evidence/source layer 带真实路径和行号，提升出处层密度与可追溯性。
