## ADDED Requirements

### Requirement: 页面增强必须优先消费 dossier 与高价值源码片段
系统 MUST 让页面增强优先消费结构化 dossier、child rollup 和高价值源码片段，而不是继续只依赖扁平 `facts / child_summaries`。页面增强或 research session 裁剪输入时 MUST 优先保留一手代码材料、evidence rollup 和关键子页结果。

#### Scenario: module 或 topic 页增强消费 source snippets
- **WHEN** dossier 已经提供关键源码片段和 symbol 级上下文
- **THEN** 页面增强或 research session MUST 基于这些片段组织正文和结构化结果
- **THEN** 系统不得只把路径字符串或标签摘要传给模型

#### Scenario: 裁剪时优先保留高价值输入
- **WHEN** 当前页面增强输入达到预算上限
- **THEN** 系统 MUST 优先保留关键源码片段、evidence rollup 和 child rollup
- **THEN** 系统不得优先保留低价值标签噪音而裁掉这些输入

### Requirement: LLM 执行路径必须统一受预算、batch 和 session contract 约束
系统 MUST 让 provider 直连与 provider tool-calling 至少共享同一套 budget、batch、session 和 structured result contract；后续 agent-bridge 也 MUST 复用同一套 contract。相同页面输入的 cache key 语义 MUST 与执行路径解耦。

#### Scenario: provider 直连与 provider-tools 共享 structured result
- **WHEN** 同一页面在 provider 直连增强与 provider tool-calling research session 之间切换
- **THEN** 两条路径 MUST 共享相同的 structured result schema 和 cache key 语义
- **THEN** 系统不得因为 provider 执行模式变化而重新定义页面研究结果结构

#### Scenario: 同类型 gate 批量后仍保持单条回退粒度
- **WHEN** 某批 uncertainty gate 的部分条目解析失败或回退
- **THEN** 系统 MUST 只回退对应条目
- **THEN** 系统不得因为批量化而丢失单条判断单元的 cache 与 fallback 粒度

#### Scenario: tool capability 降级不改变上层输入与结果 contract
- **WHEN** 某个 provider 在 `native_tools`、`emulated_tools` 与 `no_tools` 之间发生降级
- **THEN** 上层 dossier 输入、tool schema 和最终 `PageResearchResult` schema MUST 保持一致
- **THEN** 系统不得因为 provider capability 变化而重定义页面研究结果结构

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
