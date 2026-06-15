---
verification-result: pass
scope: full
---

# refactor-specwiki-around-contract-closure-page-tree-contract 测试报告

## 验证结论

- verification-result: pass
- scope: full
- 结论摘要：页面树合同收口相关的 model predicate、runtime 写入 / 查询 / 恢复路径、acceptance baseline 与文档规格搜索复核均通过。

## 执行信息

| 字段 | 值 |
| --- | --- |
| 执行时间 | 2026-06-15T21:35:54+08:00 |
| 执行环境 | Windows / PowerShell / Rust cargo test / UniSpec CLI |
| 测试方式 | 自动化测试 + 文档搜索复核 |

## 验证范围

- `wiki-model` 正式页面树 predicate
- `wiki-runtime` init / update / restore / query / sync / rebuild / acceptance 路径
- `.spec/changes/**`、代码测试与文档中旧页面目录口径

## 部分范围

- 无

## 测试结果汇总

| 指标 | 值 |
| --- | --- |
| 单元测试总数 | 4 |
| 单元测试通过 | 4 |
| 单元测试失败 | 0 |
| 系统测试总数 | 34 |
| 系统测试通过 | 34 |
| 系统测试失败 | 0 |
| 跳过 | 0 |
| 证据缺口 | 0 |

## 单元测试报告

### 单元测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试框架 | cargo test |
| 执行命令 | `cargo test -p wiki-model official_wiki_relative_path -- --nocapture`; `cargo test -p wiki-runtime --lib storage::wiki_fs::tests::official_page_path -- --nocapture` |
| 执行时间 | 未记录 |
| 框架报告 | 控制台摘要 |

### 单元测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 4 |
| 通过 | 4 |
| 失败 | 0 |
| 跳过 | 0 |
| 覆盖率 | 未统计 |

### 单元测试框架输出

| 输出类型 | 路径 / 摘要 | 用途 |
| --- | --- | --- |
| 控制台摘要 | `wiki-model` 2 passed；`wiki-runtime` lib 2 passed | 正式页面树 predicate 与 storage guard |
| JUnit XML | 无 | 无 |
| Coverage | 无 | 无 |

### 单元测试失败项

| 测试文件 | 用例名 | 错误摘要 | 影响 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

### UT / Task 覆盖

| UT / Task | 验证方式 | 框架证据 | 结果 | 说明 |
| --- | --- | --- | --- | --- |
| Task 1.3 / 1.5 | `cargo test -p wiki-model official_wiki_relative_path -- --nocapture` | 2 passed | pass | 覆盖根 `INDEX.md`、栏目 `INDEX.md`、`NN-主题.md` 与隐藏目录排除 |
| Task 2.3 / 2.6 | `cargo test -p wiki-runtime --lib storage::wiki_fs::tests::official_page_path -- --nocapture` | 2 passed | pass | 覆盖 runtime storage 层正式路径 guard |

## 系统测试报告

### 系统测试执行信息

| 字段 | 值 |
| --- | --- |
| 测试方式 | CLI / runtime integration / acceptance / 搜索复核 |
| 测试环境 | 本地仓库；临时测试目录由 runtime tests 创建 |
| 工具 | cargo test、unispec、rg |
| configured playwright | true |
| verification mode | CLI/API |
| 入口 URL | 无 |
| 浏览器 / viewport | 不适用 |
| 执行命令或动作 | 见“测试命令和结果” |
| 动作摘要 | 无浏览器交互；本 change 不涉及 UI |
| 断言点 | 文件路径 predicate、metadata/state/restore/query 输出、搜索命中语义 |
| 证据路径 | 控制台摘要 |
| fallback 原因 | change 不涉及 UI/browser；Playwright 不适用 |

### 浏览器交互验证 evidence

| 字段 | 值 |
| --- | --- |
| configured playwright | true |
| verification mode | CLI/API |
| host Playwright availability | not-applicable |
| manual verification | 无 |
| configured imageAnalysis | true |
| actual image analysis | not-used |
| evidence paths | 无 |
| assertion points | 非图片断言：cargo test、unispec validate、rg 搜索复核 |
| fallback reason | change 不涉及 UI/browser 或图片内容验证 |

### 系统测试结果汇总

| 指标 | 值 |
| --- | --- |
| 总用例数 | 34 |
| 通过 | 34 |
| 失败 | 0 |
| 跳过 | 0 |
| 关键路径通过率 | 100% |

### 系统测试用例详情

| 用例 ID | 标题 | 类型 | 结果 | 证据 / 备注 |
| --- | --- | --- | --- | --- |
| ST-001 | init / planner 只生成正式页面树 | 正常 | pass | `cargo test -p wiki-runtime --test acceptance -- --nocapture` 11 passed；`init_builds_formal_runtime_and_reports_v0_2_ready_state` passed |
| ST-002 | metadata / state / SQLite 只记录正式页面 | 正常 | pass | acceptance suite、wiki_fs guard、restore drift test passed |
| ST-003 | query 不读取旧页面目录 | 异常 | pass | `query_ignores_pages_directory_markdown_fallback` passed；query/sync/rebuild suite passed |
| ST-004 | update / rebuild / restore / status 不消费旧页面目录 | 边界 | pass | `update_keeps_formal_runtime_ready_after_source_change`、`restore_refuses_page_snapshot_drift_even_when_artifacts_exist`、query/sync/rebuild suite passed |
| ST-005 | 文档、测试与 fixture 不再把旧页面目录写作新版目标 | 搜索复核 | pass | `.spec/changes` 搜索仅剩目标 change 的排除语境和 parent split；全仓搜索仅剩代码负向断言 |

