---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-projection-writeback-boundaries 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：自动化测试、lint、构建、runtime 集成测试和残留扫描均通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-17T13:00:27+08:00 |
| 执行环境 | Windows / PowerShell / Rust cargo / pnpm / Vitest / SQLite runtime |
| 测试方式 | 自动化测试 + 静态扫描 |

## 验证范围

- projection model serde / validate
- declared writeback knowledge validation
- runtime marker protocol 和 typed diagnostics
- declared owner gate 与 derived section illegal drift
- page-level atomic writeback
- metadata reverse refs
- projection digest persistence and restore rejection
- TypeScript wrapper runtime readiness parsing
- package build、script e2e、distribution tests

## 部分范围

- 无

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | cargo test 全量通过；Vitest package 47 passed；script tests 41 passed |
| 单元测试通过 | 全部通过 |
| 单元测试失败 | 0 |
| 系统测试总数 | Rust runtime integration 145 passed；script e2e/distribution/reference tests 全部通过 |
| 系统测试通过 | 全部通过 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | Rust cargo test / Vitest / ESLint |
| 执行命令 | `cargo test`; `pnpm run lint`; `pnpm run test` |
| 执行时间 | 已记录于本报告执行信息 |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | cargo 全量通过；Vitest 47 + 41 passed |
| 通过 | 全部通过 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `cargo test` 全量通过 | Rust model / knowledge / runtime 验证 |
| 控制台摘要 | `pnpm run lint` 通过 | TS/script lint 验证 |
| 控制台摘要 | `pnpm run test` 通过 | Rust runtime、Vitest、build、script e2e、distribution 验证 |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 未统计 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| UT-001 / Task 1 | `cargo test -p wiki-model --test projection_contract` | cargo 摘要 | pass | SectionOwnership / SectionBinding / ProjectionDigest / SyncResultKind |
| UT-002 / UT-003 / Task 2 | `cargo test -p wiki-runtime --test runtime managed_section_kernel` | cargo 摘要 | pass | marker v2、diagnostics、roundtrip binding |
| UT-004 / Task 3 | `cargo test -p wiki-runtime --test runtime editable_runtime` | cargo 摘要 | pass | drift 五分类、derived declared illegal drift、page-level atomic |
| UT-005 / Task 4 | `cargo test -p wiki-knowledge --test declared_writeback` + runtime sync tests | cargo 摘要 | pass | runtime candidate -> knowledge validate -> runtime commit |
| UT-006 / Task 5 | `metadata_mapper_builds_section_reverse_refs` + knowledge artifact restore tests | cargo 摘要 | pass | section reverse refs 与 restore binding |
| UT-007 / Task 6 | projection digest artifact tests | cargo 摘要 | pass | model ProjectionDigest 作为 recovery anchor |
| UT-008 / Task 7 | `parse_wiki_page_without_markers_is_unmanaged_only` + `.wiki/pages` query guard | cargo 摘要 | pass | legacy heading fallback / old page surface 不作为 runtime success path |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | Rust integration / CLI script automation / package build |
| 测试环境 | 本地临时仓库 fixture、SQLite cache、文件系统 `.wiki` runtime |
| 工具 | cargo / pnpm / Vitest / ESLint |
| verification mode | project automation |
| tool availability | available |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | `cargo test`; `pnpm run lint`; `pnpm run test` |
| 动作摘要 | 执行全量 Rust、TS、script、build、distribution 验证 |
| 断言点 | runtime state、artifact files、metadata refs、snapshot restore、sync outcomes、query/status readiness |
| 证据路径 | 控制台摘要 |
| fallback 原因 | 无 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| verification mode | not-applicable |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| evidence paths | 无 |
| assertion points | 本 change 不涉及浏览器 UI |
| fallback reason | 不适用 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | Rust runtime integration 145；script tests 41 |
| 通过 | 全部通过 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | Generated Section Binding Roundtrip | 正常 | pass | metadata roundtrip / projection contract / runtime artifact tests |
| ST-002 | Declared Writeback Contract | 正常 | pass | `sync_classifies_valid_declared_block_as_declared_writeback`; knowledge validator tests |
| ST-003 | Non-Declared Drift Does Not Pollute Knowledge | 异常 | pass | `sync_rejects_declared_block_inside_derived_section`; `sync_classifies_user_only_edit_as_metadata_only` |
| ST-004 | Page-Level Atomic Writeback | 异常 | pass | `sync_keeps_page_level_atomicity_while_unrelated_page_commits` |
| ST-005 | Restore Binding Closure | 正常 | pass | `knowledge_artifacts_roundtrip_preserves_declared_and_health_records`; restore tests |
| ST-006 | Restore Rejects Broken Binding | 异常 | pass | invalid projection digest / page drift / metadata mismatch tests |
| ST-007 | Legacy Paths And Heading Fallback Are Not Runtime Surface | 边界 | pass | marker-missing parser test; `.wiki/pages` query guard |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test` | pass | Rust workspace 全量通过 |
| `pnpm run lint` | pass | ESLint 通过 |
| `pnpm run test` | pass | Rust runtime、Vitest、build、script e2e、distribution 全部通过 |
| `rg -n "PlannedPage|PlannedSection|LegacyHeadings|parse_with_legacy_headings|flush_legacy_section|legacy-page-digest|legacy heading|legacy 页面" crates/wiki-knowledge crates/wiki-runtime` | pass | 无输出 |
| `rg -n "owner=derived_managed.*wiki:declared|wiki:declared.*owner=derived_managed" crates/wiki-runtime/tests/runtime` | pass | 无输出 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | Rust integration / unit | pass | section binding、metadata reverse refs、projection digest |
| ST-002 | Rust integration / knowledge unit | pass | declared owner gate + knowledge validation |
| ST-003 | Rust integration | pass | derived/static 不写回 declared truth，manual metadata-only |
| ST-004 | Rust integration | pass | 同页 illegal drift 阻止该页提交，无关页可提交 |
| ST-005 | Rust integration | pass | artifact / metadata / manifest restore 闭环 |
| ST-006 | Rust integration | pass | broken binding restore rejection |
| ST-007 | Rust integration / scan | pass | legacy fallback 删除，old page surface 不消费 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| managed section 具备 stable id / ownership / binding / hash | projection contract、managed section、metadata tests | pass |
| metadata 可反查 page -> section -> knowledge refs | metadata mapper tests | pass |
| 合法 declared authoring 走 declared_writeback | editable runtime + knowledge validator tests | pass |
| derived/static drift 不污染 derived knowledge | illegal drift tests | pass |
| manual/unmanaged 只 metadata-only | editable runtime tests | pass |
| init/update/sync/restore binding 一致 | knowledge artifact roundtrip tests | pass |
| runtime 不从 Markdown 反推 derived knowledge | restore refusal / illegal drift tests | pass |
| 删除旧 fallback 和旧 page surface | parser/query guard tests + rg scan | pass |

## 失败项

- 无

## 未验证项

- 无

## 证据缺口

- 无

## 下一步

- `unispec validate refactor-specwiki-around-contract-closure-projection-writeback-boundaries`
- `unispec archive refactor-specwiki-around-contract-closure-projection-writeback-boundaries`
