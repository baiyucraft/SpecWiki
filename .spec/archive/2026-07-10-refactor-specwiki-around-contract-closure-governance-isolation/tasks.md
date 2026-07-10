---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-governance-isolation 任务计划

## 任务总览

任务按五个可独立验收的能力块拆分：治理对象语言与 policy、evidence discovery、SQLite cache/fingerprint、runtime/query 组合、TS transport。每个能力块采用 TDD 的 Red -> Green -> Refactor，并通过对应 `ST-*` 验证 proposal 成功标准。CLI command router 和 archive 写事务不进入本计划。

## 实现模式

tdd

先按 `unit-tests.md` 写出能够因目标行为缺失而失败的测试，确认 Red 原因后实现最小行为，最后在测试持续通过的前提下清理边界与重复。系统级 `ST-*` 在对应能力块完成后执行，不用编译错误、环境错误或无关回归代替有效 Red。

## 1. 治理对象语言与唯一 Policy 合同

- [x] 1.1 Red: UT-001 编写 governance DTO 闭集与序列化失败测试，确认 `wiki-model::domain::governance` 缺失导致预期失败
- [x] 1.2 Green: UT-001 在 `wiki-model` 增加 readiness、summary、change/artifact ref、issue、gate 和 validation DTO，使闭集 roundtrip 通过
- [x] 1.3 Refactor: UT-001 统一 serde 命名、字段注释和 query contract 复用，保持 governance 与 core readiness 分离
- [x] 1.4 Red: UT-003 写入 stage/artifact/metadata parity fixtures 与失败测试，确认 required matrix 和 metadata rule 尚未实现
- [x] 1.5 Green: UT-003 在 `wiki-runtime::domain::governance` 实现版本化 `GovernancePolicy`、稳定 rule id 和 required artifact matrix
- [x] 1.6 Refactor: UT-003 将规则目录收口为唯一入口，删除测试或 TS/Skill 中出现的重复 rule 计算
- [x] 1.7 Red: UT-004 编写 report frontmatter、archive marker、blocked/conflict 分类失败测试
- [x] 1.8 Green: UT-004 实现 review/verification gate、parent/child/archive consistency 和中间 stage archive gate 语义
- [x] 1.9 Refactor: UT-004 拆分 parse evidence 与 policy evaluation，确保只读 policy 无 `.spec` 写副作用
- [x] 1.10 执行 ST-003、ST-004、ST-005 的规范化 fixture 验证，确认 rule id、parity 与 archive gate 行为

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] ST-003 / ST-004 / ST-005 通过
- [x] 本大 task 局部质量检查已完成（相关 tests 与 `cargo fmt --check` 已通过；`-D warnings` 被本 change 外既有 Clippy 告警阻断，详见 review 证据）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. EvidenceStore discovery 与治理 fingerprint

- [x] 2.1 Red: UT-002 编写 not-enabled、enabled-empty、active/archive discovery、损坏 change 隔离和路径逃逸失败测试
- [x] 2.2 Green: UT-002 实现 `GovernanceEvidenceStore` 端口与 `FsGovernanceEvidenceStore`，输出规范化 evidence snapshot、artifact refs 和 failures
- [x] 2.3 Refactor: UT-002 收口路径规范化、文件大小限制、稳定排序和跨平台权限错误适配，保持 store 不解释 readiness
- [x] 2.4 为 UT-003/UT-004 补齐版本化 `.spec` fixture builder 和 expected JSON，记录 `@uni-sw/unispec` 0.1.0 来源、目标落点与改写方式
- [x] 2.5 执行 ST-001、ST-002、ST-004、ST-009 的 evidence/tree/hash 验证，确认 enabled-empty ready 且 `.spec` 原文不进入其它 truth 层

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] ST-001 / ST-002 / ST-004 / ST-009 通过
- [x] 本大 task 局部质量检查已完成（相关 tests 与 `cargo fmt --check` 已通过；`-D warnings` 被本 change 外既有 Clippy 告警阻断，详见 review 证据）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. SQLite derived cache 与独立 refresh plan

- [x] 3.1 Red: UT-005 编写 content-based fingerprint、governance tables、fingerprint-bound read 和 transaction failure 保留旧 snapshot 的失败测试
- [x] 3.2 Green: UT-005 实现 bytes/path/location fingerprint，并新增 `governance_snapshots/governance_changes/governance_artifact_refs/governance_issues` schema 与 `SqliteGovernanceCache` 事务 wrapper
- [x] 3.3 Refactor: UT-005 固定 fingerprint 排序与 mtime 非 identity 约束，将底层 SQL 留在 `sqlite_store`、业务入口留在 wrapper，移除 JSON 双写或 artifact body persistence
- [x] 3.4 Red: UT-005 增加 cache schema/policy version 不匹配和其它 runtime tables 不受影响的失败断言
- [x] 3.5 Green: UT-005 实现治理派生表的局部丢弃/重建逻辑，不清理 index、knowledge、runtime 或 LLM cache
- [x] 3.6 Refactor: UT-005 统一 snapshot replace/read API 和确定性排序，保证失败 rollback 后旧 fingerprint 仍可查询
- [x] 3.7 执行 ST-006 的 `.spec`-only delta 验证，比较 source/graph/knowledge snapshot 与 governance cache 前后状态

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] ST-006 通过
- [x] 本大 task 局部质量检查已完成（SQLite 专项 tests 与 `cargo fmt --check` 已通过；`-D warnings` 被本 change 外既有 Clippy 告警阻断，详见 review 证据）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. Governance service、workflow composition 与 query refs

