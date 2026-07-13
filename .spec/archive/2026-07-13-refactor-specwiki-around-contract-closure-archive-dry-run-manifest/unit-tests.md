# refactor-specwiki-around-contract-closure-archive-dry-run-manifest 单元测试设计

## 测试总览

TDD 蓝图覆盖 DTO、确定性输入、planner、ArchiveFs、reconcile、transport 和 CLI。真实 rename、并发、平台路径和 Wiki tree hash 由 system/integration tests 覆盖。

## 单元测试用例

### UT-001 Archive DTO serde 闭集

**目标行为**

合法 archive DTO 使用稳定 snake_case JSON，未知枚举拒绝。

**关联**

- Design: 数据设计、接口设计
- 系统测试用例: ST-002、ST-015
- Tasks: 1.1 / 1.2 / 1.3

**Files**

- Test: `crates/wiki-model/tests/archive_contract.rs`
- Modify: `crates/wiki-model/src/domain/governance.rs`
- Reference: `crates/wiki-model/tests/governance_contract.rs`

**测试代码蓝图**

构造 dry-run manifest，断言 mode/outcome/step/status 序列化；未知值反序列化失败。

**测试数据 / Fixture / Mock 边界**

只使用内存 DTO，不读取文件系统。

**运行命令**

`cargo test -p wiki-model --test archive_contract archive_contract`

**预期 Red 失败**

archive DTO 不存在或未知闭集值被接受。

**Green 通过条件**

完整 roundtrip 且既有 governance DTO 不变。

**Refactor 守卫**

不把 filesystem/policy 规则放入 model。

### UT-002 固定 clock 与 operation id

**目标行为**

注入 clock/id 后 target 和 operation root 稳定，resume 不重算日期。

**关联**

- Design: 固定 target、设计决策
- 系统测试用例: ST-002、ST-011
- Tasks: 1.4 / 1.5 / 1.6

**Files**

- Test: `crates/wiki-runtime/tests/archive_planning.rs`
- Modify: `crates/wiki-runtime/src/workflows/archive.rs`
- Reference: `time` 依赖

**测试代码蓝图**

注入跨 UTC 午夜的 clock，生成 plan 后再次 resume，断言 target/date 不变。

**测试数据 / Fixture / Mock 边界**

只注入 clock/id provider，不 mock filesystem truth。

**运行命令**

`cargo test -p wiki-runtime --test archive_planning fixed_utc_target`

**预期 Red 失败**

target 使用系统当前日期或 operation id 不稳定。

**Green 通过条件**

plan/resume 使用同一固定 target。

**Refactor 守卫**

生产路径不读取时间两次；provider 不进入 CLI 公共 API。

### UT-003 canonical source snapshot

**目标行为**

source tree 按规范化路径/type/size/content hash 稳定排序并流式读取。

**关联**

- Design: Precondition digest
- 系统测试用例: ST-003、ST-013
- Tasks: 2.1 / 2.2 / 2.3

**Files**

- Test: `crates/wiki-runtime/tests/archive_storage.rs`
- Modify: `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: `governance_fs.rs`

**测试代码蓝图**

相同内容不同创建顺序、不同 mtime，断言 snapshot/digest 相同。

**测试数据 / Fixture / Mock 边界**

TempDir 普通文件；危险条目由 UT-005 覆盖。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage canonical_snapshot`

**预期 Red 失败**

遍历顺序或 mtime 导致 digest 不同。

**Green 通过条件**

snapshot 只绑定设计规定的 source 输入。

**Refactor 守卫**

保持流式 hash，避免整树载入内存。

### UT-004 precondition digest 输入矩阵

**目标行为**

绑定输入变化必改变 digest，排除 operation id、mtime、绝对路径、无关 Wiki、全仓 fingerprint。

**关联**

- Design: Precondition digest
- 系统测试用例: ST-003
- Tasks: 2.4 / 2.5 / 2.6

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: BLAKE3 helpers

**测试代码蓝图**

表驱动修改 source/parent/target sentinel/version 与排除项，断言 digest 变化关系。

**测试数据 / Fixture / Mock 边界**

