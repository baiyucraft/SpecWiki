## MODIFIED Requirements

### Requirement: CodeBuddy hooks 只能提示入口与边界
`spec-wiki init --tool codebuddy` 生成出的 `.codebuddy/hooks/spec-wiki/*.mjs` MUST 继续保持 context/guardrail 角色。它们 MAY 提示用户何时优先使用 `wiki-status` 或 `wiki-query`，也 MAY 提醒宿主 thin host boundary；项目内 hook 命令 MUST 通过 `CODEBUDDY_PROJECT_DIR` 解析受管脚本路径；但它们 MUST NOT 引导宿主在 hook 层重建 query 结果的业务解释、页面语义或 runtime 状态机。

#### Scenario: UserPromptSubmit hook 提示 query 薄消费边界
- **WHEN** CodeBuddy 执行 `UserPromptSubmit` hook，且当前问题与 repo 结构、模块、影响面或 Wiki 查询相关
- **THEN** hook MAY 提醒宿主优先查看 `wiki-status` 或 `wiki-query`
- **THEN** hook MUST 提醒宿主直接消费 runtime 的结构化结果
- **THEN** hook MUST NOT 指示宿主自行重建 Wiki 业务状态

