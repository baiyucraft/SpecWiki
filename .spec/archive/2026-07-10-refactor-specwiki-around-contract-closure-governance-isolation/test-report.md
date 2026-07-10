---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-governance-isolation 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：UT-001 至 UT-008、ST-001 至 ST-010 和 proposal 成功标准均有自动化或结构化替代证据；本 change 范围内无失败项。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-11T01:56:57+08:00 |
| 执行环境 | Windows；rustc/cargo 1.94.1；Node 20.20.0；pnpm 10.6.3；SQLite bundled runtime |
| 测试方式 | Rust/Vitest 自动化测试、lint、formatter、结构化文件与 hash 断言 |

## 验证范围

- Governance DTO、evidence discovery、policy parity、fingerprint/cache、workflow composition、query refs、Rust/TS transport、只读与隔离边界。

## 部分范围

- 无。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 78 个 governance/model/action/TS 相关用例 |
| 单元测试通过 | 78 |
| 单元测试失败 | 0 |
| 系统测试总数 | 10 |
| 系统测试通过 | 10 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust cargo test、Vitest |
| 执行命令 | `cargo test -p wiki-model`; governance 四个专项 test targets；`cargo test -p wiki-runtime --lib domain::runtime_profile::tests`; `pnpm --filter spec-wiki test` |
| 执行时间 | 多轮执行；单命令耗时见控制台摘要，未生成独立 timing report |
| 框架报告 | 控制台摘要；无 JUnit/Coverage 文件 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 78 |
| 通过 | 78 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | governance contract 4/4、evidence 4/4、policy 5/5、cache 2/2、workflow 7/7、runtime_profile 4/4、TS 52/52 | 用例统计与失败定位 |
| JUnit XML | 无 | 不适用 |
| Coverage | 无 | 未统计 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / Task 1.1-1.3 | `cargo test -p wiki-model --test governance_contract` | 4/4 | pass | DTO 闭集、serde、未知值拒绝 |
| UT-002 / Task 2.1-2.3 | `cargo test -p wiki-runtime --test governance_evidence` | 4/4 | pass | disabled/empty、active/archive、fingerprint、大小与路径边界 |
| UT-003 / Task 1.4-1.6 | `cargo test -p wiki-runtime --test governance_policy` | parity/metadata cases | pass | required matrix 与稳定 rule id |
| UT-004 / Task 1.7-1.9 | 同上 | report/archive/parent-child cases | pass | blocked/conflict 与 archive gate |
| UT-005 / Task 3.1-3.6 | `cargo test -p wiki-runtime --test governance_cache` | 2/2 | pass | fingerprint/version 绑定、事务回滚、其它表保留 |
| UT-006 / Task 4.1-4.3 | `governance_workflows` + runtime_profile lib tests | 7/7 + 4/4 | pass | status/update、source no-op、动作优先级与 core 隔离 |
| UT-007 / Task 4.4-4.6 | `governance_workflows` | query cases | pass | 结构化 refs、freshness gate、diagnostic refs |
| UT-008 / Task 5.1-5.4 | `pnpm --filter spec-wiki test` | 52/52 | pass | status/query/update/rebuild transport、五态闭集与 source ref 保真 |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | project automation / unit / integration |
| 测试环境 | 本地临时仓库、真实文件系统、真实 SQLite；无外部服务依赖 |
| 工具 | cargo test、Vitest、ESLint、rustfmt、rg/hash/tree 断言 |
| verification mode | project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | governance 专项、runtime/acceptance、TS test/lint、formatter、UniSpec artifact 检查 |
| 动作摘要 | 构造 `.spec` active/archive fixtures，执行 status/query/update/validate/cache refresh，比较 DTO、fingerprint、state 与 query refs |
| 断言点 | readiness、rule id、artifact refs、transaction rollback、source state 不变、route/trust/action、旧字段不存在、路径/诊断字段保真 |
| 证据路径 | 测试源码与本报告命令摘要；无外部附件 |
| fallback 原因 | 目标不涉及 UI/browser，Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | unit / project automation |
| host Playwright availability | not-applicable |
| manual verification | 无；全部关键行为由文件系统/SQLite/transport 自动化断言覆盖 |
| evidence paths | 无浏览器附件 |
| assertion points | Rust/TS DTO、文件树、hash、SQLite snapshot、query refs 与 workflow response |
| fallback reason | 本 change 无 UI 或浏览器交互 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 10 |
| 通过 | 10 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | 无 `.spec` 与 enabled-empty | 正常/边界 | pass | evidence + workflow tests |
| ST-002 | active/archive discovery | 正常 | pass | evidence/policy tests |
| ST-003 | required artifact 与 metadata | 异常 | pass | policy parity tests |
| ST-004 | 损坏 evidence 隔离 | 异常 | pass | evidence/policy tests |
| ST-005 | report/archive marker parity | 异常/边界 | pass | policy tests |
| ST-006 | `.spec`-only delta 隔离 | 正常 | pass | source no-op workflow test |
| ST-007 | governance blocker 与 core 可用性 | 异常 | pass | query trust/action tests |
| ST-008 | query refs freshness | 正常/边界 | pass | fresh/stale/blocked/no-cache query tests |
| ST-009 | `.spec` 不进入其它 truth/FTS | 安全边界 | pass | evidence refs 不含正文、结构检查 |
| ST-010 | Rust/TS transport 与只读性 | 合同/安全 | pass | model contract + TS + tree/state equality assertions |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test -p wiki-model` | pass | 22/22 |
| governance 四个 runtime test targets | pass | 18/18 |
| `cargo test -p wiki-runtime --lib domain::runtime_profile::tests` | pass | 4/4 |
| `cargo test -p wiki-runtime --test acceptance` | pass | 11/11 |
| `cargo test -p wiki-runtime --test runtime` | pass | 157/157 |
| `pnpm --filter spec-wiki test` | pass | 52/52 |
| `pnpm run lint` | pass | ESLint 0 error |
| `cargo fmt --all -- --check` | pass | 无 diff |
| `git diff --check` | pass | 无 whitespace error |
| `cargo test --workspace` | external-blocked | 本 change 外 `symbols/symbol_graph_analysis.rs:255` 旧 fixture 缺字段；不影响上述相关 targets |
| `cargo clippy --workspace --all-targets -- -D warnings` | external-blocked | 本 change 外 `knowledge_artifact.rs:402` 与 `wiki-index` 既有告警 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001-ST-005 | evidence/policy/model 专项测试 | pass | discovery、parity、gate、错误分类 |
| ST-006 | workflow + state equality | pass | source no-op，仅治理 cache 改变 |
| ST-007-ST-009 | workflow query/status/update tests | pass | blocker 隔离、freshness、truth 分层 |
| ST-010 | Rust model + TS parser + read-only assertions | pass | 跨语言合同与无写副作用 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| 无 `.spec` 返回 not_enabled 且 core 可用 | ST-001、workflow tests | pass |
| 合法 `.spec` 可列出 change/关系/refs/readiness | ST-002、service tests | pass |
| 缺失/冲突/report gate 生成稳定 issue | ST-003-ST-005 | pass |
| validator parity 与规则唯一所有权 | policy tests + TS 无规则扫描 | pass |
| `.spec` delta 只刷新治理派生状态 | ST-006 | pass |
| governance blocker 不全局阻断 core | ST-007 | pass |
| ready/blocked query 返回结构化 refs/action | ST-008 | pass |
| `.spec` 原文不进入 Wiki/code facts/FTS | ST-009 | pass |
| Rust/transport/TS 闭集一致 | ST-010 | pass |
| 无 archive move/manifest/parent 写操作 | 文件树/state equality 与实现审查 | pass |

## 失败项

- 无本 change 范围内失败项。

## 未验证项

- 无。

## 证据缺口

- 无。workspace 级失败已由更窄且直接覆盖本 change 的通过命令替代，并明确记录为外部现有问题。

## 下一步

- review-report.md 已通过，可使用 `unispec-archive`。
