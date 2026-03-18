# wiki-llm-enhancement Specification

## Purpose
定义 Repo Wiki 在 deterministic 主链之上接入可选 LLM 辅助判断、内容增强、缓存与回退语义的约束。
## Requirements
### Requirement: 系统必须在 deterministic 主链之上提供可选 LLM 辅助判断
系统 MUST 只在 deterministic 规则无法稳定收敛的场景中触发 LLM 辅助判断，而不是让 LLM 直接取代 scanner、hierarchy 或 symbol graph 的事实层。Uncertainty Gate MUST 至少覆盖文件角色兜底、顶层目录晋升临界值、`module_kind` 兜底和低置信度跨模块依赖语义这四类场景。每次辅助判断 MUST 绑定稳定 `prompt_type`、规范化输入摘要和调用预算；当 LLM 关闭、不可用、超时或预算耗尽时，系统 MUST 回退到当前 deterministic 结果。

#### Scenario: 文件角色或模块边界进入模糊区间时触发辅助判断
- **WHEN** 某文件只命中 `FilePurpose::Utility` 兜底，或某顶层目录的模块晋升评分落在临界区间
- **THEN** 系统 MUST 允许通过 LLM 请求额外判断该文件角色或目录边界
- **THEN** 若辅助判断成功，系统 MUST 只更新对应的推断结果，而不得改写已稳定的扫描事实字段

#### Scenario: 同类型不确定性候选优先批量判断
- **WHEN** 同一阶段内存在多个 schema 相同的 uncertainty gate 候选，例如多条 `file_purpose` 或多条低置信度 dependency edge
- **THEN** 系统 SHOULD 优先把这些候选按批次送入同一次 LLM 请求，而不是逐条独立请求
- **THEN** 不同阶段、不同 schema 的候选不得被揉成一个跨阶段 mega prompt

#### Scenario: LLM 不可用时回退到 deterministic
- **WHEN** 当前 workflow 未启用 LLM、Agent 不支持桥接、请求超时或本轮已达到调用上限
- **THEN** 系统 MUST 回退到 deterministic 结果继续完成 workflow
- **THEN** `init`、`update` 或 `rebuild` 不得因为辅助判断不可用而失败

### Requirement: core 必须优先使用本地 provider 直连，再回退到 Agent bridge
系统 MUST 同时支持 `wiki-core` 直连供应商 API 和通过 Agent bridge 代调用两种 LLM 路径。当 provider 直连配置完整可用时，core MUST 优先使用 provider 直连；只有在 provider 配置缺失或不可用时，系统才 MAY 使用 Agent bridge。两条路径 MUST 共享同一组 prompt 契约、输入哈希和缓存语义。

#### Scenario: provider 配置完整时优先直连
- **WHEN** 当前 workflow 的 LLM 配置中已经解析出可用的 provider 端点、认证信息和模型标识
- **THEN** core MUST 直接完成对应 LLM 请求
- **THEN** Agent bridge 即使可用，也不得抢占这次请求

#### Scenario: provider 缺失时回退到 Agent bridge
- **WHEN** 当前 workflow 未配置可用 provider，但 transport 已协商可用的 Agent bridge
- **THEN** 系统 MUST 通过 Agent bridge 发起对应请求
- **THEN** prompt 契约、输入哈希和 `llm_cache` 的 key 语义必须与 provider 直连路径一致

#### Scenario: provider 直连按配置执行有限并行增强
- **WHEN** 当前 workflow 走 provider 直连，且某一深度层存在多个未命中缓存的独立页面增强请求
- **THEN** 系统 MUST 允许这些请求在 `llm.parallel_requests` 限制内有限并行执行
- **THEN** 超出并行上限的请求 MUST 排队等待，而不是无上限并发

#### Scenario: Agent bridge 继续作为串行回退路径
- **WHEN** 当前 workflow 没有可用 provider，只能通过 Agent bridge 发起增强请求
- **THEN** 系统 MAY 继续按串行顺序请求 Agent bridge
- **THEN** 父子页顺序、缓存语义和 deterministic fallback 不得因为 bridge 路径而改变

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