canonical input table，不依赖真实 UTC。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage precondition_digest`

**预期 Red 失败**

绑定输入未改变 digest，或排除项改变 digest。

**Green 通过条件**

所有 inclusion/exclusion 关系成立。

**Refactor 守卫**

不复用过宽的 governance fingerprint。

### UT-005 unsafe source entry 拒绝

**目标行为**

拒绝 symlink、reparse/junction、special file、绝对路径和 repo escape。

**关联**

- Design: source tree 路径安全
- 系统测试用例: ST-013
- Tasks: 2.7 / 2.8 / 2.9

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: `governance_evidence.rs`

**测试代码蓝图**

普通文件通过；平台原生或 test classifier 构造危险 entry，断言 typed conflict 和 offending path。

**测试数据 / Fixture / Mock 边界**

Unix symlink、Windows capability probe/reparse。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage rejects_unsafe_source_entries`

**预期 Red 失败**

`expected Err, got Ok`。

**Green 通过条件**

危险条目在业务写入前被拒绝。

**Refactor 守卫**

检查集中在 ArchiveFs。

### UT-006 planner validate-first

**目标行为**

planner 单次 live preflight；not-ready 透传 issues、短路写侧，并生成 target/parent diff/Wiki refs。

**关联**

- Design: ArchiveService.plan
- 系统测试用例: ST-001、ST-002、ST-014
- Tasks: 3.1 / 3.2 / 3.3

**Files**

- Test: `crates/wiki-runtime/tests/archive_planning.rs`
- Modify: `crates/wiki-runtime/src/workflows/archive.rs`
- Reference: `governance_workflows.rs`

**测试代码蓝图**

blocked fixture + write-port recorder，断言调用顺序和零写。

**测试数据 / Fixture / Mock 边界**

允许 fake read-only evaluation 和 write-port recorder，不 mock policy 内容。

**运行命令**

`cargo test -p wiki-runtime --test archive_planning planner_validate_first`

**预期 Red 失败**

write port 被调用或 issues 丢失。

**Green 通过条件**

blocked 计划零写，ready 计划只产生内存 manifest。

**Refactor 守卫**

workflow 不复制 GovernancePolicy。

### UT-007 parent meta 语义变换

**目标行为**

更新 archiveStatus/archivedAt/archivedTo，保留未知字段语义。

**关联**

- Design: Parent meta.yaml
- 系统测试用例: ST-006
- Tasks: 1.7 / 1.8 / 1.9

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: parent metadata fixtures

**测试代码蓝图**

含未知 key 的 YAML patch 后重新读取，断言未知 key/value 保留且 before/after hash 可算。

**测试数据 / Fixture / Mock 边界**

只断言语义，不承诺注释和排版。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage parent_meta_patch`

**预期 Red 失败**

`unknown_field == None` 或 child entry 非唯一。

**Green 通过条件**

唯一 child entry 正确更新且语义字段完整。

**Refactor 守卫**

明确 canonical YAML rewrite，不声称保注释。

### UT-008 split marker 唯一变换

**目标行为**

只将唯一 child section 的 pending marker 变为 archived；0/多匹配 conflict。

**关联**

- Design: Parent split.md
- 系统测试用例: ST-006
- Tasks: 1.10 / 1.11 / 1.12

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: `split_marks_archived`

**测试代码蓝图**

表驱动 0/1/多 section 与 marker，断言唯一变换和错误分类。

**测试数据 / Fixture / Mock 边界**

最小 split 文本，不引入通用 Markdown parser。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage split_patch_unique_marker`

**预期 Red 失败**

`expected Err, got Ok` 或修改了错误 section。

**Green 通过条件**

唯一合法 marker 变更，歧义 fail closed。

**Refactor 守卫**

不扩大 Markdown 重写范围。

### UT-009 mutation-set lock key

**目标行为**

同一 parent 的 sibling 使用相同完整 mutation-set lock，输入顺序不影响 key。

**关联**

- Design: Mutation-set lock
- 系统测试用例: ST-012
- Tasks: 2.10 / 2.11 / 2.12

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: design lock contract

**测试代码蓝图**

验证 sibling lock key 相同；持锁时第二次 try_lock 返回 `ArchiveLocked`。

**测试数据 / Fixture / Mock 边界**

