## MODIFIED Requirements

### Requirement: 第一阶段 Agent 集成必须限定为 Windows 下的 CodeBuddy Agent
系统 MUST 将 CodeBuddy 视为第一批正式支持的宿主之一，并通过 `spec-wiki init` 生成的 `.codebuddy` 资产完成接入，而不再要求用户手动安装或加载独立 `codebuddy-wiki` 包。该接入范围 MUST 继续受当前 `wiki-runtime` 平台分发矩阵约束；系统 MUST NOT 因新增 skill/hook 主链就自动声称所有平台都已受支持。

#### Scenario: 通过 bootstrap 初始化 CodeBuddy 宿主
- **WHEN** 用户在当前受支持平台对项目执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 生成 `.codebuddy/skills/wiki-init/SKILL.md`、`.codebuddy/skills/wiki-status/SKILL.md`、`.codebuddy/skills/wiki-query/SKILL.md`、`.codebuddy/skills/wiki-update/SKILL.md`、`.codebuddy/hooks/spec-wiki/*.mjs` 与 `spec-wiki` 管理的 `.codebuddy/settings.json`
- **THEN** 用户随后 MUST 能在 CodeBuddy 中显式调用这些 wiki action skills

### Requirement: CodeBuddy Agent 必须通过显式 action skills 暴露当前正式支持的一组 Wiki 工具
CodeBuddy 接入在本轮 MUST 通过显式 action skills 暴露当前宿主正式支持的一组 Wiki 工具集合：`init`、`status`、`update`、`query`。

#### Scenario: 生成 CodeBuddy action skill 集合
- **WHEN** 用户对项目执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 在 `.codebuddy/skills/` 下生成 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query` 四个显式 action skill
- **THEN** 每个 action skill MUST 映射到对应的 core action

### Requirement: CodeBuddy 宿主资产必须复用共享 workflow semantics
CodeBuddy 的 action skills 与 hooks MUST 来自共享 workflow semantics 层，而不是分别维护互相漂移的独立文案模板。该共享语义层 MUST 能被其他宿主的 skill renderer 继续消费。

#### Scenario: CodeBuddy 与其他宿主共享 action workflow 语义
- **WHEN** 系统为 `codebuddy`、`claude` 与 `codex` 生成各自资产
- **THEN** 它们 MUST 能共享同一份 action metadata、CLI call、steps、guardrails 或 action notes 的单一真相来源
- **THEN** 宿主差异与 hook/settings 结构 MUST 留在各自 renderer，而不是回流成多份独立 workflow 文案

### Requirement: CodeBuddy Agent 必须保持 thin Agent 边界
CodeBuddy 接入在本轮 MUST 继续保持 thin host boundary。生成出的 CodeBuddy action skill、hooks 与全局 CLI 只负责参数收集、CLI 调用、结果解析、上下文注入和错误透传，而 MUST NOT 在宿主层实现 Wiki 业务规则。hooks MUST NOT 承担 action dispatch、bridge/session 或 runtime 状态机编排。

#### Scenario: CodeBuddy action skill 调用全局 CLI
- **WHEN** 用户在 CodeBuddy 中触发 `wiki-query` 或其他显式 action skill
- **THEN** 该 skill MUST 引导宿主调用 `spec-wiki wiki <action>`
- **THEN** CodeBuddy 宿主侧 MUST 不得自行重写 query、update 或 rebuild 的 Wiki 业务语义

#### Scenario: CodeBuddy hooks 仅做 context/guardrail
- **WHEN** CodeBuddy 执行 `spec-wiki` 管理的 `SessionStart` 或 `UserPromptSubmit` hook
- **THEN** hook MAY 注入 repo/runtime 背景与使用提示
- **THEN** hook MUST NOT 直接承担 Wiki action dispatch 或 bridge/session 编排



