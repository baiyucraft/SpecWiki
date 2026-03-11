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
系统 MUST 在 `build_page_context` 之后、`build_section_drafts` 之前构造页面增强输入，并对 `overview`、`architecture`、`module` 和 `workflow` 页面执行可选的内容增强。页面增强输入 MUST 只消费稳定的 `PageContext`、graph summary、页面 hints 和子页面摘要，不得重新扫描仓库或绕过现有 planner。增强结果 MUST 能回填到现有 managed sections，并与 deterministic section 模板共享稳定 `section_id`。

#### Scenario: 模块页基于 graph facts 生成增强正文
- **WHEN** 系统为某个模块页执行内容增强，且该模块已有稳定的 `PageContext`、依赖关系和 graph hotspots
- **THEN** 增强请求 MUST 基于这些稳定输入生成模块职责、关键协作关系和实现重点说明
- **THEN** 生成结果 MUST 写回现有模块页的 managed sections，而不是新建随机 section 标识

#### Scenario: 父页消费子页摘要而不是直接重做全仓库生成
- **WHEN** 系统为父模块页、overview 页或 architecture 页执行内容增强
- **THEN** 系统 MUST 优先消费已生成的子模块摘要和 graph summary
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
