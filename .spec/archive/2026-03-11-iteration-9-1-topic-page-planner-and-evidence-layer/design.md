## Context

当前主链已经稳定收敛为 `build_module_tree -> build_contexts -> plan_pages -> render_pages`。从真实代码看：

- [`planner.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/planner.rs) 现在只会规划 `overview / architecture / workflow / module` 四类页面，模块页也仍然主要按目录模块展开。
- [`context.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/domain/context.rs) 里的 `PageContext` 只有 `facts / summary_inputs / hints / child_summaries`，没有稳定的 evidence 结构，也没有图输入结构。
- [`generation/context.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 已经能选出 `key_sources`，但这些信息目前只被折叠成字符串 facts，没有形成可持久化的来源层。
- [`sections.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 只按固定 section 模板拼正文，没有“关键来源块”或“事实驱动图块”的独立 contract。
- [`page_render.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/workflows/page_render.rs) 已经有叶子优先 LLM 增强编排，但它只能消费现有 `PageEnrichmentInput`，无法弥补 planner 主题过粗和 evidence 缺位。
- [`hierarchy.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs) 负责目录模块发现；它擅长目录边界，但不擅长把根目录高信号文件簇提升成页面主题。

迭代 9 的 reference 报告显示，当前主要差距集中在：

- 页面主题拆分不足，大量 reference 专题页被折叠到少数 overview / architecture / module 页；
- 页面缺少出处层；
- Mermaid 图表达仍然稀疏，且不够 facts-driven；
- 根级核心源码簇没有进入页面规划。

## Goals / Non-Goals

**Goals:**

- 在不绕开现有 workflow 主链的前提下，引入“专题页 planner”，让页面主题不再只等于目录模块。
- 为 `PageContext` 和 section 渲染引入稳定 evidence layer，让页面能明确落下关键来源文件和证据簇。
- 把图表达收敛为 deterministic-first：图结构来自现有 facts/graph summary，LLM 只负责解释层补强。
- 允许根级高信号文件簇进入页面规划，但不破坏当前 `ModuleTree` 作为目录/模块事实树的职责。
- 让 reference 报告和验证口径从“页数/段落密度”升级为“专题覆盖率 / evidence 落页 / 图表达覆盖”。

**Non-Goals:**

- 不追求在 9.1 直接生成接近 reference 的全部页面数量。
- 不引入新的 runtime 目录层，不新增 `.wiki/` 之外的 sidecar 产物。
- 不把 LLM 升级为 planner 主导者；页面主题发现仍以 deterministic 规则和 graph summary 为主。
- 不在 9.1 重做 query、symbol graph 或 metadata 的整体模型。

## Decisions

### 1. 专题页在 planner 层以 `TopicCandidate` 引入，而不是伪装成 `ModuleNode`

决定：

- 在 `plan_pages` 前增加专题发现输入，生成 `TopicCandidate` 集合；
- `PlannedPage` 增加 `topic` 类型和稳定 `topic_kind/topic_key` 标识；
- 专题页与模块页共存，但仍落在同一套 `PlannedPage -> PageContext -> SectionDraft -> RenderedPage` 主链中。

原因：

- 当前 [`hierarchy.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/repo/hierarchy.rs) 的 `ModuleTree` 表达的是“目录/模块事实树”，强行把 `chi` 的 `context.go / tree.go / chain.go` 之类根级文件簇塞成 `ModuleNode`，会污染模块语义，也会影响已有 module relation、query 和 metadata。
- reference 缺口本质上是“专题发现不足”，不是“目录模块不够多”。

备选方案：

- 方案 A：把根级文件簇直接提升成 synthetic module。
  否决原因：会让 `ModuleTree` 同时承载目录模块和主题页，边界变乱。
- 方案 B：保持现状，只让 LLM 把现有模块页写得更长。
  否决原因：不能解决 reference 页面被大面积折叠的问题。

### 2. evidence layer 直接进入 `PageContext`，而不是只作为 Markdown 渲染技巧

决定：

- 在 `PageContext` 中增加结构化 evidence 输入，例如 `evidence_items / evidence_groups / diagram_inputs`；
- 在 section 渲染时引入稳定 evidence block；
- evidence identity 基于 `page_id + section_id + evidence_key` 稳定生成，继续复用现有 runtime/state，而不是增加独立 evidence 数据库或 sidecar 文件。

原因：

- 当前 [`generation/context.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/context.rs) 已经能拿到 `key_sources` 和 graph hotspots，但这些信息在 [`sections.rs`](/E:/project/!byAI/spec-wiki/crates/wiki-core/src/generation/sections.rs) 里只是散落成字符串，无法形成稳定出处层。
- reference 里最有价值的并不是单纯更长的正文，而是“专题 + 来源文件 + 图”的组合。

备选方案：

- 方案 A：只在 renderer 里追加一个“来源文件”段落。
  否决原因：没有 identity，也无法作为 LLM 输入或测试目标。
- 方案 B：单独新增 `.wiki/evidence/` 产物层。
  否决原因：超出当前 runtime 边界，且会破坏现有 managed page contract。

### 3. facts-driven 图表达由 deterministic 输入主导，LLM 只负责补充说明

决定：

- 在 `build_page_context` 阶段直接生成有限类型的 `DiagramInput`；
- 9.1 只先支持三类图：模块依赖图、父子结构图、流程图；
- renderer 根据 `DiagramInput` 直接生成受控 Mermaid block，LLM 增强只能做可选解释和轻量补充，不能重写图结构。

原因：

- 当前迭代 9 的图表达主要依赖 LLM 增强输出，导致图少且不稳定。
- 现有 `GraphSummary / ModuleTree / detected_processes` 已经足够支撑基础结构图，不需要再让模型“猜图”。

备选方案：

- 方案 A：继续完全依赖 LLM 生成 Mermaid。
  否决原因：reference 报告已经证明这一策略覆盖率和稳定性都不足。
- 方案 B：完全不生成图，只继续优化正文。
  否决原因：无法追平 reference 的结构表达差距。

### 4. 根级高信号文件簇在“专题发现”阶段识别，而不是在 scanner 阶段改写 `FilePurpose`

决定：

- 不再扩大 `FilePurpose` 枚举来表达“文件簇主题”；
- 基于现有 `select_key_sources`、graph hotspots、dependency evidence 和文件共现，追加“根级核心机制簇”发现规则；
- 这些簇只作为 planner 输入，不回写 scanner/hierarchy 的基础事实。

原因：

- `FilePurpose` 解决的是单文件角色，不适合描述“`context.go + mux.go + tree.go` 共同构成路由机制主题”这种集合语义。
- 把主题判断留在 planner 输入层，更符合当前代码的职责分层。

备选方案：

- 方案 A：在 scanner 阶段直接给文件打“topic”标签。
  否决原因：会把解释层主题和事实层角色混在一起。

### 5. reference 验证从“匹配页数”升级为“专题覆盖 + evidence + 图覆盖”

决定：

- `collect-reference-project-reports.mjs` 继续保留逐项目逐文件对比；
- 新增对专题页覆盖率、evidence block 覆盖、图 block 覆盖的统计；
- 重点观测 `chi / axum / storybook / dagger / restaurant-app` 五类代表项目。

原因：

- 当前只看“matched/missing”会把很多问题都折叠成“页面数量不够”，但 9.1 的目标是结构收敛，必须能看出是哪类专题和哪类来源层仍然没落页。

### 6. debug trace 模式通过启动参数或 steering 显式开启，不污染正式协议

决定：

- `wiki-core` 增加可选 debug trace 模式，默认关闭；
- debug trace 同时支持启动参数和 steering 配置显式开启，启动参数优先；
- trace 输出只写入文件和 `stderr`，不得污染现有 stdout JSON IPC 协议；
- provider 直连时必须记录完整请求 JSON、原始响应 JSON 和解析后的 completion；Agent bridge 路径必须记录 `llm_request / llm_response / llm_unavailable` 会话消息。

原因：

- 当前 JSON IPC 的 stdout 只承载正式协议，进度虽然可见，但 provider 直连路径下看不到完整 LLM 请求/响应。
- 只靠一次性脚本做代理抓包不够稳定；真正需要的是 core 自己保留一条“可观测但默认关闭”的 debug 通道。
- Storybook 这类大仓库在 `init` 时调用多轮 `file_purpose` 和 `page_enrichment`，没有完整 trace 很难判断慢在什么阶段、模型到底收到了什么输入、返回了什么结构。

备选方案：

- 方案 A：只在测试脚本里做代理转发并打印 HTTP body。
  否决原因：只能覆盖 provider 直连，无法统一覆盖 Agent bridge，也不能保证后续每个宿主都复用同一套调试口径。
- 方案 B：把 debug 信息直接混到 stdout NDJSON 中。
  否决原因：会破坏现有 transport contract，Agent 很难继续把 stdout 当正式协议流消费。

## Risks / Trade-offs

- [专题页过多导致页面爆炸] → 先把主题发现限制在高信号文件簇、能力簇和流程主题三类，并设置每仓库/每模块上限。
- [主题页与模块页重复严重] → 在 planner 中引入覆盖去重规则，优先保留“目录页 + 高信号专题页”，而不是复制同一组 evidence。
- [evidence block 让页面显得过于机械] → 只要求每个核心 section 落 3 到 8 个关键来源，不追求完整文件枚举。
- [facts-driven 图输入不够丰富] → 9.1 先只覆盖三种稳定图类型，复杂图仍允许回退到无图。
- [根级文件簇识别误判] → 保持 deterministic 阈值和 fallback，不让专题发现反向改写模块树。

## Migration Plan

1. 先扩展 planner/context/runtime 数据模型，但保持现有页面类型仍可正常渲染。
2. 引入 `topic` 页面类型和 evidence block 后，先在 `init` 主链启用，再补齐 `update/rebuild` 的受影响集合。
3. reference 报告脚本升级后，用 `chi / axum` 做第一轮 A/B，再跑全量有 reference 的项目集。
4. 若专题页数量或 evidence 落页噪声过高，优先调 planner 阈值，不回退到“只调 prompt”的路线。

## Open Questions

- 专题页是否需要在 `wiki.metadata.json` 中单独标注 `topic_kind`，还是只通过 `page_type/scope` 表达即可。
- evidence block 是否需要独立 section，还是嵌入现有 section 尾部更合适。
- 大型仓库中专题页上限是否要跟仓库规模、语言或 graph hotspot 数量联动。
