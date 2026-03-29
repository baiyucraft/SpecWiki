## MODIFIED Requirements

### Requirement: CodeBuddy action skills 必须保持 skill 形态而不是 command 文案搬运
`spec-wiki init --tool codebuddy` 生成出的 `.codebuddy/skills/wiki-*/SKILL.md` MUST 是真正的 action skill 资产。它们 MUST 具备稳定 frontmatter，并以“何时使用、如何执行、哪些边界不能越过”的 workflow 说明为主，而不是把 command/prompt 文案直接搬进 `SKILL.md`。

#### Scenario: 生成 CodeBuddy action skill
- **WHEN** 系统生成 CodeBuddy action skills
- **THEN** 系统 MUST 只暴露 `.codebuddy/skills/wiki-init/SKILL.md`、`.codebuddy/skills/wiki-status/SKILL.md`、`.codebuddy/skills/wiki-query/SKILL.md` 与 `.codebuddy/skills/wiki-update/SKILL.md`
- **THEN** 系统 MUST NOT 暴露 `.codebuddy/skills/wiki-sync/SKILL.md` 或 `.codebuddy/skills/wiki-rebuild/SKILL.md`
- **THEN** 这些 action skills MUST 以 action 工作流和 guardrails 为主
- **THEN** 这些 action skills MUST 不得退化成 command 说明书或宿主层业务规则副本
#### Scenario: v0.1.0 不暴露未正式支持的 action skills
- **WHEN** 系统执行 `spec-wiki init --tool codebuddy`
- **THEN** `.codebuddy/skills/` MUST 只暴露 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update`
- **THEN** 系统 MUST NOT 生成 `wiki-sync` 或 `wiki-rebuild`
- **THEN** 若旧的 `wiki-sync` 或 `wiki-rebuild` 受管 skill 残留存在，系统 MUST 清理它们

#### Scenario: 不再生成 CodeBuddy shared skill
- **WHEN** 系统执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST NOT 生成仅承载共享背景规则的隐藏 shared skill
- **THEN** 旧的 `.codebuddy/skills/spec-wiki-runtime/`、`.codebuddy/skills/wiki-sync/` 与 `.codebuddy/skills/wiki-rebuild/` 受管资产 MUST 被清理
- **THEN** 跨 action 的入口提示与宿主边界 MUST 改由 hooks 与 action skill guardrails 承载

### Requirement: CodeBuddy 宿主必须薄消费 query 的稳定结构化字段
CodeBuddy 的 `wiki-query` skill 在本轮 MUST 直接消费 `wiki-runtime query` 已承诺的稳定结构化字段来回答或转述结果，包括 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 与 `provenance_summary`。宿主 MUST NOT 在 skill、hook 或其他 TS 资产中重建独立的 Wiki 状态机、页面语义或 knowledge/page projection。

#### Scenario: wiki-query 消费稳定 query 字段
- **WHEN** 用户在 CodeBuddy 中触发 `wiki-query`
- **THEN** 该 skill MUST 引导宿主优先依据 `query_mode`、`query_trust`、`recommended_action`、`matched_pages` 与 `provenance_summary` 理解结果
- **THEN** 宿主 MAY 对这些字段做面向用户的薄转述
- **THEN** 宿主 MUST NOT 额外推导不存在的 page、state 或 knowledge 语义

#### Scenario: query 结果不足时不在宿主层补写语义
- **WHEN** `wiki-runtime query` 返回的结构化结果不足以直接回答用户问题
- **THEN** CodeBuddy MUST 明确说明结果覆盖不足或建议下一步 action
- **THEN** CodeBuddy MUST NOT 以宿主自定义规则补写新的 Wiki 业务结论




