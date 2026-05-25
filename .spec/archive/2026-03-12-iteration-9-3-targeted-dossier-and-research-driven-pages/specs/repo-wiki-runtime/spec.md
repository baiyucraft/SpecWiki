## MODIFIED Requirements

### Requirement: 正式 Wiki 页面必须支持可回退的 LLM 增强 section 与图内容
系统 MUST 允许 `overview`、`architecture`、`module` 和 `topic` 页面在现有 managed sections 中承载 research-driven section 内容和 Mermaid 图内容。增强内容 MUST 复用现有 `page_id`、`section_id` 和 managed marker，而不是创建新的运行时层或脱离页面状态表的 sidecar 文件。增强内容不可用时，系统 MUST 回退到 deterministic section body，并继续写出合法 Markdown 页面。

#### Scenario: research-driven section 不会改变 section 身份
- **WHEN** 同一页面在启用 research-driven section 组合的情况下被重复生成，且对应 section 的输入哈希未变化
- **THEN** 页面中的 `section_id` 和 managed marker MUST 保持稳定
- **THEN** 该 section 允许更新正文，但不得更换 section 身份

#### Scenario: 增强内容失效时页面仍写出合法 Markdown
- **WHEN** 某个页面的 research 结果、section 计划或 Mermaid 图未通过校验，或当前运行环境不提供增强结果
- **THEN** 系统 MUST 写出 deterministic 的 section 正文
- **THEN** 页面文件 MUST 仍是包含 managed marker 的正常 Markdown 文档

### Requirement: runtime 必须持久化专题页与 evidence block 的稳定身份
系统 MUST 在现有页面 runtime 中持久化专题页和 evidence block 的稳定身份。专题页 MUST 与现有 `overview / architecture / module / workflow` 页面共用同一套页面状态、managed section 和缓存 contract；evidence block MUST 复用现有 page/section runtime，而不是写入新的 sidecar 层。section-scoped evidence provenance MUST 同时持久化 `source_id`、`line span` 和 `section_refs`。

#### Scenario: 专题页进入正式 runtime
- **WHEN** planner 生成专题页
- **THEN** runtime MUST 为其写入正式页面状态、input hash 和 managed sections
- **THEN** 该页面 MUST 与其他正式页面一样进入 `.wiki/*.md` 和状态库

#### Scenario: evidence block 复用现有 runtime contract
- **WHEN** 页面 section 中存在 evidence block
- **THEN** 这些 block MUST 继续受现有 managed section contract 管理
- **THEN** 系统 MUST 为 evidence provenance 写入 `source_id`、`start_line`、`end_line` 和 `section_refs`
- **THEN** 系统不得为 evidence 单独引入新的正式 runtime 目录

### Requirement: runtime 必须持久化 dossier、child rollup 与 session 摘要缓存
系统 MUST 在现有 runtime/state/cache 主链内持久化 dossier、child rollup、`section_plan` 和显式 research session state，而不是新增 `.wiki/` 之外的 sidecar 层。相关 identity/hash MUST 可被 `update`、`rebuild` 和 cache 命中逻辑复用。显式 session state MUST 至少包含 `session_id`、`session_summary`、`recent_turns` 和 `tool_artifact_refs`。

#### Scenario: dossier、child rollup 与 section_plan 进入现有 cache/state
- **WHEN** workflow 完成某个页面的 dossier 组装、child rollup 计算或 section 计划生成
- **THEN** 系统 MUST 把对应 identity、input hash 和必要摘要写入现有 runtime/cache 主链
- **THEN** 后续 `update` MUST 能基于这些缓存判断是否需要重建父页

#### Scenario: 显式 session state 复用现有 runtime contract
- **WHEN** research session 生成 `session_id`、`session_summary`、`recent_turns` 或 `tool_artifact_refs`
- **THEN** 系统 MUST 在现有 cache/state contract 内持久化这些可复用状态
- **THEN** 系统不得为此新增独立正式 runtime 目录