### Requirement: 系统必须按叶子优先顺序生成增强内容
系统 MUST 按模块树和页面父子关系的叶子优先顺序生成增强内容。子模块页的增强摘要 MUST 先完成，父模块页和 overview / architecture / workflow 页再基于这些摘要继续生成。对同一层级的独立页面，系统 MAY 有限并行执行增强，但不同层级之间 MUST 保持父页晚于子页。

#### Scenario: 子模块先于父模块生成增强摘要
- **WHEN** 某模块页存在一个或多个子模块页
- **THEN** 系统 MUST 先完成这些子模块页的增强摘要
- **THEN** 父模块页的增强输入 MUST 包含这些子摘要

#### Scenario: overview 或 architecture 页消费顶层模块摘要
- **WHEN** 系统为 overview 或 architecture 页执行内容增强
- **THEN** 系统 MUST 基于顶层模块摘要、communities、processes 和 cycle warnings 生成正文
- **THEN** overview 或 architecture 页不得跳过这些已存在的 graph-derived 信息

#### Scenario: 同深度并行不打破叶子优先
- **WHEN** 系统对同一深度层的多个页面执行并行增强
- **THEN** 该深度层开始前，所有更深层页面的摘要 MUST 已稳定可读
- **THEN** 下一层父页 MUST 只在当前层所有并行任务完成后才开始构造增强输入

### Requirement: LLM 输出必须经过结构化校验，并在图生成失败时降级
系统 MUST 对 LLM 返回的正文和 Mermaid 图执行结构化校验。正文响应 MUST 满足约定的字段结构；Mermaid 图 MUST 受限于受支持的图类型并通过基础语法守卫。若正文或图校验失败，系统 MUST 丢弃无效部分并回退到 deterministic 文本，而不是把非法内容直接写入正式 Wiki 页面。

#### Scenario: Mermaid 图通过校验后以内联 managed section 内容写入页面
- **WHEN** 页面增强返回了受支持类型且通过语法守卫的 Mermaid 图
- **THEN** 系统 MUST 把该图以 Mermaid fenced block 的形式写入对应 managed section
- **THEN** 图内容 MUST 与该 section 共享现有 managed marker 和 section identity

#### Scenario: Mermaid 图或结构化正文校验失败时降级
- **WHEN** 增强响应缺少必需字段、JSON 解析失败，或 Mermaid 图未通过基础守卫
- **THEN** 系统 MUST 丢弃对应增强结果并回退到 deterministic section 文本
- **THEN** 页面写盘流程 MUST 继续完成

### Requirement: LLM 增强必须消费专题页与 evidence 输入
系统 MUST 让 LLM 增强消费结构化专题页输入和 evidence layer，而不是只消费扁平化 facts 字符串。对于专题页、模块页和 workflow 页，LLM 增强 MUST 优先组织已存在的主题摘要、证据分组和图输入说明，不得重新决定页面集合或篡改 evidence identity。

#### Scenario: 模块页增强消费 evidence groups
- **WHEN** 页面上下文已经提供 evidence groups 和关键来源文件
- **THEN** LLM 增强 MUST 基于这些结构化输入组织正文
- **THEN** 增强结果不得丢失或重写已有 evidence identity

#### Scenario: 专题页增强不决定页面集合
- **WHEN** planner 已经确定专题页集合
- **THEN** LLM MUST 只增强该页内容
- **THEN** 系统不得让 LLM 额外创建或删除正式页面

### Requirement: LLM 图增强必须服从 facts-driven diagram contract
系统 MUST 把 Mermaid 图结构的主导权保留给 deterministic diagram input。LLM MAY 为图补充标题、解释和轻量节点说明，但不得在输入缺失时凭空构造新的模块依赖、流程步骤或层级关系。

