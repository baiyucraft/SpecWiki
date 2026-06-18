# refactor-specwiki-around-contract-closure-truth-restore-snapshot 设计方案

## 方案概述

本 change 将 runtime 的恢复语义从“cache 恢复后就是 ready”改为分层合同：`.wiki/.knowledge/**`、official page tree、`wiki.metadata.json` 和 `.wiki/.cache/**` 各自有独立 truth kind；`.cache` 缺失后的 Level 1 restore 只能恢复 knowledge / projection / diagnostic runtime mirror，不能声明 index graph ready；只有从当前源码重新构建 SQLite graph 后，Level 2 才能把 index readiness 标记为 ready。

### 方案目标

- 固定 truth kind 与职责边界，避免 `wiki.metadata.json`、runtime gate、projection digest 和 cache mirror 互相承担主状态。
- 将 restore 分成 Level 1 knowledge/projection restore 与 Level 2 code graph rebuild。
- 引入 committed snapshot manifest 作为 shared runtime recovery truth 的主记录，替代 `wiki.metadata.json` 或 `.cache` 暗含 snapshot 主状态。
- 将公开 readiness 拆成 `index_readiness`、`knowledge_readiness`、`projection_readiness` 和现有 health / recommended action 的组合。
- 让 status/query 在 `.cache` 缺失、manifest stale、metadata mismatch、graph missing 时能给出机器可区分状态。

### 方案范围

- 覆盖 `status`、`query`、knowledge artifact restore、runtime state/cache restore、外部 transport 投影和相关测试断言。
- 不重构完整 query route DTO；query route tags 和 result DTO 全量升级留给 `refactor-specwiki-around-contract-closure-query-route-readiness`。
- 不处理旧 `.wiki/pages/**`，不迁移、不清理、不诊断。
- 不保留旧 manifest 双轨兼容；当前测试开发阶段允许直接替换。

## 架构分析

当前实现已经具备正式产物、restore guard 和部分 cache mirror 能力，但外部状态仍主要压缩成：

- `StatusReport.facts_ready`
- `StatusReport.query_readiness`
- `QueryReport.runtime_state`
- `QueryReport.query_trust`
- `restore_runtime_cache_from_artifacts(repo_root) -> bool`

主要问题是 `restore_runtime_cache_from_artifacts` 会重建 SQLite state、scan/module mirror、knowledge units、page digests、runtime gates 和 page caches。现有 `facts_snapshot_ready` 只检查 SQLite DB 中 scan/module/source/module 最小条件，因此 Level 1 restore 后容易被上层投影成完整 `facts_ready == true` 和 `query_readiness == ready`。

### 现有架构概述

| 层级 | 组件 / 模块 | 当前职责 | 本 change 调整方向 |
| --- | --- | --- | --- |
| truth artifact | `crates/wiki-runtime/src/storage/knowledge_artifacts.rs` | 持久化 `.wiki/.knowledge/**` 与 `recovery-manifest.json`，支持 restore guard | 增加 committed snapshot manifest 与 restore outcome，区分 Level 1 mirror 和 Level 2 graph |
| runtime status | `crates/wiki-runtime/src/workflows/status.rs` | 基于 change plan、facts_ready、runtime summary 生成外部状态 | 输出分层 readiness，不再用单一 `facts_ready` 代表 graph ready |
| query workflow | `crates/wiki-runtime/src/workflows/query.rs` | 先尝试 restore，再要求 facts_ready，之后查 index/knowledge/page fallback | index 不 ready 时不执行 graph query；可返回 knowledge/projection 诊断或受限结果 |
| state/cache | `crates/wiki-runtime/src/storage/state_store.rs` 和 SQLite store | 判断 facts snapshot 是否 ready，读写 WikiState | 新增 index graph readiness 判断，避免 Level 1 mirror 被当作 graph snapshot |
| transport | `crates/wiki-runtime/src/transport/query_payload.rs` | 映射外部 query payload | 暴露 readiness summary 的稳定字段，并移除 `facts_ready/query_readiness` 公开字段 |

### 依赖关系

