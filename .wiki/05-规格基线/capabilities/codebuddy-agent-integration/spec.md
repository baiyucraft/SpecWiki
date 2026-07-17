# codebuddy-agent-integration Specification

## Purpose

定义 CodeBuddy 作为 `compatible host` 的专属接入合同。CodeBuddy 消费共享 action、Codex reference trigger 合同和 Runtime canonical DTO；本页只补充 CodeBuddy 的 skills、hooks 与 settings 机制，不定义公共宿主架构。

## Requirements

### Requirement: CodeBuddy 必须作为 compatible host 接入

系统 MUST 将 CodeBuddy 声明为 `compatible host`，不得将其描述为唯一宿主、reference host 或公共 trigger authority。

#### Scenario: 通过 bootstrap 生成 CodeBuddy 资产

- **WHEN** 用户执行 `spec-wiki init --host codebuddy`
- **THEN** 系统 MUST 写入 CodeBuddy 受管资产
- **THEN** 资产语义 MUST 服从 `host-trigger-contract` 与共享 Runtime consumption contract

### Requirement: CodeBuddy 必须公开六个 action skills

CodeBuddy MUST 在 `.codebuddy/skills/wiki-<action>/SKILL.md` 生成 `init`、`status`、`query`、`update`、`sync`、`rebuild` 六个 action skills。旧 commands 或 shared skill 不属于当前合同。

#### Scenario: 生成 action skills

- **WHEN** 系统构建 CodeBuddy bootstrap assets
- **THEN** 六个公开 action MUST 各有独立 repo-local skill
- **THEN** skill guidance MUST 从共享 policy/template 派生

### Requirement: CodeBuddy hooks/settings 必须保持次级适配边界

系统 MAY 为 CodeBuddy 生成 SessionStart、UserPromptSubmit hooks 和 settings patch。这些机制 MUST 只服务 CodeBuddy delivery，不得反向修改共享 taxonomy、Codex reference projection 或 Runtime query 语义。

#### Scenario: SessionStart 注入 orientation

- **WHEN** CodeBuddy 会话启动
- **THEN** 受管 hook MAY 注入固定 Wiki orientation context
- **THEN** context MUST NOT 声称任何 action 已选择或执行

#### Scenario: UserPromptSubmit 触发评估

- **WHEN** CodeBuddy 收到已支持的结构化 prompt event
- **THEN** hook MUST 调用共享 trigger 语义的生成投影
- **THEN** hook MUST NOT 维护私有关键词 authority 或扫描 raw JSON substring

### Requirement: CodeBuddy 必须保持 thin host boundary

CodeBuddy MUST 通过 `spec-wiki` 调用本地 Runtime，并只负责参数收集、事件解析、结果透传和必要 context delivery。它 MUST NOT 重建 Wiki 状态机、query ranking、readiness、answer 或 knowledge/page projection。

#### Scenario: Runtime 返回错误或降级

- **WHEN** Runtime 返回 typed error、blocked、stale 或 degraded response
- **THEN** CodeBuddy MUST 呈现 Runtime conclusion 与 recommended action
- **THEN** 宿主不得静默改写为可信成功

### Requirement: CodeBuddy 不得承诺未实现的 production agent bridge

CodeBuddy hooks 与 `--bridge-stdio` transport 不等同于 production host-agent LLM bridge。当前 capability MUST NOT 把 provider 缺失自动描述为 CodeBuddy bridge fallback，也不得持久化 provider session state。

#### Scenario: provider 不可用

- **WHEN** Runtime 没有可用 provider 且没有独立验证的 bridge capability
- **THEN** 系统 MUST 按 Runtime reliability contract 返回 blocked/error 或已定义回退
- **THEN** CodeBuddy MUST NOT 伪造 `llm_response` 或复用 trigger state 继续执行

