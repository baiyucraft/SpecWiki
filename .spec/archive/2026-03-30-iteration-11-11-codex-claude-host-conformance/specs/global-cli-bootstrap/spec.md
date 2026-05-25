## MODIFIED Requirements

### Requirement: 宿主 bootstrap 必须把资产写入宿主真实可识别路径
`spec-wiki init` MUST 按宿主真实规则生成命令、skill、hook 或 prompt 资产，而不是把三类宿主强行写成同一路径。当前正式落点更新为：`Claude` 使用 `.claude/skills/wiki-*/SKILL.md`；`CodeBuddy` 使用 `.codebuddy/skills/wiki-*/SKILL.md`、`.codebuddy/hooks/spec-wiki/*.mjs` 与 `.codebuddy/settings.json`；`Codex` 使用 `.codex/skills/wiki-*/SKILL.md`。

#### Scenario: 为 Claude 写入项目内 skill
- **WHEN** 用户对 `claude` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.claude/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 继续生成 `sync` 或 `rebuild` 的显式入口
- **THEN** 系统 MUST NOT 再保留 Claude command 形态

#### Scenario: 为 Codex 写入项目内 skill
- **WHEN** 用户对 `codex` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.codex/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 继续生成 `wiki-sync` 或 `wiki-rebuild`
- **THEN** 系统 MUST NOT 再保留 Codex prompt 形态

#### Scenario: 重新 bootstrap 时清理旧的非正式入口资产
- **WHEN** 用户重新执行 `spec-wiki init`，且宿主目录里残留旧的 Claude command、旧的 Codex prompt，或旧的 Claude/Codex shared skill 目录
- **THEN** 系统 MUST 清理这些旧的受管资产
- **THEN** 系统 MUST 不得误删不属于 `spec-wiki` 管理范围的其他宿主文件

### Requirement: Claude / Codex 宿主入口正文必须与 CodeBuddy action skill 使用同构骨架
Claude skill 与 Codex skill MUST 与 CodeBuddy action skill 使用同一组宿主正文 section 骨架：`Purpose`、`Inputs`、`Steps`、`Output`、`Action Notes` 与 `Guardrails`。

#### Scenario: Claude / Codex skill 使用统一 section 骨架
- **WHEN** 系统生成 Claude skill 或 Codex skill
- **THEN** 正文 MUST 使用 `Purpose`、`Inputs`、`Steps`、`Output`、`Action Notes` 与 `Guardrails`
- **THEN** 正文 MUST NOT 再使用一套与 CodeBuddy 漂移的独立 section 结构

### Requirement: Claude / Codex 宿主入口必须直接消费 query 的稳定结构化字段
Claude skill 与 Codex skill 在 `wiki-query` 场景 MUST 直接引导宿主消费 `wiki-runtime query` 已承诺的稳定结构化字段，包括 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 与 `provenance_summary`。宿主 MUST NOT 在 skill 文案中重建独立的 Wiki 状态机、页面语义或 knowledge/page projection。

#### Scenario: Claude / Codex query 入口声明薄消费边界
- **WHEN** 系统生成 Claude 的 `wiki-query` skill 或 Codex 的 `wiki-query` skill
- **THEN** 该入口 MUST 明确说明回答应直接基于 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 与 `provenance_summary`
- **THEN** 宿主 MAY 对这些字段做面向用户的薄转述
- **THEN** 宿主 MUST NOT 额外推导不存在的 page、state 或 knowledge 语义

#### Scenario: query 结果不足时不补写宿主语义
- **WHEN** `wiki-runtime query` 返回的结构化结果不足以直接回答用户问题
- **THEN** Claude / Codex 宿主入口 MUST 明确说明结果覆盖不足或建议下一步 action
- **THEN** 宿主 MUST NOT 以 skill 侧规则补写新的 Wiki 业务结论