| 依赖项 | 类型 | 用途 | 来源 / 文档 | 备注 |
| --- | --- | --- | --- | --- |
| official page tree | runtime artifact | Level 1 restore 校验页面 content hash 和 binding | `.docs/design/specwiki-contract-closure.md` | `.wiki/pages/**` 不参与 |
| `.wiki/.knowledge/**` | formal knowledge artifact | Level 1 restore 输入和 committed snapshot 主体 | `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md` | declared/derived/runtime 均属正式 snapshot |
| `wiki.metadata.json` | binding index | 绑定 page/section/knowledge/source refs 与 current snapshot pointer | `.docs/design/knowledge-to-wiki-projection-contract.md` | 不保存 runtime gates 主状态 |
| `.wiki/.cache/wiki-cache.db` | local cache | Level 2 graph query substrate 与 runtime mirror | `state_store.rs`、`sqlite_store.rs` | 不上库，不是 shared truth |

## 功能设计

### 功能模块划分

| 模块名称 | 功能描述 | 优先级 | 依赖模块 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- |
| Snapshot Manifest | 定义 committed snapshot manifest 路径、字段、写入与读取 | P0 | knowledge artifacts、metadata | committed snapshot manifest |
| Restore Outcome | 将 restore 返回值从 bool 升级为包含 level、readiness、诊断原因的结构 | P0 | knowledge artifacts、state_store | 两级 restore |
| Runtime Readiness | 新增分层 readiness DTO 与 status 投影 | P0 | runtime_profile、status | 拆分 readiness |
| Query Degradation | index 不 ready 时阻止 graph 伪命中，并允许 knowledge/projection 受限消费或显式拒绝 | P0 | query、runtime_profile | query 不伪装 graph 命中 |
| Restore Guards | manifest、metadata、page hash、declared snapshot、knowledge snapshot 不一致时返回 diagnostic/blocked | P0 | knowledge_artifacts | manifest stale / metadata mismatch |
| Test Realignment | 调整 `facts_ready/query_readiness` 断言为分层 readiness 断言 | P0 | runtime tests | 成功标准 |

### 功能详细设计

#### Snapshot Manifest

新增 committed snapshot manifest 主记录，建议路径：

```text
.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml
```

`snapshot-id` 使用当前 knowledge/projection 输入稳定 hash 派生，避免依赖时间戳。manifest 最小字段：

```text
schema_version
snapshot_id
graph_snapshot_id
knowledge_snapshot_id
declared_snapshot_id
projection_snapshot_id
metadata_hash
page_hashes
projection_digest_refs
runtime_gate_refs
facts_input_hash
created_at
status
```

`recovery-manifest.json` 不再作为唯一主记录，也不保留 JSON 等价过渡 artifact。实现上直接让现有 `KnowledgeRecoveryManifest` 升级为 `CommittedSnapshotManifest`，并通过现有 `serde_yaml` 依赖读写 `manifest.yaml`。当前阶段不保留旧 manifest 兼容双轨。

`wiki.metadata.json` 只保存：

```text
current_snapshot_id
binding index
page / section / knowledge / source refs
content_hash / input_hash
reverse refs
```

metadata 不保存 projection digest 主状态、runtime gates 主状态或 committed snapshot manifest 正文。

#### Restore Outcome

将 `restore_runtime_cache_from_artifacts(repo_root) -> io::Result<bool>` 替换为结构化结果：

```text
RestoreOutcome
  attempted: bool
  level: none | knowledge_projection | code_graph
  restored_cache: bool
  index_readiness: missing | stale | rebuilding | ready | blocked
  knowledge_readiness: missing | stale | conflict | ready | blocked
  projection_readiness: missing | stale | conflict | ready | blocked
  reason: Option<String>
  snapshot_id: Option<String>
```

Level 1 restore 的输出只允许：

```text
level = knowledge_projection
index_readiness = missing | stale
knowledge_readiness = ready | stale | conflict | blocked
projection_readiness = ready | stale | conflict | blocked
```

它可以重建以下本地 mirror：

- `WikiState` / page state rows
- knowledge domains / units mirror
- page digest mirror
- runtime gate mirror
- page context / page generation cache
- scan/module 的 recovery mirror

但这些 mirror 必须带有 `restored_from_snapshot` 或等价 runtime meta，不能被 `facts_snapshot_ready` 解释为当前源码的 Level 2 graph snapshot。

#### Runtime Readiness

新增正式 readiness DTO，放在 `runtime_profile.rs`：

```text
RuntimeReadiness
  index: LayerReadiness
  knowledge: LayerReadiness
  projection: LayerReadiness
  governance: Option<LayerReadiness>
  fusion: FusionReadiness
  restored_level: none | level1 | level2
  snapshot_id: Option<String>
```

