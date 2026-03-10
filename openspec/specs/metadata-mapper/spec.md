# metadata-mapper Specification

## Purpose
定义 MetadataMapper 模块，提供 WikiState 到 WikiMetadata 的单向导出链，确保所有 workflow 通过统一路径生成 wiki.metadata.json。

## Requirements
### Requirement: MetadataMapper 必须提供 WikiState 到 WikiMetadata 的单向导出
系统 MUST 提供 `MetadataMapper` 模块，将 `WikiState` 映射为 `WikiMetadata`（外部导出格式）。所有 workflow MUST 通过 MetadataMapper 生成 `wiki.metadata.json`，而不是直接手工拼装 `WikiMetadata`。

#### Scenario: init 通过 MetadataMapper 导出 metadata
- **WHEN** init 完成 WikiState 装配后
- **THEN** 系统 MUST 调用 MetadataMapper 将 WikiState 转换为 WikiMetadata
- **THEN** 导出的 WikiMetadata MUST 包含 schema_version、language、repo_root、branch、generated_at、last_indexed_commit
- **THEN** 导出的 WikiMetadata MUST 包含从 WikiState 映射的 wiki_items、modules、relations、source_files、dirty_state

#### Scenario: sync 通过 MetadataMapper 更新 metadata
- **WHEN** sync 更新 WikiState 后
- **THEN** 系统 MUST 通过 MetadataMapper 重新导出 WikiMetadata 并写盘
- **THEN** 导出结果 MUST 反映 WikiState 中已更新的 content_hash

### Requirement: MetadataMapper 导出的 WikiMetadata 必须与迭代 1 格式兼容
MetadataMapper 导出的 `wiki.metadata.json` MUST 保持与迭代 1 的字段结构兼容。新增字段允许，但不得删除或重命名已有字段。

#### Scenario: 字段兼容性
- **WHEN** MetadataMapper 导出 WikiMetadata
- **THEN** 导出的 JSON 结构 MUST 包含迭代 1 定义的所有字段（schema_version、language、repo_root、branch、generated_at、last_indexed_commit、modules、wiki_items、relations、source_files、dirty_state）
- **THEN** wiki_items 中每个条目 MUST 包含 id、title、path、item_type、parent_id、ancestor_ids、module_ids、source_files、content_hash、summary、provenance

#### Scenario: WikiPageState 到 WikiItem 的映射
- **WHEN** MetadataMapper 将 WikiPageState 映射为 WikiItem
- **THEN** MUST 从 WikiState 中的 WikiPageState 提取 page_id → id、path、content_hash、source_ids → source_paths 映射、module_ids
- **THEN** MUST 从构建上下文中补齐 title、item_type、parent_id、ancestor_ids、summary、provenance 等展示字段

### Requirement: MetadataMapper 必须补齐外部展示字段
`WikiState` 不承载 `language`、`branch`、`last_indexed_commit` 等外部展示字段。MetadataMapper MUST 从构建上下文或环境信息中补齐这些字段。

#### Scenario: 补齐 Git 元信息
- **WHEN** MetadataMapper 导出 WikiMetadata
- **THEN** MUST 填充 repo_root、branch、last_indexed_commit（从 Git 或默认值获取）
- **THEN** MUST 填充 generated_at（当前时间戳）
- **THEN** MUST 填充 language（默认 "zh"）和 schema_version
