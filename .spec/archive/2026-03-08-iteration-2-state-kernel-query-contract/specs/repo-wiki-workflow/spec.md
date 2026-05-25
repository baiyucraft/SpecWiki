## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache，并在页面规划阶段基于递归模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。模块页摘要 MUST 围绕高信号结构事实组织，包括模块角色、关键源码、依赖模块和被依赖模块。init 完成后 MUST 先装配 WikiState 并持久化，再通过 MetadataMapper 导出 WikiMetadata。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须装配 WikiState 并写入 `.wiki/.cache/wiki-state.json`
- **THEN** 系统必须通过 MetadataMapper 从 WikiState 导出 WikiMetadata 并写入 `wiki.metadata.json`
- **THEN** 系统必须生成项目总览页、系统架构页以及至少一类模块页
- **THEN** 模块页不得仅停留为占位页面

#### Scenario: 初始化输入无效
- **WHEN** 用户在不存在的路径或非目录路径上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统仍必须完成页面、WikiState、metadata 和 cache 的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `status` 必须报告第一阶段 Runtime 状态
系统 MUST 根据 WikiState（优先）或 metadata（回退）中的源码指纹与当前扫描结果比对，报告 Repo Wiki 当前是否 fresh、stale、missing 或需要重建。

#### Scenario: Runtime 新鲜
- **WHEN** WikiState 或 metadata 中的源码指纹与当前扫描一致
- **THEN** `status` 必须返回 fresh

#### Scenario: Runtime 过期
- **WHEN** 源码指纹与 WikiState 或 metadata 中记录的不一致
- **THEN** `status` 必须返回 stale

#### Scenario: Runtime 缺失
- **WHEN** `.wiki/` 或 `wiki.metadata.json` 不存在
- **THEN** `status` 必须返回 missing

#### Scenario: WikiState 丢失但 metadata 存在
- **WHEN** `.wiki/.cache/wiki-state.json` 不存在但 `wiki.metadata.json` 存在
- **THEN** `status` MUST 从 metadata 重建 WikiState 后继续工作
- **THEN** `status` 返回结果 MUST 与 WikiState 存在时一致

### Requirement: `query` 必须返回结构化查询结果
系统 MUST 基于 WikiState（优先）或 metadata（回退）返回结构化查询结果。结构化索引 MUST 作为主命中来源；Markdown 内容匹配只可作为回退路径。查询结果 MUST 返回命中对象、命中原因、provenance 和上下文打包，使 Agent 可以直接消费统一结构。

#### Scenario: 查询命中页面
- **WHEN** 用户提供查询词且存在相关 Wiki 页面
- **THEN** 系统必须返回命中的页面标识或路径
- **THEN** 系统必须返回相关模块标识或模块信息
- **THEN** 系统必须返回相关源码或来源线索
- **THEN** 系统必须返回可供 Agent 消费的摘要或命中说明
- **THEN** 每个命中页面 MUST 包含 context_pack（关联模块摘要、关键源码路径、关系证据）

#### Scenario: 结构化命中不足时回退 Markdown
- **WHEN** 用户提供查询词但结构化索引无法提供足够命中
- **THEN** 系统可以回退到 Markdown 内容匹配
- **THEN** 返回结果必须显式标出该命中为回退路径或文本命中

#### Scenario: 查询无结果
- **WHEN** 用户提供查询词但无任何命中
- **THEN** 系统必须返回结构化的空结果，而不是进程级失败

#### Scenario: 结构索引优先于 Markdown 回退
- **WHEN** 结构化页面、模块、源码或关系对象已经足以命中查询
- **THEN** 系统必须优先返回结构命中结果
- **THEN** 结果中必须能够区分结构命中与 Markdown 回退命中

#### Scenario: 查询结果包含 provenance 摘要
- **WHEN** query 返回命中结果
- **THEN** QueryReport MUST 包含 provenance_summary 字段，描述整体命中来源的结构化摘要

### Requirement: `sync` 必须同步用户对 Wiki 页面的外部修改
系统 MUST 在用户直接修改 `.wiki/*.md` 后重新计算页面状态，更新 WikiState，并通过 MetadataMapper 将变更同步到 metadata。

#### Scenario: 用户修改 Wiki 页面
- **WHEN** 用户修改已有 `.wiki/*.md` 后执行 `sync`
- **THEN** 系统必须更新 WikiState 中对应页面的 content_hash
- **THEN** 系统必须通过 MetadataMapper 导出更新后的 WikiMetadata
- **THEN** 系统必须持久化更新后的 WikiState

### Requirement: `rebuild` 必须执行强制全量重建
系统 MUST 在用户显式执行 `rebuild` 时忽略或重建旧 runtime，并生成新的完整 Repo Wiki Runtime，包括 WikiState 和 WikiMetadata。

#### Scenario: 强制重建
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统必须重新生成 Wiki 页面、WikiState、metadata 和缓存
- **THEN** 旧的脏状态不得直接复用为新的 fresh 状态
