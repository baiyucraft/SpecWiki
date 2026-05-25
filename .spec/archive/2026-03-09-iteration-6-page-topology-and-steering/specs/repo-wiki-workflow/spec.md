## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。模块页摘要 MUST 围绕高信号结构事实组织，包括模块角色、关键源码、依赖模块和被依赖模块。init 完成后 MUST 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。为了支撑 editable runtime，init 还 MUST 初始化 page context cache、page generation cache、section 状态和 managed section marker，而不是只落盘 plain Markdown。init MUST 在页面规划阶段读取 steering 配置（如果存在），并按 steering 配置的忽略路径、模块提升/降级和合并阈值影响 planner 决策。init 生成的页面 MUST 按模块树层级分配父子关系，而不是将所有模块页平铺在 overview 页下面。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须装配 WikiState 并写入 `.wiki/.cache/wiki-state.json`
- **THEN** 系统必须通过 MetadataMapper 从 WikiState 导出 WikiMetadata 并写入 `wiki.metadata.json`
- **THEN** 系统必须生成项目总览页、系统架构页以及至少一类模块页
- **THEN** 模块页不得仅停留为占位页面
- **THEN** 系统必须初始化 per-page 的 page context / generation cache
- **THEN** 初始化写出的页面必须包含 managed section marker
- **THEN** 页面父子关系必须按模块树层级分配

#### Scenario: 初始化时读取 steering 配置
- **WHEN** 用户在有效本地代码目录上执行 `init`，且 `.wiki/wiki.steering.yaml` 存在
- **THEN** 系统 MUST 读取 steering 配置并传递给 planner
- **THEN** steering 配置中的忽略路径（全局 + 当前语言匹配的按语言忽略）MUST 在扫描阶段生效
- **THEN** steering 配置中的模块提升/降级 MUST 在页面规划阶段生效
- **THEN** steering 配置中的合并阈值 MUST 影响小模块合并决策

#### Scenario: 初始化输入无效
- **WHEN** 用户在不存在的路径或非目录路径上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统仍必须完成页面、WikiState、metadata 和 cache 的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 `stale` 时基于 `ChangeSet` 和 `AffectedSet` 执行增量刷新，而不是无条件重跑 `init`。局部可修复时，`update` MUST 只重建受影响页面及其 page context / generation cache，并保持未受影响页面不重写。对于仍然存在的同一 `page_id` 页面，`update` MUST 只替换 managed sections，并保留已同步的 user sections。Runtime 为 `missing` 时，`update` MUST 以等价于 `init` 的方式恢复运行时；Runtime 为 `needs_rebuild` 时，`update` MUST 走 full rebuild 路径。`update` MUST 在增量路径中读取 steering 配置并传递给 planner，确保 steering 配置的变化能在下一次 update 中生效。

#### Scenario: 过期后局部更新
- **WHEN** `status` 为 `stale` 且变化只影响已有页面集合
- **THEN** 系统 MUST 只重建受影响页面
- **THEN** 系统 MUST 保持未受影响页面不重写
- **THEN** 系统 MUST 保留同页 user sections

#### Scenario: update 消费 steering 配置
- **WHEN** `update` 在增量路径中执行页面规划
- **THEN** 系统 MUST 读取 steering 配置并传递给 planner
- **THEN** steering 配置中的合并阈值和模块提升/降级 MUST 影响页面规划结果

### Requirement: `rebuild` 必须强制全量重建并保留同页 user sections
系统 MUST 在 `rebuild` 时忽略旧 generation cache 和旧 dirty state，但对仍然存在的同一 `page_id` 页面继续复用已同步的 user sections。`rebuild` MUST 读取 steering 配置并传递给 planner。

#### Scenario: rebuild 消费 steering 配置
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统 MUST 读取 steering 配置并传递给 planner
- **THEN** rebuild 后的页面拓扑 MUST 反映 steering 配置的影响