#### Scenario: diagram input 存在时允许 LLM 补充说明
- **WHEN** 某个页面已经拥有稳定 diagram input
- **THEN** LLM MAY 为该图补充解释性正文
- **THEN** 图的结构边和节点集合 MUST 仍来自 deterministic 输入

#### Scenario: diagram input 缺失时不得虚构图结构
- **WHEN** 当前页面没有稳定 diagram input
- **THEN** LLM MUST 回退到纯文本增强
- **THEN** 系统不得仅凭模型输出写入新的结构图

### Requirement: debug trace 模式必须记录完整 LLM 请求与响应 JSON
系统 MUST 在 debug trace 模式下记录完整的 LLM 请求与响应 JSON，覆盖 provider 直连路径和 Agent bridge 路径。provider 路径 MUST 记录发给模型的 HTTP body、原始响应 JSON 以及解析后的 completion；Agent bridge 路径 MUST 记录 `llm_request` 和回写给 core 的会话消息。

#### Scenario: provider 直连记录完整请求与响应
- **WHEN** 当前 workflow 走 provider API 直连
- **THEN** debug trace MUST 记录完整请求 JSON
- **THEN** debug trace MUST 记录原始 provider 响应 JSON 与解析后的 completion

#### Scenario: Agent bridge 记录会话消息
- **WHEN** 当前 workflow 走 Agent bridge
- **THEN** debug trace MUST 记录发出的 `llm_request`
- **THEN** debug trace MUST 记录收到的 `llm_response` 或 `llm_unavailable`

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

### Requirement: `response_format` 必须默认顶层发送，并与 prompt 内 schema 形成双保险
系统 MUST 默认在 provider 请求顶层发送 `response_format`，并继续在 prompt 中保留等价的 `response_schema` 说明。`response_format` 不应被建模为与 tools 相同的 capability 分层；其目标是为最终结构化结果提供默认约束，而不是决定是否进入 tool/session 路径。

#### Scenario: provider 请求默认同时包含 response_format 与 prompt schema
- **WHEN** core 通过 provider 直连路径请求结构化页面研究结果
- **THEN** 请求 MUST 默认在顶层包含 `response_format`
- **THEN** prompt 内容 MUST 继续保留等价的 schema 说明作为双保险

#### Scenario: transport 层拒绝 response_format 字段时允许单次容错重试
- **WHEN** 某个代理或兼容接口因 `response_format` 字段本身返回明确的 transport-level 错误
- **THEN** 系统 MAY 仅在传输层移除该字段并重试一次
- **THEN** 该重试不得被视为上层结构 contract 的语义降级

### Requirement: LLM 增强必须支持 leaf-first/source-fed/research-first compose
系统 MUST 让 LLM 增强正式支持 `leaf-first/source-fed/research-first compose` 路径。叶子 family 页、叶子模块页和高置信 topic 页 MUST 优先消费一手源码、API/config/docs surface；父页 research/compose MUST 优先消费子页结果。系统不得继续只依赖 page-level summary patch 作为主要增强模式。

#### Scenario: 叶子页优先消费一手材料
- **WHEN** 当前页面为叶子 family child、叶子模块页或高置信 topic 页
- **THEN** LLM 输入 MUST 优先包含一手源码、API/config/docs surface 和精准证据
- **THEN** 系统不得先只给模型父页摘要再反推叶子页内容

#### Scenario: 父页优先消费子页结果
- **WHEN** 当前页面为 family index、overview、architecture 或父模块页
- **THEN** LLM 输入 MUST 优先消费子页结构化结果和 digest
- **THEN** 系统不得把相同的一手材料再次作为父页主要输入

### Requirement: Research 和 Compose 层必须接入 LLM，不支持退化
系统 MUST 要求正式 runtime 的 Research 层（R1 system / R2 domain / R3 unit）和 Compose 层全部通过 provider-backed LLM 生成结构化研究材料和页面内容。系统 MUST NOT 让 `StructuralResearchProvider` 继续作为正式 `init / update / rebuild` 的默认实现。结构型 provider 仅 MAY 在测试、fixture 或显式开发模式下使用。若正式 runtime 无可用 provider，pipeline MUST 返回错误并保存检查点，而不是退化到模板填充。

