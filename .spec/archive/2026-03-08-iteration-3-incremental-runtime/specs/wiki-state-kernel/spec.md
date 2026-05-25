## MODIFIED Requirements

### Requirement: WikiState 必须作为所有 workflow 的内部事实主模型
系统 MUST 维护 `WikiState` 作为内部状态的唯一事实来源，承载页面状态、源码状态、模块列表、关系列表、脏状态和构建状态。所有 workflow（init / status / update / query / sync / rebuild）MUST 围绕 `WikiState` 工作，而不是直接操作 `WikiMetadata`。为了驱动增量 runtime，`WikiState` 还 MUST 承载稳定的 `source -> module -> page -> section` 映射、页面输入指纹和 section 级状态。

#### Scenario: init 装配 WikiState
- **WHEN** 系统完成 init pipeline（扫描 → 模块树 → 页面规划 → 渲染）
- **THEN** 系统 MUST 从构建结果装配完整的 `WikiState`
- **THEN** `WikiState` MUST 包含所有页面的 `WikiPageState`（page_id、path、content_hash、input_hash、source_ids、source_paths、module_ids、sections）
- **THEN** `WikiState` MUST 包含所有源码的 `SourceState`（source_id、path、fingerprint、page_ids、module_ids）
- **THEN** `WikiState` MUST 包含模块列表、关系列表、`DirtyState::fresh()` 和 `BuildState`

#### Scenario: status 读取 WikiState
- **WHEN** 系统执行 status 检查
- **THEN** 系统 MUST 优先从持久化的 WikiState 读取页面、section 和源码状态
- **THEN** 系统 MUST 基于 WikiState 中的映射和 fingerprint 计算受影响页面与 section

#### Scenario: query 消费 WikiState
- **WHEN** 系统执行 query
- **THEN** 系统 MUST 从 WikiState 构建查询索引，而不是从 WikiMetadata 构建
- **THEN** query 的结构化匹配逻辑 MUST 基于 WikiState 中的页面、模块、源码和关系数据

#### Scenario: update 使用 WikiState 做局部重生成
- **WHEN** 系统执行增量 update
- **THEN** 系统 MUST 通过 WikiState 中的 source/page/section 映射确定受影响页面
- **THEN** 系统 MUST 只更新受影响页面对应的页面状态和 section 状态

#### Scenario: sync 更新 WikiState
- **WHEN** 用户修改 `.wiki/*.md` 后执行 sync
- **THEN** 系统 MUST 更新 WikiState 中对应页面的 content_hash
- **THEN** 系统 MUST 通过 MetadataMapper 将更新后的 WikiState 导出为 WikiMetadata

### Requirement: WikiState 必须持久化到 `.wiki/.cache/wiki-state.json`
系统 MUST 将 `WikiState` 序列化写入 `.wiki/.cache/wiki-state.json`。WikiState 文件丢失时，系统 MUST 能够从 `WikiMetadata` 重建或通过重新 init 恢复。metadata 回退路径 MUST 足以支撑 status/query 对正式索引的理解；如果增量 runtime 所需字段或 companion cache 缺失，系统 MUST 显式提升为 `needs_rebuild` 或 full rebuild，而不是假装 runtime 仍然 fresh。

#### Scenario: WikiState 持久化
- **WHEN** init、update、sync 或 rebuild 完成后
- **THEN** 系统 MUST 将当前 WikiState 写入 `.wiki/.cache/wiki-state.json`

#### Scenario: WikiState 丢失时回退
- **WHEN** `.wiki/.cache/wiki-state.json` 不存在但 `wiki.metadata.json` 存在
- **THEN** 系统 MUST 能够从 WikiMetadata 重建足够的状态视图
- **THEN** 重建后的状态视图 MUST 足以支撑 status 和 query 理解正式索引
- **THEN** 如果增量 runtime 所需 cache 缺失，系统 MUST 将运行时状态提升为 `needs_rebuild`

#### Scenario: WikiState 和 metadata 都丢失
- **WHEN** `.wiki/.cache/wiki-state.json` 和 `wiki.metadata.json` 都不存在
- **THEN** 系统 MUST 报告 runtime missing，引导用户执行 init

## ADDED Requirements

### Requirement: WikiState 必须维护稳定的页面输入与 section 指纹
系统 MUST 为每个页面维护稳定的 `input_hash`，并为每个 section 维护稳定的 `content_hash`。这些指纹 MUST 可用于判断页面输入是否变化、section 是否需要重渲染，以及未受影响页面是否可以保持不写盘。

#### Scenario: 页面输入未变化
- **WHEN** 某个页面的输入事实、关系和 section 输入都未变化
- **THEN** 系统必须保持该页面的 `input_hash` 和 section hash 不变
- **THEN** update 不得把该页面误判为需要重生成

#### Scenario: 部分 section 输入变化
- **WHEN** 某个页面只有部分 section 的输入事实发生变化
- **THEN** 系统必须能识别出具体受影响的 section
- **THEN** 系统必须允许只重渲染这些 section 后再组装整页
