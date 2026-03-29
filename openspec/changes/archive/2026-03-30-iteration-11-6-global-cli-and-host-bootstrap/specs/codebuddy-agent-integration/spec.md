## MODIFIED Requirements

### Requirement: 第一阶段 Agent 集成必须限定为 Windows 下的 CodeBuddy Agent
系统 MUST 将 CodeBuddy 视为第一批正式支持的宿主之一，并通过 `spec-wiki init` 生成的 `.codebuddy` 资产完成接入，而不再要求用户手动安装或加载 `codebuddy-wiki` 包。该接入范围 MUST 继续受当前 `wiki-runtime` 平台分发矩阵约束；系统 MUST NOT 因新增宿主 bootstrap 就自动声称所有平台都已受支持。

#### Scenario: 通过 bootstrap 初始化 CodeBuddy 宿主
- **WHEN** 用户在当前受支持平台对项目执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 生成 `.codebuddy/skills/wiki-init/SKILL.md`、`.codebuddy/skills/wiki-status/SKILL.md`、`.codebuddy/skills/wiki-query/SKILL.md`、`.codebuddy/skills/wiki-update/SKILL.md`、`.codebuddy/hooks/spec-wiki/*.mjs` 与 `spec-wiki` 管理的 `.codebuddy/settings.json`
- **THEN** 用户随后 MUST 能在 CodeBuddy 中识别并调用这些显式 action skills

#### Scenario: 当前平台不受支持时不虚假宣称可用
- **WHEN** 用户尝试在当前 `wiki-runtime` 平台分发矩阵之外使用 CodeBuddy bootstrap
- **THEN** 系统 MUST 明确说明当前平台不在已支持范围内
- **THEN** 系统 MUST 不得因为 `.codebuddy` 目录已生成就声称 runtime 已可正常工作

### Requirement: CodeBuddy Agent 必须暴露当前正式支持的一组 Wiki 工具
CodeBuddy 接入在本轮 MUST 通过 `.codebuddy/skills/wiki-*/SKILL.md` 暴露 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query` 四个显式 action skills，并与当前宿主正式支持的 core action 一一对应。

#### Scenario: 生成 CodeBuddy action skill 集合
- **WHEN** 用户对项目执行 `spec-wiki init --tool codebuddy`
- **THEN** 系统 MUST 在 `.codebuddy/skills/` 下生成 `wiki-init`、`wiki-status`、`wiki-update`、`wiki-query` 四个显式 action skills
- **THEN** 系统 MUST NOT 继续生成 `wiki-sync` 或 `wiki-rebuild`
- **THEN** 每个 action skill MUST 映射到对应的 core action

### Requirement: CodeBuddy Agent 必须保持 thin Agent 边界
CodeBuddy 接入在本轮 MUST 继续保持 thin host boundary。生成出的 CodeBuddy action skill、hook、settings 与全局 CLI 只负责参数收集、CLI 调用、结果解析、上下文注入和错误透传，而 MUST NOT 在宿主文案或 TS 层实现 Wiki 业务规则。CodeBuddy 的宿主差异实现 MUST 收敛在宿主层，而 MUST NOT 回流成 `init` 编排层或 runtime forwarding 层的一部分。

#### Scenario: CodeBuddy action skill 调用全局 CLI
- **WHEN** 用户在 CodeBuddy 中触发 `wiki-query` 或其他显式 action skill
- **THEN** 该 action skill MUST 引导宿主调用 `spec-wiki wiki <action>`
- **THEN** CodeBuddy 宿主侧 MUST 不得自行重写 query、update 或 rebuild 的 Wiki 业务语义

#### Scenario: CLI 或 runtime 返回错误
- **WHEN** `spec-wiki` CLI 或 `wiki-runtime` 返回错误
- **THEN** CodeBuddy 接入 MUST 向宿主返回明确错误信息
- **THEN** 宿主侧 MUST 不得静默改写 Wiki 业务状态

### Requirement: CodeBuddy 集成必须继续保留长流程 progress 与终态合同
即使正式入口从 `codebuddy-wiki` JS 工具包切换为显式 action skills 与全局 `spec-wiki` CLI，CodeBuddy 集成在 `v0.1.0` 仍 MUST 对 `init` 和 `update` 保留当前 `wiki-runtime` 的 `progress / result / error` 终态合同。但该终态在本轮正式定义为“index-only runtime 已完成”，而不是“完整 knowledge/page runtime 已完成”。

#### Scenario: CodeBuddy index-only 长流程命令保留 progress 链
- **WHEN** 用户在 CodeBuddy 中执行 `wiki-init` 或 `wiki-update`
- **THEN** CodeBuddy 集成 MUST 通过共享 `spec-wiki wiki <action>` 路径保留 runtime 的 `progress / result / error` 输出链
- **THEN** 集成面 MUST 不得因为从 JS 工具改成 skill 资产就吞掉中间进度或最终终态
- **THEN** 集成面 MUST 不得把 index-only 终态错误描述为完整知识页生成已经完成

#### Scenario: 宿主不消费中间进度时仍返回终态
- **WHEN** CodeBuddy 当前只消费最终结果，而未单独展示中间 progress
- **THEN** CodeBuddy 集成 MUST 仍然完整完成长流程调用
- **THEN** CodeBuddy 集成 MUST 继续返回最终结果或错误，而不是因 progress 未展示而提前中断

### Requirement: CodeBuddy 集成必须继续保留可选 LLM / session bridge 路径
`v0.1.0` 不再把需要完整 knowledge/page runtime 或宿主侧 bridge/session 协作的 workflow 当成正式发布承诺。CodeBuddy 集成在本轮 MUST 正式保证的是 index-only `init / update / query` 可用；对于仍保留在 CLI 层的 `sync / rebuild` 或需要 bridge 的路径，集成 MAY 不暴露对应宿主入口，但 MUST 不得向宿主伪装为本轮已正式支持。

#### Scenario: CodeBuddy 正式保证 index-only `query`
- **WHEN** 用户在 CodeBuddy 中执行 `wiki-query`
- **THEN** CodeBuddy 集成 MUST 正式返回 index-first 的结构化查询结果
- **THEN** 该能力 MUST 不依赖 knowledge/page projection 已经完整生成

#### Scenario: 非本轮正式承诺动作不作为宿主显式入口
- **WHEN** 用户对项目执行 `spec-wiki init --tool codebuddy`
- **THEN** 集成面 MUST NOT 继续生成 `wiki-sync` 或 `wiki-rebuild`
- **THEN** 但 CLI 层仍 MAY 保留这些 runtime actions 供后续演进和调试
