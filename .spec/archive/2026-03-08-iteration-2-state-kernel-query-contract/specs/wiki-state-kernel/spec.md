## ADDED Requirements

### Requirement: WikiState 必须作为所有 workflow 的内部事实主模型
系统 MUST 维护 `WikiState` 作为内部状态的唯一事实来源，承载页面状态、源码状态、模块列表、关系列表、脏状态和构建状态。所有 workflow（init / status / update / query / sync / rebuild）MUST 围绕 `WikiState` 工作，而不是直接操作 `WikiMetadata`。

#### Scenario: init 装配 WikiState
- **WHEN** 系统完成 init pipeline（扫描 → 模块树 → 页面规划 → 渲染）
- **THEN** 系统 MUST 从构建结果装配完整的 `WikiState`
- **THEN** `WikiState` MUST 包含所有页面的 `WikiPageState`（page_id、path、content_hash、source_ids、source_paths、module_ids）
- **THEN** `WikiState` MUST 包含所有源码的 `SourceState`（source_id、path、fingerprint、page_ids、module_ids）
- **THEN** `WikiState` MUST 包含模块列表、关系列表、`DirtyState::fresh()` 和 `BuildState`

#### Scenario: status 读取 WikiState
- **WHEN** 系统执行 status 检查
- **THEN** 系统 MUST 优先从持久化的 WikiState 读取页面和源码状态
- **THEN** 系统 MUST 基于 WikiState 中的 fingerprint 与当前扫描结果比对来判定脏状态

#### Scenario: query 消费 WikiState
- **WHEN** 系统执行 query
- **THEN** 系统 MUST 从 WikiState 构建查询索引，而不是从 WikiMetadata 构建
- **THEN** query 的结构化匹配逻辑 MUST 基于 WikiState 中的页面、模块、源码和关系数据

#### Scenario: sync 更新 WikiState
- **WHEN** 用户修改 `.wiki/*.md` 后执行 sync
- **THEN** 系统 MUST 更新 WikiState 中对应页面的 content_hash
- **THEN** 系统 MUST 通过 MetadataMapper 将更新后的 WikiState 导出为 WikiMetadata

### Requirement: WikiState 必须持久化到 `.wiki/.cache/wiki-state.json`
系统 MUST 将 `WikiState` 序列化写入 `.wiki/.cache/wiki-state.json`。WikiState 文件丢失时，系统 MUST 能够从 `WikiMetadata` 重建或通过重新 init 恢复。

#### Scenario: WikiState 持久化
- **WHEN** init、update、sync 或 rebuild 完成后
- **THEN** 系统 MUST 将当前 WikiState 写入 `.wiki/.cache/wiki-state.json`

#### Scenario: WikiState 丢失时回退
- **WHEN** `.wiki/.cache/wiki-state.json` 不存在但 `wiki.metadata.json` 存在
- **THEN** 系统 MUST 能够从 WikiMetadata 重建 WikiState
- **THEN** 重建后的 WikiState MUST 包含足够信息支撑 status 和 query 工作

#### Scenario: WikiState 和 metadata 都丢失
- **WHEN** `.wiki/.cache/wiki-state.json` 和 `wiki.metadata.json` 都不存在
- **THEN** 系统 MUST 报告 runtime missing，引导用户执行 init

### Requirement: WikiState 内部字段不得泄漏到外部协议
`WikiState` 中仅服务内部运算的字段（如 `source_ids`、`SourceState.page_ids`、`BuildState`）MUST NOT 出现在 JSON IPC 响应或 `wiki.metadata.json` 中，除非通过 MetadataMapper 显式映射。

#### Scenario: 外部协议不暴露内部字段
- **WHEN** Agent 通过 JSON IPC 调用任意 workflow
- **THEN** 响应结构 MUST 与迭代 1 保持兼容
- **THEN** 响应中 MUST NOT 包含 WikiState 的内部结构或 BuildState 细节
