## ADDED Requirements

### Requirement: `init` 必须为有效 Git 仓库建立第一阶段 Repo Wiki
系统 MUST 在有效 Git 仓库上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache。

#### Scenario: 初始化成功
- **WHEN** 用户在有效 Git 仓库上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须写入 fresh 状态的 metadata

#### Scenario: 初始化输入无效
- **WHEN** 用户在非 Git 仓库上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

### Requirement: `status` 必须报告第一阶段 Runtime 状态
系统 MUST 根据源码摘要、正式索引和运行产物存在性，报告 Repo Wiki 当前是否 fresh、stale、missing 或需要重建。

#### Scenario: Runtime 新鲜
- **WHEN** `.wiki/*.md`、`wiki.metadata.json` 与当前源码摘要一致
- **THEN** `status` 必须返回 fresh

#### Scenario: Runtime 过期
- **WHEN** 源码摘要与 metadata 中记录的 source files 指纹不一致
- **THEN** `status` 必须返回 stale

#### Scenario: Runtime 缺失
- **WHEN** `.wiki/` 或 `wiki.metadata.json` 不存在
- **THEN** `status` 必须返回 missing

### Requirement: `update` 必须将过期 Runtime 刷新到 fresh
系统 MUST 在发现 Runtime 为 stale 或 cache 缺失时刷新必要产物，并使 Repo Wiki 返回 fresh 状态。

#### Scenario: 过期后更新
- **WHEN** `status` 为 stale 且用户执行 `update`
- **THEN** 系统必须刷新 Wiki 页面、metadata 和必要缓存
- **THEN** 更新完成后 `status` 必须能够返回 fresh

#### Scenario: Runtime 缺失时更新
- **WHEN** `status` 为 missing 且用户执行 `update`
- **THEN** 系统必须以等价于初始化的方式恢复第一阶段运行产物

### Requirement: `query` 必须返回结构化查询结果
系统 MUST 基于正式 Wiki 页面与 metadata 返回结构化查询结果，而不是仅输出非结构化文本。

#### Scenario: 查询命中页面
- **WHEN** 用户提供查询词且存在相关 Wiki 页面
- **THEN** 系统必须返回命中的页面标识或路径
- **THEN** 系统必须返回可供 Agent 消费的摘要或命中说明

#### Scenario: 查询无结果
- **WHEN** 用户提供查询词但无任何命中
- **THEN** 系统必须返回结构化的空结果，而不是进程级失败

### Requirement: `sync` 必须同步用户对 Wiki 页面的外部修改
系统 MUST 在用户直接修改 `.wiki/*.md` 后重新计算页面状态，并将变更同步回 metadata。

#### Scenario: 用户修改 Wiki 页面
- **WHEN** 用户修改已有 `.wiki/*.md` 后执行 `sync`
- **THEN** 系统必须更新对应页面的内容摘要或 hash
- **THEN** 系统必须更新 metadata 中对应页面的状态

### Requirement: `rebuild` 必须执行强制全量重建
系统 MUST 在用户显式执行 `rebuild` 时忽略或重建旧 runtime，并生成新的完整 Repo Wiki Runtime。

#### Scenario: 强制重建
- **WHEN** 用户执行 `rebuild`
- **THEN** 系统必须重新生成 Wiki 页面、metadata 和缓存
- **THEN** 旧的脏状态不得直接复用为新的 fresh 状态

