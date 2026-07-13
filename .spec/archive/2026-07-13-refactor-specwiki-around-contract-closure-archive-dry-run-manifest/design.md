# refactor-specwiki-around-contract-closure-archive-dry-run-manifest 设计方案

## 方案概述

本方案把 archive 实现为一个可审计、可恢复、不可误报成功的 durable operation protocol。默认 `archive <change-id>` 只做 dry-run，`--apply` 才执行写入，`--resume <operation-id>` 根据持久化 plan、checkpoint 和当前文件系统状态继续操作。

方案不承诺跨目录和多文件的文件系统级 ACID 原子性，而是保证：首次写入前重新校验 precondition；每个可见变更都有 checkpoint；崩溃后可根据 source、target、parent 文件 hash 和 marker 对账；未知外部修改进入 conflict；完成状态必须通过 live governance verification。

### 方案范围

- 新增 archive 的 domain DTO、workflow、写侧 storage adapter 和 CLI 路由。
- 复用 `GovernanceService` 的单次 live evaluation 与 archive readiness。
- 事务边界限定为 `.spec/changes/<change-id>`、`.spec/archive/<date>-<change-id>`、active parent 的 `meta.yaml` / `split.md` 和 operation manifest。
- Wiki 只输出 issue/ref，不写 `.wiki/**`，不触发 sync/update。

### 设计边界

- 不实现通用 repair、rollback、doctor 或 trace。
- 不改变现有治理规则，不在 archive workflow 复制 policy。
- 不提供旧 archive CLI 或旧 manifest schema 兼容层。
- 不把 `.wiki`、SQLite governance cache 或全仓 fingerprint 纳入 archive 写事务。

### 核心设计思路

```text
CLI archive
  -> ArchiveService.plan
       -> GovernanceService 单次 live preflight
       -> ArchiveFs source/parent snapshot
       -> canonical BLAKE3 precondition digest
  -> dry-run: 返回内存 manifest
  -> apply: 写入 immutable plan + checkpoints，再执行状态机
  -> resume: 读取 plan/checkpoints，按文件系统事实 reconcile
  -> live verify -> completed / recovery_required / conflict
```

## 架构分析

### 现有架构概述

| 架构层级 | 组件 / 模块 | 说明 |
| --- | --- | --- |
| 接入层 | `packages/spec-wiki` CLI | 负责命令解析、help、human/JSON 输出和退出码映射 |
| transport | `crates/wiki-runtime/src/transport` | 负责 CoreCommand、CoreResponse 和结构化错误 |
| workflow | `GovernanceService` | 只读读取 `.spec` evidence 并生成一次 live evaluation |
| domain | `wiki-model::domain::governance` | Rust/TS 共享的治理 DTO 和稳定枚举 |
| storage | `governance_fs.rs` | 只读 evidence discovery；不承载 archive 写操作 |

### 方案与现有架构的关系

| 维度 | 说明 |
| --- | --- |
| 复用模块 | `GovernanceService`、治理 readiness、现有 CoreResponse、CLI CommandSpec、human renderer、exit policy |
| 新增组件 | `ArchiveService`、`archive_fs.rs`、archive domain DTO、manifest/checkpoint storage |
| 改造模块 | CLI archive 路由、transport command/错误映射、governance domain DTO、测试与 CLI 文档 |
| 保持隔离 | `governance_fs.rs` 保持只读；archive 不写 Wiki、不刷新 SQLite cache |

### 依赖关系

| 依赖项 | 类型 | 用途 | 来源 / 文档 | 备注 |
| --- | --- | --- | --- | --- |
| `GovernanceService` | runtime service | 单次 live preflight、validation、archive readiness | governance isolation archived change | 直接复用，不复制规则 |
| `GovernancePolicy` | runtime policy | policy version 和规则结果 | `crates/wiki-runtime/src/domain/governance.rs` | 只读调用 |
| BLAKE3 | library | canonical precondition digest 和 artifact hash | 现有 runtime 依赖 | 不使用全仓 governance fingerprint |
| OS advisory lock | filesystem primitive | mutation set 单 writer | design decision | 进程崩溃时由 OS 释放 |
| 本地 archive 设计稿 | project design | archive 边界、manifest、Wiki 隔离 | `.docs/design/*.md` | 改写落地，不是 upstream 直接迁移 |

