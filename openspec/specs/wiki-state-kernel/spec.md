# wiki-state-kernel Specification

## Purpose
定义 WikiState 作为 runtime 内部事实主模型的正式边界，说明状态持久化、恢复与 workflow 消费应遵循的基础约束。

## Requirements

### Requirement: WikiState 必须作为所有 workflow 的内部事实主模型
系统 MUST 继续维护 `WikiState` 作为内部状态的唯一事实主模型，承载页面状态、section 状态、源码状态、模块列表、关系列表、脏状态和构建状态。所有 workflow（init / status / update / query / sync / rebuild）MUST 围绕 `WikiState` 工作，而不是直接操作 `WikiMetadata`。与迭代 6 不同的是，`WikiState` 的持久化和恢复 MUST 基于关系型状态表组装，而不是单条 JSON blob。为了驱动 editable runtime，`WikiState` 还 MUST 为每个页面维护稳定的 `section_anchors` 聚合字段，并保持 `source -> module -> page -> section` 映射可重建。

#### Scenario: init 装配并持久化 WikiState
- **WHEN** 系统完成 init pipeline（扫描 → 模块树 → 页面规划 → 渲染）
- **THEN** 系统 MUST 从构建结果装配完整的 `WikiState`
- **THEN** `WikiState` MUST 被拆分持久化到页面、section、源码、模块、映射和关系表
- **THEN** 每个页面状态 MUST 记录 `page_id`、`path`、`content_hash`、`input_hash`、`source_ids`、`module_ids`、`sections` 和 `section_anchors`

#### Scenario: query 消费行式持久化恢复的 WikiState
- **WHEN** 系统执行 query
- **THEN** 系统 MUST 先从关系型状态表恢复 `WikiState`
- **THEN** query 的结构化匹配逻辑 MUST 基于恢复后的页面、模块、源码和关系数据执行

#### Scenario: sync 更新页面 section 与 section anchors
- **WHEN** 用户修改 `.wiki/*.md` 后执行 sync
- **THEN** 系统 MUST 更新目标页面的 `content_hash`、`summary`、`sections` 和 `section_anchors`
- **THEN** 更新后的页面状态 MUST 能被后续 `change_set` 和 `update` 直接消费

### Requirement: WikiState 必须维护稳定的页面输入与 section 指纹
系统 MUST 为每个页面维护稳定的 `input_hash`，并为每个 section 维护可区分 generated 与 observed 内容的稳定指纹。对于 managed section，系统 MUST 同时维护最近一次生成内容的 `generated_content_hash` 与当前磁盘内容的 `content_hash`；对于 user section，系统 MUST 维护当前内容 hash 和锚点信息。页面级 `section_anchors` MUST 反映该页面当前受 runtime 管理的稳定锚点集合，并可用于 section-level 脏检测和 user section 回插。

#### Scenario: 页面输入未变化
- **WHEN** 某个页面的输入事实、关系和 managed section 输入都未变化
- **THEN** 系统 MUST 保持该页面的 `input_hash`、managed section 的 `generated_content_hash` 和页面级 `section_anchors` 不变
- **THEN** update 不得把该页面误判为需要重生成

#### Scenario: 只有 user section 发生变化
- **WHEN** 用户只修改了页面中的 user section 并执行 `sync`
- **THEN** 系统 MUST 更新该 user section 的 `content_hash`
- **THEN** 系统 MUST 保持未变化 managed sections 的 `generated_content_hash` 不变
- **THEN** 系统 MUST 保持页面级 `section_anchors` 与 managed section 序列一致

#### Scenario: managed section 发生磁盘漂移
- **WHEN** 用户直接改写了 managed section 的正文并执行 `sync`
- **THEN** 系统 MUST 更新该区段当前磁盘内容对应的 `content_hash`
- **THEN** 系统 MUST 保留该区段原有的 `generated_content_hash`
- **THEN** 系统 MUST 能据此识别该区段存在 managed drift，并保持对应锚点仍可用于后续重组