`LayerReadiness` 最小枚举：

```text
ready
stale
missing
rebuilding
conflict
blocked
not_enabled
```

`StatusReport` 和 `QueryReport` 直接以 `readiness` 作为机器可读主合同；`facts_ready` 与 `query_readiness` 不再作为公开字段保留。后续实现和测试以 `readiness.index` 等字段为主，不再用 `facts_ready == true` 证明 graph ready。

状态映射：

| 场景 | index | knowledge | projection | recommended action |
| --- | --- | --- | --- | --- |
| full init/rebuild 完成 | ready | ready | ready | none |
| `.cache` 缺失，Level 1 restore 成功 | missing 或 stale | ready | ready | rebuild 或 update |
| manifest / metadata / page hash 不一致 | blocked | blocked 或 stale | blocked 或 conflict | rebuild / sync / repair |
| 当前源码变更但 graph 仍可读 | stale | stale | stale | update |
| metadata 缺失但 graph snapshot 存在 | ready | missing | missing | init 或 rebuild |

#### Query Degradation

`query` 必须先读取 `RuntimeReadiness`：

- `index.ready`：可以调用 `wiki-index::query`，允许返回 symbol/source/module/edge。
- `index.stale`：可以返回 index 命中，但 `query_trust` 必须降级并建议 `update`。
- `index.missing/blocked`：不得调用 graph query 并伪装为空命中；可以：
  - 如果 `knowledge/projection ready`，返回 knowledge/projection 受限结果，`answer_mode = degraded`。
  - 如果 knowledge/projection 也不可用，返回 `index not ready` 或 structured diagnostic。

本 change 不全量升级 query route DTO，但要保证现有输出中：

- index graph 不 ready 时不会出现 `index_hit` / graph edge 命中。
- knowledge/projection 受限结果带 `query_trust = stale_but_queryable` 或 `blocked`，以及 `recommended_action = rebuild/update/sync`。
- 空命中与 index not ready 仍可区分。

#### Restore Guards

Level 1 restore 前置校验：

- committed snapshot manifest 可读，schema version 可识别。
- `wiki.metadata.json` 中 `current_snapshot_id` 与 manifest 一致。
- metadata hash 与 manifest 一致。
- official page tree 文件存在且 content hash 与 metadata 一致。
- declared snapshot id 与 manifest 一致。
- knowledge snapshot id 与 manifest 一致。
- page digest 与 metadata page binding 一致。

失败结果不再只返回 `false`，而是写出可诊断 reason，例如：

```text
manifest_missing
metadata_hash_mismatch
page_hash_mismatch
declared_snapshot_mismatch
knowledge_snapshot_mismatch
projection_digest_mismatch
```

status/query 消费这些 reason 时，不得静默 full init，也不得把 restore 失败投影成 ready。

### 异常处理设计

| 异常场景 | 异常类型 | 处理策略 | 用户提示 / 系统行为 |
| --- | --- | --- | --- |
| `.cache` 缺失但 manifest 可恢复 | recovery | 执行 Level 1 restore，标记 index missing/stale | 建议 rebuild 或 update |
| manifest 与 metadata hash 不一致 | consistency | restore blocked，不写新的 graph ready 状态 | 建议 repair/sync/rebuild |
| page hash 与 metadata 不一致 | projection conflict | projection readiness conflict 或 blocked | 建议 sync 或 repair |
| declared snapshot 不一致 | knowledge conflict | knowledge readiness blocked | 建议 sync/update |
| graph DB 不存在 | index missing | 不执行 index query | 返回 index not ready 或 degraded knowledge result |
| Level 2 rebuild 完成 | recovery completion | 写入 current graph snapshot，index ready | 清除 restored-only diagnostic |

## 数据设计

### 数据模型设计

| 字段 / 实体 | 类型 | 是否必填 | 业务含义 | 数据约束 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- | --- |
| `CommittedSnapshotManifest` | artifact | 是 | shared runtime recovery truth 主记录 | 位于 `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml` | committed snapshot manifest |
| `RuntimeReadiness` | DTO | 是 | 分层表达 index/knowledge/projection 状态 | status/query 共同消费 | readiness 拆分 |
| `RestoreOutcome` | internal DTO | 是 | restore 尝试结果与诊断原因 | 不可只用 bool 表达 | 两级 restore |
| `current_snapshot_id` | metadata pointer | 是 | metadata 指向当前 committed snapshot | 不能包含 manifest 正文 | metadata 边界 |
| `graph_snapshot_origin` | runtime meta | 是 | 标识 SQLite graph 是 current source rebuild 还是 restored mirror | Level 1 不得标 ready | graph ready 边界 |

