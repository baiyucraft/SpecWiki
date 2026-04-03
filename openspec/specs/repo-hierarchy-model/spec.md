# repo-hierarchy-model Specification

## Purpose
定义 Repo Wiki 仓库层级模型与页面规划输入的正式边界，说明模块树、页面合并拆分、workflow 线索与专题线索如何稳定产出。

## Requirements

### Requirement: 系统必须生成稳定的模块树
系统 MUST 基于归一化后的仓库扫描结果生成稳定的递归 `ModuleTree`，用于表达模块边界、父子层级、模块类型和模块包含的关键源码集合，而不是只生成根模块与一层子模块。系统 MUST 在模块提升阶段抑制单文件模块，不允许只含单个文件且无子模块的候选节点被提升为独立模块。系统 MUST 在模块提升评分中对 test / fixture 路径下的文件施加降权，避免测试产物主导模块发现。系统 MUST 为每个模块计算"页面权重"评分，基于源码文件数量、是否有子模块、是否是 workspace 成员、是否有入口文件等因素，供 planner 的合并策略消费。

#### Scenario: 从仓库结构生成模块树
- **WHEN** 系统完成仓库扫描并进入层级拆分阶段
- **THEN** 系统必须生成包含根模块和递归子模块的模块树
- **THEN** 每个模块必须具有稳定标识、名称、根路径、模块类型和关联源码集合
- **THEN** 每个非叶子模块必须能够导出稳定的 `child_ids`

#### Scenario: 重复扫描同一仓库
- **WHEN** 同一仓库在源码结构未发生影响模块边界的变化时被重复扫描
- **THEN** 系统必须保持模块发现顺序和父子关系稳定
- **THEN** 同一模块必须保持稳定标识

#### Scenario: 抑制单文件模块
- **WHEN** hierarchy 层发现一个候选模块根路径下只包含单个文件且没有子模块
- **THEN** 系统 MUST 不将该候选节点提升为独立模块
- **THEN** 该文件 MUST 被归入其最近的父模块

#### Scenario: 抑制非代码目录提升为模块
- **WHEN** hierarchy 层发现一个候选模块根路径对应的目录不包含任何源码文件（kind 为 `"source"` 的文件），只包含文档、配置或产物文件
- **THEN** 系统 MUST 不将该候选节点提升为独立模块

#### Scenario: test / fixture 文件不主导模块提升评分
- **WHEN** hierarchy 层在评估候选模块根路径时，该路径下的文件大部分位于 test / fixture 子目录
- **THEN** 系统 MUST 对这些文件施加降权
- **THEN** 系统 MUST 不因为 test / fixture 文件数量多而将该路径提升为高优先级模块

#### Scenario: 模块页面权重评分
- **WHEN** 系统完成模块树构建
- **THEN** 每个模块 MUST 具有可查询的页面权重评分
- **THEN** 页面权重 MUST 基于源码文件数量、是否有子模块、是否是 workspace 成员、是否有入口文件等因素计算

### Requirement: planner 必须支持页面合并与拆分策略
系统 MUST 在页面规划阶段基于模块页面权重和 steering 配置决定哪些模块生成独立页面、哪些模块合并到父模块页面。planner MUST 支持小模块合并（权重低于阈值的模块合并到父模块页）和 steering 配置的提升/降级覆盖。

#### Scenario: 小模块合并到父模块页
- **WHEN** planner 评估一个模块的页面权重低于合并阈值且该模块没有子模块
- **THEN** planner MUST 不为该模块生成独立页面
- **THEN** planner MUST 将该模块的内容标记为父模块页面的子模块概述

#### Scenario: 有子模块的模块不被合并
- **WHEN** planner 评估一个模块的页面权重低于合并阈值但该模块拥有子模块
- **THEN** planner MUST 仍为该模块生成独立页面

#### Scenario: steering promote 覆盖合并
- **WHEN** steering 配置中 promote 了一个低权重模块
- **THEN** planner MUST 为该模块生成独立页面

### Requirement: planner 必须支持扩展的页面类型集合
系统 MUST 支持至少四种页面类型：`overview`、`architecture`、`module`、`workflow`。当仓库存在明确的工作流线索（CI/CD 配置、Makefile、Docker 编排文件）时，planner MUST 生成 `workflow` 类型页面。每种页面类型 MUST 有差异化的 section 模板。

#### Scenario: 生成 workflow 页面
- **WHEN** 仓库中存在 CI/CD 配置文件（如 `.github/workflows/`）、Makefile 或 Dockerfile
- **THEN** planner MUST 生成一个 `workflow` 类型页面
- **THEN** 该页面 MUST 包含工作流相关的 section（构建流程、CI/CD 配置、容器化等）

#### Scenario: 无工作流线索时不生成 workflow 页面
- **WHEN** 仓库中不存在任何 CI/CD 配置、Makefile 或 Dockerfile
- **THEN** planner MUST 不生成 `workflow` 类型页面

#### Scenario: 各页面类型有差异化 section 模板
- **WHEN** planner 为不同类型的页面生成 section 草稿
- **THEN** overview 页面 MUST 包含简介、项目事实、技术栈、入口与构建、关键信息等 section
- **THEN** architecture 页面 MUST 包含架构概览、模块结构、跨模块关系、架构提示等 section
- **THEN** module 页面 MUST 包含模块说明、关键源码、依赖关系、模块事实等 section
- **THEN** workflow 页面 MUST 包含工作流概述、构建流程、CI/CD 配置等 section

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

### Requirement: hierarchy 必须为专题发现输出根级高信号文件簇线索
系统 MUST 在保持目录模块树语义稳定的前提下，为 planner 输出根级高信号文件簇和能力簇线索。这些线索 MAY 来自根路径关键源码、graph hotspot、依赖证据和命名模式的组合，但它们不得被强制提升为独立 `ModuleNode`。

#### Scenario: 根级机制线索进入专题发现输入
- **WHEN** 仓库根目录存在多份高信号源码，并且这些源码共同支撑同一核心机制
- **THEN** hierarchy 或其后续聚合输入 MUST 把该文件簇作为专题发现线索输出
- **THEN** 这些文件在模块树中仍可继续属于根模块

#### Scenario: 专题线索不得破坏模块树稳定性
- **WHEN** 系统为 planner 发现新的根级专题候选
- **THEN** 现有目录模块的 `module_id`、父子关系和模块边界 MUST 保持稳定
- **THEN** 系统不得为了专题页而重写目录模块树语义
