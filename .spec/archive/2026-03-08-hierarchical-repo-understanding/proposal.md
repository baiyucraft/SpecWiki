## Why

当前仓库已经具备 Repo Wiki 的基础调用链和运行时外壳，但生成结果仍偏向“能跑通”，还没有形成按系统结构组织的 Wiki。仅靠平铺页面和基础索引，无法稳定表达模块边界、模块层级和模块间关系，也不足以支撑后续增量更新、结构化查询和高质量内容生成。

现在推进这一变更，是为了把 `.wiki/06-设计文档/00-总体设计.md` 中的 `Hierarchical Decomposition` 和层级化页面规划真正落到实现里，让 Repo Wiki 从基础页面集合升级为按仓库结构组织的知识层。同时需要明确：Repo Wiki 的核心能力面向本地代码目录，Git 只是一种可选元信息来源，而不是功能前置条件。

## What Changes

- 在 `wiki-core` 中引入层级化仓库理解能力，输出稳定的模块树、模块边界和跨模块关系。
- 将页面规划从“目录驱动”升级为“模块 + 关系 + 层级驱动”，生成总览页、架构页、模块页之间的父子关系。
- 扩展 runtime 和 metadata，使页面、模块、源码之间的映射可以表达层级结构。
- 收敛 `init`、`query` 与后续 `update` 所需的结构输入，使模块树成为后续生成器和查询的稳定基础。
- 将初始化前提从“必须是 Git 仓库”收敛为“必须是可扫描的本地代码目录”，并把 Git 信息降级为可选元数据。

## Capabilities

### New Capabilities
- `repo-hierarchy-model`: 定义模块树、模块边界、跨模块关系和层级化页面规划的行为要求。

### Modified Capabilities
- `repo-wiki-runtime`: 扩展 runtime 与 `wiki.metadata.json`，使其能够表达模块、页面层级和页面来源关系。
- `repo-wiki-workflow`: 扩展 `init` 与 `query` 的要求，使其必须基于层级化结构生成和返回结果。

## Impact

- 受影响核心代码集中在 `crates/wiki-core/` 的扫描、上下文、规划、生成、metadata 和 query 相关模块。
- 现有 `.wiki/` 页面结构和 `wiki.metadata.json` 字段会扩展，但不会改变当前 Windows + CodeBuddy 的宿主边界。
- `init` 的输入边界会放宽到任意本地代码目录；后续 Git / SVN 支持则留给后续迭代的版本控制元信息层处理。
- `agents/codebuddy/` 主要受益于更稳定的查询和页面结构，不应引入新的 Wiki 业务逻辑。
