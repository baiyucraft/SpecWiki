---
implementation-ready: true
---

# refactor-specwiki-around-contract-closure-archive-dry-run-manifest 任务计划

## 任务总览

任务按 design 能力块和 system-tests 验收边界拆分，采用 TDD Red -> Green -> Refactor。Rust domain/storage/workflow 先于 transport/CLI，系统和分发门禁最后执行。

## 实现模式

tdd

先写失败单元测试并确认失败，再写最小实现，通过后重构；真实文件系统、并发和 CLI 行为由对应系统测试补充。

## 1. 共享 archive 合同与确定性输入

- [x] 1.1 Red: UT-001 为 archive DTO、闭集枚举和 serde 字段编写失败测试
- [x] 1.2 Green: UT-001 在 `wiki-model` 增加 request/report/manifest/checkpoint/outcome/error DTO
- [x] 1.3 Refactor: UT-001 保持既有 governance DTO JSON 不变
- [x] 1.4 Red: UT-002 为固定 UTC clock、operation-id 和 target 编写失败测试
- [x] 1.5 Green: UT-002 增加可注入 clock/id provider，固定 plan target
- [x] 1.6 Refactor: UT-002 保证 provider 不污染 CLI 公共 API
- [x] 1.7 Red: UT-007 为 parent meta 未知字段语义保留编写失败测试
- [x] 1.8 Green: UT-007 实现 archiveStatus/archivedAt/archivedTo 变换
- [x] 1.9 Refactor: UT-007 明确 canonical YAML rewrite 和 before/after hash
- [x] 1.10 Red: UT-008 为 split 唯一 section/marker 编写失败测试
- [x] 1.11 Green: UT-008 实现唯一 marker 变换和歧义 conflict
- [x] 1.12 Refactor: UT-008 限定文本范围，不引入通用 Markdown 重写

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（`cargo fmt --all -- --check`、`cargo test -p wiki-model --test archive_contract`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 2. ArchiveFs 快照、路径安全、锁与 operation storage

- [x] 2.1 Red: UT-003 为 canonical source snapshot 编写失败测试
- [x] 2.2 Green: UT-003 实现稳定排序、流式 hash 和 repo-relative path 规范化
- [x] 2.3 Refactor: UT-003 排除 mtime，收敛 snapshot helper
- [x] 2.4 Red: UT-004 为 precondition digest inclusion/exclusion matrix 编写失败测试
- [x] 2.5 Green: UT-004 实现版本化 canonical BLAKE3 digest
- [x] 2.6 Refactor: UT-004 固定 test vectors，不复用全仓 fingerprint
- [x] 2.7 Red: UT-005 为 symlink/reparse/special/path escape 编写失败测试
- [x] 2.8 Green: UT-005 实现危险 source entry 拒绝和 target no-overwrite
- [x] 2.9 Refactor: UT-005 将路径安全集中在 storage adapter
- [x] 2.10 Red: UT-009 为完整 mutation-set lock key/OS lock 编写失败测试
- [x] 2.11 Green: UT-009 接入 OS advisory exclusive lock，覆盖 child+parent
- [x] 2.12 Refactor: UT-009 统一 fresh/resume lock 获取和释放
- [x] 2.13 Red: UT-010 为 immutable plan、append-only checkpoint、attempt 序号编写失败测试
- [x] 2.14 Green: UT-010 实现 plan/checkpoint/result operation storage 和 best-effort flush/rename
- [x] 2.15 Refactor: UT-010 让聚合 manifest 从 plan/checkpoints/result 重建
- [x] 2.16 Red: UT-011 为 operation discovery 和多 operation conflict 编写失败测试
- [x] 2.17 Green: UT-011 实现按 change/mutation-set 发现 recovery operation
- [x] 2.18 Refactor: UT-011 拒绝 fresh apply 创建第二个未完成 operation

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（`cargo test -p wiki-runtime --test archive_storage`、`cargo clippy -p wiki-runtime --all-targets -- -D warnings`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 3. Planning 与 dry-run

- [x] 3.1 Red: UT-006 为 validate-first、单次 live evaluation 和 zero-write planner 编写失败测试
- [x] 3.2 Green: UT-006 实现 `ArchiveService::plan`、readiness/blocking issue、parent/Wiki refs
- [x] 3.3 Refactor: UT-006 保证 dry-run `persisted=false`、`resumable=false` 且不创建 `.spec/.runtime`
- [x] 3.4 Red: ST-001/ST-002 为默认 archive、显式 `--dry-run` 和 not-ready 补充系统测试
- [x] 3.5 Green: 使默认/显式 dry-run 输出等价且业务树 hash 不变
- [x] 3.6 Refactor: 固定 operation root/target 输出合同
- [x] 3.7 Red: ST-003 为 source/parent/target 变化补充系统测试
- [x] 3.8 Green: 在首个 mutation 前获取完整锁并重算 digest
- [x] 3.9 Refactor: 统一 precondition failure 和 recovery hint

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（`cargo test -p wiki-runtime --test archive_planning`、`archive_workflows`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 4. Apply、reconcile 与恢复

