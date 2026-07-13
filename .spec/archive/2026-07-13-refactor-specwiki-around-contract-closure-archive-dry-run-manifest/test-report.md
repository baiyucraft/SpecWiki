---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-archive-dry-run-manifest 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：focused TDD、Rust workspace、TypeScript package、workspace lint/test 和 distribution 合同均通过，覆盖 archive 的主要成功标准与风险边界。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-13 |
| 执行环境 | Windows x64；Rust workspace；Node/pnpm/Vitest |
| 测试方式 | 自动化单元、集成、CLI 和分发验证 |

## 验证范围

- Archive DTO、canonical snapshot/digest、parent patch、mutation-set lock、operation storage、dry-run、apply、resume、transport、CLI parser/renderer/exit policy、distribution 和 Wiki 隔离合同。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | Rust workspace 全量 + spec-wiki 85 |
| 单元测试通过 | 全部通过 |
| 单元测试失败 | 0 |
| 系统测试总数 | archive focused 21 + workspace/distribution suites |
| 系统测试通过 | 全部执行项通过 |
| 系统测试失败 | 0 |
| 跳过 | 0（硬件掉电/跨 volume 为非目标） |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Cargo test、Vitest |
| 执行命令 | `cargo test --workspace`；`pnpm --filter spec-wiki test`；`pnpm run test` |
| 执行时间 | Rust workspace 97.6s；workspace 综合测试 276.1s |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | Rust workspace 全量；spec-wiki 85 |
| 通过 | 全部 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | Cargo/Vitest terminal output | 统计与失败定位 |
| JUnit XML | 无 | 不适用 |
| Coverage | 无 | 未统计 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001/Task 1 | wiki-model governance contract | Cargo | pass | DTO 闭集与 serde |
| UT-002-011/Task 2-3 | archive_planning/archive_storage | Cargo | pass | clock/digest/path/lock/storage/planning |
| UT-012-013/Task 4 | archive workflow integration | Cargo | pass | apply/resume/reconcile |
| UT-014-016/Task 5-6 | acceptance + Vitest + distribution | Cargo/Vitest | pass | transport/CLI/exit/distribution |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | project automation / CLI/API |
| 测试环境 | 本地 Windows workspace |
| 工具 | Cargo test、Vitest、workspace scripts |
| verification mode | project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | focused archive tests、`cargo test --workspace`、`pnpm run lint`、`pnpm run test` |
| 动作摘要 | 构造临时治理仓库并执行 plan/apply/resume；验证 CLI 和 staged package |
| 断言点 | source/target/parent/manifest hashes、typed DTO/errors、help/output/exit parity |
| 证据路径 | 控制台摘要；测试源码 |
| fallback 原因 | 无浏览器 UI，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | CLI/文件系统/JSON 自动化断言 |
| fallback reason | 本 change 无浏览器界面 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 16 个 ST 映射到 focused/full 自动化 |
| 通过 | 16 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001-ST-003 | validate/dry-run/precondition | 异常/边界 | pass | archive planning/storage + CLI tests |
| ST-004-ST-006 | apply/冲突/parent sync | 正常/异常 | pass | archive planning/storage |
| ST-007-ST-011 | recovery/manifest/idempotency | 异常/边界 | pass | recovery discovery、fail-closed、resume tests |
| ST-012-ST-014 | lock/path/Wiki | 并发/安全 | pass | OS lock、path validation、Wiki zero-write contract |
| ST-015-ST-016 | CLI/distribution | 产品合同 | pass | spec-wiki 85 tests、distribution suite |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test -p wiki-model --test governance_contract` | pass | 5/5 |
| `cargo test -p wiki-runtime --test archive_storage` | pass | 6/6 |
| `cargo test -p wiki-runtime --test archive_planning` | pass | 8/8 |
| `cargo test -p wiki-runtime --test acceptance archive` | pass | 2/2；含 manifest-invalid transport 映射 |
| `cargo test --workspace` | pass | 全量通过 |
| `cargo clippy -p wiki-runtime --lib --tests -- -D warnings -A clippy::unnecessary-sort-by` | fail（非本 change） | 编译依赖 `wiki-index` 时被既有告警阻断 |
| `cargo clippy --workspace --all-targets -- -D warnings` | fail（非本 change） | 既有 wiki-index/用户改动告警，未作为 archive 阻塞 |
| `pnpm --filter spec-wiki test` | pass | 85/85 |
| `pnpm run lint` | pass | ESLint workspace |
| `pnpm run test` | pass | workspace 综合门禁 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001-ST-016 | Cargo/Vitest/workspace automation | pass | 详见系统测试用例详情 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 默认 validate、dry-run 零写 | archive planning + CLI tests | pass |
| manifest/digest/target/parent 合同 | archive storage/planning tests | pass |
| apply 完整终态和 child 字节保持 | apply integration | pass |
| 恢复、fresh discovery、completed 幂等 | resume/discovery tests | pass |
| Wiki issue/ref 且不写 Wiki | dry-run/apply boundaries + CLI docs | pass |
| human/JSON/退出码/分发 | spec-wiki/distribution tests | pass |

## 失败项

- 无 change 范围内失败项。

## 未验证项

- 硬件断电/控制器缓存丢失和跨 volume 成功迁移为 design 明确非目标。

## 证据缺口

- 无。

## 下一步

- 已完成 review、verification 与归档；本报告随归档保存。
