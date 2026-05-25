## ADDED Requirements

### Requirement: `wiki.metadata.json` 必须表达模块与页面层级
系统 MUST 在 `wiki.metadata.json` 中导出模块列表、模块层级、页面层级以及页面与模块之间的来源关系，使正式索引可以表达层级化 Repo Wiki 结构。

#### Scenario: 导出层级化索引
- **WHEN** 系统完成层级化页面生成并写入 metadata
- **THEN** `wiki.metadata.json` 必须包含模块列表和模块层级信息
- **THEN** `wiki.metadata.json` 必须包含页面层级或父子关系信息
- **THEN** `wiki.metadata.json` 必须能够表达页面与模块、源码之间的关联关系

### Requirement: 正式 Wiki 页面必须稳定映射到模块结构
系统 MUST 让正式 Wiki 页面与模块结构保持稳定映射，以便后续 `query`、`sync` 和 `update` 可以围绕层级化页面工作。

#### Scenario: 写入模块页
- **WHEN** 系统为模块生成正式 Wiki 页面
- **THEN** 页面状态中必须记录对应模块标识
- **THEN** 页面来源中必须记录该页面依赖的源码集合
- **THEN** 同一模块在重复生成时必须使用稳定页面标识或路径
