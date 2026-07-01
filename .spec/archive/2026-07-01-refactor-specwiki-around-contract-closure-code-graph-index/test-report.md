---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-code-graph-index 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：`wiki-index`、`wiki-runtime` runtime integration 和 UniSpec validate 均已通过；code graph/index contract closure 的成功标准全部覆盖。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-07-01 |
| 执行环境 | Windows / PowerShell；本地工作区 `E:\project\!byAI\spec-wiki` |
| 测试方式 | TDD 单元测试 + runtime 集成测试 + UniSpec validate |

## 验证范围

- `wiki-index` symbol/raw DTO、store trait、facts-only query adapter。
- `wiki-runtime` SQLite graph schema、snapshot/scoped refresh、phase diagnostics、readiness、query projection。
- `wiki-model` query source refs 扩展与 serde contract。
- `.spec` persistence guard、source authority、FTS、runtime workflow 和 Wiki 稳定说明。

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| `wiki-index` 测试 | 9 passed |
| `wiki-runtime --test runtime` | 157 passed |
| 失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `cargo test -p wiki-model --test query_contract` | pass | 2 passed |
| `cargo test -p wiki-index` | pass | 9 passed |
| `cargo test -p wiki-runtime --test runtime` | pass | 157 passed |
| `cargo test -p wiki-runtime workflows_write_graph_phase_diagnostics_and_unresolved_refs --test runtime` | pass | phase diagnostics / unresolved refs |
| `cargo test -p wiki-runtime query_projects_symbol_range_and_graph_refs_into_public_results --test runtime` | pass | source refs path/range/provenance projection |
| `cargo test -p wiki-runtime query_suppresses_index_routes_for_non_ready_graph_phase_states --test runtime` | pass | non-ready graph route gating |
| `cargo test -p wiki-runtime update_recomputes_process_labels_and_workflow_page --test runtime` | pass | scoped update regression |
| `cargo test -p wiki-runtime update_adds_and_removes_pages_after_structural_changes --test runtime` | pass | snapshot upsert regression |
| `cargo test -p wiki-runtime update_rebuilds_topic_page_and_parent_pages_when_topic_sources_change --test runtime` | pass | graph analysis FK regression |
| `cargo test -p wiki-runtime update_reclaims_removed_units_pages_and_caches_after_structural_delete --test runtime` | pass | snapshot upsert regression |
| `unispec validate refactor-specwiki-around-contract-closure-code-graph-index` | pass | Change is valid |
| `cargo fmt --check -p wiki-index -p wiki-model -p wiki-runtime` | fail / non-blocking | 失败项来自多个既有未触碰文件；本 change 修改过的 Rust 文件已单独 `rustfmt` |

## UT / Task 覆盖

| UT / Task | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| UT-001 / Task 1 | `cargo test -p wiki-index` | pass | SymbolNode identity、range、provenance |
| UT-002 / Task 1 | `cargo test -p wiki-index` | pass | raw capture / unresolved ref DTO |
| UT-003 / UT-009 / Task 2 | `cargo test -p wiki-runtime --test runtime` | pass | graph snapshot、schema/read API、source authority |
| UT-004 / Task 2 | `cargo test -p wiki-runtime --test runtime` | pass | `.spec` persistence / FTS guard |
| UT-010 / Task 2 | `cargo test -p wiki-runtime --test runtime` | pass | scoped graph refresh 清理旧 facts |
| UT-005 / Task 3 | `cargo test -p wiki-runtime --test runtime` | pass | GraphReadiness missing/stale/blocked/rebuilding/ready |
| UT-008 / Task 3 | `cargo test -p wiki-runtime --test runtime` | pass | workflow phase diagnostics / unresolved refs |
| UT-006 / Task 4 | `cargo test -p wiki-index` | pass | enriched symbol/path/graph facts-only hits |
| UT-007 / Task 4 | `cargo test -p wiki-runtime --test runtime` | pass | index route gating and projection |
| Task 5 | Wiki + final validation | pass | 稳定 Wiki 已沉淀，最终命令见本报告 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | `wiki-index` DTO tests + runtime SQLite tests | pass | raw capture 与 unresolved ref 可持久化、读取和诊断 |
| ST-002 | `wiki-index` + runtime query projection tests | pass | SymbolNode 合同和 query refs |
| ST-003 | runtime `.spec` graph persistence guard tests | pass | `.spec` 不进入 graph facts / FTS |
| ST-004 | runtime readiness matrix tests | pass | missing/stale/blocked/rebuilding/ready |
| ST-005 | workflow/runtime integration tests | pass | init/update/rebuild/query 通过 `wiki-index` DTO/traits 消费 graph |
| ST-006 | `wiki-index::query` + runtime route tests | pass | index_symbol_hit / index_path_hit / index_graph_hit |
| ST-007 | workflow diagnostics tests | pass | phase diagnostics 与 unresolved refs 可追踪 |
| ST-008 | SQLite schema/read API tests | pass | graph source authority 闭合 |

## 失败项

- 无。实现过程中曾出现 scoped update FK / snapshot duplicate 回归，已通过 SQLite scoped snapshot 过滤与 graph snapshot upsert 修复，并由 runtime 全套测试验证。

## 未验证项

- 无

## 证据缺口

- 无

## 下一步

- `review-report.md` 已通过后，使用 `unispec archive refactor-specwiki-around-contract-closure-code-graph-index` 归档。
