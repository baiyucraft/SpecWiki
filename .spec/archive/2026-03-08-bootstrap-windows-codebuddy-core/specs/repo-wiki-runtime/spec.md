## ADDED Requirements

### Requirement: Repo Wiki Runtime 必须写入统一的 `.wiki/` 目录
系统 MUST 将第一阶段的 Repo Wiki 运行产物统一写入目标仓库的 `.wiki/` 目录，并区分正式文档、正式索引和运行时缓存。

#### Scenario: 初始化 Repo Wiki Runtime
- **WHEN** 用户在有效 Git 仓库上执行 `init`
- **THEN** 系统必须创建 `.wiki/`
- **THEN** 系统必须写入至少一个正式 Wiki 页面
- **THEN** 系统必须写入 `.wiki/wiki.metadata.json`
- **THEN** 系统必须创建 `.wiki/.cache/`

### Requirement: `wiki.metadata.json` 必须承载第一阶段最小索引字段
系统 MUST 在 `wiki.metadata.json` 中稳定保存第一阶段最小索引字段，并保持字段职责与 `tmp/reference-zh/meta/repowiki-metadata.json` 尽量一致。

#### Scenario: 写入 metadata
- **WHEN** 系统完成 `init`、`update` 或 `rebuild`
- **THEN** `wiki.metadata.json` 必须包含 schema 或版本字段
- **THEN** `wiki.metadata.json` 必须包含语言、仓库根路径、分支、生成时间和最近索引提交信息
- **THEN** `wiki.metadata.json` 必须包含 wiki items、relations、source files 和 dirty state

#### Scenario: 读取 metadata
- **WHEN** 系统执行 `status`、`query` 或 `sync`
- **THEN** 系统必须能够从 `wiki.metadata.json` 读取并解释第一阶段最小字段，而不依赖 `.cache/` 才能理解正式索引

### Requirement: 运行时缓存必须与正式索引分层
系统 MUST 将运行时缓存写入 `.wiki/.cache/`，且缓存缺失时不应使正式 Wiki 页面或 metadata 失去有效性。

#### Scenario: 缓存丢失但正式索引仍在
- **WHEN** `.wiki/.cache/` 被删除，但 `.wiki/*.md` 和 `wiki.metadata.json` 仍存在
- **THEN** 系统必须仍能执行 `status`
- **THEN** 系统必须将需要重建缓存的情况与正式索引缺失区分开