key 计算 unit；真实 OS lock 由 ST-012 覆盖。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage mutation_set_lock`

**预期 Red 失败**

sibling key 不同或第二次锁获取成功。

**Green 通过条件**

lock key 规范化且 fresh/resume 共用协议。

**Refactor 守卫**

不靠删除 stale 文件释放锁。

### UT-010 immutable plan 与 append-only checkpoint

**目标行为**

plan 不可覆盖，checkpoint sequence/attempt 单调，临时文件不成为有效 checkpoint。

**关联**

- Design: Durable operation protocol
- 系统测试用例: ST-009
- Tasks: 2.13 / 2.14 / 2.15

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: operation directory design

**测试代码蓝图**

尝试覆盖 plan、重复/倒序 checkpoint、失败后同 step 更高 attempt，断言聚合视图。

**测试数据 / Fixture / Mock 边界**

TempDir operation root，不模拟断电。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage durable_operation_protocol`

**预期 Red 失败**

plan 被覆盖或重复序号被接受。

**Green 通过条件**

plan/checkpoints/result 可重建聚合 manifest。

**Refactor 守卫**

不改成可变单文件 manifest。

### UT-011 operation discovery

**目标行为**

按 change/mutation-set 发现唯一未完成 operation；多个候选 conflict，fresh apply 不创建第二 operation。

**关联**

- Design: Resume、operation discovery
- 系统测试用例: ST-005、ST-010
- Tasks: 2.16 / 2.17 / 2.18

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_storage.rs`, `crates/wiki-runtime/src/storage/archive_fs.rs`
- Reference: operation manifest schema

**测试代码蓝图**

构造唯一 recovery、多个 recovery、completed operation，断言返回 id/conflict/already-completed。

**测试数据 / Fixture / Mock 边界**

动态 operation directories，不触发实际 mutation。

**运行命令**

`cargo test -p wiki-runtime --test archive_storage operation_discovery`

**预期 Red 失败**

创建第二 operation 或忽略已有 recovery。

**Green 通过条件**

discovery 结果稳定且提供 operation id。

**Refactor 守卫**

不扫描 `.wiki` 或 SQLite cache。

### UT-012 reconcile 状态矩阵

**目标行为**

根据 source/target 和 parent before/after/unknown 状态输出 continue/completed/recovery_required/conflict。

**关联**

- Design: reconcile
- 系统测试用例: ST-008、ST-009、ST-011
- Tasks: 4.1 / 4.2 / 4.3

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_workflows.rs`, `crates/wiki-runtime/src/workflows/archive.rs`
- Reference: design state machine

**测试代码蓝图**

表驱动 source/target 四态和 parent before/after/unknown 组合，未知态断言 fail closed。

**测试数据 / Fixture / Mock 边界**

纯状态模型 fake snapshot；真实 I/O 由 ST-008 覆盖。

**运行命令**

`cargo test -p wiki-runtime --test archive_workflows reconcile_state_matrix`

**预期 Red 失败**

unknown 被当作 completed 或自动覆盖。

**Green 通过条件**

所有状态组合分类稳定。

**Refactor 守卫**

不堆叠不可解释 fallback。

### UT-013 apply workflow 状态推进

**目标行为**

验证 lock -> preflight/digest -> plan -> mutations -> live verify -> completed 顺序。

**关联**

- Design: Apply 状态机
- 系统测试用例: ST-003、ST-004、ST-007
- Tasks: 4.4 / 4.5 / 4.6

**Files**

- Test/Modify: `crates/wiki-runtime/tests/archive_workflows.rs`, `crates/wiki-runtime/src/workflows/archive.rs`
- Reference: `governance_workflows.rs`

**测试代码蓝图**

使用 recorder ports 断言首个 mutation 前完成 lock/precondition，live verify 后才产生 completed。

**测试数据 / Fixture / Mock 边界**

fake ports 只记录顺序；policy 使用真实 fixture。

**运行命令**

`cargo test -p wiki-runtime --test archive_workflows apply_state_machine`

**预期 Red 失败**

mutation 早于 digest，或 live verify 前报告 completed。

**Green 通过条件**

成功/失败状态聚合正确。

**Refactor 守卫**

workflow 只编排，文件细节留在 ArchiveFs。

### UT-014 typed transport envelope

**目标行为**

CoreCommand archive 参数和 typed errors 稳定映射为 JSON envelope。