### 存储方案

| 数据类型 | 存储介质 | 存储位置 | 索引 / 查询策略 | 保留策略 |
| --- | --- | --- | --- | --- |
| committed manifest | YAML artifact | `.wiki/.knowledge/runtime/snapshots/<snapshot-id>/manifest.yaml` | snapshot id 读取 | 上库 |
| snapshot pointer | JSON metadata | `.wiki/wiki.metadata.json` | current snapshot pointer | 上库 |
| knowledge artifacts | JSON/JSONL | `.wiki/.knowledge/declared|derived|runtime/**` | artifact loader | 上库 |
| restored mirror marker | SQLite runtime_meta | `.wiki/.cache/wiki-cache.db` | key-value | 本地可重建 |
| graph snapshot | SQLite index tables | `.wiki/.cache/wiki-cache.db` | `wiki-index` store | 本地可重建 |

### 数据迁移方案

当前阶段不考虑历史兼容性。实现时允许：

- 直接停止写 `runtime/recovery-manifest.json` 作为唯一主记录。
- 改为写 `runtime/snapshots/<snapshot-id>/manifest.yaml`。
- 同步更新测试 fixture 和 `.wiki/05-规格基线/**` 中对 recovery manifest 的旧路径描述。

## 接口设计

### 接口概览

| 接口名称 | 接口类型 | 方向 / 方法 | 所属模块 | 对应功能 |
| --- | --- | --- | --- | --- |
| `RuntimeReadiness` | Rust DTO / JSON | status/query 输出 | `wiki-runtime::domain::runtime_profile` | 分层 readiness |
| `RestoreOutcome` | Rust DTO | 内部 restore 返回 | `wiki-runtime::storage::knowledge_artifacts` | Level 1 restore 诊断 |
| `index_graph_ready` / `runtime_mirror_ready` | Rust API | 内部判断 | `wiki-runtime::storage::state_store` | 区分 graph ready 和 mirror ready |
| `ExternalQueryReport.readiness` | transport JSON | query 输出 | `wiki-runtime::transport` | 宿主消费 |

### 接口详细定义

#### `RuntimeReadiness`

- 接口描述：status/query 的机器可读 readiness 主合同，直接替代 `facts_ready/query_readiness` 公开字段。
- 所属模块：`crates/wiki-runtime/src/domain/runtime_profile.rs`
- 输出结构：

```text
readiness:
  index: ready | stale | missing | rebuilding | conflict | blocked
  knowledge: ready | stale | missing | conflict | blocked
  projection: ready | stale | missing | conflict | blocked
  governance: ready | not_enabled | blocked
  fusion: ready | degraded | blocked
  restored_level: none | level1 | level2
  snapshot_id: string?
  reasons: string[]
```

#### `RestoreOutcome`