## 功能设计

### 功能模块划分

| 模块名称 | 功能描述 | 优先级 | 依赖模块 | 对应 proposal 内容 |
| --- | --- | --- | --- | --- |
| Archive planning | 读取 evidence、计算 readiness、目标路径、parent diff、Wiki refs 和 digest | P0 | GovernanceService、ArchiveFs | dry-run、manifest、precondition |
| Archive apply | 受锁保护地移动 source、同步 parent、写 checkpoint 并最终验证 | P0 | planning、archive_fs | apply、parent 同步 |
| Archive resume | 根据 plan/checkpoint 和当前文件系统状态继续或返回 conflict | P0 | apply storage | 失败恢复、幂等重试 |
| CLI product surface | archive 参数、JSON/human 输出和退出码 | P0 | transport、CLI renderer | 对外合同 |

### 功能详细设计

#### ArchiveService

- `plan(change_id)`：调用一次 live governance evaluation；只接受 active change；检查 archive readiness、parent 关系、target 冲突和 source tree 类型；构造内存 `ArchiveOperationManifest`。
- `apply(change_id)`：重新生成 plan，获取 mutation-set lock，重新计算 digest；在首个业务写前保存 immutable plan，然后按状态机执行。
- `resume(operation_id)`：读取 operation 目录，校验 manifest schema、change id 和路径；获取同一 mutation-set lock；不重新计算恢复日期或 target，按 plan 对账并继续。
- `reconcile`：以 source/target 是否存在、parent before/after hash、split marker 和未知文件为事实来源，不仅依赖 checkpoint 的 current step。

#### Mutation-set lock

锁覆盖本次可能变更的完整集合：child source、archive target、active parent、parent `meta.yaml` 和 `split.md`。路径统一为 repo-relative、规范化分隔符后排序生成稳定 lock key。fresh apply 与 resume 必须使用同一锁协议；获取锁后才进行 live preflight 和 digest 重验。

#### Durable operation protocol

apply 操作目录为：

```text
.spec/.runtime/archive-operations/<operation-id>/
  plan.json
  checkpoints/000001-*.json
  staging/
  result.json
```

- `plan.json` immutable，记录 source/target、UTC 日期、规则版本、validation/readiness、artifact hash、parent before/after hash、precondition digest 和预期步骤。
- checkpoint 只追加，序号单调递增；每一步最多产生一个终态 checkpoint。
- checkpoint 采用临时文件、flush、rename；表述为 best-effort durable，不宣称 ACID。
- 对外 manifest 是 plan、checkpoint 和 result 的聚合视图。
- dry-run 的 manifest 明确 `persisted=false`、`resumable=false`，不会写 `.spec/.runtime`。

#### Apply 状态机

```text
validate/readiness
  -> acquire mutation-set lock
  -> revalidate + digest check
  -> persist plan
  -> stage parent before/after and backups
  -> checkpoint prepared
  -> rename source -> target
  -> checkpoint source_moved
  -> replace parent meta.yaml
  -> checkpoint parent_meta_updated
  -> replace parent split.md
  -> checkpoint parent_split_updated
  -> live governance verification
  -> checkpoint completed + result
```

每一步状态为 `pending / in_progress / completed / failed`，operation 状态为 `planned / applying / recovery_required / completed / rejected`。如果文件 mutation 已成功但 checkpoint 未写入，resume 通过 before/after hash 前滚判断；未知内容则返回 conflict，不覆盖、不自动 rollback。

#### Parent meta.yaml

以语义保留未知字段为最低保证。若使用 `serde_yaml::Value` 重新序列化，则明确这是 canonical whole-file rewrite，不称为文本定点更新；若后续要求保留注释或原始格式，必须另选 round-trip YAML 编辑器。更新内容包括 `archiveStatus`、`archivedAt` 和 `archivedTo`，并将 before/after hash 写入 plan。

