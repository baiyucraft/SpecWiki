## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。模块页摘要 MUST 围绕高信号结构事实组织，包括模块角色、关键源码、依赖模块和被依赖模块。init 完成后 MUST 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。为了支撑增量 runtime，init 还 MUST 初始化 page context cache、page generation cache 和 section 状态，而不是只落盘整页 Markdown。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须装配 WikiState 并写入 `.wiki/.cache/wiki-state.json`
- **THEN** 系统必须通过 MetadataMapper 从 WikiState 导出 WikiMetadata 并写入 `wiki.metadata.json`
- **THEN** 系统必须生成项目总览页、系统架构页以及至少一类模块页
- **THEN** 模块页不得仅停留为占位页面
- **THEN** 系统必须初始化 per-page 的 page context / generation cache

#### Scenario: 初始化输入无效
- **WHEN** 用户在不存在的路径或非目录路径上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统仍必须完成页面、WikiState、metadata 和 cache 的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `status` 必须报告第一阶段 Runtime 状态
系统 MUST 基于当前仓库快照、最近一次 scan cache，以及 WikiState（优先）或 metadata（回退）计算 `ChangeSet`，报告 Repo Wiki 当前是否 `fresh`、`stale`、`missing` 或 `needs_rebuild`。`dirty_pages` MUST 表达受影响页面集合，而不是简单把所有页面视为脏页。对无法局部修复的 runtime 缺失、关键 cache 缺失或状态映射损坏，系统 MUST 提升为 `needs_rebuild`。

#### Scenario: Runtime 新鲜
- **WHEN** 当前仓库快照与 WikiState 记录一致，且增量 runtime 所需 cache 完整
- **THEN** `status` 必须返回 `fresh`

#### Scenario: Runtime 过期但仍可局部修复
- **WHEN** 某些源码 fingerprint 或结构线索发生变化，但运行时映射仍可重建
- **THEN** `status` 必须返回 `stale`
- **THEN** `dirty_sources` 必须只包含变化源码
- **THEN** `dirty_pages` 必须只包含受影响页面

#### Scenario: Runtime 缺失
- **WHEN** `.wiki/` 或 `wiki.metadata.json` 不存在
- **THEN** `status` 必须返回 `missing`

#### Scenario: 关键运行时缺失
- **WHEN** 页面文件缺失、关键 cache 文件缺失、或状态与页面映射无法建立一致性
- **THEN** `status` 必须返回 `needs_rebuild`
- **THEN** `needs_rebuild_reason` 必须说明缺失或损坏原因

#### Scenario: WikiState 丢失但 metadata 存在
- **WHEN** `.wiki/.cache/wiki-state.json` 不存在但 `wiki.metadata.json` 存在
- **THEN** `status` 必须仍能基于 metadata 理解正式索引
- **THEN** 如果增量 runtime 所需 cache 不完整，`status` 必须返回 `needs_rebuild` 而不是 `missing`

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 `stale` 时基于 `ChangeSet` 和 `AffectedSet` 执行增量刷新，而不是无条件重跑 `init`。局部可修复时，`update` MUST 只重建受影响页面及其 page context / generation cache，并保持未受影响页面不重写。Runtime 为 `missing` 时，`update` MUST 以等价于 `init` 的方式恢复运行时；Runtime 为 `needs_rebuild` 时，`update` MUST 走 full rebuild 路径。

#### Scenario: 过期后局部更新
- **WHEN** `status` 为 `stale` 且变化只影响已有页面集合
- **THEN** 系统必须只刷新受影响页面、metadata 和必要 cache
- **THEN** `updated_pages` 必须只包含本次实际重建的页面
- **THEN** 未受影响页面不得被无条件重写
- **THEN** 更新完成后 `status` 必须能够返回 `fresh`

#### Scenario: 结构变化后更新
- **WHEN** 新增或删除源码导致模块树或页面规划发生变化
- **THEN** 系统必须重新计算模块树和页面计划
- **THEN** 系统必须新增、删除或重建受影响页面，而不是直接清空整个 runtime
- **THEN** 更新完成后 metadata 和 WikiState 必须反映新的页面集合

#### Scenario: Runtime 缺失时更新
- **WHEN** `status` 为 `missing` 且用户执行 `update`
- **THEN** 系统必须以等价于初始化的方式恢复第一阶段运行产物

#### Scenario: 需要重建时更新
- **WHEN** `status` 为 `needs_rebuild` 且用户执行 `update`
- **THEN** 系统必须走显式 full rebuild 路径
- **THEN** 旧的局部 cache 不得继续被视为 fresh

### Requirement: `rebuild` 必须执行强制全量重建
系统 MUST 在用户显式执行 `rebuild` 时忽略或重建旧 runtime，并生成新的完整 Repo Wiki Runtime，包括 WikiState、WikiMetadata、page context cache 和 page generation cache。

#### Scenario: 强制重建
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统必须重新生成 Wiki 页面、WikiState、metadata 和缓存
- **THEN** 旧的脏状态不得直接复用为新的 fresh 状态
- **THEN** 旧的 per-page cache 必须被替换为当前页面计划对应的新 cache
