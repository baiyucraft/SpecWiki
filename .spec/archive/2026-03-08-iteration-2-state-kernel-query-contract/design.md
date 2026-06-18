## Context

迭代 1 已完成 deterministic structural baseline。当前所有 workflow 直接拼装和读取 `WikiMetadata`（`domain/metadata.rs`），它同时充当内部状态模型和外部导出格式。`WikiState`（`domain/state.rs`）已有骨架定义但未被任何 workflow 使用。

现状问题：

- `init.rs` 在 120-142 行手工拼装 `WikiMetadata`，状态逻辑与导出逻辑混杂
- `status.rs`、`query.rs`、`sync.rs` 都通过 `read_metadata()` 读取 `WikiMetadata` 作为内部事实来源
- `WikiMetadata` 包含外部展示字段（`language`、`branch`、`last_indexed_commit`）和内部状态字段（`dirty_state`、`source_files`），职责不清
- query 的输出虽然已有结构化匹配，但数据来源仍是 metadata 而非独立状态层，后续增量更新无法在不影响导出格式的前提下扩展内部字段

参考实现：

- `deepwiki-rs`：workflow 围绕内部 state/memory 工作，cache 和 outlet 是独立层
- `CodeWiki`：内部模型和导出模型分离，生成结果先写入内部状态再导出

## Goals / Non-Goals

Goals:

- `WikiState` 成为所有 workflow 的事实主模型，承载页面、源码、模块、关系、脏状态和构建状态
- 引入 `MetadataMapper`，建立 `WikiState -> WikiMetadataExport` 的单向导出链
- 所有 workflow 迁移到围绕 `WikiState` 工作，外部协议（JSON IPC 响应结构）不变
- query 切换到消费 `WikiState`，输出统一围绕结构化上下文和 provenance
- cache 与状态层边界明确：cache 可丢弃，状态层是事实来源
- 为迭代 3（增量更新）准备 fingerprint 和 invalidation 边界

Non-Goals:

- 不实现增量更新逻辑（迭代 3）
- 不实现 managed section 和用户内容保留（迭代 4）
- 不引入 LLM 增强或 TOON/RAG 格式（迭代 5）
- 不修改 Agent 层代码（外部协议不变）
- 不引入数据库或外部存储依赖
- 不改变 `.wiki/*.md` 页面的生成内容和格式

## Decisions

### 决策 1：WikiState 持久化方式

选项：

- A：WikiState 写入独立文件 `.wiki/.state/wiki-state.json`
- B：WikiState 不持久化，每次从 cache + metadata 重建
- C：WikiState 写入 `.wiki/.cache/wiki-state.json`

选择 C。理由：

- WikiState 是内部运行时模型，不应出现在 `.wiki/` 正式层（排除 A）
- 每次重建成本高且丧失了状态层的意义（排除 B）
- 放在 `.cache/` 符合"可丢弃但有价值"的定位，cache 丢失时可从 init 重建

### 决策 2：WikiState 与 WikiMetadata 的关系

当前 `WikiMetadata` 同时是内部模型和外部格式。迁移策略：

- `WikiMetadata` 保留为外部导出格式（重命名概念为 `WikiMetadataExport`，但 struct 名暂不改，避免大面积重命名）
- 新增 `MetadataMapper` 模块（`domain/metadata_mapper.rs`），提供 `fn export_metadata(state: &WikiState, ...) -> WikiMetadata`
- `init` 先装配 `WikiState`，再通过 `MetadataMapper` 导出 `WikiMetadata` 写盘
- `status` / `query` / `sync` 优先读取 `WikiState`，WikiState 不存在时回退到从 `WikiMetadata` 重建

备选方案（直接重命名 `WikiMetadata` 为 `WikiMetadataExport`）被否决，因为会导致大面积 import 变更且对外部 JSON 格式无影响。

### 决策 3：WikiState 的装配时机

`WikiState` 在 `init` pipeline 的最后一步装配，输入来自：

- `pages`（PlannedPage + PageContext + 渲染结果）→ `WikiPageState`
- `scan_report.files` → `SourceState`
- `module_tree.modules` → 直接复用 `ModuleNode`
- `module_tree.cross_module_edges` + 页面父子关系 → `WikiRelation`
- 构建时间 + 计数 → `BuildState`
- 初始 → `DirtyState::fresh()`

这与当前 `init.rs` 120-142 行的拼装逻辑对应，但抽取为独立函数。

### 决策 4：Workflow 迁移顺序

按依赖关系分步迁移，每步保持外部协议不变：

1. 先实现 `WikiState` 装配和持久化（`state_store.rs`）
2. 迁移 `init`：装配 WikiState → 持久化 → 通过 MetadataMapper 导出 metadata
3. 迁移 `status`：优先读 WikiState，回退读 metadata
4. 迁移 `query`：切换到消费 WikiState，统一输出结构
5. 迁移 `sync`：更新 WikiState 后再导出 metadata
6. `update` / `rebuild` 当前都是调用 `init`，自动跟随迁移

### 决策 5：Query 输出统一

当前 `QueryReport` 已有 `matched_pages`、`matched_modules`、`matched_sources`、`matched_relations`、`matches` 五个字段。本迭代的调整：

- `QueryMatch` 增加 `context_pack` 字段：包含该页面关联的模块摘要、关键源码路径和关系证据，让 Agent 不需要二次查询
- `QueryReport` 增加 `provenance_summary` 字段：整体命中来源的结构化摘要
- query 内部从 `read_metadata()` 切换到 `read_state()`，回退到 metadata 重建 state
- 输出结构保持向后兼容（新增字段，不删除旧字段）

### 决策 6：Cache 与状态层分层

当前 cache 包含 `repo-scan.json` 和 `module-tree.json`。本迭代新增 `wiki-state.json`。

分层规则：

- `.wiki/.cache/repo-scan.json`：扫描快照，可丢弃，丢失后重新扫描
- `.wiki/.cache/module-tree.json`：模块树快照，可丢弃，丢失后重新构建
- `.wiki/.cache/wiki-state.json`：状态内核，可丢弃，丢失后从 metadata 重建或重新 init
- `.wiki/wiki.metadata.json`：正式索引导出，不可丢弃，由 MetadataMapper 从 WikiState 生成

## Risks / Trade-offs

- [WikiState 与 WikiMetadata 数据冗余] → 两者包含重叠信息（pages、modules、relations）。这是有意为之：WikiState 面向内部运算（含 source_ids、fingerprint 等），WikiMetadata 面向外部消费（含 language、branch 等展示字段）。MetadataMapper 是唯一的同步点。
- [回退路径复杂度] → status/query 需要处理"WikiState 存在"和"WikiState 不存在但 metadata 存在"两种情况。通过统一的 `load_or_rebuild_state()` 函数封装，避免每个 workflow 重复处理。
- [迁移期间测试断裂] → 内部模型变化会导致现有测试需要适配。策略是先保持外部协议不变，逐步迁移，每步都跑通现有测试。
- [WikiState 文件体积] → 大型仓库的 WikiState 可能较大。当前阶段不做压缩或分片，后续如有需要可在迭代 3 引入。