#### Parent split.md

只处理唯一的 child section。必须唯一匹配 child id 和 archive marker；0 个或多个匹配均返回 conflict。只允许把 pending marker 改为 archived marker，不实现通用 Markdown 重写器。格式漂移、重复 marker 或 parent 状态不一致均阻断。

#### Wiki 边界

ArchiveService 只生成 `wiki_sync_issues` 和 `evidence_refs`。它不调用 sync/update，不写 `.wiki/**`，也不刷新 `.wiki/.knowledge/**` 或 SQLite cache。Wiki issue 默认是 warning/non-blocking；影响 archive 正确性的 source、parent、target 或治理问题仍是 blocking。

### 异常处理设计

| 异常场景 | 异常类型 | 处理策略 | 用户提示 / 系统行为 |
| --- | --- | --- | --- |
| validate/readiness 未通过 | `archive_not_ready` | 零业务写入 | 展示 blocking issues 和下一步 |
| source/parent/target 在 apply 前变化 | `archive_precondition_changed` | 零业务写入 | 要求重新 dry-run |
| target 已存在、双占、parent marker 歧义 | `archive_conflict` | 不覆盖、不合并 | 返回冲突路径和恢复提示 |
| mutation set 已被占用 | `archive_locked` | 不执行写入 | 返回当前 operation 引用 |
| 部分步骤完成且可识别 | `archive_recovery_required` | 保存失败步骤，允许 resume | 返回 operation id、failure step、recovery hint |
| plan/checkpoint/schema 损坏 | `archive_manifest_invalid` | 停止，不猜测状态 | 要求人工检查，不自动覆盖 |

## 数据设计

### 数据模型设计

| 实体 | 关键字段 | 约束 |
| --- | --- | --- |
| `ArchiveRequest` | `change_id`、`mode`、`operation_id?` | `dry_run/apply/resume` 互斥；resume 必须带 operation id |
| `ArchiveOperationManifest` | schema/policy/algorithm version、paths、validation、digest、steps、parent diff、Wiki refs | 路径必须 repo-relative；dry-run 不可恢复 |
| `ArchiveCheckpoint` | sequence、step、status、before/after hashes、timestamp、failure | sequence 单调；追加写入 |
| `ArchiveReport` | outcome、governance、validation、manifest、issues、evidence refs、recovery hint | human/JSON 共用同一 DTO |

### 存储方案

| 数据类型 | 存储介质 | 存储位置 | 保留策略 |
| --- | --- | --- | --- |
| dry-run manifest | 内存/CLI 输出 | stdout 或 JSON response | 不持久化 |
| apply plan/checkpoint | repo-local 文件 | `.spec/.runtime/archive-operations/<operation-id>/` | 作为恢复和审计证据保留 |
| parent staging/backup | repo-local 临时文件 | operation `staging/` | operation 完成后按 design 定义清理；恢复中保留 |

### Precondition digest

使用 canonical JSON 或等价确定性编码后计算 BLAKE3。source tree 按 repo-relative path 稳定排序并流式读取，记录 path/type/size/content hash。拒绝 symlink、junction、reparse point、special file 和路径逃逸。digest 不包含 mtime、绝对路径、operation id、created_at、全仓 governance fingerprint 或无关 Wiki 文件。

## 接口设计

### CLI 接口

```text
spec-wiki archive <change-id>
spec-wiki archive <change-id> --dry-run
spec-wiki archive <change-id> --apply
spec-wiki archive <change-id> --resume <operation-id>
```

- 默认 archive 等同 dry-run；`--dry-run` 为显式同义形式。
- `--dry-run`、`--apply`、`--resume` 互斥。
- archive 进入 `--help-all` 和命令级 help，不进入默认 help。
- resume 的 change id 必须与 plan 一致。

### Rust/TypeScript DTO

`wiki-model::domain::governance` 增加 archive request/report/manifest/step/outcome 和 typed error 结构；transport 的 `CoreCommand` 增加 archive 参数，TS parser、human renderer 和 exit policy 使用同一闭集字段，不靠错误文案推导状态。

