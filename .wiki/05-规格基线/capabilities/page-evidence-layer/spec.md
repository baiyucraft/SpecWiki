# page-evidence-layer Specification

## Purpose
定义 Repo Wiki 页面上下文、正式渲染和 LLM 增强共享的 evidence layer 约束。
## Requirements
### Requirement: 页面上下文必须提供稳定的 evidence layer
系统 MUST 为正式页面、managed sections、renderer 和验证脚本提供统一的 evidence layer。evidence layer 除了在页面上下文中保持结构化外，还 MUST 能被 renderer 映射成最终 Markdown 中可统计、可追溯的 citation / evidence block。每条 evidence MUST 至少记录 `path`、`source_id`、`start_line`、`end_line`、`evidence_type`、`section_refs` 和 `note`，并支持最终页面中的稳定引用格式。

#### Scenario: 最终页面可追溯到结构化 evidence
- **WHEN** 系统为页面构建了结构化 evidence layer 并完成正式渲染
- **THEN** 最终 `.wiki/*.md` MUST 保留可回溯到对应 evidence identity 的 citation / evidence block
- **THEN** reference 报告和 lifecycle 验证 MUST 能从最终 Markdown 识别这些引用

### Requirement: evidence layer 必须限制粒度并保持可读性
系统 MUST 控制 evidence layer 的粒度，使其既能表达出处，又不会退化为完整文件列表转储。每个核心 section 的 evidence block MUST 优先呈现 3 到 8 个高信号来源，并允许按机制、能力或流程主题分组。若当前语言或解析能力不足以提供精确行号，系统 MUST 显式标记为 coarse span，而不是伪造精确行号。

#### Scenario: evidence block 不得退化为全量文件清单
- **WHEN** 某个页面依赖大量源码文件
- **THEN** evidence block MUST 只呈现高信号来源
- **THEN** 页面不得把所有相关文件机械展开为长列表

#### Scenario: evidence block 支持主题分组与 coarse fallback
- **WHEN** 同一页面下的 evidence 明显分属不同机制或能力簇，或部分来源只具备文件级跨度
- **THEN** 系统 MUST 允许按稳定主题分组呈现这些 evidence
- **THEN** 分组结果 MUST 能被 LLM research、renderer 和 reference 报告复用
- **THEN** 对仅具备文件级跨度的来源，系统 MUST 显式标记为 coarse span

### Requirement: evidence layer 必须支持 unit-scoped provenance 与非代码来源
系统 MUST 让 evidence layer 支持 KnowledgeUnit/section/projection-scoped provenance，并允许 docs anchors、public API surface、config surface 和 type surface 进入正式 evidence 对象。每条 evidence 除现有代码行段外，还 MUST 能标记所属 unit、section/page projection scope 和来源类别。

#### Scenario: unit 页面投影引用 docs/API/config 证据
- **WHEN** 某个 KnowledgeUnit 的页面投影主要由 docs、API 或配置入口支撑
- **THEN** evidence layer MUST 能表达这些来源
- **THEN** 页面 evidence block MUST 保留可追溯的路径、锚点或 surface 标识

#### Scenario: unit-scoped provenance 在重复生成时稳定
- **WHEN** 同一 KnowledgeUnit 页面投影的 evidence 集合未变化
- **THEN** 对应 evidence identity 与 unit/projection scope MUST 保持稳定
- **THEN** runtime 不得因正文调整而重建无关的 evidence 身份

### Requirement: evidence layer 必须支持 section-scoped citation
系统 MUST 让 evidence layer 可以按 section 作用域被 compose / renderer 精确引用，而不是只按页面作用域附着。renderer MUST 仅在命中该 `section_refs` 的 section 内输出对应 citation / evidence block，不得把整页 evidence 机械平铺到所有 section。

#### Scenario: section 只落自己的 evidence
- **WHEN** compose 计划中某个 section 只命中了部分 evidence groups
- **THEN** renderer MUST 只在该 section 输出这些 evidence
- **THEN** 未命中的 section 不得自动复用同组 evidence block

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

