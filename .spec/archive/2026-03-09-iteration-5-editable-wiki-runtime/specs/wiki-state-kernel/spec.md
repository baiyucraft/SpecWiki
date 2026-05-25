## MODIFIED Requirements

### Requirement: WikiState 必须作为所有 workflow 的内部事实主模型
系统 MUST 维护 `WikiState` 作为内部状态的唯一事实来源，承载页面状态、源码状态、模块列表、关系列表、脏状态和构建状态。所有 workflow（init / status / update / query / sync / rebuild）MUST 围绕 `WikiState` 工作，而不是直接操作 `WikiMetadata`。为了驱动 editable runtime，`WikiState` 还 MUST 承载稳定的 `source -> module -> page -> section` 映射、页面输入指纹、managed/user section 状态、managed section 的 generated hash，以及 user section 的锚点信息。

#### Scenario: init 装配 WikiState
- **WHEN** 系统完成 init pipeline（扫描 → 模块树 → 页面规划 → 渲染）
- **THEN** 系统 MUST 从构建结果装配完整的 `WikiState`
- **THEN** `WikiState` MUST 包含所有页面的 `WikiPageState`（page_id、path、content_hash、input_hash、source_ids、source_paths、module_ids、sections）
- **THEN** `WikiState` MUST 包含所有源码的 `SourceState`（source_id、path、fingerprint、page_ids、module_ids）
- **THEN** `WikiState` MUST 包含模块列表、关系列表、`DirtyState::fresh()` 和 `BuildState`
- **THEN** `WikiState.sections` 中的 managed section 必须记录 generated hash，user section 必须记录锚点信息

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
- **THEN** 同页 user sections 必须从上一轮 WikiState 中恢复，而不是仅从新生成结果重建

#### Scenario: sync 更新 WikiState
- **WHEN** 用户修改 `.wiki/*.md` 后执行 sync
- **THEN** 系统 MUST 更新 WikiState 中对应页面的整页 `content_hash`
- **THEN** 系统 MUST 更新页面的 `summary` 和 section 状态
- **THEN** 系统 MUST 把用户新增内容写成 `managed = false` 的 section 状态
- **THEN** 系统 MUST 通过 MetadataMapper 将更新后的 WikiState 导出为 WikiMetadata

### Requirement: WikiState 必须维护稳定的页面输入与 section 指纹
系统 MUST 为每个页面维护稳定的 `input_hash`，并为每个 section 维护可区分 generated 与 observed 内容的稳定指纹。对于 managed section，系统 MUST 同时维护最近一次生成内容的 `generated_content_hash` 与当前磁盘内容的 `content_hash`；对于 user section，系统 MUST 维护当前内容 hash 和锚点信息。这些指纹 MUST 可用于判断页面输入是否变化、managed section 是否需要重渲染，以及 user section 是否只是被重新插回页面。

#### Scenario: 页面输入未变化
- **WHEN** 某个页面的输入事实、关系和 managed section 输入都未变化
- **THEN** 系统必须保持该页面的 `input_hash` 和 managed section 的 `generated_content_hash` 不变
- **THEN** update 不得把该页面误判为需要重生成

#### Scenario: 只有 user section 发生变化
- **WHEN** 用户只修改了页面中的 user section 并执行 `sync`
- **THEN** 系统必须更新该 user section 的 `content_hash`
- **THEN** 系统必须保持未变化 managed sections 的 `generated_content_hash` 不变

#### Scenario: managed section 发生磁盘漂移
- **WHEN** 用户直接改写了 managed section 的正文并执行 `sync`
- **THEN** 系统必须更新该区段当前磁盘内容对应的 `content_hash`
- **THEN** 系统必须保留该区段原有的 `generated_content_hash`
- **THEN** 系统必须能够据此识别该区段存在 managed drift
