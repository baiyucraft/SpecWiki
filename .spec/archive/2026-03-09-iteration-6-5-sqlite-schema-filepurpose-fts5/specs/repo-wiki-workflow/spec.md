## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata、关系型状态表和增量缓存，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。模块页摘要 MUST 围绕高信号结构事实组织，包括模块角色、关键源码、依赖模块和被依赖模块。init 完成后 MUST 先装配 `WikiState` 并写入 SQLite 关系型状态表，再通过 MetadataMapper 导出 WikiMetadata。为了支撑 editable runtime 和后续检索，init 还 MUST 初始化 page context cache、page generation cache、section 状态、managed section marker、页面级 `section_anchors` 和 `wiki_pages_fts`，而不是只落盘 plain Markdown。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统 MUST 生成 `.wiki/` 运行产物
- **THEN** 系统 MUST 装配 `WikiState` 并写入 `.wiki/.cache/wiki-cache.db` 的关系型状态表
- **THEN** 系统 MUST 通过 MetadataMapper 从 `WikiState` 导出 WikiMetadata 并写入 `wiki.metadata.json`
- **THEN** 系统 MUST 生成项目总览页、系统架构页以及至少一类模块页
- **THEN** 系统 MUST 初始化 per-page 的 page context / generation cache、section 状态和 `wiki_pages_fts`
- **THEN** 初始化写出的页面 MUST 包含 managed section marker
- **THEN** 页面父子关系 MUST 按模块树层级分配

#### Scenario: 初始化时读取 steering 配置
- **WHEN** 用户在有效本地代码目录上执行 `init`，且 `.wiki/wiki.steering.yaml` 存在
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner 与 planner
- **THEN** steering 配置中的扫描忽略/包含规则 MUST 在扫描阶段生效
- **THEN** steering 配置中的模块提升/降级与页面阈值配置 MUST 在页面规划阶段生效

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统 MUST 仍然完成页面、SQLite 状态表、metadata 和缓存的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 `stale` 时基于 `ChangeSet` 和 `AffectedSet` 执行增量刷新，而不是无条件重跑 `init`。局部可修复时，`update` MUST 只重建受影响页面及其 page context / generation cache、页面状态行、section 状态行和 FTS 索引，并保持未受影响页面不重写。对于仍然存在的同一 `page_id` 页面，`update` MUST 只替换 managed sections，并保留已同步的 user sections 与稳定锚点。Runtime 为 `missing` 时，`update` MUST 以等价于 `init` 的方式恢复运行时；Runtime 为 `needs_rebuild` 时，`update` MUST 走 full rebuild 路径。

#### Scenario: 过期后局部更新
- **WHEN** `status` 为 `stale` 且变化只影响已有页面集合
- **THEN** 系统 MUST 只重建受影响页面
- **THEN** 系统 MUST 只刷新这些页面对应的状态行、section 行、page cache 和 FTS 记录
- **THEN** 系统 MUST 保持未受影响页面不重写
- **THEN** 系统 MUST 保留同页 user sections

#### Scenario: update 消费 steering 配置
- **WHEN** `update` 在增量路径中执行扫描与页面规划
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner 与 planner
- **THEN** steering 配置中的扫描忽略/包含、合并阈值和模块提升/降级 MUST 影响页面规划结果

### Requirement: `rebuild` 必须强制全量重建并保留同页 user sections
系统 MUST 在 `rebuild` 时忽略旧 generation cache 和旧 dirty state，但对仍然存在的同一 `page_id` 页面继续复用已同步的 user sections。`rebuild` MUST 删除旧数据库后重新创建全量状态表、缓存表和 FTS 索引，并读取 steering 配置参与新一轮扫描与页面规划。

#### Scenario: rebuild 消费 steering 配置
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统 MUST 读取 steering 配置并传递给 scanner 与 planner
- **THEN** rebuild 后的页面拓扑和扫描结果 MUST 反映 steering 配置的影响
- **THEN** 新数据库 MUST 包含完整的状态表、缓存表和 FTS 索引