#### Scenario: 正式 workflow 使用 provider-backed LLM
- **WHEN** 系统执行正式的 `init`、`update` 或 `rebuild`
- **THEN** Research 与 Compose 阶段 MUST 使用 provider-backed LLM
- **THEN** 正式 workflow 不得默认绑定 `StructuralResearchProvider`

#### Scenario: 测试或显式开发模式允许结构型 provider
- **WHEN** 系统处于测试、fixture 或显式开发调试模式
- **THEN** workflow MAY 使用 `StructuralResearchProvider`
- **THEN** 该路径不得被当作正式 runtime 成功口径

### Requirement: pipeline 必须支持中断恢复
系统 MUST 在 init / rebuild / update workflow 入口处检查 `pipeline_checkpoint` 表。若存在有效检查点且 `facts_input_hash` 匹配当前 Facts 层输出，系统 MUST 从中断处恢复，跳过已缓存的 research / compose 结果。Pipeline 正常完成后 MUST 清除检查点。

#### Scenario: 从检查点恢复并跳过已完成步骤
- **WHEN** `pipeline_checkpoint` 存在且 `facts_input_hash` 匹配
- **THEN** pipeline MUST 从 `interrupted_stage` + `interrupted_target_id` 处恢复
- **THEN** 已缓存的 `research_cache` 和 `page_drafts` MUST 被直接复用

#### Scenario: Facts 输入变化时丢弃检查点
- **WHEN** `pipeline_checkpoint` 存在但 `facts_input_hash` 不匹配
- **THEN** 系统 MUST 丢弃检查点，从头开始整个 pipeline

#### Scenario: pipeline 正常完成后清除检查点
- **WHEN** pipeline 所有阶段正常完成
- **THEN** 系统 MUST 清除 `pipeline_checkpoint` 表中的记录

### Requirement: 清理旧 LLM 子开关和废弃参数
系统 MUST 移除 `LlmConfig` 中的 `content_enrichment_enabled` / `session_enabled` / `uncertainty_gate_enabled` / `page_enrichment_max_input_tokens` / `page_enrichment_parallel_requests` / `session_max_context_tokens` / `session_max_recent_turns` / `uncertainty_gate_max_input_tokens` / `uncertainty_gate_parallel_requests` 等废弃字段。LLM 调用限制 MUST 统一为 `max_research_calls` 和 `max_compose_calls`。

#### Scenario: Steering 配置只保留简化后的 LLM 字段
- **WHEN** 用户编写 `wiki.dev.yaml` 或 `wiki.steering.yaml`
- **THEN** LLM 配置 MUST 只包含 `enabled` / `model` / `max_research_calls` / `max_compose_calls` / `cache_ttl_seconds` / `cache_mode` / `allow_mermaid` / `providers` 等字段
- **THEN** 旧的 `content_enrichment_enabled` 等字段 MUST 被忽略或报 warning

### Requirement: provider 选择必须在 workflow 入口统一决策
系统 MUST 在 workflow 入口统一选择正式 runtime 使用的 provider-backed `ResearchProvider / ComposeProvider`，并把该选择传入整条 `run_compose_pipeline()`。provider 选择、cache key、checkpoint 与 debug trace 语义 MUST 对 `init / update / rebuild` 保持一致。

#### Scenario: init / update / rebuild 共享同一 provider 选择语义
- **WHEN** 用户分别执行 `init`、`update` 和 `rebuild`
- **THEN** 这三个 workflow MUST 通过统一的 provider 选择逻辑决定 runtime provider
- **THEN** 系统不得在某个 workflow 中偷偷回退到结构型 provider 而其它 workflow 不回退

