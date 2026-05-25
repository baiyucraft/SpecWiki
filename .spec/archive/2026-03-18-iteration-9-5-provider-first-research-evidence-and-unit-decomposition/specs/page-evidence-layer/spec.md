## MODIFIED Requirements

### Requirement: 页面上下文必须提供稳定的 evidence layer
系统 MUST 为正式页面、managed sections、renderer 和验证脚本提供统一的 evidence layer。evidence layer 除了在页面上下文中保持结构化外，还 MUST 能被 renderer 映射成最终 Markdown 中可统计、可追溯的 citation / evidence block。每条 evidence MUST 至少记录 `path`、`source_id`、`start_line`、`end_line`、`evidence_type`、`section_refs` 和 `note`，并支持最终页面中的稳定引用格式。

#### Scenario: 最终页面可追溯到结构化 evidence
- **WHEN** 系统为页面构建了结构化 evidence layer 并完成正式渲染
- **THEN** 最终 `.wiki/*.md` MUST 保留可回溯到对应 evidence identity 的 citation / evidence block
- **THEN** reference 报告和 lifecycle 验证 MUST 能从最终 Markdown 识别这些引用

### Requirement: evidence layer 必须支持 section-scoped citation
系统 MUST 让 evidence layer 可以按 section 作用域被 compose / renderer 精确引用，而不是只按页面作用域附着。renderer MUST 仅在命中该 `section_refs` 的 section 内输出对应 citation / evidence block，不得把整页 evidence 机械平铺到所有 section。

#### Scenario: section 只落自己的 evidence
- **WHEN** compose 计划中某个 section 只命中了部分 evidence groups
- **THEN** renderer MUST 只在该 section 输出这些 evidence
- **THEN** 未命中的 section 不得自动复用同组 evidence block

## ADDED Requirements

### Requirement: diagram draft 必须与 evidence contract 一起正式落页
系统 MUST 让 Research/Compose 产出的 `DiagramSuggestion / DiagramDraft` 通过统一 contract 落成最终 Markdown 中的 Mermaid fenced block，并与对应 section 的 evidence / citation 共同成为正式验收对象。若 diagram 未通过结构守卫，系统 MUST 丢弃该图，但不得伪造图结构。

#### Scenario: diagram 通过守卫后以 Mermaid block 落页
- **WHEN** 某个 section 具备合法的 diagram draft 且通过结构守卫
- **THEN** renderer MUST 在最终 Markdown 中输出对应的 ` ```mermaid ` block
- **THEN** reference 报告脚本 MUST 能从最终页面统计到该图表达

#### Scenario: diagram 无效时只保留文本与 evidence
- **WHEN** diagram draft 未通过结构守卫或缺少必要节点/边
- **THEN** renderer MUST 丢弃该图并继续写出正文与 evidence
- **THEN** 系统不得把无效图结构写入正式页面