**关联**

- Design: 接口设计、错误与退出码
- 系统测试用例: ST-015
- Tasks: 5.1 / 5.2 / 5.3

**Files**

- Test/Modify: `crates/wiki-runtime/tests/acceptance/command_contract.rs`, `crates/wiki-runtime/src/transport/dto.rs`, `transport/cli.rs`
- Reference: existing changes/validate command tests

**测试代码蓝图**

断言缺 changeId、互斥 mode 和各 domain error 的结构化映射。

**测试数据 / Fixture / Mock 边界**

JSON transport only，不执行真实 mutation。

**运行命令**

`cargo test -p wiki-runtime --test acceptance command_contract archive_transport`

**预期 Red 失败**

archive 被压成 unsupported/workflow_failed，或非法参数被接受。

**Green 通过条件**

typed envelope 稳定且 transport 不复制 workflow 规则。

**Refactor 守卫**

错误映射不依赖 message 文本。

### UT-015 CLI 参数与 advanced help

**目标行为**

默认 dry-run、`--apply`、`--resume` 解析和 help 分层稳定。

**关联**

- Design: CLI 接口
- 系统测试用例: ST-015、ST-016
- Tasks: 5.4 / 5.5 / 5.6

**Files**

- Test: `packages/spec-wiki/src/cli.test.ts`
- Modify: `packages/spec-wiki/src/cli.ts`, `cli/commandSpec.ts`, `runtime/invokeCore.ts`
- Reference: current CLI help/validate tests

**测试代码蓝图**

mock forwardCore，断言默认/显式 dry-run、apply、resume、互斥/缺参和 help 可见性。

**测试数据 / Fixture / Mock 边界**

只 mock forwardCore，不执行 Rust binary。

**运行命令**

`pnpm --filter spec-wiki exec vitest run src/cli.test.ts -t archive`

**预期 Red 失败**

unsupported action、help-all 缺 archive 或非法组合未返回 64。

**Green 通过条件**

所有 mode/help/互斥规则稳定转发。

**Refactor 守卫**

archive 保持 advanced non-streaming，不污染默认 help。

### UT-016 parser/human/exit policy 闭集

**目标行为**

TS 解析 archive report，human 输出关键字段，exit policy 正确映射 0/1/2/64。

**关联**

- Design: 错误与退出码
- 系统测试用例: ST-015、ST-016
- Tasks: 5.7 / 5.8 / 5.9

**Files**

- Test: `packages/spec-wiki/src/runtime/parseResult.test.ts`, `exitPolicy.test.ts`, `humanRenderer.test.ts`
- Modify: `packages/spec-wiki/src/runtime/parseResult.ts`, `exitPolicy.ts`, `humanRenderer.ts`
- Reference: existing governance/errorKind tests

**测试代码蓝图**

合法 report、未知 enum、failure_step/recovery_hint 和 exit matrix 的 parser/renderer 断言。

**测试数据 / Fixture / Mock 边界**

纯 DTO parser/renderer，不调用子进程。

**运行命令**

`pnpm --filter spec-wiki exec vitest run src/runtime/parseResult.test.ts src/runtime/exitPolicy.test.ts src/runtime/humanRenderer.test.ts -t archive`

**预期 Red 失败**

unknown errorKind、exit 误映射 1 或 human 缺 recovery hint。

**Green 通过条件**

结构化字段完整映射，退出码不依赖错误文本。

**Refactor 守卫**

renderer 只翻译 DTO，不重算 readiness/outcome。

## 测试辅助边界

- 允许新增 archive fixture builder、clock/id provider、filesystem snapshot helper、fault injector 和 lock barrier。
- fault injector 通过 crate-private/test-only adapter 暴露，不加入公共生产 API。
- 允许新增 `archive_contract.rs`、`archive_planning.rs`、`archive_storage.rs`、`archive_workflows.rs`、`archive_recovery.rs`、`archive_concurrency.rs`、`archive_path_safety.rs` 测试目标。

## 不纳入单元测试的内容

- 真实跨文件系统原子性、断电、磁盘缓存丢失：由 system/integration fault evidence 替代。
- 真实 sibling 多进程竞争、Windows reparse/junction、distribution parity 和 `.wiki` tree hash：由系统测试覆盖。
