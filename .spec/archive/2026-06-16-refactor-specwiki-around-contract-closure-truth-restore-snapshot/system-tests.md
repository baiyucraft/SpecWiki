# refactor-specwiki-around-contract-closure-truth-restore-snapshot 系统测试用例

## 用例总览

本用例覆盖 runtime truth / restore / snapshot / readiness 合同：Level 1 restore 只能恢复 knowledge / projection / diagnostic mirror，不能声明 index graph ready；Level 2 graph rebuild 完成后才允许 index ready；manifest、metadata、page hash 与 formal artifacts 不一致时必须显式阻断；query 在 index 不 ready 时不得返回 graph 命中。

## 系统测试用例

### ST-001 Level 1 restore 不伪装 index ready

- 关联成功标准: 删除 `.wiki/.cache/**` 后，Level 1 restore 不得把 `index_readiness` 标记为 `ready`；status 能表达 knowledge / projection 可消费。
- 覆盖设计点: Restore Outcome、Runtime Readiness、两级 restore。
- 前置条件: 仓库完成 init，`.wiki/.knowledge/**`、official page tree、`wiki.metadata.json` 和 snapshot manifest 均存在。
- 操作 / 触发: 删除 `.wiki/.cache/**` 后运行 status。
- 期望结果: `readiness.index` 为 `missing` 或 `stale`，`readiness.knowledge` 和 `readiness.projection` 为 `ready`，`readiness.restored_level` 为 `level1`，`recommended_action` 为 `rebuild` 或 `update`。
- 验证方式: Rust runtime 单元/集成测试，命令 `cargo test -p wiki-runtime status_restores_level1_without_index_ready --test runtime`。

### ST-002 Level 2 rebuild 才能声明 index ready

- 关联成功标准: Level 2 graph rebuild 完成后才能允许 index graph ready。
- 覆盖设计点: index graph readiness marker、state_store graph origin。
- 前置条件: ST-001 的 Level 1 restore 状态。
- 操作 / 触发: 运行 rebuild，然后运行 status。
- 期望结果: `readiness.index`、`readiness.knowledge`、`readiness.projection` 均为 `ready`，`readiness.restored_level` 为 `level2` 或 `none`，`recommended_action` 为 `none`。
- 验证方式: Rust runtime 单元/集成测试，命令 `cargo test -p wiki-runtime rebuild_marks_index_readiness_ready_after_level2 --test runtime`。

### ST-003 committed snapshot manifest 是 restore 主记录

- 关联成功标准: committed snapshot manifest 主记录位于 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml`；`wiki.metadata.json` 不保存 runtime gates / projection digest / snapshot 主状态。
- 覆盖设计点: Snapshot Manifest、metadata 边界、serde_yaml。
- 前置条件: 仓库完成 init。
- 操作 / 触发: 检查 manifest 路径和 metadata 内容。
- 期望结果: `manifest.yaml` 存在且可解析，包含 snapshot / metadata / declared / knowledge 锚点；旧 JSON recovery manifest 不生成；metadata 只保存 current snapshot pointer 与 binding index。
- 验证方式: Rust artifact roundtrip 测试，命令 `cargo test -p wiki-runtime committed_snapshot_manifest_is_yaml_restore_truth --test runtime`。

### ST-004 restore guard 对锚点不一致显式阻断

- 关联成功标准: manifest、metadata binding、page hashes 或 formal artifacts 锚点不一致时 restore 进入 diagnostic / blocked / needs_update，不静默 full init 或 restore success。
- 覆盖设计点: Restore Guards、Restore Outcome reason。
- 前置条件: 仓库完成 init 并持久化 snapshot manifest。
- 操作 / 触发: 修改 metadata hash、page content、declared snapshot 或 knowledge snapshot 后删除 `.cache` 并触发 restore/status。
- 期望结果: restore outcome `restored_cache=false`，readiness 包含 `blocked/conflict/stale` 与机器可读 reason，status 不输出 all ready。
- 验证方式: Rust artifact roundtrip/status 测试，命令 `cargo test -p wiki-runtime restore_blocks_on_snapshot_anchor_mismatch --test runtime`。

### ST-005 query 在 index 不 ready 时禁用 graph 命中

- 关联成功标准: query 在 index graph 不 ready 时不能伪装 graph 命中；允许 Level 1 degraded knowledge/projection 消费。
- 覆盖设计点: Query Degradation、Runtime Readiness。
- 前置条件: 仓库完成 init 后删除 `.cache`，Level 1 restore 可用。
- 操作 / 触发: 查询页面或知识相关 term。
- 期望结果: `readiness.index != ready`，`query_trust=stale_but_queryable` 或 `blocked`，`answer.answer_mode=degraded`；结果不包含 `index_hit`、`graph_hit`、symbol edge 或 graph edge。
- 验证方式: Rust query 测试，命令 `cargo test -p wiki-runtime query_degrades_without_graph_hits_when_index_missing --test runtime`。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| Level 1 restore 不得把 index 标记为 ready | ST-001 | Rust runtime status 测试 |
| Level 1 restore 可表达 knowledge / projection 可诊断或可消费 | ST-001 | Rust runtime status 测试 |
| Level 2 rebuild 完成后 index ready | ST-002 | Rust runtime rebuild/status 测试 |
| committed snapshot manifest 为 YAML 主记录 | ST-003 | Rust artifact roundtrip 测试 |
| metadata 不保存 projection digest/runtime gates/snapshot 主状态 | ST-003 | Rust metadata artifact 测试 |
| manifest / metadata / page / formal artifacts 不一致显式阻断 | ST-004 | Rust restore guard 测试 |
| query 不伪装 graph 命中 | ST-005 | Rust query 测试 |
| 旧 facts_ready/query_readiness 断言改为分层 readiness | ST-001, ST-005 | Rust status/query contract 测试 |
| `unispec validate` 通过 | ST-001-ST-005 | `unispec validate refactor-specwiki-around-contract-closure-truth-restore-snapshot` |

## 边界与异常

- `.wiki/pages/**` 不参与本 change 的 restore、诊断或查询。
- restore guard 失败必须返回 outcome reason，不允许通过重新 init 掩盖。
- health degradation 可以继续影响 `recommended_action`，但不得覆盖 index/knowledge/projection 分层状态。

## 验证数据与环境

- 使用 `tempfile` 创建临时 repo fixture。
- 使用 `run_init`、`run_rebuild`、`run_status`、`run_query` 作为系统级 workflow 入口。
- 使用现有 `.wiki/.knowledge/**` artifact loader/writer 和 SQLite store。

## 未覆盖项

- query route tags / result DTO 全量升级留给后续 query-route child change。
- code graph schema 深度调整留给后续 code-graph-index child change。

## 参考资料

- `proposal.md`
- `design.md`
