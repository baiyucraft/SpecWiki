## ADDED Requirements

### Requirement: `v0.1.0` 宿主 skill frontmatter description 必须只声明当前正式能力
`spec-wiki` 为 `Codex`、`Claude` 和 `CodeBuddy` 生成的 `wiki-query` 与 `wiki-update` skill frontmatter description MUST 只描述当前 `v0.1.0` 正式承诺的能力边界。`wiki-query` MUST 聚焦代码位置、模块/文件/符号相关命中和结构化 query 结果，MUST NOT 把 `owner`、稳定 `entrypoint` 查询或 `impact_slice` 写成已正式开放的宿主 contract；`wiki-update` MUST 明确其作用是刷新当前 index-only runtime，而不是宣称完整 knowledge/page runtime refresh。

#### Scenario: query skill 不再提前承诺 intent-aware 能力
- **WHEN** 系统为 `Codex`、`Claude` 或 `CodeBuddy` 生成 `wiki-query` skill
- **THEN** frontmatter `description` MUST 聚焦代码位置、模块/文件/符号相关命中与结构化 query 结果
- **THEN** `description` MUST NOT 把 `owner`、稳定 `entrypoint` 或 `impact` 写成当前正式能力

#### Scenario: update skill 与当前 index-only 行为对齐
- **WHEN** 系统为任一宿主生成 `wiki-update` skill
- **THEN** frontmatter `description` 与正文 action note MUST 说明它刷新的是当前 `v0.1.0` index-only runtime
- **THEN** skill MUST NOT 把该动作写成完整 knowledge/page runtime refresh

### Requirement: 宿主 hook 提示与触发词必须和 skill description 同步收口
`CodeBuddy` 的 hook 附加上下文和 `UserPromptSubmit` 触发词 MUST 与 `wiki-query/wiki-status` 的正式 description 覆盖范围保持一致。hook MAY 覆盖模块、结构、文件、符号、调用和概念相关问法，但 MUST NOT 继续把未正式开放的宿主 contract 写成硬提示。

#### Scenario: CodeBuddy hook 覆盖 query 的正式问法
- **WHEN** 系统生成 `CodeBuddy` 的 `UserPromptSubmit` hook
- **THEN** 触发词 MUST 覆盖模块、结构、文件、符号、调用、概念、query、status 与 wiki 相关问法
- **THEN** hook MUST NOT 继续把未正式开放的 owner 能力写成强承诺

### Requirement: `--bridge-stdio` 只可暴露给长流程公开动作
`spec-wiki wiki <action>` 在 `v0.1.0` 下 MUST 只允许对长流程动作暴露 `--bridge-stdio`。`query` 和 `status` 这类非流式动作 MUST NOT 接受该参数，以免制造“可桥接双向会话”的假象。

#### Scenario: query 不接受 bridge-stdio
- **WHEN** 调用方执行 `spec-wiki wiki query --bridge-stdio`
- **THEN** CLI MUST 返回明确参数错误
- **THEN** 错误消息 MUST 指出该参数只适用于长流程动作

### Requirement: CLI passthrough 必须把 runtime 逻辑失败映射为非零退出码
`spec-wiki wiki <action>` 作为 shell/CI 入口时，MUST 在保持 stdout/stderr 原样透传的前提下，根据短流程 JSON 响应或长流程 terminal event 的 `ok` 终态给出正确退出码。若 runtime 协议终态为 `ok=false`，CLI MUST 返回非零退出码，而不是只依赖子进程进程码。

#### Scenario: 短流程 ok=false 返回非零退出码
- **WHEN** `spec-wiki wiki status` 或 `spec-wiki wiki query` 收到 `ok=false` 的 JSON 响应
- **THEN** CLI MUST 保留原始 stdout/stderr
- **THEN** CLI MUST 返回非零退出码

#### Scenario: 长流程 terminal error 返回非零退出码
- **WHEN** `spec-wiki wiki init` 或 `spec-wiki wiki update` 收到 `ok=false` 的 terminal event
- **THEN** CLI MUST 保留已透传的 progress 与终态输出
- **THEN** CLI MUST 返回非零退出码
