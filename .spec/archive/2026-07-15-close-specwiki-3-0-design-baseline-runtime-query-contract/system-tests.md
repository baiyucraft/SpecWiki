# close-specwiki-3-0-design-baseline-runtime-query-contract 系统测试用例

## 用例总览

用例覆盖稳定 term-only request、route-groups-only response、module route、route-local ranking、readiness/trust/provenance、typed errors、Rust/TS/CLI/Agents parity、长期 Wiki authority 和 richer query 延期边界。验证以 Rust integration、TS Vitest、workspace contract test 与 CLI E2E 组合完成。

## 系统测试用例

### ST-001 唯一 Runtime / Query authority 与跨层字段一致

- 关联成功标准: 存在唯一 authority；Runtime、CLI、Agents 对稳定字段逐项一致。
- 覆盖设计点: route groups 唯一结果 authority；删除旧派生视图；canonical Wiki 页面。
- 前置条件: 当前仓库源码和 Wiki 可读。
- 操作 / 触发: 执行 contract test，检查 canonical 页面、Rust transport、TS parser/类型、Agents 资产与 E2E payload。
- 期望结果: 外部合同只包含 canonical 字段；`route_groups` 必备；旧 `results/matched_pages/provenance_summary/summary/hits` 不再是 transport 或宿主稳定字段。
- 验证方式: `pnpm exec vitest run --config vitest.config.mjs scripts/tests/runtime-query-contract.test.ts`、Rust transport tests、package tests。

### ST-002 Term-only 输入、空结果与 typed errors 一致

- 关联成功标准: term-only/richer 关系明确；错误、空结果、blocked 与恢复建议跨入口一致。
- 覆盖设计点: non-empty term；`invalid_argument`；`index_not_ready`；合法无命中成功。
- 前置条件: 一个未初始化临时仓库和一个已初始化 fixture。
- 操作 / 触发: 分别通过 Rust IPC/TS CLI 发出缺失 term、空白 term、未初始化 query 和合法无命中 query。
- 期望结果: 前两者为 `invalid_argument`；未初始化为 `index_not_ready` 且 action=`init`；合法无命中成功并返回 `route_groups=[]` 与 empty answer。
- 验证方式: Rust acceptance transport tests、`packages/spec-wiki/src/cli.test.ts`、workspace E2E。

### ST-003 Route fusion、module 命中与 fallback 边界可观测

- 关联成功标准: route tag/result 最小语义可验证；fallback 不伪装 formal hit；后续不需重定义 DTO。
- 覆盖设计点: formal layer fusion + conditional fallback；`index_module_hit/source_module`；固定 route 语义顺序。
- 前置条件: 包含 module、symbol、graph、knowledge、projection 和 fallback 文本的真实 runtime fixture。
- 操作 / 触发: 对已知 module/symbol/knowledge/fallback term 执行 query。
- 期望结果: formal routes 可并行出现；module 命中有独立 route/ref；只有 formal 结果不足时出现 rendered fallback，且 fallback 降级 trust/answer。
- 验证方式: `cargo test -p wiki-runtime --test runtime query_`、query model contract tests。

### ST-004 Route-local ranking、截断、去重与 confidence 正确

- 关联成功标准: ranking、score basis、截断、去重可验证；不可比较 route 不混排。
- 覆盖设计点: group ranking metadata、rank、optional score、真实 BM25 lower-is-better、稳定 tie-break。
- 前置条件: SQLite FTS fixture 产生至少两个不同 BM25 rank 的 symbol/source 命中，并有重复 projection ref。
- 操作 / 触发: 执行 query 并检查 route groups。
- 期望结果: BM25 group 声明 lower-is-better，组内 rank 与数据库相关性一致；source 不再按路径覆盖排序；计数/截断正确；去重稳定；confidence 不由 raw BM25 阈值误判。
- 验证方式: Rust sqlite/index/runtime tests。

### ST-005 Readiness、trust、provenance、answer 与 governance 正交

- 关联成功标准: 每类结果有 provenance/confidence/action/source refs；readiness/trust/answer/action 关系可验证。
- 覆盖设计点: ready/stale/missing/blocked/fallback matrix；闭集 provenance；governance 与 core trust 正交。
- 前置条件: 可切换 graph/readiness 状态、knowledge/projection 可用性和 governance blocker 的 fixture。
- 操作 / 触发: 分别查询 ready、stale、index-not-ready-but-restorable、fallback-only、governance-blocked 场景。
- 期望结果: 每个 route 使用真实 layer/state；source/graph 不硬编码 ready；fallback=`fallback` 且 degraded；governance blocker 不降低普通 core query trust。
- 验证方式: Rust runtime/governance tests、TS strict parser tests。

### ST-006 Richer query 能力具有明确延期门禁

- 关联成功标准: owner/entrypoint/impact/process/community 和 richer input 均有当前或延期分类。
- 覆盖设计点: term-only、module 例外、richer capability upgrade conditions。
- 前置条件: canonical Wiki 页面与宿主资产生成器可读。
- 操作 / 触发: contract test 扫描公开 request、route/ref 闭集和延期表。
- 期望结果: 公开输入没有 intent/focus/scope/traversal；module 已进入稳定 route；owner/entrypoint/impact/process/community 未被宿主私自承诺，且页面记录升级条件。
- 验证方式: workspace contract test、Agents asset tests。

## 覆盖矩阵

| 成功标准 | 系统测试用例 | 验证方式 |
| --- | --- | --- |
| 唯一 authority 与字段一致 | ST-001 | contract + transport + TS/Agents tests |
| term-only 与 richer 边界 | ST-002、ST-006 | IPC/CLI + contract tests |
| route/ranking/截断/去重 | ST-003、ST-004 | Rust model/index/runtime tests |
| provenance/confidence/source refs | ST-004、ST-005 | Rust runtime + TS parser tests |
| readiness/trust/answer/action | ST-002、ST-005 | transport/runtime/governance tests |
| fallback 显式降级 | ST-003、ST-005 | runtime tests |
| 错误、空结果和恢复建议 | ST-002、ST-005 | acceptance + CLI tests |
| richer 能力分类 | ST-006 | Wiki/Agents contract tests |
| 后续 child 可复用 | ST-001、ST-003、ST-006 | canonical authority contract |

## 边界与异常

- 不把 SQLite BM25 绝对值写死，只验证方向、rank 和 metadata。
- 不把固定 route 顺序解释为全局 relevance。
- corruption/I/O error 不允许被 `.ok()` 或默认空数组掩盖；相关路径以 typed failure 或明确 degraded diagnostic 验证。
- 不要求本 change 实现 richer request 或 deferred routes。

## 验证数据与环境

- Rust 临时仓库 fixture、真实 SQLite FTS 表、已存在的 graph/knowledge/governance helpers。
- Node 20+、pnpm workspace、Rust toolchain。
- 不需要网络、浏览器、外部服务或 upstream。

## 未覆盖项

- Storybook/Dagger 大样本质量与 19 项目全量分析属于后续场景/质量门禁 change，本合同 change 不新增该成本；全量 `pnpm test` 继续执行现有综合门禁。

## 参考资料

- `./proposal.md`
- `./design.md`
- `./research/runtime-query-contract-audit.md`
- `.wiki/02-开发指南/01-测试与验收.md`
