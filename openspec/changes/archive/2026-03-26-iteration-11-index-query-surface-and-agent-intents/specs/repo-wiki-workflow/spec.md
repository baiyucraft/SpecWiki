## ADDED Requirements

### Requirement: `init`、`update` 与 `rebuild` 必须在 knowledge planning 前提交 facts snapshot
系统 MUST 在 `init`、`update` 与 `rebuild` 的 facts 阶段完成后，先提交可恢复的 facts snapshot，再进入 `Knowledge Planning -> Research -> Compose`。该 snapshot MUST 至少覆盖 `scan_cache`、`module_tree`、`modules / module_source_map`、`symbols / edges / communities / processes`，而不得继续等到 assemble 或 `WikiState` 写盘之后才统一落盘。

#### Scenario: 全量 workflow 在 knowledge planning 前写入 facts snapshot
- **WHEN** 系统执行 `init` 或 `rebuild`
- **THEN** 系统 MUST 在 `scan`、`resolve/analyze graph` 与 `build_module_tree` 完成后先写入 facts snapshot
- **THEN** 系统 MUST 在 facts snapshot 可恢复后才进入 `knowledge_planning`、`research` 与 `compose`

#### Scenario: 增量 update 在 compose 前刷新受影响 facts snapshot
- **WHEN** 系统执行 scoped `update`，且只涉及有限数量的受影响源码文件
- **THEN** 系统 MUST 在进入 `knowledge_planning` 前先刷新这些文件相关的 `symbols / edges` 与对应的 `module_tree / module_source_map`
- **THEN** 系统 MUST NOT 把本轮 facts 刷新继续耦在最终 `WikiState` 写盘之后

### Requirement: downstream incomplete 或失败不得清空已提交的 facts snapshot
系统 MUST 将 facts readiness 与 downstream readiness 分离。只要 facts snapshot 已经提交，后续 `knowledge_planning`、`research`、`compose` 或 `assemble` 即使未完成、失败或中断，也 MUST 保留已经写好的 facts snapshot，并允许 index-first 查询继续工作。

#### Scenario: research 或 compose 中断后 facts snapshot 仍保留
- **WHEN** workflow 在 facts snapshot 提交之后于 `research` 或 `compose` 阶段失败或中断
- **THEN** 系统 MUST 保留已提交的 `modules / module_source_map / symbols / edges`
- **THEN** 后续 query MUST 仍可基于这些 snapshot 执行 index-first 查询

#### Scenario: downstream incomplete 状态不得伪装成 facts 为空
- **WHEN** runtime 当前处于 `researching`、`compose_pending` 或等价 downstream incomplete 状态
- **THEN** 系统 MUST 将该状态与 facts readiness 分离表达
- **THEN** 系统 MUST NOT 让 `modules`、`module_source_map`、`symbols` 或 `edges` 因下游未完成而表现为 `0`
