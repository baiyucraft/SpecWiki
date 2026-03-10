## ADDED Requirements

### Requirement: Scanner 必须为每个已分析文件产出稳定的 FilePurpose
系统 MUST 为每个进入 `ScanReport.files` 的文件产出稳定的 `FilePurpose`，替代当前粗粒度的 `kind` 字符串。`FilePurpose` MUST 至少覆盖以下角色：`Entry`、`Router`、`Controller`、`Handler`、`Service`、`Model`、`Repository`、`Domain`、`Agent`、`Library`、`Middleware`、`Plugin`、`Utility`、`Helper`、`Constant`、`Type`、`Page`、`Component`、`Widget`、`Layout`、`Config`、`Migration`、`Test`、`Docs`。分类 MUST 先走 deterministic 的路径/文件名规则；本迭代不得依赖 LLM 才能完成基础分类。

#### Scenario: 后端入口与职责文件按路径和名称分类
- **WHEN** scanner 遍历到 `main.rs`、`routes.ts`、`user_controller.go`、`payment_service.py`、`auth_handler.rs` 这类文件
- **THEN** 系统 MUST 分别将其分类为 `Entry`、`Router`、`Controller`、`Service`、`Handler`
- **THEN** 相同路径和文件名在重复扫描时 MUST 产出相同的 `FilePurpose`

#### Scenario: 前端页面与组件文件按目录结构分类
- **WHEN** scanner 遍历到位于 `pages/`、`components/`、`widgets/`、`layouts/` 等目录下的前端文件
- **THEN** 系统 MUST 优先将其分类为 `Page`、`Component`、`Widget`、`Layout`
- **THEN** 不得仅因为扩展名相同就把这些文件退化成统一的 `Library`

#### Scenario: 配置、测试、文档和迁移文件分类
- **WHEN** scanner 遍历到 manifest、配置文件、数据库迁移、测试源码或 Markdown 文档
- **THEN** 系统 MUST 将其分类为 `Config`、`Migration`、`Test` 或 `Docs`
- **THEN** 这些文件仍可进入 `ScanReport.files`，但其角色 MUST 与业务实现源码区分

#### Scenario: 未命中特化规则的源码文件回落到保守角色
- **WHEN** 某个代码文件未命中任何专门的路径或文件名分类规则
- **THEN** 系统 MUST 回落到保守的通用角色（如 `Library`、`Helper` 或 `Utility`）
- **THEN** 系统不得因为分类不确定而中断扫描流程

### Requirement: FilePurpose 必须被下游页面规划和关键源码选择消费
系统 MUST 让 hierarchy、planner、context builder 和关键源码选择逻辑消费 `FilePurpose`，而不是继续只依赖 `source/config/docs` 这类粗粒度标签。高信号角色（如 `Entry`、`Router`、`Controller`、`Handler`、`Service`）MUST 能影响模块权重和关键源码排序；低信号角色（如 `Test`、`Docs`、纯 `Config`）MUST 被适当降权。

#### Scenario: 高信号角色提升模块和页面摘要质量
- **WHEN** 某模块同时包含 `Entry`、`Router`、`Service` 与普通 `Library` 文件
- **THEN** 页面规划和关键源码选择 MUST 优先保留前述高信号角色文件
- **THEN** 模块摘要不得被低价值辅助文件主导

#### Scenario: 测试与文档角色在下游被降权
- **WHEN** 某模块中的 `Test`、`Docs` 或纯 `Config` 文件数量很多
- **THEN** 系统 MUST 在模块权重、关键源码选择和页面摘要中对这些角色降权
- **THEN** 不得因为测试或文档文件数量占优就错误抬高该模块的重要性
