## 1. 模块树与层级拆分

- [x] 1.1 在 `wiki-core` 中补齐 `ModuleNode`、`ModuleTree`、`RelationEdge` 相关领域模型与稳定 ID 生成逻辑
- [x] 1.2 基于现有扫描结果实现第一版 `Hierarchical Decomposition`，按 workspace、目录、入口和依赖线索生成模块树
- [x] 1.3 为模块树补齐跨模块关系与基础架构提示的构建逻辑，并保证结果可序列化到 runtime 或 cache

## 2. 页面规划与层级化生成

- [x] 2.1 调整 `Context Builder`，让 `RepoContext`、`ModuleContext`、`PageContext` 能消费模块树而不是仅消费扫描结果
- [x] 2.2 重写 `Page Planner`，生成项目总览页、系统架构页和模块页，并建立页面父子关系
- [x] 2.3 调整 `Document Generator` 与 `Wiki Assembler`，让层级化页面可以稳定落盘到 `.wiki/`

## 3. Runtime、Metadata 与 Query

- [x] 3.1 扩展 `WikiState` 与 metadata 导出逻辑，写入模块列表、模块层级、页面层级以及页面与模块/源码的来源关系
- [x] 3.2 调整页面状态和源码状态模型，使模块页与其来源源码保持稳定映射
- [x] 3.3 调整 `query`，让返回结果显式包含命中页面、相关模块、相关源码和结构化摘要

## 4. 验证与文档

- [x] 4.1 为模块树生成、页面规划、metadata 导出补齐 `wiki-core` 单元测试或集成测试
- [x] 4.2 更新端到端验证，覆盖 `init` 生成层级化页面以及 `query` 返回模块相关结果
- [x] 4.3 更新仓库文档与 UniSpec 说明，明确“层级化仓库理解”已成为 Repo Wiki 的核心行为之一
