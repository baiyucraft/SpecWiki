# agent-session-bridge Specification

## Purpose

定义 provider request-local research session 与 host-agent bridge 的边界。基础 `llm_request / llm_response / llm_unavailable` forwarding 已实现；当前正式 research 主路径是 provider-backed request-local execution，production `research_page` 与多轮 agent-session bridge 尚未启用。本 capability 不授权 durable session resume，也不参与 host trigger decision。

## Requirements

### Requirement: Provider research session 必须严格 request-local

Provider session MAY 在单次 `research_page` 调用内执行有限的 message/tool loop，并以结构化 `PageResearchResult` 结束。临时 correlation id、turn 和 tool-call state 只能在该调用内存在。

#### Scenario: 单次 request 内执行 tool loop

- **WHEN** provider 为某个 research request 返回受控 tool call
- **THEN** Runtime MAY 在同一 request 内关联 tool result 与后续 turn
- **THEN** request 完成或失败后 MUST 丢弃临时 session state

### Requirement: Durable provider session state 必须被禁止

系统 MUST NOT 将 `session_id`、`session_summary`、`recent_turns`、`tool_artifact_refs` 或原始对话写入 checkpoint、cache、`.wiki/`、metadata 或宿主资产。Workflow resume MUST 从 `session=None` 开始，并从 durable facts/dossier/context 重建请求。

#### Scenario: 从 checkpoint 恢复 workflow

- **WHEN** workflow 在进程重启后从 checkpoint 恢复
- **THEN** 新 provider request MUST 从 `session=None` 开始
- **THEN** Runtime MUST NOT 恢复上一进程的 turns、tool refs 或 provider conversation identity

### Requirement: Provider tools 必须只读且受 schema 约束

Provider request MAY 使用受控只读工具读取已有 facts、source snippets、module context、symbol graph、process trace、page children 或 evidence。系统 MUST NOT 默认暴露泛化 shell、任意文件写入或绕过 core facts 的工具。

#### Scenario: Provider 请求源码证据

- **WHEN** research request 需要补充源码或图关系
- **THEN** Runtime MUST 只调用已声明的只读 tool schema
- **THEN** tool result MUST 来自现有 facts/state/context，而不是建立平行事实层

### Requirement: Tool capability 降级不得改变结构化结果合同

Runtime MAY 根据 provider 能力选择 native tools、emulated tools 或 no-tools 路径，但所有路径 MUST 保持结构化 research result、budget、cache key 与可靠性合同。降级不得产生 durable session resume。

#### Scenario: Provider 不支持 tools

- **WHEN** 当前 provider 无可用 tool-calling 能力
- **THEN** Runtime MAY 使用已验证的 no-tools request 或按 reliability contract 阻断
- **THEN** 系统不得通过持久化旧 session 弥补能力缺失

### Requirement: Production research bridge 必须独立协商和验证

基础 host-agent bridge forwarding 属于已实现 transport capability，但 active path 只接受 `llm_response / llm_unavailable`，不启用完整 agent-session events；production `research_page` 选择 bridge 时当前保持 blocked。只有独立实现、capability negotiation、协议测试和 production verification 证据齐备后，production research bridge 才 MAY 启用。Provider 缺失 MUST NOT 自动推导该路径可用；CodeBuddy hooks/settings 与 `--bridge-stdio` 也不自动证明 production research bridge 可用。

#### Scenario: 未协商 bridge

- **WHEN** Runtime 没有可用 provider，且调用方未协商已验证 bridge
- **THEN** Runtime MUST 按正式 reliability contract 返回 blocked/error 或已定义回退
- **THEN** 宿主不得伪造 bridge response

### Requirement: Trigger 与 session/bridge identity 必须完全分离

Host trigger 只选择或建议公开 action。Trigger decision、corpus 和 hook output MUST NOT 携带 provider session 或 bridge identity；bridge/provider 也不得把一次 action selection 当作可恢复会话。

#### Scenario: Trigger 建议 query

- **WHEN** host trigger 返回 target action `query`
- **THEN** 该 decision MUST 不包含 provider turns、tool refs 或 bridge id
- **THEN** 后续 Runtime request MUST 独立建立自己的 request-local execution context
