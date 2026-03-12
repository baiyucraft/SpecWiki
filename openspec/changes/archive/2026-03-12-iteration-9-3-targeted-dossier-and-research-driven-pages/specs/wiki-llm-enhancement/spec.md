## MODIFIED Requirements

### Requirement: 系统必须基于稳定页面输入执行可缓存的内容增强
系统 MUST 在 `build_page_context` 之后、`build_section_drafts` 之前构造 research-driven 页面输入，并对 `overview`、`architecture`、`module` 和 `topic` 页面执行可选的内容增强。页面输入 MUST 只消费稳定的 `PageContext`、dossier、targeted snippets、graph summary、页面 hints 和子页面 rollup，不得重新扫描仓库或绕过现有 planner。增强结果 MUST 能回填到现有 managed sections，并与 deterministic section 模板共享稳定 `section_id`。

#### Scenario: overview 或 architecture 基于 research 结果组织正文
- **WHEN** 系统为 overview 或 architecture 页执行内容增强，且页面已有稳定 dossier、module graph 和 child rollup
- **THEN** 增强请求 MUST 基于这些稳定输入生成结构化 section 计划和页面定位
- **THEN** 生成结果 MUST 写回现有页面的 managed sections，而不是新建随机 section 标识

#### Scenario: 父页消费子页 rollup 而不是直接重做全仓库生成
- **WHEN** 系统为父模块页、overview 页或 architecture 页执行内容增强
- **THEN** 系统 MUST 优先消费已生成的子页 rollup 和 graph summary
- **THEN** 系统 MUST 不得要求对整个仓库重新发起一次不分层的大 prompt

### Requirement: 页面增强必须优先消费 dossier 与高价值源码片段
系统 MUST 让页面增强优先消费结构化 dossier、child rollup 和高价值源码片段，而不是继续只依赖扁平 `facts / child_summaries`。页面增强或 research session 裁剪输入时 MUST 优先保留 `TargetedSnippet`、section-scoped evidence 和关键子页结果。

#### Scenario: overview、architecture、module 或 topic 页增强消费 targeted snippets
- **WHEN** dossier 已经提供关键源码片段、symbol 级上下文和流程/入口点线索
- **THEN** 页面增强或 research session MUST 基于这些片段组织正文和结构化结果
- **THEN** 系统不得只把路径字符串或标签摘要传给模型

#### Scenario: 裁剪时优先保留高价值输入
- **WHEN** 当前页面增强输入达到预算上限
- **THEN** 系统 MUST 优先保留关键源码片段、evidence rollup 和 child rollup
- **THEN** 系统不得优先保留低价值标签噪音而裁掉这些输入

### Requirement: LLM 执行路径必须统一受预算、batch 和 session contract 约束
系统 MUST 让 provider 直连与 provider tool-calling 至少共享同一套 budget、batch、显式 session 和 structured result contract；后续 agent-bridge 也 MUST 复用同一套 contract。相同页面输入的 cache key 语义 MUST 与执行路径解耦。显式 session state MUST 在每轮 research 请求中带上稳定 `session_id`，并允许同页重复生成复用 `session_summary`、`recent_turns` 和 `tool_artifact_refs`。

#### Scenario: provider 直连与 provider-tools 共享 structured result
- **WHEN** 同一页面在 provider 直连增强与 provider tool-calling research session 之间切换
- **THEN** 两条路径 MUST 共享相同的 structured result schema 和 cache key 语义
- **THEN** 系统不得因为 provider 执行模式变化而重新定义页面研究结果结构

#### Scenario: 同页重复 research 复用显式 session state
- **WHEN** 同一页面在输入未变化的情况下重复执行 research
- **THEN** 系统 MUST 继续使用稳定 `session_id` 和压缩后的 session state 参与请求
- **THEN** 系统不得把上一轮原始长对话全文重新无上限灌入下一轮
