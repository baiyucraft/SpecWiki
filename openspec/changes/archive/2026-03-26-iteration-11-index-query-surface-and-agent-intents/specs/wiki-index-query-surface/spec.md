## ADDED Requirements

### Requirement: `wiki-index` 必须提供统一的 index-first 查询意图
系统 MUST 在 `wiki-index` 内定义正式查询 contract，至少支持 `auto`、`symbol_lookup`、`source_lookup`、`module_lookup`、`entrypoint_lookup`、`callers`、`callees` 与 `impact_slice`。相同查询场景 MUST 复用同一套 intent 名称与结果类型，`wiki-runtime` 或其他调用方 MUST NOT 再定义平行 taxonomy。

#### Scenario: `symbol_lookup` 返回正式符号命中
- **WHEN** 调用方向 `wiki-index` 提交 `symbol_lookup`，且目标仓库存在同名稳定符号
- **THEN** 系统 MUST 返回 `SymbolHit`
- **THEN** `SymbolHit` MUST 至少包含稳定 `symbol_id`、符号名、标签和锚点源码路径

#### Scenario: `callers`、`callees` 与 `impact_slice` 走统一图查询意图
- **WHEN** 调用方向 `wiki-index` 提交 `callers`、`callees` 或 `impact_slice`
- **THEN** 系统 MUST 返回对应的 `CallEdgeHit` 或 `ImpactSlice`
- **THEN** 系统 MUST NOT 把这些场景重新折叠回泛化的 `graph_neighbors`

### Requirement: `wiki-index` 查询必须只消费 facts snapshot
系统 MUST 只使用 `scan_report`、`module_tree`、`modules / module_source_map`、`symbols / edges / communities / processes` 等 facts snapshot 回答查询，而 MUST NOT 依赖 `WikiState`、页面关系或 page fallback 数据。

#### Scenario: runtime page 状态缺失时仍可回答 index 查询
- **WHEN** 仓库已经提交了 facts snapshot，但 runtime page 状态缺失、downstream 未完成或 page fallback 不可用
- **THEN** `wiki-index` 查询 MUST 仍能返回 `symbol_lookup`、`source_lookup`、`module_lookup`、`entrypoint_lookup`、`callers`、`callees` 或 `impact_slice` 的正式结果
- **THEN** 系统 MUST NOT 因缺失 `WikiState` 或页面关系而拒绝这些 index-first 查询

#### Scenario: facts snapshot 尚未提交时返回显式未就绪错误
- **WHEN** 调用方向 `wiki-index` 提交查询，但当前仓库尚未完成首次 facts snapshot 提交
- **THEN** 系统 MUST 返回显式 `index not ready` 错误
- **THEN** 系统 MUST NOT 把“snapshot 未就绪”伪装成空命中成功

### Requirement: `wiki-index` 必须提供 source、module 与 entrypoint 的正式命中结果
系统 MUST 为 `source_lookup`、`module_lookup` 与 `entrypoint_lookup` 提供正式结果结构，并通过 index-owned 的 `module_tree`、`module_source_map` 与 `scan_report` 回答这些查询，而不是从页面状态倒推出来源信息。

#### Scenario: `source_lookup` 返回 source 与 module 归属
- **WHEN** 调用方向 `wiki-index` 提交 `source_lookup`，且输入命中某个已扫描源码路径
- **THEN** 系统 MUST 返回 `SourceHit`
- **THEN** `SourceHit` MUST 至少包含稳定 `source_id`、源码路径和所属 `module_ids`

#### Scenario: `entrypoint_lookup` 返回入口路径与模块归属
- **WHEN** 调用方向 `wiki-index` 提交 `entrypoint_lookup`，且仓库存在稳定入口路径
- **THEN** 系统 MUST 返回 `EntrypointHit`
- **THEN** `EntrypointHit` MUST 至少包含入口路径、命中依据和所属 `module_ids`

### Requirement: `wiki-index` 结果字段必须与当前 facts substrate 对齐
系统 MUST 只暴露当前仓库底层 facts substrate 能稳定产出的命中字段。所有命中 MUST 包含 `match_basis`；FTS 驱动的 symbol/source 命中 MUST 在可用时暴露 `score`；graph-derived 的 `callers`、`callees` 与 `impact_slice` MUST 在可用时暴露 `confidence` 与 `reason`。系统 MUST NOT 伪造跨场景统一的 `tier` 语义。

#### Scenario: FTS 驱动命中只暴露稳定 `score`
- **WHEN** 某个 `symbol_lookup` 或 `source_lookup` 通过当前 FTS 路径命中
- **THEN** 返回结果 MUST 包含对应 `match_basis`
- **THEN** 返回结果 MAY 包含 `score`
- **THEN** 系统 MUST NOT 为这类命中伪造统一 `confidence` 或 `tier`

#### Scenario: graph-derived 命中保留现有 `confidence / reason`
- **WHEN** 某个 `callers`、`callees` 或 `impact_slice` 结果来自已解析的 symbol graph
- **THEN** 返回结果 MUST 透传底层已有的 `confidence` 与 `reason`
- **THEN** `ImpactSlice` MUST 标记是否发生截断，并保留 supporting edges
