## Why

当前所有 workflow（init / status / query / sync / rebuild）直接拼装 `WikiMetadata` 并以此充当内部主模型。`WikiMetadata` 本应只是面向外部消费的导出格式，但实际上承担了内部状态管理、脏检测、页面追溯等职责，导致 workflow 之间的状态逻辑散落且难以统一演进。同时，query 的输出结构尚未围绕统一状态模型组织，Agent 作为核心消费者无法获得稳定的结构化上下文和 provenance。

本迭代需要在迭代 1 deterministic baseline 闭环后立即推进，因为：状态内核重构会重新定义内部模型，正好是统一 query 输出结构的最佳时机；拖延会导致后续迭代（增量更新、可编辑 Wiki）在不稳定的内部模型上堆叠逻辑。

## What Changes

- 引入 `WikiState` 作为内部事实主模型，承载页面、源码、模块、关系、脏状态和构建状态的完整映射
- 引入 `MetadataMapper`，建立 `WikiState -> WikiMetadataExport` 的稳定导出链，停止在 workflow 内直接手工拼装 metadata
- 迁移所有 workflow（init / status / update / sync / query / rebuild）到状态内核，保持外部协议不变
- 重新梳理 cache 与状态层的边界，让 cache 成为"可丢弃的复用层"，为后续增量更新准备 fingerprint 和 invalidation 边界
- query 切换到消费 `WikiState`，输出统一围绕页面、模块、源码、关系和 provenance 组织，补齐结构化命中说明和基础上下文打包
- **BREAKING**：`wiki.metadata.json` 的内部生成路径从 workflow 直接拼装改为 `WikiState -> MetadataMapper` 导出，外部字段结构保持兼容

## Capabilities

### New Capabilities

- `wiki-state-kernel`: 内部状态模型（WikiState / WikiPageState / SourceState / BuildState）的定义、装配、读写和持久化
- `metadata-mapper`: WikiState 到 WikiMetadataExport 的映射层，包括导出策略、字段兼容和 schema version 管理

### Modified Capabilities

- `repo-wiki-workflow`: 所有 workflow 从直接操作 metadata 迁移到围绕 WikiState 工作；query 输出统一为结构化上下文 + provenance
- `repo-wiki-runtime`: metadata 从内部主模型降级为由状态内核导出的外部格式；cache 与状态层边界重新明确

## Impact

- `crates/wiki-core/src/domain/`：新增 `state.rs`（WikiState 及子模型），修改 `metadata.rs`（WikiMetadata 降级为导出格式）
- `crates/wiki-core/src/workflows/`：所有 workflow 文件（init / status / update / query / sync / rebuild）内部实现迁移
- `crates/wiki-core/src/storage/`：新增状态持久化路径，调整 metadata_store 为导出层
- `crates/wiki-core/src/generation/`：可能需要调整 mapper 层，但生成 pipeline 本身不变
- Agent 层（`agents/codebuddy/`）：外部协议不变，不需要修改
- 测试：新增状态不变量测试和 query 输出结构测试，现有测试需要适配内部模型变化
