## ADDED Requirements

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
