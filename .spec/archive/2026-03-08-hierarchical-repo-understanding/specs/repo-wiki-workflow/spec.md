## MODIFIED Requirements

### Requirement: `init` 必须为有效本地代码目录建立第一阶段 Repo Wiki
系统 MUST 在有效本地代码目录上执行完整初始化，生成第一阶段所需的 Wiki 页面、metadata 和 cache，并在页面规划阶段基于模块树生成层级化页面。Git 信息在存在时可作为元数据补充，但不得成为初始化前提。

#### Scenario: 初始化成功
- **WHEN** 用户在有效本地代码目录上执行 `init`
- **THEN** 系统必须生成 `.wiki/` 运行产物
- **THEN** 系统必须写入 fresh 状态的 metadata
- **THEN** 系统必须生成项目总览页、系统架构页以及至少一类模块页

#### Scenario: 初始化输入无效
- **WHEN** 用户在不存在的路径或非目录路径上执行 `init`
- **THEN** 系统必须返回明确错误
- **THEN** 系统不得写入半成品 runtime

#### Scenario: 在非 Git 目录中初始化
- **WHEN** 用户在不包含 `.git` 的本地代码目录上执行 `init`
- **THEN** 系统仍必须完成页面、metadata 和 cache 的生成
- **THEN** 系统不得因为缺失 Git 元信息而拒绝初始化

### Requirement: `query` 必须返回结构化查询结果
系统 MUST 基于正式 Wiki 页面、metadata 与模块结构返回结构化查询结果，而不是仅输出非结构化文本。

#### Scenario: 查询命中页面
- **WHEN** 用户提供查询词且存在相关 Wiki 页面
- **THEN** 系统必须返回命中的页面标识或路径
- **THEN** 系统必须返回相关模块标识或模块信息
- **THEN** 系统必须返回可供 Agent 消费的摘要或命中说明

#### Scenario: 查询无结果
- **WHEN** 用户提供查询词但无任何命中
- **THEN** 系统必须返回结构化的空结果，而不是进程级失败
