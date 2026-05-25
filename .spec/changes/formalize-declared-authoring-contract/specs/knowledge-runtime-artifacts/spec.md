## MODIFIED Requirements

### Requirement: 正式知识产物必须与 metadata 共享稳定 snapshot 身份
系统 MUST 让 `.wiki/.knowledge/**` 与 `wiki.metadata.json` 共享稳定的 snapshot 身份线索，用于审计、恢复和一致性校验。`recovery-manifest` MUST 至少包含 `schema_version`、`repo_root`、`facts_input_hash`、`knowledge_snapshot_id`、`metadata_hash` 或等价锚点字段；当正式 snapshot 包含 declared artifact 时，这些锚点 MUST 同时覆盖 `.wiki/.knowledge/declared/**` 的 identity 与版本线索。系统 MUST 能基于这些字段判断当前正式产物是否可用于 restore，而不是依赖目录存在与否盲猜。

#### Scenario: restore 前校验 declared snapshot 一致性
- **WHEN** 系统尝试基于 `.wiki/.knowledge/** + pages + metadata` 恢复本地 runtime，且其中包含 declared artifact
- **THEN** 系统 MUST 先校验 declared snapshot 与 `wiki.metadata.json` / `recovery-manifest` 的锚点是否一致
- **THEN** 若 declared 锚点不一致，系统 MUST 返回显式 `stale`、`needs_update` 或 blocker，而不是静默继续恢复

### Requirement: restore 必须从正式产物恢复本地 runtime，而不是重新生成知识
系统 MUST 支持在 `.wiki/.cache/**` 缺失、被删除或需要重建时，仅凭 `.wiki/.knowledge/** + .wiki/pages/** + wiki.metadata.json` 恢复本地 runtime。该 restore 语义 MUST 是“重建本地 `.cache` 与可消费状态”，而不是重新执行 planning、research、compose 或 assemble。系统 MUST 直接消费 `.wiki/.knowledge/declared/**`、`derived/**` 与 `runtime/**` 的正式对象恢复 declared / derived / projection 关系，MUST NOT 通过重新扫描页面正文来反推 declared truth，也 MUST NOT 因 `.cache` 缺失就自动退回 full `init` 并把它伪装成 restore 成功。

#### Scenario: B 用户从正式产物恢复 declared runtime
- **WHEN** 仓库中已存在可读取的 `.wiki/.knowledge/declared/**`、`.wiki/.knowledge/derived/**`、`.wiki/pages/**` 与 `wiki.metadata.json`，但本地 `.wiki/.cache/**` 缺失
- **THEN** 系统 MUST 仅基于这些正式产物重建本地 `.wiki/.cache/**`
- **THEN** restore MUST NOT 重新调用 planning、research 或 compose 主链

#### Scenario: restore 不得依赖页面正文反推 declared
- **WHEN** 系统恢复本地 runtime 中的 declared state
- **THEN** 系统 MUST 直接读取 `.wiki/.knowledge/declared/**` 中的正式 artifact
- **THEN** 系统 MUST NOT 通过重新解析页面正文或 managed section 来重建 declared truth

### Requirement: 最小正式知识产物集不得混入工作态缓存与 declared lifecycle
系统 MUST 让 `.wiki/.knowledge/**` 保持正式知识产物定位。本轮正式 contract MUST 明确覆盖 `declared/**`、`derived/**` 与 `runtime/**` 的最小对象集，其中 `declared/**` MUST 承载正式 declared lifecycle contract；与此同时，`page_drafts`、`llm_cache`、临时 session state、工作中的 compose artifact 和其它工作态缓存 MUST 继续留在 `.wiki/.cache/**` 或等价本地 working state，MUST NOT 混入 `.wiki/.knowledge/**` 的正式 contract。

#### Scenario: declared lifecycle 进入正式知识产物集
- **WHEN** 仓库中存在至少一条正式 declared record
- **THEN** `.wiki/.knowledge/declared/**` MUST 成为正式 snapshot 组成部分
- **THEN** declared 的 scope、status 与 relation lifecycle MUST 能被持久化与恢复

#### Scenario: 工作态缓存继续留在 `.cache`
- **WHEN** 系统持久化 `page_drafts`、`llm_cache`、临时 session state 或其它工作态缓存
- **THEN** 这些对象 MUST 继续留在 `.wiki/.cache/**` 或等价本地 working state
- **THEN** 系统 MUST NOT 把它们升级为 `.wiki/.knowledge/**` 的正式对象

### Requirement: 正式知识产物必须补充 declared 与 health artifacts
系统 MUST 在 `.wiki/.knowledge/**` 的最小正式产物集中补充 `declared/**` 与 health artifacts，而不再只稳定承诺 `derived/**` 与 `runtime/**`。最小补充对象 MUST 至少包括：结构化 `declared records` 与可供 `status` / `query` / `sync` 诊断消费的 `health signals` 摘要。每条正式 declared artifact MUST 至少包含 `record_id`、`record_kind`、typed `scope`、`status`、`relations`、`source_ref`、`unit_refs`、`projection_refs` 与 `updated_at` 或等价稳定字段。

#### Scenario: declared artifact 成为正式 snapshot 组成部分
- **WHEN** 仓库中存在至少一条正式 declared record
- **THEN** `.wiki/.knowledge/**` 的正式 snapshot MUST 包含对应 declared artifact
- **THEN** restore 或 audit MUST 能直接消费该对象，而不依赖页面正文反推

#### Scenario: declared artifact 可 roundtrip 恢复关系与范围
- **WHEN** 系统读取已持久化的 declared artifact
- **THEN** runtime MUST 能恢复其 scope、状态与 `supersedes / replaced_by / deprecated` 关系
- **THEN** 调用方 MUST 不需要附加 page-side 补丁才能还原 declared lifecycle

#### Scenario: health artifact 进入正式 runtime snapshot
- **WHEN** 当前 runtime 检测到 orphan unit、stale projection 或 declared/derived 不一致
- **THEN** `.wiki/.knowledge/runtime/**` 或等价正式层 MUST 写出可聚合的 health artifact
- **THEN** 系统 MUST NOT 只把这些问题留在临时日志或内存态
