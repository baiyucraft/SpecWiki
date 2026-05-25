## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。模块页摘要 MUST 围绕高信号结构事实组织，包括模块角色、关键源码、依赖模块和被依赖模块。init 完成后 MUST 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。为了支撑 editable runtime，init 还 MUST 初始化 page context cache、page generation cache、section 状态和 managed section marker，而不是只落盘 plain Markdown。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须装配 WikiState 并写入 `.wiki/.cache/wiki-state.json`
- **THEN** 系统必须通过 MetadataMapper 从 WikiState 导出 WikiMetadata 并写入 `wiki.metadata.json`
- **THEN** 系统必须生成项目总览页、系统架构页以及至少一类模块页
- **THEN** 模块页不得仅停留为占位页面
- **THEN** 系统必须初始化 per-page 的 page context / generation cache
- **THEN** 初始化写出的页面必须包含 managed section marker

#### Scenario: 初始化输入无效
- **WHEN** 用户在不存在的路径或非目录路径上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统仍必须完成页面、WikiState、metadata 和 cache 的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 `stale` 时基于 `ChangeSet` 和 `AffectedSet` 执行增量刷新，而不是无条件重跑 `init`。局部可修复时，`update` MUST 只重建受影响页面及其 page context / generation cache，并保持未受影响页面不重写。对于仍然存在的同一 `page_id` 页面，`update` MUST 只替换 managed sections，并保留已同步的 user sections。Runtime 为 `missing` 时，`update` MUST 以等价于 `init` 的方式恢复运行时；Runtime 为 `needs_rebuild` 时，`update` MUST 走 full rebuild 路径。

#### Scenario: 过期后局部更新
- **WHEN** `status` 为 `stale` 且变化只影响已有页面集合
- **THEN** 系统必须只刷新受影响页面、metadata 和必要 cache
- **THEN** `updated_pages` 必须只包含本次实际重建的页面
- **THEN** 未受影响页面不得被无条件重写
- **THEN** 更新完成后 `status` 必须能够返回 `fresh`

#### Scenario: 源码变化后保留同页 user section
- **WHEN** 某个页面已经通过 `sync` 持久化了 user sections，且本次 `update` 后该页面的 `page_id` 仍然存在
- **THEN** 系统必须重新生成该页面的 managed sections
- **THEN** 系统必须把这些 user sections 合并回最终页面
- **THEN** 系统不得因为整页重写而覆盖这些 user sections

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

### Requirement: `sync` 必须同步用户对 Wiki 页面的外部修改
系统 MUST 在用户直接修改 `.wiki/*.md` 后解析页面结构，更新 WikiState，并通过 MetadataMapper 将变更同步到 metadata。`sync` 不得只刷新整页 hash；它还 MUST 回写 section 状态、当前页面 summary，以及 managed/user section 边界。

#### Scenario: 用户新增 Wiki 区段
- **WHEN** 用户在已有页面中新增一个不带 managed marker 的 section 或 Markdown 段落后执行 `sync`
- **THEN** 系统必须把这段内容识别为 user section 并写回 WikiState
- **THEN** 系统必须更新对应页面的整页 `content_hash`
- **THEN** 系统必须通过 MetadataMapper 导出更新后的 WikiMetadata

#### Scenario: 用户修改 managed section 正文
- **WHEN** 用户直接改写了已有 managed section 的正文后执行 `sync`
- **THEN** 系统必须继续把该区段识别为 managed section
- **THEN** 系统必须能够记录该区段当前磁盘内容与最近一次生成内容不一致
- **THEN** 系统不得把这类改动误记为 user section

#### Scenario: legacy 页面同步
- **WHEN** 页面仍处于无 managed marker 的 legacy 格式，且用户执行 `sync`
- **THEN** 系统必须尝试按已知 section 标题恢复 managed/user section 边界
- **THEN** 成功恢复时，系统必须把结果写回 WikiState

### Requirement: `rebuild` 必须执行强制全量重建
系统 MUST 在用户显式执行 `rebuild` 时忽略或重建旧 runtime，并生成新的完整 Repo Wiki Runtime，包括 WikiState、WikiMetadata、page context cache 和 page generation cache。`rebuild` 虽然必须忽略旧的 generation cache 和旧 dirty state，但对于仍然存在的同一 `page_id` 页面，仍 MUST 复用已同步的 user sections，而不是把人工内容整页覆盖。

#### Scenario: 强制重建
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统必须重新生成 Wiki 页面、WikiState、metadata 和缓存
- **THEN** 旧的脏状态不得直接复用为新的 fresh 状态
- **THEN** 旧的 per-page cache 必须被替换为当前页面计划对应的新 cache

#### Scenario: rebuild 保留同页 user section
- **WHEN** 用户已经对某个页面同步过 user sections，且该页面在本次 `rebuild` 后仍然存在同一 `page_id`
- **THEN** 系统必须重新生成该页面的 managed sections
- **THEN** 系统必须把旧页面中的 user sections 合并回新页面
- **THEN** `rebuild` 不得把这些同页 user sections 直接抹掉
