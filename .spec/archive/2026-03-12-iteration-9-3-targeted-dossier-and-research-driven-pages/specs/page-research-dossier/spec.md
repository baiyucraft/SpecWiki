## MODIFIED Requirements

### Requirement: 系统必须在页面规划与渲染之间引入稳定的 dossier 层
系统 MUST 在 `RepoContext / ModuleContext` 与正式页面生成之间引入稳定的 `RepoDossier`、`ModuleDossier`、`TopicDossier` 和 `PageResearchResult`。dossier MUST 基于 deterministic facts、graph summary、section-scoped evidence layer、child rollup 和显式 session state 组装，而不是只把这些输入压平成字符串 facts 后直接交给 LLM。dossier 中的源码材料 MUST 以 `TargetedSnippet` 集合表达，并围绕 symbol 定义、call/process trace、entry point、关键接口/实现对和高价值 child 引用定点提取，而不是继续默认使用文件头预览。

#### Scenario: overview、architecture、module 或 topic 页构建稳定 dossier
- **WHEN** 系统为 `overview`、`architecture`、`module` 或 `topic` 页构建页面输入
- **THEN** dossier MUST 至少包含 `key_sources`、`key_symbols`、`targeted_snippets`、`cross_module_edges`、`process_candidates`、`evidence_rollup`、`diagram_rollup`、`child_rollup` 和 `session_state`
- **THEN** dossier MUST 具有稳定 identity 与 input hash，便于缓存和增量更新

#### Scenario: 同一页重复生成时 dossier identity 保持稳定
- **WHEN** 同一页面的 dossier 输入未发生变化
- **THEN** 重复执行 `init`、`update` 或 `rebuild` 时，对应 dossier 的稳定 identity MUST 保持不变
- **THEN** runtime 不得因为正文改写而重新生成一套无关 dossier identity

### Requirement: `PageResearchResult` 必须使用固定字段集合
系统 MUST 让 research session 的最终结果使用固定字段集合，而不是返回自由形状 JSON。`PageResearchResult` MUST 至少包含 `summary`、`section_plan`、`evidence_rollup`、`diagram_rollup` 和 `open_questions`；不得直接携带最终 Markdown、任意新页面建议或无约束图文本。`section_plan` 中的每个 section 条目 MUST 至少包含 `section_key`、`section_title`、`section_summary`、`evidence_refs`、`diagram_refs` 和 `child_refs`。

#### Scenario: 最终研究结果字段完整且受约束
- **WHEN** 某个 `overview`、`architecture`、`module` 或 `topic` 页完成 research session
- **THEN** 最终结果 MUST 包含 `summary`、`section_plan`、`evidence_rollup`、`diagram_rollup`、`open_questions`
- **THEN** `summary` MUST 为单段高密度摘要，`section_plan` MUST 为有限条目集合且每条都带稳定 `section_key`

#### Scenario: diagram_rollup 只能引用 deterministic 现有图输入
- **WHEN** 最终研究结果包含 `diagram_rollup`
- **THEN** 每个 diagram 条目 MUST 只引用现有 deterministic diagram inputs 对应的 `diagram_key`
- **THEN** 模型不得凭空在最终结果中新增不受控图结构

### Requirement: 父页必须消费 child rollup 和高价值一手材料
系统 MUST 让父页优先消费子页产出的结构化 rollup，而不是重复重扫扁平 facts。child rollup MUST 至少覆盖 `summary`、`section_plan_rollup`、`evidence_rollup`、`diagram_rollup` 和 `key_sources_rollup`；页面 research 裁剪时 MUST 优先保留高价值 `TargetedSnippet`、section-scoped evidence 和 child rollup，而不是优先保留低价值标签噪音。

#### Scenario: 父页消费 child rollup
- **WHEN** `overview`、`architecture`、`module` 或 `topic` 父页存在一个或多个子页
- **THEN** 父页上下文 MUST 显式吸收这些子页的 rollup
- **THEN** 父页不得只重复罗列已经在子页稳定表达过的相同 facts

#### Scenario: 高价值源码片段优先保留
- **WHEN** dossier 或 research 输入达到上下文上限，需要裁剪
- **THEN** 系统 MUST 优先保留入口文件、关键调度点、跨模块边界文件、流程终点文件、核心接口/实现对和已命中子页 section 的源码片段
- **THEN** 系统不得优先保留低价值标签噪音而裁掉这些高价值一手材料
