## ADDED Requirements

### Requirement: workflow 主链必须在 page planner 中支持专题页
系统 MUST 在 `build_contexts -> plan_pages -> render_pages` 主链中支持专题页，而不是绕过现有 planner 直接拼装额外页面。专题页的父子关系、受影响集合和增量更新语义 MUST 与现有页面主链保持一致。

#### Scenario: init 在正式 planner 中生成专题页
- **WHEN** 用户执行 `init`，且当前仓库存在稳定专题候选
- **THEN** 系统 MUST 在 `plan_pages` 阶段生成专题页计划
- **THEN** 后续 `render_pages` MUST 把这些页面与其他正式页面一起渲染

#### Scenario: update 只重建受影响专题页
- **WHEN** 用户执行 `update`，且变化只影响部分专题候选或其 evidence 输入
- **THEN** 系统 MUST 只重建对应专题页及其受影响父页
- **THEN** 未受影响的专题页不得被无谓重写

### Requirement: 页面渲染必须支持 facts-driven 图表达
系统 MUST 在正式页面渲染阶段支持 facts-driven 图表达。图输入 MUST 由现有 `ModuleTree`、cross-module edges、父子层级和 detected processes 等稳定事实构造，而不是要求 LLM 自由生成结构。系统 MUST 至少支持模块依赖图、父子结构图和流程图三类受控图表达。

#### Scenario: 模块关系图来自稳定 cross-module edges
- **WHEN** 某个页面包含明确的跨模块依赖事实
- **THEN** renderer MUST 能基于这些事实构造受控 Mermaid 图
- **THEN** 图结构不得依赖 LLM 凭空补完

#### Scenario: 流程图来自 detected processes
- **WHEN** 某个专题页或 workflow 页消费稳定 detected process
- **THEN** renderer MUST 能基于该流程生成受控流程图
- **THEN** 当流程事实不足时，系统 MUST 回退到无图或纯文本说明