### 系统测试失败项

| 用例 ID | 预期 | 实际 | 原因分析 | 后续处理 |
| --- | --- | --- | --- | --- |
| 无 | 无 | 无 | 无 | 无 |

## 测试命令和结果

| 命令 / 验证动作 | 结果 | 证据 / 说明 |
| --- | --- | --- |
| `unispec validate refactor-specwiki-around-contract-closure-page-tree-contract` | pass | change artifact 校验通过 |
| `unispec validate formalize-declared-authoring-contract` | pass | sibling active spec 同步后校验通过 |
| `unispec validate iteration-12-9-knowledge-system-completeness` | pass | sibling active design 口径校验通过 |
| `cargo test -p wiki-model official_wiki_relative_path -- --nocapture` | pass | 2 passed |
| `cargo test -p wiki-runtime --test acceptance -- --nocapture` | pass | 11 passed |
| `cargo test -p wiki-runtime --test runtime query_ignores_pages_directory_markdown_fallback -- --nocapture` | pass | 1 passed |
| `cargo test -p wiki-runtime --test runtime query_falls_back_to_markdown_and_returns_empty_result -- --nocapture` | pass | 1 passed |
| `cargo test -p wiki-runtime --lib storage::wiki_fs::tests::official_page_path -- --nocapture` | pass | 2 passed |
| `cargo test -p wiki-runtime --test runtime init_builds_formal_runtime_and_reports_v0_2_ready_state -- --nocapture` | pass | 1 passed |
| `cargo test -p wiki-runtime --test runtime update_keeps_formal_runtime_ready_after_source_change -- --nocapture` | pass | 1 passed |
| `cargo test -p wiki-runtime --test runtime restore_refuses_page_snapshot_drift_even_when_artifacts_exist -- --nocapture` | pass | 1 passed |
| `cargo test -p wiki-runtime --test runtime query_sync_rebuild -- --nocapture` | pass | 16 passed |
| `rg -n "\.wiki/pages|\.wiki/pages/\*\*|\.knowledge \+ pages|pages \+ metadata|runtime page projection|runtime 页面投影" .spec\changes -S` | pass | 仅剩目标 change 的排除 / 历史 / 验收语境和 parent split |
| `rg -n "\.wiki/pages|\.wiki/pages/\*\*" . --glob "!.git/**" --glob "!target/**" --glob "!node_modules/**" -S` | pass | 仅剩代码测试负向断言 |
| `cargo test -p wiki-runtime --test runtime query_ignores_pages_directory_markdown_fallback query_falls_back_to_markdown_and_returns_empty_result -- --nocapture` | skipped | Cargo 不接受第二个 filter；两个用例已分别重跑并通过 |

## 系统测试用例覆盖

| 系统测试用例 | 验证方式 | 结果 | 说明 |
| --- | --- | --- | --- |
| ST-001 | acceptance suite、init targeted test、model predicate tests | pass | 覆盖正式页面树生成与路径合法性 |
| ST-002 | acceptance suite、wiki_fs guard、restore drift test | pass | 覆盖 metadata/state/restore 边界 |
| ST-003 | query ignored pages targeted test、query/sync/rebuild suite | pass | 覆盖旧目录干扰文件不被 query 返回 |
| ST-004 | update targeted test、restore targeted test、query/sync/rebuild suite | pass | 覆盖 update / rebuild / restore / status 相关边界 |
| ST-005 | rg 搜索复核 | pass | 确认旧页面目录不再作为新版目标目录正向出现 |

## 成功标准覆盖

| 成功标准 | 验证证据 | 结果 |
| --- | --- | --- |
| `init / update / rebuild` 写入目标不包含旧页面目录 | ST-001、ST-004、acceptance suite、query/sync/rebuild suite | pass |
| metadata、state、SQLite 只记录正式页面树 | ST-002、acceptance suite、wiki_fs guard、restore test | pass |
| 默认 query 不读取旧页面目录 | ST-003、query ignored pages targeted test | pass |
| restore / rebuild 不从旧页面目录重建正式状态 | ST-002、ST-004、restore drift test、query/sync/rebuild suite | pass |
| update 不刷新旧页面目录，也不纳入 affected scope | ST-004、update targeted test | pass |
| status 不提供旧目录专用诊断或清理建议 | ST-004、runtime ready-state test 与 status 输出路径检查 | pass |
| 文档、测试和 fixture 不再把旧页面目录写作新版目标目录 | ST-005、rg 搜索复核、sibling active specs 修正 | pass |

## 失败项

- 无

## 未验证项

- 无

## 证据缺口

- 无

## 下一步

- review-report.md 通过后使用 unispec-archive 进入归档检查。
