## ADDED Requirements

### Requirement: research 结果必须驱动正式页面 section 结构
系统 MUST 让 `overview`、`architecture`、`module` 和 `topic` 页在进入 research 后由结构化 `section_plan` 驱动正式页面 section 结构，而不是继续仅把 research 结果当作摘要补丁。`section_plan` MUST 至少包含稳定 `section_key`、`section_title`、`section_summary`、`evidence_refs`、`diagram_refs` 和 `child_refs`，并继续禁止模型直接返回最终 Markdown 页面。

#### Scenario: research 结果为页面提供 section 计划
- **WHEN** 某个 `overview`、`architecture`、`module` 或 `topic` 页完成 research
- **THEN** 最终结果 MUST 包含稳定的 `section_plan`
- **THEN** renderer MUST 依据 `section_plan` 组织正文 section，而不是只把 `summary` 插入固定模板

#### Scenario: section 计划仍受 runtime contract 约束
- **WHEN** 系统把 research 结果转换为正式页面
- **THEN** 每个 section MUST 继续映射到稳定的 managed section identity
- **THEN** 模型不得直接返回最终 Markdown 文件或绕过现有 renderer

### Requirement: section 计划必须由受控证据与图输入支撑
系统 MUST 让 `section_plan` 只引用当前页面已有的 evidence、child rollup 和 deterministic diagram inputs。research 结果 MAY 重排章节顺序和表达重点，但不得凭空发明新来源、新子页或新结构图。

#### Scenario: section 计划引用现有 evidence 和子页结果
- **WHEN** `section_plan` 中某节声明 `evidence_refs` 或 `child_refs`
- **THEN** 这些引用 MUST 命中当前 dossier 或 child rollup 中已有的稳定标识
- **THEN** renderer MUST 能基于这些引用回溯到实际来源或子页摘要

#### Scenario: section 计划不得虚构结构图
- **WHEN** `section_plan` 中某节声明 `diagram_refs`
- **THEN** 每个 `diagram_ref` MUST 指向当前页面已有的 deterministic diagram input
- **THEN** 若页面不存在对应图输入，系统 MUST 回退到无图 section，而不是接受模型虚构的图结构
