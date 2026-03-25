# workspace-crate-boundaries Specification

## Purpose
TBD - created by archiving change iteration-10-four-package-boundaries-and-foundation-smoke. Update Purpose after archive.
## Requirements
### Requirement: 工作区必须正式拆成四个 crate 并遵守固定依赖方向
系统 MUST 将 Rust 工作区正式拆成 `wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime` 四个 crate，并满足固定依赖方向：`wiki-model <- wiki-index <- wiki-knowledge <- wiki-runtime`。系统 MUST NOT 保留以 `wiki-core` 为中心的单包主实现，也 MUST NOT 引入 `wiki-runtime -> wiki-index/knowledge` 之外的反向依赖。

#### Scenario: 工作区声明四个 crate
- **WHEN** 开发者检查根级 `Cargo.toml` 与各 crate manifest
- **THEN** 工作区 MUST 明确包含 `wiki-model`、`wiki-index`、`wiki-knowledge`、`wiki-runtime`
- **THEN** `wiki-runtime` MUST 依赖 `wiki-index` 与 `wiki-knowledge`
- **THEN** `wiki-index` 与 `wiki-knowledge` MUST NOT 反向依赖 `wiki-runtime`

#### Scenario: 旧 `wiki-core` 不再作为主实现存在
- **WHEN** 开发者构建或测试当前工作区
- **THEN** 正式主实现 MUST 不再以 `wiki-core` 单包承担 facts、knowledge 与 runtime 全链路
- **THEN** 系统 MUST NOT 通过长期 re-export 或并行旧入口保留 `wiki-core` 主实现

### Requirement: 对象归属必须按 crate 边界固定且禁止 helper 偷渡
系统 MUST 以对象归属矩阵固定四层边界。`wiki-model` 只允许承载稳定共享对象与正式状态/元数据 DTO；`wiki-knowledge` 只允许承载 KnowledgeUnit 主线上的 planning/research/compose 合同；`wiki-runtime` 只允许承载 orchestration、projection、storage、transport 与 lifecycle 对象。`wiki-model` MUST NOT 依赖 IO、SQL、transport、prompt、render helper。

#### Scenario: 共享模型与 runtime helper 分离
- **WHEN** 系统迁移 `WikiState`、`WikiMetadata`、`DirtyState` 或等价正式对象到 `wiki-model`
- **THEN** `wiki-model` 中 MUST 只保留正式 DTO
- **THEN** builder、state assembly、metadata export、page merge 或其它 runtime helper MUST 留在 `wiki-runtime`

#### Scenario: projection decision 不得冒充 runtime 真相
- **WHEN** 系统迁移 `PlannedPage` 或等价 projection decision
- **THEN** 该对象 MUST 只作为 knowledge-side projection contract 存在
- **THEN** runtime/query MUST 使用 `WikiPageState` 或等价 page-state view 作为正式状态
- **THEN** 系统 MUST NOT 把 `PlannedPage` 直接持久化为 runtime 真相对象

### Requirement: runtime 不得重新导出 index 或 knowledge 的内部实现
系统 MUST 让 `wiki-runtime` 只暴露 workflow、storage adapter、transport、query route 骨架与 lifecycle API surface。`wiki-runtime` MUST NOT 重新导出 `wiki-index` 或 `wiki-knowledge` 的内部实现模块给外部直接使用。

#### Scenario: runtime 只暴露壳层 API
- **WHEN** 外部调用方依赖 `wiki-runtime`
- **THEN** 调用方 MUST 只能通过 runtime 提供的正式 workflow、storage adapter、transport 或 lifecycle API 使用系统
- **THEN** 调用方 MUST NOT 通过 `wiki-runtime` 跳转访问 `wiki-index` 或 `wiki-knowledge` 的内部实现模块

