## ADDED Requirements

### Requirement: SQLite 必须支持增量 workflow 所需的文件级 symbol 与 edge 读取
系统 MUST 为增量 workflow 提供按文件路径集合读取 `symbols` 与 `edges` 的能力，而不是只暴露全量枚举接口。文件级读取能力 MUST 能支撑 `update` 组装局部 symbol/edge 工作集，并允许继续向外扩展必要的一跳 dependents 或 graph frontier。相关查询不得破坏现有事务一致性要求。

#### Scenario: 按文件集合读取 symbol rows
- **WHEN** `update` 需要刷新一组受影响源码文件的 symbol snapshot
- **THEN** 系统 MUST 能只读取这些文件对应的 `symbols` rows
- **THEN** 结果中不得混入与该文件集合无关的 symbol rows

#### Scenario: 按文件集合读取 edge rows 并补充必要 frontier
- **WHEN** `update` 需要为一组受影响源码文件重建局部 graph 工作集
- **THEN** 系统 MUST 能读取这些文件相关的 `edges` rows，并补充后续合并所需的必要 frontier
- **THEN** 系统不得要求任何小范围更新都先执行一次无条件全表 edge 枚举
