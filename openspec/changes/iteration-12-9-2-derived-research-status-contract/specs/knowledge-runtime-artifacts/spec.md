## MODIFIED Requirements

### Requirement: `.wiki/.knowledge/**` 必须落最小正式知识产物集
系统 MUST 将 `research-summaries` 视为带正式状态合同的 derived summary snapshot，而不是只要求文件存在。每条正式 `KnowledgeResearchSummary` 除最小 identity 外，MUST 通过其 status / reason contract 校验；persist 与 restore MUST NOT 接受非法 summary snapshot。

#### Scenario: restore 拒绝非法 research summary snapshot
- **WHEN** `.wiki/.knowledge/derived/research-summaries.jsonl` 中存在 status 非法、reason contract 不一致或 identity 缺失的 research summary
- **THEN** restore MUST 显式拒绝该 snapshot
- **THEN** 系统 MUST NOT 静默继续恢复 `.wiki/.cache/**`
