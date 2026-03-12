# agent-session-bridge Specification

## Purpose
TBD - created by archiving change iteration-9-2-dossier-session-and-llm-budget-controls. Update Purpose after archive.
## Requirements
### Requirement: 系统必须支持有边界的 research session 协议
系统 MUST 为 `module` 与 `topic` 页提供有边界的 research session，而不是把 `init` 整条链改成无限自由对话 agent。research session MUST 发生在 dossier 组装之后、正式渲染之前，最终产物 MUST 是结构化 `PageResearchResult`，而不是最终 Markdown。9.2 的首个落地路径 MUST 是 core/provider 直连路径；agent-bridge 可以在后续迭代复用同一协议。

#### Scenario: module 或 topic 页进入 research session
- **WHEN** 当前页面类型为 `module` 或 `topic`，且本轮 workflow 已开启 research session
- **THEN** 系统 MUST 先完成 deterministic dossier 组装
- **THEN** research session 的最终结果 MUST 回填为结构化 `PageResearchResult`

#### Scenario: 非试点页面不进入多轮 research session
- **WHEN** 当前页面类型为 `overview`、`architecture` 或其他未纳入试点的页面
- **THEN** 系统 MAY 继续使用现有单轮增强或纯 deterministic 路径
- **THEN** 系统不得无边界地把所有页面都升级为多轮 session

### Requirement: `agent_session_v1` 必须提供稳定会话身份与受控事件集
系统 MUST 为 research session 提供稳定 `session_id` 和受控事件集。协议 MUST 至少支持 `agent_session_start`、`agent_message`、`agent_tool_call`、`agent_tool_result`、`agent_final`、`agent_abort`。会话状态 MUST 以 `session_summary`、`recent_turns` 和 `tool_artifact_refs` 为核心，而不是无上限保留整段对话历史。9.2 的测试面 MUST 先覆盖 provider 直连路径。

#### Scenario: 会话消息与工具调用使用同一 session_id
- **WHEN** core 与 provider-tools 或 agent-bridge 交换 research session 事件
- **THEN** 同一会话中的消息、tool 调用、tool 结果和最终结果 MUST 共享稳定 `session_id`
- **THEN** 调用方 MUST 能用该 `session_id` 关联 progress、usage 和 trace

#### Scenario: 会话历史受 recent turn 上限约束
- **WHEN** 某次 research session 的 turn 数超过配置上限
- **THEN** 系统 MUST 把较早轮次压缩为 `session_summary`
- **THEN** 只保留最近有限轮原始消息，而不是无限扩张上下文

### Requirement: provider-tools 必须先使用受控只读 tool schema，后续 agent-bridge 复用同一 schema
系统 MUST 先让 provider tool-calling 路径使用受控只读 tool schema。该 schema MUST 设计成后续 agent-bridge 可复用。首批工具 MUST 围绕现有 core facts/state 提供受控读取能力，而不得暴露泛化 shell 或任意文件写入能力。

#### Scenario: provider research session 调用只读源码和图工具
- **WHEN** provider research session 需要进一步查看源码、模块上下文或 graph 邻域
- **THEN** 系统 MUST 允许通过受控工具读取 `source_snippets`、模块上下文、symbol neighbors、process trace、page children、evidence group 或 topic candidates
- **THEN** 工具结果 MUST 优先复用现有 state/cache/context 数据，而不是平行复制另一套事实层

#### Scenario: 禁止开放泛化 shell 工具
- **WHEN** provider-tools 或 agent-bridge 建立 research session
- **THEN** 系统不得默认暴露任意 shell、文件写入或脱离 core facts 的自由工具
- **THEN** tool 面 MUST 保持只读且受 schema 约束

#### Scenario: 首批只读工具名称、参数和结果保持稳定
- **WHEN** 系统在 9.2 中向 provider 或后续 agent-bridge 暴露首批只读工具
- **THEN** 工具集合 MUST 固定为 `read_source_snippets`、`get_module_context`、`get_symbol_neighbors`、`get_process_trace`、`get_page_children`、`get_evidence_group`、`search_topic_candidates`
- **THEN** 每个工具的参数和结果字段 MUST 保持稳定，不得在同一迭代内反复变更语义

