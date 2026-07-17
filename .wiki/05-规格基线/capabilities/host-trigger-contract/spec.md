# host-trigger-contract Specification

## Purpose

定义 Codex-first 的跨宿主 Wiki action 触发合同。Codex 是唯一 `reference host`；Claude、CodeBuddy 是 `compatible host`。本 capability 只决定是否建议或选择公开 action，不执行 CLI，不重建 Runtime query 语义，也不承载 bridge 或 provider session 状态。

## Requirements

### Requirement: 宿主兼容角色必须显式且 Codex 是唯一 reference

系统 MUST 为每个受支持宿主声明 `reference` 或 `compatible` 角色。Codex MUST 是唯一 `reference host`；Claude 与 CodeBuddy MUST 是 `compatible host`。公共合同 MUST 按 shared kernel、Codex reference projection、Claude compatible projection、CodeBuddy compatible projection 的顺序设计和验证。

#### Scenario: 校验宿主注册表

- **WHEN** 系统加载完整宿主 registry
- **THEN** registry MUST 恰好包含一个 reference host
- **THEN** 该 reference host MUST 是 Codex
- **THEN** CodeBuddy hooks/settings 不得改变 shared/Codex 合同形态

### Requirement: TriggerDecision 必须使用共享三态 taxonomy

共享 trigger authority MUST 将输入归约为 `should_trigger`、`should_not_trigger` 或 `ambiguous`，并返回公开 target action、闭集 reason 和可审计 evidence。Decision identity MUST NOT 包含 host id、raw prompt、query 私参、bridge identity 或 provider session 字段。

#### Scenario: 同一请求投影到三个宿主

- **WHEN** 同一 corpus case 分别用于 Codex、Claude 与 CodeBuddy 投影
- **THEN** 三者的 decision、target action 与 reason/evidence MUST 一致
- **THEN** delivery mechanism MAY 因真实 capability 不同而不同

### Requirement: Query 和 status 可语义触发，mutation action 只允许显式触发

`query` 与 `status` MUST 使用 semantic-or-explicit policy。`init`、`update`、`sync`、`rebuild` MUST 使用 explicit-only policy。任何描述、讨论或关键词近碰撞都不得被解释为 mutation 执行请求。

#### Scenario: 用户隐式请求仓库定位

- **WHEN** 用户明确表达定位模块、调用路径或实现入口的语义需求，但未写 action 名
- **THEN** 系统 MAY 返回 `should_trigger` 与 target action `query`
- **THEN** query 执行前仍 MUST 获得非空 term

#### Scenario: 用户只讨论 rebuild

- **WHEN** 用户解释 rebuild 行为或引用包含 rebuild 的代码，但未明确要求执行
- **THEN** 系统 MUST 返回 `should_not_trigger` 或 `ambiguous`
- **THEN** 系统 MUST NOT 执行 CLI

### Requirement: Negative、ambiguous 和 invalid input 必须 fail closed

明确 opt-out、冲突意图、缺少 query term、上下文不足和 malformed input MUST NOT 直接执行 action。可恢复的 ambiguous 只能提供澄清 guidance；invalid input 不得投递 action context。

#### Scenario: 明确无需 Wiki

- **WHEN** 用户明确要求不要查询或更新 Wiki
- **THEN** 系统 MUST 返回 `should_not_trigger`
- **THEN** 所有宿主 MUST 不执行 Wiki action

#### Scenario: 请求缺少 query term

- **WHEN** 输入指向 query 但没有非空 term
- **THEN** 系统 MUST 返回 `ambiguous`
- **THEN** 宿主 MAY 请求补充 term，但 MUST NOT 调用 query CLI

### Requirement: Trigger evaluator 必须与宿主 delivery 分离

共享 evaluator MUST 是无 I/O、无 host id 的确定性合同。宿主 adapter MAY 把 decision 投影为 action context、clarification context、orientation context 或 none，但 delivery MUST NOT 修改 decision，也 MUST NOT 直接执行 CLI。

#### Scenario: CodeBuddy 处理 UserPromptSubmit

- **WHEN** CodeBuddy 收到已支持的结构化 UserPromptSubmit event
- **THEN** adapter MUST 先解析受支持字段，再调用共享 evaluator
- **THEN** adapter MUST NOT 扫描 raw serialized JSON 的 substring

#### Scenario: CodeBuddy 收到 malformed event

- **WHEN** event 不是已支持的结构化包络
- **THEN** adapter MUST fail closed 且不注入 action context
- **THEN** malformed event MUST NOT 阻断宿主会话

### Requirement: Trigger corpus 必须版本化并区分语义与机制测试

系统 MUST 维护版本化中英文 corpus，覆盖三态、显式/隐式请求、opt-out、冲突、关键词近碰撞、上下文不足与 malformed adapter input。Semantic cases MUST 对所有受支持宿主运行；adapter cases MUST 只对声明真实机制的宿主运行。

#### Scenario: 宿主没有 prompt hook

- **WHEN** Codex 或 Claude 声明不支持项目可控 prompt hook
- **THEN** conformance MUST 验证 skill guidance 和 shared decision projection
- **THEN** 测试不得伪造 hook mechanism parity 或外部模型选择准确率

### Requirement: 宿主不得重建 Runtime Query authority

Trigger 只选择或建议公开 action。Query skill 执行后 MUST 薄消费 Runtime canonical DTO，不得重建 route、ranking、readiness、trust、recommended action 或 answer，也不得通过私有 payload 模拟 richer query。

#### Scenario: Runtime 返回 stale、blocked 或 degraded

- **WHEN** query Runtime 返回非 ready 状态、typed error 或降级结论
- **THEN** 宿主 MUST 呈现 Runtime conclusion 与 recommended action
- **THEN** trigger reason MUST NOT 将其改写为可信成功

### Requirement: Trigger、bridge 和 provider session 必须正交

Trigger DTO、corpus、hook output 和 generated context MUST NOT 包含 `session_id`、`session_summary`、`recent_turns` 或 `tool_artifact_refs`。基础 host-agent bridge forwarding 属于 transport；production research/agent-session bridge 尚未启用。Provider session 只在单次 `research_page` 调用内有效，内部可以多轮。三层不得共享 identity 或 durable persistence。

#### Scenario: workflow 恢复后重新评估请求

- **WHEN** Runtime 从 checkpoint 恢复 workflow
- **THEN** provider session MUST 从 `session=None` 开始
- **THEN** 系统不得从 host trigger decision 恢复 provider turns 或 tool refs