- 接口描述：restore 尝试不再返回布尔值，而是返回分层结果。
- 所属模块：`crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- 错误定义：
  - IO / parse error 仍返回 `io::Error`。
  - artifact 不满足 restore 条件返回 `RestoreOutcome { attempted: true, restored_cache: false, reason: ... }`，由 status/query 投影为 blocked/stale。

## 非功能性设计

### 可靠性设计

| 可靠性要求 | 需求描述 | 实现方案 |
| --- | --- | --- |
| 不伪装 ready | Level 1 restore 不能让 index graph ready | runtime meta 标记 restored mirror，`facts_snapshot_ready` 或新 API 必须识别 graph origin |
| 可诊断 | restore 失败必须有机器可读 reason | `RestoreOutcome.reason` 和 `readiness.reasons` |
| 可恢复 | Level 2 rebuild 后清除 degraded 状态 | rebuild 写 current graph snapshot marker |

### 可维护性设计

| 设计维度 | 实现方案 |
| --- | --- |
| DTO 边界 | readiness DTO 放在 `runtime_profile.rs`，status/query 共用 |
| 文件边界 | committed manifest loader/writer 放在 `knowledge_artifacts.rs`，避免散落到 workflow |
| 测试边界 | status/query/roundtrip 测试统一断言 `readiness.*` 字段 |
| 版本兼容 | 不保留旧 manifest 双轨，直接更新 fixtures 与测试 |

## 资源评估

无新增外部资源要求。新增 manifest 和 readiness 元数据为小型 YAML/JSON/SQLite runtime_meta 记录，对磁盘和运行时间影响可以忽略。Level 2 rebuild 仍沿用现有 rebuild/update 机制，不新增后台服务。

## 风险与对策

### 技术风险

| 风险描述 | 影响程度 | 发生概率 | 应对策略 | 预留方案 |
| --- | --- | --- | --- | --- |
| `facts_ready` 调用点较多，语义替换遗漏 | 高 | 中 | 新增 `RuntimeReadiness` 后直接替换 status/query 对外字段，并同步更新所有断言 | 不保留旧字段公开兼容层 |
| Level 1 restore 当前会重建 scan/module mirror | 高 | 高 | 增加 graph origin marker，禁止 mirror 满足 index ready | 将 `facts_snapshot_ready` 拆成 `index_graph_ready` 与 `runtime_mirror_ready` |
| manifest 格式切换影响现有 artifact roundtrip 测试 | 中 | 高 | 同步更新 `knowledge_artifacts_roundtrip` 与 status tests | 不保留旧路径兼容 |
| query 在 index missing 但 knowledge ready 时路径复杂 | 中 | 中 | 本 change 只做受限 knowledge/projection 消费或显式拒绝，不做 route DTO 全量升级 | 复杂 result shaping 留给 query-route child |

## 设计决策

- `manifest.yaml` 作为 committed snapshot manifest 的目标文件名与格式；现有 `recovery-manifest.json` 不再作为长期主记录。
- `RuntimeReadiness` 成为 status/query 的机器可读主合同，旧 `facts_ready/query_readiness` 不作为公开字段保留。
- Level 1 restore 可以恢复本地 runtime mirror，但必须留下 `restored_level=level1` 和 `index_readiness != ready`。
- Level 2 graph ready 必须来自当前源码的 index build/rebuild/update 提交，不能来自 `.knowledge + metadata + pages` 的恢复。
- manifest / metadata / page / declared / knowledge 任一锚点不一致时，restore 进入 diagnostic/blocked，不自动 full init。
- `.wiki/pages/**` 不参与本 change 的任何判断。

## 已确认取舍

- `RuntimeReadiness` 在本 change 中直接替换 `facts_ready/query_readiness` 公开字段；内部如需判断，改用更窄的 `index_graph_ready` 或 `runtime_mirror_ready`。
- Level 1 下 query 不强制必须返回 knowledge/projection 命中；允许返回受限 knowledge/projection 消费结果，但必须 degraded，并且 graph 相关结果必须禁用。
- `manifest.yaml` 的 YAML 序列化直接复用现有 `serde_yaml` 依赖；不接受 JSON 等价 artifact 作为第一步实现。

## 验证思路

- 删除 `.wiki/.cache/**` 后运行 status：`readiness.knowledge/projection` 可 ready，`readiness.index` 不得 ready，recommended action 指向 rebuild/update。
- 执行 Level 2 rebuild 后运行 status：`readiness.index` 变为 ready。
- 修改 manifest metadata hash、page hash、declared snapshot 或 knowledge snapshot：restore 返回 blocked/conflict reason，不生成 ready。
- index missing 但 knowledge/projection 可恢复时 query 不返回 graph edge/symbol fake hit；若返回结果，必须 degraded 并带 recommended action。
- 当前 `status_restores_runtime_from_formal_artifacts_when_cache_is_missing` 和 `query_restores_runtime_cache_from_formal_artifacts` 不能继续断言 `facts_ready == true` 或 `runtime_state == fresh` 作为 Level 1 成功证据。

## 参考资料

- `proposal.md`
- `../refactor-specwiki-around-contract-closure/split.md`
- `.docs/design/specwiki-contract-closure.md`
- `.docs/design/knowledge-to-wiki-projection-contract.md`
- `.wiki/06-设计文档/01-Runtime设计.md`
- `.wiki/05-规格基线/capabilities/knowledge-runtime-artifacts/spec.md`
- `crates/wiki-runtime/src/workflows/status.rs`
- `crates/wiki-runtime/src/workflows/query.rs`
- `crates/wiki-runtime/src/storage/knowledge_artifacts.rs`
- `crates/wiki-runtime/src/storage/state_store.rs`
- `crates/wiki-runtime/tests/runtime/status_and_update.rs`
- `crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs`
