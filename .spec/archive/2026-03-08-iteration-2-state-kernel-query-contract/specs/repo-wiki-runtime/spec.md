## MODIFIED Requirements

### Requirement: `wiki.metadata.json` 必须承载第一阶段最小索引字段
系统 MUST 在 `wiki.metadata.json` 中稳定保存第一阶段最小索引字段，并保持字段职责与 `tmp/reference-zh/meta/repowiki-metadata.json` 尽量一致。`wiki.metadata.json` MUST 由 MetadataMapper 从 WikiState 导出生成，而不是由 workflow 直接手工拼装。

#### Scenario: 写入 metadata
- **WHEN** 系统完成 `init`、`update` 或 `rebuild`
- **THEN** 系统 MUST 通过 MetadataMapper 从 WikiState 导出 WikiMetadata
- **THEN** `wiki.metadata.json` 必须包含 schema 或版本字段
- **THEN** `wiki.metadata.json` 必须包含语言、仓库根路径、分支、生成时间和最近索引提交信息
- **THEN** `wiki.metadata.json` 必须包含 wiki items、relations、source files 和 dirty state

#### Scenario: 读取 metadata
- **WHEN** 系统执行 `status`、`query` 或 `sync`
- **THEN** 系统 MUST 优先读取 WikiState；WikiState 不存在时 MUST 能够从 `wiki.metadata.json` 重建 WikiState
- **THEN** 系统不得依赖 `.cache/` 才能理解正式索引

### Requirement: 运行时缓存必须与正式索引分层
系统 MUST 将运行时缓存写入 `.wiki/.cache/`，且缓存缺失时不应使正式 Wiki 页面或 metadata 失去有效性。WikiState 持久化文件 MUST 存放在 `.wiki/.cache/wiki-state.json`，与 scan cache 和 module tree cache 同层。

#### Scenario: 缓存丢失但正式索引仍在
- **WHEN** `.wiki/.cache/` 被删除，但 `.wiki/*.md` 和 `wiki.metadata.json` 仍存在
- **THEN** 系统必须仍能执行 `status`（从 metadata 重建 WikiState）
- **THEN** 系统必须将需要重建缓存的情况与正式索引缺失区分开

#### Scenario: WikiState 缓存布局
- **WHEN** init、update、sync 或 rebuild 完成后
- **THEN** `.wiki/.cache/` MUST 包含 `repo-scan.json`、`module-tree.json` 和 `wiki-state.json`