- [x] 4.1 Red: UT-006 编写 status live-read、validate bypass-cache、source no-op update refresh 和 core fusion 隔离失败测试
- [x] 4.2 Green: UT-006 实现 `GovernanceService` 的 status/list/inspect/validate/refresh，并接入 status/update 产品响应
- [x] 4.3 Refactor: UT-006 将 governance preflight 与 source `ChangeSet` 拆成独立 plan，统一 next action 优先级并新增 workflow `review_governance`
- [x] 4.4 Red: UT-007 编写 ready/blocked/stale/conflict query refs、freshness gate 和非治理 trust 隔离失败测试
- [x] 4.5 Green: UT-007 实现结构化 governance query adapter，投影 summary/artifact/diagnostic refs 和 provenance/confidence/action
- [x] 4.6 Refactor: UT-007 复用既有 query route/result DTO，移除固定 `QueryGovernanceReadiness::NotEnabled` 与旧扁平字段
- [x] 4.7 执行 ST-001、ST-006、ST-007、ST-008、ST-009，确认普通 Wiki route 不被治理问题连带降级

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] ST-001 / ST-006 / ST-007 / ST-008 / ST-009 通过
- [x] 本大 task 局部质量检查已完成（runtime integration tests 与 `cargo fmt --check` 已通过；`-D warnings` 被本 change 外既有 Clippy 告警阻断，详见 review 证据）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. Rust/TS transport 闭集与无副作用验收

- [x] 5.1 Red: UT-008 在 `packages/spec-wiki/src/index.test.ts` 编写 governance summary、五态 readiness、issue/ref 和旧字段移除的失败测试
- [x] 5.2 Green: UT-008 扩展 `parseResult.ts` 的类型与 parser，接入 status/query/update 唯一 `governance` summary 和 `review_governance` action
- [x] 5.3 Refactor: UT-008 抽取 governance parser helper，保留未知扩展字段但拒绝未知闭集与残缺必需字段，不复制 policy rule
- [x] 5.4 对齐 Rust model contract tests与 TS fixture payload，扫描并删除产品路径中的 `governance_readiness: "not_enabled"` 专用兼容类型/断言
- [x] 5.5 执行 ST-010 的 Rust -> JSON -> TS contract 与 active/archive tree/hash 前后比较，确认所有治理入口只读
- [x] 5.6 执行 change 级完整验证：Rust model/runtime tests、TS tests、lint、format、clippy，并记录后续 review 所需命令证据

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] ST-010 通过
- [x] 本大 task 局部质量检查已完成（TS tests、lint 与 `cargo fmt --check` 已通过；workspace Clippy 被本 change 外既有告警阻断，详见 review 证据）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001 | 2、4 | 2.1-2.3、4.1-4.3、UT-002、UT-006 |
| ST-002 | 2 | 2.1-2.5、UT-002 |
| ST-003 | 1 | 1.4-1.10、UT-003、UT-004 |
| ST-004 | 1、2 | 1.7-1.10、2.1-2.5、UT-002、UT-004 |
| ST-005 | 1、2 | 1.4-1.10、2.4、UT-003、UT-004 |
| ST-006 | 3、4 | 3.1-3.7、4.1-4.3、UT-005、UT-006 |
| ST-007 | 4 | 4.1-4.7、UT-006、UT-007 |
| ST-008 | 4 | 4.4-4.7、UT-007 |
| ST-009 | 2、4 | 2.5、4.4-4.7、UT-002、UT-007 |
| ST-010 | 1、5 | 1.1-1.3、5.1-5.6、UT-001、UT-008 |

## 执行顺序

- 先完成 Task 1，固定 DTO、rule id 和 policy 闭集，避免后续 storage/workflow 反复改合同。
- Task 2 依赖 Task 1 的 evidence DTO 与 policy input；完成后可稳定生成 fingerprint 和 parity snapshot。
- Task 3 依赖 Task 2 的 fingerprint/derived snapshot，独立完成 SQLite transaction 后再接 workflow。
- Task 4 依赖 Task 1-3，统一接入 status/update/query，禁止在各 workflow 内重复计算 policy。
- Task 5 最后对齐 TS parser 和跨语言 contract，并执行全 change 验收。

## 暂缓事项

- `spec-wiki changes/change/validate` 顶层命令路由、help 与 host forwarding：留给 `cli-product-surface` child。
- archive dry-run、manifest、confirm、目录移动、parent 写回与恢复：留给 `archive-dry-run-manifest` child。
- governance 全文 FTS、release gate、quality gate、capability baseline 和 Wiki 正文投影：本 proposal 非目标。