- [x] 4.1 Red: UT-012 为 source/target/parent before-after/unknown 状态矩阵编写失败测试
- [x] 4.2 Green: UT-012 实现 fail-closed reconcile 状态分类
- [x] 4.3 Refactor: 统一 continue/completed/recovery_required/conflict 状态表
- [x] 4.4 Red: UT-013 为 apply 顺序、final live verify 和 completed 条件编写失败测试
- [x] 4.5 Green: UT-013 实现 source rename、parent meta/split replace、checkpoint、final verify 编排
- [x] 4.6 Refactor: 保持 workflow 编排与 ArchiveFs 分离
- [x] 4.7 Red: ST-004/ST-006/ST-007 为成功终态和 mutation failure 注入补充系统测试
- [x] 4.8 Green: 实现 recovery_required、failure_step、recovery_hint 和 mutation 对账
- [x] 4.9 Refactor: 不自动 rollback，不误报 completed
- [x] 4.10 Red: ST-008/ST-009/ST-010/ST-011 为 checkpoint lag、manifest corruption、fresh discovery、completed resume 补充系统测试
- [x] 4.11 Green: 实现 resume 前滚、损坏 fail closed、已有 operation discovery、already_completed
- [x] 4.12 Refactor: 保留 completed operation plan/checkpoints/staging，不在本 change 自动清理

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（`cargo test -p wiki-runtime --test archive_workflows`、`archive_recovery`、`archive_concurrency`）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 5. Rust transport 与 TypeScript CLI 合同

- [x] 5.1 Red: UT-014 为 archive CoreCommand、typed error 和 JSON envelope 编写失败测试
- [x] 5.2 Green: UT-014 扩展 Rust transport、CoreErrorKind 和 archive dispatch
- [x] 5.3 Refactor: UT-014 保持 transport 不复制 workflow 规则
- [x] 5.4 Red: UT-015 为默认 dry-run、`--apply`、`--resume`、互斥参数和 help 编写失败测试
- [x] 5.5 Green: UT-015 扩展 CommandSpec、CLI parser 和 forwardCore 参数
- [x] 5.6 Refactor: UT-015 保持 archive 仅在 advanced/command help 且不进入 streaming
- [x] 5.7 Red: UT-016 为 parser、human renderer 和 exit policy 编写失败测试
- [x] 5.8 Green: UT-016 扩展 TS DTO parser、human 输出和 0/1/2/64 映射
- [x] 5.9 Refactor: UT-016 禁止根据错误文本推导 outcome/exit code
- [x] 5.10 Red: ST-015/ST-016 为 source/staged help、JSON、退出码补充系统测试
- [x] 5.11 Green: 使 distribution parity 和 dry-run smoke 通过
- [x] 5.12 Refactor: 更新 `.wiki/04-对外方法/00-CLI.md`，标明 archive 不写 Wiki

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（`pnpm --filter spec-wiki test`、`pnpm run lint`、distribution tests）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 6. 系统门禁、分发和全量回归

- [x] 6.1 Red: ST-012/ST-013 为 sibling concurrency、Windows path safety 和 Wiki tree hash 编写失败系统测试
- [x] 6.2 Green: 使真实 lock、平台 path safety 和 Wiki isolation 通过
- [x] 6.3 Refactor: 补齐 capability probe、平台 skip evidence 和故障诊断
- [x] 6.4 Red: ST-016 为 distribution parity 和 staged archive smoke 编写失败测试
- [x] 6.5 Green: 使 source/staged CLI 合同一致
- [x] 6.6 Refactor: 清理测试辅助重复，不扩大 archive 事务边界
- [x] 6.7 运行最终门禁：`cargo fmt --all -- --check`、`cargo test`、`cargo clippy --workspace --all-targets -- -D warnings`、`pnpm run lint`、`pnpm run test`

### CheckList

- [x] 失败测试已确认
- [x] 最小实现后测试通过
- [x] 重构后测试仍通过
- [x] 本大 task 局部质量检查通过（Rust/TypeScript/分发/生命周期全量门禁）
- [x] 注释规范检查完成（参考 `.wiki/02-开发指南/00-代码注释规范.md`）

## 用例到任务映射

| 系统测试用例 | 大 task | 小 task / 验证 |
| --- | --- | --- |
| ST-001、ST-002 | 3 | 3.1-3.6 |
| ST-003 | 3、4 | 3.7-3.9、4.4-4.6 |
| ST-004、ST-006、ST-007 | 4 | 4.4-4.9 |
| ST-005、ST-009、ST-010、ST-011 | 2、4 | 2.13-2.18、4.10-4.12 |
| ST-008 | 4 | 4.1-4.3、4.10-4.12 |
| ST-012 | 2、6 | 2.10-2.12、6.1-6.3 |
| ST-013 | 2、6 | 2.7-2.9、6.1-6.3 |
| ST-014 | 3、6 | 3.4-3.6、6.1-6.3 |
| ST-015、ST-016 | 5、6 | 5.4-5.12、6.4-6.6 |

## 执行顺序

```text
1 共享 DTO、clock/id、parent transforms
2 snapshot/digest/path safety/lock/operation storage
3 planner、dry-run、precondition
4 apply、reconcile、resume、fault integration
5 Rust transport、TypeScript CLI、renderer、exit policy
6 system/distribution/full regression
```

## 暂缓事项

- 真正断电、硬件缓存丢失和跨 volume copy+delete 不在本 change 执行。
- 不实现自动 rollback、通用 repair 或 operation staging 自动清理。
- 不保留旧 archive CLI/manifest schema 兼容层。
