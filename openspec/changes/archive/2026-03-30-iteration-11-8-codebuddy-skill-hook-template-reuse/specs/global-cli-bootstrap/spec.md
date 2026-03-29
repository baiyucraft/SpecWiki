## MODIFIED Requirements

### Requirement: 宿主 bootstrap 必须把资产写入宿主真实可识别路径
`spec-wiki init` MUST 按宿主真实规则生成命令、skill、hook 或 prompt 资产，而不是把三类宿主强行写成同一路径。当前正式落点更新为：`Claude` 使用 `.claude/skills/wiki-*/SKILL.md`；`CodeBuddy` 使用 `.codebuddy/skills/wiki-*/SKILL.md`、`.codebuddy/hooks/spec-wiki/*.mjs` 与 `.codebuddy/settings.json`；`Codex` 使用 `.codex/skills/wiki-*/SKILL.md`。

#### Scenario: 为 CodeBuddy 写入项目内 skills、hooks 与 settings
- **WHEN** 用户对 `codebuddy` 执行 `spec-wiki init`
- **THEN** 系统 MUST 在 `.codebuddy/skills/` 下只生成 `wiki-init`、`wiki-status`、`wiki-query` 与 `wiki-update` 四个 action skills
- **THEN** 系统 MUST 在 `.codebuddy/hooks/spec-wiki/` 下生成 `spec-wiki` 管理的 hook 脚本
- **THEN** 系统 MUST 为 `.codebuddy/settings.json` 写入或刷新 `spec-wiki` 管理的 hook 配置

#### Scenario: Claude 与 Codex 也复用共享语义并走 repo 内 skill
- **WHEN** 用户分别对 `claude` 或 `codex` 执行 `spec-wiki init`
- **THEN** 系统 MUST 生成 `.claude/skills/wiki-*/SKILL.md` 与 `.codex/skills/wiki-*/SKILL.md`
- **THEN** Claude、Codex 与 CodeBuddy 的显式入口正文来源 MUST 仍能复用同一 workflow semantics

### Requirement: 生成出的宿主资产必须把显式入口路由回全局 CLI
本轮生成出的 skill、hook、command 与 prompt 资产 MUST 只负责引导宿主回到全局 `spec-wiki` CLI，而 MUST NOT 在宿主资产中复制 Wiki 业务规则。对正式显式入口动作，资产内容 MUST 明确映射到 `spec-wiki wiki <action>`。

#### Scenario: CodeBuddy action skill 映射到 runtime 子命令
- **WHEN** 宿主读取 CodeBuddy 的 `wiki-query` 或其他 action skill
- **THEN** 该 skill MUST 明确引导宿主调用 `spec-wiki wiki <action>`
- **THEN** 该 skill MUST 不得把 Wiki 业务语义重写在宿主层

#### Scenario: CodeBuddy hooks 不替代 runtime forwarding
- **WHEN** 宿主执行 `spec-wiki` 管理的 hooks
- **THEN** hooks MAY 注入上下文、步骤提示或护栏
- **THEN** 但 hooks MUST NOT 替代 `spec-wiki wiki <action>` 的正式 runtime forwarding 路径

### Requirement: CodeBuddy settings 写入必须保持 `spec-wiki` ownership 边界
当 `spec-wiki init --tool codebuddy` 写入 `.codebuddy/settings.json` 时，系统 MUST 只刷新 `spec-wiki` 自己管理的 hook 配置，而 MUST NOT 覆盖或删除不属于 `spec-wiki` 命名空间的用户现有配置。

#### Scenario: 重复执行时只刷新 `spec-wiki` 管理的 settings 项
- **WHEN** 用户重复执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 只更新 `spec-wiki` 自己管理的 hook 配置
- **THEN** 系统 MUST 不得覆盖无关 settings 项