#### Scenario: provider 请求按通用 tool-calling 字段发送工具定义
- **WHEN** core 通过 provider 直连路径发起 tool-calling research session
- **THEN** 工具定义 MUST 通过 provider 请求顶层 `tools` 字段发送，并与 `messages` 同级
- **THEN** 系统不得把工具定义伪装进 `input` 或 `user.content`

#### Scenario: tool 结果以连续 turn 回灌而不是回塞到 tools 字段
- **WHEN** provider 返回 tool call，且 core 已完成对应只读工具调用
- **THEN** 工具结果 MUST 作为后续会话 turn 回灌给模型
- **THEN** 系统不得把 tool 结果重新写回 `tools` 定义字段

### Requirement: provider tools 必须支持能力分层降级
系统 MUST 不假设所有 provider 都原生支持 tools。provider tools 路径 MUST 至少支持 `native_tools`、`emulated_tools` 和 `no_tools` 三种能力层级；当上层配置为 `auto` 时，系统 MUST 按 `native_tools -> emulated_tools -> no_tools` 的顺序显式降级，而不是直接失败。

#### Scenario: provider 原生支持 tools 时走 native_tools
- **WHEN** 当前 provider 能力已声明或探测为支持原生 tools
- **THEN** 系统 MUST 通过顶层 `tools` / `tool_choice` 发送工具定义
- **THEN** provider research session MUST 使用原生 tool-calling 语义

#### Scenario: provider 不支持原生 tools 时降级到 emulated_tools
- **WHEN** 当前 provider 不支持原生 tools，但支持稳定的多轮结构化 JSON 输出
- **THEN** 系统 MUST 允许通过消息内协议模拟 tool call / tool result 循环
- **THEN** 上层 tool schema 和最终 `PageResearchResult` schema MUST 保持不变

#### Scenario: provider 不适合模拟 tools 时降级到 no_tools
- **WHEN** 当前 provider 既不支持原生 tools，也不适合稳定进行模拟 tools
- **THEN** 系统 MUST 跳过 research session 的 tool loop
- **THEN** workflow MUST 回退到单轮增强或 deterministic 路径继续完成

#### Scenario: emulated_tools 的 envelope 语义对齐 native_tools
- **WHEN** 系统通过 `emulated_tools` 模式模拟 tool-calling
- **THEN** assistant 发起工具调用的 envelope MUST 尽量复用 `tool_calls[].id`、`tool_calls[].type`、`tool_calls[].function.name`、`tool_calls[].function.arguments` 这些字段语义
- **THEN** core 回灌工具结果的 envelope MUST 复用 `tool_call_id` 与工具名，而不是发明一套完全不同的调用关联字段

#### Scenario: emulated_tools 的最终结果继续返回 PageResearchResult
- **WHEN** emulated session 完成全部工具调用并准备结束
- **THEN** 模型最终 envelope MUST 返回结构化 `PageResearchResult`
- **THEN** 系统不得因为 emulated 模式而把最终结果降级成不受约束的自由文本

#### Scenario: emulated_tools 的 assistant tool-call envelope 使用固定字段形状
- **WHEN** emulated session 中模型请求调用只读工具
- **THEN** assistant 消息 MUST 使用 `type = "assistant"`、`content = null` 和 `tool_calls[]`
- **THEN** 每个 `tool_calls[]` 条目 MUST 包含 `id`、`type = "function"`、`function.name` 和 JSON 字符串形式的 `function.arguments`

#### Scenario: emulated_tools 的 tool-result envelope 使用固定字段形状
- **WHEN** core 完成某个 emulated tool call 并回灌结果
- **THEN** tool 消息 MUST 使用 `type = "tool"`、`tool_call_id`、`name`、`content`
- **THEN** `content` MUST 为 JSON 字符串，以降低不同 provider 对 object/array content 的兼容差异

#### Scenario: emulated_tools 的 final envelope 使用固定字段形状
- **WHEN** emulated session 完成全部推理并返回最终研究结果
- **THEN** 最终 envelope MUST 使用 `type = "final"` 与 `result`
- **THEN** `result` MUST 满足 `PageResearchResult` 的结构约束

