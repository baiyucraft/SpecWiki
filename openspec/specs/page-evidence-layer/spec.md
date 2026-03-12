# page-evidence-layer Specification

## Purpose
定义 Repo Wiki 页面上下文、正式渲染和 LLM 增强共享的 evidence layer 约束。
## Requirements
### Requirement: 页面上下文必须提供稳定的 evidence layer
系统 MUST 为正式页面和 managed sections 提供结构化 evidence layer。evidence layer MUST 至少包含关键来源文件、证据分组、真实来源跨度和稳定 evidence identity，而不是只把这些信息压平成字符串 facts。每条 evidence MUST 至少记录 `path`、`source_id`、`start_line`、`end_line`、`evidence_type`、`section_refs` 和 `note`，并可同时服务 renderer、runtime、LLM research 输入和 reference 验证。

#### Scenario: 页面上下文输出带行号的关键来源文件块
- **WHEN** 系统为 `overview`、`architecture`、模块页或专题页构建页面上下文
- **THEN** 页面上下文 MUST 提供带 `path + line span` 的稳定关键来源集合
- **THEN** renderer MUST 能把这些来源文件落为正式页面中的 evidence block，并保留可追溯来源信息

#### Scenario: evidence identity 在重复生成时保持稳定
- **WHEN** 同一页面在 evidence 集合未变化的情况下被重复生成
- **THEN** 相同 evidence 条目的稳定标识 MUST 保持不变
- **THEN** runtime 不得因为正文改写而重新生成一套无关的 evidence identity

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

