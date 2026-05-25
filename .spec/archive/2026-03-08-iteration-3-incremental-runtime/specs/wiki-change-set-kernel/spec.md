## ADDED Requirements

### Requirement: 系统必须从当前仓库快照和最近一次 runtime 计算稳定 ChangeSet
系统 MUST 基于当前仓库快照、最近一次 `repo-scan.json`、`WikiState` 和页面落盘状态计算稳定的 `ChangeSet`。`ChangeSet` MUST 至少区分新增、修改、删除和结构性变化的源码集合，并保留能够回溯到源码路径或 runtime 缺失位置的证据。

#### Scenario: 单个源码文件内容变化
- **WHEN** 某个已登记源码文件的内容 fingerprint 与最近一次 runtime 记录不一致
- **THEN** 系统必须把该源码记录为 modified source
- **THEN** 系统必须保留该源码路径作为 change evidence

#### Scenario: 新增或删除源码文件
- **WHEN** 当前仓库快照中出现新增源码文件，或某个已登记源码文件消失
- **THEN** 系统必须把对应源码记录为 added source 或 removed source
- **THEN** 系统不得把这类变化静默折叠成“普通 stale”

#### Scenario: 结构性线索发生变化
- **WHEN** manifest、workspace 成员、入口文件或模块边界相关线索发生变化
- **THEN** 系统必须把这些变化标记为 structural change
- **THEN** 系统必须允许后续 workflow 触发 module tree 或 page planner 的重新计算

### Requirement: 系统必须把 ChangeSet 映射到受影响模块、页面和 section
系统 MUST 基于稳定的 `source -> module -> page -> section` 映射，把 `ChangeSet` 转换为 `AffectedSet`。`AffectedSet` MUST 能表达受影响模块、受影响页面、页面内受影响 section，以及需要新增或删除的页面集合。

#### Scenario: 局部源码变化只影响局部页面
- **WHEN** 某个源码变化只命中已有模块和已有页面映射
- **THEN** 系统必须只把与该源码直接相关的模块、页面和 section 标记为受影响
- **THEN** 系统不得把无关联页面一并提升为 dirty

#### Scenario: 模块树变化触发局部 replan
- **WHEN** 结构性变化导致模块树或页面规划发生变化
- **THEN** 系统必须重新计算受影响模块对应的页面集合
- **THEN** 系统必须识别新增页面、删除页面和父子关系变化页面

### Requirement: 系统必须定义 cache invalidation 与回退边界
系统 MUST 按 cache 层级执行失效和回退，而不是对任何变化都直接清空整个 runtime。局部可修复时，系统 MUST 只失效受影响页面的 page context / generation cache；关键 runtime 不一致时，系统 MUST 提升为 `needs_rebuild` 或显式 full rebuild。

#### Scenario: 局部页面变化只失效局部缓存
- **WHEN** `AffectedSet` 只包含已有页面的局部变化
- **THEN** 系统必须只失效这些页面对应的 page context / generation cache
- **THEN** 未受影响页面的缓存必须继续可复用

#### Scenario: 关键运行时不一致触发回退
- **WHEN** 页面文件缺失、关键 cache 文件损坏、或状态与页面映射无法重建一致性
- **THEN** 系统必须把运行时状态提升为 `needs_rebuild`
- **THEN** `update` 不得继续在该状态上尝试局部修复
