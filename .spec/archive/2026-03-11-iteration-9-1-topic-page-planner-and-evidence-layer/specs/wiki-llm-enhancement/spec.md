## ADDED Requirements

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
