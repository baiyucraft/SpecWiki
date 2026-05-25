## MODIFIED Requirements

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
