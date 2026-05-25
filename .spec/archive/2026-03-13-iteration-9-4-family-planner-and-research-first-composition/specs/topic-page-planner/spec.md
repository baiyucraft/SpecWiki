## ADDED Requirements

### Requirement: 专题页规划必须与 family planner 协同工作
系统 MUST 让现有 topic planner 与 family planner 协同工作。topic 仍用于表达实现机制、流程主题和 repo archetype 主题；family 用于表达产品知识域、API/config/docs/plugin/framework 树。planner MUST 明确哪类主题进入 family，哪类主题保留为 topic，并在两者重叠时优先收编或抑制重复页面。

#### Scenario: 产品知识域优先进入 family 而不是 topic
- **WHEN** 某个候选主题主要由 docs/API/config/plugin/framework 信号驱动
- **THEN** planner MUST 优先把它归入 family 页面集合
- **THEN** 系统不得再为同一主题额外生成近义 topic 页

#### Scenario: 实现机制和流程主题继续保留为 topic
- **WHEN** 某个候选主题主要表达请求链、模块能力、核心机制或 repo archetype 机制
- **THEN** planner MUST 继续把它归为 topic
- **THEN** family planner 不得抢占这类实现型主题
