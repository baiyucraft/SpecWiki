## ADDED Requirements

### Requirement: 正式 Wiki 页面必须支持可回退的 LLM 增强 section 与图内容
系统 MUST 允许 overview、architecture、module 和 workflow 页面在现有 managed sections 中承载 LLM 增强正文和 Mermaid 图内容。增强内容 MUST 复用现有 `page_id`、`section_id` 和 managed marker，而不是创建新的运行时层或脱离页面状态表的 sidecar 文件。增强内容不可用时，系统 MUST 回退到 deterministic section body，并继续写出合法 Markdown 页面。

#### Scenario: LLM 增强不会改变 section 身份
- **WHEN** 同一页面在启用 LLM 增强的情况下被重复生成，且对应 section 的输入哈希未变化
- **THEN** 页面中的 `section_id` 和 managed marker MUST 保持稳定
- **THEN** 该 section 允许更新正文，但不得更换 section 身份

#### Scenario: 增强内容失效时页面仍写出合法 Markdown
- **WHEN** 某个页面的增强正文或 Mermaid 图未通过校验，或当前运行环境不提供增强结果
- **THEN** 系统 MUST 写出 deterministic 的 section 正文
- **THEN** 页面文件 MUST 仍是包含 managed marker 的正常 Markdown 文档
