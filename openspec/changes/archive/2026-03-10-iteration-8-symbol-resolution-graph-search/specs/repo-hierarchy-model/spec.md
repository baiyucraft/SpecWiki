## ADDED Requirements

### Requirement: 模块关系推断必须消费 symbol graph 摘要
系统 MUST 在保持目录结构和 steering 规则主导模块发现的前提下，消费 symbol graph 聚合后的高层信号增强模块关系推断。`ModuleTree.cross_module_edges` MUST 结合现有文件级 `DependencyHint` 与由 `IMPORTS / CALLS` 聚合得到的模块级依赖证据；低置信度或噪声边不得主导模块关系。模块级关系的 evidence MUST 能回溯到具体源码文件或 symbol edge 来源。

#### Scenario: symbol graph 强化跨模块关系
- **WHEN** 两个模块之间存在明确的 symbol-level `IMPORTS` 或 `CALLS` 关系
- **THEN** 系统 MUST 能把这些关系聚合为模块级 cross-module edge
- **THEN** 该 edge MUST 记录可追溯的 evidence

#### Scenario: 低质量 symbol edges 不主导模块关系
- **WHEN** 某些 symbol-level 关系只来自低置信度或噪声边
- **THEN** 系统 MUST 不让这些边单独主导模块级关系推断
- **THEN** 模块树稳定性不得因为局部噪声调用而大幅波动

### Requirement: workflow 页面规划必须消费 detected processes
系统 MUST 让 planner 在存在 detected processes 时生成 `workflow` 页面，即使仓库中缺少显式 CI/CD 配置或 Makefile/Dockerfile 线索。workflow 页面 MUST 优先展示执行流 process、关键入口点与跨模块流转，而不是只列出工作流文件；当 detected processes 与 CI/CD 线索同时存在时，页面 MUST 同时覆盖二者。

#### Scenario: 由 detected processes 触发 workflow 页面
- **WHEN** 仓库中检测到至少一个稳定 process，但没有显式 CI/CD 文件线索
- **THEN** planner MUST 仍生成 `workflow` 类型页面
- **THEN** 该页面 MUST 包含至少一个 detected process 的概述

#### Scenario: process 与 CI/CD 线索共同出现在 workflow 页面
- **WHEN** 仓库中同时存在 detected processes 和 CI/CD / Makefile / Dockerfile 线索
- **THEN** workflow 页面 MUST 同时包含执行流和工作流文件线索
- **THEN** 页面内容不得退化为仅罗列文件路径