### 错误与退出码

| 场景 | 错误 | 退出码 |
| --- | --- | --- |
| dry-run ready / apply completed / already completed | success | 0 |
| not ready / precondition changed / conflict / locked / recovery required | typed domain error | 2 |
| manifest invalid / 不可恢复 I/O / internal failure | typed failure | 1 |
| 参数缺失、互斥或非法组合 | invalid argument | 64 |

## 非功能性设计

### 可靠性设计

- 首个业务写入前完成 validate、lock、digest 重验和 plan 持久化。
- 每个 mutation 边界记录 checkpoint；resume 结合 checkpoint 与文件系统实际状态。
- 成功必须同时满足 source 不存在、target 存在、parent 两处 marker 一致和 live governance verification 通过。
- Windows 下只在同一 `.spec` volume 使用 rename；不做跨 volume copy+delete fallback。
- 目标已存在时绝不覆盖、合并或自动改名。

### 可维护性设计

- 只读 governance storage 与 archive 写侧 storage 分离。
- archive workflow 不复制治理 policy。
- operation plan/checkpoint 使用稳定 schema version，错误分类和 DTO 有 fixture parity。
- 所有路径和 hash 采用稳定规范化格式，便于测试和审计。

## 资源评估

无新增常驻服务和网络资源。archive 额外成本是一次 source tree 流式 hash、parent staging 文件和 operation checkpoint；其规模与待归档 change 目录和 parent 文件大小成正比。

## 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| Windows 多文件替换无法 ACID | 高 | durable checkpoint、FS reconcile、禁止误报成功，不承诺原子事务 |
| sibling 并发覆盖 parent | 高 | 锁完整 mutation set，而非仅 child |
| digest 过宽或过窄 | 高 | canonical 输入、版本化算法、apply 前零写重算 |
| checkpoint 落盘晚于文件 mutation | 高 | resume 以 before/after hash 和实际状态为准 |
| YAML 重写损失格式 | 中 | 明确 canonical whole-file rewrite 语义；需要注释保留时改用 round-trip 工具 |
| split marker 格式漂移 | 中 | 唯一 section/marker 匹配，歧义即 conflict |
| symlink/reparse 路径逃逸 | 高 | source tree snapshot 明确拒绝特殊文件和 repo 外路径 |

## 设计决策

- dry-run 零写；只有 apply 持久化 operation plan/checkpoints。
- 使用完整 mutation-set advisory lock，不采用 child-only lock。
- 使用 immutable plan + append-only checkpoints，聚合 manifest 不作为单一可变 JSON 文件覆盖写。
- resume 以文件系统状态 reconcile 为真，不仅依赖 checkpoint current step。
- 不自动 rollback；可识别状态优先于不可靠的跨平台回滚承诺。
- archive target 固定使用 UTC 日期和 plan 中保存的 target，resume 不按当天重新计算。
- 本地设计和现有实现是主要来源；没有直接迁移 upstream 的 archive 或索引实现。

## 待确认问题

- 是否引入现有 Rust 生态中的 OS advisory lock crate，或基于当前依赖实现同等语义。
- canonical YAML rewrite 是否满足项目对 parent 注释/格式的长期要求；若不满足，design 后续迭代需切换 round-trip 编辑器。
- operation staging 清理时机和 checkpoint 保留周期由 plan 阶段结合仓库维护策略定稿。

## 参考资料

- `./proposal.md`
- `../refactor-specwiki-around-contract-closure/split.md`
- `../../../.docs/design/specwiki-contract-closure.md`
- `../../../.docs/design/governance-runtime-integration.md`
- `../../../.docs/design/specwiki-cli-unification.md`
- `../../../.spec/archive/2026-07-10-refactor-specwiki-around-contract-closure-governance-isolation/`
- `../../../.spec/archive/2026-07-12-refactor-specwiki-around-contract-closure-cli-product-surface/`
- `../../../crates/wiki-runtime/src/storage/governance_fs.rs`
- `../../../crates/wiki-runtime/src/workflows/governance.rs`
